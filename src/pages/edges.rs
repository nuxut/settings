//! Shell geometrisi için görsel seçiciler: bar kenarı (dört kenarlı ekran şeması — kenara
//! tıkla, bar oraya yayla kayar) ve dört kenarın kalınlığı (şema + dört kaydırıcı; bar
//! kenarının kaydırıcısı `bar.size`'dır, diğerleri genel `frame.thickness`'i izler ya da
//! `frame.<kenar>` ile elle ayarlanır).
//!
//! Şemalardaki iç köşe kavsi shell'deki gibi temanın `blob.smoothing`'idir (ADR-0008).

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::motion::Animated;
use nuxut::ui::input::{CursorShape, Button as MouseButton, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{faded_mark, state_layer_under};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, Orientation, Slider, TextRole, Ui, text_style};
use nuxut::{BlobField, Corners, FrameShape, Insets, Point, Rect, RectStyle, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::registry::{SettingDesc, by_id, number};
use crate::rows::row_padding;
use crate::store::{FileId, KeyPath, Store};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

impl Edge {
    pub const ALL: [Edge; 4] = [Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];

    pub fn key(self) -> &'static str {
        match self {
            Edge::Top => "top",
            Edge::Right => "right",
            Edge::Bottom => "bottom",
            Edge::Left => "left",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Edge::Top => crate::i18n::text_or("settings-edge-top", "Üst"),
            Edge::Right => crate::i18n::text_or("settings-edge-right", "Sağ"),
            Edge::Bottom => crate::i18n::text_or("settings-edge-bottom", "Alt"),
            Edge::Left => crate::i18n::text_or("settings-edge-left", "Sol"),
        }
    }

    pub fn from_key(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.key() == s)
    }

    pub fn get(self, i: &Insets) -> f32 {
        match self {
            Edge::Top => i.top,
            Edge::Right => i.right,
            Edge::Bottom => i.bottom,
            Edge::Left => i.left,
        }
    }

    fn set(self, i: &mut Insets, v: f32) {
        match self {
            Edge::Top => i.top = v,
            Edge::Right => i.right = v,
            Edge::Bottom => i.bottom = v,
            Edge::Left => i.left = v,
        }
    }

    /// Şemada bu kenarın tıklama/vurgu şeridi.
    pub fn strip(self, r: Rect, depth: f32) -> Rect {
        match self {
            Edge::Top => Rect::new(r.x, r.y, r.width, depth),
            Edge::Bottom => Rect::new(r.x, r.bottom() - depth, r.width, depth),
            Edge::Left => Rect::new(r.x, r.y, depth, r.height),
            Edge::Right => Rect::new(r.right() - depth, r.y, depth, r.height),
        }
    }
}

pub fn bar_edge(store: &Store) -> Edge {
    store.get(FileId::Shell, &KeyPath::parse("bar.edge")).and_then(Value::as_str).and_then(Edge::from_key).unwrap_or(Edge::Top)
}

/// Kenarın açık kalınlık geçersiz kılması (`frame.<kenar>`), yoksa `None`.
fn edge_override(store: &Store, e: Edge) -> Option<f64> {
    number(store.get(FileId::Shell, &KeyPath::new(["frame", e.key()])))
}

/// Shell'in gerçek kenar kalınlıkları (piksel): bar kenarı = bar boyu.
pub fn shell_insets(store: &Store) -> Insets {
    let num = |id: &str| by_id(id).map_or(0.0, |d| d.number(store)) as f32;
    let enabled = by_id("frame.enabled").is_none_or(|d| d.bool(store));
    let base = num("frame.thickness");
    let bar = bar_edge(store);
    let mut i = Insets::ZERO;
    for e in Edge::ALL {
        let v = if e == bar {
            num("bar.size")
        } else if enabled {
            edge_override(store, e).map_or(base, |v| v as f32)
        } else {
            0.0
        };
        e.set(&mut i, v);
    }
    i
}

