//! Hoş geldin penceresi görünümü (`nuxut-settings welcome`): karşılama, 3 adım, bitiş. Durum
//! [`crate::welcome::Welcome`]'dadır; her adımın satırları kayıttaki bloklardır
//! ([`crate::pages::build_ids`]) ve tek başlıksız kartta toplanır. Değişiklikler sayfalardaki
//! gibi anında yazılır, geri alınabilir.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use nuxut::motion::Animated;
use nuxut::render::TextAlign;
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent};
use nuxut::i18n::t;
use nuxut::ui::style::{faded_mark, state_layer};
use nuxut::ui::{BrandMark, Button, ButtonVariant, Icon, IconName, IconView, Label, Mascot, TextInput, TextRole, Ui, text_style};
use nuxut::{Corners, Point, Rect, RectStyle, Theme};

use crate::block::{Block, Env};
use crate::page::Page;
use crate::pages::fiyu::Spot;
use crate::pages::wallpaper_grid::{WELCOME_COLS, WallpaperGrid};
use crate::registry::PageId;
use crate::store::Store;
use crate::welcome::{self, Screen, Welcome};

/// Adım satırlarını tek başlıksız kartta toplar (kayıttaki bölüm adları burada anlamsız).
#[derive(Debug)]
struct OneCard {
    inner: Box<dyn Block>,
}

impl Block for OneCard {
    fn section(&self) -> Option<&str> {
        Some("")
    }
    fn id(&self) -> Option<&'static str> {
        self.inner.id()
    }
    fn visible(&self, store: &Store) -> bool {
        self.inner.visible(store)
    }
    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.inner.height(env, width)
    }
    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.inner.place(env, rect)
    }
    fn pointer(&mut self, env: &mut Env, event: &PointerEvent) -> bool {
        self.inner.pointer(env, event)
    }
    fn wants_scroll(&self, p: Point) -> bool {
        self.inner.wants_scroll(p)
    }
    fn key(&mut self, env: &mut Env, event: &KeyEvent) -> bool {
        self.inner.key(env, event)
    }
    fn captures_keys(&self) -> bool {
        self.inner.captures_keys()
    }
    fn focusable(&self) -> bool {
        self.inner.focusable()
    }
    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        self.inner.set_focus(focused, visible, now)
    }
    fn focus_step(&mut self, forward: bool, now: Instant) -> bool {
        self.inner.focus_step(forward, now)
    }
    fn overlay_open(&self) -> bool {
        self.inner.overlay_open()
    }
    fn sync(&mut self, env: &mut Env) {
        self.inner.sync(env)
    }
    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.inner.set_theme(theme, now)
    }
    fn draw(&mut self, ui: &mut Ui) {
        self.inner.draw(ui)
    }
    fn text_input(&mut self) -> Option<&mut TextInput> {
        self.inner.text_input()
    }
    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.inner.cursor(p)
    }
    fn demo_expand(&mut self, now: Instant) {
        self.inner.demo_expand(now)
    }
    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        self.inner.service_event(env, event)
    }
}

/// Bir adımın satırları: kayıttaki bloklar tek kartta; Uygulamalar adımında birleşik görünüme
/// bağlı satırlar kayıttaki koşulla (`registry::UNIFIED_GATED`), kapalıyken anahtarın altında
/// Görünüm sayfasıyla ortak açıklama (burada düğmesiz: anahtar hemen üstünde). Masaüstü adımında
/// `takeover` varsa Hyprland ana ayarı devralma seçimi en üstte; açılış görünümü adımı özel kart.
fn step_blocks(theme: &Theme, step: &welcome::Step, store: &Store, takeover: Option<&Rc<Cell<bool>>>) -> Vec<Box<dyn Block>> {
    if step.kind == welcome::StepKind::System {
        return vec![Box::new(crate::pages::system_theme::SystemThemeCard::new(theme))];
    }
    let mut out: Vec<Box<dyn Block>> = Vec::new();
    // Masaüstü adımı: Hyprland ana ayarının devralınması en üstte (yalnız devralınabilirken).
    if let (welcome::StepKind::Desktop, Some(choice)) = (step.kind, takeover) {
        out.push(Box::new(crate::pages::hyprland_config::HyprTakeover::new(theme, choice.clone())));
    }
    for b in crate::pages::build_ids(theme, step.settings, store) {
        let id = b.id();
        // Yerleşik duvar kağıtları: sayfadaki ızgaranın aynısı, dar pencerede 2 sütun.
        let b: Box<dyn Block> = match id.and_then(crate::registry::by_id).filter(|d| d.control == crate::registry::Control::Custom(crate::registry::CustomId::WallpaperBuiltins)) {
            Some(desc) => Box::new(WallpaperGrid::new(theme, desc, store).with_columns(WELCOME_COLS).with_title()),
            None => b,
        };
        out.push(Box::new(OneCard { inner: b }));
        if id == Some("desktop.unified") {
            out.push(crate::pages::unified_off_note(theme, "", false));
        }
    }
    out
}

