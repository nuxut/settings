//! Widget listeleri: bar bölümleri (`bar.start`, `bar.center`, `bar.end`) ve kenar panelinin
//! kartları (`panels.<kenar>.widgets`, yukarıdan aşağı). Tutamaçtan sürükleyerek sıralanır,
//! bar'da listeler arasında taşınır, × ile kaldırılır, "+ Ekle" ile katalogdaki uygun
//! widget'lardan eklenir (bar: `bar = true`, panel: `panel = true`; shell sözleşmesi
//! `docs/design/panels.md`). Bar'da satıra tıklamak widget'ı seçer (önizleme ve Seçili widget
//! kartı, sayfa durumu), panelde widget'ın ilk ayarına götürür. Kuyular kenarsızdır (tonla
//! ayrılır, §5.2); adlar yan yana sığmıyorsa sütunlar alt alta dizilir (§4, ad kırpılmaz). Her
//! değişiklik etkilenen listeleri (taşımada ikisini) tek geri al kaydıyla yazar.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, ReorderEvent, ReorderItem, ReorderList, TextRole, Ui, text_style};
use nuxut::{Point, Rect, RectStyle, Theme};
use toml_edit::{Array, Value};

use crate::block::{Action, Block, Env};
use crate::pages::edges::{Edge, bar_edge};
use crate::new_badge::NewBadge;
use crate::registry::SettingDesc;
use crate::services::seen;
use crate::services::widgets;
use crate::rows::row_padding;
use crate::store::{FileId, KeyPath, Store};

/// Bar bölümleri, bar'daki sırasıyla (`bar.start`, `bar.center`, `bar.end`).
pub(crate) const SECTIONS: [&str; 3] = ["start", "center", "end"];

/// Başlık ve ikon shell'in kataloğundan ([`widgets::catalog`]); katalogda yoksa (shell yok,
/// eski sürüm, henüz yüklenmedi) config'teki ad ve genel ikon.
pub(crate) fn widget_info(id: &str) -> (String, IconName) {
    widgets::catalog().iter().find(|w| w.name == id).map_or_else(
        || (id.to_owned(), IconName::Widgets),
        |w| (w.title.clone(), w.icon.parse().unwrap_or(IconName::Widgets)),
    )
}

/// Hangi listeler düzenleniyor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Bar'ın üç bölümü; bir widget bar'da yalnızca bir kez.
    Bar,
    /// `panels.<kenar>.widgets`; aynı kart farklı panellerde olabilir.
    Panel(&'static str),
}

impl Target {
    fn paths(self) -> Vec<KeyPath> {
        match self {
            Target::Bar => SECTIONS.iter().map(|k| KeyPath::new(["bar", k])).collect(),
            Target::Panel(edge) => vec![KeyPath::new(["panels", edge, "widgets"])],
        }
    }

    fn label(self) -> String {
        match self {
            Target::Bar => t!("settings-sections-bar-widgets"),
            Target::Panel(_) => t!("settings-sections-panel-cards"),
        }
    }

    /// Boş kuyuda gösterilen kısa açıklama (boş kenarsız kuyu bozuk görünüyordu).
    fn empty_hint(self) -> String {
        match self {
            Target::Bar => t!("settings-sections-empty-bar"),
            Target::Panel(_) => t!("settings-sections-empty-panel"),
        }
    }

    fn accepts(self, w: &widgets::WidgetSpec) -> bool {
        match self {
            Target::Bar => w.bar,
            Target::Panel(_) => w.panel,
        }
    }
}

pub(crate) fn read_list(store: &Store, path: &KeyPath) -> Vec<String> {
    let value = store.get(FileId::Shell, path).or_else(|| crate::defaults::get(store, FileId::Shell, path));
    value.and_then(Value::as_array).map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect()).unwrap_or_default()
}

fn item(id: &str) -> ReorderItem {
    let (title, icon) = widget_info(id);
    ReorderItem::new(title).with_icon(Icon::named(icon))
}

