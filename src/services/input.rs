//! Girdi ayarlarının etkin durumu (`nuxut-shell msg input status`, sözleşme girdi-v1).
//!
//! Ayarlar `shell.toml [input]` yazar; shell bunu Hyprland satırlarına çevirir. Yazılmamış anahtar
//! için Hyprland'in (kullanıcının kendi ayar dosyasındaki) değeri geçerli kalır — "içe aktar,
//! ezme". Klavye ve Fare sayfaları satır değerini şu sırayla gösterir: dosyada yazılıysa dosya,
//! değilse buradaki ETKİN değer (Hyprland `getoption`), o da yoksa şema varsayılanı.
//!
//! Yanıt (sürüm 1): `{"v":1, "available", "keyboard":{layouts, options, repeat_rate,
//! repeat_delay, numlock_by_default}, "mouse":{sensitivity, accel_profile, natural_scroll},
//! "touchpad":{natural_scroll, tap_to_click, disable_while_typing, scroll_factor, scroll_method},
//! "managed":[anahtar], "overrides":[{key, hypr, file, line}]}`. `available: false` → değer
//! alanları `null`. Shell'in yanıtı güvenilmez girdi gibi doğrulanır (ad biçimi, sınırlar).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use toml_edit::Value;

use nuxut::i18n::t;

use super::shell_ipc::{IpcLink, LinkError, ShellLink};

/// Desteklenen yanıt sürümü.
pub const VERSION: u64 = 1;
/// Düzen listesinin üst sınırı (sözleşme: 1..=4).
pub const MAX_LAYOUTS: usize = 4;
/// `kb_options` listesinin üst sınırı (sözleşme: 0..=16).
pub const MAX_OPTIONS: usize = 16;
/// Kabul edilen en çok geçersiz kılma kaydı.
const MAX_OVERRIDES: usize = 64;

/// Bir düzen: `kb_layout` öğesi + aynı sıradaki `kb_variant` öğesi (boş = varsayılan varyant).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KbLayout {
    pub layout: String,
    pub variant: String,
}

impl KbLayout {
    pub fn new(layout: impl Into<String>, variant: impl Into<String>) -> Self {
        Self { layout: layout.into(), variant: variant.into() }
    }
}

/// Kullanıcı dosyasının geçersiz kıldığı yönetilen anahtar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Override {
    /// `input.keyboard.repeat_rate`
    pub key: String,
    /// `input:repeat_rate`
    pub hypr: String,
    /// `~` kısaltmalı dosya yolu; shell satırı bulamadıysa (Lua config, `getoption` yedeği) `None`.
    pub file: Option<String>,
    pub line: Option<u64>,
}

/// `msg input status` yanıtının ayrıştırılmış hâli.
#[derive(Debug, Clone, Default)]
pub struct InputStatus {
    /// Hyprland'e ulaşıldı mı; `false` ise değerler boş.
    pub available: bool,
    /// Tekil anahtarların etkin değerleri (`input.keyboard.repeat_rate` → 70). Boş metin
    /// (`accel_profile = ""` = libinput varsayılanı) ve tipi uymayan değer yoktur.
    pub values: BTreeMap<String, Value>,
    /// Etkin düzenler (en az bir öğe; yoksa `None`).
    pub layouts: Option<Vec<KbLayout>>,
    /// Etkin `kb_options`.
    pub options: Option<Vec<String>>,
    pub managed: Vec<String>,
    pub overrides: Vec<Override>,
}

impl PartialEq for InputStatus {
    fn eq(&self, o: &Self) -> bool {
        // `toml_edit::Value` eşitlik sunmaz: değerler metin hâliyle karşılaştırılır.
        let vals = |s: &Self| s.values.iter().map(|(k, v)| (k.clone(), v.to_string())).collect::<Vec<_>>();
        self.available == o.available && self.layouts == o.layouts && self.options == o.options && self.managed == o.managed && self.overrides == o.overrides && vals(self) == vals(o)
    }
}

impl InputStatus {
    /// Anahtarın geçersiz kılma kaydı.
    pub fn override_for(&self, key: &str) -> Option<&Override> {
        self.overrides.iter().find(|o| o.key == key)
    }
}

/// Shell'den okunan son durum.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum InputState {
    /// Henüz sorulmadı / yanıt bekleniyor.
    #[default]
    Pending,
    Ready(Arc<InputStatus>),
    /// Shell çalışmıyor.
    NotRunning,
    /// Shell bu komutu tanımıyor (eski sürüm) ya da yanıt sözleşme dışı.
    Unsupported,
    /// Test örneği: bilerek sorulmadı.
    TestInstance,
    Failed(String),
}

