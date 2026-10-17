//! Klavye ve Fare sayfalarının üstündeki sakin not (kartsız):
//!
//! - `compositor.sync_input` kapalı: girdi ayarları masaüstüne uygulanmıyor; satırlar gizli,
//!   "Aç" düğmesi anahtarı varsayılanına (açık) döndürür.
//! - Kabuk çalışmıyor / girdi durumunu bildirmiyor / Hyprland'e ulaşamıyor
//!   ([`input::status_note`]): değerler şema varsayılanıyla gösteriliyor.
//!
//! Not ayrıca `[input]` değişince durumu yeniden sordurur (shell uygulasın diye kısa gecikmeyle):
//! etkin değerler ve geçersiz kılma uyarıları güncel kalır.

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Notice, NoticeKind, Ui};
use nuxut::{Point, Rect, Size, TextSystem, Theme};

use super::notice::{notice_height, place_notice};

use crate::block::{Block, Env};
use crate::registry::{self, SettingDesc};
use crate::rows::row_padding;
use crate::services::input;
use crate::store::Store;

/// Shell'in `shell.toml` değişimini uygulaması için beklenen süre.
const APPLY_SETTLE: Duration = Duration::from_millis(700);

/// Girdi eşitlemesi kapalıyken not.
pub fn sync_off_note() -> String {
    t!("settings-input-sync-off")
}

/// Gösterilecek not ve "Aç" düğmesi gerekip gerekmediği.
pub fn note_for(sync_input: bool, state: &input::InputState) -> Option<(String, bool)> {
    if !sync_input {
        return Some((sync_off_note(), true));
    }
    input::status_note(state).map(|n| (n, false))
}

/// `[input]` anahtarlarının ve eşitleme anahtarının değerleri (değişim izi).
fn input_signature(store: &Store) -> String {
    registry::SETTINGS
        .iter()
        .filter(|d| d.is_live() || d.id == "compositor.sync_input")
        .map(|d| store.get(d.file, &d.key()).map(|v| v.to_string()).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("|")
}

#[derive(Debug)]
pub struct InputNote {
    desc: &'static SettingDesc,
    /// Sakin bilgi notu (gri "i"); "Aç" gerekiyorsa notun sağında.
    notice: Notice,
    action: Button,
    /// Son çizilen not: (metin, düğme).
    current: Option<(String, bool)>,
    seen: Option<String>,
}

impl InputNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        input::ensure();
        let mut n = Self {
            desc,
            notice: Notice::new(theme, NoticeKind::Info, ""),
            action: Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-input-turn-on")),
            current: None,
            seen: Some(input_signature(store)),
        };
        n.update(store);
        n
    }

    fn update(&mut self, store: &Store) {
        self.current = note_for(registry::input_synced(store), &input::status());
        self.notice.set_text(self.current.as_ref().map_or("", |c| c.0.as_str()));
    }

    /// Notun düğmeden kalan payı (düğme + aradaki boşluk; düğme yoksa 0).
    fn action_inset(&mut self, text: &mut TextSystem, theme: &Theme) -> f32 {
        if self.has_action() { self.action_size(text, theme).width + theme.space(3) } else { 0.0 }
    }

    fn has_action(&self) -> bool {
        self.current.as_ref().is_some_and(|c| c.1)
    }

    fn action_size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        if self.has_action() { self.action.preferred_size(text, theme) } else { Size::ZERO }
    }

    /// "Aç": eşitleme anahtarını açık değerine döndürür (varsayılan açık → anahtar silinir).
    fn turn_on(env: &mut Env) {
        if let Some(d) = registry::by_id("compositor.sync_input") {
            env.set(d.label(), d.file, d.key(), d.toggle_value(true).and_then(|v| d.stored(v, env.store)));
        }
    }
}

impl Block for InputNote {
    fn section(&self) -> Option<&str> {
        // Kartsız: sayfanın en üstünde sakin bir satır.
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        note_for(registry::input_synced(store), &input::status()).is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let (_, py) = row_padding(env.theme);
        let bs = self.action_size(env.text, env.theme);
        let inset = self.action_inset(env.text, env.theme);
        notice_height(&mut self.notice, env, width, inset).max(bs.height + py * 2.0)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let inset = self.action_inset(env.text, t);
        place_notice(&mut self.notice, t, rect, inset);
        let bs = self.action_size(env.text, t);
        self.action.set_rect(Rect::new(rect.right() - px - bs.width, rect.center().y - bs.height * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        if !self.has_action() {
            return false;
        }
        let r = self.action.pointer(e, env.now);
        if r.event.is_some() {
            Self::turn_on(env);
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if !self.has_action() {
            return false;
        }
        let r = self.action.key(e, env.now);
        if r.event.is_some() {
            Self::turn_on(env);
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        self.has_action()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.action.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.has_action().then(|| self.action.cursor(p)).flatten()
    }

    fn sync(&mut self, env: &mut Env) {
        let sig = input_signature(env.store);
        if self.seen.as_ref() != Some(&sig) {
            // Kullanıcı bir girdi ayarını değiştirdi: shell uygulayınca etkin değerler ve
            // geçersiz kılmalar yeniden sorulur.
            input::refresh(APPLY_SETTLE);
            self.seen = Some(sig);
        }
        self.update(env.store);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::Input) {
            self.update(env.store);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
        self.action.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.notice.draw(ui);
        if self.has_action() {
            self.action.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::services::input::{InputState, InputStatus};

    #[test]
    fn sync_off_wins_and_offers_action() {
        crate::i18n::use_turkish_for_tests();
        let ready = InputState::Ready(Arc::new(InputStatus { available: true, ..Default::default() }));
        assert_eq!(note_for(false, &ready), Some((sync_off_note(), true)));
        assert_eq!(note_for(true, &ready), None, "her şey yolunda: not yok");
        let (text, action) = note_for(true, &InputState::NotRunning).unwrap();
        assert!(text.contains("çalışmıyor") && !action, "sakin not, düğme yok");
        assert!(note_for(true, &InputState::Pending).is_none());
    }
}
