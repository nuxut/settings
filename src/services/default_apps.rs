//! Varsayılan uygulamalar (sözleşme defaults-v1, shell `docs/design/defaults.md`; Ayarlar
//! tarafı kararları `docs/design/default-apps.md`).
//!
//! Ayarlar `mimeapps.list`'e YAZMAZ (sözleşme 1: yalnızca shell.toml + framework.toml): aday
//! listesi, mevcut seçim ve yazım shell'dedir. Ayarlar `msg defaults status` ile okur,
//! `msg defaults set <rol> <desktop-id>` ile ister; geri alma shell'in snapshot'ıdır
//! (`SettingId` `defaults.<rol>`, [`super::snapshots`]).
//!
//! Yanıt (sürüm 1): `{"v":1, "roles":[{role, current:{id,name,icon}|null, source, mixed,
//! shadowed_by, available, candidates:[{id,name,icon,hidden}]}]}`. Bilinmeyen alan ve bilinmeyen
//! rol yok sayılır; shell'in yanıtı güvenilmez girdi gibi doğrulanır (kimlik biçimi, uzunluk,
//! denetim karakteri, sayı sınırı). Bekleme 2 sn (çerçeve varsayılanı); dosya izleme yok —
//! pencere odağı dönünce yeniden sorulur ([`recheck`]).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use nuxut::i18n::t;

use super::shell_ipc::{IpcLink, LinkError, ShellLink};

/// Desteklenen yanıt sürümü.
pub const VERSION: u64 = 1;
/// Rol başına kabul edilen en çok aday (fazlası atılır; menü sınırı da ayrıca kırpar).
pub const MAX_CANDIDATES: usize = 256;
/// Görünen ad sınırı (karakter).
const MAX_NAME: usize = 128;
/// `shadowed_by` yolu sınırı (karakter).
const MAX_PATH: usize = 512;
/// `set` isteğinin bekleme süresi: shell yazımdan önce snapshot alır (dosya + depo kilidi).
pub const SET_TIMEOUT: Duration = Duration::from_secs(5);
/// `set` sonrası durum yeniden sorulmadan önceki kısa bekleme (yazım otursun).
const AFTER_SET: Duration = Duration::from_millis(150);

/// Sözleşmedeki roller, tablo sırasıyla.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Role {
    Browser,
    Email,
    FileManager,
    Terminal,
    TextEditor,
    Image,
    Video,
    Music,
    Pdf,
}

impl Role {
    pub const ALL: [Role; 9] =
        [Role::Browser, Role::Email, Role::FileManager, Role::Terminal, Role::TextEditor, Role::Image, Role::Video, Role::Music, Role::Pdf];

    /// IPC adı (`msg defaults set <rol>`).
    pub fn key(self) -> &'static str {
        match self {
            Role::Browser => "browser",
            Role::Email => "email",
            Role::FileManager => "file_manager",
            Role::Terminal => "terminal",
            Role::TextEditor => "text_editor",
            Role::Image => "image",
            Role::Video => "video",
            Role::Music => "music",
            Role::Pdf => "pdf",
        }
    }

    pub fn from_key(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.key() == s)
    }

    /// Snapshot `SettingId`'si ve kayıttaki satır kimliği (aynı: `defaults.<rol>`).
    pub fn setting_id(self) -> &'static str {
        match self {
            Role::Browser => "defaults.browser",
            Role::Email => "defaults.email",
            Role::FileManager => "defaults.file_manager",
            Role::Terminal => "defaults.terminal",
            Role::TextEditor => "defaults.text_editor",
            Role::Image => "defaults.image",
            Role::Video => "defaults.video",
            Role::Music => "defaults.music",
            Role::Pdf => "defaults.pdf",
        }
    }

    pub fn from_setting_id(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.setting_id() == s)
    }
}

/// Mevcut seçimin nereden çözüldüğü (`source`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Source {
    /// `$XDG_CONFIG_HOME/mimeapps.list` (Nuxut'un yazdığı yer).
    User,
    /// `$XDG_CONFIG_HOME/<masaüstü>-mimeapps.list`.
    DesktopUser,
    /// Sistem/dağıtım `mimeapps.list`'i.
    System,
    /// Hiçbir liste tanımlamıyor; `mimeinfo.cache`'in ilk adayı.
    SystemCache,
    #[default]
    None,
    /// Daha yeni shell'in bilinmeyen değeri.
    Unknown,
}

