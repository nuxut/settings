//! Bar sayfasının hero'su ve "Seçili widget" kartının başlığı (`docs/design/settings-ux.md`
//! §6.1 öneri A).
//!
//! - [`BarPreview`]: masaüstünün küçük resmi; bar config'teki kenarda, widget hapları bar'daki
//!   sırasıyla (baş/orta/son bölüm). Haba tıklamak (ya da önizleme odaktayken oklar) widget'ı
//!   seçer. Adlar sığmayınca haplar yalnızca ikon olur (§4); dikey bar'da haplar zaten ikon.
//!   Uç eylemi varsayılandan farklı olan uçtaki hapta küçük nokta.
//! - [`SelectedWidget`]: kartın başlığı (ikon, ad, bar'daki yeri) ve şemada olup kayıtta satırı
//!   olmayan widget ayarları (`WidgetSpec.settings`'ten dinamik satırlar). Kayıttaki widget
//!   ayarları ve uç eylemleri aynı kartta kendi satırlarıdır; görünürlükleri seçime bağlı
//!   (`registry` `ui_if`).
//!
//! Seçim sayfa durumundadır (`PageUi::selected_widget`; `None` = sağ uç, karar 4): config'e,
//! geçmişe ya da geri al'a girmez. Uç: yatay bar'da en sol/en sağ, dikeyde en üst/en alt widget
//! — shell gibi bölüm bazında (`bar.start`'ın ilki, `bar.end`'in sonuncusu; bölüm boşsa o uçta
//! eylem yok).

use std::sync::Mutex;
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{FOCUS_RING, TONAL_MIX, accent_on, focus_ring, pill, tinted, tonal_container};
use nuxut::ui::{Icon, IconName, IconView, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, Size, Theme};
use toml_edit::Value;

use crate::block::{Block, Env};
use crate::page_ui::PageUi;
use crate::pages::edges::{Edge, bar_edge, wallpaper_tones};
use crate::pages::sections::{SECTIONS, read_list, widget_info};
use crate::registry::{self, ChoiceStyle, Control, PageId, SettingDesc, Tier, by_id};
use crate::rows::{SettingRow, row_padding};
use crate::services::widgets::{self, SettingKind, SettingSpec, WidgetSpec};
use crate::store::{FileId, KeyPath, Store};

// ---- Seçim ---------------------------------------------------------------------------------

/// Bar'ın iki ucu (shell `bar_edge::EdgeSide`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarEnd {
    /// Yatay: en sol, dikey: en üst.
    Start,
    /// Yatay: en sağ, dikey: en alt.
    End,
}

impl BarEnd {
    pub const ALL: [BarEnd; 2] = [BarEnd::Start, BarEnd::End];

    /// `[bar.edge_actions.<key>]`.
    pub fn key(self) -> &'static str {
        match self {
            BarEnd::Start => "start",
            BarEnd::End => "end",
        }
    }
}

/// Bar'ın üç bölümündeki widget'lar (config, yoksa varsayılan).
pub fn bar_lists(store: &Store) -> [Vec<String>; 3] {
    SECTIONS.map(|k| read_list(store, &KeyPath::new(["bar", k])))
}

/// Uçtaki widget: baş bölümün ilki ya da son bölümün sonuncusu (bölüm boşsa yok).
pub fn end_widget(store: &Store, end: BarEnd) -> Option<String> {
    let [start, _, last] = bar_lists(store);
    match end {
        BarEnd::Start => start.into_iter().next(),
        BarEnd::End => last.into_iter().next_back(),
    }
}

/// Seçim yokken seçili sayılan widget (karar 4): sağ uç; yoksa sol uç; yoksa ilk widget.
pub fn default_selection(store: &Store) -> Option<String> {
    end_widget(store, BarEnd::End).or_else(|| end_widget(store, BarEnd::Start)).or_else(|| bar_lists(store).into_iter().flatten().next())
}

/// Seçili widget'ın adı (sayfa durumu, yoksa varsayılan).
pub fn selected(ui: &PageUi, store: &Store) -> Option<String> {
    ui.selected_widget.clone().or_else(|| default_selection(store))
}

/// Seçili widget bar'ın bir ucunda mı.
pub fn selected_end(ui: &PageUi, store: &Store) -> Option<BarEnd> {
    let name = selected(ui, store)?;
    BarEnd::ALL.into_iter().find(|e| end_widget(store, *e).as_deref() == Some(name.as_str()))
}

fn vertical(store: &Store) -> bool {
    matches!(bar_edge(store), Edge::Left | Edge::Right)
}

/// Bu ucun tık/tekerlek eylemlerinden biri varsayılandan farklı mı (önizlemedeki nokta).
fn end_customized(store: &Store, end: BarEnd) -> bool {
    ["left_click", "right_click", "scroll"]
        .iter()
        .any(|a| by_id(&format!("bar.edge_actions.{}.{a}", end.key())).is_some_and(|d| d.differs_from_default(store)))
}

/// Widget'ın bar'daki yerinin metni (kart başlığının altı). Hitapsız (karar 2).
fn location_text(store: &Store, name: &str) -> String {
    let lists = bar_lists(store);
    let v = vertical(store);
    let Some(section) = lists.iter().position(|l| l.iter().any(|w| w == name)) else {
        return t!("settings-bar-preview-location-not-in-bar");
    };
    let at_end = BarEnd::ALL.into_iter().find(|e| end_widget(store, *e).as_deref() == Some(name));
    match (at_end, v) {
        (Some(BarEnd::Start), false) => t!("settings-bar-preview-location-left-end"),
        (Some(BarEnd::Start), true) => t!("settings-bar-preview-location-top-end"),
        (Some(BarEnd::End), false) => t!("settings-bar-preview-location-right-end"),
        (Some(BarEnd::End), true) => t!("settings-bar-preview-location-bottom-end"),
        (None, _) => match (section, v) {
            (0, false) => t!("settings-bar-preview-location-left"),
            (0, true) => t!("settings-bar-preview-location-top"),
            (1, _) => t!("settings-bar-preview-location-center"),
            (_, false) => t!("settings-bar-preview-location-right"),
            (_, true) => t!("settings-bar-preview-location-bottom"),
        },
    }
}

// ---- Dinamik widget ayarları ---------------------------------------------------------------

/// Kartta dinamik satır olarak gösterilecek ayarlar: şemada olup kayıtta satırı olmayanlar
/// (kayıttakiler kendi satırlarıyla, aranabilir). `vertical_*` ayarları yalnızca dikey bar'da
/// anlamlıdır.
pub fn extra_settings<'a>(catalog: &'a [WidgetSpec], widget: &str, vertical: bool) -> Vec<&'a SettingSpec> {
    let Some(w) = catalog.iter().find(|w| w.name == widget) else { return Vec::new() };
    w.settings
        .iter()
        .filter(|s| vertical || !s.key.starts_with("vertical_"))
        .filter(|s| {
            let path = format!("bar.widget.{widget}.{}", s.key);
            !registry::SETTINGS.iter().any(|d| d.path == path)
        })
        .collect()
}

