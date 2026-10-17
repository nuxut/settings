
//! Güç: güç profilleri (power-profiles-daemon), pil durumu ve pil şarj sınırı (UPower) — Sistem ›
//! Güç sayfası (t117; `docs/design/power-page.md`, ortak plan `../shell/docs/design/power.md`).
//!
//! Veri kaynağı paylaşılan `nuxut-power` crate'idir (veri yolu B: Ayarlar kabuğun `msg power`'ına
//! BAĞLANMAZ; 2026-10-13). Sayfa crate türünü doğrudan değil, bu
//! modülün küçük görünüm modelini ([`PowerState`]) okur: tek dönüşüm [`from_live`] (testli); sahte
//! kaynak aynı modeli dosyadan kurar.
//!
//! - **Kaynak** ([`Source`]): ürün yapısında [`Source::Live`] (sistem bus'ı; servis açılamazsa
//!   [`Source::None`] — sayfa gizli). `NUXUT_POWER_FAKE=<dosya.toml|dosya.json>` ([`FAKE_ENV`]) ile
//!   [`Source::Fake`]: durum ve yazma sonucu dosyadan (test, canlı görsel deneme; [`FakeSpec`]).
//!   `test-support` özelliğiyle `NUXUT_POWER_BUS=session` canlı kaynağı oturum bus'ına yönlendirir.
//! - **Yazma iyimser değil** (power.md): istek gidince [`OpState::Waiting`]; durum ancak kaynak
//!   yeni durumu bildirince değişir. Hata sonrası durum aynen kalır, anahtar/seçim gerçek duruma
//!   döner. Aynı anda tek yazma.
//! - **Görünürlük** ([`listed`]): kaynak varsa sayfa her zaman kenar çubuğunda
//!   (2026-10-15): UPower/ppd sonradan gelip gidebilir, kenar çubuğu yeniden kurulmaz. Gösterecek bir
//!   şey yoksa (pil yok, şarj sınırı yok, ppd yok) sayfada maskotlu boş durum ([`empty`]).

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use serde::Deserialize;

pub use nuxut_power::{Battery, ChargeState};

use super::{Event, emit};

/// Sahte kaynak dosyası (yalnızca test/deneme): TOML ya da JSON ([`FakeSpec`]).
pub const FAKE_ENV: &str = "NUXUT_POWER_FAKE";
/// Sahte yazma gecikmesinin üst sınırı (dosyada daha büyüğü kırpılır).
const MAX_FAKE_DELAY: Duration = Duration::from_secs(10);
/// Sahte dosyanın üst sınırı (bayt).
const MAX_FAKE_BYTES: u64 = 64 * 1024;

// ---- Model (crate türlerinin yerel karşılığı) ---------------------------------------------------

/// power-profiles-daemon profili (crate: `PowerProfile`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PowerProfile {
    PowerSaver,
    Balanced,
    Performance,
}

impl PowerProfile {
    /// Seçimdeki sıra: tasarruf · dengeli · performans.
    pub const ALL: [PowerProfile; 3] = [PowerProfile::PowerSaver, PowerProfile::Balanced, PowerProfile::Performance];

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "power-saver" => Some(Self::PowerSaver),
            "balanced" => Some(Self::Balanced),
            "performance" => Some(Self::Performance),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::PowerSaver => "power-saver",
            Self::Balanced => "balanced",
            Self::Performance => "performance",
        }
    }
}

/// power-profiles-daemon durumu (crate: `Profiles`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Profiles {
    pub active: Option<PowerProfile>,
    pub available: Vec<PowerProfile>,
    /// Performans profili kısıtlı ise nedeni (`lap-detected`, `high-operating-temperature`).
    pub degraded: Option<String>,
}

/// Performans kısıtının bilinen nedenleri (ppd `PerformanceDegraded`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Degraded {
    LapDetected,
    HighTemperature,
    /// Tanınmayan neden (metin genel kalır).
    Other,
}

impl Degraded {
    /// Boş dize "kısıt yok" demektir.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "" => None,
            "lap-detected" => Some(Self::LapDetected),
            "high-operating-temperature" => Some(Self::HighTemperature),
            _ => Some(Self::Other),
        }
    }
}

impl Profiles {
    pub fn degraded(&self) -> Option<Degraded> {
        self.degraded.as_deref().and_then(Degraded::parse)
    }
}

/// Pil şarj sınırı: tüm sistem pillerinin birleşimi (crate: `ChargeLimit`). Eşikler UPower'da
/// udev hwdb'den gelir, salt okunurdur; yalnızca açılıp kapatılır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChargeLimit {
    /// UPower yok, pil yok ya da hiçbir pil desteklemiyor.
    #[default]
    Unsupported,
    Supported {
        /// Destekleyen pillerin hepsinde açık mı.
        enabled: bool,
        /// Bazılarında açık, bazılarında kapalı. Anahtar kapalı görünür; açmak hepsini açar.
        mixed: bool,
        /// Şarjın yeniden başladığı yüzde (`None`: ayarlanmadı / aygıt yazılımı / piller farklı).
        start: Option<u8>,
        /// Şarjın durduğu yüzde.
        end: Option<u8>,
    },
}

impl ChargeLimit {
    pub fn is_supported(self) -> bool {
        matches!(self, Self::Supported { .. })
    }

    pub fn is_enabled(self) -> bool {
        matches!(self, Self::Supported { enabled: true, .. })
    }
}

