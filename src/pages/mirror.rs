//! Ayna satır (`docs/design/settings-ux.md` §3.4): bir ayar ikinci bir sayfada da çizilir
//! (`SettingDesc::mirror_on`). Satır aynı bloktur (aynı kayıt, tek anahtar, tek geri al kaydı);
//! bu sarmalayıcı yalnızca bölümünü değiştirir: ayna sayfada satır, kayıtta kendinden önceki
//! satırın kartına girer (ör. Sağ paneli aç / kapat kısayolu Paneller'de "Sağ panel" kartında,
//! Kısayollar'da "Bar ve paneller" kartında).

use std::time::Instant;

use nuxut::ui::Ui;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::{Point, Rect, Theme};

use crate::block::{Block, Env};
use crate::page_ui::PageUi;
use crate::store::Store;

#[derive(Debug)]
pub struct Mirror {
    inner: Box<dyn Block>,
    /// Ayna sayfadaki kartı (`None`: sayfada önünde satır yok → kendi bölümü).
    section: Option<&'static str>,
}

impl Mirror {
    pub fn new(inner: Box<dyn Block>, section: Option<&'static str>) -> Self {
        Self { inner, section }
    }
}

impl Block for Mirror {
    fn section(&self) -> Option<&str> {
        match self.section {
            Some(s) => Some(s),
            None => self.inner.section(),
        }
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
        self.inner.place(env, rect);
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
        self.inner.set_focus(focused, visible, now);
    }

    fn sync(&mut self, env: &mut Env) {
        self.inner.sync(env);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.inner.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.inner.draw(ui);
    }

    fn text_input(&mut self) -> Option<&mut nuxut::ui::TextInput> {
        self.inner.text_input()
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.inner.cursor(p)
    }

    fn demo_expand(&mut self, now: Instant) {
        self.inner.demo_expand(now);
    }

    fn contains_id(&self, id: &str) -> bool {
        self.inner.contains_id(id)
    }

    fn reveal(&mut self, env: &mut Env, id: &str) -> Option<(f32, bool)> {
        self.inner.reveal(env, id)
    }

    fn disclosure(&self) -> Option<(&'static str, bool)> {
        self.inner.disclosure()
    }

    fn set_disclosed(&mut self, open: bool, now: Instant) {
        self.inner.set_disclosed(open, now);
    }

    fn sync_ui(&mut self, ui: &PageUi, now: Instant) {
        self.inner.sync_ui(ui, now);
    }

    fn take_ui(&mut self, ui: &mut PageUi) -> bool {
        self.inner.take_ui(ui)
    }

    fn focus_step(&mut self, forward: bool, now: Instant) -> bool {
        self.inner.focus_step(forward, now)
    }

    fn overlay_open(&self) -> bool {
        self.inner.overlay_open()
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        self.inner.service_event(env, event)
    }
}
