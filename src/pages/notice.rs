//! Uyarı satırı: framework `Notice` (`NoticeKind::Warning`) + kısa metin (`desc.description()`).
//! Görünürlüğü kayıttaki `visible_if` belirler (ör. ses 100'ün üstüne çıkarılınca).

use std::time::Instant;

use nuxut::ui::input::PointerEvent;
use nuxut::ui::{Notice as UiNotice, NoticeKind, Ui};
use nuxut::{Rect, Theme};

use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::store::Store;

#[derive(Debug)]
pub struct Notice {
    desc: &'static SettingDesc,
    notice: UiNotice,
}

impl Notice {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        Self { desc, notice: UiNotice::new(theme, NoticeKind::Warning, desc.description()) }
    }
}

impl Block for Notice {
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
        notice_height(&mut self.notice, env, width, 0.0)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        place_notice(&mut self.notice, env.theme, rect, 0.0);
    }

    fn pointer(&mut self, _env: &mut Env, _event: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.notice.draw(ui);
    }
}

/// Kartsız not satırının yüksekliği: `width - right_inset` içinde yatay iç boşluk satırlarla
/// aynı, alt-üst satır boşluğu (yanında düğme varsa düğme yüksekliği çağıranda).
pub fn notice_height(notice: &mut UiNotice, env: &mut Env, width: f32, right_inset: f32) -> f32 {
    let (px, py) = row_padding(env.theme);
    notice.measure(env.text, env.theme, (width - px * 2.0 - right_inset).max(env.theme.space(20))).height + py * 2.0
}

/// Notu satır dikdörtgenine yerleştirir (dikeyde ortalanır); `right_inset` sağdaki düğme payı.
pub fn place_notice(notice: &mut UiNotice, theme: &Theme, rect: Rect, right_inset: f32) {
    let (px, _) = row_padding(theme);
    notice.set_rect(Rect::new(rect.x + px, rect.y, (rect.width - px * 2.0 - right_inset).max(theme.space(20)), rect.height));
}
