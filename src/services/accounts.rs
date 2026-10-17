//! Kullanıcı hesabı: `AccountsService` (`org.freedesktop.Accounts`, sistem bus'ı) istemcisi ve
//! profil resmi hattı (t118, `docs/design/account.md`).
//!
//! - Okuma: `FindUserById(uid)` → kullanıcı nesnesi; `Properties.GetAll` (RealName, UserName,
//!   IconFile, AccountType). Hesap türü salt okunur gösterilir.
//! - Yazma: `SetRealName(s)`, `SetIconFile(s)` — çağrı iletisi `ALLOW_INTERACTIVE_AUTHORIZATION`
//!   bayrağıyla gider (AccountsService'in `interactive` argümanı yok; polkit ajanı nuxut-polkit
//!   yetki sorar). Zaman aşımı [`CALL_TIMEOUT`]; bekleme sırasında [`OpState::Waiting`].
//! - Hata eşleme ([`classify_name`]): polkit reddi/iptali → sessiz (önceki değer kalır;
//!   timedate ile aynı karar); servis yok → [`AccountState::Missing`]; diğerleri genel hata.
//! - Profil resmi: framework dosya seçicisi (`nuxut::portal`, görsel filtresi) → seçilen dosya
//!   YALITILMIŞ işçide çözülür (`nuxut_image_sandbox::Sandbox`, önizleme servisiyle aynı yol;
//!   işçi yoksa yalnızca temel raster süreç içinde, ADR-0007 K2-b) → kare merkez kırpma +
//!   en çok [`AVATAR_SIDE`]² ([`square_avatar`]) → PNG, Ayarlar'ın önbellek dizinine
//!   ([`avatar_cache_path`]) → `SetIconFile(o yol)` → başarılıysa `~/.face` ([`install_face`]).
//! - Gösterilen resim: `IconFile` de kullanıcı içeriğidir; aynı yalıtımla çözülür
//!   ([`Snapshot::avatar`]).
//! - Canlı: kullanıcı nesnesinin yolundaki `Changed` ve `PropertiesChanged` sinyalleri yeniden
//!   okutur.
//! - Bus seçimi enjekte edilebilir: `NUXUT_ACCOUNTS_BUS=session` yalnızca test/örnek içindir
//!   (`examples/fake_accounts.rs`); gerçek AccountsService'e testte ASLA çağrı yapılmaz.
//!
//! Gizlilik: ad, kullanıcı adı ve yollar loglanmaz (yalnızca hata türü).

use std::collections::HashMap;
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use nuxut_image::{DecodeOptions, Fit, Image, Limits, TargetSize};
use zbus::zvariant::{OwnedObjectPath, OwnedValue};

use super::{Event, emit};

pub const DEST: &str = "org.freedesktop.Accounts";
pub const PATH: &str = "/org/freedesktop/Accounts";
pub const IFACE: &str = "org.freedesktop.Accounts";
pub const USER_IFACE: &str = "org.freedesktop.Accounts.User";
/// Bus seçimi (yalnızca test/örnek): `session` → oturum bus'ı.
pub const BUS_ENV: &str = "NUXUT_ACCOUNTS_BUS";
/// Yazma çağrılarının zaman aşımı: polkit diyaloğu kullanıcıyı bekler (timedate ile aynı).
pub const CALL_TIMEOUT: Duration = Duration::from_secs(300);
/// Üretilen profil resminin kenarı (px). AccountsService 1 MiB üstünü reddeder; 256² PNG çok altında.
pub const AVATAR_SIDE: u32 = 256;
/// Görünen adın üst sınırı (karakter). GECOS alanında pratik sınır; GNOME de benzer.
pub const MAX_NAME_CHARS: usize = 64;
/// `~/.face` Nuxut'un değilse bir kez alınan yedek.
pub const FACE_BACKUP: &str = ".face.nuxut-bak";
/// Ürettiğimiz PNG'nin işareti (`tEXt` parçası): `~/.face` bizim mi diye bakılır.
const MARK_KEY: &str = "Software";
const MARK_VALUE: &str = "nuxut-settings";
/// Okunan `IconFile` / seçilen görsel üst sınırı (yalıtılmış işçinin girdi sınırı).
const MAX_INPUT: u64 = nuxut_image_sandbox::protocol::MAX_INPUT_BYTES as u64;

/// Hesap türü (`AccountType`: 0 standart, 1 yönetici).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AccountType {
    #[default]
    Standard,
    Administrator,
    /// Tanınmayan değer.
    Unknown,
}

impl AccountType {
    pub fn from_raw(v: i32) -> Self {
        match v {
            0 => Self::Standard,
            1 => Self::Administrator,
            _ => Self::Unknown,
        }
    }
}

/// Kullanıcı özellikleri. `Debug` kişisel veriyi yazmaz.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct User {
    pub real_name: String,
    pub user_name: String,
    /// AccountsService'in kopyası (`/var/lib/AccountsService/icons/<kullanıcı>`); `None` = yok.
    pub icon_file: Option<PathBuf>,
    pub account_type: AccountType,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("real_name", &self.real_name.len())
            .field("user_name", &self.user_name.len())
            .field("icon_file", &self.icon_file.is_some())
            .field("account_type", &self.account_type)
            .finish()
    }
}

