//! Kayıttan üretilen standart ayar satırı: solda başlık + açıklama, sağda kontrol
//! (anahtar, kaydırıcı + değer, seçim hapları, renk karesi, metin alanı). Renk satırı
//! tıklanınca altında renk seçici yayla açılır (palet önerileri her an görünür).

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::motion::Animated;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{HAIRLINE, content_on};
use nuxut::ui::IconName;
use nuxut::ui::{
    Button, ButtonVariant, ColorEvent, ColorPicker, Icon, IconButton, IconButtonVariant, IconView, Interaction, Label,
    MAX_SEGMENTS, MIN_SEGMENTS, Notice, NoticeKind, Orientation, Segment, SegmentedButton, Select, SelectEvent, Slider, TextInput, TextInputEvent, TextRole, Toggle, Ui,
    text_style,
};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, Size, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::new_badge::NewBadge;
use crate::registry::{Choice, ChoiceStyle, Control, SettingDesc};
use crate::services::seen;
use crate::store::Store;

/// Satır iç boşlukları (4'lük ızgara).
pub fn row_padding(theme: &Theme) -> (f32, f32) {
    (theme.space(4), theme.space(3))
}

/// İçerik yüksekliği `content` olan tek satırlık (yan yana) satırın yüksekliği: dikey iç boşluk
/// dahil, en az framework standart satır yüksekliği (`style::row_height`, açılır satır başlığıyla
/// aynı ritim).
pub fn row_height_for(theme: &Theme, content: f32) -> f32 {
    let (_, py) = row_padding(theme);
    (content + py * 2.0).max(nuxut::ui::style::row_height(theme))
}

#[derive(Debug)]
enum Ctl {
    Toggle(Toggle),
    Slider { slider: Slider, value: Label, steps: Option<u32> },
    Segmented(SegmentedButton),
    Chips { buttons: Vec<Button>, selected: Option<usize> },
    Color { color: Animated<Color>, hex: Label, picker: Box<ColorPicker>, open: bool, expand: Animated<f32>, chevron: IconView, swatch: Rect },
    /// Kutulu: framework TextInput büyük (parola göster/gizle, d576359); enum boyutu küçük kalsın.
    Text(Box<TextInput>),
    /// Açılır seçim; menü kaplama katmanında (satırın kırpmasında kalmaz).
    Dropdown(Box<Select>),
}

#[derive(Debug)]
pub struct SettingRow {
    desc: &'static SettingDesc,
    title: Label,
    subtitle: Option<Label>,
    ctl: Ctl,
    /// Denetimin kurulduğu seçenekler/aralık (şema gelince değişirse yeniden kurulur).
    shape: Shape,
    reset: IconButton,
    show_reset: bool,
    previous: PreviousAction,
    hover: Interaction,
    rect: Rect,
    /// Sürükleme başladıysa bırakınca geçmiş kaydı mühürlenir.
    dragging: bool,
    focused: bool,
    /// Son yerleşimde başlık kısmının yüksekliği (renk seçici hariç).
    head_h: f32,
    /// Son ölçümde satır yığılmış mı (kontrol başlığın altında; [`should_stack`]).
    stack: bool,
    /// Etkin değerli satırda kullanıcının Hyprland dosyası değeri geçersiz kılıyorsa uyarı
    /// (`msg input status` → `overrides`); satırın altında tam genişlikte.
    note: Option<Notice>,
    /// Son ölçümde uyarının kapladığı yükseklik (alt boşluk dahil).
    note_h: f32,
    /// "Yeni" hapı (başlığın sağında; görünürlüğü [`crate::services::seen`]).
    badge: NewBadge,
}

impl SettingRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Option<Self> {
        let shape = Shape::of(desc);
        let ctl = build_ctl(theme, desc, store, &shape)?;
        let subtitle = Some(desc.subtitle_in(store)).filter(|s| !s.is_empty()).map(|s| Label::new(s, text_style(theme, TextRole::Caption)));
        Some(Self {
            desc,
            title: Label::new(desc.display_title(), text_style(theme, TextRole::Body)),
            subtitle,
            ctl,
            shape,
            reset: IconButton::new(theme, Icon::named(IconName::Restart), IconButtonVariant::Standard),
            show_reset: desc.is_set(store) && !is_default(desc, store),
            previous: PreviousAction::new(theme, desc),
            hover: Interaction::new(theme),
            rect: Rect::ZERO,
            dragging: false,
            focused: false,
            head_h: 0.0,
            stack: false,
            note: override_text(desc, store).map(|s| override_notice(theme, &s)),
            note_h: 0.0,
            badge: NewBadge::new(theme),
        })
    }
}

/// Denetimin kurulduğu seçenekler ve aralık: şema sonradan gelip bunları değiştirirse
/// ([`SettingDesc::choices`], [`SettingDesc::bounds`]) denetim yeniden kurulur — seçim
/// sırası (indis) ile yazılan değer hep aynı listeden.
#[derive(Debug, Clone, PartialEq)]
struct Shape {
    choices: Vec<Choice>,
    bounds: Option<(f64, f64)>,
}

impl Shape {
    fn of(desc: &SettingDesc) -> Self {
        Self { choices: desc.choices(), bounds: desc.bounds() }
    }
}

fn build_ctl(theme: &Theme, desc: &'static SettingDesc, store: &Store, shape: &Shape) -> Option<Ctl> {
    let ctl = match desc.control {
        Control::Toggle { .. } => Ctl::Toggle(Toggle::new(theme, desc.bool(store))),
        Control::Slider { step, .. } => {
            let (min, max) = shape.bounds.unwrap_or((0.0, 1.0));
            let n = ((max - min) / step).round();
            let steps = (n <= 64.0).then_some(n as u32);
            let mut slider = Slider::new(theme, Orientation::Horizontal, norm(desc, desc.number(store)));
            if let Some(n) = steps {
                slider = slider.with_steps(n);
            }
            Ctl::Slider { slider, value: Label::new(format_number(desc, desc.number(store)), text_style(theme, TextRole::Label)), steps }
        }
        Control::Choice { style, .. } => {
            let options = shape.choices.as_slice();
            let current = desc.string(store);
            let selected = options.iter().position(|(v, _)| *v == current);
            if style == ChoiceStyle::Dropdown {
                Ctl::Dropdown(Box::new(new_select(theme, options, selected)))
            } else if style == ChoiceStyle::Segmented && (MIN_SEGMENTS..=MAX_SEGMENTS).contains(&options.len()) {
                let segments = options.iter().map(|(_, l)| Segment::new(l)).collect();
                Ctl::Segmented(SegmentedButton::new(theme, segments, selected.unwrap_or(0)))
            } else {
                let buttons = options
                    .iter()
                    .enumerate()
                    .map(|(i, (_, label))| Button::new(theme, chip_variant(Some(i) == selected)).with_label(theme, label))
                    .collect();
                Ctl::Chips { buttons, selected }
            }
        }
        Control::Color => {
            let c = parse_color(&desc.string(store));
            let mut picker = ColorPicker::new(theme, c);
            picker.set_recent(theme, &[]);
            Ctl::Color {
                color: Animated::new(c, theme.motion.effects),
                hex: Label::new(c.to_hex(), text_style(theme, TextRole::Mono)),
                picker: Box::new(picker),
                open: false,
                expand: Animated::new(0.0, theme.motion.spatial),
                chevron: IconView::with_role(theme, Icon::named(IconName::ChevronDown), TextRole::IconSmall),
                swatch: Rect::ZERO,
            }
        }
        Control::Text { .. } => {
            Ctl::Text(Box::new(TextInput::new(theme).with_placeholder(desc.placeholder(store)).with_text(&desc.string(store)).with_max_bytes(256)))
        }
        Control::Custom(_) => return None,
    };
    Some(ctl)
}

