//! Çalışan shell'e `msg` isteği (`nuxut-shell msg …` ile aynı tek örnek soketi). Yanıt
//! protokolü (shell `src/ipc.rs`): başarı `ok` ya da tek satır JSON, hata `error: …`.
//!
//! İstekler [`ShellLink`] trait'inin arkasındadır: birim testleri sahte bağlantı kullanır,
//! gerçek soket hiç açılmaz. Gerçek bağlantı ([`IpcLink`]) bloklar (varsayılan ≤ 2 s; uzun
//! işlemler için [`IpcLink::with_timeout`], ör. `snapshot` ≥ 15 s); yalnızca arka plan
//! thread'lerinden çağrılır.
//!
//! **Test örneği kapısı:** Ayarlar test adıyla çalışıyorsa (`NUXUT_SETTINGS_INSTANCE`) ve
//! hangi shell'e konuşacağı açıkça verilmemişse (`NUXUT_SHELL_INSTANCE`) hiçbir istek
//! gönderilmez — test örneği kullanıcının çalışan shell'ine asla ulaşmaz.

use nuxut::wayland::ipc;

/// Shell'in tek örnek IPC adı (shell `ipc::APP_NAME`).
pub const SHELL_APP: &str = "nuxut-shell";
/// Shell'in test/ikinci örnek adı ortam değişkeni (shell `ipc::INSTANCE_ENV`).
pub const SHELL_INSTANCE_ENV: &str = "NUXUT_SHELL_INSTANCE";
/// Ayarlar'ın kendi test örneği adı (bkz. `main.rs`).
pub const SETTINGS_INSTANCE_ENV: &str = "NUXUT_SETTINGS_INSTANCE";
/// Kabul edilen en uzun yanıt (bayt); daha uzunu sözleşme dışıdır.
pub const MAX_REPLY: usize = 256 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LinkError {
    /// Test örneği, shell örneği verilmemiş: istek bilerek gönderilmedi.
    #[error("test örneği: shell'e istek gönderilmedi")]
    TestInstance,
    #[error("geçersiz shell örnek adı")]
    InvalidInstance,
    /// Shell çalışmıyor ya da soket yok.
    #[error("shell çalışmıyor")]
    NotRunning,
    /// İstek gitti, yanıt süresinde gelmedi (ya da shell yanıt penceresi dolunca bağlantıyı
    /// boş kapattı). İş shell'de yapılmış olabilir: çağıran sonucu başka yoldan doğrular.
    #[error("yanıt zamanında gelmedi")]
    Timeout,
    /// Shell komutu tanımıyor (sürüm farkı) ya da hata döndürdü.
    #[error("{0}")]
    Shell(String),
    #[error("bağlantı hatası: {0}")]
    Io(String),
    /// Yanıt sözleşmeye uymuyor (JSON değil, çok uzun…).
    #[error("geçersiz yanıt")]
    BadReply,
}

/// Shell'e tek istek: argüman vektörü → yanıt satırı (`error: …` satırları `Err(Shell)` olur).
pub trait ShellLink: Send + Sync + std::fmt::Debug {
    fn request(&self, args: &[&str]) -> Result<String, LinkError>;
}

/// Hedef shell IPC adı. `shell_instance` boş değilse `nuxut-shell-<ad>` (shell ile aynı kural:
/// `[a-z0-9_-]`, en çok 16); yoksa ve Ayarlar test örneğiyse istek yok.
pub fn target_app(shell_instance: Option<&str>, settings_instance: Option<&str>) -> Result<String, LinkError> {
    match shell_instance.filter(|s| !s.is_empty()) {
        Some(s) if s.len() <= 16 && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-') => {
            Ok(format!("{SHELL_APP}-{s}"))
        }
        Some(_) => Err(LinkError::InvalidInstance),
        None if settings_instance.is_some_and(|s| !s.is_empty()) => Err(LinkError::TestInstance),
        None => Ok(SHELL_APP.to_owned()),
    }
}

/// Gerçek bağlantı: `$XDG_RUNTIME_DIR/nuxut/<ad>.sock`.
#[derive(Debug, Clone)]
pub struct IpcLink {
    app: Result<String, LinkError>,
    /// `None`: framework varsayılanı (`nuxut::ipc::TIMEOUT`, 2 s).
    timeout: Option<std::time::Duration>,
}

