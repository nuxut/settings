//! Tarih ve saat sayfası (t119, `docs/design/datetime.md`): systemd-timedated'in otomatik saat,
//! saat dilimi ve elle tarih/saat ayarları. Değerler dosyada değil timedated'de
//! ([`crate::services::timedate`]); bloklar servis anlık görüntüsünden çizer, yazma isteğini
//! servise verir (polkit yetkisi sorulur; bekleme sırasında "Yetki bekleniyor…").
//!
//! - [`TimedateNote`]: kartsız üst not — timedated yok / okunamadı / donanım saati yerel saatte.
//! - [`AutoTime`]: otomatik saat anahtarı; alt metin eşitlenme durumu. `CanNTP=false` → kapalı.
//! - [`TimezoneRow`]: geçerli dilim ("Istanbul, Europe — UTC+03:00") + "Değiştir" → arama
//!   alanlı satır içi seçici (`ListTimezones`; aksan/büyük harf duyarsız `registry::fold`).
//!   Diğer dilimlerin ofseti gösterilmez (framework'te saat dilimi veritabanı yok; bilinen sınır).
//! - [`ManualTime`]: yalnızca otomatik saat kapalıyken tarih + saat seçicisi ve "Uygula".
//! - [`ClockFormatNote`]: bar saati açık strftime kalıbı kullanıyorsa not + "Temizle" (saat
//!   biçimi anahtarlarını siler; saat `[locale] hour_cycle`'a döner).

use std::sync::Arc;
use std::time::{Instant, SystemTime};

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::state_layer_under;
use nuxut::ui::{
    Button, ButtonVariant, Date, DateEvent, DatePicker, Interaction, Label, Notice, NoticeKind, TextInput, TextInputEvent, TextRole, Time, TimeEvent, TimePicker, Toggle,
    Ui, text_style,
};
use nuxut::{Corners, Point, Rect, TextSystem, Theme};

use crate::block::{Action, Block, Env};
use crate::registry::{self, SettingDesc, fold};
use super::notice::{notice_height, place_notice};
use crate::rows::{row_height_for, row_padding};
use crate::services::timedate::{self, ErrorKind, Op, OpState, Snapshot, TimedateState};
use crate::store::{FileId, KeyPath, Store};

/// Seçicide en çok kaç sonuç.
const MAX_RESULTS: usize = 8;

/// Elle saat seçicisinin dakika adımı (ızgara, ↑/↓, tekerlek).
const MINUTE_STEP: u8 = 1;

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// `Europe/Istanbul` → `("Istanbul, Europe")`; `America/Argentina/Buenos_Aires` →
/// "Buenos Aires, America / Argentina"; bölgesiz ad (`UTC`) olduğu gibi. Alt çizgi boşluk olur.
pub fn zone_label(zone: &str) -> String {
    let clean = |s: &str| s.replace('_', " ");
    match zone.rsplit_once('/') {
        Some((region, city)) => format!("{}, {}", clean(city), clean(&region.replace('/', " / "))),
        None => clean(zone),
    }
}

/// UTC ofseti metni: `+03:00`, `-04:30`, `+00:00`.
pub fn offset_text(secs: i64) -> String {
    let sign = if secs < 0 { '-' } else { '+' };
    let m = secs.abs() / 60;
    format!("{sign}{:02}:{:02}", m / 60, m % 60)
}

/// Seçici sonuçları: sorgu (katlanmış) dilim adının ya da görünen adın içinde; boş sorguda
/// geçerli dilimin bölgesindekiler. Geçerli dilim hariç; en çok [`MAX_RESULTS`].
pub fn search_zones(zones: &[String], query: &str, current: &str) -> Vec<String> {
    let q = fold(query.trim());
    let region = current.split_once('/').map(|(r, _)| r);
    let mut exact = Vec::new();
    let mut rest = Vec::new();
    for z in zones.iter().filter(|z| z.as_str() != current) {
        if q.is_empty() {
            if region.is_some_and(|r| z.starts_with(r) && z.as_bytes().get(r.len()) == Some(&b'/')) {
                rest.push(z.clone());
            }
            continue;
        }
        let label = fold(&zone_label(z));
        let raw = fold(&z.replace('_', " "));
        // Şehir adı sorguyla başlıyorsa öne.
        if label.starts_with(&q) {
            exact.push(z.clone());
        } else if label.contains(&q) || raw.contains(&q) {
            rest.push(z.clone());
        }
    }
    exact.extend(rest);
    exact.truncate(MAX_RESULTS);
    exact
}

/// Yazma hatasının metni.
pub fn error_text(kind: ErrorKind) -> String {
    match kind {
        ErrorKind::Missing => t!("settings-datetime-missing-short"),
        // Yetki reddi/iptali hata olarak gösterilmez (servis `Idle` yapar); buraya gelmez.
        ErrorKind::NotAuthorized | ErrorKind::Other => t!("settings-datetime-failed"),
    }
}

/// Bir işlem satırının durum metni: bekleniyor / hata (`true` = hata rengi).
pub fn op_status(op: &OpState, which: Op) -> Option<(String, bool)> {
    if op.waiting(which) {
        return Some((t!("settings-datetime-waiting"), false));
    }
    op.error(which).map(|k| (error_text(k), true))
}

/// Otomatik saat satırının alt metni.
pub fn ntp_subtitle(s: &Snapshot) -> (String, bool) {
    if let Some(x) = op_status(&s.op, Op::Ntp) {
        return x;
    }
    let Some(p) = s.state.props() else { return (String::new(), false) };
    let text = if !p.can_ntp {
        t!("settings-datetime-ntp-unavailable")
    } else if !p.ntp {
        t!("settings-datetime-ntp-off")
    } else if p.ntp_synced {
        t!("settings-datetime-ntp-synced")
    } else {
        t!("settings-datetime-ntp-unsynced")
    };
    (text, false)
}

