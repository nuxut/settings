//! Palet üretimi (nuxut-palette kütüphanesi, arka plan thread'i): duvar kağıdından tohum
//! renk, dokuz şema varyantının önizleme şeritleri ve seçilen parametrelerle `palette.toml`
//! ve şablon dışa aktarımları. Tohum renk görsel başına önbelleklenir: 128 px'e küçültme ve
//! Celebi niceleme bir kez yapılır. Yerleşik Nuxut duvar kağıdında (`[meta] image =
//! "nuxut:<id>"`) görsel çözülmez: tohum, paletin kipindeki varyantın `seed`'idir
//! ([`PaletteSource::Seed`]; sözleşme `wallpaper-builtins-v1` §4).

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use nuxut::Color;
use nuxut::i18n::t;
use nuxut_palette::{Mode, Palette, Variant, export};

use super::{Event, emit};
use crate::store::PaletteRequest;

/// Önizleme şeridindeki roller (Material 3 adlarıyla).
pub const STRIP_ROLES: [&str; 6] = ["primary", "secondary", "tertiary", "primary_container", "surface_container", "surface"];

#[derive(Debug, Clone)]
pub enum Command {
    /// Görselin tohum rengini çıkar ve tüm varyantların şeritlerini üret.
    Preview { image: PathBuf, mode: String, contrast: f64 },
    /// Parametrelerle palette.toml'u ve dışa aktarımları yaz.
    Apply(PaletteRequest),
}

#[derive(Debug, Clone)]
pub enum Update {
    Previews { image: PathBuf, source: Color, strips: Vec<(&'static str, [Color; 6])> },
    Applied { scheme: String },
    Failed(String),
}

static WORKER: OnceLock<Mutex<mpsc::Sender<Command>>> = OnceLock::new();

pub fn send(cmd: Command) {
    let tx = WORKER.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        if let Err(e) = std::thread::Builder::new().name("palette".into()).spawn(move || worker(rx)) {
            tracing::warn!("palet thread'i açılamadı: {e}");
        }
        Mutex::new(tx)
    });
    if let Ok(tx) = tx.lock() {
        let _ = tx.send(cmd);
    }
}

fn update(u: Update) {
    emit(Event::Palette(u));
}

#[derive(Default)]
struct Cache {
    key: Option<(PathBuf, Option<SystemTime>)>,
    source: Option<Color>,
}

/// Paletin kaynağı: dosya görseli (çözülür) ya da doğrudan tohum rengi (yerleşik duvar kağıdı).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaletteSource<'a> {
    Image(&'a Path),
    Seed(Color),
}

/// `[meta] image` değeri ve paletin kipi (`[meta] mode`) → kaynak. Tohum bilerek paletin kendi
/// kipinin varyantından alınır (`[colors] mode`'dan değil): palet o kipte üretilir
/// (karar, 2026-10-13).
pub fn palette_source<'a>(image: &'a Path, mode: &str) -> PaletteSource<'a> {
    use super::wallpaper_source as ws;
    match ws::seed_for(image, ws::variant_named(mode)) {
        Some(seed) => PaletteSource::Seed(seed),
        None => PaletteSource::Image(image),
    }
}

impl Cache {
    fn source(&mut self, image: &Path, mode: &str) -> Result<Color, String> {
        match palette_source(image, mode) {
            PaletteSource::Seed(seed) => Ok(seed),
            PaletteSource::Image(image) => self.decoded(image),
        }
    }

    fn decoded(&mut self, image: &Path) -> Result<Color, String> {
        let mtime = std::fs::metadata(image).and_then(|m| m.modified()).ok();
        let key = (image.to_owned(), mtime);
        if self.key.as_ref() == Some(&key)
            && let Some(c) = self.source
        {
            return Ok(c);
        }
        // Görsel Ayarlar sürecinde çözülmez: 128² Contain kare paylaşılan yalıtılmış çözücüden
        // (services::wallpaper), renk süreç içinde güvenilir RGBA'dan. Log yalnızca hata türü.
        let frame = super::wallpaper::color_frame(image).map_err(|e| {
            tracing::warn!(kind = super::wallpaper::error_kind(&e), "palet: duvar kağıdı çözülemedi");
            super::wallpaper::error_message(&e)
        })?;
        let c = nuxut_palette::source_color_from_frame(&frame);
        self.key = Some(key);
        self.source = Some(c);
        Ok(c)
    }
}

pub fn parse_mode(s: &str) -> Mode {
    s.parse().unwrap_or_default()
}

pub fn parse_variant(s: &str) -> Variant {
    s.parse().unwrap_or_default()
}

