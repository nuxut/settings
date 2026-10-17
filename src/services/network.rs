//! Ağ: NetworkManager durumu ve eylemleri (Bağlantılar › Wi-Fi, VPN). İstemci kabukla ortak
//! `nuxut-nm` crate'idir (`../shell/crates/nuxut-nm`); bu modül onu Ayarlar'ın olay döngüsüne
//! bağlar, saf görünümleri türetir ve eylemlerin durum makinesini tutar. Kararlar:
//! `docs/design/network-pages.md`.
//!
//! - **Ajansız** (karar, 2026-10-13): `NetworkOptions::default()` ajansızdır. Ayarlar
//!   asla gizli bilgi ajanı olmaz; NM şifre isterse kabuğun ajanı sorar.
//! - **Canlı:** crate ObjectManager sinyalleriyle durumu günceller ve `sink` ile tam anlık görüntü
//!   yollar; burada saklanır, UI'a [`Event::Network`] gider.
//! - **Eylemler** ([`Op`]): her eylem bir hedefe ([`Target`]) bağlanır (hedef başına en çok bir
//!   bekleyen) ve crate'in döndürdüğü [`RequestId`] ile izlenir; sonuç olayı (`Done`/`Failed`)
//!   aynı kimliği taşır (2026-10-14). Kimlik `None` → istek kuyruğa girmedi → satır
//!   hatası. Crate `Done`'dan önce o ana dek gelen durumu yayınlar: `Done` gelince durum günceldir.
//!   Etkinleştirmeler (bağlan, VPN aç, gizli ağ) `Done` ile değil, bağlantı gerçekten etkinleşince
//!   ya da `ActivationFailed` ile biter (NM çağrıyı kabul edip sonradan vazgeçebilir). Zaman aşımı
//!   yalnız güvenlik ağıdır (sessiz). **İyimser değil:** UI gerçek durumu gösterir, bekleyen
//!   eylem yalnızca ilerleme ve kapalı düğme olarak görünür.
//! - **polkit reddi** ([`NmError::Denied`]) sessizdir (timedate/accounts kalıbı): eylem düşer,
//!   durum gerçeği gösterir. Diğer hatalar hedefin satırında metin olur ([`RowError`]).
//! - **Kayıtlı ağın şifresi** `set_wifi_password` ile yerinde güncellenir (otomatik bağlanma,
//!   ölçülü vb. korunur), başarıda bağlantı etkinleştirilir; eski "unut + yeni bağlantı" yolu
//!   kalktı (karar K3, 2026-10-14).
//! - **Gizli bilgi:** şifre UI'nın parola alanından tek kopya `Zeroizing<Vec<u8>>` olarak gelir
//!   ve kopyasız [`Secret`]'e devredilir (ara `String` yok). WireGuard dosyası aynı biçimde
//!   okunur. Şifre, SSID, bağlantı adı ve dosya yolu loglanmaz.
//! - **Tarama** yalnız Wi-Fi sayfası görünürken ([`scan`]).
//! - **Bus seçimi:** ürün yapısında her zaman sistem bus'ı. Yalnız `test-support` özelliğiyle (test/demo;
//!   crate'in `test-support`'u) `NUXUT_NM_BUS=session` oturum bus'ına yönlendirir; o zaman oturum
//!   adresi yoksa servis açılmaz (sistem bus'ına DÜŞÜLMEZ, zbus'ın varsayılan oturum yoluna da).

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use nuxut_nm::{
    AccessPoint, ActivationFailure, ActiveState, BusKind, ConnectionKind, Connectivity, DeviceDetails, DeviceKind, Metered,
    NetworkEvent, NetworkOptions, NetworkService, NetworkState, NmError, RequestId, SavedConnection, Secret, Security,
    Unavailable, WireGuardError, valid_secret,
};
use zeroize::Zeroizing;

use super::{Event, emit};

/// Servisin durumu.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum NetState {
    /// Henüz ilk anlık görüntü gelmedi.
    #[default]
    Pending,
    Ready(NetworkState),
    /// Kullanılamıyor: bus'a ulaşılamıyor ya da NetworkManager çalışmıyor (ayrı metin).
    Unavailable(Unavailable),
    /// Servis başlatılamadı (thread açılamadı ya da test bus'ı eksik).
    Failed,
}

impl NetState {
    pub fn ready(&self) -> Option<&NetworkState> {
        match self {
            NetState::Ready(s) => Some(s),
            _ => None,
        }
    }
}

// ---- Eylemler ----------------------------------------------------------------------------------

/// Eylemin hedefi (satır). `Debug` redakte: SSID/bağlantı yolu yazılmaz.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    /// Wi-Fi aç/kapa.
    Wifi,
    /// Görünen ağ (SSID baytları).
    Ap(Vec<u8>),
    /// Kayıtlı bağlantı (Wi-Fi, VPN; yol).
    Conn(String),
    /// "Gizli ağa bağlan…" formu.
    Hidden,
    /// "WireGuard içe aktar…".
    Import,
}

impl std::fmt::Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Wifi => "Wifi",
            Self::Ap(_) => "Ap(<redacted>)",
            Self::Conn(_) => "Conn(<redacted>)",
            Self::Hidden => "Hidden",
            Self::Import => "Import",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpKind {
    Wifi(bool),
    /// Görünen ağa bağlan (kayıtlıysa etkinleştir, değilse yeni bağlantı).
    Connect,
    /// Kayıtlı bağlantıyı (VPN) etkinleştir.
    Activate,
    /// Etkin bağlantıyı kapat (bağlı Wi-Fi ya da VPN).
    Deactivate,
    Forget,
    Autoconnect(bool),
    Metered(Metered),
    Hidden,
    /// Dosya seçici açık ya da içe aktarma sürüyor.
    Import,
}

impl OpKind {
    /// Sonuç gözlenmezse eylem bu süreden sonra sessizce düşer — yalnız güvenlik ağı: her eylemin
    /// crate sonucu (`Done`/`Failed`) istek kimliğiyle gelir; etkinleştirme NM'de sürebilir, durum
    /// yine gerçeği gösterir.
    pub fn timeout(self) -> Duration {
        match self {
            Self::Connect | Self::Activate | Self::Hidden => Duration::from_secs(45),
            // Dosya seçici açıkken kullanıcı beklenir.
            Self::Import => Duration::from_secs(600),
            _ => Duration::from_secs(15),
        }
    }

    /// Bağlantı etkinleştiren eylem mi (crate `Done`'u bitirmez; etkinleşme ya da
    /// `ActivationFailed` bitirir).
    fn activates(self) -> bool {
        matches!(self, Self::Connect | Self::Activate | Self::Hidden)
    }
}

/// Bekleyen eylem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub id: u64,
    pub target: Target,
    pub kind: OpKind,
    /// Crate isteğinin kimliği (kuyruğa girince; sonuç olayı bununla eşlenir). İki adımlı
    /// eylemde (şifre güncelle → etkinleştir) o anki adımın isteği.
    pub request: Option<RequestId>,
    /// Etkinleşmesi beklenen kayıtlı bağlantı (`Done.object` ya da hedef; bilinmiyorsa SSID'den).
    pub connection: Option<String>,
    /// Kayıtlı ağın şifresi güncelleniyor: `Done` gelince bu bağlantı etkinleştirilir
    /// ([`FollowUp::Activate`]).
    pub then_activate: Option<String>,
    /// Crate isteği kabul etti (`Done`); etkinleştirme ancak bundan sonra durumda gözlenerek biter
    /// (zaten etkin görünen bağlantıyı yeniden etkinleştirirken erken bitmesin).
    pub acked: bool,
}

/// WireGuard içe aktarma hatası (dosya tarafı; ayrıştırma crate'te).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportError {
    /// Dosya seçici açılamadı.
    Portal,
    /// Dosya okunamadı.
    Read,
    /// [`nuxut_nm::wireguard::MAX_CONF_BYTES`]'tan büyük.
    TooLarge,
    /// UTF-8 değil.
    NotUtf8,
    /// Yapılandırma geçersiz (satır + tür; değer içermez).
    Parse(WireGuardError),
}

/// Satırda gösterilen hata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowError {
    Nm(NmError),
    /// Etkinleşemedi: gizli bilgi yok ya da yanlış (Wi-Fi: şifre alanı açılır).
    NoSecrets,
    AuthFailed,
    /// Etkinleşemedi, başka neden.
    ActivationFailed,
    /// İstemci doğrulaması: şifre bu güvenlik türüne uymuyor.
    InvalidPassword,
    /// İstemci doğrulaması: ağ adı 1–32 bayt olmalı.
    InvalidSsid,
    Import(ImportError),
}

impl RowError {
    /// Bu hata şifre alanını (yeniden) açar mı?
    pub fn wants_password(&self) -> bool {
        matches!(self, Self::NoSecrets | Self::AuthFailed | Self::InvalidPassword | Self::Nm(NmError::InvalidPassword))
    }
}

