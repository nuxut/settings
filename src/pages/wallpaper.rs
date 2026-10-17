//! Duvar kağıdı sayfası ve renk şeması.
//!
//! - [`WallpaperPicker`]: sayfanın hero'su (§5.4) — ana monitör oranında büyük önizleme;
//!   görsel, yerleşim ("Tamamı"nda kenar zemini: bulanık kopya ya da tema rengi) ayar değişince
//!   anında burada görünür. "Değiştir…" xdg-desktop-portal dosya seçicisini açar; seçim shell'in
//!   `wallpaper.path`'ine ve palet üretim görseline (`palette.toml [meta] image`) tek geri al
//!   adımıyla yazılır.
//! - [`ColorsLink`]: renklerin bu görselden gelip gelmediği ve Görünüm › Renkler'e giden düğme
//!   (karar 5: renk şeması tek yerde, Görünüm'de).
//! - [`SchemePicker`]: dokuz Material You varyantının o görselden üretilmiş renk şeritleri
//!   (Görünüm › Renkler'de); tıklanan varyant hemen uygulanır (palette.toml yeniden üretilir).

use std::path::PathBuf;
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::motion::Animated;
use nuxut::render::ImageFit;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent, PointerKind};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, IconView, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, TextSystem, Theme};
use nuxut_image::WallpaperBackdrop;
use toml_edit::Value;

use super::wallpaper_grid::WallThumb;
use super::wallpaper_outputs::{ask_displays, on_displays, primary_screen};
use crate::block::{Action, Block, Env};
use crate::registry::{SettingDesc, by_id, number};
use crate::rows::{is_narrow, row_padding};
use crate::services::palette::{Command as PaletteCommand, Update as PaletteUpdate};
use crate::services::portal::Update as PortalUpdate;
use crate::services::wallpaper as wallpaper_decode;
use crate::services::wallpaper_source::{self, WallSource};
use crate::services::{Event, Request};
use crate::store::{FileId, KeyPath, PaletteRequest, Store};

const PICK_TAG: &str = "wallpaper";
/// Shell `WALLPAPER_BACKDROP_BLUR` varsayılanı (şema yokken).
const DEFAULT_BACKDROP_BLUR: f64 = 48.0;

/// Geçerli duvar kağıdı: shell'in `wallpaper.path`'i (`~` açılır), yoksa palet görseli, ikisi
/// de yoksa varsayılan yerleşik (`nuxut:` adresleri yerleşiktir; bkz. [`wallpaper_source`]).
pub(super) fn current_source(store: &Store) -> WallSource {
    wallpaper_source::current(store)
}

/// Genel yerleşim (`[wallpaper] fit`; yoksa sahibin varsayılanı, o da yoksa `cover`).
pub(super) fn general_fit(store: &Store) -> String {
    Some(by_id("wallpaper.fit").map(|d| d.string(store)).unwrap_or_default()).filter(|s| !s.is_empty()).unwrap_or_else(|| "cover".into())
}

/// Shell yerleşim adı → çizim yerleşimi (bilinmeyen → `Cover`, shell'in yedeği).
pub(super) fn image_fit(v: &str) -> ImageFit {
    match v {
        "contain" => ImageFit::Contain,
        "fill" => ImageFit::Fill,
        _ => ImageFit::Cover,
    }
}

/// Kenar zemininin tema rolü (`[wallpaper] backdrop_color`; sabit renk yok).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackdropRole {
    Surface,
    Container,
    Bright,
}

/// Kenar zemini ayarları (`[wallpaper] backdrop*`; monitör başına değil, hepsine ortak). Renk
/// çizimde temadan çözülür: tema değişince önizleme de izler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Backdrop {
    blur: bool,
    /// Ekrandaki yarıçap (mantıksal px).
    radius: f32,
    role: BackdropRole,
}

impl Backdrop {
    pub(super) fn read(store: &Store) -> Self {
        let kind = by_id("wallpaper.backdrop").map(|d| d.string(store)).unwrap_or_default();
        let radius = by_id("wallpaper.backdrop_blur").and_then(|d| number(d.value(store).as_deref())).unwrap_or(DEFAULT_BACKDROP_BLUR) as f32;
        let role = match by_id("wallpaper.backdrop_color").map(|d| d.string(store)).unwrap_or_default().as_str() {
            "surface_container" => BackdropRole::Container,
            "surface_bright" => BackdropRole::Bright,
            _ => BackdropRole::Surface,
        };
        Self { blur: kind != "color", radius, role }
    }

