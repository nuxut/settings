//! CUPS istemcisi (UI'sız): IPP isteğini `ipp` crate'iyle kurar/ayrıştırır, HTTP/1.1'i kendisi
//! konuşur (yerel Unix soketi ya da TCP; TLS yok). Kararlar: `docs/design/printers.md` §1–2.
//!
//! Her çağrı tek bağlantı açar, `Connection: close` ile tek istek gönderir ve bloklar (servis
//! thread'inde çağrılır). Yazıcı/iş adları ve kullanıcı adı loglanmaz.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use ipp::attribute::{IppAttribute, IppAttributeGroup};
use ipp::model::{DelimiterTag, IppVersion, Operation};
use ipp::parser::IppParser;
use ipp::request::IppRequestResponse;
use ipp::value::IppValue;

/// libcups'ın yerel soketi.
pub const DEFAULT_SOCKET: &str = "/run/cups/cups.sock";
/// CUPS'un standart portu.
pub const DEFAULT_PORT: u16 = 631;
/// Bağlanma, okuma ve yazma zaman aşımı.
pub const TIMEOUT: Duration = Duration::from_secs(5);
/// Yanıt gövdesinin üst sınırı (fazlası bozuk yanıt sayılır). Bin kuyruklu sunucunun
/// istenen öznitelikleri ~1 KB/kuyruk.
pub const MAX_RESPONSE: usize = 1024 * 1024;
/// Bir yanıtın okunmasına toplam süre (bayt bayt gönderen sunucu da burada kesilir).
pub const TOTAL_TIMEOUT: Duration = Duration::from_secs(15);
/// HTTP satırı, başlık satırı ve ara (1xx) yanıt sınırları.
pub const MAX_LINE: usize = 8192;
pub const MAX_HEADERS: usize = 128;
pub const MAX_INTERIM: usize = 4;
/// IPP yanıtında en çok değer (öznitelik + ek değer) ve koleksiyon derinliği.
pub const MAX_IPP_VALUES: usize = 50_000;
pub const MAX_IPP_DEPTH: usize = 8;
/// Gösterilen en çok kuyruk ve iş (fazlası kesilir).
pub const MAX_PRINTERS: usize = 1000;
pub const MAX_JOBS: usize = 1000;
/// Gönderilecek belgenin üst sınırı (deneme sayfası birkaç KB).
pub const MAX_DOCUMENT: usize = 1024 * 1024;

/// CUPS'un deneme sayfası (cups paketi) ve biçimi.
pub const TEST_PAGE: &str = "/usr/share/cups/data/testprint";
pub const TEST_PAGE_FORMAT: &str = "application/vnd.cups-banner";

/// CUPS sunucusunun adresi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    /// Yerel soket.
    Unix(PathBuf),
    Tcp { host: String, port: u16 },
}

impl Server {
    /// libcups'la aynı seçim: `CUPS_SERVER`, yoksa yerel soket varsa o, yoksa `localhost:631`.
    pub fn from_env() -> Server {
        let env = std::env::var("CUPS_SERVER").ok();
        Self::choose(env.as_deref(), Path::new(DEFAULT_SOCKET).exists())
    }

    /// [`Self::from_env`]'in saf hâli (testli).
    pub fn choose(cups_server: Option<&str>, socket_exists: bool) -> Server {
        match cups_server.map(str::trim).filter(|s| !s.is_empty()) {
            Some(s) => Self::parse(s),
            None if socket_exists => Server::Unix(DEFAULT_SOCKET.into()),
            None => Server::Tcp { host: "localhost".into(), port: DEFAULT_PORT },
        }
    }

    /// `CUPS_SERVER` biçimi: `/yol` (soket) ya da `ana-makine[:port]` (`[v6]:port` dahil).
    pub fn parse(s: &str) -> Server {
        if s.starts_with('/') {
            return Server::Unix(s.into());
        }
        let (host, port) = match s.rsplit_once(':') {
            Some((h, p)) if !h.is_empty() && (!h.contains(':') || h.ends_with(']')) => {
                (h, p.parse().unwrap_or(DEFAULT_PORT))
            }
            _ => (s, DEFAULT_PORT),
        };
        Server::Tcp { host: host.trim_start_matches('[').trim_end_matches(']').into(), port }
    }

    /// Sunucu bu makinede mi (yerel soket ya da loopback adı).
    pub fn is_local(&self) -> bool {
        match self {
            Server::Unix(_) => true,
            Server::Tcp { host, .. } => matches!(host.as_str(), "localhost" | "127.0.0.1" | "::1"),
        }
    }

    /// CUPS web arayüzünün yönetim sayfası (yalnız yerel sunucuda; soket kurulumunda CUPS
    /// varsayılanı `localhost:631`).
    pub fn admin_url(&self) -> Option<String> {
        match self {
            Server::Unix(_) => Some(format!("http://localhost:{DEFAULT_PORT}/admin")),
            Server::Tcp { port, .. } if self.is_local() => Some(format!("http://localhost:{port}/admin")),
            Server::Tcp { .. } => None,
        }
    }

    /// HTTP `Host` başlığı (libcups yerel sokette `localhost` gönderir).
    fn host_header(&self) -> String {
        match self {
            Server::Unix(_) => "localhost".into(),
            Server::Tcp { host, port } if host.contains(':') => format!("[{host}]:{port}"),
            Server::Tcp { host, port } => format!("{host}:{port}"),
        }
    }
}

/// İstemci hatası.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CupsError {
    /// Sunucuya bağlanılamadı (soket yok, bağlantı reddedildi, ad çözülemedi): CUPS kurulu
    /// değil ya da çalışmıyor.
    #[error("CUPS'a bağlanılamadı")]
    Unreachable,
    #[error("CUPS yanıt vermedi (zaman aşımı)")]
    Timeout,
    /// HTTP düzeyinde ret (401 kimlik doğrulama, 403, 426 TLS…).
    #[error("HTTP {0}")]
    Http(u16),
    /// IPP durum kodu başarısız (`client-error-*`, `server-error-*`).
    #[error("IPP durumu {0:#06x}")]
    Ipp(u16),
    #[error("bozuk yanıt")]
    Malformed,
    /// Gönderilecek belge okunamadı (deneme sayfası yok).
    #[error("belge okunamadı")]
    Document,
}

