//! Yerleşik Nuxut duvar kağıtları (sözleşme `wallpaper-builtins-v1` + EK KARARLAR; dizin, adlar,
//! tohumlar ve render shell deposundaki `nuxut-wallpapers` crate'inde): Duvar kağıdı sayfasında
//! hero'nun altındaki "Nuxut duvar kağıtları" kart ızgarası ve Hoş geldin'in görünüm adımı aynı
//! bileşendir ([`WallpaperGrid`]; Hoş geldin'de 2 sütun).
//!
//! - 9 kart (6 maskotsuz + 3 maskotlu), 16:9, etkin renk kipinin (temanın çözdüğü kip:
//!   kaydedilmemiş seçim dahil, bkz. `live_theme`) varyantı. Önizleme kartın fiziksel boyutunda
//!   arka plan thread'inde gerçek SVG'den çizilir ([`BuiltinThumb`], crate `render_preview`);
//!   hazır olana dek tema `surface_container` dolgusu (crate ortalama renk vermiyor).
//! - Seçili kartta vurgu çerçevesi ve onay işareti; ad kartın altında, crate'in kataloğundan
//!   (`Builtin::name`, `nuxut_wallpapers::i18n_source`).
//! - Seçim tek geri al kaydıyla `shell.toml [wallpaper] path` ve `palette.toml [meta] image`'a
//!   `nuxut:<id>` yazar (varyantsız; palet görsel çözülmeden tohumdan üretilir). Boş yol =
//!   varsayılan kart seçili görünür, dosyaya yazılmaz.
//! - Klavye: Görünüm › Uygulamalar ızgarasıyla aynı (oklar, Home/End, Enter/Boşluk).
//!
//! [`WallThumb`]: hero, monitör satırları ve Görünüm önizlemesi için duvar kağıdı çizimi; dosya
//! framework `Wallpaper`'a (çözücü), `nuxut:` yerleşiği [`BuiltinThumb`]'a gider (SVG çözücüye
//! gitmez; gömülü SVG güvenilir, süreç içinde çizilir).

use std::sync::Arc;
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::motion::Motion;
use nuxut::render::{ImageData, ImageFit, ImageStyle, Texture, TextureOptions};
use nuxut::theme::{ColorThemeId, ThemeMode};
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::FOCUS_RING;
use nuxut::ui::{Icon, IconName, IconView, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, TextSystem, Theme};
use nuxut_image::{Wallpaper, WallpaperBackdrop, WallpaperStatus};

use super::look::Head;
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::services::wallpaper_source::{self, BUILTINS, Builtin, BuiltinRender, Variant, WallSource, variant_of};
use crate::store::Store;

/// Kartın en dar hâli (`space` birimi) ve sütun sınırları (9 kart: 3×3, dar sayfada 2 sütun).
const MIN_CARD_SPACE: u32 = 44;
const MIN_COLS: usize = 2;
const MAX_COLS: usize = 3;
/// Hoş geldin'deki sütun sayısı.
pub const WELCOME_COLS: usize = 2;

/// Sayfanın solma değeri (`ui.fade` opak bir renge uygulanınca kalan alfa).
fn page_alpha(ui: &Ui) -> f32 {
    ui.fade(ui.theme.colors.surface.with_alpha(1.0)).a
}

// ---- Yerleşik önizlemesi ------------------------------------------------------------------

/// Bir yerleşikin kartın fiziksel boyutunda çizimi: istek arka planda, doku hazır olana dek
/// tema `surface_container` dolgusu.
#[derive(Debug, Default)]
pub struct BuiltinThumb {
    render: BuiltinRender,
    texture: Option<Texture>,
    /// Dokunun hazır olduğu görülüp hasar bildirildi.
    shown: bool,
}

impl BuiltinThumb {
    /// Etkin renk temasıyla [`draw_themed`](Self::draw_themed): masaüstündeki yerleşikle aynı
    /// renkler (renkler duvar kağıdından geliyorsa Nuxut mavisi).
    pub fn draw(&mut self, ui: &mut Ui, rect: Rect, radii: Corners, builtin: &'static Builtin, variant: Variant) -> bool {
        let theme = ui.theme.color_theme.unwrap_or_default();
        self.draw_themed(ui, rect, radii, builtin, variant, theme)
    }