impl IpcLink {
    pub fn from_env() -> Self {
        // Birim testleri gerçek soket açmaz (kullanıcının shell'i çalışıyor olabilir).
        if cfg!(test) {
            return Self { app: Err(LinkError::TestInstance), timeout: None };
        }
        let shell = std::env::var(SHELL_INSTANCE_ENV).ok();
        let settings = std::env::var(SETTINGS_INSTANCE_ENV).ok();
        Self { app: target_app(shell.as_deref(), settings.as_deref()), timeout: None }
    }

    /// İstek başına bekleme süresi (framework `Client::with_timeout`; 10 ms..=30 s'ye kırpılır).
    /// Yanıtını shell'de ayrı thread'in verdiği uzun komutlar için (`snapshot …`, ADR-0001 §9.5).
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl ShellLink for IpcLink {
    fn request(&self, args: &[&str]) -> Result<String, LinkError> {
        let app = self.app.clone()?;
        let sent = ipc::Client::new(&app).and_then(|c| match self.timeout {
            Some(t) => c.with_timeout(t).send_args(args),
            None => c.send_args(args),
        });
        let reply = sent.map_err(|e| match e {
            ipc::IpcError::NotRunning => LinkError::NotRunning,
            ipc::IpcError::Timeout => LinkError::Timeout,
            e => LinkError::Io(e.to_string()),
        })?;
        check_reply(reply)
    }
}

/// Yanıt satırını sınıflandırır: `error: …` → `Err(Shell)`, aşırı uzun → `BadReply`, boş →
/// `Timeout` (shell yanıt penceresi dolunca yanıtı düşürür ve bağlantıyı boş kapatır; shell her
/// komuta en az `ok` döner).
pub fn check_reply(reply: String) -> Result<String, LinkError> {
    if reply.len() > MAX_REPLY {
        return Err(LinkError::BadReply);
    }
    if reply.trim().is_empty() {
        return Err(LinkError::Timeout);
    }
    match reply.strip_prefix("error:") {
        Some(msg) => Err(LinkError::Shell(msg.trim().chars().take(200).collect())),
        None => Ok(reply),
    }
}

/// Testlerde: istekleri kaydeder, sırayla hazır yanıtları döner.
#[cfg(test)]
#[derive(Debug, Default)]
pub struct FakeLink {
    pub requests: std::sync::Mutex<Vec<Vec<String>>>,
    pub replies: std::sync::Mutex<std::collections::VecDeque<Result<String, LinkError>>>,
}

#[cfg(test)]
impl FakeLink {
    pub fn with(replies: impl IntoIterator<Item = Result<String, LinkError>>) -> Self {
        Self { requests: Default::default(), replies: std::sync::Mutex::new(replies.into_iter().collect()) }
    }

    pub fn sent(&self) -> Vec<Vec<String>> {
        self.requests.lock().unwrap().clone()
    }
}

#[cfg(test)]
impl ShellLink for FakeLink {
    fn request(&self, args: &[&str]) -> Result<String, LinkError> {
        self.requests.lock().unwrap().push(args.iter().map(|s| (*s).to_owned()).collect());
        let r = self.replies.lock().unwrap().pop_front().unwrap_or(Err(LinkError::NotRunning))?;
        check_reply(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_never_reaches_user_shell() {
        assert_eq!(target_app(None, Some("test-look")), Err(LinkError::TestInstance));
        assert_eq!(target_app(Some(""), Some("test-look")), Err(LinkError::TestInstance));
        assert_eq!(target_app(Some("test-look"), Some("test-look")).as_deref(), Ok("nuxut-shell-test-look"));
        assert_eq!(target_app(None, None).as_deref(), Ok("nuxut-shell"));
        assert_eq!(target_app(None, Some("")).as_deref(), Ok("nuxut-shell"));
        assert_eq!(target_app(Some("Kötü ad"), None), Err(LinkError::InvalidInstance));
        assert_eq!(target_app(Some("a-very-long-instance-name"), None), Err(LinkError::InvalidInstance));
    }

    #[test]
    fn error_replies_become_errors() {
        assert_eq!(check_reply("ok".into()).as_deref(), Ok("ok"));
        assert_eq!(check_reply("error: bilinmeyen komut".into()), Err(LinkError::Shell("bilinmeyen komut".into())));
        assert_eq!(check_reply("x".repeat(MAX_REPLY + 1)), Err(LinkError::BadReply));
        // Shell yanıt penceresi doldu: bağlantı boş kapanır.
        assert_eq!(check_reply(String::new()), Err(LinkError::Timeout));
        assert_eq!(check_reply("\n".into()), Err(LinkError::Timeout));
    }
}
