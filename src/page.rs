//! Sayfa: kaydırılabilir blok sütunu. Aynı bölümdeki ardışık bloklar tek kartta toplanır,
//! bölüm başlığı kartın üstünde durur. Görünürlüğü değişen bloklar (ör. gölge kapanınca
//! ayarları) yükseklikleri yayla açılıp kapanarak gelir/gider.

use std::time::Instant;

use nuxut::motion::Animated;
use nuxut::ui::input::{Button as MouseButton, CursorShape, KeyEvent, PointerEvent, PointerKind};
use nuxut::ui::style::{FLASH_TONE, state_layer_under};
use nuxut::ui::{Label, ScrollView, TextRole, Ui, text_style};
use nuxut::{Point, Rect, RectStyle, TextSystem, Theme};

use crate::block::{Action, Block, Env};
use crate::page_ui::PageUi;
use crate::registry::{self, PageId, SettingDesc, UiCond};
use crate::store::Store;

/// İçerik sütununun azami genişliği: TÜM sayfalarda tek değer `space(260)` (1040 px; hero,
/// tuval, üç widget sütunu sığar). Sayfa geçişinde sütun kenarı oynamaz (kullanıcı kararı;
/// t094'teki metin/görsel ayrımı kalktı).
pub fn content_width(theme: &Theme) -> f32 {
    theme.space(260)
}

/// Dar viewport'ta sütunun iki yanındaki en az boşluk.
pub fn content_margin(theme: &Theme) -> f32 {
    theme.space(6)
}

/// Yatayda ortalı sütun: `(x, genişlik)`, viewport'un yerel koordinatında. Genişlik azami
/// [`content_width`]; dar viewport'ta iki yanda [`content_margin`] kalır.
pub fn column(theme: &Theme, viewport_width: f32) -> (f32, f32) {
    let margin = content_margin(theme);
    let width = (viewport_width - margin * 2.0).min(content_width(theme)).max(theme.space(40));
    (((viewport_width - width) / 2.0).max(margin).round(), width)
}

#[derive(Debug)]
struct Slot {
    block: Box<dyn Block>,
    /// Kayıttaki `ui_if` (sayfa durumuna bağlı görünürlük).
    ui_if: Option<UiCond>,
    /// Görünürlük (0..1), yükseklik çarpanı ve opaklık.
    presence: Animated<f32>,
    /// Arama sonucundan gelince parlama (framework açılır satırıyla aynı: `timing.flash_hold` tam,
    /// sonra `motion.effects` ile söner).
    highlight: Animated<f32>,
    /// Vurgunun tam opaklıkta kalacağı son an (bekleyiş kare değil `wake_at`).
    highlight_until: Option<Instant>,
    /// İçerik koordinatında yerleşim (kaydırmasız).
    local: Rect,
}

#[derive(Debug)]
struct Header {
    label: Label,
    at: Point,
    alpha: f32,
}

#[derive(Debug)]
pub struct Page {
    pub id: PageId,
    slots: Vec<Slot>,
    headers: Vec<Header>,
    cards: Vec<(Rect, f32)>,
    scroll: ScrollView,
    viewport: Rect,
    content_h: f32,
    focus: Option<usize>,
    hover_inside: bool,
    /// Dosyaya yazılmayan sayfa durumu (açılır satırlar, seçili widget); sayfa oturum boyunca
    /// yaşadığı için oturumda hatırlanır.
    ui: PageUi,
    /// Blokların kayıt satırları (`ui_if` ve aramanın kuracağı durum buradan).
    settings: &'static [SettingDesc],
}

impl Slot {
    fn shown(&self, store: &Store, ui: &PageUi) -> bool {
        self.block.visible(store) && self.ui_if.is_none_or(|c| (c.shown)(ui, store))
    }
}

impl Page {
    pub fn new(theme: &Theme, id: PageId, blocks: Vec<Box<dyn Block>>, store: &Store) -> Self {
        Self::with_settings(theme, id, blocks, store, registry::SETTINGS)
    }

