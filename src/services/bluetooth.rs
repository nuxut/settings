//! Bluetooth: BlueZ durumu ve eylemleri (Bağlantılar › Bluetooth). İstemci kabukla ortak
//! `nuxut-bluez` crate'idir (`../shell/crates/nuxut-bluez`); bu modül onu Ayarlar'ın olay döngüsüne
//! bağlar, eşleştirme/görünürlük gibi Ayarlar'a özgü durumu tutar ve sayfanın okuduğu saf
//! görünümleri türetir. Kararlar: `docs/design/bluetooth-page.md`.
//!
//! - **Ajansız** (karar, 2026-10-13): `BluetoothOptions::default()` ajan yayınlamaz,
//!   `RegisterAgent` çağrılmaz. PIN/onay kabuğun ajanında sorulur; Ayarlar yalnız `Pair`
//!   sonucunu gösterir ([`Snapshot::pairing`], [`Snapshot::pair_failed`]).
//! - **Canlı:** crate ObjectManager sinyalleriyle durumu günceller ve `sink` ile tam anlık görüntü
//!   yollar; burada saklanır, UI'a [`Event::Bluetooth`] gider.
//! - **Tarama** yalnız Bluetooth sayfası görünürken ([`set_page_visible`]; sayfa açılınca bir kez,
//!   adaptör açılınca, "Yeniden ara"da ve odak dönünce). Sayfadan çıkınca durdurulur — yalnız
//!   bizim başlattığımız tarama (crate `StopDiscovery`'yi yalnız kendi taraması sürerken çağırır).
//! - **Bus:** üründe yalnız sistem bus'ı. `test-support` özelliğiyle (testler, canlı test ikilisi)
//!   `nuxut_bluez::bus_from_env()`: `NUXUT_BLUEZ_BUS=session` → `dbus-run-session`'ın oturum
//!   bus'ı; oturum adresi yoksa servis açılmaz, sistem bus'ına **düşülmez** ([`bus`]; testli).
//! - **Gizlilik:** cihaz adı, adresi ve D-Bus yolu (MAC içerir) loglanmaz; yalnız eylem adı ve
//!   crate'in redakte `Debug`'ı.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use nuxut::ui::IconName;
use nuxut_bluez::{
    Adapter, AgentStatus, BluetoothEvent, BluetoothOptions, BluetoothService, BluetoothState, BluezError, BusKind, Device,
    PowerState, RequestId, Unavailable,
};

use super::shell_ipc::{IpcLink, ShellLink};
use super::{Event, emit};

/// Görünürlük açılınca BlueZ'in kendiliğinden kapatacağı süre (saniye; BlueZ varsayılanı 180).
pub const DISCOVERABLE_SECS: u32 = 180;
/// Taramanın kendiliğinden durduğu süre (sonra "Yeniden ara").
pub const DISCOVERY_TIMEOUT: Duration = Duration::from_secs(60);
/// Odak dönüşünde yeniden tarama için en az aralık.
pub const RESCAN_AFTER: Duration = Duration::from_secs(10);

/// Servisin durumu.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum BtState {
    /// Henüz ilk anlık görüntü gelmedi.
    #[default]
    Pending,
    Ready(BluetoothState),
    /// BlueZ'e ulaşılamıyor; neden crate'ten (bus yok / BlueZ çalışmıyor).
    Missing(Unavailable),
    /// Servis başlatılamadı (thread açılamadı ya da test bus'ı istendi ama adresi yok).
    Failed,
}

impl BtState {
    pub fn ready(&self) -> Option<&BluetoothState> {
        match self {
            BtState::Ready(s) => Some(s),
            _ => None,
        }
    }
}

/// Başarısız eylem (eşleştirme ve tarama hariç; onların kendi gösterimi var).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub action: &'static str,
    pub error: BluezError,
}

/// Süren eşleştirme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pairing {
    /// Cihaz yolu.
    pub device: String,
    /// Kullanıcı "İptal"e bastı (`CancelPairing` gönderildi): `AuthenticationCanceled` hata
    /// sayılmaz.
    pub cancelling: bool,
    /// `Pair` isteğinin kimliği; sonuç (`Done`/`Failed { action: "pair" }`) bununla eşlenir.
    /// `None`: kabuğun ajan durumu sorulurken (`Pair` henüz gönderilmedi).
    pub request: Option<RequestId>,
    /// Kabuğun eşleştirme ajanı varsayılan değil (kabuk kapalı ya da ajansız): satırda "onay için
    /// kabuk çalışmalı" uyarısı. Kabuk yanıt vermediyse `false` (eski davranış).
    pub no_agent: bool,
}

impl Pairing {
    pub fn new(device: impl Into<String>) -> Self {
        Self { device: device.into(), cancelling: false, request: None, no_agent: false }
    }
}

/// UI'nın gördüğü anlık görüntü.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Snapshot {
    pub state: BtState,
    /// Son başarısız eylem (üst not). Yeni eylemde temizlenir.
    pub failure: Option<Failure>,
    /// Süren eşleştirme (en çok bir; Ayarlar'dan başlatılan).
    pub pairing: Option<Pairing>,
    /// Son eşleştirme başarısızsa: cihaz yolu + hata (satırın altında gösterilir). Başarı satırın
    /// "Eşleşmiş cihazlar"a geçmesiyle görünür.
    pub pair_failed: Option<(String, BluezError)>,
    /// Görünürlüğün açıldığı an (BlueZ kalan süreyi bildirmez; geçişi görünce tutulur). Açık
    /// gelen ama açılışını görmediğimiz görünürlükte `None`: süre gösterilmez.
    pub discoverable_since: Option<Instant>,
}

// ---- Saf görünümler (testli) --------------------------------------------------------------------

/// Sayfanın ana durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageStatus {
    Reading,
    /// Servis açılamadı.
    Failed,
    /// Sistem bus'ına ulaşılamıyor.
    Unreachable,
    /// BlueZ çalışmıyor (bus'ta `org.bluez` yok).
    NoBluez,
    /// BlueZ var, Bluetooth adaptörü yok.
    NoAdapter,
    Ready,
}

pub fn page_status(s: &Snapshot) -> PageStatus {
    match &s.state {
        BtState::Pending => PageStatus::Reading,
        BtState::Failed => PageStatus::Failed,
        BtState::Missing(Unavailable::BusUnreachable) => PageStatus::Unreachable,
        BtState::Missing(_) => PageStatus::NoBluez,
        BtState::Ready(st) if st.adapter().is_none() => PageStatus::NoAdapter,
        BtState::Ready(_) => PageStatus::Ready,
    }
}

