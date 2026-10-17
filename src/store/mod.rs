//! Config yazma katmanı: biçim/yorum koruyan, yalnızca değişen anahtarı yazan, atomik ve
//! kendi yazımını dış değişiklikten ayırt eden dosya modeli. Bkz. `docs/ARCHITECTURE.md` §3.

mod path;

use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

use toml_edit::{Decor, DocumentMut, Item, Table, TableLike, Value};

pub use path::KeyPath;

/// Okunacak en büyük config dosyası; daha büyüğü bozuk sayılır.
pub const MAX_FILE_BYTES: u64 = 1 << 20;

/// Ayarlar'ın düzenlediği dosyalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileId {
    Framework,
    Shell,
    /// Üretilmiş palet (`$XDG_STATE_HOME/nuxut/palette.toml`). Yalnızca `[meta]` üretim
    /// parametreleri düzenlenir; dosyayı toml_edit değil palet üreticisi yazar (renkler
    /// parametrelerle tutarlı kalsın diye) — bkz. [`Store::take_palette_request`].
    Palette,
}

impl FileId {
    pub const ALL: [FileId; 3] = [FileId::Framework, FileId::Shell, FileId::Palette];

    /// `<config_dir>/<ad>.toml` içindeki ad (runtime'ın `config_changed` adı).
    pub fn name(self) -> &'static str {
        match self {
            FileId::Framework => "framework",
            FileId::Shell => "shell",
            FileId::Palette => "palette",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|f| f.name() == name)
    }
}

/// Dosya değişim bildiriminin sonucu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiskChange {
    /// İçerik bizim son yazdığımız/okuduğumuzla aynı: hiçbir şey yapılmaz.
    Unchanged,
    /// Dış değişiklik yüklendi (bekleyen değişikliklerimiz üstüne yeniden uygulandı).
    Reloaded,
    /// Dosya silindi; belge boşaldı (varsayılanlar).
    Removed,
    /// Dosya ayrıştırılamadı; belge korunur, yazım durur.
    Broken(String),
}

/// Bir config dosyasının bellekteki düzenlenebilir hâli.
#[derive(Debug)]
pub struct ConfigFile {
    path: PathBuf,
    doc: DocumentMut,
    /// Diskte olduğunu bildiğimiz son içerik (okunan ya da yazılan). `None`: dosya yok.
    disk_text: Option<String>,
    /// Henüz yazılmamış değişiklikler (dış yeniden yüklemede yeniden uygulanır).
    pending: Vec<(KeyPath, Option<Value>)>,
    broken: Option<String>,
    /// `flush` yazım çakışmasında dış hâli yükledi (bkz. [`take_reloaded`](Self::take_reloaded)).
    reloaded: bool,
    /// Dosyayı yaratan yazımda damgalanacak `config_version` (yalnızca shell.toml; bkz.
    /// [`set_config_version`](Self::set_config_version)).
    config_version: Option<i64>,
}

