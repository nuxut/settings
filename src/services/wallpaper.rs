//! Duvar kağıdı çözümü (UI'dan bağımsız): süreç başına tek [`WallpaperDecoder`].
//!
//! Kullanıcının seçtiği görsel güvenilmeyen girdidir; Ayarlar süreci onu kendisi çözmez
//! (framework ADR-0007 takip, "Duvar kağıdı yalıtımı"). Bütün çözümler — Duvar kağıdı sayfasının
//! hero'su, Görünüm önizlemesi, monitör başına satırlar ve renk örneklemesi — tek
//! `Sandbox::new(SandboxConfig::wallpaper())` üzerinden yalıtılmış `nuxut-image-worker`'da yapılır.
//! JPEG XL çözücüsü (jxl-oxide) bu sürece hiç bağlanmaz.
//!
//! Hedefler (framework ARCHITECTURE "Tüketici kurulumu"):
//! - hero/önizleme: kartın fiziksel boyutu, üst sınır [`HERO_MAX`], `Cover`;
//! - monitör satırı: [`ROW_LOGICAL`] × ölçek, `Cover`;
//! - renk: [`COLOR_TARGET`] `Contain` (kırpma kenar renklerini atardı) →
//!   `nuxut_palette::source_color_from_frame`.
//!
//! İşçi yoksa (`NUXUT_IMAGE_WORKER` → ikilinin yanı → `~/.local/lib/nuxut` → `/usr/lib/nuxut`)
//! framework'ün S1 kuralı: temel raster (PNG/JPEG/WebP/GIF/QOI) süreç içinde kapılarla, JXL/AVIF/SVG
//! `Sandbox(Unavailable)` → kart yer tutucuda kalır, renk satırı [`error_message`] metnini gösterir.
//! Boşta: çözülmüş kare, thread ve işçi süreci yok (`linger` sonrası).

use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, OnceLock};

use nuxut::i18n::t;
use nuxut_image::{Fit, ImageError, SandboxFailure, WallpaperDecoder, WallpaperDecoderOptions, WallpaperFrame, WallpaperRequest};
use nuxut_image_sandbox::{Sandbox, SandboxConfig};

/// Hero ve Görünüm önizlemesinin çözüm üst sınırı (fiziksel px).
pub const HERO_MAX: (u32, u32) = (1280, 800);
/// Monitör başına satır küçük resminin mantıksal boyutu (ölçekle çarpılır).
pub const ROW_LOGICAL: (f32, f32) = (192.0, 120.0);
/// Renk örneklemesi hedefi (`nuxut_palette::SAMPLE_TARGET` ile aynı kutu, `Contain`).
pub const COLOR_TARGET: (u32, u32) = (128, 128);
/// Önbellek: kahraman ≈ 4 MB, satırlar 100–200 KB, renk 64 KB; 16 MiB fazlasıyla yeter.
const CACHE_BYTES: u64 = 16 << 20;
/// Kart genişliği bu adıma yukarı yuvarlanır: pencere sürüklenirken her pikselde yeniden
/// çözülmesin (ara boyutlarda doku GPU'da küçültülür).
const STEP: f32 = 64.0;

static SHARED: OnceLock<Arc<WallpaperDecoder>> = OnceLock::new();

/// Sürecin paylaşılan çözücüsü. İlk çağrıda kurulur; süreç başlatmaz (işçi ilk istekte, iş
/// bitince kapanır).
pub fn shared() -> Arc<WallpaperDecoder> {
    SHARED
        .get_or_init(|| {
            let sandbox = Arc::new(Sandbox::new(SandboxConfig::wallpaper()));
            WallpaperDecoder::new(sandbox, WallpaperDecoderOptions { cache_bytes: CACHE_BYTES, ..WallpaperDecoderOptions::default() })
        })
        .clone()
}

