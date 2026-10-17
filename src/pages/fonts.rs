//! Yazı sayfası (ADR-0006): rol başına yazı tipi seçici, canlı tipografi önizlemesi ve ikon
//! seti seçimi. Ölçek satırları kayıttaki standart kaydırıcılardır; kalınlık seçicidedir.
//!
//! - [`TypePreview`]: her rolün örneği o anki temanın rol stiliyle (canlı).
//! - [`IconSetPicker`]: iki set yan yana, aynı ikonlarla; tıklanan set seçilir.
//! - [`FontFamilyPicker`]: rolün ailesi kendi fontuyla; kurulu değilse yedek bilgisi ve Arch
//!   paketi; açılınca framework `FontPicker` (aile + kalınlık, her aile kendi fontuyla).

use std::time::Instant;

use nuxut::render::{FamilyInfo, GenericFamily, TextStyle};
use nuxut::theme::IconSet;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::HAIRLINE;
use nuxut::ui::{
    Button, ButtonVariant, FontEvent, FontPicker, FontPickerLabels, Icon, IconName, IconView, Interaction, Label, TextInput, TextRole, Ui, text_style,
};
use nuxut::{Border, Point, Rect, RectStyle, TextSystem, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::i18n::text_or;
use nuxut::i18n::t;
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::store::{FileId, KeyPath, Store};
use crate::typography::{self, FontRole};

/// Önizlemede ve set kartlarında gösterilen ikonlar.
const SAMPLE_ICONS: [IconName; 8] = [
    IconName::Wifi,
    IconName::VolumeHigh,
    IconName::Bluetooth,
    IconName::Battery80,
    IconName::Notifications,
    IconName::CalendarMonth,
    IconName::Settings,
    IconName::Power,
];


fn role_text(role: FontRole) -> TextRole {
    match role {
        FontRole::Ui => TextRole::Body,
        FontRole::Display => TextRole::Display,
        FontRole::Document => TextRole::Document,
        FontRole::Mono => TextRole::Mono,
        FontRole::Icon => TextRole::Icon,
    }
}

fn generic(role: FontRole) -> GenericFamily {
    match role {
        FontRole::Document => GenericFamily::Serif,
        FontRole::Mono => GenericFamily::Monospace,
        _ => GenericFamily::SansSerif,
    }
}

/// Satırdaki "kurulu değil" bilgisi.
fn missing_text(family: &str, used: Option<&str>) -> String {
    let used = used.filter(|u| !u.is_empty());
    match (used, typography::arch_package(family)) {
        (Some(u), Some(p)) => t!("settings-fonts-missing-used-package", used = u, package = p),
        (Some(u), None) => t!("settings-fonts-missing-used", used = u),
        (None, Some(p)) => t!("settings-fonts-missing-fallback-package", package = p),
        (None, None) => t!("settings-fonts-missing-fallback"),
    }
}

// ---- Önizleme ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct TypePreview {
    desc: &'static SettingDesc,
    rect: Rect,
    lines: Vec<(Label, Label)>,
    icons: Vec<IconView>,
}

impl TypePreview {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, _store: &Store) -> Self {
        let lines = [FontRole::Display, FontRole::Ui, FontRole::Document, FontRole::Mono]
            .into_iter()
            .map(|r| (Label::new(r.title(), text_style(theme, TextRole::Caption)), Label::new(r.sample(), text_style(theme, role_text(r)))))
            .collect();
        Self { desc, rect: Rect::ZERO, lines, icons: icon_views(theme) }
    }
}

fn icon_views(theme: &Theme) -> Vec<IconView> {
    SAMPLE_ICONS.iter().map(|n| IconView::with_role(theme, Icon::named(*n), TextRole::IconLarge)).collect()
}