/// Kullanıcının bir düğmeye basmasının sonucu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    None,
    /// "Kapat" (bitiş ekranı): shell'e `setup done` bildirilir, pencere kapanır.
    Finished,
}

#[derive(Debug)]
pub struct WelcomeView {
    model: Welcome,
    page: Option<Page>,
    hero: IconView,
    /// Karşılamada ikon yerine marka: iki tonlu logo + slogan.
    mark: BrandMark,
    /// Maskot Fiyu: yalnız bitişte kutlama (ikonun yerine); gizliyken ikon döner. Karşılamada
    /// her zaman marka logosu (2026-10-14; [`crate::pages::fiyu::Spot::WelcomeDone`]).
    fiyu: Mascot,
    slogan: Label,
    title: Label,
    body: Label,
    progress: Label,
    back: Button,
    skip: Button,
    primary: Button,
    transition: Animated<f32>,
    rect: Rect,
    column: Rect,
    page_rect: Rect,
    /// Sayfa odağı (Tab ile satırlar arasında).
    page_focus: bool,
    /// Hyprland ana ayarı devralma seçimi (Masaüstü adımı; yalnız devralınabilirken `Some`, seçili
    /// gelir). Bitir'de okunur ([`Self::takeover_chosen`]).
    takeover: Option<Rc<Cell<bool>>>,
}

/// Metin sütununun en geniş hâli.
fn column_width(theme: &Theme) -> f32 {
    theme.space(150)
}

/// Karşılama ekranındaki marka logosunun kenarı (`space` birimi): 128 px.
pub const MARK_SPACE: u32 = 32;

/// İşaretin görünür üst ve alt kenarı viewBox yüksekliğinin bu oranlarında (≈80/512, ≈442/512):
/// dikey denge ve logo–başlık aralığı işaretin boş paylarını saymadan kurulur. Yatayda görünür
/// işaret (≈92–404/512) kutuda zaten ortalı.
const MARK_TOP: f32 = 80.0 / 512.0;
const MARK_BOTTOM: f32 = 442.0 / 512.0;

/// Karşılama/bitiş metinleri ortalı, adım metinleri sola hizalı.
fn hero_style(theme: &Theme, role: TextRole, centered: bool) -> nuxut::render::TextStyle {
    let style = text_style(theme, role);
    if centered { style.with_align(TextAlign::Center) } else { style }
}

/// Karşılama/bitiş ekranının (adım satırı olmayan) yerleşimi; geçiş kaydırması hariç.
#[derive(Debug, Clone, Copy, PartialEq)]
struct HeroLayout {
    /// Karşılamada logo, bitişte ikonun yuvarlak zemini.
    hero: Rect,
    /// Karşılamada sloganın sol üstü (bitişte yok).
    slogan: Option<Point>,
    title: Point,
    body: Point,
    /// Açıklamanın altı.
    bottom: f32,
}