    /// Çizer; doku GPU'ya yüklenmeyi bekliyorsa `true` (bir kare daha isteyin). Çizim bitince
    /// servis UI'yi uyandırır, burada kare istenmez (boşta iş yok).
    pub fn draw_themed(&mut self, ui: &mut Ui, rect: Rect, radii: Corners, builtin: &'static Builtin, variant: Variant, theme: ColorThemeId) -> bool {
        let alpha = page_alpha(ui);
        let fill = ui.fade(ui.theme.colors.surface_container);
        let painter = &mut *ui.painter;
        let scale = painter.scale().max(1.0);
        let px = ((rect.width * scale).round() as u32, (rect.height * scale).round() as u32);
        if px.0 == 0 || px.1 == 0 {
            return false;
        }
        self.render.request_themed(builtin, variant, theme, px);
        if let Some(img) = self.render.poll() {
            let (width, height) = img.size();
            self.texture = Some(painter.upload_texture(ImageData { width, height, pixels: Arc::from(img.into_pixels()) }, TextureOptions::default()));
            self.shown = false;
        }
        painter.rect(rect, &RectStyle::filled(fill, radii));
        let Some(tex) = &self.texture else { return false };
        if !tex.is_ready() {
            return true;
        }
        if !self.shown {
            self.shown = true;
            painter.damage(rect);
        }
        painter.image(rect, tex, &ImageStyle::new(ImageFit::Cover).with_radii(radii).with_opacity(alpha));
        false
    }

    /// Gösterilen dokunun boyutu.
    pub fn content_size(&self) -> Option<(u32, u32)> {
        self.texture.as_ref().filter(|t| t.is_ready()).map(Texture::size)
    }

    /// Son istek (test ve tanı).
    pub fn requested(&self) -> Option<(&'static Builtin, Variant, (u32, u32))> {
        self.render.requested()
    }
}

/// Duvar kağıdı önizlemesi: dosya framework `Wallpaper`'ında çözülür, yerleşik süreç içinde
/// çizilir. Yerleşik her zaman `cover`'dır (sözleşme §3; yerleşim ve zemin yok sayılır).
#[derive(Debug)]
pub struct WallThumb {
    file: Wallpaper,
    builtin: BuiltinThumb,
    source: Option<WallSource>,
    radii: Corners,
}

impl WallThumb {
    /// Dosyalar sürecin paylaşılan, yalıtılmış çözücüsüne gider (`services::wallpaper::shared`).
    pub fn new(fit: ImageFit, motion: impl Into<Motion>) -> Self {
        let file = Wallpaper::new(fit, motion, crate::services::wallpaper::shared(), crate::services::wake);
        Self { file, builtin: BuiltinThumb::default(), source: None, radii: Corners::ZERO }
    }

    /// Yeni kaynak. Dosya `draw`'da hedef boyutuyla istenir (yerleşim ve ölçek orada belli);
    /// yerleşik çizildiği fiziksel boyda üretilir.
    pub fn set_source(&mut self, source: &WallSource) {
        self.source = Some(source.clone());
    }

    pub fn source(&self) -> Option<&WallSource> {
        self.source.as_ref()
    }

    pub fn set_fit(&mut self, fit: ImageFit) {
        self.file.set_fit(fit);
    }

    pub fn set_radii(&mut self, radii: Corners) {
        self.radii = radii;
        self.file.set_radii(radii);
    }

    pub fn set_backdrop(&mut self, backdrop: WallpaperBackdrop) {
        self.file.set_backdrop(backdrop);
    }

    pub fn set_motion(&mut self, motion: impl Into<Motion>, now: Instant) {
        self.file.set_motion(motion, now);
    }

    pub fn set_visible(&mut self, visible: bool, now: Instant) {
        self.file.set_visible(visible, now);
    }

    pub fn content_size(&self) -> Option<(u32, u32)> {
        match &self.source {
            Some(WallSource::Builtin { .. }) => self.builtin.content_size(),
            _ => self.file.content_size(),
        }
    }