impl InputState {
    /// Hyprland'den okunmuş, kullanılabilir durum.
    pub fn ready(&self) -> Option<&InputStatus> {
        match self {
            InputState::Ready(s) if s.available => Some(s),
            _ => None,
        }
    }
}

/// Config'e yazılabilir düzen/varyant adı (shell doğrulamasıyla aynı: `[A-Za-z0-9_()+:-]{1,64}`).
pub fn valid_name(s: &str) -> bool {
    super::xkb::valid_name(s)
}

/// `kb_options` öğesi: `grup:ad` (`^[a-z0-9_]+:[A-Za-z0-9_()+-]{1,64}$`).
pub fn valid_option(s: &str) -> bool {
    let Some((group, name)) = s.split_once(':') else { return false };
    !group.is_empty()
        && group.len() <= 64
        && group.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        && (1..=64).contains(&name.len())
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'(' | b')' | b'+' | b'-'))
}

/// Düzen değiştirme (`grp:`) seçeneği mi.
pub fn is_switch_option(s: &str) -> bool {
    s.starts_with("grp:")
}

fn parse_layouts(v: Option<&serde_json::Value>) -> Option<Vec<KbLayout>> {
    let arr = v?.as_array()?;
    let mut out = Vec::new();
    for item in arr.iter().take(MAX_LAYOUTS * 2) {
        let o = item.as_object()?;
        let layout = o.get("layout")?.as_str()?;
        let variant = o.get("variant").and_then(|v| v.as_str()).unwrap_or("");
        if !valid_name(layout) || !(variant.is_empty() || valid_name(variant)) {
            return None;
        }
        out.push(KbLayout::new(layout, variant));
    }
    (!out.is_empty()).then_some(out)
}

fn parse_options(v: Option<&serde_json::Value>) -> Option<Vec<String>> {
    let arr = v?.as_array()?;
    // Geçersiz öğe atlanır (Hyprland'in bilmediğimiz biçimdeki seçeneği yazılmaz, korunmaz).
    Some(arr.iter().filter_map(|x| x.as_str()).filter(|s| valid_option(s)).take(MAX_OPTIONS).map(str::to_owned).collect())
}

/// Bir bölümün tekil değerleri: `(json alanı, tür)`.
const FIELDS: &[(&str, &str, Kind)] = &[
    ("keyboard", "repeat_rate", Kind::Int),
    ("keyboard", "repeat_delay", Kind::Int),
    ("keyboard", "numlock_by_default", Kind::Bool),
    ("mouse", "sensitivity", Kind::Float),
    ("mouse", "accel_profile", Kind::Text),
    ("mouse", "natural_scroll", Kind::Bool),
    ("touchpad", "natural_scroll", Kind::Bool),
    ("touchpad", "tap_to_click", Kind::Bool),
    ("touchpad", "disable_while_typing", Kind::Bool),
    ("touchpad", "scroll_factor", Kind::Float),
    ("touchpad", "scroll_method", Kind::Text),
];

#[derive(Debug, Clone, Copy)]
enum Kind {
    Int,
    Float,
    Bool,
    Text,
}