impl Block for TypePreview {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let mut h = py;
        for (cap, line) in &mut self.lines {
            line.set_max_width(Some(width - px * 2.0));
            h += cap.measure(env.text).height + line.measure(env.text).height + t.space(3);
        }
        let ih = self.icons.first_mut().map_or(0.0, |i| i.measure(env.text).height);
        h + ih + py
    }

    fn place(&mut self, _env: &mut Env, rect: Rect) {
        self.rect = rect;
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        let roles = [FontRole::Display, FontRole::Ui, FontRole::Document, FontRole::Mono];
        for ((cap, line), r) in self.lines.iter_mut().zip(roles) {
            cap.set_style(text_style(theme, TextRole::Caption));
            line.set_style(text_style(theme, role_text(r)));
        }
        // İkon seti/fontu değişmiş olabilir: yeniden kur.
        self.icons = icon_views(theme);
        let _ = now;
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let mut y = self.rect.y + py;
        let x = self.rect.x + px;
        for (cap, line) in &mut self.lines {
            let ch = cap.measure_in(ui.painter).height;
            cap.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface_variant));
            y += ch;
            let lh = line.measure_in(ui.painter).height;
            line.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
            y += lh + t.space(3);
        }
        let mut ix = x;
        for icon in &mut self.icons {
            let s = icon.measure(ui.painter.text_system());
            icon.draw_centered(ui.painter, Point::new(ix + s.width * 0.5, y + s.height * 0.5), ui.fade(c.on_surface));
            ix += s.width + t.space(4);
        }
    }
}

// ---- İkon seti ---------------------------------------------------------------------------

#[derive(Debug)]
struct SetCard {
    set: IconSet,
    title: Label,
    detail: Label,
    icons: Vec<IconView>,
    hover: Interaction,
    rect: Rect,
}

#[derive(Debug)]
pub struct IconSetPicker {
    desc: &'static SettingDesc,
    rect: Rect,
    cards: Vec<SetCard>,
    selected: IconSet,
    /// Kurulu-mu bilgisi bir sonraki ölçümde yenilenir.
    dirty: bool,
}

const SETS: [(IconSet, &str, &str); 2] = [(IconSet::Material, "material", "Material Symbols"), (IconSet::Nerd, "nerd", "Nerd Font")];

fn set_key(set: IconSet) -> &'static str {
    SETS.iter().find(|(s, _, _)| *s == set).map_or("material", |(_, k, _)| k)
}

/// Seçili set (config ya da varsayılan).
fn current_set(desc: &SettingDesc, store: &Store) -> IconSet {
    let v = desc.string(store);
    SETS.iter().find(|(_, k, _)| *k == v).map_or(IconSet::Material, |(s, _, _)| *s)
}

/// Bu setin ikonlarını çizen tema: set seçili, ikon ailesi setin kendi fontu.
fn theme_for(theme: &Theme, set: IconSet) -> Theme {
    let mut t = theme.clone();
    t.typography.icon_set = set;
    t.typography.icon.family.clear();
    t
}

impl IconSetPicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let cards = SETS
            .iter()
            .map(|(set, _, title)| SetCard {
                set: *set,
                title: Label::new(*title, text_style(theme, TextRole::Label)),
                detail: Label::new("", text_style(theme, TextRole::Caption)),
                icons: set_icons(theme, *set),
                hover: Interaction::new(theme),
                rect: Rect::ZERO,
            })
            .collect();
        Self { desc, rect: Rect::ZERO, cards, selected: current_set(desc, store), dirty: true }
    }

    fn refresh_details(&mut self, text: &mut TextSystem, theme: &Theme) {
        for card in &mut self.cards {
            let t = theme_for(theme, card.set);
            let family = t.typography.icon_family().to_owned();
            let detail = if text.has_family(&family) { family } else { missing_text(&family, None) };
            card.detail.set_text(&detail);
        }
        self.dirty = false;
    }

    fn card_h(theme: &Theme) -> f32 {
        theme.space(26)
    }
}

fn set_icons(theme: &Theme, set: IconSet) -> Vec<IconView> {
    let t = theme_for(theme, set);
    SAMPLE_ICONS.iter().map(|n| IconView::with_role(&t, Icon::named(*n), TextRole::Icon)).collect()
}

