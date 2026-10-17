//! Yazıcılar (Sistem › Yazıcılar): CUPS durumu ve eylemleri. İstemci [`cups`] (IPP, UI'sız),
//! kullanıcı varsayılanı [`lpoptions`]; bu modül onları Ayarlar'ın olay döngüsüne bağlar.
//! Kararlar: `docs/design/printers.md`.
//!
//! - **Yoklama** yalnız sayfa görünürken ([`set_page_visible`]): açılınca hemen, sonra bekleyen
//!   iş varken [`POLL_BUSY`], yokken [`POLL_IDLE`] arayla; odak dönünce [`recheck`]. Görünmezken
//!   thread komut bekler, CUPS'a istek gitmez.
//! - **Eylemler** (varsayılan yap, deneme sayfası, iş iptali) aynı thread'de sırayla; aynı anda
//!   tek eylem ([`Snapshot::op`]). Sonuç iyimser değil: durum ancak yeniden okumayla değişir;
//!   hata [`Snapshot::failure`]'da (sonraki eylem temizler).
//! - **Gizlilik:** yazıcı/iş adı ve kullanıcı adı loglanmaz; yalnız işlem adı ve hata türü.

pub mod cups;
pub mod lpoptions;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

pub use cups::{CupsError, Job, JobState, Printer, PrinterState, Server};

use super::{Event, emit};

/// Bekleyen iş varken yoklama aralığı.
pub const POLL_BUSY: Duration = Duration::from_secs(2);
/// Bekleyen iş yokken yoklama aralığı.
pub const POLL_IDLE: Duration = Duration::from_secs(5);

/// Servis yapılandırması (üründe [`Config::from_env`]; testlerde açıkça).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server: Server,
    /// `requesting-user-name` (iş sahipliği).
    pub user: String,
    /// Kullanıcı varsayılanının dosyası (`None`: HOME yok; varsayılan yalnız sunucudan, yazılamaz).
    pub lpoptions: Option<PathBuf>,
    /// Deneme sayfası belgesi.
    pub test_page: PathBuf,
}

impl Config {
    pub fn from_env() -> Config {
        let user = std::env::var("USER").or_else(|_| std::env::var("LOGNAME")).unwrap_or_default();
        Config { server: Server::from_env(), user, lpoptions: lpoptions::path(), test_page: cups::TEST_PAGE.into() }
    }
}

/// Sayfanın ana durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    /// Henüz okunmadı.
    #[default]
    Reading,
    /// CUPS'a bağlanılamadı (kurulu değil / çalışmıyor).
    Unavailable,
    /// Bağlandı ama okunamadı (HTTP/IPP hatası, bozuk yanıt, zaman aşımı).
    Failed,
    Ready,
}

/// Okunan durum.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    /// Görünen ada göre sıralı.
    pub printers: Vec<Printer>,
    /// Etkin varsayılan kuyruk ([`effective_default`]).
    pub default: Option<String>,
    /// Kullanıcının bitmemiş işleri (kimliğe göre, eski önce).
    pub jobs: Vec<Job>,
}

/// Kullanıcı eylemi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    SetDefault(String),
    /// Deneme sayfası: (kuyruk adı, iş başlığı).
    TestPage(String, String),
    Cancel(i32),
}

impl Op {
    fn name(&self) -> &'static str {
        match self {
            Op::SetDefault(_) => "set-default",
            Op::TestPage(..) => "test-page",
            Op::Cancel(_) => "cancel-job",
        }
    }
}

/// Eylem hatası.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpError {
    Cups(CupsError),
    /// `~/.cups/lpoptions` yazılamadı (ya da HOME yok).
    Write,
    /// Kuyruk artık yok.
    NoPrinter,
}

/// Başarısız eylem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub op: Op,
    pub error: OpError,
}

