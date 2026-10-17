//! Görünüm › Renkler › Renk teması (framework `[colors] theme`, kullanıcı isteği 2026-10-16):
//! altı önizleme kartı — Nuxut mavisi (varsayılan), Nötr, Adaçayı, Gül kurusu, Kehribar, Grafit.
//! Yalnız renkler Nuxut temasından gelirken görünür (`colors.source = "static"`).
//!
//! Kart, temanın etkin kipteki paletiyle küçük bir masaüstüdür: seçili duvar kağıdı bir Nuxut
//! yerleşiğiyse o temaya boyanmış hâli (shell masaüstüyle aynı çizim), değilse temanın yüzeyi;
//! üstünde bar (vurgu hapı) ve bir panel kartı. Varsayılan seçilince anahtar silinir.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::theme::{ColorThemeId, Palette, ThemeMode};
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::FOCUS_RING;
use nuxut::ui::{Icon, IconName, IconView, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, TextSystem, Theme};
use toml_edit::Value;

use super::wallpaper_grid::BuiltinThumb;
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::services::wallpaper_source::{self, Builtin, variant_of};
use crate::store::{FileId, Store};

/// Kartın en dar hâli (`space` birimi); 6 kart: geniş sayfada 3×2, dar sayfada 2×3.
const MIN_CARD_SPACE: u32 = 40;

/// Görünen ad (`settings-color-theme-<ad>`).
pub fn title(id: ColorThemeId) -> String {
    nuxut::i18n::current().text(&format!("settings-color-theme-{}", id.name())).into_owned()
}

/// Kayıtlı tema (yoksa ya da tanınmıyorsa varsayılan).
fn current(desc: &SettingDesc, store: &Store) -> ColorThemeId {
    ColorThemeId::from_name(&desc.string(store)).unwrap_or_default()
}

/// Seçimin yazımı: varsayılan anahtarı siler.
fn value_for(id: ColorThemeId) -> Option<Value> {
    (id != ColorThemeId::default()).then(|| Value::from(id.name()))
}

#[derive(Debug)]
struct Card {
    id: ColorThemeId,
    name: Label,
    hover: Interaction,
    rect: Rect,
    preview: Rect,
    thumb: BuiltinThumb,
    /// Temanın paleti, `palette_mode` kipinde.
    palette: Palette,
}

#[derive(Debug)]
pub struct ColorThemePicker {
    desc: &'static SettingDesc,
    cards: Vec<Card>,
    check: IconView,
    selected: ColorThemeId,
    /// Önizleme zemini: seçili Nuxut yerleşiği (dosya duvar kağıdında `None`).
    builtin: Option<&'static Builtin>,
    palette_mode: ThemeMode,
    cols: usize,
    focus_i: usize,
    focused: bool,
    focus_visible: bool,
    rect: Rect,
}

