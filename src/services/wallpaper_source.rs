//! Duvar kağıdı kaynağı: config değeri (`shell.toml [wallpaper] path`, `[wallpaper.outputs.<ad>]
//! path`, `palette.toml [meta] image`) → dosya ya da yerleşik Nuxut duvar kağıdı (`nuxut:<id>`,
//! sözleşme `wallpaper-builtins-v1` + EK KARARLAR), ve yerleşiklerin arka planda çizimi.
//!
//! Yerleşiklerin dizini, adres ayrıştırması, tohumları, adları ve render'ı shell deposundaki
//! `nuxut-wallpapers` crate'indedir; burası Ayarlar'a özgü kurallardır:
//! - Boş yol ve palet görseli yoksa geçerli kaynak **varsayılan yerleşik**tir (`pair-flight`);
//!   dosyaya yazılmaz (sözleşme §5, karar 2026-10-13).
//! - Geçersiz/bilinmeyen `nuxut:` adresi → crate'in yedeği (varsayılan yerleşik) + uyarı.
//! - Palet tohumu: yerleşikte görsel çözülmez, `Builtin::seed(varyant)` kullanılır.
//! - Çizim: tek bir arka plan thread'i (`builtin-wallpapers`, `render_preview` bloklar); her
//!   tüketicinin [`BuiltinRender`]'ı yalnızca son isteğini bekler, eskileri thread'de atlanır.
//!   Bitince UI uyandırılır.
//! - Renk teması (`[colors] theme`, yalnız `source = "static"`): yerleşik, çizimden önce SVG
//!   renkleri temaya taşınarak çizilir (crate `render_preview_themed`; shell masaüstüyle aynı; Nuxut
//!   mavisinde yeniden boyama yok).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};

use nuxut::Color;
use nuxut::theme::{ColorThemeId, ThemeMode};
use nuxut_image::Image;
use nuxut_wallpapers::{Parsed, Selection};
use toml_edit::Value;

pub use nuxut_wallpapers::{BUILTINS, Builtin, Variant};

use crate::store::{FileId, KeyPath, Store};

/// Temanın çözdüğü kip → yerleşik varyantı.
pub fn variant_of(mode: ThemeMode) -> Variant {
    match mode {
        ThemeMode::Dark => Variant::Dark,
        ThemeMode::Light => Variant::Light,
    }
}

/// Paletin `[meta] mode` değeri → varyant (bilinmeyen → koyu, palet üretimiyle aynı yedek).
pub fn variant_named(mode: &str) -> Variant {
    Variant::from_name(mode.trim()).unwrap_or(Variant::Dark)
}

/// Yerleşiğin palet tohumu (crate `#rrggbb` tablosu).
pub fn seed(builtin: &Builtin, variant: Variant) -> Option<Color> {
    Color::from_hex(builtin.seed(variant)).ok()
}

/// Bir config değerinin gösterdiği duvar kağıdı.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSource {
    /// Kullanıcı dosyası (`~` açılmış).
    File(PathBuf),
    /// Yerleşik; `fixed` adresteki sabit varyant, yoksa etkin kipten.
    Builtin { builtin: &'static Builtin, fixed: Option<Variant> },
}

impl WallSource {
    /// Boş olmayan config değeri → kaynak. Geçersiz `nuxut:` adresi varsayılan yerleşiktir.
    pub fn parse(value: &str) -> Self {
        let sel = |s: Selection| Self::Builtin { builtin: s.builtin, fixed: s.variant };
        match nuxut_wallpapers::parse_source(value) {
            Parsed::NotBuiltin => Self::File(expand_home(value)),
            Parsed::Builtin(s) => sel(s),
            Parsed::Invalid { fallback, warning } => {
                tracing::warn!("{warning}");
                sel(fallback)
            }
        }
    }

    pub fn default_builtin() -> Self {
        Self::Builtin { builtin: nuxut_wallpapers::default_builtin(), fixed: None }
    }

    pub fn builtin(&self) -> Option<&'static Builtin> {
        match self {
            Self::Builtin { builtin, .. } => Some(builtin),
            Self::File(_) => None,
        }
    }

    pub fn file(&self) -> Option<&Path> {
        match self {
            Self::File(p) => Some(p),
            Self::Builtin { .. } => None,
        }
    }

    /// Yerleşikte gösterilecek varyant: sabit varyant, yoksa etkin kip.
    pub fn variant(&self, active: ThemeMode) -> Variant {
        match self {
            Self::Builtin { fixed: Some(v), .. } => *v,
            _ => variant_of(active),
        }
    }
}

