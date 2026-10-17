//! Hareket ön ayarı (`[motion] preset`): dört kart — Sakin (varsayılan), Canlı, Çevik, Kapalı.
//! Her kartta küçük canlı önizleme: bar'dan büyüyen panel, o ön ayarın `spatial` hareketi ve
//! esnemesiyle; imleç karta girince bir kez oynar (boşta iş yok).
//!
//! Alt satırlarda rol değeri ya da stil elle yazılmışsa (`MotionConfig::is_customized` ile
//! aynı kural: `motion.style` ve rol anahtarları; `enabled`/`speed` sayılmaz) "Özelleştirilmiş"
//! rozeti ve "Karaktere dön" görünür; o anahtarlar tek geri al adımında silinir.

use std::time::Instant;

use nuxut::motion::{Animated, Deform};
use nuxut::theme::{FrameworkConfig, MotionConfig, MotionPreset};
use nuxut::ui::input::{CursorShape, PointerEvent, PointerKind};
use nuxut::ui::style::{HAIRLINE, faded_mark, state_layer};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{BlobField, BlobShape, Border, Corners, Point, Rect, RectStyle, Theme};
use nuxut::i18n::t;
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::registry::{MotionRole, SettingDesc, by_id};
use crate::rows::row_padding;
use crate::store::{FileId, KeyPath, Store};

const ROLE_FIELDS: [&str; 5] = ["style", "response", "damping", "easing", "duration_ms"];

/// Kart anahtarı (`settings-motion-preset-<ad>`); açıklama `.description` özniteliğinde.
fn key(p: MotionPreset) -> &'static str {
    match p {
        MotionPreset::Calm => "settings-motion-preset-calm",
        MotionPreset::Lively => "settings-motion-preset-lively",
        MotionPreset::Snappy => "settings-motion-preset-snappy",
        MotionPreset::Instant => "settings-motion-preset-instant",
    }
}

fn title(p: MotionPreset) -> String {
    nuxut::i18n::current().text(key(p)).into_owned()
}

fn subtitle(p: MotionPreset) -> String {
    nuxut::i18n::current().attr(key(p), "description", None).map_or_else(String::new, |s| s.into_owned())
}

/// Ön ayarın üstüne yazılmış anahtarlar (dosyada olanlar).
pub fn custom_keys(store: &Store) -> Vec<KeyPath> {
    let mut keys = vec![KeyPath::parse("motion.style")];
    for role in MotionRole::ALL {
        keys.extend(ROLE_FIELDS.iter().map(|f| KeyPath::new(["motion", role.key(), f])));
    }
    keys.retain(|k| store.get(FileId::Framework, k).is_some());
    keys
}

/// Ön ayarın önizleme teması (kullanıcının hız çarpanıyla).
fn preset_theme(p: MotionPreset, speed: f32) -> Theme {
    let cfg = FrameworkConfig { motion: MotionConfig { speed, ..MotionConfig::for_preset(p) }, ..Default::default() };
    Theme::from_config(&cfg).0
}

#[derive(Debug)]
struct Card {
    preset: MotionPreset,
    title: Label,
    sub: Label,
    hover: Interaction,
    rect: Rect,
    /// Panel açıklığı (0 → 1), ön ayarın `spatial` hareketi.
    open: Animated<f32>,
    deform: Deform,
    last: Option<(Instant, f32)>,
    inside: bool,
}

impl Card {
    fn new(theme: &Theme, preset: MotionPreset, speed: f32) -> Self {
        let pt = preset_theme(preset, speed);
        Self {
            preset,
            title: Label::new(title(preset), text_style(theme, TextRole::Label)),
            sub: Label::new(subtitle(preset), text_style(theme, TextRole::Caption)),
            hover: Interaction::new(theme),
            rect: Rect::ZERO,
            open: Animated::new(1.0, pt.motion.spatial),
            deform: Deform::new(pt.blob.deform_spring, pt.blob.deform),
            last: None,
            inside: false,
        }
    }

    fn set_speed(&mut self, speed: f32, now: Instant) {
        let pt = preset_theme(self.preset, speed);
        self.open.set_motion(pt.motion.spatial, now);
        self.deform.set_params(pt.blob.deform_spring, pt.blob.deform, now);
    }

    fn play(&mut self, now: Instant) {
        self.open.jump_to(0.0);
        self.open.set_target(1.0, now);
        self.last = None;
    }
}

#[derive(Debug)]
pub struct PresetPicker {
    desc: &'static SettingDesc,
    rect: Rect,
    cards: Vec<Card>,
    selected: MotionPreset,
    speed: f32,
    customized: bool,
    badge: Label,
    note: Label,
    revert: Button,
}

