//! Panel kartındaki "Bar'dan açma" satırı (`docs/design/settings-ux.md` §7): bar'ın bir uç
//! eylemi bu paneli açıyorsa hangi widget'a hangi tıkla açıldığını söyler ("Sağ uçtaki widget
//! (Oturum) · sol tık"); "Ayarla" o uç eyleminin satırına gider (Bar sayfası; seçili widget
//! kartı geldiğinde arama adımı o widget'ı seçer). Uç eylemi bar'ın ayarıdır; burada yalnızca
//! gösterilir.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use crate::block::{Action, Block, Env};
use crate::pages::bar_preview::{BarEnd, end_widget};
use crate::registry::{SettingDesc, bar_actions_opening, by_id};
use crate::rows::row_padding;
use crate::services::widgets;
use crate::store::Store;

#[derive(Debug)]
pub struct PanelHint {
    desc: &'static SettingDesc,
    edge: &'static str,
    title: Label,
    text: Label,
    go: Button,
    /// "Ayarla"nın götürdüğü uç eylemi ayarı.
    target: Option<String>,
    rect: Rect,
}

fn widget_title(id: &str) -> String {
    widgets::catalog().iter().find(|w| w.name == id).map_or_else(|| id.to_owned(), |w| w.title.clone())
}

/// Satır metni ve ilk eylemin ayar kimliği; bu paneli açan eylem yoksa `None`.
pub fn hint_text(store: &Store, edge: &str) -> Option<(String, String)> {
    let actions = bar_actions_opening(store, edge);
    let (first_end, first_click) = *actions.first()?;
    let vertical = by_id("bar.edge").is_some_and(|d| matches!(d.string(store).as_str(), "left" | "right"));
    let parts: Vec<String> = actions
        .iter()
        .map(|&(end, click)| {
            let widget = end_widget(store, if end == "start" { BarEnd::Start } else { BarEnd::End });
            let name = widget.map(|w| format!(" ({})", widget_title(&w))).unwrap_or_default();
            match (end, vertical, click == "left_click") {
                ("start", false, true) => t!("settings-panel-hint-left-left-click", widget = name),
                ("start", false, false) => t!("settings-panel-hint-left-right-click", widget = name),
                ("start", true, true) => t!("settings-panel-hint-top-left-click", widget = name),
                ("start", true, false) => t!("settings-panel-hint-top-right-click", widget = name),
                (_, false, true) => t!("settings-panel-hint-right-left-click", widget = name),
                (_, false, false) => t!("settings-panel-hint-right-right-click", widget = name),
                (_, true, true) => t!("settings-panel-hint-bottom-left-click", widget = name),
                (_, true, false) => t!("settings-panel-hint-bottom-right-click", widget = name),
            }
        })
        .collect();
    Some((parts.join("; "), format!("bar.edge_actions.{first_end}.{first_click}")))
}

impl PanelHint {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, edge: &'static str, store: &Store) -> Self {
        let mut s = Self {
            desc,
            edge,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            text: Label::new("", text_style(theme, TextRole::Caption)),
            go: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::ArrowForward)).with_label(theme, t!("settings-link-configure")),
            target: None,
            rect: Rect::ZERO,
        };
        s.refresh(store);
        s
    }

    fn refresh(&mut self, store: &Store) {
        let hint = hint_text(store, self.edge);
        self.text.set_text(hint.as_ref().map_or("", |(t, _)| t.as_str()));
        self.target = hint.map(|(_, id)| id);
    }

    fn text_width(&self, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        (width - px * 2.0 - self.go.rect().width - theme.space(4)).max(theme.space(20))
    }

    fn go(&self, env: &mut Env) {
        if let Some(id) = self.target.as_deref().and_then(by_id) {
            env.push(Action::Reveal(id.id));
        }
    }
}