impl CupsError {
    /// IPP `client-error-not-authorized` / `forbidden` / `not-authenticated` ya da HTTP 401/403.
    pub fn is_denied(&self) -> bool {
        matches!(self, CupsError::Ipp(0x0401..=0x0403) | CupsError::Http(401 | 403))
    }

    /// İş/yazıcı artık yok (`client-error-not-found`, `-gone`) ya da iş zaten bitmiş
    /// (`client-error-not-possible`).
    pub fn is_gone(&self) -> bool {
        matches!(self, CupsError::Ipp(0x0404 | 0x0406 | 0x0407))
    }
}

fn io_error(e: &std::io::Error) -> CupsError {
    use std::io::ErrorKind::*;
    match e.kind() {
        TimedOut | WouldBlock => CupsError::Timeout,
        _ => CupsError::Malformed,
    }
}

// ---- Model -------------------------------------------------------------------------------------

/// `printer-state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrinterState {
    #[default]
    Idle,
    Processing,
    Stopped,
}

/// Bir CUPS kuyruğu (yazıcı ya da sınıf).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Printer {
    /// Kuyruk adı (`printer-name`; kimlik).
    pub name: String,
    /// Açıklama (`printer-info`; boşsa ad gösterilir).
    pub info: String,
    pub location: String,
    pub model: String,
    pub state: PrinterState,
    /// `printer-state-reasons` (`none` hariç).
    pub reasons: Vec<String>,
    pub message: String,
    pub accepting: bool,
    /// `printer-uri-supported`'ın ilki (işlemlerde hedef).
    pub uri: String,
    /// CUPS sınıfı (`printer-type` bit 0).
    pub is_class: bool,
}

impl Printer {
    /// Görünen ad: açıklama, yoksa kuyruk adı.
    pub fn display_name(&self) -> &str {
        if self.info.trim().is_empty() { &self.name } else { self.info.trim() }
    }
}

/// `job-state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JobState {
    #[default]
    Pending,
    Held,
    Processing,
    Stopped,
    Canceled,
    Aborted,
    Completed,
}

impl JobState {
    fn from_enum(v: i32) -> Option<JobState> {
        Some(match v {
            3 => JobState::Pending,
            4 => JobState::Held,
            5 => JobState::Processing,
            6 => JobState::Stopped,
            7 => JobState::Canceled,
            8 => JobState::Aborted,
            9 => JobState::Completed,
            _ => return None,
        })
    }

    /// Bitmemiş (iptal edilebilir) iş mi.
    pub fn active(self) -> bool {
        matches!(self, JobState::Pending | JobState::Held | JobState::Processing | JobState::Stopped)
    }
}

/// Bir yazdırma işi.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Job {
    pub id: i32,
    pub name: String,
    /// Kuyruk adı (`job-printer-uri`'nin son parçası).
    pub printer: String,
    pub state: JobState,
    /// `job-k-octets`.
    pub size_kb: Option<u32>,
    /// `time-at-creation` (UNIX saniye).
    pub created: Option<i64>,
}

// ---- IPP değer yardımcıları ---------------------------------------------------------------------

/// Dize türündeki değer (metin, ad, anahtar sözcük, URI, MIME…); dizi ise ilki.
fn text(v: &IppValue) -> Option<String> {
    match v {
        IppValue::TextWithoutLanguage(_)
        | IppValue::NameWithoutLanguage(_)
        | IppValue::TextWithLanguage { .. }
        | IppValue::NameWithLanguage { .. }
        | IppValue::Keyword(_)
        | IppValue::Uri(_)
        | IppValue::UriScheme(_)
        | IppValue::MimeMediaType(_)
        | IppValue::Charset(_)
        | IppValue::NaturalLanguage(_) => Some(match v {
            IppValue::TextWithLanguage { text, .. } => text.to_string(),
            IppValue::NameWithLanguage { name, .. } => name.to_string(),
            other => other.to_string(),
        }),
        IppValue::Array(a) => a.first().and_then(text),
        _ => None,
    }
}

/// Çok değerli dize (tek değer de liste olur).
fn texts(v: &IppValue) -> Vec<String> {
    match v {
        IppValue::Array(a) => a.iter().filter_map(text).collect(),
        other => text(other).into_iter().collect(),
    }
}

fn int(v: &IppValue) -> Option<i32> {
    match v {
        IppValue::Integer(i) | IppValue::Enum(i) => Some(*i),
        IppValue::Array(a) => a.first().and_then(int),
        _ => None,
    }
}

fn get<'a>(g: &'a IppAttributeGroup, name: &str) -> Option<&'a IppValue> {
    g.get(name).map(IppAttribute::value)
}

fn get_text(g: &IppAttributeGroup, name: &str) -> String {
    get(g, name).and_then(text).unwrap_or_default()
}

/// URI'nin son yol parçası (`ipp://localhost/printers/ofis` → `ofis`), yüzde kodları çözülmüş.
pub fn last_segment(uri: &str) -> String {
    let seg = uri.trim_end_matches('/').rsplit('/').next().unwrap_or_default();
    percent_decode(seg)
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && let Some(h) = s.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(h, 16).ok())
        {
            out.push(h);
            i += 3;
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Kuyruk adından yazıcı URI'si (sunucu `printer-uri-supported` vermediyse).
pub fn printer_uri(name: &str) -> String {
    let mut s = String::from("ipp://localhost/printers/");
    for b in name.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            s.push(char::from(b));
        } else {
            s.push_str(&format!("%{b:02X}"));
        }
    }
    s
}