fn scalar(v: &serde_json::Value, kind: Kind) -> Option<Value> {
    match kind {
        Kind::Int => v.as_i64().map(Value::from),
        // Tamsayı olarak gelen ondalık (ör. `0`) de kabul.
        Kind::Float => v.as_f64().filter(|f| f.is_finite()).map(Value::from),
        Kind::Bool => v.as_bool().map(Value::from),
        // Boş metin = libinput varsayılanı: etkin değer bilinmiyor sayılır.
        Kind::Text => v.as_str().filter(|s| !s.is_empty() && s.len() <= 32 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')).map(Value::from),
    }
}

/// Yanıtı ayrıştırır; sürüm uymuyor ya da sözleşme dışıysa `None`.
pub fn parse_status(text: &str) -> Option<InputStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    if o.get("v")?.as_u64()? != VERSION {
        return None;
    }
    let available = o.get("available")?.as_bool()?;
    let mut s = InputStatus { available, ..Default::default() };
    s.managed = o
        .get("managed")
        .and_then(|x| x.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str()).filter(|x| x.starts_with("input.") && x.len() <= 64).take(64).map(str::to_owned).collect())
        .unwrap_or_default();
    s.overrides = o
        .get("overrides")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| {
                    let x = x.as_object()?;
                    let key = x.get("key")?.as_str().filter(|k| k.starts_with("input.") && k.len() <= 64)?;
                    let file = x.get("file").and_then(|f| f.as_str()).filter(|f| !f.is_empty()).map(|f| f.chars().take(256).collect());
                    let hypr = x.get("hypr").and_then(|f| f.as_str()).unwrap_or("").chars().take(64).collect();
                    let line = x.get("line").and_then(|l| l.as_u64()).filter(|l| *l > 0);
                    Some(Override { key: key.to_owned(), hypr, file, line })
                })
                .take(MAX_OVERRIDES)
                .collect()
        })
        .unwrap_or_default();
    if !available {
        return Some(s);
    }
    let kb = o.get("keyboard");
    s.layouts = parse_layouts(kb.and_then(|k| k.get("layouts")));
    s.options = parse_options(kb.and_then(|k| k.get("options")));
    for (section, field, kind) in FIELDS {
        if let Some(v) = o.get(*section).and_then(|x| x.get(*field)).and_then(|x| scalar(x, *kind)) {
            s.values.insert(format!("input.{section}.{field}"), v);
        }
    }
    Some(s)
}

/// Bir bağlantıdan durumu sorar (saf; testte sahte bağlantı).
pub fn query_status(link: &dyn ShellLink) -> InputState {
    match link.request(&["input", "status"]) {
        Ok(reply) => parse_status(&reply).map_or(InputState::Unsupported, |s| InputState::Ready(Arc::new(s))),
        Err(LinkError::NotRunning) => InputState::NotRunning,
        Err(LinkError::TestInstance) => InputState::TestInstance,
        Err(LinkError::Shell(_) | LinkError::BadReply) => InputState::Unsupported,
        Err(e) => InputState::Failed(e.to_string()),
    }
}

/// Sayfanın üstündeki sakin not (değerler neden varsayılanla gösteriliyor); durum hazırsa ya da
/// henüz sorulmadıysa `None`.
pub fn status_note(state: &InputState) -> Option<String> {
    match state {
        InputState::Pending => None,
        InputState::Ready(s) if s.available => None,
        InputState::Ready(_) => Some(t!("settings-input-unavailable")),
        InputState::NotRunning | InputState::TestInstance | InputState::Failed(_) => {
            Some(t!("settings-input-not-running"))
        }
        InputState::Unsupported => Some(t!("settings-input-unsupported")),
    }
}

/// Satırın geçersiz kılma uyarısı. Dosya bilinmiyorsa (Lua config; shell etkin değeri
/// `getoption` ile karşılaştırıp farkı gördü) dosya yerine aranacak Hyprland anahtarı söylenir.
pub fn override_note(o: &Override) -> String {
    match (&o.file, o.line) {
        (Some(file), Some(line)) => t!("settings-input-override-file-line", file = file.as_str(), line = line),
        (Some(file), None) => t!("settings-input-override-file", file = file.as_str()),
        (None, _) if !o.hypr.is_empty() => t!("settings-input-override-hypr", key = o.hypr.as_str()),
        (None, _) => t!("settings-input-override"),
    }
}

// ---- Süreç geneli durum ---------------------------------------------------------------------

static STATUS: Mutex<InputState> = Mutex::new(InputState::Pending);
/// (çalışan sorgu var mı, bittiğinde bir kez daha sor)
static QUERY: Mutex<(bool, bool)> = Mutex::new((false, false));
/// Bu süreçte durum en az bir kez soruldu mu (Klavye ya da Fare sayfası açıldı).
static ASKED: AtomicBool = AtomicBool::new(false);