impl Block for PanelHint {
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
        let (_, py) = row_padding(t);
        let bs = self.go.preferred_size(env.text, t);
        self.go.set_rect(Rect::new(0.0, 0.0, bs.width, bs.height));
        self.text.set_max_width(Some(self.text_width(t, width)));
        let text = self.title.measure(env.text).height + t.space(1) + self.text.measure(env.text).height;
        text.max(bs.height) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        self.rect = rect;
        let bs = self.go.preferred_size(env.text, t);
        self.go.set_rect(Rect::new(rect.right() - px - bs.width + t.space(2), rect.center().y - bs.height * 0.5, bs.width, bs.height));
        self.text.set_max_width(Some(self.text_width(t, rect.width)));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.go.pointer(e, env.now);
        if r.event.is_some() {
            self.go(env);
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.go.key(e, env.now);
        if r.event.is_some() {
            self.go(env);
        }
        r.repaint || r.event.is_some()
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.go.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.go.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.text.set_style(text_style(theme, TextRole::Caption));
        self.go.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let th = self.title.measure_in(ui.painter).height;
        let sh = self.text.measure_in(ui.painter).height;
        let y = self.rect.center().y - (th + t.space(1) + sh) * 0.5;
        self.title.draw(ui.painter, Point::new(self.rect.x + px, y), ui.fade(c.on_surface));
        self.text.draw(ui.painter, Point::new(self.rect.x + px, y + th + t.space(1)), ui.fade(c.on_surface_variant));
        self.go.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{FileId, KeyPath};
    use toml_edit::Value;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    #[test]
    fn hint_names_edge_click_and_target() {
        crate::i18n::use_turkish_for_tests();
        let (_d, mut store) = store();
        assert!(hint_text(&store, "right").is_none(), "varsayılan: uç eylemi widget'ın kendisi");
        assert!(!by_id("panels.right.from_bar").unwrap().is_visible(&store));
        store.set(FileId::Shell, &KeyPath::parse("bar.edge_actions.end.left_click"), Some(Value::from("panel:right")));
        let (text, target) = hint_text(&store, "right").unwrap();
        assert!(text.starts_with("Sağ uçtaki widget") && text.ends_with("sol tık"), "{text}");
        assert_eq!(target, "bar.edge_actions.end.left_click");
        assert!(by_id("panels.right.from_bar").unwrap().is_visible(&store));
        assert!(hint_text(&store, "left").is_none());
        // Dikey bar: uçlar üst/alt.
        store.set(FileId::Shell, &KeyPath::parse("bar.edge"), Some(Value::from("left")));
        store.set(FileId::Shell, &KeyPath::parse("bar.edge_actions.start.right_click"), Some(Value::from("panel:right")));
        let (text, target) = hint_text(&store, "right").unwrap();
        assert!(text.starts_with("Üst uçtaki widget") && text.contains("sağ tık") && text.contains("; Alt uçtaki"), "{text}");
        assert_eq!(target, "bar.edge_actions.start.right_click", "ilk eylem (kayıt sırası)");
        // Panel kapalıyken satır yok.
        store.set(FileId::Shell, &KeyPath::parse("panels.right.enabled"), Some(Value::from(false)));
        assert!(!by_id("panels.right.from_bar").unwrap().is_visible(&store));
    }

    /// "Ayarla" uç eyleminin satırına gider; Bar sayfası arama adımıyla o uçtaki widget'ı seçer
    /// (`SEL_END`, `PageUi::selected_widget`).
    #[test]
    fn go_selects_the_edge_widget_on_bar() {
        use crate::page::Page;
        use crate::registry::PageId;
        let (_d, mut store) = store();
        store.set(FileId::Shell, &KeyPath::parse("bar.edge_actions.end.left_click"), Some(Value::from("panel:right")));
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        let hint = PanelHint::new(&theme, by_id("panels.right.from_bar").unwrap(), "right", env.store);
        hint.go(&mut env);
        let Some(Action::Reveal(id)) = actions.pop() else { panic!("Reveal yok") };
        assert_eq!(id, "bar.edge_actions.end.left_click");
        let mut page = Page::new(&theme, PageId::Bar, crate::pages::build(&theme, PageId::Bar, &store), &store);
        page.set_viewport(Rect::new(0.0, 0.0, 900.0, 600.0));
        page.update_ui(&mut Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions }, |ui| ui.selected_widget = Some("clock".into()));
        page.reveal(&mut Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions }, id);
        assert_eq!(page.ui().selected_widget, end_widget(&store, BarEnd::End), "sağ uçtaki widget seçili");
        assert!(page.ui().selected_widget.is_some());
    }
}
