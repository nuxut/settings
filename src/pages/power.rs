//! Sistem › Güç sayfası (t117; `docs/design/power-page.md`). Değerler sistem
//! hizmetlerinde ([`crate::services::power`]); bloklar servis anlık görüntüsünden çizer, yazma
//! isteğini servise verir. Durum ancak kaynak bildirince değişir (iyimser değil): hata sonrası
//! seçim/anahtar gerçek duruma döner.
//!
//! - [`PowerProfileRow`]: "Güç profili" — başlık + alt metin, altında üç seçenekli bölümlü düğme
//!   (Güç tasarrufu · Dengeli · Performans). ppd yoksa devre dışı + "gerekli" notu.
//! - [`ChargeLimitRow`]: "Pil ömrünü koru" anahtarı (yalnızca şarj sınırı destekleniyorsa).
//! - [`BatteryRow`]: pil yüzdesi, şarj durumu, kalan süre (salt okunur; pil yoksa gizli).
//! - [`PowerStatus`]: pil, şarj sınırı ve profil hizmeti yoksa maskotlu boş durum (kartsız).

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent};
use nuxut::ui::{Icon, IconName, IconView, Interaction, Label, Segment, SegmentedButton, TextRole, Toggle, Ui, text_style};
use nuxut::{Corners, Point, Rect, Theme};

use super::datetime::RowText;
use super::fiyu::{FiyuEmpty, Spot};
use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{row_height_for, row_padding};
use crate::services::power::{self, ChargeLimit, ChargeState, Degraded, Empty, OpKind, PowerError, PowerProfile, Snapshot};
use crate::store::Store;

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

pub fn profile_label(p: PowerProfile) -> String {
    match p {
        PowerProfile::PowerSaver => t!("settings-power-profile-power-saver"),
        PowerProfile::Balanced => t!("settings-power-profile-balanced"),
        PowerProfile::Performance => t!("settings-power-profile-performance"),
    }
}

fn profile_icon(p: PowerProfile) -> IconName {
    match p {
        PowerProfile::PowerSaver => IconName::PowerSaver,
        PowerProfile::Balanced => IconName::Balanced,
        PowerProfile::Performance => IconName::Performance,
    }
}

pub fn degraded_text(d: Degraded) -> String {
    match d {
        Degraded::LapDetected => t!("settings-power-degraded-lap"),
        Degraded::HighTemperature => t!("settings-power-degraded-temperature"),
        Degraded::Other => t!("settings-power-degraded-other"),
    }
}

/// Yazma hatasının metni. `Unsupported` şarj sınırında metin değil davranıştır (anahtar gizlenir).
pub fn error_text(kind: OpKind, e: &PowerError) -> String {
    match (kind, e) {
        (_, PowerError::Denied) => t!("settings-power-error-denied"),
        (OpKind::ChargeLimit, PowerError::Unavailable) => t!("settings-power-error-upower-missing"),
        (OpKind::Profile, PowerError::Unavailable) => t!("settings-power-profiles-missing"),
        (_, PowerError::Unsupported | PowerError::Failed(_)) => t!("settings-power-error-failed"),
    }
}

/// Şarj sınırı alt metninin türü (metinden bağımsız; testli).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeText {
    /// Pillerin bir kısmında açık.
    Mixed,
    /// "Şarj %{end}'de durur, %{start}'in altına düşünce yeniden başlar".
    Range { start: u8, end: u8 },
    /// "Şarj %{end}'de durur".
    End(u8),
    /// Eşik bilinmiyor: aygıt yazılımı belirler.
    Firmware,
}

pub fn charge_text(limit: ChargeLimit) -> Option<ChargeText> {
    let ChargeLimit::Supported { mixed, start, end, .. } = limit else { return None };
    Some(match (mixed, start, end) {
        (true, ..) => ChargeText::Mixed,
        (false, Some(start), Some(end)) if start < end => ChargeText::Range { start, end },
        (false, _, Some(end)) => ChargeText::End(end),
        // Yalnız başlangıç eşiği: durma noktası aygıt yazılımında.
        (false, _, None) => ChargeText::Firmware,
    })
}