/// UI'nın gördüğü anlık görüntü.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Snapshot {
    pub state: NetState,
    /// Hedefsiz son hata (sayfa üstü not).
    pub failure: Option<NmError>,
    /// Bekleyen eylemler.
    pub ops: Vec<Op>,
    /// Hedef başına son hata (yeni eylem başlayınca silinir).
    pub errors: BTreeMap<Target, RowError>,
    /// Son WireGuard içe aktarmasında içe aktarılmayan wg-quick anahtarlarının **adları**
    /// (`PostUp`…; değer asla). İçe aktarma satırında bilgi notu; yeni içe aktarmada silinir.
    pub import_ignored: Vec<&'static str>,
    /// Bağlı cihazların ayrıntısı (yol → IP/DNS/hız).
    pub details: BTreeMap<String, DeviceDetails>,
    /// Eylem kimlik sayacı.
    pub next_id: u64,
}

impl Snapshot {
    pub fn op(&self, target: &Target) -> Option<&Op> {
        self.ops.iter().find(|o| o.target == *target)
    }

    pub fn busy(&self, target: &Target) -> bool {
        self.op(target).is_some()
    }

    pub fn error(&self, target: &Target) -> Option<&RowError> {
        self.errors.get(target)
    }

    /// Eylemi başlatır: hedef meşgulse `None`. Hedefin eski hatası (ve notu) ile sayfa notu
    /// silinir. Crate isteği [`Snapshot::attach`] ile bağlanır.
    pub fn begin(&mut self, target: Target, kind: OpKind) -> Option<u64> {
        if self.busy(&target) {
            return None;
        }
        self.next_id += 1;
        let connection = match (&target, kind) {
            (Target::Conn(p), OpKind::Activate) => Some(p.clone()),
            _ => None,
        };
        if target == Target::Import {
            self.import_ignored.clear();
        }
        self.errors.remove(&target);
        self.failure = None;
        self.ops.push(Op { id: self.next_id, target, kind, request: None, connection, then_activate: None, acked: false });
        Some(self.next_id)
    }

    /// Eylemin crate isteğini bağlar. `None`: istek kuyruğa girmedi (servis kapandı) → satır
    /// hatası. Eylem yoksa (bu arada bitti) `false`.
    pub fn attach(&mut self, id: u64, request: Option<RequestId>) -> bool {
        let Some(op) = self.ops.iter_mut().find(|o| o.id == id) else { return false };
        match request {
            Some(r) => {
                op.request = Some(r);
                true
            }
            None => {
                self.fail(id, RowError::Nm(NmError::NmUnavailable));
                false
            }
        }
    }

    fn take(&mut self, pred: impl Fn(&Op) -> bool) -> Option<Op> {
        let i = self.ops.iter().position(pred)?;
        Some(self.ops.remove(i))
    }

    /// Eylemi hatasız bitirir (zaman aşımı, iptal).
    pub fn finish(&mut self, id: u64) -> bool {
        self.take(|o| o.id == id).is_some()
    }

    /// Eylemi hatayla bitirir.
    pub fn fail(&mut self, id: u64, error: RowError) {
        if let Some(op) = self.take(|o| o.id == id) {
            self.errors.insert(op.target, error);
        }
    }

    /// İstemci tarafı doğrulama hatası (eylem başlamadan).
    pub fn reject(&mut self, target: Target, error: RowError) {
        self.errors.insert(target, error);
    }

    /// Satırın hatasını siler (ör. form kapatıldı).
    pub fn clear_error(&mut self, target: &Target) -> bool {
        self.errors.remove(target).is_some()
    }
}

/// Olayın ardından yapılacak servis çağrısı ([`apply`] saf kalsın diye dışarıda).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FollowUp {
    /// Kayıtlı ağın şifresi güncellendi: bağlantıyı etkinleştir (aynı eylem, ikinci adım).
    Activate { id: u64, connection: String },
}

/// [`apply`] sonucu.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Applied {
    pub changed: bool,
    pub follow: Vec<FollowUp>,
}

fn saved_by_path<'a>(s: &'a NetworkState, path: &str) -> Option<&'a SavedConnection> {
    s.saved.iter().find(|c| c.path == path)
}

fn saved_wifi_by_ssid<'a>(s: &'a NetworkState, ssid: &[u8]) -> Option<&'a SavedConnection> {
    s.saved.iter().find(|c| c.kind == ConnectionKind::Wifi && c.ssid.as_deref() == Some(ssid))
}

/// Durumda gözlenen sonuçla biten eylemler (ör. Wi-Fi gerçekten kapandı, bağlantı etkinleşti).
fn settle(s: &mut Snapshot) -> bool {
    let NetState::Ready(st) = &s.state else { return false };
    let activated = |conn: Option<&SavedConnection>| conn.is_some_and(|c| active_state(st, c) == Some(ActiveState::Activated));
    let done: Vec<u64> = s
        .ops
        .iter()
        .filter(|op| match (&op.target, op.kind) {
            // Şifre adımı sürerken etkinleşme beklenmez (bağlantı eski hâliyle etkin olabilir).
            _ if op.then_activate.is_some() => false,
            // Etkinleştirme: crate kabul ettikten sonra, bağlantı etkinleşince.
            (_, kind) if kind.activates() && !op.acked => false,
            (Target::Ap(ssid), OpKind::Connect) => match &op.connection {
                Some(p) => activated(saved_by_path(st, p)),
                None => activated(saved_wifi_by_ssid(st, ssid)),
            },
            (Target::Hidden, OpKind::Hidden) => op.connection.as_deref().is_some_and(|p| activated(saved_by_path(st, p))),
            (Target::Conn(p), OpKind::Activate) => saved_by_path(st, p).is_none_or(|c| active_state(st, c) == Some(ActiveState::Activated)),
            (Target::Wifi, OpKind::Wifi(on)) => st.wifi_enabled == on,
            (Target::Conn(p), OpKind::Deactivate) => saved_by_path(st, p).is_none_or(|c| c.active.is_none()),
            (Target::Conn(p), OpKind::Forget) => saved_by_path(st, p).is_none(),
            (Target::Conn(p), OpKind::Autoconnect(on)) => saved_by_path(st, p).is_none_or(|c| c.autoconnect == on),
            (Target::Conn(p), OpKind::Metered(m)) => saved_by_path(st, p).is_none_or(|c| c.metered == m),
            _ => false,
        })
        .map(|o| o.id)
        .collect();
    s.ops.retain(|o| !done.contains(&o.id));
    !done.is_empty()
}

fn activation_error(reason: ActivationFailure) -> RowError {
    match reason {
        ActivationFailure::NoSecrets => RowError::NoSecrets,
        ActivationFailure::AuthFailed => RowError::AuthFailed,
        // `Other(_)` ve crate'in ileride ekleyeceği nedenler (`#[non_exhaustive]`).
        _ => RowError::ActivationFailed,
    }
}

/// Crate olayı → anlık görüntü (saf; testli).
pub fn apply(s: &mut Snapshot, event: NetworkEvent) -> Applied {
    let mut out = Applied::default();
    match event {
        NetworkEvent::State(state) => {
            let next = if state.available {
                NetState::Ready(state)
            } else {
                NetState::Unavailable(state.unavailable.unwrap_or(Unavailable::ServiceNotRunning))
            };
            if s.state != next {
                s.state = next;
                out.changed = true;
            }
            if let NetState::Ready(st) = &s.state {
                let connected: BTreeSet<&str> = st.devices.iter().filter(|d| d.connected).map(|d| d.path.as_str()).collect();
                let before = s.details.len();
                s.details.retain(|p, _| connected.contains(p.as_str()));
                out.changed |= before != s.details.len();
            } else {
                out.changed |= !s.details.is_empty() || !s.ops.is_empty();
                s.details.clear();
                s.ops.clear();
            }
            out.changed |= settle(s);
        }
        NetworkEvent::Failed { request, action, error } => {
            let ours = request.and_then(|r| s.take(|o| o.request == Some(r)));
            match (ours, error) {
                // polkit reddi: sessiz geri dönüş (durum gerçeği gösterir).
                (op, NmError::Denied) => {
                    tracing::info!(action, "ağ: izin reddedildi");
                    out.changed = op.is_some();
                }
                (Some(op), error) => {
                    tracing::info!(action, "ağ: eylem başarısız: {error:?}");
                    s.errors.insert(op.target, RowError::Nm(error));
                    out.changed = true;
                }
                // Arka plan isteği (tarama reddi — "az önce tarandı", Wi-Fi kapalı — ve ayrıntı
                // hatası) gösterilmez.
                (None, error) if matches!(action, "scan" | "device_details") => {
                    tracing::debug!(action, "ağ: arka plan isteği başarısız: {error:?}");
                }
                (None, error) => {
                    tracing::info!(action, "ağ: eylem başarısız: {error:?}");
                    s.failure = Some(error);
                    out.changed = true;
                }
            }
        }
        NetworkEvent::Done { request, action, object } => {
            let Some(op) = s.ops.iter_mut().find(|o| o.request == Some(request)) else {
                return out;
            };
            tracing::debug!(action, "ağ: eylem kabul edildi");
            out.changed = true;
            if let Some(connection) = op.then_activate.take() {
                // Şifre güncellendi: ikinci adım (etkinleştir) dışarıda başlar.
                op.request = None;
                op.connection = Some(connection.clone());
                out.follow.push(FollowUp::Activate { id: op.id, connection });
            } else if op.kind.activates() {
                op.acked = true;
                if let Some(p) = object {
                    op.connection = Some(p);
                }
                // Gizli ağın kayıtlı bağlantısı bilinmiyorsa etkinleşme izlenemez: kabulde biter.
                if op.kind == OpKind::Hidden && op.connection.is_none() {
                    let id = op.id;
                    s.finish(id);
                }
                settle(s);
            } else {
                let id = op.id;
                s.finish(id);
            }
        }
        NetworkEvent::ActivationFailed { connection, reason } => {
            tracing::debug!("ağ: etkinleştirme başarısız: {reason:?}");
            let st = s.state.ready();
            let matches = |o: &Op| match connection.as_deref() {
                Some(c) => {
                    o.connection.as_deref() == Some(c)
                        || matches!(&o.target, Target::Ap(ssid) if st.and_then(|st| saved_by_path(st, c)).is_some_and(|c| c.ssid.as_deref() == Some(ssid.as_slice())))
                }
                None => false,
            };
            let candidate = |o: &Op| o.kind.activates() && o.then_activate.is_none();
            let id = s
                .ops
                .iter()
                .find(|o| candidate(o) && matches(o))
                .or_else(|| {
                    let mut c = s.ops.iter().filter(|o| candidate(o));
                    match (c.next(), c.next()) {
                        (Some(only), None) => Some(only),
                        _ => None,
                    }
                })
                .map(|o| o.id);
            if let Some(id) = id {
                s.fail(id, activation_error(reason));
                out.changed = true;
            }
        }
        // Aynı ayrıntı yeniden çizdirmez (aşağıdaki `_` koluna düşer).
        NetworkEvent::DeviceDetails(d) if s.details.get(&d.path) != Some(&d) => {
            s.details.insert(d.path.clone(), d);
            out.changed = true;
        }
        // Ajansız istemciye gizli bilgi isteği gelmez; gelse de yok sayılır. Crate'in ileride
        // ekleyeceği olaylar da (`#[non_exhaustive]`) yok sayılır.
        _ => {}
    }
    out
}

