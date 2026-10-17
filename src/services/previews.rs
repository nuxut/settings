//! İkon/imleç teması kart önizlemeleri için çözme servisi (UI'dan bağımsız).
//!
//! Tek bir süreç geneli [`PreviewCache`] (framework varsayılanı 8 MiB LRU; çözülmüş piksel
//! baytı) ve tek bir düşük öncelikli [`Loader`] (2 thread, `nice` 19, ilk istekte başlar). Her
//! ızgara kendi [`Inbox`]'ını verir: sonuç önce önbelleğe, sonra o kutuya yazılır ve
//! [`super::wake`] ile UI uyanır; ızgara bir sonraki çiziminde kutuyu boşaltıp dokuya yükler.
//! Böylece Görünüm sayfası ve Hoş geldin'in Uygulamalar adımı aynı önbelleği paylaşır, ama
//! birbirinin sonucunu "çalmaz".
//!
//! Boşta iş yok: thread'ler kanalda bekler; istek yalnızca bir kart ilk kez göründüğünde
//! (`ThemePreview::take_load_request`) ya da ölçek değişince gelir.

use std::sync::{Arc, Mutex, OnceLock};

use nuxut_image::preview::{PreviewEntry, outcome};
use nuxut_image::{ImageError, LoadKeys, LoadRequest, Loader, LoaderOptions, PreviewCache, PreviewKey, derive_key};
use nuxut_image_sandbox::{Sandbox, SandboxConfig};

/// Bir ızgaranın teslim kutusu: (anahtar, sonuç); `None` = temada yok / çözülemedi.
pub type Inbox = Arc<Mutex<Vec<(PreviewKey, PreviewEntry)>>>;

pub fn inbox() -> Inbox {
    Arc::new(Mutex::new(Vec::new()))
}

/// Kutudaki teslimleri alır (kilit zehirlense de).
pub fn drain(inbox: &Inbox) -> Vec<(PreviewKey, PreviewEntry)> {
    std::mem::take(&mut *inbox.lock().unwrap_or_else(|e| e.into_inner()))
}

enum Backend {
    Loader(Mutex<Loader>),
    /// Testler: iş verilmez, istenen anahtarlar kaydedilir.
    #[cfg(test)]
    Record(Mutex<Vec<PreviewKey>>),
}

/// Önizleme önbelleği + çözücü.
pub struct Previews {
    cache: PreviewCache,
    backend: Backend,
}

impl std::fmt::Debug for Previews {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Previews").field("cache", &self.cache).finish_non_exhaustive()
    }
}

static SHARED: OnceLock<Arc<Previews>> = OnceLock::new();

/// Düşük öncelikli yükleyici. Kullanıcı dizinindeki SVG tema dosyaları yalıtılmış işçide
/// çözülür (framework ADR-0007; işçi tek kullanımlık, ilk güvenilmeyen SVG'de başlar). İşçi
/// ikilisi framework'ün `nuxut-image-worker`'ı; aynı framework sürümüyle derlenmiş olmalı (el
/// sıkışma). Bulunamazsa (`NUXUT_IMAGE_WORKER` → Ayarlar ikilisinin yanı → `../lib/nuxut` →
/// `~/.local/lib/nuxut` → `/usr/lib/nuxut`) yalıtım kurulmaz; o yuvalar yer tutucu kalır,
/// SVG süreç içinde asla çözülmez.
pub(crate) fn loader() -> Loader {
    let loader = Loader::with_options(LoaderOptions { threads: 2, low_priority: true });
    match nuxut_image_sandbox::locate::locate() {
        Some(worker) => loader.with_sandbox(Arc::new(Sandbox::new(SandboxConfig { worker: Some(worker), ..SandboxConfig::default() }))),
        None => {
            tracing::debug!("nuxut-image-worker bulunamadı; kullanıcı SVG temaları önizlenmez");
            loader
        }
    }
}

/// Önizleme işinin anahtarları.
///
/// İptal = ızgara + tema + rol: aynı ızgaradaki yuvanın yeni isteği (ör. ölçek değişimi) eskisini
/// iptal eder; başka ızgaranın (Hoş geldin) aynı temadaki isteğine dokunmaz — iptal edilen iş
/// teslim etmez, o ızgaranın yuvası beklemede kalırdı.
///
/// Kaynak = tema dizini (tema kimliği arama yolunda tek dizine çözülür): susturma, negatif kova
/// ve hız sınırı tema başına ve ızgaralar arasında ortak. Art arda bozuk dosya veren tema bütün
/// rolleriyle susturulur; başka temanın önizlemesi etkilenmez. İmleç ve ikon önizlemesi aynı
/// dizini okuyorsa aynı kovadadır.
fn load_keys(key: &PreviewKey, inbox: &Inbox) -> LoadKeys {
    let theme = match key {
        PreviewKey::Cursor { theme, .. } | PreviewKey::Icon { theme, .. } => theme,
    };
    LoadKeys {
        cancel: key.load_key() ^ (Arc::as_ptr(inbox) as usize as u64).rotate_left(32),
        source: derive_key(&[b"theme-dir", theme.as_bytes()]),
    }
}