impl SettingRow {
    fn control_size(&mut self, env: &mut Env, width: f32) -> Size {
        let t = env.theme;
        match &mut self.ctl {
            Ctl::Toggle(tg) => tg.preferred_size(t),
            Ctl::Slider { slider, value, .. } => {
                let len = (width * 0.42).clamp(t.space(30), t.space(70));
                let s = slider.preferred_size(t, len);
                let v = value.measure(env.text);
                Size::new(s.width + t.space(3) + v.width.max(t.space(14)), s.height.max(v.height))
            }
            Ctl::Chips { .. } => Size::ZERO,
            Ctl::Segmented(seg) => seg.preferred_size(env.text, t),
            Ctl::Color { hex, chevron, .. } => {
                let h = hex.measure(env.text);
                let c = chevron.measure(env.text);
                Size::new(t.space(8) + t.space(2) + h.width + t.space(2) + c.width, t.space(8))
            }
            Ctl::Text(input) => input.preferred_size(t, (width * 0.4).clamp(t.space(40), t.space(64))),
            Ctl::Dropdown(sel) => sel.preferred_size(t, (width * 0.4).clamp(t.space(40), t.space(70))),
        }
    }

    /// Yığılabilen kontrol: geniş kontroller (kaydırıcı, segmentli, metin, açılır seçim) dar
    /// satırda başlığın altına, tam genişliğe iner. Küçük kontroller (anahtar, renk) yerinde
    /// kalır; haplar zaten alttadır.
    fn stackable(&self) -> bool {
        matches!(self.ctl, Ctl::Slider { .. } | Ctl::Segmented(_) | Ctl::Text(_) | Ctl::Dropdown(_))
    }

    /// Bu genişlikte satır yığılır mı ([`should_stack`]; karar `self.stack`'te saklanır).
    fn stacked(&mut self, env: &mut Env, width: f32) -> bool {
        let stack = self.stackable() && {
            let ctl = self.control_size(env, width);
            let prev_w = if self.previous.visible() { self.previous.size(env.theme).width + env.theme.space(1) } else { 0.0 };
            should_stack(env.theme, env.text, width, ctl.width + prev_w, &mut self.title, self.subtitle.as_mut())
        };
        self.stack = stack;
        stack
    }

    /// Yığılmış satırda başlık ve açıklama yüksekliği (kontrol hariç).
    /// "Yeni" rozeti için kimlik ve `since` (yalnızca shell ayarı; kimlik shell.toml yolu).
    fn seen_key(&self) -> Option<(&'static str, i64)> {
        let since = self.desc.since()?;
        seen::badge(self.desc.path, Some(since)).then_some((self.desc.path, since))
    }

    /// Başlığın yanındaki rozetin kapladığı genişlik (boşluk dahil); rozet yoksa 0.
    fn badge_width(&mut self, env: &mut Env) -> f32 {
        if self.seen_key().is_none() {
            return 0.0;
        }
        self.badge.size(env.text, env.theme).width + env.theme.space(2)
    }

    fn text_height(&mut self, env: &mut Env, text_w: f32) -> f32 {
        let bw = self.badge_width(env);
        self.title.set_max_width(Some((text_w - bw).max(env.theme.space(20))));
        let mut h = self.title.measure(env.text).height;
        if let Some(s) = &mut self.subtitle {
            s.set_max_width(Some(text_w));
            h += env.theme.space(1) + s.measure(env.text).height;
        }
        h
    }

    fn head_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let (px, py) = row_padding(env.theme);
        if self.stacked(env, width) {
            // Başlığın sağında yalnızca sıfırla düğmesine yer.
            let text_w = stacked_text_width(env.theme, width);
            let text = self.text_height(env, text_w);
            let ctl = self.control_size(env, width);
            return text + env.theme.space(2) + ctl.height + py * 2.0;
        }
        let ctl = self.control_size(env, width);
        let prev_w = if self.previous.visible() { self.previous.size(env.theme).width + env.theme.space(1) } else { 0.0 };
        let text_w = side_text_width(env.theme, width, ctl.width + prev_w).max(env.theme.space(20));
        let bw = self.badge_width(env);
        self.title.set_max_width(Some((text_w - bw).max(env.theme.space(20))));
        let mut h = self.title.measure(env.text).height;
        if let Some(s) = &mut self.subtitle {
            s.set_max_width(Some(text_w));
            h += env.theme.space(1) + s.measure(env.text).height;
        }
        if let Ctl::Chips { buttons, .. } = &mut self.ctl {
            let area = Rect::new(0.0, 0.0, width - px * 2.0, 0.0);
            let (_, bottom) = flow_buttons(buttons.iter_mut(), env.text, env.theme, area, false);
            h += env.theme.space(3) + bottom;
        }
        row_height_for(env.theme, h.max(ctl.height))
    }

    fn picker_height(&self, theme: &Theme, width: f32) -> f32 {
        match &self.ctl {
            Ctl::Color { picker, .. } => {
                let (px, py) = row_padding(theme);
                picker.preferred_size(theme, picker_width(theme, width - px * 2.0)).height + py
            }
            _ => 0.0,
        }
    }

    fn write(&mut self, env: &mut Env, value: Option<Value>) {
        let d = self.desc;
        let value = value.and_then(|v| d.stored(v, env.store));
        env.set(d.display_title(), d.file, d.key(), value);
    }

    /// Anahtar yazımı: tek anahtar ya da (ör. "Hareketi azalt" kapanırken ön ayar) tek geri al kaydında grup.
    fn write_toggle(&mut self, env: &mut Env, on: bool) {
        let d = self.desc;
        let mut writes = d.toggle_writes(on, env.store);
        if writes.len() == 1 {
            let (file, path, value) = writes.remove(0);
            env.set(d.display_title(), file, path, value);
        } else {
            env.push(Action::Group { label: d.display_title(), writes });
        }
    }

    fn set_choice(&mut self, i: Option<usize>, now: Instant) {
        match &mut self.ctl {
            Ctl::Chips { buttons, selected } => {
                *selected = i;
                for (j, b) in buttons.iter_mut().enumerate() {
                    b.set_variant(chip_variant(Some(j) == i));
                }
            }
            Ctl::Segmented(seg) => {
                seg.set_selected(i.unwrap_or(0), now);
            }
            Ctl::Dropdown(sel) => sel.set_selected(i.unwrap_or(0)),
            _ => {}
        }
    }
}

/// Satır dar düzende mi (§4: içerik sütunu `space(140)`'tan dar).
pub fn is_narrow(theme: &Theme, width: f32) -> bool {
    width < theme.space(140)
}

/// Yan yana satırda kontrolün yanında metne kalan genişlik (`ctl_width` kontrol + yanındaki
/// küçük düğmeler; sağda sıfırla düğmesine `space(10)` ayrılır).
pub fn side_text_width(theme: &Theme, width: f32, ctl_width: f32) -> f32 {
    let (px, _) = row_padding(theme);
    width - px * 2.0 - ctl_width - theme.space(4) - theme.space(10)
}

/// Yığılmış satırda başlığa kalan genişlik (sağında yalnızca sıfırla düğmesi).
pub fn stacked_text_width(theme: &Theme, width: f32) -> f32 {
    let (px, _) = row_padding(theme);
    (width - px * 2.0 - theme.space(10)).max(theme.space(20))
}

/// Satır yığılsın mı (§4, §5.1): kontrol başlığın altına, tam genişliğe iner. Ölçüt:
/// - sütun dar (`is_narrow`), ya da
/// - kontrolün yanında metne `space(30)`'dan az yer kalıyor, ya da
/// - başlık o yerde tek satıra sığmıyor, ya da açıklama tek satıra sığmıyor (§5.1 ≤ 1 satır).
///
/// Böylece genişlik eşiğinin üstünde de (ör. uzun başlık + geniş segmentli kontrol, 1.5 ölçek)
/// metin kontrolün altında ezilmez, üst üste binmez. Ölçüm için etiketlerin azami genişliği
/// kaldırılır; çağıran yerleşimde yeniden kurar. Her standart satır bloğu (kayıt satırı, dil
/// satırları, Ekranlar, Hakkında) aynı kuralı kullanır.
pub fn should_stack(theme: &Theme, text: &mut nuxut::TextSystem, width: f32, ctl_width: f32, title: &mut Label, subtitle: Option<&mut Label>) -> bool {
    if is_narrow(theme, width) {
        return true;
    }
    let avail = side_text_width(theme, width, ctl_width);
    if avail < theme.space(30) {
        return true;
    }
    title.set_max_width(None);
    if title.measure(text).width > avail {
        return true;
    }
    // Açıklama ≤ 1 satır (§5.1): kontrolün yanında sarılacaksa satır yığılır (yığılmışta tam
    // genişlikte tek satıra iner). Açık satır sonu da yığar.
    subtitle.is_some_and(|s| {
        s.set_max_width(None);
        s.measure(text).width > avail || s.text().contains('\n')
    })
}