/// Şifre bu güvenlik türü için geçerli mi (istemci doğrulaması; ara kopya yok).
pub fn check_password(security: Security, password: &[u8]) -> bool {
    std::str::from_utf8(password).is_ok_and(|p| valid_secret(security, p))
}

/// Gizli ağ adı (SSID) geçerli mi: 1–32 bayt.
pub fn valid_ssid(ssid: &str) -> bool {
    (1..=nuxut_nm::MAX_SSID).contains(&ssid.len())
}

// ---- Saf görünümler (testli) --------------------------------------------------------------------

/// Wi-Fi sayfasının ana durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiStatus {
    /// NM'de Wi-Fi aygıtı yok.
    NoDevice,
    /// Donanım (rfkill) anahtarı kapalı.
    HardwareOff,
    /// Yazılımla kapalı.
    Off,
    On,
}

pub fn wifi_status(s: &NetworkState) -> WifiStatus {
    if s.wifi_devices.is_empty() {
        WifiStatus::NoDevice
    } else if !s.wifi_hardware_enabled {
        WifiStatus::HardwareOff
    } else if !s.wifi_enabled {
        WifiStatus::Off
    } else {
        WifiStatus::On
    }
}

/// Bağlı Wi-Fi ağı (Wi-Fi açıkken).
pub fn connected_ap(s: &NetworkState) -> Option<&AccessPoint> {
    (wifi_status(s) == WifiStatus::On).then(|| s.active_access_point()).flatten()
}

/// Bağlı ağın kayıtlı bağlantısı (kes/ayrıntı için).
pub fn connected_saved(s: &NetworkState) -> Option<&SavedConnection> {
    connected_ap(s).and_then(|ap| saved_wifi_by_ssid(s, &ap.ssid_bytes))
}

/// Görünen ağlar: bağlı olan hariç, sinyale göre (güçlü önce), eşitlikte ada göre. Wi-Fi
/// kapalıysa boş.
pub fn visible_networks(s: &NetworkState) -> Vec<&AccessPoint> {
    if wifi_status(s) != WifiStatus::On {
        return Vec::new();
    }
    let mut v: Vec<&AccessPoint> = s.access_points.iter().filter(|a| !a.active).collect();
    v.sort_by(|a, b| b.strength.cmp(&a.strength).then_with(|| a.ssid.to_lowercase().cmp(&b.ssid.to_lowercase())));
    v
}

/// Kayıtlı Wi-Fi bağlantısının yerinde durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedPresence {
    Connected,
    Connecting,
    /// Kapsama alanında (taramada görünüyor).
    InRange,
    OutOfRange,
}

/// Kayıtlı Wi-Fi bağlantıları (ada göre; crate sırası) ve durumları.
pub fn saved_wifi(s: &NetworkState) -> Vec<(&SavedConnection, SavedPresence)> {
    s.saved.iter().filter(|c| c.kind == ConnectionKind::Wifi).map(|c| (c, presence(s, c))).collect()
}

/// Kayıtlı ağın taramadaki erişim noktası (kapsamadaysa).
pub fn saved_ap<'a>(s: &'a NetworkState, c: &SavedConnection) -> Option<&'a AccessPoint> {
    let ssid = c.ssid.as_deref()?;
    s.access_points.iter().find(|a| a.ssid_bytes == ssid)
}

fn presence(s: &NetworkState, c: &SavedConnection) -> SavedPresence {
    match active_state(s, c) {
        Some(ActiveState::Activated) => SavedPresence::Connected,
        Some(ActiveState::Activating) => SavedPresence::Connecting,
        _ if saved_ap(s, c).is_some() => SavedPresence::InRange,
        _ => SavedPresence::OutOfRange,
    }
}

/// Kayıtlı bağlantının etkin durumu (etkin değilse `None`).
fn active_state(s: &NetworkState, c: &SavedConnection) -> Option<ActiveState> {
    let path = c.active.as_deref()?;
    Some(s.active.iter().find(|a| a.path == path).map_or(ActiveState::Activated, |a| a.state))
}

/// VPN bağlantısının durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnPresence {
    Active,
    Connecting,
    Inactive,
}

/// VPN ve WireGuard bağlantıları: etkinler önce, sonra ada göre (crate sırası korunur).
pub fn vpn_list(s: &NetworkState) -> Vec<(&SavedConnection, VpnPresence)> {
    let mut v: Vec<_> = s
        .vpns()
        .map(|c| {
            let p = match active_state(s, c) {
                Some(ActiveState::Activated) => VpnPresence::Active,
                Some(ActiveState::Activating) => VpnPresence::Connecting,
                _ => VpnPresence::Inactive,
            };
            (c, p)
        })
        .collect();
    // Kararlı sıralama: aynı gruptakiler crate'in ad sırasında kalır.
    v.sort_by_key(|(_, p)| *p != VpnPresence::Active && *p != VpnPresence::Connecting);
    v
}

/// Kablolu aygıtlar (Ethernet kartı).
pub fn wired_devices(s: &NetworkState) -> Vec<&nuxut_nm::NetDevice> {
    s.devices.iter().filter(|d| d.kind == DeviceKind::Ethernet).collect()
}

/// Bağlı ağın internet durumu (alt metin seçimi için).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reach {
    Internet,
    /// Esir portal: tarayıcıda giriş gerekiyor.
    Portal,
    /// Yerel ağ var, internet yok.
    Limited,
    /// NM bilmiyor (denetim kapalı).
    Unknown,
}

pub fn reach(s: &NetworkState) -> Reach {
    match s.connectivity {
        Connectivity::Full => Reach::Internet,
        Connectivity::Portal => Reach::Portal,
        Connectivity::Limited | Connectivity::None => Reach::Limited,
        Connectivity::Unknown => Reach::Unknown,
    }
}

/// Wi-Fi bandı (MHz → GHz kademesi).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Band {
    G2_4,
    G5,
    G6,
}

pub fn band(frequency_mhz: u32) -> Option<Band> {
    match frequency_mhz {
        2400..=2500 => Some(Band::G2_4),
        5150..=5925 => Some(Band::G5),
        5926..=7125 => Some(Band::G6),
        _ => None,
    }
}

/// Şifre gerektiren ya da kimlik doğrulamalı ağ mı (kilit işareti).
pub fn is_secured(security: Security) -> bool {
    !matches!(security, Security::Open | Security::Owe)
}

/// Ayrıntısı istenecek cihazlar: bağlı Wi-Fi ve kablolu cihazlar, (yol, etkin bağlantı).
pub fn detail_keys(s: &NetworkState) -> Vec<(String, Option<String>)> {
    s.devices
        .iter()
        .filter(|d| d.connected && matches!(d.kind, DeviceKind::Wifi | DeviceKind::Ethernet))
        .map(|d| (d.path.clone(), d.active_connection.clone()))
        .collect()
}