/// Sayfa üstü not.
pub fn note_text(s: &Snapshot) -> Option<String> {
    match &s.state {
        TimedateState::Missing => Some(t!("settings-datetime-missing")),
        TimedateState::Failed => Some(t!("settings-datetime-read-failed")),
        TimedateState::Ready(p) if p.local_rtc => Some(t!("settings-datetime-local-rtc")),
        _ => None,
    }
}

/// Geçerli dilimin görünen metni (ofset yalnızca geçerli dilim için, sistemin yerel saatinden).
pub fn current_zone_text(zone: &str, offset: Option<i64>) -> String {
    if zone.is_empty() {
        return t!("settings-datetime-zone-unknown");
    }
    let label = zone_label(zone);
    match offset {
        Some(o) => {
            let off = offset_text(o);
            let utc = t!("settings-datetime-utc", offset = off.as_str());
            t!("settings-datetime-zone-with-offset", zone = label.as_str(), offset = utc.as_str())
        }
        None => label,
    }
}

/// Elle seçilen yerel tarih/saat → UNIX µs (`offset` = yerel − UTC, sn).
pub fn local_to_usec(date: Date, time: Time, offset: i64) -> i64 {
    let secs = date.to_days() * 86_400 + i64::from(time.hour) * 3600 + i64::from(time.minute) * 60 - offset;
    secs * 1_000_000
}

/// UNIX sn → yerel tarih ve saat (`offset` = yerel − UTC, sn).
pub fn local_parts(unix: i64, offset: i64) -> (Date, Time) {
    let local = unix + offset;
    let day_secs = local.rem_euclid(86_400);
    (Date::from_unix_seconds(unix, offset), Time::new((day_secs / 3600) as u8, ((day_secs % 3600) / 60) as u8))
}

fn now_unix() -> i64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

/// Başlık + alt metin çifti (satırların ortak metin düzeni).
#[derive(Debug)]
pub(super) struct RowText {
    title: Label,
    subtitle: Label,
    /// Alt metin hata renginde mi.
    error: bool,
}

impl RowText {
    pub(super) fn new(theme: &Theme, title: &str) -> Self {
        Self { title: Label::new(title, text_style(theme, TextRole::Body)), subtitle: Label::new("", text_style(theme, TextRole::Caption)), error: false }
    }

    pub(super) fn set_subtitle(&mut self, text: &str, error: bool) {
        self.subtitle.set_text(text);
        self.error = error;
    }

    pub(super) fn set_title(&mut self, text: &str) {
        self.title.set_text(text);
    }

    pub(super) fn height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        self.title.set_max_width(Some(width));
        self.subtitle.set_max_width(Some(width));
        let sub = if self.subtitle.text().is_empty() { 0.0 } else { self.subtitle.measure(text).height };
        self.title.measure(text).height + if sub > 0.0 { theme.space(1) + sub } else { 0.0 }
    }

    /// Çizimdeki metin bloğu yüksekliği (boş alt metin yer kaplamaz).
    pub(super) fn drawn_height(&mut self, ui: &mut Ui) -> f32 {
        let th = self.title.measure_in(ui.painter).height;
        if self.subtitle.text().is_empty() {
            return th;
        }
        th + ui.theme.space(1) + self.subtitle.measure_in(ui.painter).height
    }

    /// `top` = metin bloğunun üstü.
    pub(super) fn draw(&mut self, ui: &mut Ui, x: f32, top: f32) {
        let t = ui.theme;
        let c = &t.colors;
        let th = self.title.measure_in(ui.painter).height;
        self.title.draw(ui.painter, Point::new(x, top), ui.fade(c.on_surface));
        let sub = if self.error { c.error } else { c.on_surface_variant };
        self.subtitle.draw(ui.painter, Point::new(x, top + th + t.space(1)), ui.fade(sub));
    }

    pub(super) fn set_theme(&mut self, theme: &Theme) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
    }
}

fn is_timedate(event: &crate::services::Event) -> bool {
    matches!(event, crate::services::Event::Timedate | crate::services::Event::LocaleChanged)
}

// ---- Üst not ------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TimedateNote {
    desc: &'static SettingDesc,
    /// Üç durum da (servis yok, okunamadı, donanım saati yerel saatte) uyarı düzeyinde.
    notice: Notice,
}

impl TimedateNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        timedate::ensure();
        let mut n = Self { desc, notice: Notice::new(theme, NoticeKind::Warning, "") };
        n.update();
        n
    }

    fn update(&mut self) {
        self.notice.set_text(&note_text(&timedate::snapshot()).unwrap_or_default());
    }
}

