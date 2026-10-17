//! Bağlantılar › Bluetooth sayfası (t115; `docs/design/bluetooth-page.md`). Değerler BlueZ'de
//! ([`crate::services::bluetooth`]); bloklar servis anlık görüntüsünden çizer, eylemi servise
//! verir. Durum ancak BlueZ bildirince değişir (iyimser değil): hata sonrası anahtar gerçek
//! duruma döner, not nedeni söyler.
//!
//! - [`BtStatus`]: kartsız üst blok — BlueZ/adaptör yoksa maskotlu hata (sayfanın tek içeriği),
//!   değilse son eylem hatasının notu.
//! - [`BtPowerRow`]: Bluetooth aç/kapa; rfkill yumuşak ("Uçak kipi") / sert ("donanım anahtarı").
//! - [`BtNameRow`]: bu bilgisayarın adı (adaptör `Alias`; Enter ya da odak kaybında yazılır).
//! - [`BtVisibleRow`]: görünürlük (süreli; kalan süre alt metinde, saniyede bir).
//! - [`BtDeviceList`]: eşleşmiş cihazlar (satır içi ayrıntı: ad, güvenilir, engelle, bağlan,
//!   unut) ve yakındaki cihazlar (eşleştir/iptal; tarama durumu; adsızları göster).

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{FOCUS_RING, focus_ring};
use nuxut::ui::{
    Button, ButtonVariant, ConfirmButton, ConfirmEvent, Icon, IconName, IconView, Interaction, Label, Notice, NoticeKind,
    Spinner, TextInput, TextInputEvent, TextRole, Toggle, Ui, text_style,
};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, Theme};
use nuxut_bluez::{Adapter, BluezError, Device};

use super::datetime::RowText;
use super::fiyu::{FiyuEmpty, Spot};
use super::notice::{notice_height, place_notice};
use crate::block::{Action, Block, Env};
use crate::registry::{BtList, SettingDesc};
use crate::rows::{row_height_for, row_padding};
use crate::services::bluetooth::{
    self, PageStatus, Power, Snapshot, device_icon, discoverable_remaining, mmss, nearby_devices, page_status, paired_devices,
    power,
};
use crate::store::Store;

/// Alt metin parçalarının ayırıcısı.
const SEP: &str = " · ";

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// Sayfanın maskotlu hata durumu: (başlık, gövde). Hazırken ve okunurken `None`.
pub fn unavailable_text(status: PageStatus) -> Option<(String, String)> {
    Some(match status {
        PageStatus::Failed => (t!("settings-bluetooth-failed"), t!("settings-bluetooth-failed-body")),
        PageStatus::Unreachable => (t!("settings-bluetooth-unreachable"), t!("settings-bluetooth-unreachable-body")),
        PageStatus::NoBluez => (t!("settings-bluetooth-missing"), t!("settings-bluetooth-missing-body")),
        PageStatus::NoAdapter => (t!("settings-bluetooth-no-adapter"), t!("settings-bluetooth-no-adapter-body")),
        PageStatus::Reading | PageStatus::Ready => return None,
    })
}

/// `BluezError` → kullanıcı metni (üst not). `CallFailed` ayrıntısı BlueZ'in temizlenmiş
/// metnidir: gösterilir, loglanmaz.
pub fn error_text(e: &BluezError) -> String {
    match e {
        BluezError::CallFailed(detail) => t!("settings-bluetooth-error-call-failed", detail = detail.as_str()),
        BluezError::Timeout => t!("settings-bluetooth-error-timeout"),
        BluezError::NoAdapter => t!("settings-bluetooth-error-no-adapter"),
        BluezError::BluezUnavailable => t!("settings-bluetooth-error-unavailable"),
        BluezError::UnknownDevice => t!("settings-bluetooth-error-unknown-device"),
        BluezError::AuthenticationRejected => t!("settings-bluetooth-error-auth-rejected"),
        BluezError::AuthenticationFailed => t!("settings-bluetooth-error-auth-failed"),
        BluezError::AuthenticationCanceled => t!("settings-bluetooth-error-auth-canceled"),
        BluezError::AuthenticationTimeout => t!("settings-bluetooth-error-auth-timeout"),
        BluezError::ConnectionAttemptFailed => t!("settings-bluetooth-error-connection-failed"),
        BluezError::AlreadyExists => t!("settings-bluetooth-error-already-exists"),
        BluezError::InProgress => t!("settings-bluetooth-error-in-progress"),
        BluezError::NotReady => t!("settings-bluetooth-error-not-ready"),
        BluezError::InvalidValue => t!("settings-bluetooth-error-invalid-value"),
        BluezError::RfkillBlocked { hard: Some(true) } => t!("settings-bluetooth-error-rfkill-hard"),
        BluezError::RfkillBlocked { .. } => t!("settings-bluetooth-error-rfkill"),
        // Crate'in ileride ekleyeceği hatalar (`#[non_exhaustive]`): genel metin + `Display`.
        e => t!("settings-bluetooth-error-call-failed", detail = e.to_string()),
    }
}

/// Güç satırının alt metni.
pub fn power_text(p: Power) -> String {
    match p {
        Power::On => t!("settings-bluetooth-power-on"),
        Power::Off => t!("settings-bluetooth-power-off"),
        Power::Turning => t!("settings-bluetooth-power-turning"),
        Power::SoftBlocked => t!("settings-bluetooth-power-soft-blocked"),
        Power::HardBlocked => t!("settings-bluetooth-power-hard-blocked"),
    }
}

/// Görünürlük satırının alt metni.
pub fn discoverable_text(a: &Adapter, since: Option<Instant>, now: Instant) -> String {
    if !a.discoverable {
        return t!("settings-bluetooth-discoverable-off");
    }
    match discoverable_remaining(a, since, now) {
        Some(secs) => {
            let time = mmss(secs);
            t!("settings-bluetooth-discoverable-remaining", time = time.as_str())
        }
        None => t!("settings-bluetooth-discoverable-on"),
    }
}

/// Sinyal gücü metni (yakındaki cihazın alt metni).
pub fn signal_text(rssi: Option<i16>) -> String {
    match rssi {
        Some(r) if r >= -60 => t!("settings-bluetooth-signal-strong"),
        Some(r) if r >= -80 => t!("settings-bluetooth-signal-medium"),
        Some(_) => t!("settings-bluetooth-signal-weak"),
        None => t!("settings-bluetooth-signal-none"),
    }
}

/// Bir cihaz satırının verisi (görünümden bağımsız; testli).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowData {
    /// Cihaz yolu (kimlik).
    pub path: String,
    pub icon: IconName,
    /// Bağlı (ikon dolu ve vurgu renginde).
    pub active: bool,
    pub title: String,
    /// Ham takma ad (yeniden adlandır alanının başlangıcı).
    pub alias: String,
    pub subtitle: String,
    /// Alt metin hata renginde (eşleştirme başarısız).
    pub error: bool,
    /// Pil yüzdesi (`Battery1` varsa).
    pub battery: Option<u8>,
    pub kind: RowKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    Paired { connected: bool, trusted: bool, blocked: bool, powered: bool },
    /// `pairing`: bu cihaz eşleştiriliyor (düğme "İptal"); `can_pair`: başka eşleştirme yok ve
    /// adaptör açık.
    Nearby { pairing: bool, can_pair: bool },
}

/// Yakındaki cihazlar kartının ek bilgisi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NearbyInfo {
    /// Bizim taramamız sürüyor.
    pub scanning: bool,
    /// Gizlenen adsız cihaz sayısı.
    pub hidden: usize,
    /// Görünen adsız cihaz sayısı ("Gizle" düğmesi için).
    pub unnamed_shown: usize,
}

/// Kartın içeriği.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// Kart hiç çizilmez.
    Hidden,
    /// Kart yalnız bir açıklama satırı taşır.
    Empty(String),
    /// Eşleşmiş cihaz yok: maskotlu boş durum ([`list_spot`]).
    NoPaired,
    /// Satırlar (yakındakilerde boş olabilir: "bulunamadı" metni + tarama satırı).
    Rows(Vec<RowData>, Option<NearbyInfo>),
}

/// Pil yüzdesi metni ("%80"; etkin biçimin yüzde yazımı).
pub fn battery_text(percent: u8) -> String {
    crate::numfmt::percent(f64::from(percent) / 100.0, 0)
}