/// Şemada gerçek pikseller abartılı ölçeklenir (ince kenar da görünsün).
fn diagram_insets(i: Insets, scale: f32) -> Insets {
    let f = |v: f32| if v <= 0.0 { 0.0 } else { (v * scale).max(1.5) };
    Insets::new(f(i.top), f(i.right), f(i.bottom), f(i.left))
}

/// Küçük ekran şeması: duvar kağıdı + çerçeve (blob alanı, iç kavisli) ve bar kenarında
/// workspace noktaları (barın nerede olduğu bir bakışta okunsun).
/// Şemalardaki "duvar kağıdı": tema rollerinden dört köşeli geçiş (Bar önizlemesi de kullanır).
pub(crate) fn wallpaper_tones(theme: &Theme) -> [nuxut::Color; 4] {
    let c = &theme.colors;
    [
        c.primary.mix(&c.surface, 0.45),
        c.surface_bright.mix(&c.primary, 0.2),
        c.surface.mix(&c.on_surface_variant, 0.2),
        c.primary.mix(&c.surface_container, 0.65),
    ]
}

fn draw_screen(ui: &mut Ui, rect: Rect, insets: Insets, bar: Edge, hover: Option<Edge>) {
    let t = ui.theme;
    let c = &t.colors;
    let inner_radius = t.blob.smoothing * DIAGRAM_SCALE;
    let wall = wallpaper_tones(t);
    ui.painter.gradient_rect(rect, t.shape.small(), wall.map(|w| ui.fade(w)));
    let inner = rect.inset(insets);
    ui.painter.push_clip(rect);
    ui.painter.blob(&BlobField {
        frame: Some(FrameShape { outer: rect, inner, inner_radius }),
        shapes: Vec::new(),
        smoothing: 0.0,
        corner_radius: 0.0,
        fill: ui.fade(c.surface),
        shadow: None,
    });
    if let Some(e) = hover {
        let depth = Edge::get(e, &insets).max(t.space(2));
        ui.painter.rect(e.strip(rect, depth), &RectStyle::filled(ui.fade(state_layer_under(t, c.primary, 0.18, &[c.surface], &[c.on_surface_variant])), Corners::ZERO));
    }
    let strip = bar.strip(rect, Edge::get(bar, &insets));
    let horizontal = matches!(bar, Edge::Top | Edge::Bottom);
    let dot = (if horizontal { strip.height } else { strip.width }) * 0.36;
    if dot > 1.0 {
        for i in 0..4 {
            let (x, y) = if horizontal {
                (strip.x + t.space(2) + i as f32 * dot * 1.8, strip.center().y - dot * 0.5)
            } else {
                (strip.center().x - dot * 0.5, strip.y + t.space(2) + i as f32 * dot * 1.8)
            };
            let col = if i == 0 { c.primary } else { faded_mark(t, c.on_surface_variant, 0.5, &[c.surface]) };
            ui.painter.rect(Rect::new(x, y, dot, dot), &RectStyle::filled(ui.fade(col), Corners::uniform(dot * 0.5)));
        }
    }
    ui.painter.pop_clip();
}

// ---- Bar kenarı ---------------------------------------------------------------------------

/// Bar kenarı satırı (§8: "Ekran kenarı + kompakt şema"): solda başlık, sağda seçili kenarın
/// adı ve küçük ekran şeması; şemanın dört kenarı tıklanır (oklar da seçer). Standart satır
/// yüksekliğinde — açıklama metni ve boş alan yok.
#[derive(Debug)]
pub struct BarEdgePicker {
    desc: &'static SettingDesc,
    title: Label,
    value: Label,
    insets: Animated<Insets>,
    edge: Edge,
    hover: Option<Edge>,
    rect: Rect,
    screen: Rect,
    focus_visible: bool,
}