/// Son durum (sormaz).
pub fn status() -> InputState {
    STATUS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Hiç sorulmadıysa arka planda bir kez sorar (sayfa ilk kurulurken).
pub fn ensure() {
    if !ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Anahtarın etkin değeri (yalnızca Hyprland'den okunmuş durumda).
pub fn effective(path: &str) -> Option<Value> {
    match &*STATUS.lock().unwrap_or_else(|e| e.into_inner()) {
        InputState::Ready(s) if s.available => s.values.get(path).cloned(),
        _ => None,
    }
}

/// Pencere odağı geri geldi: durum daha önce sorulduysa yeniden sorar (kullanıcı Hyprland
/// dosyasını elle düzenleyip dönmüş olabilir). Yoklama yok.
pub fn recheck() {
    if ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Durumu `delay` sonra sorar (shell `shell.toml` değişimini uygulasın diye beklenir); bitince
/// [`Event::Input`](super::Event::Input). Sorgu sürüyorsa bittiğinde bir kez daha sorulur.
pub fn refresh(delay: Duration) {
    // Birim testleri shell'e gitmez (ve arka plan thread'i açmaz).
    if cfg!(test) {
        return;
    }
    ASKED.store(true, Ordering::Relaxed);
    let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
    if q.0 {
        q.1 = true;
        return;
    }
    q.0 = true;
    let spawned = std::thread::Builder::new().name("input-status".into()).spawn(move || {
        let link = IpcLink::from_env();
        let mut delay = delay;
        loop {
            std::thread::sleep(delay);
            let state = query_status(&link);
            *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = state;
            super::emit(super::Event::Input);
            let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
            if q.1 {
                q.1 = false;
                delay = Duration::from_millis(300);
                continue;
            }
            q.0 = false;
            break;
        }
    });
    if spawned.is_err() {
        q.0 = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::shell_ipc::FakeLink;

    pub const SAMPLE: &str = r#"{ "v": 1, "available": true,
      "keyboard": { "layouts": [{"layout":"tr","variant":""}], "options": ["caps:escape"],
                    "repeat_rate": 70, "repeat_delay": 200, "numlock_by_default": true },
      "mouse":    { "sensitivity": 0, "accel_profile": "", "natural_scroll": false },
      "touchpad": { "natural_scroll": true, "tap_to_click": true, "disable_while_typing": true,
                    "scroll_factor": 1.0, "scroll_method": "" },
      "managed":  ["input.keyboard.layouts"],
      "overrides": [{"key": "input.keyboard.repeat_rate", "hypr": "input:repeat_rate", "file": "~/.config/hypr/hyprland.conf", "line": 12}] }"#;

    #[test]
    fn parses_contract_sample() {
        crate::i18n::use_turkish_for_tests();
        let s = parse_status(SAMPLE).unwrap();
        assert!(s.available);
        assert_eq!(s.layouts, Some(vec![KbLayout::new("tr", "")]));
        assert_eq!(s.options, Some(vec!["caps:escape".to_owned()]));
        assert_eq!(s.values.get("input.keyboard.repeat_rate").and_then(Value::as_integer), Some(70));
        assert_eq!(s.values.get("input.keyboard.repeat_delay").and_then(Value::as_integer), Some(200));
        assert_eq!(s.values.get("input.keyboard.numlock_by_default").and_then(Value::as_bool), Some(true));
        assert_eq!(s.values.get("input.touchpad.natural_scroll").and_then(Value::as_bool), Some(true));
        // Tamsayı gibi yazılmış ondalık kabul; boş metin (libinput varsayılanı) etkin değer değil.
        assert_eq!(s.values.get("input.mouse.sensitivity").and_then(Value::as_float), Some(0.0));
        assert!(!s.values.contains_key("input.mouse.accel_profile"));
        assert!(!s.values.contains_key("input.touchpad.scroll_method"));
        assert_eq!(s.managed, vec!["input.keyboard.layouts"]);
        let o = s.override_for("input.keyboard.repeat_rate").unwrap();
        assert_eq!((o.hypr.as_str(), o.line), ("input:repeat_rate", Some(12)));
        assert_eq!(override_note(o), "Hyprland ayar dosyan bu değeri geçersiz kılıyor: ~/.config/hypr/hyprland.conf:12");
        assert!(s.override_for("input.keyboard.repeat_delay").is_none());
    }

    #[test]
    fn override_without_file_has_meaningful_note() {
        crate::i18n::use_turkish_for_tests();
        // Shell'in getoption yedeği (Lua kullanıcısı): dosya/satır null ya da hiç yok.
        let text = r#"{"v":1,"available":true,"managed":["input.mouse.sensitivity","input.keyboard.repeat_delay","input.touchpad.scroll_factor"],
          "overrides":[{"key":"input.mouse.sensitivity","hypr":"input:sensitivity","file":null,"line":null},
                       {"key":"input.keyboard.repeat_delay","hypr":"input:repeat_delay"},
                       {"key":"input.touchpad.scroll_factor","hypr":"","file":"~/.config/hypr/hyprland.lua","line":null}]}"#;
        let s = parse_status(text).unwrap();
        let o = s.override_for("input.mouse.sensitivity").expect("null dosyalı kayıt atılmaz");
        assert_eq!((o.file.as_deref(), o.line), (None, None));
        let note = override_note(o);
        assert_eq!(note, "Hyprland ayarın bu değeri geçersiz kılıyor (input:sensitivity).");
        assert_eq!(override_note(s.override_for("input.keyboard.repeat_delay").unwrap()), "Hyprland ayarın bu değeri geçersiz kılıyor (input:repeat_delay).");
        assert_eq!(
            override_note(s.override_for("input.touchpad.scroll_factor").unwrap()),
            "Hyprland ayar dosyan bu değeri geçersiz kılıyor: ~/.config/hypr/hyprland.lua"
        );
        let bare = Override { key: "input.mouse.natural_scroll".into(), hypr: String::new(), file: None, line: None };
        assert_eq!(override_note(&bare), "Hyprland ayarın bu değeri geçersiz kılıyor.");
        // Boş dosya ve sıfır satır da "yok" sayılır.
        let s = parse_status(r#"{"v":1,"available":true,"overrides":[{"key":"input.mouse.sensitivity","hypr":"input:sensitivity","file":"","line":0}]}"#).unwrap();
        assert_eq!((s.overrides[0].file.as_deref(), s.overrides[0].line), (None, None));
    }

    #[test]
    fn unavailable_has_no_values_and_rejects_bad_input() {
        let s = parse_status(r#"{"v":1,"available":false,"keyboard":null,"mouse":null,"touchpad":null,"managed":[],"overrides":[]}"#).unwrap();
        assert!(!s.available && s.values.is_empty() && s.layouts.is_none());
        assert_eq!(parse_status(r#"{"v":2,"available":true}"#), None, "bilinmeyen sürüm");
        assert_eq!(parse_status("ok"), None);
        // Geçersiz düzen adı: liste hiç kabul edilmez; geçersiz seçenek atlanır.
        let bad = r#"{"v":1,"available":true,"keyboard":{"layouts":[{"layout":"tr;rm","variant":""}],"options":["grp:alt_shift_toggle","kötü seçenek",""]}}"#;
        let s = parse_status(bad).unwrap();
        assert_eq!(s.layouts, None);
        assert_eq!(s.options, Some(vec!["grp:alt_shift_toggle".to_owned()]));
        // Tipi uymayan değer yok sayılır.
        let s = parse_status(r#"{"v":1,"available":true,"keyboard":{"repeat_rate":"çok"}}"#).unwrap();
        assert!(s.values.is_empty());
    }

    #[test]
    fn option_validation() {
        for ok in ["grp:alt_shift_toggle", "caps:escape", "compose:ralt", "lv3:ralt_switch", "grp:win_space_toggle"] {
            assert!(valid_option(ok), "{ok}");
        }
        for bad in ["", "grp", ":x", "grp:", "Grp:x", "grp:a b", "grp:x,y", "grp:x;y"] {
            assert!(!valid_option(bad), "{bad}");
        }
        assert!(is_switch_option("grp:alt_shift_toggle") && !is_switch_option("caps:escape"));
    }

    #[test]
    fn query_uses_input_status_and_classifies_errors() {
        crate::i18n::use_turkish_for_tests();
        let link = FakeLink::with([
            Ok(SAMPLE.to_owned()),
            Ok("error: bilinmeyen komut".to_owned()),
            Err(LinkError::NotRunning),
            Err(LinkError::TestInstance),
            Err(LinkError::Timeout),
        ]);
        let ready = query_status(&link);
        assert!(ready.ready().is_some());
        assert_eq!(status_note(&ready), None);
        let unsupported = query_status(&link);
        assert_eq!(unsupported, InputState::Unsupported);
        assert!(status_note(&unsupported).is_some());
        let not_running = query_status(&link);
        assert_eq!(not_running, InputState::NotRunning);
        assert!(status_note(&not_running).unwrap().contains("çalışmıyor"));
        assert_eq!(query_status(&link), InputState::TestInstance);
        assert!(matches!(query_status(&link), InputState::Failed(_)));
        assert!(link.sent().iter().all(|a| a == &["input", "status"]));
        assert_eq!(status_note(&InputState::Pending), None, "sorulurken not yok");
        let off = InputState::Ready(Arc::new(InputStatus::default()));
        assert!(off.ready().is_none() && status_note(&off).is_some(), "available:false");
    }
}