fn paired_row(d: &Device, powered: bool) -> RowData {
    let mut parts = vec![if d.connected { t!("settings-bluetooth-connected") } else { t!("settings-bluetooth-not-connected") }];
    if d.blocked {
        parts.push(t!("settings-bluetooth-blocked"));
    }
    RowData {
        path: d.path.clone(),
        icon: device_icon(d),
        active: d.connected,
        title: d.name.clone(),
        alias: d.alias.clone(),
        subtitle: parts.join(SEP),
        error: false,
        battery: d.battery,
        kind: RowKind::Paired { connected: d.connected, trusted: d.trusted, blocked: d.blocked, powered },
    }
}

fn nearby_row(d: &Device, s: &Snapshot) -> RowData {
    let pairing = s.pairing.as_ref().is_some_and(|p| p.device == d.path);
    let failed = s.pair_failed.as_ref().filter(|(p, _)| *p == d.path).map(|(_, e)| e);
    let (subtitle, error) = match (pairing, failed) {
        // Kabuğun ajanı varsayılan değil: onay penceresi açılamaz (Pair yine denenir).
        (true, _) if s.pairing.as_ref().is_some_and(|p| p.no_agent) => (t!("settings-bluetooth-pairing-no-agent"), false),
        (true, _) => (t!("settings-bluetooth-pairing-wait"), false),
        (false, Some(e)) => (error_text(e), true),
        (false, None) => (signal_text(d.rssi), false),
    };
    RowData {
        path: d.path.clone(),
        icon: device_icon(d),
        active: false,
        title: d.name.clone(),
        alias: d.alias.clone(),
        subtitle,
        error,
        battery: None,
        kind: RowKind::Nearby { pairing, can_pair: s.pairing.is_none() },
    }
}

/// Kartın içeriği, anlık görüntüden.
pub fn content(kind: BtList, s: &Snapshot, show_unnamed: bool) -> Content {
    let state = match (page_status(s), s.state.ready()) {
        (PageStatus::Reading, _) => return Content::Empty(t!("settings-bluetooth-reading")),
        (PageStatus::Ready, Some(state)) => state,
        _ => return Content::Hidden,
    };
    let on = power(state) == Some(Power::On);
    match kind {
        BtList::Paired => {
            let rows: Vec<RowData> = paired_devices(state).into_iter().map(|d| paired_row(d, on)).collect();
            if rows.is_empty() { Content::NoPaired } else { Content::Rows(rows, None) }
        }
        BtList::Nearby => {
            if !on {
                return Content::Empty(t!("settings-bluetooth-nearby-off"));
            }
            let pairing = s.pairing.as_ref().map(|p| p.device.as_str());
            let n = nearby_devices(state, show_unnamed, pairing);
            let unnamed_shown = n.devices.iter().filter(|d| bluetooth::is_unnamed(d)).count();
            let rows = n.devices.into_iter().map(|d| nearby_row(d, s)).collect();
            Content::Rows(rows, Some(NearbyInfo { scanning: state.own_discovery, hidden: n.hidden, unnamed_shown }))
        }
    }
}

/// Listenin boş maskotu: yalnız eşleşmiş cihaz yokken (yakındakiler maskotsuz; sayfada en çok
/// bir maskot — hata maskotu varken listeler gizlidir).
pub fn list_spot(kind: BtList) -> Option<Spot> {
    match kind {
        BtList::Paired => Some(Spot::BluetoothPairedEmpty),
        BtList::Nearby => None,
    }
}

fn is_bt(event: &crate::services::Event) -> bool {
    matches!(event, crate::services::Event::Bluetooth | crate::services::Event::LocaleChanged)
}

/// Adaptör satırları (ad, görünürlük) yalnız sayfa hazırken.
fn adapter_ready(s: &Snapshot) -> Option<&Adapter> {
    (page_status(s) == PageStatus::Ready).then(|| s.state.ready().and_then(bluetooth::adapter)).flatten()
}

fn draw_ring(ui: &mut Ui, rect: Rect) {
    let t = ui.theme;
    let radii = Corners::uniform(t.shape.radius_medium * 0.5);
    let color = ui.fade(focus_ring(t, &[t.colors.surface_bright]));
    ui.painter.rect(rect, &RectStyle { fill: Color::TRANSPARENT, radii, border: Some(Border { width: FOCUS_RING, color }) });
}

// ---- Durum (kartsız) ----------------------------------------------------------------------------

/// BlueZ/adaptör yoksa maskotlu hata; değilse son eylem hatasının notu.
#[derive(Debug)]
pub struct BtStatus {
    desc: &'static SettingDesc,
    fiyu: Option<FiyuEmpty>,
    notice: Notice,
    note: bool,
}

impl BtStatus {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        bluetooth::ensure();
        let mut b = Self { desc, fiyu: None, notice: Notice::new(theme, NoticeKind::Warning, ""), note: false };
        b.update(theme);
        b
    }

    fn update(&mut self, theme: &Theme) {
        let s = bluetooth::snapshot();
        self.fiyu = match unavailable_text(page_status(&s)) {
            Some((title, body)) => Some(match self.fiyu.take() {
                Some(mut f) => {
                    f.set_text(theme, &title, Some(&body));
                    f
                }
                None => FiyuEmpty::new(theme, Spot::BluetoothUnavailable, &title, Some(&body)),
            }),
            None => None,
        };
        let note = s.failure.as_ref().map(|f| error_text(&f.error));
        self.note = note.is_some() && self.fiyu.is_none();
        self.notice.set_text(&note.unwrap_or_default());
    }
}

impl Block for BtStatus {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.fiyu.as_ref().map(FiyuEmpty::spot)
    }

    fn visible(&self, _store: &Store) -> bool {
        self.fiyu.is_some() || self.note
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        if let Some(f) = &mut self.fiyu {
            return f.height(env.text, env.theme, width);
        }
        notice_height(&mut self.notice, env, width, 0.0)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        if let Some(f) = &mut self.fiyu {
            f.set_rect(rect);
        } else {
            place_notice(&mut self.notice, env.theme, rect, 0.0);
        }
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_bt(event) {
            self.update(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        match &mut self.fiyu {
            Some(f) => f.draw(ui),
            None => self.notice.draw(ui),
        }
    }
}

// ---- Anahtar satırları (güç, görünürlük) --------------------------------------------------------

/// Satırın tamamı tıklanabilir anahtar (Güç › "Pil ömrünü koru" gibi).
#[derive(Debug)]
struct SwitchRow {
    text: RowText,
    toggle: Toggle,
    hover: Interaction,
    enabled: bool,
    rect: Rect,
}

impl SwitchRow {
    fn new(theme: &Theme, title: &str) -> Self {
        Self { text: RowText::new(theme, title), toggle: Toggle::new(theme, false), hover: Interaction::new(theme), enabled: false, rect: Rect::ZERO }
    }

    fn set(&mut self, subtitle: &str, on: bool, enabled: bool, now: Instant) {
        self.text.set_subtitle(subtitle, false);
        self.toggle.set_on(on, now);
        self.toggle.set_enabled(enabled, now);
        self.enabled = enabled;
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - ts.width).max(t.space(20)));
        row_height_for(t, text.max(ts.height))
    }

    fn place(&mut self, theme: &Theme, rect: Rect) {
        self.rect = rect;
        let (px, _) = row_padding(theme);
        let ts = self.toggle.preferred_size(theme);
        self.toggle.set_rect(Rect::new(rect.right() - px - ts.width, rect.center().y - ts.height * 0.5, ts.width, ts.height));
    }

    /// Tıklandıysa istenen yeni durum.
    fn pointer(&mut self, e: &PointerEvent, now: Instant) -> (bool, Option<bool>) {
        if !self.enabled {
            return (false, None);
        }
        let tap = self.hover.handle(self.rect, e, now);
        (tap.repaint, tap.clicked.then_some(!self.toggle.is_on()))
    }

    fn key(&self, e: &KeyEvent) -> Option<bool> {
        (self.enabled && e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space)).then_some(!self.toggle.is_on())
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rect.contains(p).then_some(if self.enabled { CursorShape::Pointer } else { CursorShape::Default })
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.toggle.set_theme(theme, now);
        self.hover.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        self.hover.draw_layer(ui.painter, self.rect, Corners::uniform(t.shape.radius_medium * 0.5), t, ui.now);
        if self.hover.is_animating(ui.now) {
            ui.request_frame();
        }
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.center().y - h * 0.5);
        self.toggle.draw(ui);
    }
}

/// Bluetooth aç/kapa.
#[derive(Debug)]
pub struct BtPowerRow {
    desc: &'static SettingDesc,
    row: SwitchRow,
}

