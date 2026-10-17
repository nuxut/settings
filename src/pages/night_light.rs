//! Ekranlar › Gece ışığı: salt okunur konum satırı (v1 = seçenek b;
//! kararı 2026-10-13, `docs/design/night-light.md`).
//!
//! Yalnızca `schedule = "sun"` iken ve shell `msg night-light status` konumu bildirdiyse görünür
//! (shell yok / eski shell / sözleşme dışı yanıt → satır yok). Başlık konumun kaynağını ve
//! koordinatı, alt satır bugünün gün batımı/doğumunu (ya da kutup günü/gecesini) yazar. Saatler
//! framework'ün saat locale'iyle (`[locale] formats` + `hour_cycle`), koordinat sayı locale'iyle
//! (ondalık ayırıcı) biçimlenir. Elle konum `shell.toml [night_light] latitude/longitude`'dan;
//! Ayarlar'da düzenlenebilir alan v2.
//!
//! Durum sayfa açılınca, pencere odağı dönünce ve `[night_light]` yazılınca (shell uygulasın
//! diye kısa gecikmeyle) yeniden sorulur; yoklama yok.
//!
//! Başlangıç/bitiş saati ([`NightTime`]): framework `TimePicker` (12/24 saat `[locale]
//! hour_cycle`'a uyar, 1 dk adım, serbest giriş); dosyaya her zaman 24 saat `"SS:DD"` yazılır
//! (shell `night_light::model::parse_hhmm`). Dosyadaki değer geçersizse shell'in varsayılanı
//! (20:00 / 07:00) gösterilir; dosya, kullanıcı bir saat seçene dek değişmez (inceleme,
//! 2026-10-13).

use std::time::{Duration, Instant, SystemTime};