/// `~` ve `~/…` → ev dizini (shell'in yorumuyla aynı).
pub fn expand_home(path: &str) -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    match (path.strip_prefix('~'), home) {
        (Some(""), Some(h)) => h,
        (Some(rest), Some(h)) if rest.starts_with('/') => h.join(rest.trim_start_matches('/')),
        _ => path.into(),
    }
}

fn non_empty(store: &Store, file: FileId, key: &str) -> Option<String> {
    store.get(file, &KeyPath::parse(key)).and_then(Value::as_str).filter(|p| !p.is_empty()).map(str::to_owned)
}

/// Geçerli (genel) duvar kağıdı: shell'in `wallpaper.path`'i, yoksa palet görseli, ikisi de
/// yoksa varsayılan yerleşik (yazılmadan). Kaydedilmemiş değerler dahil (bellekteki belge).
pub fn current(store: &Store) -> WallSource {
    non_empty(store, FileId::Shell, "wallpaper.path")
        .or_else(|| non_empty(store, FileId::Palette, "meta.image"))
        .map_or_else(WallSource::default_builtin, |v| WallSource::parse(&v))
}

/// Yol da palet görseli de boş: varsayılan yerleşik dosyaya yazılmadan gösteriliyor.
pub fn is_unset(store: &Store) -> bool {
    non_empty(store, FileId::Shell, "wallpaper.path").is_none() && non_empty(store, FileId::Palette, "meta.image").is_none()
}

/// Kart ızgarasında seçili görünen yerleşik (dosya seçiliyse `None`).
pub fn selected_builtin(store: &Store) -> Option<&'static Builtin> {
    current(store).builtin()
}

/// Yerleşik seçiminin yazımı (tek geri al kaydı): shell'in yolu ve palet görseli aynı adres
/// (varyantsız: kipi izler).
pub fn choice_writes(builtin: &Builtin) -> Vec<(FileId, KeyPath, Option<Value>)> {
    let v = Value::from(builtin.source());
    vec![(FileId::Shell, KeyPath::parse("wallpaper.path"), Some(v.clone())), (FileId::Palette, KeyPath::parse("meta.image"), Some(v))]
}

/// Değer bir yerleşik adresi mi (geçerliliğine bakılmaz).
pub fn is_builtin_address(value: &str) -> bool {
    value.starts_with(nuxut_wallpapers::SCHEME)
}

/// Palet görseli bir yerleşik adresiyse tohum rengi (`variant`: üretilen paletin kipi; adresteki
/// sabit varyant önce gelir). Dosya yolunda `None` (görsel çözülür).
pub fn seed_for(image: &Path, variant: Variant) -> Option<Color> {
    let s = image.to_str().filter(|s| is_builtin_address(s))?;
    let WallSource::Builtin { builtin, fixed } = WallSource::parse(s) else { return None };
    seed(builtin, fixed.unwrap_or(variant))
}

// ---- Arka planda çizim --------------------------------------------------------------------

struct Job {
    builtin: &'static Builtin,
    variant: Variant,
    theme: ColorThemeId,
    size: (u32, u32),
    generation: Arc<AtomicU64>,
    expected: u64,
    reply: Sender<(u64, Image)>,
}

static WORKER: OnceLock<Mutex<Sender<Job>>> = OnceLock::new();

fn render(job: &Job) -> Option<Image> {
    let r = nuxut_wallpapers::render_preview_themed(job.builtin.id, job.variant, job.theme, job.size.0, job.size.1)
        .map_err(|e| tracing::warn!("yerleşik duvar kağıdı çizilemedi: {e}"))
        .ok()?;
    // Çıktı opak (alfa 255): düz RGBA premultiplied ile aynı.
    Image::from_premultiplied(r.width, r.height, r.rgba).map_err(|e| tracing::warn!("yerleşik duvar kağıdı: {e}")).ok()
}

