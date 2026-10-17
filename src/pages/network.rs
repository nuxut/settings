//! Bağlantılar › Wi-Fi ve VPN sayfaları (t114 v1; `docs/design/network-pages.md`). Değerler
//! NetworkManager'da ([`crate::services::network`]); bloklar servis anlık görüntüsünden çizer,
//! eylemleri servise iletir. Sistem eylemleri geri al geçmişine girmez (dosya yazımı değil).
//!
//! - [`NetworkNote`]: kartsız üst not — bus yok / NM yok / servis açılamadı / hedefsiz hata.
//! - [`WifiStatusRow`]: Wi-Fi aç/kapa (canlı, iyimser değil: bekleyen eylem ilerleme gösterir).
//! - [`NetListBlock`]: bağlı ağ, görünen ağlar, gizli ağ, kayıtlı ağlar, kablolu, VPN, içe aktarma.
//!   Satır tıklanınca altında bir panel açılır (aynı anda bir tane): şifre, bağlı ağ ayrıntısı,
//!   kayıtlı ağ ayarları, VPN sil, gizli ağ formu.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::{
    Button, ButtonVariant, ConfirmButton, ConfirmEvent, Icon, IconName, IconView, Interaction, Label, Notice, NoticeKind, Segment,
    SegmentedButton, Spinner, TextInput, TextInputEvent, TextRole, Toggle, Ui, text_style,
};
use nuxut::{Point, Rect, Theme};
use nuxut_nm::{ConnectionKind, DeviceDetails, Metered, NmError, Security, Unavailable, WireGuardErrorKind};
use zeroize::Zeroizing;

use super::datetime::RowText;
use super::fiyu::{FiyuEmpty, Spot};
use super::notice::{notice_height, place_notice};
use crate::block::{Action, Block, Env};
use crate::registry::{NetList, SettingDesc};
use crate::rows::{row_height_for, row_padding};
use crate::services::network::{
    self, Band, ImportError, NetState, Reach, RowError, SavedPresence, Snapshot, Target, VpnPresence, WifiStatus, band, connected_ap,
    connected_saved, is_secured, reach, saved_ap, saved_wifi, visible_networks, vpn_list, wifi_status, wired_devices,
};
use crate::store::Store;

/// Alt metin parçalarının ayırıcısı.
const SEP: &str = " · ";

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// Sayfa üstü not.
pub fn note_text(s: &Snapshot) -> Option<String> {
    match &s.state {
        NetState::Unavailable(Unavailable::BusUnreachable) => Some(t!("settings-network-no-bus")),
        // `ServiceNotRunning` ve crate'in ileride ekleyeceği nedenler (`#[non_exhaustive]`).
        NetState::Unavailable(_) => Some(t!("settings-network-missing")),
        NetState::Failed => Some(t!("settings-network-failed")),
        NetState::Ready(_) => s.failure.as_ref().map(error_text),
        NetState::Pending => None,
    }
}

/// `NmError` → kullanıcı metni. `CallFailed` ayrıntısı NM'nin (temizlenmiş) metnidir: gösterilir,
/// loglanmaz.
pub fn error_text(e: &NmError) -> String {
    match e {
        NmError::CallFailed(detail) => t!("settings-network-error-call-failed", detail = detail.as_str()),
        NmError::NmUnavailable => t!("settings-network-error-unavailable"),
        NmError::ConnectionUnknown => t!("settings-network-error-connection-unknown"),
        NmError::ConnectionInactive => t!("settings-network-error-connection-inactive"),
        NmError::ApUnknown => t!("settings-network-error-ap-unknown"),
        NmError::Enterprise => t!("settings-network-error-enterprise"),
        NmError::InvalidPassword => t!("settings-network-error-invalid-password"),
        NmError::NoWifiDevice => t!("settings-network-error-no-device"),
        NmError::InvalidSsid => t!("settings-network-error-invalid-ssid"),
        NmError::DeviceUnknown => t!("settings-network-error-device-unknown"),
        // polkit reddi normalde sessizdir (satıra/nota gelmez); yine de metni var.
        NmError::Denied => t!("settings-network-error-denied"),
        NmError::Unsupported => t!("settings-network-error-unsupported"),
        NmError::SecretNotSaved => t!("settings-network-error-secret-not-saved"),
        // Crate'in ileride ekleyeceği hatalar (`#[non_exhaustive]`): genel metin + `Display`.
        e => t!("settings-network-error-call-failed", detail = e.to_string()),
    }
}

/// İçe aktarma hatası → metin (satır numarası varsa metinde; değer asla).
pub fn import_error_text(e: ImportError) -> String {
    match e {
        ImportError::Portal => t!("settings-vpn-import-error-portal"),
        ImportError::Read => t!("settings-vpn-import-error-read"),
        ImportError::TooLarge => t!("settings-vpn-import-error-too-large"),
        ImportError::NotUtf8 => t!("settings-vpn-import-error-not-text"),
        ImportError::Parse(p) => {
            let line = p.line as i64;
            match p.kind {
                WireGuardErrorKind::TooLarge => t!("settings-vpn-import-error-too-large"),
                WireGuardErrorKind::MissingPrivateKey => t!("settings-vpn-import-error-private-key"),
                WireGuardErrorKind::MissingPublicKey => t!("settings-vpn-import-error-public-key", line = line),
                WireGuardErrorKind::InvalidKey => t!("settings-vpn-import-error-key", line = line),
                WireGuardErrorKind::InvalidAddress | WireGuardErrorKind::InvalidNumber | WireGuardErrorKind::InvalidEndpoint => {
                    t!("settings-vpn-import-error-value", line = line)
                }
                WireGuardErrorKind::TooMany => t!("settings-vpn-import-error-too-many"),
                WireGuardErrorKind::InvalidName => t!("settings-vpn-import-error-name"),
                WireGuardErrorKind::Syntax
                | WireGuardErrorKind::UnknownSection
                | WireGuardErrorKind::KeyOutsideSection
                | WireGuardErrorKind::UnknownKey
                | WireGuardErrorKind::DuplicateKey
                | WireGuardErrorKind::MissingInterface
                | WireGuardErrorKind::DuplicateInterface => t!("settings-vpn-import-error-syntax", line = line),
                // Crate'in ileride ekleyeceği türler (`#[non_exhaustive]`): genel sözdizimi metni.
                _ => t!("settings-vpn-import-error-syntax", line = line),
            }
        }
    }
}

/// İçe aktarılmayan wg-quick anahtarları notu: "Şu ayarlar içe aktarılmadı: PostUp, PostDown".
/// Anahtar adları dosyadaki biçimiyle (çevrilmez; yapılandırma sözcüğü), değer asla.
pub fn import_ignored_text(keys: &[&str]) -> String {
    t!("settings-vpn-import-ignored", keys = keys.join(", "))
}

/// Satır hatası → metin. `vpn`: VPN satırı (gizli bilgi/giriş metni farklı).
pub fn row_error_text(e: &RowError, vpn: bool) -> String {
    match e {
        RowError::Nm(e) => error_text(e),
        RowError::NoSecrets if vpn => t!("settings-vpn-error-no-secrets"),
        RowError::NoSecrets => t!("settings-wifi-error-no-secrets"),
        RowError::AuthFailed if vpn => t!("settings-vpn-error-auth-failed"),
        RowError::AuthFailed => t!("settings-wifi-error-auth-failed"),
        RowError::ActivationFailed => t!("settings-network-error-activation"),
        RowError::InvalidPassword => t!("settings-network-error-invalid-password"),
        RowError::InvalidSsid => t!("settings-network-error-invalid-ssid"),
        RowError::Import(e) => import_error_text(*e),
    }
}

pub fn security_text(s: Security) -> String {
    match s {
        Security::Open => t!("settings-network-security-open"),
        Security::Owe => t!("settings-network-security-owe"),
        Security::Wep => t!("settings-network-security-wep"),
        Security::WpaPsk => t!("settings-network-security-wpa"),
        Security::Sae => t!("settings-network-security-sae"),
        Security::Enterprise => t!("settings-network-security-enterprise"),
    }
}

pub fn band_text(b: Band) -> String {
    match b {
        Band::G2_4 => t!("settings-network-band-2g"),
        Band::G5 => t!("settings-network-band-5g"),
        Band::G6 => t!("settings-network-band-6g"),
    }
}

/// Wi-Fi durum satırının alt metni.
pub fn status_text(status: WifiStatus) -> String {
    match status {
        WifiStatus::NoDevice => t!("settings-wifi-status-no-device"),
        WifiStatus::HardwareOff => t!("settings-wifi-status-hardware-off"),
        WifiStatus::Off => t!("settings-wifi-status-off"),
        WifiStatus::On => t!("settings-wifi-status-on"),
    }
}

fn reach_text(r: Reach) -> String {
    match r {
        Reach::Internet | Reach::Unknown => t!("settings-wifi-connected"),
        Reach::Portal => t!("settings-wifi-connected-portal"),
        Reach::Limited => t!("settings-wifi-connected-limited"),
    }
}

/// Ağ ayrıntısı: "WPA/WPA2 · 5 GHz" (bant bilinmiyorsa yalnız güvenlik).
fn ap_detail(security: Security, frequency: u32) -> Vec<String> {
    let mut parts = vec![security_text(security)];
    parts.extend(band(frequency).map(band_text));
    parts
}

/// Ölçülü bağlantı seçenekleri: Otomatik / Evet / Hayır (NM tahminleri Otomatik sayılır).
pub const METERED: [Metered; 3] = [Metered::Unknown, Metered::Yes, Metered::No];

pub fn metered_index(m: Metered) -> usize {
    match m {
        Metered::Yes => 1,
        Metered::No => 2,
        Metered::Unknown | Metered::GuessYes | Metered::GuessNo => 0,
    }
}

/// Gizli ağ formunun güvenlik seçenekleri (v1: kişisel; WEP ve 802.1X yok).
pub const HIDDEN_SECURITY: [Security; 3] = [Security::Open, Security::WpaPsk, Security::Sae];