impl Source {
    fn parse(s: Option<&str>) -> Self {
        match s {
            Some("user") => Self::User,
            Some("desktop_user") => Self::DesktopUser,
            Some("system") => Self::System,
            Some("system_cache") => Self::SystemCache,
            Some("none") | None => Self::None,
            Some(_) => Self::Unknown,
        }
    }
}

/// Bir uygulama (`current` ya da aday).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// XDG desktop-id (`firefox.desktop`).
    pub id: String,
    pub name: String,
    /// Tema ikon adı ya da mutlak yol (`.desktop` `Icon=`); yoksa `None`.
    pub icon: Option<String>,
    /// `NoDisplay=true` (yalnızca adaylarda anlamlı).
    pub hidden: bool,
}

/// Bir rolün durumu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleStatus {
    pub role: Role,
    /// `None`: bu türü hiçbir şey açmıyor.
    pub current: Option<App>,
    pub source: Source,
    /// Yazılan kümedeki MIME'lar birden çok uygulamaya gidiyor.
    pub mixed: bool,
    /// Masaüstüne özgü kullanıcı dosyası (`~` kısaltmalı) aynı türü tanımlıyor; o kazanır.
    pub shadowed_by: Option<String>,
    /// `false`: rol bu sistemde yazılamaz (terminal: `xdg-terminal-exec` yok).
    pub available: bool,
    /// Shell sırasıyla (ada göre); [`Self::ordered`] görünür olanları öne alır.
    pub candidates: Vec<App>,
}

impl RoleStatus {
    /// Seçicinin sırası: görünür adaylar, ardından `NoDisplay` olanlar (her grup shell sırasıyla).
    pub fn ordered(&self) -> Vec<&App> {
        let (shown, hidden): (Vec<&App>, Vec<&App>) = self.candidates.iter().partition(|a| !a.hidden);
        shown.into_iter().chain(hidden).collect()
    }

    /// Mevcut seçim aday listesinde mi (değilse kurulu değil ya da rol için uygun değil).
    pub fn current_installed(&self) -> bool {
        self.current.as_ref().is_some_and(|c| self.candidates.iter().any(|a| a.id == c.id))
    }
}

/// `msg defaults status` yanıtının ayrıştırılmış hâli.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DefaultsStatus {
    pub roles: Vec<RoleStatus>,
}

impl DefaultsStatus {
    pub fn role(&self, role: Role) -> Option<&RoleStatus> {
        self.roles.iter().find(|r| r.role == role)
    }
}

/// Shell'den okunan son durum.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DefaultsState {
    /// Henüz sorulmadı / yanıt bekleniyor.
    #[default]
    Pending,
    Ready(Arc<DefaultsStatus>),
    /// Shell çalışmıyor.
    NotRunning,
    /// Shell bu komutu tanımıyor (eski sürüm) ya da yanıt sözleşme dışı.
    Unsupported,
    /// Test örneği: bilerek sorulmadı.
    TestInstance,
    /// Bağlantı hatası, zaman aşımı.
    Failed(String),
}

impl DefaultsState {
    pub fn ready(&self) -> Option<&DefaultsStatus> {
        match self {
            DefaultsState::Ready(s) => Some(s),
            _ => None,
        }
    }
}

// ---- Doğrulama ------------------------------------------------------------------------------

/// XDG desktop-id: `[A-Za-z0-9._+-]`, `.desktop` ile biter, nokta ya da tire ile başlamaz,
/// ≤ 255 bayt (shell'in `msg defaults set` doğrulamasıyla aynı küme; `-` ile başlayan kimlik
/// komut satırında seçenek gibi okunabilirdi). Alt dizindeki dosyanın kimliği
/// `kde-org.kde.foo.desktop` biçimindedir; `/` olmaz.
pub fn valid_desktop_id(s: &str) -> bool {
    s.len() > ".desktop".len()
        && s.len() <= 255
        && s.ends_with(".desktop")
        && !s.starts_with(['.', '-'])
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'+'))
}

/// Tema ikon adı (`firefox`, `org.gnome.Nautilus`): yol değil. Mutlak yol ayrıca kabul edilir
/// ama Ayarlar onu çözmez ([`icon_name`]).
fn valid_icon(s: &str) -> bool {
    !s.is_empty() && s.len() <= 512 && !s.chars().any(char::is_control) && (s.starts_with('/') || is_icon_name(s))
}

fn is_icon_name(s: &str) -> bool {
    (1..=128).contains(&s.len()) && !s.starts_with('.') && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'+'))
}

/// Temadan çözülecek ikon adı (mutlak yol ve geçersiz ad → `None`).
pub fn icon_name(app: &App) -> Option<&str> {
    app.icon.as_deref().filter(|s| is_icon_name(s))
}