fn to_value(ids: &[String]) -> Value {
    Value::Array(Array::from_iter(ids.iter().map(String::as_str)))
}

#[derive(Debug)]
struct Column {
    path: KeyPath,
    header: Label,
    ids: Vec<String>,
    list: ReorderList,
    add: Button,
    rect: Rect,
}

#[derive(Debug)]
pub struct WidgetLists {
    desc: &'static SettingDesc,
    target: Target,
    rect: Rect,
    columns: Vec<Column>,
    /// "+ Ekle" açık olan sütun.
    adding: Option<usize>,
    chips: Vec<(String, Button)>,
    /// Ekleme menüsündeki widget'ların `since`'i (`chips` ile aynı sıra; "Yeni" rozeti).
    chip_since: Vec<Option<i64>>,
    /// "Yeni" rozeti (ekleme menüsünde düğmenin sağ üst köşesi; kimlik `widget:<ad>`).
    badge: NewBadge,
    cancel: Button,
    empty: Label,
    /// Boş sütun kuyusunun açıklaması ([`Target::empty_hint`]).
    well_hint: Label,
    /// Bar'da satıra tıklanınca seçilecek widget (sayfa durumuna [`Block::take_ui`] ile).
    pending_select: Option<String>,
    /// Bir sütunun adları kırpmadan sığması için gereken genişlik (§4); daha darsa sütunlar
    /// alt alta dizilir. `None`: yeniden ölçülecek.
    min_column: Option<f32>,
    /// Ölçüm için (adlar listedeki satır yazısıyla aynı rolde).
    probe: Label,
}

