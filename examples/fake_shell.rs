//! Canlı testler için sahte shell `msg` sunucusu: `look status` (sürüm 4: katalog + hedefli eksik
//! paketler + `themes_rev`), `snapshot list|restore`, `setup` sorularına sabit yanıt verir. YALNIZCA
//! `NUXUT_SHELL_INSTANCE` ile (ör. `stlook`) çalışır —
//! adsız `nuxut-shell` soketini asla almaz, kullanıcının shell'inin yerine geçemez.
//!
//! `input status` (girdi-v1): kullanıcı benzeri etkin değerler (tr, tekrar 70/200, Num Lock açık,
//! dokunmatik yüzeyde doğal kaydırma); `NUXUT_FAKE_INPUT_OVERRIDES=1` ise tekrar hızı ve düzenler
//! kullanıcının Hyprland dosyasında geçersiz kılınmış gibi döner; işaretçi hızı ise dosyasız
//! (`file`/`line` null: Lua config, shell'in `getoption` yedeği) geçersiz kılınmış döner.
//!
//! `defaults status` / `defaults set` (defaults-v1): kullanıcı benzeri dokuz rol; durumlar
//! `NUXUT_FAKE_DEFAULTS` ile (`mixed`, `shadowed`, `noterm`, `nulls`, `gone`, `unsupported`,
//! `readonly`, `failwrite`; bkz.
//! [`defaults_status`]). `set` seçimi süreç içinde değiştirir, snapshot kimliği döner.
//!
//! `fake_shell schema`: `NUXUT_FAKE_SCHEMA` dosyasındaki (gerçek bir shell'in `schema` çıktısı)
//! şemaya girdi-v1 ve gece ışığı (`[night_light]`) anahtarlarını ekleyip basar — Ayarlar
//! `NUXUT_SHELL_BIN` ile bunu okur. (Temel şema gece ışığını zaten içeriyorsa satırlar iki kez
//! olur; son gelen geçerli.)
//!
//! `night-light status`: kip `NUXUT_FAKE_NIGHT` (bkz. [`night_status`]).
//!
//! `NUXUT_FAKE_LOOK=external`: `look status` sürüm 5, `last_apply.external` iki dosyayla (eksik paket yok).
//!
//! `magnifier status|on|off`, `osk status|show|hide` (magnifier-v1, osk-v1): süreç içi durum;
//! `NUXUT_FAKE_ASSIST=unsupported` iken ikisi de `available:false`, eylemler hata.
//!
//! `NUXUT_SHELL_INSTANCE=stlook cargo run --example fake_shell`

use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

