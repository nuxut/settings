//! Tema kartının önizleme şeridi (t103): framework `CursorPreview` / `IconThemePreview` +
//! [`crate::services::previews`] akışı.
//!
//! - Yalnızca **kurulu** temanın kartı önizleme taşır ([`preview_id`]); kurulu olmayan aile ve
//!   dosyadaki kurulu olmayan seçim tür/indirme ikonlu yer tutucuda kalır.
//! - Kart küçük: öğe [`ITEM_SPACE`] (24 mantıksal piksel; imleçte gerçek tipik boyut). Şerit
//!   kaç öğe sığıyorsa yalnızca onları ister (bileşenin tembel isteği); sıra en tanıdık
//!   şekillerle başlar — imleçte animasyonlu `wait` üçüncüdür, dar kartta da görünsün diye.
//! - Ölçek değişince ([`CardPreview::sync_scale`]) yuvalar `Loading`'e döner ve yeni fiziksel
//!   piksel boyutuyla yeniden istenir (anahtar `px` taşır: eski boyutun sonucu eşleşmez).
//! - Önce paylaşılan [`PreviewCache`](nuxut_image::PreviewCache) (8 MiB LRU), yoksa düşük
//!   öncelikli çözme; UI thread'i hiç dosya okumaz.

use std::sync::Arc;
use std::time::Instant;

use nuxut::render::{ImageData, Painter, TextureOptions};
use nuxut::ui::input::PointerEvent;
use nuxut::ui::{CursorPreview, IconThemePreview, PreviewFrame, PreviewSlot, Ui};
use nuxut::{Rect, Theme};
use nuxut_image::preview::{PreviewEntry, cursor_request, icon_request};
use nuxut_image::{CursorRole, CursorTheme, IconRole, IconTheme, LoadRequest, PreviewImage, PreviewKey};

use crate::services::look::ThemeKind;

/// Kart önizlemesinin öğe boyutu (`space` birimi).
pub const ITEM_SPACE: u32 = 6;

/// Kartın imleç sırası: framework `CursorRole::PREVIEW`'in aynısı, `wait` öne alınmış.
pub const CURSOR_ROLES: [CursorRole; 9] = [
    CursorRole::Default,
    CursorRole::Pointer,
    CursorRole::Wait,
    CursorRole::Text,
    CursorRole::Crosshair,
    CursorRole::Move,
    CursorRole::NotAllowed,
    CursorRole::ResizeEw,
    CursorRole::ResizeNwse,
];

/// Kartın ikon sırası (framework varsayılan seti).
pub const ICON_ROLES: [IconRole; 8] = IconRole::PREVIEW;

/// Animasyonlu imleç rolleri (tüm kareleriyle yüklenir; yalnızca üzerindeyken oynar).
fn animated(role: CursorRole) -> bool {
    matches!(role, CursorRole::Wait | CursorRole::Progress)
}

/// Kurulu bir temanın önizleme şeridi.
#[derive(Debug)]
pub enum CardPreview {
    Cursor { theme: Arc<CursorTheme>, view: CursorPreview },
    Icon { theme: Arc<IconTheme>, view: IconThemePreview },
}

impl CardPreview {
    pub fn new(theme: &Theme, kind: ThemeKind, id: &str) -> Self {
        let item = theme.space(ITEM_SPACE);
        match kind {
            ThemeKind::Cursors => {
                Self::Cursor { theme: Arc::new(CursorTheme::new(id)), view: CursorPreview::new(theme, CURSOR_ROLES.len()).with_item_size(item) }
            }
            ThemeKind::Icons => {
                Self::Icon { theme: Arc::new(IconTheme::new(id)), view: IconThemePreview::new(theme, ICON_ROLES.len()).with_item_size(item) }
            }
        }
    }

    /// Tema kimliği (dizin adı).
    pub fn id(&self) -> &str {
        match self {
            Self::Cursor { theme, .. } => theme.name(),
            Self::Icon { theme, .. } => theme.theme_name(),
        }
    }

    /// Önizleme şeridinin yüksekliği (öğe + kuyu iç boşluğu).
    pub fn height(theme: &Theme) -> f32 {
        CursorPreview::new(theme, 1).with_item_size(theme.space(ITEM_SPACE)).preferred_height(theme)
    }

    /// `slot` yuvasının `scale` ölçekteki önbellek anahtarı.
    pub fn key(&self, slot: usize, scale: f32) -> Option<PreviewKey> {
        Some(match self {
            Self::Cursor { theme, view } => PreviewKey::Cursor { theme: theme.name().to_owned(), role: *CURSOR_ROLES.get(slot)?, px: view.load_px(scale) },
            Self::Icon { theme, view } => {
                let (size, s) = icon_px(view.item_size(), scale);
                PreviewKey::Icon { theme: theme.theme_name().to_owned(), role: *ICON_ROLES.get(slot)?, px: size * s }
            }
        })
    }