/// Cihaz ayrıntısı → (etiket, değer) satırları. `speed`: hızı da göster (kablolu).
pub fn detail_lines(d: &DeviceDetails, speed: bool) -> Vec<(String, String)> {
    let mut v = Vec::new();
    if !d.ipv4.is_empty() {
        v.push((t!("settings-network-detail-ipv4"), d.ipv4.iter().map(|n| format!("{}/{}", n.addr, n.prefix)).collect::<Vec<_>>().join(", ")));
    }
    if let Some(ip) = d.ipv6.first() {
        v.push((t!("settings-network-detail-ipv6"), format!("{}/{}", ip.addr, ip.prefix)));
    }
    if !d.dns.is_empty() {
        v.push((t!("settings-network-detail-dns"), d.dns.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")));
    }
    if speed && let Some(mbps) = d.speed_mbps.filter(|s| *s > 0) {
        v.push((t!("settings-network-detail-speed"), t!("settings-network-speed-mbps", mbps = mbps as i64)));
    }
    v
}

/// Satır kimliği (panel açık durumu bununla eşlenir).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowKey {
    Ap(Vec<u8>),
    Current(Vec<u8>),
    Saved(String),
    Vpn(String),
    Wired(String),
    Hidden,
    Import,
    /// Bilgi satırı (ör. "Görünen ağ yok").
    Info,
}

/// Satır altında açılan panelin türü.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelKind {
    /// Şifre alanı + Bağlan/Vazgeç.
    Password { security: Security },
    /// Kurumsal ağ: "desteklenmiyor (v2)" notu, bağlan yok.
    Enterprise,
    /// Bağlı ağ: IP/DNS (salt okunur) + Bağlantıyı kes (`connection`: kayıtlı bağlantı).
    Current { connection: Option<String> },
    /// Kayıtlı ağ: otomatik bağlan, ölçülü, unut.
    Saved { connection: String, autoconnect: bool, metered: Metered },
    /// VPN: sil.
    Vpn { connection: String },
    /// Gizli ağ formu.
    Hidden,
}

/// Satıra basınca ne olur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Press {
    None,
    /// Doğrudan bağlan (açık ya da kayıtlı ağ).
    Connect,
    Open(PanelKind),
    /// Dosya seçici (WireGuard içe aktar).
    Pick,
}

/// Bir liste satırının verisi (görünümden bağımsız; testli).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowData {
    pub key: RowKey,
    /// Eylem hedefi (meşgul/hata).
    pub target: Option<Target>,
    pub icon: IconName,
    /// Etkin/bağlı (ikon dolu ve vurgu renginde).
    pub active: bool,
    pub title: String,
    pub subtitle: String,
    /// Şifreli ağ kilidi.
    pub lock: bool,
    /// Sağdaki kısa etiket ("Kayıtlı", "Etkin").
    pub tag: Option<String>,
    /// Eylem sürüyor: ilerleme, düğmeler kapalı.
    pub busy: bool,
    /// Satır hatası (alt metnin yerine, hata renginde).
    pub error: Option<String>,
    /// Hata şifre alanını yeniden açsın.
    pub wants_password: bool,
    /// Sağdaki anahtar (VPN aç/kapa).
    pub switch: Option<bool>,
    pub press: Press,
    /// Ayrıntı satırları (kablolu kartta her zaman; bağlı ağda panel açıkken).
    pub details: Vec<(String, String)>,
}

impl RowData {
    fn new(key: RowKey, icon: IconName, title: String) -> Self {
        Self {
            key,
            target: None,
            icon,
            active: false,
            title,
            subtitle: String::new(),
            lock: false,
            tag: None,
            busy: false,
            error: None,
            wants_password: false,
            switch: None,
            press: Press::None,
            details: Vec::new(),
        }
    }

    /// Hedefin bekleyen eylemi ve hatası.
    fn with_target(mut self, s: &Snapshot, target: Target, vpn: bool) -> Self {
        self.busy = s.busy(&target);
        if let Some(e) = s.error(&target) {
            self.error = Some(row_error_text(e, vpn));
            self.wants_password = e.wants_password();
        }
        self.target = Some(target);
        self
    }
}

/// Listenin içeriği.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListContent {
    /// Kart hiç çizilmez (servis hazır değil ya da bu durumda anlamsız).
    Hidden,
    /// Kart yalnız bir açıklama satırı taşır.
    Empty(String),
    Rows(Vec<RowData>),
}

fn rows_or(rows: Vec<RowData>, empty: impl FnOnce() -> String) -> ListContent {
    if rows.is_empty() { ListContent::Empty(empty()) } else { ListContent::Rows(rows) }
}

/// Listenin içeriği, anlık görüntüden.
pub fn content(kind: NetList, s: &Snapshot) -> ListContent {
    let state = match &s.state {
        NetState::Ready(state) => state,
        // İlk okuma sürerken (kısa an) ana kartlar yerinde durur, sayfa boş görünmez ve zıplamaz.
        NetState::Pending if matches!(kind, NetList::Visible | NetList::Vpn) => return ListContent::Empty(t!("settings-network-reading")),
        _ => return ListContent::Hidden,
    };
    match kind {
        NetList::Current => {
            let Some(ap) = connected_ap(state) else { return ListContent::Hidden };
            let saved = connected_saved(state).map(|c| c.path.clone());
            let mut parts = vec![reach_text(reach(state))];
            parts.extend(ap_detail(ap.security, ap.frequency));
            let mut row = RowData::new(RowKey::Current(ap.ssid_bytes.clone()), IconName::wifi(ap.strength), ap.ssid.clone());
            row.active = true;
            row.subtitle = parts.join(SEP);
            row.lock = is_secured(ap.security);
            row.details = wifi_details(state, s);
            row.press = Press::Open(PanelKind::Current { connection: saved.clone() });
            if let Some(p) = saved {
                row = row.with_target(s, Target::Conn(p), false);
            }
            ListContent::Rows(vec![row])
        }
        NetList::Visible => match wifi_status(state) {
            WifiStatus::NoDevice => ListContent::Hidden,
            WifiStatus::HardwareOff | WifiStatus::Off => ListContent::Empty(t!("settings-wifi-visible-off")),
            WifiStatus::On => {
                let rows = visible_networks(state)
                    .into_iter()
                    .map(|ap| {
                        let mut row = RowData::new(RowKey::Ap(ap.ssid_bytes.clone()), IconName::wifi(ap.strength), ap.ssid.clone())
                            .with_target(s, Target::Ap(ap.ssid_bytes.clone()), false);
                        let saved = state.saved.iter().find(|c| c.kind == ConnectionKind::Wifi && c.ssid.as_deref() == Some(ap.ssid_bytes.as_slice()));
                        let connecting = saved.and_then(|c| c.active.as_ref()).is_some();
                        let mut parts = ap_detail(ap.security, ap.frequency);
                        if row.busy || connecting {
                            parts.insert(0, t!("settings-network-connecting"));
                        }
                        row.subtitle = parts.join(SEP);
                        row.lock = is_secured(ap.security);
                        row.tag = ap.saved.then(|| t!("settings-wifi-tag-saved"));
                        row.press = match ap.security {
                            Security::Enterprise => Press::Open(PanelKind::Enterprise),
                            Security::Open | Security::Owe => Press::Connect,
                            // Kayıtlı ağ kayıtlı şifreyle bağlanır; NM şifre yok/yanlış derse alan açılır.
                            _ if ap.saved && !row.wants_password => Press::Connect,
                            security => Press::Open(PanelKind::Password { security }),
                        };
                        row
                    })
                    .collect();
                rows_or(rows, || t!("settings-wifi-visible-empty"))
            }
        },
        NetList::Hidden => match wifi_status(state) {
            WifiStatus::On => {
                let mut row = RowData::new(RowKey::Hidden, IconName::Add, t!("settings-wifi-hidden-label")).with_target(s, Target::Hidden, false);
                if row.busy {
                    row.subtitle = t!("settings-network-connecting");
                }
                row.press = Press::Open(PanelKind::Hidden);
                ListContent::Rows(vec![row])
            }
            _ => ListContent::Hidden,
        },
        NetList::Saved => {
            let rows = saved_wifi(state)
                .into_iter()
                .map(|(c, p)| {
                    let strength = saved_ap(state, c).map(|a| a.strength);
                    let (subtitle, active) = match p {
                        SavedPresence::Connected => (t!("settings-wifi-connected"), true),
                        SavedPresence::Connecting => (t!("settings-network-connecting"), false),
                        SavedPresence::InRange => (t!("settings-wifi-saved-in-range"), false),
                        SavedPresence::OutOfRange => (t!("settings-wifi-saved-out-of-range"), false),
                    };
                    // Kapsama dışında sinyal yok: nötr "kayıtlı" ikonu (review notu).
                    let icon = strength.map_or(IconName::Bookmark, IconName::wifi);
                    let mut row = RowData::new(RowKey::Saved(c.path.clone()), icon, c.id.clone()).with_target(s, Target::Conn(c.path.clone()), false);
                    row.active = active;
                    row.subtitle = subtitle;
                    row.press = Press::Open(PanelKind::Saved { connection: c.path.clone(), autoconnect: c.autoconnect, metered: c.metered });
                    row
                })
                .collect();
            rows_or(rows, || t!("settings-wifi-saved-empty"))
        }
        NetList::Wired => {
            let rows: Vec<RowData> = wired_devices(state)
                .into_iter()
                .map(|d| {
                    let name = d
                        .active_connection
                        .as_deref()
                        .and_then(|a| state.active.iter().find(|x| x.path == a))
                        .map_or_else(|| d.interface.clone(), |a| a.id.clone());
                    let mut row = RowData::new(RowKey::Wired(d.path.clone()), IconName::Ethernet, name);
                    row.active = d.connected;
                    row.subtitle = if d.connected { t!("settings-wifi-connected") } else { t!("settings-wired-disconnected") };
                    row.details = s.details.get(&d.path).map(|x| detail_lines(x, true)).unwrap_or_default();
                    row
                })
                .collect();
            if rows.is_empty() { ListContent::Hidden } else { ListContent::Rows(rows) }
        }
        NetList::Vpn => {
            let rows = vpn_list(state)
                .into_iter()
                .map(|(c, p)| {
                    let kind = if c.kind == ConnectionKind::WireGuard { t!("settings-vpn-kind-wireguard") } else { t!("settings-vpn-kind-vpn") };
                    let mut row = RowData::new(RowKey::Vpn(c.path.clone()), IconName::Vpn, c.id.clone()).with_target(s, Target::Conn(c.path.clone()), true);
                    let mut parts = vec![kind];
                    if p == VpnPresence::Connecting {
                        parts.push(t!("settings-network-connecting"));
                    }
                    row.active = p == VpnPresence::Active;
                    row.subtitle = parts.join(SEP);
                    row.switch = Some(p != VpnPresence::Inactive);
                    row.press = Press::Open(PanelKind::Vpn { connection: c.path.clone() });
                    row
                })
                .collect();
            rows_or(rows, || t!("settings-vpn-empty"))
        }
        NetList::Import => {
            let mut row = RowData::new(RowKey::Import, IconName::Upload, t!("settings-vpn-import-label")).with_target(s, Target::Import, true);
            if row.busy {
                row.subtitle = t!("settings-vpn-import-busy");
            } else if !s.import_ignored.is_empty() {
                // Bilgi notu: içe aktarılmayan wg-quick anahtarlarının adları (değer asla).
                row.subtitle = import_ignored_text(&s.import_ignored);
            }
            row.press = Press::Pick;
            ListContent::Rows(vec![row])
        }
    }
}