const STATUS: &str = r#"{"version":4,"themes_rev":0,"unified":true,"mode":"dark","wallpaper_colors":false,"icons":{"requested":"Papirus","current":"Adwaita","fallback":true,"recommended":"Papirus","explicit":false,"installed":[],"catalog":[{"family":"papirus","name":"Papirus","package":"papirus-icon-theme","source":"repo","installed":[]},{"family":"breeze","name":"Breeze","package":"breeze-icons","source":"repo","installed":["breeze","breeze-dark","breeze-plus","breeze-plus-dark"]},{"family":"tela-circle","name":"Tela Circle","package":"tela-circle-icon-theme-git","source":"aur","installed":[]},{"family":"colloid","name":"Colloid","package":"colloid-icon-theme-git","source":"aur","installed":[]},{"family":"qogir","name":"Qogir","package":"qogir-icon-theme","source":"aur","installed":[]},{"family":"kora","name":"Kora","package":"kora-icon-theme","source":"aur","installed":[]},{"family":"fluent","name":"Fluent","package":"fluent-icon-theme-git","source":"aur","installed":[]},{"family":"whitesur","name":"WhiteSur","package":"whitesur-icon-theme","source":"aur","installed":[]}]},"cursors":{"requested":"Vimix-cursors","current":"Vimix-cursors","fallback":false,"recommended":"Vimix-cursors","explicit":false,"size":24,"installed":[],"catalog":[{"family":"vimix","name":"Vimix","package":"vimix-cursors","source":"repo","installed":["Vimix-cursors","Vimix-white-cursors"]},{"family":"capitaine","name":"Capitaine","package":"capitaine-cursors","source":"repo","installed":[]},{"family":"material","name":"Material","package":"material-cursors-git","source":"aur","installed":[]},{"family":"oreo","name":"Oreo","package":"oreo-cursors-bin","source":"aur","installed":[]},{"family":"volantes","name":"Volantes","package":"volantes-cursors","source":"aur","installed":[]},{"family":"qogir","name":"Qogir","package":"qogir-cursor-theme","source":"aur","installed":[]},{"family":"sweet","name":"Sweet","package":"sweet-cursors-git","source":"aur","installed":[]},{"family":"bibata","name":"Bibata","package":"bibata-cursor-theme-bin","source":"aur","installed":["Bibata-Modern-Classic"]}]},"gtk_theme":"Adwaita-dark","qt_backend":{"selected":"auto","active":null,"available":[],"style":"Fusion"},"missing_packages":[{"name":"papirus-icon-theme","source":"repo","target":"icons"},{"name":"plasma-integration","source":"repo","target":"qt"}],"conflicts":[],"restart_needed":["gtk3_colors"],"writer":"disabled","last_apply":null,"warnings":[]}"#;
// Yanıt biçimleri `nuxut-apply`'ın gerçek çıktısıyla aynı (framework `nuxut_apply` serde sırası;
// shell ADR-0001 §9.3/§9.5).
const DISPLAYS_LIST: &str = r#"{"v":1,"gen":1,"provider":"lua","pending":null,"primary":"Dell|U2720Q|T1","displays":[{"id":"Dell|U2720Q|T1","connector":"HEADLESS-1","make":"Dell Inc.","model":"Dell U2720Q","description":"Dell Inc. Dell U2720Q","connected":true,"enabled":true,"mode":{"w":2560,"h":1600,"mhz":60000},"pos":[0,0],"scale":2.0,"transform":0,"resolutions":[]}]}"#;
const LIST_ICONS: &str = r#"[{"id":"20261010T120000Z-3fa1","at":"2026-10-10T12:00:00Z","setting":"look.icons","label":"İkon teması","source":"settings","state":"applied","pinned":false,"fragments":3,"conflicts":0}]"#;
const LIST_MODE: &str = r#"[{"id":"20261010T120500Z-0b2c","at":"2026-10-10T12:05:00Z","setting":"look.mode","label":"Renk kipi","source":"welcome","state":"applied","pinned":false,"fragments":2,"conflicts":0}]"#;
/// Varsayılan tarayıcının önceki hâli (defaults-v1 `SettingId` `defaults.browser`).
const LIST_DEFAULTS_BROWSER: &str = r#"[{"id":"20261013T110000Z-d0e0","at":"2026-10-13T11:00:00Z","setting":"defaults.browser","label":"Tarayıcı","source":"settings","state":"applied","pinned":false,"fragments":1,"conflicts":0}]"#;
/// `msg hypr status` (shell `SyncStatus`; eski `backup` alanı yok, bu turda snapshot alınmadı).
const HYPR_STATUS: &str = r#"{"source":"removed_by_user","kind":"hyprlang","config":"~/.config/hypr/hyprland.conf","generated":"~/.config/hypr/nuxut/nuxut.conf","launch":"exec_once","overrides":[],"conflicts":[],"errors":[]}"#;

/// Bilinen kayıtların ayarı (`restore --setting` denetimi için).
fn setting_of(id: &str) -> Option<&'static str> {
    match id {
        "20261010T120000Z-3fa1" => Some("look.icons"),
        "20261010T120500Z-0b2c" => Some("look.mode"),
        "20261013T110000Z-d0e0" => Some("defaults.browser"),
        _ => None,
    }
}

/// `snapshot restore <id> [--force] [--setting <ayar>]`: kayıt yoksa ya da başka ayarınsa
/// `not_found` (shell gibi hiçbir şey yazılmaz).
fn restore_reply(id: &str, flags: &[&str]) -> String {
    let setting = flags.iter().position(|f| *f == "--setting").and_then(|i| flags.get(i + 1)).copied();
    match setting_of(id) {
        Some(s) if setting.is_none_or(|want| want == s) => r#"{"result":"restored","id":"20261010T130000Z-0c3d"}"#.to_owned(),
        _ => r#"{"result":"not_found"}"#.to_owned(),
    }
}