/// Kartın (`size` mantıksal px, `scale` çıkış ölçeği) çözüm hedefi: fiziksel boyut, genişlik
/// [`STEP`] katına yukarı yuvarlanır, oran korunarak `max`'a sığdırılır. Kart henüz
/// yerleşmemişse (`0×0`) ya da ölçü geçersizse `None`.
pub fn card_target(size: (f32, f32), scale: f32, max: (u32, u32)) -> Option<(u32, u32)> {
    let scale = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
    let (w, h) = (size.0 * scale, size.1 * scale);
    if !(w.is_finite() && h.is_finite() && w >= 1.0 && h >= 1.0) {
        return None;
    }
    let qw = (w / STEP).ceil() * STEP;
    let qh = qw * h / w;
    let k = (max.0 as f32 / qw).min(max.1 as f32 / qh).min(1.0);
    Some(((qw * k).round().max(1.0) as u32, (qh * k).round().max(1.0) as u32))
}

/// Monitör satırı küçük resminin hedefi: `192×120 × ölçek`.
pub fn row_target(scale: f32) -> (u32, u32) {
    let scale = if scale.is_finite() && scale > 0.0 { scale.min(4.0) } else { 1.0 };
    ((ROW_LOGICAL.0 * scale).round() as u32, (ROW_LOGICAL.1 * scale).round() as u32)
}

/// Renk örneklemesi için kareyi paylaşılan çözücüden ister ve bekler (palet thread'i). Elde aynı
/// dosyanın daha büyük `Contain` karesi varsa framework onu türetir.
pub fn color_frame(path: &Path) -> Result<WallpaperFrame, ImageError> {
    color_frame_with(&shared(), path)
}

fn color_frame_with(decoder: &Arc<WallpaperDecoder>, path: &Path) -> Result<WallpaperFrame, ImageError> {
    let (tx, rx) = mpsc::sync_channel(1);
    // Bilet sonuç gelene dek yaşar; düşerse istek iptal olurdu.
    let _ticket = decoder.request(WallpaperRequest::new(path, COLOR_TARGET, Fit::Contain), move |r| {
        let _ = tx.send(r);
    });
    rx.recv().unwrap_or(Err(ImageError::Cancelled))
}

/// Çözüm hatasının kullanıcı metni (framework hata eşlemesi; yol ve ayrıntı loglanmaz).
pub fn error_message(e: &ImageError) -> String {
    match e {
        ImageError::TooLarge { .. } | ImageError::FileTooLarge { .. } => t!("settings-wallpaper-error-too-large"),
        ImageError::Sandbox(SandboxFailure::Unavailable) => t!("settings-wallpaper-error-worker"),
        _ => t!("settings-wallpaper-error-source"),
    }
}