fn speed_of(store: &Store) -> f32 {
    by_id("motion.speed").map_or(1.0, |d| d.number(store)) as f32
}

impl PresetPicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let speed = speed_of(store);
        Self {
            desc,
            rect: Rect::ZERO,
            cards: MotionPreset::ALL.iter().map(|p| Card::new(theme, *p, speed)).collect(),
            selected: crate::defaults::preset(store),
            speed,
            customized: !custom_keys(store).is_empty(),
            badge: Label::new(t!("settings-motion-preset-customized"), text_style(theme, TextRole::Caption)),
            note: Label::new(t!("settings-motion-preset-customized-note"), text_style(theme, TextRole::Caption)),
            revert: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::Restart)).with_label(theme, t!("settings-motion-preset-revert")),
        }
    }

    fn cols(&self, theme: &Theme, width: f32) -> usize {
        if width > theme.space(140) { 4 } else { 2 }
    }

    fn card_h(theme: &Theme) -> f32 {
        theme.space(36)
    }

    fn banner_h(&mut self, env: &mut Env) -> f32 {
        if !self.customized {
            return 0.0;
        }
        let t = env.theme;
        self.revert.preferred_size(env.text, t).height + t.space(3)
    }
}

impl Block for PresetPicker {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let cols = self.cols(t, width - px * 2.0);
        let rows = self.cards.len().div_ceil(cols) as f32;
        py * 2.0 + self.banner_h(env) + rows * (Self::card_h(t) + t.space(3)) - t.space(3)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let mut top = rect.y + py;
        if self.customized {
            let s = self.revert.preferred_size(env.text, t);
            self.revert.set_rect(Rect::new(rect.right() - px - s.width, top, s.width, s.height));
            self.note.set_max_width(Some((rect.width - px * 3.0 - s.width - t.space(30)).max(t.space(20))));
            top += s.height + t.space(3);
        }
        let w = rect.width - px * 2.0;
        let cols = self.cols(t, w);
        let gap = t.space(3);
        let cw = (w - gap * (cols as f32 - 1.0)) / cols as f32;
        for (i, card) in self.cards.iter_mut().enumerate() {
            let (r, c) = (i / cols, i % cols);
            card.rect = Rect::new(rect.x + px + c as f32 * (cw + gap), top + r as f32 * (Self::card_h(t) + gap), cw, Self::card_h(t));
            card.sub.set_max_width(Some(cw - t.space(6)));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let mut chosen = None;
        for card in &mut self.cards {
            let tap = card.hover.handle(card.rect, e, now);
            repaint |= tap.repaint;
            let inside = e.kind != PointerKind::Leave && card.rect.contains(e.position);
            if inside && !card.inside {
                card.play(now);
                repaint = true;
            }
            card.inside = inside;
            if e.kind == PointerKind::Leave {
                repaint |= card.hover.reset(now);
            }
            if tap.clicked {
                chosen = Some(card.preset);
            }
        }
        if let Some(p) = chosen
            && p != self.selected
        {
            self.selected = p;
            let value = (p != MotionPreset::default()).then(|| Value::from(p.name()));
            env.set(t!("settings-motion-preset-changed", preset = title(p)), FileId::Framework, self.desc.key(), value);
            return true;
        }
        if self.customized {
            let r = self.revert.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                let writes = custom_keys(env.store).into_iter().map(|k| (FileId::Framework, k, None)).collect();
                env.push(Action::Group { label: t!("settings-motion-preset-revert"), writes });
                return true;
            }
        }
        repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.customized
            && let Some(c) = self.revert.cursor(p)
        {
            return Some(c);
        }
        self.cards.iter().any(|c| c.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.selected = crate::defaults::preset(env.store);
        self.customized = !custom_keys(env.store).is_empty();
        let speed = speed_of(env.store);
        if (speed - self.speed).abs() > f32::EPSILON {
            self.speed = speed;
            for c in &mut self.cards {
                c.set_speed(speed, env.now);
            }
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for c in &mut self.cards {
            c.title.set_style(text_style(theme, TextRole::Label));
            c.sub.set_style(text_style(theme, TextRole::Caption));
            c.hover.set_theme(theme, now);
        }
        self.badge.set_style(text_style(theme, TextRole::Caption));
        self.note.set_style(text_style(theme, TextRole::Caption));
        self.revert.set_theme(theme, now);
    }

    fn demo_expand(&mut self, now: Instant) {
        for c in &mut self.cards {
            c.play(now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        if self.customized {
            // Rozet: vurgu tonunda hap.
            let bs = self.badge.measure_in(ui.painter);
            let rr = self.revert.rect();
            let pad = t.space(2);
            let pill = Rect::new(self.rect.x + px, rr.center().y - bs.height * 0.5 - pad * 0.5, bs.width + pad * 2.0, bs.height + pad);
            ui.painter.rect(pill, &RectStyle::filled(ui.fade(state_layer(t, c.primary, &[], 0.16)), Corners::uniform(pill.height * 0.5)));
            self.badge.draw(ui.painter, Point::new(pill.x + pad, pill.y + pad * 0.5), ui.fade(c.primary));
            let ns = self.note.measure_in(ui.painter);
            self.note.draw(ui.painter, Point::new(pill.right() + t.space(3), rr.center().y - ns.height * 0.5), ui.fade(c.on_surface_variant));
            self.revert.draw(ui);
        }
        let _ = py;
        for card in &mut self.cards {
            let r = card.rect;
            let selected = card.preset == self.selected;
            let border = if selected { Border { width: 2.0, color: ui.fade(c.primary) } } else { Border { width: HAIRLINE, color: ui.fade(c.outline) } };
            ui.painter.rect(r, &RectStyle { fill: ui.fade(c.surface), radii: t.shape.medium(), border: Some(border) });
            card.hover.draw_layer(ui.painter, r, t.shape.medium(), t, now);
            if card.hover.is_animating(now) {
                ui.request_frame();
            }
            let pad = t.space(3);
            // Mini sahne: üstte bar, ortasından büyüyen panel.
            let stage = Rect::new(r.x + pad, r.y + pad, r.width - pad * 2.0, r.height * 0.55);
            ui.painter.rect(stage, &RectStyle::filled(ui.fade(c.surface_container), t.shape.small()));
            let bar_h = stage.height * 0.16;
            let p = card.open.value(now);
            let full_h = stage.height - bar_h - t.space(2);
            let h = (full_h * p).max(0.0);
            if let Some((prev, last_h)) = card.last {
                let dt = now.saturating_duration_since(prev).as_secs_f32();
                if dt > 0.0 {
                    card.deform.update(Point::new(0.0, (h - last_h) / dt), now);
                }
            }
            card.last = Some((now, h));
            let pw = stage.width * 0.42;
            let mut shapes = vec![BlobShape::new(Rect::new(stage.x, stage.y, stage.width, bar_h), Corners::uniform(t.shape.radius_small))];
            if h > 0.5 {
                let panel = Rect::new(stage.center().x - pw * 0.5, stage.y + bar_h - t.space(1), pw, h + t.space(1));
                shapes.push(BlobShape::new(panel, Corners::uniform(t.shape.radius_small)).with_deform(card.deform.matrix(now), Point::new(0.5, 0.0)));
            }
            let field = BlobField { frame: None, shapes, smoothing: t.blob.smoothing * 0.4, corner_radius: t.shape.radius_small, fill: ui.fade(if selected { c.primary } else { faded_mark(t, c.on_surface_variant, 0.55, &[c.surface_container]) }), shadow: None };
            ui.painter.push_clip(stage);
            ui.painter.blob(&field);
            ui.painter.pop_clip();
            if !card.open.is_settled(now) || !card.deform.is_settled(now) {
                ui.request_frame();
            }
            let ty = stage.bottom() + t.space(2);
            let th = card.title.measure_in(ui.painter).height;
            card.title.draw(ui.painter, Point::new(r.x + pad, ty), ui.fade(if selected { c.primary } else { c.on_surface }));
            card.sub.draw(ui.painter, Point::new(r.x + pad, ty + th), ui.fade(c.on_surface_variant));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_keys_cover_framework_rule() {
        // Ezilebilen her rol alanı framework'ün RoleConfig'inde var (alan adı değişirse yakala).
        let cfg: nuxut::theme::RoleConfig =
            toml_edit::de::from_str("style = \"spring\"\nresponse = 0.3\ndamping = 0.9\neasing = \"ease\"\nduration_ms = 200.0\n").expect("RoleConfig");
        assert!(!cfg.is_empty());
        let (t, w) = Theme::parse("[motion]\npreset = \"lively\"\n[motion.spatial]\ndamping = 0.5\n").expect("tema");
        assert!(w.is_empty(), "{w:?}");
        assert!((t.blob.deform - MotionPreset::Lively.values().deform).abs() < 1e-6);
    }
}