/// UI'nın gördüğü anlık görüntü.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub status: Status,
    pub state: State,
    /// Deneme sayfası belgesi var mı (yoksa düğme gizli).
    pub test_page: bool,
    /// "Yazıcı ekle…" bağlantısı (yalnız yerel sunucu).
    pub admin_url: Option<String>,
    /// Süren eylem.
    pub op: Option<Op>,
    pub failure: Option<Failure>,
    /// Son gönderilen deneme sayfasının kuyruğu (satırda "gönderildi" notu; sonraki eylem siler).
    pub sent: Option<String>,
}

impl Snapshot {
    pub fn printer(&self, name: &str) -> Option<&Printer> {
        self.state.printers.iter().find(|p| p.name == name)
    }
}

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// Etkin varsayılan: kullanıcınınki (`lpoptions`) kuyruk hâlâ varsa, yoksa sunucununki.
pub fn effective_default(user: Option<String>, server: Option<String>, printers: &[Printer]) -> Option<String> {
    let exists = |n: &String| printers.iter().any(|p| &p.name == n);
    user.filter(exists).or_else(|| server.filter(exists))
}

/// Görünen ada göre sıralar (büyük/küçük harf duyarsız; eşitlikte kuyruk adı).
pub fn sort_printers(printers: &mut [Printer]) {
    printers.sort_by(|a, b| a.display_name().to_lowercase().cmp(&b.display_name().to_lowercase()).then_with(|| a.name.cmp(&b.name)));
}

/// Durumu okur: kuyruklar, varsayılan, işler. Kuyruk okunamazsa hata; varsayılan/iş okunamazsa
/// boş kabul edilir (kuyruk listesi asıl içerik).
pub fn read_state(cfg: &Config) -> Result<State, CupsError> {
    let mut printers = cups::printers(&cfg.server)?;
    sort_printers(&mut printers);
    let user_default = cfg.lpoptions.as_deref().and_then(lpoptions::read_default);
    let server_default = cups::server_default(&cfg.server).unwrap_or_else(|e| {
        tracing::debug!("yazıcılar: varsayılan okunamadı: {e}");
        None
    });
    let default = effective_default(user_default, server_default, &printers);
    let mut jobs = cups::jobs(&cfg.server, &cfg.user).unwrap_or_else(|e| {
        tracing::debug!("yazıcılar: işler okunamadı: {e}");
        Vec::new()
    });
    jobs.sort_by_key(|j| j.id);
    Ok(State { printers, default, jobs })
}

/// Okuma sonucunu anlık görüntüye uygular; `true`: görünür bir şey değişti.
pub fn apply_read(s: &mut Snapshot, read: Result<State, CupsError>) -> bool {
    let (status, state) = match read {
        Ok(state) => (Status::Ready, state),
        Err(CupsError::Unreachable) => (Status::Unavailable, State::default()),
        Err(e) => {
            tracing::info!("yazıcılar: CUPS okunamadı: {e}");
            (Status::Failed, State::default())
        }
    };
    if s.status == status && s.state == state {
        return false;
    }
    s.status = status;
    s.state = state;
    // Gönderildi notu, kuyruk gidince düşer.
    if s.sent.as_ref().is_some_and(|n| s.printer(n).is_none()) {
        s.sent = None;
    }
    true
}

/// Eylemi uygular (bloklar).
pub fn perform(cfg: &Config, op: &Op, printers: &[Printer]) -> Result<(), OpError> {
    match op {
        Op::SetDefault(name) => {
            if !printers.iter().any(|p| &p.name == name) {
                return Err(OpError::NoPrinter);
            }
            let path = cfg.lpoptions.as_deref().ok_or(OpError::Write)?;
            lpoptions::write_default(path, name).map_err(|e| {
                tracing::warn!("yazıcılar: lpoptions yazılamadı: {}", e.kind());
                OpError::Write
            })
        }
        Op::TestPage(name, title) => {
            let p = printers.iter().find(|p| &p.name == name).ok_or(OpError::NoPrinter)?;
            cups::print_test_page(&cfg.server, &cfg.user, &p.uri, title, &cfg.test_page).map(drop).map_err(OpError::Cups)
        }
        Op::Cancel(id) => match cups::cancel_job(&cfg.server, &cfg.user, *id) {
            // İş bu arada bitti ya da silindi: amaç gerçekleşti.
            Err(e) if e.is_gone() => Ok(()),
            r => r.map_err(OpError::Cups),
        },
    }
}