fn enqueue(job: Job) {
    let tx = WORKER.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Job>();
        if let Err(e) = std::thread::Builder::new().name("builtin-wallpapers".into()).spawn(move || {
            while let Ok(job) = rx.recv() {
                // Tüketici bu arada yeni istek yaptıysa eskisi çizilmez.
                if job.generation.load(Ordering::Relaxed) != job.expected {
                    continue;
                }
                if let Some(img) = render(&job)
                    && job.reply.send((job.expected, img)).is_ok()
                {
                    super::wake();
                }
            }
        }) {
            tracing::warn!("yerleşik duvar kağıdı thread'i açılamadı: {e}");
        }
        Mutex::new(tx)
    });
    if let Ok(tx) = tx.lock() {
        let _ = tx.send(job);
    }
}

/// Bir tüketicinin (kart, önizleme) çizim kuyruğu: son istenen (yerleşik, varyant, fiziksel boy).
#[derive(Debug)]
pub struct BuiltinRender {
    requested: Option<(&'static Builtin, Variant, ColorThemeId, (u32, u32))>,
    generation: Arc<AtomicU64>,
    tx: Sender<(u64, Image)>,
    rx: Receiver<(u64, Image)>,
}

impl Default for BuiltinRender {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { requested: None, generation: Arc::new(AtomicU64::new(0)), tx, rx }
    }
}

impl BuiltinRender {
    /// Son istek.
    pub fn requested(&self) -> Option<(&'static Builtin, Variant, (u32, u32))> {
        self.requested.map(|(b, v, _, s)| (b, v, s))
    }

    /// Son isteğin renk teması.
    pub fn requested_theme(&self) -> Option<ColorThemeId> {
        self.requested.map(|r| r.2)
    }

    /// Nuxut mavisiyle (yeniden boyamadan) çizim; bkz. [`request_themed`](Self::request_themed).
    pub fn request(&mut self, builtin: &'static Builtin, variant: Variant, size: (u32, u32)) -> bool {
        self.request_themed(builtin, variant, ColorThemeId::Blue, size)
    }

    /// İstek değiştiyse arka planda çizim başlatır (`true`). Aynı istek tekrarlanmaz.
    pub fn request_themed(&mut self, builtin: &'static Builtin, variant: Variant, theme: ColorThemeId, size: (u32, u32)) -> bool {
        let size = (size.0.max(1), size.1.max(1));
        let key = (builtin, variant, theme, size);
        if self.requested == Some(key) {
            return false;
        }
        self.requested = Some(key);
        let expected = self.generation.fetch_add(1, Ordering::Relaxed) + 1;
        enqueue(Job { builtin, variant, theme, size, generation: self.generation.clone(), expected, reply: self.tx.clone() });
        true
    }