    /// Yerleşikte varyant temanın çözdüğü kipten (adres sabit varyantlıysa o). `file_target`:
    /// dosyanın çözüleceği fiziksel boy (`services::wallpaper::card_target`/`row_target`);
    /// `None` (kart yerleşmemiş) → istek yok, eldeki kare çizilir.
    pub fn draw(&mut self, ui: &mut Ui, rect: Rect, file_target: Option<(u32, u32)>) -> WallpaperStatus {
        match &self.source {
            Some(src @ WallSource::Builtin { builtin, .. }) => {
                let (b, v) = (*builtin, src.variant(ui.theme.mode));
                WallpaperStatus { redraw: self.builtin.draw(ui, rect, self.radii, b, v), next_frame_in: None }
            }
            Some(WallSource::File(p)) => {
                if let Some(target) = file_target {
                    // Aynı kaynak + boy yeniden istenirse framework bir şey yapmaz.
                    self.file.set_source(p, target);
                }
                self.file.draw(ui.painter, rect, ui.now)
            }
            None => WallpaperStatus::default(),
        }
    }
}

// ---- Kart ızgarası ------------------------------------------------------------------------

#[derive(Debug)]
struct Card {
    builtin: &'static Builtin,
    name: Label,
    hover: Interaction,
    /// Önizleme (16:9) ve ad dahil kart.
    rect: Rect,
    preview: Rect,
    thumb: BuiltinThumb,
}

#[derive(Debug)]
pub struct WallpaperGrid {
    desc: &'static SettingDesc,
    head: Head,
    /// Başlık kartın içinde de çizilsin mi (Hoş geldin: bölüm başlığı yok). Sayfada bölüm
    /// başlığı aynı metin, yalnızca açıklama satırı çizilir.
    show_title: bool,
    cards: Vec<Card>,
    check: IconView,
    /// Sabit sütun (Hoş geldin); `None` genişliğe göre 2–3.
    fixed_cols: Option<usize>,
    cols: usize,
    /// Seçili kartın sırası (dosya seçiliyken `None`).
    selected: Option<usize>,
    mode: ThemeMode,
    focus_i: usize,
    focused: bool,
    focus_visible: bool,
    rect: Rect,
}

impl WallpaperGrid {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let cards = BUILTINS
            .iter()
            .map(|b| Card {
                builtin: b,
                name: Label::new(b.name(), text_style(theme, TextRole::Label).with_max_lines(1)),
                hover: Interaction::new(theme),
                rect: Rect::ZERO,
                preview: Rect::ZERO,
                thumb: BuiltinThumb::default(),
            })
            .collect();
        let mut s = Self {
            desc,
            head: Head::new(theme, desc),
            show_title: false,
            cards,
            check: IconView::with_role(theme, Icon::named(IconName::Check), TextRole::IconSmall),
            fixed_cols: None,
            cols: MIN_COLS,
            selected: None,
            mode: theme.mode,
            focus_i: 0,
            focused: false,
            focus_visible: false,
            rect: Rect::ZERO,
        };
        s.refresh(store);
        s
    }

    /// Sabit sütun sayısı (Hoş geldin: [`WELCOME_COLS`]).
    pub fn with_columns(mut self, cols: usize) -> Self {
        self.fixed_cols = Some(cols.max(1));
        self
    }

    /// Başlığı kartın içinde de çizer (bölüm başlığı olmayan yerler: Hoş geldin).
    pub fn with_title(mut self) -> Self {
        self.show_title = true;
        self
    }

    fn head_height(&mut self, text: &mut TextSystem, theme: &Theme, w: f32) -> f32 {
        if self.show_title {
            return self.head.measure(text, theme, w);
        }
        self.head.subtitle.as_mut().map_or(0.0, |s| {
            s.set_max_width(Some(w));
            s.measure(text).height
        })
    }

    fn draw_head(&mut self, ui: &mut Ui, x: f32, y: f32) {
        if self.show_title {
            self.head.draw(ui, x, y);
        } else if let Some(s) = &mut self.head.subtitle {
            s.draw(ui.painter, Point::new(x, y), ui.fade(ui.theme.colors.on_surface_variant));
        }
    }

    fn refresh(&mut self, store: &Store) {
        let sel = wallpaper_source::selected_builtin(store);
        self.selected = sel.and_then(|b| self.cards.iter().position(|c| c.builtin == b));
    }

    fn cols_for(&self, theme: &Theme, width: f32) -> usize {
        if let Some(n) = self.fixed_cols {
            return n;
        }
        let gap = theme.space(3);
        (((width + gap) / (theme.space(MIN_CARD_SPACE) + gap)).floor() as usize).clamp(MIN_COLS, MAX_COLS)
    }

    fn card_w(theme: &Theme, width: f32, cols: usize) -> f32 {
        let gap = theme.space(3);
        (width - gap * (cols as f32 - 1.0)) / cols as f32
    }

    fn card_h(&mut self, text: &mut TextSystem, theme: &Theme, cw: f32) -> f32 {
        let name = self.cards.first_mut().map_or(0.0, |c| c.name.measure(text).height);
        (cw * 9.0 / 16.0).round() + theme.space(2) + name
    }

    fn grid_height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let w = width - px * 2.0;
        let cols = self.cols_for(theme, w);
        let rows = self.cards.len().div_ceil(cols) as f32;
        let gap = theme.space(3);
        rows * (self.card_h(text, theme, Self::card_w(theme, w, cols)) + gap) - gap
    }

    /// Kartı seçer: zaten seçiliyse yazmaz.
    fn choose(&mut self, env: &mut Env, i: usize) {
        let Some(b) = self.cards.get(i).map(|c| c.builtin) else { return };
        if self.selected == Some(i) {
            return;
        }
        self.selected = Some(i);
        env.push(Action::Group { label: t!("settings-wallpaper-label"), writes: wallpaper_source::choice_writes(b) });
    }

    fn move_focus(&mut self, key: Key) -> bool {
        let n = self.cards.len();
        if n == 0 {
            return false;
        }
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
}