impl Block for IconSetPicker {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, _width: f32) -> f32 {
        if self.dirty {
            self.refresh_details(env.text, env.theme);
        }
        let (_, py) = row_padding(env.theme);
        Self::card_h(env.theme) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let gap = t.space(3);
        let w = (rect.width - px * 2.0 - gap) * 0.5;
        for (i, card) in self.cards.iter_mut().enumerate() {
            card.rect = Rect::new(rect.x + px + i as f32 * (w + gap), rect.y + py, w, Self::card_h(t));
            card.detail.set_max_width(Some(w - t.space(8)));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        let mut chosen = None;
        for card in &mut self.cards {
            let tap = card.hover.handle(card.rect, e, env.now);
            repaint |= tap.repaint;
            if tap.clicked {
                chosen = Some(card.set);
            }
            if e.kind == PointerKind::Leave {
                repaint |= card.hover.reset(env.now);
            }
        }
        if let Some(set) = chosen
            && set != self.selected
        {
            self.selected = set;
            // Varsayılan set seçilince anahtar silinir.
            let is_default = self.desc.default_value(env.store).and_then(Value::as_str) == Some(set_key(set));
            let value = (!is_default).then(|| Value::from(set_key(set)));
            env.set(text_or("settings-typography-icon-set-label", "İkon seti"), FileId::Framework, self.desc.key(), value);
            repaint = true;
        }
        repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.cards.iter().any(|c| c.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.selected = current_set(self.desc, env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for card in &mut self.cards {
            card.title.set_style(text_style(theme, TextRole::Label));
            card.detail.set_style(text_style(theme, TextRole::Caption));
            card.hover.set_theme(theme, now);
            card.icons = set_icons(theme, card.set);
        }
        self.dirty = true;
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        for card in &mut self.cards {
            let r = card.rect;
            let selected = card.set == self.selected;
            let border = if selected { Border { width: 2.0, color: ui.fade(c.primary) } } else { Border { width: HAIRLINE, color: ui.fade(c.outline) } };
            ui.painter.rect(r, &RectStyle { fill: ui.fade(c.surface), radii: t.shape.medium(), border: Some(border) });
            card.hover.draw_layer(ui.painter, r, t.shape.medium(), t, now);
            if card.hover.is_animating(now) {
                ui.request_frame();
            }
            let pad = t.space(4);
            let th = card.title.measure_in(ui.painter).height;
            card.title.draw(ui.painter, Point::new(r.x + pad, r.y + pad), ui.fade(if selected { c.primary } else { c.on_surface }));
            card.detail.draw(ui.painter, Point::new(r.x + pad, r.y + pad + th), ui.fade(c.on_surface_variant));
            // İkon şeridi kartın altında, eşit aralıklı.
            let n = card.icons.len() as f32;
            let step = (r.width - pad * 2.0) / n;
            let cy = r.bottom() - pad - t.space(4);
            for (i, icon) in card.icons.iter_mut().enumerate() {
                icon.draw_centered(ui.painter, Point::new(r.x + pad + step * (i as f32 + 0.5), cy), ui.fade(c.on_surface));
            }
        }
    }
}

// ---- Yazı tipi seçici --------------------------------------------------------------------

/// CSS ağırlık adları (100 … 900); katalogdaki settings-fonts-weight-* anahtarları.
const WEIGHT_KEYS: [&str; 9] = [
    "settings-fonts-weight-thin",
    "settings-fonts-weight-extra-light",
    "settings-fonts-weight-light",
    "settings-fonts-weight-regular",
    "settings-fonts-weight-medium",
    "settings-fonts-weight-semi-bold",
    "settings-fonts-weight-bold",
    "settings-fonts-weight-extra-bold",
    "settings-fonts-weight-black",
];

#[derive(Debug)]
pub struct FontFamilyPicker {
    desc: &'static SettingDesc,
    role: FontRole,
    rect: Rect,
    head: Rect,
    title: Label,
    sample: Label,
    status: Label,
    chevron: IconView,
    head_hover: Interaction,
    /// Açıkken framework seçicisi (aile listesi açılışta kurulur, kapanınca bırakılır).
    picker: Option<FontPicker>,
    reset: Button,
    /// Gösterilen aile ve kurulu mu; ölçümde yenilenir.
    shown: Option<(String, bool)>,
    /// Rolün bir anahtarı dosyada yazılı mı (sıfırlama düğmesi).
    role_set: bool,
    focused: bool,
    /// Test senaryosu açılış istedi; aile listesi Env ister, ilk ölçümde kurulur.
    demo_open: bool,
}

impl FontFamilyPicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, role: FontRole, _store: &Store) -> Self {
        Self {
            desc,
            role,
            rect: Rect::ZERO,
            head: Rect::ZERO,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            sample: Label::new("", text_style(theme, TextRole::Body)),
            status: Label::new("", text_style(theme, TextRole::Caption)),
            chevron: IconView::new(theme, Icon::named(IconName::ChevronDown)),
            head_hover: Interaction::new(theme),
            picker: None,
            reset: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Restart)).with_label(theme, t!("settings-fonts-reset")),
            shown: None,
            role_set: false,
            focused: false,
            demo_open: false,
        }
    }

    /// Rolün geçerli ailesi; ikon rolünde boşsa setin fontu.
    fn family(&self, store: &Store, theme: &Theme) -> (String, bool) {
        let f = self.desc.string(store);
        if self.role == FontRole::Icon && f.is_empty() {
            return (theme.typography.icon_family().to_owned(), true);
        }
        (f, false)
    }

    fn weight_key(&self) -> KeyPath {
        KeyPath::new(["typography", self.role.key(), "weight"])
    }

    /// Rolün geçerli kalınlığı (dosya, yoksa framework varsayılanı).
    fn weight(&self, store: &Store) -> u16 {
        let key = self.weight_key();
        store.get(FileId::Framework, &key).or_else(|| crate::defaults::get(store, FileId::Framework, &key)).and_then(Value::as_integer).map_or(400, |w| w.clamp(1, 1000) as u16)
    }

    /// Başlık satırı: aile adı kendi fontuyla, kurulu değilse yedek bilgisi.
    fn refresh_head(&mut self, env: &mut Env) {
        self.role_set = self.role_is_set(env.store);
        let (family, from_set) = self.family(env.store, env.theme);
        let installed = env.text.has_family(&family);
        if self.shown.as_ref() == Some(&(family.clone(), installed)) {
            return;
        }
        self.sample.set_style(self.family_style(env.theme, &family, 16.0));
        self.sample.set_text(&family);
        let status = if !installed {
            let used = env.text.effective_family(&family, generic(self.role));
            missing_text(&family, Some(&used))
        } else if from_set {
            t!("settings-fonts-icon-set-font")
        } else if self.desc.is_set(env.store) {
            String::new()
        } else {
            t!("settings-fonts-default")
        };
        self.status.set_text(&status);
        self.shown = Some((family, installed));
    }

    fn family_style(&self, theme: &Theme, family: &str, size: f32) -> TextStyle {
        let weight = if self.role == FontRole::Icon { theme.typography.ui.weight } else { 400 };
        // İkon fontunun adı ikon ligatürü olarak çizilmesin: ikon rolünde arayüz fontu.
        let family = if self.role == FontRole::Icon { theme.typography.ui.family.as_str() } else { family };
        TextStyle::new(family, size, weight).with_fallback(generic(self.role))
    }

    /// Seçiciyi rolün listesiyle kurar (ikon rolünde seçili setin fontları).
    fn open(&mut self, env: &mut Env) {
        let set = env.theme.typography.icon_set;
        let role = self.role;
        let families: Vec<FamilyInfo> = env.text.families().iter().filter(|f| typography::listed(f, role, set)).cloned().collect();
        let labels = FontPickerLabels {
            search: t!("settings-fonts-search-placeholder"),
            all: t!("settings-fonts-filter-all"),
            monospace: t!("settings-section-mono"),
            sample: sample_for(role, set).into(),
            weight_names: WEIGHT_KEYS.map(|k| nuxut::i18n::current().text(k).into_owned()),
        };
        let (family, _) = self.family(env.store, env.theme);
        // Eş aralıklı ve ikon rollerinde liste zaten süzülü: "Tümü / Eş aralıklı" anlamsız.
        let filter = !matches!(role, FontRole::Mono | FontRole::Icon);
        let mut picker = FontPicker::new(env.theme, &families, labels, Some((&family, self.weight(env.store)))).with_filter(env.theme, filter);
        picker.search_mut().set_focused(self.focused, env.now);
        self.picker = Some(picker);
        self.chevron.set_icon(Icon::named(IconName::ChevronUp));
    }

    fn close(&mut self) {
        self.picker = None;
        self.chevron.set_icon(Icon::named(IconName::ChevronDown));
    }

    /// Seçilen aile ve kalınlık tek geri alma adımı; varsayılana eşit değer anahtarı siler.
    fn choose(&mut self, env: &mut Env, family: String, weight: u16) {
        let store = env.store;
        let fkey = self.desc.key();
        let wkey = self.weight_key();
        let fdef = crate::defaults::get(store, FileId::Framework, &fkey).and_then(Value::as_str).unwrap_or("");
        // İkon rolünde boş varsayılan "setin fontu" demek.
        let family_default = family == fdef || (self.role == FontRole::Icon && fdef.is_empty() && family == env.theme.typography.icon_family());
        let wdef = crate::defaults::get(store, FileId::Framework, &wkey).and_then(Value::as_integer);
        let writes = vec![
            (FileId::Framework, fkey, (!family_default).then(|| Value::from(family))),
            (FileId::Framework, wkey, (wdef != Some(i64::from(weight))).then(|| Value::from(i64::from(weight)))),
        ];
        env.push(Action::Group { label: t!("settings-fonts-role-family", role = self.role.title()), writes });
    }

    fn reset_keys(&self) -> Vec<(FileId, KeyPath, Option<Value>)> {
        ["family", "weight", "scale"].iter().map(|f| (FileId::Framework, KeyPath::new(["typography", self.role.key(), f]), None)).collect()
    }

    fn role_is_set(&self, store: &Store) -> bool {
        self.reset_keys().iter().any(|(f, k, _)| store.get(*f, k).is_some())
    }

    /// Seçicinin yüksekliği: liste birkaç satır gösterecek kadar, sayfayı doldurmadan.
    fn picker_h(theme: &Theme) -> f32 {
        FontPicker::preferred_size(theme).height.min(theme.space(110))
    }
}