    /// Önizlemenin zemini: yalnızca "Tamamı"nda; yarıçap önizlemenin küçültme oranıyla
    /// (`scale` = önizleme genişliği / ekran genişliği) — ekranda göründüğü kadar bulanık.
    pub(super) fn resolve(self, theme: &Theme, fit: ImageFit, scale: f32) -> WallpaperBackdrop {
        if fit != ImageFit::Contain {
            return WallpaperBackdrop::None;
        }
        let c = &theme.colors;
        let color = match self.role {
            BackdropRole::Surface => c.surface,
            BackdropRole::Container => c.surface_container,
            BackdropRole::Bright => c.surface_bright,
        };
        if self.blur { WallpaperBackdrop::Blur { radius: (self.radius * scale).max(1.0), fallback: color } } else { WallpaperBackdrop::Color(color) }
    }
}

fn variant_title(name: &str) -> String {
    match name {
        "tonal-spot" => t!("settings-wallpaper-variant-tonal"),
        "vibrant" => t!("settings-wallpaper-variant-vibrant"),
        "expressive" => t!("settings-wallpaper-variant-expressive"),
        "neutral" => t!("settings-wallpaper-variant-neutral"),
        "fidelity" => t!("settings-wallpaper-variant-fidelity"),
        "content" => t!("settings-wallpaper-variant-content"),
        "monochrome" => t!("settings-wallpaper-variant-monochrome"),
        "rainbow" => t!("settings-wallpaper-variant-rainbow"),
        "fruit-salad" => t!("settings-wallpaper-variant-fruit-salad"),
        _ => "?".into(),
    }
}

/// Monitör başına ayrı görseli olan monitör sayısı.
fn overrides(store: &Store) -> usize {
    let file = store.file(FileId::Shell);
    let Some(t) = file.get(&KeyPath::parse("wallpaper.outputs")).and_then(|i| i.as_table_like()) else { return 0 };
    t.iter().filter(|(_, o)| o.get("path").and_then(|p| p.as_str()).is_some_and(|p| !p.is_empty())).count()
}

// ---- Hero: duvar kağıdı önizlemesi ----------------------------------------------------------

#[derive(Debug)]
pub struct WallpaperPicker {
    desc: &'static SettingDesc,
    rect: Rect,
    /// Önizlemenin (monitörün) dikdörtgeni.
    frame: Rect,
    name: Label,
    dir: Label,
    /// "Tüm monitörlerde" / "2 monitörde ayrı görsel var".
    scope: Label,
    /// Görsel yokken "Görsel seç…" (dolu), varken "Değiştir…" (tonal).
    choose: Button,
    change: Button,
    error: Label,
    source: Option<Color>,
    swatch: Animated<Color>,
    icon: IconView,
    image: Option<WallSource>,
    /// Yol ve palet görseli boş: varsayılan yerleşik gösteriliyor.
    unset: bool,
    thumb: WallThumb,
    fit: ImageFit,
    backdrop: Backdrop,
    /// Gösterilen monitörün mantıksal boyu (önizlemenin oranı, zemin bulanıklığının ölçeği).
    screen: (f32, f32),
    hover: bool,
    focus_visible: bool,
}