/// Bağlı Wi-Fi'nin ayrıntısı (Wi-Fi cihazının IP/DNS'i).
fn wifi_details(state: &nuxut_nm::NetworkState, s: &Snapshot) -> Vec<(String, String)> {
    state
        .devices
        .iter()
        .find(|d| d.kind == nuxut_nm::DeviceKind::Wifi && d.connected)
        .and_then(|d| s.details.get(&d.path))
        .map(|d| detail_lines(d, false))
        .unwrap_or_default()
}

/// Listenin boşken maskot yeri: kayıtlı ağ yok / VPN yok → boş maskotu. Görünen ağlar (Wi-Fi
/// kapalı ya da çekim yok) ve bağlı ağ maskotsuz kalır: aynı sayfada en çok bir maskot.
pub fn list_spot(kind: NetList) -> Option<Spot> {
    match kind {
        NetList::Saved => Some(Spot::WifiSavedEmpty),
        NetList::Vpn => Some(Spot::VpnEmpty),
        _ => None,
    }
}

fn is_network(event: &crate::services::Event) -> bool {
    matches!(event, crate::services::Event::Network | crate::services::Event::LocaleChanged)
}

// ---- Üst not ------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct NetworkNote {
    desc: &'static SettingDesc,
    notice: Notice,
}

impl NetworkNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        network::ensure();
        let mut n = Self { desc, notice: Notice::new(theme, NoticeKind::Warning, "") };
        n.update();
        n
    }

    fn update(&mut self) {
        self.notice.set_text(&note_text(&network::snapshot()).unwrap_or_default());
    }
}

impl Block for NetworkNote {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        note_text(&network::snapshot()).is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        notice_height(&mut self.notice, env, width, 0.0)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        place_notice(&mut self.notice, env.theme, rect, 0.0);
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, _env: &mut Env) {
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if is_network(event) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.notice.draw(ui);
    }
}

// ---- Wi-Fi durumu -------------------------------------------------------------------------------

/// Wi-Fi satırı: başlık + durum, sağda anahtar. Anahtar gerçek durumu gösterir; basınca istek
/// gider, bekleme sırasında anahtar kapalı ve yanında ilerleme. Hata alt metinde.
#[derive(Debug)]
pub struct WifiStatusRow {
    desc: &'static SettingDesc,
    text: RowText,
    toggle: Toggle,
    spinner: Spinner,
    /// Aygıt yok: anahtar çizilmez.
    show_toggle: bool,
    busy: bool,
    rect: Rect,
}

impl WifiStatusRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        network::ensure();
        let mut b = Self {
            desc,
            text: RowText::new(theme, desc.label()),
            toggle: Toggle::new(theme, false),
            spinner: Spinner::new(theme),
            show_toggle: false,
            busy: false,
            rect: Rect::ZERO,
        };
        b.update(Instant::now());
        b
    }

    fn update(&mut self, now: Instant) {
        let s = network::snapshot();
        let status = s.state.ready().map(wifi_status);
        let error = s.error(&Target::Wifi).map(|e| row_error_text(e, false));
        let text = match (&s.state, status, &error) {
            (_, _, Some(e)) => e.clone(),
            (NetState::Pending, _, _) => t!("settings-network-reading"),
            (_, Some(st), _) => status_text(st),
            _ => String::new(),
        };
        self.text.set_subtitle(&text, error.is_some());
        self.busy = s.busy(&Target::Wifi);
        self.show_toggle = status.is_some_and(|st| st != WifiStatus::NoDevice);
        self.toggle.set_on(status == Some(WifiStatus::On), now);
        // Donanım anahtarı kapalıyken yazılımla açılamaz.
        self.toggle.set_enabled(!self.busy && matches!(status, Some(WifiStatus::On | WifiStatus::Off)), now);
    }
}

impl Block for WifiStatusRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        matches!(network::snapshot().state, NetState::Ready(_) | NetState::Pending)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (width - px * 4.0 - ts.width * 2.0).max(t.space(20)));
        row_height_for(t, text.max(ts.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let tr = Rect::new(rect.right() - px - ts.width, rect.center().y - ts.height * 0.5, ts.width, ts.height);
        self.toggle.set_rect(tr);
        let ss = self.spinner.preferred_size(t);
        self.spinner.set_rect(Rect::new(tr.x - t.space(3) - ss.width, rect.center().y - ss.height * 0.5, ss.width, ss.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        if !self.show_toggle {
            return false;
        }
        let r = self.toggle.pointer(e, env.now);
        if let Some(on) = r.event {
            network::set_wifi(on);
            // İyimser değil: anahtar gerçek duruma döner, bekleme ilerlemeyle görünür.
            self.update(env.now);
            return true;
        }
        r.repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.show_toggle { self.toggle.cursor(p) } else { None }
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_network(event) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.toggle.set_theme(theme, now);
        self.spinner.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let (px, _) = row_padding(ui.theme);
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.center().y - h * 0.5);
        if self.show_toggle {
            self.toggle.draw(ui);
        }
        if self.busy {
            self.spinner.draw(ui);
        }
    }
}

// ---- Listeler -----------------------------------------------------------------------------------

#[derive(Debug)]
struct RowView {
    data: RowData,
    icon: IconView,
    title: Label,
    subtitle: Label,
    lock: Option<IconView>,
    tag: Option<Label>,
    switch: Option<Toggle>,
    details: Vec<(Label, Label)>,
    hover: Interaction,
    /// Satırın üst (tıklanır) kısmı.
    head: Rect,
    head_height: f32,
    rect: Rect,
}

impl RowView {
    fn new(theme: &Theme, d: &RowData, now: Instant) -> Self {
        let caption = text_style(theme, TextRole::Caption).with_max_lines(1);
        let mut switch = d.switch.map(|on| Toggle::new(theme, on));
        if let Some(s) = &mut switch {
            s.set_enabled(!d.busy, now);
        }
        let subtitle = d.error.as_deref().unwrap_or(&d.subtitle);
        Self {
            data: d.clone(),
            icon: IconView::new(theme, Icon::named(d.icon)).filled(d.active),
            title: Label::new(d.title.as_str(), text_style(theme, TextRole::Body).with_max_lines(1)),
            subtitle: Label::new(subtitle, text_style(theme, TextRole::Caption).with_max_lines(2)),
            lock: d.lock.then(|| IconView::with_role(theme, Icon::named(IconName::Lock), TextRole::IconSmall)),
            tag: d.tag.as_deref().map(|s| Label::new(s, caption.clone())),
            switch,
            details: d.details.iter().map(|(k, v)| (Label::new(k.as_str(), caption.clone()), Label::new(v.as_str(), text_style(theme, TextRole::Body).with_max_lines(2)))).collect(),
            hover: Interaction::new(theme),
            head: Rect::ZERO,
            head_height: 0.0,
            rect: Rect::ZERO,
        }
    }

    /// Satıra basılabilir mi (hover katmanı, imleç).
    fn pressable(&self) -> bool {
        self.data.press != Press::None
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.title.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        self.subtitle.set_style(text_style(theme, TextRole::Caption).with_max_lines(2));
        if let Some(l) = &mut self.lock {
            l.set_theme(theme, now);
        }
        if let Some(t) = &mut self.tag {
            t.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
        }
        if let Some(s) = &mut self.switch {
            s.set_theme(theme, now);
        }
        for (k, v) in &mut self.details {
            k.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
            v.set_style(text_style(theme, TextRole::Body).with_max_lines(2));
        }
        self.hover.set_theme(theme, now);
    }
}

fn icon_side(theme: &Theme) -> f32 {
    text_style(theme, TextRole::Icon).size
}

fn small_icon_side(theme: &Theme) -> f32 {
    text_style(theme, TextRole::IconSmall).size
}

/// Panelin odaktaki metin alanı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Ssid,
    Password,
}

/// Satır altındaki açık panel.
#[derive(Debug)]
struct Panel {
    key: RowKey,
    kind: PanelKind,
    /// Panel bir eylem gönderdi (başarıyla bitince kapanır).
    submitted: bool,
    ssid: Option<TextInput>,
    security: Option<SegmentedButton>,
    password: Option<TextInput>,
    focus: Field,
    note: Option<Label>,
    auto_label: Option<Label>,
    autoconnect: Option<Toggle>,
    metered_label: Option<Label>,
    metered: Option<SegmentedButton>,
    primary: Option<Button>,
    secondary: Option<Button>,
    confirm: Option<ConfirmButton>,
    height: f32,
    rect: Rect,
}

