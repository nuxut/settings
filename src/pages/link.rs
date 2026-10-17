//! Yönlendirme satırı: ayar başka bir sayfada yaşıyorsa (ör. çerçevenin iç köşeleri temanın
//! birleşim yumuşaklığıdır) başlık + açıklama + "Ayarla" düğmesi; düğme hedef ayarı açar.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::store::Store;

#[derive(Debug)]
pub struct Link {
    desc: &'static SettingDesc,
    target: &'static str,
    title: Label,
    subtitle: Label,
    go: Button,
    rect: Rect,
}

impl Link {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, target: &'static str, store: &Store) -> Self {
        Self {
            desc,
            target,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            // Duruma bağlı alt metin (ör. Erişilebilirlik'teki kısayol yönlendirmesi tuşları gösterir).
            subtitle: Label::new(desc.subtitle_in(store), text_style(theme, TextRole::Caption)),
            go: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::ArrowForward)).with_label(theme, t!("settings-link-configure")),
            rect: Rect::ZERO,
        }
    }

    fn text_width(&self, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        (width - px * 2.0 - self.go.rect().width - theme.space(4)).max(theme.space(20))
    }
}

impl Block for Link {
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
        self.subtitle.set_max_width(Some(self.text_width(t, width)));
        let text = self.title.measure(env.text).height + t.space(1) + self.subtitle.measure(env.text).height;
        text.max(bs.height) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        self.rect = rect;
        let bs = self.go.preferred_size(env.text, t);
        // Metin düğmesinin iç boşluğu satır kenarıyla hizalansın (Hakkında'daki gibi).
        self.go.set_rect(Rect::new(rect.right() - px - bs.width + t.space(2), rect.center().y - bs.height * 0.5, bs.width, bs.height));
        self.subtitle.set_max_width(Some(self.text_width(t, rect.width)));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.go.pointer(e, env.now);
        if r.event.is_some() {
            env.push(Action::Reveal(self.target));
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.go.key(e, env.now);
        if r.event.is_some() {
            env.push(Action::Reveal(self.target));
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
        self.subtitle.set_text(&self.desc.subtitle_in(env.store));
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        self.go.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let th = self.title.measure_in(ui.painter).height;
        let sh = self.subtitle.measure_in(ui.painter).height;
        let y = self.rect.center().y - (th + t.space(1) + sh) * 0.5;
        self.title.draw(ui.painter, Point::new(self.rect.x + px, y), ui.fade(c.on_surface));
        self.subtitle.draw(ui.painter, Point::new(self.rect.x + px, y + th + t.space(1)), ui.fade(c.on_surface_variant));
        self.go.draw(ui);
    }
}
