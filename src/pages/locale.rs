//! Dil ve biçimler (ADR-0010 §5 adım 3): `framework.toml [locale] language` ve `formats`.
//!
//! - Şema, varsayılan ve doğrulama framework'tedir ([`LocaleSetting`]); burada yerel kopya yok.
//!   `auto` = anahtar yok: "Otomatik" seçilince anahtar silinir (config kısa kalır). Dosyadaki
//!   geçersiz değer (yanlış tür, sözdizimi) "Otomatik" gösterilir; sıfırla düğmesi onu siler.
//! - Yazım yolu öbür satırlarla aynı: [`Env::set`] → `App` (Store, toml_edit, geçmiş/geri al).
//! - Dil seçenekleri framework'ün listesidir (`nuxut::i18n::available_languages`: kayıtlı
//!   kaynakların gömülü dilleri ∪ kullanıcı çeviri dizini); ad dilin kendi dilinde
//!   (`nuxut::i18n::language_label`: "Türkçe", "English"). Yerel geri düşüş yok: liste boşsa
//!   yalnızca Otomatik (+ dosyadaki değer).
//! - Biçim seçenekleri: Otomatik + framework'ün gömülü locale listesi (`nuxut::locale::available`),
//!   menüde `Locale::display_name` ("Türkçe (Türkiye)"), ada göre sıralı. Dosyadaki ad listedeki
//!   kanonik ada çözülür (`tr_TR.UTF-8` → `tr_TR`); tanınmayan ad "Otomatik" + sıfırla.
//! - Sayfa (Bölge ve dil, karar 9): üstte başlıksız kartta seçili (ya da sistemin) biçiminin
//!   canlı önizlemesi ([`LocalePreview`]: saat, uzun tarih, kısa tarih, sayı, yüzde —
//!   `Locale::format_system_time`, `Locale::format_number`, [`crate::numfmt::percent_in`]),
//!   dakika dönünce yenilenir; altında "Dil ve biçimler" kartında iki satır. Satır açıklaması
//!   tek satır: Otomatik'te çözülen değer, yoksa ayarın kısa açıklaması.

use std::str::FromStr;
use std::sync::OnceLock;
use std::time::{Instant, SystemTime};

use nuxut::i18n::{LanguageIdentifier, LanguagePrefs, LocaleEnv, LocaleSetting, t};
use nuxut::locale::{DateStyle, DateTimeStyle, Locale};
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Icon, IconButton, IconButtonVariant, IconName, Label, Select, SelectEvent, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Size, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::registry::{LocaleField, SettingDesc};
use crate::rows::{row_height_for, row_padding, should_stack, side_text_width, stacked_text_width};
use crate::store::Store;

/// Sözdizimi geçersiz bir ad: framework ayrıştırıcısından `auto` + `invalid` almak için
/// (`LocaleSetting`in alanları özel).
const INVALID: &str = "?";

/// Dosyadaki `[locale]` değeri (framework'ün ayrıştırıcısıyla).
pub fn setting(desc: &SettingDesc, store: &Store) -> LocaleSetting {
    match store.get(desc.file, &desc.key()) {
        None => LocaleSetting::AUTO,
        Some(v) => match v.as_str() {
            Some(s) => LocaleSetting::parse(s),
            // Yanlış tür: framework de `auto` + geçersiz sayar.
            None => LocaleSetting::parse(INVALID),
        },
    }
}

/// Seçimin yazılacak değeri: `None` (Otomatik) anahtarı siler.
pub fn stored(choice: Option<&str>) -> Option<Value> {
    choice.map(Value::from)
}

/// Seçilebilir arayüz dilleri (BCP 47 etiketleri): framework listesi, olduğu gibi.
pub fn available_languages() -> Vec<String> {
    language_tags(&nuxut::i18n::available_languages())
}

/// Dil kimlikleri → yazılacak etiketler (framework sırası korunur).
pub fn language_tags(ids: &[LanguageIdentifier]) -> Vec<String> {
    ids.iter().map(ToString::to_string).collect()
}

/// Dil etiketinin kendi dilindeki adı ("Türkçe", "English", "Português (Brasil)");
/// çözülemeyen etiket olduğu gibi.
pub fn language_label(tag: &str) -> String {
    LanguageIdentifier::from_str(tag).map_or_else(|_| tag.to_owned(), |id| nuxut::i18n::language_label(&id))
}

/// Seçilebilir biçim locale'leri (kanonik adlar), menü adına göre sıralı. Liste gömülü ve
/// sabit: bir kez kurulur.
pub fn available_formats() -> &'static [String] {
    static LIST: OnceLock<Vec<String>> = OnceLock::new();
    LIST.get_or_init(|| {
        let mut named: Vec<(String, &str)> =
            nuxut::locale::available().iter().map(|n| (formats_label(n).to_lowercase(), *n)).collect();
        named.sort();
        named.into_iter().map(|(_, n)| n.to_owned()).collect()
    })
}

/// Biçim locale'inin menü adı ("Türkçe (Türkiye)", "English (United States)"); tanınmayan
/// ad olduğu gibi.
pub fn formats_label(name: &str) -> String {
    Locale::parse(name).map_or_else(|| name.to_owned(), |l| l.display_name())
}