/// Seçici satırlarının ve önizlemesinin örneği. Material ikon setinde ligatür adları
/// (ikon olarak çizilir).
fn sample_for(role: FontRole, set: IconSet) -> &'static str {
    match (role, set) {
        (FontRole::Mono, _) => "0O 1lI {} => != 0x1F",
        (FontRole::Icon, IconSet::Material) => "wifi volume_up bluetooth settings",
        (FontRole::Icon, IconSet::Nerd) => "Aa 0123",
        _ => text_or("settings-fonts-sample-text", "Ağaç Şiir Ğğ İı 0123"),
    }
}

impl Block for FontFamilyPicker {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, _width: f32) -> f32 {
        self.refresh_head(env);
        if std::mem::take(&mut self.demo_open) && self.picker.is_none() {
            self.open(env);
        }
        let t = env.theme;
        let (_, py) = row_padding(t);
        let th = self.title.measure(env.text).height;
        let sh = if self.status.text().is_empty() { 0.0 } else { self.status.measure(env.text).height };
        let head = (th + sh).max(self.sample.measure(env.text).height).max(nuxut::ui::style::row_height(t) - py * 2.0);
        let mut h = py + head + py;
        if self.picker.is_some() {
            h += Self::picker_h(t) + py;
            if self.role_set {
                h += t.space(2) + self.reset.preferred_size(env.text, t).height;
            }
        }
        h
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let th = self.title.measure(env.text).height;
        let sh = if self.status.text().is_empty() { 0.0 } else { self.status.measure(env.text).height };
        let head = (th + sh).max(self.sample.measure(env.text).height).max(nuxut::ui::style::row_height(t) - py * 2.0);
        self.head = Rect::new(rect.x, rect.y, rect.width, head + py * 2.0);
        self.status.set_max_width(Some(rect.width * 0.6));
        self.sample.set_max_width(Some(rect.width * 0.4));
        let Some(picker) = &mut self.picker else { return };
        let area = Rect::new(rect.x + px, self.head.bottom(), rect.width - px * 2.0, Self::picker_h(t));
        if picker.rect() != area {
            picker.set_rect(env.text, t, area, env.now);
        }
        let rs = self.reset.preferred_size(env.text, t);
        self.reset.set_rect(Rect::new(rect.x + px, area.bottom() + t.space(2), rs.width, rs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let tap = self.head_hover.handle(self.head, e, now);
        repaint |= tap.repaint;
        if e.kind == PointerKind::Leave {
            repaint |= self.head_hover.reset(now);
        }
        if tap.clicked {
            if self.picker.is_some() {
                self.close();
            } else {
                self.open(env);
            }
            return true;
        }
        let Some(picker) = &mut self.picker else { return repaint };
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && picker.rect().contains(e.position)
        {
            env.push(Action::Focus);
        }
        let r = picker.pointer(env.text, env.theme, e, now);
        repaint |= r.repaint;
        if let Some(FontEvent::Changed { family, weight }) = r.event {
            self.choose(env, family, weight);
            return true;
        }
        if self.role_set {
            let r = self.reset.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                env.push(Action::Group { label: t!("settings-fonts-role-reset", role = self.role.title()), writes: self.reset_keys() });
                return true;
            }
        }
        repaint
    }

