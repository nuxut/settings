//! Kullanıcının varsayılan yazıcısı: `~/.cups/lpoptions` (libcups'ın `lpoptions -d` ile yazdığı
//! dosya; GTK/Qt yazdırma pencereleri ve `lp` okur). Biçim satır başına
//! `Dest|Default <ad>[/<örnek>] [seçenek=değer …]`; varsayılan tek `Default` satırıdır.
//! Yazım biçim korur: yalnız anahtar sözcük değişir ya da sona satır eklenir. Bkz.
//! `docs/design/printers.md` §2.

use std::io::Write;
use std::path::{Path, PathBuf};

/// `$HOME/.cups/lpoptions` (HOME yoksa `None`).
pub fn path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").filter(|h| !h.is_empty())?;
    Some(PathBuf::from(home).join(".cups/lpoptions"))
}

/// Satırın (anahtar sözcük, ad, örnek) parçaları; `Dest`/`Default` değilse `None`.
fn parse_line(line: &str) -> Option<(&str, &str, Option<&str>)> {
    let mut it = line.split_whitespace();
    let kw = it.next()?;
    if !(kw.eq_ignore_ascii_case("dest") || kw.eq_ignore_ascii_case("default")) {
        return None;
    }
    let dest = it.next()?;
    let (name, instance) = match dest.split_once('/') {
        Some((n, i)) => (n, Some(i)),
        None => (dest, None),
    };
    Some((kw, name, instance))
}

/// Dosyadaki varsayılan kuyruğun adı (örnek eki atılmış).
pub fn default_in(text: &str) -> Option<String> {
    text.lines().filter_map(parse_line).find(|(kw, ..)| kw.eq_ignore_ascii_case("default")).map(|(_, name, _)| name.to_owned())
}

/// `name`'i varsayılan yapan yeni dosya metni (saf; testli). Önceki `Default` satırları `Dest`
/// olur (seçenekleri kalır); `name`'in örneksiz `Dest` satırı `Default` olur, yoksa sona
/// `Default <name>` eklenir. Diğer satırlar aynen kalır.
pub fn with_default(text: &str, name: &str) -> String {
    let mut out = String::with_capacity(text.len() + name.len() + 10);
    let mut done = false;
    for line in text.lines() {
        match parse_line(line) {
            Some((kw, n, inst)) => {
                let rest = line.trim_start()[kw.len()..].to_owned();
                let target = n == name && inst.is_none() && !done;
                if target {
                    done = true;
                }
                out.push_str(if target { "Default" } else { "Dest" });
                out.push_str(&rest);
            }
            None => out.push_str(line),
        }
        out.push('\n');
    }
    if !done {
        out.push_str("Default ");
        out.push_str(name);
        out.push('\n');
    }
    out
}

/// Dosyadaki varsayılan (dosya yoksa ya da okunamıyorsa `None`).
pub fn read_default(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok().as_deref().and_then(default_in)
}

/// `name`'i varsayılan yapar: dizin (0700) gerekirse açılır, dosya geçici dosya + rename ile
/// atomik yazılır (izinler korunur; yeni dosya 0644, libcups gibi).
pub fn write_default(path: &Path, name: &str) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
    if name.is_empty() || name.contains(|c: char| c.is_whitespace() || c == '/' || c == '#') {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "geçersiz kuyruk adı"));
    }
    let dir = path.parent().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "dizin yok"))?;
    std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir)?;
    let old = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e),
    };
    let mode = std::fs::metadata(path).map(|m| m.permissions().mode() & 0o777).unwrap_or(0o644);
    let tmp = dir.join(format!(".lpoptions.nuxut-{}", std::process::id()));
    let res = (|| {
        let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(mode).open(&tmp)?;
        f.write_all(with_default(&old, name).as_bytes())?;
        f.sync_all()?;
        std::fs::rename(&tmp, path)
    })();
    if res.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_default() {
        assert_eq!(default_in(""), None);
        assert_eq!(default_in("Dest a sides=two-sided-long-edge\nDefault b/foto media=A4\n"), Some("b".into()));
        assert_eq!(default_in("  default   c\n"), Some("c".into()), "büyük/küçük harf ve boşluk");
        assert_eq!(default_in("# yorum\nDest a\n"), None);
    }

    #[test]
    fn rewrites_keyword_only() {
        let text = "Dest a sides=two-sided-long-edge\nDefault b media=A4\n# yorum\n";
        assert_eq!(with_default(text, "a"), "Default a sides=two-sided-long-edge\nDest b media=A4\n# yorum\n");
        assert_eq!(with_default(text, "c"), "Dest a sides=two-sided-long-edge\nDest b media=A4\n# yorum\nDefault c\n");
        assert_eq!(with_default("", "c"), "Default c\n");
        assert_eq!(with_default("Default c\n", "c"), "Default c\n", "zaten varsayılan: değişmez");
        // Örnek satırı ana kuyruğun yerine geçmez; örneğe verilmiş varsayılan Dest olur.
        assert_eq!(with_default("Default a/foto x=1\nDest a\n", "a"), "Dest a/foto x=1\nDefault a\n");
        assert_eq!(with_default("Dest a/foto\n", "a"), "Dest a/foto\nDefault a\n");
        assert_eq!(default_in(&with_default("Dest x\nDefault y\n", "x")), Some("x".into()));
    }

    #[test]
    fn writes_atomically() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join(".cups/lpoptions");
        assert_eq!(read_default(&p), None);
        write_default(&p, "ofis").unwrap();
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "Default ofis\n");
        assert_eq!(std::fs::metadata(p.parent().unwrap()).unwrap().permissions().mode() & 0o777, 0o700);
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o600)).unwrap();
        write_default(&p, "ev").unwrap();
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "Dest ofis\nDefault ev\n");
        assert_eq!(std::fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o600, "izin korunur");
        assert_eq!(read_default(&p), Some("ev".into()));
        assert!(write_default(&p, "a b").is_err() && write_default(&p, "").is_err());
        assert_eq!(std::fs::read_dir(p.parent().unwrap()).unwrap().count(), 1, "geçici dosya kalmaz");
    }
}
