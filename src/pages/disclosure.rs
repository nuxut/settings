//! Açılır satır bloğu (aşamalı açıklama, `docs/design/settings-ux.md` §3): kartın sonunda
//! framework [`nuxut::ui::Disclosure`] başlığı (başlık, açıklama, kapalıyken özet "11 renk ·
//! 2 tanesi elle ayarlı", nokta rozeti, dönen chevron, `spatial_fast` yayıyla açılma) ve altında
//! kayıttaki satırlar. Bu blok yalnızca kayıt bağlamasını tutar: satır listesi ve yerleşimi,
//! özet/rozet hesabı, içerideki odak (Tab), arama (açma, satıra kaydırma, framework
//! `flash_content` ile parlatma). Açık/kapalı durumu sayfanın `PageUi`'ında (oturumda)
//! hatırlanır.

use std::time::Instant;

use nuxut::motion::Animated;
use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::{Disclosure as DisclosureHeader, DisclosureEvent, Ui};
use nuxut::{Point, Rect, Theme};

use crate::block::{Block, Env};
use crate::page_ui::PageUi;
use crate::registry::{Counted, SettingDesc, UiCond, detail_group};
use crate::store::Store;

/// İçindeki bir satır.
#[derive(Debug)]
struct Child {
    block: Box<dyn Block>,
    ui_if: Option<UiCond>,
    /// Son sayfa durumu (`ui_if` config'le birlikte değerlendirilir).
    ui: PageUi,
    presence: Animated<f32>,
    /// İçeriğin üstüne göre yerleşim (son yerleşimde; y içerik koordinatında).
    local: Rect,
}

impl Child {
    fn shown(&self, store: &Store) -> bool {
        self.ui_if.is_none_or(|u| (u.shown)(&self.ui, store)) && self.block.visible(store)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    Header,
    Child(usize),
}

#[derive(Debug)]
pub struct Disclosure {
    name: &'static str,
    section: &'static str,
    noun: Counted,
    /// Özet ve rozet için içindeki ayarlar.
    descs: Vec<&'static SettingDesc>,
    /// Başlık, chevron, açılma yayı, rozet, vurgu ve başlık odağı framework'te.
    header: DisclosureHeader,
    children: Vec<Child>,
    focus: Option<Focus>,
    /// Aramanın parlattığı satır (vurgu tam opaklıktayken izlenir).
    flash: Option<FlashRow>,
}

/// Arama vurgusu: `flash_content` dikdörtgeni içerik koordinatında sabittir; satırın yeri ya
/// da genişliği değişirse (sayfa geçişi, üstteki satırın belirmesi) bekleyiş içinde yenilenir.
#[derive(Debug, Clone, Copy)]
struct FlashRow {
    index: usize,
    /// Son bildirilen dikdörtgen (`None` = henüz yerleşmedi).
    rect: Option<Rect>,
    /// Yerleşim bu andan sonra değişirse izlenmez (vurgu sönmeye başladı).
    until: Option<Instant>,
}

impl Disclosure {
    /// `section`: içinde durduğu kart; `name`: `Tier::Detail` adı; `children`: satırlar ve
    /// kayıttaki tanımları (aynı sırayla).
    pub fn new(theme: &Theme, section: &'static str, name: &'static str, children: Vec<(Box<dyn Block>, &'static SettingDesc)>, store: &Store) -> Self {
        let group = detail_group(name);
        let subtitle = group.map(|g| g.description()).filter(|s| !s.is_empty());
        let noun = group.map_or(Counted::Settings, |g| g.noun);
        let descs: Vec<&'static SettingDesc> = children.iter().map(|(_, d)| *d).collect();
        let children = children
            .into_iter()
            .map(|(block, desc)| {
                let v = if block.visible(store) { 1.0 } else { 0.0 };
                Child { block, ui_if: desc.ui_if, ui: PageUi::default(), presence: Animated::new(v, theme.motion.spatial_fast), local: Rect::ZERO }
            })
            .collect();
        let mut header = DisclosureHeader::new(theme, group.map_or(name, |g| g.label()));
        header.set_description(theme, subtitle);
        let mut d = Self { name, section, noun, descs, header, children, focus: None, flash: None };
        // İlk durumda rozet yaysız görünür: yay geçmişte başlamış, şimdiden oturmuş sayılır.
        let now = Instant::now();
        d.update_summary(store, now.checked_sub(std::time::Duration::from_secs(10)).unwrap_or(now));
        d
    }

    pub fn is_open(&self) -> bool {
        self.header.is_open()
    }

    fn update_summary(&mut self, store: &Store, now: Instant) {
        let n = self.descs.len();
        let k = self.descs.iter().filter(|d| d.differs_from_default(store)).count();
        self.header.set_badge(k > 0, now);
        self.header.set_summary(&summary_text(n, k, self.noun));
    }

    /// Açar/kapatır (kullanıcı olayı ya da sayfa durumu); kapanan içerikte odak kalmaz.
    fn set_open(&mut self, open: bool, animate: bool, now: Instant) {
        self.header.set_open(open, animate, now);
        self.after_toggle(now);
    }

    fn after_toggle(&mut self, now: Instant) {
        if !self.header.is_open()
            && let Some(Focus::Child(i)) = self.focus
        {
            self.children[i].block.set_focus(false, false, now);
            self.focus = Some(Focus::Header);
        }
    }

    /// İçerik yüksekliği (tamamen açıkken; satırların belirme yayı dahil).
    fn body_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let store = env.store;
        let now = env.now;
        let mut h = 0.0;
        for c in &mut self.children {
            let p = c.presence.value(now).clamp(0.0, 1.0);
            if p <= 0.001 && !c.shown(store) {
                continue;
            }
            h += c.block.height(env, width) * p;
        }
        h
    }

