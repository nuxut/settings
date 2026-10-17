//! "Yeni" hapı: satır başlığının yanında (ve widget ekleme menüsünde düğmenin köşesinde)
//! framework [`Pill`]'i. Bu dosya yalnızca ince bir sarmalayıcıdır — çizim, renk (seçili
//! tonal zeminde `set_on_tonal`) ve yerleşim framework'te; burada tek iş, sayfalar dil
//! değişiminde yeniden kurulmadığı için metni her ölçümde katalogdan tazelemek. Arama
//! sonuçlarında aynı hap `ListTrailing::Pill` ile çizilir. Görülme modeli:
//! [`crate::services::seen`].

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::{Pill, Ui};
use nuxut::{Point, Rect, Size, TextSystem, Theme};

#[derive(Debug)]
pub struct NewBadge {
    pill: Pill,
}

impl NewBadge {
    pub fn new(theme: &Theme) -> Self {
        Self { pill: Pill::new(theme, t!("settings-badge-new")) }
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.pill.set_theme(theme, now);
    }

    /// Dil değişmiş olabilir; metin aynıysa işlem yok.
    fn refresh(&mut self) {
        self.pill.set_text(&t!("settings-badge-new"));
    }

    /// Hap boyu (framework [`Pill::size`]).
    pub fn size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        self.refresh();
        self.pill.size(text, theme)
    }

    /// Hapı sol kenarı `x`, dikey ortası `cy` olacak biçimde çizer ([`Pill::place_inline`]);
    /// çizilen dikdörtgen döner.
    pub fn draw(&mut self, ui: &mut Ui, x: f32, cy: f32) -> Rect {
        self.refresh();
        let r = self.pill.place_inline(ui.painter.text_system(), ui.theme, x, cy);
        self.pill.draw(ui);
        r
    }

    /// Sağ üst köşesi `corner`'a yaslı hap ([`Pill::place_corner`]; düğmenin içine biraz biner).
    pub fn draw_at_corner(&mut self, ui: &mut Ui, corner: Point) -> Rect {
        self.refresh();
        let r = self.pill.place_corner(ui.painter.text_system(), ui.theme, corner);
        self.pill.draw(ui);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hap framework ölçüsünde: rozetsiz kopya çizim yok, boy `Pill` ile birebir.
    #[test]
    fn badge_matches_framework_pill_and_placement() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut b = NewBadge::new(&theme);
        let s = b.size(&mut text, &theme);
        let mut p = Pill::new(&theme, t!("settings-badge-new"));
        assert_eq!(s, p.size(&mut text, &theme));
        assert!(s.width >= s.height && s.height > 0.0);
        let inline = b.pill.place_inline(&mut text, &theme, 40.3, 25.0);
        assert_eq!(inline.x, 40.0);
        assert!((inline.center().y - 25.0).abs() <= 0.5);
        let corner = b.pill.place_corner(&mut text, &theme, Point::new(120.0, 10.0));
        assert_eq!(corner.right(), 120.0 + theme.space(1), "köşeye biner");
    }

    /// Satır içi çizim: önce tonal zemin (dönen dikdörtgende), sonra metin; boşta kare yok.
    #[test]
    fn draw_is_framework_pill_frame_then_text() {
        crate::i18n::use_turkish_for_tests();
        use nuxut::render::{DrawKind, Recorder};
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut b = NewBadge::new(&theme);
        let mut rec = Recorder::new();
        let mut drawn = Rect::ZERO;
        rec.record(&mut text, (300, 80), 1.0, |painter| {
            let mut ui = Ui::new(painter, &theme, Instant::now());
            drawn = b.draw(&mut ui, 12.0, 30.0);
            assert!(!ui.wants_frame());
        });
        let draws = rec.draws();
        assert_eq!(draws[0].kind, DrawKind::Quad);
        assert_eq!(draws[0].rect, drawn);
        assert!(draws[1..].iter().all(|d| d.kind == DrawKind::Glyphs) && draws.len() > 1);
    }
}
