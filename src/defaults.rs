//! Varsayılan değerler — Ayarlar'da kopya tutulmaz; tek kaynak config'in sahibidir.
//!
//! - framework.toml: `nuxut_theme::FrameworkConfig::default()` serileştirilir; renk
//!   rolleri `Theme::default().colors`'tan (framework config'te renkler opsiyoneldir).
//! - palette.toml `[meta]`: `nuxut_palette` varsayılan varyant/kip.
//! - shell.toml: `nuxut-shell schema` yüklendiyse onun varsayılanları (shell sürümüyle değişir),
//!   yoksa shell'in şema belgesindeki yedek tablo ([`SHELL_DEFAULTS`]). Shell'in
//!   config tipi ayrı bir crate'e çıkınca buradan silinip oradan okunacak (bkz.
//!   ARCHITECTURE §9).
//!
//! "Varsayılana dön" anahtarı siler; bu tablolar yalnızca *gösterim* içindir.

use std::sync::OnceLock;

use nuxut::Theme;
use nuxut::theme::{FrameworkConfig, MotionPreset, MotionStyle};
use toml_edit::{DocumentMut, Item, TableLike, Value};

use crate::store::{FileId, KeyPath, Store};

/// Shell'in `docs/ARCHITECTURE.md` "shell.toml şeması" varsayılanları (shell f818523; `nuxut-shell schema` ile aynı).
/// `[compositor]`, `[windows]`, `[keybinds]`: shell 6b5e8c9 `hypr_config/{settings,keybinds}.rs`.
/// `[night_light]`: shell `night_light` (8d3deba) `NightLightConfig::default`.
/// `[idle]`: shell `services/idle/config.rs` `IdleConfig::default` (7b3c80e: lock_before_sleep).
/// `[lock]`: shell `services/lock_launcher.rs` `LockConfig::default`.
/// `[magnifier]`, `[osk]`: shell docs/design/magnifier.md ve osk.md "Ayarlar sözleşmesi" (since 2).
/// `[input.*]`: sözleşme girdi-v1 şema `default`ları (Hyprland varsayılanı; yalnızca gösterim — shell
/// yazılmamış anahtar için satır üretmez, kullanıcının Hyprland değeri geçerli kalır).
pub const SHELL_DEFAULTS: &str = r#"
[frame]
enabled = true
thickness = 6.0

[bar]
edge = "top"
size = 34.0
padding = 10.0
spacing = 12.0
start = ["workspaces"]
center = ["clock"]
end = ["tray", "notifications", "session"]

[bar.edge_actions]
scroll_step = 5

[bar.edge_actions.start]
left_click = "widget"
right_click = "widget"
scroll = "widget"

[bar.edge_actions.end]
left_click = "widget"
right_click = "widget"
scroll = "widget"

[bar.widget.workspaces]
count = 10
style = "icons"
scroll = "all"

[bar.widget.clock]
# "" = auto: [locale] hour_cycle (shell docs/design/clock-locale.md)
format = ""
vertical_format = ""

[bar.widget.active_window]
max_width = 360.0

[bar.widget.tray]
icon_size = 18.0

[bar.widget.session]
panel = "right"

[bar.widget.notifications]
show_zero = false
panel = "right"

[bar.widget.panel-toggle]
panel = "right"
icon = "menu"

[panels.right]
enabled = true
size = 380.0
widgets = ["session", "volume", "brightness", "network", "bluetooth", "power", "media", "notifications"]

[panels.left]
enabled = false
size = 340.0
widgets = []

[panels.top]
enabled = false
size = 320.0
widgets = []

[panels.bottom]
enabled = false
size = 320.0
widgets = []

[wallpaper]
path = ""
fit = "cover"
# shell 88885da: letterbox zemini.
backdrop = "blur"
backdrop_blur = 48
backdrop_color = "surface"

[audio]
max_volume = 100

[osd]
timeout_ms = 1200

[notifications]
enabled = true
dnd = false
popup_timeout_ms = 5000
max_popups = 4
history_limit = 100