use nuxut::i18n::t;
use nuxut::locale::{DateTimeStyle, Locale};
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Label, TextRole, Time, TimeEvent, TimePicker, Ui, text_style};
use nuxut::{Point, Rect, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{row_height_for, row_padding};
use crate::services::night_light::{self as service, Location, LocationSource, NightState, SunToday};
use crate::store::{FileId, Store};

/// Shell'in `shell.toml` değişimini uygulaması için beklenen süre.
const APPLY_SETTLE: Duration = Duration::from_millis(700);

/// Koordinat, 1 ondalık ve yön harfiyle ("41,0° K"); sayı locale'inin ondalık ayırıcısı.
pub fn coord_text(locale: &Locale, value: f64, latitude: bool) -> String {
    let v = crate::numfmt::format_range_in(locale, value.abs(), 1, 1);
    // -0,0 yazılmaz; sıfır kuzey/doğu sayılır.
    let negative = value < 0.0 && v != crate::numfmt::format_range_in(locale, 0.0, 1, 1);
    match (latitude, negative) {
        (true, false) => t!("settings-night-light-north", value = v),
        (true, true) => t!("settings-night-light-south", value = v),
        (false, false) => t!("settings-night-light-east", value = v),
        (false, true) => t!("settings-night-light-west", value = v),
    }
}

/// Konum satırının başlığı.
pub fn location_title(numbers: &Locale, loc: &Location) -> String {
    let coords = |(lat, lon): (f64, f64)| (coord_text(numbers, lat, true), coord_text(numbers, lon, false));
    match (loc.source, loc.coords) {
        (LocationSource::Timezone, Some(c)) => {
            let (lat, lon) = coords(c);
            t!("settings-night-light-location-timezone", lat = lat, lon = lon)
        }
        (LocationSource::Manual, Some(c)) => {
            let (lat, lon) = coords(c);
            t!("settings-night-light-location-manual", lat = lat, lon = lon)
        }
        _ => t!("settings-night-light-location-none"),
    }
}

/// Konum satırının alt metni: bugünün olayları; konum yoksa elle konumun yeri. Olaylardan biri
/// yoksa (yüksek enlemde geçiş günü) alt metin yok.
pub fn location_detail(times: &Locale, loc: &Location, today: Option<&SunToday>) -> Option<String> {
    if loc.coords.is_none() {
        return Some(t!("settings-night-light-location-hint"));
    }
    let hm = |s: i64| times.format_system_time(SystemTime::UNIX_EPOCH + Duration::from_secs(s.max(0) as u64), DateTimeStyle::Time).ok();
    match today? {
        SunToday::PolarDay => Some(t!("settings-night-light-polar-day")),
        SunToday::PolarNight => Some(t!("settings-night-light-polar-night")),
        SunToday::Events { sunrise: Some(rise), sunset: Some(set) } => {
            Some(t!("settings-night-light-today", sunset = hm(*set)?, sunrise = hm(*rise)?))
        }
        SunToday::Events { .. } => None,
    }
}

/// `[night_light]` anahtarlarının değerleri (değişim izi).
fn night_signature(store: &Store) -> String {
    crate::registry::SETTINGS
        .iter()
        .filter(|d| d.file == FileId::Shell && d.path.starts_with("night_light."))
        .map(|d| store.get(d.file, &d.key()).map(|v| v.to_string()).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("|")
}

#[derive(Debug)]
pub struct NightLocation {
    desc: &'static SettingDesc,
    title: Label,
    detail: Label,
    has_detail: bool,
    /// Gösterilecek bir konum var mı (shell bildirdi).
    shown: bool,
    seen: Option<String>,
    rect: Rect,
}

impl NightLocation {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        service::ensure();
        let mut n = Self {
            desc,
            title: Label::new("", text_style(theme, TextRole::Body)),
            detail: Label::new("", text_style(theme, TextRole::Caption)),
            has_detail: false,
            shown: false,
            seen: Some(night_signature(store)),
            rect: Rect::ZERO,
        };
        n.update();
        n
    }

    fn update(&mut self) {
        let state = service::status();
        let Some((loc, today)) = location_of(&state) else {
            self.shown = false;
            return;
        };
        self.shown = true;
        self.title.set_text(&location_title(&crate::numfmt::current(), &loc));
        let detail = location_detail(&nuxut::i18n::time_locale(), &loc, today.as_ref());
        self.has_detail = detail.is_some();
        self.detail.set_text(detail.as_deref().unwrap_or(""));
    }
}

fn location_of(state: &NightState) -> Option<(Location, Option<SunToday>)> {
    let s = state.ready()?;
    Some((s.location?, s.today))
}

impl Block for NightLocation {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store) && self.shown
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let w = Some((width - px * 2.0).max(t.space(20)));
        self.title.set_max_width(w);
        self.detail.set_max_width(w);
        let mut h = self.title.measure(env.text).height;
        if self.has_detail {
            h += t.space(1) + self.detail.measure(env.text).height;
        }
        row_height_for(t, h)
    }

    fn place(&mut self, _env: &mut Env, rect: Rect) {
        self.rect = rect;
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        let sig = night_signature(env.store);
        if self.seen.as_ref() != Some(&sig) {
            // Zamanlama ya da elle konum değişti: shell uygulayınca konum yeniden sorulur.
            service::refresh(APPLY_SETTLE);
            self.seen = Some(sig);
        }
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::NightLight | crate::services::Event::LocaleChanged) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, _now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.detail.set_style(text_style(theme, TextRole::Caption));
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let title_h = self.title.measure_in(ui.painter).height;
        let detail_h = if self.has_detail { self.detail.measure_in(ui.painter).height + t.space(1) } else { 0.0 };
        let ty = self.rect.y + (self.rect.height - title_h - detail_h) * 0.5;
        self.title.draw(ui.painter, Point::new(self.rect.x + px, ty), ui.fade(c.on_surface));
        if self.has_detail {
            self.detail.draw(ui.painter, Point::new(self.rect.x + px, ty + title_h + t.space(1)), ui.fade(c.on_surface_variant));
        }
    }
}

