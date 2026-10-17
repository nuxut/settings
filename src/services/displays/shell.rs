//! Ekranlar ↔ shell bağlantısı (`nuxut-shell msg displays …`, shell ADR-0005 §13 donuk v1
//! sözleşmesi).
//!
//! Ayarlar `list`/`status` yoklar, `apply` ile tam istenen durumu gönderir. Shell'de onay
//! penceresi henüz yok (ADR §13.3 ürün kararı): onay isteyen değişiklikte sayfa geri sayımlı
//! "Koru / Geri al" gösterir; Koru → `displays confirm <jeton>`, Geri al → `displays revert
//! <jeton>`. Süre dolarsa shell kendisi geri döner; sonuç `status.last`'tan okunur. Sayfa açıkken
//! bir arka plan thread'i `status`'u saniyede bir sorgular, `gen` değişince `list` alır. Shell'e
//! konuşan kısım [`ShellLink`] arkasındadır: birim testleri sahte bağlantı kullanır, kullanıcının
//! shell'ine hiçbir istek gitmez. İstemci zaman aşımı framework IPC'sinindir (2 sn).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock, mpsc};
use std::time::{Duration, Instant};

use serde::Deserialize;

use nuxut::i18n::t;

use super::{Last, Layout, List, MAX_APPLY_BYTES, Outcome, Pending};

/// Shell'in örnek adı ortam değişkeni (shell `src/ipc.rs` ile aynı kural): tanımlıysa IPC adı
/// `nuxut-shell-<ad>`. Testlerde Ayarlar da kendi shell örneğine konuşur.
pub const INSTANCE_ENV: &str = "NUXUT_SHELL_INSTANCE";
/// Canlı görsel test: shell yerine bu dosyadaki sabit `list` JSON'u (gerçek shell'e hiç gidilmez).
pub const FAKE_ENV: &str = "NUXUT_SETTINGS_DISPLAYS_LIST";
/// Sayfa açıkken `status` yoklama aralığı (ADR K4).
const POLL: Duration = Duration::from_secs(1);

/// Shell'e tek istek (argüman vektörü). Gerçeği framework IPC'si, testte sahte.
pub trait ShellLink: Send {
    fn request(&mut self, args: &[&str]) -> Result<String, LinkError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkError {
    /// Shell çalışmıyor (socket yok).
    NotRunning,
    /// Shell 2 sn içinde cevap vermedi (framework IPC zaman aşımı).
    Timeout,
    Failed(String),
}

/// `error: <kod>: <mesaj>` cevabı ya da yerel hata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellError {
    /// Sözleşme kodu (`stale`, `invalid_scale`, …) ya da yerel kod (`not_running`, `timeout`,
    /// `ipc`, `bad_reply`); kodsuz hata (eski shell) için boş.
    pub code: String,
    pub message: String,
}

impl ShellError {
    fn local(code: &str, message: &str) -> Self {
        Self { code: code.into(), message: message.into() }
    }

    fn from_link(e: LinkError) -> Self {
        match e {
            LinkError::NotRunning => Self::local("not_running", "Shell çalışmıyor"),
            LinkError::Timeout => Self::local("timeout", "Shell zamanında cevap vermedi"),
            LinkError::Failed(m) => Self { code: "ipc".into(), message: m },
        }
    }

    /// Bu hata ekran listesinin eskidiğini söylüyor (liste yeniden alınmalı).
    pub fn needs_refresh(&self) -> bool {
        matches!(self.code.as_str(), "stale" | "unknown_display" | "missing_display")
    }
}

fn is_code(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
}

/// Cevabı veri ya da hata olarak ayırır (`error: <kod>` ve `error: <kod>: <mesaj>`).
pub fn split_reply(reply: &str) -> Result<&str, ShellError> {
    let reply = reply.trim();
    let Some(rest) = reply.strip_prefix("error:") else { return Ok(reply) };
    let rest = rest.trim();
    match rest.split_once(':') {
        Some((code, msg)) if is_code(code) => Err(ShellError { code: code.into(), message: msg.trim().into() }),
        None if is_code(rest) => Err(ShellError { code: rest.into(), message: String::new() }),
        // SÖZLEŞME: kodsuz hata (ör. `bilinmeyen komut "displays"`) eski shell demektir.
        _ => Err(ShellError { code: String::new(), message: rest.into() }),
    }
}