/// Adaptörün güç durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Power {
    On,
    Off,
    /// Açılıyor / kapanıyor (`PowerState` geçişte).
    Turning,
    /// rfkill yazılım engeli (uçak kipi; kullanıcı kaldırabilir).
    SoftBlocked,
    /// rfkill donanım engeli (fiziksel anahtar; yazılım kaldıramaz).
    HardBlocked,
}

/// v1: yalnız ilk adaptör (birden çok adaptör varsa diğerleri gösterilmez; belgede).
pub fn adapter(s: &BluetoothState) -> Option<&Adapter> {
    s.adapter()
}

/// İlk adaptörün güç durumu (adaptör yoksa `None`). rfkill engelinde sysfs okunamadıysa
/// yumuşak sayılır (kullanıcı uçak kipini kapatmayı dener; BlueZ reddederse not söyler).
pub fn power(s: &BluetoothState) -> Option<Power> {
    let a = adapter(s)?;
    Some(match a.power_state {
        PowerState::Blocked if s.rfkill.is_some_and(|r| r.hard) => Power::HardBlocked,
        PowerState::Blocked => Power::SoftBlocked,
        PowerState::Enabling | PowerState::Disabling => Power::Turning,
        _ if a.powered => Power::On,
        _ => Power::Off,
    })
}

/// Hazır ve adaptör açık mı.
pub fn powered_on(s: &Snapshot) -> bool {
    s.state.ready().and_then(power) == Some(Power::On)
}

/// İlk adaptörün cihazları (adaptör yolu boşsa da dahil).
fn adapter_devices(s: &BluetoothState) -> impl Iterator<Item = &Device> {
    let path = adapter(s).map(|a| a.path.as_str()).unwrap_or_default();
    s.devices.iter().filter(move |d| d.adapter.is_empty() || d.adapter == path)
}

/// Eşleşmiş cihazlar: bağlı önce, sonra ada göre (crate sırası).
pub fn paired_devices(s: &BluetoothState) -> Vec<&Device> {
    adapter_devices(s).filter(|d| d.paired).collect()
}

/// Adı bilinmeyen cihaz mı (BlueZ `Name` yok; takma ad adresten türetilmiştir).
pub fn is_unnamed(d: &Device) -> bool {
    d.remote_name.is_none()
}

/// Yakındaki cihazlar listesi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nearby<'a> {
    pub devices: Vec<&'a Device>,
    /// Gizlenen adsız cihaz sayısı (`show_unnamed` kapalıyken).
    pub hidden: usize,
}

/// Yakındaki (eşleşmemiş) cihazlar: sinyali güçlü olan önce (RSSI'siz — artık duyulmayan — en
/// sonda), eşitlikte ada göre. Adsız cihazlar `show_unnamed` kapalıyken gizlenir; eşleştirilmekte
/// olan cihaz her zaman görünür.
pub fn nearby_devices<'a>(s: &'a BluetoothState, show_unnamed: bool, pairing: Option<&str>) -> Nearby<'a> {
    let mut hidden = 0;
    let mut devices: Vec<&Device> = adapter_devices(s)
        .filter(|d| !d.paired)
        .filter(|d| {
            let keep = show_unnamed || !is_unnamed(d) || pairing == Some(d.path.as_str());
            hidden += usize::from(!keep);
            keep
        })
        .collect();
    devices.sort_by(|a, b| {
        let rssi = |d: &Device| d.rssi.map_or(i32::MIN, i32::from);
        rssi(b)
            .cmp(&rssi(a))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            .then_with(|| a.path.cmp(&b.path))
    });
    Nearby { devices, hidden }
}

/// Cihaz türü ikonu: BlueZ `Icon` (freedesktop adı) → `Class` (BR/EDR cihaz sınıfı) →
/// `Appearance` (LE görünüm kodu) → genel Bluetooth ikonu. Framework'te karşılığı olmayan türler
/// (telefon, saat, oyun kolu…) genel ikona düşer.
pub fn device_icon(d: &Device) -> IconName {
    d.icon
        .as_deref()
        .and_then(icon_from_name)
        .or_else(|| d.class.and_then(icon_from_class))
        .or_else(|| d.appearance.and_then(icon_from_appearance))
        .unwrap_or(IconName::Bluetooth)
}

fn icon_from_name(name: &str) -> Option<IconName> {
    Some(match name {
        "audio-headset" | "audio-headphones" => IconName::Headphones,
        "audio-card" | "audio-speakers" => IconName::Speaker,
        "audio-input-microphone" => IconName::Mic,
        "input-keyboard" => IconName::Keyboard,
        "input-mouse" => IconName::Mouse,
        "input-tablet" => IconName::Touchpad,
        "computer" => IconName::Desktop,
        "video-display" => IconName::Monitor,
        "camera-photo" => IconName::Camera,
        "camera-video" | "camera-web" => IconName::Videocam,
        "printer" => IconName::Print,
        _ => return None,
    })
}

/// Bluetooth Core "Class of Device": ana sınıf bit 8–12, alt sınıf bit 2–7.
fn icon_from_class(class: u32) -> Option<IconName> {
    let major = (class >> 8) & 0x1f;
    let minor = (class >> 2) & 0x3f;
    Some(match major {
        0x01 => IconName::Desktop,
        0x04 => match minor {
            0x01 | 0x02 | 0x06 => IconName::Headphones,
            0x04 => IconName::Mic,
            0x05 | 0x07 | 0x08 | 0x0a => IconName::Speaker,
            0x0c | 0x0d => IconName::Videocam,
            0x0e..=0x10 => IconName::Monitor,
            _ => IconName::Speaker,
        },
        0x05 => match minor >> 4 {
            0x01 | 0x03 => IconName::Keyboard,
            0x02 => IconName::Mouse,
            _ => return None,
        },
        0x06 => match minor {
            m if m & 0x04 != 0 => IconName::Monitor,
            m if m & 0x08 != 0 => IconName::Camera,
            m if m & 0x20 != 0 => IconName::Print,
            _ => return None,
        },
        _ => return None,
    })
}