    /// Son isteğin çizimi geldiyse alır (eskiler atılır).
    pub fn poll(&mut self) -> Option<Image> {
        let current = self.generation.load(Ordering::Relaxed);
        let mut out = None;
        while let Ok((g, img)) = self.rx.try_recv() {
            if g == current {
                out = Some(img);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn store(shell: &str, palette: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("s.toml"), shell).unwrap();
        std::fs::write(dir.path().join("p.toml"), palette).unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    fn find(id: &str) -> &'static Builtin {
        nuxut_wallpapers::find(id).unwrap()
    }

    /// Crate'in sabit varyantlı adresi (sözdizimini crate belirler).
    fn fixed(id: &str, v: Variant) -> String {
        Selection { builtin: find(id), variant: Some(v) }.source()
    }

    #[test]
    fn addresses_and_paths_parse() {
        assert_eq!(WallSource::parse("nuxut:pair-flight"), WallSource::Builtin { builtin: find("pair-flight"), fixed: None });
        for v in Variant::ALL {
            assert_eq!(WallSource::parse(&fixed("aurora", v)), WallSource::Builtin { builtin: find("aurora"), fixed: Some(v) });
        }
        assert_eq!(WallSource::parse("nuxut:chase"), WallSource::Builtin { builtin: find("chase"), fixed: None }, "maskotlu");
        assert_eq!(WallSource::parse("nuxut:bilinmeyen"), WallSource::default_builtin(), "geçersiz adres → varsayılan");
        assert_eq!(WallSource::parse("/srv/a.png"), WallSource::File("/srv/a.png".into()));
        let home = PathBuf::from(std::env::var_os("HOME").expect("HOME"));
        assert_eq!(WallSource::parse("~/a.png"), WallSource::File(home.join("a.png")));
        assert_eq!(expand_home("~kullanici/a.png"), PathBuf::from("~kullanici/a.png"));
        assert_eq!(choice_writes(find("climb"))[0].2.as_ref().and_then(Value::as_str), Some("nuxut:climb"), "varyantsız yazılır");
    }

    #[test]
    fn mode_picks_variant_unless_fixed() {
        let s = WallSource::parse("nuxut:climb");
        assert_eq!(s.variant(ThemeMode::Light), Variant::Light);
        assert_eq!(s.variant(ThemeMode::Dark), Variant::Dark);
        assert_eq!(WallSource::parse(&fixed("climb", Variant::Light)).variant(ThemeMode::Dark), Variant::Light);
        let b = find("migration");
        assert_eq!(seed_for(Path::new("nuxut:migration"), Variant::Light), seed(b, Variant::Light));
        assert_ne!(seed(b, Variant::Light), seed(b, Variant::Dark), "kip başına tohum");
        assert_eq!(seed_for(Path::new(&fixed("migration", Variant::Dark)), Variant::Light), seed(b, Variant::Dark), "sabit varyant önce");
        assert_eq!(seed_for(Path::new("/srv/a.png"), Variant::Dark), None);
        assert!(BUILTINS.iter().all(|b| Variant::ALL.iter().all(|v| seed(b, *v).is_some())), "her tohum renk");
    }

    #[test]
    fn empty_path_selects_default_without_writing() {
        let (_d, s) = store("", "");
        assert_eq!(selected_builtin(&s).map(|b| b.id), Some("pair-flight"));
        assert!(is_unset(&s) && !s.is_dirty(), "yazılmaz");
        let (_d, s) = store("[wallpaper]\npath = \"\"\n", "");
        assert_eq!(selected_builtin(&s).map(|b| b.id), Some("pair-flight"), "boş yol");
        let (_d, s) = store("", "[meta]\nimage = \"nuxut:migration\"\n");
        assert_eq!(selected_builtin(&s).map(|b| b.id), Some("migration"), "palet görseli yedek");
        let (_d, s) = store("[wallpaper]\npath = \"nuxut:aurora\"\n", "[meta]\nimage = \"/a.png\"\n");
        assert_eq!(selected_builtin(&s).map(|b| b.id), Some("aurora"));
        let (_d, s) = store("[wallpaper]\npath = \"/srv/a.png\"\n", "");
        assert_eq!(selected_builtin(&s), None, "dosya seçiliyken kart seçili değil");
    }

    #[test]
    fn render_queue_delivers_only_latest() {
        let mut r = BuiltinRender::default();
        let a = find("aurora");
        assert!(r.request(a, Variant::Dark, (8, 6)));
        assert!(!r.request(a, Variant::Dark, (8, 6)), "aynı istek tekrarlanmaz");
        assert!(r.request(a, Variant::Light, (32, 18)));
        let until = Instant::now() + Duration::from_secs(10);
        let img = loop {
            if let Some(img) = r.poll() {
                break img;
            }
            assert!(Instant::now() < until, "çizim gelmedi");
            std::thread::sleep(Duration::from_millis(5));
        };
        assert_eq!(img.size(), (32, 18), "yalnızca son istek");
        assert!(img.pixels().chunks(4).all(|p| p[3] == 255), "opak");
    }

    /// Renk teması yerleşiği yeniden boyar; mavide crate'in çizimiyle aynı.
    #[test]
    fn themed_render_recolors_builtin() {
        let a = find("aurora");
        let draw = |theme| {
            let mut r = BuiltinRender::default();
            assert!(r.request_themed(a, Variant::Dark, theme, (32, 18)));
            assert_eq!(r.requested_theme(), Some(theme));
            let until = Instant::now() + Duration::from_secs(10);
            loop {
                if let Some(img) = r.poll() {
                    break img.pixels().to_vec();
                }
                assert!(Instant::now() < until, "çizim gelmedi");
                std::thread::sleep(Duration::from_millis(5));
            }
        };
        let blue = draw(ColorThemeId::Blue);
        assert_eq!(blue, nuxut_wallpapers::render_preview("aurora", Variant::Dark, 32, 18).unwrap().rgba, "mavi: yeniden boyama yok");
        assert_ne!(draw(ColorThemeId::Sage), blue, "adaçayı farklı renkte");
    }
}
