//! Canlı testler için sahte `org.freedesktop.timedate1` (systemd-timedated) — YALNIZCA oturum
//! bus'ında (`dbus-run-session` içinde). Gerçek sistem saatine ve sistem bus'ına hiç dokunmaz:
//! özellikleri bellekte tutar, `Set*` çağrısında günceller ve `PropertiesChanged` yayar.
//! Ayarlar `NUXUT_TIMEDATE_BUS=session` ile buna bağlanır.
//!
//! Ortam:
//! - `NUXUT_FAKE_TIMEDATE_NTP=0` → otomatik saat kapalı başlar (varsayılan açık, eşitlenmiş).
//! - `NUXUT_FAKE_TIMEDATE_DENY=1` → her `Set*` `org.freedesktop.PolicyKit1.Error.NotAuthorized` döner.
//! - `NUXUT_FAKE_TIMEDATE_DELAY_MS=<n>` → `Set*` yanıtı n ms gecikir ("yetki bekleniyor" görünür).
//! - `NUXUT_FAKE_TIMEDATE_LOCALRTC=1` → donanım saati yerel saatte.
//! - `NUXUT_FAKE_TIMEDATE_NO_NTP=1` → ağ saati hizmeti yok (`CanNTP=false`).
//!
//! `dbus-run-session -- cargo run --example fake_timedated`

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use zbus::object_server::SignalEmitter;

fn env_on(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| v == "1")
}

#[derive(Debug, zbus::DBusError)]
#[zbus(prefix = "org.freedesktop.PolicyKit1.Error")]
enum PkError {
    #[zbus(error)]
    ZBus(zbus::Error),
    NotAuthorized(String),
}

#[derive(Debug)]
struct Fake {
    timezone: String,
    local_rtc: bool,
    can_ntp: bool,
    ntp: bool,
    /// Elle ayarlanan saatin gerçek saate farkı (µs).
    skew: i64,
    deny: bool,
    delay: Duration,
}

impl Fake {
    async fn gate(&self) -> Result<(), PkError> {
        if !self.delay.is_zero() {
            // Yetki diyaloğu bekleniyormuş gibi (sahte sunucu bu sürede başka yanıt vermez).
            std::thread::sleep(self.delay);
        }
        if self.deny {
            return Err(PkError::NotAuthorized("sahte: yetki verilmedi".into()));
        }
        Ok(())
    }

    fn now_usec(&self) -> u64 {
        let real = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_micros() as i64);
        (real + self.skew).max(0) as u64
    }
}

const ZONES: &[&str] = &[
    "Africa/Cairo",
    "America/Argentina/Buenos_Aires",
    "America/New_York",
    "America/Sao_Paulo",
    "Asia/Istanbul",
    "Asia/Kolkata",
    "Asia/Tokyo",
    "Australia/Sydney",
    "Europe/Berlin",
    "Europe/Istanbul",
    "Europe/Kyiv",
    "Europe/London",
    "Europe/Paris",
    "Europe/Zurich",
    "UTC",
];

#[zbus::interface(name = "org.freedesktop.timedate1")]
impl Fake {
    #[zbus(property, name = "Timezone")]
    fn timezone(&self) -> String {
        self.timezone.clone()
    }

    #[zbus(property, name = "LocalRTC")]
    fn local_rtc(&self) -> bool {
        self.local_rtc
    }

    #[zbus(property, name = "CanNTP")]
    fn can_ntp(&self) -> bool {
        self.can_ntp
    }

    #[zbus(property, name = "NTP")]
    fn ntp(&self) -> bool {
        self.ntp
    }

    #[zbus(property(emits_changed_signal = "false"), name = "NTPSynchronized")]
    fn ntp_synchronized(&self) -> bool {
        self.ntp
    }

    #[zbus(property(emits_changed_signal = "false"), name = "TimeUSec")]
    fn time_usec(&self) -> u64 {
        self.now_usec()
    }

    #[zbus(name = "ListTimezones")]
    fn list_timezones(&self) -> Vec<String> {
        ZONES.iter().map(|s| (*s).to_owned()).collect()
    }

    #[zbus(name = "SetNTP")]
    async fn set_ntp(&mut self, use_ntp: bool, _interactive: bool, #[zbus(signal_emitter)] e: SignalEmitter<'_>) -> Result<(), PkError> {
        self.gate().await?;
        if !self.can_ntp && use_ntp {
            return Err(PkError::ZBus(zbus::Error::Failure("NTP yok".into())));
        }
        self.ntp = use_ntp;
        self.n_t_p_changed(&e).await?;
        eprintln!("fake_timedated: NTP = {use_ntp}");
        Ok(())
    }

    #[zbus(name = "SetTimezone")]
    async fn set_timezone(&mut self, timezone: String, _interactive: bool, #[zbus(signal_emitter)] e: SignalEmitter<'_>) -> Result<(), PkError> {
        self.gate().await?;
        if !ZONES.contains(&timezone.as_str()) {
            return Err(PkError::ZBus(zbus::Error::Failure("geçersiz dilim".into())));
        }
        self.timezone = timezone;
        self.timezone_changed(&e).await?;
        eprintln!("fake_timedated: Timezone = {}", self.timezone);
        Ok(())
    }

    #[zbus(name = "SetTime")]
    async fn set_time(&mut self, usec: i64, relative: bool, _interactive: bool) -> Result<(), PkError> {
        self.gate().await?;
        if self.ntp {
            return Err(PkError::ZBus(zbus::Error::Failure("otomatik saat açık".into())));
        }
        let real = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_micros() as i64);
        self.skew = if relative { self.skew + usec } else { usec - real };
        eprintln!("fake_timedated: SetTime {usec} relative={relative}");
        Ok(())
    }

    #[zbus(name = "SetLocalRTC")]
    async fn set_local_rtc(&mut self, local_rtc: bool, _fix_system: bool, _interactive: bool, #[zbus(signal_emitter)] e: SignalEmitter<'_>) -> Result<(), PkError> {
        self.gate().await?;
        self.local_rtc = local_rtc;
        self.local_r_t_c_changed(&e).await?;
        Ok(())
    }
}

fn main() -> zbus::Result<()> {
    let fake = Fake {
        timezone: "Europe/Istanbul".into(),
        local_rtc: env_on("NUXUT_FAKE_TIMEDATE_LOCALRTC"),
        can_ntp: !env_on("NUXUT_FAKE_TIMEDATE_NO_NTP"),
        ntp: std::env::var("NUXUT_FAKE_TIMEDATE_NTP").map_or(true, |v| v != "0") && !env_on("NUXUT_FAKE_TIMEDATE_NO_NTP"),
        skew: 0,
        deny: env_on("NUXUT_FAKE_TIMEDATE_DENY"),
        delay: Duration::from_millis(std::env::var("NUXUT_FAKE_TIMEDATE_DELAY_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(0)),
    };
    // Yalnızca oturum bus'ı: gerçek timedated'in (sistem bus'ı) yerine asla geçmez.
    let _conn = zbus::blocking::connection::Builder::session()?.name("org.freedesktop.timedate1")?.serve_at("/org/freedesktop/timedate1", fake)?.build()?;
    eprintln!("fake_timedated: hazır (oturum bus'ı)");
    loop {
        std::thread::sleep(Duration::from_secs(3600));
    }
}