[compositor]
ensure_source = true
sync_look = true
sync_motion = true
sync_windows = true
sync_keybinds = true
sync_input = true
base_layer = true
take_over_keys = true

[windows]
gap_inner = 4
gap_outer = 8
border = 2
blur = true
blur_size = 8

[input.keyboard]
repeat_rate = 25
repeat_delay = 600
numlock_by_default = false

[input.mouse]
sensitivity = 0.0
accel_profile = "adaptive"
natural_scroll = false

[input.touchpad]
natural_scroll = false
tap_to_click = true
disable_while_typing = true
scroll_factor = 1.0
scroll_method = "2fg"

[night_light]
enabled = false
temperature = 4000
schedule = "sun"
from = "20:00"
to = "07:00"
transition = 30

[idle]
enabled = true
dim = false
dim_after = 300
screen_off = false
screen_off_after = 600
lock = false
lock_after = 600
lock_before_sleep = true
suspend = false
suspend_after = 1800
suspend_on_ac = false
media_inhibits = true

[lock]
listen_logind = true

[magnifier]
step = 1.25
max = 8.0
factor = 2.0

[osk]
layout = "auto"
key_height = 48

[keybinds]
volume_up = "XF86AudioRaiseVolume"
volume_down = "XF86AudioLowerVolume"
volume_mute = "XF86AudioMute"
mic_mute = "XF86AudioMicMute"
brightness_up = "XF86MonBrightnessUp"
brightness_down = "XF86MonBrightnessDown"
media_play = "XF86AudioPlay"
media_pause = "XF86AudioPause"
media_next = "XF86AudioNext"
media_prev = "XF86AudioPrev"
panel_right = ""
panel_left = ""
panel_clock = ""
bar_toggle = ""
bar_focus = "CTRL+ALT+Tab"
shot_region = "Print"
shot_output = "SHIFT+Print"
shot_window = "ALT+Print"
clipboard = "SUPER+SHIFT+V"
magnifier_in = "SUPER+ALT+Up"
magnifier_out = "SUPER+ALT+Down"
magnifier_reset = "SUPER+ALT+Escape"
emoji = "SUPER+period"
osk_toggle = "SUPER+ALT+K"
launcher = "SUPER"
settings = "SUPER+I"
terminal = "CTRL+ALT+T"
file_manager = "SUPER+E"
browser = "SUPER+B"
lock = "SUPER+L"
session_menu = "CTRL+ALT+Delete"
close_window = "SUPER+Q"
close_window_alt = "ALT+F4"
force_kill = "CTRL+ALT+Escape"
fullscreen = "SUPER+F"
maximize = "SUPER+M"
toggle_floating = "SUPER+SHIFT+space"
pin = "SUPER+P"
cycle_windows = "ALT+Tab"
cycle_windows_back = "ALT+SHIFT+Tab"
toggle_split = "SUPER+J"
focus_left = "SUPER+Left"
focus_right = "SUPER+Right"
focus_up = "SUPER+Up"
focus_down = "SUPER+Down"
move_left = "SUPER+SHIFT+Left"
move_right = "SUPER+SHIFT+Right"
move_up = "SUPER+SHIFT+Up"
move_down = "SUPER+SHIFT+Down"
shrink_width = "SUPER+CTRL+Left"
grow_width = "SUPER+CTRL+Right"
shrink_height = "SUPER+CTRL+Up"
grow_height = "SUPER+CTRL+Down"
workspace = "SUPER"
move_to_workspace = "SUPER+SHIFT"
workspace_next = "SUPER+Page_Down"
workspace_prev = "SUPER+Page_Up"
move_to_workspace_next = "SUPER+SHIFT+Page_Down"
move_to_workspace_prev = "SUPER+SHIFT+Page_Up"
workspace_last = "SUPER+Tab"
scratchpad = "SUPER+S"
move_to_scratchpad = "SUPER+SHIFT+S"
window_drag = "SUPER+mouse:272"
window_resize_drag = "SUPER+mouse:273"
workspace_scroll_next = "SUPER+mouse_down"
workspace_scroll_prev = "SUPER+mouse_up"
"#;

