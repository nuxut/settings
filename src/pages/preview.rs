//! Görünüm önizlemesi: küçültülmüş bir masaüstü — gerçek duvar kağıdı (yerleşimi ve kenar
//! zeminiyle; seçilmemişse temadan sahte bir geçiş), çerçeve + bar ve
//! bar'dan blob olarak büyüyen bir panel (içinde gerçek düğme, anahtar ve metin). Renkler,
//! yarıçaplar, gölge, blob yumuşaklığı/esneme ve hareket o anki (bellekteki) temadan gelir;
//! dört kenarın kalınlığı (bar kenarı = bar boyu, elle ayarlı kenarlar) ve bar kenarı shell.toml'dan,
//! iç köşe kavsi shell'deki gibi temanın `blob.smoothing`'i. Değer değişince panel yeniden
//! açılır (puding görünsün); tıklamak da yeniden oynatır.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::motion::{Animated, Deform};
use nuxut::ui::input::{CursorShape, PointerEvent, PointerKind};
use nuxut::ui::style::faded_mark;
use nuxut::ui::{Button, ButtonVariant, Interaction, Label, TextRole, Toggle, Ui, text_style};
use nuxut::{BlobField, BlobShape, Corners, FrameShape, Insets, Point, Rect, RectStyle, ShadowStyle, Theme};

use crate::block::{Block, Env};
use super::edges::{Edge, bar_edge, shell_insets};
use crate::registry::SettingDesc;
use crate::store::Store;
use super::wallpaper::{Backdrop, current_source, general_fit, image_fit};
use super::wallpaper_grid::WallThumb;
use super::wallpaper_outputs::primary_screen;
use nuxut::render::ImageFit;
use crate::services::wallpaper as wallpaper_decode;

/// Önizlemede shell ölçüleri bu oranla küçülür.
const SCALE: f32 = 0.6;

#[derive(Debug)]
pub struct Preview {
    desc: &'static SettingDesc,
    rect: Rect,
    /// Panelin açıklığı (0 kapalı → 1 açık), `motion.spatial`.
    open: Animated<f32>,
    deform: Deform,
    last_h: f32,
    last_t: Option<Instant>,
    clock: Label,
    title: Label,
    body: Label,
    button: Button,
    toggle: Toggle,
    hover: Interaction,
    signature: String,
    /// Ölçeklenmiş kenar kalınlıkları ve bar kenarı.
    frame: (Insets, Edge),
    /// Gerçek duvar kağıdı (Duvar kağıdı sayfasıyla aynı kaynak, yerleşim ve zemin).
    wall: WallThumb,
    wall_image: Option<crate::services::wallpaper_source::WallSource>,
    wall_fit: ImageFit,
    wall_backdrop: Backdrop,
}

impl Preview {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut open = Animated::new(0.0, theme.motion.spatial);
        open.set_target(1.0, Instant::now());
        let mut s = Self {
            desc,
            rect: Rect::ZERO,
            open,
            deform: Deform::new(theme.blob.deform_spring, theme.blob.deform),
            last_h: 0.0,
            last_t: None,
            clock: Label::new("14:32", text_style(theme, TextRole::Label)),
            title: Label::new(t!("settings-preview-demo-title"), text_style(theme, TextRole::Label)),
            body: Label::new(t!("settings-preview-demo-body"), text_style(theme, TextRole::Caption)),
            button: Button::new(theme, ButtonVariant::Filled).with_label(theme, t!("settings-preview-demo-button")),
            toggle: Toggle::new(theme, true),
            hover: Interaction::new(theme),
            signature: signature(theme),
            frame: frame_values(store),
            wall: WallThumb::new(ImageFit::Cover, theme.motion.effects),
            wall_image: None,
            wall_fit: ImageFit::Cover,
            wall_backdrop: Backdrop::read(store),
        };
        s.refresh_wall(store);
        s
    }

    /// Duvar kağıdı, yerleşimi ve zemini config'ten (değişmediyse görsel yeniden çözülmez).
    fn refresh_wall(&mut self, store: &Store) {
        self.wall_fit = image_fit(&general_fit(store));
        self.wall.set_fit(self.wall_fit);
        self.wall_backdrop = Backdrop::read(store);
        // Dosya `draw`'da istenir (hedef önizlemenin fiziksel boyutu); yerleşik süreç içinde çizilir.
        let src = current_source(store);
        if self.wall_image.as_ref() != Some(&src) {
            self.wall.set_source(&src);
            self.wall_image = Some(src);
        }
    }

    fn replay(&mut self, now: Instant) {
        self.open.jump_to(0.0);
        self.open.set_target(1.0, now);
    }

    /// Duvar kağıdı + çerçeve iç alanı.
    fn geometry(&self, theme: &Theme) -> (Rect, Rect) {
        let pad = theme.space(3);
        let screen = self.rect.inset(Insets::uniform(pad));
        let inner = screen.inset(self.frame.0);
        let inner = Rect::new(inner.x, inner.y, inner.width.max(0.0), inner.height.max(0.0));
        (screen, inner)
    }
}