/// Log için hata türü (yol yok).
pub fn error_kind(e: &ImageError) -> &'static str {
    match e {
        ImageError::Io { .. } => "io",
        ImageError::FileTooLarge { .. } => "file-too-large",
        ImageError::TooLarge { .. } => "too-large",
        ImageError::UnknownFormat | ImageError::Unsupported(_) => "format",
        ImageError::Cancelled => "cancelled",
        ImageError::Sandbox(SandboxFailure::Unavailable) => "worker-unavailable",
        ImageError::Sandbox(SandboxFailure::Timeout) => "timeout",
        ImageError::Sandbox(SandboxFailure::Killed) => "killed",
        ImageError::Sandbox(SandboxFailure::Protocol) => "protocol",
        ImageError::Sandbox(SandboxFailure::RateLimited) => "rate-limited",
        _ => "decode",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_target_is_physical_quantized_and_capped() {
        // 600×338 mantıksal, 1× → genişlik 640'a yuvarlanır, oran korunur.
        let (w, h) = card_target((600.0, 337.5), 1.0, HERO_MAX).unwrap();
        assert_eq!(w, 640);
        assert!((h as f32 - 640.0 * 337.5 / 600.0).abs() <= 1.0, "{h}");
        // 1.5 ölçek: 900 → 960.
        assert_eq!(card_target((600.0, 337.5), 1.5, HERO_MAX).unwrap().0, 960);
        // 2× büyük kart: üst sınır, oran korunarak.
        let (w, h) = card_target((960.0, 540.0), 2.0, HERO_MAX).unwrap();
        assert!(w <= HERO_MAX.0 && h <= HERO_MAX.1 && (w == HERO_MAX.0 || h == HERO_MAX.1), "{w}×{h}");
        assert!(((w as f32 / h as f32) - 16.0 / 9.0).abs() < 0.01);
        // Birkaç piksellik genişlik değişimi aynı hedef (yeniden çözüm yok).
        assert_eq!(card_target((601.0, 338.0), 1.0, HERO_MAX).map(|t| t.0), card_target((630.0, 354.0), 1.0, HERO_MAX).map(|t| t.0));
    }

    #[test]
    fn card_target_rejects_unplaced_and_bad_scale() {
        assert_eq!(card_target((0.0, 0.0), 1.0, HERO_MAX), None);
        assert_eq!(card_target((f32::NAN, 10.0), 1.0, HERO_MAX), None);
        assert_eq!(card_target((300.0, 200.0), f32::NAN, HERO_MAX), card_target((300.0, 200.0), 1.0, HERO_MAX));
    }

    #[test]
    fn row_target_scales() {
        assert_eq!(row_target(1.0), (192, 120));
        assert_eq!(row_target(1.5), (288, 180));
        assert_eq!(row_target(0.0), (192, 120));
    }

    #[test]
    fn error_messages_come_from_catalog() {
        crate::i18n::use_turkish_for_tests();
        let big = error_message(&ImageError::TooLarge { width: 1, height: 1 });
        let worker = error_message(&ImageError::Sandbox(SandboxFailure::Unavailable));
        let other = error_message(&ImageError::UnknownFormat);
        for m in [&big, &worker, &other] {
            assert!(!m.is_empty() && !m.starts_with("settings-"), "{m}");
        }
        assert_ne!(big, other);
        assert_ne!(worker, other);
        assert_eq!(error_kind(&ImageError::Sandbox(SandboxFailure::Unavailable)), "worker-unavailable");
    }

    /// Süreç içi çözücüyle (işçisiz test) renk karesi Contain 128 içinde ve tutarlı.
    #[test]
    fn color_frame_is_contain_128() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.png");
        std::fs::write(&path, tiny_png()).unwrap();
        let decoder = WallpaperDecoder::in_process_unsandboxed(WallpaperDecoderOptions::default());
        let frame = color_frame_with(&decoder, &path).unwrap();
        assert!(frame.width <= COLOR_TARGET.0 && frame.height <= COLOR_TARGET.1);
        assert_eq!(frame.size(), (2, 1));
        let missing = color_frame_with(&decoder, &dir.path().join("yok.png"));
        assert!(missing.is_err());
    }

    /// 2×1 RGBA PNG (kırmızı, mavi), sıkıştırmasız deflate bloğuyla elle kodlanmış.
    fn tiny_png() -> Vec<u8> {
        fn crc32(data: &[u8]) -> u32 {
            let mut c = 0xffff_ffffu32;
            for &b in data {
                c ^= b as u32;
                for _ in 0..8 {
                    c = if c & 1 != 0 { 0xedb8_8320 ^ (c >> 1) } else { c >> 1 };
                }
            }
            !c
        }
        fn adler32(data: &[u8]) -> u32 {
            let (mut a, mut b) = (1u32, 0u32);
            for &x in data {
                a = (a + x as u32) % 65521;
                b = (b + a) % 65521;
            }
            (b << 16) | a
        }
        fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
            out.extend_from_slice(&(data.len() as u32).to_be_bytes());
            let mut c = kind.to_vec();
            c.extend_from_slice(data);
            out.extend_from_slice(&c);
            out.extend_from_slice(&crc32(&c).to_be_bytes());
        }
        let raw = [0u8, 255, 0, 0, 255, 0, 0, 255, 255];
        let mut z = vec![0x78, 0x01, 0x01];
        z.extend_from_slice(&(raw.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(raw.len() as u16)).to_le_bytes());
        z.extend_from_slice(&raw);
        z.extend_from_slice(&adler32(&raw).to_be_bytes());
        let mut ihdr = Vec::new();
        ihdr.extend_from_slice(&2u32.to_be_bytes());
        ihdr.extend_from_slice(&1u32.to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
        let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
        chunk(&mut out, b"IHDR", &ihdr);
        chunk(&mut out, b"IDAT", &z);
        chunk(&mut out, b"IEND", &[]);
        out
    }
}
