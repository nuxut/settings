//! Canlı testler için sahte `org.freedesktop.Accounts` (AccountsService) — YALNIZCA oturum
//! bus'ında (`dbus-run-session` içinde). Gerçek hesaba ve sistem bus'ına hiç dokunmaz: tek bir
//! kullanıcıyı bellekte tutar, `SetRealName`/`SetIconFile`'da günceller (resmi kendi dizinine
//! kopyalar, gerçek AccountsService gibi) ve `Changed` + `PropertiesChanged` yayar.
//! Ayarlar `NUXUT_ACCOUNTS_BUS=session` ile buna bağlanır.
//!
//! Yazma çağrısı `ALLOW_INTERACTIVE_AUTHORIZATION` bayrağı olmadan gelirse
//! `org.freedesktop.Accounts.Error.PermissionDenied` döner (istemcinin bayrağı koyduğunu sınar).
//!
//! Ortam:
//! - `NUXUT_FAKE_ACCOUNTS_DIR=<dizin>` → resim kopyalarının dizini (zorunlu; mutlak yol).
//! - `NUXUT_FAKE_ACCOUNTS_NAME=<ad>` → başlangıç görünen adı (varsayılan "Deniz Yılmaz").
//! - `NUXUT_FAKE_ACCOUNTS_ICON=<yol>` → başlangıç resmi (kopyalanır).
//! - `NUXUT_FAKE_ACCOUNTS_ADMIN=1` → hesap türü yönetici.
//! - `NUXUT_FAKE_ACCOUNTS_DENY=1` → her yazma `PermissionDenied` döner (polkit reddi).
//! - `NUXUT_FAKE_ACCOUNTS_DELAY_MS=<n>` → yazma yanıtı n ms gecikir ("yetki bekleniyor").
//! - `NUXUT_FAKE_ACCOUNTS_LIFETIME_S=<n>` → n sn sonra kendiliğinden çıkar (varsayılan 600).
//!
//! `dbus-run-session -- cargo run --example fake_accounts`

use std::path::PathBuf;
use std::time::Duration;

use zbus::message::{Flags, Header};
use zbus::object_server::SignalEmitter;
use zbus::zvariant::OwnedObjectPath;

const USER_PATH: &str = "/org/freedesktop/Accounts/User1000";
const USER_NAME: &str = "deniz";
/// AccountsService'in resim sınırı.
const MAX_ICON: u64 = 1 << 20;

fn env_on(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| v == "1")
}

#[derive(Debug, zbus::DBusError)]
#[zbus(prefix = "org.freedesktop.Accounts.Error")]
enum AccError {
    #[zbus(error)]
    ZBus(zbus::Error),
    PermissionDenied(String),
    Failed(String),
    UserDoesNotExist(String),
}

#[derive(Debug)]
struct Accounts;

#[zbus::interface(name = "org.freedesktop.Accounts")]
impl Accounts {
    #[zbus(name = "FindUserById")]
    fn find_user_by_id(&self, _id: i64) -> OwnedObjectPath {
        OwnedObjectPath::try_from(USER_PATH).expect("geçerli yol")
    }

    #[zbus(name = "FindUserByName")]
    fn find_user_by_name(&self, name: String) -> Result<OwnedObjectPath, AccError> {
        if name != USER_NAME {
            return Err(AccError::UserDoesNotExist("sahte: yok".into()));
        }
        Ok(OwnedObjectPath::try_from(USER_PATH).expect("geçerli yol"))
    }
}

#[derive(Debug)]
struct User {
    real_name: String,
    icon_file: String,
    admin: bool,
    deny: bool,
    delay: Duration,
    dir: PathBuf,
}

impl User {
    fn gate(&self, hdr: &Header<'_>, method: &str) -> Result<(), AccError> {
        let interactive = hdr.primary().flags().contains(Flags::AllowInteractiveAuth);
        eprintln!("fake_accounts: {method} interactive={interactive}");
        if !self.delay.is_zero() {
            // Yetki diyaloğu bekleniyormuş gibi (sahte sunucu bu sürede başka yanıt vermez).
            std::thread::sleep(self.delay);
        }
        if !interactive || self.deny {
            return Err(AccError::PermissionDenied("sahte: yetki verilmedi".into()));
        }
        Ok(())
    }

