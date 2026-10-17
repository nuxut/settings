//! Tarih ve saat: `systemd-timedated` (`org.freedesktop.timedate1`, sistem bus'ı) istemcisi.
//!
//! Sistem ayarıdır (shell.toml/framework.toml değil): Ayarlar timedated'i DOĞRUDAN çağırır
//! (karar: 2026-10-13; `docs/design/datetime.md`).
//!
//! - Okuma: `Properties.GetAll` (Timezone, LocalRTC, CanNTP, NTP, NTPSynchronized, TimeUSec).
//! - Yazma: `SetNTP(b, true)`, `SetTimezone(s, true)`, `SetTime(usec, false, true)` — hepsi
//!   `interactive = true`: polkit ajanı (nuxut-polkit) yetki sorar. Yöntem zaman aşımı
//!   [`CALL_TIMEOUT`] (polkit diyaloğu uzun sürebilir); bekleme sırasında [`OpState::Waiting`].
//! - Çağrılar tek bir arka plan thread'inde (zbus blocking; UI beklemez); sonuç [`Event::Timedate`].
//! - Canlı: ayrı dinleyici thread'i `PropertiesChanged`'i (Timezone, NTP, LocalRTC) dinler ve
//!   yeniden okur. `NTPSynchronized`/`TimeUSec` sinyal vermez: sayfa açılınca ve pencere odağı
//!   dönünce [`recheck`].
//! - Bus seçimi enjekte edilebilir: `NUXUT_TIMEDATE_BUS=session` yalnızca test/örnek içindir
//!   (`examples/fake_timedated.rs`); gerçek sistem saatine testte ASLA yazılmaz.
//! - Hata eşleme ([`classify_name`]): polkit reddi/iptali → sessiz (satır önceki değere döner;
//!   karar 2026-10-13); servis yok →
//!   [`TimedateState::Missing`]; diğerleri genel hata. Servisin hata ayrıntısı kullanıcıya
//!   gösterilmez (yalnızca log).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use zbus::zvariant::OwnedValue;

use super::{Event, emit};

pub const DEST: &str = "org.freedesktop.timedate1";
pub const PATH: &str = "/org/freedesktop/timedate1";
pub const IFACE: &str = "org.freedesktop.timedate1";
/// Bus seçimi (yalnızca test/örnek): `session` → oturum bus'ı.
pub const BUS_ENV: &str = "NUXUT_TIMEDATE_BUS";
/// Yazma çağrılarının zaman aşımı: polkit diyaloğu kullanıcıyı bekler (≥ 120 sn kararı).
pub const CALL_TIMEOUT: Duration = Duration::from_secs(300);
/// Saat dilimi adının üst sınırı (IANA adları ~30 bayt).
const MAX_ZONE_LEN: usize = 64;
/// Kabul edilen en çok saat dilimi (gerçek liste ~600).
const MAX_ZONES: usize = 2048;

/// timedated özellikleri.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Props {
    /// IANA adı (`Europe/Istanbul`); boş = bilinmiyor.
    pub timezone: String,
    /// Donanım saati yerel saatte mi.
    pub local_rtc: bool,
    /// Ağ saati hizmeti (systemd-timesyncd vb.) var mı.
    pub can_ntp: bool,
    /// Otomatik saat açık mı.
    pub ntp: bool,
    /// Saat eşitlendi mi.
    pub ntp_synced: bool,
    /// Sistem saati (µs, UNIX). Okunduğu an; sinyal vermez.
    pub time_usec: u64,
}

/// Servisin durumu.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TimedateState {
    /// Henüz okunmadı.
    #[default]
    Pending,
    Ready(Props),
    /// systemd-timedated (ya da bus) yok: sayfa notu, seçenekler kapalı.
    Missing,
    /// Okuma başka bir nedenle başarısız.
    Failed,
}

impl TimedateState {
    pub fn props(&self) -> Option<&Props> {
        match self {
            TimedateState::Ready(p) => Some(p),
            _ => None,
        }
    }
}

/// Yazma işlemi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Ntp,
    Timezone,
    Time,
}