impl Block for TimedateNote {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        note_text(&timedate::snapshot()).is_some()
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
        if is_timedate(event) {
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

// ---- Otomatik saat ------------------------------------------------------------------------------

#[derive(Debug)]
pub struct AutoTime {
    desc: &'static SettingDesc,
    text: RowText,
    toggle: Toggle,
    hover: Interaction,
    rect: Rect,
}

impl AutoTime {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        timedate::ensure();
        let mut b = Self { desc, text: RowText::new(theme, desc.label()), toggle: Toggle::new(theme, false), hover: Interaction::new(theme), rect: Rect::ZERO };
        b.update(Instant::now());
        b
    }

    /// Değiştirilebilir mi: servis hazır, ağ saati hizmeti var, başka yazma sürmüyor.
    fn enabled(s: &Snapshot) -> bool {
        s.state.props().is_some_and(|p| p.can_ntp) && !s.op.busy()
    }

    fn update(&mut self, now: Instant) {
        let s = timedate::snapshot();
        let (text, error) = ntp_subtitle(&s);
        self.text.set_subtitle(&text, error);
        // Bekleme sırasında anahtar kullanıcının istediği konumda kalır.
        if !s.op.waiting(Op::Ntp) {
            self.toggle.set_on(s.state.props().is_some_and(|p| p.ntp), now);
        }
        self.toggle.set_enabled(Self::enabled(&s), now);
    }

    fn flip(&mut self, now: Instant) {
        if !Self::enabled(&timedate::snapshot()) {
            return;
        }
        let on = !self.toggle.is_on();
        self.toggle.set_on(on, now);
        timedate::set_ntp(on);
    }
}

impl Block for AutoTime {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
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
        // Satırın tamamı tıklanabilir (standart anahtar satırı gibi).
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
        self.rect.contains(p).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_timedate(event) {
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

// ---- Saat dilimi --------------------------------------------------------------------------------

#[derive(Debug)]
struct ZoneRow {
    zone: String,
    name: Label,
    code: Label,
    hover: Interaction,
    rect: Rect,
}

#[derive(Debug)]
pub struct TimezoneRow {
    desc: &'static SettingDesc,
    text: RowText,
    change: Button,
    open: bool,
    search: TextInput,
    results: Vec<ZoneRow>,
    /// Klavyeyle seçili sonuç.
    cursor: usize,
    status: Label,
    /// Son kurulan listenin kimliği (liste geldiğinde sonuçlar yeniden kurulur).
    zones: Option<Arc<Vec<String>>>,
    current: String,
    focused: bool,
    focus_visible: bool,
    demo: bool,
    head_h: f32,
    rect: Rect,
}

impl TimezoneRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        timedate::ensure();
        let mut b = Self {
            desc,
            text: RowText::new(theme, desc.label()),
            change: Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-datetime-zone-change")),
            open: false,
            search: TextInput::new(theme).with_placeholder(t!("settings-datetime-zone-search")).with_max_bytes(64),
            results: Vec::new(),
            cursor: 0,
            status: Label::new("", text_style(theme, TextRole::Body)),
            zones: None,
            current: String::new(),
            focused: false,
            focus_visible: false,
            demo: false,
            head_h: 0.0,
            rect: Rect::ZERO,
        };
        b.update(theme, Instant::now());
        b
    }

    fn update(&mut self, theme: &Theme, now: Instant) {
        let s = timedate::snapshot();
        self.current = s.state.props().map(|p| p.timezone.clone()).unwrap_or_default();
        let (text, error) = op_status(&s.op, Op::Timezone).unwrap_or_else(|| {
            if s.state.props().is_some() { (current_zone_text(&self.current, timedate::local_offset(now_unix())), false) } else { (String::new(), false) }
        });
        self.text.set_subtitle(&text, error);
        let enabled = s.state.props().is_some() && !s.op.busy();
        self.change.set_enabled(enabled || self.open, now);
        if !enabled && self.open && s.op.busy() {
            self.open = false;
        }
        if self.zones != s.zones || self.open {
            self.zones = s.zones.clone();
            self.refresh_results(theme, &s);
        }
    }

    fn refresh_results(&mut self, theme: &Theme, s: &Snapshot) {
        if !self.open {
            self.results.clear();
            return;
        }
        let Some(zones) = &s.zones else {
            self.results.clear();
            self.status.set_text(&if s.zones_failed { t!("settings-datetime-zone-list-failed") } else { t!("settings-datetime-zone-loading") });
            return;
        };
        let found = search_zones(zones, self.search.text(), &self.current);
        self.status.set_text(&if found.is_empty() { t!("settings-datetime-zone-none") } else { String::new() });
        self.results = found
            .into_iter()
            .map(|zone| ZoneRow {
                name: Label::new(zone_label(&zone), text_style(theme, TextRole::Body).with_max_lines(1)),
                code: Label::new(zone.as_str(), text_style(theme, TextRole::Caption).with_max_lines(1)),
                zone,
                hover: Interaction::new(theme),
                rect: Rect::ZERO,
            })
            .collect();
        self.cursor = self.cursor.min(self.results.len().saturating_sub(1));
    }

    fn set_open(&mut self, env: &mut Env, open: bool) {
        self.open = open;
        self.change.set_label(&if open { t!("settings-datetime-zone-cancel") } else { t!("settings-datetime-zone-change") });
        self.search.set_text("");
        self.cursor = 0;
        self.search.set_focused(open && self.focused, env.now);
        if open {
            timedate::want_zones();
            env.push(Action::Focus);
        }
        self.refresh_results(env.theme, &timedate::snapshot());
    }

    fn pick(&mut self, env: &mut Env, i: usize) {
        let Some(zone) = self.results.get(i).map(|r| r.zone.clone()) else { return };
        timedate::set_timezone(zone);
        self.set_open(env, false);
    }

    fn row_h(theme: &Theme) -> f32 {
        nuxut::ui::style::row_height(theme)
    }

    fn head_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let bs = self.change.preferred_size(env.text, t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - bs.width).max(t.space(20)));
        row_height_for(t, text.max(bs.height))
    }
}

