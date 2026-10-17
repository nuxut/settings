//! Sahte CUPS (IPP) sunucusu: testler (`tests/printers_ipp.rs`) ve canlı deneme
//! (`examples/fake_cups.rs`). Geçici dizindeki Unix soketinde ya da 127.0.0.1'de dinler; istekleri
//! `ipp` crate'iyle ayrıştırır, yazıcı/iş tablosu tutar. Gerçek CUPS'a hiçbir şey gitmez.
//!
//! Desteklenen işlemler: CUPS-Get-Printers, CUPS-Get-Default, Get-Jobs (`chunked` yanıtla;
//! istemcinin iki gövde biçimi de sınansın), Cancel-Job (`job-uri`), Print-Job.

#![allow(dead_code)]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use ipp::attribute::{IppAttribute, IppAttributeGroup};
use ipp::model::{DelimiterTag, IppVersion, StatusCode};
use ipp::parser::IppParser;
use ipp::request::IppRequestResponse;
use ipp::value::IppValue;

#[derive(Debug, Clone, Default)]
pub struct FakePrinter {
    pub name: String,
    pub info: String,
    pub location: String,
    pub model: String,
    /// 3 boşta, 4 yazdırıyor, 5 durdu.
    pub state: i32,
    pub message: String,
    pub accepting: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FakeJob {
    pub id: i32,
    pub name: String,
    pub printer: String,
    pub owner: String,
    /// 3 sırada, 4 bekletiliyor, 5 yazdırılıyor, 7 iptal.
    pub state: i32,
    pub kb: i32,
}

/// Gönderilen belge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printed {
    pub printer: String,
    pub title: String,
    pub format: String,
    pub user: String,
    pub bytes: usize,
}

#[derive(Debug, Default)]
pub struct FakeState {
    pub printers: Vec<FakePrinter>,
    pub default: Option<String>,
    pub jobs: Vec<FakeJob>,
    pub next_job: i32,
    /// Alınan işlem kodları (sırayla).
    pub ops: Vec<i16>,
    pub printed: Vec<Printed>,
    /// Cancel-Job'u `client-error-not-authorized` ile reddet.
    pub deny_cancel: bool,
    /// Düşmanca test: istek ne olursa olsun bu baytları HTTP yanıtı olarak gönder.
    pub raw: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct FakeCups {
    pub state: Arc<Mutex<FakeState>>,
}

/// Dinlenen adres (istemcinin `Server`'ına çevrilir).
#[derive(Debug, Clone)]
pub enum Addr {
    Unix(PathBuf),
    Tcp(u16),
}

impl FakeCups {
    pub fn new(state: FakeState) -> Self {
        Self { state: Arc::new(Mutex::new(state)) }
    }

    /// Unix soketinde arka plan thread'iyle dinler.
    pub fn serve_unix(&self, path: &Path) -> Addr {
        let l = UnixListener::bind(path).expect("sahte CUPS soketi");
        let me = self.clone();
        std::thread::spawn(move || {
            for c in l.incoming().flatten() {
                me.handle(c);
            }
        });
        Addr::Unix(path.to_owned())
    }

    /// 127.0.0.1'de rastgele portta dinler.
    pub fn serve_tcp(&self) -> Addr {
        let l = TcpListener::bind("127.0.0.1:0").expect("sahte CUPS portu");
        let port = l.local_addr().unwrap().port();
        let me = self.clone();
        std::thread::spawn(move || {
            for c in l.incoming().flatten() {
                me.handle(c);
            }
        });
        Addr::Tcp(port)
    }