impl ColorThemePicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let cards = ColorThemeId::ALL
            .iter()
            .map(|&id| Card {
                id,
                name: Label::new(title(id), text_style(theme, TextRole::Label).with_max_lines(1)),
                hover: Interaction::new(theme),
                rect: Rect::ZERO,
                preview: Rect::ZERO,
                thumb: BuiltinThumb::default(),
                palette: id.theme().palette(theme.mode),
            })
            .collect();
        Self {
            desc,
            cards,
            check: IconView::with_role(theme, Icon::named(IconName::Check), TextRole::IconSmall),
            selected: current(desc, store),
            builtin: wallpaper_source::selected_builtin(store),
            palette_mode: theme.mode,
            cols: 2,
            focus_i: 0,
            focused: false,
            focus_visible: false,
            rect: Rect::ZERO,
        }
    }

    fn cols_for(theme: &Theme, width: f32) -> usize {
        let gap = theme.space(3);
        (((width + gap) / (theme.space(MIN_CARD_SPACE) + gap)).floor() as usize).clamp(2, 3)
    }

    fn card_w(theme: &Theme, width: f32, cols: usize) -> f32 {
        (width - theme.space(3) * (cols as f32 - 1.0)) / cols as f32
    }

    fn preview_h(cw: f32) -> f32 {
        (cw * 9.0 / 16.0).round()
    }

    fn card_h(&mut self, text: &mut TextSystem, theme: &Theme, cw: f32) -> f32 {
        let name = self.cards.first_mut().map_or(0.0, |c| c.name.measure(text).height);
        Self::preview_h(cw) + theme.space(2) + name
    }

    /// Kartı seçer: zaten seçiliyse yazmaz.
    fn choose(&mut self, env: &mut Env, i: usize) {
        let Some(id) = self.cards.get(i).map(|c| c.id) else { return };
        if id == self.selected {
            return;
        }
        self.selected = id;
        env.set(t!("settings-color-theme-changed", theme = title(id)), FileId::Framework, self.desc.key(), value_for(id));
    }

    fn move_focus(&mut self, key: Key) -> bool {
        let n = self.cards.len();
        let i = self.focus_i.min(n - 1);
        let cols = self.cols.max(1);
        self.focus_i = match key {
            Key::Left => i.saturating_sub(1),
            Key::Right => (i + 1).min(n - 1),
            Key::Up => i.saturating_sub(cols),
            Key::Down => (i + cols).min(n - 1),
            Key::Home => 0,
            Key::End => n - 1,
            _ => return false,
        };
        true
    }

    fn set_palette_mode(&mut self, mode: ThemeMode) {
        if mode != self.palette_mode {
            self.palette_mode = mode;
            for c in &mut self.cards {
                c.palette = c.id.theme().palette(mode);
            }
        }
    }
}

/// Kartın küçük masaüstü: bar (vurgu hapı, iki nokta, saat çizgisi) ve sağ altta panel kartı.
fn draw_scene(ui: &mut Ui, p: Rect, pal: &Palette) {
    let t = ui.theme;
    let pad = (p.height * 0.08).max(t.space(1));
    let bar_h = (p.height * 0.13).max(6.0);
    let bar = Rect::new(p.x + pad, p.y + pad, p.width - pad * 2.0, bar_h);
    ui.painter.rect(bar, &RectStyle::filled(ui.fade(pal.surface.with_alpha(0.94)), Corners::uniform(bar_h * 0.5)));
    let dot = bar_h * 0.36;
    let cy = bar.center().y;
    let pill = Rect::new(bar.x + bar_h * 0.4, cy - dot * 0.5, dot * 3.0, dot);
    ui.painter.rect(pill, &RectStyle::filled(ui.fade(pal.primary), Corners::uniform(dot * 0.5)));
    for k in 0..2 {
        let x = pill.right() + dot * (0.8 + k as f32 * 1.6);
        ui.painter.rect(Rect::new(x, cy - dot * 0.5, dot, dot), &RectStyle::filled(ui.fade(pal.on_surface_variant), Corners::uniform(dot * 0.5)));
    }
    let clock = Rect::new(bar.right() - bar_h * 0.4 - dot * 3.4, cy - dot * 0.35, dot * 3.4, dot * 0.7);
    ui.painter.rect(clock, &RectStyle::filled(ui.fade(pal.on_surface), Corners::uniform(dot * 0.35)));

    // Panel: yüzey kabı, başlık çizgisi, ikincil çizgi ve vurgu düğmesi.
    let pw = p.width * 0.42;
    let ph = p.height * 0.5;
    let panel = Rect::new(p.right() - pad - pw, p.bottom() - pad - ph, pw, ph);
    let r = (ph * 0.16).min(t.shape.radius_small * 1.5);
    ui.painter.rect(panel, &RectStyle::filled(ui.fade(pal.surface_container), Corners::uniform(r)));
    let lx = panel.x + ph * 0.16;
    let lw = pw - ph * 0.32;
    let line = ph * 0.09;
    ui.painter.rect(Rect::new(lx, panel.y + ph * 0.18, lw * 0.7, line), &RectStyle::filled(ui.fade(pal.on_surface), Corners::uniform(line * 0.5)));
    ui.painter.rect(Rect::new(lx, panel.y + ph * 0.36, lw * 0.9, line * 0.8), &RectStyle::filled(ui.fade(pal.on_surface_variant), Corners::uniform(line * 0.4)));
    let bh = ph * 0.22;
    let btn = Rect::new(panel.right() - ph * 0.16 - lw * 0.5, panel.bottom() - ph * 0.14 - bh, lw * 0.5, bh);
    ui.painter.rect(btn, &RectStyle::filled(ui.fade(pal.primary), Corners::uniform(bh * 0.5)));
}