impl Panel {
    fn new(theme: &Theme, key: RowKey, kind: PanelKind) -> Self {
        let mut p = Self {
            key,
            kind: kind.clone(),
            submitted: false,
            ssid: None,
            security: None,
            password: None,
            focus: Field::Password,
            note: None,
            auto_label: None,
            autoconnect: None,
            metered_label: None,
            metered: None,
            primary: None,
            secondary: None,
            confirm: None,
            height: 0.0,
            rect: Rect::ZERO,
        };
        let body = text_style(theme, TextRole::Body);
        let password = || TextInput::new(theme).password(true).with_placeholder(t!("settings-wifi-password-placeholder"));
        let connect = || Button::new(theme, ButtonVariant::Filled).with_label(theme, t!("settings-wifi-connect"));
        let cancel = || Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-wifi-cancel"));
        match kind {
            PanelKind::Password { .. } => {
                p.password = Some(password());
                p.primary = Some(connect());
                p.secondary = Some(cancel());
            }
            PanelKind::Enterprise => p.note = Some(Label::new(t!("settings-wifi-enterprise-note"), body)),
            PanelKind::Current { connection } => {
                if connection.is_some() {
                    p.primary = Some(Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::LinkOff)).with_label(theme, t!("settings-wifi-disconnect")));
                }
            }
            PanelKind::Saved { autoconnect, metered, .. } => {
                p.auto_label = Some(Label::new(t!("settings-wifi-autoconnect"), body.clone()));
                p.autoconnect = Some(Toggle::new(theme, autoconnect));
                p.metered_label = Some(Label::new(t!("settings-wifi-metered"), body));
                let segs = vec![Segment::new(t!("settings-wifi-metered-auto")), Segment::new(t!("settings-wifi-metered-yes")), Segment::new(t!("settings-wifi-metered-no"))];
                p.metered = Some(SegmentedButton::new(theme, segs, metered_index(metered)));
                p.confirm = Some(
                    ConfirmButton::new(theme, ButtonVariant::Text, Icon::named(IconName::Delete), t!("settings-wifi-forget-confirm"))
                        .with_label(theme, t!("settings-wifi-forget")),
                );
            }
            PanelKind::Vpn { .. } => {
                p.confirm = Some(
                    ConfirmButton::new(theme, ButtonVariant::Text, Icon::named(IconName::Delete), t!("settings-vpn-delete-confirm"))
                        .with_label(theme, t!("settings-vpn-delete")),
                );
            }
            PanelKind::Hidden => {
                p.ssid = Some(TextInput::new(theme).with_placeholder(t!("settings-wifi-hidden-ssid-placeholder")).with_max_bytes(nuxut_nm::MAX_SSID));
                let segs = HIDDEN_SECURITY.iter().map(|s| Segment::new(security_text(*s))).collect();
                p.security = Some(SegmentedButton::new(theme, segs, 1));
                p.password = Some(password());
                p.focus = Field::Ssid;
                p.primary = Some(connect());
                p.secondary = Some(cancel());
            }
        }
        p
    }

    fn hidden_security(&self) -> Security {
        self.security.as_ref().map_or(Security::WpaPsk, |s| HIDDEN_SECURITY[s.selected().min(HIDDEN_SECURITY.len() - 1)])
    }

    /// Şifre alanı görünür mü (gizli ağda "Açık" seçiliyse yok).
    fn shows_password(&self) -> bool {
        self.password.is_some() && (self.kind != PanelKind::Hidden || self.hidden_security().needs_password())
    }

    fn input_mut(&mut self, f: Field) -> Option<&mut TextInput> {
        match f {
            Field::Ssid => self.ssid.as_mut(),
            Field::Password if self.shows_password() => self.password.as_mut(),
            Field::Password => None,
        }
    }

    fn focused_input(&mut self) -> Option<&mut TextInput> {
        let f = self.focus;
        self.input_mut(f)
    }

    fn has_input(&self) -> bool {
        self.ssid.is_some() || self.shows_password()
    }

    fn set_busy(&mut self, busy: bool, now: Instant) {
        for b in [&mut self.primary, &mut self.secondary].into_iter().flatten() {
            b.set_enabled(!busy, now);
        }
        if let Some(c) = &mut self.confirm {
            c.set_enabled(!busy, now);
        }
        if let Some(t) = &mut self.autoconnect {
            t.set_enabled(!busy, now);
        }
        if let Some(m) = &mut self.metered {
            m.set_enabled(!busy, now);
        }
    }

    /// Kayıtlı ağ ayarları gerçek değere döner (iyimser değil).
    fn sync_saved(&mut self, kind: &PanelKind, now: Instant) {
        if let (PanelKind::Saved { autoconnect, metered, .. }, Some(t), Some(m)) = (kind, &mut self.autoconnect, &mut self.metered) {
            t.set_on(*autoconnect, now);
            m.set_selected(metered_index(*metered), now);
        }
        self.kind = kind.clone();
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for i in [&mut self.ssid, &mut self.password].into_iter().flatten() {
            i.set_theme(theme, now);
        }
        for s in [&mut self.security, &mut self.metered].into_iter().flatten() {
            s.set_theme(theme, now);
        }
        for l in [&mut self.note, &mut self.auto_label, &mut self.metered_label].into_iter().flatten() {
            l.set_style(text_style(theme, TextRole::Body));
        }
        if let Some(t) = &mut self.autoconnect {
            t.set_theme(theme, now);
        }
        for b in [&mut self.primary, &mut self.secondary].into_iter().flatten() {
            b.set_theme(theme, now);
        }
        if let Some(c) = &mut self.confirm {
            c.set_theme(theme, now);
        }
    }

    /// Yükseklik (`x`: içerik sol kenarı, `w`: genişlik). Yerleşim de burada (yüzey konumu
    /// `place`'te kaydırılır): bileşen dikdörtgenleri panele göre.
    fn layout(&mut self, env: &mut Env, x: f32, top: f32, w: f32) -> f32 {
        let t = env.theme;
        let gap = t.space(3);
        let mut y = top;
        if let Some(i) = &mut self.ssid {
            let h = i.preferred_size(t, w).height;
            i.set_rect(Rect::new(x, y, w, h));
            y += h + gap;
        }
        if let Some(s) = &mut self.security {
            let sz = s.preferred_size(env.text, t);
            s.set_rect(t, Rect::new(x, y, sz.width.min(w), sz.height));
            y += sz.height + gap;
        }
        if self.shows_password()
            && let Some(i) = &mut self.password
        {
            let h = i.preferred_size(t, w).height;
            i.set_rect(Rect::new(x, y, w, h));
            y += h + gap;
        }
        if let Some(n) = &mut self.note {
            n.set_max_width(Some(w));
            y += n.measure(env.text).height + gap;
        }
        if let (Some(l), Some(tg)) = (&mut self.auto_label, &mut self.autoconnect) {
            let ts = tg.preferred_size(t);
            let lh = l.measure(env.text).height;
            let h = ts.height.max(lh);
            tg.set_rect(Rect::new(x + w - ts.width, y + (h - ts.height) * 0.5, ts.width, ts.height));
            y += h + gap;
        }
        if let (Some(l), Some(m)) = (&mut self.metered_label, &mut self.metered) {
            let ms = m.preferred_size(env.text, t);
            let lh = l.measure(env.text).height;
            let h = ms.height.max(lh);
            m.set_rect(t, Rect::new(x + w - ms.width, y + (h - ms.height) * 0.5, ms.width, ms.height));
            y += h + gap;
        }
        // Düğmeler sağa yaslı: [ikincil] [birincil] ya da onay düğmesi.
        let mut right = x + w;
        let mut row_h: f32 = 0.0;
        if let Some(b) = &mut self.primary {
            let s = b.preferred_size(env.text, t);
            b.set_rect(Rect::new(right - s.width, y, s.width, s.height));
            right -= s.width + t.space(2);
            row_h = row_h.max(s.height);
        }
        if let Some(b) = &mut self.secondary {
            let s = b.preferred_size(env.text, t);
            b.set_rect(Rect::new(right - s.width, y, s.width, s.height));
            row_h = row_h.max(s.height);
        }
        if let Some(c) = &mut self.confirm {
            let s = c.preferred_size(env.text, t);
            c.set_rect(Rect::new(x + w - s.width, y, s.width, s.height));
            row_h = row_h.max(s.height);
        }
        y += row_h;
        if row_h == 0.0 {
            y -= gap;
        }
        y - top
    }

    fn draw(&mut self, ui: &mut Ui, x: f32) {
        let c = &ui.theme.colors;
        let (on_surface, variant) = (c.on_surface, c.on_surface_variant);
        for i in [&mut self.ssid].into_iter().flatten() {
            i.draw(ui);
        }
        if let Some(s) = &mut self.security {
            s.draw(ui);
        }
        if self.shows_password()
            && let Some(i) = &mut self.password
        {
            i.draw(ui);
        }
        if let Some(n) = &mut self.note {
            let y = self.rect.y + ui.theme.space(1);
            n.draw(ui.painter, Point::new(x, y), ui.fade(variant));
        }
        if let (Some(l), Some(tg)) = (&mut self.auto_label, &mut self.autoconnect) {
            let r = tg.rect();
            let h = l.measure_in(ui.painter).height;
            l.draw(ui.painter, Point::new(x, r.center().y - h * 0.5), ui.fade(on_surface));
            tg.draw(ui);
        }
        if let (Some(l), Some(m)) = (&mut self.metered_label, &mut self.metered) {
            let r = m.rect();
            let h = l.measure_in(ui.painter).height;
            l.draw(ui.painter, Point::new(x, r.center().y - h * 0.5), ui.fade(on_surface));
            m.draw(ui);
        }
        for b in [&mut self.primary, &mut self.secondary].into_iter().flatten() {
            b.draw(ui);
        }
        if let Some(cb) = &mut self.confirm {
            cb.draw(ui);
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        let inputs = [self.ssid.as_ref(), if self.shows_password() { self.password.as_ref() } else { None }];
        let mut v: Vec<Option<CursorShape>> = inputs.into_iter().flatten().map(|i| i.cursor(p)).collect();
        v.extend([self.primary.as_ref(), self.secondary.as_ref()].into_iter().flatten().map(|b| b.cursor(p)));
        v.extend(self.confirm.as_ref().map(|c| c.cursor(p)));
        v.extend(self.autoconnect.as_ref().map(|t| t.cursor(p)));
        v.extend([self.security.as_ref(), self.metered.as_ref()].into_iter().flatten().map(|s| s.cursor(p)));
        v.into_iter().flatten().next()
    }
}