/// Servisin durumu.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AccountState {
    /// Henüz okunmadı.
    #[default]
    Pending,
    Ready(User),
    /// AccountsService (ya da bus) yok: sayfa notu, düzenleme kapalı.
    Missing,
    /// Okuma başka bir nedenle başarısız.
    Failed,
}

impl AccountState {
    pub fn user(&self) -> Option<&User> {
        match self {
            AccountState::Ready(u) => Some(u),
            _ => None,
        }
    }
}

/// Yazma işlemi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    RealName,
    Avatar,
}

/// Yazma hatasının kullanıcıya gösterilen sınıfı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// polkit reddetti ya da diyalog iptal edildi (gösterilmez; işlem `Idle`'a döner).
    NotAuthorized,
    /// AccountsService yok.
    Missing,
    /// Seçilen görsel çözülemedi / çok büyük / biçim desteklenmiyor.
    BadImage,
    /// Dosya seçici portalı yok.
    Portal,
    /// Ad geçersiz (`:`/`,`/denetim karakteri ya da çok uzun).
    InvalidName,
    /// Profil resmi değişti ama `~/.face` yazılamadı.
    Face,
    /// Diğer (ayrıntı yalnızca logda).
    Other,
}

/// Son yazma işleminin durumu.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OpState {
    #[default]
    Idle,
    /// Dosya seçici açık (profil resmi düğmesi kapalı).
    Picking,
    /// İstek gönderildi; yanıt (görsel hazırlığı, polkit diyaloğu) bekleniyor.
    Waiting(Op),
    Failed(Op, ErrorKind),
}

impl OpState {
    pub fn waiting(&self, op: Op) -> bool {
        *self == OpState::Waiting(op)
    }

    pub fn error(&self, op: Op) -> Option<ErrorKind> {
        match self {
            OpState::Failed(o, k) if *o == op => Some(*k),
            _ => None,
        }
    }

    pub fn busy(&self) -> bool {
        matches!(self, OpState::Waiting(_) | OpState::Picking)
    }
}

/// UI'nın gördüğü anlık görüntü.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub state: AccountState,
    pub op: OpState,
    /// `IconFile`'ın çözülmüş hâli (premultiplied, en çok [`AVATAR_SIDE`]²); `None` = yok ya da
    /// çözülemedi (baş harfler gösterilir). Her yeni çözüm yeni `Arc` (UI dokuyu yeniler).
    pub avatar: Option<Arc<Image>>,
}

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// D-Bus hata adını sınıflar. AccountsService polkit reddini kendi adıyla döndürür.
pub fn classify_name(name: &str) -> ErrorKind {
    use super::timedate::{ErrorKind as Td, classify_name as td};
    match name {
        "org.freedesktop.Accounts.Error.PermissionDenied" => ErrorKind::NotAuthorized,
        "org.freedesktop.Accounts.Error.UserDoesNotExist" => ErrorKind::Missing,
        _ => match td(name) {
            Td::NotAuthorized => ErrorKind::NotAuthorized,
            Td::Missing => ErrorKind::Missing,
            Td::Other => ErrorKind::Other,
        },
    }
}

/// zbus hatasını sınıflar: yöntem hatası adıyla; bağlantı kurulamadıysa servis yok sayılır.
fn classify(err: &zbus::Error) -> ErrorKind {
    use zbus::DBusError as _;
    match err {
        zbus::Error::MethodError(name, _, _) => classify_name(name.as_str()),
        zbus::Error::FDO(e) => classify_name(e.name().as_str()),
        zbus::Error::Address(_) | zbus::Error::Handshake(_) | zbus::Error::InputOutput(_) => ErrorKind::Missing,
        _ => ErrorKind::Other,
    }
}

/// Yazılacak görünen ad: kırpılmış; boşsa `None` (yazılmaz). Geçersizse `Err`.
/// `:` ve `,` GECOS alanını bozar (AccountsService `:`'yı reddeder), denetim karakteri yok.
pub fn clean_real_name(input: &str) -> Result<Option<String>, ErrorKind> {
    let s = input.trim();
    if s.is_empty() {
        return Ok(None);
    }
    if s.chars().count() > MAX_NAME_CHARS || s.chars().any(|c| c.is_control() || c == ':' || c == ',') {
        return Err(ErrorKind::InvalidName);
    }
    Ok(Some(s.to_owned()))
}

/// `GetAll` yanıtından kullanıcı (tipi uymayan alan varsayılanda kalır). `IconFile` yalnızca
/// mutlak yolsa alınır.
pub fn parse_user(map: &HashMap<String, OwnedValue>) -> User {
    let s = |k: &str| map.get(k).and_then(|v| String::try_from(v.try_clone().ok()?).ok()).unwrap_or_default();
    let icon = s("IconFile");
    let icon_file = (!icon.is_empty() && icon.starts_with('/')).then(|| PathBuf::from(icon));
    let account_type = map.get("AccountType").and_then(|v| i32::try_from(v.try_clone().ok()?).ok()).map_or(AccountType::Unknown, AccountType::from_raw);
    User { real_name: s("RealName"), user_name: s("UserName"), icon_file, account_type }
}