/// Bluetooth "Appearance": kategori = değer >> 6.
fn icon_from_appearance(appearance: u16) -> Option<IconName> {
    Some(match appearance {
        0x03c1 => IconName::Keyboard,
        0x03c2 => IconName::Mouse,
        0x03c5 => IconName::Touchpad,
        _ => match appearance >> 6 {
            0x002 => IconName::Desktop,
            0x005 => IconName::Monitor,
            0x021 => IconName::Speaker,
            0x022 => IconName::Mic,
            0x025 => IconName::Headphones,
            _ => return None,
        },
    })
}

/// Görünürlüğün kalan süresi (saniye). Görünmezken, süresizken (`DiscoverableTimeout = 0`) ya da
/// açılışı görülmediyse `None`.
pub fn discoverable_remaining(a: &Adapter, since: Option<Instant>, now: Instant) -> Option<u32> {
    if !a.discoverable || a.discoverable_timeout == 0 {
        return None;
    }
    let elapsed = now.saturating_duration_since(since?).as_secs();
    Some(u32::try_from(u64::from(a.discoverable_timeout).saturating_sub(elapsed)).unwrap_or(0))
}

/// Kalan süre metni: "2:05" (dakika:saniye; dile bağımsız rakamlar).
pub fn mmss(secs: u32) -> String {
    format!("{}:{:02}", secs / 60, secs % 60)
}

/// Crate olayı → anlık görüntü (saf; testli). `true`: görünür bir şey değişti.
pub fn apply(s: &mut Snapshot, event: BluetoothEvent, now: Instant) -> bool {
    match event {
        BluetoothEvent::State(state) => {
            // Açılışı ancak önceki hazır durumda kapalıyken görürsek biliriz.
            let seen_closed = s.state.ready().and_then(adapter).is_some_and(|a| !a.discoverable);
            let next = if state.available {
                BtState::Ready(state)
            } else {
                BtState::Missing(state.unavailable.unwrap_or(Unavailable::ServiceNotRunning))
            };
            if s.state == next {
                return false;
            }
            s.state = next;
            let st = s.state.ready();
            let discoverable = st.and_then(adapter).is_some_and(|a| a.discoverable);
            if !discoverable {
                s.discoverable_since = None;
            } else if seen_closed {
                s.discoverable_since = Some(now);
            }
            if let Some(p) = &s.pairing {
                // Başarı: cihaz eşleşti ya da (eşleşme gerektirmeyen LE cihaz) bağlandı. Cihaz
                // listeden düştüyse (BlueZ geçici cihazı sildi) eşleştirme de bitti sayılır.
                let device = st.and_then(|st| st.devices.iter().find(|d| d.path == p.device));
                if device.is_none_or(|d| d.paired || d.connected) {
                    s.pairing = None;
                }
            }
            true
        }
        // Eşleştirme sonucu istek kimliğiyle eşlenir. Bizim olmayan (eski, durumdan zaten bitmiş
        // sayılan) bir eşleştirmenin hatası süren eşleştirmeye dokunmaz.
        BluetoothEvent::Failed { request, action: "pair", device, error } => {
            let ours = s.pairing.as_ref().is_some_and(|p| p.request.is_some() && p.request == request);
            if !ours && s.pairing.is_some() {
                tracing::debug!("bluetooth: başka eşleştirmenin sonucu yok sayıldı: {error:?}");
                return false;
            }
            let ours = s.pairing.take();
            let device = device.or_else(|| ours.as_ref().map(|p| p.device.clone())).unwrap_or_default();
            let cancelled = ours.is_some_and(|p| p.cancelling) && error == BluezError::AuthenticationCanceled;
            tracing::info!("bluetooth: eşleştirme bitti: {error:?}");
            s.pair_failed = (!cancelled).then_some((device, error));
            true
        }
        // Eşleşme kuruldu (crate `Paired=true` durumunu bundan önce yayınlar; çoğunlukla durum
        // zaten bitirmiştir).
        BluetoothEvent::Done { request, action: "pair", .. } => {
            if s.pairing.as_ref().is_some_and(|p| p.request == Some(request)) {
                s.pairing = None;
                return true;
            }
            false
        }
        // Diğer eylemlerin başarısı durumda görünür (crate durumu `Done`'dan önce yayınlar).
        BluetoothEvent::Done { .. } => false,
        // İptal isteğinin hatası: eşleştirme zaten bitmiş olabilir; sonucu `pair` getirir.
        BluetoothEvent::Failed { action: "cancel_pairing", error, .. } => {
            tracing::debug!("bluetooth: eşleştirme iptali: {error:?}");
            false
        }
        // Tarama hatası (ör. başka istemci tarıyor, adaptör hazır değil) kullanıcıya gösterilmez;
        // `own_discovery` durumu gerçeği söyler.
        BluetoothEvent::Failed { action: "discovery", error, .. } => {
            tracing::debug!("bluetooth: tarama: {error:?}");
            false
        }
        BluetoothEvent::Failed { action, error, .. } => {
            tracing::info!("bluetooth: {action} başarısız: {error:?}");
            s.failure = Some(Failure { action, error });
            true
        }
        // Ajansız istemciye ajan isteği gelmez; gelse de yok sayılır. Crate'in ileride ekleyeceği
        // olaylar da (`#[non_exhaustive]`) yok sayılır.
        _ => false,
    }
}

/// Kabuğun eşleştirme ajanı durumu: `nuxut-shell msg bluetooth agent-status` →
/// `{"registered":bool,"default":bool}` (çevrilmez; shell ADR-0010). Sözleşme dışıysa `None`.
pub fn parse_agent_status(text: &str) -> Option<AgentStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    Some(AgentStatus { registered: o.get("registered")?.as_bool()?, default: o.get("default")?.as_bool()? })
}

/// Kabuğa ajan durumunu sorar (saf; testte sahte bağlantı). Kabuk yok / eski / yanıt vermedi →
/// `None` (eski davranış: uyarı yok).
pub fn query_agent(link: &dyn ShellLink) -> Option<AgentStatus> {
    parse_agent_status(&link.request(&["bluetooth", "agent-status"]).ok()?)
}

/// Uyarı gerekir mi: kabuk yanıt verdi ve ajanı varsayılan değil (Ayarlar'ın `Pair`'ı ona
/// ulaşmaz; onay isteyen cihaz eşleşemez). Yanıt yoksa uyarı yok.
pub fn needs_agent_warning(status: Option<AgentStatus>) -> bool {
    status.is_some_and(|a| !a.default)
}