// ---- WireGuard dosyası --------------------------------------------------------------------------

/// `.conf` dosyasını boyut sınırıyla, tek sıfırlanan tampona okur (yeniden ayırma yok, ara
/// `String` yok) ve kopyasız [`Secret`]'e devreder.
pub fn read_conf(path: &Path) -> Result<Secret, ImportError> {
    const MAX: usize = nuxut_nm::wireguard::MAX_CONF_BYTES;
    let mut file = std::fs::File::open(path).map_err(|_| ImportError::Read)?;
    let meta = file.metadata().map_err(|_| ImportError::Read)?;
    if !meta.is_file() {
        return Err(ImportError::Read);
    }
    if meta.len() > MAX as u64 {
        return Err(ImportError::TooLarge);
    }
    // Sınırın bir fazlası: dosya okurken büyüdüyse fark edilir.
    let mut buf = Zeroizing::new(vec![0u8; MAX + 1]);
    let mut n = 0;
    while n < buf.len() {
        match file.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return Err(ImportError::Read),
        }
    }
    if n > MAX {
        return Err(ImportError::TooLarge);
    }
    buf.truncate(n);
    Secret::from_bytes(buf).ok_or(ImportError::NotUtf8)
}

/// Bağlantı adı = dosya adı (uzantısız). Geçersizse crate `InvalidName` döner.
pub fn conf_name(path: &Path) -> String {
    path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_owned()
}

// ---- Süreç geneli durum ve servis ---------------------------------------------------------------

static SNAPSHOT: Mutex<Option<Snapshot>> = Mutex::new(None);
static SERVICE: OnceLock<Mutex<Option<NetworkService>>> = OnceLock::new();
static STARTED: AtomicBool = AtomicBool::new(false);
/// Ayrıntısı istenmiş (cihaz, etkin bağlantı) çiftleri.
static DETAILS_ASKED: Mutex<Vec<(String, Option<String>)>> = Mutex::new(Vec::new());

/// Son anlık görüntü.
pub fn snapshot() -> Snapshot {
    SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default()
}

/// Anlık görüntüyü değiştirir; değiştiyse UI'a haber verir.
fn update<R>(f: impl FnOnce(&mut Snapshot) -> R) -> R {
    let r = {
        let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        f(s.get_or_insert_with(Snapshot::default))
    };
    emit(Event::Network);
    r
}

fn with_service<R>(f: impl FnOnce(&NetworkService) -> R) -> Option<R> {
    let m = SERVICE.get()?;
    let g = m.lock().unwrap_or_else(|e| e.into_inner());
    g.as_ref().map(f)
}

/// Bus kararı (saf; testli): oturum bus'ı istendiyse adres zorunlu (`None` → servis açılmaz).
pub fn bus_address(kind: BusKind, session_address: Option<&str>) -> Option<Option<String>> {
    match kind {
        BusKind::System => Some(None),
        BusKind::Session => session_address.filter(|a| !a.is_empty()).map(|a| Some(a.to_owned())),
    }
}

/// İstemci seçenekleri: ajansız (varsayılan), bus seçimi yalnız `test-support` özelliğiyle.
pub fn options() -> Option<NetworkOptions> {
    #[cfg(feature = "test-support")]
    let kind = nuxut_nm::bus_from_env();
    #[cfg(not(feature = "test-support"))]
    let kind = BusKind::System;
    let session = std::env::var("DBUS_SESSION_BUS_ADDRESS").ok();
    let address = bus_address(kind, session.as_deref())?;
    Some(NetworkOptions { bus: kind, bus_address: address, ..NetworkOptions::default() })
}

/// Sayfa ilk kurulurken: servisi bir kez başlatır (sonraki çağrılar bir şey yapmaz).
pub fn ensure() {
    // Birim testleri bus'a gitmez (ve thread açmaz).
    if cfg!(test) || STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let Some(options) = options() else {
        tracing::warn!("ağ: oturum bus'ı istendi ama adresi yok; servis açılmadı");
        update(|s| s.state = NetState::Failed);
        return;
    };
    let sink = |event: NetworkEvent| {
        let applied = {
            let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
            apply(s.get_or_insert_with(Snapshot::default), event)
        };
        for f in applied.follow {
            follow_up(f);
        }
        if applied.changed {
            emit(Event::Network);
            // Sayfa açıkken ilk durum (ya da Wi-Fi açılışı) gelince bekleyen tarama.
            try_scan();
            ask_details();
        }
    };
    match NetworkService::spawn(options, sink) {
        Ok(svc) => {
            let _ = SERVICE.set(Mutex::new(Some(svc)));
            // İlk durum servis kaydedilmeden gelmiş olabilir.
            try_scan();
            ask_details();
        }
        Err(e) => {
            tracing::warn!("ağ: servis thread'i açılamadı: {e}");
            update(|s| s.state = NetState::Failed);
        }
    }
}

/// Bağlı cihazların ayrıntısını (IP, DNS, hız) bir kez ister; etkin bağlantı değişince yeniden.
fn ask_details() {
    let Some(st) = snapshot().state.ready().cloned() else { return };
    let keys = detail_keys(&st);
    let mut asked = DETAILS_ASKED.lock().unwrap_or_else(|e| e.into_inner());
    asked.retain(|k| keys.contains(k));
    for k in keys {
        if !asked.contains(&k) && with_service(|svc| svc.device_details(&k.0)).flatten().is_some() {
            asked.push(k);
        }
    }
}

/// Ayrıntıyı yeniden iste (sayfa açıldı: DHCP adresi sonradan gelmiş olabilir).
pub fn refresh_details() {
    DETAILS_ASKED.lock().unwrap_or_else(|e| e.into_inner()).clear();
    ask_details();
}

fn follow_up(f: FollowUp) {
    match f {
        FollowUp::Activate { id, connection } => {
            // İstek kimliği sonuç olayından önce bağlansın diye anlık görüntü kilidi altında
            // gönderilir (sıra: SNAPSHOT → SERVICE; servis thread'i sink'te SERVICE tutmaz).
            {
                let mut g = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
                let s = g.get_or_insert_with(Snapshot::default);
                let Some(op) = s.ops.iter().find(|o| o.id == id) else { return };
                // Görünen ağ: ilk denemedeki gibi cihaz + erişim noktasıyla (crate kayıtlı
                // bağlantıyı etkinleştirir); görünmüyorsa NM cihazı kendi seçer.
                let ap = match (&op.target, s.state.ready()) {
                    (Target::Ap(ssid), Some(st)) => st.access_points.iter().find(|a| a.ssid_bytes == *ssid).map(|a| a.path.clone()),
                    _ => None,
                };
                let request = with_service(|svc| match &ap {
                    Some(ap) => svc.connect_access_point(ap, None),
                    None => svc.activate(&connection),
                })
                .flatten();
                s.attach(id, request);
            }
            emit(Event::Network);
        }
    }
}

/// Eylemi başlatır ve crate isteğini gönderir; hedef meşgulse `None`. `then_activate`: şifre
/// güncellemesinin ardından etkinleştirilecek bağlantı. İstek kimliği, sonuç olayı gelmeden
/// eyleme bağlansın diye anlık görüntü kilidi altında gönderilir (gönderim bloklamaz:
/// `try_send`). Zaman aşımı zamanlayıcısı (güvenlik ağı) kurulur.
fn start(
    target: Target,
    kind: OpKind,
    then_activate: Option<String>,
    call: impl FnOnce(&NetworkService) -> Option<RequestId>,
) -> Option<u64> {
    if cfg!(test) {
        return None;
    }
    let id = {
        let mut g = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        let s = g.get_or_insert_with(Snapshot::default);
        let id = s.begin(target, kind)?;
        if let Some(op) = s.ops.iter_mut().find(|o| o.id == id) {
            op.then_activate = then_activate;
        }
        let request = with_service(call).flatten();
        s.attach(id, request);
        id
    };
    emit(Event::Network);
    watchdog(id, kind);
    Some(id)
}

/// Güvenlik ağı: sonuç hiç gelmezse eylem sessizce düşer.
fn watchdog(id: u64, kind: OpKind) {
    let _ = std::thread::Builder::new().name("nuxut-net-op".into()).spawn(move || {
        std::thread::sleep(kind.timeout());
        let mut s = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        if s.as_mut().is_some_and(|s| s.finish(id)) {
            drop(s);
            tracing::debug!("ağ: eylem zaman aşımıyla düştü");
            emit(Event::Network);
        }
    });
}

fn reject(target: Target, error: RowError) {
    if !cfg!(test) {
        update(|s| s.reject(target, error));
    }
}

/// Wi-Fi aç/kapa (gerçek durum gelince biter).
pub fn set_wifi(on: bool) {
    start(Target::Wifi, OpKind::Wifi(on), None, |svc| svc.set_wifi_enabled(on));
}