impl Block for TimezoneRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        if std::mem::take(&mut self.demo) {
            // Canlı test (`expand=datetime.timezone`): seçiciyi temalı kur.
            self.change.set_label(&t!("settings-datetime-zone-cancel"));
            timedate::want_zones();
            self.refresh_results(t, &timedate::snapshot());
        }
        let head = self.head_height(env, width);
        if !self.open {
            return head;
        }
        let field = self.search.preferred_size(t, width - px * 2.0).height + py;
        let list = if self.results.is_empty() {
            self.status.set_max_width(Some(width - px * 2.0));
            self.status.measure(env.text).height + t.space(2)
        } else {
            self.results.len() as f32 * Self::row_h(t)
        };
        head + field + list + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.head_h = self.head_height(env, rect.width);
        let bs = self.change.preferred_size(env.text, t);
        let cy = rect.y + self.head_h * 0.5;
        self.change.set_rect(Rect::new(rect.right() - px - bs.width, cy - bs.height * 0.5, bs.width, bs.height));
        let mut y = rect.y + self.head_h;
        if self.open {
            let w = rect.width - px * 2.0;
            let ih = self.search.preferred_size(t, w).height;
            self.search.set_rect(Rect::new(rect.x + px, y, w, ih));
            y += ih + py;
            for r in &mut self.results {
                r.rect = Rect::new(rect.x, y, rect.width, Self::row_h(t));
                y += r.rect.height;
            }
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let r = self.change.pointer(e, now);
        repaint |= r.repaint;
        if r.event.is_some() {
            let open = !self.open;
            self.set_open(env, open);
            return true;
        }
        if self.open {
            let r = self.search.pointer(env.text, env.theme, e, now);
            repaint |= r.repaint;
            if let PointerKind::Press(MouseButton::Left) = e.kind
                && self.search.rect().contains(e.position)
            {
                env.push(Action::Focus);
            }
            let mut picked = None;
            for (i, row) in self.results.iter_mut().enumerate() {
                let tap = row.hover.handle(row.rect, e, now);
                repaint |= tap.repaint;
                if tap.clicked {
                    picked = Some(i);
                }
            }
            if let Some(i) = picked {
                self.pick(env, i);
                return true;
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released {
            return false;
        }
        if !self.open {
            if matches!(e.key, Key::Enter | Key::Space) && self.change.is_enabled() {
                self.set_open(env, true);
                return true;
            }
            return false;
        }
        match e.key {
            Key::Down => {
                self.cursor = (self.cursor + 1).min(self.results.len().saturating_sub(1));
                return true;
            }
            Key::Up => {
                self.cursor = self.cursor.saturating_sub(1);
                return true;
            }
            _ => {}
        }
        let r = self.search.key(e, env.now);
        match r.event {
            Some(TextInputEvent::Submit) => {
                let i = self.cursor;
                self.pick(env, i);
                true
            }
            Some(TextInputEvent::Cancel) => {
                self.set_open(env, false);
                true
            }
            Some(TextInputEvent::Changed) => {
                self.cursor = 0;
                self.refresh_results(env.theme, &timedate::snapshot());
                true
            }
            _ => r.repaint,
        }
    }

    fn captures_keys(&self) -> bool {
        self.open
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        self.focused = focused;
        self.focus_visible = focused && visible;
        self.change.set_focus_visible(self.focus_visible && !self.open);
        self.search.set_focused(focused && self.open, now);
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        self.open.then_some(&mut self.search)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if let Some(c) = self.change.cursor(p) {
            return Some(c);
        }
        if self.open {
            if let Some(c) = self.search.cursor(p) {
                return Some(c);
            }
            if self.results.iter().any(|r| r.rect.contains(p)) {
                return Some(CursorShape::Pointer);
            }
        }
        None
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme, env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_timedate(event) {
            self.update(env.theme, env.now);
            return true;
        }
        false
    }

    fn demo_expand(&mut self, _now: Instant) {
        self.open = true;
        self.demo = true;
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.change.set_theme(theme, now);
        self.search.set_theme(theme, now);
        self.status.set_style(text_style(theme, TextRole::Body));
        for r in &mut self.results {
            r.name.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
            r.code.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
            r.hover.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.y + (self.head_h - h) * 0.5);
        self.change.draw(ui);
        if !self.open {
            return;
        }
        self.search.draw(ui);
        if self.results.is_empty() {
            let y = self.search.rect().bottom() + row_padding(t).1;
            self.status.draw(ui.painter, Point::new(self.rect.x + px, y), ui.fade(c.on_surface_variant));
        }
        let now = ui.now;
        for (i, r) in self.results.iter_mut().enumerate() {
            let layer = Rect::new(r.rect.x + t.space(1), r.rect.y, r.rect.width - t.space(2), r.rect.height);
            r.hover.draw_layer(ui.painter, layer, Corners::uniform(t.shape.radius_medium * 0.5), t, now);
            if r.hover.is_animating(now) {
                ui.request_frame();
            }
            if i == self.cursor && self.focus_visible {
                ui.painter.rect(layer, &nuxut::RectStyle { fill: ui.fade(state_layer_under(t, c.primary, 0.12, &[], &[c.on_surface, c.on_surface_variant])), radii: Corners::uniform(t.shape.radius_medium * 0.5), border: None });
            }
            let cy = r.rect.center().y;
            let ns = r.name.measure_in(ui.painter);
            r.name.draw(ui.painter, Point::new(r.rect.x + px, cy - ns.height * 0.5), ui.fade(c.on_surface));
            let cs = r.code.measure_in(ui.painter);
            r.code.draw(ui.painter, Point::new(r.rect.x + px + ns.width + t.space(3), cy - cs.height * 0.5), ui.fade(c.on_surface_variant));
        }
    }
}

// ---- Elle tarih ve saat -------------------------------------------------------------------------

#[derive(Debug)]
pub struct ManualTime {
    desc: &'static SettingDesc,
    text: RowText,
    date: DatePicker,
    time: TimePicker,
    apply: Button,
    /// Kullanıcı alanlara dokundu: "şimdi"ye eşitlenmez.
    edited: bool,
    /// Otomatik saat kapalı ve servis hazır: alanlar etkin.
    enabled: bool,
    /// Alanların gösterdiği dakika (düzenlenmemişken dakika dönünce yenilenir).
    minute: i64,
    focus: usize,
    focus_visible: bool,
    head_h: f32,
    rect: Rect,
}

impl ManualTime {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        timedate::ensure();
        Self::build(theme, desc, store)
    }

    /// Servisi başlatmadan kurar (testler de bunu kullanır).
    fn build(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let (d, tm) = Self::now_parts();
        let mut b = Self {
            desc,
            text: RowText::new(theme, desc.label()),
            date: DatePicker::with_locale(theme, d, d, &super::locale::date_locale(store)),
            // Elle ayarda tam dakika seçilebilsin: 1 dk adım (talep); 12/24 saat
            // `[locale] hour_cycle`'a uyar (store'dan, bkz. `locale::time_locale`).
            time: TimePicker::with_locale(theme, tm, &super::locale::time_locale(store)).with_minute_step(MINUTE_STEP),
            apply: Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-datetime-manual-apply")),
            edited: false,
            enabled: false,
            minute: now_unix() / 60,
            focus: 0,
            focus_visible: false,
            head_h: 0.0,
            rect: Rect::ZERO,
        };
        b.update(Instant::now());
        b
    }

    fn now_parts() -> (Date, Time) {
        let now = now_unix();
        local_parts(now, timedate::local_offset(now).unwrap_or(0))
    }

    fn update(&mut self, now: Instant) {
        let s = timedate::snapshot();
        self.enabled = s.state.props().is_some_and(|p| !p.ntp) && !s.op.waiting(Op::Ntp);
        let (text, error) = op_status(&s.op, Op::Time).unwrap_or_else(|| match s.state.props() {
            Some(p) if p.ntp => (t!("settings-datetime-manual-auto-on"), false),
            Some(_) => (t!("settings-datetime-manual-hint"), false),
            None => (String::new(), false),
        });
        self.text.set_subtitle(&text, error);
        self.apply.set_enabled(self.enabled && !s.op.busy(), now);
        if !self.edited || !self.enabled {
            self.edited = false;
            self.reset_to_now(now);
        }
    }

    fn reset_to_now(&mut self, now: Instant) {
        let (d, tm) = Self::now_parts();
        self.date.set_date(d);
        self.time.set_time(tm, now);
        self.minute = now_unix() / 60;
    }

    fn submit(&mut self) {
        if !self.enabled || timedate::snapshot().op.busy() {
            return;
        }
        let usec_guess = local_to_usec(self.date.date(), self.time.time(), 0) / 1_000_000;
        // Ofset hedef anın kuralına göre (yaz saati sınırı: bilinen sınır, docs/design/datetime.md).
        let offset = timedate::local_offset(usec_guess).unwrap_or(0);
        timedate::set_time(local_to_usec(self.date.date(), self.time.time(), offset));
        self.edited = false;
    }

    fn pickers_open(&self) -> bool {
        self.date.is_open() || self.time.is_open()
    }

    fn line_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let bs = self.apply.preferred_size(env.text, t);
        self.date.preferred_size(t, width).height.max(bs.height)
    }
}