/// İmleç boyu sınırları: framework `[desktop] cursor_size` şemasından (kopya yok).
pub const DESKTOP_CURSOR_MIN: f64 = nuxut::theme::CURSOR_SIZE_MIN as f64;
pub const DESKTOP_CURSOR_MAX: f64 = nuxut::theme::CURSOR_SIZE_MAX as f64;

fn framework_doc(preset: MotionPreset) -> DocumentMut {
    let mut doc = toml_edit::ser::to_document(&FrameworkConfig::default()).unwrap_or_default();
    // Renk rolleri config'te yazılmadığında temanın varsayılan paletinden gelir.
    let colors = Theme::default().colors;
    let table = doc.entry("colors").or_insert(Item::Table(toml_edit::Table::new()));
    if let Some(t) = table.as_table_like_mut() {
        for (name, c) in palette_roles(&colors) {
            t.insert(name, Item::Value(Value::from(c.to_hex())));
        }
    }
    // Tipografi rolleri config'te opsiyoneldir; varsayılanları çözülmüş temadan.
    let typo = Theme::default().typography;
    let table = doc.entry("typography").or_insert(Item::Table(toml_edit::Table::new()));
    if let Some(t) = table.as_table_like_mut() {
        for (name, role) in font_roles(&typo) {
            let mut r = toml_edit::Table::new();
            r.insert("family", Item::Value(Value::from(role.family.as_str())));
            r.insert("weight", Item::Value(Value::from(i64::from(role.weight))));
            r.insert("scale", Item::Value(Value::from(f64::from(role.scale))));
            t.insert(name, Item::Table(r));
        }
    }
    // `[appearance]` varsayılanı dosyaya serileştirilmez (`skip_serializing_if`); varsayılanlar
    // yine şemadan (`AppearanceConfig::default()`), kopya yok. Şeması bölümü tanımayan
    // framework'te bu satır derlenmez → "Fiyu'yu gizle" o sürümle birlikte gelir.
    if !doc.contains_key("appearance")
        && let Ok(a) = toml_edit::ser::to_document(&nuxut::theme::AppearanceConfig::default())
    {
        doc.insert("appearance", Item::Table(a.as_table().clone()));
    }
    // Hareket rolleri, stil ve esneme: dosyada yoksa seçili ön ayarın değeri.
    let v = preset.values();
    let style = match v.style {
        MotionStyle::Spring => "spring",
        MotionStyle::Curve => "curve",
    };
    let motion = doc.entry("motion").or_insert(Item::Table(toml_edit::Table::new()));
    if let Some(t) = motion.as_table_like_mut() {
        t.insert("preset", Item::Value(Value::from(MotionPreset::default().name())));
        t.insert("style", Item::Value(Value::from(style)));
        for (name, r) in [("spatial", v.spatial), ("spatial_fast", v.spatial_fast), ("effects", v.effects)] {
            let mut role = toml_edit::Table::new();
            role.insert("response", Item::Value(Value::from(f64::from(r.response))));
            role.insert("damping", Item::Value(Value::from(f64::from(r.damping))));
            role.insert("easing", Item::Value(Value::from(r.easing)));
            role.insert("duration_ms", Item::Value(Value::from(f64::from(r.duration_ms))));
            t.insert(name, Item::Table(role));
        }
    }
    let blob = doc.entry("blob").or_insert(Item::Table(toml_edit::Table::new()));
    if let Some(t) = blob.as_table_like_mut() {
        t.insert("deform", Item::Value(Value::from(f64::from(v.deform))));
    }
    doc
}

