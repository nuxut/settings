//! Hyprland ayarlarından okunan, Ayarlar'ın davranışını etkileyen değerler (yalnızca okuma).
//!
//! Şimdilik tek değer: `input:resolve_binds_by_sym`. Hyprland bind'i basılan tuşun seviye-0
//! keysym'iyle eşler; bu ayar kapalıyken (varsayılan) ilk düzenin, açıkken etkin düzenin
//! keysym'i kullanılır. Kısayol yakalayıcı (`crate::keybind::capture`) aynı seçimi yapar.
//!
//! Değer `hyprctl getoption input:resolve_binds_by_sym -j` ile arka plan thread'inde okunur
//! ([`refresh`]) ve önbellekte tutulur ([`resolve_binds_by_sym`]); UI thread'i süreç beklemez.
//! Hyprland oturumu yoksa, `hyprctl` bulunamazsa ya da yanıt çözülemezse `false` (Hyprland'in
//! varsayılanı).

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

/// Son okunan değer (okunmadıysa Hyprland varsayılanı `false`).
static BY_SYM: AtomicBool = AtomicBool::new(false);
/// Bir okuma sürüyor: üst üste thread açılmaz.
static IN_FLIGHT: AtomicBool = AtomicBool::new(false);

const OPTION: &str = "input:resolve_binds_by_sym";

/// Önbellekteki `input:resolve_binds_by_sym` değeri.
pub fn resolve_binds_by_sym() -> bool {
    BY_SYM.load(Ordering::Relaxed)
}

/// Değeri arka planda yeniden okur (kısayol yakalama başlarken). Kullanıcı ilk tuşa basana dek
/// biter; bitmezse önceki değer kullanılır.
pub fn refresh() {
    // Birim testleri kullanıcının Hyprland'ine sormaz (sonuç ortama bağlı olmasın).
    if cfg!(test) || std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_none() {
        BY_SYM.store(false, Ordering::Relaxed);
        return;
    }
    if IN_FLIGHT.swap(true, Ordering::AcqRel) {
        return;
    }
    let spawned = std::thread::Builder::new().name("hypr-option".into()).spawn(|| {
        BY_SYM.store(read_option().unwrap_or(false), Ordering::Relaxed);
        IN_FLIGHT.store(false, Ordering::Release);
    });
    if let Err(e) = spawned {
        tracing::warn!("hypr-option thread açılamadı: {e}");
        IN_FLIGHT.store(false, Ordering::Release);
    }
}

fn read_option() -> Option<bool> {
    let out = Command::new("hyprctl")
        .args(["getoption", OPTION, "-j"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let v = parse_bool_option(&String::from_utf8_lossy(&o.stdout));
            if v.is_none() {
                tracing::warn!("hyprctl getoption {OPTION}: yanıt çözülemedi; varsayılan false");
            }
            v
        }
        Ok(o) => {
            tracing::warn!("hyprctl getoption {OPTION}: çıkış {}; varsayılan false", o.status);
            None
        }
        Err(e) => {
            tracing::warn!("hyprctl çalıştırılamadı ({e}); resolve_binds_by_sym varsayılan false");
            None
        }
    }
}

/// `hyprctl getoption <bool seçenek> -j` yanıtı → değer. Bool seçenekler `int` alanında
/// (0/1) döner; bilinmeyen seçenekte Hyprland JSON değil `no such option` yazar → `None`.
pub fn parse_bool_option(json: &str) -> Option<bool> {
    let v: serde_json::Value = serde_json::from_str(json.trim()).ok()?;
    v.get("int")?.as_i64().map(|i| i != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hyprctl_getoption_json() {
        // Hyprland 0.56.2'nin gerçek çıktısı.
        let off = r#"{"option": "input:resolve_binds_by_sym", "int": 0, "set": false }"#;
        assert_eq!(parse_bool_option(off), Some(false));
        let on = "{\"option\": \"input:resolve_binds_by_sym\", \"int\": 1, \"set\": true }\n";
        assert_eq!(parse_bool_option(on), Some(true));
    }

    #[test]
    fn unreadable_answers_are_none() {
        assert_eq!(parse_bool_option("no such option"), None);
        assert_eq!(parse_bool_option(""), None);
        assert_eq!(parse_bool_option(r#"{"option": "x", "str": "1"}"#), None);
        assert_eq!(parse_bool_option(r#"{"int": "1"}"#), None);
    }
}