impl WidgetLists {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store, target: Target) -> Self {
        let columns = target
            .paths()
            .into_iter()
            .map(|path| {
                let ids = read_list(store, &path);
                Column {
                    path,
                    header: Label::new("", text_style(theme, TextRole::Label)),
                    list: ReorderList::new(theme, ids.iter().map(|i| item(i)).collect()),
                    ids,
                    add: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Add)).with_label(theme, t!("settings-sections-add")),
                    rect: Rect::ZERO,
                }
            })
            .collect();
        let mut s = Self {
            desc,
            target,
            rect: Rect::ZERO,
            columns,
            adding: None,
            chips: Vec::new(),
            chip_since: Vec::new(),
            badge: NewBadge::new(theme),
            cancel: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-sections-cancel")),
            empty: Label::new("", text_style(theme, TextRole::Caption)),
            well_hint: Label::new(target.empty_hint(), text_style(theme, TextRole::Caption)),
            pending_select: None,
            min_column: None,
            probe: Label::new("", text_style(theme, TextRole::Body)),
        };
        s.set_headers(bar_edge(store));
        s
    }

    fn set_headers(&mut self, edge: Edge) {
        let names: [String; 3] = match self.target {
            Target::Bar if matches!(edge, Edge::Left | Edge::Right) => {
                [t!("settings-edge-top"), t!("settings-sections-center"), t!("settings-edge-bottom")]
            }
            Target::Bar => [t!("settings-edge-left"), t!("settings-sections-center"), t!("settings-edge-right")],
            Target::Panel(_) => [self.desc.label().to_owned(), String::new(), String::new()],
        };
        for (c, n) in self.columns.iter_mut().zip(names) {
            c.header.set_text(&n);
        }
    }

    /// Eklemede dışarıda kalanlar: bar'da tüm bölümler, panelde yalnız o panel.
    fn used(&self) -> Vec<&str> {
        self.columns.iter().flat_map(|c| c.ids.iter().map(String::as_str)).collect()
    }

    fn open_add(&mut self, theme: &Theme, col: usize) {
        self.adding = Some(col);
        let used = self.used();
        let target = self.target;
        let catalog = widgets::catalog();
        let offered: Vec<&widgets::WidgetSpec> = catalog.iter().filter(|w| target.accepts(w) && !used.contains(&w.name.as_str())).collect();
        self.chip_since = offered.iter().map(|w| w.since).collect();
        self.chips = offered
            .iter()
            .map(|w| {
                let (title, icon) = widget_info(&w.name);
                (w.name.clone(), Button::new(theme, ButtonVariant::Outlined).with_icon(theme, Icon::named(icon)).with_label(theme, &title))
            })
            .collect();
        let all_in = match target {
            Target::Bar => t!("settings-sections-all-in-bar"),
            Target::Panel(_) => t!("settings-sections-all-in-panel"),
        };
        let text = match (widgets::catalog().is_empty(), widgets::load_state()) {
            (false, _) => all_in,
            (true, None) => t!("settings-sections-loading"),
            (true, Some(Err(e))) => e.message(),
            (true, Some(Ok(()))) => widgets::LoadError::Unparsable.message(),
        };
        self.empty.set_text(&text);
    }

    fn write(&self, env: &mut Env, cols: &[usize]) {
        let writes = cols
            .iter()
            .map(|&i| {
                let c = &self.columns[i];
                (FileId::Shell, c.path.clone(), Some(to_value(&c.ids)))
            })
            .collect();
        env.push(Action::Group { label: self.target.label().to_owned(), writes });
    }

    /// Kuyu yüksekliği: yan yana düzende en uzun listeninki (sütunlar aynı boyda), alt alta
    /// düzende her sütun kendi listesi kadar. Birden çok sütunda altta bir satırlık bırakma yeri
    /// (başka sütundan sürüklenen öğe sona bırakılır); tek listede (panel içeriği) yalnızca
    /// sıralama var, son satıra bırakmak yeter — boş kuyu payı yok.
    fn list_height(&self, theme: &Theme, col: Option<usize>) -> f32 {
        let lists = self.columns.iter().enumerate().filter(|(i, _)| col.is_none_or(|c| c == *i));
        let drop = if self.columns.len() > 1 { theme.space(12) } else { theme.space(2) };
        lists.map(|(_, c)| c.list.preferred_height(theme)).fold(theme.space(12), f32::max) + drop
    }

    /// En uzun adın kırpılmadan sığdığı sütun genişliği: tutamaç, ikon, ad, kaldır düğmesi
    /// (framework `ReorderList` satırının ölçüleri) ve kuyunun iç boşluğu.
    fn min_column(&mut self, env: &mut Env) -> f32 {
        if let Some(w) = self.min_column {
            return w;
        }
        let t = env.theme;
        let mut widest: f32 = 0.0;
        for c in &self.columns {
            for id in &c.ids {
                self.probe.set_text(&widget_info(id).0);
                widest = widest.max(self.probe.measure(env.text).width);
            }
        }
        // Tutamaç space(10) + ikon ve boşluğu ~space(9) + kaldır space(8) + kenarlar.
        let w = widest + t.space(10) + t.space(9) + t.space(8) + t.space(6);
        self.min_column = Some(w);
        w
    }

    /// Sütunlar alt alta mı (yan yana sığmıyor; §4: adlar kırpılmaz).
    fn stacked(&mut self, env: &mut Env, inner_width: f32) -> bool {
        let n = self.columns.len() as f32;
        if n <= 1.0 {
            return false;
        }
        let cw = (inner_width - env.theme.space(3) * (n - 1.0)) / n;
        cw < self.min_column(env)
    }

    /// Bir sütunun yüksekliği: başlık + kuyu + "Ekle" düğmesi.
    fn column_height(&self, env: &mut Env, header: f32, col: Option<usize>) -> f32 {
        header + self.list_height(env.theme, col) + env.theme.space(10)
    }

    fn chips_height(&mut self, env: &mut Env, width: f32) -> f32 {
        if self.adding.is_none() {
            return 0.0;
        }
        let t = env.theme;
        let (mut x, mut rows) = (0.0f32, 1.0f32);
        for (_, b) in &mut self.chips {
            let s = b.preferred_size(env.text, t);
            if x > 0.0 && x + s.width > width {
                x = 0.0;
                rows += 1.0;
            }
            x += s.width + t.space(2);
        }
        t.space(4) + rows * (t.space(10) + t.space(2)) + t.space(10)
    }
}