/// Yazıcı özniteliği grubu → model (saf; testli). Adsız grup `None`.
pub fn parse_printer(g: &IppAttributeGroup) -> Option<Printer> {
    let name = get_text(g, "printer-name");
    if name.is_empty() {
        return None;
    }
    let state = match get(g, "printer-state").and_then(int) {
        Some(4) => PrinterState::Processing,
        Some(5) => PrinterState::Stopped,
        _ => PrinterState::Idle,
    };
    let reasons = get(g, "printer-state-reasons").map(texts).unwrap_or_default().into_iter().filter(|r| r != "none").collect();
    let accepting = !matches!(get(g, "printer-is-accepting-jobs"), Some(IppValue::Boolean(false)));
    let uri = get(g, "printer-uri-supported").and_then(text).unwrap_or_else(|| printer_uri(&name));
    let is_class = get(g, "printer-type").and_then(int).is_some_and(|t| t & 1 != 0);
    Some(Printer {
        info: get_text(g, "printer-info"),
        location: get_text(g, "printer-location"),
        model: get_text(g, "printer-make-and-model"),
        message: get_text(g, "printer-state-message"),
        name,
        state,
        reasons,
        accepting,
        uri,
        is_class,
    })
}

/// İş özniteliği grubu → model (saf; testli). Kimliksiz grup `None`.
pub fn parse_job(g: &IppAttributeGroup) -> Option<Job> {
    let id = get(g, "job-id").and_then(int).filter(|i| *i > 0)?;
    Some(Job {
        id,
        name: get_text(g, "job-name"),
        printer: get(g, "job-printer-uri").and_then(text).map(|u| last_segment(&u)).unwrap_or_default(),
        state: get(g, "job-state").and_then(int).and_then(JobState::from_enum).unwrap_or_default(),
        size_kb: get(g, "job-k-octets").and_then(int).and_then(|k| u32::try_from(k).ok()),
        created: get(g, "time-at-creation").and_then(int).map(i64::from),
    })
}

// ---- İstekler -----------------------------------------------------------------------------------

fn request(op: Operation) -> IppRequestResponse {
    // `None`: printer-uri gerekiyorsa `op_attr` ile (charset ve dil öznitelikleri önce gelir).
    IppRequestResponse::new(IppVersion::v1_1(), op, None).expect("URI'siz istek kurulamaz mı?")
}

fn op_attr(req: &mut IppRequestResponse, name: &str, value: IppValue) -> Result<(), CupsError> {
    let attr = IppAttribute::with_name(name, value).map_err(|_| CupsError::Malformed)?;
    req.attributes_mut().add(DelimiterTag::OperationAttributes, attr);
    Ok(())
}

fn keywords(list: &[&str]) -> Result<IppValue, CupsError> {
    let v: Result<Vec<_>, _> = list.iter().map(|k| IppValue::new_keyword(*k)).collect();
    Ok(IppValue::Array(v.map_err(|_| CupsError::Malformed)?))
}

fn user_name(req: &mut IppRequestResponse, user: &str) -> Result<(), CupsError> {
    let user = if user.is_empty() { "anonymous" } else { user };
    op_attr(req, "requesting-user-name", IppValue::new_name_without_language(user).map_err(|_| CupsError::Malformed)?)
}

fn uri_value(uri: &str) -> Result<IppValue, CupsError> {
    IppValue::new_uri(uri).map_err(|_| CupsError::Malformed)
}

/// İstenen yazıcı öznitelikleri.
pub const PRINTER_ATTRS: &[&str] = &[
    "printer-name",
    "printer-info",
    "printer-location",
    "printer-make-and-model",
    "printer-state",
    "printer-state-reasons",
    "printer-state-message",
    "printer-is-accepting-jobs",
    "printer-uri-supported",
    "printer-type",
];

/// İstenen iş öznitelikleri.
pub const JOB_ATTRS: &[&str] = &["job-id", "job-name", "job-printer-uri", "job-state", "job-k-octets", "time-at-creation"];

/// Yanıtın IPP durumu başarılıysa yanıt.
fn ok(resp: IppRequestResponse) -> Result<IppRequestResponse, CupsError> {
    let status = resp.header().operation_or_status;
    if (0..0x0100).contains(&status) { Ok(resp) } else { Err(CupsError::Ipp(status as u16)) }
}

/// `CUPS-Get-Printers`: tüm kuyruklar (ada göre sıralı değil; sunucu sırası).
pub fn printers(server: &Server) -> Result<Vec<Printer>, CupsError> {
    let mut req = request(Operation::CupsGetPrinters);
    op_attr(&mut req, "requested-attributes", keywords(PRINTER_ATTRS)?)?;
    let resp = match send(server, &req, &[]).and_then(ok) {
        // Hiç kuyruk yok: CUPS `client-error-not-found` döner.
        Err(CupsError::Ipp(0x0406)) => return Ok(Vec::new()),
        r => r?,
    };
    Ok(resp.attributes().groups_of(DelimiterTag::PrinterAttributes).filter_map(parse_printer).take(MAX_PRINTERS).collect())
}

/// `CUPS-Get-Default`: sunucunun varsayılan kuyruğu (yoksa `None`).
pub fn server_default(server: &Server) -> Result<Option<String>, CupsError> {
    let mut req = request(Operation::CupsGetDefault);
    op_attr(&mut req, "requested-attributes", keywords(&["printer-name"])?)?;
    let resp = match send(server, &req, &[]).and_then(ok) {
        Err(CupsError::Ipp(0x0406)) => return Ok(None),
        r => r?,
    };
    Ok(resp.attributes().groups_of(DelimiterTag::PrinterAttributes).find_map(|g| Some(get_text(g, "printer-name")).filter(|n| !n.is_empty())))
}