// ---- Başlangıç / bitiş saati ---------------------------------------------------------------------

/// Elle girilen saatte tam dakika seçilebilsin (Tarih ve saat'teki elle saatle aynı).
const MINUTE_STEP: u8 = 1;

/// `S:DD` / `SS:DD` → saat, 24 saat (shell `night_light::model::parse_hhmm` ile aynı kural).
pub fn parse_clock(text: &str) -> Option<Time> {
    let (h, m) = text.split_once(':')?;
    if h.is_empty() || h.len() > 2 || m.len() != 2 || !h.bytes().chain(m.bytes()).all(|b| b.is_ascii_digit()) {
        return None;
    }
    let (h, m) = (h.parse::<u8>().ok()?, m.parse::<u8>().ok()?);
    (h < 24 && m < 60).then(|| Time::new(h, m))
}

/// Dosyaya yazılan biçim: locale'den bağımsız, hep 24 saat `"SS:DD"`.
pub fn stored_clock(t: Time) -> String {
    format!("{:02}:{:02}", t.hour, t.minute)
}

/// Satırın gösterdiği saat: dosyadaki (ya da şema varsayılanındaki) değer; geçersizse shell'in
/// varsayılanı (`NightLightConfig::default`: başlangıç 20:00, bitiş 07:00).
pub fn shown_time(desc: &SettingDesc, store: &Store) -> Time {
    parse_clock(&desc.string(store)).unwrap_or(if desc.path == "night_light.to" { Time::new(7, 0) } else { Time::new(20, 0) })
}

#[derive(Debug)]
pub struct NightTime {
    desc: &'static SettingDesc,
    title: Label,
    subtitle: Label,
    picker: TimePicker,
    rect: Rect,
}

impl NightTime {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        Self {
            desc,
            title: Label::new(desc.display_title(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(desc.display_subtitle(), text_style(theme, TextRole::Caption)),
            picker: TimePicker::with_locale(theme, shown_time(desc, store), &super::locale::time_locale(store)).with_minute_step(MINUTE_STEP),
            rect: Rect::ZERO,
        }
    }

    /// Seçilen saati yazar (aynıysa geçmiş kaydı oluşmaz; ardışık adımlar tek kayıtta birleşir).
    fn write(&self, env: &mut Env, t: Time) {
        env.set(self.desc.display_title(), self.desc.file, self.desc.key(), Some(Value::from(stored_clock(t))));
    }

    fn field_size(&self, theme: &Theme, width: f32) -> nuxut::Size {
        self.picker.preferred_size(theme, (width * 0.3).clamp(theme.space(28), theme.space(44)))
    }