    fn load_request(&self, slot: usize, scale: f32) -> Option<LoadRequest> {
        Some(match self {
            Self::Cursor { theme, view } => {
                let role = *CURSOR_ROLES.get(slot)?;
                cursor_request(theme.clone(), role, view.load_px(scale), animated(role))
            }
            Self::Icon { theme, view } => {
                let (size, s) = icon_px(view.item_size(), scale);
                icon_request(theme.clone(), *ICON_ROLES.get(slot)?, size, s)
            }
        })
    }

    /// Yeni görünür yuvaların işleri (çizimden sonra; her yuva bir kez).
    pub fn take_jobs(&mut self, scale: f32) -> Vec<(PreviewKey, LoadRequest)> {
        let range = match self {
            Self::Cursor { view, .. } => view.take_load_request(),
            Self::Icon { view, .. } => view.take_load_request(),
        };
        range.into_iter().flatten().filter_map(|slot| Some((self.key(slot, scale)?, self.load_request(slot, scale)?))).collect()
    }

    /// Ölçek değişti: tüm yuvalar `Loading`'e döner (eski dokular bırakılır), bir sonraki
    /// çizimde görünenler yeni boyutla istenir.
    pub fn reset(&mut self, theme: &Theme) {
        match self {
            Self::Cursor { view, .. } => view.reset(theme, CURSOR_ROLES.len()),
            Self::Icon { view, .. } => view.reset(theme, ICON_ROLES.len()),
        }
    }

    /// Teslim edilen sonucu bu ölçekte eşleşen yuvaya verir (`upload` dokuya yükler).
    /// Eşleşme yoksa (başka tema, eski ölçek) `false`.
    pub fn deliver(&mut self, key: &PreviewKey, entry: &PreviewEntry, scale: f32, now: Instant, upload: &mut dyn FnMut(&PreviewImage) -> Vec<PreviewFrame>) -> bool {
        let n = self.len();
        let Some(slot) = (0..n).find(|&i| self.key(i, scale).as_ref() == Some(key)) else { return false };
        let value = match entry {
            Some(img) => PreviewSlot::Ready(upload(img)),
            None => PreviewSlot::Missing,
        };
        match self {
            Self::Cursor { view, .. } => view.set_slot(slot, value, now),
            Self::Icon { view, .. } => view.set_slot(slot, value, now),
        }
        true
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Cursor { view, .. } => view.len(),
            Self::Icon { view, .. } => view.len(),
        }
    }

    #[cfg(test)]
    pub fn slot(&self, i: usize) -> Option<&PreviewSlot> {
        match self {
            Self::Cursor { view, .. } => view.slot(i),
            Self::Icon { view, .. } => view.slot(i),
        }
    }

    pub fn set_rect(&mut self, r: Rect) {
        match self {
            Self::Cursor { view, .. } => view.set_rect(r),
            Self::Icon { view, .. } => view.set_rect(r),
        }
    }

    /// İşaretçi (animasyonlu imleç yalnızca üzerindeyken oynar); `true` = yeniden çiz.
    pub fn pointer(&mut self, e: &PointerEvent, now: Instant) -> bool {
        match self {
            Self::Cursor { view, .. } => view.pointer(e, now).repaint,
            Self::Icon { view, .. } => view.pointer(e, now).repaint,
        }
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        match self {
            Self::Cursor { view, .. } => view.set_theme(theme, now),
            Self::Icon { view, .. } => view.set_theme(theme, now),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        match self {
            Self::Cursor { view, .. } => view.draw(ui),
            Self::Icon { view, .. } => view.draw(ui),
        }
    }
}

/// İkon isteğinin (mantıksal boyut, tamsayı ölçek) çifti: kesirli ölçekte bir üst tamsayı
/// (1,5 → 2; çizimde küçültülür, bulanık büyütme yok).
fn icon_px(item: f32, scale: f32) -> (u32, u32) {
    (item.round().max(1.0) as u32, scale.max(1.0).ceil() as u32)
}

/// Önizleme görseli → yüklenmiş kareler.
pub fn upload(painter: &mut Painter, img: &PreviewImage) -> Vec<PreviewFrame> {
    img.frames
        .iter()
        .map(|f| {
            let (width, height) = f.image.size();
            let data = ImageData { width, height, pixels: Arc::from(f.image.pixels()) };
            PreviewFrame { texture: painter.upload_texture(data, TextureOptions::default()), delay: f.delay }
        })
        .collect()
}