    fn wants_scroll(&self, p: Point) -> bool {
        self.picker.as_ref().is_some_and(|f| f.rect().contains(p))
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }
        let Some(picker) = &mut self.picker else { return false };
        if e.state == KeyState::Pressed && e.key == Key::Escape && picker.search_mut().text().is_empty() {
            self.close();
            return true;
        }
        let r = picker.key(env.text, env.theme, e, env.now);
        if let Some(FontEvent::Changed { family, weight }) = r.event {
            self.choose(env, family, weight);
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        self.focused = focused;
        if let Some(p) = &mut self.picker {
            p.search_mut().set_focused(focused, now);
        }
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        self.picker.as_mut().map(|p| p.search_mut().input_mut())
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.head.contains(p) {
            return Some(CursorShape::Pointer);
        }
        let picker = self.picker.as_ref()?;
        picker.cursor(p).or_else(|| if self.role_set { self.reset.cursor(p) } else { None })
    }

    fn demo_expand(&mut self, _now: Instant) {
        self.demo_open = true;
    }

    fn sync(&mut self, env: &mut Env) {
        self.shown = None;
        self.refresh_head(env);
        // Dış değişiklik (geri al, başka pencere) seçiciye akar; olay üretmez.
        let (family, _) = self.family(env.store, env.theme);
        let weight = self.weight(env.store);
        if let Some(p) = &mut self.picker
            && p.selected() != Some((family.as_str(), weight))
        {
            p.set_selected(env.theme, &family, weight, env.now);
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.status.set_style(text_style(theme, TextRole::Caption));
        self.chevron.set_theme(theme, now);
        self.head_hover.set_theme(theme, now);
        self.reset.set_theme(theme, now);
        if let Some(p) = &mut self.picker {
            p.set_theme(theme, now);
        }
        // Başlık örneği temaya bağlı (ikon rolünde arayüz fontu): yeniden kurulsun.
        self.shown = None;
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let head = self.head;
        self.head_hover.draw_layer(ui.painter, head, t.shape.medium(), t, now);
        if self.head_hover.is_animating(now) {
            ui.request_frame();
        }
        let th = self.title.measure_in(ui.painter).height;
        let sh = if self.status.text().is_empty() { 0.0 } else { self.status.measure_in(ui.painter).height };
        let top = head.center().y - (th + sh) * 0.5;
        self.title.draw(ui.painter, Point::new(head.x + px, top), ui.fade(c.on_surface));
        let missing = self.shown.as_ref().is_some_and(|(_, ok)| !ok);
        self.status.draw(ui.painter, Point::new(head.x + px, top + th), ui.fade(if missing { c.error } else { c.on_surface_variant }));
        let cs = self.chevron.measure(ui.painter.text_system());
        let cx = head.right() - px - cs.width * 0.5;
        self.chevron.draw_centered(ui.painter, Point::new(cx, head.center().y), ui.fade(c.on_surface_variant));
        let ss = self.sample.measure_in(ui.painter);
        self.sample.draw(ui.painter, Point::new(cx - cs.width * 0.5 - t.space(3) - ss.width, head.center().y - ss.height * 0.5), ui.fade(c.on_surface));
        let Some(picker) = &mut self.picker else { return };
        picker.draw(ui);
        if self.role_set {
            self.reset.draw(ui);
        }
    }
}