/// Yazma hatasının kullanıcıya gösterilen sınıfı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// polkit reddetti ya da diyalog iptal edildi (gösterilmez; işlem `Idle`'a döner).
    NotAuthorized,
    /// timedated yok.
    Missing,
    /// Diğer (ayrıntı yalnızca logda).
    Other,
}

/// Son yazma işleminin durumu.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OpState {
    #[default]
    Idle,
    /// İstek gönderildi; yanıt (polkit diyaloğu) bekleniyor.
    Waiting(Op),
    Failed(Op, ErrorKind),
}

impl OpState {
    /// Bu işlem için bekleniyor mu.
    pub fn waiting(&self, op: Op) -> bool {
        *self == OpState::Waiting(op)
    }

    /// Bu işlemin hatası.
    pub fn error(&self, op: Op) -> Option<ErrorKind> {
        match self {
            OpState::Failed(o, k) if *o == op => Some(*k),
            _ => None,
        }
    }

    pub fn busy(&self) -> bool {
        matches!(self, OpState::Waiting(_))
    }
}

/// UI'nın gördüğü anlık görüntü.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Snapshot {
    pub state: TimedateState,
    pub op: OpState,
    /// `ListTimezones` (bir kez okunur); `None` = henüz yok ya da okunamadı.
    pub zones: Option<Arc<Vec<String>>>,
    /// Liste okunamadı (seçicide not).
    pub zones_failed: bool,
}

/// D-Bus hata adını sınıflar (saf; testli).
pub fn classify_name(name: &str) -> ErrorKind {
    const DENIED: [&str; 5] = [
        "org.freedesktop.PolicyKit1.Error.NotAuthorized",
        "org.freedesktop.PolicyKit1.Error.Cancelled",
        "org.freedesktop.DBus.Error.AccessDenied",
        "org.freedesktop.DBus.Error.AuthFailed",
        "org.freedesktop.DBus.Error.InteractiveAuthorizationRequired",
    ];
    const MISSING: [&str; 4] = [
        "org.freedesktop.DBus.Error.ServiceUnknown",
        "org.freedesktop.DBus.Error.NameHasNoOwner",
        "org.freedesktop.DBus.Error.UnknownObject",
        "org.freedesktop.DBus.Error.UnknownInterface",
    ];
    if DENIED.contains(&name) {
        ErrorKind::NotAuthorized
    } else if MISSING.contains(&name) {
        ErrorKind::Missing
    } else {
        ErrorKind::Other
    }
}

/// zbus hatasını sınıflar: yöntem hatası adıyla; bağlantı kurulamadıysa servis yok sayılır.
fn classify(err: &zbus::Error) -> ErrorKind {
    use zbus::DBusError as _;
    match err {
        zbus::Error::MethodError(name, _, _) => classify_name(name.as_str()),
        zbus::Error::FDO(e) => classify_name(e.name().as_str()),
        zbus::Error::Address(_) | zbus::Error::Handshake(_) => ErrorKind::Missing,
        zbus::Error::InputOutput(e) if e.kind() == std::io::ErrorKind::NotFound || e.kind() == std::io::ErrorKind::ConnectionRefused => ErrorKind::Missing,
        _ => ErrorKind::Other,
    }
}

/// Yazılabilir saat dilimi adı: IANA biçimi (`Bölge/Şehir`, `UTC`), göreli yol yok.
pub fn valid_zone(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= MAX_ZONE_LEN
        && !s.starts_with('/')
        && !s.ends_with('/')
        && s.split('/').all(|p| !p.is_empty() && p != "." && p != "..")
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'_' | b'-' | b'+'))
}

/// `GetAll` yanıtından özellikler (tipi uymayan alan varsayılanda kalır; ad doğrulanır).
pub fn parse_props(map: &HashMap<String, OwnedValue>) -> Props {
    let b = |k: &str| map.get(k).and_then(|v| bool::try_from(v.try_clone().ok()?).ok()).unwrap_or(false);
    let timezone = map.get("Timezone").and_then(|v| String::try_from(v.try_clone().ok()?).ok()).filter(|z| valid_zone(z)).unwrap_or_default();
    let time_usec = map.get("TimeUSec").and_then(|v| u64::try_from(v.try_clone().ok()?).ok()).unwrap_or(0);
    Props { timezone, local_rtc: b("LocalRTC"), can_ntp: b("CanNTP"), ntp: b("NTP"), ntp_synced: b("NTPSynchronized"), time_usec }
}