    /// Framework sözleşmesi: önce içerik yüksekliği, sonra başlık ölçümü.
    fn measure(&mut self, env: &mut Env, width: f32) -> f32 {
        let body = self.body_height(env, width);
        self.header.set_content_height(body);
        self.header.header_height(env.text, env.theme, width)
    }

    fn focusable_children(&self, store_visible: impl Fn(&Child) -> bool) -> Vec<usize> {
        self.children.iter().enumerate().filter(|(_, c)| c.block.focusable() && store_visible(c)).map(|(i, _)| i).collect()
    }

    fn set_child_focus(&mut self, focus: Option<Focus>, visible: bool, now: Instant) {
        if let Some(Focus::Child(i)) = self.focus
            && Some(Focus::Child(i)) != focus
        {
            self.children[i].block.set_focus(false, false, now);
        }
        self.focus = focus;
        if let Some(Focus::Child(i)) = focus {
            self.children[i].block.set_focus(true, visible, now);
        }
        self.header.set_focus_visible(visible && focus == Some(Focus::Header));
    }

    /// Satırın içerik koordinatındaki yeri (sayfa koordinatına `content_rect` ile).
    fn child_rect(&self, c: &Child) -> Rect {
        c.local.translate(Point::new(0.0, self.header.content_rect().y))
    }
}

impl Block for Disclosure {
    fn section(&self) -> Option<&str> {
        Some(self.section)
    }

    fn visible(&self, store: &Store) -> bool {
        // İçinde görünür satır kalmadıysa açılır satır da gizlenir.
        self.children.iter().any(|c| c.shown(store))
    }

    fn contains_id(&self, id: &str) -> bool {
        self.children.iter().any(|c| c.block.contains_id(id))
    }

    fn reveal(&mut self, env: &mut Env, id: &str) -> Option<(f32, bool)> {
        let i = self.children.iter().position(|c| c.block.contains_id(id))?;
        let now = env.now;
        // Kaydırma hedefi son hâle göre hesaplansın: anında aç.
        self.set_open(true, false, now);
        let width = self.header.rect().width;
        let head = self.measure(env, width);
        let store = env.store;
        let mut y = 0.0;
        for c in &mut self.children[..i] {
            if c.shown(store) {
                y += c.block.height(env, width);
            }
        }
        let (inner, self_hl) = self.children[i].block.reveal(env, id).unwrap_or((0.0, false));
        // Vurgu dikdörtgeni bir sonraki yerleşimde (gerçek genişlik ve satır yeri belli olunca).
        self.flash = (!self_hl).then_some(FlashRow { index: i, rect: None, until: None });
        Some((head + y + inner, true))
    }

    fn disclosure(&self) -> Option<(&'static str, bool)> {
        Some((self.name, self.header.is_open()))
    }

    fn set_disclosed(&mut self, open: bool, now: Instant) {
        if open != self.header.is_open() {
            self.set_open(open, true, now);
        }
    }