/// `msg input status` (girdi-v1 sürüm 1; Hyprland `getoption` etkin değerleri).
const INPUT_STATUS: &str = r#"{"v":1,"available":true,"keyboard":{"layouts":[{"layout":"tr","variant":""}],"options":["caps:escape"],"repeat_rate":70,"repeat_delay":200,"numlock_by_default":true},"mouse":{"sensitivity":0.0,"accel_profile":"","natural_scroll":false},"touchpad":{"natural_scroll":true,"tap_to_click":true,"disable_while_typing":true,"scroll_factor":1.0,"scroll_method":""},"managed":[],"overrides":[]}"#;
const INPUT_OVERRIDES: &str = r#""overrides":[{"key":"input.keyboard.repeat_rate","hypr":"input:repeat_rate","file":"~/.config/hypr/hyprland.conf","line":12},{"key":"input.keyboard.layouts","hypr":"input:kb_layout","file":"~/.config/hypr/hyprland.conf","line":9},{"key":"input.mouse.sensitivity","hypr":"input:sensitivity","file":null,"line":null}]"#;

/// Girdi-v1 şema anahtarları (shell `schema` çıktısıyla aynı biçim; hepsi `since = 2`).
const INPUT_SCHEMA: &str = r#"
[[setting]]
key = "compositor.sync_input"
title = "Girdi ayarlarını uygula"
kind = "bool"
default = true
since = 2

[[setting]]
key = "input.keyboard.repeat_rate"
title = "Tekrar hızı"
kind = "int"
default = 25
min = 1.0
max = 100.0
unit = "/sn"
since = 2

[[setting]]
key = "input.keyboard.repeat_delay"
title = "Tekrar gecikmesi"
kind = "int"
default = 600
min = 100.0
max = 2000.0
unit = "ms"
since = 2

[[setting]]
key = "input.keyboard.numlock_by_default"
title = "Num Lock"
kind = "bool"
default = false
since = 2

[[setting]]
key = "input.mouse.sensitivity"
title = "Hız"
kind = "float"
default = 0.0
min = -1.0
max = 1.0
since = 2

[[setting]]
key = "input.mouse.accel_profile"
title = "İvme"
kind = "choice"
default = "adaptive"
options = ["adaptive", "flat"]
since = 2

[[setting]]
key = "input.mouse.natural_scroll"
title = "Doğal kaydırma"
kind = "bool"
default = false
since = 2

[[setting]]
key = "input.touchpad.natural_scroll"
title = "Doğal kaydırma"
kind = "bool"
default = false
since = 2

[[setting]]
key = "input.touchpad.tap_to_click"
title = "Dokunarak tıkla"
kind = "bool"
default = true
since = 2

[[setting]]
key = "input.touchpad.disable_while_typing"
title = "Yazarken kapat"
kind = "bool"
default = true
since = 2

[[setting]]
key = "input.touchpad.scroll_factor"
title = "Kaydırma hızı"
kind = "float"
default = 1.0
min = 0.1
max = 3.0
since = 2

[[setting]]
key = "input.touchpad.scroll_method"
title = "Kaydırma yöntemi"
kind = "choice"
default = "2fg"
options = ["2fg", "edge", "on_button_down", "no_scroll"]
since = 2
"#;


/// Gece ışığı şema satırları (shell `night_light` `NightLightConfig::schema` çıktısıyla aynı).
const NIGHT_SCHEMA: &str = r#"
[[setting]]
key = "night_light.enabled"
title = "Gece ışığı"
kind = "bool"
default = false
since = 2
description = "Akşamları ekran renklerini sıcaklaştırır. hyprsunset kullanıyorsan kapalı bırak."

[[setting]]
key = "night_light.temperature"
title = "Gece sıcaklığı"
kind = "int"
default = 4000
min = 1500
max = 6000
step = 100
unit = "K"
since = 2

[[setting]]
key = "night_light.schedule"
title = "Zamanlama"
kind = "choice"
default = "sun"
options = ["always", "manual", "sun"]
option_labels = ["Her zaman", "Belirli saatlerde", "Gün batımından gün doğumuna"]
since = 2

[[setting]]
key = "night_light.from"
title = "Başlangıç saati"
kind = "text"
default = "20:00"
since = 2
description = "SS:DD biçiminde. Bitiş daha erkense gece yarısını aşar."

[[setting]]
key = "night_light.to"
title = "Bitiş saati"
kind = "text"
default = "07:00"
since = 2