impl BtPowerRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        bluetooth::ensure();
        let mut b = Self { desc, row: SwitchRow::new(theme, desc.label()) };
        b.update(Instant::now());
        b
    }

    fn update(&mut self, now: Instant) {
        let s = bluetooth::snapshot();
        match (page_status(&s), s.state.ready().and_then(power)) {
            (PageStatus::Ready, Some(p)) => {
                // Donanım anahtarı ve geçiş sürerken anahtar kullanılamaz; yumuşak engelde açmayı
                // deneyebilir (BlueZ reddederse not nedeni söyler).
                let enabled = !matches!(p, Power::HardBlocked | Power::Turning);
                self.row.set(&power_text(p), p == Power::On, enabled, now);
            }
            _ => self.row.set(&t!("settings-bluetooth-reading"), false, false, now),
        }
    }
}

impl Block for BtPowerRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        matches!(page_status(&bluetooth::snapshot()), PageStatus::Ready | PageStatus::Reading)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.row.height(env, width)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.row.place(env.theme, rect);
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let (repaint, flip) = self.row.pointer(e, env.now);
        if let Some(on) = flip {
            bluetooth::set_powered(on);
            return true;
        }
        repaint
    }

    fn key(&mut self, _env: &mut Env, e: &KeyEvent) -> bool {
        self.row.key(e).map(bluetooth::set_powered).is_some()
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.row.toggle.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.row.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_bt(event) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.row.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.row.draw(ui);
    }
}

/// Görünürlük (süreli).
#[derive(Debug)]
pub struct BtVisibleRow {
    desc: &'static SettingDesc,
    row: SwitchRow,
}

impl BtVisibleRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        bluetooth::ensure();
        let mut b = Self { desc, row: SwitchRow::new(theme, desc.label()) };
        b.update(Instant::now());
        b
    }

    fn update(&mut self, now: Instant) {
        let s = bluetooth::snapshot();
        let Some(a) = adapter_ready(&s) else { return };
        let on = bluetooth::powered_on(&s);
        self.row.set(&discoverable_text(a, s.discoverable_since, now), a.discoverable, on, now);
    }
}

impl Block for BtVisibleRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        adapter_ready(&bluetooth::snapshot()).is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.row.height(env, width)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.row.place(env.theme, rect);
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let (repaint, flip) = self.row.pointer(e, env.now);
        if let Some(on) = flip {
            bluetooth::set_discoverable(on);
            return true;
        }
        repaint
    }

    fn key(&mut self, _env: &mut Env, e: &KeyEvent) -> bool {
        self.row.key(e).map(bluetooth::set_discoverable).is_some()
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.row.toggle.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.row.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_bt(event) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.row.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.row.draw(ui);
    }
}

// ---- Adaptör adı --------------------------------------------------------------------------------

/// En fazla bayt (Bluetooth adı sınırı; crate `valid_alias` aynı sınırı denetler).
const MAX_ALIAS_BYTES: usize = 248;

/// Bu bilgisayarın Bluetooth adı: Enter ya da odak kaybında yazılır; Esc geri alır.
#[derive(Debug)]
pub struct BtNameRow {
    desc: &'static SettingDesc,
    text: RowText,
    input: TextInput,
    focused: bool,
    /// Kullanıcı alana yazdı (servisten gelen değer üzerine yazılmaz).
    edited: bool,
    rect: Rect,
}

impl BtNameRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        bluetooth::ensure();
        let mut b = Self {
            desc,
            text: RowText::new(theme, desc.label()),
            input: TextInput::new(theme).with_max_bytes(MAX_ALIAS_BYTES),
            focused: false,
            edited: false,
            rect: Rect::ZERO,
        };
        b.text.set_subtitle(&t!("settings-bluetooth-name-hint"), false);
        b.update();
        b
    }

    fn update(&mut self) {
        let s = bluetooth::snapshot();
        let Some(a) = adapter_ready(&s) else { return };
        if !self.edited && self.input.text() != a.name {
            self.input.set_text(&a.name);
        }
    }

    fn commit(&mut self) {
        if !self.edited {
            return;
        }
        self.edited = false;
        let current = adapter_ready(&bluetooth::snapshot()).map(|a| a.name.clone()).unwrap_or_default();
        if self.input.text().trim() != current {
            bluetooth::set_name(self.input.text());
        }
        self.update();
    }

    fn input_width(theme: &Theme, width: f32) -> f32 {
        (width * 0.4).clamp(theme.space(40), theme.space(64))
    }
}

impl Block for BtNameRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        adapter_ready(&bluetooth::snapshot()).is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let iw = Self::input_width(t, width);
        let ih = self.input.preferred_size(t, iw).height;
        let text = self.text.height(env.text, t, (width - px * 3.0 - iw).max(t.space(20)));
        row_height_for(t, text.max(ih))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let iw = Self::input_width(t, rect.width);
        let ih = self.input.preferred_size(t, iw).height;
        self.input.set_rect(Rect::new(rect.right() - px - iw, rect.center().y - ih * 0.5, iw, ih));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.input.pointer(env.text, env.theme, e, env.now);
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && self.input.rect().contains(e.position)
        {
            env.push(Action::Focus);
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.input.key(e, env.now);
        match r.event {
            Some(TextInputEvent::Submit) => {
                self.commit();
                true
            }
            Some(TextInputEvent::Cancel) => {
                self.edited = false;
                self.input.set_text("");
                self.update();
                true
            }
            Some(TextInputEvent::Changed) => {
                self.edited = true;
                true
            }
            _ => r.repaint,
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        let was = self.focused;
        self.focused = focused;
        self.input.set_focused(focused, now);
        if was && !focused {
            // Odak giderken yazılanı kaydet (Enter'a basılmasa da).
            self.commit();
        }
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        Some(&mut self.input)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.input.cursor(p)
    }

    fn sync(&mut self, _env: &mut Env) {
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if is_bt(event) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.input.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let (px, _) = row_padding(ui.theme);
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.center().y - h * 0.5);
        self.input.draw(ui);
    }
}

// ---- Cihaz listeleri ----------------------------------------------------------------------------

fn icon_side(theme: &Theme) -> f32 {
    text_style(theme, TextRole::Icon).size
}

#[derive(Debug)]
struct RowUi {
    data: RowData,
    icon: IconView,
    title: Label,
    subtitle: Label,
    battery: Option<(IconView, Label)>,
    hover: Interaction,
    /// Yakındaki cihazda "Eşleştir" / "İptal".
    action: Option<Button>,
    height: f32,
    rect: Rect,
}