/// Dosyadaki değerin seçeneklerdeki karşılığı: dilde değerin kendisi; biçimde kanonik ad
/// (`de_DE.UTF-8` → `de_DE`), tanınmayan ad `None` (Otomatik gösterilir, sıfırla açık).
pub fn current_choice(field: LocaleField, current: &LocaleSetting) -> Option<String> {
    let v = current.value()?;
    match field {
        LocaleField::Language => Some(v.to_owned()),
        LocaleField::Formats => Locale::parse(v).map(|l| l.name().to_owned()),
    }
}

/// Seçenekler: önce Otomatik (`None`), sonra `available`; dosyadaki seçim listede yoksa
/// (ör. kullanıcı çevirisi kaldırıldı) sona eklenir ki görünür kalsın.
pub fn options(choice: Option<&str>, available: &[String]) -> Vec<Option<String>> {
    let mut out: Vec<Option<String>> = Vec::with_capacity(available.len() + 2);
    out.push(None);
    out.extend(available.iter().cloned().map(Some));
    if let Some(v) = choice
        && !available.iter().any(|a| a == v)
    {
        out.push(Some(v.to_owned()));
    }
    out
}

/// Seçeneğin açılır menü etiketi.
fn option_label(field: LocaleField, opt: &Option<String>) -> String {
    match (opt, field) {
        (None, _) => t!("settings-locale-auto"),
        (Some(tag), LocaleField::Language) => language_label(tag),
        (Some(name), LocaleField::Formats) => formats_label(name),
    }
}

/// Otomatikte çözülen arayüz dili (sistem ortamından; framework'ün çözümüyle).
pub fn system_language(env: &LocaleEnv) -> String {
    let prefs = LanguagePrefs::resolve(&LocaleSetting::AUTO, &LocaleSetting::AUTO, env);
    prefs.requested().first().map_or_else(|| "en".to_owned(), ToString::to_string)
}

/// Biçim önizlemesinin locale'i: açık ve tanınan ad o; değilse sistemin (`LC_TIME` sırası).
pub fn effective_formats(current: &LocaleSetting) -> Locale {
    nuxut::i18n::formats_locale(current).unwrap_or_else(Locale::from_env)
}

/// Önizlemedeki örnek sayı.
const SAMPLE_NUMBER: f64 = 1_234_567.89;

/// Örnek sayı, locale'in `LC_NUMERIC` ayırıcılarıyla (framework'ün tek sayı biçimleyicisi).
pub fn format_sample_number(locale: &Locale) -> String {
    locale.format_number(SAMPLE_NUMBER, 2)
}

/// Önizlemedeki örnek yüzde (kesir).
const SAMPLE_PERCENT: f64 = 0.25;

/// Önizlemenin bir anı: her alan seçili locale'le biçimlenmiş (tarih biçimlenemezse boş).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    /// Saat:dakika (12/24 saat locale'den).
    pub time: String,
    /// Haftanın günüyle uzun tarih.
    pub long_date: String,
    /// Kısa tarih (`%x`: gün/ay/yıl sırası ve ayırıcısı).
    pub short_date: String,
    pub number: String,
    pub percent: String,
}

/// `time` anının önizlemesi `locale`'le.
pub fn sample_at(locale: &Locale, time: SystemTime) -> Sample {
    let fmt = |style| locale.format_system_time(time, style).unwrap_or_default();
    Sample {
        time: fmt(DateTimeStyle::Time),
        long_date: fmt(DateTimeStyle::Date(DateStyle::Full)),
        short_date: fmt(DateTimeStyle::Date(DateStyle::Short)),
        number: format_sample_number(locale),
        percent: crate::numfmt::percent_in(locale, SAMPLE_PERCENT, 0),
    }
}

/// Satırın açıklaması (tek satır): Otomatik'te çözülen değer; tanınmayan biçim adında uyarı;
/// açık değerde boş (ayarın kendi kısa açıklaması gösterilir).
pub fn subtitle(field: LocaleField, current: &LocaleSetting, env: &LocaleEnv) -> String {
    match field {
        LocaleField::Language if current.is_auto() => {
            let language = language_label(&system_language(env));
            t!("settings-locale-subtitle-system-language", language = language.as_str())
        }
        LocaleField::Language => String::new(),
        LocaleField::Formats => match current.value() {
            None => {
                let name = effective_formats(current).display_name();
                t!("settings-locale-subtitle-system-formats", formats = name.as_str())
            }
            Some(_) if nuxut::i18n::formats_locale(current).is_none() => {
                let sample = sample_at(&effective_formats(current), SystemTime::now());
                let sample = format!("{} · {}", sample.short_date, sample.number);
                t!("settings-locale-subtitle-unrecognized", sample = sample.as_str())
            }
            Some(_) => String::new(),
        },
    }
}

/// Önizleme başlığı: hangi biçimin gösterildiği.
pub fn preview_caption(current: &LocaleSetting) -> String {
    let locale = effective_formats(current);
    match current.value() {
        Some(_) if nuxut::i18n::formats_locale(current).is_some() => locale.display_name(),
        _ => {
            let name = locale.display_name();
            t!("settings-locale-preview-caption-system", formats = name.as_str())
        }
    }
}