impl Previews {
    /// Süreç geneli örnek (önbellek tüm ızgaralarca paylaşılır).
    pub fn shared() -> Arc<Self> {
        SHARED
            .get_or_init(|| {
                Arc::new(Self {
                    cache: PreviewCache::default(),
                    backend: Backend::Loader(Mutex::new(loader())),
                })
            })
            .clone()
    }

    /// Test örneği: çözücü yok, istekler kaydedilir.
    #[cfg(test)]
    pub fn recording(budget: usize) -> Arc<Self> {
        Arc::new(Self { cache: PreviewCache::new(budget), backend: Backend::Record(Mutex::new(Vec::new())) })
    }

    /// Kaydedilen istekleri alır (yalnızca test örneği).
    #[cfg(test)]
    pub fn take_requests(&self) -> Vec<PreviewKey> {
        match &self.backend {
            Backend::Record(r) => std::mem::take(&mut *r.lock().unwrap()),
            Backend::Loader(_) => Vec::new(),
        }
    }

    pub fn cache(&self) -> &PreviewCache {
        &self.cache
    }

    /// Önbellekte varsa hemen `inbox`'a yazar (`true`); yoksa düşük öncelikli çözme işi verir.
    pub fn request(self: &Arc<Self>, key: PreviewKey, request: LoadRequest, inbox: &Inbox) -> bool {
        if let Some(entry) = self.cache.get(&key) {
            inbox.lock().unwrap_or_else(|e| e.into_inner()).push((key, entry));
            return true;
        }
        match &self.backend {
            Backend::Loader(loader) => {
                let inbox = inbox.clone();
                let shared = self.clone();
                let mut loader = loader.lock().unwrap_or_else(|e| e.into_inner());
                let keys = load_keys(&key, &inbox);
                let _ = loader.load_with_keys(keys, request, move |r| {
                    if let Err(e) = &r
                        && !matches!(e, ImageError::Cancelled)
                    {
                        tracing::debug!("tema önizlemesi çözülemedi: {e}");
                    }
                    // İptal ya da geçici yalıtım hatası: "yok" diye önbelleğe yazılmaz.
                    let Some(entry) = outcome(r) else { return };
                    shared.cache.insert(key.clone(), entry.clone());
                    inbox.lock().unwrap_or_else(|e| e.into_inner()).push((key, entry));
                    super::wake();
                });
            }
            #[cfg(test)]
            Backend::Record(r) => r.lock().unwrap().push(key),
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nuxut_image::{CursorRole, IconRole};

    fn icon(theme: &str, role: IconRole, px: u32) -> PreviewKey {
        PreviewKey::Icon { theme: theme.into(), role, px }
    }

    #[test]
    fn cancel_is_per_grid_and_slot_source_is_per_theme() {
        let (a, b) = (inbox(), inbox());
        let [r0, r1] = [IconRole::ALL[0], IconRole::ALL[1]];
        let k = load_keys(&icon("Papirus", r0, 48), &a);
        // Aynı yuva başka ölçekte: eski iş iptal edilir.
        assert_eq!(k.cancel, load_keys(&icon("Papirus", r0, 72), &a).cancel);
        // Başka rol ya da başka ızgara: birbirini iptal etmez, susturma kovası ortak.
        for other in [load_keys(&icon("Papirus", r1, 48), &a), load_keys(&icon("Papirus", r0, 48), &b)] {
            assert_ne!(k.cancel, other.cancel);
            assert_eq!(k.source, other.source);
        }
        // Başka tema: ayrı kova (bozuk tema diğerini susturmaz).
        assert_ne!(k.source, load_keys(&icon("Adwaita", r0, 48), &a).source);
        // Aynı dizindeki imleç teması aynı kovada.
        let c = PreviewKey::Cursor { theme: "Papirus".into(), role: CursorRole::ALL[0], px: 24 };
        assert_eq!(k.source, load_keys(&c, &a).source);
        assert!(![k.cancel, k.source].iter().any(|&x| x == nuxut_image::NO_KEY || x == nuxut_image::ANON_KEY));
    }
}