/// Görünen ağa bağlanır. `password`: parola alanının tek kopyası (yoksa kayıtlı/açık ağ).
/// Kayıtlı ağa şifreyle bağlanmak kayıtlı şifreyi yerinde günceller (`set_wifi_password`;
/// otomatik bağlanma/ölçülü korunur), sonra bağlantıyı etkinleştirir.
pub fn connect(ssid: &[u8], password: Option<Zeroizing<Vec<u8>>>) {
    let target = Target::Ap(ssid.to_vec());
    let Some(st) = snapshot().state.ready().cloned() else { return };
    let Some(ap) = st.access_points.iter().find(|a| a.ssid_bytes == ssid) else {
        return reject(target, RowError::Nm(NmError::ApUnknown));
    };
    if ap.security == Security::Enterprise {
        return reject(target, RowError::Nm(NmError::Enterprise));
    }
    if let Some(pw) = &password
        && !check_password(ap.security, pw)
    {
        return reject(target, RowError::InvalidPassword);
    }
    let secret = password.and_then(Secret::from_bytes);
    let saved = saved_wifi_by_ssid(&st, ssid).map(|c| c.path.clone());
    match (saved, secret) {
        // Yalnız wpa-psk/sae; WEP/kurumsal/açık → crate `Unsupported`, "her seferinde sor" →
        // `SecretNotSaved` (satırda metin; bağlantı değişmez).
        (Some(conn), Some(secret)) => {
            start(target, OpKind::Connect, Some(conn.clone()), |svc| svc.set_wifi_password(&conn, secret));
        }
        (_, secret) => {
            let path = ap.path.clone();
            start(target, OpKind::Connect, None, |svc| svc.connect_access_point(&path, secret));
        }
    }
}

/// Gizli ağa bağlanır (yeni bağlantı).
pub fn connect_hidden(ssid: &str, security: Security, password: Option<Zeroizing<Vec<u8>>>) {
    if !valid_ssid(ssid) {
        return reject(Target::Hidden, RowError::InvalidSsid);
    }
    let password = if security.needs_password() { password } else { None };
    match &password {
        Some(pw) if !check_password(security, pw) => return reject(Target::Hidden, RowError::InvalidPassword),
        None if security.needs_password() => return reject(Target::Hidden, RowError::InvalidPassword),
        _ => {}
    }
    let secret = password.and_then(Secret::from_bytes);
    start(Target::Hidden, OpKind::Hidden, None, |svc| svc.connect_hidden(ssid.as_bytes(), security, secret));
}

/// Etkin bağlantıyı kapatır (bağlı Wi-Fi ya da VPN; kayıtlı bağlantı yolu).
pub fn disconnect(connection: &str) {
    let Some(active) = snapshot().state.ready().and_then(|st| saved_by_path(st, connection)).and_then(|c| c.active.clone()) else {
        return;
    };
    start(Target::Conn(connection.to_owned()), OpKind::Deactivate, None, |svc| svc.deactivate(&active));
}

/// VPN aç/kapa.
pub fn set_vpn(connection: &str, on: bool) {
    if !on {
        return disconnect(connection);
    }
    start(Target::Conn(connection.to_owned()), OpKind::Activate, None, |svc| svc.activate(connection));
}

/// Kayıtlı bağlantıyı siler (Wi-Fi "Unut", VPN "Sil"; onay UI'da).
pub fn forget(connection: &str) {
    start(Target::Conn(connection.to_owned()), OpKind::Forget, None, |svc| svc.forget(connection));
}

pub fn set_autoconnect(connection: &str, on: bool) {
    start(Target::Conn(connection.to_owned()), OpKind::Autoconnect(on), None, |svc| svc.set_autoconnect(connection, on));
}

pub fn set_metered(connection: &str, metered: Metered) {
    start(Target::Conn(connection.to_owned()), OpKind::Metered(metered), None, |svc| svc.set_metered(connection, metered));
}

/// İçe aktarma eylemini başlatır (dosya seçici ya da doğrudan dosya); crate isteği dosya
/// okununca bağlanır.
fn begin_import() -> Option<u64> {
    if cfg!(test) {
        return None;
    }
    let id = update(|s| s.begin(Target::Import, OpKind::Import))?;
    watchdog(id, OpKind::Import);
    Some(id)
}

/// "WireGuard içe aktar…": framework dosya seçicisi (`.conf` filtresi); seçim gelince
/// [`import_file`].
pub fn pick_wireguard() {
    let Some(id) = begin_import() else { return };
    let filter = nuxut::portal::Filter::new(nuxut::i18n::t!("settings-vpn-import-filter")).glob("*.conf");
    let mut options = nuxut::portal::OpenFileOptions::new(nuxut::i18n::t!("settings-vpn-import-dialog-title"));
    options.filters = vec![filter];
    portal(id).open_file(&nuxut::portal::ParentWindow::none(), options);
}

/// Süreç geneli portal istemcisi. Yanıt hangi eyleme ait: son başlatılan içe aktarma.
fn portal(id: u64) -> &'static nuxut::portal::Portal {
    static PORTAL: OnceLock<nuxut::portal::Portal> = OnceLock::new();
    static PICK: Mutex<u64> = Mutex::new(0);
    *PICK.lock().unwrap_or_else(|e| e.into_inner()) = id;
    PORTAL.get_or_init(|| {
        nuxut::portal::Portal::new(|response| {
            let id = *PICK.lock().unwrap_or_else(|e| e.into_inner());
            match response.result {
                Ok(paths) => match paths.into_iter().next() {
                    Some(p) => import_with(id, &p),
                    None => {
                        update(|s| s.finish(id));
                    }
                },
                Err(nuxut::portal::PortalError::Cancelled) => {
                    update(|s| s.finish(id));
                }
                Err(e) => {
                    tracing::warn!("ağ: dosya seçici: {e}");
                    update(|s| s.fail(id, RowError::Import(ImportError::Portal)));
                }
            }
        })
    })
}

/// Seçilen dosyayı içe aktarır (canlı test adımı da bunu çağırır; dosya seçici olmadan).
pub fn import_file(path: &Path) {
    let Some(id) = begin_import() else { return };
    import_with(id, path);
}

fn import_with(id: u64, path: &Path) {
    let secret = match read_conf(path) {
        Ok(s) => s,
        Err(e) => return update(|s| s.fail(id, RowError::Import(e))),
    };
    let name = conf_name(path);
    // İstek kimliği sonuçtan önce bağlansın: kilit altında gönderilir (bkz. [`start`]).
    {
        let mut g = SNAPSHOT.lock().unwrap_or_else(|e| e.into_inner());
        let s = g.get_or_insert_with(Snapshot::default);
        match with_service(|svc| svc.import_wireguard(&name, &secret)) {
            Some(Ok(import)) => {
                if s.attach(id, import.request) && !import.ignored.is_empty() {
                    // Yalnız anahtar adları (değer asla); log'a sayı.
                    tracing::info!(count = import.ignored.len(), "ağ: WireGuard: içe aktarılmayan wg-quick anahtarları");
                    s.import_ignored = import.ignored;
                }
            }
            Some(Err(e)) => s.fail(id, RowError::Import(ImportError::Parse(e))),
            None => s.fail(id, RowError::Nm(NmError::NmUnavailable)),
        }
    }
    emit(Event::Network);
}

/// Satır hatasını siler (form kapatıldı).
pub fn clear_error(target: &Target) {
    if !cfg!(test) && snapshot().error(target).is_some() {
        update(|s| s.clear_error(target));
    }
}

/// Wi-Fi sayfası görünüyor mu (tarama yalnız o zaman).
static WIFI_VISIBLE: AtomicBool = AtomicBool::new(false);
/// Bu görünürlük döneminde tarama istendi mi.
static SCANNED: AtomicBool = AtomicBool::new(false);

/// Tarama kararı (saf; testli): sayfa görünür, bu dönemde taranmadı, Wi-Fi açık.
pub fn should_scan(visible: bool, scanned: bool, s: &Snapshot) -> bool {
    visible && !scanned && s.state.ready().is_some_and(|s| wifi_status(s) == WifiStatus::On)
}

/// Gezinmede `App` çağırır: Wi-Fi sayfası görünür oldu/olmadı. Görünür olunca bir kez taranır
/// (servis henüz hazır değilse ilk durum gelince — [`try_scan`]) ve ayrıntı tazelenir.
pub fn set_wifi_visible(on: bool) {
    tracing::debug!("ağ: Wi-Fi sayfası görünür: {on}");
    let was = WIFI_VISIBLE.swap(on, Ordering::Relaxed);
    if on && !was {
        SCANNED.store(false, Ordering::Relaxed);
        refresh_details();
    }
    try_scan();
}

/// Son taramanın zamanı (odak dönüşünde art arda tarama olmasın).
static LAST_SCAN: Mutex<Option<Instant>> = Mutex::new(None);
/// Odak dönüşünde yeniden tarama için en az aralık.
pub const RESCAN_AFTER: Duration = Duration::from_secs(10);