    fn handle<S: Read + Write>(&self, stream: S) {
        let mut r = BufReader::new(stream);
        let mut line = String::new();
        if r.read_line(&mut line).is_err() || !line.starts_with("POST ") {
            return;
        }
        let mut length = 0usize;
        loop {
            line.clear();
            if r.read_line(&mut line).unwrap_or(0) == 0 {
                return;
            }
            let l = line.trim_end();
            if l.is_empty() {
                break;
            }
            if let Some((k, v)) = l.split_once(':')
                && k.eq_ignore_ascii_case("content-length")
            {
                length = v.trim().parse().unwrap_or(0);
            }
        }
        let mut body = vec![0; length];
        if r.read_exact(&mut body).is_err() {
            return;
        }
        let Ok(req) = IppParser::new(std::io::Cursor::new(body)).parse() else { return };
        let raw = self.state.lock().unwrap().raw.clone();
        if let Some(raw) = raw {
            let mut s = r.into_inner();
            let _ = s.write_all(&raw);
            let _ = s.flush();
            return;
        }
        let (resp, chunked) = self.respond(req);
        let bytes = resp.to_bytes();
        let mut s = r.into_inner();
        let _ = if chunked {
            let mid = bytes.len() / 2;
            let head = "HTTP/1.1 200 OK\r\nContent-Type: application/ipp\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n";
            s.write_all(head.as_bytes())
                .and_then(|_| write!(s, "{:x}\r\n", mid))
                .and_then(|_| s.write_all(&bytes[..mid]))
                .and_then(|_| write!(s, "\r\n{:x}\r\n", bytes.len() - mid))
                .and_then(|_| s.write_all(&bytes[mid..]))
                .and_then(|_| s.write_all(b"\r\n0\r\n\r\n"))
        } else {
            write!(s, "HTTP/1.1 200 OK\r\nContent-Type: application/ipp\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", bytes.len()).and_then(|_| s.write_all(&bytes))
        };
        let _ = s.flush();
    }

