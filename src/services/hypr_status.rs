//! Hyprland ana dosyasındaki Nuxut bağlantı satırının gerçek durumu (`nuxut-shell msg hypr
//! status` → `source`) ve kısayol çakışmaları (`conflicts`, `taken_over`). Ayarlar yalnızca okur;
//! satırı shell ekler/kaldırır, çakışan tuşta kimin bind'inin çalışacağını `[keybind_owner]`'a
//! yazar (Kısayollar sayfası, 2026-10-17).
//!
//! "Önceki hâline dön" `hypr.source`'u geri yükleyince shell satırı kaldırır ve bunu
//! `removed_by_user` sayar — kendiliğinden yeniden eklemez, devralınmış yabancı `nuxut.conf`'u
//! yeniden ezmez (shell ADR-0001 §9.5). Bu sırada `[compositor] ensure_source` açık
//! görünebilir; satır bunu sakin bir notla söyler: anahtar kapatılınca shell "ben ekledim"
//! işaretini siler, yeniden açılınca satırı ekler.
//!
//! Yanıt (shell `SyncStatus`): `{"source", "reason"?, "kind"?, "config"?,
//! "generated"?, "snapshot"?, "launch"?, "pending_switch"?, "overrides", "conflicts", "taken_over",
//! "errors"}`. `conflicts[]`/`taken_over[]`: `{action, combo, file?, line?, reason, can_take}`
//! (eski shell'de `reason`/`can_take` yok: çözülmemiş sayılır).
//! `nuxut-apply` eski `backup` (görünür `.bak` yolu) alanını kaldırdı; yerine `snapshot` = bu turda
//! alınan `hypr.source` snapshot kimliği (`msg snapshot restore <kimlik>` ile geri alınır).
//! Eski shell'in `backup` alanı yok sayılır.

use std::sync::Mutex;
use std::time::Duration;

use super::shell_ipc::{IpcLink, ShellLink};

/// Okunan çakışma/devralma öğesi sınırı (dış girdi).
const MAX_KEYS: usize = 256;

/// Bir eylemin kullanıcının Hyprland bind'iyle ilişkisi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyRelation {
    /// Kullanıcınınki bilerek çalışıyor: temel katman varsayılanı (`user_wins`) ya da kullanıcının
    /// seçimi (`user_choice`). `can_take`: "Nuxut'unkini kullan" işe yarar mı.
    UserKept { chosen: bool, can_take: bool },
    /// Nuxut'unki isteniyor ama devralınamıyor (`read_after|not_loaded|special_mods|unlocated`;
    /// eski shell'de neden yok).
    Blocked { reason: Option<String> },
    /// Nuxut devraldı (kullanıcının dosyası değişmedi).
    Taken,
}

/// `msg hypr status`'taki bir kısayol öğesi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyConflict {
    pub action: String,
    /// Kullanıcının tuşu (`SUPER+q`).
    pub combo: String,
    /// Kullanıcının dosyası (`~` kısaltmalı) ve satırı.
    pub file: Option<String>,
    pub line: Option<u64>,
    pub relation: KeyRelation,
}

impl KeyConflict {
    /// `keybinds.conf:12` (dosya adı + satır; yol uzun ve sayfada gürültü).
    pub fn place(&self) -> Option<String> {
        let name = self.file.as_deref()?.rsplit('/').next().filter(|n| !n.is_empty())?;
        Some(match self.line {
            Some(l) => format!("{name}:{l}"),
            None => name.to_owned(),
        })
    }
}

/// `msg hypr status` yanıtından Ayarlar'ın okuduğu alanlar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyprStatus {
    /// `present|added|off|disabled_by_user|removed_by_user|missing_config|unsupported|unknown|
    /// error|inactive|undone|pending` — bilinmeyen değer metin olarak tutulur.
    pub source: String,
    /// Bu turun `hypr.source` snapshot kimliği (geçerli biçimdeyse). Yalnızca tanı logu; UI'da
    /// kullanılmaz (geri al kaynağı `snapshot list --setting hypr.source`).
    pub snapshot: Option<String>,
    /// Kısayol çakışmaları ve devralınanlar.
    pub keys: Vec<KeyConflict>,
}