/// Sayının okunuşundaki son sözcüğün değeri (dilden bağımsız): 75 → 5, 80 → 80, 100 → 100,
/// 0 → 0. Sayıya göre ek alan diller (Türkçe "%80'de", "%75'in") eki katalogda bu değere göre
/// seçer (`*-tail` bağımsız değişkenleri); dilbilgisi kodda değil `tr.ftl`'de.
pub fn spoken_tail(n: u8) -> u32 {
    let n = u32::from(n);
    match n {
        0 => 0,
        _ if n % 10 != 0 => n % 10,
        _ if n % 100 != 0 => n % 100,
        _ => 100,
    }
}

pub fn charge_text_string(c: ChargeText) -> String {
    match c {
        ChargeText::Mixed => t!("settings-power-charge-mixed"),
        ChargeText::Range { start, end } => t!(
            "settings-power-charge-range",
            start = u32::from(start),
            end = u32::from(end),
            start_tail = spoken_tail(start),
            end_tail = spoken_tail(end)
        ),
        ChargeText::End(end) => t!("settings-power-charge-end", end = u32::from(end), end_tail = spoken_tail(end)),
        ChargeText::Firmware => t!("settings-power-charge-firmware"),
    }
}

/// Profil satırının görünümü.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileView {
    /// Seçenekler (sırayla): üçü de her zaman çizilir; ppd'nin sunmadığı seçenek tek başına
    /// devre dışı (`available`; framework 86f0125 `SegmentedButton::set_segment_enabled`).
    pub segments: Vec<PowerProfile>,
    /// `segments` ile aynı sırada: seçenek ppd listesinde mi.
    pub available: Vec<bool>,
    pub selected: usize,
    pub enabled: bool,
    pub subtitle: String,
    pub error: bool,
}

pub fn profile_view(s: &Snapshot) -> ProfileView {
    let segments = PowerProfile::ALL.to_vec();
    let index = |p: PowerProfile| segments.iter().position(|x| *x == p);
    let profiles = s.state.as_ref().and_then(|st| st.profiles.as_ref());
    let Some(ps) = profiles.filter(|ps| ps.available.len() >= 2) else {
        // ppd yok (ya da anlamsız liste): üç seçenek devre dışı, "gerekli" notu.
        let missing = profiles.is_none();
        return ProfileView {
            available: vec![true; segments.len()],
            selected: 1,
            enabled: false,
            subtitle: if missing { t!("settings-power-profiles-missing") } else { String::new() },
            error: false,
            segments,
        };
    };
    let available: Vec<bool> = segments.iter().map(|p| ps.available.contains(p)).collect();
    // Bekleme sırasında seçim kullanıcının istediğinde kalır; yanıt gelince gerçek duruma.
    let shown = match s.op.waiting(OpKind::Profile) {
        Some(power::Op::Profile(p)) => Some(p),
        _ => ps.active,
    };
    let selected = shown
        .filter(|p| ps.available.contains(p))
        .or_else(|| ps.available.iter().copied().find(|x| *x == PowerProfile::Balanced))
        .or_else(|| ps.available.first().copied())
        .and_then(index)
        .unwrap_or(1);
    let (subtitle, error) = match s.op.error(OpKind::Profile) {
        Some(e) => (error_text(OpKind::Profile, e), true),
        None => (ps.degraded().map(degraded_text).unwrap_or_default(), false),
    };
    ProfileView { segments, available, selected, enabled: !s.op.busy(), subtitle, error }
}

/// Şarj sınırı satırının görünümü; `None` = satır gizli.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChargeView {
    pub on: bool,
    pub enabled: bool,
    pub subtitle: String,
    pub error: bool,
}

pub fn charge_view(s: &Snapshot) -> Option<ChargeView> {
    if !power::charge_shown(s) {
        return None;
    }
    let limit = s.state.as_ref()?.charge_limit;
    let on = match s.op.waiting(OpKind::ChargeLimit) {
        Some(power::Op::ChargeLimit(on)) => on,
        _ => limit.is_enabled(),
    };
    let (subtitle, error) = match s.op.error(OpKind::ChargeLimit) {
        Some(e) => (error_text(OpKind::ChargeLimit, e), true),
        None => (charge_text(limit).map(charge_text_string).unwrap_or_default(), false),
    };
    Some(ChargeView { on, enabled: !s.op.busy(), subtitle, error })
}

