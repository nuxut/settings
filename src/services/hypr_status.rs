//! Hyprland ana dosyasındaki Nuxut bağlantı satırının gerçek durumu (`nuxut-shell msg hypr
//! status` → `source`). Ayarlar yalnızca okur; satırı shell ekler/kaldırır.
//!
//! "Önceki hâline dön" `hypr.source`'u geri yükleyince shell satırı kaldırır ve bunu
//! `removed_by_user` sayar — kendiliğinden yeniden eklemez, devralınmış yabancı `nuxut.conf`'u
//! yeniden ezmez (shell ADR-0001 §9.5). Bu sırada `[compositor] ensure_source` açık
//! görünebilir; satır bunu sakin bir notla söyler: anahtar kapatılınca shell "ben ekledim"
//! işaretini siler, yeniden açılınca satırı ekler.
//!
//! Yanıt (shell `SyncStatus`): `{"source", "reason"?, "kind"?, "config"?,
//! "generated"?, "snapshot"?, "launch"?, "pending_switch"?, "overrides", "conflicts", "errors"}`.
//! `nuxut-apply` eski `backup` (görünür `.bak` yolu) alanını kaldırdı; yerine `snapshot` = bu turda
//! alınan `hypr.source` snapshot kimliği (`msg snapshot restore <kimlik>` ile geri alınır).
//! Eski shell'in `backup` alanı yok sayılır.

use std::sync::Mutex;
use std::time::Duration;

use super::shell_ipc::{IpcLink, ShellLink};

/// `msg hypr status` yanıtından Ayarlar'ın okuduğu alanlar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyprStatus {
    /// `present|added|off|disabled_by_user|removed_by_user|missing_config|unsupported|unknown|
    /// error|inactive|undone|pending` — bilinmeyen değer metin olarak tutulur.
    pub source: String,
    /// Bu turun `hypr.source` snapshot kimliği (geçerli biçimdeyse). Yalnızca tanı logu; UI'da
    /// kullanılmaz (geri al kaynağı `snapshot list --setting hypr.source`).
    pub snapshot: Option<String>,
}

/// Yanıtı ayrıştırır; `source` yoksa ya da sözleşme dışıysa `None`.
pub fn parse_status(text: &str) -> Option<HyprStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    let source = o.get("source")?.as_str().filter(|s| !s.is_empty() && s.len() <= 32)?.to_owned();
    let snapshot = o.get("snapshot").and_then(|s| s.as_str()).filter(|s| super::snapshots::valid_snapshot_id(s)).map(str::to_owned);
    Some(HyprStatus { source, snapshot })
}

/// Yalnızca `source`.
pub fn parse_source(text: &str) -> Option<String> {
    parse_status(text).map(|s| s.source)
}

/// Bağlantı satırı notu: anahtar açıkken satır kullanıcı (ya da geri yükleme) tarafından
/// kaldırılmışsa; aksi hâlde `None` (not gösterilmez). `&'static str` dönebilmek için katalog
/// metni [`crate::i18n::text_or`] ile istenir (kaynak Türkçe yedek).
pub fn source_note(ensure_source: bool, source: Option<&str>) -> Option<&'static str> {
    (ensure_source && source == Some("removed_by_user")).then(|| {
        crate::i18n::text_or("settings-hypr-status-source-removed", "Bağlantı satırı ayar dosyasından kaldırıldı. Yeniden eklemek için bu ayarı kapatıp açın.")
    })
}

/// Bir bağlantıdan `source`'u sorar (saf; testte sahte bağlantı). Shell yok / desteklemiyor →
/// `None` (not gösterilmez; hata değil).
pub fn query_source(link: &dyn ShellLink) -> Option<String> {
    let status = parse_status(&link.request(&["hypr", "status"]).ok()?)?;
    // `snapshot` UI'da kullanılmaz: yalnızca tanı. Hoş geldin geri alının kaynağı
    // `snapshot list --setting hypr.source`.
    if let Some(id) = &status.snapshot {
        tracing::debug!(source = %status.source, snapshot = %id, "hypr status");
    }
    Some(status.source)
}

/// (son `source`, sorgu yapıldı mı)
static SOURCE: Mutex<(Option<String>, bool)> = Mutex::new((None, false));
static RUNNING: Mutex<bool> = Mutex::new(false);

