//! Henüz yapılmamış özel blok için yer tutucu satır.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::PointerEvent;
use nuxut::ui::{Label, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::store::Store;

#[derive(Debug)]
pub struct Placeholder {
    desc: &'static SettingDesc,
    title: Label,
    note: Label,
    rect: Rect,
}

impl Placeholder {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        Self {
            desc,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            note: Label::new(t!("settings-placeholder-note", subtitle = desc.description()), text_style(theme, TextRole::Caption)),
            rect: Rect::ZERO,
        }
    }
}

impl Block for Placeholder {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store)
    }

    fn height(&mut self, env: &mut Env, _width: f32) -> f32 {
        let (_, py) = row_padding(env.theme);
        self.title.measure(env.text).height + env.theme.space(1) + self.note.measure(env.text).height + py * 2.0
    }

    fn place(&mut self, _env: &mut Env, rect: Rect) {
        self.rect = rect;
    }

    fn pointer(&mut self, _env: &mut Env, _event: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, _now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.note.set_style(text_style(theme, TextRole::Caption));
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, py) = row_padding(t);
        let h = self.title.measure_in(ui.painter).height;
        let c = ui.fade(t.colors.on_surface);
        self.title.draw(ui.painter, Point::new(self.rect.x + px, self.rect.y + py), c);
        let c = ui.fade(t.colors.on_surface_variant);
        self.note.draw(ui.painter, Point::new(self.rect.x + px, self.rect.y + py + h + t.space(1)), c);
    }
}