/// `displays status` (ADR-0005 §13.2).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Status {
    pub v: u32,
    #[serde(rename = "gen")]
    pub generation: u64,
    #[serde(default)]
    pub provider: Option<String>,
    /// `false`: bu shell örneği Hyprland'e yazamaz; `apply` → `unavailable`.
    #[serde(default = "yes")]
    pub available: bool,
    #[serde(default)]
    pub pending: Option<Pending>,
    #[serde(default)]
    pub last: Option<Last>,
}

fn yes() -> bool {
    true
}

impl Status {
    pub fn parse(json: &str) -> Option<Self> {
        let s: Status = serde_json::from_str(json).ok()?;
        (s.v == 1).then_some(s)
    }
}

/// `displays apply` başarı cevabı.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ApplyReply {
    pub token: String,
    /// `false`: onaysız değişiklik (yalnızca konum/birincil); uygulandı ve kalıcı yazıldı.
    #[serde(default)]
    pub confirm: bool,
    #[serde(default)]
    pub deadline_ms: u64,
}

/// Sayfanın neden çalışamadığı (sakin boş durum metni buna göre).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unavailable {
    NotRunning,
    /// Shell `displays` komutunu bilmiyor ya da sözleşme sürümü farklı.
    Unsupported,
    /// Shell var ama ekranları yönetemiyor (Hyprland yok, test örneği) ya da bağlantı hatası.
    Other(String),
}

/// `available:false` olan shell'in boş durum metni.
pub fn not_available() -> String {
    t!("settings-displays-empty-not-available")
}

/// Servis → sayfa.
#[derive(Debug, Clone, PartialEq)]
pub enum Update {
    Unavailable(Unavailable),
    List(Box<List>),
    Status(Status),
    Applied(Result<ApplyReply, ShellError>),
    /// `confirm`/`revert` cevabı.
    Decided(Result<(), ShellError>),
}

/// Yoklama mantığı (thread'den bağımsız, testli): son durum ve erişilebilirlik hatırlanır,
/// yalnızca değişiklikler yayılır.
pub struct Poller {
    link: Box<dyn ShellLink>,
    status: Option<Status>,
    unavailable: Option<Unavailable>,
}

impl std::fmt::Debug for Poller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Poller").field("status", &self.status).field("unavailable", &self.unavailable).finish_non_exhaustive()
    }
}

impl Poller {
    pub fn new(link: Box<dyn ShellLink>) -> Self {
        Self { link, status: None, unavailable: None }
    }

    fn unavailable(&mut self, why: Unavailable, out: &mut Vec<Update>) {
        self.status = None;
        if self.unavailable.as_ref() != Some(&why) {
            self.unavailable = Some(why.clone());
            out.push(Update::Unavailable(why));
        }
    }

    fn classify(e: ShellError) -> Unavailable {
        match e.code.as_str() {
            "unavailable" if e.message.is_empty() => Unavailable::Other(not_available()),
            "unavailable" => Unavailable::Other(e.message),
            _ => Unavailable::Unsupported,
        }
    }

    fn ask(&mut self, args: &[&str], out: &mut Vec<Update>) -> Option<String> {
        match self.link.request(args) {
            Ok(r) => match split_reply(&r) {
                Ok(data) => Some(data.to_owned()),
                Err(e) => {
                    self.unavailable(Self::classify(e), out);
                    None
                }
            },
            Err(LinkError::NotRunning) => {
                self.unavailable(Unavailable::NotRunning, out);
                None
            }
            Err(LinkError::Timeout) => {
                self.unavailable(Unavailable::Other(t!("settings-displays-empty-timeout")), out);
                None
            }
            Err(LinkError::Failed(m)) => {
                self.unavailable(Unavailable::Other(m), out);
                None
            }
        }
    }

