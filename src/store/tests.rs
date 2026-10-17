use super::*;

const SAMPLE: &str = r##"# Kullanıcının elle yazdığı başlık yorumu

[colors]
source = "static"      # elle not
primary = "#9ec5ff"

# hareket ayarları
[motion]
speed = 1.0   # hız
style = "spring"

[motion.spatial]
damping = 0.7

[unknown_section]
foo = [1, 2, 3]   # dokunulmamalı
"##;

fn setup(content: Option<&str>) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("framework.toml");
    if let Some(c) = content {
        std::fs::write(&path, c).unwrap();
    }
    (dir, path)
}

fn k(s: &str) -> KeyPath {
    KeyPath::parse(s)
}

#[test]
fn changes_only_the_key_and_keeps_comments() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(1.5)));
    assert!(f.flush().unwrap());
    let out = std::fs::read_to_string(&path).unwrap();
    assert_eq!(out, SAMPLE.replace("speed = 1.0   # hız", "speed = 1.5   # hız"));
}

#[test]
fn adds_missing_keys_and_tables_without_touching_others() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.spatial.response"), Some(Value::from(0.5)));
    f.set(&k("motion.effects.easing"), Some(Value::from("ease-out")));
    f.set(&k("shadow.enabled"), Some(Value::from(false)));
    f.flush().unwrap();
    let out = std::fs::read_to_string(&path).unwrap();
    assert!(out.starts_with("# Kullanıcının elle yazdığı başlık yorumu"));
    assert!(out.contains("foo = [1, 2, 3]   # dokunulmamalı"));
    assert!(out.contains("source = \"static\"      # elle not"));
    let reparsed: toml_edit::DocumentMut = out.parse().unwrap();
    assert_eq!(reparsed["motion"]["spatial"]["response"].as_float(), Some(0.5));
    assert_eq!(reparsed["motion"]["spatial"]["damping"].as_float(), Some(0.7));
    assert_eq!(reparsed["motion"]["effects"]["easing"].as_str(), Some("ease-out"));
    assert_eq!(reparsed["shadow"]["enabled"].as_bool(), Some(false));
    assert!(out.contains("[motion.effects]"), "{out}");
    assert!(out.contains("[shadow]"), "{out}");
}

#[test]
fn creates_file_when_missing() {
    let (_d, path) = setup(None);
    let mut f = ConfigFile::open(&path);
    assert!(f.get(&k("motion.speed")).is_none());
    f.set(&k("motion.speed"), Some(Value::from(2.0)));
    f.flush().unwrap();
    let out = std::fs::read_to_string(&path).unwrap();
    assert_eq!(out.trim(), "[motion]\nspeed = 2.0");
}

#[test]
fn remove_resets_to_default_and_drops_empty_table() {
    let (_d, path) = setup(Some("[shadow]\nblur = 20.0\n\n[blob]\ndeform = 0.2\n"));
    let mut f = ConfigFile::open(&path);
    f.set(&k("shadow.blur"), None);
    f.flush().unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "[blob]\ndeform = 0.2\n");
}

#[test]
fn table_with_comment_survives_becoming_empty() {
    let src = "# gölgeyi ben ayarladım\n[shadow]\nblur = 20.0\n";
    let (_d, path) = setup(Some(src));
    let mut f = ConfigFile::open(&path);
    f.set(&k("shadow.blur"), None);
    f.flush().unwrap();
    assert!(std::fs::read_to_string(&path).unwrap().contains("[shadow]"));
}

#[test]
fn no_write_when_nothing_changed() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(1.0)));
    assert!(!f.is_dirty(), "aynı değer kirletmez");
    assert!(!f.flush().unwrap());
    // Değiştirip geri almak da yazım üretmez.
    f.set(&k("motion.speed"), Some(Value::from(3.0)));
    f.set(&k("motion.speed"), Some(Value::from(1.0)));
    assert!(!f.flush().unwrap());
}