/// Son okunan `source`; hiç sorulmadıysa arka planda bir kez sorar.
pub fn source() -> Option<String> {
    let (value, asked) = SOURCE.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if !asked {
        refresh(Duration::ZERO);
    }
    value
}

/// `delay` sonra yeniden sorar (shell anahtar değişimini uygulasın diye beklenir); bitince
/// [`Event::Hypr`](super::Event::Hypr). Aynı anda tek sorgu.
pub fn refresh(delay: Duration) {
    {
        let mut running = RUNNING.lock().unwrap_or_else(|e| e.into_inner());
        if *running {
            return;
        }
        *running = true;
    }
    SOURCE.lock().unwrap_or_else(|e| e.into_inner()).1 = true;
    let spawned = std::thread::Builder::new().name("hypr-status".into()).spawn(move || {
        std::thread::sleep(delay);
        let value = query_source(&IpcLink::from_env());
        *SOURCE.lock().unwrap_or_else(|e| e.into_inner()) = (value, true);
        *RUNNING.lock().unwrap_or_else(|e| e.into_inner()) = false;
        super::emit(super::Event::Hypr);
    });
    if spawned.is_err() {
        *RUNNING.lock().unwrap_or_else(|e| e.into_inner()) = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::shell_ipc::{FakeLink, LinkError};

    #[test]
    fn note_only_when_switch_on_and_line_removed() {
        crate::i18n::use_turkish_for_tests();
        assert!(source_note(true, Some("removed_by_user")).is_some());
        assert_eq!(source_note(false, Some("removed_by_user")), None, "anahtar kapalı: not yok");
        for s in ["present", "added", "off", "disabled_by_user", "undone", "pending", "yepyeni"] {
            assert_eq!(source_note(true, Some(s)), None, "{s}");
        }
        assert_eq!(source_note(true, None), None, "shell yok: not yok");
        let note = source_note(true, Some("removed_by_user")).unwrap();
        assert!(!note.contains("Hyprland") && note.contains("kapatıp açın"));
    }

    #[test]
    fn parses_hypr_status_source() {
        let link = FakeLink::with([
            Ok(r#"{"source":"removed_by_user","config":"~/.config/hypr/hyprland.conf","overrides":[],"conflicts":[],"errors":[]}"#.to_owned()),
            Ok(r#"{"source":"yeni_durum"}"#.to_owned()),
            Ok("ok".to_owned()),
            Err(LinkError::NotRunning),
        ]);
        assert_eq!(query_source(&link).as_deref(), Some("removed_by_user"));
        assert_eq!(query_source(&link).as_deref(), Some("yeni_durum"), "bilinmeyen değerde kırılmaz");
        assert_eq!(query_source(&link), None);
        assert_eq!(query_source(&link), None);
        assert!(link.sent().iter().all(|a| a == &["hypr", "status"]));
        assert_eq!(parse_source(r#"{"source":""}"#), None);
    }

    /// `nuxut-apply`: `backup` → `snapshot`. Eski shell'in `backup`'ı yok sayılır, sakin.
    #[test]
    fn snapshot_field_replaces_backup() {
        let new = r#"{"source":"added","kind":"hyprlang","config":"~/.config/hypr/hyprland.conf","generated":"~/.config/hypr/nuxut/nuxut.conf","snapshot":"20261011T065000Z-1a2b","launch":"exec_once","overrides":[],"conflicts":[],"errors":[]}"#;
        assert_eq!(
            parse_status(new),
            Some(HyprStatus { source: "added".into(), snapshot: Some("20261011T065000Z-1a2b".into()) })
        );
        let old = r#"{"source":"added","config":"~/.config/hypr/hyprland.conf","backup":"~/.config/hypr/hyprland.conf.bak","overrides":[],"conflicts":[],"errors":[]}"#;
        assert_eq!(parse_status(old), Some(HyprStatus { source: "added".into(), snapshot: None }));
        let bad = r#"{"source":"present","snapshot":"../../x"}"#;
        assert_eq!(parse_status(bad).unwrap().snapshot, None);
        // Satır geri yüklemeyle kalktı: removed_by_user, anahtar açıkken not.
        let removed = r#"{"source":"removed_by_user","config":"~/.config/hypr/hyprland.conf","overrides":[],"conflicts":[],"errors":[]}"#;
        assert!(source_note(true, parse_source(removed).as_deref()).is_some());
    }
}