impl Block for WidgetLists {
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
        let header = self.columns[0].header.measure(env.text).height + t.space(2);
        let columns = if self.stacked(env, width - px * 2.0) {
            let n = self.columns.len();
            (0..n).map(|i| self.column_height(env, header, Some(i))).sum::<f32>() + t.space(4) * (n - 1) as f32
        } else {
            self.column_height(env, header, None)
        };
        py + columns + self.chips_height(env, width - px * 2.0) + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let now = env.now;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let gap = t.space(3);
        let w = rect.width - px * 2.0;
        let stacked = self.stacked(env, w);
        let n = self.columns.len() as f32;
        let cw = if stacked { w } else { (w - gap * (n - 1.0)) / n };
        let header = self.columns[0].header.measure(env.text).height + t.space(2);
        let shared = self.list_height(t, None);
        let heights: Vec<f32> = (0..self.columns.len()).map(|i| if stacked { self.list_height(t, Some(i)) } else { shared }).collect();
        let mut y = rect.y + py;
        for (i, c) in self.columns.iter_mut().enumerate() {
            let lh = heights[i];
            let x = if stacked { rect.x + px } else { rect.x + px + i as f32 * (cw + gap) };
            c.rect = Rect::new(x, y, cw, header + lh);
            let list_rect = Rect::new(x + t.space(1), y + header + t.space(1), cw - t.space(2), lh - t.space(2));
            c.list.set_rect(t, list_rect, now);
            let s = c.add.preferred_size(env.text, t);
            c.add.set_rect(Rect::new(x, c.rect.bottom() + t.space(1), s.width, s.height));
            if stacked {
                y = c.rect.bottom() + t.space(10) + t.space(4);
            }
        }
        if self.adding.is_some() {
            let top = self.columns.iter().map(|c| c.add.rect().bottom()).fold(0.0, f32::max) + t.space(4);
            let (mut x, mut y) = (rect.x + px, top);
            for (_, b) in &mut self.chips {
                let s = b.preferred_size(env.text, t);
                if x > rect.x + px && x + s.width > rect.right() - px {
                    x = rect.x + px;
                    y += t.space(10) + t.space(2);
                }
                b.set_rect(Rect::new(x, y, s.width, s.height));
                x += s.width + t.space(2);
            }
            let cs = self.cancel.preferred_size(env.text, t);
            let cy = if self.chips.is_empty() { top } else { y + t.space(10) + t.space(2) };
            self.cancel.set_rect(Rect::new(rect.x + px - t.space(2), cy, cs.width, cs.height));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let t = env.theme;
        let now = env.now;
        let mut repaint = false;
        // Ekleme haplari.
        if self.adding.is_some() {
            let mut picked = None;
            for (id, b) in &mut self.chips {
                let r = b.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some() {
                    picked = Some(id.clone());
                }
            }
            let r = self.cancel.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.adding = None;
                return true;
            }
            if let (Some(id), Some(col)) = (picked, self.adding) {
                let c = &mut self.columns[col];
                let at = c.ids.len();
                c.list.insert(t, at, item(&id), now);
                c.ids.push(id.clone());
                self.adding = None;
                self.write(env, &[col]);
                return true;
            }
        }
        let mut action: Option<(usize, ReorderEvent)> = None;
        for (i, c) in self.columns.iter_mut().enumerate() {
            let r = c.list.pointer(t, e, now);
            repaint |= r.repaint;
            if let Some(ev) = r.event {
                action = Some((i, ev));
            }
            let r = c.add.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.open_add(t, i);
                return true;
            }
        }
        let Some((col, ev)) = action else { return repaint };
        match ev {
            ReorderEvent::Moved { from, to } => {
                let ids = &mut self.columns[col].ids;
                if from < ids.len() {
                    let id = ids.remove(from);
                    ids.insert(to.min(ids.len()), id);
                }
                self.write(env, &[col]);
            }
            ReorderEvent::Removed(i) => {
                if i < self.columns[col].ids.len() {
                    let id = self.columns[col].ids.remove(i);
                    self.write(env, &[col]);
                    let _ = id;
                }
            }
            // Bar'da satır tıklandı: widget seçilir (önizleme ve "Seçili widget" kartı ona
            // geçer) ve sayfa karta kayar.
            ReorderEvent::Activated(i) if self.target == Target::Bar => {
                let Some(id) = self.columns[col].ids.get(i) else { return repaint };
                self.pending_select = Some(id.clone());
                env.push(Action::Reveal("bar.selected"));
            }
            // Panelde: widget'ın ilk ayarına git (ayarı yoksa bir şey olmaz).
            ReorderEvent::Activated(i) => {
                let Some(id) = self.columns[col].ids.get(i) else { return repaint };
                let prefix = format!("bar.widget.{id}.");
                match crate::registry::SETTINGS.iter().find(|d| d.path.starts_with(&prefix)) {
                    Some(d) => env.push(Action::Reveal(d.id)),
                    None => return repaint,
                }
            }
            ReorderEvent::DraggedOut { index, position } => {
                let target = self.columns.iter().position(|c| c.rect.expand(t.space(2)).contains(position));
                if let Some(to) = target
                    && to != col
                    && index < self.columns[col].ids.len()
                {
                    let id = self.columns[col].ids.remove(index);
                    let _ = self.columns[col].list.remove(t, index, now);
                    let dst = &mut self.columns[to];
                    let pitch = t.space(12) + t.space(1);
                    let at = (((position.y - dst.list.rect().y) / pitch).round().max(0.0) as usize).min(dst.ids.len());
                    dst.list.insert(t, at, item(&id), now);
                    dst.ids.insert(at, id.clone());
                    self.write(env, &[col, to]);
                }
            }
        }
        true
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        let cols = self.columns.iter().find_map(|c| c.list.cursor(p).or_else(|| c.add.cursor(p)));
        cols.or_else(|| self.adding.and_then(|_| self.chips.iter().find_map(|(_, b)| b.cursor(p)).or_else(|| self.cancel.cursor(p))))
    }

    fn sync(&mut self, env: &mut Env) {
        let t = env.theme;
        self.set_headers(bar_edge(env.store));
        for c in &mut self.columns {
            let ids = read_list(env.store, &c.path);
            if c.ids != ids {
                c.ids = ids;
                c.list = ReorderList::new(t, c.ids.iter().map(|i| item(i)).collect());
                self.min_column = None;
            }
        }
    }

    fn take_ui(&mut self, ui: &mut crate::page_ui::PageUi) -> bool {
        let Some(name) = self.pending_select.take() else { return false };
        ui.selected_widget = Some(name);
        true
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if !matches!(event, crate::services::Event::Widgets) {
            return false;
        }
        // Katalog geldi: başlık ve ikonlar shell'inkine döner.
        self.min_column = None;
        for c in &mut self.columns {
            c.list = ReorderList::new(env.theme, c.ids.iter().map(|i| item(i)).collect());
        }
        if let Some(col) = self.adding {
            self.open_add(env.theme, col);
        }
        true
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for c in &mut self.columns {
            c.header.set_style(text_style(theme, TextRole::Label));
            c.list.set_theme(theme, now);
            c.add.set_theme(theme, now);
        }
        for (_, b) in &mut self.chips {
            b.set_theme(theme, now);
        }
        self.cancel.set_theme(theme, now);
        self.badge.set_theme(theme, now);
        self.empty.set_style(text_style(theme, TextRole::Caption));
        self.well_hint.set_style(text_style(theme, TextRole::Caption));
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        for col in &mut self.columns {
            let hh = col.header.measure_in(ui.painter).height;
            col.header.draw(ui.painter, Point::new(col.rect.x + t.space(1), col.rect.y), ui.fade(c.on_surface_variant));
            let well = Rect::new(col.rect.x, col.rect.y + hh + t.space(2), col.rect.width, col.rect.height - hh - t.space(2));
            // Kuyu tonla ayrılır, kenarsız (§5.2).
            ui.painter.rect(well, &RectStyle::filled(ui.fade(c.surface), t.shape.small()));
            col.list.draw(ui);
            if col.list.is_empty() {
                // Kuyunun iç genişliğinde sarılır; soldan satır boşluğuyla, dikeyde ortada.
                self.well_hint.set_max_width(Some(well.width - t.space(4) * 2.0));
                let h = self.well_hint.measure_in(ui.painter).height;
                let at = Point::new(well.x + t.space(4), well.y + (well.height - h) / 2.0);
                self.well_hint.draw(ui.painter, at, ui.fade(c.on_surface_variant));
            }
            col.add.draw(ui);
        }
        if self.adding.is_some() {
            if self.chips.is_empty() {
                let at = Point::new(self.rect.x + row_padding(t).0, self.cancel.rect().y - t.space(5));
                self.empty.draw(ui.painter, at, ui.fade(c.on_surface_variant));
            }
            for ((name, b), since) in self.chips.iter_mut().zip(&self.chip_since) {
                b.draw(ui);
                // "Yeni" rozeti düğmenin sağ üst köşesinde; menüde görünmesi görülme sayılır.
                let id = seen::widget_id(name);
                if seen::badge(&id, *since) {
                    let r = b.rect();
                    let pill = self.badge.draw_at_corner(ui, Point::new(r.right(), r.y));
                    seen::sight(&id, *since, r.union(&pill));
                }
            }
            self.cancel.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(bar: bool, panel: bool) -> widgets::WidgetSpec {
        widgets::WidgetSpec { name: "x".into(), title: "X".into(), icon: String::new(), bar, panel, settings: Vec::new(), since: None }
    }

    /// §4: adlar kırpılmaz — dar kartta bar sütunları alt alta, her biri tam genişlikte;
    /// genişte yan yana. Bar'da satıra tıklamak widget'ı seçer (sayfa durumuna bir kez).
    #[test]
    fn bar_columns_stack_when_narrow_and_rows_select() {
        use std::time::Instant;
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let desc = crate::registry::by_id("bar.sections").unwrap();
        let mut w = WidgetLists::new(&theme, desc, &store, Target::Bar);
        let wide = w.height(&mut env, 1000.0);
        w.place(&mut env, Rect::new(0.0, 0.0, 1000.0, wide));
        assert!(w.columns[0].rect.y == w.columns[2].rect.y && w.columns[0].rect.right() < w.columns[1].rect.x, "geniş: yan yana");
        let narrow = w.height(&mut env, 360.0);
        w.place(&mut env, Rect::new(0.0, 0.0, 360.0, narrow));
        assert!(narrow > wide, "dar: alt alta, daha uzun");
        assert!(w.columns.windows(2).all(|c| c[0].rect.bottom() < c[1].rect.y), "dar: alt alta");
        assert!(w.columns.iter().all(|c| c.rect.width >= w.min_column.unwrap()), "dar: ad sığar");
        w.pending_select = Some("clock".into());
        let mut ui = crate::page_ui::PageUi::default();
        assert!(w.take_ui(&mut ui) && ui.selected_widget.as_deref() == Some("clock"));
        assert!(!w.take_ui(&mut ui));
    }

    #[test]
    fn targets_write_their_own_keys() {
        assert_eq!(Target::Bar.paths(), ["bar.start", "bar.center", "bar.end"].map(KeyPath::parse));
        assert_eq!(Target::Panel("left").paths(), [KeyPath::parse("panels.left.widgets")]);
        assert!(Target::Bar.accepts(&spec(true, false)) && !Target::Bar.accepts(&spec(false, true)));
        assert!(Target::Panel("right").accepts(&spec(false, true)) && !Target::Panel("right").accepts(&spec(true, false)));
    }
}
