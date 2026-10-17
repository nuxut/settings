//! Çok hedefli yönlendirme satırı: ayarlar başka sayfalarda yaşıyorsa tek satır, her hedef için
//! bir metin düğmesi (ör. Pencereler › "Köşe, gölge ve hareket" → Görünüm, Hareket). Kayıtta
//! `CustomId::Alias(<bu satır>)` olan kimlikler kendi satırı olmadan bu satırda yaşar: arama
//! onları da buraya getirir (§3.5 "üç kimlik aramada aynı satırı bulur").

use std::time::Instant;

use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use crate::block::{Action, Block, Env};
use crate::registry::{Control, CustomId, SETTINGS, SettingDesc};
use crate::rows::row_padding;

#[derive(Debug)]
pub struct Links {
    desc: &'static SettingDesc,
    targets: &'static [(&'static str, &'static str)],
    title: Label,
    subtitle: Label,
    buttons: Vec<Button>,
    /// Klavye odağındaki düğme.
    focus: Option<usize>,
    focus_visible: bool,
    rect: Rect,
}

/// `id` bu satırın takma adı mı (`CustomId::Alias(<satır>)`).
pub fn is_alias_of(row: &str, id: &str) -> bool {
    SETTINGS.iter().any(|s| s.id == id && matches!(s.control, Control::Custom(CustomId::Alias(r)) if r == row))
}

impl Links {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, targets: &'static [(&'static str, &'static str)]) -> Self {
        Self {
            desc,
            targets,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(desc.description(), text_style(theme, TextRole::Caption)),
            buttons: targets.iter().map(|(label, _)| Self::button(theme, label)).collect(),
            focus: None,
            focus_visible: false,
            rect: Rect::ZERO,
        }
    }

    fn button(theme: &Theme, label: &str) -> Button {
        Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::ArrowForward)).with_label(theme, label)
    }

    /// Düğmelerin toplam genişliği (aralar dahil).
    fn buttons_width(&mut self, env: &mut Env) -> f32 {
        let t = env.theme;
        let n = self.buttons.len() as f32;
        self.buttons.iter_mut().map(|b| b.preferred_size(env.text, t).width).sum::<f32>() + t.space(1) * (n - 1.0).max(0.0)
    }

    fn text_width(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        (width - px * 2.0 - self.buttons_width(env) - t.space(4)).max(t.space(20))
    }

    fn sync_focus(&mut self) {
        for (i, b) in self.buttons.iter_mut().enumerate() {
            b.set_focus_visible(self.focus_visible && self.focus == Some(i));
        }
    }
}

impl Block for Links {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &crate::store::Store) -> bool {
        self.desc.is_visible(store)
    }

    fn contains_id(&self, id: &str) -> bool {
        id == self.desc.id || is_alias_of(self.desc.id, id)
    }

    fn reveal(&mut self, _env: &mut Env, id: &str) -> Option<(f32, bool)> {
        self.contains_id(id).then_some((0.0, false))
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let bh = self.buttons.iter_mut().map(|b| b.preferred_size(env.text, t).height).fold(0.0, f32::max);
        let tw = self.text_width(env, width);
        self.title.set_max_width(Some(tw));
        self.subtitle.set_max_width(Some(tw));
        let mut text = self.title.measure(env.text).height;
        if !self.desc.subtitle.is_empty() {
            text += t.space(1) + self.subtitle.measure(env.text).height;
        }
        text.max(bh) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        self.rect = rect;
        let tw = self.text_width(env, rect.width);
        self.title.set_max_width(Some(tw));
        self.subtitle.set_max_width(Some(tw));
        // Sağdan sola; metin düğmesinin iç boşluğu satır kenarıyla hizalı (Link gibi).
        let mut x = rect.right() - px + t.space(2);
        for b in self.buttons.iter_mut().rev() {
            let s = b.preferred_size(env.text, t);
            x -= s.width;
            b.set_rect(Rect::new(x, rect.center().y - s.height * 0.5, s.width, s.height));
            x -= t.space(1);
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        for (i, b) in self.buttons.iter_mut().enumerate() {
            let r = b.pointer(e, env.now);
            repaint |= r.repaint;
            if r.event.is_some() {
                env.push(Action::Reveal(self.targets[i].1));
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let Some(i) = self.focus else { return false };
        let r = self.buttons[i].key(e, env.now);
        if r.event.is_some() {
            env.push(Action::Reveal(self.targets[i].1));
        }
        r.repaint || r.event.is_some()
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focus = focused.then_some(self.focus.unwrap_or(0));
        self.focus_visible = focused && visible;
        self.sync_focus();
    }

    /// Tab düğmeler arasında gezer; sondan sonra sayfaya.
    fn focus_step(&mut self, forward: bool, _now: Instant) -> bool {
        let Some(i) = self.focus else { return false };
        let next = if forward { i + 1 } else { i.wrapping_sub(1) };
        if next >= self.buttons.len() {
            return false;
        }
        self.focus = Some(next);
        self.sync_focus();
        true
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.buttons.iter().find_map(|b| b.cursor(p))
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        for b in &mut self.buttons {
            b.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let th = self.title.measure_in(ui.painter).height;
        let sh = if self.desc.subtitle.is_empty() { 0.0 } else { t.space(1) + self.subtitle.measure_in(ui.painter).height };
        let y = self.rect.center().y - (th + sh) * 0.5;
        self.title.draw(ui.painter, Point::new(self.rect.x + px, y), ui.fade(c.on_surface));
        if sh > 0.0 {
            self.subtitle.draw(ui.painter, Point::new(self.rect.x + px, y + th + t.space(1)), ui.fade(c.on_surface_variant));
        }
        for b in &mut self.buttons {
            b.draw(ui);
        }
    }
}