/// Görünen metin: denetim karakterleri atılır, kırpılır, sınırlanır; boşsa `None`.
fn clean_text(s: &str, max: usize) -> Option<String> {
    let s: String = s.chars().filter(|c| !c.is_control()).take(max).collect();
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_owned())
}

fn parse_app(v: &serde_json::Value) -> Option<App> {
    let o = v.as_object()?;
    let id = o.get("id")?.as_str().filter(|s| valid_desktop_id(s))?.to_owned();
    // Ad yoksa ya da boşsa kimlik (sözleşme: kurulu olmayan `current`'ta ad = kimlik).
    let name = o.get("name").and_then(|n| n.as_str()).and_then(|n| clean_text(n, MAX_NAME)).unwrap_or_else(|| id.clone());
    let icon = o.get("icon").and_then(|i| i.as_str()).filter(|i| valid_icon(i)).map(str::to_owned);
    let hidden = o.get("hidden").and_then(|h| h.as_bool()).unwrap_or(false);
    Some(App { id, name, icon, hidden })
}

fn parse_role(v: &serde_json::Value) -> Option<RoleStatus> {
    let o = v.as_object()?;
    let role = Role::from_key(o.get("role")?.as_str()?)?;
    let mut candidates: Vec<App> = Vec::new();
    for c in o.get("candidates").and_then(|c| c.as_array()).into_iter().flatten().filter_map(parse_app) {
        if candidates.len() >= MAX_CANDIDATES {
            break;
        }
        // Aynı kimlik iki kez: ilki kazanır (shell'in "ilk bulunan" kuralıyla aynı).
        if !candidates.iter().any(|a| a.id == c.id) {
            candidates.push(c);
        }
    }
    Some(RoleStatus {
        role,
        // `current` bozuksa "hiçbir şey açmıyor" sayılmaz: seçim bilinmiyor → yine `None`,
        // ama yanlış uygulama gösterilmez.
        current: o.get("current").and_then(parse_app).map(|a| App { hidden: false, ..a }),
        source: Source::parse(o.get("source").and_then(|s| s.as_str())),
        mixed: o.get("mixed").and_then(|m| m.as_bool()).unwrap_or(false),
        shadowed_by: o.get("shadowed_by").and_then(|s| s.as_str()).and_then(|s| clean_text(s, MAX_PATH)),
        // Alan yoksa kullanılabilir (yalnızca terminal `false` döner).
        available: o.get("available").and_then(|a| a.as_bool()).unwrap_or(true),
        candidates,
    })
}

/// Yanıtı ayrıştırır; sürüm uymuyor ya da `roles` dizisi yoksa `None`. Bilinmeyen rol ve
/// bozuk rol girdisi atlanır; aynı rol iki kez gelirse ilki kalır. Roller sözleşme sırasına
/// dizilir.
pub fn parse_status(text: &str) -> Option<DefaultsStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    if o.get("v")?.as_u64()? != VERSION {
        return None;
    }
    let mut roles: Vec<RoleStatus> = Vec::new();
    for r in o.get("roles")?.as_array()?.iter().take(Role::ALL.len() * 4).filter_map(parse_role) {
        if !roles.iter().any(|x| x.role == r.role) {
            roles.push(r);
        }
    }
    roles.sort_by_key(|r| r.role);
    Some(DefaultsStatus { roles })
}

/// Bir bağlantıdan durumu sorar (saf; testte sahte bağlantı).
pub fn query_status(link: &dyn ShellLink) -> DefaultsState {
    match link.request(&["defaults", "status"]) {
        Ok(reply) => parse_status(&reply).map_or(DefaultsState::Unsupported, |s| DefaultsState::Ready(Arc::new(s))),
        Err(LinkError::NotRunning) => DefaultsState::NotRunning,
        Err(LinkError::TestInstance) => DefaultsState::TestInstance,
        Err(LinkError::Shell(_) | LinkError::BadReply) => DefaultsState::Unsupported,
        Err(e) => DefaultsState::Failed(e.to_string()),
    }
}