/// İstemcinin bağlanacağı bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bus {
    System,
    /// Test: `dbus-run-session`'ın oturum bus'ı.
    Session(String),
    /// Test bus'ı istendi ama adresi yok: servis açılmaz (sistem bus'ına DÜŞÜLMEZ).
    Unavailable,
}

/// Bus türü + `DBUS_SESSION_BUS_ADDRESS` → bus. Oturum bus'ı açık adresle açılır (zbus'ın
/// `$XDG_RUNTIME_DIR/bus` yedeğine de düşülmez).
pub fn bus(kind: BusKind, session_address: Option<&str>) -> Bus {
    match kind {
        BusKind::System => Bus::System,
        BusKind::Session => session_address.filter(|a| !a.is_empty()).map_or(Bus::Unavailable, |a| Bus::Session(a.to_owned())),
    }
}

/// Ortamdaki bus türü: üründe her zaman sistem; `test-support` ile `NUXUT_BLUEZ_BUS`.
fn env_bus_kind() -> BusKind {
    #[cfg(feature = "test-support")]
    {
        nuxut_bluez::bus_from_env()
    }
    #[cfg(not(feature = "test-support"))]
    {
        BusKind::System
    }
}

/// Test: rfkill sysfs dizinini geçici dizine çevirir (yalnız `test-support`).
#[cfg(feature = "test-support")]
pub const RFKILL_ENV: &str = "NUXUT_BLUEZ_RFKILL_DIR";

/// İstemci seçenekleri: **ajansız** (varsayılan; `agent_path: None`), sistem bus'ı ya da test
/// bus'ının açık adresi. `Bus::Unavailable` → `None` (servis açılmaz).
pub fn options(bus: &Bus) -> Option<BluetoothOptions> {
    let mut o = BluetoothOptions { discovery_timeout: DISCOVERY_TIMEOUT, ..BluetoothOptions::default() };
    match bus {
        Bus::System => {}
        Bus::Session(address) => {
            o.bus_address = Some(address.clone());
            #[cfg(feature = "test-support")]
            {
                o.rfkill_dir = std::env::var_os(RFKILL_ENV).map(std::path::PathBuf::from);
            }
        }
        Bus::Unavailable => return None,
    }
    debug_assert!(o.agent_path.is_none(), "Ayarlar asla eşleştirme ajanı olmaz");
    Some(o)
}

// ---- Süreç geneli durum ve servis ---------------------------------------------------------------

static SNAPSHOT: Mutex<Option<Snapshot>> = Mutex::new(None);
static SERVICE: OnceLock<Mutex<Option<BluetoothService>>> = OnceLock::new();
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
    emit(Event::Bluetooth);
}

/// Sayfa ilk kurulurken: servisi bir kez başlatır (sonraki çağrılar bir şey yapmaz).
pub fn ensure() {
    // Birim testleri bus'a gitmez (ve thread açmaz).
    if cfg!(test) || STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let session = std::env::var("DBUS_SESSION_BUS_ADDRESS").ok();
    let Some(opts) = options(&bus(env_bus_kind(), session.as_deref())) else {
        tracing::warn!("bluetooth: test bus'ı istendi ama oturum bus adresi yok; servis açılmadı");
        update(|s| s.state = BtState::Failed);
        return;
    };
    let sink = |event: BluetoothEvent| {
        let changed = {
            let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
            apply(s.get_or_insert_with(Snapshot::default), event, Instant::now())
        };
        if changed {
            emit(Event::Bluetooth);
            after_change();
        }
    };
    match BluetoothService::spawn(opts, sink) {
        Ok(svc) => {
            let _ = SERVICE.set(Mutex::new(Some(svc)));
            // İlk durum servis kaydedilmeden gelmiş olabilir.
            after_change();
        }
        Err(e) => {
            tracing::warn!("bluetooth: servis thread'i açılamadı: {e}");
            update(|s| s.state = BtState::Failed);
        }
    }
}

/// Servise komut gönderir: istek kimliği; servis yoksa ya da kuyruğa girmediyse `None`.
fn with_service(f: impl FnOnce(&BluetoothService) -> Option<RequestId>) -> Option<RequestId> {
    SERVICE.get().and_then(|m| m.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(f))
}

/// Kullanıcı eylemi: önceki hata notu kalkar, komut gönderilir. Sonuç durumda görünür; hata
/// `Failed` ile üst nota gelir. Kuyruğa girmediyse (servis yok/kapandı) üst notta hata.
fn act(action: &'static str, f: impl FnOnce(&BluetoothService) -> Option<RequestId>) {
    tracing::debug!("bluetooth: {action}");
    update(|s| s.failure = None);
    if with_service(f).is_none() {
        tracing::debug!("bluetooth: servis yok, {action} gönderilmedi");
        update(|s| s.failure = Some(Failure { action, error: BluezError::BluezUnavailable }));
    }
}

fn first_adapter() -> Option<String> {
    snapshot().state.ready().and_then(adapter).map(|a| a.path.clone())
}

pub fn set_powered(on: bool) {
    act("power", |svc| svc.set_powered(on));
}

/// Bu bilgisayarın Bluetooth adı (boş: sistem adı).
pub fn set_name(name: &str) {
    let Some(a) = first_adapter() else { return };
    let name = name.trim().to_owned();
    act("adapter_alias", |svc| svc.set_adapter_alias(&a, &name));
}

/// Görünürlük: açarken [`DISCOVERABLE_SECS`] süreyle.
pub fn set_discoverable(on: bool) {
    let Some(a) = first_adapter() else { return };
    act("discoverable", |svc| svc.set_discoverable(&a, on, DISCOVERABLE_SECS));
}

pub fn connect(device: &str) {
    act("connect", |svc| svc.connect(device));
}

pub fn disconnect(device: &str) {
    act("disconnect", |svc| svc.disconnect(device));
}

pub fn set_trusted(device: &str, on: bool) {
    act("trust", |svc| svc.set_trusted(device, on));
}

pub fn set_blocked(device: &str, on: bool) {
    act("block", |svc| svc.set_blocked(device, on));
}

/// Cihazın görünen adı (boş: cihazın kendi adı).
pub fn rename(device: &str, alias: &str) {
    let alias = alias.trim().to_owned();
    act("alias", |svc| svc.set_device_alias(device, &alias));
}