#[test]
fn own_write_is_not_seen_as_external_change() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(1.25)));
    f.flush().unwrap();
    // inotify bildirimi: kendi yazımımız → hiçbir şey yapılmaz, yazım da yok.
    let mtime = std::fs::metadata(&path).unwrap().modified().unwrap();
    assert_eq!(f.on_disk_changed(), DiskChange::Unchanged);
    assert_eq!(f.on_disk_changed(), DiskChange::Unchanged);
    assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), mtime);
    assert!(!f.flush().unwrap(), "yükleme yazım tetiklemez (save-on-parse döngüsü yok)");
}

#[test]
fn external_change_reloads_and_keeps_pending_edits() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(2.0))); // henüz yazılmadı
    // Kullanıcı editörde başka bir anahtarı değiştirip kaydetti (rename ile).
    write_atomic(&path, SAMPLE.replace("primary = \"#9ec5ff\"", "primary = \"#ff0000\"").as_bytes()).unwrap();
    assert_eq!(f.on_disk_changed(), DiskChange::Reloaded);
    assert_eq!(f.get_value(&k("colors.primary")).and_then(Value::as_str), Some("#ff0000"));
    assert_eq!(f.get_value(&k("motion.speed")).and_then(Value::as_float), Some(2.0));
    f.flush().unwrap();
    let out = std::fs::read_to_string(&path).unwrap();
    assert!(out.contains("primary = \"#ff0000\""));
    assert!(out.contains("speed = 2.0   # hız"));
}

#[test]
fn broken_external_edit_pauses_writes_until_fixed() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    std::fs::write(&path, "[motion\nspeed = ").unwrap();
    assert!(matches!(f.on_disk_changed(), DiskChange::Broken(_)));
    assert!(f.broken().is_some());
    // Önceki belge korunur (UI değerleri göstermeye devam eder).
    assert_eq!(f.get_value(&k("motion.speed")).and_then(Value::as_float), Some(1.0));
    f.set(&k("motion.speed"), Some(Value::from(3.0)));
    assert!(!f.flush().unwrap(), "bozuk dosyanın üstüne yazılmaz");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "[motion\nspeed = ");
    // Kullanıcı düzeltti: bekleyen değişiklik yeniden uygulanır ve yazılabilir.
    std::fs::write(&path, SAMPLE).unwrap();
    assert_eq!(f.on_disk_changed(), DiskChange::Reloaded);
    assert!(f.broken().is_none());
    assert!(f.flush().unwrap());
    assert!(std::fs::read_to_string(&path).unwrap().contains("speed = 3.0   # hız"));
}

#[test]
fn removed_file_resets_document() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    std::fs::remove_file(&path).unwrap();
    assert_eq!(f.on_disk_changed(), DiskChange::Removed);
    assert!(f.get(&k("motion.speed")).is_none());
}

#[test]
fn atomic_write_leaves_no_temp_and_keeps_mode() {
    use std::os::unix::fs::PermissionsExt as _;
    let (d, path) = setup(Some(SAMPLE));
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(1.1)));
    f.flush().unwrap();
    let names: Vec<_> = std::fs::read_dir(d.path()).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(names, vec![std::ffi::OsString::from("framework.toml")]);
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
}

#[test]
fn atomic_write_is_rename_new_inode() {
    use std::os::unix::fs::MetadataExt as _;
    let (_d, path) = setup(Some(SAMPLE));
    let before = std::fs::metadata(&path).unwrap().ino();
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(1.1)));
    f.flush().unwrap();
    assert_ne!(std::fs::metadata(&path).unwrap().ino(), before, "yerinde yazım değil, rename");
}