    fn has_subtitle(&self) -> bool {
        !self.subtitle.text().is_empty()
    }
}

impl Block for NightTime {
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
        let (px, _) = row_padding(t);
        let field = self.field_size(t, width);
        let w = Some((width - px * 2.0 - field.width - t.space(3)).max(t.space(20)));
        self.title.set_max_width(w);
        self.subtitle.set_max_width(w);
        let mut h = self.title.measure(env.text).height;
        if self.has_subtitle() {
            h += t.space(1) + self.subtitle.measure(env.text).height;
        }
        row_height_for(t, h.max(field.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let s = self.field_size(t, rect.width);
        self.picker.set_rect(Rect::new(rect.x + rect.width - px - s.width, rect.y + (rect.height - s.height) * 0.5, s.width, s.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let was_open = self.picker.is_open();
        let r = self.picker.pointer(env.theme, e, env.now);
        if let Some(TimeEvent::Changed(t)) = r.event {
            self.write(env, t);
        }
        if !was_open && self.picker.is_open() {
            env.push(Action::Focus);
        }
        // Açık seçici olayın tek sahibi.
        was_open || r.repaint || r.event.is_some()
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let was_open = self.picker.is_open();
        let r = self.picker.key(env.theme, e, env.now);
        if let Some(TimeEvent::Changed(t)) = r.event {
            self.write(env, t);
        }
        was_open || r.repaint || r.event.is_some()
    }

    fn captures_keys(&self) -> bool {
        self.picker.is_open()
    }

    fn overlay_open(&self) -> bool {
        self.picker.is_open()
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        if !focused {
            // Yarım serbest giriş bırakılır (alan saati yeniden gösterir).
            self.picker.cancel_entry();
        }
        self.picker.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.picker.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        // Dış düzenleme / geri al: değer dosyadan; kullanıcı seçerken ya da yazarken dokunulmaz.
        if !self.picker.is_open() && !self.picker.is_entering() {
            let t = shown_time(self.desc, env.store);
            if t != self.picker.time() {
                self.picker.set_time(t, env.now);
            }
        }
        // 12/24 saat ya da biçim değişti: yerinde (aynıysa iş yok).
        self.picker.set_locale(env.theme, &super::locale::time_locale(env.store), env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::LocaleChanged) {
            return self.picker.set_locale(env.theme, &super::locale::time_locale(env.store), env.now);
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        self.picker.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let title_h = self.title.measure_in(ui.painter).height;
        let sub_h = if self.has_subtitle() { self.subtitle.measure_in(ui.painter).height + t.space(1) } else { 0.0 };
        let ty = self.rect.y + (self.rect.height - title_h - sub_h) * 0.5;
        self.title.draw(ui.painter, Point::new(self.rect.x + px, ty), ui.fade(c.on_surface));
        if self.has_subtitle() {
            self.subtitle.draw(ui.painter, Point::new(self.rect.x + px, ty + title_h + t.space(1)), ui.fade(c.on_surface_variant));
        }
        self.picker.draw(ui);
        self.picker.set_bounds(ui.surface_rect());
        self.picker.draw_overlay(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tr() -> Locale {
        Locale::parse("tr_TR").unwrap()
    }

    #[test]
    fn coordinates_use_locale_and_hemisphere_letters() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(coord_text(&tr(), 41.0, true), "41,0° K");
        assert_eq!(coord_text(&tr(), -33.87, true), "33,9° G");
        assert_eq!(coord_text(&tr(), 28.98, false), "29,0° D");
        assert_eq!(coord_text(&tr(), -0.01, false), "0,0° D", "-0,0 yok");
        assert_eq!(coord_text(&tr(), -74.0, false), "74,0° B");
        assert_eq!(coord_text(&Locale::parse("en_US").unwrap(), 41.0, true), "41.0° K");
    }

    #[test]
    fn titles_per_source() {
        crate::i18n::use_turkish_for_tests();
        let tz = Location { source: LocationSource::Timezone, coords: Some((41.0, 29.0)) };
        assert_eq!(location_title(&tr(), &tz), "Konum: saat diliminden (≈ 41,0° K, 29,0° D)");
        let manual = Location { source: LocationSource::Manual, coords: Some((78.2, 15.6)) };
        assert_eq!(location_title(&tr(), &manual), "Konum: elle (78,2° K, 15,6° D)");
        let none = Location { source: LocationSource::None, coords: None };
        assert_eq!(location_title(&tr(), &none), "Konum bulunamadı — 20:00–07:00 kullanılıyor");
    }

    #[test]
    fn detail_lines() {
        crate::i18n::use_turkish_for_tests();
        let loc = Location { source: LocationSource::Timezone, coords: Some((41.0, 29.0)) };
        let times = tr().with_12_hour(Some(false));
        let ev = SunToday::Events { sunrise: Some(1_760_328_840), sunset: Some(1_760_369_520) };
        let d = location_detail(&times, &loc, Some(&ev)).unwrap();
        assert!(d.starts_with("Bugün gün batımı ") && d.contains(" · gün doğumu "), "{d}");
        assert_eq!(location_detail(&times, &loc, Some(&SunToday::PolarDay)).as_deref(), Some("Bugün güneş batmıyor"));
        assert_eq!(location_detail(&times, &loc, Some(&SunToday::PolarNight)).as_deref(), Some("Bugün güneş doğmuyor"));
        assert_eq!(location_detail(&times, &loc, Some(&SunToday::Events { sunrise: None, sunset: Some(1) })), None);
        assert_eq!(location_detail(&times, &loc, None), None);
        let none = Location { source: LocationSource::None, coords: None };
        assert!(location_detail(&times, &none, None).unwrap().contains("shell.toml"));
    }

    #[test]
    fn clock_parse_and_store_format() {
        assert_eq!(parse_clock("7:05"), Some(Time::new(7, 5)));
        assert_eq!(parse_clock("20:00"), Some(Time::new(20, 0)));
        for bad in ["", "24:00", "12:60", "12", "12:5", "1a:00", "12:00:00", "123:00", " 7:00", "8pm"] {
            assert_eq!(parse_clock(bad), None, "{bad}");
        }
        assert_eq!(stored_clock(Time::new(7, 5)), "07:05");
        assert_eq!(stored_clock(Time::new(0, 0)), "00:00");
    }

    /// Saat satırı: geçersiz dosya değeri yerine shell varsayılanı; h12 locale'de de dosyaya
    /// 24 saat "SS:DD" yazılır.
    #[test]
    fn time_rows_write_24_hour_even_in_h12() {
        use crate::block::Action;
        use nuxut::TextSystem;
        use nuxut::ui::input::{Key, KeyState, Modifiers};
        let dir = tempfile::tempdir().unwrap();
        let fw = dir.path().join("f.toml");
        let sh = dir.path().join("s.toml");
        std::fs::write(&fw, "[locale]\nformats = \"tr_TR.UTF-8\"\nhour_cycle = \"h12\"\n").unwrap();
        std::fs::write(&sh, "[night_light]\nenabled = true\nschedule = \"manual\"\nfrom = \"8pm\"\nto = \"6:30\"\n").unwrap();
        let store = Store::open(&fw, &sh, dir.path().join("p.toml"));
        let theme = Theme::default();
        let from = crate::registry::by_id("night_light.from").unwrap();
        let to = crate::registry::by_id("night_light.to").unwrap();
        assert_eq!(shown_time(from, &store), Time::new(20, 0), "geçersiz değer → 20:00");
        assert_eq!(shown_time(to, &store), Time::new(6, 30), "\"6:30\" geçerli (shell de kabul eder)");

        let key = |k| KeyEvent::new(k, KeyState::Pressed, None, Modifiers::default());
        let written = |b: &mut NightTime, k| {
            let mut text = TextSystem::new();
            let mut actions = Vec::new();
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
            assert!(b.key(&mut env, &key(k)));
            match actions.as_slice() {
                [Action::Set { path, value: Some(v), .. }] => {
                    assert_eq!(*path, b.desc.key());
                    v.as_str().unwrap().to_owned()
                }
                a => panic!("tek yazım bekleniyordu: {a:?}"),
            }
        };
        let mut b = NightTime::new(&theme, from, &store);
        assert!(b.picker.is_twelve_hour(), "h12 geçersiz kılması");
        assert_eq!(b.picker.minute_step(), 1);
        assert_eq!(written(&mut b, Key::PageUp), "21:00", "h12'de de 24 saat");
        assert_eq!(written(&mut b, Key::Up), "21:01", "1 dk adım");
        let mut b = NightTime::new(&theme, to, &store);
        assert_eq!(written(&mut b, Key::PageDown), "05:30", "sıfırla doldurulur");
    }

    #[test]
    fn hidden_without_location() {
        assert!(location_of(&NightState::Unavailable).is_none());
        assert!(location_of(&NightState::Pending).is_none());
        let no_loc = NightState::Ready(std::sync::Arc::new(service::NightStatus::default()));
        assert!(location_of(&no_loc).is_none(), "sun dışı ya da eski shell");
    }
}