impl Block for WallpaperGrid {
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
        let head = self.head_height(env.text, t, width - px * 2.0);
        let grid = self.grid_height(env.text, t, width);
        py * 2.0 + head + t.space(3) + grid
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let head = self.head_height(env.text, t, rect.width - px * 2.0);
        let w = rect.width - px * 2.0;
        let cols = self.cols_for(t, w);
        self.cols = cols;
        let gap = t.space(3);
        let cw = Self::card_w(t, w, cols);
        let ch = self.card_h(env.text, t, cw);
        let ph = (cw * 9.0 / 16.0).round();
        let top = rect.y + py + head + t.space(3);
        for (i, card) in self.cards.iter_mut().enumerate() {
            let (r, c) = (i / cols, i % cols);
            card.rect = Rect::new(rect.x + px + c as f32 * (cw + gap), top + r as f32 * (ch + gap), cw, ch);
            card.preview = Rect::new(card.rect.x, card.rect.y, cw, ph);
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
        !self.cards.is_empty()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        if focused && !self.focused {
            self.focus_i = self.selected.unwrap_or(0);
        }
        self.focused = focused;
        self.focus_visible = focused && visible;
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.cards.iter().any(|c| c.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.mode = env.theme.mode;
        self.refresh(env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.mode = theme.mode;
        self.head.set_theme(theme);
        self.check.set_theme(theme, now);
        for c in &mut self.cards {
            c.name.set_style(text_style(theme, TextRole::Label).with_max_lines(1));
            c.hover.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        self.draw_head(ui, self.rect.x + px, self.rect.y + py);
        // Kip temanın çözdüğü kip (kaydedilmemiş Koyu/Açık seçimi dahil).
        self.mode = t.mode;
        let variant = variant_of(self.mode);
        let radii = t.shape.medium();
        let surface_h = ui.painter.size().height;
        let ring_on = self.focus_visible;
        let mut waiting = false;
        for (i, card) in self.cards.iter_mut().enumerate() {
            let p = card.preview;
            let is_sel = self.selected == Some(i);
            // Yalnızca görünen kart çizilir (dolayısıyla istenir).
            if p.bottom() >= 0.0 && p.y <= surface_h {
                waiting |= card.thumb.draw(ui, p, radii, card.builtin, variant);
            }
            card.hover.draw_layer(ui.painter, p, radii, t, now);
            if card.hover.is_animating(now) {
                ui.request_frame();
            }
            if is_sel {
                // Vurgu çerçevesi önizlemenin içinde; köşede onay işareti.
                ui.painter.rect(p, &RectStyle { fill: Color::TRANSPARENT, radii, border: Some(Border { width: 3.0, color: ui.fade(c.primary) }) });
                let s = self.check.measure(ui.painter.text_system());
                let d = s.width.max(s.height) + t.space(2);
                let center = Point::new(p.right() - t.space(2) - d * 0.5, p.y + t.space(2) + d * 0.5);
                ui.painter.rect(Rect::new(center.x - d * 0.5, center.y - d * 0.5, d, d), &RectStyle::filled(ui.fade(c.primary), Corners::uniform(d * 0.5)));
                self.check.draw_centered(ui.painter, center, ui.fade(c.on_primary));
            }
            let nc = if is_sel { c.primary } else { c.on_surface };
            card.name.draw(ui.painter, Point::new(card.rect.x, p.bottom() + t.space(2)), ui.fade(nc));
            if ring_on && i == self.focus_i {
                let ring = p.expand(FOCUS_RING + 1.0);
                let o = FOCUS_RING + 1.0;
                let rr = Corners::new(radii.top_left + o, radii.top_right + o, radii.bottom_right + o, radii.bottom_left + o);
                ui.painter.rect(ring, &RectStyle { fill: Color::TRANSPARENT, radii: rr, border: Some(Border { width: FOCUS_RING, color: ui.fade(c.primary) }) });
            }
        }
        if waiting {
            ui.request_frame();
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::store::{FileId, KeyPath};
    use nuxut::ui::input::Modifiers;
    use toml_edit::Value;

    pub(crate) fn store(shell: &str, palette: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("s.toml"), shell).unwrap();
        std::fs::write(dir.path().join("p.toml"), palette).unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    fn desc() -> &'static SettingDesc {
        crate::registry::by_id("wallpaper.builtin").expect("kayıtta")
    }

    fn key(k: Key) -> KeyEvent {
        KeyEvent::new(k, KeyState::Pressed, None, Modifiers::default())
    }

    fn laid_out(grid: &mut WallpaperGrid, theme: &Theme, store: &Store, width: f32) -> Vec<Action> {
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme, store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let h = grid.height(&mut env, width);
        grid.place(&mut env, Rect::new(0.0, 0.0, width, h));
        actions
    }

    fn press(grid: &mut WallpaperGrid, theme: &Theme, store: &Store, k: Key) -> Vec<Action> {
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme, store, text: &mut text, now: Instant::now(), actions: &mut actions };
        grid.key(&mut env, &key(k));
        actions
    }

    /// Seçimin yazımı: iki dosya, tek geri al kaydı (Group), aynı `nuxut:<id>` adresi.
    pub(crate) fn assert_builtin_choice(actions: &[Action], id: &str) {
        let groups: Vec<_> = actions.iter().filter(|a| matches!(a, Action::Group { .. } | Action::Set { .. })).collect();
        assert_eq!(groups.len(), 1, "tek geri al kaydı: {actions:?}");
        let Action::Group { writes, .. } = groups[0] else { panic!("Group değil: {:?}", groups[0]) };
        let addr = format!("nuxut:{id}");
        let w: Vec<_> = writes.iter().map(|(f, k, v)| (*f, k.clone(), v.as_ref().and_then(Value::as_str).map(str::to_owned))).collect();
        assert_eq!(
            w,
            vec![(FileId::Shell, KeyPath::parse("wallpaper.path"), Some(addr.clone())), (FileId::Palette, KeyPath::parse("meta.image"), Some(addr))]
        );
    }

    #[test]
    fn empty_path_shows_default_card_selected() {
        let theme = Theme::default();
        let (_d, s) = store("", "");
        let g = WallpaperGrid::new(&theme, desc(), &s);
        assert_eq!(g.selected, Some(0));
        assert_eq!(g.cards[0].builtin.id, "pair-flight");
        assert_eq!(g.cards.len(), 9);
        assert!(!s.is_dirty(), "varsayılan yazılmaz");
        let (_d, s) = store("[wallpaper]\npath = \"/srv/a.png\"\n", "");
        assert_eq!(WallpaperGrid::new(&theme, desc(), &s).selected, None, "dosya seçiliyken kart yok");
        let fixed = nuxut_wallpapers::Selection { builtin: nuxut_wallpapers::find("aurora").unwrap(), variant: Some(Variant::Light) }.source();
        let (_d, s) = store(&format!("[wallpaper]\npath = \"{fixed}\"\n"), "");
        assert_eq!(WallpaperGrid::new(&theme, desc(), &s).selected, Some(5), "sabit varyant da kartını seçer");
    }

    #[test]
    fn keyboard_moves_and_writes_both_files_once() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let (_d, s) = store("", "");
        let mut g = WallpaperGrid::new(&theme, desc(), &s);
        laid_out(&mut g, &theme, &s, 720.0);
        assert_eq!(g.cols, 3, "geniş sayfada 3 sütun");
        g.set_focus(true, true, Instant::now());
        assert_eq!(g.focus_i, 0, "odak seçili kartta başlar");
        assert!(press(&mut g, &theme, &s, Key::Enter).is_empty(), "zaten seçili kart yazmaz");
        press(&mut g, &theme, &s, Key::Down);
        assert_eq!(g.focus_i, 3);
        press(&mut g, &theme, &s, Key::Right);
        let a = press(&mut g, &theme, &s, Key::Space);
        assert_builtin_choice(&a, "silhouette");
        assert_eq!(g.selected, Some(4));
        press(&mut g, &theme, &s, Key::Home);
        assert_eq!(g.focus_i, 0);
        press(&mut g, &theme, &s, Key::End);
        assert_eq!(g.focus_i, 8, "9 kart");
        assert_eq!(g.cards[0].name.text(), "Eş uçuş", "ad crate kataloğundan (cümle düzeni)");
        assert_eq!(g.cards[8].name.text(), "Kovalamaca", "maskotlu");
    }

    #[test]
    fn welcome_grid_has_two_columns_and_16_9_cards() {
        let theme = Theme::default();
        let (_d, s) = store("", "");
        let mut g = WallpaperGrid::new(&theme, desc(), &s).with_columns(WELCOME_COLS);
        laid_out(&mut g, &theme, &s, 900.0);
        assert_eq!(g.cols, 2);
        let p = g.cards[0].preview;
        assert!((p.width / p.height - 16.0 / 9.0).abs() < 0.02, "16:9: {p:?}");
        assert_eq!(g.cards[2].rect.x, g.cards[0].rect.x, "üçüncü kart ikinci satırda");
    }

    #[test]
    fn previews_follow_active_mode_and_card_size() {
        let mut theme = Theme::default();
        let (_d, s) = store("", "");
        let mut g = WallpaperGrid::new(&theme, desc(), &s);
        assert_eq!(g.mode, theme.mode);
        theme.mode = ThemeMode::Light;
        g.set_theme(&theme, Instant::now());
        assert_eq!(g.mode, ThemeMode::Light, "kip temadan (kaydedilmemiş seçim dahil)");
        laid_out(&mut g, &theme, &s, 720.0);
        // Çizim: her kart etkin kipin varyantını kartın fiziksel boyutunda ister.
        let mut text = TextSystem::new();
        let mut rec = nuxut::render::Recorder::new();
        rec.record(&mut text, (1500, 1500), 1.5, |p| {
            let mut ui = Ui::new(p, &theme, Instant::now());
            g.draw(&mut ui);
        });
        let p = g.cards[1].preview;
        let (b, m, px) = g.cards[1].thumb.requested().expect("istendi");
        assert_eq!((b.id, m), ("wingtip", Variant::Light));
        assert_eq!(px, ((p.width * 1.5).round() as u32, (p.height * 1.5).round() as u32));
        // Hero/monitör önizlemesinde de aynı kural; sabit varyant kipi geçersiz kılar.
        assert_eq!(WallSource::parse("nuxut:climb").variant(ThemeMode::Light), Variant::Light);
    }
}