impl RowUi {
    fn new(theme: &Theme, d: RowData) -> Self {
        let action = match d.kind {
            RowKind::Nearby { pairing: true, .. } => {
                Some(Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-bluetooth-cancel")))
            }
            RowKind::Nearby { can_pair, .. } => {
                let mut b = Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-bluetooth-pair"));
                b.set_enabled(can_pair, Instant::now());
                Some(b)
            }
            RowKind::Paired { .. } => None,
        };
        let battery = d.battery.map(|pct| {
            (
                IconView::with_role(theme, Icon::named(IconName::battery(pct)), TextRole::IconSmall),
                Label::new(battery_text(pct), text_style(theme, TextRole::Caption).with_tabular_numbers().with_max_lines(1)),
            )
        });
        Self {
            icon: IconView::new(theme, Icon::named(d.icon)).filled(d.active),
            title: Label::new(d.title.as_str(), text_style(theme, TextRole::Body).with_max_lines(1)),
            subtitle: Label::new(d.subtitle.as_str(), text_style(theme, TextRole::Caption).with_max_lines(1)),
            battery,
            hover: Interaction::new(theme),
            action,
            height: 0.0,
            rect: Rect::ZERO,
            data: d,
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.title.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        self.subtitle.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
        if let Some((i, l)) = &mut self.battery {
            i.set_theme(theme, now);
            l.set_style(text_style(theme, TextRole::Caption).with_tabular_numbers().with_max_lines(1));
        }
        self.hover.set_theme(theme, now);
        if let Some(b) = &mut self.action {
            b.set_theme(theme, now);
        }
    }

    fn paired(&self) -> bool {
        matches!(self.data.kind, RowKind::Paired { .. })
    }
}

/// Eşleşmiş cihazın satır içi ayrıntısı.
#[derive(Debug)]
struct DetailUi {
    path: String,
    name_text: Label,
    name: TextInput,
    name_edited: bool,
    trusted_text: RowText,
    trusted: Toggle,
    blocked_text: RowText,
    blocked: Toggle,
    connect: Button,
    forget: ConfirmButton,
    /// Satır yükseklikleri: ad, güvenilir, engelle, düğmeler.
    heights: [f32; 4],
    rect: Rect,
}

impl DetailUi {
    fn new(theme: &Theme, d: &RowData) -> Self {
        let mut trusted_text = RowText::new(theme, &t!("settings-bluetooth-trusted"));
        trusted_text.set_subtitle(&t!("settings-bluetooth-trusted-hint"), false);
        let mut blocked_text = RowText::new(theme, &t!("settings-bluetooth-block"));
        blocked_text.set_subtitle(&t!("settings-bluetooth-block-hint"), false);
        let mut b = Self {
            path: d.path.clone(),
            name_text: Label::new(t!("settings-bluetooth-rename"), text_style(theme, TextRole::Body).with_max_lines(1)),
            name: TextInput::new(theme).with_max_bytes(MAX_ALIAS_BYTES),
            name_edited: false,
            trusted_text,
            trusted: Toggle::new(theme, false),
            blocked_text,
            blocked: Toggle::new(theme, false),
            connect: Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-bluetooth-connect")),
            forget: ConfirmButton::new(theme, ButtonVariant::Text, Icon::named(IconName::Delete), t!("settings-bluetooth-forget-confirm"))
                .with_label(theme, t!("settings-bluetooth-forget")),
            heights: [0.0; 4],
            rect: Rect::ZERO,
        };
        b.update(d, Instant::now());
        b
    }

    fn update(&mut self, d: &RowData, now: Instant) {
        let RowKind::Paired { connected, trusted, blocked, powered } = d.kind else { return };
        if !self.name_edited && self.name.text() != d.alias {
            self.name.set_text(&d.alias);
        }
        self.trusted.set_on(trusted, now);
        self.blocked.set_on(blocked, now);
        self.connect.set_label(&if connected { t!("settings-bluetooth-disconnect") } else { t!("settings-bluetooth-connect") });
        // Engelli cihaz bağlanamaz; adaptör kapalıyken bağlantı yok.
        self.connect.set_enabled(powered && (connected || !blocked), now);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.name_text.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        self.name.set_theme(theme, now);
        self.trusted_text.set_theme(theme);
        self.trusted.set_theme(theme, now);
        self.blocked_text.set_theme(theme);
        self.blocked.set_theme(theme, now);
        self.connect.set_theme(theme, now);
        self.forget.set_theme(theme, now);
    }

    /// Ayrıntının sol kenarı (satır metniyle hizalı).
    fn indent(theme: &Theme) -> f32 {
        let (px, _) = row_padding(theme);
        px + icon_side(theme) + theme.space(4)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let left = Self::indent(t);
        let inner = (width - left - px).max(t.space(20));
        let iw = (inner * 0.5).clamp(t.space(30), t.space(64));
        let ih = self.name.preferred_size(t, iw).height;
        let nh = self.name_text.measure(env.text).height;
        let ts = self.trusted.preferred_size(t);
        let text_w = (inner - ts.width - px).max(t.space(20));
        let th = self.trusted_text.height(env.text, t, text_w);
        let bh = self.blocked_text.height(env.text, t, text_w);
        let cb = self.connect.preferred_size(env.text, t);
        let fb = self.forget.max_size(env.text, t);
        self.heights = [
            row_height_for(t, nh.max(ih)),
            row_height_for(t, th.max(ts.height)),
            row_height_for(t, bh.max(ts.height)),
            cb.height.max(fb.height) + py * 2.0,
        ];
        self.heights.iter().sum()
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let left = rect.x + Self::indent(t);
        let right = rect.right() - px;
        let inner = (right - left).max(t.space(20));
        let iw = (inner * 0.5).clamp(t.space(30), t.space(64));
        let mut y = rect.y;
        let ih = self.name.preferred_size(t, iw).height;
        let h0 = self.heights[0];
        self.name.set_rect(Rect::new(right - iw, y + (h0 - ih) * 0.5, iw, ih));
        y += h0;
        let ts = self.trusted.preferred_size(t);
        self.trusted.set_rect(Rect::new(right - ts.width, y + (self.heights[1] - ts.height) * 0.5, ts.width, ts.height));
        y += self.heights[1];
        self.blocked.set_rect(Rect::new(right - ts.width, y + (self.heights[2] - ts.height) * 0.5, ts.width, ts.height));
        y += self.heights[2];
        let cb = self.connect.preferred_size(env.text, t);
        self.connect.set_rect(Rect::new(left, y + py, cb.width, cb.height));
        let fb = self.forget.max_size(env.text, t);
        self.forget.set_rect(Rect::new(right - fb.width, y + py, fb.width, fb.height));
    }

    /// Yazılan adı servise verir (değişmediyse yazmaz).
    fn commit_name(&mut self, alias: &str) {
        if !self.name_edited {
            return;
        }
        self.name_edited = false;
        if self.name.text().trim() != alias {
            bluetooth::rename(&self.path, self.name.text());
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let left = self.rect.x + Self::indent(t);
        let mut y = self.rect.y;
        let nh = self.name_text.measure_in(ui.painter).height;
        self.name_text.draw(ui.painter, Point::new(left, y + (self.heights[0] - nh) * 0.5), ui.fade(c.on_surface));
        self.name.draw(ui);
        y += self.heights[0];
        let h = self.trusted_text.drawn_height(ui);
        self.trusted_text.draw(ui, left, y + (self.heights[1] - h) * 0.5);
        self.trusted.draw(ui);
        y += self.heights[1];
        let h = self.blocked_text.drawn_height(ui);
        self.blocked_text.draw(ui, left, y + (self.heights[2] - h) * 0.5);
        self.blocked.draw(ui);
        self.connect.draw(ui);
        self.forget.draw(ui);
    }
}

/// Klavye odağının listedeki hedefi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    /// Eşleşmiş cihaz satırı (Enter: ayrıntıyı aç/kapa).
    Row(usize),
    Name,
    Trusted,
    Blocked,
    Connect,
    Forget,
    /// Yakındaki cihazın "Eşleştir"/"İptal" düğmesi.
    Action(usize),
    Rescan,
    Unnamed,
}

/// Cihaz listesi kartı (eşleşmiş / yakındaki). Satırlar yalnızca veri değişince yeniden kurulur;
/// açık ayrıntı yeniden kurulumda korunur.
#[derive(Debug)]
pub struct BtDeviceList {
    desc: &'static SettingDesc,
    kind: BtList,
    data: Content,
    rows: Vec<RowUi>,
    /// Açık ayrıntının cihaz yolu (eşleşmiş listede; en çok bir).
    expanded: Option<String>,
    detail: Option<DetailUi>,
    empty: Label,
    fiyu: Option<FiyuEmpty>,
    // Yakındakiler: tarama satırı ve adsız cihaz satırı.
    show_unnamed: bool,
    scan_text: Label,
    spinner: Spinner,
    rescan: Button,
    none_text: Label,
    unnamed_text: Label,
    unnamed: Button,
    scan_h: f32,
    none_h: f32,
    unnamed_h: f32,
    focus: Option<Target>,
    focus_visible: bool,
    rect: Rect,
}

impl BtDeviceList {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, kind: BtList) -> Self {
        bluetooth::ensure();
        let mut b = Self {
            desc,
            kind,
            data: Content::Hidden,
            rows: Vec::new(),
            expanded: None,
            detail: None,
            empty: Label::new("", text_style(theme, TextRole::Body)),
            fiyu: None,
            show_unnamed: false,
            scan_text: Label::new("", text_style(theme, TextRole::Caption).with_max_lines(1)),
            spinner: Spinner::new(theme),
            rescan: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-bluetooth-rescan")),
            none_text: Label::new(t!("settings-bluetooth-nearby-empty"), text_style(theme, TextRole::Body)),
            unnamed_text: Label::new("", text_style(theme, TextRole::Caption).with_max_lines(1)),
            unnamed: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-bluetooth-unnamed-show")),
            scan_h: 0.0,
            none_h: 0.0,
            unnamed_h: 0.0,
            focus: None,
            focus_visible: false,
            rect: Rect::ZERO,
        };
        b.update(theme);
        b
    }

    fn update(&mut self, theme: &Theme) {
        self.apply(theme, content(self.kind, &bluetooth::snapshot(), self.show_unnamed));
    }

    fn apply(&mut self, theme: &Theme, data: Content) {
        if data == self.data {
            return;
        }
        let now = Instant::now();
        match &data {
            Content::Rows(rows, info) => {
                self.rows = rows.iter().cloned().map(|d| RowUi::new(theme, d)).collect();
                if let Some(info) = info {
                    self.scan_text.set_text(&if info.scanning { t!("settings-bluetooth-scanning") } else { t!("settings-bluetooth-scan-stopped") });
                    let (text, button) = if info.hidden > 0 {
                        (t!("settings-bluetooth-unnamed-hidden", count = info.hidden as i64), t!("settings-bluetooth-unnamed-show"))
                    } else {
                        (t!("settings-bluetooth-unnamed-shown", count = info.unnamed_shown as i64), t!("settings-bluetooth-unnamed-hide"))
                    };
                    self.unnamed_text.set_text(&text);
                    self.unnamed.set_label(&button);
                }
            }
            Content::Empty(text) => {
                self.rows.clear();
                self.empty.set_text(text);
            }
            Content::NoPaired | Content::Hidden => self.rows.clear(),
        }
        // Açık ayrıntı: cihaz hâlâ listedeyse korunur (alan/anahtarlar güncellenir).
        let open = self.expanded.as_deref().and_then(|p| self.rows.iter().find(|r| r.data.path == p)).map(|r| r.data.clone());
        match (open, &mut self.detail) {
            (Some(d), Some(ui)) if ui.path == d.path => ui.update(&d, now),
            (Some(d), _) => self.detail = Some(DetailUi::new(theme, &d)),
            (None, _) => {
                self.expanded = None;
                self.detail = None;
            }
        }
        self.fiyu = match (&data, list_spot(self.kind)) {
            (Content::NoPaired, Some(spot)) => {
                let (title, body) = (t!("settings-bluetooth-paired-empty"), t!("settings-bluetooth-paired-empty-body"));
                Some(match self.fiyu.take() {
                    Some(mut f) => {
                        f.set_text(theme, &title, Some(&body));
                        f
                    }
                    None => FiyuEmpty::new(theme, spot, &title, Some(&body)),
                })
            }
            _ => None,
        };
        self.data = data;
        // Odak hedefi kaybolduysa (satır gitti) odak listenin başına.
        if let Some(f) = self.focus
            && !self.targets().contains(&f)
        {
            self.focus = self.targets().first().copied();
            self.sync_focus();
        }
    }

    fn nearby_info(&self) -> Option<NearbyInfo> {
        match &self.data {
            Content::Rows(_, info) => *info,
            _ => None,
        }
    }

    /// Klavye hedefleri, görsel sırada.
    fn targets(&self) -> Vec<Target> {
        let mut v = Vec::new();
        if let Some(info) = self.nearby_info() {
            if !info.scanning {
                v.push(Target::Rescan);
            }
            for (i, r) in self.rows.iter().enumerate() {
                if r.action.as_ref().is_some_and(Button::is_enabled) {
                    v.push(Target::Action(i));
                }
            }
            if info.hidden > 0 || info.unnamed_shown > 0 {
                v.push(Target::Unnamed);
            }
            return v;
        }
        for (i, r) in self.rows.iter().enumerate() {
            v.push(Target::Row(i));
            if self.expanded.as_deref() == Some(r.data.path.as_str()) {
                v.extend([Target::Name, Target::Trusted, Target::Blocked]);
                if self.detail.as_ref().is_some_and(|d| d.connect.is_enabled()) {
                    v.push(Target::Connect);
                }
                v.push(Target::Forget);
            }
        }
        v
    }

    /// Odak görselini bileşenlere yansıtır.
    fn sync_focus(&mut self) {
        let f = if self.focus_visible { self.focus } else { None };
        if let Some(d) = &mut self.detail {
            d.trusted.set_focus_visible(f == Some(Target::Trusted));
            d.blocked.set_focus_visible(f == Some(Target::Blocked));
            d.connect.set_focus_visible(f == Some(Target::Connect));
            d.forget.set_focus_visible(f == Some(Target::Forget));
            d.name.set_focused(self.focus == Some(Target::Name), Instant::now());
        }
        for (i, r) in self.rows.iter_mut().enumerate() {
            if let Some(b) = &mut r.action {
                b.set_focus_visible(f == Some(Target::Action(i)));
            }
        }
        self.rescan.set_focus_visible(f == Some(Target::Rescan));
        self.unnamed.set_focus_visible(f == Some(Target::Unnamed));
    }

    /// Odak ayrıntının adından çıkıyorsa yazılanı kaydet.
    fn leave_name(&mut self, next: Option<Target>) {
        if self.focus == Some(Target::Name) && next != Some(Target::Name) {
            let alias = self.expanded_row().map(|r| r.alias.clone()).unwrap_or_default();
            if let Some(d) = &mut self.detail {
                d.commit_name(&alias);
            }
        }
    }

    fn set_target(&mut self, next: Option<Target>) {
        self.leave_name(next);
        self.focus = next;
        self.sync_focus();
    }

    fn expanded_row(&self) -> Option<&RowData> {
        let p = self.expanded.as_deref()?;
        self.rows.iter().find(|r| r.data.path == p).map(|r| &r.data)
    }

    fn toggle_expand(&mut self, theme: &Theme, i: usize) {
        let Some(data) = self.rows.get(i).map(|r| r.data.clone()) else { return };
        // Açık ayrıntının adı yazılmışsa kaydedilir; odak satıra döner.
        self.leave_name(None);
        if self.focus.is_some() {
            self.focus = Some(Target::Row(i));
        }
        if self.expanded.as_deref() == Some(data.path.as_str()) {
            self.expanded = None;
            self.detail = None;
        } else {
            self.detail = Some(DetailUi::new(theme, &data));
            self.expanded = Some(data.path);
        }
        self.sync_focus();
    }

    /// Hedefi etkinleştir (Enter/Space ya da tıklama sonrası ortak yol).
    fn activate(&mut self, theme: &Theme, target: Target, now: Instant) -> bool {
        match target {
            Target::Row(i) => self.toggle_expand(theme, i),
            Target::Name => return false,
            Target::Trusted | Target::Blocked => {
                let Some(d) = &mut self.detail else { return false };
                let path = d.path.clone();
                if target == Target::Trusted {
                    bluetooth::set_trusted(&path, !d.trusted.is_on());
                } else {
                    bluetooth::set_blocked(&path, !d.blocked.is_on());
                }
            }
            Target::Connect => self.connect_expanded(),
            Target::Forget => {
                // Klavye: ConfirmButton kendi iki basış akışını yürütür (key içinde).
                let _ = now;
                return false;
            }
            Target::Action(i) => self.row_action(i),
            Target::Rescan => bluetooth::rescan(true),
            Target::Unnamed => {
                self.show_unnamed = !self.show_unnamed;
                self.update(theme);
            }
        }
        true
    }

    fn connect_expanded(&mut self) {
        let Some(r) = self.expanded_row() else { return };
        let path = r.path.clone();
        if matches!(r.kind, RowKind::Paired { connected: true, .. }) {
            bluetooth::disconnect(&path);
        } else {
            bluetooth::connect(&path);
        }
    }

    fn row_action(&mut self, i: usize) {
        let Some(r) = self.rows.get(i) else { return };
        match r.data.kind {
            RowKind::Nearby { pairing: true, .. } => bluetooth::cancel_pairing(),
            RowKind::Nearby { can_pair: true, .. } => bluetooth::pair(&r.data.path),
            _ => {}
        }
    }

    fn forget_expanded(&mut self) {
        let Some(path) = self.expanded.take() else { return };
        self.detail = None;
        bluetooth::forget(&path);
    }

    /// Satır başlık bölümünün dikdörtgeni (ayrıntı hariç).
    fn trailing_width(r: &mut RowUi, env: &mut Env) -> f32 {
        let t = env.theme;
        let mut w = 0.0;
        if let Some((_, l)) = &mut r.battery {
            w += text_style(t, TextRole::IconSmall).size + t.space(1) + l.measure(env.text).width + t.space(3);
        }
        if let Some(b) = &mut r.action {
            w += b.preferred_size(env.text, t).width + t.space(3);
        }
        w
    }

    fn line_height(env: &mut Env, label: &mut Label, width: f32, extra: f32) -> f32 {
        label.set_max_width(Some(width.max(env.theme.space(20))));
        row_height_for(env.theme, label.measure(env.text).height.max(extra))
    }
}

impl Block for BtDeviceList {
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
        content(self.kind, &bluetooth::snapshot(), self.show_unnamed) != Content::Hidden
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            return f.height(env.text, t, width);
        }
        if self.detail.is_none()
            && let Some(d) = self.expanded_row().cloned()
        {
            self.detail = Some(DetailUi::new(t, &d));
        }
        if !matches!(self.data, Content::Rows(..)) {
            self.empty.set_max_width(Some((width - px * 2.0).max(t.space(20))));
            return row_height_for(t, self.empty.measure(env.text).height);
        }
        let mut total = 0.0;
        if let Some(info) = self.nearby_info() {
            let bw = if info.scanning { self.spinner.preferred_size(t).width } else { self.rescan.preferred_size(env.text, t).width };
            let bh = self.rescan.preferred_size(env.text, t).height;
            self.scan_h = Self::line_height(env, &mut self.scan_text, width - px * 3.0 - bw, bh);
            total += self.scan_h;
            self.none_h = if self.rows.is_empty() && !info.scanning { Self::line_height(env, &mut self.none_text, width - px * 2.0, 0.0) } else { 0.0 };
            total += self.none_h;
            self.unnamed_h = if info.hidden > 0 || info.unnamed_shown > 0 {
                let uw = self.unnamed.preferred_size(env.text, t).width;
                Self::line_height(env, &mut self.unnamed_text, width - px * 3.0 - uw, bh)
            } else {
                0.0
            };
            total += self.unnamed_h;
        }
        for i in 0..self.rows.len() {
            let trailing = Self::trailing_width(&mut self.rows[i], env);
            let r = &mut self.rows[i];
            let tw = (width - px * 2.0 - icon_side(t) - t.space(4) - trailing).max(t.space(20));
            r.title.set_max_width(Some(tw));
            r.subtitle.set_max_width(Some(tw));
            let mut text = r.title.measure(env.text).height;
            if !r.subtitle.text().is_empty() {
                text += t.space(1) + r.subtitle.measure(env.text).height;
            }
            r.height = row_height_for(t, text.max(icon_side(t)));
            total += r.height;
            if self.expanded.as_deref() == Some(r.data.path.as_str())
                && let Some(d) = &mut self.detail
            {
                total += d.height(env, width);
            }
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
        let mut y = rect.y;
        if let Some(info) = self.nearby_info() {
            let mid = y + self.scan_h * 0.5;
            if info.scanning {
                let s = self.spinner.preferred_size(t);
                self.spinner.set_rect(Rect::new(rect.right() - px - s.width, mid - s.height * 0.5, s.width, s.height));
            } else {
                let b = self.rescan.preferred_size(env.text, t);
                self.rescan.set_rect(Rect::new(rect.right() - px - b.width, mid - b.height * 0.5, b.width, b.height));
            }
            y += self.scan_h + self.none_h;
        }
        let expanded = self.expanded.clone();
        for r in &mut self.rows {
            r.rect = Rect::new(rect.x, y, rect.width, r.height);
            if let Some(b) = &mut r.action {
                let s = b.preferred_size(env.text, t);
                b.set_rect(Rect::new(r.rect.right() - px - s.width, r.rect.center().y - s.height * 0.5, s.width, s.height));
            }
            y += r.height;
            if expanded.as_deref() == Some(r.data.path.as_str())
                && let Some(d) = &mut self.detail
            {
                let h = d.heights.iter().sum();
                d.place(env, Rect::new(rect.x, y, rect.width, h));
                y += h;
            }
        }
        if self.unnamed_h > 0.0 {
            let b = self.unnamed.preferred_size(env.text, t);
            self.unnamed.set_rect(Rect::new(rect.right() - px - b.width, y + (self.unnamed_h - b.height) * 0.5, b.width, b.height));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let theme = env.theme;
        let mut repaint = false;
        let press = matches!(e.kind, PointerKind::Press(MouseButton::Left));
        // Ayrıntı.
        let mut clicked: Option<Target> = None;
        let mut forget = false;
        if let Some(d) = &mut self.detail {
            let r = d.name.pointer(env.text, theme, e, now);
            repaint |= r.repaint;
            if press && d.name.rect().contains(e.position) {
                clicked = Some(Target::Name);
            }
            let r = d.trusted.pointer(e, now);
            repaint |= r.repaint;
            if let Some(on) = r.event {
                bluetooth::set_trusted(&d.path, on);
            }
            let r = d.blocked.pointer(e, now);
            repaint |= r.repaint;
            if let Some(on) = r.event {
                bluetooth::set_blocked(&d.path, on);
            }
            let r = d.connect.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                clicked = Some(Target::Connect);
            }
            let r = d.forget.pointer(e, now);
            repaint |= r.repaint;
            forget = r.event == Some(ConfirmEvent::Confirmed);
        }
        if press && self.rect.contains(e.position) && clicked != Some(Target::Name) && self.focus == Some(Target::Name) {
            // Alanın dışına tıklandı: yazılan ad kaydedilir.
            self.set_target(None);
        }
        if clicked == Some(Target::Name) {
            self.set_target(Some(Target::Name));
            env.push(Action::Focus);
        }
        if clicked == Some(Target::Connect) {
            self.connect_expanded();
        }
        if forget {
            self.forget_expanded();
            return true;
        }
        // Satırlar.
        let mut row_click = None;
        let mut action_click = None;
        for (i, r) in self.rows.iter_mut().enumerate() {
            if let Some(b) = &mut r.action {
                let res = b.pointer(e, now);
                repaint |= res.repaint;
                if res.event.is_some() {
                    action_click = Some(i);
                }
            }
            if r.paired() {
                let tap = r.hover.handle(r.rect, e, now);
                repaint |= tap.repaint;
                if tap.clicked {
                    row_click = Some(i);
                }
            }
        }
        if let Some(i) = action_click {
            self.row_action(i);
            repaint = true;
        }
        if let Some(i) = row_click {
            self.toggle_expand(theme, i);
            self.focus = Some(Target::Row(i));
            self.sync_focus();
            repaint = true;
        }
        if self.nearby_info().is_some_and(|i| !i.scanning) && self.rescan.pointer(e, now).event.is_some() {
            bluetooth::rescan(true);
            repaint = true;
        }
        if self.unnamed_h > 0.0 {
            let r = self.unnamed.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.show_unnamed = !self.show_unnamed;
                self.update(theme);
                repaint = true;
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let now = env.now;
        match self.focus {
            Some(Target::Name) => {
                let alias = self.expanded_row().map(|r| r.alias.clone()).unwrap_or_default();
                let Some(d) = &mut self.detail else { return false };
                let r = d.name.key(e, now);
                match r.event {
                    Some(TextInputEvent::Submit) => {
                        d.commit_name(&alias);
                        true
                    }
                    Some(TextInputEvent::Cancel) => {
                        d.name_edited = false;
                        d.name.set_text(&alias);
                        true
                    }
                    Some(TextInputEvent::Changed) => {
                        d.name_edited = true;
                        true
                    }
                    _ => r.repaint,
                }
            }
            Some(Target::Forget) => {
                let Some(d) = &mut self.detail else { return false };
                let r = d.forget.key(e, now);
                if r.event == Some(ConfirmEvent::Confirmed) {
                    self.forget_expanded();
                    self.focus = self.targets().first().copied();
                    self.sync_focus();
                    return true;
                }
                r.repaint || r.event.is_some()
            }
            Some(target) if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) => {
                self.activate(env.theme, target, now)
            }
            Some(_) if e.state != KeyState::Released && e.key == Key::Escape && self.expanded.is_some() => {
                // Esc: ayrıntıyı kapat, odak satıra döner.
                let i = self.rows.iter().position(|r| Some(r.data.path.as_str()) == self.expanded.as_deref());
                if let Some(i) = i {
                    self.toggle_expand(env.theme, i);
                    self.focus = Some(Target::Row(i));
                    self.sync_focus();
                }
                true
            }
            _ => false,
        }
    }