/// Bir paletin önizleme şeridi (seçili kip).
pub fn strip(p: &Palette) -> [Color; 6] {
    STRIP_ROLES.map(|r| p.active().get(r).unwrap_or(Color::TRANSPARENT))
}

fn worker(rx: mpsc::Receiver<Command>) {
    let mut cache = Cache::default();
    while let Ok(mut cmd) = rx.recv() {
        // Birikmiş isteklerden yalnızca sonuncusu anlamlı (kaydırıcı sürüklenirken).
        while let Ok(next) = rx.try_recv() {
            cmd = next;
        }
        match cmd {
            Command::Preview { image, mode, contrast } => match cache.source(&image, &mode) {
                Ok(source) => {
                    let mode = parse_mode(&mode);
                    let strips = Variant::ALL.iter().map(|v| (v.name(), strip(&Palette::generate(source, *v, mode, contrast)))).collect();
                    update(Update::Previews { image, source, strips });
                }
                Err(msg) => update(Update::Failed(msg)),
            },
            Command::Apply(req) => match apply(&mut cache, &req) {
                Ok(()) => update(Update::Applied { scheme: req.scheme }),
                Err(msg) => update(Update::Failed(msg)),
            },
        }
    }
}

/// Şablon dışa aktarımlarına giden görsel yolu: yerleşik adres (`nuxut:<id>`) dosya yolu
/// beklenen yere gitmez, görsel boş verilir (2026-10-13; framework yerleşiği PNG
/// önbelleğine yazınca bağlanır — takip işi).
pub fn template_image(image: &Path) -> Option<&Path> {
    (!image.to_str().is_some_and(super::wallpaper_source::is_builtin_address)).then_some(image)
}

fn apply(cache: &mut Cache, req: &PaletteRequest) -> Result<(), String> {
    let image = req.image.as_deref().ok_or_else(|| t!("settings-wallpaper-scheme-need-image"))?;
    let source = cache.source(image, &req.mode)?;
    let palette = Palette::generate(source, parse_variant(&req.scheme), parse_mode(&req.mode), req.contrast);
    let path = nuxut::theme::palette_path().ok_or_else(|| t!("settings-wallpaper-error-state-dir"))?;
    export::generated_palette(&palette, Some(image)).save(&path).map_err(|e| {
        tracing::warn!("palette.toml yazılamadı: {e}");
        t!("settings-wallpaper-error-save")
    })?;
    if let Some(dir) = export::state_dir() {
        let out = dir.join("exports");
        for e in export::export_templates(&palette, template_image(image), &out, export::user_template_dir().as_deref()) {
            tracing::debug!(?e, "şablon");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::wallpaper_source::{Variant as WV, seed as builtin_seed};

    /// Yerleşik duvar kağıdı: palet görsel çözülmeden, paletin kipindeki varyantın tohumundan.
    #[test]
    fn builtin_palette_comes_from_seed() {
        let aurora = nuxut_wallpapers::find("aurora").unwrap();
        let s = |v| builtin_seed(aurora, v).unwrap();
        let fixed_light = nuxut_wallpapers::Selection { builtin: aurora, variant: Some(WV::Light) }.source();
        let image = Path::new("nuxut:aurora");
        assert_eq!(palette_source(image, "light"), PaletteSource::Seed(s(WV::Light)));
        assert_eq!(palette_source(image, "dark"), PaletteSource::Seed(s(WV::Dark)));
        assert_eq!(palette_source(image, "bozuk"), PaletteSource::Seed(s(WV::Dark)), "bilinmeyen kip → koyu");
        assert_eq!(palette_source(Path::new(&fixed_light), "dark"), PaletteSource::Seed(s(WV::Light)), "sabit varyant");
        assert_eq!(palette_source(Path::new("/yok/a.png"), "dark"), PaletteSource::Image(Path::new("/yok/a.png")));

        // Önbellek dosyaya dokunmaz (yol diskte yok); şeritler tohumdan üretilir.
        let mut cache = Cache::default();
        let seed = cache.source(image, "light").unwrap();
        assert_eq!(seed, s(WV::Light));
        assert!(cache.key.is_none(), "görsel önbelleği kullanılmaz");
        let p = Palette::generate(seed, Variant::default(), parse_mode("light"), 0.0);
        assert_eq!(strip(&p), strip(&Palette::generate(s(WV::Light), Variant::default(), Mode::Light, 0.0)));
        assert!(cache.source(Path::new("/yok/a.png"), "dark").is_err(), "dosya yolu yine çözülür");
        assert_eq!(template_image(image), None, "şablonlara adres gitmez");
        assert_eq!(template_image(Path::new("/srv/a.png")), Some(Path::new("/srv/a.png")));
    }
}