impl WallpaperPicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        ask_displays();
        let mut s = Self {
            desc,
            rect: Rect::ZERO,
            frame: Rect::ZERO,
            // Ad ve klasör tek satır; uzun dosya adı "…" ile kısalır (kelime ortasından kırılmaz).
            name: Label::new("", text_style(theme, TextRole::Body).with_max_lines(1)),
            dir: Label::new("", text_style(theme, TextRole::Caption).with_max_lines(1)),
            scope: Label::new("", text_style(theme, TextRole::Caption)),
            choose: Button::new(theme, ButtonVariant::Filled).with_icon(theme, Icon::named(IconName::Image)).with_label(theme, t!("settings-wallpaper-button-choose")),
            change: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::Image)).with_label(theme, t!("settings-wallpaper-button-change")),
            error: Label::new("", text_style(theme, TextRole::Caption)),
            source: None,
            swatch: Animated::new(theme.colors.surface_bright, theme.motion.effects),
            icon: IconView::with_role(theme, Icon::named(IconName::Wallpaper), TextRole::IconLarge),
            image: None,
            unset: false,
            thumb: WallThumb::new(ImageFit::Cover, theme.motion.effects),
            fit: ImageFit::Cover,
            backdrop: Backdrop::read(store),
            screen: primary_screen(),
            hover: false,
            focus_visible: false,
        };
        s.refresh(store);
        s
    }

    fn button(&self) -> &Button {
        if self.image.is_some() { &self.change } else { &self.choose }
    }

    fn button_mut(&mut self) -> &mut Button {
        if self.image.is_some() { &mut self.change } else { &mut self.choose }
    }

    fn refresh(&mut self, store: &Store) {
        self.fit = image_fit(&general_fit(store));
        self.thumb.set_fit(self.fit);
        self.backdrop = Backdrop::read(store);
        let n = overrides(store);
        self.scope.set_text(&match n {
            0 => t!("settings-wallpaper-scope-all"),
            n => t!("settings-wallpaper-scope-per-monitor", count = n),
        });
        let src = current_source(store);
        let unset = wallpaper_source::is_unset(store);
        if self.image.as_ref() == Some(&src) && self.unset == unset && !self.name.text().is_empty() {
            return;
        }
        self.unset = unset;
        // Dosya `draw`'da istenir: hedef kartın fiziksel boyutu (yerleşim + ölçek orada belli).
        self.thumb.set_source(&src);
        match &src {
            WallSource::File(p) => {
                self.name.set_text(&p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default());
                self.dir.set_text(&p.parent().map(|d| d.display().to_string()).unwrap_or_default());
            }
            WallSource::Builtin { builtin, .. } => {
                // Boş yolda shell varsayılan yerleşiği gösterir (dosyaya yazılmadan).
                self.name.set_text(&builtin.name());
                self.dir.set_text(&if unset { t!("settings-wallpaper-builtin-default") } else { t!("settings-wallpaper-builtin-caption") });
            }
        }
        self.image = Some(src);
    }

    fn aspect(&self) -> f32 {
        (self.screen.0 / self.screen.1).clamp(0.5, 3.5)
    }

    /// Önizlemenin boyu: geniş kartta solda (kartın ~yarısı, en çok `space(120)`), dar kartta
    /// tam genişlik. Yükseklik monitör oranından.
    fn frame_size(&self, theme: &Theme, width: f32) -> (f32, f32) {
        let (px, _) = row_padding(theme);
        let avail = width - px * 2.0;
        let w = if is_narrow(theme, width) { avail } else { (avail * 0.56).clamp(theme.space(60), theme.space(120)) };
        // Dikey monitörde önizleme çok uzamasın.
        let h = (w / self.aspect()).min(theme.space(90));
        ((h * self.aspect()).min(w).round(), h.round())
    }

    /// Metin sütunu: ad, klasör, kapsam, düğme (dar kartta önizlemenin altında).
    fn text_height(&mut self, text: &mut TextSystem, theme: &Theme, text_w: f32) -> f32 {
        for l in [&mut self.scope, &mut self.name, &mut self.dir] {
            l.set_max_width(Some(text_w));
        }
        let bs = self.button_mut().preferred_size(text, theme);
        let mut h = self.scope.measure(text).height + theme.space(1) + self.name.measure(text).height + self.dir.measure(text).height + theme.space(4) + bs.height;
        if !self.error.text().is_empty() {
            self.error.set_max_width(Some(text_w));
            h += theme.space(2) + self.error.measure(text).height;
        }
        h
    }

    fn text_width(&self, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        if is_narrow(theme, width) {
            width - px * 2.0
        } else {
            (width - px * 2.0 - self.frame_size(theme, width).0 - theme.space(6)).max(theme.space(30))
        }
    }
}

impl Block for WallpaperPicker {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let (_, fh) = self.frame_size(t, width);
        let tw = self.text_width(t, width);
        let text = self.text_height(env.text, t, tw);
        if is_narrow(t, width) { py * 2.0 + fh + t.space(4) + text } else { py * 2.0 + fh.max(text) }
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let (fw, fh) = self.frame_size(t, rect.width);
        let tw = self.text_width(t, rect.width);
        let text = self.text_height(env.text, t, tw);
        let narrow = is_narrow(t, rect.width);
        self.frame = Rect::new(rect.x + px, rect.y + py, fw, fh);
        let (tx, ty) = if narrow { (rect.x + px, self.frame.bottom() + t.space(4)) } else { (self.frame.right() + t.space(6), self.frame.center().y - text * 0.5) };
        let top = ty + self.scope.measure(env.text).height + t.space(1) + self.name.measure(env.text).height + self.dir.measure(env.text).height + t.space(4);
        let bs = self.button_mut().preferred_size(env.text, t);
        for b in [&mut self.choose, &mut self.change] {
            b.set_rect(Rect::new(tx, top, bs.width, bs.height));
        }
        let _ = tw;
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        // Hareketli görsel (GIF) yalnızca imleç üstündeyken oynar: boşta iş yok.
        let hover = e.kind != PointerKind::Leave && self.frame.contains(e.position);
        let mut repaint = false;
        if hover != self.hover {
            self.hover = hover;
            self.thumb.set_visible(hover, env.now);
            repaint = true;
        }
        let now = env.now;
        let r = self.button_mut().pointer(e, now);
        if r.event.is_some() {
            self.error.set_text("");
            env.push(Action::Service(Request::PickWallpaper { tag: PICK_TAG.into() }));
        }
        r.repaint || repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let now = env.now;
        let r = self.button_mut().key(e, now);
        if r.event.is_some() {
            self.error.set_text("");
            env.push(Action::Service(Request::PickWallpaper { tag: PICK_TAG.into() }));
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focus_visible = focused && visible;
        let fv = self.focus_visible;
        self.button_mut().set_focus_visible(fv);
    }