// ---- Süreç geneli durum ve servis ---------------------------------------------------------------

#[derive(Debug)]
enum Cmd {
    Refresh,
    Act(Op),
}

static SNAPSHOT: Mutex<Option<Snapshot>> = Mutex::new(None);
static SENDER: OnceLock<Mutex<Sender<Cmd>>> = OnceLock::new();
static STARTED: AtomicBool = AtomicBool::new(false);
static VISIBLE: AtomicBool = AtomicBool::new(false);

/// Son anlık görüntü.
pub fn snapshot() -> Snapshot {
    SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default()
}

/// Değiştirir; `f` `true` dönerse UI'a bildirir.
fn update(f: impl FnOnce(&mut Snapshot) -> bool) {
    let changed = {
        let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        f(s.get_or_insert_with(Snapshot::default))
    };
    if changed {
        emit(Event::Printers);
    }
}

fn send(cmd: Cmd) {
    if let Some(tx) = SENDER.get() {
        let _ = tx.lock().unwrap_or_else(|e| e.into_inner()).send(cmd);
    }
}

/// Sayfa ilk kurulurken: servisi bir kez başlatır.
pub fn ensure() {
    // Birim testleri CUPS'a gitmez (ve thread açmaz).
    if cfg!(test) || STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let cfg = Config::from_env();
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = SENDER.set(Mutex::new(tx));
    update(|s| {
        s.test_page = cfg.test_page.is_file();
        s.admin_url = cfg.server.admin_url();
        true
    });
    if let Err(e) = std::thread::Builder::new().name("nuxut-printers".into()).spawn(move || run(&cfg, &rx)) {
        tracing::warn!("yazıcılar: thread açılamadı: {e}");
        update(|s| {
            s.status = Status::Failed;
            true
        });
        return;
    }
    if VISIBLE.load(Ordering::Relaxed) {
        send(Cmd::Refresh);
    }
}

/// Yazıcılar sayfası görünür oldu / gizlendi: görünürken yoklanır.
pub fn set_page_visible(on: bool) {
    let was = VISIBLE.swap(on, Ordering::Relaxed);
    if on && !was {
        send(Cmd::Refresh);
    }
}

/// Pencere odağı döndü: sayfa görünüyorsa hemen yeniden oku.
pub fn recheck() {
    if VISIBLE.load(Ordering::Relaxed) {
        send(Cmd::Refresh);
    }
}

/// Eylemi başlatır (başka eylem sürüyorsa yok sayılır).
fn act(op: Op) {
    let mut accepted = false;
    update(|s| {
        if s.op.is_some() {
            return false;
        }
        s.op = Some(op.clone());
        s.failure = None;
        s.sent = None;
        accepted = true;
        true
    });
    if accepted {
        send(Cmd::Act(op));
    }
}

pub fn set_default(name: &str) {
    act(Op::SetDefault(name.to_owned()));
}

/// Deneme sayfası; `title` kuyrukta görünen iş adı (etkin dilde).
pub fn print_test_page(name: &str, title: &str) {
    act(Op::TestPage(name.to_owned(), title.to_owned()));
}

pub fn cancel_job(id: i32) {
    act(Op::Cancel(id));
}

fn has_active_jobs() -> bool {
    SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).as_ref().is_some_and(|s| !s.state.jobs.is_empty())
}

fn refresh(cfg: &Config) {
    let read = read_state(cfg);
    update(|s| apply_read(s, read));
}