fn short(v: &serde_json::Value, key: &str, max: usize) -> Option<String> {
    v.get(key)?.as_str().filter(|s| !s.is_empty() && s.len() <= max && !s.chars().any(char::is_control)).map(str::to_owned)
}

fn parse_key(v: &serde_json::Value, taken: bool) -> Option<KeyConflict> {
    let action = short(v, "action", 64).filter(|a| a.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'))?;
    let reason = short(v, "reason", 32);
    let can_take = v.get("can_take").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let relation = if taken {
        KeyRelation::Taken
    } else {
        match reason.as_deref() {
            Some("user_wins") => KeyRelation::UserKept { chosen: false, can_take },
            Some("user_choice") => KeyRelation::UserKept { chosen: true, can_take },
            _ => KeyRelation::Blocked { reason },
        }
    };
    Some(KeyConflict {
        action,
        combo: short(v, "combo", 64).unwrap_or_default(),
        file: short(v, "file", 512),
        line: v.get("line").and_then(serde_json::Value::as_u64),
        relation,
    })
}

/// Yanıtı ayrıştırır; `source` yoksa ya da sözleşme dışıysa `None`.
pub fn parse_status(text: &str) -> Option<HyprStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    let source = o.get("source")?.as_str().filter(|s| !s.is_empty() && s.len() <= 32)?.to_owned();
    let snapshot = o.get("snapshot").and_then(|s| s.as_str()).filter(|s| super::snapshots::valid_snapshot_id(s)).map(str::to_owned);
    let list = |key: &str, taken: bool| -> Vec<KeyConflict> {
        o.get(key).and_then(|a| a.as_array()).map(|a| a.iter().take(MAX_KEYS).filter_map(|v| parse_key(v, taken)).collect()).unwrap_or_default()
    };
    let mut keys = list("conflicts", false);
    keys.extend(list("taken_over", true));
    Some(HyprStatus { source, snapshot, keys })
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
    query_status(link).map(|s| s.source)
}

/// Bir bağlantıdan durumun tamamını sorar.
pub fn query_status(link: &dyn ShellLink) -> Option<HyprStatus> {
    let status = parse_status(&link.request(&["hypr", "status"]).ok()?)?;
    // `snapshot` UI'da kullanılmaz: yalnızca tanı. Hoş geldin geri alının kaynağı
    // `snapshot list --setting hypr.source`.
    if let Some(id) = &status.snapshot {
        tracing::debug!(source = %status.source, snapshot = %id, "hypr status");
    }
    Some(status)
}

/// (son durum, sorgu yapıldı mı)
static STATUS: Mutex<(Option<HyprStatus>, bool)> = Mutex::new((None, false));
/// (sorgu sürüyor, sürerken yeni istek geldi)
static RUNNING: Mutex<(bool, bool)> = Mutex::new((false, false));

fn asked_status() -> Option<HyprStatus> {
    let (value, asked) = STATUS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if !asked {
        refresh(Duration::ZERO);
    }
    value
}

/// Son okunan `source`; hiç sorulmadıysa arka planda bir kez sorar.
pub fn source() -> Option<String> {
    asked_status().map(|s| s.source)
}

/// Eylemin (`volume_up`) kullanıcının Hyprland bind'iyle ilişkisi; çakışma yoksa `None`.
pub fn key(action: &str) -> Option<KeyConflict> {
    asked_status()?.keys.into_iter().find(|k| k.action == action)
}

/// Testte son durumu doğrudan koyar (satır testleri).
#[cfg(test)]
pub fn set_for_test(status: Option<HyprStatus>) {
    *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = (status, true);
}