/// `ListTimezones` sonucunu süzer: geçerli adlar, sıralı, tekrarsız, sınırlı.
pub fn clean_zones(mut zones: Vec<String>) -> Vec<String> {
    zones.retain(|z| valid_zone(z));
    zones.sort();
    zones.dedup();
    zones.truncate(MAX_ZONES);
    zones
}

// ---- Süreç geneli durum ve thread'ler ----------------------------------------------------------

#[derive(Debug)]
enum Command {
    Refresh,
    Zones,
    SetNtp(bool),
    SetTimezone(String),
    SetTime(i64),
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
    emit(Event::Timedate);
}

fn send(cmd: Command) {
    // Birim testleri sistem bus'ına gitmez (ve thread açmaz).
    if cfg!(test) {
        return;
    }
    let sender = SENDER.get_or_init(|| {
        let (tx, rx) = channel();
        let _ = std::thread::Builder::new().name("timedate".into()).spawn(move || worker(rx));
        let _ = std::thread::Builder::new().name("timedate-signals".into()).spawn(listen);
        Mutex::new(tx)
    });
    STARTED.store(true, Ordering::Relaxed);
    let _ = sender.lock().unwrap_or_else(|e| e.into_inner()).send(cmd);
}

/// Sayfa ilk kurulurken: servisi başlatır ve bir kez okur (sonraki bloklar yeniden okutmaz).
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

/// Saat dilimi listesi (bir kez; seçici açılınca).
pub fn want_zones() {
    let s = snapshot();
    if s.zones.is_none() {
        send(Command::Zones);
    }
}

pub fn set_ntp(on: bool) {
    begin(Op::Ntp, Command::SetNtp(on));
}

pub fn set_timezone(zone: String) {
    if valid_zone(&zone) {
        begin(Op::Timezone, Command::SetTimezone(zone));
    }
}

/// `usec`: UNIX µs (UTC).
pub fn set_time(usec: i64) {
    begin(Op::Time, Command::SetTime(usec));
}

fn begin(op: Op, cmd: Command) {
    if cfg!(test) {
        return;
    }
    update(|s| s.op = OpState::Waiting(op));
    send(cmd);
}

fn session_bus() -> bool {
    std::env::var(BUS_ENV).is_ok_and(|v| v == "session")
}

fn connect() -> zbus::Result<zbus::blocking::Connection> {
    let builder = if session_bus() { zbus::blocking::connection::Builder::session()? } else { zbus::blocking::connection::Builder::system()? };
    builder.method_timeout(CALL_TIMEOUT).build()
}

fn read(conn: &zbus::blocking::Connection) -> zbus::Result<Props> {
    let reply = conn.call_method(Some(DEST), PATH, Some("org.freedesktop.DBus.Properties"), "GetAll", &(IFACE,))?;
    let map: HashMap<String, OwnedValue> = reply.body().deserialize()?;
    Ok(parse_props(&map))
}

fn worker(rx: Receiver<Command>) {
    let mut conn: Option<zbus::blocking::Connection> = None;
    while let Ok(cmd) = rx.recv() {
        if conn.is_none() {
            match connect() {
                Ok(c) => conn = Some(c),
                Err(e) => {
                    tracing::info!("timedate: bus'a bağlanılamadı: {e}");
                    let missing = classify(&e) == ErrorKind::Missing || matches!(e, zbus::Error::InputOutput(_));
                    finish_without_bus(&cmd, missing);
                    continue;
                }
            }
        }
        let Some(c) = conn.as_ref() else { continue };
        match cmd {
            Command::Refresh => refresh_with(c),
            Command::Zones => {
                let zones = c.call_method(Some(DEST), PATH, Some(IFACE), "ListTimezones", &()).and_then(|m| m.body().deserialize::<Vec<String>>());
                match zones {
                    Ok(z) => update(|s| {
                        s.zones = Some(Arc::new(clean_zones(z)));
                        s.zones_failed = false;
                    }),
                    Err(e) => {
                        tracing::warn!("timedate: ListTimezones: {e}");
                        update(|s| s.zones_failed = true);
                    }
                }
            }
            Command::SetNtp(on) => write(c, Op::Ntp, "SetNTP", &(on, true)),
            Command::SetTimezone(z) => write(c, Op::Timezone, "SetTimezone", &(z.as_str(), true)),
            Command::SetTime(usec) => write(c, Op::Time, "SetTime", &(usec, false, true)),
        }
    }
}