/// Eşleşmeyi siler (UI onay aldıktan sonra çağırır).
pub fn forget(device: &str) {
    act("forget", |svc| svc.forget(device));
}

/// Eşleştirir. Aynı anda tek eşleştirme: süren varsa yok sayılır. Onay kabuğun ajanında:
/// `Pair`'dan önce kabuğa ajan durumu sorulur (`msg bluetooth agent-status`; kısa ömürlü
/// thread, en çok IPC zaman aşımı); ajanı varsayılan değilse satırda uyarı gösterilir ve `Pair`
/// yine denenir (onay istemeyen cihazlar eşleşir). Kabuk yanıt vermezse uyarı yok (eski davranış).
pub fn pair(device: &str) {
    let mut started = false;
    update(|s| {
        if s.pairing.is_none() {
            s.pairing = Some(Pairing::new(device));
            s.pair_failed = None;
            s.failure = None;
            started = true;
        }
    });
    if !started {
        return;
    }
    let device = device.to_owned();
    let spawned = std::thread::Builder::new().name("bt-pair".into()).spawn(move || {
        let agent = query_agent(&IpcLink::from_env());
        send_pair(&device, needs_agent_warning(agent));
    });
    if spawned.is_err() {
        update(|s| s.pairing = None);
    }
}

/// `Pair`'ı gönderir (ajan sorgusundan sonra). İstek kimliği sonuçtan önce bağlansın diye
/// anlık görüntü kilidi altında gönderilir (sıra SNAPSHOT → SERVICE; gönderim bloklamaz).
/// Bu arada iptal edildiyse gönderilmez.
fn send_pair(device: &str, no_agent: bool) {
    {
        let mut g = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        let s = g.get_or_insert_with(Snapshot::default);
        let Some(p) = s.pairing.as_mut().filter(|p| p.device == device && p.request.is_none()) else { return };
        if p.cancelling {
            s.pairing = None;
        } else {
            p.no_agent = no_agent;
            match with_service(|svc| svc.pair(device)) {
                Some(r) => p.request = Some(r),
                None => s.pairing = None,
            }
        }
    }
    emit(Event::Bluetooth);
}

/// Süren eşleştirmeyi iptal eder (sonuç `pair` hatası olarak gelir, gösterilmez). `Pair` henüz
/// gönderilmediyse (ajan sorgusu sürüyor) gönderilmez.
pub fn cancel_pairing() {
    let mut device = None;
    update(|s| {
        if let Some(p) = &mut s.pairing {
            p.cancelling = true;
            device = p.request.is_some().then(|| p.device.clone());
        }
    });
    if let Some(d) = device {
        with_service(|svc| svc.cancel_pairing(&d));
    }
}

/// Eşleştirme hatasını kapatır (kullanıcı satırdan yeniden denerken).
pub fn clear_pair_failure() {
    update(|s| s.pair_failed = None);
}

// ---- Tarama: yalnız sayfa görünürken ------------------------------------------------------------

/// Bluetooth sayfası görünüyor mu.
static PAGE_VISIBLE: AtomicBool = AtomicBool::new(false);
/// Bu görünürlük döneminde (ya da son "Yeniden ara"dan beri) tarama istendi mi.
static DISCOVERY_REQUESTED: AtomicBool = AtomicBool::new(false);
static LAST_DISCOVERY: Mutex<Option<Instant>> = Mutex::new(None);

/// Tarama kararı (saf; testli): sayfa görünür, bu dönemde istenmedi, adaptör açık, kendi
/// taramamız sürmüyor.
pub fn should_discover(visible: bool, requested: bool, s: &Snapshot) -> bool {
    visible && !requested && powered_on(s) && !s.state.ready().is_some_and(|st| st.own_discovery)
}

/// Gezinmede `App` çağırır. Görünür olunca bir kez taranır (servis hazır değilse ilk durum
/// gelince); görünmez olunca bizim taramamız durdurulur.
pub fn set_page_visible(on: bool) {
    let was = PAGE_VISIBLE.swap(on, Ordering::Relaxed);
    if on == was {
        return;
    }
    tracing::debug!("bluetooth: sayfa görünür: {on}");
    DISCOVERY_REQUESTED.store(false, Ordering::Relaxed);
    if on {
        after_change();
    } else {
        // Crate yalnız kendi taraması sürerken `StopDiscovery` çağırır (başkasınınkine dokunmaz).
        with_service(BluetoothService::stop_discovery);
    }
}

/// "Yeniden ara" ya da odak dönüşü: sayfa görünüyorsa yeniden tara. `force` değilse son
/// taramadan [`RESCAN_AFTER`] geçmiş olmalı.
pub fn rescan(force: bool) {
    let last = *LAST_DISCOVERY.lock().unwrap_or_else(|e| e.into_inner());
    if force || last.is_none_or(|t| t.elapsed() >= RESCAN_AFTER) {
        DISCOVERY_REQUESTED.store(false, Ordering::Relaxed);
    }
    try_discover();
}

/// Durum değişti ya da sayfa açıldı: bekleyen taramayı başlat, görünürlük sayacını işlet.
fn after_change() {
    if !powered_on(&snapshot()) {
        // Adaptör kapandı: yeniden açılınca tarama yeniden istenebilsin.
        DISCOVERY_REQUESTED.store(false, Ordering::Relaxed);
    }
    try_discover();
    ensure_ticker();
}

fn try_discover() {
    if !should_discover(PAGE_VISIBLE.load(Ordering::Relaxed), DISCOVERY_REQUESTED.load(Ordering::Relaxed), &snapshot()) {
        return;
    }
    if SERVICE.get().is_none() {
        return;
    }
    // UI ve servis thread'i aynı anda gelebilir: hakkı önce kapan ister (tek tarama).
    if DISCOVERY_REQUESTED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return;
    }
    if with_service(BluetoothService::start_discovery).is_some() {
        tracing::debug!("bluetooth: tarama istendi");
        *LAST_DISCOVERY.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());
    } else {
        DISCOVERY_REQUESTED.store(false, Ordering::Release);
    }
}

/// Görünürlüğün kalan süresi saniyede bir güncellensin: sayfa görünürken ve süre işlerken
/// [`Event::Bluetooth`] yayan kısa ömürlü thread (ayrı bileşen gerekmez).
static TICKING: AtomicBool = AtomicBool::new(false);