impl BarEdgePicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let edge = bar_edge(store);
        Self {
            desc,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            value: Label::new(edge.title(), text_style(theme, TextRole::Label)),
            insets: Animated::new(shell_insets(store), theme.motion.spatial),
            edge,
            hover: None,
            rect: Rect::ZERO,
            screen: Rect::ZERO,
            focus_visible: false,
        }
    }

    fn edge_at(&self, p: Point) -> Option<Edge> {
        let r = self.screen.expand(8.0);
        if !r.contains(p) {
            return None;
        }
        // En yakın kenar (şemanın oranına göre normalize).
        let nx = (p.x - self.screen.x) / self.screen.width.max(1.0);
        let ny = (p.y - self.screen.y) / self.screen.height.max(1.0);
        let d = [(Edge::Top, ny), (Edge::Right, 1.0 - nx), (Edge::Bottom, 1.0 - ny), (Edge::Left, nx)];
        d.into_iter().min_by(|a, b| a.1.total_cmp(&b.1)).map(|(e, _)| e)
    }

    fn choose(&mut self, env: &mut Env, e: Edge) {
        if e == self.edge {
            return;
        }
        self.edge = e;
        self.value.set_text(e.title());
        env.push(Action::Set { label: t!("settings-edges-bar-position"), file: FileId::Shell, path: KeyPath::parse("bar.edge"), value: Some(Value::from(e.key())) });
    }
}

const DIAGRAM_SCALE: f32 = 0.22;

impl Block for BarEdgePicker {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, _width: f32) -> f32 {
        let s = compact_diagram_size(env.theme);
        let title = self.title.measure(env.text).height;
        crate::rows::row_height_for(env.theme, s.height.max(title))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let (px, _) = row_padding(env.theme);
        self.rect = rect;
        let s = compact_diagram_size(env.theme);
        self.screen = Rect::new(rect.right() - px - s.width, rect.center().y - s.height * 0.5, s.width, s.height);
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let hover = if e.kind == PointerKind::Leave { None } else { self.edge_at(e.position) };
        let mut repaint = hover != self.hover;
        self.hover = hover;
        if let (PointerKind::Press(MouseButton::Left), Some(edge)) = (e.kind, hover) {
            self.focus_visible = false;
            self.choose(env, edge);
            repaint = true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released {
            return false;
        }
        let edge = match e.key {
            Key::Up => Edge::Top,
            Key::Down => Edge::Bottom,
            Key::Left => Edge::Left,
            Key::Right => Edge::Right,
            _ => return false,
        };
        self.focus_visible = true;
        self.choose(env, edge);
        true
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focus_visible = focused && visible;
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.edge_at(p).map(|_| CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.edge = bar_edge(env.store);
        self.value.set_text(self.edge.title());
        self.insets.set_target(shell_insets(env.store), env.now);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.value.set_style(text_style(theme, TextRole::Label));
        self.insets.set_motion(theme.motion.spatial, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let r = self.rect;
        let th = self.title.measure_in(ui.painter).height;
        self.title.draw(ui.painter, Point::new(r.x + px, r.center().y - th * 0.5), ui.fade(c.on_surface));
        let vs = self.value.measure_in(ui.painter);
        self.value.draw(ui.painter, Point::new(self.screen.x - t.space(3) - vs.width, r.center().y - vs.height * 0.5), ui.fade(c.primary));

        // Küçük şemada kalınlıklar daha az abartılır (ince kenar yine görünür).
        let insets = diagram_insets(self.insets.value(now), DIAGRAM_SCALE * 0.8);
        if !self.insets.is_settled(now) {
            ui.request_frame();
        }
        draw_screen(ui, self.screen, insets, self.edge, self.hover.filter(|h| *h != self.edge));
        if self.focus_visible {
            ui.painter.rect(
                self.screen.expand(3.0),
                &RectStyle { fill: nuxut::Color::TRANSPARENT, radii: t.shape.small(), border: Some(nuxut::Border { width: 2.0, color: c.primary }) },
            );
        }
    }
}

/// Bar kenarı satırının küçük şeması: 16:9, standart satır yüksekliğine iç boşlukla sığar.
fn compact_diagram_size(theme: &Theme) -> nuxut::Size {
    let (_, py) = row_padding(theme);
    let h = nuxut::ui::style::row_height(theme) - py * 2.0;
    nuxut::Size::new((h * 16.0 / 9.0).round(), h)
}

fn diagram_size(theme: &Theme, width: f32) -> nuxut::Size {
    let w = (width * 0.38).clamp(theme.space(40), theme.space(60));
    nuxut::Size::new(w, (w * 9.0 / 16.0).round())
}

// ---- Dört kenarın kalınlığı ----------------------------------------------------------------

/// Satırın durumu: kalınlığını ne belirliyor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeSource {
    /// Bar kenarı: `bar.size`.
    Bar,
    /// Genel `frame.thickness`.
    General,
    /// Elle: `frame.<kenar>`.
    Manual,
}

#[derive(Debug)]
struct EdgeRow {
    edge: Edge,
    label: Label,
    slider: Slider,
    value: Label,
    /// "Genele bağla" — yalnızca elle ayarlı kenarda.
    link: Button,
    source: EdgeSource,
    /// Kaydırıcının yazdığı aralık (bar kenarında `bar.size`'ınki).
    range: (f64, f64),
}

impl EdgeRow {
    fn path(&self) -> KeyPath {
        match self.source {
            EdgeSource::Bar => KeyPath::parse("bar.size"),
            _ => KeyPath::new(["frame", self.edge.key()]),
        }
    }