#[test]
fn keys_with_dots_are_quoted() {
    let (_d, path) = setup(None);
    let mut f = ConfigFile::open(&path);
    let p = KeyPath::new(["notifications", "apps", "org.gnome.Nautilus", "popups"]);
    f.set(&p, Some(Value::from(false)));
    f.flush().unwrap();
    let out = std::fs::read_to_string(&path).unwrap();
    let doc: toml_edit::DocumentMut = out.parse().unwrap();
    assert_eq!(doc["notifications"]["apps"]["org.gnome.Nautilus"]["popups"].as_bool(), Some(false));
    assert_eq!(p.to_string(), "notifications.apps.\"org.gnome.Nautilus\".popups");
}

#[test]
fn inline_tables_are_edited_in_place() {
    let (_d, path) = setup(Some("motion = { speed = 1.0, style = \"curve\" }\n"));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(0.5)));
    f.flush().unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "motion = { speed = 0.5, style = \"curve\" }\n");
}

#[test]
fn oversized_file_is_broken() {
    let (_d, path) = setup(Some(&"#".repeat(MAX_FILE_BYTES as usize + 1)));
    let f = ConfigFile::open(&path);
    assert!(f.broken().is_some());
}

#[test]
fn symlinked_config_keeps_link_and_writes_target() {
    use std::os::unix::fs::PermissionsExt as _;
    let (d, _) = setup(None);
    let dots = d.path().join("dots");
    std::fs::create_dir(&dots).unwrap();
    let real = dots.join("framework.toml");
    std::fs::write(&real, SAMPLE).unwrap();
    std::fs::set_permissions(&real, std::fs::Permissions::from_mode(0o640)).unwrap();
    let cfg = d.path().join("config");
    std::fs::create_dir(&cfg).unwrap();
    let link = cfg.join("framework.toml");
    std::os::unix::fs::symlink("../dots/framework.toml", &link).unwrap();

    let mut f = ConfigFile::open(&link);
    f.set(&k("motion.speed"), Some(Value::from(1.5)));
    assert!(f.flush().unwrap());
    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink(), "symlink düz dosyayla değişmez");
    assert!(std::fs::read_to_string(&real).unwrap().contains("speed = 1.5   # hız"));
    assert_eq!(std::fs::metadata(&real).unwrap().permissions().mode() & 0o777, 0o640);
    let leftovers: Vec<_> = std::fs::read_dir(&dots).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(leftovers, vec![std::ffi::OsString::from("framework.toml")], "geçici dosya kalmaz");
}

#[test]
fn dangling_symlink_creates_its_target() {
    let (d, _) = setup(None);
    let link = d.path().join("framework.toml");
    std::os::unix::fs::symlink("real.toml", &link).unwrap();
    let mut f = ConfigFile::open(&link);
    f.set(&k("motion.speed"), Some(Value::from(2.0)));
    assert!(f.flush().unwrap());
    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert!(std::fs::read_to_string(d.path().join("real.toml")).unwrap().contains("speed = 2.0"));
}

#[test]
fn symlink_loop_is_an_error() {
    let (d, _) = setup(None);
    let a = d.path().join("a.toml");
    std::os::unix::fs::symlink("b.toml", &a).unwrap();
    std::os::unix::fs::symlink("a.toml", d.path().join("b.toml")).unwrap();
    assert!(write_atomic(&a, b"x = 1").is_err());
}

#[test]
fn conditional_write_refuses_when_file_changed() {
    let (_d, path) = setup(Some(SAMPLE));
    let err = write_atomic_if(&path, b"x = 1\n", Some(Some("başka içerik"))).unwrap_err();
    assert!(is_changed_on_disk(&err));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), SAMPLE, "dış içerik ezilmez");
    let err = write_atomic_if(&path, b"x = 1\n", Some(None)).unwrap_err();
    assert!(is_changed_on_disk(&err), "yok sanılan dosya varsa da yazılmaz");
    write_atomic_if(&path, b"x = 1\n", Some(Some(SAMPLE))).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "x = 1\n");
}