    /// Gerçek AccountsService gibi: dosyayı kendi dizinine kopyalar.
    fn store_icon(&self, file: &str) -> Result<String, AccError> {
        if file.is_empty() {
            return Ok(String::new());
        }
        let m = std::fs::metadata(file).map_err(|e| AccError::Failed(format!("okunamadı: {e}")))?;
        if !m.is_file() || m.len() > MAX_ICON {
            return Err(AccError::Failed("dosya uygun değil".into()));
        }
        let dest = self.dir.join(USER_NAME);
        std::fs::copy(file, &dest).map_err(|e| AccError::Failed(format!("kopyalanamadı: {e}")))?;
        Ok(dest.to_string_lossy().into_owned())
    }
}

#[zbus::interface(name = "org.freedesktop.Accounts.User")]
impl User {
    #[zbus(property, name = "RealName")]
    fn real_name(&self) -> String {
        self.real_name.clone()
    }

    #[zbus(property, name = "UserName")]
    fn user_name(&self) -> String {
        USER_NAME.into()
    }

    #[zbus(property, name = "IconFile")]
    fn icon_file(&self) -> String {
        self.icon_file.clone()
    }

    #[zbus(property, name = "AccountType")]
    fn account_type(&self) -> i32 {
        i32::from(self.admin)
    }

    #[zbus(property, name = "Uid")]
    fn uid(&self) -> u64 {
        1000
    }

    #[zbus(name = "SetRealName")]
    async fn set_real_name(&mut self, name: String, #[zbus(header)] hdr: Header<'_>, #[zbus(signal_emitter)] e: SignalEmitter<'_>) -> Result<(), AccError> {
        self.gate(&hdr, "SetRealName")?;
        if name.contains(':') || name.contains('\n') {
            return Err(AccError::Failed("geçersiz ad".into()));
        }
        self.real_name = name;
        self.real_name_changed(&e).await?;
        Self::changed(&e).await?;
        eprintln!("fake_accounts: RealName değişti");
        Ok(())
    }

    #[zbus(name = "SetIconFile")]
    async fn set_icon_file(&mut self, file: String, #[zbus(header)] hdr: Header<'_>, #[zbus(signal_emitter)] e: SignalEmitter<'_>) -> Result<(), AccError> {
        self.gate(&hdr, "SetIconFile")?;
        self.icon_file = self.store_icon(&file)?;
        self.icon_file_changed(&e).await?;
        Self::changed(&e).await?;
        eprintln!("fake_accounts: IconFile değişti");
        Ok(())
    }

    #[zbus(signal, name = "Changed")]
    async fn changed(e: &SignalEmitter<'_>) -> zbus::Result<()>;
}

fn main() -> zbus::Result<()> {
    let dir = std::env::var_os("NUXUT_FAKE_ACCOUNTS_DIR").map(PathBuf::from).filter(|d| d.is_absolute()).expect("NUXUT_FAKE_ACCOUNTS_DIR (mutlak) gerekli");
    std::fs::create_dir_all(&dir).expect("resim dizini");
    let mut user = User {
        real_name: std::env::var("NUXUT_FAKE_ACCOUNTS_NAME").unwrap_or_else(|_| "Deniz Yılmaz".into()),
        icon_file: String::new(),
        admin: env_on("NUXUT_FAKE_ACCOUNTS_ADMIN"),
        deny: env_on("NUXUT_FAKE_ACCOUNTS_DENY"),
        delay: Duration::from_millis(std::env::var("NUXUT_FAKE_ACCOUNTS_DELAY_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(0)),
        dir,
    };
    if let Ok(icon) = std::env::var("NUXUT_FAKE_ACCOUNTS_ICON") {
        user.icon_file = user.store_icon(&icon).unwrap_or_default();
    }
    // Yalnızca oturum bus'ı: gerçek AccountsService'in (sistem bus'ı) yerine asla geçmez.
    let _conn = zbus::blocking::connection::Builder::session()?
        .name("org.freedesktop.Accounts")?
        .serve_at("/org/freedesktop/Accounts", Accounts)?
        .serve_at(USER_PATH, user)?
        .build()?;
    eprintln!("fake_accounts: hazır (oturum bus'ı)");
    // Sınırlı ömür: betik öldürmeyi unutsa da (ya da öldürülemese de) sahte servis kalıcı olmaz.
    let lifetime = Duration::from_secs(std::env::var("NUXUT_FAKE_ACCOUNTS_LIFETIME_S").ok().and_then(|v| v.parse().ok()).unwrap_or(600));
    std::thread::sleep(lifetime);
    eprintln!("fake_accounts: ömür doldu, çıkılıyor");
    Ok(())
}