fn new_select(theme: &Theme, options: &[Choice], selected: Option<usize>) -> Select {
    let labels = options.iter().map(|(_, l)| l.clone()).collect();
    Select::new(theme, labels, selected.unwrap_or(0))
}

fn picker_width(theme: &Theme, available: f32) -> f32 {
    available.min(theme.space(90))
}

fn norm(desc: &SettingDesc, v: f64) -> f32 {
    let Some((min, max)) = desc.bounds() else { return 0.0 };
    ((v - min) / (max - min)).clamp(0.0, 1.0) as f32
}

fn denorm(desc: &SettingDesc, t: f32) -> f64 {
    let Some((min, max)) = desc.bounds() else { return 0.0 };
    min + (max - min) * t as f64
}

/// Kaydırıcı değerinin metni, etkin biçim locale'inin ondalık ayırıcısıyla ([`crate::numfmt`]).
pub fn format_number(desc: &SettingDesc, v: f64) -> String {
    format_number_in(desc, v, &crate::numfmt::current())
}

/// [`format_number`] verilen sayı locale'iyle (saf; testler).
pub fn format_number_in(desc: &SettingDesc, v: f64, locale: &nuxut::locale::Locale) -> String {
    let Control::Slider { step, unit, integer, percent, .. } = desc.control else { return String::new() };
    // Yüzde tek yerden (framework `NumberOptions::percent`, işaretin konumu locale'den):
    // `percent` = değer 0..1 kesir; birimi "%" olan = değer zaten yüzde.
    if percent {
        return crate::numfmt::percent_in(locale, v, 0);
    }
    if unit == "%" {
        let decimals = if integer { 0 } else { (-step.log10()).ceil().max(0.0) as usize };
        return crate::numfmt::percent_in(locale, v / 100.0, decimals);
    }
    if unit == "ms" && v >= 1000.0 {
        // Uzun süreler saniyeyle okunur.
        return format!("{} {}", crate::numfmt::format_in(locale, v / 1000.0, 1), unit_label("s"));
    }
    if unit == "s" && v >= 60.0 {
        // Dakikalık süreler dakikayla okunur ("600 sn" yerine "10 dk"; boşta süreleri).
        return format!("{} {}", crate::numfmt::format_in(locale, v / 60.0, if (v % 60.0).abs() < 1e-9 { 0 } else { 1 }), unit_label("min"));
    }
    let decimals = if integer { 0 } else { (-step.log10()).ceil().max(0.0) as usize };
    // Çarpan en az bir ondalıkla ("1,0×", "1,25×"): kaydırırken "1×" ↔ "1,1×" zıplaması yok.
    if unit == "×" {
        return format!("{}×", crate::numfmt::format_range_in(locale, v, 1, decimals.max(1)));
    }
    // Sıfırın iki yanına uzanan birimsiz değer (işaretçi hızı −1..1): sabit ondalık ve pozitifte
    // "+" — işaret varsayılana (0) göre yönü söyler, sabit genişlik etiketin sürüklerken
    // titremesini önler ("+0,25", "0,00", "-0,30").
    if unit.is_empty() && !integer && desc.bounds().is_some_and(|(lo, hi)| lo < 0.0 && hi > 0.0) {
        let s = crate::numfmt::format_range_in(locale, v, decimals, decimals);
        let zero = crate::numfmt::format_range_in(locale, 0.0, decimals, decimals);
        return if v > 0.0 && s != zero { format!("+{s}") } else { s };
    }
    let s = crate::numfmt::format_in(locale, v, decimals);
    match unit {
        "" => s,
        u => format!("{s} {}", unit_label(u)),
    }
}

/// Birim sembolünün görünen adı. Shell şeması dile bağımsız kapalı bir sembol kümesi gönderir
/// (px, %, ms, s, min, K — i18n kararı A, 2026-10-13; "K" Kelvin SI sembolü, çevrilmez);
/// eski shell'in "sn"si de saniye sayılır.
/// Bilinmeyen sembol (Ayarlar'ın yerel birimleri dahil) olduğu gibi gösterilir.
fn unit_label(unit: &str) -> std::borrow::Cow<'_, str> {
    match unit {
        "px" => t!("settings-unit-pixel").into(),
        "ms" => t!("settings-unit-ms").into(),
        "s" | "sn" => t!("settings-unit-second").into(),
        "min" => t!("settings-unit-minute").into(),
        u => u.into(),
    }
}

pub fn parse_color(s: &str) -> Color {
    // Geçersiz değer görünmez (dolgu yok); doğrulama uyarısını tema yükleyicisi verir.
    Color::from_hex(s).unwrap_or(Color::TRANSPARENT)
}

/// Satırın uyarısı: kayıttaki duruma bağlı not ([`SettingDesc::note_in`]) ya da geçersiz kılma
/// uyarısı (yalnızca etkin değerli satırlar; durum hazır değilse yok).
fn override_text(desc: &SettingDesc, store: &Store) -> Option<String> {
    desc.note_in(store).or_else(|| override_text_in(desc, &crate::services::input::status()))
}

/// [`override_text`], durum verilerek (saf; testler).
fn override_text_in(desc: &SettingDesc, state: &crate::services::input::InputState) -> Option<String> {
    if !desc.is_live() {
        return None;
    }
    state.ready()?.override_for(desc.path).map(crate::services::input::override_note)
}

/// Geçersiz kılma uyarısı: framework'ün sıkı (Caption + küçük ikon) uyarı notu.
pub(crate) fn override_notice(theme: &Theme, text: &str) -> Notice {
    Notice::new(theme, NoticeKind::Warning, text).compact(theme)
}

impl SettingRow {
    /// Uyarının yüksekliği (alt boşluk dahil); uyarı yoksa 0.
    fn note_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let (px, py) = row_padding(env.theme);
        let Some(n) = &mut self.note else { return 0.0 };
        n.measure(env.text, env.theme, (width - px * 2.0).max(env.theme.space(20))).height + py * 0.5
    }

    /// Uyarıyı durumdan yeniden kurar.
    fn refresh_note(&mut self, theme: &Theme, store: &Store) {
        match (override_text(self.desc, store), &mut self.note) {
            (None, n) => *n = None,
            (Some(s), Some(l)) => {
                l.set_text(&s);
            }
            (Some(s), n) => *n = Some(override_notice(theme, &s)),
        }
    }
}

fn is_default(desc: &SettingDesc, store: &Store) -> bool {
    let Some(v) = store.get(desc.file, &desc.key()) else { return true };
    // Etkin değerli satırda yazılmış değer hep "değiştirilmiş" sayılır: ↺ anahtarı siler.
    if desc.is_live() {
        return false;
    }
    desc.default_value(store).is_some_and(|d| desc.same_value(v, d))
}