/// Dinamik tanımın önbellek anahtarı: yalnızca dile bağlı OLMAYAN alanlar. Başlık ve seçenek
/// adları tanımda tutulmaz; satır onları her kurulumda o anki şemadan okur
/// ([`SettingDesc::display_title`], [`SettingDesc::choices`]). Böylece dil değişiminde (şema
/// yeniden okunur) aynı tanım yeniden kullanılır, önbellek büyümez.
#[derive(Debug, PartialEq)]
struct DescKey {
    path: String,
    /// Seçimde değerler yok (satır onları şemadan alır); yalnızca tür, sınır, adım, birim.
    kind: SettingKind,
    segmented: bool,
    placeholder: String,
}

/// Dinamik tanımlar. Satır bileşeni (`SettingRow`) `'static` tanım ister; tanım anahtar başına
/// bir kez kurulur ve süreç boyunca kalır. Sınır: şemadaki widget ayarı sayısı (dil sayısından
/// bağımsız; shell bir ayarın türünü/sınırını oturum ortasında değiştirmedikçe artmaz).
static DYNAMIC: Mutex<Vec<(DescKey, &'static SettingDesc)>> = Mutex::new(Vec::new());

fn desc_key(path: String, spec: &SettingSpec) -> DescKey {
    let (kind, segmented) = match &spec.kind {
        SettingKind::Choice(o) => (SettingKind::Choice(Vec::new()), o.len() <= 4),
        k => (k.clone(), false),
    };
    let placeholder = match spec.kind {
        SettingKind::Text => spec.default.as_ref().and_then(Value::as_str).unwrap_or("").to_owned(),
        _ => String::new(),
    };
    DescKey { path, kind, segmented, placeholder }
}

/// Dinamik satırın kayıt tanımı (bkz. [`DescKey`]). Kimlik = anahtar yolu; aramada yoktur.
pub fn dynamic_desc(widget: &str, spec: &SettingSpec) -> Option<&'static SettingDesc> {
    let key = desc_key(format!("bar.widget.{widget}.{}", spec.key), spec);
    let mut cache = DYNAMIC.lock().ok()?;
    if let Some((_, d)) = cache.iter().find(|(k, _)| *k == key) {
        return Some(d);
    }
    // Aynı yolun önceki tanımı varsa yol dizesi paylaşılır.
    let path: &'static str = match cache.iter().find(|(k, _)| k.path == key.path) {
        Some((_, d)) => d.path,
        None => Box::leak(key.path.clone().into_boxed_str()),
    };
    let leak = |s: &str| -> &'static str { if s.is_empty() { "" } else { Box::leak(s.to_owned().into_boxed_str()) } };
    let control = match &key.kind {
        SettingKind::Bool => Control::Toggle { invert: false, on: None },
        SettingKind::Int { min: Some(lo), max: Some(hi), unit } => {
            Control::Slider { min: *lo, max: *hi, step: 1.0, unit: leak(unit), integer: true, percent: false }
        }
        SettingKind::Float { min: Some(lo), max: Some(hi), step, unit } => {
            Control::Slider { min: *lo, max: *hi, step: step.unwrap_or(1.0), unit: leak(unit), integer: false, percent: false }
        }
        SettingKind::Text => Control::Text { placeholder: leak(&key.placeholder) },
        // Değerler ve adlar şemadan (`SettingDesc::choices`): yerel tablo yok.
        SettingKind::Choice(_) => Control::Choice { options: &[], style: if key.segmented { ChoiceStyle::Segmented } else { ChoiceStyle::Dropdown } },
        // Sınırsız sayı kaydırıcıya dönüşmez.
        _ => return None,
    };
    let desc: &'static SettingDesc = Box::leak(Box::new(SettingDesc {
        id: path,
        page: PageId::Bar,
        section: "Seçili widget", // i18n: tanı kayıt bölüm kimliği (kaynak Türkçe); görünen ad section_title ile katalogdan
        // Boş: başlık şemadan, etkin dilde (`SettingDesc::display_title`).
        title: "",
        subtitle: "",
        keywords: &[],
        file: FileId::Shell,
        path,
        control,
        visible_if: None,
        tier: Tier::Basic,
        mirror_on: &[],
        ui_if: None,
    }));
    cache.push((key, desc));
    Some(desc)
}

// ---- Bar önizlemesi (hero) -------------------------------------------------------------------

#[derive(Debug)]
struct Pill {
    name: String,
    /// Bar bölümü (0 baş, 1 orta, 2 son).
    section: usize,
    label: Label,
    icon: IconView,
    hover: Interaction,
    rect: Rect,
}

#[derive(Debug)]
pub struct BarPreview {
    desc: &'static SettingDesc,
    pills: Vec<Pill>,
    edge: Edge,
    frame_on: bool,
    /// Uç eylemi varsayılandan farklı (baş, son).
    marks: [bool; 2],
    /// Adlar sığmıyor: haplar yalnızca ikon.
    icon_only: bool,
    hint: Label,
    rect: Rect,
    screen: Rect,
    bar: Rect,
    /// Sayfa durumundaki seçim (`None` = varsayılan).
    ui_selected: Option<String>,
    /// Tıklanan, sayfa durumuna henüz yazılmamış seçim.
    pending: Option<String>,
    focus_visible: bool,
    focused: bool,
}