impl Block for ColorThemePicker {
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
        let w = width - px * 2.0;
        let cols = Self::cols_for(t, w);
        let rows = self.cards.len().div_ceil(cols) as f32;
        let gap = t.space(3);
        let ch = self.card_h(env.text, t, Self::card_w(t, w, cols));
        py * 2.0 + rows * (ch + gap) - gap
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let w = rect.width - px * 2.0;
        let cols = Self::cols_for(t, w);
        self.cols = cols;
        let gap = t.space(3);
        let cw = Self::card_w(t, w, cols);
        let ch = self.card_h(env.text, t, cw);
        for (i, card) in self.cards.iter_mut().enumerate() {
            let (r, c) = (i / cols, i % cols);
            card.rect = Rect::new(rect.x + px + c as f32 * (cw + gap), rect.y + py + r as f32 * (ch + gap), cw, ch);
            card.preview = Rect::new(card.rect.x, card.rect.y, cw, Self::preview_h(cw));
            card.name.set_max_width(Some(cw));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let mut chosen = None;
        for (i, card) in self.cards.iter_mut().enumerate() {
            let tap = card.hover.handle(card.rect, e, now);
            repaint |= tap.repaint;
            if e.kind == PointerKind::Leave {
                repaint |= card.hover.reset(now);
            }
            if tap.clicked {
                chosen = Some(i);
            }
        }
        if let Some(i) = chosen {
            self.focus_i = i;
            env.push(Action::Focus);
            self.choose(env, i);
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released {
            return false;
        }
        match e.key {
            Key::Enter | Key::Space => {
                let i = self.focus_i;
                self.focus_visible = self.focused;
                self.choose(env, i);
                true
            }
            k => {
                let moved = self.move_focus(k);
                if moved {
                    self.focus_visible = self.focused;
                }
                moved
            }
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        if focused && !self.focused {
            self.focus_i = self.cards.iter().position(|c| c.id == self.selected).unwrap_or(0);
        }
        self.focused = focused;
        self.focus_visible = focused && visible;
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.cards.iter().any(|c| c.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.selected = current(self.desc, env.store);
        self.builtin = wallpaper_source::selected_builtin(env.store);
        self.set_palette_mode(env.theme.mode);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.check.set_theme(theme, now);
        for c in &mut self.cards {
            c.name.set_style(text_style(theme, TextRole::Label).with_max_lines(1));
            c.hover.set_theme(theme, now);
        }
        self.set_palette_mode(theme.mode);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        // Kip temanın çözdüğü kip (kaydedilmemiş Koyu/Açık seçimi dahil).
        self.set_palette_mode(t.mode);
        let variant = variant_of(t.mode);
        let radii = t.shape.medium();
        let surface_h = ui.painter.size().height;
        let ring_on = self.focus_visible;
        let mut waiting = false;
        for (i, card) in self.cards.iter_mut().enumerate() {
            let p = card.preview;
            let is_sel = card.id == self.selected;
            let on_screen = p.bottom() >= 0.0 && p.y <= surface_h;
            match self.builtin {
                // Yalnızca görünen kartın yerleşiği istenir.
                Some(b) if on_screen => waiting |= card.thumb.draw_themed(ui, p, radii, b, variant, card.id),
                _ => ui.painter.rect(p, &RectStyle::filled(ui.fade(card.palette.surface), radii)),
            }
            ui.painter.push_clip(p);
            draw_scene(ui, p, &card.palette);
            ui.painter.pop_clip();
            card.hover.draw_layer(ui.painter, p, radii, t, now);
            if card.hover.is_animating(now) {
                ui.request_frame();
            }
            if is_sel {
                ui.painter.rect(p, &RectStyle { fill: Color::TRANSPARENT, radii, border: Some(Border { width: 3.0, color: ui.fade(c.primary) }) });
                let s = self.check.measure(ui.painter.text_system());
                let d = s.width.max(s.height) + t.space(2);
                let center = Point::new(p.x + t.space(2) + d * 0.5, p.bottom() - t.space(2) - d * 0.5);
                ui.painter.rect(Rect::new(center.x - d * 0.5, center.y - d * 0.5, d, d), &RectStyle::filled(ui.fade(c.primary), Corners::uniform(d * 0.5)));
                self.check.draw_centered(ui.painter, center, ui.fade(c.on_primary));
            } else {
                // Seçili olmayan kartın kenarı: koyu/açık zeminde önizlemeyi sayfadan ayırır.
                ui.painter.rect(p, &RectStyle { fill: Color::TRANSPARENT, radii, border: Some(Border { width: nuxut::ui::style::HAIRLINE, color: ui.fade(c.outline) }) });
            }
            let nc = if is_sel { c.primary } else { c.on_surface };
            card.name.draw(ui.painter, Point::new(card.rect.x, p.bottom() + t.space(2)), ui.fade(nc));
            if ring_on && i == self.focus_i {
                let o = FOCUS_RING + 1.0;
                let rr = Corners::new(radii.top_left + o, radii.top_right + o, radii.bottom_right + o, radii.bottom_left + o);
                ui.painter.rect(p.expand(o), &RectStyle { fill: Color::TRANSPARENT, radii: rr, border: Some(Border { width: FOCUS_RING, color: ui.fade(c.primary) }) });
            }
        }
        if waiting {
            ui.request_frame();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::by_id;
    use crate::store::KeyPath;

    fn store(framework: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("f.toml"), framework).unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    #[test]
    fn reads_theme_and_writes_default_as_removal() {
        let d = by_id("colors.theme").expect("kayıtta");
        assert_eq!(d.key(), KeyPath::parse("colors.theme"));
        let (_t, s) = store("");
        assert_eq!(current(d, &s), ColorThemeId::Blue, "varsayılan Nuxut mavisi");
        let (_t, s) = store("[colors]\ntheme = \"sage\"\n");
        assert_eq!(current(d, &s), ColorThemeId::Sage);
        let (_t, s) = store("[colors]\ntheme = \"yok\"\n");
        assert_eq!(current(d, &s), ColorThemeId::Blue, "tanınmayan ad varsayılana düşer");
        assert!(value_for(ColorThemeId::Blue).is_none(), "varsayılan anahtarı siler");
        assert_eq!(value_for(ColorThemeId::Rose).and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some("rose"));
    }

    #[test]
    fn visible_only_for_static_colors() {
        let d = by_id("colors.theme").unwrap();
        let (_t, s) = store("");
        assert!(d.is_visible(&s));
        let (_t, s) = store("[colors]\nsource = \"wallpaper\"\n");
        assert!(!d.is_visible(&s), "renkler duvar kağıdından gelirken gizli");
    }

    #[test]
    fn every_theme_has_a_name() {
        crate::i18n::use_turkish_for_tests();
        let names: Vec<_> = ColorThemeId::ALL.iter().map(|&id| title(id)).collect();
        assert_eq!(names, ["Nuxut mavisi", "Nötr", "Adaçayı", "Gül kurusu", "Kehribar", "Grafit"]);
    }
}
