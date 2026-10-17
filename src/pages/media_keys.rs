//! Hyprland medya tuşları için örnek bind bloğu. Kullanıcının hyprland.conf'una yazılmaz;
//! gösterilir ve "Kopyala" düğmesiyle sistem panosuna alınır.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, TextRole, Ui, text_style};
use nuxut::{Point, Rect, RectStyle, Theme};

use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::store::Store;

/// Shell `msg` sözdizimi: `volume up|down [adım]`, `volume mute`, `mic mute`,
/// `brightness up|down [adım]`. Medya oynatma bu turda playerctl ile. Satırlar Hyprland config
/// sözdizimi olduğundan çevrilmez; gösterilen ve kopyalanan bloktaki Türkçe yorum satırı
/// [`binds_text`] ile katalogdan eklenir (`settings-media-keys-example-comment`).
pub const BINDS: &str = "\
bindel = , XF86AudioRaiseVolume, exec, nuxut-shell msg volume up 5
bindel = , XF86AudioLowerVolume, exec, nuxut-shell msg volume down 5
bindl = , XF86AudioMute, exec, nuxut-shell msg volume mute
bindl = , XF86AudioMicMute, exec, nuxut-shell msg mic mute
bindel = , XF86MonBrightnessUp, exec, nuxut-shell msg brightness up 5
bindel = , XF86MonBrightnessDown, exec, nuxut-shell msg brightness down 5
bindl = , XF86AudioPlay, exec, playerctl play-pause
bindl = , XF86AudioPause, exec, playerctl play-pause
bindl = , XF86AudioNext, exec, playerctl next
bindl = , XF86AudioPrev, exec, playerctl previous";

/// Gösterilen ve panoya kopyalanan örnek blok: `BINDS`'in üstüne katalogdaki yorum satırı.
fn binds_text() -> String {
    format!("{comment}\n{BINDS}", comment = t!("settings-media-keys-example-comment"))
}

#[derive(Debug)]
pub struct MediaKeys {
    desc: &'static SettingDesc,
    title: Label,
    subtitle: Label,
    lines: Vec<Label>,
    copy: Button,
    code: Rect,
    rect: Rect,
}

impl MediaKeys {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        let binds = binds_text();
        Self {
            desc,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(desc.description(), text_style(theme, TextRole::Caption)),
            lines: binds.lines().map(|l| Label::new(l, text_style(theme, TextRole::Mono))).collect(),
            copy: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::Copy)).with_label(theme, t!("settings-media-keys-copy")),
            code: Rect::ZERO,
            rect: Rect::ZERO,
        }
    }

    /// Satırlar kod kutusunun iç genişliğine sarılır (dar pencerede kart dışına taşmaz);
    /// toplam yükseklik.
    fn lines_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let inner = (width - row_padding(t).0 * 2.0 - t.space(3) * 2.0).max(1.0);
        self.lines.iter_mut().map(|l| {
            l.set_max_width(Some(inner));
            l.measure(env.text).height
        }).sum()
    }

    /// Başlık satırı: solda başlık + alt yazı (düğmeye kadar sarılır), sağda "Kopyala";
    /// yükseklik ikisinin büyüğü.
    fn head_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let button = self.copy.preferred_size(env.text, t);
        let text_w = (width - row_padding(t).0 * 2.0 - button.width - t.space(3)).max(1.0);
        self.title.set_max_width(Some(text_w));
        self.subtitle.set_max_width(Some(text_w));
        let text = self.title.measure(env.text).height + t.space(1) + self.subtitle.measure(env.text).height;
        text.max(button.height)
    }
}

impl Block for MediaKeys {
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
        let code = self.lines_height(env, width) + t.space(3) * 2.0;
        py + self.head_height(env, width) + t.space(3) + code + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        self.lines_height(env, rect.width);
        let head = self.head_height(env, rect.width);
        let b = self.copy.preferred_size(env.text, t);
        self.copy.set_rect(Rect::new(rect.right() - px - b.width, rect.y + py + (head - b.height) * 0.5, b.width, b.height));
        let top = rect.y + py + head + t.space(3);
        self.code = Rect::new(rect.x + px, top, rect.width - px * 2.0, rect.bottom() - py - top);
    }

    fn pointer(&mut self, env: &mut Env, event: &PointerEvent) -> bool {
        let r = self.copy.pointer(event, env.now);
        if r.event.is_some() {
            env.push(Action::Copy(format!("{}\n", binds_text())));
        }
        r.repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.copy.cursor(p)
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.copy.set_theme(theme, now);
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        for l in &mut self.lines {
            l.set_style(text_style(theme, TextRole::Mono));
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let (x, y) = (self.rect.x + px, self.rect.y + py);
        let h = self.title.measure_in(ui.painter).height;
        self.title.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
        self.subtitle.draw(ui.painter, Point::new(x, y + h + t.space(1)), ui.fade(c.on_surface_variant));
        self.copy.draw(ui);
        // Kod kutusu kart zemininden yalnızca tonla ayrılır (kenar çizgisi yok).
        ui.painter.rect(self.code, &RectStyle::filled(ui.fade(c.surface), t.shape.small()));
        let mut ly = self.code.y + t.space(3);
        for l in &mut self.lines {
            let lh = l.measure_in(ui.painter).height;
            let color = if l.text().starts_with('#') { c.on_surface_variant } else { c.on_surface };
            l.draw(ui.painter, Point::new(self.code.x + t.space(3), ly), ui.fade(color));
            ly += lh;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BINDS;

    #[test]
    fn binds_are_hyprland_lines() {
        for l in BINDS.lines().filter(|l| !l.starts_with('#')) {
            let (kw, rest) = l.split_once(" = ").expect(l);
            assert!(kw.starts_with("bind"), "{l}");
            let fields: Vec<&str> = rest.split(", ").collect();
            assert_eq!(fields.len(), 4, "{l}");
            assert!(fields[1].starts_with("XF86"), "{l}");
            assert_eq!(fields[2], "exec", "{l}");
        }
    }
}