impl BarPreview {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut s = Self {
            desc,
            pills: Vec::new(),
            edge: Edge::Top,
            frame_on: true,
            marks: [false; 2],
            icon_only: false,
            hint: Label::new(t!("settings-bar-preview-hint"), text_style(theme, TextRole::Caption)),
            rect: Rect::ZERO,
            screen: Rect::ZERO,
            bar: Rect::ZERO,
            ui_selected: None,
            pending: None,
            focus_visible: false,
            focused: false,
        };
        s.rebuild(theme, store);
        s
    }

    fn rebuild(&mut self, theme: &Theme, store: &Store) {
        let lists = bar_lists(store);
        let same = self.pills.len() == lists.iter().map(Vec::len).sum::<usize>()
            && self.pills.iter().zip(lists.iter().enumerate().flat_map(|(i, l)| l.iter().map(move |n| (i, n)))).all(|(p, (i, n))| p.section == i && &p.name == n);
        if !same {
            self.pills = lists
                .iter()
                .enumerate()
                .flat_map(|(section, l)| l.iter().map(move |n| (section, n)))
                .map(|(section, name)| Self::pill(theme, section, name))
                .collect();
        }
        self.edge = bar_edge(store);
        self.frame_on = by_id("frame.enabled").is_none_or(|d| d.bool(store));
        self.marks = [end_customized(store, BarEnd::Start), end_customized(store, BarEnd::End)];
    }

    fn pill(theme: &Theme, section: usize, name: &str) -> Pill {
        let (title, icon) = widget_info(name);
        Pill {
            name: name.to_owned(),
            section,
            label: Label::new(title, text_style(theme, TextRole::Label)),
            icon: IconView::with_role(theme, Icon::named(icon), TextRole::IconSmall),
            hover: Interaction::new(theme),
            rect: Rect::ZERO,
        }
    }

    fn vertical(&self) -> bool {
        matches!(self.edge, Edge::Left | Edge::Right)
    }

    /// Seçili hap: sayfa durumu (ya da bekleyen tıklama), yoksa sağ uç / sol uç / ilk.
    fn selected_index(&self) -> Option<usize> {
        if let Some(n) = self.pending.as_ref().or(self.ui_selected.as_ref()) {
            return self.pills.iter().position(|p| &p.name == n);
        }
        let last_end = self.pills.iter().rposition(|p| p.section == 2);
        let first_start = self.pills.iter().position(|p| p.section == 0);
        last_end.or(first_start).or((!self.pills.is_empty()).then_some(0))
    }

    /// Uçtaki hap mı (nokta için): baş bölümün ilki ya da son bölümün sonuncusu.
    fn end_of(&self, i: usize) -> Option<BarEnd> {
        let p = &self.pills[i];
        if p.section == 0 && self.pills.iter().position(|q| q.section == 0) == Some(i) {
            return Some(BarEnd::Start);
        }
        if p.section == 2 && self.pills.iter().rposition(|q| q.section == 2) == Some(i) {
            return Some(BarEnd::End);
        }
        None
    }

    fn bar_thickness(theme: &Theme) -> f32 {
        theme.space(12)
    }

    fn pill_height(theme: &Theme) -> f32 {
        theme.space(8)
    }

    /// Dikey bar'da şemanın yüksekliği: üç bölüm üst üste binmeden sığsın.
    fn screen_height(&self, theme: &Theme) -> f32 {
        let horizontal = Self::bar_thickness(theme) + theme.space(14);
        if !self.vertical() {
            return horizontal;
        }
        let (ph, gap) = (Self::pill_height(theme), theme.space(2));
        let len = |s: usize| {
            let n = self.pills.iter().filter(|p| p.section == s).count() as f32;
            if n == 0.0 { 0.0 } else { n * ph + (n - 1.0) * gap }
        };
        let need = 2.0 * len(0).max(len(2)) + len(1) + gap * 4.0 + theme.space(4);
        need.max(theme.space(36))
    }

    /// Hap genişliği: ikon + ad, ya da yalnızca ikon.
    fn pill_size(theme: &Theme, text: &mut nuxut::TextSystem, p: &mut Pill, icon_only: bool) -> Size {
        let ph = Self::pill_height(theme);
        let iw = p.icon.measure(text).width;
        if icon_only {
            return Size::new(ph.max(iw + theme.space(4)), ph);
        }
        Size::new(theme.space(3) + iw + theme.space(2) + p.label.measure(text).width + theme.space(3), ph)
    }

    /// Hapları bar'a dizer; `icon_only` değilken sığmazsa `false`.
    fn lay_out(&mut self, theme: &Theme, text: &mut nuxut::TextSystem, icon_only: bool) -> bool {
        let gap = theme.space(2);
        let m = theme.space(2);
        let vertical = self.vertical();
        let sizes: Vec<Size> = self.pills.iter_mut().map(|p| Self::pill_size(theme, text, p, icon_only)).collect();
        let along = |s: &Size| if vertical { s.height } else { s.width };
        let total = |sec: usize| {
            let v: Vec<f32> = self.pills.iter().zip(&sizes).filter(|(p, _)| p.section == sec).map(|(_, s)| along(s)).collect();
            if v.is_empty() { 0.0 } else { v.iter().sum::<f32>() + gap * (v.len() - 1) as f32 }
        };
        let (start_len, center_len, end_len) = (total(0), total(1), total(2));
        let bar = self.bar;
        let (lo, hi) = if vertical { (bar.y + m, bar.bottom() - m) } else { (bar.x + m, bar.right() - m) };
        let mid = (lo + hi) * 0.5;
        let center_lo = mid - center_len * 0.5;
        // Sığma: dolu bölümler arasında en az bir aralık; orta bölüm tam ortada kalır.
        let sep = |len: f32| if len > 0.0 { gap } else { 0.0 };
        let fits = if center_len > 0.0 {
            center_lo >= lo + start_len + sep(start_len) && center_lo + center_len + sep(end_len) <= hi - end_len
        } else {
            lo + start_len + sep(start_len.min(end_len)) <= hi - end_len
        };
        let mut cursor = [lo, center_lo, hi - end_len];
        for (p, s) in self.pills.iter_mut().zip(&sizes) {
            let at = cursor[p.section];
            p.rect = if vertical {
                Rect::new(bar.center().x - s.width * 0.5, at, s.width, s.height)
            } else {
                Rect::new(at, bar.center().y - s.height * 0.5, s.width, s.height)
            };
            cursor[p.section] += along(s) + gap;
        }
        fits
    }
}