/// Güç durumunun Ayarlar'ın okuduğu kısmı (crate `PowerState`'inden [`from_live`] ile).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PowerState {
    /// UPower çalışıyor mu.
    pub upower: bool,
    /// Pilden mi çalışıyor (`false`: prizde ya da pil yok).
    pub on_battery: bool,
    /// Birleşik pil (`None`: pil yok — masaüstü, VM — ya da UPower yok).
    pub battery: Option<Battery>,
    /// `None`: power-profiles-daemon (ya da tuned-ppd) yok.
    pub profiles: Option<Profiles>,
    pub charge_limit: ChargeLimit,
}

/// Crate durumu → görünüm modeli. Profiller sabit sırada (tasarruf · dengeli · performans),
/// boş kısıt nedeni "kısıt yok".
pub fn from_live(s: &nuxut_power::PowerState) -> PowerState {
    let profile = |p: nuxut_power::PowerProfile| PowerProfile::parse(p.as_str());
    PowerState {
        upower: s.upower,
        on_battery: s.on_battery,
        battery: s.battery(),
        profiles: s.profiles.as_ref().map(|p| {
            let available: Vec<PowerProfile> = p.available.iter().copied().filter_map(profile).collect();
            Profiles {
                active: p.active.and_then(profile),
                available: PowerProfile::ALL.into_iter().filter(|x| available.contains(x)).collect(),
                degraded: p.degraded.clone().filter(|d| !d.trim().is_empty()),
            }
        }),
        charge_limit: match s.charge_limit() {
            nuxut_power::ChargeLimit::Unsupported => ChargeLimit::Unsupported,
            nuxut_power::ChargeLimit::Supported { enabled, mixed, start, end } => ChargeLimit::Supported { enabled, mixed, start, end },
        },
    }
}

/// Crate hatası → görünüm hatası (aynı türler).
pub fn error_from_live(e: nuxut_power::PowerError) -> PowerError {
    match e {
        nuxut_power::PowerError::Denied => PowerError::Denied,
        nuxut_power::PowerError::Unsupported => PowerError::Unsupported,
        nuxut_power::PowerError::Unavailable => PowerError::Unavailable,
        nuxut_power::PowerError::Failed(name) => PowerError::Failed(name),
    }
}

/// Yazma hatası (crate `PowerError`'ın karşılığı; profil ve şarj sınırı ortak).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerError {
    /// polkit reddi ya da iptal (yalnızca etkin yerel oturum yazabilir).
    Denied,
    /// Aygıt/hizmet bu işlemi desteklemiyor (durum yeniden okunur).
    Unsupported,
    /// Hizmet (UPower / ppd) çalışmıyor.
    Unavailable,
    /// Diğer; içerik yalnızca D-Bus hata adıdır (log için, kullanıcıya gösterilmez).
    Failed(String),
}

// ---- Yazma durumu -------------------------------------------------------------------------------

/// Yazma isteği.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Profile(PowerProfile),
    ChargeLimit(bool),
}

/// Yazmanın türü (hata hangi satırın).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpKind {
    Profile,
    ChargeLimit,
}

impl Op {
    pub fn kind(self) -> OpKind {
        match self {
            Op::Profile(_) => OpKind::Profile,
            Op::ChargeLimit(_) => OpKind::ChargeLimit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OpState {
    #[default]
    Idle,
    /// İstek gönderildi, yanıt bekleniyor.
    Waiting(Op),
    Failed(OpKind, PowerError),
}

impl OpState {
    pub fn busy(&self) -> bool {
        matches!(self, OpState::Waiting(_))
    }

    /// Bu türden bekleyen istek.
    pub fn waiting(&self, kind: OpKind) -> Option<Op> {
        match self {
            OpState::Waiting(op) if op.kind() == kind => Some(*op),
            _ => None,
        }
    }

    /// Bu türün son hatası.
    pub fn error(&self, kind: OpKind) -> Option<&PowerError> {
        match self {
            OpState::Failed(k, e) if *k == kind => Some(e),
            _ => None,
        }
    }
}

/// UI'nın gördüğü anlık görüntü.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Snapshot {
    /// `None`: kaynak yok (servis açılamadı) — sayfa gizli.
    pub state: Option<PowerState>,
    pub op: OpState,
}

/// Sayfa kenar çubuğunda ve aramada görünür mü: kaynak varsa her zaman (içerik yoksa boş durum).
pub fn listed(s: &Snapshot) -> bool {
    s.state.is_some()
}

/// Sayfada gösterecek bir şey yokken nedeni.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Empty {
    /// UPower ve power-profiles-daemon ikisi de çalışmıyor (hata).
    NoServices,
    /// UPower var ama pil yok (masaüstü, VM) ve power-profiles-daemon yok.
    NoBattery,
}

/// Boş durum: pil yok, şarj sınırı yok ve profil hizmeti yok. `None`: kartlar gösterilir.
pub fn empty(s: &Snapshot) -> Option<Empty> {
    let st = s.state.as_ref()?;
    if st.battery.is_some() || st.charge_limit.is_supported() || st.profiles.is_some() {
        return None;
    }
    Some(if st.upower { Empty::NoBattery } else { Empty::NoServices })
}

/// Profil kartı görünür mü: sayfa boş değilse her zaman (ppd yoksa devre dışı + not).
pub fn profiles_shown(s: &Snapshot) -> bool {
    listed(s) && empty(s).is_none()
}

/// Pil durumu satırı görünür mü.
pub fn battery_shown(s: &Snapshot) -> bool {
    s.state.as_ref().is_some_and(|st| st.battery.is_some())
}

/// Şarj sınırı satırı görünür mü: destekleniyor ve son yazma "desteklenmiyor" dönmedi.
pub fn charge_shown(s: &Snapshot) -> bool {
    s.state.as_ref().is_some_and(|st| st.charge_limit.is_supported()) && !matches!(s.op.error(OpKind::ChargeLimit), Some(PowerError::Unsupported))
}

/// İstek başlatılabilir mi ve başlat (saf; testli). Meşgulken, kaynak yokken, istenen durum
/// zaten geçerliyken ya da istek anlamsızken (profil listede yok, sınır desteklenmiyor) `false`.
pub fn begin(s: &mut Snapshot, op: Op) -> bool {
    let Some(st) = &s.state else { return false };
    if s.op.busy() {
        return false;
    }
    let valid = match op {
        Op::Profile(p) => st.profiles.as_ref().is_some_and(|ps| ps.available.contains(&p) && ps.active != Some(p)),
        // Karışık durumda açmak hepsini açar; kapatmak (bazıları açık) da anlamlıdır.
        Op::ChargeLimit(on) => match st.charge_limit {
            ChargeLimit::Supported { enabled, mixed, .. } => mixed || enabled != on,
            ChargeLimit::Unsupported => false,
        },
    };
    if valid {
        s.op = OpState::Waiting(op);
    }
    valid
}

/// Kaynak yanıt verdi (saf; testli): durum kaynağın bildirdiğidir (iyimser güncelleme yok).
pub fn finish(s: &mut Snapshot, op: Op, result: Result<(), PowerError>, state: Option<PowerState>) {
    if let Some(st) = state {
        s.state = Some(st);
    }
    s.op = match result {
        Ok(()) => OpState::Idle,
        Err(e) => {
            tracing::info!("güç: {op:?} başarısız: {e:?}");
            OpState::Failed(op.kind(), e)
        }
    };
}

// ---- Sahte kaynak -------------------------------------------------------------------------------

/// Sahte yazmanın sonucu (dosyada küçük harf: `ok`, `denied`, `unsupported`, `unavailable`,
/// `failed`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FakeOutcome {
    #[default]
    Ok,
    Denied,
    Unsupported,
    Unavailable,
    Failed,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FakeProfiles {
    pub active: Option<PowerProfile>,
    #[serde(default)]
    pub available: Vec<PowerProfile>,
    pub degraded: Option<String>,
}

/// Sahte pil durumu (dosyada küçük harf, crate `ChargeState::as_str` adları).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FakeChargeState {
    #[default]
    Unknown,
    Charging,
    Discharging,
    Empty,
    FullyCharged,
    PendingCharge,
    PendingDischarge,
}