    /// `status` sorgular; `gen` değiştiyse (ya da `force`) `list` alır.
    pub fn tick(&mut self, force: bool) -> Vec<Update> {
        self.poll(force, false)
    }

    /// `force_status`: durum değişmemiş olsa da yayılır — `apply`/`confirm`/`revert` sonrası
    /// akış sonucu bu durumdan okur (aynı durum daha önce yayılmış ama akış o sırada başka
    /// aşamada olabilir).
    fn poll(&mut self, force: bool, force_status: bool) -> Vec<Update> {
        let mut out = Vec::new();
        let Some(reply) = self.ask(&["displays", "status"], &mut out) else { return out };
        let Some(status) = Status::parse(&reply) else {
            self.unavailable(Unavailable::Unsupported, &mut out);
            return out;
        };
        if !status.available {
            self.unavailable(Unavailable::Other(not_available()), &mut out);
            return out;
        }
        let gen_changed = self.status.as_ref().map(|s| s.generation) != Some(status.generation);
        if force_status || self.status.as_ref() != Some(&status) {
            out.push(Update::Status(status.clone()));
        }
        self.status = Some(status);
        if gen_changed || force || self.unavailable.is_some() {
            let Some(reply) = self.ask(&["displays", "list"], &mut out) else { return out };
            match List::parse(&reply) {
                Ok(list) => {
                    self.unavailable = None;
                    out.push(Update::List(Box::new(list)));
                }
                Err(e) => {
                    tracing::warn!("displays list okunamadı: {e}");
                    self.unavailable(Unavailable::Unsupported, &mut out);
                }
            }
        }
        out
    }

    /// `displays apply <json>`; ardından durum hemen yoklanır.
    pub fn apply(&mut self, json: &str) -> Vec<Update> {
        let result = match self.link.request(&["displays", "apply", json]) {
            Ok(r) => match split_reply(&r) {
                Ok(data) => serde_json::from_str::<ApplyReply>(data).map_err(|_| ShellError::local("bad_reply", "Shell'in cevabı okunamadı")),
                Err(e) => Err(e),
            },
            Err(e) => Err(ShellError::from_link(e)),
        };
        let mut out = vec![Update::Applied(result)];
        out.extend(self.poll(false, true));
        out
    }

    /// `displays confirm|revert <jeton>`; ardından durum hemen yoklanır (`last` sonucu verir).
    pub fn decide(&mut self, keep: bool, token: &str) -> Vec<Update> {
        let verb = if keep { "confirm" } else { "revert" };
        let result = match self.link.request(&["displays", verb, token]) {
            Ok(r) => split_reply(&r).map(|_| ()),
            Err(e) => Err(ShellError::from_link(e)),
        };
        let mut out = vec![Update::Decided(result)];
        out.extend(self.poll(false, true));
        out
    }
}

// ---- Uygulama akışı ---------------------------------------------------------------------------

/// `apply` sonrası sayfanın gösterdiği durum: boşta → gönderildi → (onay isteyen değişiklikte)
/// Koru/Geri al bekleniyor → karar gönderildi → sonuç (`status.last`) ya da hata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Idle,
    Sending,
    /// Değişiklik ekranda; kullanıcı Koru ya da Geri al demeli, yoksa shell süre dolunca döner.
    Awaiting { token: String, remaining_ms: u64 },
    /// Koru (`keep`) ya da Geri al gönderildi, cevap bekleniyor.
    Deciding { token: String, keep: bool },
    /// Bekleyen değişiklik bitti; sonuç `status.last`'tan bekleniyor. `token` yoksa `apply`
    /// cevabı zaman aşımına uğradı: shell'de bekleyen ya da yeni bir sonuç olup olmadığına bakılır.
    Resolving { token: Option<String> },
    Done { outcome: Outcome, detail: Option<String> },
    Failed(ShellError),
}