impl Block for BarPreview {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, _width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        py + self.screen_height(t) + t.space(3) + self.hint.measure(env.text).height + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        self.screen = Rect::new(rect.x + px, rect.y + py, (rect.width - px * 2.0).max(0.0), self.screen_height(t));
        let s = self.screen;
        let b = Self::bar_thickness(t);
        self.bar = match self.edge {
            Edge::Top => Rect::new(s.x, s.y, s.width, b),
            Edge::Bottom => Rect::new(s.x, s.bottom() - b, s.width, b),
            Edge::Left => Rect::new(s.x, s.y, b, s.height),
            Edge::Right => Rect::new(s.right() - b, s.y, b, s.height),
        };
        // Dikey bar'da adlar zaten sığmaz: yalnızca ikon (gerçek dikey bar da öyle).
        let vertical = self.vertical();
        self.icon_only = vertical || !self.lay_out(t, env.text, false);
        if self.icon_only {
            self.lay_out(t, env.text, true);
        }
    }

    fn pointer(&mut self, _env: &mut Env, e: &PointerEvent) -> bool {
        let now = _env.now;
        let mut repaint = false;
        let mut clicked = None;
        for (i, p) in self.pills.iter_mut().enumerate() {
            let tap = p.hover.handle(p.rect, e, now);
            repaint |= tap.repaint;
            if tap.clicked {
                clicked = Some(i);
            }
        }
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && self.pills.iter().any(|p| p.rect.contains(e.position))
        {
            self.focus_visible = false;
        }
        if let Some(i) = clicked {
            self.pending = Some(self.pills[i].name.clone());
            repaint = true;
        }
        repaint
    }

    fn key(&mut self, _env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released || self.pills.is_empty() {
            return false;
        }
        let step: isize = match e.key {
            Key::Left | Key::Up => -1,
            Key::Right | Key::Down => 1,
            _ => return false,
        };
        let n = self.pills.len() as isize;
        let cur = self.selected_index().map_or(0, |i| i as isize);
        let next = (cur + step).clamp(0, n - 1) as usize;
        self.pending = Some(self.pills[next].name.clone());
        self.focus_visible = true;
        true
    }

    fn focusable(&self) -> bool {
        !self.pills.is_empty()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focused = focused;
        self.focus_visible = focused && visible;
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.pills.iter().any(|x| x.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.rebuild(env.theme, env.store);
    }

    fn sync_ui(&mut self, ui: &PageUi, _now: Instant) {
        self.ui_selected = ui.selected_widget.clone();
        if self.pending == self.ui_selected {
            self.pending = None;
        }
    }

    fn take_ui(&mut self, ui: &mut PageUi) -> bool {
        let Some(name) = self.pending.take() else { return false };
        ui.selected_widget = Some(name.clone());
        self.ui_selected = Some(name);
        true
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if !matches!(event, crate::services::Event::Widgets) {
            return false;
        }
        // Katalog geldi: adlar ve ikonlar shell'inkine döner.
        self.pills.clear();
        self.rebuild(env.theme, env.store);
        true
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.hint.set_style(text_style(theme, TextRole::Caption));
        for p in &mut self.pills {
            p.label.set_style(text_style(theme, TextRole::Label));
            p.icon.set_theme(theme, now);
            p.hover.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let s = self.screen;
        // Masaüstü: tema tonlarıyla duvar kağıdı, bar kenarı dışında ince çerçeve, bar.
        ui.painter.gradient_rect(s, t.shape.medium(), wallpaper_tones(t).map(|w| ui.fade(w)));
        ui.painter.push_clip(s);
        if self.frame_on {
            let f = t.space(1);
            for e in Edge::ALL.into_iter().filter(|e| *e != self.edge) {
                ui.painter.rect(e.strip(s, f), &RectStyle::filled(ui.fade(c.surface), Corners::ZERO));
            }
        }
        let r = t.shape.radius_medium;
        let radii = match self.edge {
            Edge::Top => Corners { top_left: r, top_right: r, bottom_left: 0.0, bottom_right: 0.0 },
            Edge::Bottom => Corners { top_left: 0.0, top_right: 0.0, bottom_left: r, bottom_right: r },
            Edge::Left => Corners { top_left: r, top_right: 0.0, bottom_left: r, bottom_right: 0.0 },
            Edge::Right => Corners { top_left: 0.0, top_right: r, bottom_left: 0.0, bottom_right: r },
        };
        ui.painter.rect(self.bar, &RectStyle::filled(ui.fade(c.surface), radii));
        ui.painter.pop_clip();

        let sel = self.selected_index();
        // Seçili hap: bar zemini üzerinde tonal kap (framework tonal karışımı), içerik vurgu.
        let selected_fill = tinted(t, c.surface, c.primary, TONAL_MIX, &[c.on_surface, c.on_surface_variant]);
        let selected_fg = accent_on(t, selected_fill);
        let mut animating = false;
        for i in 0..self.pills.len() {
            let on = sel == Some(i);
            let end = self.end_of(i);
            let p = &mut self.pills[i];
            let radii = pill(p.rect);
            if on {
                ui.painter.rect(p.rect, &RectStyle::filled(ui.fade(selected_fill), radii));
            }
            p.hover.draw_layer(ui.painter, p.rect, radii, t, now);
            animating |= p.hover.is_animating(now);
            let fg = if on { selected_fg } else { c.on_surface };
            let iw = p.icon.measure(ui.painter.text_system()).width;
            if self.icon_only {
                p.icon.draw_centered(ui.painter, p.rect.center(), ui.fade(fg));
            } else {
                let x = p.rect.x + t.space(3);
                p.icon.draw_centered(ui.painter, Point::new(x + iw * 0.5, p.rect.center().y), ui.fade(fg));
                let lh = p.label.measure_in(ui.painter).height;
                p.label.draw(ui.painter, Point::new(x + iw + t.space(2), p.rect.center().y - lh * 0.5), ui.fade(fg));
            }
            // Uca özel eylem atanmış: hapın köşesinde nokta.
            if let Some(e) = end
                && self.marks[e as usize]
            {
                let d = t.space(2);
                let dot = Rect::new(p.rect.right() - d * 0.75, p.rect.y - d * 0.25, d, d);
                ui.painter.rect(dot, &RectStyle::filled(ui.fade(c.primary), Corners::uniform(d * 0.5)));
            }
            if on && self.focus_visible {
                let ring = p.rect.expand(FOCUS_RING + 1.0);
                ui.painter.rect(ring, &RectStyle { fill: Color::TRANSPARENT, radii: pill(ring), border: Some(Border { width: FOCUS_RING, color: ui.fade(focus_ring(t, &[c.surface, selected_fill])) }) });
            }
        }
        if !self.pills.is_empty() {
            self.hint.draw(ui.painter, Point::new(self.rect.x + px, s.bottom() + t.space(3)), ui.fade(c.on_surface_variant));
        }
        if animating {
            ui.request_frame();
        }
    }
}

// ---- Seçili widget kartının başlığı ----------------------------------------------------------

#[derive(Debug)]
struct Row {
    row: SettingRow,
    rect: Rect,
}

/// Kartın ilk satırı: seçili widget'ın ikonu, adı ve bar'daki yeri; altında şemadaki diğer
/// ayarları (dinamik satırlar). Ayarı olmayan ve uçta olmayan widget'ta tek satır bilgi.
#[derive(Debug)]
pub struct SelectedWidget {
    desc: &'static SettingDesc,
    icon: IconView,
    title: Label,
    place: Label,
    rows: Vec<Row>,
    /// Kurulan satırların yolları (değişmediyse satırlar korunur: odak, metin, menü).
    paths: Vec<&'static str>,
    ui_selected: Option<String>,
    /// Sayfa durumu ya da katalog değişti: başlık ve satırlar yeniden kurulacak.
    dirty: bool,
    rect: Rect,
    head_h: f32,
    focus: Option<usize>,
}

impl SelectedWidget {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut s = Self {
            desc,
            icon: IconView::with_role(theme, Icon::named(IconName::Widgets), TextRole::Icon),
            title: Label::new("", text_style(theme, TextRole::Body)),
            place: Label::new("", text_style(theme, TextRole::Caption)),
            rows: Vec::new(),
            paths: Vec::new(),
            ui_selected: None,
            dirty: true,
            rect: Rect::ZERO,
            head_h: 0.0,
            focus: None,
        };
        s.refresh(theme, store);
        s
    }

    /// Başlık metinleri ve dinamik satırlar (seçim, config ya da katalog değişince).
    fn refresh(&mut self, theme: &Theme, store: &Store) {
        self.dirty = false;
        let name = self.ui_selected.clone().or_else(|| default_selection(store));
        let Some(name) = name else {
            self.icon.set_icon(Icon::named(IconName::Widgets));
            self.title.set_text(&t!("settings-bar-preview-empty-title"));
            self.place.set_text(&t!("settings-bar-preview-empty-place"));
            self.set_rows(theme, store, Vec::new());
            return;
        };
        let (title, icon) = widget_info(&name);
        self.icon.set_icon(Icon::named(icon));
        self.title.set_text(&title);
        let catalog = widgets::catalog();
        let specs = extra_settings(&catalog, &name, vertical(store));
        let descs: Vec<&'static SettingDesc> = specs.into_iter().filter_map(|s| dynamic_desc(&name, s)).collect();
        let prefix = format!("bar.widget.{name}.");
        let own = registry::SETTINGS.iter().any(|d| d.page == PageId::Bar && d.path.starts_with(&prefix) && d.is_visible(store));
        let at_end = BarEnd::ALL.into_iter().any(|e| end_widget(store, e).as_deref() == Some(name.as_str()));
        let mut place = location_text(store, &name);
        if !own && descs.is_empty() && !at_end {
            place = t!("settings-bar-preview-place-no-settings", place = place);
        }
        self.place.set_text(&place);
        self.set_rows(theme, store, descs);
    }

    fn set_rows(&mut self, theme: &Theme, store: &Store, descs: Vec<&'static SettingDesc>) {
        let paths: Vec<&'static str> = descs.iter().map(|d| d.path).collect();
        if paths == self.paths {
            return;
        }
        self.paths = paths;
        self.focus = None;
        self.rows = descs.into_iter().filter_map(|d| SettingRow::new(theme, d, store)).map(|row| Row { row, rect: Rect::ZERO }).collect();
    }

    fn icon_box(theme: &Theme) -> f32 {
        theme.space(10)
    }

    fn header_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let text_w = (width - px * 2.0 - Self::icon_box(t) - t.space(3)).max(t.space(20));
        self.title.set_max_width(Some(text_w));
        self.place.set_max_width(Some(text_w));
        let text = self.title.measure(env.text).height + t.space(1) + self.place.measure(env.text).height;
        crate::rows::row_height_for(t, text.max(Self::icon_box(t)))
    }

    fn set_child_focus(&mut self, focus: Option<usize>, visible: bool, now: Instant) {
        if let Some(i) = self.focus
            && Some(i) != focus
            && let Some(r) = self.rows.get_mut(i)
        {
            r.row.set_focus(false, false, now);
        }
        self.focus = focus;
        if let Some(i) = focus
            && let Some(r) = self.rows.get_mut(i)
        {
            r.row.set_focus(true, visible, now);
        }
    }

    fn focusable_rows(&self) -> Vec<usize> {
        self.rows.iter().enumerate().filter(|(_, r)| r.row.focusable()).map(|(i, _)| i).collect()
    }
}