/// `Get-Jobs`: kullanıcının bitmemiş işleri (tüm kuyruklarda).
pub fn jobs(server: &Server, user: &str) -> Result<Vec<Job>, CupsError> {
    let mut req = request(Operation::GetJobs);
    op_attr(&mut req, "printer-uri", uri_value("ipp://localhost/")?)?;
    user_name(&mut req, user)?;
    op_attr(&mut req, "which-jobs", IppValue::new_keyword("not-completed").map_err(|_| CupsError::Malformed)?)?;
    op_attr(&mut req, "my-jobs", IppValue::Boolean(true))?;
    op_attr(&mut req, "requested-attributes", keywords(JOB_ATTRS)?)?;
    let resp = match send(server, &req, &[]).and_then(ok) {
        Err(CupsError::Ipp(0x0406)) => return Ok(Vec::new()),
        r => r?,
    };
    Ok(resp.attributes().groups_of(DelimiterTag::JobAttributes).filter_map(parse_job).filter(|j| j.state.active()).take(MAX_JOBS).collect())
}

/// `Cancel-Job` (`job-uri` ile; yalnız kendi işi — sahiplik CUPS'ta denetlenir).
pub fn cancel_job(server: &Server, user: &str, job_id: i32) -> Result<(), CupsError> {
    let mut req = request(Operation::CancelJob);
    op_attr(&mut req, "job-uri", uri_value(&format!("ipp://localhost/jobs/{job_id}"))?)?;
    user_name(&mut req, user)?;
    send(server, &req, &[]).and_then(ok).map(drop)
}

/// `Print-Job`: belgeyi kuyruğa gönderir; iş kimliğini döndürür.
pub fn print_document(server: &Server, user: &str, printer_uri: &str, title: &str, format: &str, document: &[u8]) -> Result<i32, CupsError> {
    if document.len() > MAX_DOCUMENT {
        return Err(CupsError::Document);
    }
    let mut req = request(Operation::PrintJob);
    op_attr(&mut req, "printer-uri", uri_value(printer_uri)?)?;
    user_name(&mut req, user)?;
    op_attr(&mut req, "job-name", IppValue::new_name_without_language(title).map_err(|_| CupsError::Malformed)?)?;
    op_attr(&mut req, "document-format", IppValue::MimeMediaType(format.try_into().map_err(|_| CupsError::Malformed)?))?;
    let resp = send(server, &req, document).and_then(ok)?;
    Ok(resp.attributes().groups_of(DelimiterTag::JobAttributes).find_map(|g| get(g, "job-id").and_then(int)).unwrap_or(0))
}

/// CUPS deneme sayfasını yazıcıya gönderir.
pub fn print_test_page(server: &Server, user: &str, printer_uri: &str, title: &str, page: &Path) -> Result<i32, CupsError> {
    let doc = std::fs::read(page).map_err(|_| CupsError::Document)?;
    print_document(server, user, printer_uri, title, TEST_PAGE_FORMAT, &doc)
}

// ---- HTTP --------------------------------------------------------------------------------------

/// Bağlantı (soket ya da TCP) üzerinden okuma/yazma.
trait Conn: Read + Write {}
impl<T: Read + Write> Conn for T {}

fn connect(server: &Server) -> Result<Box<dyn Conn>, CupsError> {
    match server {
        Server::Unix(path) => {
            let s = UnixStream::connect(path).map_err(|_| CupsError::Unreachable)?;
            s.set_read_timeout(Some(TIMEOUT)).map_err(|_| CupsError::Unreachable)?;
            s.set_write_timeout(Some(TIMEOUT)).map_err(|_| CupsError::Unreachable)?;
            Ok(Box::new(s))
        }
        Server::Tcp { host, port } => {
            let addrs = (host.as_str(), *port).to_socket_addrs().map_err(|_| CupsError::Unreachable)?;
            let s = addrs.into_iter().find_map(|a| TcpStream::connect_timeout(&a, TIMEOUT).ok()).ok_or(CupsError::Unreachable)?;
            s.set_read_timeout(Some(TIMEOUT)).map_err(|_| CupsError::Unreachable)?;
            s.set_write_timeout(Some(TIMEOUT)).map_err(|_| CupsError::Unreachable)?;
            Ok(Box::new(s))
        }
    }
}

/// İsteği gönderir, yanıtı ayrıştırır (IPP durumuna bakmaz; bkz. [`ok`]).
fn send(server: &Server, req: &IppRequestResponse, document: &[u8]) -> Result<IppRequestResponse, CupsError> {
    let ipp = req.to_bytes();
    let length = ipp.len().checked_add(document.len()).ok_or(CupsError::Document)?;
    let mut conn = connect(server)?;
    let head = format!(
        "POST / HTTP/1.1\r\nHost: {}\r\nContent-Type: application/ipp\r\nContent-Length: {length}\r\nUser-Agent: nuxut-settings\r\nConnection: close\r\n\r\n",
        server.host_header(),
    );
    let write = (|| {
        conn.write_all(head.as_bytes())?;
        conn.write_all(&ipp)?;
        conn.write_all(document)?;
        conn.flush()
    })();
    write.map_err(|e| io_error(&e))?;
    let deadline = Deadline { inner: conn, until: Instant::now() + TOTAL_TIMEOUT };
    let (status, body) = read_response(&mut BufReader::new(deadline))?;
    if status != 200 {
        return Err(CupsError::Http(status));
    }
    parse_ipp(body)
}

/// Okumaya toplam süre sınırı: okuma başına zaman aşımı ([`TIMEOUT`]) damla damla veri gönderen
/// sunucuya karşı yetmez (her bayt süreyi yeniler).
#[derive(Debug)]
pub struct Deadline<R> {
    pub inner: R,
    pub until: Instant,
}

impl<R: Read> Read for Deadline<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if Instant::now() >= self.until {
            return Err(std::io::ErrorKind::TimedOut.into());
        }
        self.inner.read(buf)
    }
}

/// IPP gövdesini ayrıştırır. Önce [`check_ipp`]: `ipp` ayrıştırıcısı derinlik ve öznitelik
/// sayısı sınırlamaz (iç içe koleksiyonlar özyinelemeli `Drop`'ta yığını taşırır).
pub fn parse_ipp(body: Vec<u8>) -> Result<IppRequestResponse, CupsError> {
    check_ipp(&body)?;
    IppParser::new(std::io::Cursor::new(body)).parse().map_err(|_| CupsError::Malformed)
}