/// Sayfanın üstündeki sakin not (satırlar neden yok ya da henüz yok); durum hazırsa `None`.
pub fn status_note(state: &DefaultsState) -> Option<String> {
    match state {
        // Hiç rol yok: sözleşme "hepsi her zaman var" der; böyle shell bu özelliği sunmuyor sayılır.
        DefaultsState::Ready(s) if s.roles.is_empty() => Some(t!("settings-defaults-note-unsupported")),
        DefaultsState::Ready(_) => None,
        // İlk yanıt gelene dek (genelde birkaç ms; ilk taramada en çok 2 sn) sayfa boş kalmasın.
        DefaultsState::Pending => Some(t!("settings-defaults-note-loading")),
        DefaultsState::NotRunning | DefaultsState::TestInstance => Some(t!("settings-defaults-note-not-running")),
        DefaultsState::Failed(_) => Some(t!("settings-defaults-note-failed")),
        DefaultsState::Unsupported => Some(t!("settings-defaults-note-unsupported")),
    }
}

// ---- Seçim ----------------------------------------------------------------------------------

/// `defaults set` hata kodları (sözleşme) + yerel durumlar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetError {
    UnknownRole,
    /// Uygulama bu rolün adayı değil (kaldırılmış olabilir).
    UnknownApp,
    /// Rol bu sistemde yazılamaz (terminal, araç yok).
    Unavailable,
    /// Shell bu örnekte dosya yazmıyor.
    WriterDisabled,
    WriteFailed,
    /// Bilinmeyen kod ya da kodsuz hata (eski shell `bilinmeyen komut`).
    Rejected(String),
    NotRunning,
    TestInstance,
    /// Yanıt süresinde gelmedi: yazım yapılmış olabilir, durum yeniden okunur.
    Timeout,
    /// Yanıt sözleşme dışı.
    BadReply,
    Io(String),
}

impl SetError {
    fn from_code(code: &str) -> Self {
        match code {
            "unknown_role" => Self::UnknownRole,
            "unknown_app" => Self::UnknownApp,
            "unavailable" => Self::Unavailable,
            "writer_disabled" => Self::WriterDisabled,
            "write_failed" => Self::WriteFailed,
            other => Self::Rejected(other.chars().take(32).collect()),
        }
    }
}

/// `defaults set` sonucu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetOutcome {
    /// `snapshot`: geri alma kaydı (değişiklik yoksa `None`); `written`: yazılan MIME sayısı.
    Done { snapshot: Option<String>, written: usize },
    Failed(SetError),
}

/// `set` isteğinin argümanları; geçersiz kimlikte `None` (shell'e hiç gitmez).
pub fn set_args(role: Role, id: &str) -> Option<[String; 4]> {
    valid_desktop_id(id).then(|| ["defaults".to_owned(), "set".to_owned(), role.key().to_owned(), id.to_owned()])
}

fn is_code(s: &str) -> bool {
    !s.is_empty() && s.len() <= 32 && s.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
}

/// Hata satırının (`error: <kod>[: <ileti>]`, `error:` önekini [`check_reply`] atmıştır) kodu.
///
/// [`check_reply`]: super::shell_ipc::check_reply
fn error_code(msg: &str) -> Option<&str> {
    let head = msg.split_once(':').map_or(msg, |(c, _)| c).trim();
    is_code(head).then_some(head)
}

/// Başarı yanıtı (`{"ok":true,"snapshot":…,"written":[…]}`) ya da JSON hata biçimi
/// (`{"ok":false,"error":"<kod>"[,"detail":…]}` — shell'in kullandığı biçim; `detail`
/// kullanıcıya gösterilmez). Eski/elle yazılmış `error: <kod>` satırı [`set`]'te çözülür.
pub fn parse_set_reply(text: &str) -> SetOutcome {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(text.trim()) else {
        return SetOutcome::Failed(SetError::BadReply);
    };
    match v.get("ok").and_then(|o| o.as_bool()) {
        Some(true) => SetOutcome::Done {
            snapshot: v.get("snapshot").and_then(|s| s.as_str()).filter(|s| super::snapshots::valid_snapshot_id(s)).map(str::to_owned),
            written: v.get("written").and_then(|w| w.as_array()).map_or(0, Vec::len),
        },
        Some(false) => {
            let code = v.get("error").or_else(|| v.get("code")).and_then(|c| c.as_str()).filter(|c| is_code(c));
            SetOutcome::Failed(code.map_or(SetError::Rejected(String::new()), SetError::from_code))
        }
        None => SetOutcome::Failed(SetError::BadReply),
    }
}