#[derive(Debug)]
pub struct LocaleRow {
    desc: &'static SettingDesc,
    field: LocaleField,
    title: Label,
    /// Ayarın sabit açıklaması + çözülen değer / önizleme.
    subtitle: Label,
    select: Box<Select>,
    options: Vec<Option<String>>,
    reset: IconButton,
    show_reset: bool,
    rect: Rect,
    /// Son kurulan durum (değer, geçersiz mi); değişmedikçe seçenek/önizleme yeniden kurulmaz.
    shown: Option<(String, bool)>,
    /// Son ölçümde yığılmış mı (seçim başlığın altında).
    stack: bool,
}

impl LocaleRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, field: LocaleField, store: &Store) -> Self {
        let mut row = Self {
            desc,
            field,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: Label::new("", text_style(theme, TextRole::Caption)),
            select: Box::new(Select::new(theme, vec![t!("settings-locale-auto")], 0)),
            options: vec![None],
            reset: IconButton::new(theme, Icon::named(IconName::Restart), IconButtonVariant::Standard),
            show_reset: false,
            rect: Rect::ZERO,
            shown: None,
            stack: false,
        };
        row.refresh(theme, store, false);
        row
    }

    /// Dosyadan durumu kurar. `force` (dil/biçim ortamı değişti): değer aynı olsa da
    /// açıklama ve önizleme yenilenir.
    fn refresh(&mut self, theme: &Theme, store: &Store, force: bool) {
        let current = setting(self.desc, store);
        // Dosyada yazılı ama `auto` olmayan ya da geçersiz değer sıfırlanabilir.
        self.show_reset = self.desc.is_set(store) && (current.invalid || !current.is_auto());
        let key = (current.as_str().to_owned(), current.invalid);
        if !force && self.shown.as_ref() == Some(&key) {
            return;
        }
        let langs;
        let available: &[String] = match self.field {
            LocaleField::Language => {
                langs = available_languages();
                &langs
            }
            LocaleField::Formats => available_formats(),
        };
        let choice = current_choice(self.field, &current);
        let options = options(choice.as_deref(), available);
        let selected = options.iter().position(|o| *o == choice).unwrap_or(0);
        if options != self.options || !self.select.is_open() {
            let labels = options.iter().map(|o| option_label(self.field, o)).collect();
            *self.select = Select::new(theme, labels, selected);
            self.options = options;
        } else {
            self.select.set_selected(selected);
        }
        // Tek satır (§5.1): çözülen değer ya da uyarı, yoksa ayarın kısa açıklaması.
        let detail = subtitle(self.field, &current, &LocaleEnv::from_process());
        let text = if detail.is_empty() { self.desc.description().to_owned() } else { detail };
        self.subtitle.set_text(&text);
        self.shown = Some(key);
    }

    fn write(&self, env: &mut Env, i: usize) {
        let Some(choice) = self.options.get(i) else { return };
        env.set(self.desc.label(), self.desc.file, self.desc.key(), stored(choice.as_deref()));
    }

    fn control_size(&self, theme: &Theme, width: f32) -> Size {
        self.select.preferred_size(theme, (width * 0.4).clamp(theme.space(40), theme.space(70)))
    }

    /// `(metin genişliği, yığılmış mı)`: dar ya da kalabalık satırda seçim başlığın altına
    /// iner ([`should_stack`], §4/§5.1). Karar `self.stack`'te (çizim için) saklanır.
    fn text_width(&mut self, env: &mut Env, width: f32) -> (f32, bool) {
        let t = env.theme;
        let ctl = self.control_size(t, width);
        self.stack = should_stack(t, env.text, width, ctl.width, &mut self.title, Some(&mut self.subtitle));
        if self.stack {
            return (stacked_text_width(t, width), true);
        }
        (side_text_width(t, width, ctl.width).max(t.space(20)), false)
    }

    fn text_height(&mut self, env: &mut Env, text_w: f32) -> f32 {
        self.title.set_max_width(Some(text_w));
        self.subtitle.set_max_width(Some(text_w));
        let sub = self.subtitle.measure(env.text).height;
        self.title.measure(env.text).height + if sub > 0.0 { env.theme.space(1) + sub } else { 0.0 }
    }
}