/// Shell'in gerçek dört kenarı (bar kenarı = bar boyu), önizleme ölçeğinde.
fn frame_values(store: &Store) -> (Insets, Edge) {
    let i = shell_insets(store);
    (Insets::new(i.top * SCALE, i.right * SCALE, i.bottom * SCALE, i.left * SCALE), bar_edge(store))
}

/// Görünümü etkileyen token'ların özeti; değişince panel yeniden oynar.
fn signature(theme: &Theme) -> String {
    format!("{:?}{:?}{:?}{:?}", theme.shape, theme.shadow, theme.blob, theme.motion.spatial)
}

impl Block for Preview {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        (width * 0.36).clamp(env.theme.space(52), env.theme.space(72))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (_, inner) = self.geometry(t);
        let panel = panel_rect(t, inner, self.frame.1);
        let content = panel.inset(Insets::uniform(t.space(3)));
        let bs = self.button.preferred_size(env.text, t);
        let ts = self.toggle.preferred_size(t);
        let y = content.bottom() - bs.height;
        self.button.set_rect(Rect::new(content.right() - bs.width, y, bs.width, bs.height));
        self.toggle.set_rect(Rect::new(content.x, y + (bs.height - ts.height) * 0.5, ts.width, ts.height));
        self.body.set_max_width(Some(content.width));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = self.button.pointer(e, now).repaint;
        let r = self.toggle.pointer(e, now);
        repaint |= r.repaint;
        let tap = self.hover.handle(self.rect, e, now);
        if tap.clicked && !self.button.rect().contains(e.position) && !self.toggle.rect().contains(e.position) {
            self.replay(now);
            repaint = true;
        }
        if e.kind == PointerKind::Leave {
            repaint |= self.hover.reset(now);
        }
        repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.button.cursor(p).or_else(|| self.toggle.cursor(p)).or_else(|| self.rect.contains(p).then_some(CursorShape::Pointer))
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh_wall(env.store);
        let f = frame_values(env.store);
        if f != self.frame {
            self.frame = f;
            self.replay(env.now);
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.open.set_motion(theme.motion.spatial, now);
        self.wall.set_motion(theme.motion.effects, now);
        self.deform.set_params(theme.blob.deform_spring, theme.blob.deform, now);
        self.clock.set_style(text_style(theme, TextRole::Label));
        self.title.set_style(text_style(theme, TextRole::Label));
        self.body.set_style(text_style(theme, TextRole::Caption));
        self.button.set_theme(theme, now);
        self.toggle.set_theme(theme, now);
        self.hover.set_theme(theme, now);
        let sig = signature(theme);
        if sig != self.signature {
            self.signature = sig;
            self.replay(now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (screen, inner) = self.geometry(t);
        let (insets, bar) = self.frame;

        // Sahte duvar kağıdı (vurgu ve yüzey tonlarından yumuşak geçiş); gerçek görsel hazırsa
        // üstünde, Duvar kağıdı sayfasındaki yerleşim ve zeminle.
        let fake = [
            c.primary.mix(&c.surface, 0.35),
            c.surface_bright.mix(&c.primary, 0.25),
            c.surface.mix(&c.on_surface_variant, 0.25),
            c.primary.mix(&c.surface_container, 0.6),
        ];
        ui.painter.gradient_rect(screen, t.shape.small(), fake.map(|w| ui.fade(w)));
        if self.wall_image.is_some() {
            // Dosya: yalıtılmış çözücüye istek, hedef önizlemenin fiziksel boyutu (en çok 1280×800).
            let target = wallpaper_decode::card_target((screen.width, screen.height), ui.painter.scale(), wallpaper_decode::HERO_MAX);
            self.wall.set_radii(t.shape.small());
            self.wall.set_backdrop(self.wall_backdrop.resolve(t, self.wall_fit, screen.width / primary_screen().0));
            if self.wall.draw(ui, screen, target).redraw {
                ui.request_frame();
            }
        }

        // Panel: bar kenarının ortasından içeri büyür; hızına göre esner.
        let p = self.open.value(now);
        let full = panel_rect(t, inner, bar);
        let (shape, h, anchor, dir) = panel_open(full, inner, bar, p, t.space(1));
        if let Some(prev) = self.last_t {
            let dt = now.saturating_duration_since(prev).as_secs_f32();
            if dt > 0.0 {
                let v = (h - self.last_h) / dt;
                self.deform.update(Point::new(dir.x * v, dir.y * v), now);
            }
        }
        self.last_t = Some(now);
        self.last_h = h;
        let mut shapes = Vec::new();
        if h > 0.5 {
            shapes.push(BlobShape::new(shape, Corners::uniform(t.shape.radius_large * SCALE)).with_deform(self.deform.matrix(now), anchor));
        }
        let field = BlobField {
            // İç köşe kavsi shell'deki gibi temanın birleşim yumuşaklığı (ADR-0008).
            frame: Some(FrameShape { outer: screen, inner, inner_radius: t.blob.smoothing * SCALE }),
            shapes,
            smoothing: t.blob.smoothing * SCALE,
            corner_radius: t.shape.radius_large * SCALE,
            fill: ui.fade(c.surface),
            shadow: t.shadow.enabled.then(|| ShadowStyle {
                color: ui.fade(t.shadow.color),
                blur: t.shadow.blur * SCALE,
                offset: Point::new(0.0, t.shadow.offset_y * SCALE),
                spread: 0.0,
            }),
        };
        ui.painter.push_clip(screen);
        ui.painter.blob(&field);

        // Bar içeriği (bar hangi kenardaysa): workspace göstergeleri ve sığarsa saat.
        let strip = bar.strip(screen, bar.get(&insets));
        let horizontal = matches!(bar, Edge::Top | Edge::Bottom);
        let depth = if horizontal { strip.height } else { strip.width };
        let dot = depth * 0.32;
        let mid = strip.center();
        let mut along = if horizontal { inner.x } else { inner.y } + t.space(2);
        for i in 0..4 {
            let len = if i == 1 { dot * 2.6 } else { dot };
            let col = if i == 1 { c.primary } else { faded_mark(t, c.on_surface_variant, 0.5, &[c.surface]) };
            let r = if horizontal { Rect::new(along, mid.y - dot * 0.5, len, dot) } else { Rect::new(mid.x - dot * 0.5, along, dot, len) };
            ui.painter.rect(r, &RectStyle::filled(ui.fade(col), Corners::uniform(dot * 0.5)));
            along += len + dot * 0.8;
        }
        let cs = self.clock.measure_in(ui.painter);
        let fits = if horizontal { cs.height <= depth } else { cs.width + t.space(1) <= depth };
        if fits {
            let at = if horizontal { mid } else { Point::new(mid.x, inner.center().y) };
            self.clock.draw(ui.painter, Point::new(at.x - cs.width * 0.5, at.y - cs.height * 0.5), ui.fade(c.on_surface));
        }

        // Panel içeriği (yeterince açılınca belirir).
        let alpha = ((p - 0.75) / 0.25).clamp(0.0, 1.0);
        if alpha > 0.0 && h > 0.5 {
            let content = full.inset(Insets::uniform(t.space(3)));
            ui.with_alpha(alpha, |ui| {
                let card = Rect::new(content.x, content.y, content.width, content.height - self.button.rect().height - t.space(3));
                ui.painter.rect(card, &RectStyle::filled(ui.fade(c.surface_container), Corners::uniform(t.shape.radius_medium * SCALE * 1.4)));
                let th = self.title.measure_in(ui.painter).height;
                self.title.draw(ui.painter, Point::new(card.x + t.space(3), card.y + t.space(2)), ui.fade(c.on_surface));
                self.body.set_max_width(Some(card.width - t.space(6)));
                self.body.draw(ui.painter, Point::new(card.x + t.space(3), card.y + t.space(2) + th + t.space(1)), ui.fade(c.on_surface_variant));
                self.button.draw(ui);
                self.toggle.draw(ui);
            });
        }
        ui.painter.pop_clip();

        if !self.open.is_settled(now) || !self.deform.is_settled(now) {
            ui.request_frame();
        }
    }
}

/// Tam açık panel: bar kenarına yapışık, kenarın ortasında (saat popout'u gibi).
fn panel_rect(theme: &Theme, inner: Rect, bar: Edge) -> Rect {
    let w = (inner.width * 0.46).clamp(theme.space(44), theme.space(72)).min(inner.width);
    let h = (inner.height * 0.74).max(0.0);
    let c = inner.center();
    match bar {
        Edge::Top => Rect::new(c.x - w * 0.5, inner.y, w, h),
        Edge::Bottom => Rect::new(c.x - w * 0.5, inner.bottom() - h, w, h),
        Edge::Left => Rect::new(inner.x, c.y - h * 0.5, w, h),
        Edge::Right => Rect::new(inner.right() - w, c.y - h * 0.5, w, h),
    }
}

/// Panelin `open` kadar açılmış hali: bar kenarından içeri büyür, kenarın altına `tuck` kadar
/// girer (çerçeveyle birleşsin). Dönüş: şekil, açılma yönündeki boy, esneme çapası, yön.
fn panel_open(full: Rect, inner: Rect, bar: Edge, open: f32, tuck: f32) -> (Rect, f32, Point, Point) {
    let open = open.max(0.0);
    match bar {
        Edge::Top => {
            let h = full.height * open;
            (Rect::new(full.x, inner.y - tuck, full.width, h + tuck), h, Point::new(0.5, 0.0), Point::new(0.0, 1.0))
        }
        Edge::Bottom => {
            let h = full.height * open;
            (Rect::new(full.x, inner.bottom() - h, full.width, h + tuck), h, Point::new(0.5, 1.0), Point::new(0.0, -1.0))
        }
        Edge::Left => {
            let w = full.width * open;
            (Rect::new(inner.x - tuck, full.y, w + tuck, full.height), w, Point::new(0.0, 0.5), Point::new(1.0, 0.0))
        }
        Edge::Right => {
            let w = full.width * open;
            (Rect::new(inner.right() - w, full.y, w + tuck, full.height), w, Point::new(1.0, 0.5), Point::new(-1.0, 0.0))
        }
    }
}
