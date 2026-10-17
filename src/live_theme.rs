//! Ayarlar penceresinin kendi teması: diskteki dosyayı beklemeden **bellekteki** belgeden
//! üretilir, böylece bir renge dokunur dokunmaz pencere de (önizleme dahil) o renge akar.
//! Diske yazım 150 ms sonra gider; shell ve diğer uygulamalar oradan alır.

use std::time::Instant;

use nuxut::theme::{AnimatedTheme, FrameworkConfig, GeneratedPalette, ThemeMode, palette_path};
use nuxut::Theme;
use toml_edit::Value;

use crate::store::{FileId, KeyPath, Store};

#[derive(Debug)]
pub struct LiveTheme {
    anim: AnimatedTheme,
    /// Üretilmiş palet (palette.toml: kayıtlı kip + kip başına bölümler); bozuksa son geçerli korunur.
    palette: Option<GeneratedPalette>,
    revision: u64,
    warnings: Vec<String>,
}

impl LiveTheme {
    pub fn new(store: &Store) -> Self {
        let palette = load_palette(None);
        let (theme, warnings) = build(store, palette.as_ref());
        Self { anim: AnimatedTheme::new(theme), palette, revision: store.revision(), warnings }
    }

    /// Hedef tema (renk dışı token'lar için).
    pub fn theme(&self) -> &Theme {
        self.anim.theme()
    }

    /// Çizimde kullanılacak tema (renkler geçişin o anki değeri).
    pub fn current(&self, now: Instant) -> Theme {
        self.anim.current(now)
    }

    pub fn is_animating(&mut self, now: Instant) -> bool {
        self.anim.is_animating(now)
    }

    /// Doğrulama uyarıları (geçersiz değer sınırlandı vb.).
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Store değiştiyse yeniden üretir. Tema değiştiyse `true` (bileşenlere `set_theme`).
    pub fn update(&mut self, store: &Store, now: Instant) -> bool {
        if store.revision() == self.revision {
            return false;
        }
        self.revision = store.revision();
        self.rebuild(store, now)
    }

    /// palette.toml değişti (runtime `theme_changed`).
    pub fn reload_palette(&mut self, store: &Store, now: Instant) -> bool {
        self.palette = load_palette(self.palette.take());
        self.rebuild(store, now)
    }

    fn rebuild(&mut self, store: &Store, now: Instant) -> bool {
        let (theme, warnings) = build(store, self.palette.as_ref());
        self.warnings = warnings;
        if &theme == self.anim.theme() {
            return false;
        }
        self.anim.set(theme, now);
        true
    }
}

/// Bellekteki belgeden tema. Palet dosyasının tamamı (kayıtlı kip + kip başına bölümler)
/// `Theme::from_generated`'a gider: `[colors] mode = "auto"` iken duvar kağıdı paletinin kipi
/// çözülür. Ayarlar'da palette.toml `[meta] mode` henüz yeniden üretilmeden değişmiş olabilir
/// (Duvar kağıdı › Kip ya da Koyu/Açık seçimi); bellekteki değer kaydın üstüne konur, böylece
/// önizleme üretimi beklemeden doğru kipe (dosyada o kipin bölümü varsa) geçer.
fn build(store: &Store, palette: Option<&GeneratedPalette>) -> (Theme, Vec<String>) {
    let mut palette = palette.cloned();
    if let Some(p) = &mut palette
        && let Some(mode) = store.get(FileId::Palette, &KeyPath::parse("meta.mode")).and_then(Value::as_str).filter(|m| ThemeMode::from_name(m).is_some())
    {
        p.meta.mode = Some(mode.to_owned());
    }
    let text = store.file(FileId::Framework).doc().to_string();
    match nuxut::config::parse_str::<FrameworkConfig>(&text) {
        Ok((cfg, mut warnings)) => {
            let (theme, mut more) = Theme::from_generated(&cfg, palette.as_ref());
            warnings.append(&mut more);
            (theme, warnings)
        }
        Err(e) => (Theme::from_generated(&FrameworkConfig::default(), palette.as_ref()).0, vec![e]),
    }
}

fn load_palette(previous: Option<GeneratedPalette>) -> Option<GeneratedPalette> {
    let path = palette_path()?;
    match GeneratedPalette::load(&path) {
        Ok(l) if l.from_file => Some(l.value),
        Ok(_) => None,
        Err(e) => {
            tracing::warn!("palet okunamadı, son geçerli korunuyor: {e}");
            previous
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nuxut::theme::PalettePatch;

    fn patch(surface: &str) -> PalettePatch {
        PalettePatch { surface: Some(nuxut::Color::from_hex(surface).unwrap()), ..Default::default() }
    }

    /// Duvar kağıdı paleti: kayıtlı kip `mode`, iki kip bölümü.
    fn generated(mode: &str) -> GeneratedPalette {
        let mut g = GeneratedPalette::default();
        g.meta.mode = Some(mode.into());
        g.colors = patch(if mode == "light" { "#fafafa" } else { "#101010" });
        g.dark = Some(patch("#101010"));
        g.light = Some(patch("#fafafa"));
        g
    }

    fn store(framework: &str, palette: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("f.toml"), framework).unwrap();
        std::fs::write(dir.path().join("p.toml"), palette).unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    #[test]
    fn auto_follows_wallpaper_palette_mode() {
        let g = generated("light");
        let (_d, s) = store("[colors]\nsource = \"wallpaper\"\n", "");
        let (t, _) = build(&s, Some(&g));
        assert_eq!(t.mode, ThemeMode::Light, "otomatik: paletin kayıtlı kipi");
        assert_eq!(t.colors.surface, g.light.as_ref().unwrap().surface.unwrap(), "o kipin bölümü");

        // Açık seçim paleti geçersiz kılar.
        let (_d, s) = store("[colors]\nsource = \"wallpaper\"\nmode = \"dark\"\n", "");
        let (t, _) = build(&s, Some(&g));
        assert_eq!(t.mode, ThemeMode::Dark);
        assert_eq!(t.colors.surface, g.dark.as_ref().unwrap().surface.unwrap());

        // Sabit renklerde palet yok sayılır; otomatik → statik temanın yüzeyi (koyu).
        let (_d, s) = store("", "");
        assert_eq!(build(&s, Some(&g)).0.mode, ThemeMode::Dark);
    }

    #[test]
    fn unsaved_palette_mode_is_previewed_before_regeneration() {
        // Dosyadaki kayıt koyu; Ayarlar bellekte `[meta] mode = "light"` yazdı (üretim sürüyor).
        let g = generated("dark");
        let (_d, s) = store("[colors]\nsource = \"wallpaper\"\n", "[meta]\nmode = \"light\"\n");
        let (t, _) = build(&s, Some(&g));
        assert_eq!(t.mode, ThemeMode::Light);
        assert_eq!(t.colors.surface, g.light.as_ref().unwrap().surface.unwrap());
        // Geçersiz bellek değeri kaydı bozmaz.
        let (_d, s) = store("[colors]\nsource = \"wallpaper\"\n", "[meta]\nmode = \"mor\"\n");
        assert_eq!(build(&s, Some(&g)).0.mode, ThemeMode::Dark);
    }
}