/// IPP ikili yapısının sınır denetimi (saf; testli; ayrıştırmadan ve ayırmadan önce): her değerin
/// ad ve değer uzunluğu gövdenin içinde kalmalı, değer sayısı [`MAX_IPP_VALUES`]'u, koleksiyon
/// derinliği [`MAX_IPP_DEPTH`]'i aşmamalı. Gövde sonundaki belge verisi (`end-of-attributes`
/// sonrası) denetlenmez.
pub fn check_ipp(body: &[u8]) -> Result<(), CupsError> {
    // Sürüm (2) + işlem/durum (2) + istek kimliği (4).
    let mut pos: usize = 8;
    if body.len() < pos {
        return Err(CupsError::Malformed);
    }
    let mut values = 0usize;
    let mut depth = 0usize;
    let take_u16 = |pos: &mut usize| -> Result<usize, CupsError> {
        let end = pos.checked_add(2).filter(|e| *e <= body.len()).ok_or(CupsError::Malformed)?;
        let v = usize::from(u16::from_be_bytes([body[*pos], body[*pos + 1]]));
        *pos = end;
        Ok(v)
    };
    loop {
        let tag = *body.get(pos).ok_or(CupsError::Malformed)?;
        pos += 1;
        match tag {
            // Bitiş: kalan bayt belge verisidir.
            0x03 => return if depth == 0 { Ok(()) } else { Err(CupsError::Malformed) },
            0x00..=0x0f => {}
            _ => {
                values += 1;
                if values > MAX_IPP_VALUES {
                    return Err(CupsError::Malformed);
                }
                let name = take_u16(&mut pos)?;
                pos = pos.checked_add(name).filter(|e| *e <= body.len()).ok_or(CupsError::Malformed)?;
                let len = take_u16(&mut pos)?;
                pos = pos.checked_add(len).filter(|e| *e <= body.len()).ok_or(CupsError::Malformed)?;
                match tag {
                    0x34 => {
                        depth += 1;
                        if depth > MAX_IPP_DEPTH {
                            return Err(CupsError::Malformed);
                        }
                    }
                    0x37 => depth = depth.checked_sub(1).ok_or(CupsError::Malformed)?,
                    _ => {}
                }
            }
        }
    }
}

fn read_line(r: &mut impl BufRead, buf: &mut String) -> Result<(), CupsError> {
    buf.clear();
    // Satır sınırı: başlıklar kısa; uzun satır bozuk yanıttır.
    let n = r.by_ref().take(MAX_LINE as u64).read_line(buf).map_err(|e| io_error(&e))?;
    if n == 0 || !buf.ends_with('\n') {
        return Err(CupsError::Malformed);
    }
    Ok(())
}

/// Başlık bloğunu boş satıra dek okur (en çok [`MAX_HEADERS`] satır).
fn header_lines(r: &mut impl BufRead, mut each: impl FnMut(&str) -> Result<(), CupsError>) -> Result<(), CupsError> {
    let mut line = String::new();
    for _ in 0..MAX_HEADERS {
        read_line(r, &mut line)?;
        let l = line.trim_end();
        if l.is_empty() {
            return Ok(());
        }
        each(l)?;
    }
    Err(CupsError::Malformed)
}

/// HTTP/1.1 yanıtı: (durum kodu, gövde). `Content-Length` ya da `chunked`; ikisi de yoksa
/// bağlantı kapanana dek (saf; testli). Gövde [`MAX_RESPONSE`] ile sınırlı; sınır her ayırmadan
/// önce, taşmasız toplamla denetlenir. Ara (1xx) yanıtlar en çok [`MAX_INTERIM`].
pub fn read_response(r: &mut impl BufRead) -> Result<(u16, Vec<u8>), CupsError> {
    let mut line = String::new();
    let mut interim = 0;
    let status = loop {
        read_line(r, &mut line)?;
        let mut parts = line.split_whitespace();
        if !parts.next().is_some_and(|v| v.starts_with("HTTP/1.")) {
            return Err(CupsError::Malformed);
        }
        let code: u16 = parts.next().and_then(|c| c.parse().ok()).ok_or(CupsError::Malformed)?;
        if !(100..600).contains(&code) {
            return Err(CupsError::Malformed);
        }
        // 1xx (100 Continue): başlıklarını atla, asıl yanıtı bekle.
        if code < 200 {
            interim += 1;
            if interim > MAX_INTERIM {
                return Err(CupsError::Malformed);
            }
            header_lines(r, |_| Ok(()))?;
            continue;
        }
        break code;
    };
    let mut length: Option<usize> = None;
    let mut chunked = false;
    header_lines(r, |l| {
        let (k, v) = l.split_once(':').ok_or(CupsError::Malformed)?;
        let v = v.trim();
        if k.eq_ignore_ascii_case("content-length") {
            // `usize`'a sığmayan ya da sınırı aşan uzunluk: ayırmadan reddedilir.
            // Yalnız ondalık rakam (`+5`, boşluklu ya da işaretli değer reddedilir).
            if v.is_empty() || !v.bytes().all(|b| b.is_ascii_digit()) {
                return Err(CupsError::Malformed);
            }
            let n: u64 = v.parse().map_err(|_| CupsError::Malformed)?;
            let n = usize::try_from(n).ok().filter(|n| *n <= MAX_RESPONSE).ok_or(CupsError::Malformed)?;
            if length.is_some_and(|old| old != n) {
                return Err(CupsError::Malformed);
            }
            length = Some(n);
        } else if k.eq_ignore_ascii_case("transfer-encoding") && v.to_ascii_lowercase().contains("chunked") {
            chunked = true;
        }
        Ok(())
    })?;
    let body = if chunked {
        read_chunked(r)?
    } else if let Some(n) = length {
        let mut body = vec![0; n];
        r.read_exact(&mut body).map_err(|e| io_error(&e))?;
        body
    } else {
        let mut body = Vec::new();
        r.take(MAX_RESPONSE as u64 + 1).read_to_end(&mut body).map_err(|e| io_error(&e))?;
        if body.len() > MAX_RESPONSE {
            return Err(CupsError::Malformed);
        }
        body
    };
    Ok((status, body))
}