    /// `settings`: blokların geldiği kayıt (testte test kaydı).
    pub fn with_settings(theme: &Theme, id: PageId, blocks: Vec<Box<dyn Block>>, store: &Store, settings: &'static [SettingDesc]) -> Self {
        let ui = PageUi::default();
        let now = Instant::now();
        let find = |id: &str| settings.iter().find(|d| d.id == id);
        let slots = blocks
            .into_iter()
            .map(|mut block| {
                let ui_if = block.id().and_then(find).and_then(|d| d.ui_if);
                block.sync_ui(&ui, now);
                if let Some((name, _)) = block.disclosure() {
                    block.set_disclosed(ui.is_open(name), now);
                }
                let mut slot = Slot { block, ui_if, presence: Animated::new(0.0, theme.motion.spatial_fast), highlight: Animated::new(0.0, theme.motion.effects), highlight_until: None, local: Rect::ZERO };
                slot.presence.jump_to(if slot.shown(store, &ui) { 1.0 } else { 0.0 });
                slot
            })
            .collect();
        Self { id, slots, headers: Vec::new(), cards: Vec::new(), scroll: ScrollView::new(theme), viewport: Rect::ZERO, content_h: 0.0, focus: None, hover_inside: false, ui, settings }
    }

    /// Sayfa durumu.
    pub fn ui(&self) -> &PageUi {
        &self.ui
    }

    /// Sayfa durumunu değiştir (ör. Bar önizlemesinde widget seçimi); görünürlükler ve açılır
    /// satırlar yeni duruma akar.
    pub fn update_ui(&mut self, env: &mut Env, f: impl FnOnce(&mut PageUi)) {
        f(&mut self.ui);
        for s in &mut self.slots {
            s.block.sync_ui(&self.ui, env.now);
            if let Some((name, open)) = s.block.disclosure() {
                let want = self.ui.is_open(name);
                if want != open {
                    s.block.set_disclosed(want, env.now);
                }
            }
        }
        self.sync(env);
    }

    /// Blokların açılır satır durumunu sayfa durumuna yazar (tıklama/tuştan sonra).
    fn capture_ui(&mut self) {
        for s in &self.slots {
            if let Some((name, open)) = s.block.disclosure() {
                self.ui.set_open(name, open);
            }
        }
    }

