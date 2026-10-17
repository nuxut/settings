//! Pencereler › Gelişmiş: "Masaüstü ayar dosyasına bağlan" açıkken bağlantı satırı kaldırılmışsa
//! (`msg hypr status` → `removed_by_user`, ör. "önceki hâline dön" sonrası) altında sakin bir
//! not. Not yokken satır gizlidir. Mantık `services::hypr_status`'ta (saf, testli).

use std::time::{Duration, Instant};

use nuxut::ui::input::PointerEvent;
use nuxut::ui::{Notice, NoticeKind, Ui};
use nuxut::{Rect, Theme};

use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::services::hypr_status;
use crate::store::Store;

/// Anahtar değişince shell'in satırı ekleyip/kaldırması için beklenen süre.
const APPLY_SETTLE: Duration = Duration::from_millis(700);

fn ensure_source(store: &Store) -> bool {
    crate::registry::by_id("compositor.ensure_source").is_some_and(|d| d.bool(store))
}

#[derive(Debug)]
pub struct SourceNote {
    desc: &'static SettingDesc,
    /// Framework'ün sıkı bilgi notu (Caption + küçük ikon, `on_surface_variant`).
    note: Notice,
    /// Son ölçümde notun yüksekliği (dolgusuz).
    note_h: f32,
    rect: Rect,
    /// Son görülen anahtar değeri (değişince durum yeniden sorulur).
    seen: Option<bool>,
}

impl SourceNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        Self { desc, note: Notice::new(theme, NoticeKind::Info, "").compact(theme), note_h: 0.0, rect: Rect::ZERO, seen: None }
    }

    fn note(store: &Store) -> Option<&'static str> {
        hypr_status::source_note(ensure_source(store), hypr_status::source().as_deref())
    }
}

impl Block for SourceNote {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store) && Self::note(store).is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.note.set_text(Self::note(env.store).unwrap_or_default());
        self.note_h = self.note.measure(env.text, t, (width - px * 2.0).max(t.space(24))).height;
        self.note_h + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let (px, _) = row_padding(env.theme);
        let w = (rect.width - px * 2.0).max(env.theme.space(24));
        self.note.set_rect(Rect::new(rect.x + px, rect.center().y - self.note_h * 0.5, w, self.note_h));
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        let on = ensure_source(env.store);
        if self.seen.is_some_and(|s| s != on) {
            hypr_status::refresh(APPLY_SETTLE);
        }
        self.seen = Some(on);
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        matches!(event, crate::services::Event::Hypr)
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.note.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.note.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{FileId, KeyPath};

    #[test]
    fn row_sits_under_switch_and_follows_it() {
        let d = crate::registry::by_id("compositor.source_note").expect("kayıtta");
        let sw = crate::registry::by_id("compositor.ensure_source").unwrap();
        assert_eq!((d.page, d.section), (sw.page, sw.section));
        let i = crate::registry::SETTINGS.iter().position(|s| s.id == d.id).unwrap();
        assert_eq!(crate::registry::SETTINGS[i - 1].id, sw.id, "notu anahtarın hemen altında");
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        // ensure_source varsayılan açık.
        assert!(ensure_source(&store) && d.is_visible(&store));
        store.set(FileId::Shell, &KeyPath::parse("compositor.ensure_source"), Some(toml_edit::Value::from(false)));
        assert!(!d.is_visible(&store), "anahtar kapalıyken satır yok");
    }

    /// Not framework'ün sıkı bilgi notuyla çizilir; metin hypr_status'tan aynen gelir.
    #[test]
    fn note_is_compact_info_notice() {
        let theme = Theme::default();
        let d = crate::registry::by_id("compositor.source_note").unwrap();
        let mut n = SourceNote::new(&theme, d);
        assert_eq!((n.note.kind(), n.note.size()), (NoticeKind::Info, nuxut::ui::NoticeSize::Compact));
        let msg = hypr_status::source_note(true, Some("removed_by_user")).unwrap();
        n.note.set_text(msg);
        assert_eq!(n.note.text(), msg);
        let mut text = nuxut::TextSystem::new();
        assert!(n.note.measure(&mut text, &theme, 600.0).height > 0.0);
    }
}