    fn history_label(&self) -> String {
        match self.source {
            EdgeSource::Bar => t!("settings-edges-bar-thickness"),
            _ => t!("settings-edges-edge-thickness", edge = self.edge.title()),
        }
    }

    fn to_px(&self, v: f32) -> f64 {
        let (lo, hi) = self.range;
        (lo + v as f64 * (hi - lo)).round()
    }

    fn to_norm(&self, px: f64) -> f32 {
        let (lo, hi) = self.range;
        ((px - lo) / (hi - lo).max(1.0)).clamp(0.0, 1.0) as f32
    }
}

/// Kayıttaki kaydırıcı aralığı (şema yüklüyse shell'in sınırlarıyla kesişimi, bkz. `SettingDesc::bounds`).
fn slider_range(id: &str, fallback: (f64, f64)) -> (f64, f64) {
    by_id(id).and_then(SettingDesc::bounds).unwrap_or(fallback)
}

#[derive(Debug)]
pub struct FrameEdges {
    desc: &'static SettingDesc,
    rows: Vec<EdgeRow>,
    insets: Animated<Insets>,
    bar: Edge,
    rect: Rect,
    screen: Rect,
    hover: Option<Edge>,
    dragging: Option<usize>,
}

impl FrameEdges {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let rows = Edge::ALL
            .into_iter()
            .map(|edge| EdgeRow {
                edge,
                label: Label::new(edge.title(), text_style(theme, TextRole::Body)),
                slider: Slider::new(theme, Orientation::Horizontal, 0.0),
                value: Label::new("", text_style(theme, TextRole::Label)),
                link: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Link)).with_label(theme, t!("settings-edges-link-general")),
                source: EdgeSource::General,
                range: (0.0, 1.0),
            })
            .collect();
        let mut s = Self {
            desc,
            rows,
            insets: Animated::new(shell_insets(store), theme.motion.spatial),
            bar: bar_edge(store),
            rect: Rect::ZERO,
            screen: Rect::ZERO,
            hover: None,
            dragging: None,
        };
        s.refresh(theme, store, Instant::now());
        s
    }

    fn refresh(&mut self, theme: &Theme, store: &Store, now: Instant) {
        let base = by_id("frame.thickness").map_or(0.0, |d| d.number(store));
        let bar_size = by_id("bar.size").map_or(0.0, |d| d.number(store));
        let frame_range = slider_range("frame.thickness", (0.0, 64.0));
        let bar_range = slider_range("bar.size", (16.0, 128.0));
        self.bar = bar_edge(store);
        for row in &mut self.rows {
            let o = edge_override(store, row.edge);
            let (source, v) = if row.edge == self.bar {
                (EdgeSource::Bar, bar_size)
            } else if let Some(v) = o {
                (EdgeSource::Manual, v)
            } else {
                (EdgeSource::General, base)
            };
            let range = if source == EdgeSource::Bar { bar_range } else { frame_range };
            if range != row.range {
                row.range = range;
                // Aralık değişti (bar başka kenara geçti): adım sayısı da değişir.
                let rect = row.slider.rect();
                row.slider = Slider::new(theme, Orientation::Horizontal, 0.0).with_steps((range.1 - range.0).max(1.0) as u32);
                row.slider.set_rect(rect);
            }
            row.source = source;
            if self.dragging.is_none() {
                row.slider.set_value(row.to_norm(v), now);
            }
            let px = v.round();
            row.value.set_text(&match source {
                EdgeSource::Bar => t!("settings-edges-value-bar", px = px),
                EdgeSource::Manual => t!("settings-edges-value-manual", px = px),
                EdgeSource::General => t!("settings-edges-value-general", px = px),
            });
        }
        self.insets.set_target(shell_insets(store), now);
    }

    fn row_height(theme: &Theme) -> f32 {
        theme.space(12)
    }
}