    fn service_event(&mut self, env: &mut Env, event: &Event) -> bool {
        match event {
            Event::Portal(PortalUpdate::Picked { tag, path }) if tag == PICK_TAG => {
                // Shell çizer, palet aynı görselden üretilir: ikisi tek geri al adımı.
                let v = Value::from(path.display().to_string());
                env.push(Action::Group {
                    label: t!("settings-wallpaper-label"),
                    writes: vec![(FileId::Shell, KeyPath::parse("wallpaper.path"), Some(v.clone())), (FileId::Palette, KeyPath::parse("meta.image"), Some(v))],
                });
                true
            }
            Event::Portal(PortalUpdate::Failed { tag, message }) if tag == PICK_TAG => {
                self.error.set_text(message);
                true
            }
            Event::Palette(PaletteUpdate::Previews { image, source, .. }) if self.image.as_ref() == Some(&WallSource::parse(&image.to_string_lossy())) => {
                self.source = Some(*source);
                self.swatch.set_target(*source, env.now);
                true
            }
            Event::Displays(_) => {
                on_displays(event);
                let screen = primary_screen();
                let changed = screen != self.screen;
                self.screen = screen;
                changed
            }
            _ => false,
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.button().cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        let had = self.image.is_some();
        self.refresh(env.store);
        if had != self.image.is_some() {
            let fv = self.focus_visible;
            self.button_mut().set_focus_visible(fv);
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.name.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        self.dir.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
        for l in [&mut self.scope, &mut self.error] {
            l.set_style(text_style(theme, TextRole::Caption));
        }
        self.choose.set_theme(theme, now);
        self.change.set_theme(theme, now);
        self.icon.set_theme(theme, now);
        self.swatch.set_motion(theme.motion.effects, now);
        self.thumb.set_motion(theme.motion.effects, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let frame = self.frame;
        let radii = t.shape.medium();
        // Görsel hazır olana dek tohum rengiyle (yoksa yüzeyle) dolu ekran + ikon.
        let col = if self.source.is_some() { self.swatch.value(now) } else { c.surface };
        if !self.swatch.is_settled(now) {
            ui.request_frame();
        }
        ui.painter.rect(frame, &RectStyle::filled(ui.fade(col), radii));
        // Dosya: yalıtılmış çözücüye istek (aynı kaynak + hedef + yerleşim yeniden istenmez).
        let target = wallpaper_decode::card_target((frame.width, frame.height), ui.painter.scale(), wallpaper_decode::HERO_MAX);
        self.thumb.set_radii(radii);
        self.thumb.set_backdrop(self.backdrop.resolve(t, self.fit, frame.width / self.screen.0));
        let st = self.thumb.draw(ui, frame, target);
        if st.redraw || (self.hover && st.next_frame_in.is_some()) {
            ui.request_frame();
        }
        // Yerleşikte ortalama renk dolgusu yeter; ikon yalnızca dosya çözülürken.
        if self.thumb.content_size().is_none() && self.image.as_ref().is_some_and(|s| s.file().is_some()) {
            let ic = nuxut::ui::style::content_on(t, col);
            self.icon.draw_centered(ui.painter, frame.center(), ui.fade(nuxut::ui::style::faded_mark(t, ic, 0.7, &[col])));
        }
        let b = self.button().rect();
        let (x, mut y) = (b.x, b.y - t.space(4));
        let dh = self.dir.measure_in(ui.painter).height;
        let nh = self.name.measure_in(ui.painter).height;
        let sh = self.scope.measure_in(ui.painter).height;
        y -= dh + nh + t.space(1) + sh;
        self.scope.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface_variant));
        y += sh + t.space(1);
        self.name.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
        self.dir.draw(ui.painter, Point::new(x, y + nh), ui.fade(c.on_surface_variant));
        if self.image.is_some() { self.change.draw(ui) } else { self.choose.draw(ui) }
        if !self.error.text().is_empty() {
            self.error.draw(ui.painter, Point::new(x, b.bottom() + t.space(2)), ui.fade(c.error));
        }
    }
}

// ---- Renkler: Görünüm'e yönlendirme ----------------------------------------------------------

/// Duvar kağıdı › Renkler: renk kaynağının durumu ("Renkler bu görselden alınıyor" / "Renkler
/// sabit") ve Görünüm › Renkler'deki kaynak satırına giden düğme. Kaynak tek yerde yaşar
/// (karar 5); bu satır yalnızca gösterir ve götürür.
#[derive(Debug)]
pub struct ColorsLink {
    desc: &'static SettingDesc,
    icon: IconView,
    title: Label,
    subtitle: Label,
    go: Button,
    from_wallpaper: bool,
    rect: Rect,
}

/// Başlık ve açıklama (saf; testli).
pub fn colors_link_text(from_wallpaper: bool) -> (String, String) {
    if from_wallpaper {
        (t!("settings-wallpaper-colors-link-title-wallpaper"), t!("settings-wallpaper-colors-link-subtitle-wallpaper"))
    } else {
        (t!("settings-wallpaper-colors-link-title-static"), t!("settings-wallpaper-colors-link-subtitle-static"))
    }
}

impl ColorsLink {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut s = Self {
            desc,
            icon: IconView::with_role(theme, Icon::named(IconName::Palette), TextRole::IconSmall),
            title: Label::new("", text_style(theme, TextRole::Body)),
            subtitle: Label::new("", text_style(theme, TextRole::Caption)),
            go: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::ArrowForward)).with_label(theme, t!("settings-wallpaper-colors-link-go")),
            from_wallpaper: false,
            rect: Rect::ZERO,
        };
        s.update(store);
        s
    }

    fn update(&mut self, store: &Store) {
        self.from_wallpaper = by_id("colors.source").is_some_and(|d| d.string(store) == "wallpaper");
        let (title, subtitle) = colors_link_text(self.from_wallpaper);
        self.title.set_text(&title);
        self.subtitle.set_text(&subtitle);
    }

    fn text_width(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let is = self.icon.measure(text).width;
        let bs = self.go.preferred_size(text, theme);
        (width - px * 2.0 - is - theme.space(3) - bs.width - theme.space(4)).max(theme.space(20))
    }
}