    /// Kaplamada açık parçası olan blok (açılır seçimin menüsü).
    fn overlay_owner(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.local.height > 0.0 && s.block.overlay_open())
    }

    /// Bu sayfada açık bir kaplama (menü) var mı: varken işaretçi olayları önce sayfaya gider.
    pub fn overlay_open(&self) -> bool {
        self.overlay_owner().is_some()
    }

    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    /// Yerel yerleşimi hesaplar (her karede ucuz; etiketler önbellekli).
    fn layout(&mut self, env: &mut Env) {
        let t = env.theme;
        let (x, width) = column(t, self.viewport.width);
        let mut y = t.space(2);
        let mut section: Option<String> = None;
        let mut header_i = 0;
        self.cards.clear();
        let card_pad = t.space(1);
        let mut card_start: Option<(f32, f32)> = None; // (y, alpha)
        for i in 0..self.slots.len() {
            let p = self.slots[i].presence.value(env.now).clamp(0.0, 1.0);
            if p <= 0.001 {
                self.slots[i].local = Rect::new(x, y, width, 0.0);
                continue;
            }
            let sec = self.slots[i].block.section().map(str::to_owned);
            if sec != section {
                if let Some((top, a)) = card_start.take() {
                    self.cards.push((Rect::new(x, top, width, y - top + card_pad), a));
                    y += card_pad;
                }
                if let Some(name) = &sec {
                    let first = y <= t.space(2);
                    y += if first { t.space(2) } else { t.space(6) };
                    if name.is_empty() {
                        // Başlıksız kart (ör. önizleme).
                        card_start = Some((y, p));
                        y += card_pad;
                        section = sec;
                        let h = self.slots[i].block.height(env, width) * p;
                        self.slots[i].local = Rect::new(x, y, width, h);
                        y += h;
                        continue;
                    }
                    if self.headers.len() <= header_i {
                        self.headers.push(Header { label: Label::new(name.clone(), text_style(t, TextRole::Label)), at: Point::ZERO, alpha: 1.0 });
                    }
                    let h = &mut self.headers[header_i];
                    // Bölüm kimliği kaynak Türkçe ad; başlık etkin dilde.
                    h.label.set_text(crate::registry::section_title(name));
                    h.at = Point::new(x + t.space(4), y);
                    h.alpha = p;
                    y += h.label.measure(env.text).height + t.space(2);
                    header_i += 1;
                    card_start = Some((y, p));
                    y += card_pad;
                }
                section = sec;
            }
            let h = self.slots[i].block.height(env, width) * p;
            self.slots[i].local = Rect::new(x, y, width, h);
            y += h;
        }
        if let Some((top, a)) = card_start.take() {
            self.cards.push((Rect::new(x, top, width, y - top + card_pad), a));
            y += card_pad;
        }
        self.headers.truncate(header_i);
        self.content_h = y + t.space(8);
        self.scroll.set_geometry(self.viewport, self.content_h, env.now);
    }

    fn place(&mut self, env: &mut Env) {
        self.layout(env);
        let origin = self.scroll.content_origin(env.now);
        for s in &mut self.slots {
            if s.local.height > 0.0 {
                s.block.place(env, s.local.translate(origin));
            }
        }
    }

    pub fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.pointer_inner(env, e);
        self.capture_ui();
        self.pull_ui(env) || r
    }

    /// Blokların istediği sayfa durumu değişikliği ([`Block::take_ui`], ör. önizlemede seçilen
    /// widget); değiştiyse tüm bloklara akar. `true` = durum değişti.
    fn pull_ui(&mut self, env: &mut Env) -> bool {
        let mut ui = self.ui.clone();
        let mut changed = false;
        for s in &mut self.slots {
            changed |= s.block.take_ui(&mut ui);
        }
        if changed && ui != self.ui {
            self.update_ui(env, |u| *u = ui);
            return true;
        }
        false
    }

    fn pointer_inner(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        // Açık menü (kaplama) olayın tek sahibi: dışına tıklama yalnızca menüyü kapatır.
        if let Some(i) = self.overlay_owner() {
            self.place(env);
            let _ = self.slots[i].block.pointer(env, e);
            return true;
        }
        if matches!(e.kind, PointerKind::Scroll { .. } | PointerKind::ScrollEnd)
            && !self.scroll.is_dragging_bar()
            && self.viewport.contains(e.position)
            && let Some(s) = self.slots.iter_mut().find(|s| s.local.height > 0.0 && s.block.wants_scroll(e.position))
        {
            return s.block.pointer(env, e);
        }
        let mut repaint = self.scroll.pointer(env.theme, e, env.now).repaint;
        if matches!(e.kind, PointerKind::Scroll { .. } | PointerKind::ScrollEnd) || self.scroll.is_dragging_bar() {
            // Kaydırma blokları değiştirmez (kaydırıcılar tekerlekle oynamaz); hover güncellenir.
            if matches!(e.kind, PointerKind::Scroll { .. }) {
                self.place(env);
                let hover = PointerEvent::new(e.position, PointerKind::Motion);
                for s in &mut self.slots {
                    if s.local.height > 0.0 {
                        let _ = s.block.pointer(env, &hover);
                    }
                }
            }
            return true;
        }
        // Dokunmada kaydırma görünümü basmayı bekletir: bloklar `forward`'ın verdiklerini alır
        // (kaydırırken hiçbir şey; kaydırma sonrası tık sızmaz).
        for ev in self.scroll.forward(e) {
            repaint |= self.pointer_content(env, &ev);
        }
        repaint || self.scroll.is_panning()
    }

    fn pointer_content(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        self.place(env);
        let inside = self.viewport.contains(e.position);
        if inside != self.hover_inside {
            self.hover_inside = inside;
        }
        if let PointerKind::Press(MouseButton::Left) = e.kind {
            let hit = self.slots.iter().position(|s| s.local.height > 0.0 && inside && s.block_rect_contains(self.scroll.content_origin(env.now), e.position));
            if hit != self.focus {
                self.set_focus(hit, false, env.now);
            }
        }
        for s in &mut self.slots {
            if s.local.height <= 0.0 {
                continue;
            }
            let ev = if inside { *e } else { e.with_kind(e.position, PointerKind::Leave) };
            repaint |= s.block.pointer(env, &ev);
        }
        repaint
    }

    pub fn set_focus(&mut self, focus: Option<usize>, visible: bool, now: Instant) {
        if let Some(old) = self.focus
            && let Some(s) = self.slots.get_mut(old)
        {
            s.block.set_focus(false, false, now);
        }
        self.focus = focus;
        if let Some(f) = focus
            && let Some(s) = self.slots.get_mut(f)
        {
            s.block.set_focus(true, visible, now);
        }
    }

    pub fn clear_focus(&mut self, now: Instant) {
        self.set_focus(None, false, now);
    }

    /// Tab / Shift+Tab: sonraki/önceki odaklanabilir görünür blok.
    pub fn move_focus(&mut self, env: &mut Env, forward: bool) -> bool {
        let n = self.slots.len();
        if n == 0 {
            return false;
        }
        // Önce odaktaki blok kendi içinde ilerlesin (açık açılır satırın satırları).
        if let Some(f) = self.focus
            && self.slots[f].block.focus_step(forward, env.now)
        {
            let r = self.slots[f].local;
            self.scroll.ensure_visible(r.y - env.theme.space(4), r.bottom() + env.theme.space(4), env.now);
            return true;
        }
        let mut i = self.focus;
        for _ in 0..n {
            let next = match (i, forward) {
                (None, true) => 0,
                (None, false) => n - 1,
                (Some(k), true) => (k + 1) % n,
                (Some(k), false) => (k + n - 1) % n,
            };
            i = Some(next);
            let s = &self.slots[next];
            if s.block.focusable() && s.shown(env.store, &self.ui) {
                self.set_focus(Some(next), true, env.now);
                let r = self.slots[next].local;
                self.scroll.ensure_visible(r.y - env.theme.space(4), r.bottom() + env.theme.space(4), env.now);
                return true;
            }
        }
        false
    }

    pub fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let Some(f) = self.focus else { return false };
        self.place(env);
        let used = self.slots[f].block.key(env, e);
        self.capture_ui();
        self.pull_ui(env);
        used
    }

    pub fn focused_text_input(&mut self) -> Option<&mut nuxut::ui::TextInput> {
        let f = self.focus?;
        self.slots.get_mut(f)?.block.text_input()
    }

    /// İmleç: kaydırma çubuğu, sonra imlecin altındaki blok.
    pub fn cursor(&self, p: Point) -> Option<CursorShape> {
        // Açık menü sayfanın dışına taşabilir; imleç önce onun.
        if let Some(i) = self.overlay_owner() {
            return self.slots[i].block.cursor(p);
        }
        if !self.viewport.contains(p) {
            return None;
        }
        self.scroll.cursor(p).or_else(|| self.slots.iter().filter(|s| s.local.height > 0.0).find_map(|s| s.block.cursor(p)))
    }

    /// Odaktaki blok tüm tuşları istiyor mu (kısayol yakalayıcı).
    pub fn captures_keys(&self) -> bool {
        self.focus.and_then(|f| self.slots.get(f)).is_some_and(|s| s.block.captures_keys())
    }

    pub fn has_focus(&self) -> bool {
        self.focus.is_some()
    }

    /// Store değişti: görünürlükler ve değerler.
    pub fn sync(&mut self, env: &mut Env) {
        let ui = &self.ui;
        for s in &mut self.slots {
            let v = if s.shown(env.store, ui) { 1.0 } else { 0.0 };
            if s.presence.target() != v {
                s.presence.set_target(v, env.now);
            }
            s.block.sync(env);
        }
    }

    pub fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        let mut any = false;
        let ui = &self.ui;
        for s in &mut self.slots {
            any |= s.block.service_event(env, event);
            // Görünürlüğü servis durumuna bağlı bloklar (ör. bağlantı satırı notu) da yerinde açılır.
            let v = if s.shown(env.store, ui) { 1.0 } else { 0.0 };
            if s.presence.target() != v {
                s.presence.set_target(v, env.now);
                any = true;
            }
        }
        any
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.scroll.set_theme(theme, now);
        for s in &mut self.slots {
            s.block.set_theme(theme, now);
            s.presence.set_motion(theme.motion.spatial_fast, now);
            s.highlight.set_motion(theme.motion.effects, now);
        }
        for h in &mut self.headers {
            h.label.set_style(text_style(theme, TextRole::Label));
        }
    }

    /// Arama sonucundan: bloğa kaydır ve kısa süre parlat.
    pub fn reveal(&mut self, env: &mut Env, id: &str) {
        // Satır sayfa durumuna bağlıysa (ör. seçili widget) önce o durum kurulur.
        let store = env.store;
        if let Some(c) = self.settings.iter().find(|d| d.id == id).and_then(|d| d.ui_if)
            && !(c.shown)(&self.ui, store)
        {
            self.update_ui(env, |ui| (c.reveal)(ui, store));
        }
        // Yerleşim + yerleştirme: açılır satırın genişliği bilinsin.
        self.place(env);
        let Some(i) = self.slots.iter().position(|s| s.block.contains_id(id)) else { return };
        // Hedef satır yayını beklemeden yerinde: kaydırma hedefi son yerleşime göre.
        if self.slots[i].presence.target() > 0.5 {
            self.slots[i].presence.jump_to(1.0);
        }
        let (offset, self_hl) = self.slots[i].block.reveal(env, id).unwrap_or((0.0, false));
        self.capture_ui();
        // Açılır satır anında açıldı: hedef son yerleşime göre.
        self.layout(env);
        let r = self.slots[i].local;
        let top = (r.y + offset - env.theme.space(10)).max(0.0);
        self.scroll.scroll_to(top, env.now);
        if !self_hl {
            self.slots[i].highlight.jump_to(1.0);
            self.slots[i].highlight_until = Some(env.now + env.theme.timing.flash_hold);
        }
    }

    pub fn demo_expand(&mut self, id: &str, now: Instant) {
        if let Some(s) = self.slots.iter_mut().find(|s| s.block.contains_id(id)) {
            s.block.demo_expand(now);
        }
        self.capture_ui();
    }

    /// Görünür bloklardan maskotlu durum gösteren ilki (en çok bir maskot kuralı).
    pub fn mascot(&self, store: &Store) -> Option<crate::pages::fiyu::Spot> {
        self.slots.iter().filter(|s| s.shown(store, &self.ui)).find_map(|s| s.block.mascot())
    }

    pub fn scroll_to_top(&mut self, now: Instant) {
        self.scroll.scroll_to(0.0, now);
    }

    pub fn draw(&mut self, ui: &mut Ui, store: &Store) {
        let now = ui.now;
        let t = ui.theme;
        {
            let mut sink: Vec<Action> = Vec::new();
            let text: &mut TextSystem = ui.painter.text_system();
            let mut env = Env { theme: t, store, text, now, actions: &mut sink };
            self.place(&mut env);
        }
        let mut animating = false;
        for s in &mut self.slots {
            if let Some(until) = s.highlight_until {
                if now >= until {
                    s.highlight_until = None;
                    s.highlight.set_target(0.0, now);
                } else {
                    ui.wake_at(until);
                }
            }
            animating |= !s.presence.is_settled(now) || !s.highlight.is_settled(now);
        }
        let cards = std::mem::take(&mut self.cards);
        let headers = &mut self.headers;
        let slots = &mut self.slots;
        let viewport = self.viewport;
        self.scroll.draw(ui, |ui, origin, _span| {
            let c = &t.colors;
            for (r, a) in &cards {
                let r = r.translate(origin);
                if r.bottom() < viewport.y || r.y > viewport.bottom() {
                    continue;
                }
                ui.painter.rect(r, &RectStyle::filled(ui.fade(c.surface_container.multiply_alpha(*a)), t.shape.medium()));
            }
            for h in headers.iter_mut() {
                let at = Point::new(h.at.x + origin.x, h.at.y + origin.y);
                // Bölüm başlığı ikincil metin (§5.2): vurgu rengi yalnızca seçili/etkin öğeye kalır.
                h.label.draw(ui.painter, at, ui.fade(c.on_surface_variant.multiply_alpha(h.alpha)));
            }
            // Satır ayırıcısı yok (§5.1): kart içindeki satırlar boşlukla ayrılır.
            for s in slots.iter_mut() {
                if s.local.height <= 0.5 {
                    continue;
                }
                let r = s.local.translate(origin);
                if r.bottom() < viewport.y || r.y > viewport.bottom() {
                    continue;
                }
                let p = s.presence.value(now).clamp(0.0, 1.0);
                let hl = s.highlight.value(now).clamp(0.0, 1.0);
                if hl > 0.01 {
                    let flash = state_layer_under(t, c.primary, FLASH_TONE * hl, &[c.surface_container], &[c.on_surface, c.on_surface_variant]);
                    ui.painter.rect(r.inset(nuxut::Insets::uniform(t.space(1))), &RectStyle::filled(ui.fade(flash), t.shape.small()));
                }
                ui.painter.push_clip(r);
                ui.with_alpha(p, |ui| s.block.draw(ui));
                ui.painter.pop_clip();
            }
        });
        self.cards = cards;
        if animating {
            ui.request_frame();
        }
    }
}