/// Saf seçim isteği (arka plan thread'inde; `link` [`SET_TIMEOUT`] bekler).
pub fn set(link: &dyn ShellLink, role: Role, id: &str) -> SetOutcome {
    let Some(args) = set_args(role, id) else { return SetOutcome::Failed(SetError::UnknownApp) };
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match link.request(&args) {
        Ok(reply) => parse_set_reply(&reply),
        Err(LinkError::Shell(msg)) => SetOutcome::Failed(error_code(&msg).map_or_else(|| SetError::Rejected(String::new()), SetError::from_code)),
        Err(LinkError::NotRunning) => SetOutcome::Failed(SetError::NotRunning),
        Err(LinkError::TestInstance) => SetOutcome::Failed(SetError::TestInstance),
        Err(LinkError::Timeout) => SetOutcome::Failed(SetError::Timeout),
        Err(LinkError::BadReply) => SetOutcome::Failed(SetError::BadReply),
        Err(e @ (LinkError::InvalidInstance | LinkError::Io(_))) => SetOutcome::Failed(SetError::Io(e.to_string())),
    }
}

/// Hatanın kullanıcı metni (şeritte gösterilir; shell metni geçmez). Başarıda `None`: satır
/// yeni seçimi zaten gösterir.
pub fn outcome_text(outcome: &SetOutcome) -> Option<String> {
    let SetOutcome::Failed(e) = outcome else { return None };
    Some(match e {
        SetError::UnknownRole => t!("settings-defaults-error-unknown-role"),
        SetError::UnknownApp => t!("settings-defaults-error-unknown-app"),
        SetError::Unavailable => t!("settings-defaults-error-unavailable"),
        SetError::WriterDisabled => t!("settings-defaults-error-writer-disabled"),
        SetError::WriteFailed => t!("settings-defaults-error-write-failed"),
        SetError::Rejected(_) | SetError::BadReply => t!("settings-defaults-error-rejected"),
        SetError::NotRunning | SetError::TestInstance => t!("settings-defaults-error-not-running"),
        SetError::Timeout => t!("settings-defaults-error-timeout"),
        SetError::Io(e) => t!("settings-defaults-error-io", error = e.as_str()),
    })
}

// ---- Süreç geneli durum ---------------------------------------------------------------------

static STATUS: Mutex<DefaultsState> = Mutex::new(DefaultsState::Pending);
/// (çalışan sorgu var mı, bittiğinde bir kez daha sor)
static QUERY: Mutex<(bool, bool)> = Mutex::new((false, false));
/// Bu süreçte durum en az bir kez soruldu mu (sayfa açıldı).
static ASKED: AtomicBool = AtomicBool::new(false);