impl FakeChargeState {
    fn state(self) -> ChargeState {
        match self {
            Self::Unknown => ChargeState::Unknown,
            Self::Charging => ChargeState::Charging,
            Self::Discharging => ChargeState::Discharging,
            Self::Empty => ChargeState::Empty,
            Self::FullyCharged => ChargeState::FullyCharged,
            Self::PendingCharge => ChargeState::PendingCharge,
            Self::PendingDischarge => ChargeState::PendingDischarge,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FakeBattery {
    pub percentage: f64,
    #[serde(default)]
    pub state: FakeChargeState,
    /// Kalan süre (dakika; yalnız pilden çalışırken anlamlı).
    pub time_to_empty_min: Option<u32>,
    /// Dolmaya kalan süre (dakika; yalnız şarj olurken).
    pub time_to_full_min: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FakeChargeLimit {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub mixed: bool,
    pub start: Option<u8>,
    pub end: Option<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FakeWrite {
    #[serde(default)]
    pub profile: FakeOutcome,
    #[serde(default)]
    pub charge_limit: FakeOutcome,
    /// Yanıt gecikmesi (ms; bekleme durumunu görmek için).
    #[serde(default)]
    pub delay_ms: u64,
}

/// `NUXUT_POWER_FAKE` dosyası. Örnek (dizüstü, sınır açık, yazma reddedilir):
///
/// ```toml
/// upower = true
/// on_battery = true
/// [battery]             # yoksa: pil yok
/// percentage = 64.0
/// state = "discharging" # charging, fully-charged, pending-charge …
/// time_to_empty_min = 130
/// [profiles]            # yoksa: power-profiles-daemon yok
/// active = "balanced"
/// available = ["power-saver", "balanced", "performance"]
/// degraded = "lap-detected"
/// [charge_limit]        # yoksa: şarj sınırı desteklenmiyor / pil yok
/// enabled = true
/// start = 75
/// end = 80
/// [write]
/// charge_limit = "denied"
/// delay_ms = 600
/// ```
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FakeSpec {
    #[serde(default = "yes")]
    pub upower: bool,
    /// Prize takılı değil (pilden çalışıyor).
    #[serde(default)]
    pub on_battery: bool,
    /// Yoksa: pil yok (masaüstü, VM).
    pub battery: Option<FakeBattery>,
    pub profiles: Option<FakeProfiles>,
    pub charge_limit: Option<FakeChargeLimit>,
    #[serde(default)]
    pub write: FakeWrite,
}

fn yes() -> bool {
    true
}

impl FakeSpec {
    /// Dosya içeriği; biçim uzantıdan (`.json` → JSON, diğerleri TOML).
    pub fn parse(text: &str, json: bool) -> Result<Self, String> {
        if json { serde_json::from_str(text).map_err(|e| e.to_string()) } else { toml_edit::de::from_str(text).map_err(|e| e.to_string()) }
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
        if meta.len() > MAX_FAKE_BYTES {
            return Err("dosya çok büyük".into()); // i18n: tanı yalnızca log (sahte kaynak, test aracı)
        }
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse(&text, path.extension().is_some_and(|e| e.eq_ignore_ascii_case("json")))
    }

    pub fn state(&self) -> PowerState {
        let minutes = |m: Option<u32>| m.filter(|&m| m > 0).map(|m| Duration::from_secs(u64::from(m) * 60));
        PowerState {
            upower: self.upower,
            on_battery: self.upower && self.on_battery,
            // UPower yoksa pil de yok (gerçek kaynakla aynı).
            battery: self.battery.as_ref().filter(|_| self.upower).map(|b| Battery {
                percentage: if b.percentage.is_finite() { b.percentage.clamp(0.0, 100.0) } else { 0.0 },
                state: b.state.state(),
                time_to_empty: minutes(b.time_to_empty_min),
                time_to_full: minutes(b.time_to_full_min),
                icon_name: String::new(),
                warning: nuxut_power::WarningLevel::None,
            }),
            profiles: self.profiles.as_ref().map(|p| Profiles {
                active: p.active,
                available: PowerProfile::ALL.into_iter().filter(|x| p.available.contains(x)).collect(),
                degraded: p.degraded.clone().filter(|d| !d.is_empty()),
            }),
            // UPower yoksa şarj sınırı da yok (gerçek kaynakla aynı).
            charge_limit: match (&self.charge_limit, self.upower) {
                (Some(c), true) => ChargeLimit::Supported { enabled: c.enabled && !c.mixed, mixed: c.mixed, start: c.start.filter(|&n| n <= 100), end: c.end.filter(|&n| n <= 100) },
                _ => ChargeLimit::Unsupported,
            },
        }
    }
}

/// Sahte kaynak: durum + yazma sonucu simülasyonu (saf; testli).
#[derive(Debug, Clone, PartialEq)]
pub struct Fake {
    pub state: PowerState,
    pub write: FakeWrite,
}

impl Fake {
    pub fn new(spec: &FakeSpec) -> Self {
        Self { state: spec.state(), write: spec.write.clone() }
    }

    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.write.delay_ms).min(MAX_FAKE_DELAY)
    }

    /// Yazmayı uygular; sonuç dosyadaki. Başarıda durum gerçek kaynak gibi değişir;
    /// "desteklenmiyor"da yeniden okuma sonucu (özellik yok) simüle edilir.
    pub fn write(&mut self, op: Op) -> Result<(), PowerError> {
        let outcome = match op {
            Op::Profile(_) => self.write.profile,
            Op::ChargeLimit(_) => self.write.charge_limit,
        };
        let result = match outcome {
            FakeOutcome::Ok => Ok(()),
            FakeOutcome::Denied => Err(PowerError::Denied),
            FakeOutcome::Unsupported => Err(PowerError::Unsupported),
            FakeOutcome::Unavailable => Err(PowerError::Unavailable),
            FakeOutcome::Failed => Err(PowerError::Failed("org.freedesktop.UPower.GeneralError".into())),
        };
        match (op, &result) {
            (Op::Profile(p), Ok(())) => {
                if let Some(ps) = &mut self.state.profiles {
                    ps.active = Some(p);
                }
            }
            (Op::ChargeLimit(on), Ok(())) => {
                if let ChargeLimit::Supported { enabled, mixed, .. } = &mut self.state.charge_limit {
                    *enabled = on;
                    *mixed = false;
                }
            }
            (Op::Profile(p), Err(PowerError::Unsupported)) => {
                if let Some(ps) = &mut self.state.profiles {
                    ps.available.retain(|x| *x != p);
                }
            }
            (Op::ChargeLimit(_), Err(PowerError::Unsupported)) => self.state.charge_limit = ChargeLimit::Unsupported,
            _ => {}
        }
        result
    }
}

// ---- Kaynak ve süreç geneli durum ---------------------------------------------------------------

/// Veri kaynağı.
#[derive(Debug)]
pub enum Source {
    /// Veri yok: sayfa gizli (canlı servis açılamadı ya da sahte dosya okunamadı).
    None,
    Fake(Fake),
    /// `nuxut-power` servisi: olaylar [`on_live_event`] ile anlık görüntüye işlenir.
    Live(nuxut_power::PowerService),
}

/// Sahte dosyadan kaynak (saf; testli): dosya okunamazsa `None`.
pub fn source_from(fake_path: &Path) -> Source {
    match FakeSpec::load(fake_path) {
        Ok(spec) => {
            tracing::info!("güç: sahte kaynak ({FAKE_ENV})");
            Source::Fake(Fake::new(&spec))
        }
        Err(e) => {
            tracing::warn!("güç: {FAKE_ENV} okunamadı: {e}");
            Source::None
        }
    }
}

/// Canlı istemci seçenekleri: üründe sistem bus'ı; bus seçimi yalnız `test-support` özelliğiyle
/// (`NUXUT_POWER_BUS=session` → `dbus-run-session`'ın oturum bus'ı; adres yoksa açılmaz).
fn live_options() -> Option<nuxut_power::PowerOptions> {
    #[cfg(feature = "test-support")]
    let kind = nuxut_power::bus_from_env();
    #[cfg(not(feature = "test-support"))]
    let kind = nuxut_power::BusKind::System;
    let session = std::env::var("DBUS_SESSION_BUS_ADDRESS").ok();
    let address = super::network::bus_address(kind, session.as_deref())?;
    Some(nuxut_power::PowerOptions { bus: kind, bus_address: address, ..nuxut_power::PowerOptions::default() })
}

static SNAPSHOT: Mutex<Option<Snapshot>> = Mutex::new(None);
static SOURCE: OnceLock<Mutex<Source>> = OnceLock::new();
static STARTED: AtomicBool = AtomicBool::new(false);
/// Canlı kaynakta uçuştaki yazma: servisin verdiği kimlik ve işlem. İstek gönderilirken kilit
/// tutulur: sonuç kimlik kaydedilmeden gelirse servis thread'i kaydı bekler.
static PENDING: Mutex<Option<(u64, Op)>> = Mutex::new(None);

/// Son anlık görüntü.
pub fn snapshot() -> Snapshot {
    ensure();
    SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default()
}

fn update(f: impl FnOnce(&mut Snapshot) -> bool) -> bool {
    let changed = {
        let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        f(s.get_or_insert_with(Snapshot::default))
    };
    if changed {
        emit(Event::Power);
    }
    changed
}

/// Canlı servis olayını anlık görüntüye uygular (saf; testli). `pending`: uçuştaki yazma
/// (sonucu gelince boşalır). Yeni durum sonuçtan önce `Changed` ile geldiği için sonuçta durum
/// verilmez. Görünür bir şey değiştiyse `true`.
pub fn apply_live(s: &mut Snapshot, pending: &mut Option<(u64, Op)>, event: nuxut_power::PowerEvent) -> bool {
    use nuxut_power::PowerEvent as E;
    let (id, result) = match event {
        E::Changed(st) => {
            let new = Some(from_live(&st));
            return std::mem::replace(&mut s.state, new.clone()) != new;
        }
        E::BatteryWarning(_) => return false,
        E::ProfileDone { id, result } | E::ChargeLimitDone { id, result } => (id, result),
    };
    match *pending {
        Some((p, op)) if p == id => {
            *pending = None;
            finish(s, op, result.map_err(error_from_live), None);
            true
        }
        // Başka bir istemcinin değil, bu sürecin eski/bilinmeyen isteği: yok sayılır.
        _ => false,
    }
}

fn on_live_event(event: nuxut_power::PowerEvent) {
    let mut pending = PENDING.lock().unwrap_or_else(|e| e.into_inner());
    update(|s| apply_live(s, &mut pending, event));
}

/// Kaynağı bir kez seçer (sonraki çağrılar bir şey yapmaz): `NUXUT_POWER_FAKE` verildiyse sahte,
/// değilse canlı servis. Birim testleri ortamı okumaz: kaynak `None`, sayfa gizli.
pub fn ensure() {
    if cfg!(test) || STARTED.swap(true, Ordering::AcqRel) {
        return;
    }
    let fake = std::env::var_os(FAKE_ENV).filter(|p| !p.is_empty()).map(std::path::PathBuf::from);
    let source = match fake {
        Some(path) => source_from(&path),
        None => match live_options().map(|o| nuxut_power::PowerService::spawn(o, on_live_event)) {
            Some(Ok(svc)) => Source::Live(svc),
            Some(Err(e)) => {
                tracing::warn!("güç: servis açılamadı: {e}");
                Source::None
            }
            None => {
                tracing::warn!("güç: oturum bus'ı istendi ama adres yok");
                Source::None
            }
        },
    };
    // Canlı kaynakta ilk durum servisten gelir; o gelene dek boş (UPower yok) görünür.
    let state = match &source {
        Source::None => None,
        Source::Fake(f) => Some(f.state.clone()),
        Source::Live(_) => Some(PowerState::default()),
    };
    {
        let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        let snap = s.get_or_insert_with(Snapshot::default);
        // Canlı servis ilk durumu bu satırdan önce bildirmiş olabilir: ezilmez.
        if snap.state.is_none() {
            snap.state = state;
        }
    }
    let _ = SOURCE.set(Mutex::new(source));
}

/// Sayfa görünür mü (kenar çubuğu, arama, kayıt satırlarının `visible_if`'i).
pub fn page_listed() -> bool {
    listed(&snapshot())
}

/// Şarj sınırı satırı görünür mü (kayıt satırının `visible_if`'i: arama).
pub fn charge_listed() -> bool {
    charge_shown(&snapshot())
}

/// Pil durumu satırı görünür mü (kayıt satırının `visible_if`'i: arama).
pub fn battery_listed() -> bool {
    battery_shown(&snapshot())
}

/// Profil kartı görünür mü (kayıt satırının `visible_if`'i: arama).
pub fn profiles_listed() -> bool {
    profiles_shown(&snapshot())
}

/// Güç profilini değiştir. Sonuç [`Event::Power`] ile; hata [`OpState::Failed`].
pub fn set_profile(profile: PowerProfile) {
    request(Op::Profile(profile));
}

/// Şarj sınırını aç/kapat (tüm destekleyen piller).
pub fn set_charge_limit(on: bool) {
    request(Op::ChargeLimit(on));
}

fn request(op: Op) {
    if cfg!(test) || !update(|s| begin(s, op)) {
        return;
    }
    let Some(m) = SOURCE.get() else { return };
    let delay = match &*m.lock().unwrap_or_else(|e| e.into_inner()) {
        Source::None => {
            update(|s| {
                finish(s, op, Err(PowerError::Unavailable), None);
                true
            });
            return;
        }
        Source::Live(svc) => {
            let mut pending = PENDING.lock().unwrap_or_else(|e| e.into_inner());
            let id = match op {
                Op::Profile(p) => svc.set_profile(live_profile(p)),
                Op::ChargeLimit(on) => svc.set_charge_limit(on),
            };
            match id {
                Some(id) => *pending = Some((id, op)),
                // Servis meşgul ya da durmuş: olay gelmez, hemen yanıtlanır.
                None => {
                    drop(pending);
                    update(|s| {
                        finish(s, op, Err(PowerError::Unavailable), None);
                        true
                    });
                }
            }
            return;
        }
        Source::Fake(f) => f.delay(),
    };
    // Gerçek kaynak gibi eşzamansız: yanıt arka plandan gelir (gecikme 0 da olsa).
    let spawned = std::thread::Builder::new().name("power-fake".into()).spawn(move || {
        std::thread::sleep(delay);
        complete(op);
    });
    if spawned.is_err() {
        complete(op);
    }
}

fn live_profile(p: PowerProfile) -> nuxut_power::PowerProfile {
    match p {
        PowerProfile::PowerSaver => nuxut_power::PowerProfile::PowerSaver,
        PowerProfile::Balanced => nuxut_power::PowerProfile::Balanced,
        PowerProfile::Performance => nuxut_power::PowerProfile::Performance,
    }
}

fn complete(op: Op) {
    let Some(m) = SOURCE.get() else { return };
    let (result, state) = match &mut *m.lock().unwrap_or_else(|e| e.into_inner()) {
        Source::Fake(f) => (f.write(op), Some(f.state.clone())),
        Source::None | Source::Live(_) => (Err(PowerError::Unavailable), None),
    };
    update(|s| {
        finish(s, op, result, state);
        true
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn battery(pct: f64, state: ChargeState) -> Battery {
        Battery { percentage: pct, state, time_to_empty: None, time_to_full: None, icon_name: String::new(), warning: nuxut_power::WarningLevel::None }
    }

    fn laptop() -> PowerState {
        PowerState {
            upower: true,
            on_battery: false,
            battery: Some(battery(81.0, ChargeState::Charging)),
            profiles: Some(Profiles { active: Some(PowerProfile::Balanced), available: PowerProfile::ALL.to_vec(), degraded: None }),
            charge_limit: ChargeLimit::Supported { enabled: false, mixed: false, start: Some(75), end: Some(80) },
        }
    }

    fn snap(state: PowerState) -> Snapshot {
        Snapshot { state: Some(state), op: OpState::Idle }
    }

    #[test]
    fn listed_and_empty_rules() {
        assert!(!listed(&Snapshot::default()), "kaynak yok → sayfa gizli");
        assert!(listed(&snap(laptop())) && empty(&snap(laptop())).is_none());
        assert!(battery_shown(&snap(laptop())));
        let mut desktop = laptop();
        desktop.battery = None;
        desktop.charge_limit = ChargeLimit::Unsupported;
        assert!(profiles_shown(&snap(desktop.clone())), "pil yok → yalnız profil kartı");
        assert!(!charge_shown(&snap(desktop.clone())) && !battery_shown(&snap(desktop.clone())));
        let mut no_ppd = laptop();
        no_ppd.profiles = None;
        assert!(profiles_shown(&snap(no_ppd)), "ppd yoksa profil kartı devre dışı görünür");
        desktop.profiles = None;
        assert!(listed(&snap(desktop.clone())), "içerik yoksa da sayfa listede (boş durum)");
        assert_eq!(empty(&snap(desktop.clone())), Some(Empty::NoBattery));
        assert!(!profiles_shown(&snap(desktop.clone())));
        desktop.upower = false;
        assert_eq!(empty(&snap(desktop)), Some(Empty::NoServices));
        assert_eq!(empty(&snap(PowerState::default())), Some(Empty::NoServices), "canlı kaynağın ilk hâli");
    }

    #[test]
    fn begin_rejects_busy_noop_and_invalid() {
        let mut s = snap(laptop());
        assert!(!begin(&mut s, Op::Profile(PowerProfile::Balanced)), "zaten etkin");
        assert!(!begin(&mut s, Op::ChargeLimit(false)), "zaten kapalı");
        assert!(begin(&mut s, Op::ChargeLimit(true)));
        assert_eq!(s.op, OpState::Waiting(Op::ChargeLimit(true)));
        assert!(!begin(&mut s, Op::Profile(PowerProfile::Performance)), "aynı anda tek yazma");
        let mut s = snap(laptop());
        s.state.as_mut().unwrap().profiles.as_mut().unwrap().available.retain(|p| *p != PowerProfile::Performance);
        assert!(!begin(&mut s, Op::Profile(PowerProfile::Performance)), "listede olmayan profil");
        assert!(!begin(&mut Snapshot::default(), Op::ChargeLimit(true)), "kaynak yok");
        let mut mixed = laptop();
        mixed.charge_limit = ChargeLimit::Supported { enabled: false, mixed: true, start: None, end: None };
        assert!(begin(&mut snap(mixed.clone()), Op::ChargeLimit(true)), "karışıkta açmak hepsini açar");
        assert!(begin(&mut snap(mixed), Op::ChargeLimit(false)), "karışıkta kapatmak da anlamlı");
    }

    #[test]
    fn fake_write_success_updates_state() {
        let mut fake = Fake { state: laptop(), write: FakeWrite::default() };
        let mut s = snap(fake.state.clone());
        assert!(begin(&mut s, Op::ChargeLimit(true)));
        assert!(!s.state.as_ref().unwrap().charge_limit.is_enabled(), "iyimser değil: bekleme sırasında durum eski");
        let r = fake.write(Op::ChargeLimit(true));
        finish(&mut s, Op::ChargeLimit(true), r, Some(fake.state.clone()));
        assert_eq!(s.op, OpState::Idle);
        assert!(s.state.as_ref().unwrap().charge_limit.is_enabled());
        assert!(begin(&mut s, Op::Profile(PowerProfile::Performance)));
        let r = fake.write(Op::Profile(PowerProfile::Performance));
        finish(&mut s, Op::Profile(PowerProfile::Performance), r, Some(fake.state.clone()));
        assert_eq!(s.state.unwrap().profiles.unwrap().active, Some(PowerProfile::Performance));
    }

    #[test]
    fn fake_write_error_keeps_real_state() {
        for (outcome, err) in [
            (FakeOutcome::Denied, PowerError::Denied),
            (FakeOutcome::Unavailable, PowerError::Unavailable),
            (FakeOutcome::Failed, PowerError::Failed("org.freedesktop.UPower.GeneralError".into())),
        ] {
            let mut fake = Fake { state: laptop(), write: FakeWrite { profile: outcome, charge_limit: outcome, delay_ms: 0 } };
            let mut s = snap(fake.state.clone());
            assert!(begin(&mut s, Op::ChargeLimit(true)));
            let r = fake.write(Op::ChargeLimit(true));
            finish(&mut s, Op::ChargeLimit(true), r, Some(fake.state.clone()));
            assert_eq!(s.op, OpState::Failed(OpKind::ChargeLimit, err.clone()));
            assert!(!s.state.as_ref().unwrap().charge_limit.is_enabled(), "{outcome:?}: hata sonrası gerçek durum (kapalı)");
            assert!(charge_shown(&s));
            assert!(begin(&mut s, Op::Profile(PowerProfile::PowerSaver)), "hata sonrası yeni istek serbest");
            let r = fake.write(Op::Profile(PowerProfile::PowerSaver));
            finish(&mut s, Op::Profile(PowerProfile::PowerSaver), r, Some(fake.state.clone()));
            assert_eq!(s.state.as_ref().unwrap().profiles.as_ref().unwrap().active, Some(PowerProfile::Balanced), "{outcome:?}: profil geri döner");
            assert_eq!(s.op.error(OpKind::Profile), Some(&err));
            assert_eq!(s.op.error(OpKind::ChargeLimit), None, "hata yalnız son işlemin satırında");
        }
    }

    #[test]
    fn unsupported_hides_toggle_and_rereads() {
        let mut fake = Fake { state: laptop(), write: FakeWrite { charge_limit: FakeOutcome::Unsupported, profile: FakeOutcome::Unsupported, delay_ms: 0 } };
        let mut s = snap(fake.state.clone());
        assert!(begin(&mut s, Op::ChargeLimit(true)));
        let r = fake.write(Op::ChargeLimit(true));
        finish(&mut s, Op::ChargeLimit(true), r, Some(fake.state.clone()));
        assert!(!charge_shown(&s), "desteklenmiyor → anahtar gizli");
        assert_eq!(s.state.as_ref().unwrap().charge_limit, ChargeLimit::Unsupported, "yeniden okunan durum");
        // Hata durumu kalsa da (yeniden okuma özelliği hâlâ bildiriyor olsa da) gizli kalır.
        let mut s = Snapshot { state: Some(laptop()), op: OpState::Failed(OpKind::ChargeLimit, PowerError::Unsupported) };
        assert!(!charge_shown(&s));
        s.op = OpState::Idle;
        assert!(charge_shown(&s));
        let mut s = snap(fake.state.clone());
        assert!(begin(&mut s, Op::Profile(PowerProfile::Performance)));
        let r = fake.write(Op::Profile(PowerProfile::Performance));
        finish(&mut s, Op::Profile(PowerProfile::Performance), r, Some(fake.state.clone()));
        assert_eq!(s.state.unwrap().profiles.unwrap().available, [PowerProfile::PowerSaver, PowerProfile::Balanced], "profil listeden düşer");
    }

    #[test]
    fn fake_spec_toml_and_json() {
        let toml = r#"
            upower = true
            [profiles]
            active = "performance"
            available = ["performance", "power-saver", "balanced"]
            degraded = "high-operating-temperature"
            [charge_limit]
            enabled = true
            start = 75
            end = 80
            [write]
            charge_limit = "denied"
            delay_ms = 99999
        "#;
        let spec = FakeSpec::parse(toml, false).unwrap();
        let st = spec.state();
        let ps = st.profiles.as_ref().unwrap();
        assert_eq!(ps.available, PowerProfile::ALL, "sıra sabit: tasarruf · dengeli · performans");
        assert_eq!(ps.degraded(), Some(Degraded::HighTemperature));
        assert_eq!(st.charge_limit, ChargeLimit::Supported { enabled: true, mixed: false, start: Some(75), end: Some(80) });
        let fake = Fake::new(&spec);
        assert_eq!(fake.write.charge_limit, FakeOutcome::Denied);
        assert_eq!(fake.delay(), MAX_FAKE_DELAY, "gecikme kırpılır");

        let json = r#"{"upower": false, "charge_limit": {"enabled": true, "end": 80}}"#;
        let st = FakeSpec::parse(json, true).unwrap().state();
        assert_eq!(st.profiles, None, "profiles yok → ppd yok");
        assert_eq!(st.charge_limit, ChargeLimit::Unsupported, "UPower yoksa sınır da yok");

        let empty = FakeSpec::parse("", false).unwrap().state();
        assert!(empty.upower && empty.profiles.is_none() && !empty.charge_limit.is_supported());
        assert!(FakeSpec::parse("bilinmeyen = 1", false).is_err(), "yazım hatası sessiz geçmez");
        let mixed = FakeSpec::parse("[charge_limit]\nenabled = true\nmixed = true\nend = 101", false).unwrap().state();
        assert_eq!(mixed.charge_limit, ChargeLimit::Supported { enabled: false, mixed: true, start: None, end: None }, "karışık → açık sayılmaz; 100 üstü eşik yok");
    }

    #[test]
    fn source_none_with_bad_file() {
        assert!(matches!(source_from(Path::new("/nonexistent/nuxut-power-fake.toml")), Source::None));
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("p.json");
        std::fs::write(&p, r#"{"profiles": {"active": "balanced", "available": ["balanced", "power-saver"]}}"#).unwrap();
        let Source::Fake(f) = source_from(&p) else { panic!("sahte kaynak bekleniyordu") };
        assert_eq!(f.state.profiles.unwrap().available, [PowerProfile::PowerSaver, PowerProfile::Balanced]);
    }


    fn live_state() -> nuxut_power::PowerState {
        use nuxut_power::PropMap;
        use zbus::zvariant::{OwnedValue, Value};
        let v = |x: Value<'_>| OwnedValue::try_from(x).unwrap();
        let mut s = nuxut_power::PowerState::default();
        s.upower = true;
        s.apply_manager(&PropMap::from([("OnBattery".to_owned(), v(Value::from(true)))]));
        s.apply_device(&PropMap::from([
            ("Type".to_owned(), v(Value::from(2u32))),
            ("IsPresent".to_owned(), v(Value::from(true))),
            ("Percentage".to_owned(), v(Value::from(64.0f64))),
            ("State".to_owned(), v(Value::from(2u32))),
            ("TimeToEmpty".to_owned(), v(Value::from(7800i64))),
        ]));
        let profile = |p: &str| std::collections::HashMap::from([("Profile".to_owned(), v(Value::from(p)))]);
        s.apply_profiles(&PropMap::from([
            ("ActiveProfile".to_owned(), v(Value::from("performance"))),
            ("Profiles".to_owned(), v(Value::from(vec![profile("performance"), profile("power-saver")]))),
            ("PerformanceDegraded".to_owned(), v(Value::from(" "))),
        ]));
        s
    }

    #[test]
    fn live_state_maps_to_view() {
        let st = from_live(&live_state());
        assert!(st.upower && st.on_battery);
        let b = st.battery.as_ref().unwrap();
        assert_eq!((b.percentage, b.state), (64.0, ChargeState::Discharging));
        assert_eq!(b.time_to_empty, Some(Duration::from_secs(7800)));
        let ps = st.profiles.as_ref().unwrap();
        assert_eq!(ps.available, [PowerProfile::PowerSaver, PowerProfile::Performance], "sabit sıra");
        assert_eq!((ps.active, ps.degraded.as_deref()), (Some(PowerProfile::Performance), None), "boş kısıt yok sayılır");
        assert_eq!(st.charge_limit, ChargeLimit::Unsupported, "izlenen pil yok");
        assert_eq!(from_live(&nuxut_power::PowerState::default()), PowerState::default());
        assert_eq!(error_from_live(nuxut_power::PowerError::Failed("x".into())), PowerError::Failed("x".into()));
    }

    #[test]
    fn live_events_apply_by_request_id() {
        use nuxut_power::PowerEvent as E;
        let mut s = snap(laptop());
        let mut pending = None;
        // İlk durum ve aynı durumun tekrarı.
        assert!(apply_live(&mut s, &mut pending, E::Changed(live_state())));
        assert!(!apply_live(&mut s, &mut pending, E::Changed(live_state())));
        assert!(!apply_live(&mut s, &mut pending, E::BatteryWarning(nuxut_power::WarningLevel::Low)));
        // İstek: yeni durum sonuçtan önce gelir; sonuç kimlikle eşleşince istek biter.
        assert!(begin(&mut s, Op::Profile(PowerProfile::PowerSaver)));
        pending = Some((7, Op::Profile(PowerProfile::PowerSaver)));
        assert!(!apply_live(&mut s, &mut pending, E::ProfileDone { id: 6, result: Ok(()) }), "başka kimlik");
        assert!(s.op.busy());
        assert!(apply_live(&mut s, &mut pending, E::ProfileDone { id: 7, result: Err(nuxut_power::PowerError::Denied) }));
        assert_eq!(s.op, OpState::Failed(OpKind::Profile, PowerError::Denied));
        assert!(pending.is_none());
        assert_eq!(s.state.as_ref().unwrap().profiles.as_ref().unwrap().active, Some(PowerProfile::Performance), "durum kaynağın");
        // Bağlantı koptu: servis boş durum yayınlar → boş durum (hizmet yok).
        assert!(apply_live(&mut s, &mut pending, E::Changed(nuxut_power::PowerState::default())));
        assert_eq!(empty(&s), Some(Empty::NoServices));
    }

    #[test]
    fn fake_battery_spec() {
        let toml = "on_battery = true\n[battery]\npercentage = 140.0\nstate = \"pending-charge\"\ntime_to_full_min = 35\ntime_to_empty_min = 0\n";
        let st = FakeSpec::parse(toml, false).unwrap().state();
        let b = st.battery.unwrap();
        assert_eq!((b.percentage, b.state), (100.0, ChargeState::PendingCharge), "yüzde kırpılır");
        assert_eq!((b.time_to_full, b.time_to_empty), (Some(Duration::from_secs(35 * 60)), None));
        assert!(st.on_battery);
        let st = FakeSpec::parse("upower = false\non_battery = true\n[battery]\npercentage = 50.0\n", false).unwrap().state();
        assert!(st.battery.is_none() && !st.on_battery, "UPower yoksa pil de yok");
        assert!(FakeSpec::parse("[battery]\npercentage = 50.0\nstate = \"uçuyor\"\n", false).is_err());
    }
    #[test]
    fn degraded_parse() {
        assert_eq!(Degraded::parse(""), None);
        assert_eq!(Degraded::parse("lap-detected"), Some(Degraded::LapDetected));
        assert_eq!(Degraded::parse("high-operating-temperature"), Some(Degraded::HighTemperature));
        assert_eq!(Degraded::parse("yeni-neden"), Some(Degraded::Other));
        assert_eq!(PowerProfile::parse(PowerProfile::Performance.as_str()), Some(PowerProfile::Performance));
    }
}