#[derive(Debug, Clone)]
pub struct ApplyFlow {
    phase: Phase,
    /// Göndermeden önce görülen son `last.token` (zaman aşımında yeni sonucu eskisinden ayırmak için).
    last_seen: Option<String>,
}

impl Default for ApplyFlow {
    fn default() -> Self {
        Self { phase: Phase::Idle, last_seen: None }
    }
}

impl ApplyFlow {
    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    /// Shell'den sonuç beklenirken yeni istek gönderilmez, düzenleme kilitli.
    pub fn busy(&self) -> bool {
        matches!(self.phase, Phase::Sending | Phase::Awaiting { .. } | Phase::Deciding { .. } | Phase::Resolving { .. })
    }

    /// İsteği hazırlar (normalleştirilmiş tam durum, tek satır). Meşgulken ya da boyut sınırı
    /// aşılırsa hata; başarıda `Sending`.
    pub fn begin(&mut self, layout: &Layout) -> Result<String, ShellError> {
        if self.busy() {
            return Err(ShellError::local("pending_exists", "Önceki değişiklik henüz sonuçlanmadı"));
        }
        let Some(json) = layout.normalized().apply_json().filter(|j| j.len() <= MAX_APPLY_BYTES) else {
            let e = ShellError::local("too_large", "");
            self.phase = Phase::Failed(e.clone());
            return Err(e);
        };
        self.phase = Phase::Sending;
        Ok(json)
    }

    pub fn on_reply(&mut self, reply: Result<ApplyReply, ShellError>) {
        if self.phase != Phase::Sending {
            return;
        }
        self.phase = match reply {
            Ok(r) if r.confirm => Phase::Awaiting { token: r.token, remaining_ms: r.deadline_ms },
            Ok(_) => Phase::Done { outcome: Outcome::Applied, detail: None },
            // Cevap gelmedi ama shell isteği işlemiş olabilir: durumdan anlaşılır.
            Err(e) if e.code == "timeout" => Phase::Resolving { token: None },
            Err(e) => Phase::Failed(e),
        };
    }

    /// Koru: gönderilecek jeton (yalnızca onay beklerken).
    pub fn keep(&mut self) -> Option<String> {
        self.decide(true)
    }

    /// Geri al: gönderilecek jeton (yalnızca onay beklerken).
    pub fn revert(&mut self) -> Option<String> {
        self.decide(false)
    }

    fn decide(&mut self, keep: bool) -> Option<String> {
        let Phase::Awaiting { token, .. } = &self.phase else { return None };
        let token = token.clone();
        self.phase = Phase::Deciding { token: token.clone(), keep };
        Some(token)
    }

    /// `confirm`/`revert` cevabı. Hata çoğunlukla bekleyenin zaten bittiğidir (`stale`: süre
    /// doldu, hotplug): sonuç yine `status.last`'tan okunur. Shell kaybolduysa hata.
    pub fn on_decided(&mut self, reply: Result<(), ShellError>) {
        let Phase::Deciding { token, .. } = &self.phase else { return };
        self.phase = match reply {
            Err(e) if e.code == "not_running" => Phase::Failed(e),
            _ => Phase::Resolving { token: Some(token.clone()) },
        };
    }

