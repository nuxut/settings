//! Dosya seçimi: `org.freedesktop.portal.FileChooser.OpenFile` (zbus, arka plan thread'i).
//! Yanıt, isteğin `Request` nesnesindeki `Response` sinyaliyle gelir; yarışmasın diye
//! sinyale çağrıdan **önce** abone olunur (tutamaç yolu `handle_token`'dan bilinir).

use std::collections::HashMap;
use std::path::PathBuf;

use zbus::zvariant::{OwnedValue, Value};

use nuxut::i18n::t;

use super::{Event, emit};

#[derive(Debug, Clone)]
pub enum Update {
    /// Seçilen dosya (`tag` isteği yapan bloğu ayırt eder).
    Picked { tag: String, path: PathBuf },
    Cancelled { tag: String },
    Failed { tag: String, message: String },
}

/// Görsel/GIF seçtirir (shell video çizmiyor). `tag` yanıtla geri döner.
pub fn pick_wallpaper(tag: String, title: String) {
    let _ = std::thread::Builder::new().name("portal".into()).spawn(move || {
        let result = open_file(&title);
        emit(Event::Portal(match result {
            Ok(Some(path)) => Update::Picked { tag, path },
            Ok(None) => Update::Cancelled { tag },
            Err(e) => {
                tracing::warn!("dosya seçici: {e}");
                Update::Failed { tag, message: t!("settings-wallpaper-error-portal") }
            }
        }));
    });
}

fn open_file(title: &str) -> zbus::Result<Option<PathBuf>> {
    let conn = zbus::blocking::Connection::session()?;
    let token = format!("nuxut_settings_{}", std::process::id() ^ (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.subsec_nanos())));
    let sender = conn.unique_name().map(|n| n.trim_start_matches(':').replace('.', "_")).unwrap_or_default();
    let handle = format!("/org/freedesktop/portal/desktop/request/{sender}/{token}");
    let rule = zbus::MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface("org.freedesktop.portal.Request")?
        .member("Response")?
        .path(handle.as_str())?
        .build();
    let mut responses = zbus::blocking::MessageIterator::for_match_rule(rule, &conn, Some(4))?;

    let filter_name = t!("settings-portal-filter-images");
    let filters: Vec<(&str, Vec<(u32, &str)>)> = vec![(filter_name.as_str(), vec![(1, "image/*")])];
    let mut options: HashMap<&str, Value> = HashMap::new();
    options.insert("handle_token", Value::from(token.as_str()));
    options.insert("modal", Value::from(true));
    options.insert("filters", Value::from(filters));
    if let Some(dir) = pictures_dir() {
        let mut bytes = dir.into_os_string().into_encoded_bytes();
        bytes.push(0);
        options.insert("current_folder", Value::from(bytes));
    }
    conn.call_method(Some("org.freedesktop.portal.Desktop"), "/org/freedesktop/portal/desktop", Some("org.freedesktop.portal.FileChooser"), "OpenFile", &("", title, options))?;

    if let Some(msg) = responses.next() {
        let msg = msg?;
        let (code, results): (u32, HashMap<String, OwnedValue>) = msg.body().deserialize()?;
        if code != 0 {
            return Ok(None);
        }
        let uris: Vec<String> = results.get("uris").and_then(|v| Vec::<String>::try_from(v.clone()).ok()).unwrap_or_default();
        return Ok(uris.first().and_then(|u| file_uri_to_path(u)));
    }
    Ok(None)
}

fn pictures_dir() -> Option<PathBuf> {
    let home = PathBuf::from(std::env::var_os("HOME")?);
    ["Resimler", "Pictures"].into_iter().map(|d| home.join(d)).find(|p| p.is_dir())
}

/// `file:///a%20b/c.png` → `/a b/c.png`. Yerel olmayan URI'ler reddedilir.
pub fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let rest = rest.strip_prefix("localhost").unwrap_or(rest);
    if !rest.starts_with('/') {
        return None;
    }
    let bytes = rest.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 3 > bytes.len() {
                return None;
            }
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    use std::os::unix::ffi::OsStringExt as _;
    let p = PathBuf::from(std::ffi::OsString::from_vec(out));
    // NUL ya da `..` bileşeni olan yollar reddedilir.
    (!p.as_os_str().as_encoded_bytes().contains(&0) && !p.components().any(|c| matches!(c, std::path::Component::ParentDir))).then_some(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        assert_eq!(file_uri_to_path("file:///home/a/Resimler/deniz%20k%C4%B1y%C4%B1s%C4%B1.png"), Some(PathBuf::from("/home/a/Resimler/deniz kıyısı.png")));
        assert_eq!(file_uri_to_path("file://localhost/x.png"), Some(PathBuf::from("/x.png")));
        assert_eq!(file_uri_to_path("https://x/y.png"), None);
        assert_eq!(file_uri_to_path("file:///a/../etc/passwd"), None);
        assert_eq!(file_uri_to_path("file:///a%00b"), None);
        assert_eq!(file_uri_to_path("file:///a%2"), None);
    }
}