/// Temanın tipografi rolleri (ad → rol). Adlar framework.toml `[typography.<ad>]` tablolarıdır.
pub fn font_roles(t: &nuxut::theme::Typography) -> [(&'static str, &nuxut::theme::FontRole); 5] {
    [("ui", &t.ui), ("display", &t.display), ("document", &t.document), ("mono", &t.mono), ("icon", &t.icon)]
}

/// Temanın renk rolleri (ad → renk). Adlar framework.toml `[colors]` anahtarlarıdır.
pub fn palette_roles(p: &nuxut::theme::Palette) -> [(&'static str, nuxut::Color); 11] {
    [
        ("surface", p.surface),
        ("surface_container", p.surface_container),
        ("surface_bright", p.surface_bright),
        ("on_surface", p.on_surface),
        ("on_surface_variant", p.on_surface_variant),
        ("primary", p.primary),
        ("on_primary", p.on_primary),
        ("error", p.error),
        ("on_error", p.on_error),
        ("outline", p.outline),
        ("shadow", p.shadow),
    ]
}

fn palette_doc() -> DocumentMut {
    let mut doc = DocumentMut::new();
    let mut meta = toml_edit::Table::new();
    meta.insert("scheme", Item::Value(Value::from(nuxut_palette::Variant::default().name())));
    meta.insert("mode", Item::Value(Value::from(nuxut_palette::Mode::default().name())));
    meta.insert("contrast", Item::Value(Value::from(0.0)));
    doc.insert("meta", Item::Table(meta));
    doc
}

fn doc(file: FileId, preset: MotionPreset) -> &'static DocumentMut {
    static FRAMEWORK: [OnceLock<DocumentMut>; 4] = [const { OnceLock::new() }; 4];
    static SHELL: OnceLock<DocumentMut> = OnceLock::new();
    static PALETTE: OnceLock<DocumentMut> = OnceLock::new();
    match file {
        FileId::Framework => {
            let i = MotionPreset::ALL.iter().position(|p| *p == preset).unwrap_or(0);
            FRAMEWORK[i].get_or_init(|| framework_doc(preset))
        }
        FileId::Shell => SHELL.get_or_init(|| SHELL_DEFAULTS.parse().unwrap_or_default()),
        FileId::Palette => PALETTE.get_or_init(palette_doc),
    }
}

/// Dosyadaki hareket ön ayarı (yoksa ya da bilinmiyorsa varsayılan).
pub fn preset(store: &Store) -> MotionPreset {
    store
        .get(FileId::Framework, &KeyPath::parse("motion.preset"))
        .and_then(Value::as_str)
        .and_then(MotionPreset::from_name)
        .unwrap_or_default()
}

/// Anahtarın varsayılan değeri (yoksa `None`: ör. kenar başına opsiyonel kalınlık). Hareket
/// rolleri, stil ve `blob.deform` için dosyadaki ön ayarınki.
pub fn get(store: &Store, file: FileId, path: &KeyPath) -> Option<&'static Value> {
    // framework.toml var ama bozuksa çalışma zamanı `Theme::broken_config()`'i kurar
    // (birleşik görünüm kapalı); Ayarlar da aynı değeri gösterir.
    if file == FileId::Framework && store.file(FileId::Framework).broken().is_some() && path.segments() == ["desktop", "unified"] {
        static BROKEN_UNIFIED: OnceLock<Value> = OnceLock::new();
        return Some(BROKEN_UNIFIED.get_or_init(|| Value::from(Theme::broken_config().desktop.unified)));
    }
    get_for(preset(store), file, path)
}