    /// Yeni `status`: geri sayımı günceller, bekleyen bittiyse sonucu `last`'tan alır.
    pub fn on_status(&mut self, s: &Status) {
        if !self.busy() {
            self.last_seen = s.last.as_ref().map(|l| l.token.clone());
            return;
        }
        let ours = |t: &str| s.pending.as_ref().filter(|p| p.token == t);
        match &mut self.phase {
            Phase::Awaiting { token, remaining_ms } => match ours(token) {
                Some(p) => *remaining_ms = p.remaining_ms,
                None => {
                    self.phase = Phase::Resolving { token: Some(token.clone()) };
                    self.resolve(s);
                }
            },
            Phase::Resolving { token: Some(token) } => match ours(token) {
                // Karar shell'e ulaşmadı, değişiklik hâlâ bekliyor: düğmeler geri gelir.
                Some(p) => self.phase = Phase::Awaiting { token: token.clone(), remaining_ms: p.remaining_ms },
                None => self.resolve(s),
            },
            Phase::Resolving { token: None } => match &s.pending {
                Some(p) => self.phase = Phase::Awaiting { token: p.token.clone(), remaining_ms: p.remaining_ms },
                None => match &s.last {
                    Some(l) if Some(&l.token) != self.last_seen.as_ref() => self.finish(l),
                    _ => self.phase = Phase::Failed(ShellError::local("timeout", "")),
                },
            },
            Phase::Sending | Phase::Deciding { .. } | Phase::Idle | Phase::Done { .. } | Phase::Failed(_) => {}
        }
    }

    /// `Resolving { Some(jeton) }` ve bekleyen yok: `last` bizimse sonuç o; değilse (beklenmez:
    /// shell bekleyeni bitirirken `last`'ı aynı anda yazar) sonuç bilinmiyor.
    fn resolve(&mut self, s: &Status) {
        let Phase::Resolving { token: Some(token) } = &self.phase else { return };
        match &s.last {
            Some(l) if l.token == *token => self.finish(l),
            _ => self.phase = Phase::Done { outcome: Outcome::Other, detail: None },
        }
    }

    fn finish(&mut self, l: &Last) {
        self.last_seen = Some(l.token.clone());
        self.phase = Phase::Done { outcome: l.result, detail: l.detail.clone().filter(|d| !d.is_empty()) };
    }

    /// Shell'e ulaşılamaz oldu: bekleyen istek sonuçsuz kaldı (shell kendi zamanlayıcısıyla ya
    /// da Hyprland tarafındaki bekçiyle geri döner, ADR §5.3).
    pub fn on_lost(&mut self) {
        if self.busy() {
            self.phase = Phase::Failed(ShellError::local("not_running", ""));
        }
    }

    /// Sonuç gösterildi, kullanıcı yeniden düzenliyor.
    pub fn dismiss(&mut self) {
        if !self.busy() {
            self.phase = Phase::Idle;
        }
    }
}

// ---- Kullanıcı metinleri -----------------------------------------------------------------------

/// Shell hata mesajı `<id>: …` ile başlar (ADR §13.2); `name` id'yi kullanıcı adına çevirir.
fn display_name(e: &ShellError, name: &dyn Fn(&str) -> Option<String>) -> Option<String> {
    let (id, _) = e.message.split_once(": ")?;
    name(id)
}

/// `invalid_scale` mesajındaki `en yakın: <ölçek>` önerisi, yüzde olarak.
fn nearest_scale_hint(message: &str) -> Option<String> {
    let (_, s) = message.rsplit_once("en yakın:")?; // i18n: tanı shell hata iletisinin (çevrilmez) protokol işareti
    let v: f64 = s.trim().trim_end_matches(')').trim().replace(',', ".").parse().ok()?;
    (v.is_finite() && v > 0.0).then(|| crate::numfmt::percent(v, 0))
}