impl Block for SelectedWidget {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        if self.dirty {
            self.refresh(env.theme, env.store);
        }
        let mut h = self.header_height(env, width);
        for r in &mut self.rows {
            h += r.row.height(env, width);
        }
        h
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        if self.dirty {
            self.refresh(env.theme, env.store);
        }
        self.rect = rect;
        self.head_h = self.header_height(env, rect.width);
        let mut y = rect.y + self.head_h;
        for r in &mut self.rows {
            let h = r.row.height(env, rect.width);
            r.rect = Rect::new(rect.x, y, rect.width, h);
            r.row.place(env, r.rect);
            y += h;
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        // Açık menüsü olan satır olayın tek sahibi.
        if let Some(r) = self.rows.iter_mut().find(|r| r.row.overlay_open()) {
            return r.row.pointer(env, e);
        }
        if let PointerKind::Press(MouseButton::Left) = e.kind {
            let hit = self.rows.iter().position(|r| r.rect.contains(e.position) && r.row.focusable());
            if hit.is_some() && hit != self.focus {
                self.set_child_focus(hit, false, env.now);
            }
        }
        let mut repaint = false;
        for r in &mut self.rows {
            repaint |= r.row.pointer(env, e);
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        match self.focus {
            Some(i) if i < self.rows.len() => self.rows[i].row.key(env, e),
            _ => false,
        }
    }

    fn captures_keys(&self) -> bool {
        self.focus.and_then(|i| self.rows.get(i)).is_some_and(|r| r.row.captures_keys())
    }

    fn focusable(&self) -> bool {
        self.rows.iter().any(|r| r.row.focusable())
    }

    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        if focused {
            let f = self.focus.or_else(|| self.focusable_rows().first().copied());
            self.set_child_focus(f, visible, now);
        } else {
            self.set_child_focus(None, false, now);
        }
    }

    fn focus_step(&mut self, forward: bool, now: Instant) -> bool {
        let order = self.focusable_rows();
        let next = match (self.focus, forward) {
            (Some(i), true) => order.iter().find(|&&j| j > i).copied(),
            (Some(i), false) => order.iter().rev().find(|&&j| j < i).copied(),
            (None, _) => None,
        };
        match next {
            Some(j) => {
                self.set_child_focus(Some(j), true, now);
                true
            }
            None => false,
        }
    }

    fn overlay_open(&self) -> bool {
        self.rows.iter().any(|r| r.row.overlay_open())
    }

    fn text_input(&mut self) -> Option<&mut nuxut::ui::TextInput> {
        let i = self.focus?;
        self.rows.get_mut(i)?.row.text_input()
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rows.iter().find_map(|r| r.row.cursor(p))
    }

    fn sync(&mut self, env: &mut Env) {
        // Config değişti: yer metni (sıra, kenar), dikey bar ayarları, satır değerleri.
        self.refresh(env.theme, env.store);
        for r in &mut self.rows {
            r.row.sync(env);
        }
    }