fn is_power(event: &crate::services::Event) -> bool {
    matches!(event, crate::services::Event::Power | crate::services::Event::LocaleChanged)
}

/// Süre: "35 dk", "2 sa", "2 sa 10 dk" (yukarı yuvarlanır; en az 1 dk).
pub fn duration_text(d: Duration) -> String {
    let mins = d.as_secs().div_ceil(60).max(1);
    match (mins / 60, mins % 60) {
        (0, m) => t!("settings-power-duration-minutes", minutes = m),
        (h, 0) => t!("settings-power-duration-hours", hours = h),
        (h, m) => t!("settings-power-duration-hours-minutes", hours = h, minutes = m),
    }
}

/// Pil durumu satırının görünümü (metinden bağımsız kısmı testli).
#[derive(Debug, Clone, PartialEq)]
pub struct BatteryView {
    /// Tam sayı yüzde (0–100).
    pub percent: u8,
    pub icon: IconName,
    pub subtitle: String,
}

pub fn battery_view(s: &Snapshot) -> Option<BatteryView> {
    let st = s.state.as_ref()?;
    let b = st.battery.as_ref()?;
    let percent = b.percentage.round().clamp(0.0, 100.0) as u8;
    let charging = b.state.is_charging();
    let subtitle = match b.state {
        ChargeState::FullyCharged => t!("settings-power-battery-full"),
        _ if charging => match b.time_to_full {
            Some(d) => t!("settings-power-battery-charging-time", time = duration_text(d)),
            None => t!("settings-power-battery-charging"),
        },
        ChargeState::Discharging | ChargeState::Empty => match b.time_to_empty {
            Some(d) => t!("settings-power-battery-discharging-time", time = duration_text(d)),
            None => t!("settings-power-battery-discharging"),
        },
        // Prizde ama şarj olmuyor (eşiğe ulaştı ya da bekliyor).
        ChargeState::PendingDischarge => t!("settings-power-battery-not-charging"),
        _ if !st.on_battery => t!("settings-power-battery-not-charging"),
        _ => String::new(),
    };
    let icon = if charging { IconName::BatteryCharging } else { IconName::battery(percent) };
    Some(BatteryView { percent, icon, subtitle })
}

/// Boş durumun metni ve maskot yeri.
pub fn empty_text(e: Empty) -> (Spot, String, String) {
    match e {
        Empty::NoServices => (Spot::PowerUnavailable, t!("settings-power-empty-unavailable"), t!("settings-power-empty-unavailable-body")),
        Empty::NoBattery => (Spot::PowerEmpty, t!("settings-power-empty-no-battery"), t!("settings-power-empty-no-battery-body")),
    }
}

// ---- Boş durum (kartsız) ------------------------------------------------------------------------

/// Gösterecek bir şey yoksa (pil, şarj sınırı ve profil hizmeti yok) maskotlu boş durum.
#[derive(Debug)]
pub struct PowerStatus {
    desc: &'static SettingDesc,
    fiyu: Option<FiyuEmpty>,
}

impl PowerStatus {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        power::ensure();
        let mut b = Self { desc, fiyu: None };
        b.update(theme);
        b
    }

    fn update(&mut self, theme: &Theme) {
        self.fiyu = power::empty(&power::snapshot()).map(|e| {
            let (spot, title, body) = empty_text(e);
            match self.fiyu.take() {
                Some(mut f) if f.spot() == spot => {
                    f.set_text(theme, &title, Some(&body));
                    f
                }
                _ => FiyuEmpty::new(theme, spot, &title, Some(&body)),
            }
        });
    }
}