/// [`get`], belirli bir ön ayarla.
pub fn get_for(preset: MotionPreset, file: FileId, path: &KeyPath) -> Option<&'static Value> {
    // Shell: önce çalışan shell'in şeması (varsayılanlar sürümle değişebilir), sonra yedek tablo.
    if file == FileId::Shell
        && let Some(v) = crate::services::widgets::shell_default(path)
    {
        return Some(v);
    }
    let (last, parents) = path.segments().split_last()?;
    let mut table: &dyn TableLike = doc(file, preset).as_table();
    for seg in parents {
        table = table.get(seg)?.as_table_like()?;
    }
    table.get(last)?.as_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framework_defaults_come_from_theme() {
        let t = Theme::default();
        let v = get_for(MotionPreset::Calm, FileId::Framework, &KeyPath::parse("colors.primary")).and_then(Value::as_str).unwrap();
        assert_eq!(v, t.colors.primary.to_hex());
        let r = get_for(MotionPreset::Calm, FileId::Framework, &KeyPath::parse("shape.radius_medium")).and_then(Value::as_float).unwrap();
        assert!((r as f32 - t.shape.radius_medium).abs() < 1e-4);
        assert!(get_for(MotionPreset::Calm, FileId::Framework, &KeyPath::parse("motion.spatial.damping")).is_some());
        assert!(get_for(MotionPreset::Calm, FileId::Framework, &KeyPath::parse("motion.speed")).is_some());
    }


    #[test]
    fn motion_defaults_follow_preset() {
        let num = |p, k: &str| get_for(p, FileId::Framework, &KeyPath::parse(k)).and_then(Value::as_float).unwrap();
        for p in MotionPreset::ALL {
            let v = p.values();
            assert!((num(p, "motion.spatial.damping") as f32 - v.spatial.damping).abs() < 1e-6, "{}", p.name());
            assert!((num(p, "blob.deform") as f32 - v.deform).abs() < 1e-6, "{}", p.name());
        }
        assert_ne!(num(MotionPreset::Calm, "motion.spatial.damping"), num(MotionPreset::Lively, "motion.spatial.damping"));
    }
    #[test]
    fn look_defaults_come_from_framework_schema() {
        let s = |k: &str| get_for(MotionPreset::Calm, FileId::Framework, &KeyPath::parse(k)).map(|v| v.to_string().trim().to_owned());
        assert_eq!(s("colors.mode").as_deref(), Some("\"auto\""));
        assert_eq!(s("desktop.qt_backend").as_deref(), Some("\"auto\""));
        // Birleşik görünümün varsayılanı şemadan gelir (kullanıcı kararı: açık; framework
        // `DesktopConfig::default()` değişince Ayarlar ve Hoş geldin kendiliğinden izler).
        let unified = nuxut::theme::FrameworkConfig::default().desktop.unified;
        assert_eq!(s("desktop.unified").as_deref(), Some(if unified { "true" } else { "false" }));
        assert_eq!(s("desktop.cursor_size").as_deref(), Some("24"));
    }

    /// Bozuk framework.toml: çalışma zamanı `Theme::broken_config()` kurar (birleşik görünüm
    /// kapalı); Ayarlar'ın gösterdiği varsayılan da o. Dosya yoksa şema varsayılanı.
    #[test]
    fn broken_framework_file_shows_unified_off() {
        let dir = tempfile::tempdir().unwrap();
        let fw = dir.path().join("f.toml");
        let unified = KeyPath::parse("desktop.unified");
        let store = Store::open(&fw, dir.path().join("s.toml"), dir.path().join("p.toml"));
        let schema = FrameworkConfig::default().desktop.unified;
        assert_eq!(get(&store, FileId::Framework, &unified).and_then(Value::as_bool), Some(schema));
        std::fs::write(&fw, "[desktop\nunified = ").unwrap();
        let store = Store::open(&fw, dir.path().join("s.toml"), dir.path().join("p.toml"));
        assert!(store.file(FileId::Framework).broken().is_some());
        assert_eq!(get(&store, FileId::Framework, &unified).and_then(Value::as_bool), Some(Theme::broken_config().desktop.unified));
        assert!(!Theme::broken_config().desktop.unified);
    }

    #[test]
    fn shell_and_palette_defaults_parse() {
        assert_eq!(get_for(MotionPreset::Calm, FileId::Shell, &KeyPath::parse("bar.edge")).and_then(Value::as_str), Some("top"));
        assert!(get_for(MotionPreset::Calm, FileId::Shell, &KeyPath::parse("frame.top")).is_none());
        assert_eq!(get_for(MotionPreset::Calm, FileId::Palette, &KeyPath::parse("meta.scheme")).and_then(Value::as_str), Some("tonal-spot"));
    }
}