/// `delay` sonra yeniden sorar (shell anahtar değişimini uygulasın diye beklenir); bitince
/// [`Event::Hypr`](super::Event::Hypr). Aynı anda tek sorgu; sürerken gelen istek bittiğinde bir
/// kez daha sordurur (yazılan değişiklik eski yanıtla kalmasın).
pub fn refresh(delay: Duration) {
    // Birim testleri soket açmaz (IpcLink zaten reddeder); bekleyen thread de bırakılmaz.
    if cfg!(test) {
        STATUS.lock().unwrap_or_else(|e| e.into_inner()).1 = true;
        return;
    }
    {
        let mut running = RUNNING.lock().unwrap_or_else(|e| e.into_inner());
        if running.0 {
            running.1 = true;
            return;
        }
        *running = (true, false);
    }
    STATUS.lock().unwrap_or_else(|e| e.into_inner()).1 = true;
    let spawned = std::thread::Builder::new().name("hypr-status".into()).spawn(move || {
        let mut delay = delay;
        loop {
            std::thread::sleep(delay);
            let value = query_status(&IpcLink::from_env());
            *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = (value, true);
            super::emit(super::Event::Hypr);
            let mut running = RUNNING.lock().unwrap_or_else(|e| e.into_inner());
            if !running.1 {
                *running = (false, false);
                break;
            }
            running.1 = false;
            delay = Duration::from_millis(700);
        }
    });
    if spawned.is_err() {
        *RUNNING.lock().unwrap_or_else(|e| e.into_inner()) = (false, false);
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
            Some(HyprStatus { source: "added".into(), snapshot: Some("20261011T065000Z-1a2b".into()), keys: Vec::new() })
        );
        let old = r#"{"source":"added","config":"~/.config/hypr/hyprland.conf","backup":"~/.config/hypr/hyprland.conf.bak","overrides":[],"conflicts":[],"errors":[]}"#;
        assert_eq!(parse_status(old), Some(HyprStatus { source: "added".into(), snapshot: None, keys: Vec::new() }));
        let bad = r#"{"source":"present","snapshot":"../../x"}"#;
        assert_eq!(parse_status(bad).unwrap().snapshot, None);
        // Satır geri yüklemeyle kalktı: removed_by_user, anahtar açıkken not.
        let removed = r#"{"source":"removed_by_user","config":"~/.config/hypr/hyprland.conf","overrides":[],"conflicts":[],"errors":[]}"#;
        assert!(source_note(true, parse_source(removed).as_deref()).is_some());
    }

    /// Kısayol çakışmaları (2026-10-17): nedenine göre üç ilişki; eski shell'in nedensiz öğesi
    /// çözülmemiş sayılır; sözleşme dışı öğe atlanır.
    #[test]
    fn parses_key_conflicts() {
        let text = r#"{"source":"present","conflicts":[
            {"action":"close_window","combo":"SUPER+q","file":"~/.config/hypr/hyprland/keybinds.conf","line":40,"reason":"user_wins","can_take":true},
            {"action":"volume_mute","combo":"xf86audiomute","reason":"user_choice","can_take":true},
            {"action":"clipboard","combo":"SUPER+SHIFT+v","file":"~/.config/hypr/custom/late.conf","line":1,"reason":"read_after","can_take":false},
            {"action":"panel_right","combo":"SUPER+n"},
            {"action":"Kötü Ad","combo":"x"}
        ],"taken_over":[{"action":"volume_up","combo":"xf86audioraisevolume","file":"~/k.conf","line":7,"reason":"default","can_take":true}]}"#;
        let s = parse_status(text).unwrap();
        let rel: Vec<(&str, &KeyRelation)> = s.keys.iter().map(|k| (k.action.as_str(), &k.relation)).collect();
        assert_eq!(
            rel,
            [
                ("close_window", &KeyRelation::UserKept { chosen: false, can_take: true }),
                ("volume_mute", &KeyRelation::UserKept { chosen: true, can_take: true }),
                ("clipboard", &KeyRelation::Blocked { reason: Some("read_after".into()) }),
                ("panel_right", &KeyRelation::Blocked { reason: None }),
                ("volume_up", &KeyRelation::Taken),
            ]
        );
        assert_eq!(s.keys[0].place().as_deref(), Some("keybinds.conf:40"));
        assert_eq!(s.keys[1].place(), None);
    }
}