/// Kare merkez kırpma + uzun kenar en çok `max_side` (büyütmez). Girdi ve çıktı premultiplied.
pub fn square_avatar(img: &Image, max_side: u32) -> Result<Image, nuxut_image::ImageError> {
    let (w, h) = img.size();
    let side = w.min(h);
    let (x0, y0) = ((w - side) / 2, (h - side) / 2);
    let mut out = Vec::with_capacity(side as usize * side as usize * 4);
    let px = img.pixels();
    for y in y0..y0 + side {
        let start = ((y * w + x0) * 4) as usize;
        out.extend_from_slice(&px[start..start + side as usize * 4]);
    }
    let square = Image::from_premultiplied(side, side, out)?;
    if side <= max_side {
        return Ok(square);
    }
    nuxut_image::downscale_rgba(&square.to_straight(), side, side, side * 4, true, max_side)
}

/// Premultiplied görsel → işaretli PNG baytları (düz alfa RGBA8).
pub fn encode_png(img: &Image) -> Result<Vec<u8>, png::EncodingError> {
    let mut buf = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut buf, img.width(), img.height());
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.add_text_chunk(MARK_KEY.into(), MARK_VALUE.into())?;
        let mut w = enc.write_header()?;
        w.write_image_data(&img.to_straight())?;
        w.finish()?;
    }
    Ok(buf)
}

/// Baytlar Ayarlar'ın ürettiği PNG mi (imza + `tEXt Software=nuxut-settings` başta).
pub fn is_ours(bytes: &[u8]) -> bool {
    const SIG: &[u8] = b"\x89PNG\r\n\x1a\n";
    let mut needle = Vec::from(MARK_KEY.as_bytes());
    needle.push(0);
    needle.extend_from_slice(MARK_VALUE.as_bytes());
    let head = &bytes[..bytes.len().min(4096)];
    head.starts_with(SIG) && head.windows(needle.len()).any(|w| w == needle.as_slice())
}

/// Ayarlar'ın önbellek dizinindeki profil resmi (`$XDG_CACHE_HOME/nuxut-settings/avatar.png`).
pub fn avatar_cache_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).filter(|p| p.is_absolute()).or_else(|| Some(PathBuf::from(std::env::var_os("HOME")?).join(".cache")))?;
    Some(base.join("nuxut-settings").join("avatar.png"))
}