[[setting]]
key = "night_light.transition"
title = "Geçiş süresi"
kind = "int"
default = 30
min = 0
max = 120
step = 5
unit = "min"
since = 2
"#;

/// `msg night-light status` (shell `night_light` `status_json`). Kip `NUXUT_FAKE_NIGHT`:
/// `timezone` (varsayılan; İstanbul, 13 Ekim 2026), `manual`, `none`, `polar` (78° K, kutup gecesi),
/// `polar-day`, `old` (eski shell: komut bilinmiyor), `off` (sun dışı zamanlama: konum yok).
fn night_status() -> String {
    let mode = std::env::var("NUXUT_FAKE_NIGHT").unwrap_or_default();
    let (schedule, location, today) = match mode.as_str() {
        "old" => return "error: bilinmeyen komut".to_owned(),
        "manual" => ("sun", r#"{"source":"manual","lat":52.52,"lon":13.40}"#, r#"{"sunrise":1791868500,"sunset":1791908460,"polar":null}"#),
        "none" => ("sun", r#"{"source":"none","lat":null,"lon":null}"#, "null"),
        "polar" => ("sun", r#"{"source":"manual","lat":78.22,"lon":15.65}"#, r#"{"sunrise":null,"sunset":null,"polar":"night"}"#),
        "polar-day" => ("sun", r#"{"source":"manual","lat":-78.0,"lon":-60.0}"#, r#"{"sunrise":null,"sunset":null,"polar":"day"}"#),
        "off" => ("manual", "null", "null"),
        _ => ("sun", r#"{"source":"timezone","lat":41.0,"lon":29.0}"#, r#"{"sunrise":1791864840,"sunset":1791905520,"polar":null}"#),
    };
    format!(
        r#"{{"supported":true,"enabled":true,"schedule":"{schedule}","status":"off","active":false,"temperature":4000,"current":6500,"override":null,"next_change":null,"location":{location},"today":{today}}}"#
    )
}
fn input_status() -> String {
    if std::env::var_os("NUXUT_FAKE_INPUT_OVERRIDES").is_some_and(|v| v == "1") {
        INPUT_STATUS.replacen(r#""overrides":[]"#, INPUT_OVERRIDES, 1)
    } else {
        INPUT_STATUS.to_owned()
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("schema") {
        // Şema soket gerektirmez (Ayarlar `nuxut-shell schema` gibi çalıştırır).
        let base = std::env::var_os("NUXUT_FAKE_SCHEMA").and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_default();
        print!("{base}{INPUT_SCHEMA}{NIGHT_SCHEMA}");
        return;
    }
    let instance = std::env::var("NUXUT_SHELL_INSTANCE").unwrap_or_default();
    if instance.is_empty() || instance.len() > 16 || !instance.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_') {
        eprintln!("fake_shell: NUXUT_SHELL_INSTANCE=[a-z0-9_-]{{1,16}} gerekli (gerçek shell adı alınmaz)");
        std::process::exit(2);
    }
    let Some(runtime) = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).filter(|p| p.is_absolute()) else {
        eprintln!("fake_shell: XDG_RUNTIME_DIR yok");
        std::process::exit(2);
    };
    let dir = runtime.join("nuxut");
    std::fs::create_dir_all(&dir).expect("dizin");
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).expect("izin");
    let path = dir.join(format!("nuxut-shell-{instance}.sock"));
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path).expect("bind");
    println!("FAKE_SHELL_READY");
    // Büyüteç (magnifier-v1) ve ekran klavyesi (osk-v1): süreç içi durum; `NUXUT_FAKE_ASSIST=unsupported`
    // → ikisi de `available:false` (Hyprland yok / sanal klavye protokolü yok).
    let assist_ok = std::env::var("NUXUT_FAKE_ASSIST").as_deref() != Ok("unsupported");
    let (mut zoom, mut osk_visible) = (1.0f64, false);
    for stream in listener.incoming().flatten() {
        // Yazmayan istemci sunucuyu kilitlemesin (gerçek shell gibi 2 sn).
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(2)));
        let mut line = String::new();
        let mut reader = BufReader::new(&stream);
        if reader.read_line(&mut line).is_err() {
            continue;
        }
        let args: Vec<&str> = line.trim_end().split('\t').collect();
        println!("istek: {args:?}");
        let reply = match args.as_slice() {
            ["ping"] => "pong".to_owned(),
            ["look", "status"] => status(),
            ["hypr", "status"] => HYPR_STATUS.to_owned(),
            ["input", "status"] => input_status(),
            ["night-light", "status"] => night_status(),
            ["magnifier", "status"] => magnifier_status(assist_ok, zoom),
            ["magnifier", "on"] if assist_ok => {
                zoom = 2.0;
                "ok".to_owned()
            }
            ["magnifier", "off"] if assist_ok => {
                zoom = 1.0;
                "ok".to_owned()
            }
            ["osk", "status"] => format!(r#"{{"available":{assist_ok},"visible":{},"layout":"tr","output":null}}"#, assist_ok && osk_visible),
            ["osk", "show" | "hide"] if assist_ok => {
                osk_visible = args[1] == "show";
                "ok".to_owned()
            }
            ["magnifier" | "osk", _] => "error: not available".to_owned(),
            ["snapshot", "list", "--setting", "look.icons"] => LIST_ICONS.to_owned(),
            ["snapshot", "list", "--setting", "look.mode"] => LIST_MODE.to_owned(),
            ["snapshot", "list", "--setting", "defaults.browser"] => LIST_DEFAULTS_BROWSER.to_owned(),
            ["snapshot", "list", ..] => "[]".to_owned(),
            // Ekranlar (ADR-0005 §13): test compositor'ın tek çıkışı insan adıyla (Duvar kağıdı ›
            // Monitör başına adları buradan okur).
            ["displays", "status"] => r#"{"v":1,"gen":1,"provider":"lua","available":true,"pending":null,"last":null}"#.to_owned(),
            ["displays", "list"] => DISPLAYS_LIST.to_owned(),
            ["setup", "done" | "skip", _] => "ok".to_owned(),
            ["setup", "status"] => r#"{"first_run":true,"setup_version":null,"setup_state":null,"hypr":{"session":false,"source":"inactive","launch":null},"migration":{"pending_notice":null}}"#.to_owned(),
            ["snapshot", "restore", id, flags @ ..] => restore_reply(id, flags),
            ["defaults", "status"] => defaults_status(),
            ["defaults", "set", role, id] => defaults_set(role, id),
            _ => "error: bilinmeyen komut".to_owned(),
        };
        let mut w = &stream;
        let _ = writeln!(w, "{reply}");
    }
}

/// `look status`; `themes_rev` = `NUXUT_FAKE_THEMES_REV` dosyasındaki sayı (yoksa 0) — test
/// betiği tema kurunca dosyayı artırır (shell'in tema izleyicisi yerine).
fn status() -> String {
    let rev = std::env::var_os("NUXUT_FAKE_THEMES_REV").and_then(|p| std::fs::read_to_string(p).ok()).and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(0);
    let s = STATUS.replacen(r#""themes_rev":0"#, &format!(r#""themes_rev":{rev}"#), 1);
    if std::env::var("NUXUT_FAKE_LOOK").as_deref() != Ok("external") {
        return s;
    }
    // Sürüm 5: yazım etkin, eksik paket yok, iki dosya başka araçta (shell look-external.md).
    s.replacen(r#""version":4"#, r#""version":5"#, 1)
        .replacen(r#""missing_packages":[{"name":"papirus-icon-theme","source":"repo","target":"icons"},{"name":"plasma-integration","source":"repo","target":"qt"}]"#, r#""missing_packages":[]"#, 1)
        .replacen(r#""writer":"disabled","last_apply":null"#, LAST_APPLY_EXTERNAL, 1)
}

const LAST_APPLY_EXTERNAL: &str = r#""writer":"active","last_apply":{"ok":false,"written":["look.icons"],"unchanged":[],"skipped":[],"errors":[],"live":[],"paused":[],"external":[{"setting":"look.mode","path":"~/.config/gtk-3.0/gtk.css","reason":"read_only"},{"setting":"look.icons","path":"~/.config/kdeglobals","reason":"not_owned"}]}"#;

// ---- Varsayılan uygulamalar (defaults-v1) ---------------------------------------------------

/// `defaults status` senaryosu: `NUXUT_FAKE_DEFAULTS` virgüllü bayraklar — `mixed` (resim
/// karışık), `shadowed` (tarayıcı masaüstüne özgü dosyada), `noterm` (xdg-terminal-exec yok),
/// `nulls` (e-posta ve PDF'yi hiçbir şey açmıyor), `gone` (video kurulu olmayan uygulamada),
/// `unsupported` (eski shell: `error: bilinmeyen komut`). Seçimler `defaults set` ile süreç
/// içinde değişir (dosyaya yazılmaz).
fn defaults_flags() -> Vec<String> {
    std::env::var("NUXUT_FAKE_DEFAULTS").unwrap_or_default().split(',').map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()).collect()
}

/// Aday: (kimlik, ad, ikon, NoDisplay).
type Candidate = (&'static str, &'static str, &'static str, bool);
/// Rol başına seçim (rol, kimlik).
type Choices = std::sync::Mutex<Vec<(String, Option<String>)>>;

const DEFAULT_CANDIDATES: &[(&str, &[Candidate])] = &[
    ("browser", &[("chromium.desktop", "Chromium", "chromium", false), ("firefox.desktop", "Firefox", "firefox", false), ("org.qutebrowser.qutebrowser.desktop", "qutebrowser", "qutebrowser", false), ("xdg-open-handler.desktop", "Bağlantı işleyici", "applications-internet", true)]),
    ("email", &[("org.mozilla.Thunderbird.desktop", "Thunderbird", "thunderbird", false), ("org.gnome.Evolution.desktop", "Evolution", "evolution", false)]),
    ("file_manager", &[("org.gnome.Nautilus.desktop", "Dosyalar", "org.gnome.Nautilus", false), ("thunar.desktop", "Thunar", "org.xfce.thunar", false), ("org.kde.dolphin.desktop", "Dolphin", "system-file-manager", false)]),
    ("terminal", &[("kitty.desktop", "kitty", "kitty", false), ("foot.desktop", "Foot", "foot", false), ("Alacritty.desktop", "Alacritty", "Alacritty", false)]),
    ("text_editor", &[("org.gnome.TextEditor.desktop", "Metin Düzenleyici", "org.gnome.TextEditor", false), ("nvim.desktop", "Neovim", "nvim", false), ("code.desktop", "Visual Studio Code", "vscode", false)]),
    ("image", &[("org.gnome.Loupe.desktop", "Görüntü Görüntüleyici", "org.gnome.Loupe", false), ("imv.desktop", "imv", "multimedia-photo-viewer", false), ("gimp.desktop", "GNU Görüntü İşleme Programı", "gimp", false)]),
    ("video", &[("mpv.desktop", "mpv", "mpv", false), ("vlc.desktop", "VLC", "vlc", false), ("umpv.desktop", "umpv", "mpv", true)]),
    ("music", &[("mpv.desktop", "mpv", "mpv", false), ("org.gnome.Rhythmbox3.desktop", "Rhythmbox", "rhythmbox", false)]),
    ("pdf", &[("org.gnome.Evince.desktop", "Belge Görüntüleyici", "org.gnome.Evince", false), ("org.pwmt.zathura.desktop", "Zathura", "org.pwmt.zathura", false), ("firefox.desktop", "Firefox", "firefox", false)]),
];

/// Süreç içi seçimler (rol → kimlik); ilk değerler kullanıcı benzeri.
fn defaults_current() -> &'static Choices {
    static CURRENT: std::sync::OnceLock<Choices> = std::sync::OnceLock::new();
    CURRENT.get_or_init(|| {
        let flags = defaults_flags();
        let has = |f: &str| flags.iter().any(|x| x == f);
        let init = [
            ("browser", Some("firefox.desktop")),
            ("email", if has("nulls") { None } else { Some("org.mozilla.Thunderbird.desktop") }),
            ("file_manager", Some("org.gnome.Nautilus.desktop")),
            ("terminal", if has("noterm") { None } else { Some("kitty.desktop") }),
            ("text_editor", Some("org.gnome.TextEditor.desktop")),
            ("image", Some("org.gnome.Loupe.desktop")),
            ("video", Some(if has("gone") { "celluloid.desktop" } else { "mpv.desktop" })),
            ("music", Some("mpv.desktop")),
            ("pdf", if has("nulls") { None } else { Some("org.gnome.Evince.desktop") }),
        ];
        std::sync::Mutex::new(init.iter().map(|(r, c)| ((*r).to_owned(), c.map(str::to_owned))).collect())
    })
}

fn app_json(id: &str, name: &str, icon: &str, hidden: Option<bool>) -> String {
    let hidden = hidden.map(|h| format!(r#","hidden":{h}"#)).unwrap_or_default();
    format!(r#"{{"id":"{id}","name":"{name}","icon":"{icon}"{hidden}}}"#)
}

fn defaults_status() -> String {
    let flags = defaults_flags();
    let has = |f: &str| flags.iter().any(|x| x == f);
    if has("unsupported") {
        return "error: bilinmeyen komut".to_owned();
    }
    let current = defaults_current().lock().unwrap().clone();
    let roles: Vec<String> = DEFAULT_CANDIDATES
        .iter()
        .map(|(role, cands)| {
            let cur = current.iter().find(|(r, _)| r == role).and_then(|(_, c)| c.clone());
            let cur_json = match &cur {
                None => "null".to_owned(),
                Some(id) => match cands.iter().find(|c| c.0 == id) {
                    Some((id, name, icon, _)) => app_json(id, name, icon, None),
                    None => format!(r#"{{"id":"{id}","name":"{id}","icon":null}}"#),
                },
            };
            let source = if cur.is_none() { "none" } else if *role == "browser" && has("shadowed") { "desktop_user" } else { "user" };
            let mixed = *role == "image" && has("mixed");
            let shadowed = if *role == "browser" && has("shadowed") { r#""~/.config/hyprland-mimeapps.list""# } else { "null" };
            let available = !(*role == "terminal" && has("noterm"));
            let cand_json: Vec<String> = cands.iter().map(|(id, name, icon, hidden)| app_json(id, name, icon, Some(*hidden))).collect();
            format!(
                r#"{{"role":"{role}","current":{cur_json},"source":"{source}","mixed":{mixed},"shadowed_by":{shadowed},"available":{available},"candidates":[{}]}}"#,
                cand_json.join(",")
            )
        })
        .collect();
    format!(r#"{{"v":1,"roles":[{}]}}"#, roles.join(","))
}

/// `defaults set <rol> <kimlik>`: sözleşmenin hata kodları (shell gibi JSON: `{"ok":false,
/// "error":"<kod>"}`); başarıda seçim süreç içinde değişir. `NUXUT_FAKE_DEFAULTS=readonly` →
/// `writer_disabled`, `failwrite` → `write_failed` (+ `detail`).
fn defaults_set(role: &str, id: &str) -> String {
    let flags = defaults_flags();
    let Some((_, cands)) = DEFAULT_CANDIDATES.iter().find(|(r, _)| *r == role) else { return r#"{"ok":false,"error":"unknown_role"}"#.to_owned() };
    if role == "terminal" && flags.iter().any(|f| f == "noterm") {
        return r#"{"ok":false,"error":"unavailable"}"#.to_owned();
    }
    if !cands.iter().any(|c| c.0 == id) {
        return r#"{"ok":false,"error":"unknown_app"}"#.to_owned();
    }
    if flags.iter().any(|f| f == "readonly") {
        return r#"{"ok":false,"error":"writer_disabled"}"#.to_owned();
    }
    if flags.iter().any(|f| f == "failwrite") {
        return r#"{"ok":false,"error":"write_failed","detail":"permission_denied"}"#.to_owned();
    }
    let mut current = defaults_current().lock().unwrap();
    let Some(slot) = current.iter_mut().find(|(r, _)| r == role) else { return r#"{"ok":false,"error":"unknown_role"}"#.to_owned() };
    if slot.1.as_deref() == Some(id) {
        return r#"{"ok":true,"snapshot":null,"written":[]}"#.to_owned();
    }
    slot.1 = Some(id.to_owned());
    r#"{"ok":true,"snapshot":"20261013T120000Z-d0e1","written":["x-scheme-handler/http"]}"#.to_owned()
}

/// `magnifier status` (magnifier-v1): kullanılamıyorken `factor` null ve tanı nedeni.
fn magnifier_status(available: bool, zoom: f64) -> String {
    if !available {
        return r#"{"v":1,"available":false,"factor":null,"active":false,"managed":false,"step":1.25,"max":8.0,"reason":"no hyprland"}"#.to_owned();
    }
    format!(r#"{{"v":1,"available":true,"factor":{zoom},"active":{},"managed":{},"step":1.25,"max":8.0,"reason":null}}"#, zoom > 1.0, zoom > 1.0)
}