/// Shell (ya da yerel) hatasının kısa kullanıcı metni (ADR-0005 §13.2 kod listesi).
pub fn error_text(e: &ShellError, name: &dyn Fn(&str) -> Option<String>) -> String {
    let who = display_name(e, name);
    let subject = |generic: &str| who.clone().unwrap_or_else(|| generic.to_owned());
    match e.code.as_str() {
        "stale" => t!("settings-displays-error-stale"),
        "too_large" => t!("settings-displays-error-too-large"),
        "bad_json" | "bad_reply" => t!("settings-displays-error-protocol"),
        "unknown_display" => t!("settings-displays-error-unknown-display", display = subject(&t!("settings-displays-error-subject-indefinite"))),
        "missing_display" => t!("settings-displays-error-missing-display"),
        "invalid_mode" => t!("settings-displays-error-invalid-mode", display = subject(&t!("settings-displays-error-subject-definite"))),
        "invalid_scale" => match nearest_scale_hint(&e.message) {
            Some(n) => t!("settings-displays-error-invalid-scale-hint", display = subject(&t!("settings-displays-error-subject-definite")), scale = n),
            None => t!("settings-displays-error-invalid-scale", display = subject(&t!("settings-displays-error-subject-definite"))),
        },
        "invalid_position" => t!("settings-displays-error-invalid-position", display = subject(&t!("settings-displays-error-subject-definite"))),
        "overlap" => t!("settings-displays-error-overlap"),
        "disconnected_layout" => t!("settings-displays-issue-disconnected"),
        "no_active_display" => t!("settings-displays-error-no-active-display"),
        "mirror_chain" => t!("settings-displays-error-mirror-chain"),
        "invalid_primary" => t!("settings-displays-error-invalid-primary"),
        "pending_exists" => t!("settings-displays-error-pending-exists"),
        "not_applied" => t!("settings-displays-result-not-applied"),
        "unavailable" => t!("settings-displays-error-unavailable"),
        "locked" => t!("settings-displays-error-locked"),
        "not_running" => t!("settings-displays-error-not-running"),
        "timeout" => t!("settings-displays-error-timeout"),
        "ipc" => t!("settings-displays-error-ipc"),
        _ if !e.message.is_empty() => e.message.clone(),
        "" => t!("settings-displays-error-unknown"),
        code => t!("settings-displays-error-code", code = code),
    }
}

/// `status.last` sonucunun kısa kullanıcı metni; shell'in ek açıklaması varsa sona eklenir.
pub fn outcome_text(outcome: Outcome, detail: Option<&str>) -> String {
    let base = match outcome {
        Outcome::Applied => t!("settings-displays-result-applied"),
        Outcome::Confirmed => t!("settings-displays-result-confirmed"),
        Outcome::Reverted => t!("settings-displays-result-reverted"),
        Outcome::Timeout => t!("settings-displays-result-timeout"),
        Outcome::Hotplug => t!("settings-displays-result-hotplug"),
        Outcome::NotApplied => t!("settings-displays-result-not-applied"),
        Outcome::NotShown => t!("settings-displays-result-not-shown"),
        Outcome::Reloaded => t!("settings-displays-result-reloaded"),
        Outcome::Other => t!("settings-displays-result-other"),
    };
    match detail {
        Some(d) if !d.is_empty() => t!("settings-displays-result-detail", text = base, detail = d),
        _ => base,
    }
}

/// Geri sayım metni (Koru/Geri al beklenirken).
pub fn countdown_text(remaining_ms: u64) -> String {
    match remaining_ms.div_ceil(1000) {
        0 => t!("settings-displays-status-awaiting"),
        s => t!("settings-displays-status-awaiting-countdown", seconds = s),
    }
}

// ---- Gerçek ve sahte bağlantı -------------------------------------------------------------------

/// Etkin shell IPC adı. Geçersiz örnek adı hatadır (sessizce kullanıcının shell'ine düşülmez).
pub fn shell_app_name(instance: Option<&str>) -> Result<String, String> {
    match instance {
        None | Some("") => Ok("nuxut-shell".into()),
        Some(s) if s.len() <= 16 && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-') => Ok(format!("nuxut-shell-{s}")),
        Some(_) => Err(t!("settings-displays-error-instance-name", env = INSTANCE_ENV)),
    }
}

struct IpcLink {
    app: String,
}

impl ShellLink for IpcLink {
    fn request(&mut self, args: &[&str]) -> Result<String, LinkError> {
        match nuxut::ipc::send_args(&self.app, args) {
            Ok(r) => Ok(r),
            Err(nuxut::ipc::IpcError::NotRunning) => Err(LinkError::NotRunning),
            Err(nuxut::ipc::IpcError::Timeout) => Err(LinkError::Timeout),
            Err(e) => Err(LinkError::Failed(e.to_string())),
        }
    }
}