fn read_chunked(r: &mut impl BufRead) -> Result<Vec<u8>, CupsError> {
    let mut body = Vec::new();
    let mut line = String::new();
    loop {
        read_line(r, &mut line)?;
        let size = line.trim().split(';').next().unwrap_or_default().trim();
        // 16 hex hane u64'ü aşar; daha uzunu, boşu ya da hex dışı (`+1`, `0x1`) bozuk yanıt.
        if size.is_empty() || size.len() > 16 || !size.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(CupsError::Malformed);
        }
        let n = u64::from_str_radix(size, 16).map_err(|_| CupsError::Malformed)?;
        if n == 0 {
            // Son parça; varsa trailer başlıkları boş satıra dek.
            header_lines(r, |_| Ok(()))?;
            return Ok(body);
        }
        // Toplam sınır, taşmasız ve ayırmadan önce (sonsuz parça akışı da burada durur).
        let end = usize::try_from(n).ok().and_then(|n| body.len().checked_add(n)).filter(|e| *e <= MAX_RESPONSE).ok_or(CupsError::Malformed)?;
        let start = body.len();
        body.resize(end, 0);
        r.read_exact(&mut body[start..]).map_err(|e| io_error(&e))?;
        // Parça verisinden sonra yalnız CRLF.
        read_line(r, &mut line)?;
        if !line.trim().is_empty() {
            return Err(CupsError::Malformed);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_choice_follows_libcups() {
        assert_eq!(Server::choose(None, true), Server::Unix(DEFAULT_SOCKET.into()));
        assert_eq!(Server::choose(None, false), Server::Tcp { host: "localhost".into(), port: 631 });
        assert_eq!(Server::choose(Some("  "), false), Server::Tcp { host: "localhost".into(), port: 631 });
        assert_eq!(Server::choose(Some("/tmp/x.sock"), false), Server::Unix("/tmp/x.sock".into()));
        assert_eq!(Server::parse("print.example.com"), Server::Tcp { host: "print.example.com".into(), port: 631 });
        assert_eq!(Server::parse("127.0.0.1:8631"), Server::Tcp { host: "127.0.0.1".into(), port: 8631 });
        assert_eq!(Server::parse("[::1]:632"), Server::Tcp { host: "::1".into(), port: 632 });
        assert_eq!(Server::parse("::1"), Server::Tcp { host: "::1".into(), port: 631 });
        assert!(Server::parse("/run/cups/cups.sock").is_local());
        assert!(!Server::parse("print.example.com").is_local());
        assert_eq!(Server::parse("print.example.com").admin_url(), None, "uzak sunucuya web bağlantısı yok");
        assert_eq!(Server::parse("localhost:8631").admin_url().as_deref(), Some("http://localhost:8631/admin"));
        assert_eq!(Server::parse("[::1]:632").host_header(), "[::1]:632");
    }

    #[test]
    fn uris() {
        assert_eq!(last_segment("ipp://localhost/printers/Ofis_Yazıcı"), "Ofis_Yazıcı");
        assert_eq!(last_segment("ipp://localhost/printers/a%20b/"), "a b");
        assert_eq!(printer_uri("ofis-1"), "ipp://localhost/printers/ofis-1");
        assert_eq!(printer_uri("a b"), "ipp://localhost/printers/a%20b");
        assert_eq!(last_segment(&printer_uri("çalışma odası")), "çalışma odası");
    }

    fn group(tag: DelimiterTag, attrs: Vec<(&str, IppValue)>) -> IppAttributeGroup {
        let mut g = IppAttributeGroup::new(tag);
        for (n, v) in attrs {
            g.attributes_mut().push(IppAttribute::with_name(n, v).unwrap());
        }
        g
    }

    #[test]
    fn printer_attributes() {
        let g = group(
            DelimiterTag::PrinterAttributes,
            vec![
                ("printer-name", IppValue::new_name_without_language("ofis").unwrap()),
                ("printer-info", IppValue::new_text_without_language("Ofis yazıcısı").unwrap()),
                ("printer-state", IppValue::Enum(5)),
                ("printer-state-reasons", IppValue::Array(vec![IppValue::new_keyword("paused").unwrap(), IppValue::new_keyword("none").unwrap()])),
                ("printer-is-accepting-jobs", IppValue::Boolean(false)),
                ("printer-type", IppValue::Enum(0x0001)),
            ],
        );
        let p = parse_printer(&g).unwrap();
        assert_eq!(p.name, "ofis");
        assert_eq!(p.display_name(), "Ofis yazıcısı");
        assert_eq!(p.state, PrinterState::Stopped);
        assert_eq!(p.reasons, ["paused"]);
        assert!(!p.accepting && p.is_class);
        assert_eq!(p.uri, "ipp://localhost/printers/ofis", "URI verilmediyse addan");
        let bare = group(DelimiterTag::PrinterAttributes, vec![("printer-name", IppValue::new_name_without_language("x").unwrap())]);
        let p = parse_printer(&bare).unwrap();
        assert_eq!((p.display_name(), p.state, p.accepting), ("x", PrinterState::Idle, true));
        assert_eq!(parse_printer(&group(DelimiterTag::PrinterAttributes, vec![])), None);
    }

    #[test]
    fn job_attributes() {
        let g = group(
            DelimiterTag::JobAttributes,
            vec![
                ("job-id", IppValue::Integer(42)),
                ("job-name", IppValue::new_name_without_language("rapor.pdf").unwrap()),
                ("job-printer-uri", IppValue::new_uri("ipp://localhost/printers/ofis").unwrap()),
                ("job-state", IppValue::Enum(5)),
                ("job-k-octets", IppValue::Integer(120)),
            ],
        );
        let j = parse_job(&g).unwrap();
        assert_eq!((j.id, j.name.as_str(), j.printer.as_str(), j.state, j.size_kb), (42, "rapor.pdf", "ofis", JobState::Processing, Some(120)));
        assert!(j.state.active() && !JobState::Completed.active());
        assert_eq!(parse_job(&group(DelimiterTag::JobAttributes, vec![("job-id", IppValue::Integer(0))])), None);
    }

    #[test]
    fn http_bodies() {
        let mut r = std::io::Cursor::new(b"HTTP/1.1 200 OK\r\nContent-Type: application/ipp\r\nContent-Length: 3\r\n\r\nabcEXTRA".to_vec());
        assert_eq!(read_response(&mut r).unwrap(), (200, b"abc".to_vec()), "uzunluk kadar okunur");
        let mut r = std::io::Cursor::new(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n3\r\nabc\r\n2;x=y\r\nde\r\n0\r\nT: 1\r\n\r\n".to_vec());
        assert_eq!(read_response(&mut r).unwrap(), (200, b"abcde".to_vec()));
        let mut r = std::io::Cursor::new(b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.0 401 Unauthorized\r\n\r\nxy".to_vec());
        assert_eq!(read_response(&mut r).unwrap(), (401, b"xy".to_vec()), "100 atlanır; uzunluksuz gövde kapanışa dek");
        for bad in [&b"garbage\r\n\r\n"[..], b"HTTP/1.1 200 OK\r\nContent-Length: 9\r\n\r\nabc", b"HTTP/1.1 200 OK\r\nbozuk\r\n\r\n", b"HTTP/1.1 200 OK\r\n"] {
            assert_eq!(read_response(&mut std::io::Cursor::new(bad.to_vec())), Err(CupsError::Malformed), "{:?}", String::from_utf8_lossy(bad));
        }
        let huge = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", MAX_RESPONSE + 1);
        assert_eq!(read_response(&mut std::io::Cursor::new(huge.into_bytes())), Err(CupsError::Malformed));
    }

    /// Sonsuz bayt akışı: `head`, ardından `pattern` sonsuza dek.
    struct Endless {
        head: std::io::Cursor<Vec<u8>>,
        pattern: &'static [u8],
        pos: usize,
        served: usize,
    }

    impl Endless {
        fn new(head: &str, pattern: &'static [u8]) -> std::io::BufReader<Endless> {
            std::io::BufReader::new(Endless { head: std::io::Cursor::new(head.as_bytes().to_vec()), pattern, pos: 0, served: 0 })
        }
    }

    impl Read for Endless {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let n = self.head.read(buf)?;
            if n > 0 {
                return Ok(n);
            }
            // Güvenlik ağı: test asılı kalmasın (sınır 64 MiB'tan çok önce devreye girmeli).
            assert!(self.served < 64 * 1024 * 1024, "sınır devreye girmedi");
            for b in buf.iter_mut() {
                *b = self.pattern[self.pos % self.pattern.len()];
                self.pos += 1;
            }
            self.served += buf.len();
            Ok(buf.len())
        }
    }

    fn resp(s: &str) -> Result<(u16, Vec<u8>), CupsError> {
        read_response(&mut std::io::Cursor::new(s.as_bytes().to_vec()))
    }

    /// Düşmanca HTTP yanıtları (güvenlik bulgusu "resource-cap-defeat / integer overflow",
    /// 2026-10-15): taşma yok, sınır ayırmadan önce, sonsuz akışlar durur.
    #[test]
    fn hostile_http() {
        let m = Err(CupsError::Malformed);
        // Dev uzunluklar: ayırmadan reddedilir (u64::MAX, usize taşıran, sınırın bir fazlası).
        assert_eq!(resp("HTTP/1.1 200 OK\r\nContent-Length: 18446744073709551615\r\n\r\n"), m);
        assert_eq!(resp("HTTP/1.1 200 OK\r\nContent-Length: 99999999999999999999999\r\n\r\n"), m);
        assert_eq!(resp("HTTP/1.1 200 OK\r\nContent-Length: -1\r\n\r\n"), m);
        assert_eq!(resp("HTTP/1.1 200 OK\r\nContent-Length: 1\r\nContent-Length: 2\r\n\r\nab"), m, "çelişen uzunluk");
        // Dev parça: usize::MAX (eski kodda `body.len() + n` taşardı), u64 dışı, sınırın bir fazlası.
        for size in ["ffffffffffffffff", "1ffffffffffffffff", "00000000000000000001", &format!("{:x}", MAX_RESPONSE + 1)] {
            let r = resp(&format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1\r\na\r\n{size}\r\n"));
            assert_eq!(r, m, "{size}");
        }
        assert_eq!(resp("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\r\n"), m, "boş parça boyu");
        // İşaretli/önekli sayılar (`u64::from_str_radix` `+`'yı kabul eder): reddedilir.
        for size in ["+1", "0x1", "-1", "1 1"] {
            let r = resp(&format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{size}\r\na\r\n0\r\n\r\n"));
            assert_eq!(r, m, "{size}");
        }
        for len in ["+1", " ", "1 1", "0x1"] {
            assert_eq!(resp(&format!("HTTP/1.1 200 OK\r\nContent-Length: {len}\r\n\r\na")), m, "{len}");
        }
        // Toplamı sınırı az aşan iki parça: ikincisi ayırmadan reddedilir.
        let half = MAX_RESPONSE / 2;
        let mut two = format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{half:x}\r\n").into_bytes();
        two.extend(std::iter::repeat_n(b'a', half));
        two.extend_from_slice(format!("\r\n{:x}\r\n", half + 1).as_bytes());
        assert_eq!(read_response(&mut std::io::Cursor::new(two)), m, "toplam sınır + 1");
        // Çok uzun uzunluk satırı (uzantı dahil) ve 17 hane.
        let long = format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1;{}\r\na\r\n0\r\n\r\n", "x".repeat(MAX_LINE));
        assert_eq!(resp(&long), m);
        assert_eq!(resp("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n00000000000000001\r\na\r\n0\r\n\r\n"), m);
        assert_eq!(resp("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1;ext=1\r\na\r\n0\r\n\r\n"), Ok((200, b"a".to_vec())), "uzantı atılır");
        assert_eq!(resp("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nabXY\r\n0\r\n\r\n"), m, "parça sonrası CRLF yok");
        // Sonsuz akışlar: parçalar, trailer, başlıklar, 100 Continue, uzunluksuz gövde, satır.
        let chunked = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n";
        assert_eq!(read_response(&mut Endless::new(chunked, b"1\r\na\r\n")), m, "sonsuz parça akışı toplam sınırda durur");
        assert_eq!(read_response(&mut Endless::new("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n", b"X: y\r\n")), m, "sonsuz trailer");
        assert_eq!(read_response(&mut Endless::new("HTTP/1.1 200 OK\r\n", b"X: y\r\n")), m, "sonsuz başlık");
        assert_eq!(read_response(&mut Endless::new("", b"HTTP/1.1 100 Continue\r\n\r\n")), m, "sonsuz 1xx");
        assert_eq!(read_response(&mut Endless::new("HTTP/1.1 200 OK\r\n\r\n", b"abcdefgh")), m, "uzunluksuz sonsuz gövde");
        assert_eq!(read_response(&mut Endless::new("HTTP/1.1 200 OK\r\nX: ", b"a")), m, "bitmeyen satır");
        assert_eq!(resp("HTTP/1.1 999 X\r\n\r\n"), m);
    }

    #[test]
    fn deadline_cuts_slow_server() {
        let mut d = Deadline { inner: std::io::repeat(b'a'), until: Instant::now() };
        let mut buf = [0u8; 4];
        assert_eq!(d.read(&mut buf).unwrap_err().kind(), std::io::ErrorKind::TimedOut);
        let r = read_response(&mut BufReader::new(Deadline { inner: std::io::repeat(b'a'), until: Instant::now() }));
        assert_eq!(r, Err(CupsError::Timeout));
    }

    fn ipp_header() -> Vec<u8> {
        vec![1, 1, 0, 0, 0, 0, 0, 1]
    }

    /// Değer: etiket, ad uzunluğu, ad, değer uzunluğu, değer.
    fn value(out: &mut Vec<u8>, tag: u8, name: &[u8], v: &[u8]) {
        out.push(tag);
        out.extend_from_slice(&(name.len() as u16).to_be_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(&(v.len() as u16).to_be_bytes());
        out.extend_from_slice(v);
    }

    /// Düşmanca IPP gövdeleri: ayrıştırıcıya gitmeden reddedilir.
    #[test]
    fn hostile_ipp() {
        let m = Err(CupsError::Malformed);
        // Geçerli küçük gövde geçer.
        let mut ok = ipp_header();
        ok.push(0x04);
        value(&mut ok, 0x42, b"printer-name", b"a");
        ok.push(0x03);
        assert_eq!(check_ipp(&ok), Ok(()));
        assert!(parse_ipp(ok.clone()).is_ok());
        assert_eq!(check_ipp(&ok[..5]), m, "kısa başlık");
        assert_eq!(check_ipp(&ok[..ok.len() - 1]), m, "bitiş etiketi yok");
        // u16::MAX değer uzunluğu, gövde kısa.
        let mut b = ipp_header();
        b.extend_from_slice(&[0x04, 0x42, 0, 1, b'x', 0xff, 0xff, b'a']);
        assert_eq!(check_ipp(&b), m);
        assert_eq!(parse_ipp(b).err(), Some(CupsError::Malformed));
        // u16::MAX ad uzunluğu.
        let mut b = ipp_header();
        b.extend_from_slice(&[0x04, 0x42, 0xff, 0xff, b'x']);
        assert_eq!(check_ipp(&b), m);
        // Çok sayıda öznitelik: sınırın bir fazlası reddedilir.
        let mut b = ipp_header();
        b.push(0x04);
        for _ in 0..=MAX_IPP_VALUES {
            value(&mut b, 0x21, b"", &[0, 0, 0, 1]);
        }
        b.push(0x03);
        assert!(b.len() <= MAX_RESPONSE, "sınır gövde sınırının içinde sınanır");
        assert_eq!(check_ipp(&b), m);
        // İç içe koleksiyon: derinlik sınırı (yığını taşıracak 100 000 düzey ayrıştırıcıya gitmez).
        let nested = |depth: usize| {
            let mut b = ipp_header();
            b.push(0x04);
            value(&mut b, 0x34, b"c", b"");
            for _ in 1..depth {
                value(&mut b, 0x4a, b"", b"m");
                value(&mut b, 0x34, b"", b"");
            }
            for _ in 0..depth {
                value(&mut b, 0x37, b"", b"");
            }
            b.push(0x03);
            b
        };
        assert_eq!(check_ipp(&nested(MAX_IPP_DEPTH)), Ok(()));
        assert_eq!(check_ipp(&nested(MAX_IPP_DEPTH + 1)), m);
        assert_eq!(parse_ipp(nested(100_000)).err(), Some(CupsError::Malformed));
        // Kapanmayan / fazla kapanan koleksiyon.
        let mut b = ipp_header();
        b.push(0x04);
        value(&mut b, 0x37, b"", b"");
        b.push(0x03);
        assert_eq!(check_ipp(&b), m);
    }

    #[test]
    fn error_classes() {
        assert!(CupsError::Ipp(0x0401).is_denied() && CupsError::Http(401).is_denied() && !CupsError::Ipp(0x0406).is_denied());
        assert!(CupsError::Ipp(0x0406).is_gone() && CupsError::Ipp(0x0404).is_gone() && !CupsError::Unreachable.is_gone());
    }
}