impl Block for PowerStatus {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.fiyu.as_ref().map(FiyuEmpty::spot)
    }

    /// Kilit ve boşta kartları varken sayfa boş değildir: maskotlu boş durum gösterilmez
    /// (masaüstünde pil/profil yoksa sayfada yalnız o kartlar kalır).
    fn visible(&self, store: &Store) -> bool {
        self.fiyu.is_some() && !crate::registry::idle_shown(store)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.fiyu.as_mut().map_or(0.0, |f| f.height(env.text, env.theme, width))
    }

    fn place(&mut self, _env: &mut Env, rect: Rect) {
        if let Some(f) = &mut self.fiyu {
            f.set_rect(rect);
        }
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_power(event) {
            self.update(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        if let Some(f) = &mut self.fiyu {
            f.draw(ui);
        }
    }
}

// ---- Pil durumu ---------------------------------------------------------------------------------

/// Pil satırı: düzey ikonu + "%64" + "Pilden çalışıyor · 2 sa 10 dk kaldı" (salt okunur).
#[derive(Debug)]
pub struct BatteryRow {
    desc: &'static SettingDesc,
    icon: IconView,
    text: RowText,
    rect: Rect,
}

impl BatteryRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        power::ensure();
        let mut b = Self { desc, icon: IconView::new(theme, Icon::named(IconName::BatteryUnknown)), text: RowText::new(theme, ""), rect: Rect::ZERO };
        b.update();
        b
    }

    fn update(&mut self) {
        let Some(v) = battery_view(&power::snapshot()) else { return };
        self.icon.set_icon(Icon::named(v.icon));
        self.text.set_title(&t!("settings-power-battery-percent", percent = u32::from(v.percent)));
        self.text.set_subtitle(&v.subtitle, false);
    }

    fn icon_side(theme: &Theme) -> f32 {
        text_style(theme, TextRole::Icon).size
    }
}

impl Block for BatteryRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        power::battery_shown(&power::snapshot())
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let side = Self::icon_side(t);
        let text = self.text.height(env.text, t, (width - px * 2.0 - side - t.space(4)).max(t.space(20)));
        row_height_for(t, text.max(side))
    }

    fn place(&mut self, _env: &mut Env, rect: Rect) {
        self.rect = rect;
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, _env: &mut Env) {
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if is_power(event) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.text.set_theme(theme);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        let side = Self::icon_side(t);
        let mid = self.rect.center().y;
        self.icon.draw_centered(ui.painter, Point::new(self.rect.x + px + side * 0.5, mid), ui.fade(t.colors.on_surface_variant));
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px + side + t.space(4), mid - h * 0.5);
    }
}

// ---- Güç profili --------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PowerProfileRow {
    desc: &'static SettingDesc,
    /// Kartın başlığı zaten "Güç profili": satır başlığı çizilmez (yalnız aramada); bölümlü
    /// düğmenin altında durum notu (kısıt nedeni, ppd yok, hata).
    note: Label,
    /// Not hata renginde mi.
    error: bool,
    seg: SegmentedButton,
    /// `seg`'in seçenekleri (liste değişince düğme yeniden kurulur).
    segments: Vec<PowerProfile>,
    rect: Rect,
}

impl PowerProfileRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        power::ensure();
        let mut b = Self { desc, note: Label::new("", text_style(theme, TextRole::Caption)), error: false, seg: Self::button(theme, &PowerProfile::ALL, 1), segments: PowerProfile::ALL.to_vec(), rect: Rect::ZERO };
        b.update(theme, Instant::now());
        b
    }

    fn button(theme: &Theme, segments: &[PowerProfile], selected: usize) -> SegmentedButton {
        let segs = segments.iter().map(|p| Segment::new(profile_label(*p)).with_icon(Icon::named(profile_icon(*p)))).collect();
        SegmentedButton::new(theme, segs, selected)
    }

    fn update(&mut self, theme: &Theme, now: Instant) {
        let v = profile_view(&power::snapshot());
        if v.segments != self.segments {
            self.seg = Self::button(theme, &v.segments, v.selected);
            self.segments = v.segments;
        } else {
            self.seg.set_selected(v.selected, now);
        }
        self.seg.set_enabled(v.enabled, now);
        for (i, &on) in v.available.iter().enumerate() {
            self.seg.set_segment_enabled(i, on, now);
        }
        self.note.set_text(&v.subtitle);
        self.error = v.error;
    }

    fn choose(&mut self, env: &mut Env, index: usize) {
        if let Some(&p) = self.segments.get(index) {
            power::set_profile(p);
        }
        // İstek reddedildiyse (meşgul, zaten etkin) seçim gerçek duruma döner.
        self.update(env.theme, env.now);
    }

    fn seg_height(&mut self, env: &mut Env) -> f32 {
        self.seg.preferred_size(env.text, env.theme).height
    }
}