/// Bağlantı kurulamadı (geçersiz örnek adı): her istek aynı hatayı verir.
struct BrokenLink(String);

impl ShellLink for BrokenLink {
    fn request(&mut self, _args: &[&str]) -> Result<String, LinkError> {
        Err(LinkError::Failed(self.0.clone()))
    }
}

/// Canlı görsel test için shell taklidi (sözleşmedeki davranış): `apply` değişikliği hemen
/// listeye yansıtır ve [`FAKE_CONFIRM`] boyunca onay bekler; `confirm` kalıcılaştırır,
/// `revert` ya da süre dolması önceki listeye döner; sonuç `status.last`'ta. Yalnızca
/// [`FAKE_ENV`] ile; ağ/IPC yok.
struct FakeShell {
    list: serde_json::Value,
    /// (jeton, önceki liste, başlangıç).
    pending: Option<(String, serde_json::Value, Instant)>,
    last: serde_json::Value,
    seq: u32,
}

const FAKE_CONFIRM: Duration = Duration::from_secs(15);

impl FakeShell {
    fn new(json: &str) -> Result<Self, String> {
        let list: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        Ok(Self { list, pending: None, last: serde_json::Value::Null, seq: 0 })
    }

    fn generation(&self) -> u64 {
        self.list["gen"].as_u64().unwrap_or(0)
    }

    fn bump(&mut self) {
        self.list["gen"] = (self.generation() + 1).into();
    }

    fn finish(&mut self, result: &str, restore: bool) {
        let Some((token, previous, _)) = self.pending.take() else { return };
        if restore {
            let g = self.generation();
            self.list = previous;
            self.list["gen"] = g.into();
        }
        self.bump();
        self.last = serde_json::json!({"token": token, "result": result});
    }

    fn settle(&mut self) {
        if self.pending.as_ref().is_some_and(|(_, _, at)| at.elapsed() >= FAKE_CONFIRM) {
            self.finish("timeout", true);
        }
    }

    fn commit(&mut self, req: &serde_json::Value) {
        self.list["primary"] = req["primary"].clone();
        let Some(displays) = self.list["displays"].as_array_mut() else { return };
        for d in displays {
            let Some(e) = req["displays"].as_array().and_then(|a| a.iter().find(|e| e["id"] == d["id"])) else { continue };
            for k in ["enabled", "mode", "pos", "transform", "vrr", "mirror_of"] {
                d[k] = e[k].clone();
            }
            d["scale_120"] = e["scale_120"].clone();
            d["scale"] = (e["scale_120"].as_f64().unwrap_or(120.0) / 120.0).into();
        }
        self.bump();
    }

    fn pending_json(&self) -> serde_json::Value {
        match &self.pending {
            Some((token, _, at)) => {
                serde_json::json!({"token": token, "remaining_ms": FAKE_CONFIRM.saturating_sub(at.elapsed()).as_millis() as u64, "confirm": true})
            }
            None => serde_json::Value::Null,
        }
    }
}

impl ShellLink for FakeShell {
    fn request(&mut self, args: &[&str]) -> Result<String, LinkError> {
        self.settle();
        Ok(match args {
            ["displays", "status"] => {
                serde_json::json!({"v": 1, "gen": self.generation(), "provider": "lua", "available": true, "pending": self.pending_json(), "last": self.last}).to_string()
            }
            ["displays", "list"] => {
                let mut l = self.list.clone();
                l["pending"] = self.pending_json();
                l["available"] = true.into();
                l.to_string()
            }
            ["displays", "apply", json] => {
                let Ok(req) = serde_json::from_str::<serde_json::Value>(json) else { return Ok("error: bad_json: okunamadı".into()) }; // i18n: tanı sahte shell, shell tanı iletisini taklit eder
                if self.pending.is_some() {
                    return Ok("error: pending_exists: önceki değişiklik onay bekliyor".into()); // i18n: tanı sahte shell tanısı
                }
                if req["base_gen"].as_u64() != Some(self.generation()) {
                    return Ok(format!("error: stale: güncel gen {}", self.generation())); // i18n: tanı sahte shell tanısı
                }
                self.seq += 1;
                let token = format!("demo{:04x}", self.seq);
                self.pending = Some((token.clone(), self.list.clone(), Instant::now()));
                self.commit(&req);
                serde_json::json!({"token": token, "confirm": true, "deadline_ms": FAKE_CONFIRM.as_millis() as u64}).to_string()
            }
            ["displays", verb @ ("confirm" | "revert"), token] => {
                if self.pending.as_ref().is_none_or(|(t, _, _)| t != token) {
                    return Ok("error: stale: onay bekleyen değişiklik yok".into()); // i18n: tanı sahte shell tanısı
                }
                if *verb == "confirm" { self.finish("confirmed", false) } else { self.finish("reverted", true) }
                "ok".into()
            }
            _ => "error: kullanım: displays <list|status> | displays apply <JSON> | displays <confirm|revert> <jeton>".into(), // i18n: tanı sahte shell tanısı
        })
    }
}