    fn focusable(&self) -> bool {
        !self.targets().is_empty()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focus_visible = visible;
        if !focused {
            self.set_target(None);
            return;
        }
        if self.focus.is_none() && visible {
            self.focus = self.targets().first().copied();
        }
        self.sync_focus();
    }

    fn focus_step(&mut self, forward: bool, _now: Instant) -> bool {
        let targets = self.targets();
        let Some(cur) = self.focus.and_then(|f| targets.iter().position(|&x| x == f)) else { return false };
        let next = if forward { targets.get(cur + 1) } else { cur.checked_sub(1).and_then(|i| targets.get(i)) };
        match next {
            Some(&n) => {
                self.focus_visible = true;
                self.set_target(Some(n));
                true
            }
            None => false,
        }
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        if self.focus != Some(Target::Name) {
            return None;
        }
        self.detail.as_mut().map(|d| &mut d.name)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if let Some(d) = &self.detail {
            let c = d.name.cursor(p).or_else(|| d.trusted.cursor(p)).or_else(|| d.blocked.cursor(p)).or_else(|| d.connect.cursor(p)).or_else(|| d.forget.cursor(p));
            if c.is_some() {
                return c;
            }
        }
        for r in &self.rows {
            if let Some(c) = r.action.as_ref().and_then(|b| b.cursor(p)) {
                return Some(c);
            }
            if r.paired() && r.rect.contains(p) {
                return Some(CursorShape::Pointer);
            }
        }
        if self.nearby_info().is_some_and(|i| !i.scanning)
            && let Some(c) = self.rescan.cursor(p)
        {
            return Some(c);
        }
        if self.unnamed_h > 0.0 { self.unnamed.cursor(p) } else { None }
    }