impl WelcomeView {
    pub fn new(theme: &Theme, store: &Store) -> Self {
        let mut transition = Animated::new(1.0, theme.motion.spatial_fast);
        transition.jump_to(1.0);
        let mut s = Self {
            model: Welcome::new(),
            page: None,
            hero: IconView::with_role(theme, Icon::named(IconName::Palette), TextRole::IconLarge),
            mark: BrandMark::new().duotone().with_size(theme.space(MARK_SPACE)),
            fiyu: Mascot::new(theme, Spot::WelcomeDone.mood(), Spot::WelcomeDone.size()),
            slogan: Label::new(t!("settings-brand-slogan"), hero_style(theme, TextRole::Label, true)),
            title: Label::new("", text_style(theme, TextRole::Display)),
            body: Label::new("", text_style(theme, TextRole::Body)),
            progress: Label::new("", text_style(theme, TextRole::Label)),
            back: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::ArrowBack)).with_label(theme, t!("settings-welcome-button-back")),
            skip: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-welcome-button-skip")),
            primary: Button::new(theme, ButtonVariant::Filled).with_label(theme, t!("settings-welcome-button-start")),
            transition,
            rect: Rect::ZERO,
            column: Rect::ZERO,
            page_rect: Rect::ZERO,
            page_focus: false,
            takeover: crate::pages::hyprland_config::available().then(|| Rc::new(Cell::new(true))),
        };
        s.enter(theme, store, Instant::now(), false);
        s
    }

    /// Bitir'de Hyprland ana ayarı devralınsın mı (adım gösterildi ve seçili).
    pub fn takeover_chosen(&self) -> bool {
        self.takeover.as_ref().is_some_and(|c| c.get())
    }

    pub fn model(&self) -> &Welcome {
        &self.model
    }

    /// Bu ekranın maskot yeri: yalnız bitiş (kutlama). Karşılamada marka logosu her zaman
    /// görünür, maskot yok: aynı ekranda iki kırlangıç olmasın, ilk izlenim marka
    /// (2026-10-14). Adımlarda yok.
    pub fn spot(&self) -> Option<Spot> {
        match self.model.screen() {
            Screen::Done => Some(Spot::WelcomeDone),
            Screen::Intro | Screen::Step(_) => None,
        }
    }

    /// Kahraman görseli maskot mu (bitiş ve `[appearance] mascot` açık).
    fn fiyu_shown(&self) -> bool {
        self.spot().is_some() && self.fiyu.measure().width > 0.0
    }

    /// Ekran değişti: metinler, düğmeler ve adımın satırları.
    fn enter(&mut self, theme: &Theme, store: &Store, now: Instant, animate: bool) {
        let (icon, title, body) = match self.model.screen() {
            Screen::Intro => {
                let (title, body) = welcome::intro_text();
                (IconName::Palette, title, body)
            }
            Screen::Step(_) => {
                let s = self.model.step().expect("adım");
                (IconName::Palette, s.title(), s.body())
            }
            Screen::Done => {
                let (title, body) = welcome::done_text();
                (IconName::CheckCircle, title, body)
            }
        };
        self.hero.set_icon(Icon::named(icon));
        if let Some(spot) = self.spot() {
            self.fiyu.set_mood(spot.mood());
        }
        let step = matches!(self.model.screen(), Screen::Step(_));
        self.title.set_text(&title);
        self.title.set_style(hero_style(theme, if step { TextRole::Title } else { TextRole::Display }, !step));
        self.body.set_text(&body);
        self.body.set_style(hero_style(theme, TextRole::Body, !step));
        self.progress.set_text(self.model.progress_text().as_deref().unwrap_or(""));
        self.refresh_primary();
        self.page = self.model.step().map(|s| {
            let blocks = step_blocks(theme, s, store, self.takeover.as_ref());
            Page::new(theme, PageId::Appearance, blocks, store)
        });
        self.page_focus = false;
        if animate {
            self.transition.jump_to(0.0);
            self.transition.set_target(1.0, now);
        }
    }

    /// Pencerenin içerik alanı (başlık çubuğu altı).
    pub fn set_rect(&mut self, theme: &Theme, text: &mut nuxut::TextSystem, rect: Rect) {
        self.rect = rect;
        let w = column_width(theme).min(rect.width - theme.space(8) * 2.0).max(theme.space(60));
        let x = rect.x + ((rect.width - w) * 0.5).floor();
        self.column = Rect::new(x, rect.y, w, rect.height);
        self.title.set_max_width(Some(w));
        self.body.set_max_width(Some(w));
        // Alt eylem çubuğu.
        let bh = self.primary.preferred_size(text, theme).height;
        let by = rect.bottom() - theme.space(8) - bh;
        let ps = self.primary.preferred_size(text, theme);
        self.primary.set_rect(Rect::new(x + w - ps.width, by, ps.width, ps.height));
        let ss = self.skip.preferred_size(text, theme);
        self.skip.set_rect(Rect::new(self.primary.rect().x - theme.space(2) - ss.width, by, ss.width, ss.height));
        let gs = self.back.preferred_size(text, theme);
        self.back.set_rect(Rect::new(x - theme.space(3), by, gs.width, gs.height));
        // Başlık bloğunun yüksekliği, kalanı satırlar.
        let top = self.head_height(theme, text);
        let page_top = rect.y + top;
        // Sayfa kendi kenar boşluğunu (space(6)) bırakır: sütunla hizalansın diye genişletilir.
        let margin = theme.space(6);
        self.page_rect = Rect::new(x - margin, page_top, w + margin * 2.0, (by - theme.space(4) - page_top).max(0.0));
        if let Some(p) = &mut self.page {
            p.set_viewport(self.page_rect);
        }
    }

    /// İlerleme + başlık + açıklama yüksekliği (ekrana göre).
    fn head_height(&mut self, theme: &Theme, text: &mut nuxut::TextSystem) -> f32 {
        let step = self.page.is_some();
        let mut h = if step { theme.space(6) } else { theme.space(24) };
        if step {
            h += self.progress.measure(text).height + theme.space(3);
        } else {
            h += self.hero.measure(text).height + theme.space(6);
        }
        h += self.title.measure(text).height + theme.space(2) + self.body.measure(text).height + theme.space(4);
        h
    }

    /// Karşılama/bitiş yerleşimi (ortalanmış kahraman): içerik, düğme çubuğuna kadarki alanda
    /// dikeyde ortanın biraz üstünde, yatayda sütunun ortasında. Karşılama: iki tonlu logo →
    /// başlık → slogan → açıklama. Bitiş: vurgu renginde yuvarlak zeminli büyük ikon → başlık →
    /// açıklama. Metinler sütun genişliğinde ortalı hizalıdır; çizim noktaları sütunun solu.
    fn hero_layout(&mut self, theme: &Theme, text: &mut nuxut::TextSystem) -> HeroLayout {
        let t = theme;
        let col = self.column;
        let intro = self.model.screen() == Screen::Intro;
        let fiyu = self.fiyu_shown();
        let side = if fiyu {
            self.fiyu.size()
        } else if intro {
            t.space(MARK_SPACE)
        } else {
            let hs = self.hero.measure(text);
            hs.width.max(hs.height) + t.space(8)
        };
        // Logonun viewBox'ı görünür işaretten geniş: dikey denge görünür üst/alt kenarla kurulur.
        let (vis_top, vis_bottom) = if intro && !fiyu { (side * MARK_TOP, side * MARK_BOTTOM) } else { (0.0, side) };
        self.slogan.set_max_width(Some(col.width));
        let th = self.title.measure(text).height;
        let sh = if intro { self.slogan.measure(text).height } else { 0.0 };
        let bh = self.body.measure(text).height;
        let slogan_gap = if intro { t.space(2) + sh } else { 0.0 };
        let h = (vis_bottom - vis_top) + t.space(6) + th + slogan_gap + t.space(4) + bh;
        let avail = self.primary.rect().y - col.y;
        let top = col.y + ((avail - h) * 0.45).max(t.space(8)).round();
        let hero = Rect::new((col.x + (col.width - side) * 0.5).round(), top - vis_top.round(), side, side);
        let mut y = top + (vis_bottom - vis_top).round() + t.space(6);
        let title = Point::new(col.x, y);
        y += th;
        let slogan = intro.then(|| Point::new(col.x, y + t.space(2)));
        y += slogan_gap + t.space(4);
        HeroLayout { hero, slogan, title, body: Point::new(col.x, y), bottom: y + bh }
    }

    fn on_system_step(&self) -> bool {
        self.model.step().is_some_and(|s| s.kind == welcome::StepKind::System)
    }

    /// Birincil düğmenin metni: açılış görünümü adımında uygulanacak iş varsa "Uygula", sürerken
    /// "Uygulanıyor…"; diğer ekranlarda modelinki.
    fn refresh_primary(&mut self) {
        let label = if !self.on_system_step() {
            self.model.primary_label()
        } else if crate::services::system_theme::running() {
            t!("settings-system-theme-button-running")
        } else if crate::services::system_theme::wants_apply() {
            t!("settings-system-theme-button-apply")
        } else {
            self.model.primary_label()
        };
        self.primary.set_label(&label);
    }

    fn act(&mut self, env: &mut Env, which: usize) -> Outcome {
        match which {
            0 => self.model.back(),
            1 => self.model.skip(),
            _ if self.model.is_done() => return Outcome::Finished,
            // Açılış görünümü: anahtar açıksa birincil düğme önce uygular (pkexec), bitene
            // kadar adımda kalır; sonuç kartta görünür, ikinci basış ilerler.
            _ if self.on_system_step() && crate::services::system_theme::running() => return Outcome::None,
            _ if self.on_system_step() && crate::services::system_theme::wants_apply() => {
                crate::services::system_theme::start(crate::services::system_theme::Op::Apply);
                self.refresh_primary();
                return Outcome::None;
            }
            _ => self.model.next(),
        }
        self.enter(env.theme, env.store, env.now, true);
        Outcome::None
    }

    pub fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> (bool, Outcome) {
        let now = env.now;
        // Açık açılır menü (varyant seçimi) olayın tek sahibi: alttaki düğmeler tetiklenmez.
        if let Some(p) = self.page.as_mut().filter(|p| p.overlay_open()) {
            return (p.pointer(env, e), Outcome::None);
        }
        let mut repaint = false;
        let mut hit = None;
        let show = [self.model.can_back(), self.model.can_skip(), true];
        for (i, b) in [&mut self.back, &mut self.skip, &mut self.primary].into_iter().enumerate() {
            if !show[i] {
                continue;
            }
            let r = b.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                hit = Some(i);
            }
        }
        if let Some(i) = hit {
            return (true, self.act(env, i));
        }
        if let Some(p) = &mut self.page {
            repaint |= p.pointer(env, e);
            if let nuxut::ui::input::PointerKind::Press(_) = e.kind {
                self.page_focus = self.page_rect.contains(e.position) && p.has_focus();
            }
        }
        (repaint, Outcome::None)
    }

    pub fn key(&mut self, env: &mut Env, e: &KeyEvent) -> (bool, Outcome) {
        if e.state == KeyState::Released {
            return (false, Outcome::None);
        }
        if let Some(p) = &mut self.page {
            // Açık menü her tuşu alır (Escape kapatır, Tab/Enter sayfadan çıkmaz).
            if p.captures_keys() {
                return (p.key(env, e), Outcome::None);
            }
            if e.key == Key::Tab {
                let used = p.move_focus(env, !e.modifiers.shift);
                self.page_focus = used;
                return (used, Outcome::None);
            }
            if self.page_focus && p.key(env, e) {
                return (true, Outcome::None);
            }
        }
        // Sayfa odakta değilken: Enter ileri, Alt+Sol geri.
        match e.key {
            Key::Enter => (true, self.act(env, 2)),
            Key::Left if e.modifiers.alt && self.model.can_back() => (true, self.act(env, 0)),
            _ => (false, Outcome::None),
        }
    }

    pub fn cursor(&self, p: Point) -> Option<CursorShape> {
        let show = [self.model.can_back(), self.model.can_skip(), true];
        [&self.back, &self.skip, &self.primary]
            .into_iter()
            .zip(show)
            .find_map(|(b, s)| s.then(|| b.cursor(p)).flatten())
            .or_else(|| self.page.as_ref().and_then(|pg| pg.cursor(p)))
    }

    pub fn focused_text_input(&mut self) -> Option<&mut TextInput> {
        self.page.as_mut()?.focused_text_input()
    }

    pub fn sync(&mut self, env: &mut Env) {
        if let Some(p) = &mut self.page {
            p.sync(env);
        }
    }

    pub fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::SystemTheme) {
            self.refresh_primary();
        }
        self.page.as_mut().is_some_and(|p| p.service_event(env, event))
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        let step = self.page.is_some();
        self.hero.set_theme(theme, now);
        self.mark.set_theme(theme, now);
        self.mark.set_size(theme.space(MARK_SPACE));
        self.fiyu.set_theme(theme, now);
        self.slogan.set_style(hero_style(theme, TextRole::Label, true));
        self.title.set_style(hero_style(theme, if step { TextRole::Title } else { TextRole::Display }, !step));
        self.body.set_style(hero_style(theme, TextRole::Body, !step));
        self.progress.set_style(text_style(theme, TextRole::Label));
        for b in [&mut self.back, &mut self.skip, &mut self.primary] {
            b.set_theme(theme, now);
        }
        self.transition.set_motion(theme.motion.spatial_fast, now);
        if let Some(p) = &mut self.page {
            p.set_theme(theme, now);
        }
    }

    /// İlerleme noktaları: etkin adım uzun hap, görülen `primary` %50, diğerleri çizgi tonu.
    fn draw_dots(&self, ui: &mut Ui, x: f32, cy: f32) -> f32 {
        let t = ui.theme;
        let c = &t.colors;
        let d = t.space(2);
        let mut x = x;
        for (active, seen) in self.model.dots() {
            let w = if active { d * 3.0 } else { d };
            let color = if active { c.primary } else if seen { faded_mark(t, c.primary, 0.5, &[]) } else { c.outline };
            ui.painter.rect(Rect::new(x, cy - d * 0.5, w, d), &RectStyle::filled(ui.fade(color), Corners::uniform(d * 0.5)));
            x += w + t.space(2);
        }
        x
    }

    pub fn draw(&mut self, ui: &mut Ui, store: &Store) {
        let t = ui.theme;
        let c = &t.colors;
        let now = ui.now;
        let tr = self.transition.value(now).clamp(0.0, 1.0);
        if !self.transition.is_settled(now) {
            ui.request_frame();
        }
        let offset = (1.0 - tr) * t.space(4);
        let col = self.column;
        let step = self.page.is_some();
        let hero = (!step).then(|| self.hero_layout(t, ui.painter.text_system()));
        let down = |p: Point| Point::new(p.x, p.y + offset);
        ui.with_alpha(tr, |ui| {
            let Some(l) = hero else {
                let mut y = col.y + t.space(6) + offset;
                let ps = self.progress.measure_in(ui.painter);
                let end = self.draw_dots(ui, col.x, y + ps.height * 0.5);
                self.progress.draw(ui.painter, Point::new(end + t.space(2), y), ui.fade(c.on_surface_variant));
                y += ps.height + t.space(3);
                let th = self.title.measure_in(ui.painter).height;
                self.title.draw(ui.painter, Point::new(col.x, y), ui.fade(c.on_surface));
                self.body.draw(ui.painter, Point::new(col.x, y + th + t.space(2)), ui.fade(c.on_surface_variant));
                return;
            };
            let r = l.hero.translate(Point::new(0.0, offset));
            if self.fiyu_shown() {
                // Bitiş, maskot açık: ikonun yerinde kutlayan Fiyu.
                self.fiyu.set_rect(r);
                self.fiyu.draw(ui);
            } else if let Some(sp) = l.slogan {
                // Karşılama: marka (iki tonlu logo, renkler temadan) + slogan; maskot ayarından
                // bağımsız, her zaman (2026-10-14).
                self.mark.set_rect(r);
                self.mark.draw(ui);
                self.slogan.draw(ui.painter, down(sp), ui.fade(c.primary));
            } else {
                // Bitiş: vurgu renginde yuvarlak zeminli büyük ikon.
                let radius = r.width * 0.5;
                ui.painter.rect(r, &RectStyle::filled(ui.fade(state_layer(t, c.primary, &[], 0.16)), Corners::uniform(radius)));
                self.hero.draw_centered(ui.painter, r.center(), ui.fade(c.primary));
            }
            self.title.draw(ui.painter, down(l.title), ui.fade(c.on_surface));
            self.body.draw(ui.painter, down(l.body), ui.fade(c.on_surface_variant));
        });
        if let Some(p) = &mut self.page {
            let r = self.page_rect;
            p.set_viewport(Rect::new(r.x, r.y + offset, r.width, r.height - offset));
            ui.painter.push_clip(r);
            ui.with_alpha(tr, |ui| p.draw(ui, store));
            ui.painter.pop_clip();
        }
        if self.model.can_back() {
            self.back.draw(ui);
        }
        if self.model.can_skip() {
            self.skip.draw(ui);
        }
        self.primary.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store(framework: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let fw = dir.path().join("framework.toml");
        std::fs::write(&fw, framework).unwrap();
        let s = Store::open(&fw, dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        (dir, s)
    }

    /// Uygulamalar adımı: birleşik görünüm kapalıyken ikon/imleç satırları yok, yerinde kısa
    /// açıklama (anahtarın hemen altında); açıkken (varsayılan) tersi.
    #[test]
    fn apps_step_hides_theme_grids_when_unified_is_off() {
        let theme = Theme::default();
        let step = &welcome::STEPS[1];
        let shown = |s: &Store| -> (Vec<Option<&'static str>>, bool) {
            let blocks = step_blocks(&theme, step, s, None);
            let visible: Vec<_> = blocks.iter().filter(|b| b.visible(s)).collect();
            let ids = visible.iter().map(|b| b.id()).collect();
            // Açıklama kimliksiz tek bloktur; anahtardan hemen sonra gelir.
            let note_after_toggle = visible.windows(2).any(|w| w[0].id() == Some("desktop.unified") && w[1].id().is_none());
            (ids, note_after_toggle)
        };
        let (_t, off) = store("[desktop]\nunified = false\n");
        let (ids, note) = shown(&off);
        for id in crate::registry::UNIFIED_GATED {
            assert!(!ids.contains(&Some(id)), "kapalıyken {id} görünmez");
        }
        assert!(ids.contains(&Some("desktop.unified")), "anahtar hep görünür");
        assert!(note, "kapalıyken açıklama var: {ids:?}");
        assert_eq!(ids.iter().filter(|i| i.is_none()).count(), 1);

        for on in ["", "[desktop]\nunified = true\n"] {
            let (_t, s) = store(on);
            let (ids, note) = shown(&s);
            for id in ["desktop.icon_theme", "desktop.cursor_theme"] {
                assert!(ids.contains(&Some(id)), "açıkken {id} görünür ({on:?})");
            }
            assert!(!note && !ids.contains(&None), "açıkken açıklama yok ({on:?})");
        }
    }

    /// Görünüm adımı: sayfadaki yerleşik duvar kağıdı ızgarası 2 sütunla; seçim sayfadakiyle
    /// aynı yazılır (iki dosya, tek geri al kaydı). Boş config'te varsayılan seçili, yazım yok.
    #[test]
    fn look_step_shows_builtin_grid_and_writes_like_the_page() {
        use nuxut::ui::input::Modifiers;
        let theme = Theme::default();
        let (_d, s) = store("");
        let mut blocks = step_blocks(&theme, &welcome::STEPS[0], &s, None);
        let ids: Vec<_> = blocks.iter().map(|b| b.id()).collect();
        assert_eq!(ids, [Some("colors.mode"), Some("wallpaper"), Some("wallpaper.builtin"), Some("colors.source")]);
        let grid = &mut blocks[2];
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &s, text: &mut text, now: Instant::now(), actions: &mut actions };
        let w = column_width(&theme);
        let h = grid.height(&mut env, w);
        grid.place(&mut env, Rect::new(0.0, 0.0, w, h));
        grid.set_focus(true, true, Instant::now());
        let key = |k| KeyEvent::new(k, KeyState::Pressed, None, Modifiers::default());
        assert!(grid.key(&mut env, &key(Key::Enter)) && env.actions.is_empty(), "varsayılan zaten seçili");
        // 2 sütun: aşağı ok bir satır = 2 kart (pair-flight → climb).
        assert!(grid.key(&mut env, &key(Key::Down)));
        assert!(grid.key(&mut env, &key(Key::Enter)));
        crate::pages::wallpaper_grid::tests::assert_builtin_choice(&actions, "climb");
    }

    /// Karşılama (ilk ekran), ortalanmış kahraman: 128 px iki tonlu logo → başlık → slogan →
    /// açıklama; logo ve metinler sütunda yatay ortalı, blok düğme çubuğuna kadarki alanda
    /// dikeyde dengeli (üst boşluk alttakinden az, ama yarısından çok); hepsi 720×640
    /// pencerenin içerik alanında. Bitiş ekranı logo değil ikon gösterir; adımlarda marka yok.
    #[test]
    fn intro_shows_centered_brand_and_fits_fixed_window() {
        let (_d, store) = store("");
        // Maskot gizli: bitişte ikon (maskotlu hâli `intro_shows_brand_done_celebrates`).
        let theme = crate::pages::fiyu::tests::hidden_theme();
        let mut text = nuxut::TextSystem::new();
        let mut v = WelcomeView::new(&theme, &store);
        // Katalog testte kurulmaz (t! anahtarı döner): gerçek metinle ölç.
        v.slogan.set_text("The penguin can't fly. This one can.");
        let (w, h) = crate::app::WELCOME_SIZE;
        let top = crate::app::header_height(&theme);
        let content = Rect::new(0.0, top, w as f32, h as f32 - top);
        v.set_rect(&theme, &mut text, content);
        assert_eq!(v.model().screen(), Screen::Intro);
        let l = v.hero_layout(&theme, &mut text);
        assert_eq!(l.hero.width, 128.0);
        assert_eq!(l.hero.height, 128.0);
        assert_eq!(v.mark.size(), 128.0);
        assert_eq!(v.mark.style(), nuxut::ui::BrandStyle::Duotone);
        let col = v.column;
        assert!((l.hero.center().x - col.center().x).abs() <= 0.5, "logo ortalı: {:?} / {:?}", l.hero, col);
        assert!(l.hero.x >= content.x && l.hero.y >= content.y, "logo içerik alanında: {:?}", l.hero);
        // Sıra: logo → başlık → slogan → açıklama.
        let sp = l.slogan.expect("karşılamada slogan");
        assert!(l.title.y >= l.hero.y + 128.0 * MARK_BOTTOM, "başlık işaretin görünür altının altında");
        assert!(sp.y > l.title.y + v.title.measure(&mut text).height - 0.5, "slogan başlığın altında");
        assert!(l.body.y > sp.y + v.slogan.measure(&mut text).height, "açıklama sloganın altında");
        // Ortalı hiza: metinler sütun genişliğinde, sütunun solundan çizilir.
        for (label, p) in [(&mut v.title, l.title), (&mut v.slogan, sp), (&mut v.body, l.body)] {
            assert_eq!(label.style().align, TextAlign::Center);
            assert_eq!(p.x, col.x);
            assert!((label.measure(&mut text).width - col.width).abs() <= 0.5, "metin sütun genişliğinde");
        }
        // Dikey denge: görünür işaretin üstü ile açıklamanın altı arasındaki blok.
        let above = l.hero.y + 128.0 * MARK_TOP - col.y;
        let below = v.primary.rect().y - l.bottom;
        assert!(above < below && above > below * 0.5, "dengeli: üst {above} / alt {below}");
        assert!(v.primary.rect().bottom() <= content.bottom() && v.primary.rect().right() <= content.right(), "düğme pencerede");

        // Bitiş: ikonlu daire ortalı, slogan yok; o da sığar.
        v.model.next();
        v.model.next();
        v.model.next();
        v.model.next();
        v.model.next();
        assert!(v.model.is_done());
        v.enter(&theme, &store, Instant::now(), false);
        v.set_rect(&theme, &mut text, content);
        let l = v.hero_layout(&theme, &mut text);
        assert!(l.slogan.is_none());
        assert!((l.hero.center().x - col.center().x).abs() <= 0.5);
        assert!(l.bottom <= v.primary.rect().y);

        // Adımlar sola hizalı kalır.
        v.model.back();
        v.enter(&theme, &store, Instant::now(), false);
        assert!(v.page.is_some());
        assert_eq!(v.title.style().align, TextAlign::Start);
        assert_eq!(v.body.style().align, TextAlign::Start);
    }

    /// Maskot açıkken (karar 2026-10-14): karşılamada marka logosu yine görünür,
    /// Fiyu yok (aynı ekranda iki kırlangıç olmasın); bitişte ikonun yerinde kutlayan Fiyu;
    /// adımlarda maskot yok. Ortalı ve pencereye sığar.
    #[test]
    fn intro_shows_brand_done_celebrates() {
        use nuxut::ui::Mood;
        let (_d, store) = store("");
        let theme = Theme::default();
        assert!(theme.appearance.mascot, "varsayılan temada maskot açık");
        let mut text = nuxut::TextSystem::new();
        let mut v = WelcomeView::new(&theme, &store);
        let (w, h) = crate::app::WELCOME_SIZE;
        let top = crate::app::header_height(&theme);
        let content = Rect::new(0.0, top, w as f32, h as f32 - top);
        v.set_rect(&theme, &mut text, content);
        assert_eq!(v.model().screen(), Screen::Intro);
        assert_eq!(v.spot(), None, "ilk ekranda maskot yeri yok");
        assert!(!v.fiyu_shown(), "ilk ekranda Fiyu çizilmez");
        let l = v.hero_layout(&theme, &mut text);
        assert_eq!((l.hero.width, l.hero.height), (128.0, 128.0));
        assert_eq!(v.mark.size(), 128.0, "kahraman marka logosu");
        assert!(l.slogan.is_some(), "slogan kalır");
        assert!(l.title.y >= l.hero.y + 128.0 * MARK_BOTTOM, "başlık logonun görünür altının altında");
        assert!((l.hero.center().x - v.column.center().x).abs() <= 0.5);
        for _ in 0..=welcome::STEPS.len() {
            v.model.next();
        }
        v.enter(&theme, &store, Instant::now(), false);
        v.set_rect(&theme, &mut text, content);
        assert_eq!(v.spot(), Some(Spot::WelcomeDone));
        assert_eq!(v.fiyu.mood(), Mood::Celebrate);
        let l = v.hero_layout(&theme, &mut text);
        assert_eq!(l.hero.width, 128.0);
        assert!(l.bottom <= v.primary.rect().y);
        v.model.back();
        v.enter(&theme, &store, Instant::now(), false);
        assert_eq!(v.spot(), None);
        assert!(!v.fiyu_shown());
    }
}