/// `dir/name`'i atomik yazar: aynı dizinde geçici dosya + `fsync` + `rename`. İzin 0644
/// (`~/.face`'i giriş/kilit ekranı okur).
pub fn write_atomic(dir: &Path, name: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
    use std::os::unix::fs::OpenOptionsExt as _;
    let target = dir.join(name);
    let tmp = dir.join(format!(".{name}.nuxut-tmp-{}", std::process::id()));
    let result = (|| {
        let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o644).open(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        std::fs::rename(&tmp, &target)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result.map(|()| target)
}

/// `home/.face`'i `png` ile değiştirir. Var olan `.face` Nuxut'un değilse (ya da bağlantı/başka
/// tür dosyaysa) önce bir kez `home/.face.nuxut-bak`'a taşınır; yedek zaten varsa dokunulmaz.
/// Dizinse yazılmaz.
pub fn install_face(home: &Path, png: &[u8]) -> std::io::Result<()> {
    let face = home.join(".face");
    match std::fs::symlink_metadata(&face) {
        Ok(m) if m.is_dir() => return Err(std::io::Error::other(".face bir dizin")),
        Ok(m) => {
            let ours = m.is_file() && read_head(&face).is_ok_and(|h| is_ours(&h));
            let backup = home.join(FACE_BACKUP);
            if !ours && std::fs::symlink_metadata(&backup).is_err() {
                std::fs::rename(&face, &backup)?;
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    write_atomic(home, ".face", png).map(drop)
}

fn read_head(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(path)?.take(4096).read_to_end(&mut buf)?;
    Ok(buf)
}

/// Kullanıcı içeriği dosyayı en çok [`MAX_INPUT`] bayt okur; normal dosya değilse (FIFO,
/// aygıt) okumaz.
fn read_limited(path: &Path) -> std::io::Result<Vec<u8>> {
    let m = std::fs::metadata(path)?;
    if !m.is_file() {
        return Err(std::io::Error::other("normal dosya değil"));
    }
    if m.len() > MAX_INPUT {
        return Err(std::io::Error::other("çok büyük"));
    }
    let mut buf = Vec::with_capacity(m.len() as usize);
    std::fs::File::open(path)?.take(MAX_INPUT + 1).read_to_end(&mut buf)?;
    if buf.len() as u64 > MAX_INPUT {
        return Err(std::io::Error::other("çok büyük"));
    }
    Ok(buf)
}

/// Bu sürecin kullanıcı kimliği (`/proc/self` dizininin sahibi = etkin uid; `unsafe` yok).
fn current_uid() -> Option<u32> {
    use std::os::unix::fs::MetadataExt as _;
    std::fs::metadata("/proc/self").ok().map(|m| m.uid())
}

// ---- Görsel çözümü (yalıtılmış) ----------------------------------------------------------------

/// Süreç geneli yalıtılmış çözücü (ilk görselde kurulur; işçi tek kullanımlık, boşta süreç yok).
fn sandbox() -> &'static nuxut_image_sandbox::Sandbox {
    static SANDBOX: OnceLock<nuxut_image_sandbox::Sandbox> = OnceLock::new();
    SANDBOX.get_or_init(|| {
        nuxut_image_sandbox::Sandbox::new(nuxut_image_sandbox::SandboxConfig {
            // Telefon fotoğrafı (4000×3000 JPEG) yüklü makinede 1,5 sn'yi aşabilir.
            timeout: Duration::from_secs(6),
            prewarm: 0,
            ..nuxut_image_sandbox::SandboxConfig::default()
        })
    })
}

/// Dosyayı yalıtılmış işçide çözer ve kare profil resmine çevirir.
pub fn decode_avatar(path: &Path) -> Result<Image, ErrorKind> {
    let bytes = read_limited(path).map_err(|e| {
        tracing::info!("hesap: görsel okunamadı: {}", e.kind());
        ErrorKind::BadImage
    })?;
    let opts = DecodeOptions { limits: Limits::DEFAULT, target: Some(TargetSize::new(AVATAR_SIDE, AVATAR_SIDE, Fit::Cover)), animate: false };
    // Anahtar dosya başına: bozuk bir dosya yalnızca kendini susturur (art arda 3 hata → 60 sn).
    let key = nuxut_image::derive_key(&[b"settings-avatar", path.as_os_str().as_encoded_bytes()]);
    let img = sandbox().decode(key, Arc::from(bytes), &opts).map_err(|e| {
        tracing::info!("hesap: görsel çözülemedi: {e}");
        ErrorKind::BadImage
    })?;
    square_avatar(&img, AVATAR_SIDE).map_err(|_| ErrorKind::BadImage)
}

// ---- Süreç geneli durum ve thread'ler ----------------------------------------------------------

#[derive(Debug)]
enum Command {
    Refresh,
    SetRealName(String),
    /// Seçilen görsel (portal yanıtı).
    SetAvatar(PathBuf),
}

static SNAPSHOT: Mutex<Option<Snapshot>> = Mutex::new(None);
static SENDER: OnceLock<Mutex<Sender<Command>>> = OnceLock::new();
static STARTED: AtomicBool = AtomicBool::new(false);

/// Son anlık görüntü.
pub fn snapshot() -> Snapshot {
    SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default()
}

fn update(f: impl FnOnce(&mut Snapshot)) {
    {
        let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        f(s.get_or_insert_with(Snapshot::default));
    }
    emit(Event::Account);
}

fn send(cmd: Command) {
    // Birim testleri sistem bus'ına gitmez (ve thread açmaz).
    if cfg!(test) {
        return;
    }
    let sender = SENDER.get_or_init(|| {
        let (tx, rx) = channel();
        let _ = std::thread::Builder::new().name("accounts".into()).spawn(move || worker(rx));
        Mutex::new(tx)
    });
    STARTED.store(true, Ordering::Relaxed);
    let _ = sender.lock().unwrap_or_else(|e| e.into_inner()).send(cmd);
}

/// Sayfa ilk kurulurken: servisi başlatır ve bir kez okur.
pub fn ensure() {
    if !STARTED.load(Ordering::Relaxed) {
        send(Command::Refresh);
    }
}

/// Yeniden oku (sayfa açıldı, pencere odağı döndü) — yalnızca daha önce başlatıldıysa.
pub fn recheck() {
    if STARTED.load(Ordering::Relaxed) {
        send(Command::Refresh);
    }
}

/// Görünen adı yazar. Boş ya da değişmemiş ad yazılmaz; geçersiz ad hata durumu olur.
pub fn set_real_name(input: &str) {
    let s = snapshot();
    let Some(user) = s.state.user() else { return };
    if s.op.busy() {
        return;
    }
    match clean_real_name(input) {
        Ok(Some(name)) if name != user.real_name => begin(OpState::Waiting(Op::RealName), Command::SetRealName(name)),
        Ok(_) => {}
        Err(kind) => {
            if !cfg!(test) {
                update(|s| s.op = OpState::Failed(Op::RealName, kind));
            }
        }
    }
}

/// Profil resmi seçtirir (framework dosya seçicisi); seçim gelince hat [`Command::SetAvatar`].
pub fn pick_avatar() {
    let s = snapshot();
    if s.state.user().is_none() || s.op.busy() || cfg!(test) {
        return;
    }
    update(|s| s.op = OpState::Picking);
    let filter = nuxut::portal::Filter::new(nuxut::i18n::t!("settings-user-avatar-filter")).mime("image/png").mime("image/jpeg").mime("image/webp").mime("image/gif");
    let mut options = nuxut::portal::OpenFileOptions::new(nuxut::i18n::t!("settings-user-avatar-dialog-title"));
    options.filters = vec![filter];
    options.current_folder = pictures_dir();
    portal().open_file(&nuxut::portal::ParentWindow::none(), options);
}

/// Avatar için seçilen dosyayı doğrudan işler (canlı test senaryosu; dosya seçici olmadan).
pub fn set_avatar_file(path: PathBuf) {
    let s = snapshot();
    if s.state.user().is_none() || s.op.busy() {
        return;
    }
    begin(OpState::Waiting(Op::Avatar), Command::SetAvatar(path));
}

fn pictures_dir() -> Option<PathBuf> {
    let home = PathBuf::from(std::env::var_os("HOME")?);
    ["Resimler", "Pictures"].into_iter().map(|d| home.join(d)).find(|p| p.is_dir())
}

/// Süreç geneli portal istemcisi; yanıt doğrudan hesap thread'ine gider.
fn portal() -> &'static nuxut::portal::Portal {
    static PORTAL: OnceLock<nuxut::portal::Portal> = OnceLock::new();
    PORTAL.get_or_init(|| {
        nuxut::portal::Portal::new(|response| match response.result {
            Ok(paths) => match paths.into_iter().next() {
                Some(p) => {
                    update(|s| s.op = OpState::Waiting(Op::Avatar));
                    send(Command::SetAvatar(p));
                }
                None => update(|s| s.op = OpState::Idle),
            },
            Err(nuxut::portal::PortalError::Cancelled) => update(|s| s.op = OpState::Idle),
            Err(e) => {
                tracing::warn!("hesap: dosya seçici: {e}");
                update(|s| s.op = OpState::Failed(Op::Avatar, ErrorKind::Portal));
            }
        })
    })
}

fn begin(op: OpState, cmd: Command) {
    if cfg!(test) {
        return;
    }
    update(|s| s.op = op);
    send(cmd);
}

fn session_bus() -> bool {
    std::env::var(BUS_ENV).is_ok_and(|v| v == "session")
}

fn connect() -> zbus::Result<zbus::blocking::Connection> {
    let builder = if session_bus() { zbus::blocking::connection::Builder::session()? } else { zbus::blocking::connection::Builder::system()? };
    builder.method_timeout(CALL_TIMEOUT).build()
}

/// Kullanıcı nesnesinin yolu: uid ile, olmazsa `$USER` adıyla.
fn find_user(conn: &zbus::blocking::Connection) -> zbus::Result<OwnedObjectPath> {
    let by_id = current_uid().map(|uid| conn.call_method(Some(DEST), PATH, Some(IFACE), "FindUserById", &(i64::from(uid),)).and_then(|m| m.body().deserialize::<OwnedObjectPath>()));
    match by_id {
        Some(Ok(p)) => Ok(p),
        other => {
            let name = std::env::var("USER").unwrap_or_default();
            if name.is_empty() {
                return other.unwrap_or_else(|| Err(zbus::Error::Failure("uid yok".into())));
            }
            conn.call_method(Some(DEST), PATH, Some(IFACE), "FindUserByName", &(name.as_str(),))?.body().deserialize::<OwnedObjectPath>()
        }
    }
}

fn read_user(conn: &zbus::blocking::Connection, path: &OwnedObjectPath) -> zbus::Result<User> {
    let reply = conn.call_method(Some(DEST), path.as_str(), Some("org.freedesktop.DBus.Properties"), "GetAll", &(USER_IFACE,))?;
    let map: HashMap<String, OwnedValue> = reply.body().deserialize()?;
    Ok(parse_user(&map))
}

/// Yetki sorulabilen yazma çağrısı (`ALLOW_INTERACTIVE_AUTHORIZATION`).
fn call_interactive(conn: &zbus::blocking::Connection, path: &OwnedObjectPath, method: &str, arg: &str) -> zbus::Result<()> {
    let proxy = zbus::blocking::proxy::Builder::<zbus::blocking::Proxy<'_>>::new(conn)
        .destination(DEST)?
        .path(path.as_str())?
        .interface(USER_IFACE)?
        .cache_properties(zbus::proxy::CacheProperties::No)
        .build()?;
    proxy.call_with_flags::<_, _, ()>(method, zbus::proxy::MethodFlags::AllowInteractiveAuth.into(), &(arg,))?;
    Ok(())
}

#[derive(Debug, Default)]
struct Worker {
    conn: Option<zbus::blocking::Connection>,
    user: Option<OwnedObjectPath>,
    /// Gösterilen resmi çözülen `IconFile` (ve değiştirilme zamanı): değişmediyse yeniden çözülmez.
    icon_seen: Option<(PathBuf, Option<std::time::SystemTime>)>,
    listening: bool,
}

fn worker(rx: Receiver<Command>) {
    let mut w = Worker::default();
    while let Ok(cmd) = rx.recv() {
        if w.conn.is_none() {
            match connect() {
                Ok(c) => w.conn = Some(c),
                Err(e) => {
                    tracing::info!("hesap: bus'a bağlanılamadı: {e}");
                    let missing = classify(&e) == ErrorKind::Missing;
                    finish_without_bus(&cmd, missing);
                    continue;
                }
            }
        }
        match cmd {
            Command::Refresh => w.refresh(),
            Command::SetRealName(name) => w.write(Op::RealName, "SetRealName", &name, None),
            Command::SetAvatar(path) => w.set_avatar(&path),
        }
    }
}

fn finish_without_bus(cmd: &Command, missing: bool) {
    let kind = if missing { ErrorKind::Missing } else { ErrorKind::Other };
    match cmd {
        Command::Refresh => update(|s| s.state = if missing { AccountState::Missing } else { AccountState::Failed }),
        Command::SetRealName(_) => update(|s| s.op = OpState::Failed(Op::RealName, kind)),
        Command::SetAvatar(_) => update(|s| s.op = OpState::Failed(Op::Avatar, kind)),
    }
}

impl Worker {
    fn user_path(&mut self) -> zbus::Result<OwnedObjectPath> {
        if let Some(p) = &self.user {
            return Ok(p.clone());
        }
        let conn = self.conn.as_ref().ok_or_else(|| zbus::Error::Failure("bağlantı yok".into()))?;
        let p = find_user(conn)?;
        self.user = Some(p.clone());
        Ok(p)
    }

    fn refresh(&mut self) {
        let result = self.user_path().and_then(|p| read_user(self.conn.as_ref().expect("bağlı"), &p).map(|u| (p, u)));
        match result {
            Ok((path, user)) => {
                if !self.listening {
                    self.listening = true;
                    let _ = std::thread::Builder::new().name("accounts-signals".into()).spawn(move || listen(&path));
                }
                let avatar = self.avatar_for(user.icon_file.as_deref());
                update(|s| {
                    s.state = AccountState::Ready(user);
                    if let Some(a) = avatar {
                        s.avatar = a;
                    }
                });
            }
            Err(e) => {
                let kind = classify(&e);
                tracing::info!("hesap: okunamadı ({kind:?})");
                // Servis yeniden başlarsa nesne yolu değişmez ama bir sonraki denemede yeniden bulunur.
                self.user = None;
                update(|s| s.state = if kind == ErrorKind::Missing { AccountState::Missing } else { AccountState::Failed });
            }
        }
    }

    /// `IconFile` değiştiyse çözer: `Some(yeni)`; değişmediyse `None` (eskisi kalır).
    fn avatar_for(&mut self, icon: Option<&Path>) -> Option<Option<Arc<Image>>> {
        let stamp = icon.map(|p| (p.to_path_buf(), std::fs::metadata(p).and_then(|m| m.modified()).ok()));
        if stamp == self.icon_seen && stamp.is_some() {
            return None;
        }
        self.icon_seen.clone_from(&stamp);
        Some(icon.and_then(|p| decode_avatar(p).ok()).map(Arc::new))
    }

    fn write(&mut self, op: Op, method: &str, arg: &str, face: Option<&[u8]>) {
        let result = self.user_path().and_then(|p| call_interactive(self.conn.as_ref().expect("bağlı"), &p, method, arg));
        match result {
            Ok(()) => {
                let face_ok = face.is_none_or(|png| {
                    let home = std::env::var_os("HOME").map(PathBuf::from).filter(|h| h.is_absolute());
                    match home.map(|h| install_face(&h, png)) {
                        Some(Ok(())) => true,
                        Some(Err(e)) => {
                            tracing::warn!("hesap: ~/.face yazılamadı: {}", e.kind());
                            false
                        }
                        None => false,
                    }
                });
                // Yeni resim aynı yola (AccountsService kopyası) yazılmış olabilir: yeniden çöz.
                if face.is_some() {
                    self.icon_seen = None;
                }
                update(|s| s.op = if face_ok { OpState::Idle } else { OpState::Failed(op, ErrorKind::Face) });
            }
            Err(e) => {
                let kind = classify(&e);
                // Ayrıntı kullanıcıya gösterilmez.
                tracing::warn!("hesap: {method} başarısız ({kind:?})");
                update(|s| {
                    // polkit iptali ve reddi ayırt edilemez: sessiz (timedate ile aynı karar).
                    s.op = if kind == ErrorKind::NotAuthorized { OpState::Idle } else { OpState::Failed(op, kind) };
                    if kind == ErrorKind::Missing {
                        s.state = AccountState::Missing;
                    }
                });
                if kind == ErrorKind::Missing {
                    self.user = None;
                }
            }
        }
        self.refresh();
    }

    fn set_avatar(&mut self, path: &Path) {
        let prepared = decode_avatar(path).and_then(|img| encode_png(&img).map_err(|_| ErrorKind::Other));
        let png = match prepared {
            Ok(p) => p,
            Err(kind) => {
                update(|s| s.op = OpState::Failed(Op::Avatar, kind));
                return;
            }
        };
        let written = avatar_cache_path().ok_or_else(|| std::io::Error::other("önbellek dizini yok")).and_then(|p| {
            let dir = p.parent().ok_or_else(|| std::io::Error::other("üst dizin yok"))?;
            std::fs::create_dir_all(dir)?;
            write_atomic(dir, "avatar.png", &png)
        });
        match written {
            Ok(file) => {
                let file = file.to_string_lossy().into_owned();
                self.write(Op::Avatar, "SetIconFile", &file, Some(&png));
            }
            Err(e) => {
                tracing::warn!("hesap: profil resmi önbelleğe yazılamadı: {}", e.kind());
                update(|s| s.op = OpState::Failed(Op::Avatar, ErrorKind::Other));
            }
        }
    }
}

/// Kullanıcı nesnesinin `Changed` (AccountsService) ve `PropertiesChanged` sinyalleri yeniden
/// okutur. Eşleşme gönderen adına değil yola bakar (servis yeniden başlasa da çalışır).
fn listen(path: &OwnedObjectPath) {
    let run = || -> zbus::Result<()> {
        let conn = if session_bus() { zbus::blocking::Connection::session()? } else { zbus::blocking::Connection::system()? };
        let rule = zbus::MatchRule::builder().msg_type(zbus::message::Type::Signal).path(path.as_str())?.build();
        let it = zbus::blocking::MessageIterator::for_match_rule(rule, &conn, Some(16))?;
        for msg in it {
            let Ok(msg) = msg else { continue };
            let header = msg.header();
            let member = header.member().map(|m| m.as_str().to_owned()).unwrap_or_default();
            let iface = header.interface().map(|i| i.as_str().to_owned()).unwrap_or_default();
            let relevant = (iface == USER_IFACE && member == "Changed") || (iface == "org.freedesktop.DBus.Properties" && member == "PropertiesChanged");
            if relevant && let Some(s) = SENDER.get() {
                let _ = s.lock().unwrap_or_else(|e| e.into_inner()).send(Command::Refresh);
            }
        }
        Ok(())
    };
    if let Err(e) = run() {
        tracing::info!("hesap: sinyal dinlenemiyor: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::Value;

    fn gradient(w: u32, h: u32) -> Image {
        let mut px = Vec::new();
        for y in 0..h {
            for x in 0..w {
                px.extend_from_slice(&[(x % 256) as u8, (y % 256) as u8, 0, 255]);
            }
        }
        Image::from_premultiplied(w, h, px).unwrap()
    }

    #[test]
    fn error_names_map_to_user_classes() {
        assert_eq!(classify_name("org.freedesktop.Accounts.Error.PermissionDenied"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.PolicyKit1.Error.Cancelled"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.AccessDenied"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.InteractiveAuthorizationRequired"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.ServiceUnknown"), ErrorKind::Missing);
        assert_eq!(classify_name("org.freedesktop.Accounts.Error.UserDoesNotExist"), ErrorKind::Missing);
        assert_eq!(classify_name("org.freedesktop.Accounts.Error.Failed"), ErrorKind::Other);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.InvalidArgs"), ErrorKind::Other);
    }

    #[test]
    fn connection_failure_is_missing() {
        let e = zbus::Error::InputOutput(Arc::new(std::io::Error::from(std::io::ErrorKind::NotFound)));
        assert_eq!(classify(&e), ErrorKind::Missing);
        assert_eq!(classify(&zbus::Error::Failure("x".into())), ErrorKind::Other);
    }

    #[test]
    fn real_name_is_validated() {
        assert_eq!(clean_real_name("  Ayşe Yılmaz "), Ok(Some("Ayşe Yılmaz".into())));
        assert_eq!(clean_real_name("   "), Ok(None));
        assert_eq!(clean_real_name(""), Ok(None));
        for bad in ["a:b", "Soyad, Ad", "a\nb", "a\tb"] {
            assert_eq!(clean_real_name(bad), Err(ErrorKind::InvalidName), "{bad:?}");
        }
        assert!(clean_real_name(&"ş".repeat(MAX_NAME_CHARS)).is_ok(), "sınır karakter, bayt değil");
        assert_eq!(clean_real_name(&"a".repeat(MAX_NAME_CHARS + 1)), Err(ErrorKind::InvalidName));
    }

    #[test]
    fn user_parses_with_wrong_types_ignored() {
        let ov = |v: Value| OwnedValue::try_from(v).unwrap();
        let mut m: HashMap<String, OwnedValue> = HashMap::new();
        m.insert("RealName".into(), ov(Value::from("Hakan")));
        m.insert("UserName".into(), ov(Value::from("hakan")));
        m.insert("IconFile".into(), ov(Value::from("/var/lib/AccountsService/icons/hakan")));
        m.insert("AccountType".into(), ov(Value::from(1i32)));
        let u = parse_user(&m);
        assert_eq!(u.real_name, "Hakan");
        assert_eq!(u.user_name, "hakan");
        assert_eq!(u.icon_file.as_deref(), Some(Path::new("/var/lib/AccountsService/icons/hakan")));
        assert_eq!(u.account_type, AccountType::Administrator);
        m.insert("IconFile".into(), ov(Value::from("göreli/yol")));
        m.insert("AccountType".into(), ov(Value::from("yönetici")));
        let u = parse_user(&m);
        assert_eq!(u.icon_file, None, "göreli yol alınmaz");
        assert_eq!(u.account_type, AccountType::Unknown);
        m.insert("IconFile".into(), ov(Value::from("")));
        assert_eq!(parse_user(&m).icon_file, None);
        assert!(!format!("{u:?}").contains("Hakan"), "Debug kişisel veri yazmaz");
    }

    #[test]
    fn avatar_is_square_center_crop_and_capped() {
        // Yatay: 300×200 → orta 200×200; sol kenar x=50.
        let a = square_avatar(&gradient(300, 200), AVATAR_SIDE).unwrap();
        assert_eq!(a.size(), (200, 200), "küçük görsel büyütülmez");
        assert_eq!(a.straight_pixel(0, 0)[0], 50);
        // Dikey: 100×180 → 100×100, üst y=40.
        let b = square_avatar(&gradient(100, 180), AVATAR_SIDE).unwrap();
        assert_eq!(b.size(), (100, 100));
        assert_eq!(b.straight_pixel(0, 0)[1], 40);
        // Büyük: 600×400 → 400² → 256².
        let c = square_avatar(&gradient(600, 400), AVATAR_SIDE).unwrap();
        assert_eq!(c.size(), (AVATAR_SIDE, AVATAR_SIDE));
        let d = square_avatar(&gradient(1, 1), AVATAR_SIDE).unwrap();
        assert_eq!(d.size(), (1, 1));
    }

    #[test]
    fn png_is_marked_and_round_trips() {
        let img = square_avatar(&gradient(64, 64), AVATAR_SIDE).unwrap();
        let png = encode_png(&img).unwrap();
        assert!(is_ours(&png));
        let dec = png::Decoder::new(std::io::Cursor::new(&png)).read_info().unwrap();
        assert_eq!((dec.info().width, dec.info().height), (64, 64));
        assert!(!is_ours(b"\x89PNG\r\n\x1a\nbaska"), "işaretsiz PNG bizim değil");
        assert!(!is_ours(b"Software\0nuxut-settings"), "PNG imzası şart");
    }

    #[test]
    fn face_backup_once_and_atomic_replace() {
        let home = tempfile::tempdir().unwrap();
        let h = home.path();
        let ours = encode_png(&gradient(4, 4)).unwrap();
        // Yokken: yalnızca yazılır, yedek yok.
        install_face(h, &ours).unwrap();
        assert_eq!(std::fs::read(h.join(".face")).unwrap(), ours);
        assert!(!h.join(FACE_BACKUP).exists());
        // Bizimkinin üstüne: yedek alınmaz.
        let ours2 = encode_png(&gradient(8, 8)).unwrap();
        install_face(h, &ours2).unwrap();
        assert_eq!(std::fs::read(h.join(".face")).unwrap(), ours2);
        assert!(!h.join(FACE_BACKUP).exists());
        // Kullanıcının kendi .face'i: bir kez yedeklenir.
        std::fs::write(h.join(".face"), b"kullanicinin resmi").unwrap();
        install_face(h, &ours).unwrap();
        assert_eq!(std::fs::read(h.join(FACE_BACKUP)).unwrap(), b"kullanicinin resmi");
        // İkinci yabancı .face: yedeğin üstüne yazılmaz.
        std::fs::write(h.join(".face"), b"ikinci").unwrap();
        install_face(h, &ours).unwrap();
        assert_eq!(std::fs::read(h.join(FACE_BACKUP)).unwrap(), b"kullanicinin resmi", "yedek bir kez");
        assert_eq!(std::fs::read(h.join(".face")).unwrap(), ours);
        // Geçici dosya kalmaz.
        let leftovers: Vec<_> = std::fs::read_dir(h).unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| n.contains("nuxut-tmp")).collect();
        assert!(leftovers.is_empty(), "{leftovers:?}");
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(std::fs::metadata(h.join(".face")).unwrap().permissions().mode() & 0o777, 0o644);
    }

    #[test]
    fn face_symlink_is_moved_not_followed() {
        let home = tempfile::tempdir().unwrap();
        let h = home.path();
        let target = h.join("hedef.png");
        std::fs::write(&target, b"hedef").unwrap();
        std::os::unix::fs::symlink(&target, h.join(".face")).unwrap();
        let ours = encode_png(&gradient(4, 4)).unwrap();
        install_face(h, &ours).unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"hedef", "bağlantının hedefi değişmez");
        assert!(std::fs::symlink_metadata(h.join(FACE_BACKUP)).unwrap().file_type().is_symlink());
        assert_eq!(std::fs::read(h.join(".face")).unwrap(), ours);
        // Dizin: yazılmaz.
        let home2 = tempfile::tempdir().unwrap();
        std::fs::create_dir(home2.path().join(".face")).unwrap();
        assert!(install_face(home2.path(), &ours).is_err());
    }

    #[test]
    fn decode_rejects_non_images_and_special_files() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("x.png");
        std::fs::write(&p, b"resim degil").unwrap();
        assert_eq!(decode_avatar(&p).unwrap_err(), ErrorKind::BadImage);
        assert_eq!(decode_avatar(dir.path()).unwrap_err(), ErrorKind::BadImage, "dizin okunmaz");
        assert_eq!(decode_avatar(&dir.path().join("yok.png")).unwrap_err(), ErrorKind::BadImage);
    }

    #[test]
    fn decode_produces_square_avatar() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("foto.png");
        std::fs::write(&p, encode_png(&gradient(400, 300)).unwrap()).unwrap();
        let img = decode_avatar(&p).unwrap();
        assert_eq!(img.size(), (AVATAR_SIDE, AVATAR_SIDE));
    }

    #[test]
    fn cache_path_follows_xdg() {
        let p = avatar_cache_path().unwrap();
        assert!(p.ends_with("nuxut-settings/avatar.png"), "{}", p.display());
    }

    #[test]
    fn op_state_helpers() {
        let w = OpState::Waiting(Op::Avatar);
        assert!(w.waiting(Op::Avatar) && !w.waiting(Op::RealName) && w.busy());
        assert!(OpState::Picking.busy());
        let f = OpState::Failed(Op::RealName, ErrorKind::InvalidName);
        assert_eq!(f.error(Op::RealName), Some(ErrorKind::InvalidName));
        assert_eq!(f.error(Op::Avatar), None);
        assert!(!f.busy());
    }
}