impl Block for PowerProfileRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        power::profiles_shown(&power::snapshot())
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.note.set_max_width(Some((width - px * 2.0).max(t.space(20))));
        let note = if self.note.text().is_empty() { 0.0 } else { t.space(2) + self.note.measure(env.text).height };
        let seg = self.seg_height(env);
        seg + note + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let seg = self.seg_height(env);
        let theme = env.theme;
        self.seg.set_rect(theme, Rect::new(rect.x + px, rect.y + py, rect.width - px * 2.0, seg));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.seg.pointer(e, env.now);
        if let Some(i) = r.event {
            self.choose(env, i);
            return true;
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.seg.key(e, env.now);
        if let Some(i) = r.event {
            self.choose(env, i);
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.seg.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.seg.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme, env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_power(event) {
            // Dil değişince seçenek adları da yenilenir.
            if matches!(event, crate::services::Event::LocaleChanged) {
                self.segments.clear();
            }
            self.update(env.theme, env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.note.set_style(text_style(theme, TextRole::Caption));
        self.seg.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        self.seg.draw(ui);
        if !self.note.text().is_empty() {
            let c = if self.error { t.colors.error } else { t.colors.on_surface_variant };
            let y = self.seg.rect().bottom() + t.space(2);
            self.note.draw(ui.painter, Point::new(self.rect.x + px, y), ui.fade(c));
        }
    }
}

// ---- Şarj sınırı --------------------------------------------------------------------------------

/// "Pil ömrünü koru": satırın tamamı tıklanabilir anahtar (Otomatik saat satırı gibi).
#[derive(Debug)]
pub struct ChargeLimitRow {
    desc: &'static SettingDesc,
    text: RowText,
    toggle: Toggle,
    hover: Interaction,
    enabled: bool,
    rect: Rect,
}

impl ChargeLimitRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        power::ensure();
        let mut b = Self { desc, text: RowText::new(theme, desc.label()), toggle: Toggle::new(theme, false), hover: Interaction::new(theme), enabled: false, rect: Rect::ZERO };
        b.update(Instant::now());
        b
    }

    fn update(&mut self, now: Instant) {
        let Some(v) = charge_view(&power::snapshot()) else { return };
        self.text.set_subtitle(&v.subtitle, v.error);
        self.toggle.set_on(v.on, now);
        self.toggle.set_enabled(v.enabled, now);
        self.enabled = v.enabled;
    }

    fn flip(&mut self, now: Instant) {
        if !self.enabled {
            return;
        }
        power::set_charge_limit(!self.toggle.is_on());
        self.update(now);
    }
}