fn make_link() -> Box<dyn ShellLink> {
    if let Some(path) = std::env::var_os(FAKE_ENV) {
        return match std::fs::read_to_string(&path).map_err(|e| e.to_string()).and_then(|s| FakeShell::new(&s)) {
            Ok(f) => {
                tracing::info!("Ekranlar: sahte shell ({FAKE_ENV})");
                Box::new(f)
            }
            Err(e) => Box::new(BrokenLink(format!("{FAKE_ENV} okunamadı: {e}"))), // i18n: tanı geliştirici sahte shell dosyası
        };
    }
    match shell_app_name(std::env::var(INSTANCE_ENV).ok().as_deref()) {
        Ok(app) => Box::new(IpcLink { app }),
        Err(e) => Box::new(BrokenLink(e)),
    }
}

// ---- Arka plan thread'i ------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Command {
    /// Sayfa görünür oldu / ayrıldı: yoklama başlar / durur.
    Watch(bool),
    /// Listeyi hemen yeniden al.
    Refresh,
    Apply(String),
    /// Koru (`displays confirm <jeton>`).
    Confirm(String),
    /// Geri al (`displays revert <jeton>`).
    Revert(String),
}

static WORKER: OnceLock<Mutex<mpsc::Sender<Command>>> = OnceLock::new();
static WATCHING: AtomicBool = AtomicBool::new(false);

pub fn send(cmd: Command) {
    let tx = WORKER.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        if let Err(e) = std::thread::Builder::new().name("displays".into()).spawn(move || worker(rx, Poller::new(make_link()))) {
            tracing::warn!("ekranlar thread'i açılamadı: {e}");
        }
        Mutex::new(tx)
    });
    let _ = tx.lock().unwrap_or_else(|e| e.into_inner()).send(cmd);
}

/// Sayfa görünürlüğü (her karede çağrılabilir; yalnızca değişince komut gider).
pub fn watch(on: bool) {
    if WATCHING.swap(on, Ordering::Relaxed) != on {
        send(Command::Watch(on));
    }
}

fn worker(rx: mpsc::Receiver<Command>, mut poller: Poller) {
    let mut watching = false;
    loop {
        let cmd = if watching {
            match rx.recv_timeout(POLL) {
                Ok(c) => Some(c),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }
        } else {
            match rx.recv() {
                Ok(c) => Some(c),
                Err(_) => return,
            }
        };
        let updates = match cmd {
            None => poller.tick(false),
            Some(Command::Watch(on)) => {
                watching = on;
                if on { poller.tick(true) } else { Vec::new() }
            }
            Some(Command::Refresh) => poller.tick(true),
            Some(Command::Apply(json)) => poller.apply(&json),
            Some(Command::Confirm(token)) => poller.decide(true, &token),
            Some(Command::Revert(token)) => poller.decide(false, &token),
        };
        for u in updates {
            crate::services::emit(crate::services::Event::Displays(u));
        }
    }
}

#[cfg(test)]
mod tests;