/// Paneldeki girdinin istediği eylem.
#[derive(Debug, Clone, PartialEq)]
enum PanelAction {
    Submit,
    Close,
    Disconnect(String),
    Forget(String),
    Autoconnect(String, bool),
    Metered(String, Metered),
}

/// Ağ listesi kartı. Satırlar yalnızca veri değişince yeniden kurulur; açık panel korunur.
#[derive(Debug)]
pub struct NetListBlock {
    desc: &'static SettingDesc,
    kind: NetList,
    data: ListContent,
    rows: Vec<RowView>,
    empty: Label,
    /// Kayıtlı ağ / VPN listesi boş: metnin yerine maskotlu boş durum ([`list_spot`]).
    fiyu: Option<FiyuEmpty>,
    panel: Option<Panel>,
    spinner: Spinner,
    focused: bool,
    /// Canlı test: ilk açılabilir satırın panelini aç (`expand=`).
    demo: bool,
    rect: Rect,
}

impl NetListBlock {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, kind: NetList) -> Self {
        network::ensure();
        let mut b = Self {
            desc,
            kind,
            data: ListContent::Hidden,
            rows: Vec::new(),
            empty: Label::new("", text_style(theme, TextRole::Body)),
            fiyu: None,
            panel: None,
            spinner: Spinner::new(theme),
            focused: false,
            demo: false,
            rect: Rect::ZERO,
        };
        b.update(theme, Instant::now());
        b
    }

    fn update(&mut self, theme: &Theme, now: Instant) {
        self.apply(theme, content(self.kind, &network::snapshot()), now);
    }

    fn apply(&mut self, theme: &Theme, data: ListContent, now: Instant) {
        if data == self.data {
            return;
        }
        match &data {
            ListContent::Rows(rows) => self.rows = rows.iter().map(|d| RowView::new(theme, d, now)).collect(),
            ListContent::Empty(text) => {
                self.rows.clear();
                self.empty.set_text(text);
            }
            ListContent::Hidden => self.rows.clear(),
        }
        self.fiyu = match (&data, list_spot(self.kind)) {
            (ListContent::Empty(text), Some(spot)) => Some(match self.fiyu.take() {
                Some(mut f) => {
                    f.set_text(theme, text, None);
                    f
                }
                None => FiyuEmpty::new(theme, spot, text, None),
            }),
            _ => None,
        };
        self.data = data;
        self.sync_panel(theme, now);
    }

    /// Açık paneli satırla eşler: satır gittiyse ya da gönderilen eylem başarıyla bittiyse kapanır;
    /// şifre isteyen hata gelince ilgili satırın paneli açılır.
    fn sync_panel(&mut self, theme: &Theme, now: Instant) {
        if let Some(p) = &mut self.panel {
            match self.rows.iter().find(|r| r.data.key == p.key) {
                None => self.panel = None,
                Some(r) => {
                    let done = p.submitted && !r.data.busy && r.data.error.is_none();
                    match &r.data.press {
                        _ if done => self.panel = None,
                        Press::Open(kind) if std::mem::discriminant(kind) == std::mem::discriminant(&p.kind) => {
                            p.set_busy(r.data.busy, now);
                            p.sync_saved(kind, now);
                            if !r.data.busy {
                                p.submitted = false;
                            }
                        }
                        // Panel türü değişti (ör. kayıtlı ağ artık şifre istiyor).
                        Press::Open(kind) => {
                            let mut np = Panel::new(theme, r.data.key.clone(), kind.clone());
                            np.set_busy(r.data.busy, now);
                            self.panel = Some(np);
                        }
                        _ => self.panel = None,
                    }
                }
            }
        }
        if self.panel.is_none()
            && let Some(r) = self.rows.iter().find(|r| r.data.wants_password && matches!(r.data.press, Press::Open(PanelKind::Password { .. })))
            && let Press::Open(kind) = &r.data.press
        {
            let mut p = Panel::new(theme, r.data.key.clone(), kind.clone());
            if let Some(i) = &mut p.password {
                i.set_focused(self.focused, now);
            }
            self.panel = Some(p);
        }
    }

    fn open(&mut self, env: &mut Env, key: RowKey, kind: PanelKind) {
        if self.panel.as_ref().is_some_and(|p| p.key == key) {
            self.close(env.now);
            return;
        }
        if let Some(old) = self.panel.take() {
            clear_row_error(&self.rows, &old.key);
        }
        let mut p = Panel::new(env.theme, key.clone(), kind);
        if let Some(r) = self.rows.iter().find(|r| r.data.key == key) {
            p.set_busy(r.data.busy, env.now);
        }
        if p.has_input() {
            if let Some(i) = p.focused_input() {
                i.set_focused(true, env.now);
            }
            env.push(Action::Focus);
        }
        self.panel = Some(p);
    }

    fn close(&mut self, _now: Instant) {
        if let Some(p) = self.panel.take() {
            clear_row_error(&self.rows, &p.key);
        }
    }

    fn submit(&mut self) {
        let Some(p) = &mut self.panel else { return };
        // Parola alanından tek kopya (ara `String` yok); alan hemen temizlenir.
        let secret = |i: &mut TextInput| -> Option<Zeroizing<Vec<u8>>> {
            let v = i.with_secret(|s| (!s.is_empty()).then(|| Zeroizing::new(s.as_bytes().to_vec())));
            i.clear();
            v
        };
        match (&p.key, &p.kind) {
            (RowKey::Ap(ssid), PanelKind::Password { .. }) => {
                let pw = p.password.as_mut().and_then(secret);
                network::connect(ssid, pw);
            }
            (RowKey::Hidden, PanelKind::Hidden) => {
                let security = p.hidden_security();
                let pw = if security.needs_password() { p.password.as_mut().and_then(secret) } else { None };
                let ssid = p.ssid.as_ref().map(|i| i.text().to_owned()).unwrap_or_default();
                network::connect_hidden(&ssid, security, pw);
            }
            _ => return,
        }
        p.submitted = true;
    }

    fn panel_action(&mut self, env: &mut Env, a: PanelAction) {
        match a {
            PanelAction::Submit => self.submit(),
            PanelAction::Close => self.close(env.now),
            PanelAction::Disconnect(c) => {
                network::disconnect(&c);
                if let Some(p) = &mut self.panel {
                    p.submitted = true;
                }
            }
            PanelAction::Forget(c) => network::forget(&c),
            PanelAction::Autoconnect(c, on) => network::set_autoconnect(&c, on),
            PanelAction::Metered(c, m) => network::set_metered(&c, m),
        }
        // Gerçek duruma dön (anahtar/seçim; iyimser değil).
        self.data = ListContent::Hidden;
        self.update(env.theme, env.now);
    }

    fn row_press(&mut self, env: &mut Env, i: usize) {
        let d = self.rows[i].data.clone();
        match d.press {
            Press::None => {}
            Press::Connect => {
                if let RowKey::Ap(ssid) = &d.key {
                    network::connect(ssid, None);
                }
            }
            Press::Open(kind) => self.open(env, d.key, kind),
            Press::Pick => network::pick_wireguard(),
        }
        self.update(env.theme, env.now);
    }

    /// Panelin işaretçi olayı → istenen eylem.
    fn panel_pointer(&mut self, env: &mut Env, e: &PointerEvent) -> (bool, Option<PanelAction>) {
        let Some(p) = &mut self.panel else { return (false, None) };
        let now = env.now;
        let mut repaint = false;
        let press = matches!(e.kind, PointerKind::Press(MouseButton::Left));
        let shows_password = p.shows_password();
        for (f, input) in [(Field::Ssid, p.ssid.as_mut()), (Field::Password, if shows_password { p.password.as_mut() } else { None })] {
            let Some(input) = input else { continue };
            let r = input.pointer(env.text, env.theme, e, now);
            repaint |= r.repaint;
            if press && input.rect().contains(e.position) {
                p.focus = f;
                env.push(Action::Focus);
            }
        }
        if press {
            let f = p.focus;
            if let Some(i) = p.ssid.as_mut() {
                i.set_focused(f == Field::Ssid && self.focused, now);
            }
            if let Some(i) = p.password.as_mut() {
                i.set_focused(f == Field::Password && self.focused, now);
            }
        }
        if let Some(s) = &mut p.security {
            let r = s.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                repaint = true;
            }
        }
        if let Some(b) = &mut p.secondary {
            let r = b.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                return (true, Some(PanelAction::Close));
            }
        }
        if let Some(b) = &mut p.primary {
            let r = b.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                return match &p.kind {
                    PanelKind::Current { connection: Some(c) } => (true, Some(PanelAction::Disconnect(c.clone()))),
                    _ => (true, Some(PanelAction::Submit)),
                };
            }
        }
        let conn = match &p.kind {
            PanelKind::Saved { connection, .. } | PanelKind::Vpn { connection } => Some(connection.clone()),
            _ => None,
        };
        if let Some(c) = &mut p.confirm {
            let r = c.pointer(e, now);
            repaint |= r.repaint;
            if r.event == Some(ConfirmEvent::Confirmed)
                && let Some(conn) = &conn
            {
                return (true, Some(PanelAction::Forget(conn.clone())));
            }
        }
        if let Some(t) = &mut p.autoconnect {
            let r = t.pointer(e, now);
            repaint |= r.repaint;
            if let (Some(on), Some(conn)) = (r.event, &conn) {
                return (true, Some(PanelAction::Autoconnect(conn.clone(), on)));
            }
        }
        if let Some(m) = &mut p.metered {
            let r = m.pointer(e, now);
            repaint |= r.repaint;
            if let (Some(i), Some(conn)) = (r.event, &conn) {
                return (true, Some(PanelAction::Metered(conn.clone(), METERED[i.min(METERED.len() - 1)])));
            }
        }
        (repaint, None)
    }

    /// Satırın ve panelinin sağ kenar yerleşimi için: satır başı yüksekliği.
    fn head_height(env: &mut Env, r: &mut RowView, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let mut trailing = 0.0;
        if let Some(tag) = &mut r.tag {
            tag.set_max_width(Some(width * 0.3));
            trailing += tag.measure(env.text).width + t.space(3);
        }
        if r.lock.is_some() || r.data.busy {
            trailing += small_icon_side(t).max(t.space(5)) + t.space(3);
        }
        if let Some(s) = &r.switch {
            trailing += s.preferred_size(t).width + t.space(3);
        }
        let tw = (width - px * 2.0 - icon_side(t) - t.space(4) - trailing).max(t.space(20));
        r.title.set_max_width(Some(tw));
        r.subtitle.set_max_width(Some(tw));
        let mut text = r.title.measure(env.text).height;
        if !r.subtitle.text().is_empty() {
            text += t.space(1) + r.subtitle.measure(env.text).height;
        }
        row_height_for(t, text.max(icon_side(t)))
    }

    fn details_height(env: &mut Env, r: &mut RowView, width: f32) -> f32 {
        let t = env.theme;
        let mut h = 0.0;
        for (k, v) in &mut r.details {
            k.set_max_width(Some(width * 0.35));
            v.set_max_width(Some(width * 0.6));
            h += k.measure(env.text).height.max(v.measure(env.text).height) + t.space(1);
        }
        h
    }

    /// Satır ayrıntısı görünür mü (kablolu: her zaman; bağlı ağ: panel açıkken).
    fn shows_details(&self, i: usize) -> bool {
        let r = &self.rows[i];
        !r.details.is_empty() && (matches!(r.data.key, RowKey::Wired(_)) || self.panel.as_ref().is_some_and(|p| p.key == r.data.key))
    }

    fn text_x(&self, t: &Theme) -> f32 {
        let (px, _) = row_padding(t);
        self.rect.x + px + icon_side(t) + t.space(4)
    }
}