impl Block for ChargeLimitRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        power::charge_shown(&power::snapshot())
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - ts.width).max(t.space(20)));
        row_height_for(t, text.max(ts.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        self.toggle.set_rect(Rect::new(rect.right() - px - ts.width, rect.center().y - ts.height * 0.5, ts.width, ts.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        if !self.enabled {
            return false;
        }
        let tap = self.hover.handle(self.rect, e, env.now);
        if tap.clicked {
            self.flip(env.now);
            return true;
        }
        tap.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) {
            self.flip(env.now);
            return true;
        }
        false
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.toggle.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rect.contains(p).then_some(if self.enabled { CursorShape::Pointer } else { CursorShape::Default })
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_power(event) {
            self.update(env.now);
            return true;
        }
        false
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

#[cfg(test)]
mod tests {
    use nuxut::i18n::{Args, Catalog, LanguagePrefs};

    use super::*;
    use crate::services::power::{Op, OpState, PowerState, Profiles};

    fn laptop() -> PowerState {
        PowerState {
            upower: true,
            on_battery: true,
            battery: Some(battery(64.0, ChargeState::Discharging, Some(130 * 60), None)),
            profiles: Some(Profiles { active: Some(PowerProfile::Balanced), available: PowerProfile::ALL.to_vec(), degraded: None }),
            charge_limit: ChargeLimit::Supported { enabled: true, mixed: false, start: Some(75), end: Some(80) },
        }
    }

    fn snap(state: PowerState, op: OpState) -> Snapshot {
        Snapshot { state: Some(state), op }
    }

    fn battery(pct: f64, state: ChargeState, empty: Option<u64>, full: Option<u64>) -> power::Battery {
        power::Battery {
            percentage: pct,
            state,
            time_to_empty: empty.map(Duration::from_secs),
            time_to_full: full.map(Duration::from_secs),
            icon_name: String::new(),
            warning: nuxut_power::WarningLevel::None,
        }
    }

    #[test]
    fn battery_view_texts() {
        crate::i18n::use_turkish_for_tests();
        let v = battery_view(&snap(laptop(), OpState::Idle)).unwrap();
        assert_eq!((v.percent, v.icon), (64, IconName::Battery60));
        assert_eq!(v.subtitle, "Pilden çalışıyor · 2 sa 10 dk kaldı");
        let mut st = laptop();
        st.on_battery = false;
        st.battery = Some(battery(40.4, ChargeState::Charging, None, Some(35 * 60)));
        let v = battery_view(&snap(st.clone(), OpState::Idle)).unwrap();
        assert_eq!((v.percent, v.icon, v.subtitle.as_str()), (40, IconName::BatteryCharging, "Şarj oluyor · 35 dk içinde dolar"));
        st.battery = Some(battery(80.0, ChargeState::PendingDischarge, None, None));
        assert_eq!(battery_view(&snap(st.clone(), OpState::Idle)).unwrap().subtitle, "Prizde, şarj olmuyor", "eşikte duran pil");
        st.battery = Some(battery(100.0, ChargeState::FullyCharged, None, None));
        assert_eq!(battery_view(&snap(st.clone(), OpState::Idle)).unwrap().subtitle, "Tam dolu");
        st.battery = None;
        assert_eq!(battery_view(&snap(st, OpState::Idle)), None);
        assert_eq!(duration_text(Duration::from_secs(3600)), "1 sa");
        assert_eq!(duration_text(Duration::from_secs(10)), "1 dk", "en az 1 dk");
    }

    #[test]
    fn empty_state_spots() {
        crate::i18n::use_turkish_for_tests();
        let (spot, title, body) = empty_text(Empty::NoServices);
        assert_eq!(spot, Spot::PowerUnavailable);
        assert!(!title.is_empty() && !body.is_empty());
        assert_eq!(empty_text(Empty::NoBattery).0, Spot::PowerEmpty);
    }

    fn limit(mixed: bool, start: Option<u8>, end: Option<u8>) -> ChargeLimit {
        ChargeLimit::Supported { enabled: !mixed, mixed, start, end }
    }

    #[test]
    fn charge_subtitle_choice() {
        assert_eq!(charge_text(ChargeLimit::Unsupported), None);
        assert_eq!(charge_text(limit(false, Some(75), Some(80))), Some(ChargeText::Range { start: 75, end: 80 }));
        assert_eq!(charge_text(limit(false, None, Some(80))), Some(ChargeText::End(80)));
        assert_eq!(charge_text(limit(false, Some(60), None)), Some(ChargeText::Firmware), "yalnız başlangıç: durma noktası bilinmiyor");
        assert_eq!(charge_text(limit(false, None, None)), Some(ChargeText::Firmware));
        assert_eq!(charge_text(limit(false, Some(80), Some(80))), Some(ChargeText::End(80)), "anlamsız aralık: yalnız bitiş");
        assert_eq!(charge_text(limit(true, Some(75), Some(80))), Some(ChargeText::Mixed), "karışık her şeyden önce");
        assert_eq!(charge_text(limit(true, None, None)), Some(ChargeText::Mixed));
    }

    #[test]
    fn spoken_tail_values() {
        // Türkçe ek kuralı: n%10 ≠ 0 → n%10; değilse n%100 ≠ 0 → n%100; değilse 100 (0 → 0).
        for (n, tail) in [(75, 5), (80, 80), (100, 100), (0, 0), (40, 40), (4, 4)] {
            assert_eq!(spoken_tail(n), tail, "{n}");
        }
        let got: Vec<_> = [0, 3, 10, 40, 75, 80, 95, 100].into_iter().map(spoken_tail).collect();
        assert_eq!(got, [0, 3, 10, 40, 5, 80, 5, 100]);
    }

    /// Türkçe kaynak metinler (v1, 2026-10-13): "Şarj %N'de durur"; ek sayıya göre
    /// (`tr.ftl` seçimi, kodda dilbilgisi yok).
    #[test]
    fn turkish_charge_texts() {
        let tr = Catalog::load(&[crate::i18n::source()], &LanguagePrefs::fixed(&["tr"]));
        let end = |n: u8| {
            let mut a = Args::new();
            a.set("end", u32::from(n));
            a.set("end_tail", spoken_tail(n));
            tr.format("settings-power-charge-end", &a).into_owned()
        };
        let got: Vec<_> = [80, 60, 75, 90, 100, 70, 85, 40, 50, 20, 30, 10, 3, 0].into_iter().map(end).collect();
        let want: Vec<_> = ["80'de", "60'ta", "75'te", "90'da", "100'de", "70'te", "85'te", "40'ta", "50'de", "20'de", "30'da", "10'da", "3'te", "0'da"]
            .iter()
            .map(|s| format!("Şarj %{s} durur"))
            .collect();
        assert_eq!(got, want);
        let range = |s: u8, e: u8| {
            let mut a = Args::new();
            a.set("start", u32::from(s));
            a.set("end", u32::from(e));
            a.set("start_tail", spoken_tail(s));
            a.set("end_tail", spoken_tail(e));
            tr.format("settings-power-charge-range", &a).into_owned()
        };
        assert_eq!(range(75, 80), "Şarj %80'de durur, %75'in altına düşünce yeniden başlar");
        let starts: Vec<_> = [60, 70, 20, 40, 90, 100, 30, 2, 6, 9, 50, 3, 1, 0].into_iter().map(|s| range(s, 100)).collect();
        let want: Vec<_> = ["60'ın", "70'in", "20'nin", "40'ın", "90'ın", "100'ün", "30'un", "2'nin", "6'nın", "9'un", "50'nin", "3'ün", "1'in", "0'ın"]
            .iter()
            .map(|s| format!("Şarj %100'de durur, %{s} altına düşünce yeniden başlar"))
            .collect();
        assert_eq!(starts, want);
        // Eki olmayan dil `*_tail`'i yok sayar (geçici İngilizce; gözden geçirilecek).
        let en = Catalog::load(&[crate::i18n::source()], &LanguagePrefs::fixed(&["en"]));
        let mut a = Args::new();
        a.set("start", 75u32);
        a.set("end", 80u32);
        a.set("start_tail", spoken_tail(75));
        a.set("end_tail", spoken_tail(80));
        assert_eq!(en.format("settings-power-charge-range", &a), "Charging stops at 80% and resumes below 75%");
        assert_eq!(tr.text("settings-power-charge-mixed"), "Pillerin bir kısmında açık");
        assert_eq!(tr.text("settings-power-charge-firmware"), "Şarj sınırını aygıt yazılımı belirler");
        assert_eq!(tr.text("settings-power-charge-limit-label"), "Pil ömrünü koru");
        assert_eq!(tr.text("settings-power-profiles-missing"), "Güç profilleri için power-profiles-daemon gerekli");
        assert_eq!(tr.text("settings-power-error-denied"), "İzin verilmedi (yalnızca etkin yerel oturumda değiştirilebilir)");
        assert_eq!(tr.text("settings-power-error-upower-missing"), "UPower çalışmıyor");
    }

    #[test]
    fn errors_to_text_and_behaviour() {
        let s = |op| snap(laptop(), op);
        let v = charge_view(&s(OpState::Failed(OpKind::ChargeLimit, PowerError::Denied))).unwrap();
        assert_eq!((v.subtitle, v.error), (t!("settings-power-error-denied"), true));
        let v = charge_view(&s(OpState::Failed(OpKind::ChargeLimit, PowerError::Unavailable))).unwrap();
        assert_eq!(v.subtitle, t!("settings-power-error-upower-missing"));
        let v = charge_view(&s(OpState::Failed(OpKind::ChargeLimit, PowerError::Failed("x".into())))).unwrap();
        assert_eq!(v.subtitle, t!("settings-power-error-failed"));
        assert_eq!(charge_view(&s(OpState::Failed(OpKind::ChargeLimit, PowerError::Unsupported))), None, "desteklenmiyor → anahtar gizli");
        // Profil hatası şarj satırına yansımaz; profil satırında hata renginde.
        let snap_p = s(OpState::Failed(OpKind::Profile, PowerError::Denied));
        assert!(!charge_view(&snap_p).unwrap().error);
        let p = profile_view(&snap_p);
        assert_eq!((p.subtitle, p.error), (t!("settings-power-error-denied"), true));
        let p = profile_view(&s(OpState::Failed(OpKind::Profile, PowerError::Unavailable)));
        assert_eq!(p.subtitle, t!("settings-power-profiles-missing"));
    }

    #[test]
    fn waiting_shows_request_then_reverts() {
        let mut st = laptop();
        st.charge_limit = limit(false, Some(75), Some(80));
        if let ChargeLimit::Supported { enabled, .. } = &mut st.charge_limit {
            *enabled = false;
        }
        let waiting = charge_view(&snap(st.clone(), OpState::Waiting(Op::ChargeLimit(true)))).unwrap();
        assert!(waiting.on && !waiting.enabled, "beklerken istenen konumda, devre dışı");
        let failed = charge_view(&snap(st.clone(), OpState::Failed(OpKind::ChargeLimit, PowerError::Denied))).unwrap();
        assert!(!failed.on && failed.enabled, "hata sonrası gerçek duruma (kapalı) döner");
        let p = profile_view(&snap(st.clone(), OpState::Waiting(Op::Profile(PowerProfile::Performance))));
        assert_eq!((p.selected, p.enabled), (2, false));
        let p = profile_view(&snap(st, OpState::Failed(OpKind::Profile, PowerError::Denied)));
        assert_eq!((p.selected, p.enabled), (1, true), "hata sonrası etkin profil (Dengeli)");
    }

    #[test]
    fn profile_view_states() {
        let p = profile_view(&snap(laptop(), OpState::Idle));
        assert_eq!(p.segments, PowerProfile::ALL);
        assert!(p.enabled && p.subtitle.is_empty() && !p.error);
        let mut st = laptop();
        st.profiles.as_mut().unwrap().degraded = Some("lap-detected".into());
        assert_eq!(profile_view(&snap(st.clone(), OpState::Idle)).subtitle, t!("settings-power-degraded-lap"));
        st.profiles.as_mut().unwrap().degraded = Some("high-operating-temperature".into());
        assert_eq!(profile_view(&snap(st.clone(), OpState::Idle)).subtitle, t!("settings-power-degraded-temperature"));
        st.profiles.as_mut().unwrap().available = vec![PowerProfile::PowerSaver, PowerProfile::Balanced];
        let p = profile_view(&snap(st.clone(), OpState::Idle));
        assert_eq!((p.segments.len(), p.selected), (3, 1), "üçü de çizilir");
        assert_eq!(p.available, [true, true, false], "ppd'nin sunmadığı seçenek tek başına devre dışı");
        assert!(p.enabled);
        st.profiles.as_mut().unwrap().active = Some(PowerProfile::Performance);
        assert_eq!(profile_view(&snap(st.clone(), OpState::Idle)).selected, 1, "etkin profil listede yoksa Dengeli");
        st.profiles = None;
        let p = profile_view(&snap(st, OpState::Idle));
        assert_eq!(p.segments, PowerProfile::ALL);
        assert!(!p.enabled, "ppd yok → devre dışı");
        assert_eq!(p.subtitle, t!("settings-power-profiles-missing"));
    }

    #[test]
    fn visibility_rules() {
        assert_eq!(charge_view(&Snapshot::default()), None);
        let mut st = laptop();
        st.charge_limit = ChargeLimit::Unsupported;
        assert_eq!(charge_view(&snap(st.clone(), OpState::Idle)), None, "pil yok → yalnız profil kartı");
        assert!(power::profiles_shown(&snap(st.clone(), OpState::Idle)));
        st.profiles = None;
        assert!(power::profiles_shown(&snap(st.clone(), OpState::Idle)), "pil var: profil kartı devre dışı + not");
        st.battery = None;
        assert!(!power::profiles_shown(&snap(st.clone(), OpState::Idle)), "hiçbiri yok → kart yok, boş durum");
        assert_eq!(power::empty(&snap(st, OpState::Idle)), Some(Empty::NoBattery));
    }
}