/// Son durum (sormaz).
pub fn status() -> DefaultsState {
    STATUS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Rol satırları gösterilebilir mi (durum okundu ve rol yanıtta var).
pub fn role_ready(role: Role) -> bool {
    STATUS.lock().unwrap_or_else(|e| e.into_inner()).ready().is_some_and(|s| s.role(role).is_some())
}

/// Testler: süreç geneli durumu kurar (sayfa satırlarının durumu).
#[cfg(test)]
pub fn set_status_for_tests(state: DefaultsState) {
    *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = state;
}

/// Hiç sorulmadıysa arka planda bir kez sorar (sayfa ilk kurulurken).
pub fn ensure() {
    if !ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Pencere odağı geri geldi: sayfa daha önce açıldıysa yeniden sorar (kullanıcı başka bir
/// araçla varsayılanı değiştirmiş ya da uygulama kurmuş olabilir). Yoklama yok.
pub fn recheck() {
    if ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Durumu `delay` sonra sorar; bitince [`Event::DefaultApps`](super::Event::DefaultApps).
/// Sorgu sürüyorsa bittiğinde bir kez daha sorulur.
pub fn refresh(delay: Duration) {
    // Birim testleri shell'e gitmez (ve arka plan thread'i açmaz).
    if cfg!(test) {
        return;
    }
    ASKED.store(true, Ordering::Relaxed);
    let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
    if q.0 {
        q.1 = true;
        return;
    }
    q.0 = true;
    let spawned = std::thread::Builder::new().name("defaults-status".into()).spawn(move || {
        let link = IpcLink::from_env();
        let mut delay = delay;
        loop {
            std::thread::sleep(delay);
            let state = query_status(&link);
            *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = state;
            super::emit(super::Event::DefaultApps);
            let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
            if q.1 {
                q.1 = false;
                delay = Duration::from_millis(100);
                continue;
            }
            q.0 = false;
            break;
        }
    });
    if spawned.is_err() {
        q.0 = false;
    }
}

/// UI → servis.
#[derive(Debug, Clone)]
pub enum Command {
    /// Rolün varsayılanını `id` yap.
    Set { role: Role, id: String },
}

/// İsteği kısa ömürlü bir thread'de yürütür (seçim seyrek). Hata metni
/// [`Event::DefaultAppSet`](super::Event::DefaultAppSet) ile şeride gider; her durumda (zaman
/// aşımında yazım yapılmış olabilir) durum yeniden sorulur, snapshot alındıysa liste de.
pub fn send(cmd: Command) {
    if cfg!(test) {
        return;
    }
    let _ = std::thread::Builder::new().name("defaults-set".into()).spawn(move || {
        let Command::Set { role, id } = cmd;
        let link = IpcLink::from_env().with_timeout(SET_TIMEOUT);
        let outcome = set(&link, role, &id);
        if let SetOutcome::Failed(e) = &outcome {
            tracing::info!(role = role.key(), ?e, "varsayılan uygulama yazılamadı");
        }
        if let Some(text) = outcome_text(&outcome) {
            super::emit(super::Event::DefaultAppSet { text });
        }
        if matches!(outcome, SetOutcome::Done { snapshot: Some(_), .. } | SetOutcome::Failed(SetError::Timeout)) {
            super::snapshots::send(super::snapshots::Command::Refresh);
        }
        refresh(AFTER_SET);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::shell_ipc::FakeLink;

    pub const SAMPLE: &str = r#"{ "v": 1, "extra": {"gelecek": true},
      "roles": [
        { "role": "browser", "current": {"id":"firefox.desktop","name":"Firefox","icon":"firefox"},
          "source": "user", "mixed": false, "shadowed_by": null, "available": true,
          "candidates": [
            {"id":"chromium.desktop","name":"Chromium","icon":"chromium","hidden":false},
            {"id":"firefox.desktop","name":"Firefox","icon":"firefox","hidden":false},
            {"id":"handler.desktop","name":"Bağlantı işleyici","icon":"/opt/h.png","hidden":true},
            {"id":"firefox.desktop","name":"Kopya","icon":"x","hidden":false}
          ] },
        { "role": "terminal", "current": null, "source": "none", "mixed": false, "shadowed_by": null,
          "available": false, "candidates": [{"id":"kitty.desktop","name":"kitty","icon":"kitty","hidden":false}] },
        { "role": "image", "current": {"id":"gone.desktop","name":"gone.desktop","icon":null},
          "source": "desktop_user", "mixed": true, "shadowed_by": "~/.config/hyprland-mimeapps.list", "available": true,
          "candidates": [], "yeni_alan": 3 },
        { "role": "email", "current": null, "source": "weird", "candidates": [] },
        { "role": "spaceship", "current": null },
        { "role": "email", "current": {"id":"x.desktop","name":"X"} }
      ] }"#;

    #[test]
    fn parses_contract_sample() {
        let s = parse_status(SAMPLE).expect("geçerli");
        assert_eq!(s.roles.iter().map(|r| r.role).collect::<Vec<_>>(), vec![Role::Browser, Role::Email, Role::Terminal, Role::Image], "sözleşme sırası; bilinmeyen rol ve ikinci kopya atlanır");
        let b = s.role(Role::Browser).unwrap();
        assert_eq!(b.current.as_ref().map(|c| c.name.as_str()), Some("Firefox"));
        assert_eq!(b.source, Source::User);
        assert_eq!(b.candidates.len(), 3, "aynı kimlik ikinci kez alınmaz");
        assert!(b.current_installed());
        let order: Vec<&str> = b.ordered().iter().map(|a| a.id.as_str()).collect();
        assert_eq!(order, ["chromium.desktop", "firefox.desktop", "handler.desktop"], "NoDisplay sona");
        assert_eq!(icon_name(&b.candidates[2]), None, "mutlak yol temadan çözülmez");
        assert_eq!(icon_name(&b.candidates[0]), Some("chromium"));
        let t = s.role(Role::Terminal).unwrap();
        assert!(!t.available && t.current.is_none() && t.source == Source::None);
        let i = s.role(Role::Image).unwrap();
        assert!(i.mixed && !i.current_installed());
        assert_eq!(i.shadowed_by.as_deref(), Some("~/.config/hyprland-mimeapps.list"));
        assert_eq!(i.source, Source::DesktopUser);
        let e = s.role(Role::Email).unwrap();
        assert_eq!(e.source, Source::Unknown, "bilinmeyen kaynak");
        assert!(e.available, "alan yoksa kullanılabilir");
    }

    #[test]
    fn rejects_broken_and_sanitises_fields() {
        assert_eq!(parse_status(r#"{"v":2,"roles":[]}"#), None, "bilinmeyen sürüm");
        assert_eq!(parse_status(r#"{"roles":[]}"#), None, "sürüm yok");
        assert_eq!(parse_status(r#"{"v":1}"#), None, "roles yok");
        assert_eq!(parse_status(r#"{"v":1,"roles":{}}"#), None);
        assert_eq!(parse_status("ok"), None);
        assert_eq!(parse_status(""), None);
        assert_eq!(parse_status(r#"{"v":1,"roles":[]}"#), Some(DefaultsStatus::default()), "boş liste geçerli");
        let bad = r#"{"v":1,"roles":[{"role":"browser","current":{"id":"../etc/passwd","name":"x"},
           "candidates":[{"id":"a b.desktop","name":"A"},{"id":"ok.desktop","name":"  Ok\u0007 \n "},{"id":"noname.desktop"},
                         {"id":".hidden.desktop","name":"H"},{"id":"firefox","name":"F"},{"id":"ikon.desktop","name":"I","icon":"a/b"}]}]}"#;
        let s = parse_status(bad).unwrap();
        let b = s.role(Role::Browser).unwrap();
        assert_eq!(b.current, None, "geçersiz kimlik gösterilmez");
        let ids: Vec<&str> = b.candidates.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, ["ok.desktop", "noname.desktop", "ikon.desktop"]);
        assert_eq!(b.candidates[0].name, "Ok", "denetim karakteri atılır, kırpılır");
        assert_eq!(b.candidates[1].name, "noname.desktop", "ad yoksa kimlik");
        assert_eq!(b.candidates[2].icon, None, "göreli yollu ikon reddedilir");
        // Aday sınırı.
        let many: Vec<String> = (0..MAX_CANDIDATES + 20).map(|i| format!(r#"{{"id":"a{i}.desktop","name":"A{i}"}}"#)).collect();
        let s = parse_status(&format!(r#"{{"v":1,"roles":[{{"role":"pdf","candidates":[{}]}}]}}"#, many.join(","))).unwrap();
        assert_eq!(s.role(Role::Pdf).unwrap().candidates.len(), MAX_CANDIDATES);
    }

    #[test]
    fn desktop_id_validation() {
        for ok in ["firefox.desktop", "org.gnome.Nautilus.desktop", "kde-org.kde.dolphin.desktop", "a_b-c.desktop", "gvim+x.desktop"] {
            assert!(valid_desktop_id(ok), "{ok}");
        }
        for bad in ["", ".desktop", "firefox", "a b.desktop", "../x.desktop", "a/b.desktop", ".x.desktop", "-x.desktop", "a;rm.desktop", "ğ.desktop"] {
            assert!(!valid_desktop_id(bad), "{bad}");
        }
        assert!(set_args(Role::Pdf, "a/b.desktop").is_none());
        assert_eq!(set_args(Role::FileManager, "thunar.desktop").unwrap(), ["defaults", "set", "file_manager", "thunar.desktop"].map(str::to_owned));
        for r in Role::ALL {
            assert_eq!(Role::from_key(r.key()), Some(r));
            assert_eq!(Role::from_setting_id(r.setting_id()), Some(r));
            assert!(crate::services::snapshots::valid_setting_id(r.setting_id()));
        }
    }

    #[test]
    fn query_uses_defaults_status_and_classifies_errors() {
        let link = FakeLink::with([
            Ok(SAMPLE.to_owned()),
            Ok("error: bilinmeyen komut".to_owned()),
            Ok("{\"v\":1}".to_owned()),
            Err(LinkError::NotRunning),
            Err(LinkError::TestInstance),
            Err(LinkError::Timeout),
        ]);
        let ready = query_status(&link);
        assert!(ready.ready().is_some());
        assert_eq!(status_note(&ready), None);
        assert_eq!(query_status(&link), DefaultsState::Unsupported);
        assert_eq!(query_status(&link), DefaultsState::Unsupported, "sözleşme dışı yanıt");
        assert_eq!(query_status(&link), DefaultsState::NotRunning);
        assert_eq!(query_status(&link), DefaultsState::TestInstance);
        assert!(matches!(query_status(&link), DefaultsState::Failed(_)));
        assert!(link.sent().iter().all(|a| a == &["defaults", "status"]));
    }

    #[test]
    fn status_notes_have_text() {
        crate::i18n::use_turkish_for_tests();
        let nr = status_note(&DefaultsState::NotRunning).unwrap();
        assert!(nr.contains("çalışmıyor"), "{nr}");
        assert_eq!(status_note(&DefaultsState::TestInstance), Some(nr));
        assert!(status_note(&DefaultsState::Unsupported).unwrap().contains("sürüm"));
        assert!(status_note(&DefaultsState::Failed("x".into())).is_some());
        assert!(status_note(&DefaultsState::Pending).unwrap().contains("okunuyor"), "yanıt beklenirken sayfa boş kalmaz");
        let empty = DefaultsState::Ready(Arc::new(DefaultsStatus::default()));
        assert_eq!(status_note(&empty), status_note(&DefaultsState::Unsupported), "rolsüz yanıt: sayfa boş kalmaz");
    }

    #[test]
    fn set_parses_success_and_error_codes() {
        let link = FakeLink::with([
            Ok(r#"{"ok":true,"snapshot":"20261013T101500Z-0a1b","written":["x-scheme-handler/http","x-scheme-handler/https"]}"#.to_owned()),
            Ok(r#"{"ok":true,"snapshot":null,"written":[]}"#.to_owned()),
            Ok("error: unknown_app".to_owned()),
            Ok("error: unavailable: xdg-terminal-exec yok".to_owned()),
            Ok(r#"{"ok":false,"error":"writer_disabled"}"#.to_owned()),
            Ok("error: write_failed: izin yok".to_owned()),
            Ok(r#"{"ok":false,"error":"write_failed","detail":"permission_denied"}"#.to_owned()),
            Ok("error: unknown_role".to_owned()),
            Ok("error: bilinmeyen komut \"defaults\"".to_owned()),
            Ok("error: brand_new_code".to_owned()),
            Ok("tamam".to_owned()),
            Err(LinkError::NotRunning),
            Err(LinkError::Timeout),
        ]);
        let done = set(&link, Role::Browser, "firefox.desktop");
        assert_eq!(done, SetOutcome::Done { snapshot: Some("20261013T101500Z-0a1b".into()), written: 2 });
        assert_eq!(outcome_text(&done), None, "başarıda şerit yok");
        assert_eq!(set(&link, Role::Browser, "firefox.desktop"), SetOutcome::Done { snapshot: None, written: 0 }, "değişiklik yok");
        let expect = [
            SetError::UnknownApp,
            SetError::Unavailable,
            SetError::WriterDisabled,
            SetError::WriteFailed,
            SetError::WriteFailed,
            SetError::UnknownRole,
            SetError::Rejected(String::new()),
            SetError::Rejected("brand_new_code".into()),
            SetError::BadReply,
            SetError::NotRunning,
            SetError::Timeout,
        ];
        for e in expect {
            assert_eq!(set(&link, Role::Terminal, "kitty.desktop"), SetOutcome::Failed(e));
        }
        let sent = link.sent();
        assert_eq!(sent[0], ["defaults", "set", "browser", "firefox.desktop"]);
        assert_eq!(sent[2], ["defaults", "set", "terminal", "kitty.desktop"]);
        // Geçersiz kimlik shell'e gitmez.
        let quiet = FakeLink::with([]);
        assert_eq!(set(&quiet, Role::Pdf, "a b.desktop"), SetOutcome::Failed(SetError::UnknownApp));
        assert!(quiet.sent().is_empty());
    }

    #[test]
    fn error_texts_are_distinct_and_hide_shell_text() {
        crate::i18n::use_turkish_for_tests();
        let errors = [
            SetError::UnknownRole,
            SetError::UnknownApp,
            SetError::Unavailable,
            SetError::WriterDisabled,
            SetError::WriteFailed,
            SetError::Rejected("gizli_kod".into()),
            SetError::NotRunning,
            SetError::Timeout,
            SetError::Io("bağlantı koptu".into()),
        ];
        let texts: Vec<String> = errors.iter().map(|e| outcome_text(&SetOutcome::Failed(e.clone())).unwrap()).collect();
        let unique: std::collections::HashSet<&String> = texts.iter().collect();
        assert_eq!(unique.len(), texts.len(), "her kodun kendi metni: {texts:?}");
        assert!(!texts[5].contains("gizli_kod"), "shell kodu kullanıcıya geçmez");
        assert!(texts[2].contains("xdg-terminal-exec"));
        assert!(texts[4].contains("mimeapps.list"));
        assert!(texts[8].contains("bağlantı koptu"));
        assert_eq!(outcome_text(&SetOutcome::Failed(SetError::BadReply)), outcome_text(&SetOutcome::Failed(SetError::Rejected(String::new()))));
        assert_eq!(outcome_text(&SetOutcome::Failed(SetError::TestInstance)), Some(texts[6].clone()));
    }
}