fn clear_row_error(rows: &[RowView], key: &RowKey) {
    if let Some(target) = rows.iter().find(|r| r.data.key == *key).and_then(|r| r.data.target.as_ref()) {
        network::clear_error(target);
    }
}

impl Block for NetListBlock {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.fiyu.as_ref().map(FiyuEmpty::spot)
    }

    fn visible(&self, _store: &Store) -> bool {
        content(self.kind, &network::snapshot()) != ListContent::Hidden
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            return f.height(env.text, t, width);
        }
        if !matches!(self.data, ListContent::Rows(_)) {
            self.empty.set_max_width(Some((width - px * 2.0).max(t.space(20))));
            return row_height_for(t, self.empty.measure(env.text).height);
        }
        if self.demo
            && self.panel.is_none()
            && let Some(r) = self.rows.iter().find(|r| matches!(r.data.press, Press::Open(_)))
            && let Press::Open(kind) = r.data.press.clone()
        {
            self.demo = false;
            self.panel = Some(Panel::new(t, r.data.key.clone(), kind));
        }
        let tx = px + icon_side(t) + t.space(4);
        let inner = (width - tx - px).max(t.space(20));
        let mut total = 0.0;
        for i in 0..self.rows.len() {
            let head = Self::head_height(env, &mut self.rows[i], width);
            let mut h = head;
            if self.shows_details(i) {
                h += Self::details_height(env, &mut self.rows[i], inner);
            }
            let key = self.rows[i].data.key.clone();
            if let Some(p) = self.panel.as_mut().filter(|p| p.key == key) {
                p.height = p.layout(env, 0.0, 0.0, inner);
                if p.height > 0.0 {
                    h += p.height + py;
                }
            }
            let r = &mut self.rows[i];
            r.head_height = head;
            r.rect.height = h;
            total += h;
        }
        total
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            f.set_rect(rect);
        }
        let tx = self.text_x(t);
        let inner = (rect.width - (tx - rect.x) - px).max(t.space(20));
        let mut y = rect.y;
        for i in 0..self.rows.len() {
            let show = self.shows_details(i);
            let r = &mut self.rows[i];
            r.rect = Rect::new(rect.x, y, rect.width, r.rect.height);
            r.head = Rect::new(rect.x, y, rect.width, r.head_height);
            if let Some(s) = &mut r.switch {
                let ts = s.preferred_size(t);
                s.set_rect(Rect::new(rect.right() - px - ts.width, y + (r.head_height - ts.height) * 0.5, ts.width, ts.height));
            }
            let mut py0 = y + r.head_height;
            if show {
                py0 += Self::details_height(env, r, inner);
            }
            let key = r.data.key.clone();
            if let Some(p) = self.panel.as_mut().filter(|p| p.key == key) {
                p.rect = Rect::new(tx, py0, inner, p.height);
                p.layout(env, tx, py0, inner);
            }
            y += self.rows[i].rect.height;
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        // Önce panel (satır başının altında; kendi bileşenleri).
        let (r, action) = self.panel_pointer(env, e);
        repaint |= r;
        if let Some(a) = action {
            self.panel_action(env, a);
            return true;
        }
        let mut pressed = None;
        for i in 0..self.rows.len() {
            let row = &mut self.rows[i];
            if let Some(s) = &mut row.switch {
                let r = s.pointer(e, now);
                repaint |= r.repaint;
                if let (Some(on), Some(Target::Conn(c))) = (r.event, row.data.target.clone()) {
                    network::set_vpn(&c, on);
                    self.data = ListContent::Hidden;
                    self.update(env.theme, now);
                    return true;
                }
                if s.rect().contains(e.position) {
                    continue;
                }
            }
            if !row.pressable() {
                continue;
            }
            let tap = row.hover.handle(row.head, e, now);
            repaint |= tap.repaint;
            if tap.clicked {
                pressed = Some(i);
            }
        }
        if let Some(i) = pressed {
            self.row_press(env, i);
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let Some(p) = &mut self.panel else { return false };
        if e.state != KeyState::Pressed {
            return p.focused_input().is_some_and(|i| i.key(e, env.now).repaint);
        }
        if e.key == Key::Escape {
            self.close(env.now);
            return true;
        }
        let Some(input) = p.focused_input() else { return false };
        let r = input.key(e, env.now);
        match r.event {
            Some(TextInputEvent::Submit) => {
                self.submit();
                true
            }
            Some(TextInputEvent::Cancel) => {
                self.close(env.now);
                true
            }
            Some(TextInputEvent::Copy(_) | TextInputEvent::Cut(_)) => true,
            Some(_) => true,
            None => r.repaint,
        }
    }

    fn focusable(&self) -> bool {
        self.panel.as_ref().is_some_and(Panel::has_input)
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        self.focused = focused;
        if let Some(p) = &mut self.panel {
            let f = p.focus;
            if let Some(i) = p.ssid.as_mut() {
                i.set_focused(focused && f == Field::Ssid, now);
            }
            if let Some(i) = p.password.as_mut() {
                i.set_focused(focused && f == Field::Password, now);
            }
        }
    }

    fn focus_step(&mut self, forward: bool, now: Instant) -> bool {
        let Some(p) = &mut self.panel else { return false };
        if p.ssid.is_none() || !p.shows_password() {
            return false;
        }
        let next = match (p.focus, forward) {
            (Field::Ssid, true) => Field::Password,
            (Field::Password, false) => Field::Ssid,
            _ => return false,
        };
        p.focus = next;
        if let Some(i) = p.ssid.as_mut() {
            i.set_focused(next == Field::Ssid, now);
        }
        if let Some(i) = p.password.as_mut() {
            i.set_focused(next == Field::Password, now);
        }
        true
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        self.panel.as_mut().and_then(Panel::focused_input)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        let rows = self.rows.iter().map(|r| match &r.switch {
            Some(s) if s.rect().contains(p) => s.cursor(p),
            _ if r.pressable() && r.head.contains(p) => Some(CursorShape::Pointer),
            _ => None,
        });
        rows.chain([self.panel.as_ref().and_then(|x| x.cursor(p))]).flatten().next()
    }

    fn demo_expand(&mut self, _now: Instant) {
        // Canlı test: ilk açılabilir satırın paneli bir sonraki yerleşimde açılır (tema orada).
        self.demo = true;
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme, env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_network(event) {
            if matches!(event, crate::services::Event::LocaleChanged) {
                // Metinler yeni dilde: satırlar ve panel yeniden kurulur.
                self.data = ListContent::Hidden;
                self.panel = None;
            }
            self.update(env.theme, env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.empty.set_style(text_style(theme, TextRole::Body));
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
        for r in &mut self.rows {
            r.set_theme(theme, now);
        }
        if let Some(p) = &mut self.panel {
            p.set_theme(theme, now);
        }
        self.spinner.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            f.draw(ui);
            return;
        }
        if !matches!(self.data, ListContent::Rows(_)) {
            let h = self.empty.measure_in(ui.painter).height;
            self.empty.draw(ui.painter, Point::new(self.rect.x + px, self.rect.center().y - h * 0.5), ui.fade(c.on_surface_variant));
            return;
        }
        let side = icon_side(t);
        let tx = self.text_x(t);
        let now = ui.now;
        for i in 0..self.rows.len() {
            let show_details = self.shows_details(i);
            let r = &mut self.rows[i];
            if r.pressable() {
                r.hover.draw_layer(ui.painter, r.head, t.shape.medium(), t, now);
                if r.hover.is_animating(now) {
                    ui.request_frame();
                }
            }
            let mid = r.head.center().y;
            let icon_color = if r.active() { c.primary } else { c.on_surface_variant };
            r.icon.draw_centered(ui.painter, Point::new(r.rect.x + px + side * 0.5, mid), ui.fade(icon_color));
            let th = r.title.measure_in(ui.painter).height;
            let sh = if r.subtitle.text().is_empty() { 0.0 } else { t.space(1) + r.subtitle.measure_in(ui.painter).height };
            let top = mid - (th + sh) * 0.5;
            r.title.draw(ui.painter, Point::new(tx, top), ui.fade(c.on_surface));
            if sh > 0.0 {
                let color = if r.data.error.is_some() { c.error } else { c.on_surface_variant };
                r.subtitle.draw(ui.painter, Point::new(tx, top + th + t.space(1)), ui.fade(color));
            }
            let mut right = r.rect.right() - px;
            if let Some(s) = &mut r.switch {
                s.draw(ui);
                right = s.rect().x - t.space(3);
            }
            if r.data.busy {
                let ss = self.spinner.preferred_size(t);
                self.spinner.set_rect(Rect::new(right - ss.width, mid - ss.height * 0.5, ss.width, ss.height));
                self.spinner.draw(ui);
                right -= ss.width + t.space(3);
            } else if let Some(lock) = &mut r.lock {
                let s = small_icon_side(t);
                lock.draw_centered(ui.painter, Point::new(right - s * 0.5, mid), ui.fade(c.on_surface_variant));
                right -= s + t.space(3);
            }
            if let Some(tag) = &mut r.tag {
                let ts = tag.measure_in(ui.painter);
                let color = if r.data.active { c.primary } else { c.on_surface_variant };
                tag.draw(ui.painter, Point::new(right - ts.width, mid - ts.height * 0.5), ui.fade(color));
            }
            if show_details {
                let mut y = r.head.bottom();
                let col = r.details.iter_mut().map(|(k, _)| k.measure_in(ui.painter).width).fold(0.0, f32::max) + t.space(4);
                for (k, v) in &mut r.details {
                    let kh = k.measure_in(ui.painter).height;
                    let vh = v.measure_in(ui.painter).height;
                    k.draw(ui.painter, Point::new(tx, y), ui.fade(c.on_surface_variant));
                    v.draw(ui.painter, Point::new(tx + col, y), ui.fade(c.on_surface));
                    y += kh.max(vh) + t.space(1);
                }
            }
        }
        if let Some(p) = &mut self.panel {
            p.draw(ui, tx);
        }
    }
}

impl RowView {
    fn active(&self) -> bool {
        self.data.active
    }
}

#[cfg(test)]
mod tests {
    use nuxut_nm::{
        AccessPoint, ActiveConnection, ActiveState, Connectivity, DeviceKind, IpNet, NetDevice, NetworkState, NmState, SavedConnection,
        WifiDevice, WireGuardError,
    };

    use super::*;
    use crate::services::network::OpKind;

    fn ap(ssid: &str, strength: u8, security: Security, active: bool, saved: bool) -> AccessPoint {
        AccessPoint {
            path: format!("/ap/{ssid}"),
            ssid: ssid.into(),
            ssid_bytes: ssid.as_bytes().to_vec(),
            strength,
            security,
            frequency: 2437,
            active,
            saved,
        }
    }

    fn conn(id: &str, kind: ConnectionKind, ssid: Option<&str>, active: Option<&str>) -> SavedConnection {
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

    fn ready(state: NetworkState) -> Snapshot {
        Snapshot { state: NetState::Ready(state), ..Default::default() }
    }

    fn home() -> NetworkState {
        NetworkState {
            available: true,
            state: NmState::ConnectedGlobal,
            connectivity: Connectivity::Full,
            networking_enabled: true,
            wifi_enabled: true,
            wifi_hardware_enabled: true,
            wifi_devices: vec![WifiDevice { path: "/d/1".into(), interface: "wlan0".into(), active_ap: Some("/ap/Ev".into()) }],
            access_points: vec![
                ap("Ev", 80, Security::WpaPsk, true, true),
                ap("Misafir", 30, Security::Open, false, false),
                ap("Kafe", 55, Security::Sae, false, true),
                ap("Şirket", 20, Security::Enterprise, false, false),
                ap("Komşu", 10, Security::WpaPsk, false, false),
            ],
            saved: vec![
                conn("Ev", ConnectionKind::Wifi, Some("Ev"), Some("/a/1")),
                conn("Kafe", ConnectionKind::Wifi, Some("Kafe"), None),
                conn("Eski", ConnectionKind::Wifi, Some("Eski"), None),
                conn("İş VPN", ConnectionKind::WireGuard, None, Some("/a/2")),
                conn("Ofis", ConnectionKind::Vpn, None, None),
            ],
            active: vec![
                ActiveConnection { path: "/a/1".into(), id: "Ev".into(), kind: ConnectionKind::Wifi, state: ActiveState::Activated, connection: "/s/Ev".into() },
                ActiveConnection { path: "/a/2".into(), id: "İş VPN".into(), kind: ConnectionKind::WireGuard, state: ActiveState::Activated, connection: "/s/İş VPN".into() },
                ActiveConnection { path: "/a/w".into(), id: "Kablolu".into(), kind: ConnectionKind::Ethernet, state: ActiveState::Activated, connection: "/s/Kablolu".into() },
            ],
            devices: vec![
                NetDevice { path: "/d/1".into(), interface: "wlan0".into(), kind: DeviceKind::Wifi, managed: true, connected: true, active_connection: Some("/a/1".into()) },
                NetDevice { path: "/d/e".into(), interface: "enp1s0".into(), kind: DeviceKind::Ethernet, managed: true, connected: true, active_connection: Some("/a/w".into()) },
            ],
            ..Default::default()
        }
    }

    fn rows(c: ListContent) -> Vec<RowData> {
        match c {
            ListContent::Rows(r) => r,
            other => panic!("satır yok: {other:?}"),
        }
    }

    fn row(c: ListContent, title: &str) -> RowData {
        rows(c).into_iter().find(|r| r.title == title).unwrap_or_else(|| panic!("{title} yok"))
    }

    #[test]
    fn signal_icons_follow_strength() {
        let r = rows(content(NetList::Visible, &ready(home())));
        assert_eq!(r[0].title, "Kafe");
        assert_eq!((r[0].icon, r[1].icon), (IconName::Wifi3, IconName::Wifi2));
        let cur = rows(content(NetList::Current, &ready(home())));
        assert_eq!((cur[0].icon, cur[0].active, cur[0].lock), (IconName::Wifi4, true, true));
    }

    #[test]
    fn visible_marks_saved_and_secured() {
        let kafe = row(content(NetList::Visible, &ready(home())), "Kafe");
        assert!(kafe.lock && kafe.tag.as_deref() == Some(t!("settings-wifi-tag-saved").as_str()));
        assert_eq!(kafe.subtitle, format!("{}{SEP}{}", t!("settings-network-security-sae"), t!("settings-network-band-2g")));
        let open = row(content(NetList::Visible, &ready(home())), "Misafir");
        assert!(!open.lock && open.tag.is_none(), "açık ağ kilitsiz, kayıtsız");
    }

    /// Basış eşlemesi: açık → doğrudan bağlan; kayıtlı şifreli → doğrudan (kayıtlı şifre);
    /// kayıtsız şifreli → şifre paneli; kurumsal → not (bağlan yok).
    #[test]
    fn press_by_security() {
        let s = ready(home());
        assert_eq!(row(content(NetList::Visible, &s), "Misafir").press, Press::Connect);
        assert_eq!(row(content(NetList::Visible, &s), "Kafe").press, Press::Connect);
        assert_eq!(row(content(NetList::Visible, &s), "Komşu").press, Press::Open(PanelKind::Password { security: Security::WpaPsk }));
        assert_eq!(row(content(NetList::Visible, &s), "Şirket").press, Press::Open(PanelKind::Enterprise));
    }

    /// Bağlan → bekler (ilerleme) → şifre yok hatası → kayıtlı ağda bile şifre alanı açılır.
    #[test]
    fn connect_error_reopens_password() {
        let mut s = ready(home());
        let t = Target::Ap(b"Kafe".to_vec());
        s.begin(t.clone(), OpKind::Connect).unwrap();
        let busy = row(content(NetList::Visible, &s), "Kafe");
        assert!(busy.busy && busy.subtitle.starts_with(&t!("settings-network-connecting")));
        s.ops.clear();
        s.reject(t, RowError::NoSecrets);
        let r = row(content(NetList::Visible, &s), "Kafe");
        assert_eq!(r.error.as_deref(), Some(t!("settings-wifi-error-no-secrets").as_str()));
        assert!(r.wants_password);
        assert_eq!(r.press, Press::Open(PanelKind::Password { security: Security::Sae }), "kayıtlı ağ şifre ister");
    }

    #[test]
    fn panel_opens_on_password_error_and_closes_on_success() {
        let theme = Theme::default();
        let now = Instant::now();
        let desc = crate::registry::by_id("wifi.networks").unwrap();
        let mut b = NetListBlock {
            desc,
            kind: NetList::Visible,
            data: ListContent::Hidden,
            rows: Vec::new(),
            empty: Label::new("", text_style(&theme, TextRole::Body)),
            fiyu: None,
            panel: None,
            spinner: Spinner::new(&theme),
            focused: false,
            demo: false,
            rect: Rect::ZERO,
        };
        let mut s = ready(home());
        s.reject(Target::Ap("Komşu".as_bytes().to_vec()), RowError::InvalidPassword);
        b.apply(&theme, content(NetList::Visible, &s), now);
        assert_eq!(b.panel.as_ref().map(|p| p.key.clone()), Some(RowKey::Ap("Komşu".as_bytes().to_vec())), "hata → alan açılır");
        assert!(b.focusable(), "şifre alanı odak alır");
        b.panel.as_mut().unwrap().submitted = true;
        s.errors.clear();
        s.begin(Target::Ap("Komşu".as_bytes().to_vec()), OpKind::Connect).unwrap();
        b.apply(&theme, content(NetList::Visible, &s), now);
        assert!(b.panel.is_some(), "beklerken açık");
        s.ops.clear();
        let mut st = home();
        st.access_points.retain(|a| a.ssid != "Komşu");
        b.apply(&theme, content(NetList::Visible, &ready(st)), now);
        assert!(b.panel.is_none(), "bağlandı (satır gitti) → kapanır");
    }

    #[test]
    fn current_reports_reach_and_details() {
        let mut st = home();
        st.connectivity = Connectivity::Portal;
        let mut s = ready(st);
        let cur = rows(content(NetList::Current, &s));
        assert!(cur[0].subtitle.starts_with(&t!("settings-wifi-connected-portal")));
        assert_eq!(cur[0].press, Press::Open(PanelKind::Current { connection: Some("/s/Ev".into()) }));
        assert!(cur[0].details.is_empty(), "ayrıntı henüz gelmedi");
        let d = DeviceDetails {
            path: "/d/1".into(),
            ipv4: vec![IpNet { addr: "192.168.1.5".parse().unwrap(), prefix: 24 }],
            dns: vec!["1.1.1.1".parse().unwrap()],
            speed_mbps: Some(300),
            ..Default::default()
        };
        s.details.insert("/d/1".into(), d);
        let cur = rows(content(NetList::Current, &s));
        assert_eq!(cur[0].details, [(t!("settings-network-detail-ipv4"), "192.168.1.5/24".into()), (t!("settings-network-detail-dns"), "1.1.1.1".into())], "Wi-Fi'de hız yok");
    }

    #[test]
    fn wired_card_read_only() {
        let mut s = ready(home());
        s.details.insert("/d/e".into(), DeviceDetails { path: "/d/e".into(), speed_mbps: Some(1000), ..Default::default() });
        let r = rows(content(NetList::Wired, &s));
        assert_eq!((r[0].title.as_str(), r[0].icon, r[0].press.clone()), ("Kablolu", IconName::Ethernet, Press::None));
        assert_eq!(r[0].details, [(t!("settings-network-detail-speed"), t!("settings-network-speed-mbps", mbps = 1000))]);
        let mut st = home();
        st.devices.retain(|d| d.kind != DeviceKind::Ethernet);
        assert_eq!(content(NetList::Wired, &ready(st)), ListContent::Hidden, "kablolu aygıt yoksa kart yok");
    }

    #[test]
    fn saved_rows_and_neutral_icon() {
        let r = rows(content(NetList::Saved, &ready(home())));
        assert_eq!(r.iter().map(|r| (r.title.as_str(), r.icon, r.active)).collect::<Vec<_>>(), [("Ev", IconName::Wifi4, true), ("Kafe", IconName::Wifi3, false), ("Eski", IconName::Bookmark, false)]);
        assert_eq!(r[0].subtitle, t!("settings-wifi-connected"));
        assert_eq!(r[2].press, Press::Open(PanelKind::Saved { connection: "/s/Eski".into(), autoconnect: true, metered: Metered::Unknown }));
    }

    #[test]
    fn metered_mapping() {
        assert_eq!(METERED.map(metered_index), [0, 1, 2]);
        assert_eq!(metered_index(Metered::GuessYes), 0, "NM tahmini Otomatik sayılır");
        assert_eq!(metered_index(Metered::GuessNo), 0);
    }

    #[test]
    fn vpn_rows_switch_and_errors() {
        let mut s = ready(home());
        let r = rows(content(NetList::Vpn, &s));
        assert_eq!(r.iter().map(|r| (r.title.as_str(), r.switch)).collect::<Vec<_>>(), [("İş VPN", Some(true)), ("Ofis", Some(false))]);
        assert_eq!(r[0].subtitle, t!("settings-vpn-kind-wireguard"));
        s.reject(Target::Conn("/s/Ofis".into()), RowError::AuthFailed);
        assert_eq!(row(content(NetList::Vpn, &s), "Ofis").error.as_deref(), Some(t!("settings-vpn-error-auth-failed").as_str()), "VPN metni");
    }

    #[test]
    fn hidden_and_import_rows() {
        let mut s = ready(home());
        assert_eq!(rows(content(NetList::Hidden, &s))[0].press, Press::Open(PanelKind::Hidden));
        s.reject(Target::Hidden, RowError::InvalidSsid);
        assert_eq!(rows(content(NetList::Hidden, &s))[0].error.as_deref(), Some(t!("settings-network-error-invalid-ssid").as_str()));
        let mut off = home();
        off.wifi_enabled = false;
        assert_eq!(content(NetList::Hidden, &ready(off)), ListContent::Hidden, "Wi-Fi kapalı: gizli ağ yok");
        assert_eq!(rows(content(NetList::Import, &s))[0].press, Press::Pick);
        assert_eq!(content(NetList::Import, &Snapshot::default()), ListContent::Hidden);
    }

    #[test]
    fn empty_and_hidden_states() {
        let reading = ListContent::Empty(t!("settings-network-reading"));
        assert_eq!(content(NetList::Visible, &Snapshot::default()), reading, "okunurken ana kart yerinde");
        assert_eq!(content(NetList::Vpn, &Snapshot::default()), reading);
        assert_eq!(content(NetList::Saved, &Snapshot::default()), ListContent::Hidden);
        let missing = Snapshot { state: NetState::Unavailable(Unavailable::ServiceNotRunning), ..Default::default() };
        for k in [NetList::Current, NetList::Visible, NetList::Saved, NetList::Vpn, NetList::Wired, NetList::Hidden, NetList::Import] {
            assert_eq!(content(k, &missing), ListContent::Hidden, "NM yokken yalnız not");
        }
        let mut s = home();
        s.wifi_enabled = false;
        assert_eq!(content(NetList::Visible, &ready(s.clone())), ListContent::Empty(t!("settings-wifi-visible-off")));
        assert_eq!(content(NetList::Current, &ready(s.clone())), ListContent::Hidden, "kapalıyken bağlı ağ kartı yok");
        s.wifi_devices.clear();
        assert_eq!(content(NetList::Visible, &ready(s.clone())), ListContent::Hidden, "aygıt yok: durum satırı söyler");
        let mut s = home();
        s.access_points.retain(|a| a.active);
        assert_eq!(content(NetList::Visible, &ready(s)), ListContent::Empty(t!("settings-wifi-visible-empty")));
        let empty = ready(NetworkState { available: true, ..Default::default() });
        assert_eq!(content(NetList::Saved, &empty), ListContent::Empty(t!("settings-wifi-saved-empty")));
        assert_eq!(content(NetList::Vpn, &empty), ListContent::Empty(t!("settings-vpn-empty")));
    }

    #[test]
    fn notes_and_errors() {
        assert_eq!(note_text(&Snapshot::default()), None);
        let un = |u| Snapshot { state: NetState::Unavailable(u), ..Default::default() };
        assert_eq!(note_text(&un(Unavailable::ServiceNotRunning)), Some(t!("settings-network-missing")));
        assert_eq!(note_text(&un(Unavailable::BusUnreachable)), Some(t!("settings-network-no-bus")), "bus yok ayrı metin");
        assert_ne!(t!("settings-network-no-bus"), t!("settings-network-missing"));
        assert_eq!(note_text(&Snapshot { state: NetState::Failed, ..Default::default() }), Some(t!("settings-network-failed")));
        let mut s = ready(home());
        assert_eq!(note_text(&s), None);
        s.failure = Some(NmError::Enterprise);
        assert_eq!(note_text(&s), Some(t!("settings-network-error-enterprise")));
        let all = [
            NmError::CallFailed("x".into()),
            NmError::NmUnavailable,
            NmError::ConnectionUnknown,
            NmError::ConnectionInactive,
            NmError::ApUnknown,
            NmError::Enterprise,
            NmError::InvalidPassword,
            NmError::NoWifiDevice,
            NmError::InvalidSsid,
            NmError::DeviceUnknown,
            NmError::Denied,
            NmError::Unsupported,
            NmError::SecretNotSaved,
        ];
        let texts: Vec<String> = all.iter().map(error_text).collect();
        assert!(texts.iter().all(|t| !t.is_empty() && !t.starts_with("settings-")), "{texts:?}");
        let unique: std::collections::HashSet<_> = texts.iter().collect();
        assert_eq!(unique.len(), all.len(), "her hata ayrı metin");
        assert!(texts[0].contains('x'), "ayrıntı metne girer");
    }

    #[test]
    fn import_errors_map_to_text() {
        use WireGuardErrorKind as K;
        let kinds = [
            K::TooLarge, K::Syntax, K::UnknownSection, K::KeyOutsideSection, K::UnknownKey, K::DuplicateKey, K::MissingInterface,
            K::DuplicateInterface, K::MissingPrivateKey, K::MissingPublicKey, K::InvalidKey, K::InvalidAddress, K::InvalidNumber,
            K::InvalidEndpoint, K::TooMany, K::InvalidName,
        ];
        for k in kinds {
            let txt = import_error_text(ImportError::Parse(WireGuardError { line: 7, kind: k }));
            assert!(!txt.is_empty() && !txt.starts_with("settings-"), "{k:?}");
        }
        assert!(import_error_text(ImportError::Parse(WireGuardError { line: 7, kind: K::InvalidKey })).contains('7'), "satır numarası");
        for e in [ImportError::Portal, ImportError::Read, ImportError::TooLarge, ImportError::NotUtf8] {
            assert!(!import_error_text(e).starts_with("settings-"));
        }
    }

    #[test]
    fn status_texts_are_distinct() {
        let all = [WifiStatus::NoDevice, WifiStatus::HardwareOff, WifiStatus::Off, WifiStatus::On].map(status_text);
        let unique: std::collections::HashSet<_> = all.iter().collect();
        assert_eq!(unique.len(), 4);
    }

    /// Maskot yerleşimi: yalnız kayıtlı ağ ve VPN listesi boşken (boş maskotu); görünen ağlar
    /// ve bağlı ağ maskotsuz; satır gelince maskot kalkar.
    #[test]
    fn empty_saved_and_vpn_lists_show_empty_mascot() {
        let theme = Theme::default();
        let now = Instant::now();
        for (kind, want) in [(NetList::Saved, Some(Spot::WifiSavedEmpty)), (NetList::Vpn, Some(Spot::VpnEmpty)), (NetList::Visible, None), (NetList::Current, None)] {
            assert_eq!(list_spot(kind), want);
            let mut b = NetListBlock {
                desc: crate::registry::by_id("wifi.saved").unwrap(),
                kind,
                data: ListContent::Hidden,
                rows: Vec::new(),
                empty: Label::new("", text_style(&theme, TextRole::Body)),
                fiyu: None,
                panel: None,
                spinner: Spinner::new(&theme),
                focused: false,
                demo: false,
                rect: Rect::ZERO,
            };
            b.apply(&theme, ListContent::Empty("Boş".into()), now);
            assert_eq!(b.mascot(), want, "{kind:?}");
            assert_eq!(b.mascot().map(Spot::mood), want.map(|_| nuxut::ui::Mood::Empty));
            let row = RowData::new(RowKey::Info, IconName::Wifi, "Ev".into());
            b.apply(&theme, ListContent::Rows(vec![row]), now);
            assert_eq!(b.mascot(), None, "{kind:?}: satır var");
        }
    }
}