#[test]
fn flush_merges_unnoticed_external_edit_instead_of_clobbering() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(2.0)));
    // Dış düzenleme, inotify bildirimi henüz işlenmeden.
    std::fs::write(&path, SAMPLE.replace("primary = \"#9ec5ff\"", "primary = \"#ff0000\"")).unwrap();
    assert!(f.flush().unwrap());
    assert!(f.take_reloaded(), "sayfalar yeniden eşitlenmeli");
    assert!(!f.take_reloaded());
    let out = std::fs::read_to_string(&path).unwrap();
    assert!(out.contains("primary = \"#ff0000\""), "dış değişiklik korunur");
    assert!(out.contains("speed = 2.0   # hız"));
    assert_eq!(f.on_disk_changed(), DiskChange::Unchanged);
}

#[test]
fn flush_stops_when_external_edit_breaks_the_file() {
    let (_d, path) = setup(Some(SAMPLE));
    let mut f = ConfigFile::open(&path);
    f.set(&k("motion.speed"), Some(Value::from(2.0)));
    std::fs::write(&path, "[motion\n").unwrap();
    assert!(!f.flush().unwrap());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "[motion\n");
    assert!(f.broken().is_some());
    assert!(f.is_dirty(), "değişiklik bellekte bekler");
}

fn shell_file(content: Option<&str>) -> (tempfile::TempDir, PathBuf, ConfigFile) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("shell.toml");
    if let Some(c) = content {
        std::fs::write(&path, c).unwrap();
    }
    let mut f = ConfigFile::open(&path);
    f.set_config_version(Some(2));
    (dir, path, f)
}

#[test]
fn versionless_shell_file_gets_version_on_write() {
    let (_d, path, mut f) = shell_file(None);
    f.set(&k("bar.widget.workspaces.count"), Some(Value::from(15)));
    assert!(f.flush().unwrap());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "config_version = 2\n\n[bar.widget.workspaces]\ncount = 15\n");
}

#[test]
fn new_shell_file_gets_version_on_write() {
    let (_d, path, mut f) = shell_file(None);
    f.set(&k("frame.enabled"), Some(Value::from(false)));
    f.set(&k("wallpaper"), Some(Value::from("/a.png")));
    assert!(f.flush().unwrap());
    assert!(std::fs::read_to_string(&path).unwrap().starts_with("config_version = 2\nwallpaper = \"/a.png\"\n"));
}

#[test]
fn existing_versionless_file_is_left_for_shell() {
    // Var olan sürümsüz dosyayı (v1 ya da değil) shell ilk açılışta damgalar; Ayarlar yazmaz.
    for text in ["", "# başlık\n\n[frame]\nenabled = true\n", "[bar]\nleft = [\"clock\"]\n", "[workspaces]\ncount = 5\n"] {
        let (_d, path, mut f) = shell_file(Some(text));
        f.set(&k("frame.thickness"), Some(Value::from(4)));
        assert!(f.flush().unwrap());
        assert!(!std::fs::read_to_string(&path).unwrap().contains("config_version"), "{text:?}");
    }
}

#[test]
fn existing_version_is_kept() {
    let (_d, path, mut f) = shell_file(Some("config_version = 3\n[frame]\nenabled = true\n"));
    f.set(&k("frame.enabled"), Some(Value::from(false)));
    assert!(f.flush().unwrap());
    assert!(std::fs::read_to_string(&path).unwrap().starts_with("config_version = 3\n"));
}

#[test]
fn no_version_without_shell_schema_or_writes() {
    let (_d, path, mut f) = shell_file(Some("[frame]\nenabled = true\n"));
    assert!(!f.flush().unwrap(), "bekleyen yokken yazım yok");
    f.set_config_version(None);
    f.set(&k("frame.enabled"), Some(Value::from(false)));
    assert!(f.flush().unwrap());
    assert!(!std::fs::read_to_string(&path).unwrap().contains("config_version"));
}