    fn sync_ui(&mut self, ui: &PageUi, _now: Instant) {
        if self.ui_selected != ui.selected_widget {
            self.ui_selected = ui.selected_widget.clone();
            self.dirty = true;
        }
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        let mut any = false;
        for r in &mut self.rows {
            any |= r.row.service_event(env, event);
        }
        if matches!(event, crate::services::Event::Widgets) {
            self.paths.clear();
            self.dirty = true;
            any = true;
        }
        any
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.title.set_style(text_style(theme, TextRole::Body));
        self.place.set_style(text_style(theme, TextRole::Caption));
        for r in &mut self.rows {
            r.row.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let r = self.rect;
        let b = Self::icon_box(t);
        let head = Rect::new(r.x, r.y, r.width, self.head_h);
        // İkon tonal dairede (seçili öğe zemini), metin yanında; ikisi başlık satırında ortalı.
        let icon_rect = Rect::new(r.x + px, head.center().y - b * 0.5, b, b);
        ui.painter.rect(icon_rect, &RectStyle::filled(ui.fade(tonal_container(t)), pill(icon_rect)));
        self.icon.draw_centered(ui.painter, icon_rect.center(), ui.fade(c.primary));
        let th = self.title.measure_in(ui.painter).height;
        let ph = self.place.measure_in(ui.painter).height;
        let tx = icon_rect.right() + t.space(3);
        let ty = head.center().y - (th + t.space(1) + ph) * 0.5;
        self.title.draw(ui.painter, Point::new(tx, ty), ui.fade(c.on_surface));
        self.place.draw(ui.painter, Point::new(tx, ty + th + t.space(1)), ui.fade(c.on_surface_variant));
        for row in &mut self.rows {
            ui.painter.push_clip(row.rect);
            row.row.draw(ui);
            ui.painter.pop_clip();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nuxut::i18n::t;

    fn store_with(shell: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("s.toml"), shell).unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, store)
    }

    #[test]
    fn ends_follow_shell_sections() {
        let (_d, store) = store_with("[bar]\nstart = [\"workspaces\", \"active_window\"]\ncenter = [\"clock\"]\nend = [\"tray\", \"session\"]\n");
        assert_eq!(end_widget(&store, BarEnd::Start).as_deref(), Some("workspaces"));
        assert_eq!(end_widget(&store, BarEnd::End).as_deref(), Some("session"));
        // Bölüm boşsa o uçta widget yok (orta bölümün widget'ı uç sayılmaz — shell gibi).
        let (_d, store) = store_with("[bar]\nstart = []\ncenter = [\"clock\"]\nend = []\n");
        assert_eq!(end_widget(&store, BarEnd::Start), None);
        assert_eq!(end_widget(&store, BarEnd::End), None);
        assert_eq!(default_selection(&store).as_deref(), Some("clock"), "uç yoksa ilk widget");
    }

    /// Karar 4: seçim yokken sağ uç; sayfa durumu seçimi ezer; uç bilgisi config'ten.
    #[test]
    fn default_selection_is_right_end() {
        let (_d, store) = store_with("[bar]\nstart = [\"workspaces\"]\ncenter = [\"clock\"]\nend = [\"tray\", \"session\"]\n");
        let mut ui = PageUi::default();
        assert_eq!(selected(&ui, &store).as_deref(), Some("session"));
        assert_eq!(selected_end(&ui, &store), Some(BarEnd::End));
        ui.selected_widget = Some("workspaces".into());
        assert_eq!(selected_end(&ui, &store), Some(BarEnd::Start));
        ui.selected_widget = Some("clock".into());
        assert_eq!(selected_end(&ui, &store), None);
        ui.selected_widget = Some("tray".into());
        assert_eq!(selected_end(&ui, &store), None, "son bölümde ama uçta değil");
    }

    #[test]
    fn location_text_is_impersonal_and_follows_orientation() {
        crate::i18n::use_turkish_for_tests();
        let (_d, store) = store_with("[bar]\nedge = \"left\"\nstart = [\"workspaces\"]\ncenter = [\"clock\"]\nend = [\"session\"]\n");
        assert!(location_text(&store, "workspaces").starts_with("Bar'ın üst ucunda"));
        assert!(location_text(&store, "session").starts_with("Bar'ın alt ucunda"));
        assert_eq!(location_text(&store, "clock"), "Bar'ın ortasında");
        assert!(location_text(&store, "tray").starts_with("Bar'da değil"));
        for name in ["workspaces", "session", "clock", "tray"] {
            let s = location_text(&store, name);
            assert!(!s.contains("siz") && !s.contains("ın.") && !s.contains("tıklayın"), "hitapsız: {s}");
        }
    }

    const SCHEMA: &str = r#"
version = 1
[[widget]]
name = "clock"
title = "Saat"
icon = "schedule"
bar = true
[[widget.setting]]
key = "format"
title = "Biçim"
kind = "text"
default = "%H:%M"
[[widget.setting]]
key = "vertical_format"
title = "Dikey bar biçimi"
kind = "text"
default = "%H"
[[widget]]
name = "session"
title = "Oturum"
icon = "power"
bar = true
[[widget.setting]]
key = "panel"
title = "Açtığı panel"
kind = "choice"
default = "right"
options = ["left", "right", "top", "bottom"]
[[widget.setting]]
key = "max"
title = "Sınırsız"
kind = "int"
"#;

    fn catalog() -> Vec<WidgetSpec> {
        widgets::parse(SCHEMA)
    }

    /// §3.5: şemadaki, kayıtta satırı olmayan widget ayarları dinamik satır olur; kayıttaki
    /// (`clock.format`) tekrar edilmez, dikey ayar yalnızca dikey bar'da, sınırsız sayı yok.
    #[test]
    fn extra_settings_come_from_schema_without_duplicates() {
        widgets::set_test_schema(SCHEMA);
        let catalog = catalog();
        let keys = |w: &str, v: bool| extra_settings(&catalog, w, v).iter().map(|s| s.key.as_str()).collect::<Vec<_>>();
        assert_eq!(keys("clock", false), Vec::<&str>::new());
        assert_eq!(keys("clock", true), ["vertical_format"]);
        assert_eq!(keys("session", false), ["panel", "max"]);
        assert!(keys("unknown", false).is_empty());
        let panel = extra_settings(&catalog, "session", false)[0];
        let d = dynamic_desc("session", panel).unwrap();
        assert_eq!(d.path, "bar.widget.session.panel");
        assert_eq!(d.page, PageId::Bar);
        assert!(registry::by_id(d.id).is_none(), "dinamik satır kayıtta değil");
        // Şemada `option_labels` yok: Ayarlar kataloğundaki kenar adları.
        let want: Vec<(String, String)> = [("left", t!("settings-edge-left")), ("right", t!("settings-edge-right")), ("top", t!("settings-edge-top")), ("bottom", t!("settings-edge-bottom"))]
            .into_iter()
            .map(|(v, l)| (v.to_owned(), l))
            .collect();
        assert_eq!(d.choices(), want);
        assert_eq!(d.display_title(), "Açtığı panel", "başlık şemadan");
        assert!(std::ptr::eq(d, dynamic_desc("session", panel).unwrap()), "yol başına tek tanım");
        let max = extra_settings(&catalog, "session", false)[1];
        assert!(dynamic_desc("session", max).is_none(), "sınırsız sayı kaydırıcı olmaz");
    }

    /// Güncel shell şeması (`clock-locale.md`): dikey biçim dinamik satırının varsayılanı `""`,
    /// alt metni şemanın açıklaması, yer tutucusu "Otomatik…" (kalıp örneği değil).
    #[test]
    fn vertical_clock_row_is_automatic_when_empty() {
        crate::i18n::use_turkish_for_tests();
        let schema = "version = 1\n[[widget]]\nname = \"clock\"\ntitle = \"Saat\"\nbar = true\n[[widget.setting]]\nkey = \"vertical_format\"\ntitle = \"Dikey bar biçimi\"\nkind = \"text\"\ndefault = \"\"\ndescription = \"Boş bırakılırsa 12/24 saat tercihine uyar; bir biçim kalıbı yazılırsa o kullanılır.\"\n";
        widgets::set_test_schema(schema);
        let catalog = widgets::parse(schema);
        let spec = extra_settings(&catalog, "clock", true)[0];
        let d = dynamic_desc("clock", spec).unwrap();
        let (_d, store) = store_with("[bar]\nedge = \"left\"\n");
        assert_eq!(d.display_subtitle(), "Boş bırakılırsa 12/24 saat tercihine uyar; bir biçim kalıbı yazılırsa o kullanılır.");
        assert_eq!(d.placeholder(&store), "Otomatik (12/24 saat tercihine uyar)");
    }

    /// Aynı şemanın iki dildeki çıktısı (shell `schema`, framework.toml `[locale]`'e göre).
    fn lang_schema(title: &str, labels: &str) -> String {
        format!(
            r#"version = 1
[[widget]]
name = "langtest"
title = "{title}"
[[widget.setting]]
key = "panel"
title = "{title}"
kind = "choice"
default = "right"
options = ["left", "right", "top", "bottom"]
{labels}
[[widget.setting]]
key = "width"
title = "{title} w"
kind = "float"
min = 10.0
max = 20.0
unit = "px"
"#
        )
    }

    /// Dil değişince (şema yeniden okunur) dinamik satırın başlığı ve seçenek adları yeni dilde;
    /// tanım yeniden kullanılır, önbellek büyümez (dil başına sızıntı yok).
    #[test]
    fn dynamic_rows_follow_locale_without_cache_growth() {
        let count = || DYNAMIC.lock().unwrap().iter().filter(|(k, _)| k.path.starts_with("bar.widget.langtest.")).count();
        let descs = |text: &str| {
            widgets::set_test_schema(text);
            let catalog = widgets::parse(text);
            extra_settings(&catalog, "langtest", false).into_iter().filter_map(|s| dynamic_desc("langtest", s)).collect::<Vec<_>>()
        };
        let tr = descs(&lang_schema("Açtığı panel", r#"option_labels = ["Sol", "Sağ", "Üst", "Alt"]"#));
        assert_eq!(tr.len(), 2);
        assert_eq!(tr[0].display_title(), "Açtığı panel");
        assert_eq!(tr[0].choices().iter().map(|(_, l)| l.as_str()).collect::<Vec<_>>(), ["Sol", "Sağ", "Üst", "Alt"]);
        let before = count();
        for _ in 0..5 {
            let en = descs(&lang_schema("Opens panel", r#"option_labels = ["Left", "Right", "Top", "Bottom"]"#));
            assert!(en.iter().zip(&tr).all(|(a, b)| std::ptr::eq(*a, *b)), "aynı tanım");
            assert_eq!(en[0].display_title(), "Opens panel");
            assert_eq!(en[1].display_title(), "Opens panel w");
            assert_eq!(en[0].choices().iter().map(|(_, l)| l.as_str()).collect::<Vec<_>>(), ["Left", "Right", "Top", "Bottom"]);
            descs(&lang_schema("Açtığı panel", r#"option_labels = ["Sol", "Sağ", "Üst", "Alt"]"#));
        }
        assert_eq!(count(), before, "dil değişimi önbelleği büyütmez");
        // Uzunluğu uymayan adlar yok sayılır: kataloğa düşer.
        let bad = descs(&lang_schema("X", r#"option_labels = ["Sol", "Sağ"]"#));
        assert_eq!(bad[0].choices()[0].1, t!("settings-edge-left"));
        assert_eq!(count(), before);
    }

    fn env_parts() -> (Theme, nuxut::TextSystem, Vec<crate::block::Action>) {
        (Theme::default(), nuxut::TextSystem::new(), Vec::new())
    }

    /// Hero: tıklanan hap sayfa durumuna yazılır; varsayılan seçim sağ uç; dar önizlemede
    /// haplar yalnızca ikona iner, geniş önizlemede adlar görünür ve haplar üst üste binmez.
    #[test]
    fn preview_selects_and_degrades_to_icons() {
        let (_d, store) = store_with("[bar]\nstart = [\"workspaces\"]\ncenter = [\"clock\"]\nend = [\"tray\", \"notifications\", \"session\"]\n");
        let (theme, mut text, mut actions) = env_parts();
        let now = Instant::now();
        let desc = registry::by_id("bar.preview").unwrap();
        let mut p = BarPreview::new(&theme, desc, &store);
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        let h = p.height(&mut env, 1000.0);
        p.place(&mut env, Rect::new(0.0, 0.0, 1000.0, h));
        assert!(!p.icon_only, "geniş: adlar sığar");
        assert_eq!(p.selected_index().map(|i| p.pills[i].name.as_str()), Some("session"));
        for w in p.pills.windows(2) {
            assert!(w[0].rect.right() <= w[1].rect.x + 0.5, "haplar sırayla, üst üste değil: {:?} {:?}", w[0].name, w[1].name);
        }
        let clock = p.pills.iter().find(|x| x.name == "clock").unwrap().rect;
        assert!((clock.center().x - p.bar.center().x).abs() < 1.0, "orta bölüm ortada");
        // Tıkla: saat seçilir, sayfa durumuna yazılır.
        let at = clock.center();
        for kind in [PointerKind::Motion, PointerKind::Press(MouseButton::Left), PointerKind::Release(MouseButton::Left)] {
            p.pointer(&mut env, &PointerEvent::new(at, kind));
        }
        let mut ui = PageUi::default();
        assert!(p.take_ui(&mut ui));
        assert_eq!(ui.selected_widget.as_deref(), Some("clock"));
        assert!(!p.take_ui(&mut ui), "bir kez");
        // Dar: yalnızca ikon, yine taşmaz.
        p.place(&mut env, Rect::new(0.0, 0.0, 320.0, h));
        assert!(p.icon_only, "dar: yalnızca ikon");
        assert!(p.pills.iter().all(|x| x.rect.x >= p.bar.x && x.rect.right() <= p.bar.right()));
    }

    /// Uca özel eylem atanmışsa (varsayılandan farklı) o uçtaki hapta nokta.
    #[test]
    fn customized_end_is_marked() {
        let theme = Theme::default();
        let (_d, store) = store_with("[bar]\nstart = [\"workspaces\"]\nend = [\"session\"]\n");
        let p = BarPreview::new(&theme, registry::by_id("bar.preview").unwrap(), &store);
        assert_eq!(p.marks, [false, false]);
        let (_d, store) = store_with("[bar]\nstart = [\"workspaces\"]\nend = [\"session\"]\n[bar.edge_actions.end]\nleft_click = \"panel:launcher\"\n");
        let p = BarPreview::new(&theme, registry::by_id("bar.preview").unwrap(), &store);
        assert_eq!(p.marks, [false, true]);
        assert_eq!(p.end_of(p.pills.len() - 1), Some(BarEnd::End));
        assert_eq!(p.end_of(0), Some(BarEnd::Start));
    }

    /// Dikey bar: önizleme dikey çizilir (bar şemanın solunda, haplar alt alta).
    #[test]
    fn preview_draws_vertical_bar() {
        let (_d, store) = store_with("[bar]\nedge = \"left\"\nstart = [\"workspaces\"]\ncenter = [\"clock\"]\nend = [\"tray\", \"session\"]\n");
        let (theme, mut text, mut actions) = env_parts();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut p = BarPreview::new(&theme, registry::by_id("bar.preview").unwrap(), &store);
        let h = p.height(&mut env, 900.0);
        p.place(&mut env, Rect::new(0.0, 0.0, 900.0, h));
        assert!(p.bar.height > p.bar.width && p.bar.x == p.screen.x);
        for w in p.pills.windows(2) {
            assert!(w[0].rect.bottom() <= w[1].rect.y + 0.5);
        }
        assert!(p.pills.iter().all(|x| x.rect.y >= p.bar.y && x.rect.bottom() <= p.bar.bottom()));
    }

    /// Arama her Bar ayarını bulur ve sonuca gidince satır görünür: uç eylemleri o uçtaki
    /// widget'ı, widget ayarları o widget'ı seçer (shell'in varsayılanlarına benzer config).
    #[test]
    fn search_reveals_every_bar_setting() {
        let (_d, store) = store_with(
            "[bar]\nstart = [\"workspaces\"]\ncenter = [\"clock\"]\nend = [\"tray\", \"session\"]\n\
             [bar.edge_actions.start]\nscroll = \"brightness\"\n[bar.edge_actions.end]\nscroll = \"volume\"\n",
        );
        let (theme, mut text, mut actions) = env_parts();
        let now = Instant::now();
        for desc in registry::SETTINGS.iter().filter(|d| d.page == PageId::Bar) {
            let hits = registry::search(&store, desc.title);
            assert!(hits.iter().any(|h| h.id == desc.id), "{}: aramada yok", desc.id);
            let blocks = crate::pages::build(&theme, PageId::Bar, &store);
            let mut page = crate::page::Page::new(&theme, PageId::Bar, blocks, &store);
            page.set_viewport(Rect::new(0.0, 0.0, 1100.0, 800.0));
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            page.reveal(&mut env, desc.id);
            assert!(desc.ui_visible(page.ui(), &store), "{}: sonuç satırı göstermiyor", desc.id);
        }
        // Uçtaki seçim: sağ uç eylemleri sağ uçta, sol uçtakiler sol uçta.
        let mut ui = PageUi::default();
        let end_click = registry::by_id("bar.edge_actions.end.left_click").unwrap();
        let start_click = registry::by_id("bar.edge_actions.start.left_click").unwrap();
        let step = registry::by_id("bar.edge_actions.scroll_step").unwrap();
        assert!(end_click.ui_visible(&ui, &store) && !start_click.ui_visible(&ui, &store), "varsayılan sağ uç");
        assert!(step.ui_visible(&ui, &store), "sağ uçta tekerlek ses: adım görünür");
        ui.selected_widget = Some("clock".into());
        assert!(!end_click.ui_visible(&ui, &store) && !step.ui_visible(&ui, &store), "ortadaki widget'ta uç satırı yok");
        assert!(registry::by_id("bar.widget.clock.format").unwrap().ui_visible(&ui, &store));
        ui.selected_widget = Some("workspaces".into());
        assert!(start_click.ui_visible(&ui, &store));
    }

    /// Uç eylemi seçenekleri şemadaki geçerli değerler (tıkta parlaklık yok, tekerlekte panel
    /// yok), açılır seçimle; Uygulama başlatıcı tıkta var.
    #[test]
    fn edge_action_choices_are_dropdowns_with_valid_values() {
        for id in ["bar.edge_actions.start.left_click", "bar.edge_actions.end.right_click"] {
            let d = registry::by_id(id).unwrap();
            assert!(matches!(d.control, Control::Choice { style: ChoiceStyle::Dropdown, .. }));
            let choices = d.choices();
            let values: Vec<&str> = choices.iter().map(|(v, _)| v.as_str()).collect();
            assert!(values.contains(&"panel:launcher") && !values.contains(&"brightness"), "{id}");
        }
        let scroll = registry::by_id("bar.edge_actions.end.scroll").unwrap();
        assert!(scroll.choices().iter().all(|(v, _)| !v.starts_with("panel:")));
    }

    /// Kompakt kenar şeması: standart satır yüksekliği (boş alan yok).
    #[test]
    fn edge_picker_is_a_standard_row() {
        let (_d, store) = store_with("");
        let (theme, mut text, mut actions) = env_parts();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut blocks = crate::pages::build(&theme, PageId::Bar, &store);
        let edge = blocks.iter_mut().find(|b| b.id() == Some("bar.edge")).unwrap();
        assert_eq!(edge.height(&mut env, 1000.0), nuxut::ui::style::row_height(&theme));
    }

    /// Başlık: seçime göre ad ve yer; ayarı olmayan ortadaki widget'ta tek satır bilgi.
    #[test]
    fn selected_header_follows_page_state() {
        crate::i18n::use_turkish_for_tests();
        let (_d, store) = store_with("[bar]\nstart = [\"workspaces\"]\ncenter = [\"launcher\"]\nend = [\"session\"]\n");
        let (theme, mut text, mut actions) = env_parts();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut s = SelectedWidget::new(&theme, registry::by_id("bar.selected").unwrap(), &store);
        assert_eq!(s.title.text(), "session", "katalog yok: config'teki ad");
        assert!(s.place.text().starts_with("Bar'ın sağ ucunda"));
        let mut ui = PageUi::default();
        ui.selected_widget = Some("launcher".into());
        s.sync_ui(&ui, env.now);
        let _ = s.height(&mut env, 700.0);
        assert_eq!(s.place.text(), "Bar'ın ortasında · Bu widget'ın ayarı yok");
        ui.selected_widget = Some("workspaces".into());
        s.sync_ui(&ui, env.now);
        let _ = s.height(&mut env, 700.0);
        assert!(!s.place.text().contains("ayarı yok"), "kendi ayarları var");
    }
}