impl Block for LocaleRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let (text_w, stacked) = self.text_width(env, width);
        let ctl = self.control_size(t, width);
        let text = self.text_height(env, text_w);
        if stacked {
            text + env.theme.space(2) + ctl.height + py * 2.0
        } else {
            row_height_for(env.theme, text.max(ctl.height))
        }
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let (text_w, stacked) = self.text_width(env, rect.width);
        let text = self.text_height(env, text_w);
        let ctl = self.control_size(t, rect.width);
        let right = rect.right() - px;
        let rs = self.reset.preferred_size(t);
        let rs = Size::new(rs.width * 0.8, rs.height * 0.8);
        if stacked {
            let ctl_rect = Rect::new(rect.x + px, rect.y + py + text + t.space(2), rect.width - px * 2.0, ctl.height);
            let ty = rect.y + py + self.title.measure(env.text).height * 0.5;
            self.reset.set_rect(Rect::new(right - rs.width, ty - rs.height * 0.5, rs.width, rs.height));
            self.select.set_rect(t, ctl_rect);
        } else {
            let cy = rect.y + rect.height * 0.5;
            let ctl_rect = Rect::new(right - ctl.width, cy - ctl.height * 0.5, ctl.width, ctl.height);
            self.reset.set_rect(Rect::new(ctl_rect.x - t.space(2) - rs.width, cy - rs.height * 0.5, rs.width, rs.height));
            self.select.set_rect(t, ctl_rect);
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        if self.select.is_open() {
            // Açık menü olayın tek sahibi (dışına tıklama yalnızca kapatır).
            let r = self.select.pointer(env.text, env.theme, e, now);
            if let Some(SelectEvent::Changed(i)) = r.event {
                self.write(env, i);
            }
            return true;
        }
        let mut repaint = false;
        if self.show_reset {
            let r = self.reset.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                env.set(self.desc.label(), self.desc.file, self.desc.key(), None);
                return true;
            }
        }
        let r = self.select.pointer(env.text, env.theme, e, now);
        repaint |= r.repaint;
        if self.select.is_open() {
            // Menü açıldı: klavye (oklar, Enter, Escape) satıra gelsin.
            env.push(Action::Focus);
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let was_open = self.select.is_open();
        let r = self.select.key(env.text, env.theme, e, env.now);
        if let Some(SelectEvent::Changed(i)) = r.event {
            self.write(env, i);
            return true;
        }
        // Açıkken her tuş menünündür (Escape kapatır, Tab sayfaya geçmez).
        was_open || r.repaint
    }

    fn captures_keys(&self) -> bool {
        self.select.is_open()
    }

    fn overlay_open(&self) -> bool {
        self.select.is_open()
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.select.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.show_reset
            && let Some(c) = self.reset.cursor(p)
        {
            return Some(c);
        }
        self.select.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.theme, env.store, false);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        self.reset.set_theme(theme, now);
        self.select.set_theme(theme, now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        // Dil/biçim ortamı değişti (`App::locale_changed`): değer aynı olsa da yeniden kur.
        if matches!(event, crate::services::Event::LocaleChanged) {
            self.refresh(env.theme, env.store, true);
            return true;
        }
        false
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let rect = self.rect;
        let title_h = self.title.measure_in(ui.painter).height;
        let sub_h = self.subtitle.measure_in(ui.painter).height;
        let sub_h = if sub_h > 0.0 { sub_h + t.space(1) } else { 0.0 };
        let stacked = self.stack;
        let ty = if stacked { rect.y + py } else { rect.y + (rect.height - title_h - sub_h) * 0.5 };
        self.title.draw(ui.painter, Point::new(rect.x + px, ty), ui.fade(c.on_surface));
        self.subtitle.draw(ui.painter, Point::new(rect.x + px, ty + title_h + t.space(1)), ui.fade(c.on_surface_variant));
        if self.show_reset {
            self.reset.draw(ui);
        }
        self.select.draw(ui);
        // Menü kaplamada: sayfanın ve satırın kırpmasından bağımsız, yüzeye sığdırılır.
        self.select.set_bounds(ui.surface_rect());
        self.select.draw_overlay(ui);
    }
}

/// Bölge ve dil sayfasının hero'su: seçili biçimin canlı önizlemesi (§5.4: değişen şeyin
/// resmi). Üstte hangi biçim olduğu, büyük saat, uzun tarih; altta kısa tarih, sayı ve yüzde
/// örnekleri (etiket + değer sütunları, dar sütunda alt alta sarılır). Biçim ayarı ya da
/// ortam değişince ve dakika dönünce yeniden biçimlenir.
#[derive(Debug)]
pub struct LocalePreview {
    desc: &'static SettingDesc,
    rect: Rect,
    caption: Label,
    time: Label,
    date: Label,
    /// (etiket, değer): kısa tarih, sayı, yüzde.
    facts: [(Label, Label); 3],
    /// Biçimlenen an (dakika) ve biçim; değişince yeniden kurulur.
    shown: Option<(u64, String)>,
    /// Örneklerin yerleşimi (`place`'te): her sütunun sol üstü.
    fact_at: [Point; 3],
    /// `[locale] hour_cycle` geçersiz kılması (`Some(true)` = 12 saat); saat örneği buna uyar.
    twelve: Option<bool>,
}

/// Önizlemenin `formats` ayarı (yazılı değer; geçersizse Otomatik).
fn formats_setting(store: &Store) -> LocaleSetting {
    crate::registry::by_id("locale.formats").map_or(LocaleSetting::AUTO, |d| setting(d, store))
}

/// `[locale] hour_cycle` → 12 saat geçersiz kılması (framework `twelve_hour_override`; geçersiz ya
/// da yazılmamış değer = `auto` = locale verisi).
pub fn hour_cycle_override(store: &Store) -> Option<bool> {
    let cycle = match store.get(crate::store::FileId::Framework, &crate::store::KeyPath::parse("locale.hour_cycle")).and_then(|v| v.as_str()) {
        Some("h12") => nuxut::i18n::HourCycle::H12,
        Some("h23") => nuxut::i18n::HourCycle::H23,
        _ => nuxut::i18n::HourCycle::Auto,
    };
    nuxut::i18n::twelve_hour_override(cycle)
}