fn finish_without_bus(cmd: &Command, missing: bool) {
    let kind = if missing { ErrorKind::Missing } else { ErrorKind::Other };
    match cmd {
        Command::Refresh => update(|s| s.state = if missing { TimedateState::Missing } else { TimedateState::Failed }),
        Command::Zones => update(|s| s.zones_failed = true),
        Command::SetNtp(_) => update(|s| s.op = OpState::Failed(Op::Ntp, kind)),
        Command::SetTimezone(_) => update(|s| s.op = OpState::Failed(Op::Timezone, kind)),
        Command::SetTime(_) => update(|s| s.op = OpState::Failed(Op::Time, kind)),
    }
}

fn refresh_with(conn: &zbus::blocking::Connection) {
    let state = match read(conn) {
        Ok(p) => TimedateState::Ready(p),
        Err(e) => {
            let kind = classify(&e);
            tracing::info!("timedate: okunamadı ({kind:?}): {e}");
            if kind == ErrorKind::Missing { TimedateState::Missing } else { TimedateState::Failed }
        }
    };
    update(|s| s.state = state);
}

fn write<B>(conn: &zbus::blocking::Connection, op: Op, method: &str, body: &B)
where
    B: serde::Serialize + zbus::zvariant::DynamicType,
{
    let result = conn.call_method(Some(DEST), PATH, Some(IFACE), method, body);
    match result {
        Ok(_) => {
            update(|s| s.op = OpState::Idle);
        }
        Err(e) => {
            let kind = classify(&e);
            // Ayrıntı kullanıcıya gösterilmez.
            tracing::warn!("timedate: {method} başarısız ({kind:?}): {e}");
            update(|s| {
                // polkit iptali ve reddi ayırt edilemez: hata gösterilmez, satır önceki değere
                // döner (karar, 2026-10-13).
                s.op = if kind == ErrorKind::NotAuthorized { OpState::Idle } else { OpState::Failed(op, kind) };
                if kind == ErrorKind::Missing {
                    s.state = TimedateState::Missing;
                }
            });
        }
    }
    refresh_with(conn);
}

/// `PropertiesChanged` dinleyicisi: timedate1 özelliği değişince yeniden okutur. timedated
/// isteğe bağlı etkinleşir ve boşta çıkar; eşleşme gönderen adına değil yola ve gövdedeki
/// arayüz adına bakar (sahip değişse de çalışır).
fn listen() {
    let run = || -> zbus::Result<()> {
        let conn = if session_bus() { zbus::blocking::Connection::session()? } else { zbus::blocking::Connection::system()? };
        let rule = zbus::MatchRule::builder()
            .msg_type(zbus::message::Type::Signal)
            .interface("org.freedesktop.DBus.Properties")?
            .member("PropertiesChanged")?
            .path(PATH)?
            .build();
        let it = zbus::blocking::MessageIterator::for_match_rule(rule, &conn, Some(16))?;
        for msg in it {
            let Ok(msg) = msg else { continue };
            let iface: zbus::Result<(String, HashMap<String, OwnedValue>, Vec<String>)> = msg.body().deserialize();
            if iface.is_ok_and(|(i, _, _)| i == IFACE)
                && let Some(s) = SENDER.get()
            {
                let _ = s.lock().unwrap_or_else(|e| e.into_inner()).send(Command::Refresh);
            }
        }
        Ok(())
    };
    if let Err(e) = run() {
        tracing::info!("timedate: sinyal dinlenemiyor: {e}");
    }
}