fn run(cfg: &Config, rx: &Receiver<Cmd>) {
    loop {
        let cmd = if VISIBLE.load(Ordering::Relaxed) {
            let wait = if has_active_jobs() { POLL_BUSY } else { POLL_IDLE };
            match rx.recv_timeout(wait) {
                Ok(c) => c,
                Err(RecvTimeoutError::Timeout) => Cmd::Refresh,
                Err(RecvTimeoutError::Disconnected) => return,
            }
        } else {
            match rx.recv() {
                Ok(c) => c,
                Err(_) => return,
            }
        };
        match cmd {
            Cmd::Refresh => refresh(cfg),
            Cmd::Act(op) => {
                let printers = snapshot().state.printers;
                let result = perform(cfg, &op, &printers);
                if let Err(e) = &result {
                    tracing::info!("yazıcılar: {} başarısız: {e:?}", op.name());
                }
                let read = read_state(cfg);
                update(|s| {
                    apply_read(s, read);
                    match result {
                        Ok(()) => {
                            if let Op::TestPage(name, _) = &op {
                                s.sent = Some(name.clone());
                            }
                        }
                        Err(error) => s.failure = Some(Failure { op: op.clone(), error }),
                    }
                    s.op = None;
                    true
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn printer(name: &str, info: &str) -> Printer {
        Printer { name: name.into(), info: info.into(), uri: cups::printer_uri(name), accepting: true, ..Printer::default() }
    }

    #[test]
    fn default_resolution() {
        let ps = [printer("a", ""), printer("b", "")];
        assert_eq!(effective_default(Some("b".into()), Some("a".into()), &ps), Some("b".into()), "kullanıcınınki önce");
        assert_eq!(effective_default(Some("gitti".into()), Some("a".into()), &ps), Some("a".into()), "silinmiş kuyruk → sunucu");
        assert_eq!(effective_default(None, Some("yok".into()), &ps), None);
        assert_eq!(effective_default(None, None, &ps), None);
    }

    #[test]
    fn sorting_by_display_name() {
        let mut ps = vec![printer("z", "ofis"), printer("b", ""), printer("a", "Ev")];
        sort_printers(&mut ps);
        let names: Vec<_> = ps.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["b", "a", "z"]);
    }

    #[test]
    fn read_results() {
        let mut s = Snapshot::default();
        assert!(apply_read(&mut s, Err(CupsError::Unreachable)));
        assert_eq!(s.status, Status::Unavailable);
        assert!(!apply_read(&mut s, Err(CupsError::Unreachable)), "aynı durum: bildirim yok");
        assert!(apply_read(&mut s, Err(CupsError::Http(426))));
        assert_eq!(s.status, Status::Failed);
        s.sent = Some("a".into());
        let state = State { printers: vec![printer("a", "")], default: Some("a".into()), jobs: vec![] };
        assert!(apply_read(&mut s, Ok(state.clone())));
        assert_eq!((s.status, s.sent.as_deref()), (Status::Ready, Some("a")));
        assert!(!apply_read(&mut s, Ok(state)));
        assert!(apply_read(&mut s, Ok(State::default())));
        assert_eq!(s.sent, None, "kuyruk gidince not düşer");
    }

    #[test]
    fn set_default_needs_existing_printer_and_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut cfg = Config { server: Server::Unix(dir.path().join("yok.sock")), user: "u".into(), lpoptions: Some(dir.path().join(".cups/lpoptions")), test_page: dir.path().join("tp") };
        let ps = [printer("a", "")];
        assert_eq!(perform(&cfg, &Op::SetDefault("b".into()), &ps), Err(OpError::NoPrinter));
        assert_eq!(perform(&cfg, &Op::SetDefault("a".into()), &ps), Ok(()));
        assert_eq!(lpoptions::read_default(cfg.lpoptions.as_ref().unwrap()), Some("a".into()));
        cfg.lpoptions = None;
        assert_eq!(perform(&cfg, &Op::SetDefault("a".into()), &ps), Err(OpError::Write));
        // Sunucu yok: iptal ve deneme sayfası CUPS hatası döner (gerçek CUPS'a gidilmez).
        assert_eq!(perform(&cfg, &Op::Cancel(1), &ps), Err(OpError::Cups(CupsError::Unreachable)));
        assert_eq!(perform(&cfg, &Op::TestPage("a".into(), "t".into()), &ps), Err(OpError::Cups(CupsError::Document)), "belge yok");
    }
}