fn ensure_ticker() {
    let counting = |s: &Snapshot| s.discoverable_since.is_some() && PAGE_VISIBLE.load(Ordering::Relaxed);
    if !counting(&snapshot()) || TICKING.swap(true, Ordering::AcqRel) {
        return;
    }
    let spawned = std::thread::Builder::new().name("bt-discoverable".into()).spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(1));
            if !counting(&snapshot()) {
                break;
            }
            emit(Event::Bluetooth);
        }
        TICKING.store(false, Ordering::Release);
        // Bu arada yeniden başladıysa kaçırılmasın.
        if counting(&snapshot()) {
            ensure_ticker();
        }
    });
    if spawned.is_err() {
        TICKING.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use nuxut_bluez::Rfkill;

    use super::*;

    fn dev(path: &str, name: Option<&str>, paired: bool, rssi: Option<i16>) -> Device {
        Device {
            path: format!("/org/bluez/hci0/{path}"),
            adapter: "/org/bluez/hci0".into(),
            address: "AA:BB:CC:DD:EE:FF".into(),
            name: name.unwrap_or("AA-BB-CC-DD-EE-FF").into(),
            alias: name.unwrap_or("AA-BB-CC-DD-EE-FF").into(),
            remote_name: name.map(str::to_owned),
            paired,
            rssi,
            ..Device::default()
        }
    }

    fn state(powered: bool) -> BluetoothState {
        BluetoothState {
            available: true,
            unavailable: None,
            adapters: vec![Adapter {
                path: "/org/bluez/hci0".into(),
                name: "pc".into(),
                powered,
                power_state: if powered { PowerState::On } else { PowerState::Off },
                discoverable_timeout: 180,
                ..Adapter::default()
            }],
            devices: vec![
                dev("dev_1", Some("Kulaklık"), true, None),
                dev("dev_2", Some("Klavye"), false, Some(-70)),
                dev("dev_3", Some("hoparlör"), false, Some(-40)),
                dev("dev_4", None, false, Some(-30)),
                dev("dev_5", Some("Eski"), false, None),
                dev("dev_6", Some("Aynı"), false, Some(-70)),
            ],
            own_discovery: false,
            rfkill: None,
            agent: AgentStatus::default(),
        }
    }

    fn ready(st: BluetoothState) -> Snapshot {
        Snapshot { state: BtState::Ready(st), ..Snapshot::default() }
    }

    #[test]
    fn page_status_order() {
        assert_eq!(page_status(&Snapshot::default()), PageStatus::Reading);
        assert_eq!(page_status(&Snapshot { state: BtState::Failed, ..Snapshot::default() }), PageStatus::Failed);
        assert_eq!(
            page_status(&Snapshot { state: BtState::Missing(Unavailable::BusUnreachable), ..Snapshot::default() }),
            PageStatus::Unreachable
        );
        assert_eq!(
            page_status(&Snapshot { state: BtState::Missing(Unavailable::ServiceNotRunning), ..Snapshot::default() }),
            PageStatus::NoBluez
        );
        let mut st = state(true);
        assert_eq!(page_status(&ready(st.clone())), PageStatus::Ready);
        st.adapters.clear();
        assert_eq!(page_status(&ready(st)), PageStatus::NoAdapter);
    }

    #[test]
    fn power_states_and_rfkill() {
        let mut st = state(true);
        assert_eq!(power(&st), Some(Power::On));
        st.adapters[0].powered = false;
        st.adapters[0].power_state = PowerState::Off;
        assert_eq!(power(&st), Some(Power::Off));
        st.adapters[0].power_state = PowerState::Enabling;
        assert_eq!(power(&st), Some(Power::Turning));
        st.adapters[0].power_state = PowerState::Blocked;
        assert_eq!(power(&st), Some(Power::SoftBlocked), "sysfs okunamadı: yumuşak sayılır");
        st.rfkill = Some(Rfkill { soft: true, hard: false });
        assert_eq!(power(&st), Some(Power::SoftBlocked));
        st.rfkill = Some(Rfkill { soft: true, hard: true });
        assert_eq!(power(&st), Some(Power::HardBlocked), "donanım anahtarı yazılımdan önce");
        // Eski BlueZ: PowerState yok → Powered'a göre.
        st.adapters[0].power_state = PowerState::Unknown;
        st.adapters[0].powered = true;
        assert_eq!(power(&st), Some(Power::On));
        st.adapters.clear();
        assert_eq!(power(&st), None);
    }

    #[test]
    fn nearby_sorted_by_rssi_unnamed_hidden() {
        let st = state(true);
        let n = nearby_devices(&st, false, None);
        let names: Vec<_> = n.devices.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, ["hoparlör", "Aynı", "Klavye", "Eski"], "güçlü önce, eşitlikte ad, RSSI'siz sonda; eşleşmiş yok");
        assert_eq!(n.hidden, 1, "adsız gizli");
        let all = nearby_devices(&st, true, None);
        assert_eq!(all.hidden, 0);
        assert_eq!(all.devices[0].path, "/org/bluez/hci0/dev_4", "adsız da sinyale göre");
        let pairing = nearby_devices(&st, false, Some("/org/bluez/hci0/dev_4"));
        assert_eq!((pairing.devices.len(), pairing.hidden), (5, 0), "eşleştirilen adsız cihaz görünür kalır");
        let paired: Vec<_> = paired_devices(&st).iter().map(|d| d.name.as_str()).collect();
        assert_eq!(paired, ["Kulaklık"]);
    }

    #[test]
    fn only_first_adapter_devices() {
        let mut st = state(true);
        st.adapters.push(Adapter { path: "/org/bluez/hci1".into(), ..Adapter::default() });
        let mut other = dev("dev_9", Some("Öteki"), true, None);
        other.adapter = "/org/bluez/hci1".into();
        st.devices.push(other);
        assert!(paired_devices(&st).iter().all(|d| d.name != "Öteki"), "v1 yalnız ilk adaptör");
    }

    #[test]
    fn icon_mapping() {
        let with = |icon: Option<&str>, class: Option<u32>, appearance: Option<u16>| Device {
            icon: icon.map(str::to_owned),
            class,
            appearance,
            ..Device::default()
        };
        assert_eq!(device_icon(&with(Some("audio-headset"), None, None)), IconName::Headphones);
        assert_eq!(device_icon(&with(Some("input-keyboard"), Some(0x240404), None)), IconName::Keyboard, "Icon önce");
        assert_eq!(device_icon(&with(Some("phone"), Some(0x240404), None)), IconName::Headphones, "bilinmeyen Icon → Class");
        assert_eq!(device_icon(&with(None, Some(0x240414), None)), IconName::Speaker, "hoparlör sınıfı");
        assert_eq!(device_icon(&with(None, Some(0x002540), None)), IconName::Keyboard, "çevre birimi: klavye");
        assert_eq!(device_icon(&with(None, Some(0x002580), None)), IconName::Mouse, "çevre birimi: işaretçi");
        assert_eq!(device_icon(&with(None, Some(0x000104), None)), IconName::Desktop, "bilgisayar");
        assert_eq!(device_icon(&with(None, Some(0x5a020c), None)), IconName::Bluetooth, "telefon: framework'te ikon yok");
        assert_eq!(device_icon(&with(None, None, Some(0x03c2))), IconName::Mouse);
        assert_eq!(device_icon(&with(None, None, Some(0x0941))), IconName::Headphones, "giyilebilir ses");
        assert_eq!(device_icon(&with(None, None, Some(0x00c2))), IconName::Bluetooth, "saat → genel");
        assert_eq!(device_icon(&with(None, None, None)), IconName::Bluetooth);
    }

    #[test]
    fn discoverable_countdown() {
        let now = Instant::now();
        let mut a = state(true).adapters.remove(0);
        assert_eq!(discoverable_remaining(&a, Some(now), now), None, "görünmez");
        a.discoverable = true;
        assert_eq!(discoverable_remaining(&a, Some(now), now), Some(180));
        assert_eq!(discoverable_remaining(&a, Some(now), now + Duration::from_secs(55)), Some(125));
        assert_eq!(discoverable_remaining(&a, Some(now), now + Duration::from_secs(999)), Some(0));
        assert_eq!(discoverable_remaining(&a, None, now), None, "açılışı görülmedi");
        a.discoverable_timeout = 0;
        assert_eq!(discoverable_remaining(&a, Some(now), now), None, "süresiz");
        assert_eq!(mmss(125), "2:05");
        assert_eq!(mmss(0), "0:00");
    }

    #[test]
    fn events_fold_and_discoverable_since() {
        let now = Instant::now();
        let mut s = Snapshot::default();
        let mut st = state(true);
        assert!(apply(&mut s, BluetoothEvent::State(st.clone()), now));
        assert!(!apply(&mut s, BluetoothEvent::State(st.clone()), now), "aynı durum yeniden çizdirmez");
        st.adapters[0].discoverable = true;
        assert!(apply(&mut s, BluetoothEvent::State(st.clone()), now));
        assert_eq!(s.discoverable_since, Some(now), "açılış görüldü");
        let later = now + Duration::from_secs(5);
        st.devices[0].battery = Some(50);
        apply(&mut s, BluetoothEvent::State(st.clone()), later);
        assert_eq!(s.discoverable_since, Some(now), "başka değişiklik sayacı sıfırlamaz");
        st.adapters[0].discoverable = false;
        apply(&mut s, BluetoothEvent::State(st.clone()), later);
        assert_eq!(s.discoverable_since, None);
        let gone = BluetoothState { unavailable: Some(Unavailable::ServiceNotRunning), ..BluetoothState::default() };
        assert!(apply(&mut s, BluetoothEvent::State(gone), later));
        assert_eq!(s.state, BtState::Missing(Unavailable::ServiceNotRunning));
        assert!(!apply(&mut s, BluetoothEvent::Failed { request: None, device: None, action: "discovery", error: BluezError::InProgress }, now), "tarama hatası sessiz");
        assert!(apply(&mut s, BluetoothEvent::Failed { request: Some(RequestId(3)), device: None, action: "power", error: BluezError::RfkillBlocked { hard: Some(false) } }, now));
        assert_eq!(s.failure, Some(Failure { action: "power", error: BluezError::RfkillBlocked { hard: Some(false) } }));
    }

    /// Eşleştirme durum makinesi: başla → (başarı: eşleşti/bağlandı) | (hata: türüyle) | (iptal:
    /// sessiz).
    #[test]
    fn pairing_state_machine() {
        let now = Instant::now();
        let dev2 = "/org/bluez/hci0/dev_2".to_owned();
        let mut st = state(true);
        let mut s = ready(st.clone());
        s.pairing = Some(Pairing { request: Some(RequestId(1)), ..Pairing::new(dev2.clone()) });
        st.devices[1].rssi = Some(-50);
        apply(&mut s, BluetoothEvent::State(st.clone()), now);
        assert!(s.pairing.is_some(), "başka değişiklik eşleştirmeyi bitirmez");
        st.devices[1].paired = true;
        apply(&mut s, BluetoothEvent::State(st.clone()), now);
        assert_eq!((s.pairing.clone(), s.pair_failed.clone()), (None, None), "eşleşti → başarı");

        // Ret: hata türü saklanır.
        s.pairing = Some(Pairing { request: Some(RequestId(1)), ..Pairing::new(dev2.clone()) });
        assert!(apply(&mut s, BluetoothEvent::Failed { request: Some(RequestId(1)), device: Some(dev2.clone()), action: "pair", error: BluezError::AuthenticationRejected }, now));
        assert_eq!(s.pairing, None);
        assert_eq!(s.pair_failed, Some((dev2.clone(), BluezError::AuthenticationRejected)));
        assert_eq!(s.failure, None, "eşleştirme hatası üst nota gitmez");

        // Kullanıcı iptali: AuthenticationCanceled gösterilmez.
        s.pair_failed = None;
        s.pairing = Some(Pairing { cancelling: true, request: Some(RequestId(1)), ..Pairing::new(dev2.clone()) });
        apply(&mut s, BluetoothEvent::Failed { request: Some(RequestId(1)), device: Some(dev2.clone()), action: "pair", error: BluezError::AuthenticationCanceled }, now);
        assert_eq!((s.pairing.clone(), s.pair_failed.clone()), (None, None));
        // Cihaz tarafı iptal (biz istemedik): gösterilir.
        s.pairing = Some(Pairing { request: Some(RequestId(1)), ..Pairing::new(dev2.clone()) });
        apply(&mut s, BluetoothEvent::Failed { request: Some(RequestId(1)), device: Some(dev2.clone()), action: "pair", error: BluezError::AuthenticationCanceled }, now);
        assert_eq!(s.pair_failed, Some((dev2.clone(), BluezError::AuthenticationCanceled)));

        // Bağlanan LE cihaz da başarı; listeden düşen cihaz eşleştirmeyi bitirir.
        let mut st = state(true);
        s.pairing = Some(Pairing { request: Some(RequestId(1)), ..Pairing::new(dev2.clone()) });
        st.devices[1].connected = true;
        apply(&mut s, BluetoothEvent::State(st.clone()), now);
        assert_eq!(s.pairing, None);
        s.pairing = Some(Pairing { request: Some(RequestId(1)), ..Pairing::new(dev2.clone()) });
        st.devices.remove(1);
        apply(&mut s, BluetoothEvent::State(st), now);
        assert_eq!(s.pairing, None);
    }

    /// Eşleştirme sonucu istek kimliğiyle eşlenir: `Done { pair }` bitirir; başka
    /// isteğin sonucu ya da `Pair` gönderilmeden (ajan sorgusu sürerken) gelen hata süren
    /// eşleştirmeye dokunmaz; bilinmeyen olay yok sayılır.
    #[test]
    fn pair_result_matches_request() {
        let now = Instant::now();
        let dev2 = "/org/bluez/hci0/dev_2".to_owned();
        let mut s = ready(state(true));
        s.pairing = Some(Pairing { request: Some(RequestId(5)), ..Pairing::new(dev2.clone()) });
        assert!(!apply(&mut s, BluetoothEvent::Done { request: RequestId(4), device: Some(dev2.clone()), action: "pair" }, now));
        assert!(s.pairing.is_some(), "başka istek");
        assert!(!apply(&mut s, BluetoothEvent::Failed { request: Some(RequestId(4)), device: None, action: "pair", error: BluezError::AuthenticationFailed }, now));
        assert!(s.pairing.is_some() && s.pair_failed.is_none(), "eski eşleştirmenin hatası süreni bitirmez");
        assert!(apply(&mut s, BluetoothEvent::Done { request: RequestId(5), device: Some(dev2.clone()), action: "pair" }, now));
        assert_eq!((s.pairing.clone(), s.pair_failed.clone()), (None, None), "Done: eşleşme kuruldu");
        assert!(!apply(&mut s, BluetoothEvent::Done { request: RequestId(6), device: None, action: "power" }, now), "diğer Done durumdan görünür");
        // Ajan sorgusu sürerken (istek yok) gelen eşleştirme hatası bizim değil.
        s.pairing = Some(Pairing::new(dev2.clone()));
        assert!(!apply(&mut s, BluetoothEvent::Failed { request: Some(RequestId(7)), device: None, action: "pair", error: BluezError::AuthenticationFailed }, now));
        assert!(s.pairing.is_some());
        // Otomatik güven/bağlan adımının hatası (`request: None`) üst nota.
        assert!(apply(&mut s, BluetoothEvent::Failed { request: None, device: Some(dev2.clone()), action: "connect", error: BluezError::ConnectionAttemptFailed }, now));
        assert_eq!(s.failure.as_ref().map(|f| f.action), Some("connect"));
    }

    #[test]
    fn agent_status_query() {
        use crate::services::shell_ipc::{FakeLink, LinkError};
        let link = FakeLink::with([
            Ok(r#"{"registered":true,"default":true}"#.to_owned()),
            Ok(r#"{"registered":true,"default":false}"#.to_owned()),
            Ok(r#"{"registered":false,"default":false}"#.to_owned()),
            Ok("error: bilinmeyen komut".to_owned()),
            Ok(r#"{"registered":true}"#.to_owned()),
            Err(LinkError::NotRunning),
            Err(LinkError::Timeout),
        ]);
        let got: Vec<_> = (0..7).map(|_| query_agent(&link)).collect();
        let yes = |registered, default| Some(AgentStatus { registered, default });
        assert_eq!(got, [yes(true, true), yes(true, false), yes(false, false), None, None, None, None]);
        assert!(link.sent().iter().all(|a| a == &["bluetooth", "agent-status"]));
        let warn: Vec<_> = got.into_iter().map(needs_agent_warning).collect();
        assert_eq!(warn, [false, true, true, false, false, false, false], "varsayılan değilse uyarı; yanıt yoksa eski davranış");
    }

    #[test]
    fn discovery_only_when_visible_once_and_powered() {
        let on = ready(state(true));
        assert!(should_discover(true, false, &on));
        assert!(!should_discover(false, false, &on), "sayfa görünmüyorken tarama yok");
        assert!(!should_discover(true, true, &on), "bu dönemde zaten istendi");
        assert!(!should_discover(true, false, &Snapshot::default()), "servis hazır değil");
        assert!(!should_discover(true, false, &ready(state(false))), "adaptör kapalı");
        let mut st = state(true);
        st.own_discovery = true;
        assert!(!should_discover(true, false, &ready(st)), "kendi taramamız sürüyor");
    }

    #[test]
    fn test_bus_never_falls_back_to_system() {
        assert_eq!(bus(BusKind::System, Some("unix:path=/x")), Bus::System);
        assert_eq!(bus(BusKind::Session, Some("unix:path=/tmp/b")), Bus::Session("unix:path=/tmp/b".into()));
        assert_eq!(bus(BusKind::Session, None), Bus::Unavailable, "adres yoksa sistem bus'ına düşülmez");
        assert_eq!(bus(BusKind::Session, Some("")), Bus::Unavailable);
        assert!(options(&Bus::Unavailable).is_none(), "servis açılmaz");
        #[cfg(not(feature = "test-support"))]
        assert_eq!(env_bus_kind(), BusKind::System, "üründe ortamdan bus seçilmez");
    }

    #[test]
    fn options_are_agentless() {
        for b in [Bus::System, Bus::Session("unix:path=/tmp/b".into())] {
            let o = options(&b).unwrap();
            assert_eq!(o.agent_path, None, "Ayarlar asla ajan olmaz (2026-10-13)");
            assert_eq!(o.discovery_timeout, DISCOVERY_TIMEOUT);
        }
        assert_eq!(options(&Bus::System).unwrap().bus_address, None);
        assert_eq!(options(&Bus::Session("a".into())).unwrap().bus_address.as_deref(), Some("a"));
    }
}