/// Pencere odağı döndü: Wi-Fi sayfası görünüyorsa ve son tarama [`RESCAN_AFTER`]'dan eskiyse
/// yeniden tara (açılıştaki ilk odak ikinci tarama yapmaz).
pub fn rescan() {
    let last = *LAST_SCAN.lock().unwrap_or_else(|e| e.into_inner());
    if last.is_none_or(|t| t.elapsed() >= RESCAN_AFTER) {
        SCANNED.store(false, Ordering::Relaxed);
    }
    try_scan();
}

/// Bekleyen tarama varsa ve koşullar uygunsa iste (servis her durum değişiminde ve gezinmede çağırır).
pub fn try_scan() {
    if !should_scan(WIFI_VISIBLE.load(Ordering::Relaxed), SCANNED.load(Ordering::Relaxed), &snapshot()) {
        return;
    }
    if SERVICE.get().is_none() {
        return;
    }
    // UI ve servis thread'i aynı anda gelebilir: hakkı önce kapan ister (tek tarama).
    if SCANNED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return;
    }
    if with_service(NetworkService::scan).flatten().is_some() {
        tracing::debug!("ağ: tarama istendi");
        *LAST_SCAN.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());
    } else {
        SCANNED.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use nuxut_nm::{ActiveConnection, NetDevice, NmState, WifiDevice, WireGuardErrorKind};

    use super::*;

    fn ap(ssid: &str, strength: u8, security: Security, active: bool, saved: bool) -> AccessPoint {
        AccessPoint {
            path: format!("/ap/{ssid}"),
            ssid: ssid.into(),
            ssid_bytes: ssid.as_bytes().to_vec(),
            strength,
            security,
            frequency: 5180,
            active,
            saved,
        }
    }

    fn saved(id: &str, kind: ConnectionKind, ssid: Option<&str>, active: Option<&str>) -> SavedConnection {
        SavedConnection {
            path: format!("/s/{id}"),
            id: id.into(),
            uuid: format!("u-{id}"),
            kind,
            ssid: ssid.map(|s| s.as_bytes().to_vec()),
            active: active.map(str::to_owned),
            autoconnect: true,
            metered: Metered::default(),
        }
    }

    fn active(path: &str, kind: ConnectionKind, state: ActiveState) -> ActiveConnection {
        ActiveConnection { path: path.into(), id: String::new(), kind, state, connection: String::new() }
    }

    fn base() -> NetworkState {
        NetworkState {
            available: true,
            state: NmState::ConnectedGlobal,
            connectivity: Connectivity::Full,
            networking_enabled: true,
            wifi_enabled: true,
            wifi_hardware_enabled: true,
            wifi_devices: vec![WifiDevice { path: "/d/1".into(), interface: "wlan0".into(), active_ap: None }],
            ..Default::default()
        }
    }

    fn ready(st: NetworkState) -> Snapshot {
        Snapshot { state: NetState::Ready(st), ..Default::default() }
    }

    fn state(s: &mut Snapshot, st: NetworkState) -> Applied {
        apply(s, NetworkEvent::State(st))
    }

    #[test]
    fn status_order_device_hardware_software() {
        let mut s = base();
        assert_eq!(wifi_status(&s), WifiStatus::On);
        s.wifi_enabled = false;
        assert_eq!(wifi_status(&s), WifiStatus::Off);
        s.wifi_hardware_enabled = false;
        assert_eq!(wifi_status(&s), WifiStatus::HardwareOff, "donanım anahtarı yazılımdan önce");
        s.wifi_devices.clear();
        assert_eq!(wifi_status(&s), WifiStatus::NoDevice);
    }

    #[test]
    fn visible_sorted_by_signal_without_connected() {
        let mut s = base();
        s.access_points = vec![
            ap("Ev", 40, Security::WpaPsk, true, true),
            ap("b", 60, Security::Open, false, false),
            ap("A", 60, Security::Sae, false, true),
            ap("Kafe", 90, Security::WpaPsk, false, false),
        ];
        let names: Vec<_> = visible_networks(&s).iter().map(|a| a.ssid.as_str()).collect();
        assert_eq!(names, ["Kafe", "A", "b"], "güçlü önce, eşitlikte ad (büyük/küçük harf duyarsız), bağlı yok");
        assert_eq!(connected_ap(&s).map(|a| a.ssid.as_str()), Some("Ev"));
        s.wifi_enabled = false;
        assert!(visible_networks(&s).is_empty(), "kapalıyken liste yok");
        assert!(connected_ap(&s).is_none());
    }

    #[test]
    fn saved_presence() {
        let mut s = base();
        s.access_points = vec![ap("Kafe", 50, Security::WpaPsk, false, true)];
        s.active = vec![active("/a/1", ConnectionKind::Wifi, ActiveState::Activating)];
        s.saved = vec![
            saved("Ev", ConnectionKind::Wifi, Some("Ev"), Some("/a/1")),
            saved("Kafe", ConnectionKind::Wifi, Some("Kafe"), None),
            saved("Ofis", ConnectionKind::Wifi, Some("Ofis"), None),
            saved("Kablolu", ConnectionKind::Ethernet, None, None),
            saved("İş", ConnectionKind::WireGuard, None, None),
        ];
        let got: Vec<_> = saved_wifi(&s).iter().map(|(c, p)| (c.id.as_str(), *p)).collect();
        assert_eq!(
            got,
            [("Ev", SavedPresence::Connecting), ("Kafe", SavedPresence::InRange), ("Ofis", SavedPresence::OutOfRange)],
            "yalnız Wi-Fi; kablolu/VPN yok"
        );
    }

    #[test]
    fn vpn_active_first() {
        let mut s = base();
        s.active = vec![active("/a/2", ConnectionKind::WireGuard, ActiveState::Activated)];
        s.saved = vec![
            saved("Ana", ConnectionKind::Vpn, None, None),
            saved("Ev", ConnectionKind::Wifi, Some("Ev"), None),
            saved("İş", ConnectionKind::WireGuard, None, Some("/a/2")),
        ];
        let got: Vec<_> = vpn_list(&s).iter().map(|(c, p)| (c.id.as_str(), *p)).collect();
        assert_eq!(got, [("İş", VpnPresence::Active), ("Ana", VpnPresence::Inactive)]);
        s.saved.retain(|c| !c.kind.is_vpn());
        assert!(vpn_list(&s).is_empty());
    }

    #[test]
    fn events_fold_into_snapshot() {
        let mut snap = Snapshot::default();
        assert!(!apply(&mut snap, NetworkEvent::Failed { request: None, action: "scan", error: NmError::NoWifiDevice }).changed, "tarama hatası sessiz");
        assert_eq!(snap.failure, None);
        assert!(state(&mut snap, NetworkState::default()).changed);
        assert_eq!(snap.state, NetState::Unavailable(Unavailable::ServiceNotRunning), "neden yoksa NM yok sayılır");
        let bus = NetworkState { unavailable: Some(Unavailable::BusUnreachable), ..Default::default() };
        assert!(state(&mut snap, bus).changed);
        assert_eq!(snap.state, NetState::Unavailable(Unavailable::BusUnreachable), "bus yok ayrı");
        assert!(state(&mut snap, base()).changed);
        assert!(snap.state.ready().is_some());
        assert!(!state(&mut snap, base()).changed, "aynı durum yeniden çizdirmez");
        assert!(apply(&mut snap, NetworkEvent::Failed { request: Some(RequestId(4)), action: "connect", error: NmError::ApUnknown }).changed);
        assert_eq!(snap.failure, Some(NmError::ApUnknown), "hedefsiz hata sayfa notuna");
    }

    #[test]
    fn test_bus_never_falls_back() {
        assert_eq!(bus_address(BusKind::System, Some("unix:path=/x")), Some(None), "sistem bus'ı: adres yok sayılır");
        assert_eq!(bus_address(BusKind::Session, Some("unix:path=/tmp/b")), Some(Some("unix:path=/tmp/b".into())), "adres açıkça verilir");
        assert_eq!(bus_address(BusKind::Session, None), None, "adres yoksa servis açılmaz (sistem/varsayılan oturum yoluna düşülmez)");
        assert_eq!(bus_address(BusKind::Session, Some("")), None);
    }

    #[cfg(not(feature = "test-support"))]
    #[test]
    fn product_build_always_uses_system_bus() {
        // Ürün yapısında ortam bus seçimini değiştiremez (crate test-support kapalı).
        let o = options().expect("sistem bus'ı");
        assert_eq!((o.bus, o.bus_address), (BusKind::System, None));
    }

    #[test]
    fn options_are_agentless() {
        assert_eq!(options().map(|o| o.agent_id), Some(None), "Ayarlar asla ajan olmaz (2026-10-13)");
    }

    #[test]
    fn scan_only_when_visible_once_and_wifi_on() {
        let on = ready(base());
        assert!(should_scan(true, false, &on));
        assert!(!should_scan(false, false, &on), "sayfa görünmüyorken tarama yok");
        assert!(!should_scan(true, true, &on), "bu dönemde zaten tarandı");
        assert!(!should_scan(true, false, &Snapshot::default()), "servis hazır değil");
        let mut off = base();
        off.wifi_enabled = false;
        assert!(!should_scan(true, false, &ready(off)), "Wi-Fi kapalı");
    }

    #[test]
    fn bands_and_security() {
        assert_eq!(band(2412), Some(Band::G2_4));
        assert_eq!(band(5180), Some(Band::G5));
        assert_eq!(band(5955), Some(Band::G6));
        assert_eq!(band(0), None);
        assert!(!is_secured(Security::Open) && !is_secured(Security::Owe));
        assert!(is_secured(Security::WpaPsk) && is_secured(Security::Sae) && is_secured(Security::Enterprise) && is_secured(Security::Wep));
    }

    #[test]
    fn reach_from_connectivity() {
        let mut s = base();
        assert_eq!(reach(&s), Reach::Internet);
        s.connectivity = Connectivity::Portal;
        assert_eq!(reach(&s), Reach::Portal);
        s.connectivity = Connectivity::None;
        assert_eq!(reach(&s), Reach::Limited);
        s.connectivity = Connectivity::Unknown;
        assert_eq!(reach(&s), Reach::Unknown);
    }

    // ---- Eylem durum makinesi ----

    fn started(s: &mut Snapshot, target: Target, kind: OpKind, request: u64) -> u64 {
        let id = s.begin(target, kind).unwrap();
        assert!(s.attach(id, Some(RequestId(request))));
        id
    }

    fn done(request: u64, action: &'static str, object: Option<&str>) -> NetworkEvent {
        NetworkEvent::Done { request: RequestId(request), action, object: object.map(str::to_owned) }
    }

    fn failed(request: Option<u64>, action: &'static str, error: NmError) -> NetworkEvent {
        NetworkEvent::Failed { request: request.map(RequestId), action, error }
    }

    /// Bağlan → yanlış/eksik şifre (ActivationFailed) → hata satırda, şifre alanı → yeniden dene.
    #[test]
    fn connect_password_error_retry() {
        let mut st = base();
        st.access_points = vec![ap("Kafe", 50, Security::WpaPsk, false, false), ap("X", 50, Security::Open, false, false)];
        let mut s = ready(st.clone());
        let t = Target::Ap(b"Kafe".to_vec());
        let id = started(&mut s, t.clone(), OpKind::Connect, 1);
        assert!(s.busy(&t));
        assert_eq!(s.begin(t.clone(), OpKind::Connect), None, "hedef meşgul");
        assert!(s.begin(Target::Ap(b"X".to_vec()), OpKind::Connect).is_some(), "farklı hedefte aynı tür eylem bekleyebilir (istek kimliği ayırır)");
        s.ops.retain(|o| o.target == t);
        // NM kabul etti, sonra vazgeçti (bağlantı yolu biliniyor ya da tek aday).
        assert!(apply(&mut s, done(1, "connect", Some("/s/Kafe"))).changed);
        assert!(s.busy(&t), "etkinleştirme Done ile bitmez");
        assert!(apply(&mut s, NetworkEvent::ActivationFailed { connection: Some("/s/Kafe".into()), reason: ActivationFailure::NoSecrets }).changed);
        assert!(!s.busy(&t));
        assert_eq!(s.error(&t), Some(&RowError::NoSecrets));
        assert!(s.error(&t).unwrap().wants_password(), "şifre alanı yeniden açılır");
        // Yeniden dene: eski hata silinir.
        let id2 = started(&mut s, t.clone(), OpKind::Connect, 2);
        assert!(id2 > id && s.error(&t).is_none());
        // Kabul edilmeden önce durumda "etkin" görünmesi bitirmez.
        st.saved = vec![saved("Kafe", ConnectionKind::Wifi, Some("Kafe"), Some("/a/9"))];
        st.active = vec![active("/a/9", ConnectionKind::Wifi, ActiveState::Activated)];
        state(&mut s, st.clone());
        assert!(s.busy(&t), "Done gelmeden bitmez");
        st.active[0].state = ActiveState::Activating;
        state(&mut s, st.clone());
        apply(&mut s, done(2, "connect", Some("/s/Kafe")));
        assert!(s.busy(&t), "etkinleşirken bekler");
        st.active[0].state = ActiveState::Activated;
        state(&mut s, st);
        assert!(!s.busy(&t) && s.error(&t).is_none());
    }

    /// Kayıtlı ağın şifresi yerinde güncellenir (`set_wifi_password`), `Done` → etkinleştir
    /// (FollowUp), etkinleşince biter. Eski "unut + yeni bağlantı" yolu yok.
    #[test]
    fn saved_password_update_then_activate() {
        let mut st = base();
        st.access_points = vec![ap("Ev", 50, Security::WpaPsk, true, true)];
        st.saved = vec![saved("Ev", ConnectionKind::Wifi, Some("Ev"), Some("/a/1"))];
        st.active = vec![active("/a/1", ConnectionKind::Wifi, ActiveState::Activated)];
        let mut s = ready(st.clone());
        let t = Target::Ap(b"Ev".to_vec());
        let id = s.begin(t.clone(), OpKind::Connect).unwrap();
        s.ops[0].then_activate = Some("/s/Ev".into());
        s.attach(id, Some(RequestId(7)));
        assert!(state(&mut s, st.clone()).follow.is_empty());
        assert!(s.busy(&t), "şifre adımı sürerken eski etkin bağlantı eylemi bitirmez");
        let a = apply(&mut s, done(7, "wifi_password", Some("/s/Ev")));
        assert_eq!(a.follow, [FollowUp::Activate { id, connection: "/s/Ev".into() }]);
        let op = s.op(&t).unwrap();
        assert_eq!((op.request, op.then_activate.as_deref(), op.connection.as_deref()), (None, None, Some("/s/Ev")), "ikinci adım: etkinleştir");
        assert!(s.attach(id, Some(RequestId(8))));
        apply(&mut s, done(8, "activate", None));
        assert!(!s.busy(&t), "zaten etkin: kabulde biter");

        // Şifre güncellenemez (WEP / her seferinde sor): satırda metin, etkinleştirme yok.
        for e in [NmError::Unsupported, NmError::SecretNotSaved] {
            let id = s.begin(t.clone(), OpKind::Connect).unwrap();
            s.ops[0].then_activate = Some("/s/Ev".into());
            s.attach(id, Some(RequestId(9)));
            let a = apply(&mut s, failed(Some(9), "wifi_password", e.clone()));
            assert!(a.follow.is_empty());
            assert_eq!(s.error(&t), Some(&RowError::Nm(e)));
            assert!(!s.error(&t).unwrap().wants_password(), "şifre alanı açılmaz");
        }
    }

    /// polkit reddi (`NmError::Denied`) sessiz; metinden tanıma yok.
    #[test]
    fn denied_is_silent() {
        let mut s = ready(base());
        started(&mut s, Target::Wifi, OpKind::Wifi(false), 1);
        apply(&mut s, failed(Some(1), "wifi", NmError::Denied));
        assert!(s.ops.is_empty() && s.errors.is_empty() && s.failure.is_none(), "polkit reddi: sessiz geri dönüş");
        assert!(!apply(&mut s, failed(None, "forget", NmError::Denied)).changed, "hedefsiz red de sessiz");
        assert!(s.failure.is_none());
        started(&mut s, Target::Wifi, OpKind::Wifi(false), 2);
        let text = NmError::CallFailed("org.freedesktop.NetworkManager.PermissionDenied: Not authorized".into());
        apply(&mut s, failed(Some(2), "wifi", text.clone()));
        assert_eq!(s.error(&Target::Wifi), Some(&RowError::Nm(text)), "metin sezgisi yok: CallFailed gösterilir");
        started(&mut s, Target::Wifi, OpKind::Wifi(false), 3);
        apply(&mut s, failed(Some(3), "wifi", NmError::NoWifiDevice));
        assert_eq!(s.error(&Target::Wifi), Some(&RowError::Nm(NmError::NoWifiDevice)), "gerçek hata satırda");
    }

    /// Sonuç olayı istek kimliğiyle eşlenir: aynı eylem adından iki bekleyen karışmaz; bilinmeyen
    /// kimlik bekleyeni bitirmez.
    #[test]
    fn results_match_by_request() {
        let mut st = base();
        st.saved = vec![saved("A", ConnectionKind::Wifi, Some("A"), None), saved("B", ConnectionKind::Wifi, Some("B"), None)];
        let mut s = ready(st);
        let (a, b) = (Target::Conn("/s/A".into()), Target::Conn("/s/B".into()));
        started(&mut s, a.clone(), OpKind::Metered(Metered::Yes), 11);
        started(&mut s, b.clone(), OpKind::Metered(Metered::Yes), 12);
        assert!(!apply(&mut s, done(99, "metered", None)).changed, "bilinmeyen kimlik");
        apply(&mut s, failed(Some(12), "metered", NmError::ConnectionUnknown));
        assert!(s.busy(&a), "A etkilenmez");
        assert_eq!(s.error(&b), Some(&RowError::Nm(NmError::ConnectionUnknown)));
        apply(&mut s, done(11, "metered", None));
        assert!(!s.busy(&a) && s.error(&a).is_none());
        // Kuyruğa girmeyen istek: satır hatası.
        let id = s.begin(a.clone(), OpKind::Forget).unwrap();
        assert!(!s.attach(id, None));
        assert_eq!(s.error(&a), Some(&RowError::Nm(NmError::NmUnavailable)));
        // Hedefsiz hata (kimlik yok/eşleşmez) sayfa notuna; arka plan hataları sessiz.
        assert!(!apply(&mut s, failed(Some(5), "scan", NmError::NoWifiDevice)).changed, "tarama hatası sessiz");
        assert!(!apply(&mut s, failed(None, "device_details", NmError::DeviceUnknown)).changed);
        assert!(apply(&mut s, failed(None, "connect", NmError::ApUnknown)).changed);
        assert_eq!(s.failure, Some(NmError::ApUnknown));
    }

    #[test]
    fn wifi_toggle_ends_on_real_state() {
        let mut s = ready(base());
        started(&mut s, Target::Wifi, OpKind::Wifi(false), 1);
        state(&mut s, base());
        assert!(s.busy(&Target::Wifi), "durum hâlâ açık: iyimser değil, bekler");
        let mut off = base();
        off.wifi_enabled = false;
        state(&mut s, off);
        assert!(!s.busy(&Target::Wifi));
        started(&mut s, Target::Wifi, OpKind::Wifi(true), 2);
        apply(&mut s, done(2, "wifi", None));
        assert!(!s.busy(&Target::Wifi), "Done de bitirir (durum ondan önce yayınlanır)");
    }

    #[test]
    fn forget_autoconnect_metered_finish() {
        let mut st = base();
        st.saved = vec![saved("Ev", ConnectionKind::Wifi, Some("Ev"), None)];
        let mut s = ready(st.clone());
        let t = Target::Conn("/s/Ev".into());
        started(&mut s, t.clone(), OpKind::Metered(Metered::Yes), 1);
        assert_eq!(s.begin(t.clone(), OpKind::Forget), None, "hedef meşgul");
        assert!(apply(&mut s, done(1, "metered", None)).changed);
        assert!(!s.busy(&t), "Done bitirir");
        started(&mut s, t.clone(), OpKind::Autoconnect(false), 2);
        st.saved[0].autoconnect = false;
        state(&mut s, st.clone());
        assert!(!s.busy(&t), "durumda gözlenen değer de bitirir");
        started(&mut s, t.clone(), OpKind::Forget, 3);
        st.saved.clear();
        state(&mut s, st);
        assert!(!s.busy(&t));
    }

    #[test]
    fn vpn_activation_failure_targets_connection() {
        let mut st = base();
        st.saved = vec![saved("İş", ConnectionKind::WireGuard, None, None), saved("Ev", ConnectionKind::Wifi, Some("Ev"), None)];
        let mut s = ready(st);
        let t = Target::Conn("/s/İş".into());
        started(&mut s, t.clone(), OpKind::Activate, 1);
        started(&mut s, Target::Ap(b"Ev".to_vec()), OpKind::Connect, 2);
        apply(&mut s, done(1, "activate", None));
        assert!(s.busy(&t), "VPN etkinleşene dek bekler");
        apply(&mut s, NetworkEvent::ActivationFailed { connection: Some("/s/İş".into()), reason: ActivationFailure::AuthFailed });
        assert_eq!(s.error(&t), Some(&RowError::AuthFailed));
        assert!(s.busy(&Target::Ap(b"Ev".to_vec())), "diğer bekleyen etkilenmez");
        apply(&mut s, NetworkEvent::ActivationFailed { connection: Some("/s/Ev".into()), reason: ActivationFailure::NoSecrets });
        assert_eq!(s.error(&Target::Ap(b"Ev".to_vec())), Some(&RowError::NoSecrets), "kayıtlı yol → SSID hedefi");
        // Bekleyen yokken gelen etkinleştirme hatası (kendiliğinden bağlanma) yok sayılır.
        assert!(!apply(&mut s, NetworkEvent::ActivationFailed { connection: None, reason: ActivationFailure::Other(5) }).changed);
    }

    /// Gizli ağ: `Done.object` kayıtlı bağlantıyı verir, etkinleşince biter; yol yoksa kabulde.
    #[test]
    fn hidden_waits_for_activation() {
        let mut st = base();
        let mut s = ready(st.clone());
        started(&mut s, Target::Hidden, OpKind::Hidden, 1);
        apply(&mut s, done(1, "connect_hidden", Some("/s/Gizli")));
        assert!(s.busy(&Target::Hidden));
        st.saved = vec![saved("Gizli", ConnectionKind::Wifi, Some("Gizli"), Some("/a/3"))];
        st.active = vec![active("/a/3", ConnectionKind::Wifi, ActiveState::Activated)];
        state(&mut s, st);
        assert!(!s.busy(&Target::Hidden));
        started(&mut s, Target::Hidden, OpKind::Hidden, 2);
        apply(&mut s, done(2, "connect_hidden", None));
        assert!(!s.busy(&Target::Hidden));
    }

    #[test]
    fn finish_and_unavailable_clear_ops() {
        let mut s = ready(base());
        let id = s.begin(Target::Import, OpKind::Import).unwrap();
        assert!(s.finish(id) && !s.finish(id), "iptal/zaman aşımı sessiz");
        s.import_ignored = vec!["PostUp"];
        s.begin(Target::Import, OpKind::Import).unwrap();
        assert!(s.import_ignored.is_empty(), "yeni içe aktarma eski notu siler");
        s.begin(Target::Hidden, OpKind::Hidden).unwrap();
        state(&mut s, NetworkState::default());
        assert!(s.ops.is_empty(), "NM gidince bekleyen eylem kalmaz");
    }

    #[test]
    fn details_follow_connected_devices() {
        let mut st = base();
        st.devices = vec![
            NetDevice { path: "/d/eth".into(), interface: "enp1s0".into(), kind: DeviceKind::Ethernet, managed: true, connected: true, active_connection: Some("/a/w".into()) },
            NetDevice { path: "/d/1".into(), interface: "wlan0".into(), kind: DeviceKind::Wifi, managed: true, connected: false, active_connection: None },
        ];
        assert_eq!(detail_keys(&st), [("/d/eth".to_owned(), Some("/a/w".to_owned()))]);
        let mut s = ready(st.clone());
        let d = DeviceDetails { path: "/d/eth".into(), speed_mbps: Some(1000), ..Default::default() };
        assert!(apply(&mut s, NetworkEvent::DeviceDetails(d.clone())).changed);
        assert!(!apply(&mut s, NetworkEvent::DeviceDetails(d)).changed);
        st.devices[0].connected = false;
        state(&mut s, st);
        assert!(s.details.is_empty(), "bağlantı kopunca ayrıntı düşer");
    }

    #[test]
    fn password_and_ssid_validation() {
        assert!(check_password(Security::WpaPsk, b"12345678"));
        assert!(!check_password(Security::WpaPsk, b"1234567"), "WPA en az 8");
        assert!(!check_password(Security::WpaPsk, &[0xff; 9]), "UTF-8 değil");
        assert!(check_password(Security::Sae, b"x"));
        assert!(!check_password(Security::Open, b"x"));
        assert!(valid_ssid("Ev") && valid_ssid(&"a".repeat(32)));
        assert!(!valid_ssid("") && !valid_ssid(&"a".repeat(33)));
    }

    #[test]
    fn target_debug_is_redacted() {
        let d = format!("{:?}", Snapshot { errors: BTreeMap::from([(Target::Ap(b"GizliAg".to_vec()), RowError::NoSecrets)]), ..Default::default() });
        assert!(!d.contains("GizliAg") && !d.contains("71, 105"), "{d}");
    }

    #[test]
    fn conf_read_limits() {
        let dir = tempfile::tempdir().unwrap();
        let ok = dir.path().join("wg-ev.conf");
        std::fs::write(&ok, "[Interface]\n").unwrap();
        assert_eq!(read_conf(&ok).map(|s| s.len()), Ok(12));
        assert_eq!(conf_name(&ok), "wg-ev");
        let big = dir.path().join("big.conf");
        std::fs::write(&big, vec![b'#'; nuxut_nm::wireguard::MAX_CONF_BYTES + 1]).unwrap();
        assert_eq!(read_conf(&big).err(), Some(ImportError::TooLarge));
        let edge = dir.path().join("edge.conf");
        std::fs::write(&edge, vec![b'#'; nuxut_nm::wireguard::MAX_CONF_BYTES]).unwrap();
        assert!(read_conf(&edge).is_ok(), "sınır dahil");
        let bin = dir.path().join("bin.conf");
        std::fs::write(&bin, [0xff, 0xfe]).unwrap();
        assert_eq!(read_conf(&bin).err(), Some(ImportError::NotUtf8));
        assert_eq!(read_conf(&dir.path().join("yok.conf")).err(), Some(ImportError::Read));
        assert_eq!(read_conf(dir.path()).err(), Some(ImportError::Read), "dizin");
        let parse = ImportError::Parse(WireGuardError { line: 3, kind: WireGuardErrorKind::InvalidKey });
        assert_ne!(parse, ImportError::Read);
    }
}