/// Ayarlar'daki saat seçicilerin locale'i: framework `time_locale_for` ile aynı kural
/// (`formats` → locale, `auto`/çözülemeyen → ortam; + `hour_cycle` geçersiz kılması), ama
/// değerler süreç geneli kayıttan (`nuxut::i18n::time_locale`) değil store'dan okunur. Kayıt
/// yalnızca dosya diske yazılıp runtime yeniden yükleyince (`locale_changed`) güncellenir;
/// store ise kullanıcının seçimini (ve geri almayı) hemen gösterir — böylece saat seçicisi,
/// Bölge ve dil önizlemesiyle aynı anda değişir.
pub fn time_locale(store: &Store) -> Locale {
    date_locale(store).with_12_hour(hour_cycle_override(store))
}

/// Ayarlar'daki tarih seçicilerin locale'i: [`time_locale`] ile aynı kaynak (store'daki
/// `formats`), `hour_cycle` olmadan (tarih biçimini etkilemez).
pub fn date_locale(store: &Store) -> Locale {
    effective_formats(&formats_setting(store))
}

fn unix_minute(t: SystemTime) -> u64 {
    t.duration_since(SystemTime::UNIX_EPOCH).map_or(0, |d| d.as_secs() / 60)
}

impl LocalePreview {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let fact = |l: &str| (Label::new(l, text_style(theme, TextRole::Caption)), Label::new("", text_style(theme, TextRole::Body).with_max_lines(1)));
        let mut s = Self {
            desc,
            rect: Rect::ZERO,
            caption: Label::new("", text_style(theme, TextRole::Label).with_max_lines(1)),
            time: Label::new("", text_style(theme, TextRole::Title).with_max_lines(1)),
            date: Label::new("", text_style(theme, TextRole::Body)),
            facts: [
                fact(&t!("settings-locale-preview-short-date")),
                fact(&t!("settings-locale-preview-number")),
                fact(&t!("settings-locale-preview-percent")),
            ],
            shown: None,
            fact_at: [Point::ZERO; 3],
            twelve: None,
        };
        s.refresh(store, SystemTime::now(), false);
        s
    }

    /// `now` anıyla metinleri kurar; aynı dakika ve aynı biçimse (ve `force` değilse) dokunmaz.
    /// Metin değiştiyse `true`.
    fn refresh(&mut self, store: &Store, now: SystemTime, force: bool) -> bool {
        let current = formats_setting(store);
        let key = (unix_minute(now), current.as_str().to_owned());
        let twelve = hour_cycle_override(store);
        if !force && self.shown.as_ref() == Some(&key) && self.twelve == twelve {
            return false;
        }
        self.twelve = twelve;
        let locale = effective_formats(&current).with_12_hour(twelve);
        let s = sample_at(&locale, now);
        self.caption.set_text(&preview_caption(&current));
        self.time.set_text(&s.time);
        self.date.set_text(&s.long_date);
        for ((_, v), text) in self.facts.iter_mut().zip([&s.short_date, &s.number, &s.percent]) {
            v.set_text(text);
        }
        self.shown = Some(key);
        true
    }

    /// Örnek sütunlarını `width` içinde soldan sağa dizer (sığmayan alta); toplam yükseklik.
    fn arrange_facts(&mut self, text: &mut nuxut::TextSystem, theme: &Theme, origin: Point, width: f32) -> f32 {
        let gap_x = theme.space(8);
        let gap_y = theme.space(3);
        let (mut x, mut y, mut row_h) = (origin.x, origin.y, 0.0f32);
        for (i, (label, value)) in self.facts.iter_mut().enumerate() {
            value.set_max_width(Some(width));
            let ls = label.measure(text);
            let vs = value.measure(text);
            let w = ls.width.max(vs.width);
            let h = ls.height + theme.space(1) + vs.height;
            if x > origin.x && x + w > origin.x + width {
                x = origin.x;
                y += row_h + gap_y;
                row_h = 0.0;
            }
            self.fact_at[i] = Point::new(x, y);
            x += w + gap_x;
            row_h = row_h.max(h);
        }
        y + row_h - origin.y
    }

    /// Yükseklik; yerleşim `self.rect.x/y`'ye göre.
    fn layout(&mut self, text: &mut nuxut::TextSystem, theme: &Theme, rect: Rect) -> f32 {
        let (px, py) = row_padding(theme);
        let inner = (rect.width - px * 2.0).max(theme.space(20));
        self.caption.set_max_width(Some(inner));
        self.time.set_max_width(Some(inner));
        self.date.set_max_width(Some(inner));
        let mut y = rect.y + py + theme.space(1);
        y += self.caption.measure(text).height + theme.space(2);
        y += self.time.measure(text).height + theme.space(1);
        y += self.date.measure(text).height + theme.space(5);
        y += self.arrange_facts(text, theme, Point::new(rect.x + px, y), inner);
        y + py + theme.space(1) - rect.y
    }
}

