//! Uygulama ikonları (Varsayılan uygulamalar satırı): `.desktop` `Icon=` adı → kullanıcının ikon
//! temasından doku verisi. Framework `nuxut_image::IconTheme` + `Loader` (önizleme servisiyle
//! aynı yalıtım: kullanıcı dizinindeki SVG yalıtılmış işçide, [`super::previews`]).
//!
//! - Tema: [`IconTheme::system`] (kullanıcının GTK ayarından; birleşik görünüm açıkken Nuxut
//!   oraya seçili temayı yazar). Tema değişince önbellek [`invalidate`] ile boşalır.
//! - Ad shell'den gelir (güvenilmez): yalnızca düz tema adı çözülür, mutlak yol çözülmez
//!   ([`super::default_apps::icon_name`]); güven `Untrusted`.
//! - UI thread'i dosya okumaz: [`get`] önbellekte yoksa iş verir, `None` döner; sonuç gelince
//!   [`super::wake`] → bir sonraki çizimde yeniden sorulur. Boşta iş yok.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nuxut_image::{Decoded, Image, IconTheme, LoadRequest, Loader};

/// Önbellekteki en çok ikon (ad × boyut). Dolunca tamamı boşalır (seyrek: tek sayfa, dokuz
/// rolün adayları × boyut).
const MAX_ENTRIES: usize = 512;

#[derive(Debug, Clone)]
enum Slot {
    Loading,
    Ready(Arc<Image>),
    Missing,
}

struct Icons {
    theme: Arc<IconTheme>,
    loader: Loader,
    cache: HashMap<(String, u32), Slot>,
}

static ICONS: Mutex<Option<Icons>> = Mutex::new(None);

/// Önbellek kuşağı: [`invalidate`] her çağrıda artar. Dokuyu tutan satırlar yüklü ikonun
/// kuşağını saklar; kuşak değişince yeniden ister (tema değişti).
static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn generation() -> u64 {
    GENERATION.load(std::sync::atomic::Ordering::Relaxed)
}

/// `name` ikonunun `px` fiziksel piksellik görüntüsü. Önbellekte yoksa arka planda ister ve
/// `None` döner; temada yoksa da `None` (yeniden istenmez).
pub fn get(name: &str, px: u32) -> Option<Arc<Image>> {
    if cfg!(test) || px == 0 {
        return None;
    }
    let mut guard = ICONS.lock().unwrap_or_else(|e| e.into_inner());
    let icons = guard.get_or_insert_with(|| Icons { theme: Arc::new(IconTheme::system()), loader: super::previews::loader(), cache: HashMap::new() });
    let key = (name.to_owned(), px);
    match icons.cache.get(&key) {
        Some(Slot::Ready(img)) => return Some(img.clone()),
        Some(Slot::Loading | Slot::Missing) => return None,
        None => {}
    }
    if icons.cache.len() >= MAX_ENTRIES {
        icons.cache.clear();
    }
    icons.cache.insert(key.clone(), Slot::Loading);
    let theme = icons.theme.clone();
    let request = LoadRequest::icon(theme.clone(), name, px, 1);
    icons.loader.load(request, move |r| {
        let slot = match r {
            Ok(Decoded::Still(img)) => Slot::Ready(Arc::new(img)),
            Ok(_) => Slot::Missing,
            Err(nuxut_image::ImageError::Cancelled) => return,
            Err(e) => {
                tracing::debug!("uygulama ikonu çözülemedi: {e}");
                Slot::Missing
            }
        };
        let mut guard = ICONS.lock().unwrap_or_else(|e| e.into_inner());
        // Bu arada tema değiştiyse (önbellek yeni temaya ait) sonuç atılır.
        if let Some(icons) = guard.as_mut().filter(|i| Arc::ptr_eq(&i.theme, &theme)) {
            icons.cache.insert(key, slot);
        }
        drop(guard);
        super::wake();
    });
    None
}

/// İkon teması değişti ya da uygulama kuruldu: önbellek ve tema yeniden okunur.
pub fn invalidate() {
    GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if let Some(icons) = ICONS.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        icons.theme = Arc::new(IconTheme::system());
        icons.cache.clear();
    }
}