impl Block for ManualTime {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let text = self.text.height(env.text, t, width - px * 2.0);
        self.head_h = row_height_for(t, text);
        if !self.enabled {
            return self.head_h;
        }
        self.head_h + self.line_height(env, width) + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let h = self.line_height(env, rect.width);
        let bs = self.apply.preferred_size(env.text, t);
        let y = rect.y + self.head_h;
        let gap = t.space(3);
        let field_w = ((rect.width - px * 2.0 - bs.width - gap * 2.0) / 2.0).clamp(t.space(20), t.space(70));
        let mut x = rect.x + px;
        self.date.set_rect(Rect::new(x, y, field_w, h));
        x += field_w + gap;
        self.time.set_rect(Rect::new(x, y, field_w, h));
        x += field_w + gap;
        self.apply.set_rect(Rect::new(x, y + (h - bs.height) * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        if !self.enabled {
            return false;
        }
        let now = env.now;
        let theme = env.theme;
        // Açık açılır seçici olayın tek sahibi.
        if self.date.is_open() {
            if let Some(DateEvent::Changed(_)) = self.date.pointer(theme, e, now).event {
                self.edited = true;
            }
            return true;
        }
        if self.time.is_open() {
            if let Some(TimeEvent::Changed(_)) = self.time.pointer(theme, e, now).event {
                self.edited = true;
            }
            return true;
        }
        let mut repaint = false;
        let r = self.date.pointer(theme, e, now);
        repaint |= r.repaint;
        let r2 = self.time.pointer(theme, e, now);
        repaint |= r2.repaint;
        if self.pickers_open() {
            env.push(Action::Focus);
        }
        let r = self.apply.pointer(e, now);
        repaint |= r.repaint;
        if r.event.is_some() {
            self.submit();
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if !self.enabled {
            return false;
        }
        let (theme, now) = (env.theme, env.now);
        let was_open = self.pickers_open();
        let handled = match self.focus {
            0 => {
                let r = self.date.key(theme, e, now);
                if r.event.is_some() {
                    self.edited = true;
                }
                r.repaint || r.event.is_some()
            }
            1 => {
                let r = self.time.key(theme, e, now);
                if r.event.is_some() {
                    self.edited = true;
                }
                r.repaint || r.event.is_some()
            }
            _ => {
                let r = self.apply.key(e, now);
                if r.event.is_some() {
                    self.submit();
                    return true;
                }
                r.repaint
            }
        };
        was_open || handled
    }

    fn captures_keys(&self) -> bool {
        self.pickers_open()
    }

    fn overlay_open(&self) -> bool {
        self.pickers_open()
    }

    fn focusable(&self) -> bool {
        self.enabled
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        if !focused {
            self.focus = 0;
        }
        // Saat alanı odağı kaybetti: yarım serbest giriş bırakılır (alan saati yeniden gösterir).
        if self.focus != 1 {
            self.time.cancel_entry();
        }
        self.focus_visible = focused && visible;
        self.date.set_focus_visible(self.focus_visible && self.focus == 0);
        self.time.set_focus_visible(self.focus_visible && self.focus == 1);
        self.apply.set_focus_visible(self.focus_visible && self.focus == 2);
    }

    fn focus_step(&mut self, forward: bool, now: Instant) -> bool {
        let next = if forward { self.focus + 1 } else { self.focus.wrapping_sub(1) };
        if next > 2 {
            return false;
        }
        self.focus = next;
        self.set_focus(true, true, now);
        true
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if !self.enabled {
            return None;
        }
        self.date.cursor(p).or_else(|| self.time.cursor(p)).or_else(|| self.apply.cursor(p))
    }

    fn sync(&mut self, env: &mut Env) {
        // Alanlar yerinde güncellenir (biçim ya da hour_cycle değişti; aynıysa iş yok). Seçim,
        // odak ve açık takvim korunur.
        self.date.set_locale(env.theme, &super::locale::date_locale(env.store), env.now);
        self.time.set_locale(env.theme, &super::locale::time_locale(env.store), env.now);
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_timedate(event) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.date.set_theme(theme, now);
        self.time.set_theme(theme, now);
        self.apply.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.y + (self.head_h - h) * 0.5);
        if !self.enabled {
            return;
        }
        // Düzenlenmemiş alanlar dakika dönünce "şimdi"yi izler.
        let minute = now_unix() / 60;
        if !self.edited && !self.pickers_open() && minute != self.minute {
            self.reset_to_now(ui.now);
        }
        let secs = now_unix();
        ui.wake_at(ui.now + std::time::Duration::from_secs((60 - secs.rem_euclid(60)) as u64));
        self.date.draw(ui);
        self.time.draw(ui);
        self.apply.draw(ui);
        let bounds = ui.surface_rect();
        self.date.set_bounds(bounds);
        self.time.set_bounds(bounds);
        self.date.draw_overlay(ui);
        self.time.draw_overlay(ui);
    }
}

// ---- Bar saati özel biçim notu ------------------------------------------------------------------

/// "Temizle"nin yazımları: shell.toml'da duran her saat biçimi anahtarı silinir (yatay ve
/// dikey; `""` olanlar da — silmek ile `""` aynıdır, dosyada artık kalmasın). İkisi birden
/// silinir ki bar sonradan yön değiştirince de saat 12/24 tercihine uysun. Tek geri al kaydı.
pub fn clock_clear_writes(store: &Store) -> Vec<(FileId, KeyPath, Option<toml_edit::Value>)> {
    registry::CLOCK_FORMAT_KEYS
        .iter()
        .map(|p| KeyPath::parse(p))
        .filter(|k| store.get(FileId::Shell, k).is_some())
        .map(|k| (FileId::Shell, k, None))
        .collect()
}

/// Saat biçimi seçiminin altında: bar saati açık bir strftime kalıbı kullanıyor (12/24 seçimi
/// ona etki etmez) + "Temizle" (bkz. [`clock_clear_writes`]). Görünürlük kayıttan
/// (`registry::clock_format_overrides`: bar'ın yönündeki etkili anahtar dolu).
#[derive(Debug)]
pub struct ClockFormatNote {
    desc: &'static SettingDesc,
    /// Bilgi düzeyi (gri, kırmızısız); "Temizle" notun sağında.
    notice: Notice,
    clear: Button,
}

impl ClockFormatNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        Self { desc, notice: Notice::new(theme, NoticeKind::Info, &t!("settings-datetime-clock-custom")), clear: Self::button(theme) }
    }

    /// Notun düğmeden kalan genişliği (düğme + aradaki boşluk).
    fn button_inset(&mut self, text: &mut TextSystem, theme: &Theme) -> f32 {
        self.clear.preferred_size(text, theme).width + theme.space(3)
    }

    fn button(theme: &Theme) -> Button {
        Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-datetime-clock-clear"))
    }

    fn clear(&self, env: &mut Env) {
        let writes = clock_clear_writes(env.store);
        if !writes.is_empty() {
            env.push(Action::Group { label: self.desc.label().to_owned(), writes });
        }
    }
}