/// Sistemin yerel saat diliminin `unix` anındaki ofseti (yerel − UTC, sn): `chrono::Local`
/// (`TZ` / `/etc/localtime`; timedated dilimi değiştirince chrono önbelleği yenilenir).
pub fn local_offset(unix: i64) -> Option<i64> {
    use chrono::{Offset as _, TimeZone as _};
    let utc = chrono::DateTime::from_timestamp(unix, 0)?;
    Some(i64::from(chrono::Local.offset_from_utc_datetime(&utc.naive_utc()).fix().local_minus_utc()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::Value;

    #[test]
    fn error_names_map_to_user_classes() {
        assert_eq!(classify_name("org.freedesktop.PolicyKit1.Error.NotAuthorized"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.AccessDenied"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.PolicyKit1.Error.Cancelled"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.ServiceUnknown"), ErrorKind::Missing);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.NameHasNoOwner"), ErrorKind::Missing);
        assert_eq!(classify_name("org.freedesktop.timedate1.AutomaticTimeSyncEnabled"), ErrorKind::Other);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.InvalidArgs"), ErrorKind::Other);
        assert_eq!(classify_name("org.freedesktop.DBus.Error.InteractiveAuthorizationRequired"), ErrorKind::NotAuthorized);
        assert_eq!(classify_name("org.freedesktop.PolicyKit1.Error.Failed"), ErrorKind::Other);
    }

    #[test]
    fn zone_names_are_validated() {
        for ok in ["Europe/Istanbul", "UTC", "America/Argentina/Buenos_Aires", "Etc/GMT+3", "America/Port-au-Prince"] {
            assert!(valid_zone(ok), "{ok}");
        }
        for bad in ["", "/etc/passwd", "../x", "Europe/../x", "Europe//X", "Europe/", "a b", "x;y", &"A".repeat(65)] {
            assert!(!valid_zone(bad), "{bad}");
        }
        let z = clean_zones(vec!["UTC".into(), "Europe/Istanbul".into(), "../bad".into(), "UTC".into()]);
        assert_eq!(z, ["Europe/Istanbul", "UTC"]);
    }

    #[test]
    fn props_parse_with_wrong_types_ignored() {
        let mut m: HashMap<String, OwnedValue> = HashMap::new();
        let ov = |v: Value| OwnedValue::try_from(v).unwrap();
        m.insert("Timezone".into(), ov(Value::from("Europe/Istanbul")));
        m.insert("NTP".into(), ov(Value::from(true)));
        m.insert("CanNTP".into(), ov(Value::from(true)));
        m.insert("NTPSynchronized".into(), ov(Value::from("evet")));
        m.insert("LocalRTC".into(), ov(Value::from(false)));
        m.insert("TimeUSec".into(), ov(Value::from(1_791_115_200_000_000u64)));
        let p = parse_props(&m);
        assert_eq!(p, Props { timezone: "Europe/Istanbul".into(), local_rtc: false, can_ntp: true, ntp: true, ntp_synced: false, time_usec: 1_791_115_200_000_000 });
        m.insert("Timezone".into(), ov(Value::from("../../etc/shadow")));
        assert_eq!(parse_props(&m).timezone, "", "geçersiz ad alınmaz");
    }

    #[test]
    fn local_offset_is_sane() {
        let o = local_offset(1_791_115_200).expect("yerel ofset");
        assert!(o.abs() <= 14 * 3600 && o % 900 == 0, "{o}");
    }

    #[test]
    fn op_state_helpers() {
        let w = OpState::Waiting(Op::Ntp);
        assert!(w.waiting(Op::Ntp) && !w.waiting(Op::Time) && w.busy());
        let f = OpState::Failed(Op::Timezone, ErrorKind::NotAuthorized);
        assert_eq!(f.error(Op::Timezone), Some(ErrorKind::NotAuthorized));
        assert_eq!(f.error(Op::Ntp), None);
        assert!(!f.busy());
    }
}