impl Block for ColorsLink {
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
        let w = self.text_width(env.text, t, width);
        self.title.set_max_width(Some(w));
        self.subtitle.set_max_width(Some(w));
        let h = self.title.measure(env.text).height + t.space(1) + self.subtitle.measure(env.text).height;
        crate::rows::row_height_for(t, h.max(self.go.preferred_size(env.text, t).height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        self.rect = rect;
        let bs = self.go.preferred_size(env.text, t);
        // Metin düğmesinin iç boşluğu satır kenarıyla hizalansın (Link satırı gibi).
        self.go.set_rect(Rect::new(rect.right() - px - bs.width + t.space(2), rect.center().y - bs.height * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.go.pointer(e, env.now);
        if r.event.is_some() {
            env.push(Action::Reveal("colors.source"));
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.go.key(e, env.now);
        if r.event.is_some() {
            env.push(Action::Reveal("colors.source"));
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
        self.update(env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        self.go.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let cy = self.rect.center().y;
        let is = self.icon.measure(ui.painter.text_system());
        let ic = if self.from_wallpaper { c.primary } else { c.on_surface_variant };
        self.icon.draw_centered(ui.painter, Point::new(self.rect.x + px + is.width * 0.5, cy), ui.fade(ic));
        let x = self.rect.x + px + is.width + t.space(3);
        let th = self.title.measure_in(ui.painter).height;
        let sh = self.subtitle.measure_in(ui.painter).height;
        let y = cy - (th + t.space(1) + sh) * 0.5;
        self.title.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
        self.subtitle.draw(ui.painter, Point::new(x, y + th + t.space(1)), ui.fade(c.on_surface_variant));
        self.go.draw(ui);
    }
}

// ---- Şema varyantları ---------------------------------------------------------------------

#[derive(Debug)]
struct Card {
    name: &'static str,
    title: Label,
    strip: [Color; 6],
    hover: Interaction,
    rect: Rect,
    check: IconView,
}

#[derive(Debug)]
pub struct SchemePicker {
    desc: &'static SettingDesc,
    rect: Rect,
    cards: Vec<Card>,
    selected: String,
    /// Son istenen önizleme (görsel, kip, kontrast).
    requested: Option<(PathBuf, String, i64)>,
    status: Label,
    ready: bool,
    /// Satır başlığı ve açıklaması (kartın içinde, standart satır tipografisi).
    title: Label,
    subtitle: Label,
}

impl SchemePicker {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let cards = nuxut_palette::Variant::ALL
            .iter()
            .map(|v| Card {
                name: v.name(),
                title: Label::new(variant_title(v.name()), text_style(theme, TextRole::Label)),
                strip: [theme.colors.surface_bright; 6],
                hover: Interaction::new(theme),
                rect: Rect::ZERO,
                check: IconView::with_role(theme, Icon::named(IconName::CheckCircle), TextRole::IconSmall).filled(true),
            })
            .collect();
        let mut s = Self {
            desc,
            rect: Rect::ZERO,
            cards,
            selected: PaletteRequest::from_file(store.file(FileId::Palette)).scheme,
            requested: None,
            status: Label::new("", text_style(theme, TextRole::Caption)),
            ready: false,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(desc.description(), text_style(theme, TextRole::Caption)),
        };
        s.request_preview(store);
        s
    }

    fn request_preview(&mut self, store: &Store) {
        let p = PaletteRequest::from_file(store.file(FileId::Palette));
        self.selected = p.scheme.clone();
        let Some(image) = p.image else {
            self.status.set_text(&t!("settings-wallpaper-scheme-need-image"));
            return;
        };
        let key = (image.clone(), p.mode.clone(), (p.contrast * 100.0).round() as i64);
        if self.requested.as_ref() == Some(&key) {
            return;
        }
        if self.requested.as_ref().is_none_or(|r| r.0 != image) {
            self.ready = false;
            self.status.set_text(&t!("settings-wallpaper-scheme-preparing"));
        }
        self.requested = Some(key);
        crate::services::dispatch(Request::Palette(PaletteCommand::Preview { image, mode: p.mode, contrast: p.contrast }));
    }

    /// Sütun sayısı: kart en az `space(32)` geniş, en çok 5 sütun; satırlar dengeli (son satırda
    /// tek kart artmaz: 9 şema 5+4 ya da 3×3, çok dar kartta 2 sütun).
    fn cols(&self, theme: &Theme, width: f32) -> usize {
        let gap = theme.space(3);
        let fit = (((width + gap) / (theme.space(32) + gap)).floor() as usize).clamp(2, 5);
        let n = self.cards.len().max(1);
        n.div_ceil(n.div_ceil(fit))
    }

    fn card_h(theme: &Theme) -> f32 {
        theme.space(19)
    }

    /// Başlık + açıklama yüksekliği (ızgaranın üstünde).
    fn head_height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        self.title.set_max_width(Some(width - px * 2.0));
        self.subtitle.set_max_width(Some(width - px * 2.0));
        let sub = if self.subtitle.text().is_empty() { 0.0 } else { theme.space(1) + self.subtitle.measure(text).height };
        self.title.measure(text).height + sub + theme.space(3)
    }
}

impl Block for SchemePicker {
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
        let head = self.head_height(env.text, t, width);
        let cols = self.cols(t, width - px * 2.0);
        let rows = self.cards.len().div_ceil(cols) as f32;
        let status = if self.ready { 0.0 } else { self.status.measure(env.text).height + t.space(3) };
        py * 2.0 + head + status + rows * (Self::card_h(t) + t.space(3)) - t.space(3)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let head = self.head_height(env.text, t, rect.width);
        let w = rect.width - px * 2.0;
        let cols = self.cols(t, w);
        let gap = t.space(3);
        let cw = (w - gap * (cols as f32 - 1.0)) / cols as f32;
        let top = rect.y + py + head + if self.ready { 0.0 } else { self.status.measure(env.text).height + t.space(3) };
        for (i, card) in self.cards.iter_mut().enumerate() {
            let (r, c) = (i / cols, i % cols);
            card.rect = Rect::new(rect.x + px + c as f32 * (cw + gap), top + r as f32 * (Self::card_h(t) + gap), cw, Self::card_h(t));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        let mut chosen = None;
        for card in &mut self.cards {
            let tap = card.hover.handle(card.rect, e, env.now);
            repaint |= tap.repaint;
            if tap.clicked && self.ready {
                chosen = Some(card.name);
            }
        }
        if e.kind == PointerKind::Leave {
            for card in &mut self.cards {
                repaint |= card.hover.reset(env.now);
            }
        }
        if let Some(name) = chosen
            && name != self.selected
        {
            self.selected = name.to_owned();
            env.push(Action::Set {
                label: t!("settings-section-color-scheme"),
                file: FileId::Palette,
                path: KeyPath::parse("meta.scheme"),
                value: Some(Value::from(name)),
            });
            repaint = true;
        }
        repaint
    }

    fn service_event(&mut self, _env: &mut Env, event: &Event) -> bool {
        match event {
            Event::Palette(PaletteUpdate::Previews { image, strips, .. }) => {
                if self.requested.as_ref().is_some_and(|r| &r.0 == image) {
                    for (name, strip) in strips {
                        if let Some(card) = self.cards.iter_mut().find(|c| c.name == *name) {
                            card.strip = *strip;
                        }
                    }
                    self.ready = true;
                }
                true
            }
            Event::Palette(PaletteUpdate::Failed(msg)) => {
                self.ready = false;
                self.status.set_text(msg);
                true
            }
            _ => false,
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        (self.ready && self.cards.iter().any(|c| c.rect.contains(p))).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.request_preview(env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for card in &mut self.cards {
            card.title.set_style(text_style(theme, TextRole::Label));
            card.hover.set_theme(theme, now);
            card.check.set_theme(theme, now);
        }
        self.status.set_style(text_style(theme, TextRole::Caption));
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let x = self.rect.x + px;
        let mut y = self.rect.y + py;
        self.title.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
        y += self.title.measure_in(ui.painter).height;
        if !self.subtitle.text().is_empty() {
            self.subtitle.draw(ui.painter, Point::new(x, y + t.space(1)), ui.fade(c.on_surface_variant));
            y += t.space(1) + self.subtitle.measure_in(ui.painter).height;
        }
        if !self.ready {
            self.status.draw(ui.painter, Point::new(x, y + t.space(3)), ui.fade(c.on_surface_variant));
        }
        for card in &mut self.cards {
            let r = card.rect;
            let selected = card.name == self.selected;
            // Tonal, kenarsız (§5.2); yalnızca seçili kartın vurgu çerçevesi.
            let border = selected.then(|| Border { width: 2.0, color: ui.fade(c.primary) });
            ui.painter.rect(r, &RectStyle { fill: ui.fade(c.surface), radii: t.shape.medium(), border });
            card.hover.draw_layer(ui.painter, r, t.shape.medium(), t, now);
            if card.hover.is_animating(now) {
                ui.request_frame();
            }
            // Şerit: ilk renk geniş (vurgu), diğerleri eşit.
            let pad = t.space(3);
            let strip = Rect::new(r.x + pad, r.y + pad, r.width - pad * 2.0, t.space(8));
            let alpha = if self.ready { 1.0 } else { 0.35 };
            let n = card.strip.len() as f32;
            let first = strip.width * 0.32;
            let rest = (strip.width - first) / (n - 1.0);
            let mut x = strip.x;
            for (i, col) in card.strip.iter().enumerate() {
                let w = if i == 0 { first } else { rest };
                let radii = match i {
                    0 => Corners::new(t.shape.radius_small, 0.0, 0.0, t.shape.radius_small),
                    5 => Corners::new(0.0, t.shape.radius_small, t.shape.radius_small, 0.0),
                    _ => Corners::ZERO,
                };
                ui.painter.rect(Rect::new(x, strip.y, w + 0.5, strip.height), &RectStyle::filled(ui.fade(col.multiply_alpha(alpha)), radii));
                x += w;
            }
            let th = card.title.measure_in(ui.painter).height;
            let ty = strip.bottom() + (r.bottom() - strip.bottom() - th) * 0.5;
            card.title.draw(ui.painter, Point::new(r.x + pad, ty), ui.fade(if selected { c.primary } else { c.on_surface }));
            if selected {
                let s = card.check.measure(ui.painter.text_system());
                card.check.draw_centered(ui.painter, Point::new(r.right() - pad - s.width * 0.5, ty + th * 0.5), ui.fade(c.primary));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_home_matches_shell() {
        use crate::services::wallpaper_source::expand_home;
        let home = PathBuf::from(std::env::var_os("HOME").expect("HOME"));
        assert_eq!(expand_home("~"), home);
        assert_eq!(expand_home("~/Resimler/a.png"), home.join("Resimler/a.png"));
        assert_eq!(expand_home("~kullanici/a.png"), PathBuf::from("~kullanici/a.png"));
        assert_eq!(expand_home("/srv/a.gif"), PathBuf::from("/srv/a.gif"));
    }

    fn store(shell: &str, framework: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let (fw, sh) = (dir.path().join("framework.toml"), dir.path().join("shell.toml"));
        std::fs::write(&fw, framework).unwrap();
        std::fs::write(&sh, shell).unwrap();
        let s = Store::open(&fw, &sh, dir.path().join("palette.toml"));
        (dir, s)
    }

    #[test]
    fn hero_backdrop_follows_fit_and_scales_blur() {
        let theme = Theme::default();
        let (_d, s) = store("[wallpaper]\nbackdrop = \"blur\"\nbackdrop_blur = 40\n", "");
        let b = Backdrop::read(&s);
        assert_eq!(b.resolve(&theme, ImageFit::Cover, 0.25), WallpaperBackdrop::None, "kenar boşluğu yalnızca Tamamı'nda");
        assert_eq!(b.resolve(&theme, ImageFit::Contain, 0.25), WallpaperBackdrop::Blur { radius: 10.0, fallback: theme.colors.surface }, "önizlemede ekrandaki kadar bulanık");
        let (_d, s) = store("[wallpaper]\nbackdrop = \"color\"\nbackdrop_color = \"surface_bright\"\n", "");
        assert_eq!(Backdrop::read(&s).resolve(&theme, ImageFit::Contain, 0.25), WallpaperBackdrop::Color(theme.colors.surface_bright), "tema rolü, sabit renk yok");
        assert_eq!(image_fit("contain"), ImageFit::Contain);
        assert_eq!(image_fit("bozuk"), ImageFit::Cover, "shell'in yedeği");
    }

    #[test]
    fn letterbox_rows_only_when_something_is_contained() {
        let rows = ["wallpaper.backdrop", "wallpaper.backdrop_color", "wallpaper.backdrop_blur"].map(|id| by_id(id).unwrap());
        let (_d, s) = store("[wallpaper]\npath = \"/a.png\"\nfit = \"cover\"\n", "");
        assert!(rows.iter().all(|r| !r.is_visible(&s)), "kenar boşluğu yok → zemin satırları gizli");
        let (_d, s) = store("[wallpaper]\npath = \"/a.png\"\nfit = \"contain\"\n", "");
        assert!(rows.iter().all(|r| r.is_visible(&s)), "bulanık zemin varsayılan");
        let (_d, s) = store("[wallpaper]\npath = \"/a.png\"\nfit = \"cover\"\n[wallpaper.outputs.\"DP-1\"]\nfit = \"contain\"\n", "");
        assert!(rows[0].is_visible(&s), "bir monitörde Tamamı da yeter");
        let (_d, s) = store("[wallpaper]\npath = \"/a.png\"\nfit = \"contain\"\nbackdrop = \"color\"\n", "");
        assert!(rows[1].is_visible(&s) && !rows[2].is_visible(&s), "düz renkte bulanıklık satırı yok");
        // Yerleşik her zaman cover: Yerleşim ve zemin satırları gizli (2026-10-13).
        let fit = by_id("wallpaper.fit").unwrap();
        for shell in ["[wallpaper]\nfit = \"contain\"\n", "[wallpaper]\npath = \"nuxut:aurora\"\nfit = \"contain\"\n"] {
            let (_d, s) = store(shell, "");
            assert!(!fit.is_visible(&s) && rows.iter().all(|r| !r.is_visible(&s)), "yerleşik: {shell:?}");
        }
        // Bir monitörün kendi dosyası varsa satırlar o monitör için anlamlı.
        let (_d, s) = store("[wallpaper]\npath = \"nuxut:climb\"\n[wallpaper.outputs.\"DP-1\"]\npath = \"/b.png\"\nfit = \"contain\"\n", "");
        assert!(fit.is_visible(&s) && rows[0].is_visible(&s));
        // Monitör yerleşik gösteriyorsa kendi "Tamamı"sı zemin açmaz.
        let (_d, s) = store("[wallpaper]\npath = \"/a.png\"\n[wallpaper.outputs.\"DP-1\"]\npath = \"nuxut:climb\"\nfit = \"contain\"\n", "");
        assert!(fit.is_visible(&s) && !rows[0].is_visible(&s));
    }

    #[test]
    fn colors_link_tells_where_colors_come_from() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let desc = by_id("palette.source").unwrap();
        assert_eq!(desc.page, crate::registry::PageId::Wallpaper);
        let (_d, s) = store("", "[colors]\nsource = \"wallpaper\"\n");
        let link = ColorsLink::new(&theme, desc, &s);
        assert_eq!(link.title.text(), colors_link_text(true).0);
        let (_d, s) = store("", "");
        let link = ColorsLink::new(&theme, desc, &s);
        assert_eq!(link.title.text(), "Renkler Nuxut temasından");
        // Şema ve kontrast Görünüm'de (karar 5): Duvar kağıdı sayfasında renk denetimi yok.
        for id in ["palette", "palette.contrast", "palette.mode", "colors.source"] {
            assert_eq!(by_id(id).unwrap().page, crate::registry::PageId::Appearance, "{id}");
        }
    }
}