    fn sync_ui(&mut self, ui: &PageUi, now: Instant) {
        for c in &mut self.children {
            c.ui = ui.clone();
            c.block.sync_ui(ui, now);
        }
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.measure(env, width);
        self.header.height(env.now)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.measure(env, rect.width);
        self.header.set_rect(rect);
        let now = env.now;
        let store = env.store;
        let top = self.header.content_rect().y;
        // Sönmeye başlayan vurgu artık izlenmez.
        self.flash = self.flash.filter(|f| f.until.is_none_or(|u| now < u));
        let pad = env.theme.space(1);
        let mut y = 0.0;
        for (i, c) in self.children.iter_mut().enumerate() {
            let p = c.presence.value(now).clamp(0.0, 1.0);
            if p <= 0.001 && !c.shown(store) {
                c.local = Rect::new(rect.x, y, rect.width, 0.0);
                continue;
            }
            let full = c.block.height(env, rect.width);
            if let Some(f) = &mut self.flash
                && f.index == i
            {
                // İçerik koordinatında; satırın belirme yayı beklenmez (tam yükseklik).
                let r = Rect::new(pad, y + pad, (rect.width - pad * 2.0).max(0.0), (full - pad * 2.0).max(0.0));
                if f.rect != Some(r) {
                    self.header.flash_content(r, now);
                    f.rect = Some(r);
                    f.until = Some(now + env.theme.timing.flash_hold);
                }
            }
            let h = full * p;
            c.local = Rect::new(rect.x, y, rect.width, h);
            if h > 0.0 {
                c.block.place(env, Rect::new(rect.x, top + y, rect.width, h));
            }
            y += h;
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        // Açık menüsü olan satır olayın tek sahibi.
        if let Some(c) = self.children.iter_mut().find(|c| c.block.overlay_open()) {
            return c.block.pointer(env, e);
        }
        if let PointerKind::Press(MouseButton::Left) = e.kind {
            if self.header.header_rect().contains(e.position) {
                self.set_child_focus(Some(Focus::Header), false, now);
            } else if self.header.is_open() {
                let hit = self.children.iter().position(|c| c.local.height > 0.0 && self.child_rect(c).contains(e.position));
                if let Some(i) = hit
                    && self.children[i].block.focusable()
                {
                    self.set_child_focus(Some(Focus::Child(i)), false, now);
                }
            }
        }
        let r = self.header.pointer(e, now);
        let mut repaint = r.repaint;
        if let Some(DisclosureEvent::Toggled(_)) = r.event {
            self.after_toggle(now);
            return true;
        }
        if self.header.is_open() || self.header.is_animating(now) {
            let body = self.header.content_clip(now);
            for c in &mut self.children {
                if c.local.height <= 0.0 {
                    continue;
                }
                let ev = if body.contains(e.position) || !matches!(e.kind, PointerKind::Press(_)) { *e } else { PointerEvent::new(e.position, PointerKind::Leave) };
                repaint |= c.block.pointer(env, &ev);
            }
        }
        repaint
    }

    fn wants_scroll(&self, p: Point) -> bool {
        self.header.is_open() && self.children.iter().any(|c| c.local.height > 0.0 && c.block.wants_scroll(p))
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let now = env.now;
        match self.focus {
            Some(Focus::Child(i)) => self.children[i].block.key(env, e),
            _ => {
                if e.state == KeyState::Released {
                    return false;
                }
                if matches!(e.key, Key::Enter | Key::Space) {
                    self.header.set_focus_visible(true);
                }
                let r = self.header.key(e, now);
                if r.event.is_some() {
                    self.after_toggle(now);
                }
                r.event.is_some()
            }
        }
    }

    fn captures_keys(&self) -> bool {
        match self.focus {
            Some(Focus::Child(i)) => self.children[i].block.captures_keys(),
            _ => false,
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        if focused {
            let f = self.focus.unwrap_or(Focus::Header);
            self.set_child_focus(Some(f), visible, now);
        } else {
            self.set_child_focus(None, false, now);
        }
    }

    fn focus_step(&mut self, forward: bool, now: Instant) -> bool {
        if !self.header.is_open() {
            return false;
        }
        // Görünürlük için son yerleşim yeterli (kapalı/sönmüş satır yüksekliği 0).
        let order = self.focusable_children(|c| c.local.height > 0.5);
        let next = match (self.focus, forward) {
            (None | Some(Focus::Header), true) => order.first().map(|&i| Focus::Child(i)),
            (Some(Focus::Child(i)), true) => order.iter().find(|&&j| j > i).map(|&j| Focus::Child(j)),
            (Some(Focus::Child(i)), false) => Some(order.iter().rev().find(|&&j| j < i).map_or(Focus::Header, |&j| Focus::Child(j))),
            (None | Some(Focus::Header), false) => None,
        };
        match next {
            Some(f) => {
                self.set_child_focus(Some(f), true, now);
                true
            }
            None => false,
        }
    }

    fn overlay_open(&self) -> bool {
        self.children.iter().any(|c| c.block.overlay_open())
    }

    fn text_input(&mut self) -> Option<&mut nuxut::ui::TextInput> {
        match self.focus {
            Some(Focus::Child(i)) => self.children[i].block.text_input(),
            _ => None,
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if let Some(c) = self.header.cursor(p) {
            return Some(c);
        }
        if !self.header.is_open() {
            return None;
        }
        self.children.iter().filter(|c| c.local.height > 0.0).find_map(|c| c.block.cursor(p))
    }

    fn demo_expand(&mut self, now: Instant) {
        self.set_open(true, true, now);
    }

    fn sync(&mut self, env: &mut Env) {
        let store = env.store;
        let now = env.now;
        for c in &mut self.children {
            let v = if c.shown(store) { 1.0 } else { 0.0 };
            if c.presence.target() != v {
                c.presence.set_target(v, now);
            }
            c.block.sync(env);
        }
        self.update_summary(env.store, now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        let mut any = false;
        for c in &mut self.children {
            any |= c.block.service_event(env, event);
        }
        any
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.header.set_theme(theme, now);
        for c in &mut self.children {
            c.presence.set_motion(theme.motion.spatial_fast, now);
            c.block.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let now = ui.now;
        // Başlık, rozet, chevron, özet, odak halkası ve arama vurgusu.
        self.header.draw(ui);
        if self.header.reveal(now) <= 0.001 {
            return;
        }
        // İçerik: framework görünen bölüme kırpar ve açılma oranıyla soldurur.
        let mut animating = false;
        let children = &mut self.children;
        self.header.draw_content(ui, |ui, content| {
            // Satır ayırıcısı yok (§5.1): sayfadaki gibi satırlar boşlukla ayrılır.
            for ch in children.iter_mut() {
                if ch.local.height <= 0.5 {
                    continue;
                }
                let r = ch.local.translate(Point::new(0.0, content.y));
                let p = ch.presence.value(now).clamp(0.0, 1.0);
                animating |= !ch.presence.is_settled(now);
                ui.painter.push_clip(r);
                ui.with_alpha(p, |ui| ch.block.draw(ui));
                ui.painter.pop_clip();
            }
        });
        if animating {
            ui.request_frame();
        }
    }
}

/// Kapalı satırın özeti: "11 renk" ya da "11 renk · 2 tanesi elle ayarlı"; sayılan şeye göre
/// kendi tam cümle mesajı.
pub fn summary_text(total: usize, changed: usize, noun: Counted) -> String {
    match (noun, changed) {
        (Counted::Settings, 0) => t!("settings-disclosure-summary-settings", total = total),
        (Counted::Settings, _) => t!("settings-disclosure-summary-settings-changed", total = total, changed = changed),
        (Counted::Colors, 0) => t!("settings-disclosure-summary-colors", total = total),
        (Counted::Colors, _) => t!("settings-disclosure-summary-colors-changed", total = total, changed = changed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Arama vurgusu satırın gerçek yerine bağlanır: yerleşim değişirse (sayfa geçişinde
    /// genişlik) bekleyiş içinde yenilenir, söndükten sonra izlenmez.
    #[test]
    fn search_flash_follows_row_layout_until_hold_ends() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let children = ["colors.surface", "colors.outline"]
            .iter()
            .map(|id| {
                let d = crate::registry::by_id(id).unwrap();
                (super::super::block_for(&theme, d, &store).unwrap(), d)
            })
            .collect();
        let mut d = Disclosure::new(&theme, "Renkler", "Renkleri tek tek ayarla", children, &store);
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        let (y, own) = d.reveal(&mut env, "colors.outline").unwrap();
        assert!(own && y > 0.0);
        assert!(d.flash.is_some_and(|f| f.index == 1 && f.rect.is_none()), "dikdörtgen yerleşimde");
        let place = |d: &mut Disclosure, env: &mut Env, w: f32| {
            let h = d.height(env, w);
            d.place(env, Rect::new(0.0, 0.0, w, h));
            d.flash.and_then(|f| f.rect)
        };
        let narrow = place(&mut d, &mut env, 700.0).expect("vurgu yerleşti");
        assert!(narrow.y > 0.0, "ikinci satır: ilk satırın altında");
        let wide = place(&mut d, &mut env, 760.0).expect("vurgu izleniyor");
        assert_eq!(wide.width - narrow.width, 60.0, "genişlik değişince yenilenir");
        env.now = now + theme.timing.flash_hold;
        assert!(place(&mut d, &mut env, 700.0).is_none(), "sönmeye başlayınca izlenmez");
    }

    #[test]
    fn summary_mentions_changes_only_when_any() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(summary_text(11, 0, Counted::Colors), "11 renk");
        assert_eq!(summary_text(11, 2, Counted::Colors), "11 renk · 2 tanesi elle ayarlı");
        assert_eq!(summary_text(4, 3, Counted::Settings), "4 ayar · 3 tanesi elle ayarlı");
    }
}