impl ConfigFile {
    /// Dosyayı açar. Yoksa boş belge; bozuksa boş belge + `broken`.
    pub fn open(path: impl Into<PathBuf>) -> Self {
        let mut file = Self { path: path.into(), doc: DocumentMut::new(), disk_text: None, pending: Vec::new(), broken: None, reloaded: false, config_version: None };
        match read_text(&file.path) {
            Ok(Some(text)) => {
                match text.parse::<DocumentMut>() {
                    Ok(doc) => file.doc = doc,
                    Err(e) => file.broken = Some(short_error(&e)),
                }
                file.disk_text = Some(text);
            }
            Ok(None) => {}
            Err(e) => file.broken = Some(e.to_string()),
        }
        file
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn doc(&self) -> &DocumentMut {
        &self.doc
    }

    /// Dosya bozuksa hata metni (yazım durur).
    pub fn broken(&self) -> Option<&str> {
        self.broken.as_deref()
    }

    pub fn is_dirty(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Bekleyen değişiklikleri yazmadan unutur (dosyayı başka bir yazıcı üretecekse).
    pub fn discard_pending(&mut self) {
        self.pending.clear();
    }

    pub fn get(&self, path: &KeyPath) -> Option<&Item> {
        let (last, parents) = path.segments().split_last()?;
        let mut table: &dyn TableLike = self.doc.as_table();
        for seg in parents {
            table = table.get(seg)?.as_table_like()?;
        }
        table.get(last)
    }

    pub fn get_value(&self, path: &KeyPath) -> Option<&Value> {
        self.get(path)?.as_value()
    }

    /// Anahtarı değiştirir (`None` = sil → varsayılan). Önceki değeri döner. Bellekte hemen
    /// geçerlidir; diske [`flush`](Self::flush) ile gider.
    pub fn set(&mut self, path: &KeyPath, value: Option<Value>) -> Option<Value> {
        let before = self.get_value(path).cloned();
        if same_value(before.as_ref(), value.as_ref()) {
            return before;
        }
        apply(&mut self.doc, path, value.clone());
        self.pending.retain(|(p, _)| p != path);
        self.pending.push((path.clone(), value));
        before
    }

    /// Bekleyen değişiklikleri atomik olarak yazar. Yazılacak bir şey yoksa (ya da içerik
    /// diskteki ile aynıysa) `Ok(false)`. Dosya bozukken yazmaz. Dosya son okumamızdan beri
    /// dışarıdan değiştiyse üstüne yazılmaz: dış hâl yüklenir, bekleyenler üstüne uygulanır ve
    /// bir kez daha denenir (bkz. [`take_reloaded`](Self::take_reloaded)).
    pub fn flush(&mut self) -> io::Result<bool> {
        for attempt in 0..2 {
            if self.pending.is_empty() || self.broken.is_some() {
                return Ok(false);
            }
            // Yalnızca yeni dosyaya: var olan sürümsüz dosyayı shell ilk açılışta damgalar.
            if let (Some(v), None) = (self.config_version, &self.disk_text) {
                stamp_config_version(&mut self.doc, v);
            }
            // Baştaki tablo silinince sonrakinin önündeki boş satır dosyanın başına kalır.
            let text = self.doc.to_string().trim_start_matches('\n').to_owned();
            if self.disk_text.as_deref() == Some(text.as_str()) {
                self.pending.clear();
                return Ok(false);
            }
            match write_atomic_if(&self.path, text.as_bytes(), Some(self.disk_text.as_deref())) {
                Ok(()) => {
                    self.disk_text = Some(text);
                    self.pending.clear();
                    return Ok(true);
                }
                Err(e) if attempt == 0 && is_changed_on_disk(&e) => {
                    self.on_disk_changed();
                    self.reloaded = true;
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!("ikinci denemede her yol döner")
    }

    /// Dosyayı yaratan yazıma bu `config_version`'ı da yazar (shell ADR-0002 §11: sürümsüz
    /// dosya v1 sayılır). Var olan sürümsüz dosyaya yazılmaz; onu shell ilk açılışta damgalar
    /// (tek yazan). `None`: damga yok.
    pub fn set_config_version(&mut self, version: Option<i64>) {
        self.config_version = version;
    }

    /// `flush` dış değişikliği yükledi mi (sayfalar yeniden eşitlenmeli); bayrağı sıfırlar.
    pub fn take_reloaded(&mut self) -> bool {
        std::mem::take(&mut self.reloaded)
    }

    /// Dosya değişim bildirimi geldi. Kendi yazımımızsa hiçbir şey yapmaz; dış değişiklikse
    /// yeniden yükler ve bekleyen değişiklikleri üstüne uygular. **Asla yazmaz.**
    pub fn on_disk_changed(&mut self) -> DiskChange {
        let text = match read_text(&self.path) {
            Ok(t) => t,
            Err(e) => {
                let msg = e.to_string();
                self.broken = Some(msg.clone());
                return DiskChange::Broken(msg);
            }
        };
        if text == self.disk_text && self.broken.is_none() {
            return DiskChange::Unchanged;
        }
        let Some(text) = text else {
            self.disk_text = None;
            self.broken = None;
            self.doc = DocumentMut::new();
            self.reapply_pending();
            return DiskChange::Removed;
        };
        match text.parse::<DocumentMut>() {
            Ok(doc) => {
                self.doc = doc;
                self.disk_text = Some(text);
                self.broken = None;
                self.reapply_pending();
                DiskChange::Reloaded
            }
            Err(e) => {
                let msg = short_error(&e);
                self.disk_text = Some(text);
                self.broken = Some(msg.clone());
                DiskChange::Broken(msg)
            }
        }
    }

    fn reapply_pending(&mut self) {
        for (path, value) in &self.pending {
            apply(&mut self.doc, path, value.clone());
        }
    }
}

/// İki değer aynı mı (süsleme hariç).
/// Yeni yaratılan shell.toml'a `config_version = v` yazar (belgede sürüm yoksa).
fn stamp_config_version(doc: &mut DocumentMut, v: i64) {
    if doc.as_table().contains_key("config_version") {
        return;
    }
    let header = take_file_header(doc);
    doc.insert("config_version", Item::Value(Value::from(v)));
    // Kök değerleri tablolardan önce yazılır; sürüm onların da en üstünde dursun.
    doc.as_table_mut().sort_values_by(|a, _, b, _| (b.get() == "config_version").cmp(&(a.get() == "config_version")));
    if let (Some(h), Some(mut key)) = (header, doc.as_table_mut().key_mut("config_version")) {
        key.leaf_decor_mut().set_prefix(h);
    }
}

/// Dosyanın boş satırla ayrılmış baş yorumu: ilk yazılan öğenin önekinden son boş satıra kadarki
/// kısım. Önekten çıkarılır (öğenin kendi yorumu yerinde kalır) ve döner; yoksa `None`.
fn take_file_header(doc: &mut DocumentMut) -> Option<String> {
    let root = doc.as_table_mut();
    let first_value = root.iter().find(|(_, item)| item.is_value()).map(|(k, _)| k.to_owned());
    match first_value {
        Some(k) => split_header(root.key_mut(&k)?.leaf_decor_mut()),
        None => {
            let k = root.iter().filter_map(|(k, item)| Some((item.as_table()?.position()?, k.to_owned()))).min()?.1;
            let decor = root.get_mut(&k)?.as_table_mut()?.decor_mut();
            let header = split_header(decor);
            // Sürüm satırıyla ilk tablo arasında boş satır.
            let rest = decor.prefix().and_then(|p| p.as_str()).unwrap_or("").to_owned();
            if !rest.starts_with('\n') {
                decor.set_prefix(format!("\n{rest}"));
            }
            header
        }
    }
}

fn split_header(decor: &mut Decor) -> Option<String> {
    let prefix = decor.prefix()?.as_str()?.to_owned();
    let end = prefix.rfind("\n\n")? + 2;
    decor.set_prefix(prefix[end..].to_owned());
    Some(prefix[..end].to_owned())
}

fn same_value(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let mut a = a.clone();
            let mut b = b.clone();
            a.decor_mut().clear();
            b.decor_mut().clear();
            a.to_string() == b.to_string()
        }
        _ => false,
    }
}

fn apply(doc: &mut DocumentMut, path: &KeyPath, value: Option<Value>) {
    let Some((last, parents)) = path.segments().split_last() else { return };
    match value {
        Some(mut value) => {
            let mut table: &mut dyn TableLike = doc.as_table_mut();
            for seg in parents {
                let exists = table.get(seg).is_some_and(|i| i.is_table_like());
                if !exists {
                    // Eksik ara tablo açık başlıkla yaratılır ([a.b]); aynı adla tablo olmayan
                    // bir değer varsa (bozuk şema) yerini alır.
                    let mut t = Table::new();
                    t.set_implicit(true);
                    table.insert(seg, Item::Table(t));
                }
                table = table.get_mut(seg).and_then(Item::as_table_like_mut).expect("tablo az önce yaratıldı");
            }
            match table.get_mut(last) {
                Some(Item::Value(old)) => {
                    // Değerin önündeki/arkasındaki süsleme (satır sonu yorumu) korunur.
                    *value.decor_mut() = old.decor().clone();
                    *old = value;
                }
                _ => {
                    table.insert(last, Item::Value(value.decorated(" ", "")));
                }
            }
        }
        None => {
            remove_key(doc.as_table_mut(), path.segments());
        }
    }
}

/// Anahtarı siler; boşalan ve yorum taşımayan ara tabloları da kaldırır.
fn remove_key(table: &mut dyn TableLike, segments: &[String]) -> bool {
    match segments {
        [] => false,
        [last] => table.remove(last).is_some(),
        [first, rest @ ..] => {
            let Some(child) = table.get_mut(first).and_then(Item::as_table_like_mut) else { return false };
            let removed = remove_key(child, rest);
            if removed && child.is_empty() {
                let has_comment = match table.get(first) {
                    Some(Item::Table(t)) => decor_has_comment(t.decor()),
                    _ => false,
                };
                if !has_comment {
                    table.remove(first);
                }
            }
            removed
        }
    }
}

fn decor_has_comment(decor: &toml_edit::Decor) -> bool {
    let has = |r: Option<&toml_edit::RawString>| r.and_then(|r| r.as_str()).is_some_and(|s| s.contains('#'));
    has(decor.prefix()) || has(decor.suffix())
}

fn short_error(e: &toml_edit::TomlError) -> String {
    // İlk satır yeterli ("TOML parse error at line 3, column 7"); ayrıntı loga.
    tracing::warn!("config ayrıştırılamadı: {e}");
    let s = e.to_string();
    let first = s.lines().next().unwrap_or("").trim();
    let msg = e.message().trim();
    if first.is_empty() { msg.to_owned() } else { format!("{first}: {msg}") }
}

/// Dosyayı okur; yoksa `None`. 1 MiB'tan büyük dosya hata.
fn read_text(path: &Path) -> io::Result<Option<String>> {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    if meta.len() > MAX_FILE_BYTES {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "dosya çok büyük (> 1 MiB)"));
    }
    match std::fs::read_to_string(path) {
        Ok(t) => Ok(Some(t)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

/// `write_atomic_if` için: dosya bizim bildiğimiz içerikten farklı (yazım sırasında
/// başkası değiştirdi). [`is_changed_on_disk`] ile tanınır.
#[derive(Debug)]
struct ChangedOnDisk;

impl std::fmt::Display for ChangedOnDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dosya yazım sırasında dışarıdan değişti")
    }
}

impl std::error::Error for ChangedOnDisk {}

/// Hata [`write_atomic_if`]'in "dosya değişti, yazılmadı" hatası mı.
pub fn is_changed_on_disk(e: &io::Error) -> bool {
    e.get_ref().is_some_and(|inner| inner.is::<ChangedOnDisk>())
}

/// Symlink zincirini izleyip gerçek hedef yolu döner (hedef henüz yoksa da — kırık
/// symlink'in gösterdiği yere yazılır). Symlink değilse yolun kendisi.
fn resolve_target(path: &Path) -> io::Result<PathBuf> {
    let mut cur = path.to_path_buf();
    for _ in 0..40 {
        match std::fs::symlink_metadata(&cur) {
            Ok(m) if m.file_type().is_symlink() => {
                let link = std::fs::read_link(&cur)?;
                cur = match cur.parent() {
                    Some(dir) if link.is_relative() => dir.join(link),
                    _ => link,
                };
            }
            Ok(_) => return Ok(cur),
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(cur),
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidInput, "symlink döngüsü"))
}

/// Koşulsuz [`write_atomic_if`].
pub fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    write_atomic_if(path, bytes, None)
}

/// Aynı dizinde geçici dosyaya yazıp `rename` eder; okuyan hiçbir zaman yarım dosya görmez.
/// `path` symlink ise symlink korunur, gösterdiği dosya değiştirilir (dotfile depoları);
/// hedefin izinleri korunur. `expected` verilmişse (`Some(None)` = dosya yok) `rename`'den
/// hemen önce diskteki içerik onunla karşılaştırılır; farklıysa yazılmaz ve
/// [`is_changed_on_disk`] hatası döner (dış düzenleme ezilmez). Karşılaştırma ile `rename`
/// arasındaki çok kısa pencere kapatılamaz (POSIX'te koşullu rename yok).
pub fn write_atomic_if(path: &Path, bytes: &[u8], expected: Option<Option<&str>>) -> io::Result<()> {
    use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
    use std::sync::atomic::{AtomicU32, Ordering};
    static SEQ: AtomicU32 = AtomicU32::new(0);

    let target = resolve_target(path)?;
    let dir = target.parent().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "üst dizin yok"))?;
    std::fs::create_dir_all(dir)?;
    let mode = match std::fs::metadata(&target) {
        Ok(m) if !m.is_file() => return Err(io::Error::new(io::ErrorKind::InvalidInput, "hedef düz dosya değil")),
        Ok(m) => m.permissions().mode() & 0o777,
        Err(e) if e.kind() == io::ErrorKind::NotFound => 0o644,
        Err(e) => return Err(e),
    };
    let name = target.file_name().and_then(|n| n.to_str()).unwrap_or("config");
    let tmp = dir.join(format!(".{name}.{}-{}.tmp", std::process::id(), SEQ.fetch_add(1, Ordering::Relaxed)));
    let result = (|| {
        // create_new: önceden konmuş bir dosyanın/symlink'in içine yazılmaz.
        let mut f = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&tmp)?;
        f.write_all(bytes)?;
        f.set_permissions(std::fs::Permissions::from_mode(mode))?;
        f.sync_all()?;
        if let Some(expected) = expected
            && read_text(&target)?.as_deref() != expected
        {
            return Err(io::Error::other(ChangedOnDisk));
        }
        std::fs::rename(&tmp, &target)?;
        // Dizin girdisini (rename'in kendisini) kalıcı kıl.
        if let Ok(d) = std::fs::File::open(dir) {
            let _ = d.sync_all();
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

#[cfg(test)]
mod tests;

/// Ayarlar'ın tüm config dosyaları. `revision` her bellek değişikliğinde artar (sayfalar
/// buna bakarak bileşenlerini eşitler).
#[derive(Debug)]
pub struct Store {
    framework: ConfigFile,
    shell: ConfigFile,
    palette: ConfigFile,
    palette_request: Option<PaletteRequest>,
    revision: u64,
}

/// Palet yeniden üretim isteği (`palette.toml [meta]` parametreleri).
#[derive(Debug, Clone, PartialEq)]
pub struct PaletteRequest {
    pub image: Option<PathBuf>,
    pub scheme: String,
    pub mode: String,
    pub contrast: f64,
}

impl PaletteRequest {
    pub fn from_file(f: &ConfigFile) -> Self {
        let s = |k: &str, d: &str| f.get_value(&KeyPath::new(["meta", k])).and_then(Value::as_str).unwrap_or(d).to_owned();
        let contrast = match f.get_value(&KeyPath::parse("meta.contrast")) {
            Some(Value::Float(v)) => *v.value(),
            Some(Value::Integer(v)) => *v.value() as f64,
            _ => 0.0,
        };
        let image = s("image", "");
        Self { image: (!image.is_empty()).then(|| PathBuf::from(image)), scheme: s("scheme", "tonal-spot"), mode: s("mode", "dark"), contrast }
    }
}

impl Store {
    pub fn open(framework: impl Into<PathBuf>, shell: impl Into<PathBuf>, palette: impl Into<PathBuf>) -> Self {
        Self {
            framework: ConfigFile::open(framework),
            shell: ConfigFile::open(shell),
            palette: ConfigFile::open(palette),
            palette_request: None,
            revision: 0,
        }
    }

    /// `<config_dir>/{framework,shell}.toml` + `$XDG_STATE_HOME/nuxut/palette.toml`.
    pub fn open_default() -> Option<Self> {
        Some(Self::open(nuxut::config::config_path("framework")?, nuxut::config::config_path("shell")?, nuxut::theme::palette_path()?))
    }

    pub fn file(&self, id: FileId) -> &ConfigFile {
        match id {
            FileId::Framework => &self.framework,
            FileId::Shell => &self.shell,
            FileId::Palette => &self.palette,
        }
    }

    pub fn file_mut(&mut self, id: FileId) -> &mut ConfigFile {
        match id {
            FileId::Framework => &mut self.framework,
            FileId::Shell => &mut self.shell,
            FileId::Palette => &mut self.palette,
        }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn get(&self, id: FileId, path: &KeyPath) -> Option<&Value> {
        self.file(id).get_value(path)
    }

    /// Bellekte değiştirir, önceki değeri döner.
    pub fn set(&mut self, id: FileId, path: &KeyPath, value: Option<Value>) -> Option<Value> {
        self.revision += 1;
        self.file_mut(id).set(path, value)
    }

    pub fn is_dirty(&self) -> bool {
        FileId::ALL.into_iter().any(|f| self.file(f).is_dirty())
    }

    /// Kirli dosyaları yazar; hata olanları döner (değişiklik bellekte kalır).
    pub fn flush(&mut self) -> Vec<(FileId, io::Error)> {
        let mut errors = Vec::new();
        if self.palette.is_dirty() {
            // Palet dosyasını toml_edit yazmaz: üretici (arka planda) parametrelerden yeniden
            // üretir ve atomik kaydeder; dönüşte inotify belgeyi diskten yeniler.
            self.palette_request = Some(PaletteRequest::from_file(&self.palette));
            self.palette.discard_pending();
        }
        for id in [FileId::Framework, FileId::Shell] {
            if let Err(e) = self.file_mut(id).flush() {
                tracing::warn!("{} yazılamadı: {e}", id.name());
                errors.push((id, e));
            }
            if self.file_mut(id).take_reloaded() {
                self.revision += 1;
            }
        }
        errors
    }

    /// Çalışan shell'in şemasındaki güncel `config_version` (bkz. [`ConfigFile::set_config_version`]).
    pub fn set_shell_config_version(&mut self, version: Option<i64>) {
        self.shell.set_config_version(version);
    }

    /// `flush` sonrası bekleyen palet üretimi (varsa).
    pub fn take_palette_request(&mut self) -> Option<PaletteRequest> {
        self.palette_request.take()
    }

    pub fn on_disk_changed(&mut self, id: FileId) -> DiskChange {
        let change = self.file_mut(id).on_disk_changed();
        if change != DiskChange::Unchanged {
            self.revision += 1;
        }
        change
    }
}