    fn respond(&self, mut req: IppRequestResponse) -> (IppRequestResponse, bool) {
        let header = *req.header();
        let op = header.operation_or_status;
        let attr = |name: &str| -> Option<String> {
            req.attributes().groups_of(DelimiterTag::OperationAttributes).find_map(|g| g.get(name)).map(|a| match a.value() {
                IppValue::TextWithLanguage { text, .. } => text.to_string(),
                IppValue::NameWithLanguage { name, .. } => name.to_string(),
                v => v.to_string(),
            })
        };
        let user = attr("requesting-user-name").unwrap_or_default();
        let printer_uri = attr("printer-uri").unwrap_or_default();
        let job_uri = attr("job-uri").unwrap_or_default();
        let job_name = attr("job-name").unwrap_or_default();
        let format = attr("document-format").unwrap_or_default();
        let my_jobs = attr("my-jobs").as_deref() == Some("true");
        let mut doc = Vec::new();
        let _ = req.payload_mut().read_to_end(&mut doc);

        let mut st = self.state.lock().unwrap();
        st.ops.push(op);
        let mut status = StatusCode::SuccessfulOk;
        let mut groups: Vec<IppAttributeGroup> = Vec::new();
        let mut chunked = false;
        match op {
            0x4002 => {
                if st.printers.is_empty() {
                    status = StatusCode::ClientErrorNotFound;
                }
                groups = st.printers.iter().map(printer_group).collect();
            }
            0x4001 => match st.default.clone().and_then(|d| st.printers.iter().find(|p| p.name == d).cloned()) {
                Some(p) => groups.push(printer_group(&p)),
                None => status = StatusCode::ClientErrorNotFound,
            },
            // Get-Jobs
            0x000A => {
                chunked = true;
                groups = st.jobs.iter().filter(|j| (!my_jobs || j.owner == user) && (3..=6).contains(&j.state)).map(job_group).collect();
            }
            // Cancel-Job
            0x0008 => {
                let id: i32 = job_uri.rsplit('/').next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let deny = st.deny_cancel;
                match st.jobs.iter_mut().find(|j| j.id == id) {
                    _ if deny => status = StatusCode::ClientErrorNotAuthorized,
                    Some(j) if j.owner != user => status = StatusCode::ClientErrorNotAuthorized,
                    Some(j) if j.state >= 7 => status = StatusCode::ClientErrorNotPossible,
                    Some(j) => j.state = 7,
                    None => status = StatusCode::ClientErrorNotFound,
                }
            }
            // Print-Job
            0x0002 => {
                let name = printer_uri.rsplit('/').next().unwrap_or_default().to_owned();
                if !st.printers.iter().any(|p| p.name == name) {
                    status = StatusCode::ClientErrorNotFound;
                } else {
                    st.next_job += 1;
                    let id = st.next_job;
                    st.jobs.push(FakeJob { id, name: job_name.clone(), printer: name.clone(), owner: user.clone(), state: 3, kb: i32::try_from(doc.len().div_ceil(1024)).unwrap_or(1) });
                    st.printed.push(Printed { printer: name, title: job_name, format, user, bytes: doc.len() });
                    let mut g = IppAttributeGroup::new(DelimiterTag::JobAttributes);
                    g.attributes_mut().push(IppAttribute::with_name("job-id", IppValue::Integer(id)).unwrap());
                    groups.push(g);
                }
            }
            _ => status = StatusCode::ServerErrorOperationNotSupported,
        }
        let mut resp = IppRequestResponse::new_response(IppVersion::v1_1(), status, header.request_id).unwrap();
        resp.attributes_mut().groups_mut().extend(groups);
        (resp, chunked)
    }
}

fn name(v: &str) -> IppValue {
    IppValue::new_name_without_language(v).unwrap()
}

fn text(v: &str) -> IppValue {
    IppValue::new_text_without_language(v).unwrap()
}

fn printer_group(p: &FakePrinter) -> IppAttributeGroup {
    let mut g = IppAttributeGroup::new(DelimiterTag::PrinterAttributes);
    let uri = format!("ipp://localhost/printers/{}", p.name);
    let reasons = if p.state == 5 { "paused" } else { "none" };
    for (n, v) in [
        ("printer-name", name(&p.name)),
        ("printer-info", text(&p.info)),
        ("printer-location", text(&p.location)),
        ("printer-make-and-model", text(&p.model)),
        ("printer-state", IppValue::Enum(p.state)),
        ("printer-state-reasons", IppValue::Array(vec![IppValue::new_keyword(reasons).unwrap()])),
        ("printer-state-message", text(&p.message)),
        ("printer-is-accepting-jobs", IppValue::Boolean(p.accepting)),
        ("printer-uri-supported", IppValue::Array(vec![IppValue::new_uri(uri).unwrap()])),
        ("printer-type", IppValue::Enum(0x0004)),
    ] {
        g.attributes_mut().push(IppAttribute::with_name(n, v).unwrap());
    }
    g
}

fn job_group(j: &FakeJob) -> IppAttributeGroup {
    let mut g = IppAttributeGroup::new(DelimiterTag::JobAttributes);
    for (n, v) in [
        ("job-id", IppValue::Integer(j.id)),
        ("job-name", name(&j.name)),
        ("job-printer-uri", IppValue::new_uri(format!("ipp://localhost/printers/{}", j.printer)).unwrap()),
        ("job-state", IppValue::Enum(j.state)),
        ("job-k-octets", IppValue::Integer(j.kb)),
        ("time-at-creation", IppValue::Integer(1_760_000_000)),
    ] {
        g.attributes_mut().push(IppAttribute::with_name(n, v).unwrap());
    }
    g
}

/// Canlı deneme ve testlerdeki örnek tablo: üç yazıcı (biri yazdırıyor, biri durmuş), iki iş.
pub fn demo(user: &str) -> FakeState {
    let p = |name: &str, info: &str, location: &str, model: &str, state: i32, message: &str, accepting: bool| FakePrinter {
        name: name.into(),
        info: info.into(),
        location: location.into(),
        model: model.into(),
        state,
        message: message.into(),
        accepting,
    };
    FakeState {
        printers: vec![
            p("Ofis", "Ofis yazıcısı", "2. kat, toplantı odası", "HP LaserJet Pro M404", 4, "", true),
            p("Ev_Renkli", "Ev — renkli", "Çalışma odası", "Brother MFC-L3770CDW", 3, "", true),
            p("Etiket", "", "Depo", "Zebra ZD421", 5, "Paused", false),
        ],
        default: Some("Ofis".into()),
        jobs: vec![
            FakeJob { id: 41, name: "Fatura-Ekim.pdf".into(), printer: "Ofis".into(), owner: user.into(), state: 5, kb: 184 },
            FakeJob { id: 42, name: "Sunum taslağı".into(), printer: "Ofis".into(), owner: user.into(), state: 3, kb: 2310 },
            FakeJob { id: 43, name: "başkasının işi".into(), printer: "Ofis".into(), owner: "baska".into(), state: 3, kb: 10 },
        ],
        next_job: 100,
        ..FakeState::default()
    }
}