impl Slot {
    fn block_rect_contains(&self, origin: Point, p: Point) -> bool {
        self.local.translate(origin).contains(p)
    }
}

// Ekran okuyucu (framework ADR-0011).
impl Page {
    /// Görünür bölüm başlıkları (`Role::Heading`) ve bloklar, ekrandaki sırayla; kaydırma
    /// görünümü dışındaki bloklar ağaca girmez (odaktaki hariç).
    pub fn access(&self, tree: &mut nuxut::access::AccessTree, now: Instant) {
        let origin = self.scroll.content_origin(now);
        let vp = self.viewport;
        let on_screen = |r: Rect| {
            let r = r.translate(origin);
            r.bottom() > vp.y && r.y < vp.bottom()
        };
        enum Item<'a> {
            Head(&'a str),
            Slot(&'a dyn Block),
        }
        let mut items: Vec<(f32, Item)> = Vec::new();
        for h in &self.headers {
            if h.alpha > 0.0 && on_screen(Rect::new(h.at.x, h.at.y, 1.0, 1.0)) {
                items.push((h.at.y, Item::Head(h.label.text())));
            }
        }
        for (i, s) in self.slots.iter().enumerate() {
            let shown = s.presence.value(now) > 0.01 && s.local.height > 0.0;
            if shown && (on_screen(s.local) || self.focus == Some(i)) {
                items.push((s.local.y, Item::Slot(s.block.as_ref())));
            }
        }
        items.sort_by(|a, b| a.0.total_cmp(&b.0));
        for (y, item) in items {
            match item {
                Item::Head(text) => {
                    tree.push(("heading", text, y.to_bits()), nuxut::access::Role::Heading).label(text);
                }
                Item::Slot(block) => block.access(tree),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sütun yatayda ortalı ve her sayfada aynı azami genişlikte (1040); dar viewport'ta iki
    /// yanda `space(6)`.
    #[test]
    fn column_is_centered_with_one_max_width() {
        let t = Theme::default();
        let m = t.space(6);
        assert_eq!(t.space(260), 1040.0);
        assert_eq!(column(&t, 1600.0), (280.0, 1040.0));
        assert_eq!(column(&t, 3000.0), (980.0, 1040.0));
        assert_eq!(column(&t, 600.0), (m, 600.0 - m * 2.0));
        // Tam sınırda boşluk iki yanda eşit.
        assert_eq!(column(&t, 1040.0 + m * 2.0), (m, 1040.0));
    }

    /// Bütün sayfaların blokları aynı ortalı sütunda: sayfa geçişinde kenar oynamaz.
    #[test]
    fn blocks_share_one_centered_column() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        // Gizli sayfalar (veri yokken Güç) çizilmez.
        for id in crate::registry::nav_pages() {
            let blocks = crate::pages::build(&theme, id, &store);
            let mut page = Page::new(&theme, id, blocks, &store);
            page.set_viewport(Rect::new(0.0, 0.0, 1600.0, 900.0));
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            page.layout(&mut env);
            let shown: Vec<_> = page.slots.iter().filter(|s| s.local.height > 0.0).collect();
            assert!(!shown.is_empty(), "{id:?}");
            assert!(shown.iter().all(|s| s.local.x == 280.0 && s.local.width == 1040.0), "{id:?}");
        }
    }

    /// Düz satırın arama vurgusu açılır satırdakiyle aynı: `flash_hold` boyunca tam opak
    /// (kare değil zamanlayıcı), sonra `motion.effects` ile söner.
    #[test]
    fn search_highlight_holds_then_fades() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let blocks = crate::pages::build(&theme, PageId::Appearance, &store);
        let mut page = Page::new(&theme, PageId::Appearance, blocks, &store);
        page.set_viewport(Rect::new(0.0, 0.0, 900.0, 600.0));
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        page.reveal(&mut env, "colors.primary");
        let s = page.slots.iter_mut().find(|s| s.block.contains_id("colors.primary")).unwrap();
        assert_eq!(s.highlight_until, Some(now + theme.timing.flash_hold));
        let late = now + theme.timing.flash_hold - std::time::Duration::from_millis(1);
        assert_eq!(s.highlight.value(late), 1.0, "bekleyişte sönmez");
        assert!(s.highlight.is_settled(late), "bekleyişte kare istenmez");
    }
}