impl Block for SettingRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    // Ekran okuyucu (framework ADR-0011): denetim satırın başlığıyla adlanır, alt metin açıklama;
    // anahtar ayar kimliği (kalıcı). Satır odaktaysa denetimi ağacın odağı.
    fn access(&self, tree: &mut nuxut::access::AccessTree) {
        use nuxut::access::Role;
        let title = self.title.text();
        let desc = self.subtitle.as_ref().map_or("", |s| s.text());
        let key = self.desc.id;
        let focused = self.focused;
        match &self.ctl {
            Ctl::Toggle(t) => {
                t.access(tree, key).label(title).description(desc).focused_if(focused);
            }
            Ctl::Slider { slider, value, .. } => {
                let (lo, hi) = self.shape.bounds.unwrap_or((0.0, 100.0));
                slider.access(tree, key, lo..=hi).label(title).description(desc).value(value.text()).focused_if(focused);
            }
            Ctl::Segmented(s) => {
                s.access(tree, key, focused).label(title).description(desc);
            }
            Ctl::Chips { buttons, selected } => {
                let id = tree.push(key, Role::RadioGroup).label(title).description(desc).bounds(self.rect).id();
                tree.within(id, |tree| {
                    for (i, b) in buttons.iter().enumerate() {
                        let on = *selected == Some(i);
                        b.access(tree, i).toggled(on).focused_if(focused && on);
                    }
                });
            }
            Ctl::Color { hex, swatch, .. } => {
                let d = [desc, hex.text()].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(", ");
                tree.push(key, Role::Button).label(title).description(&d).bounds(*swatch).clickable().focused_if(focused);
            }
            Ctl::Text(input) => {
                input.access(tree, key).label(title).description(desc);
            }
            Ctl::Dropdown(sel) => {
                sel.access(tree, key).label(title).description(desc).focused_if(focused);
            }
        }
        if self.show_reset {
            self.reset.access(tree, (key, "reset")).label(&t!("settings-a11y-reset", setting = title));
        }
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let head = self.head_height(env, width);
        self.note_h = self.note_height(env, width);
        let extra = match &self.ctl {
            Ctl::Color { expand, .. } => expand.value(env.now).max(0.0) * self.picker_height(env.theme, width),
            _ => 0.0,
        };
        head + extra + self.note_h
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let head = self.head_height(env, rect.width);
        let ctl = self.control_size(env, rect.width);
        self.head_h = head;
        let right = rect.right() - px;
        let rs = self.reset.preferred_size(t);
        let rs = Size::new(rs.width * 0.8, rs.height * 0.8);
        let (cy, ctl_rect) = if self.stack {
            // Başlık üstte; kontrol altında tam genişlik; sıfırla başlığın sağında.
            let text_w = stacked_text_width(t, rect.width);
            let text = self.text_height(env, text_w);
            let ctl_rect = Rect::new(rect.x + px, rect.y + py + text + t.space(2), rect.width - px * 2.0, ctl.height);
            let ty = rect.y + py + self.title.measure(env.text).height * 0.5;
            self.reset.set_rect(Rect::new(right - rs.width, ty - rs.height * 0.5, rs.width, rs.height));
            (ctl_rect.center().y, ctl_rect)
        } else {
            let cy = rect.y + head * 0.5;
            let ctl_rect = Rect::new(right - ctl.width, cy - ctl.height * 0.5, ctl.width, ctl.height);
            self.reset.set_rect(Rect::new(ctl_rect.x - t.space(2) - rs.width, cy - rs.height * 0.5, rs.width, rs.height));
            (cy, ctl_rect)
        };
        // "Önceki hâline dön" sıfırlamanın solunda; sıfırlama gizliyken onun yerinde.
        let ps = self.previous.size(t);
        let prev_right = if self.show_reset { self.reset.rect().x - t.space(1) } else { self.reset.rect().right() };
        self.previous.set_rect(Rect::new(prev_right - ps.width, cy - ps.height * 0.5, ps.width, ps.height));
        match &mut self.ctl {
            Ctl::Toggle(tg) => tg.set_rect(ctl_rect),
            Ctl::Slider { slider, value, .. } => {
                let v = value.measure(env.text);
                let vw = v.width.max(t.space(14));
                let sw = ctl_rect.width - vw - t.space(3);
                let sh = slider.preferred_size(t, sw).height;
                slider.set_rect(Rect::new(ctl_rect.x, cy - sh * 0.5, sw, sh));
            }
            Ctl::Chips { buttons, .. } => {
                let mut top = rect.y + py + self.title.measure(env.text).height;
                if let Some(s) = &mut self.subtitle {
                    top += t.space(1) + s.measure(env.text).height;
                }
                let area = Rect::new(rect.x + px, top + t.space(3), rect.width - px * 2.0, 0.0);
                flow_buttons(buttons.iter_mut(), env.text, t, area, true);
            }
            Ctl::Segmented(seg) => seg.set_rect(t, ctl_rect),
            Ctl::Color { swatch, picker, .. } => {
                *swatch = Rect::new(ctl_rect.x, cy - t.space(4), t.space(8), t.space(8));
                let pw = picker_width(t, rect.width - px * 2.0);
                let ph = picker.preferred_size(t, pw).height;
                picker.set_rect(t, Rect::new(rect.x + px, rect.y + head, pw, ph));
            }
            Ctl::Text(input) => input.set_rect(ctl_rect),
            Ctl::Dropdown(sel) => sel.set_rect(t, ctl_rect),
        }
        let _ = py;
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let t = env.theme;
        let mut repaint = false;
        if let Ctl::Dropdown(sel) = &mut self.ctl
            && sel.is_open()
        {
            // Açık menü olayın tek sahibi (dışına tıklama yalnızca kapatır).
            let r = sel.pointer(env.text, t, e, now);
            if let Some(SelectEvent::Changed(i)) = r.event {
                let v = self.shape.choices.get(i).and_then(|(v, _)| self.desc.choice_stored(v, env.store));
                self.write(env, v);
            }
            return true;
        }
        let (r, fired) = self.previous.pointer(env, e);
        if fired {
            return true;
        }
        repaint |= r;
        if self.show_reset {
            let r = self.reset.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.write(env, None);
                return true;
            }
        }
        let head_rect = Rect::new(self.rect.x, self.rect.y, self.rect.width, self.head_height(env, self.rect.width));
        let row_click = matches!(self.ctl, Ctl::Toggle(_) | Ctl::Color { .. });
        if row_click {
            let tap = self.hover.handle(head_rect, e, now);
            repaint |= tap.repaint;
            if tap.clicked && !self.reset.rect().contains(e.position) && !(self.previous.visible() && self.previous.rect().contains(e.position)) {
                match &mut self.ctl {
                    Ctl::Toggle(tg) => {
                        let on = !tg.is_on();
                        tg.set_on(on, now);
                        self.write_toggle(env, on);
                    }
                    Ctl::Color { open, expand, chevron, .. } => {
                        *open = !*open;
                        expand.set_target(if *open { 1.0 } else { 0.0 }, now);
                        chevron.set_icon(Icon::named(if *open { IconName::ChevronUp } else { IconName::ChevronDown }));
                    }
                    _ => {}
                }
                return true;
            }
        }
        match &mut self.ctl {
            Ctl::Toggle(_) => {}
            Ctl::Slider { slider, value, steps } => {
                let was = slider.is_dragging();
                let r = slider.pointer(e, now);
                repaint |= r.repaint;
                if let Some(v) = r.event {
                    let mut x = denorm(self.desc, v);
                    if steps.is_none() {
                        x = crate::registry::quantize(x, match self.desc.control {
                            Control::Slider { step, .. } => step,
                            _ => 1.0,
                        });
                    }
                    value.set_text(&format_number(self.desc, x));
                    let val = self.desc.number_value(x);
                    self.dragging = true;
                    env.set(self.desc.display_title(), self.desc.file, self.desc.key(), Some(val));
                }
                if was && !slider.is_dragging() {
                    env.push(Action::Seal);
                    self.dragging = false;
                }
            }
            Ctl::Segmented(seg) => {
                let r = seg.pointer(e, now);
                repaint |= r.repaint;
                if let Some(i) = r.event {
                    let v = self.shape.choices.get(i).and_then(|(v, _)| self.desc.choice_stored(v, env.store));
                    self.write(env, v);
                }
            }
            Ctl::Chips { buttons, selected } => {
                let mut hit = None;
                for (i, b) in buttons.iter_mut().enumerate() {
                    let r = b.pointer(e, now);
                    repaint |= r.repaint;
                    if r.event.is_some() && Some(i) != *selected {
                        hit = Some(i);
                    }
                }
                if let Some(i) = hit {
                    self.set_choice(Some(i), now);
                    let v = self.shape.choices.get(i).and_then(|(v, _)| self.desc.choice_stored(v, env.store));
                    self.write(env, v);
                    repaint = true;
                }
            }
            Ctl::Color { picker, open, .. } => {
                if *open {
                    let r = picker.pointer(env.text, t, e, now);
                    repaint |= r.repaint;
                    match r.event {
                        Some(ColorEvent::Changed(c)) => {
                            env.set(self.desc.display_title(), self.desc.file, self.desc.key(), self.desc.stored(Value::from(c.to_hex()), env.store));
                        }
                        Some(ColorEvent::Committed(c)) => {
                            env.set(self.desc.display_title(), self.desc.file, self.desc.key(), self.desc.stored(Value::from(c.to_hex()), env.store));
                            env.push(Action::Seal);
                        }
                        None => {}
                    }
                    let below = Rect::new(self.rect.x, head_rect.bottom(), self.rect.width, self.rect.bottom() - head_rect.bottom());
                    if let PointerKind::Press(MouseButton::Left) = e.kind
                        && below.contains(e.position)
                    {
                        env.push(Action::Focus);
                    }
                }
            }
            Ctl::Text(input) => {
                let r = input.pointer(env.text, t, e, now);
                repaint |= r.repaint;
                if let PointerKind::Press(MouseButton::Left) = e.kind
                    && input.rect().contains(e.position)
                {
                    env.push(Action::Focus);
                }
            }
            Ctl::Dropdown(sel) => {
                let r = sel.pointer(env.text, t, e, now);
                repaint |= r.repaint;
                if sel.is_open() {
                    // Menü açıldı: klavye (oklar, Enter, Escape) satıra gelsin.
                    env.push(Action::Focus);
                }
            }
        }
        if !row_click {
            repaint |= self.hover.handle(Rect::ZERO, e, now).repaint;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let now = env.now;
        match &mut self.ctl {
            Ctl::Toggle(tg) => {
                if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) {
                    let on = !tg.is_on();
                    tg.set_on(on, now);
                    self.write_toggle(env, on);
                    return true;
                }
                false
            }
            Ctl::Slider { slider, value, .. } => {
                let r = slider.key(e, now);
                if let Some(v) = r.event {
                    let x = denorm(self.desc, v);
                    value.set_text(&format_number(self.desc, x));
                    let val = self.desc.number_value(x);
                    self.write(env, Some(val));
                    return true;
                }
                r.repaint
            }
            Ctl::Segmented(seg) => {
                let r = seg.key(e, now);
                if let Some(i) = r.event {
                    let v = self.shape.choices.get(i).and_then(|(v, _)| self.desc.choice_stored(v, env.store));
                    self.write(env, v);
                    return true;
                }
                r.repaint
            }
            Ctl::Chips { buttons, selected } => {
                if e.state == KeyState::Released {
                    return false;
                }
                let n = buttons.len();
                let cur = selected.unwrap_or(0);
                let next = match e.key {
                    Key::Left => cur.checked_sub(1),
                    Key::Right => (cur + 1 < n).then_some(cur + 1),
                    _ => None,
                };
                if let Some(i) = next {
                    self.set_choice(Some(i), now);
                    let v = self.shape.choices.get(i).and_then(|(v, _)| self.desc.choice_stored(v, env.store));
                    self.write(env, v);
                    return true;
                }
                false
            }
            Ctl::Color { picker, open, expand, .. } => {
                if *open {
                    let r = picker.key(env.theme, e, now);
                    if let Some(ColorEvent::Committed(c) | ColorEvent::Changed(c)) = r.event {
                        env.set(self.desc.display_title(), self.desc.file, self.desc.key(), self.desc.stored(Value::from(c.to_hex()), env.store));
                        return true;
                    }
                    if e.key == Key::Escape && e.state == KeyState::Pressed {
                        *open = false;
                        expand.set_target(0.0, now);
                        return true;
                    }
                    return r.repaint;
                }
                if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) {
                    *open = true;
                    expand.set_target(1.0, now);
                    return true;
                }
                false
            }
            Ctl::Text(input) => {
                let r = input.key(e, now);
                match r.event {
                    Some(TextInputEvent::Submit) => {
                        let v = input.text().trim().to_owned();
                        let value = (!v.is_empty()).then(|| Value::from(v));
                        self.write(env, value);
                        env.push(Action::Seal);
                        true
                    }
                    Some(TextInputEvent::Cancel) => {
                        input.set_text(&self.desc.string(env.store));
                        true
                    }
                    _ => r.repaint,
                }
            }
            Ctl::Dropdown(sel) => {
                let was_open = sel.is_open();
                let r = sel.key(env.text, env.theme, e, now);
                if let Some(SelectEvent::Changed(i)) = r.event {
                    let v = self.shape.choices.get(i).and_then(|(v, _)| self.desc.choice_stored(v, env.store));
                    self.write(env, v);
                    return true;
                }
                // Açıkken her tuş menünündür (Escape kapatır, Tab sayfaya geçmez).
                was_open || r.repaint
            }
        }
    }

    fn captures_keys(&self) -> bool {
        matches!(&self.ctl, Ctl::Dropdown(sel) if sel.is_open())
    }

    fn overlay_open(&self) -> bool {
        matches!(&self.ctl, Ctl::Dropdown(sel) if sel.is_open())
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        let was = self.focused;
        self.focused = focused;
        match &mut self.ctl {
            Ctl::Toggle(tg) => {
                tg.set_focus_visible(focused && visible);
            }
            Ctl::Slider { slider, .. } => slider.set_focus_visible(focused && visible),
            Ctl::Segmented(seg) => seg.set_focus_visible(focused && visible),
            Ctl::Dropdown(sel) => sel.set_focus_visible(focused && visible),
            Ctl::Chips { buttons, selected } => {
                for (i, b) in buttons.iter_mut().enumerate() {
                    b.set_focus_visible(focused && visible && Some(i) == *selected);
                }
            }
            Ctl::Color { picker, open, .. } => picker.set_hex_focused(focused && *open, now),
            Ctl::Text(input) => {
                input.set_focused(focused, now);
                if was && !focused {
                    // Odak giderken yazılanı kaydet (Enter'a basılmasa da).
                }
            }
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.show_reset && let Some(c) = self.reset.cursor(p) {
            return Some(c);
        }
        if let Some(c) = self.previous.cursor(p) {
            return Some(c);
        }
        let control = match &self.ctl {
            Ctl::Toggle(tg) => tg.cursor(p),
            Ctl::Slider { slider, .. } => slider.cursor(p),
            Ctl::Segmented(seg) => seg.cursor(p),
            Ctl::Chips { buttons, .. } => buttons.iter().find_map(|b| b.cursor(p)),
            Ctl::Color { picker, open, .. } => open.then(|| picker.cursor(p)).flatten(),
            Ctl::Text(input) => input.cursor(p),
            Ctl::Dropdown(sel) => sel.cursor(p),
        };
        if control.is_some() {
            return control;
        }
        // Satırın tamamı tıklanabilir (anahtar, renk başlığı).
        let clickable = matches!(self.ctl, Ctl::Toggle(_) | Ctl::Color { .. });
        let head = Rect::new(self.rect.x, self.rect.y, self.rect.width, self.head_h);
        (clickable && head.contains(p)).then_some(CursorShape::Pointer)
    }

    fn demo_expand(&mut self, now: Instant) {
        if let Ctl::Color { open, expand, chevron, .. } = &mut self.ctl {
            *open = true;
            expand.set_target(1.0, now);
            chevron.set_icon(Icon::named(IconName::ChevronUp));
        }
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        matches!(event, crate::services::Event::Snapshots) && self.previous.refresh()
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        match &mut self.ctl {
            Ctl::Text(i) => Some(i),
            _ => None,
        }
    }

    fn sync(&mut self, env: &mut Env) {
        let d = self.desc;
        let now = env.now;
        let store = env.store;
        self.show_reset = d.is_set(store) && !is_default(d, store);
        self.refresh_note(env.theme, store);
        if let Some(s) = &mut self.subtitle {
            s.set_text(&d.subtitle_in(store));
        }
        // Şema geldi ve seçenekler/aralık yerel yedekten farklı: denetim şemanınkiyle yeniden kurulur.
        let shape = Shape::of(d);
        if shape != self.shape {
            if let Some(ctl) = build_ctl(env.theme, d, store, &shape) {
                self.ctl = ctl;
            }
            self.shape = shape;
            return;
        }
        match &mut self.ctl {
            Ctl::Toggle(tg) => {
                tg.set_on(d.bool(store), now);
            }
            Ctl::Slider { slider, value, .. } => {
                if !slider.is_dragging() {
                    slider.set_value(norm(d, d.number(store)), now);
                    value.set_text(&format_number(d, d.number(store)));
                }
            }
            Ctl::Segmented(_) | Ctl::Chips { .. } | Ctl::Dropdown(_) => {
                let cur = d.string(store);
                let i = self.shape.choices.iter().position(|(v, _)| *v == cur);
                self.set_choice(i, now);
            }
            Ctl::Color { color, hex, picker, .. } => {
                let c = parse_color(&d.string(store));
                color.set_target(c, now);
                hex.set_text(&c.to_hex());
                if picker.color() != c {
                    picker.set_color(env.theme, c, now);
                }
            }
            Ctl::Text(input) => {
                if !input.is_focused() {
                    input.set_text(&d.string(store));
                }
            }
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.badge.set_theme(theme, now);
        if let Some(s) = &mut self.subtitle {
            s.set_style(text_style(theme, TextRole::Caption));
        }
        if let Some(n) = &mut self.note {
            n.set_theme(theme, now);
        }
        self.reset.set_theme(theme, now);
        self.previous.set_theme(theme, now);
        self.hover.set_theme(theme, now);
        match &mut self.ctl {
            Ctl::Toggle(t) => t.set_theme(theme, now),
            Ctl::Slider { slider, value, .. } => {
                slider.set_theme(theme, now);
                value.set_style(text_style(theme, TextRole::Label));
            }
            Ctl::Segmented(seg) => seg.set_theme(theme, now),
            Ctl::Chips { buttons, .. } => buttons.iter_mut().for_each(|b| b.set_theme(theme, now)),
            Ctl::Color { picker, hex, color, expand, chevron, .. } => {
                picker.set_theme(theme, now);
                hex.set_style(text_style(theme, TextRole::Mono));
                color.set_motion(theme.motion.effects, now);
                expand.set_motion(theme.motion.spatial, now);
                chevron.set_theme(theme, now);
            }
            Ctl::Text(i) => i.set_theme(theme, now),
            // Ölçüm tembel (sonraki set_rect/draw/pointer); açık menü de yeni temaya geçer.
            Ctl::Dropdown(sel) => sel.set_theme(theme, now),
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let rect = self.rect;
        // Hover katmanı yalnızca tıklanabilir satırlarda.
        let head_h = match &self.ctl {
            Ctl::Color { expand, .. } => rect.height - expand.value(now).max(0.0) * self.picker_height(t, rect.width),
            _ => rect.height - self.note_h,
        };
        if let Some(n) = &mut self.note {
            // Uyarı başlık kısmının altında, alt boşluğun içinde.
            n.set_rect(Rect::new(rect.x + px, rect.y + head_h - py * 0.5, (rect.width - px * 2.0).max(t.space(20)), self.note_h - py * 0.5));
            n.draw(ui);
        }
        let head = Rect::new(rect.x, rect.y, rect.width, head_h);
        self.hover.draw_layer(ui.painter, head, Corners::uniform(t.shape.radius_medium * 0.5), t, now);
        if self.hover.is_animating(now) {
            ui.request_frame();
        }
        // Metin dikeyde başlığa ortalanır.
        let title_h = self.title.measure_in(ui.painter).height;
        let sub_h = self.subtitle.as_mut().map(|s| s.measure_in(ui.painter).height + t.space(1)).unwrap_or(0.0);
        let top_aligned = matches!(self.ctl, Ctl::Chips { .. }) || self.stack;
        let ty = if top_aligned { head.y + py } else { head.y + (head.height - title_h - sub_h) * 0.5 };
        self.title.draw(ui.painter, Point::new(rect.x + px, ty), ui.fade(c.on_surface));
        if let Some((id, since)) = self.seen_key() {
            // Başlığın (en geniş satırının) sağında, ilk satırın ortasında.
            let title_w = self.title.measure_in(ui.painter).width;
            let style = self.title.style();
            let line_h = (style.size * style.line_height).min(title_h);
            let b = self.badge.draw(ui, rect.x + px + title_w + t.space(2), ty + line_h * 0.5);
            // Görüldü: başlık ve rozet görünür alanın içindeyse (bkz. `seen::sight`).
            seen::sight(id, Some(since), Rect::new(rect.x + px, ty, b.right() - rect.x - px, title_h.max(b.height)));
        }
        if let Some(s) = &mut self.subtitle {
            s.draw(ui.painter, Point::new(rect.x + px, ty + title_h + t.space(1)), ui.fade(c.on_surface_variant));
        }
        if self.show_reset {
            self.reset.draw(ui);
        }
        self.previous.draw(ui);
        match &mut self.ctl {
            Ctl::Toggle(tg) => tg.draw(ui),
            Ctl::Slider { slider, value, .. } => {
                slider.draw(ui);
                let s = value.measure_in(ui.painter);
                let r = slider.rect();
                let x = rect.right() - px - s.width;
                value.draw(ui.painter, Point::new(x, r.center().y - s.height * 0.5), ui.fade(c.on_surface_variant));
            }
            Ctl::Chips { buttons, .. } => buttons.iter_mut().for_each(|b| b.draw(ui)),
            Ctl::Segmented(seg) => seg.draw(ui),
            Ctl::Color { color, hex, picker, expand, chevron, swatch, .. } => {
                let col = color.value(now);
                if !color.is_settled(now) {
                    ui.request_frame();
                }
                ui.painter.rect(*swatch, &RectStyle {
                    fill: ui.fade(col),
                    radii: Corners::uniform(swatch.height * 0.5),
                    border: Some(Border { width: HAIRLINE, color: ui.fade(content_on(t, col).multiply_alpha(0.25)) }),
                });
                let hs = hex.measure_in(ui.painter);
                hex.draw(ui.painter, Point::new(swatch.right() + t.space(2), swatch.center().y - hs.height * 0.5), ui.fade(c.on_surface_variant));
                let cs = chevron.measure(ui.painter.text_system());
                chevron.draw_centered(ui.painter, Point::new(rect.right() - px - cs.width * 0.5, swatch.center().y), ui.fade(c.on_surface_variant));
                let e = expand.value(now);
                if !expand.is_settled(now) {
                    ui.request_frame();
                }
                if e > 0.01 {
                    let clip = Rect::new(rect.x, head.bottom(), rect.width, rect.bottom() - head.bottom());
                    ui.painter.push_clip(clip);
                    ui.with_alpha(((e - 0.3) / 0.7).clamp(0.0, 1.0), |ui| picker.draw(ui));
                    ui.painter.pop_clip();
                }
                let _ = py;
            }
            Ctl::Text(input) => input.draw(ui),
            Ctl::Dropdown(sel) => {
                sel.draw(ui);
                // Menü kaplamada: sayfanın ve satırın kırpmasından bağımsız, yüzeye sığdırılır.
                sel.set_bounds(ui.surface_rect());
                sel.draw_overlay(ui);
            }
        }
    }
}

fn chip_variant(selected: bool) -> ButtonVariant {
    if selected { ButtonVariant::Tonal } else { ButtonVariant::Outlined }
}

/// Düğmeleri `area` genişliğinde soldan sağa sarar; `(son satır y, toplam yükseklik)`.
pub(crate) fn flow_buttons<'a>(buttons: impl IntoIterator<Item = &'a mut Button>, text: &mut nuxut::TextSystem, theme: &Theme, area: Rect, place: bool) -> (f32, f32) {
    let gap = theme.space(2);
    let (mut x, mut y, mut row_h) = (area.x, area.y, 0.0f32);
    for b in buttons {
        let s = b.preferred_size(text, theme);
        if x > area.x && x + s.width > area.right() {
            x = area.x;
            y += row_h + gap;
            row_h = 0.0;
        }
        if place {
            b.set_rect(Rect::new(x, y, s.width, s.height));
        }
        x += s.width + gap;
        row_h = row_h.max(s.height);
    }
    (y, y + row_h - area.y)
}

/// Satırın "önceki hâline dön" eylemi (shell snapshot'ı; `services::snapshots`). Yalnızca
/// ayarın geri yüklenebilir bir kaydı varsa görünür; "varsayılana dön"ün yanında durur.
/// Oturum içi geri al (`History`) ile ilişkisi `services/snapshots.rs` başındadır.
#[derive(Debug)]
pub struct PreviousAction {
    desc: &'static SettingDesc,
    button: IconButton,
    entry: Option<crate::services::snapshots::SnapshotEntry>,
    /// Geri yükleme istendi, sonuç bekleniyor: düğme devre dışı (çift istek yok). Değer bu
    /// sırada dosyadan gelir; Ayarlar framework.toml'a yazmaz.
    pending: bool,
}

impl PreviousAction {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        let mut s = Self { desc, button: IconButton::new(theme, Icon::named(IconName::History), IconButtonVariant::Standard), entry: None, pending: false };
        s.refresh();
        s
    }

    /// Son kaydı yeniden okur; görünürlük değiştiyse `true`.
    pub fn refresh(&mut self) -> bool {
        // Liste yenilendi: geri yükleme (varsa) bitti.
        let was_pending = std::mem::take(&mut self.pending);
        if was_pending {
            self.button.set_enabled(true, Instant::now());
        }
        let entry = crate::services::snapshots::latest_for(self.desc.id);
        let changed = entry.as_ref().map(|e| &e.id) != self.entry.as_ref().map(|e| &e.id);
        self.entry = entry;
        changed || was_pending
    }

    pub fn is_pending(&self) -> bool {
        self.pending
    }

    pub fn visible(&self) -> bool {
        self.entry.is_some()
    }

    /// "Varsayılana dön" ile aynı küçültülmüş boyut.
    pub fn size(&self, theme: &Theme) -> Size {
        let s = self.button.preferred_size(theme);
        Size::new(s.width * 0.8, s.height * 0.8)
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.button.set_rect(rect);
    }

    pub fn rect(&self) -> Rect {
        self.button.rect()
    }

    fn request(&self) -> Option<Action> {
        let e = self.entry.as_ref()?;
        Some(Action::Service(crate::services::Request::Snapshots(crate::services::snapshots::Command::Restore {
            id: e.id.clone(),
            setting: e.setting.clone(),
            label: self.desc.display_title(),
        })))
    }

    /// `(yeniden çiz, eylem gönderildi)`.
    pub fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> (bool, bool) {
        if !self.visible() {
            return (false, false);
        }
        let r = self.button.pointer(e, env.now);
        if r.event.is_some()
            && let Some(a) = self.request()
        {
            self.pending = true;
            self.button.set_enabled(false, env.now);
            env.push(a);
            return (true, true);
        }
        (r.repaint, false)
    }

    pub fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.visible().then(|| self.button.cursor(p)).flatten()
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.button.set_theme(theme, now);
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        if self.visible() {
            self.button.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{EASINGS, PageId, Tier};
    use crate::store::FileId;
    use nuxut::render::{DrawKind, Recorder};
    use nuxut::ui::input::Modifiers;
    use nuxut::TextSystem;

    /// Girdi-v1: `overrides` içindeki anahtarın satırında uyarı (yalnızca o satır); uyarılı satır
    /// uyarının yüksekliği kadar uzar.
    #[test]
    fn override_note_on_overridden_input_row() {
        use std::sync::Arc;

        use crate::services::input::{InputState, InputStatus, Override};
        let status = InputStatus {
            available: true,
            overrides: vec![
                Override { key: "input.keyboard.repeat_rate".into(), hypr: "input:repeat_rate".into(), file: Some("~/.config/hypr/hyprland.conf".into()), line: Some(12) },
                // Lua kullanıcısı: shell dosya/satır bilmiyor.
                Override { key: "input.mouse.sensitivity".into(), hypr: "input:sensitivity".into(), file: None, line: None },
            ],
            ..Default::default()
        };
        let state = InputState::Ready(Arc::new(status));
        let rate = crate::registry::by_id("input.keyboard.repeat_rate").unwrap();
        let delay = crate::registry::by_id("input.keyboard.repeat_delay").unwrap();
        assert_eq!(override_text_in(rate, &state).as_deref(), Some("Hyprland ayar dosyan bu değeri geçersiz kılıyor: ~/.config/hypr/hyprland.conf:12"));
        assert_eq!(override_text_in(delay, &state), None);
        let sens = crate::registry::by_id("input.mouse.sensitivity").unwrap();
        assert_eq!(override_text_in(sens, &state).as_deref(), Some("Hyprland ayarın bu değeri geçersiz kılıyor (input:sensitivity)."));
        assert_eq!(override_text_in(rate, &InputState::NotRunning), None, "durum yokken uyarı yok");

        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut row = SettingRow::new(&theme, rate, &store).unwrap();
        let plain = row.height(&mut env, 900.0);
        row.note = override_text_in(rate, &state).map(|s| override_notice(&theme, &s));
        let noted = row.height(&mut env, 900.0);
        assert!(noted > plain + theme.space(3), "{noted} / {plain}");
        // Uyarı framework'ün sıkı uyarı notu (Caption + küçük ikon).
        let n = row.note.as_ref().unwrap();
        assert_eq!((n.kind(), n.size()), (NoticeKind::Warning, nuxut::ui::NoticeSize::Compact));
        assert_eq!(n.text(), "Hyprland ayar dosyan bu değeri geçersiz kılıyor: ~/.config/hypr/hyprland.conf:12");
    }

    #[test]
    fn slider_value_uses_locale_decimal_separator() {
        let tr = nuxut::locale::Locale::parse("tr_TR.UTF-8").unwrap();
        let en = nuxut::locale::Locale::parse("en_US.UTF-8").unwrap();
        let speed = crate::registry::by_id("motion.speed").unwrap();
        let resp = crate::registry::by_id("motion.spatial.response").unwrap();
        let ms = crate::registry::by_id("motion.spatial.duration_ms").unwrap();
        // Türkçe görünüm eskisiyle aynı: virgül, sondaki sıfırlar atılır.
        assert_eq!(format_number_in(speed, 1.25, &tr), "1,25×");
        assert_eq!(format_number_in(speed, 2.0, &tr), "2,0×");
        let factor = crate::registry::by_id("input.touchpad.scroll_factor").unwrap();
        assert_eq!(format_number_in(factor, 1.0, &tr), "1,0×");
        assert_eq!(format_number_in(factor, 1.0, &en), "1.0×");
        let sens = crate::registry::by_id("input.mouse.sensitivity").unwrap();
        assert_eq!(format_number_in(sens, 0.0, &tr), "0,00");
        assert_eq!(format_number_in(sens, 0.25, &tr), "+0,25");
        let neg = format_number_in(sens, -0.3, &tr);
        assert!(neg.ends_with("0,30") && !neg.starts_with('0') && !neg.starts_with('+'), "negatif işaretli: {neg}");
        assert_eq!(format_number_in(sens, 0.5, &en), "+0.50");
        assert_eq!(format_number_in(resp, 0.35, &tr), "0,35 sn");
        assert_eq!(format_number_in(ms, 250.0, &tr), "250 ms");
        assert_eq!(format_number_in(ms, 1500.0, &tr), "1,5 sn");
        assert_eq!(format_number_in(ms, 2000.0, &tr), "2 sn");
        // Dakikalık saniye süreleri dakikayla (boşta süreleri).
        let idle = crate::registry::by_id("idle.lock_after").unwrap();
        assert_eq!(format_number_in(idle, 600.0, &tr), "10 dk");
        assert_eq!(format_number_in(idle, 90.0, &tr), "1,5 dk");
        assert_eq!(format_number_in(resp, 30.0, &tr), "30 sn");
        // en_US: nokta.
        assert_eq!(format_number_in(speed, 1.25, &en), "1.25×");
        assert_eq!(format_number_in(resp, 0.35, &en), "0.35 sn");
        assert_eq!(format_number_in(ms, 1500.0, &en), "1.5 sn");
    }

    /// Yalnızca testte: açılır seçimli satır (kayıtta henüz `Dropdown` kullanan ayar yok).
    static DROPDOWN: SettingDesc = SettingDesc {
        id: "test.dropdown",
        page: PageId::Motion,
        section: "Test",
        title: "Eğri",
        subtitle: "Hareketin hız profili",
        keywords: &[],
        file: FileId::Framework,
        path: "motion.spatial.easing",
        control: Control::Choice { options: EASINGS, style: ChoiceStyle::Dropdown },
        visible_if: None,
        tier: Tier::Basic,
        mirror_on: &[],
        ui_if: None,
    };

    static SLIDER: SettingDesc = SettingDesc {
        id: "test.slider",
        page: PageId::Bar,
        section: "Test",
        title: "Bar kalınlığı",
        subtitle: "Bar'ın kenarındaki çerçeve de bu kalınlıktadır",
        keywords: &[],
        file: FileId::Shell,
        path: "bar.size",
        control: Control::Slider { min: 16.0, max: 64.0, step: 1.0, unit: "px", integer: true, percent: false },
        visible_if: None,
        tier: Tier::Basic,
        mirror_on: &[],
        ui_if: None,
    };

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    fn key(k: Key) -> KeyEvent {
        KeyEvent::new(k, KeyState::Pressed, None, Modifiers::default())
    }

    #[test]
    fn dropdown_menu_escapes_row_clip_and_writes_choice() {
        let (_d, store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let mut row = SettingRow::new(&theme, &DROPDOWN, &store).unwrap();
        assert!(matches!(row.ctl, Ctl::Dropdown(_)));
        let row_rect;
        {
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            let h = row.height(&mut env, 640.0);
            row_rect = Rect::new(0.0, 100.0, 640.0, h);
            row.place(&mut env, row_rect);
            assert!(!row.overlay_open());
            assert!(row.key(&mut env, &key(Key::Enter)), "Enter açar");
        }
        assert!(row.overlay_open() && row.captures_keys(), "açıkken tuşlar ve işaretçi satırın");

        // Satırın kırpması içinde çizilir; menü kaplamada, satırın altına taşar.
        let mut rec = Recorder::new();
        let later = now + std::time::Duration::from_secs(5);
        let mut regions = nuxut::ui::OverlayRegions::default();
        rec.record(&mut text, (800, 600), 1.0, |p| {
            let mut ui = Ui::new(p, &theme, later);
            ui.painter.push_clip(row_rect);
            row.draw(&mut ui);
            ui.painter.pop_clip();
            regions = ui.take_overlay_regions();
        });
        assert_eq!(regions.rects().len(), 1, "menünün girdi bölgesi bildirildi");
        let menu = regions.rects()[0];
        assert!(menu.bottom() > row_rect.bottom(), "menü satırdan taşar: {menu:?}");
        let below = Point::new(menu.center().x, menu.bottom() - 4.0);
        assert!(
            rec.draws().iter().any(|d| d.kind == DrawKind::Blob && d.rect.contains(below) && d.clip.is_none_or(|c| c.bottom() > row_rect.bottom() as i32)),
            "satır kırpmasının dışındaki menü kısmı çizilir"
        );

        // Aşağı + Enter: başka bir seçenek yazılır, menü kapanır (açılışta vurgu seçilide ya da
        // hiçbirinde olabilir; iki adım kesin başka seçenek).
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: later, actions: &mut actions };
        assert!(row.key(&mut env, &key(Key::Down)));
        assert!(row.key(&mut env, &key(Key::Down)));
        assert!(row.key(&mut env, &key(Key::Enter)));
        assert!(!row.overlay_open());
        let Ctl::Dropdown(sel) = &row.ctl else { unreachable!() };
        assert_ne!(sel.selected(), 0);
        let want = DROPDOWN.choice_value(sel.selected(), &store);
        let wrote = actions.iter().any(|a| matches!(a, Action::Set { path, value, .. } if *path == DROPDOWN.key() && value.as_ref().map(|v| v.to_string()) == want.as_ref().map(|v| v.to_string())));
        assert!(wrote, "seçim yazıldı: {actions:?}");
    }

    #[test]
    fn narrow_row_stacks_control_below_title() {
        let (_d, store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut row = SettingRow::new(&theme, &SLIDER, &store).unwrap();

        // Geniş: kontrol sağda, başlıkla aynı satırda.
        let wide = theme.space(180);
        let h = row.height(&mut env, wide);
        row.place(&mut env, Rect::new(0.0, 0.0, wide, h));
        let Ctl::Slider { slider, .. } = &row.ctl else { unreachable!() };
        let s = slider.rect();
        assert!(s.x > wide * 0.4, "geniş düzende kaydırıcı sağda: {s:?}");
        assert!(!row.stack);

        // Dar: başlığın altında, neredeyse tam genişlik; satır daha yüksek.
        let narrow = theme.space(120);
        assert!(row.stacked(&mut env, narrow));
        let hn = row.height(&mut env, narrow);
        row.place(&mut env, Rect::new(0.0, 0.0, narrow, hn));
        let Ctl::Slider { slider, .. } = &row.ctl else { unreachable!() };
        let s = slider.rect();
        let (px, py) = row_padding(&theme);
        assert!((s.x - px).abs() < 0.5, "dar düzende kaydırıcı soldan başlar: {s:?}");
        assert!(s.width > narrow * 0.6, "dar düzende geniş: {s:?}");
        let title_bottom = py + row.title.measure(env.text).height;
        assert!(s.y >= title_bottom, "kaydırıcı başlığın altında: {s:?}");
        assert!(s.bottom() <= hn, "satırın içinde");
    }

    #[test]
    fn toggle_rows_never_stack() {
        let (_d, store) = store();
        let theme = Theme::default();
        let desc = crate::registry::by_id("frame.enabled").unwrap();
        let mut row = SettingRow::new(&theme, desc, &store).unwrap();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        assert!(!row.stacked(&mut env, theme.space(100)), "anahtar dar düzende de yerinde kalır");
    }

    /// Uzun başlıklı segmentli satır: genişlik eşiğinin üstünde de başlık kontrolün yanına
    /// sığmıyorsa kontrol alta iner; metin ile kontrol üst üste binmez.
    static LONG_SEGMENTED: SettingDesc = SettingDesc {
        id: "test.long",
        page: PageId::Motion,
        section: "Test",
        title: "Paneller ve pencereler için hareket türünün uzun ve açıklayıcı başlığı",
        subtitle: "Yay doğal ve kesilebilir; eğri sabit sürelidir, her açılışta aynı uzunlukta oynar",
        keywords: &[],
        file: FileId::Framework,
        path: "wallpaper.fit",
        control: Control::Choice { options: crate::registry::WALLPAPER_FITS, style: ChoiceStyle::Segmented },
        visible_if: None,
        tier: Tier::Basic,
        mirror_on: &[],
        ui_if: None,
    };

    #[test]
    fn crowded_row_stacks_above_narrow_threshold() {
        let (_d, store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut row = SettingRow::new(&theme, &LONG_SEGMENTED, &store).unwrap();
        assert!(matches!(row.ctl, Ctl::Segmented(_)), "3 seçenek segmentli");
        let w = theme.space(150);
        assert!(!is_narrow(&theme, w));
        let h = row.height(&mut env, w);
        row.place(&mut env, Rect::new(0.0, 0.0, w, h));
        assert!(row.stack, "başlık kontrolün yanına sığmıyor: yığılır");
        let Ctl::Segmented(seg) = &row.ctl else { unreachable!() };
        let s = seg.rect();
        let (px, py) = row_padding(&theme);
        let title = row.title.measure(env.text);
        let sub = row.subtitle.as_mut().unwrap().measure(env.text);
        assert!(title.width <= stacked_text_width(&theme, w) + 0.5, "başlık sütuna sarılır");
        assert!(s.y >= py + title.height + theme.space(1) + sub.height, "kontrol metnin altında: {s:?}");
        assert!(s.bottom() <= h + 0.5 && s.right() <= w - px + 0.5, "satırın içinde: {s:?}");

        // Çok geniş sütunda aynı satır yan yana kalır.
        let wide = theme.space(400);
        let h = row.height(&mut env, wide);
        row.place(&mut env, Rect::new(0.0, 0.0, wide, h));
        assert!(!row.stack);
    }

    #[test]
    fn percent_values_use_framework_percent() {
        let tr = nuxut::locale::Locale::parse("tr_TR.UTF-8").unwrap();
        let en = nuxut::locale::Locale::parse("en_US.UTF-8").unwrap();
        // Birimi "%" olan (değer zaten yüzde: tekerlek adımı, en yüksek ses) ve kesir yüzdesi
        // aynı tek biçimden geçer; "5 %" gibi boşluklu birim yazılmaz.
        let step = crate::registry::by_id("bar.edge_actions.scroll_step").unwrap();
        let vol = crate::registry::by_id("audio.max_volume").unwrap();
        let opacity = crate::registry::by_id("shadow.opacity").unwrap();
        for l in [&tr, &en] {
            assert_eq!(format_number_in(step, 5.0, l), crate::numfmt::percent_in(l, 0.05, 0));
            assert_eq!(format_number_in(vol, 150.0, l), crate::numfmt::percent_in(l, 1.5, 0));
            assert_eq!(format_number_in(opacity, 0.35, l), crate::numfmt::percent_in(l, 0.35, 0));
            assert!(!format_number_in(step, 5.0, l).contains(' '));
        }
    }

    #[test]
    fn schema_unit_symbols_map_to_catalog_names() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(unit_label("px"), "px");
        assert_eq!(unit_label("ms"), "ms");
        // Yeni kapalı küme "s"; eski shell'in "sn"si aynı ada düşer.
        assert_eq!(unit_label("s"), "sn");
        assert_eq!(unit_label("sn"), "sn");
        // Bilinmeyen sembol ve Ayarlar'ın yerel birimleri olduğu gibi.
        assert_eq!(unit_label("tuş/sn"), "tuş/sn");
        assert_eq!(unit_label("×"), "×");
    }
}