impl Block for FrameEdges {
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
        let (_, py) = row_padding(env.theme);
        let rows = Self::row_height(env.theme) * 4.0;
        rows.max(diagram_size(env.theme, width).height) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let ds = diagram_size(t, rect.width);
        self.screen = Rect::new(rect.x + px, rect.y + py + (Self::row_height(t) * 4.0 - ds.height).max(0.0) * 0.5, ds.width, ds.height);
        let x0 = self.screen.right() + t.space(6);
        let label_w = t.space(10);
        let value_w = t.space(22);
        // Metin düğmesinin kendi iç boşluğu var; satır kenarına onunla hizalanır.
        let right = rect.right() - px + t.space(2);
        let link_w = self.rows.iter_mut().map(|r| r.link.preferred_size(env.text, t).width).fold(0.0, f32::max);
        for (i, row) in self.rows.iter_mut().enumerate() {
            let y = rect.y + py + i as f32 * Self::row_height(t);
            let mid = y + Self::row_height(t) * 0.5;
            let sx = x0 + label_w;
            let sw = (right - link_w - t.space(2) - value_w - sx).max(t.space(10));
            let sh = row.slider.preferred_size(t, sw).height;
            row.slider.set_rect(Rect::new(sx, mid - sh * 0.5, sw, sh));
            let ls = row.link.preferred_size(env.text, t);
            row.link.set_rect(Rect::new(right - ls.width, mid - ls.height * 0.5, ls.width, ls.height));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let mut hover = None;
        for (i, row) in self.rows.iter_mut().enumerate() {
            if row.source == EdgeSource::Manual {
                let r = row.link.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some() {
                    env.push(Action::Set { label: row.history_label(), file: FileId::Shell, path: row.path(), value: None });
                    return true;
                }
            }
            let was = row.slider.is_dragging();
            let r = row.slider.pointer(e, now);
            repaint |= r.repaint;
            if row.slider.rect().expand(4.0).contains(e.position) || row.slider.is_dragging() {
                hover = Some(row.edge);
            }
            if let Some(v) = r.event {
                self.dragging = Some(i);
                env.push(Action::Set { label: row.history_label(), file: FileId::Shell, path: row.path(), value: Some(Value::from(row.to_px(v))) });
            }
            if was && !row.slider.is_dragging() {
                self.dragging = None;
                env.push(Action::Seal);
            }
        }
        if hover != self.hover {
            self.hover = hover;
            repaint = true;
        }
        repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rows.iter().find_map(|r| (r.source == EdgeSource::Manual).then(|| r.link.cursor(p)).flatten().or_else(|| r.slider.cursor(p)))
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.theme, env.store, env.now);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for row in &mut self.rows {
            row.label.set_style(text_style(theme, TextRole::Body));
            row.value.set_style(text_style(theme, TextRole::Label));
            row.slider.set_theme(theme, now);
            row.link.set_theme(theme, now);
        }
        self.insets.set_motion(theme.motion.spatial, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let insets = diagram_insets(self.insets.value(now), DIAGRAM_SCALE * 1.6);
        if !self.insets.is_settled(now) {
            ui.request_frame();
        }
        draw_screen(ui, self.screen, insets, self.bar, self.hover);
        let x0 = self.screen.right() + t.space(6);
        for row in &mut self.rows {
            let mid = row.slider.rect().center().y;
            let ls = row.label.measure_in(ui.painter);
            row.label.draw(ui.painter, Point::new(x0, mid - ls.height * 0.5), ui.fade(c.on_surface));
            row.slider.draw(ui);
            // Değer, kalınlığın kaynağını söyler: bar (vurgu), elle (belirgin), genel (soluk).
            let vs = row.value.measure_in(ui.painter);
            let vx = row.slider.rect().right() + t.space(3);
            let col = match row.source {
                EdgeSource::Bar => c.primary,
                EdgeSource::Manual => c.on_surface,
                EdgeSource::General => c.on_surface_variant,
            };
            row.value.draw(ui.painter, Point::new(vx, mid - vs.height * 0.5), ui.fade(col));
            if row.source == EdgeSource::Manual {
                row.link.draw(ui);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with(shell: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("s.toml"), shell).unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, store)
    }

    #[test]
    fn insets_follow_shell_rules() {
        // Bar kenarı bar.size; elle ayarlı kenar kendi değeri; diğerleri genel kalınlık.
        let (_d, store) = store_with("[frame]\nthickness = 8.0\nleft = 20.0\ntop = 3.0\n[bar]\nedge = \"top\"\nsize = 40.0\n");
        let i = shell_insets(&store);
        assert_eq!((i.top, i.right, i.bottom, i.left), (40.0, 8.0, 8.0, 20.0));
        // Çerçeve kapalı: yalnızca bar kenarı kalır.
        let (_d, store) = store_with("[frame]\nenabled = false\nthickness = 8.0\n[bar]\nedge = \"left\"\n");
        let i = shell_insets(&store);
        assert_eq!((i.top, i.right, i.bottom, i.left), (0.0, 0.0, 0.0, 34.0));
    }

    #[test]
    fn rows_report_source_and_write_right_key() {
        let theme = Theme::default();
        let desc = by_id("frame.edges").unwrap();
        let (_d, store) = store_with("[frame]\nright = 12.0\n[bar]\nedge = \"bottom\"\nsize = 50.0\n");
        let fe = FrameEdges::new(&theme, desc, &store);
        let src = |e: Edge| fe.rows.iter().find(|r| r.edge == e).unwrap();
        assert_eq!(src(Edge::Bottom).source, EdgeSource::Bar);
        assert_eq!(src(Edge::Bottom).path(), KeyPath::parse("bar.size"));
        assert_eq!(src(Edge::Bottom).range, (16.0, 128.0));
        assert_eq!(src(Edge::Bottom).to_px(src(Edge::Bottom).slider.value()), 50.0);
        assert_eq!(src(Edge::Right).source, EdgeSource::Manual);
        assert_eq!(src(Edge::Right).path(), KeyPath::new(["frame", "right"]));
        assert_eq!(src(Edge::Top).source, EdgeSource::General);
        assert_eq!(src(Edge::Top).range, (0.0, 64.0));
    }
}