    fn demo_expand(&mut self, now: Instant) {
        // Canlı test (`NUXUT_SETTINGS_DEBUG=expand=bluetooth.paired`): ilk eşleşmiş cihazın ayrıntısı.
        // Ayrıntı bileşeni sonraki yerleşimde (tema orada) kurulur.
        let _ = now;
        if self.kind == BtList::Paired && self.expanded.is_none() {
            self.expanded = self.rows.first().map(|r| r.data.path.clone());
            self.detail = None;
        }
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_bt(event) {
            if matches!(event, crate::services::Event::LocaleChanged) {
                // Metinler yeni dilde: satırlar yeniden kurulur.
                self.data = Content::Hidden;
                self.detail = None;
                self.rescan.set_label(&t!("settings-bluetooth-rescan"));
                self.none_text.set_text(&t!("settings-bluetooth-nearby-empty"));
            }
            self.update(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.empty.set_style(text_style(theme, TextRole::Body));
        self.none_text.set_style(text_style(theme, TextRole::Body));
        self.scan_text.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
        self.unnamed_text.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
        self.spinner.set_theme(theme, now);
        self.rescan.set_theme(theme, now);
        self.unnamed.set_theme(theme, now);
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
        for r in &mut self.rows {
            r.set_theme(theme, now);
        }
        if let Some(d) = &mut self.detail {
            d.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            f.draw(ui);
            return;
        }
        if !matches!(self.data, Content::Rows(..)) {
            let h = self.empty.measure_in(ui.painter).height;
            self.empty.draw(ui.painter, Point::new(self.rect.x + px, self.rect.center().y - h * 0.5), ui.fade(c.on_surface_variant));
            return;
        }
        let mut y = self.rect.y;
        if let Some(info) = self.nearby_info() {
            let h = self.scan_text.measure_in(ui.painter).height;
            self.scan_text.draw(ui.painter, Point::new(self.rect.x + px, y + (self.scan_h - h) * 0.5), ui.fade(c.on_surface_variant));
            if info.scanning {
                self.spinner.draw(ui);
            } else {
                self.rescan.draw(ui);
            }
            y += self.scan_h;
            if self.none_h > 0.0 {
                let h = self.none_text.measure_in(ui.painter).height;
                self.none_text.draw(ui.painter, Point::new(self.rect.x + px, y + (self.none_h - h) * 0.5), ui.fade(c.on_surface_variant));
            }
        }
        let side = icon_side(t);
        let focus = if self.focus_visible { self.focus } else { None };
        for (i, r) in self.rows.iter_mut().enumerate() {
            let mid = r.rect.center().y;
            if r.paired() {
                r.hover.draw_layer(ui.painter, r.rect, Corners::uniform(t.shape.radius_medium * 0.5), t, ui.now);
                if r.hover.is_animating(ui.now) {
                    ui.request_frame();
                }
                if focus == Some(Target::Row(i)) {
                    draw_ring(ui, r.rect);
                }
            }
            let icon_color = if r.data.active { c.primary } else { c.on_surface_variant };
            r.icon.draw_centered(ui.painter, Point::new(r.rect.x + px + side * 0.5, mid), ui.fade(icon_color));
            let tx = r.rect.x + px + side + t.space(4);
            let th = r.title.measure_in(ui.painter).height;
            let sh = if r.subtitle.text().is_empty() { 0.0 } else { t.space(1) + r.subtitle.measure_in(ui.painter).height };
            let top = mid - (th + sh) * 0.5;
            r.title.draw(ui.painter, Point::new(tx, top), ui.fade(c.on_surface));
            if sh > 0.0 {
                let color = if r.data.error { c.error } else { c.on_surface_variant };
                r.subtitle.draw(ui.painter, Point::new(tx, top + th + t.space(1)), ui.fade(color));
            }
            let mut right = r.rect.right() - px;
            if let Some(b) = &mut r.action {
                right = b.rect().x - t.space(3);
                b.draw(ui);
            }
            if let Some((icon, label)) = &mut r.battery {
                let ls = label.measure_in(ui.painter);
                label.draw(ui.painter, Point::new(right - ls.width, mid - ls.height * 0.5), ui.fade(c.on_surface_variant));
                let s = text_style(t, TextRole::IconSmall).size;
                icon.draw_centered(ui.painter, Point::new(right - ls.width - t.space(1) - s * 0.5, mid), ui.fade(c.on_surface_variant));
            }
        }
        if let Some(d) = &mut self.detail
            && self.expanded.is_some()
        {
            d.draw(ui);
        }
        if self.unnamed_h > 0.0 {
            let ur = self.unnamed.rect();
            let h = self.unnamed_text.measure_in(ui.painter).height;
            self.unnamed_text.draw(ui.painter, Point::new(self.rect.x + px, ur.center().y - h * 0.5), ui.fade(c.on_surface_variant));
            self.unnamed.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::Duration;

    use nuxut_bluez::{BluetoothState, PowerState, Rfkill, Unavailable};

    use super::*;
    use crate::services::bluetooth::{BtState, Pairing};

    fn dev(id: &str, name: Option<&str>, paired: bool) -> Device {
        Device {
            path: format!("/org/bluez/hci0/{id}"),
            adapter: "/org/bluez/hci0".into(),
            name: name.unwrap_or("12-34").into(),
            alias: name.unwrap_or("12-34").into(),
            remote_name: name.map(str::to_owned),
            paired,
            ..Device::default()
        }
    }

    fn state() -> BluetoothState {
        let mut headset = dev("dev_1", Some("Kulaklık"), true);
        headset.connected = true;
        headset.battery = Some(80);
        headset.icon = Some("audio-headset".into());
        let mut kb = dev("dev_2", Some("Klavye"), false);
        kb.rssi = Some(-50);
        BluetoothState {
            available: true,
            adapters: vec![Adapter { path: "/org/bluez/hci0".into(), name: "pc".into(), powered: true, power_state: PowerState::On, discoverable_timeout: 180, ..Adapter::default() }],
            devices: vec![headset, kb, dev("dev_3", None, false)],
            ..BluetoothState::default()
        }
    }

    fn ready(st: BluetoothState) -> Snapshot {
        Snapshot { state: BtState::Ready(st), ..Snapshot::default() }
    }

    fn rows(c: Content) -> (Vec<RowData>, Option<NearbyInfo>) {
        match c {
            Content::Rows(r, i) => (r, i),
            other => panic!("satır yok: {other:?}"),
        }
    }

    #[test]
    fn paired_rows_show_state_battery_and_icon() {
        crate::i18n::use_turkish_for_tests();
        let (r, info) = rows(content(BtList::Paired, &ready(state()), false));
        assert_eq!(info, None);
        assert_eq!(r.len(), 1);
        assert_eq!((r[0].icon, r[0].active, r[0].battery), (IconName::Headphones, true, Some(80)));
        assert_eq!(r[0].subtitle, t!("settings-bluetooth-connected"));
        let mut st = state();
        st.devices[0].connected = false;
        st.devices[0].blocked = true;
        let (r, _) = rows(content(BtList::Paired, &ready(st), false));
        assert_eq!(r[0].subtitle, format!("{}{SEP}{}", t!("settings-bluetooth-not-connected"), t!("settings-bluetooth-blocked")));
        assert!(matches!(r[0].kind, RowKind::Paired { blocked: true, powered: true, .. }));
    }

    #[test]
    fn nearby_rows_pairing_and_failure() {
        crate::i18n::use_turkish_for_tests();
        let mut s = ready(state());
        let (r, info) = rows(content(BtList::Nearby, &s, false));
        assert_eq!(r.iter().map(|r| r.title.as_str()).collect::<Vec<_>>(), ["Klavye"]);
        assert_eq!(info, Some(NearbyInfo { scanning: false, hidden: 1, unnamed_shown: 0 }));
        assert_eq!(r[0].subtitle, t!("settings-bluetooth-signal-strong"));
        s.pairing = Some(Pairing::new("/org/bluez/hci0/dev_2"));
        let (r, _) = rows(content(BtList::Nearby, &s, true));
        let kb = r.iter().find(|r| r.title == "Klavye").unwrap();
        assert_eq!(kb.subtitle, t!("settings-bluetooth-pairing-wait"), "onay kabukta");
        assert!(matches!(kb.kind, RowKind::Nearby { pairing: true, .. }));
        assert!(r.iter().filter(|r| r.title != "Klavye").all(|r| matches!(r.kind, RowKind::Nearby { pairing: false, can_pair: false })));
        // Kabuğun ajanı varsayılan değil: bekleme metni yerine uyarı (eşleştirme yine sürer).
        s.pairing.as_mut().unwrap().no_agent = true;
        let (r, _) = rows(content(BtList::Nearby, &s, true));
        let kb = r.iter().find(|r| r.title == "Klavye").unwrap();
        assert_eq!(kb.subtitle, t!("settings-bluetooth-pairing-no-agent"));
        assert!(matches!(kb.kind, RowKind::Nearby { pairing: true, .. }), "İptal düğmesi kalır");
        s.pairing = None;
        s.pair_failed = Some(("/org/bluez/hci0/dev_2".into(), BluezError::AuthenticationRejected));
        let (r, _) = rows(content(BtList::Nearby, &s, false));
        assert!(r[0].error);
        assert_eq!(r[0].subtitle, t!("settings-bluetooth-error-auth-rejected"));
    }

    #[test]
    fn empty_and_hidden_states() {
        crate::i18n::use_turkish_for_tests();
        let reading = Content::Empty(t!("settings-bluetooth-reading"));
        assert_eq!(content(BtList::Paired, &Snapshot::default(), false), reading, "okunurken kartlar yerinde");
        assert_eq!(content(BtList::Nearby, &Snapshot::default(), false), reading);
        let missing = Snapshot { state: BtState::Missing(Unavailable::ServiceNotRunning), ..Snapshot::default() };
        for k in [BtList::Paired, BtList::Nearby] {
            assert_eq!(content(k, &missing, false), Content::Hidden, "BlueZ yokken yalnız maskotlu hata");
        }
        let mut st = state();
        st.adapters[0].powered = false;
        st.adapters[0].power_state = PowerState::Off;
        assert_eq!(content(BtList::Nearby, &ready(st.clone()), false), Content::Empty(t!("settings-bluetooth-nearby-off")));
        assert!(matches!(content(BtList::Paired, &ready(st.clone()), false), Content::Rows(..)), "kapalıyken de eşleşmişler görünür");
        st.devices.retain(|d| !d.paired);
        assert_eq!(content(BtList::Paired, &ready(st.clone()), false), Content::NoPaired);
        st.adapters.clear();
        assert_eq!(content(BtList::Paired, &ready(st), false), Content::Hidden, "adaptör yok: maskotlu hata");
    }

    #[test]
    fn unavailable_texts() {
        crate::i18n::use_turkish_for_tests();
        for status in [PageStatus::Failed, PageStatus::Unreachable, PageStatus::NoBluez, PageStatus::NoAdapter] {
            let (title, body) = unavailable_text(status).unwrap();
            assert!(!title.is_empty() && !body.is_empty() && !title.starts_with("settings-"), "{status:?}");
        }
        assert_eq!(unavailable_text(PageStatus::Ready), None);
        assert_eq!(unavailable_text(PageStatus::Reading), None);
    }

    #[test]
    fn error_texts_are_distinct() {
        crate::i18n::use_turkish_for_tests();
        let all = [
            BluezError::CallFailed("x".into()),
            BluezError::Timeout,
            BluezError::NoAdapter,
            BluezError::BluezUnavailable,
            BluezError::UnknownDevice,
            BluezError::AuthenticationRejected,
            BluezError::AuthenticationFailed,
            BluezError::AuthenticationCanceled,
            BluezError::AuthenticationTimeout,
            BluezError::ConnectionAttemptFailed,
            BluezError::AlreadyExists,
            BluezError::InProgress,
            BluezError::NotReady,
            BluezError::InvalidValue,
            BluezError::RfkillBlocked { hard: Some(true) },
            BluezError::RfkillBlocked { hard: Some(false) },
        ];
        let texts: Vec<String> = all.iter().map(error_text).collect();
        assert!(texts.iter().all(|t| !t.is_empty() && !t.starts_with("settings-")), "{texts:?}");
        assert_eq!(texts.iter().collect::<HashSet<_>>().len(), all.len(), "her hata ayrı metin");
        assert!(texts[0].contains('x'), "ayrıntı metne girer");
        assert_eq!(error_text(&BluezError::RfkillBlocked { hard: None }), error_text(&BluezError::RfkillBlocked { hard: Some(false) }), "sysfs okunamadı: yumuşak");
    }

    #[test]
    fn power_texts_and_rfkill() {
        crate::i18n::use_turkish_for_tests();
        let all = [Power::On, Power::Off, Power::Turning, Power::SoftBlocked, Power::HardBlocked].map(power_text);
        assert_eq!(all.iter().collect::<HashSet<_>>().len(), 5);
        let mut st = state();
        st.adapters[0].power_state = PowerState::Blocked;
        st.rfkill = Some(Rfkill { soft: true, hard: false });
        assert_eq!(power(&st).map(power_text), Some(t!("settings-bluetooth-power-soft-blocked")));
        st.rfkill = Some(Rfkill { soft: false, hard: true });
        assert_eq!(power(&st).map(power_text), Some(t!("settings-bluetooth-power-hard-blocked")));
    }

    #[test]
    fn discoverable_and_signal_texts() {
        crate::i18n::use_turkish_for_tests();
        let now = Instant::now();
        let mut a = state().adapters.remove(0);
        assert_eq!(discoverable_text(&a, None, now), t!("settings-bluetooth-discoverable-off"));
        a.discoverable = true;
        let text = discoverable_text(&a, Some(now), now + Duration::from_secs(55));
        assert!(text.contains("2:05"), "{text}");
        assert_eq!(discoverable_text(&a, None, now), t!("settings-bluetooth-discoverable-on"), "açılışı görülmedi");
        let s: Vec<String> = [Some(-40), Some(-70), Some(-90), None].map(signal_text).into();
        assert_eq!(s.iter().collect::<HashSet<_>>().len(), 4);
    }

    /// Maskot yerleşimi: yalnız eşleşmiş cihaz yokken boş maskotu; satır gelince kalkar; yakındakiler
    /// maskotsuz. Hata maskotu yalnız `BtStatus`'ta.
    #[test]
    fn mascot_spots() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(list_spot(BtList::Paired), Some(Spot::BluetoothPairedEmpty));
        assert_eq!(list_spot(BtList::Nearby), None);
        let theme = Theme::default();
        let mut b = BtDeviceList::new(&theme, crate::registry::by_id("bluetooth.paired").unwrap(), BtList::Paired);
        b.apply(&theme, Content::NoPaired);
        assert_eq!(b.mascot(), Some(Spot::BluetoothPairedEmpty));
        assert_eq!(b.mascot().map(Spot::mood), Some(nuxut::ui::Mood::Empty));
        let (r, _) = rows(content(BtList::Paired, &ready(state()), false));
        b.apply(&theme, Content::Rows(r, None));
        assert_eq!(b.mascot(), None);
        assert_eq!(Spot::BluetoothUnavailable.mood(), nuxut::ui::Mood::Error);
    }

    /// Ayrıntı: açılan satırın ayrıntısı veri yenilenince korunur; cihaz giderse kapanır.
    #[test]
    fn detail_survives_refresh() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let mut b = BtDeviceList::new(&theme, crate::registry::by_id("bluetooth.paired").unwrap(), BtList::Paired);
        let (r, _) = rows(content(BtList::Paired, &ready(state()), false));
        b.apply(&theme, Content::Rows(r, None));
        b.toggle_expand(&theme, 0);
        assert_eq!(b.expanded.as_deref(), Some("/org/bluez/hci0/dev_1"));
        assert!(b.targets().contains(&Target::Forget));
        let mut st = state();
        st.devices[0].trusted = true;
        let (r, _) = rows(content(BtList::Paired, &ready(st.clone()), false));
        b.apply(&theme, Content::Rows(r, None));
        assert!(b.detail.as_ref().is_some_and(|d| d.trusted.is_on()), "anahtar yeni duruma akar");
        st.devices.retain(|d| !d.paired);
        b.apply(&theme, Content::NoPaired);
        assert_eq!((b.expanded.clone(), b.detail.is_none()), (None, true));
    }
}