impl Block for ClockFormatNote {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let bs = self.clear.preferred_size(env.text, t);
        let inset = self.button_inset(env.text, t);
        let w = (width - px * 2.0 - inset).max(t.space(20));
        self.notice.measure(env.text, t, w).height.max(bs.height) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let (px, _) = row_padding(env.theme);
        let inset = self.button_inset(env.text, env.theme);
        place_notice(&mut self.notice, env.theme, rect, inset);
        let bs = self.clear.preferred_size(env.text, env.theme);
        self.clear.set_rect(Rect::new(rect.right() - px - bs.width, rect.center().y - bs.height * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.clear.pointer(e, env.now);
        if r.event.is_some() {
            self.clear(env);
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.clear.key(e, env.now);
        if r.event.is_some() {
            self.clear(env);
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.clear.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.clear.cursor(p)
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::LocaleChanged) {
            self.notice.set_text(&t!("settings-datetime-clock-custom"));
            self.clear = Self::button(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
        self.clear.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.notice.draw(ui);
        self.clear.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::timedate::Props;

    #[test]
    fn zone_labels_are_human() {
        assert_eq!(zone_label("Europe/Istanbul"), "Istanbul, Europe");
        assert_eq!(zone_label("America/Argentina/Buenos_Aires"), "Buenos Aires, America / Argentina");
        assert_eq!(zone_label("UTC"), "UTC");
        assert_eq!(offset_text(3 * 3600), "+03:00");
        assert_eq!(offset_text(-(4 * 3600 + 1800)), "-04:30");
        assert_eq!(offset_text(0), "+00:00");
        assert_eq!(offset_text(5 * 3600 + 45 * 60), "+05:45");
    }

    fn zones() -> Vec<String> {
        ["America/New_York", "America/Argentina/Buenos_Aires", "Asia/Istanbul", "Europe/Berlin", "Europe/Istanbul", "Europe/Kyiv", "Europe/Zurich", "UTC"].map(String::from).to_vec()
    }

    #[test]
    fn search_is_case_and_accent_insensitive() {
        let z = zones();
        // Türkçe büyük İ ve aksan: "İstanbul" → istanbul.
        let r = search_zones(&z, "İstanbul", "UTC");
        assert_eq!(r, ["Asia/Istanbul", "Europe/Istanbul"]);
        // Bölge adıyla da; alt çizgi boşluk sayılır.
        assert_eq!(search_zones(&z, "buenos air", "UTC"), ["America/Argentina/Buenos_Aires"]);
        assert_eq!(search_zones(&z, "new york", "UTC"), ["America/New_York"]);
        assert!(search_zones(&z, "europe", "UTC").len() == 4);
        // Şehir adıyla başlayan öne.
        let r = search_zones(&z, "zu", "UTC");
        assert_eq!(r.first().map(String::as_str), Some("Europe/Zurich"));
        // Geçerli dilim listede yok; boş sorguda aynı bölgedekiler.
        assert_eq!(search_zones(&z, "", "Europe/Istanbul"), ["Europe/Berlin", "Europe/Kyiv", "Europe/Zurich"]);
        assert!(search_zones(&z, "", "UTC").is_empty());
        assert!(search_zones(&z, "yok-böyle", "UTC").is_empty());
    }

    #[test]
    fn subtitles_and_notes_follow_state() {
        crate::i18n::use_turkish_for_tests();
        let ready = |p: Props| Snapshot { state: TimedateState::Ready(p), ..Default::default() };
        let on = Props { timezone: "Europe/Istanbul".into(), can_ntp: true, ntp: true, ntp_synced: true, ..Default::default() };
        assert_eq!(ntp_subtitle(&ready(on.clone())), ("Saat eşitlendi".to_owned(), false));
        assert_eq!(ntp_subtitle(&ready(Props { ntp_synced: false, ..on.clone() })).0, "Saat henüz eşitlenmedi");
        assert_eq!(ntp_subtitle(&ready(Props { can_ntp: false, ..on.clone() })).0, "Bu sistemde ağ saati hizmeti yok");
        let mut s = ready(on.clone());
        s.op = OpState::Waiting(Op::Ntp);
        assert_eq!(ntp_subtitle(&s), ("Yetki bekleniyor…".to_owned(), false));
        s.op = OpState::Failed(Op::Ntp, ErrorKind::Other);
        assert_eq!(ntp_subtitle(&s), ("Değiştirilemedi".to_owned(), true));
        // Başka işlemin hatası bu satırda görünmez.
        s.op = OpState::Failed(Op::Timezone, ErrorKind::Other);
        assert_eq!(ntp_subtitle(&s).0, "Saat eşitlendi");
        assert_eq!(op_status(&s.op, Op::Timezone).map(|x| x.1), Some(true));
        // Notlar.
        assert!(note_text(&ready(on.clone())).is_none());
        assert!(note_text(&ready(Props { local_rtc: true, ..on })).unwrap().contains("LocalRTC"));
        let missing = Snapshot { state: TimedateState::Missing, ..Default::default() };
        assert!(note_text(&missing).unwrap().contains("systemd-timedated"));
        assert!(note_text(&Snapshot::default()).is_none(), "okunurken not yok");
        // Geçerli dilim metni.
        assert_eq!(current_zone_text("Europe/Istanbul", Some(3 * 3600)), "Istanbul, Europe — UTC+03:00");
        assert_eq!(current_zone_text("Europe/Istanbul", None), "Istanbul, Europe");
        assert_eq!(current_zone_text("", None), "Bilinmiyor");
    }

    /// "Temizle": duran iki saat biçimi anahtarı da tek geri al kaydıyla silinir; olmayan
    /// anahtar yazılmaz. Silindikten sonra not gizlenir (saat 12/24 tercihine döner).
    #[test]
    fn clock_clear_removes_format_keys() {
        let dir = tempfile::tempdir().unwrap();
        let shell = dir.path().join("s.toml");
        std::fs::write(&shell, "[bar.widget.clock]\nformat = \"%a %H:%M\"\nvertical_format = \"%H\\n%M\"\n").unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), &shell, dir.path().join("p.toml"));
        let desc = registry::by_id("datetime.clock_format_note").unwrap();
        assert!(desc.is_visible(&store));

        let theme = Theme::default();
        let mut note = ClockFormatNote::new(&theme, desc);
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let h = note.height(&mut env, 600.0);
        note.place(&mut env, Rect::new(0.0, 0.0, 600.0, h));
        assert!(note.focusable());
        let at = note.clear.rect().center();
        for kind in [PointerKind::Enter, PointerKind::Motion, PointerKind::Press(MouseButton::Left), PointerKind::Release(MouseButton::Left)] {
            note.pointer(&mut env, &PointerEvent::new(at, kind));
        }
        let writes = match actions.as_slice() {
            [Action::Group { label, writes }] => {
                assert_eq!(label, desc.title);
                writes.clone()
            }
            other => panic!("beklenmeyen eylemler: {other:?}"),
        };
        let paths: Vec<_> = writes.iter().map(|(f, k, v)| (*f, k.clone(), v.is_none())).collect();
        assert_eq!(
            paths,
            registry::CLOCK_FORMAT_KEYS.iter().map(|k| (FileId::Shell, KeyPath::parse(k), true)).collect::<Vec<_>>(),
            "iki anahtar da silinir"
        );
        for (f, k, v) in writes {
            store.set(f, &k, v);
        }
        assert!(!desc.is_visible(&store), "temizlenince not gizli");
        assert!(clock_clear_writes(&store).is_empty(), "silinecek anahtar kalmadı");

        // Yalnız biri duruyorsa yalnız o silinir.
        store.set(FileId::Shell, &KeyPath::parse("bar.widget.clock.format"), Some(toml_edit::Value::from("")));
        let w = clock_clear_writes(&store);
        assert_eq!(w.len(), 1);
        assert_eq!(w[0].1, KeyPath::parse("bar.widget.clock.format"));
    }

    /// Elle saat: 1 dk adım, `[locale] hour_cycle` store'dan (kaydedilmeden) ve yerinde
    /// `set_locale`; odak kaybı yarım serbest girişi bırakır.
    #[test]
    fn manual_time_follows_hour_cycle_and_cancels_entry() {
        use nuxut::ui::input::Modifiers;
        let dir = tempfile::tempdir().unwrap();
        let fw = dir.path().join("f.toml");
        std::fs::write(&fw, "[locale]\nformats = \"tr_TR.UTF-8\"\nhour_cycle = \"h12\"\n").unwrap();
        let mut store = Store::open(&fw, dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let desc = registry::by_id("datetime.manual").unwrap();
        let mut b = ManualTime::build(&theme, desc, &store);
        assert_eq!(b.time.minute_step(), 1);
        assert!(b.time.is_twelve_hour(), "h12 geçersiz kılması");

        // Kaydedilmemiş değişim: store'da h23 → aynı picker 24 saate döner (yeniden kurulmaz).
        store.set(FileId::Framework, &KeyPath::parse("locale.hour_cycle"), Some(toml_edit::Value::from("h23")));
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        b.sync(&mut env);
        assert!(!b.time.is_twelve_hour());
        assert_eq!(b.time.minute_step(), 1, "adım korunur");
        assert_eq!(b.date.locale().language(), "tr", "tarih alanı store'daki biçimden");

        // Biçim değişti: tarih alanı yerinde (yeniden kurulmadan) yeni locale'e geçer.
        store.set(FileId::Framework, &KeyPath::parse("locale.formats"), Some(toml_edit::Value::from("en_US.UTF-8")));
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        b.sync(&mut env);
        assert_eq!(b.date.locale().language(), "en");

        // Saat alanında giriş başladı, odak satırdan çıktı: giriş bırakılır.
        b.set_focus(true, true, now);
        assert!(b.focus_step(true, now));
        assert_eq!(b.focus, 1);
        let _ = b.time.key(&theme, &KeyEvent::new(Key::Char('1'), KeyState::Pressed, None, Modifiers::default()), now);
        assert!(b.time.is_entering());
        b.set_focus(false, false, now);
        assert!(!b.time.is_entering(), "odak kaybı girişi bırakır");
        // Satır içinde Tab ile sonraki alana geçiş de bırakır.
        b.set_focus(true, true, now);
        b.focus_step(true, now);
        let _ = b.time.key(&theme, &KeyEvent::new(Key::Char('2'), KeyState::Pressed, None, Modifiers::default()), now);
        assert!(b.time.is_entering());
        assert!(b.focus_step(true, now));
        assert!(!b.time.is_entering());
    }

    #[test]
    fn clock_note_is_info() {
        let theme = Theme::default();
        assert_eq!(ClockFormatNote::new(&theme, registry::by_id("datetime.clock_format_note").unwrap()).notice.kind(), NoticeKind::Info);
    }

    #[test]
    fn local_time_round_trips_through_usec() {
        // 2026-10-04 12:00 UTC; İstanbul +03:00 → yerel 15:00.
        let unix = 1_791_115_200;
        let (d, tm) = local_parts(unix, 3 * 3600);
        assert_eq!((d, tm), (Date::new(2026, 10, 4), Time::new(15, 0)));
        assert_eq!(local_to_usec(d, tm, 3 * 3600), unix * 1_000_000);
        // Gün sınırı: UTC 22:30, +02:00 → ertesi gün 00:30.
        let (d, tm) = local_parts(unix + 10 * 3600 + 1800, 2 * 3600);
        assert_eq!((d, tm), (Date::new(2026, 10, 5), Time::new(0, 30)));
        // Negatif ofset.
        let (d, tm) = local_parts(unix, -5 * 3600);
        assert_eq!((d, tm), (Date::new(2026, 10, 4), Time::new(7, 0)));
        assert_eq!(local_to_usec(d, tm, -5 * 3600), unix * 1_000_000);
    }
}