impl Block for LocalePreview {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.layout(env.text, env.theme, Rect::new(0.0, 0.0, width, 0.0))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        self.layout(env.text, env.theme, rect);
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.store, SystemTime::now(), false);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        // Biçim ortamı değişti (`App::locale_changed`): ad aynı olsa da yeniden biçimlenir.
        // Saat dilimi değişti (timedated): saat ve tarih yeni dilimde.
        matches!(event, crate::services::Event::LocaleChanged | crate::services::Event::Timedate) && self.refresh(env.store, SystemTime::now(), true)
    }

    fn set_theme(&mut self, theme: &Theme, _now: Instant) {
        self.caption.set_style(text_style(theme, TextRole::Label).with_max_lines(1));
        self.time.set_style(text_style(theme, TextRole::Title).with_max_lines(1));
        self.date.set_style(text_style(theme, TextRole::Body));
        for (l, v) in &mut self.facts {
            l.set_style(text_style(theme, TextRole::Caption));
            v.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let r = self.rect;
        // Dakika döndü: saat ve tarih yenilenir (metin aynı boyda; yerleşim değişmez).
        let now = SystemTime::now();
        if self.shown.as_ref().is_some_and(|(m, _)| *m != unix_minute(now)) {
            let current = self.shown.as_ref().map(|(_, f)| LocaleSetting::parse(f)).unwrap_or(LocaleSetting::AUTO);
            let s = sample_at(&effective_formats(&current).with_12_hour(self.twelve), now);
            self.time.set_text(&s.time);
            self.date.set_text(&s.long_date);
            self.facts[0].1.set_text(&s.short_date);
            if let Some((m, _)) = &mut self.shown {
                *m = unix_minute(now);
            }
        }
        let secs = now.duration_since(SystemTime::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        ui.wake_at(ui.now + std::time::Duration::from_secs(60 - secs % 60));
        let mut y = r.y + py + t.space(1);
        let x = r.x + px;
        self.caption.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface_variant));
        y += self.caption.measure_in(ui.painter).height + t.space(2);
        self.time.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
        y += self.time.measure_in(ui.painter).height + t.space(1);
        self.date.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
        for ((label, value), at) in self.facts.iter_mut().zip(self.fact_at) {
            let lh = label.measure_in(ui.painter).height;
            label.draw(ui.painter, at, ui.fade(c.on_surface_variant));
            value.draw(ui.painter, Point::new(at.x, at.y + lh + t.space(1)), ui.fade(c.on_surface));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::by_id;
    use crate::store::FileId;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    fn store_with(framework: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("f.toml"), framework).unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    fn lang() -> &'static SettingDesc {
        by_id("locale.language").unwrap()
    }

    fn formats() -> &'static SettingDesc {
        by_id("locale.formats").unwrap()
    }

    #[test]
    fn auto_is_absent_key() {
        let (_d, store) = store();
        assert!(setting(lang(), &store).is_auto());
        assert!(!lang().is_set(&store));
        // Otomatik anahtarı siler; açık değer yazılır.
        assert!(stored(None).is_none());
        assert_eq!(stored(Some("de")).and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some("de"));
        // Sahibin varsayılanı framework'ten: "auto".
        assert_eq!(lang().default_value(&store).and_then(Value::as_str), Some("auto"));
        assert_eq!(formats().default_value(&store).and_then(Value::as_str), Some("auto"));
    }

    #[test]
    fn write_then_read_back() {
        let (_d, mut store) = store();
        store.set(FileId::Framework, &lang().key(), stored(Some("en")));
        store.set(FileId::Framework, &formats().key(), stored(Some("tr_TR.UTF-8")));
        assert_eq!(setting(lang(), &store).value(), Some("en"));
        assert_eq!(setting(formats(), &store).value(), Some("tr_TR.UTF-8"));
        let text = store.file(FileId::Framework).doc().to_string();
        assert!(text.contains("[locale]") && text.contains("language = \"en\""), "{text}");
        // Otomatik'e dönüş anahtarı siler.
        store.set(FileId::Framework, &lang().key(), stored(None));
        assert!(!lang().is_set(&store));
        assert!(!store.file(FileId::Framework).doc().to_string().contains("language"));
    }

    #[test]
    fn invalid_value_shows_auto() {
        for bad in ["[locale]\nlanguage = 42\n", "[locale]\nlanguage = \"tr; rm -rf\"\n", "[locale]\nlanguage = \"ğ\"\n"] {
            let (_d, store) = store_with(bad);
            let s = setting(lang(), &store);
            assert!(s.is_auto() && s.invalid, "{bad}");
            let theme = Theme::default();
            let row = LocaleRow::new(&theme, lang(), LocaleField::Language, &store);
            assert_eq!(row.select.selected(), 0, "{bad}: Otomatik seçili");
            assert!(row.show_reset, "{bad}: geçersiz değer sıfırlanabilir");
        }
        // Yazılı "auto" geçerli: sıfırla gerekmez.
        let (_d, store) = store_with("[locale]\nlanguage = \"auto\"\n");
        let row = LocaleRow::new(&Theme::default(), lang(), LocaleField::Language, &store);
        assert!(!row.show_reset);
    }


    #[test]
    fn language_options_come_from_framework() {
        // Framework listesi olduğu gibi (saf hâli; süreç kaydına dokunmadan).
        const SRC: nuxut::i18n::Source = nuxut::i18n::Source { domain: "t", embedded: &[("tr", ""), ("en", ""), ("pt-BR", "")] };
        let avail = language_tags(&nuxut::i18n::available_languages_in(&[SRC], None));
        assert_eq!(avail, ["en", "pt-BR", "tr"]);
        assert_eq!(options(None, &avail), vec![None, Some("en".into()), Some("pt-BR".into()), Some("tr".into())]);
        // Listede olmayan dosya değeri sona eklenir (görünür kalır).
        let o = options(current_choice(LocaleField::Language, &LocaleSetting::parse("de")).as_deref(), &avail);
        assert_eq!(o.last(), Some(&Some("de".to_owned())));
        assert_eq!(o.len(), 5);
        // Boş liste: yerel tr+en geri düşüşü yok.
        assert_eq!(options(None, &[]), vec![None]);
    }

    #[test]
    fn language_names_in_own_language() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(language_label("tr"), "Türkçe");
        assert_eq!(language_label("en"), "English");
        assert_eq!(language_label("de"), "Deutsch");
        assert_eq!(language_label("pt-BR"), "Português (Brasil)");
        assert_eq!(option_label(LocaleField::Language, &None), "Otomatik");
        assert_eq!(option_label(LocaleField::Language, &Some("tr".into())), "Türkçe");
        // Çözülemeyen etiket olduğu gibi.
        assert_eq!(language_label("ğ"), "ğ");
    }

    #[test]
    fn formats_options_are_framework_locales() {
        crate::i18n::use_turkish_for_tests();
        let list = available_formats();
        assert_eq!(list.len(), nuxut::locale::available().len());
        for name in ["tr_TR", "en_US", "de_DE"] {
            assert!(list.iter().any(|n| n == name), "{name}");
        }
        // Menü adı display_name, ada göre sıralı.
        assert_eq!(formats_label("tr_TR"), "Türkçe (Türkiye)");
        assert_eq!(formats_label("en_US"), "English (United States)");
        let labels: Vec<String> = list.iter().map(|n| formats_label(n).to_lowercase()).collect();
        assert!(labels.windows(2).all(|w| w[0] <= w[1]));
        // Dosyadaki ad kanonik ada çözülür; tanınmayan ad Otomatik'e düşer.
        assert_eq!(current_choice(LocaleField::Formats, &LocaleSetting::parse("de_DE.UTF-8")).as_deref(), Some("de_DE"));
        assert_eq!(current_choice(LocaleField::Formats, &LocaleSetting::parse("xx_YY")), None);
        assert_eq!(current_choice(LocaleField::Formats, &LocaleSetting::AUTO), None);
    }

    #[test]
    fn row_selects_current_value() {
        let (_d, store) = store_with("[locale]\nlanguage = \"en\"\nformats = \"de_DE.UTF-8\"\n");
        let theme = Theme::default();
        let row = LocaleRow::new(&theme, lang(), LocaleField::Language, &store);
        assert_eq!(row.options[row.select.selected()].as_deref(), Some("en"));
        assert!(row.show_reset);
        let row = LocaleRow::new(&theme, formats(), LocaleField::Formats, &store);
        assert_eq!(row.options.len(), available_formats().len() + 1, "Otomatik + liste, ek yok");
        assert_eq!(row.options[row.select.selected()].as_deref(), Some("de_DE"));
        assert!(row.show_reset);
    }

    #[test]
    fn unknown_formats_value_shows_auto_with_reset() {
        let (_d, store) = store_with("[locale]\nformats = \"xx_YY\"\n");
        let row = LocaleRow::new(&Theme::default(), formats(), LocaleField::Formats, &store);
        assert_eq!(row.select.selected(), 0);
        assert_eq!(row.options.len(), available_formats().len() + 1);
        assert!(row.show_reset);
    }

    /// 2026-10-04 12:00 UTC (±11 saat diliminde aynı gün).
    fn sample_time() -> SystemTime {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_791_115_200)
    }

    #[test]
    fn preview_formats_date_and_number_per_locale() {
        crate::i18n::use_turkish_for_tests();
        for (name, date, number) in [
            ("tr_TR.UTF-8", "04-10-2026", "1.234.567,89"),
            ("en_US.UTF-8", "10/04/2026", "1,234,567.89"),
            ("de_DE.UTF-8", "04.10.2026", "1.234.567,89"),
        ] {
            let l = Locale::parse(name).unwrap();
            assert_eq!(format_sample_number(&l), number, "{name}");
            let s = sample_at(&l, sample_time());
            assert_eq!(s.short_date, date, "{name}");
            assert_eq!(s.number, number, "{name}");
            assert!(s.long_date.contains("2026") && s.long_date.len() > date.len(), "{name}: {s:?}");
            assert_eq!(s.percent, crate::numfmt::percent_in(&l, SAMPLE_PERCENT, 0), "{name}");
        }
        // Saat 12/24 saat locale'den: en_US'de AM/PM, tr'de değil.
        let en = sample_at(&Locale::parse("en_US").unwrap(), sample_time()).time;
        assert!(en.contains("AM") || en.contains("PM"), "{en}");
        let tr = sample_at(&Locale::parse("tr_TR").unwrap(), sample_time()).time;
        assert!(!tr.contains("AM") && !tr.contains("PM") && tr.contains(':'), "{tr}");
        // Açıklama tek satır: açık ve tanınan biçimde boş (ayarın kendi açıklaması), tanınmayan
        // adda uyarı, Otomatik'te sistem biçiminin adı.
        assert!(subtitle(LocaleField::Formats, &LocaleSetting::parse("tr_TR"), &LocaleEnv::default()).is_empty());
        let s = subtitle(LocaleField::Formats, &LocaleSetting::parse("xx_YY"), &LocaleEnv::default());
        assert!(s.starts_with("Tanınmıyor") && !s.contains('\n'), "{s}");
        let s = subtitle(LocaleField::Formats, &LocaleSetting::AUTO, &LocaleEnv::default());
        assert!(s.starts_with("Sistem biçimi: ") && !s.contains('\n'), "{s}");
        // Önizleme başlığı hangi biçimin gösterildiğini söyler.
        assert_eq!(preview_caption(&LocaleSetting::parse("tr_TR.UTF-8")), "Türkçe (Türkiye)");
        assert!(preview_caption(&LocaleSetting::AUTO).ends_with(" · sistem"));
    }

    #[test]
    fn preview_follows_formats_setting_and_minute() {
        crate::i18n::use_turkish_for_tests();
        let (_d, mut store) = store_with("[locale]\nformats = \"de_DE.UTF-8\"\n");
        let desc = by_id("locale.preview").unwrap();
        let mut p = LocalePreview::new(&Theme::default(), desc, &store);
        assert_eq!(p.caption.text(), "Deutsch (Deutschland)");
        assert!(p.refresh(&store, sample_time(), false), "başka dakika: yeniden biçimlenir");
        assert_eq!(p.facts[0].1.text(), "04.10.2026");
        assert_eq!(p.facts[1].1.text(), "1.234.567,89");
        assert!(!p.refresh(&store, sample_time(), false), "aynı dakika ve biçim: dokunulmaz");
        store.set(FileId::Framework, &formats().key(), stored(Some("en_US")));
        assert!(p.refresh(&store, sample_time(), false), "biçim değişti");
        assert_eq!(p.facts[0].1.text(), "10/04/2026");
        assert_eq!(p.caption.text(), "English (United States)");
    }

    #[test]
    fn time_locale_matches_framework_rule() {
        use nuxut::i18n::{HourCycleSetting, LocaleConfig, time_locale_for};
        for formats in ["auto", "tr_TR.UTF-8", "en_US.UTF-8", "de_DE"] {
            for cycle in ["auto", "h12", "h23"] {
                let (_d, store) = store_with(&format!("[locale]\nformats = \"{formats}\"\nhour_cycle = \"{cycle}\"\n"));
                let cfg = LocaleConfig { formats: LocaleSetting::parse(formats), hour_cycle: HourCycleSetting::parse(cycle), ..LocaleConfig::default() };
                assert_eq!(time_locale(&store), time_locale_for(&cfg), "{formats} / {cycle}");
            }
        }
    }

    #[test]
    fn preview_time_follows_hour_cycle() {
        crate::i18n::use_turkish_for_tests();
        let (_d, mut store) = store_with("[locale]\nformats = \"en_US.UTF-8\"\nhour_cycle = \"h23\"\n");
        assert_eq!(hour_cycle_override(&store), Some(false));
        let mut p = LocalePreview::new(&Theme::default(), by_id("locale.preview").unwrap(), &store);
        p.refresh(&store, sample_time(), false);
        let h23 = p.time.text().to_owned();
        assert!(!h23.contains("AM") && !h23.contains("PM"), "{h23}");
        // Yalnızca hour_cycle değişti: aynı dakika ve biçim olsa da yeniden biçimlenir.
        store.set(FileId::Framework, &crate::store::KeyPath::parse("locale.hour_cycle"), Some(Value::from("h12")));
        assert!(p.refresh(&store, sample_time(), false));
        assert!(p.time.text().contains("PM") || p.time.text().contains("AM"), "{}", p.time.text());
        store.set(FileId::Framework, &crate::store::KeyPath::parse("locale.hour_cycle"), Some(Value::from("bozuk")));
        assert_eq!(hour_cycle_override(&store), None, "geçersiz = auto");
    }

    #[test]
    fn preview_facts_wrap_in_narrow_column() {
        let (_d, store) = store_with("[locale]\nformats = \"tr_TR.UTF-8\"\n");
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let mut p = LocalePreview::new(&theme, by_id("locale.preview").unwrap(), &store);
        let wide = p.layout(&mut text, &theme, Rect::new(0.0, 0.0, theme.space(180), 0.0));
        assert!(p.fact_at.iter().all(|a| (a.y - p.fact_at[0].y).abs() < 0.5), "geniş: tek sıra");
        let narrow = p.layout(&mut text, &theme, Rect::new(0.0, 0.0, theme.space(60), 0.0));
        assert!(p.fact_at[2].y > p.fact_at[0].y, "dar: alta sarılır");
        assert!(narrow > wide);
        let (px, _) = row_padding(&theme);
        for (i, a) in p.fact_at.iter().enumerate() {
            let w = p.facts[i].0.measure(&mut text).width.max(p.facts[i].1.measure(&mut text).width);
            assert!(a.x + w <= theme.space(60) - px + 0.5, "{i}: taşmaz");
        }
    }

    #[test]
    fn auto_language_subtitle_names_system_language() {
        crate::i18n::use_turkish_for_tests();
        let env = LocaleEnv { lang: Some("tr_TR.UTF-8".into()), ..LocaleEnv::default() };
        assert_eq!(system_language(&env), "tr-TR");
        assert_eq!(subtitle(LocaleField::Language, &LocaleSetting::AUTO, &env), "Sistem dili: Türkçe (Türkiye)");
        assert_eq!(system_language(&LocaleEnv { lang: Some("C".into()), ..LocaleEnv::default() }), "en");
        assert!(subtitle(LocaleField::Language, &LocaleSetting::parse("en"), &env).is_empty());
    }
}
