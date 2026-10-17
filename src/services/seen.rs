//! "Yeni" rozetinin görülme durumu (shell ADR-0002 §3.6; model karar, 2026-10-13).
//!
//! - **Durum dosyası:** `$XDG_STATE_HOME/nuxut/settings.toml` (yoksa `~/.local/state/nuxut/…`),
//!   `[seen]` tablosu: `"<kimlik>" = <since>`. Kimlik shell ayarında shell.toml noktalı yolu
//!   (`wallpaper.backdrop`, `bar.widget.clock.format`), widget'ta `widget:<ad>` ([`widget_id`]).
//!   Kullanıcı config'ine (shell.toml/framework.toml) hiç yazılmaz. Dosyadaki başka anahtarlar
//!   korunur: her yazımda disk yeniden okunur, yalnızca `[seen]` girdileri (büyük olan) eklenir,
//!   atomik yazılır ([`crate::store::write_atomic`]).
//! - **Rozet:** `since > 1` ve `[seen]`'deki kayıt < `since` ([`SeenState::badge`]). Açılıştaki
//!   kayıt esas alınır: görülen satırın rozeti o sayfada kalır, sayfadan ayrılınca
//!   ([`retire`]) ya da sonraki açılışta kalkar (satır gözün önünde birden kaymasın).
//! - **İlk açılış:** `[seen]` hiç yoksa (taze kurulum, okunamayan dosya) rozet yoktur; şema ilk
//!   gelince şemadaki tüm `since`'li öğeler görüldü yazılır ([`SeenState::seed`]).
//! - **Bozuk dosya:** rozet yok; üzerine yazmadan önce `settings.toml.bak` kopyası alınır,
//!   kopya alınamazsa yazılmaz. Okunamayan (izin) dosyaya hiç yazılmaz.
//! - **Görüldü:** satır kesintisiz [`DWELL`] (1 sn) ekranda kalınca ([`Tracker`]): Ayarlar
//!   penceresi odakta, sayfa açık, satırın başlığı kaydırma görünür alanında. Yazım toplu ve
//!   seyrek: `App` [`FLUSH_DELAY`] sonra (ve kapanışta) yazar.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use nuxut::Rect;
use toml_edit::{DocumentMut, Item, Table, value};

/// Satırın görüldü sayılması için kesintisiz görünme süresi.
pub const DWELL: Duration = Duration::from_secs(1);
/// Kareler arası bu kadardan uzun boşluk görünürlüğü keser (pencere gizlendi, kare gelmedi):
/// uyanma `DWELL` sonuna kurulur, ondan biraz geç gelen kare hâlâ kesintisiz sayılır.
const GAP: Duration = Duration::from_millis(1250);
/// Görüldü kayıtlarının toplu yazım gecikmesi (her karede değil).
pub const FLUSH_DELAY: Duration = Duration::from_secs(3);
const FILE: &str = "settings.toml";
const TABLE: &str = "seen";

/// Widget kataloğu öğesinin kimliği (ekleme menüsü).
pub fn widget_id(name: &str) -> String {
    format!("widget:{name}")
}

/// `$XDG_STATE_HOME/nuxut/settings.toml` (yoksa `~/.local/state/nuxut/settings.toml`).
pub fn default_path() -> Option<PathBuf> {
    nuxut_palette::export::state_dir().map(|d| d.join(FILE))
}

/// Yedek yolu: `settings.toml.bak`.
pub fn backup_path(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".bak");
    PathBuf::from(s)
}

/// Diskteki durum dosyası.
enum Disk {
    Missing,
    Ok(DocumentMut),
    /// TOML değil ya da `seen` tablo değil: üzerine yazmadan önce yedeklenir.
    Corrupt,
}

fn read_disk(path: &Path) -> io::Result<Disk> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Disk::Missing),
        // UTF-8 olmayan içerik bozuk sayılır; izin vb. hatalar çağırana (yazılmaz).
        Err(e) if e.kind() == io::ErrorKind::InvalidData => return Ok(Disk::Corrupt),
        Err(e) => return Err(e),
    };
    let Ok(doc) = text.parse::<DocumentMut>() else { return Ok(Disk::Corrupt) };
    if doc.get(TABLE).is_some_and(|i| i.as_table_like().is_none()) {
        return Ok(Disk::Corrupt);
    }
    Ok(Disk::Ok(doc))
}

/// `[seen]` girdileri; tamsayı olmayan değerler yok sayılır.
fn entries(doc: &DocumentMut) -> Option<BTreeMap<String, i64>> {
    let t = doc.get(TABLE)?.as_table_like()?;
    Some(t.iter().filter_map(|(k, v)| Some((k.to_owned(), v.as_integer()?))).collect())
}

/// Görülme kayıtları ve bu oturumun rozet durumu. UI'dan bağımsız; yol verilir (testte geçici).
#[derive(Debug)]
pub struct SeenState {
    path: Option<PathBuf>,
    /// Açılışta (ya da tohumlamada) okunan kayıt: rozetler buna göre. `None` = `[seen]` yok
    /// (taze kurulum ya da bozuk dosya) → rozet yok, şema gelince tohumlanır.
    baseline: Option<HashMap<String, i64>>,
    /// Bu oturumda bilinen (yazılacak) kayıt; yazımda diskteki daha büyük değerlerle birleşir.
    seen: BTreeMap<String, i64>,
    /// Görüldü sayılan ama rozeti sayfadan ayrılana dek kalan kimlikler.
    pending: HashSet<String>,
    /// Rozeti bu oturumda kalkmış kimlikler.
    retired: HashSet<String>,
    dirty: bool,
}

impl SeenState {
    /// Durumu dosyadan okur. Dosya yok, bozuk ya da okunamıyorsa `[seen]` yok sayılır.
    pub fn load(path: Option<PathBuf>) -> Self {
        let baseline = match path.as_deref().map(read_disk) {
            Some(Ok(Disk::Ok(doc))) => entries(&doc),
            Some(Ok(Disk::Corrupt)) => {
                tracing::warn!("Ayarlar durum dosyası okunamadı (bozuk); \"Yeni\" rozetleri kapalı, yazımdan önce yedeklenecek");
                None
            }
            Some(Err(e)) => {
                tracing::warn!(error = %e, "Ayarlar durum dosyası okunamadı");
                None
            }
            _ => None,
        };
        let seen = baseline.as_ref().map(|b| b.iter().map(|(k, v)| (k.clone(), *v)).collect()).unwrap_or_default();
        Self { path, baseline: baseline.map(|b| b.into_iter().collect()), seen, pending: HashSet::new(), retired: HashSet::new(), dirty: false }
    }

    /// `[seen]` var mı (ilk açılış tohumlaması yapıldı ya da önceden vardı).
    pub fn is_seeded(&self) -> bool {
        self.baseline.is_some()
    }

    /// "Yeni" rozeti görünür mü: `since > 1`, açılıştaki kayıt `since`'ten küçük (yoksa 0) ve
    /// rozet bu oturumda kalkmamış. Tohumlanmamışsa hiçbir rozet yok.
    pub fn badge(&self, id: &str, since: Option<i64>) -> bool {
        let (Some(base), Some(since)) = (&self.baseline, since.filter(|s| *s > 1)) else { return false };
        base.get(id).copied().unwrap_or(0) < since && !self.retired.contains(id)
    }

    /// Bu `since` için zaten görüldü yazıldı mı (izlemeye gerek yok).
    pub fn is_marked(&self, id: &str, since: i64) -> bool {
        self.seen.get(id).is_some_and(|s| *s >= since)
    }

    /// İlk açılış: `[seen]` yoksa verilen tüm öğeleri görüldü kaydeder (taze kurulumda her şey
    /// "Yeni" olmasın). `true`: tohumlandı (hemen yazılmalı). Varsa hiçbir şey yapmaz.
    pub fn seed(&mut self, items: impl IntoIterator<Item = (String, i64)>) -> bool {
        if self.baseline.is_some() {
            return false;
        }
        for (id, since) in items {
            if since > 1 {
                let e = self.seen.entry(id).or_insert(since);
                *e = (*e).max(since);
            }
        }
        self.baseline = Some(self.seen.iter().map(|(k, v)| (k.clone(), *v)).collect());
        self.dirty = true;
        true
    }

    /// Satır görüldü: kayıt yazılacak, rozet sayfadan ayrılana dek kalır ([`retire`](Self::retire)).
    pub fn mark(&mut self, id: &str, since: i64) {
        if self.baseline.is_none() || since <= 1 || self.is_marked(id, since) {
            return;
        }
        self.seen.insert(id.to_owned(), since);
        self.pending.insert(id.to_owned());
        self.dirty = true;
    }

    /// Sayfadan ayrılındı: görülen satırların rozeti kalkar. `true`: bir rozet kalktı.
    pub fn retire(&mut self) -> bool {
        let any = !self.pending.is_empty();
        self.retired.extend(self.pending.drain());
        any
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Kayıtları yazar (değişiklik varsa). Disk yeniden okunur: başka anahtarlar ve başka
    /// süreçlerin `[seen]` girdileri korunur (her kimlikte büyük olan). Bozuk dosya önce
    /// `.bak`'a kopyalanır; kopya ya da okuma başarısızsa hiçbir şey yazılmaz.
    pub fn save(&mut self) -> io::Result<()> {
        if !self.dirty {
            return Ok(());
        }
        let Some(path) = self.path.clone() else {
            self.dirty = false;
            return Ok(());
        };
        let mut doc = match read_disk(&path)? {
            Disk::Missing => DocumentMut::new(),
            Disk::Ok(doc) => doc,
            Disk::Corrupt => {
                let bak = backup_path(&path);
                std::fs::copy(&path, &bak)?;
                tracing::warn!(backup = %bak.display(), "bozuk Ayarlar durum dosyası yedeklendi, yeniden yazılıyor");
                DocumentMut::new()
            }
        };
        if let Some(disk) = entries(&doc) {
            for (k, v) in disk {
                let e = self.seen.entry(k).or_insert(v);
                *e = (*e).max(v);
            }
        }
        // Satır içi tablo (`seen = { … }`) da standart tabloya çevrilir; sıra kimliğe göre.
        let mut table = Table::new();
        for (k, v) in &self.seen {
            table.insert(k, value(*v));
        }
        doc.insert(TABLE, Item::Table(table));
        crate::store::write_atomic(&path, doc.to_string().as_bytes())?;
        self.dirty = false;
        Ok(())
    }
}

/// Kesintisiz görünürlük izleyicisi (saat dışarıdan verilir; test edilebilir). Her karede
/// rozetli ve görünür öğeler [`sight`](Self::sight) ile bildirilir, kare sonunda
/// [`end_frame`](Self::end_frame) [`DWELL`]'i dolduranları döndürür. Bir karede bildirilmeyen
/// öğenin sayacı sıfırlanır.
#[derive(Debug, Default)]
pub struct Tracker {
    /// Kimlik → (since, ilk görüldüğü an).
    candidates: HashMap<String, (i64, Instant)>,
    frame: Vec<(String, i64)>,
    last: Option<Instant>,
}

impl Tracker {
    pub fn sight(&mut self, id: &str, since: i64) {
        self.frame.push((id.to_owned(), since));
    }

    /// Kare sonu. `active`: pencere odakta ve görünür. Dönüş: görüldü sayılanlar ve sonraki
    /// uyanma anı (kare istemeden zamanlayıcı; o ana kadar görünür kalırsa görüldü sayılır).
    pub fn end_frame(&mut self, now: Instant, active: bool) -> (Vec<(String, i64)>, Option<Instant>) {
        let frame = std::mem::take(&mut self.frame);
        let continuous = self.last.is_none_or(|l| now.saturating_duration_since(l) <= GAP);
        self.last = Some(now);
        if !active || !continuous {
            self.candidates.clear();
        }
        if !active {
            return (Vec::new(), None);
        }
        let mut next = HashMap::new();
        let mut done = Vec::new();
        for (id, since) in frame {
            if next.contains_key(&id) || done.iter().any(|(d, _)| *d == id) {
                continue;
            }
            let first = self.candidates.get(&id).map_or(now, |c| c.1);
            if now.saturating_duration_since(first) >= DWELL {
                done.push((id, since));
            } else {
                next.insert(id, (since, first));
            }
        }
        self.candidates = next;
        let wake = self.candidates.values().map(|c| c.1 + DWELL).min();
        (done, wake)
    }
}

/// `rect` (başlık) görünür alanın tamamen içinde mi (yarım görünen satır sayılmaz).
pub fn within(rect: Rect, viewport: Rect) -> bool {
    const EPS: f32 = 0.5;
    rect.width > 0.0
        && rect.height > 0.0
        && rect.x >= viewport.x - EPS
        && rect.y >= viewport.y - EPS
        && rect.right() <= viewport.right() + EPS
        && rect.bottom() <= viewport.bottom() + EPS
}

// --- Süreç geneli durum (Ayarlar penceresi). Birim testlerinde kurulmaz: rozet yok. ---

#[derive(Debug)]
struct Global {
    state: SeenState,
    tracker: Tracker,
    viewport: Rect,
    /// `begin_frame`..`end_frame` arası (Hoş geldin penceresinin karesi sayılmaz).
    in_frame: bool,
    active: bool,
}

static GLOBAL: Mutex<Option<Global>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut Global) -> R) -> Option<R> {
    let mut g = GLOBAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    g.as_mut().map(f)
}

/// Durum dosyasını okur (Ayarlar açılırken bir kez).
pub fn init(path: Option<PathBuf>) {
    let state = SeenState::load(path);
    let mut g = GLOBAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *g = Some(Global { state, tracker: Tracker::default(), viewport: Rect::ZERO, in_frame: false, active: false });
}

/// Rozet görünür mü (bkz. [`SeenState::badge`]).
pub fn badge(id: &str, since: Option<i64>) -> bool {
    with(|g| g.state.badge(id, since)).unwrap_or(false)
}

/// Ayarlar karesinin başı: sayfanın görünür alanı ve pencere etkin mi.
pub fn begin_frame(viewport: Rect, active: bool) {
    with(|g| {
        g.viewport = viewport;
        g.active = active;
        g.in_frame = true;
    });
}

/// Rozetli öğe bu karede `rect`'te çizildi (yüzey koordinatı; satırın başlık kısmı).
pub fn sight(id: &str, since: Option<i64>, rect: Rect) {
    let Some(since) = since else { return };
    with(|g| {
        if g.in_frame && g.active && within(rect, g.viewport) && !g.state.is_marked(id, since) {
            g.tracker.sight(id, since);
        }
    });
}

/// Ayarlar karesinin sonu: [`DWELL`]'i dolduranlar görüldü yazılır (rozet sayfada kalır).
/// Dönüş: sonraki uyanma anı.
pub fn end_frame(now: Instant) -> Option<Instant> {
    with(|g| {
        g.in_frame = false;
        let (done, wake) = g.tracker.end_frame(now, g.active);
        for (id, since) in done {
            tracing::debug!(id, since, "\"Yeni\" öğe görüldü");
            g.state.mark(&id, since);
        }
        wake
    })
    .flatten()
}

/// Sayfadan ayrılındı (bkz. [`SeenState::retire`]).
pub fn retire() -> bool {
    with(|g| g.state.retire()).unwrap_or(false)
}

/// Yazılmamış kayıt var mı.
pub fn is_dirty() -> bool {
    with(|g| g.state.is_dirty()).unwrap_or(false)
}

/// Kayıtları yazar (değişiklik varsa); hata loglanır.
pub fn save() {
    if let Some(Err(e)) = with(|g| g.state.save()) {
        tracing::warn!(error = %e, "Ayarlar durum dosyası yazılamadı");
    }
}

/// Şemadaki `since`'li tüm öğeler (genel/widget ayarları noktalı yolla, widget'lar
/// `widget:<ad>`); şema yüklenmediyse ya da hatalıysa `None`.
fn schema_items() -> Option<Vec<(String, i64)>> {
    use super::widgets;
    if !matches!(widgets::load_state(), Some(Ok(()))) {
        return None;
    }
    let mut items: Vec<(String, i64)> = widgets::shell_settings().into_iter().filter_map(|(k, s)| s.since.map(|n| (k, n))).collect();
    items.extend(widgets::catalog().iter().filter_map(|w| w.since.map(|n| (widget_id(&w.name), n))));
    Some(items)
}

/// Şema geldi: ilk açılışsa (`[seen]` yok) şemadaki tüm öğeleri görüldü yazar (hemen).
/// `true`: tohumlandı.
pub fn seed_from_schema() -> bool {
    let Some(items) = schema_items() else { return false };
    let seeded = with(|g| g.state.seed(items)).unwrap_or(false);
    if seeded {
        tracing::info!("\"Yeni\" rozetleri: ilk açılış, mevcut öğeler görüldü sayıldı");
        save();
    }
    seeded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> (tempfile::TempDir, PathBuf) {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("state").join("nuxut").join(FILE);
        (d, p)
    }

    fn read(p: &Path) -> String {
        std::fs::read_to_string(p).unwrap()
    }

    fn seen_of(p: &Path, id: &str) -> Option<i64> {
        entries(&read(p).parse().unwrap())?.get(id).copied()
    }

    #[test]
    fn fresh_install_seeds_everything_as_seen() {
        let (_d, p) = tmp();
        let mut s = SeenState::load(Some(p.clone()));
        assert!(!s.is_seeded());
        assert!(!s.badge("a.b", Some(3)), "tohumlanmadan rozet yok");
        assert!(s.seed([("a.b".to_owned(), 3), ("widget:clock".to_owned(), 2), ("old".to_owned(), 1)]));
        s.save().unwrap();
        let text = read(&p);
        assert!(text.contains("[seen]") && text.contains("\"a.b\" = 3") && text.contains("\"widget:clock\" = 2"), "{text}");
        assert!(!text.contains("old"), "since 1 yazılmaz: {text}");
        assert!(!s.badge("a.b", Some(3)));
        // Sonraki açılış: tohumlanmış; yeni gelen ve sürümü artan öğe rozetli.
        let s = SeenState::load(Some(p));
        assert!(s.is_seeded());
        assert!(!s.badge("a.b", Some(3)));
        assert!(s.badge("a.b", Some(4)), "since arttı");
        assert!(s.badge("new.key", Some(2)), "kayıtsız yeni öğe");
        assert!(!s.badge("new.key", Some(1)) && !s.badge("new.key", None), "since ≤ 1 hiç yeni değil");
    }

    #[test]
    fn seed_is_once() {
        let (_d, p) = tmp();
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, "[seen]\n").unwrap();
        let mut s = SeenState::load(Some(p));
        assert!(s.is_seeded(), "boş [seen] de tohumlanmış sayılır");
        assert!(!s.seed([("x".to_owned(), 5)]));
        assert!(s.badge("x", Some(5)));
    }

    #[test]
    fn marked_badge_stays_until_retire_and_next_start() {
        let (_d, p) = tmp();
        let mut s = SeenState::load(Some(p.clone()));
        s.seed(Vec::new());
        assert!(s.badge("x", Some(2)));
        s.mark("x", 2);
        assert!(s.is_marked("x", 2) && s.is_dirty());
        assert!(s.badge("x", Some(2)), "görüldü ama sayfada kalır");
        assert!(s.retire());
        assert!(!s.badge("x", Some(2)), "sayfadan ayrılınca kalkar");
        s.save().unwrap();
        assert!(!SeenState::load(Some(p)).badge("x", Some(2)), "sonraki açılışta yok");
    }

    #[test]
    fn save_keeps_other_keys_and_merges_disk() {
        let (_d, p) = tmp();
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, "# not\nfoo = 1\n[window]\nwidth = 900\n[seen]\n\"a\" = 2\n").unwrap();
        let mut s = SeenState::load(Some(p.clone()));
        // Bu arada başka bir süreç yazdı.
        std::fs::write(&p, "# not\nfoo = 1\n[window]\nwidth = 900\n[seen]\n\"a\" = 2\n\"b\" = 7\n").unwrap();
        s.mark("c", 3);
        s.save().unwrap();
        let text = read(&p);
        assert!(text.contains("# not") && text.contains("foo = 1") && text.contains("width = 900"), "{text}");
        assert_eq!((seen_of(&p, "a"), seen_of(&p, "b"), seen_of(&p, "c")), (Some(2), Some(7), Some(3)), "{text}");
        let leftovers: Vec<_> = std::fs::read_dir(p.parent().unwrap()).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(leftovers.len(), 1, "geçici dosya kalmaz: {leftovers:?}");
    }

    #[test]
    fn corrupt_file_no_badges_and_backup_before_overwrite() {
        let (_d, p) = tmp();
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        let junk = "[seen\n\"a\" = = 2\n";
        std::fs::write(&p, junk).unwrap();
        let mut s = SeenState::load(Some(p.clone()));
        assert!(!s.is_seeded());
        assert!(!s.badge("a", Some(9)), "bozuk dosyada rozet yok");
        s.mark("a", 9);
        assert!(!s.is_dirty(), "tohumlanmadan işaretleme yazılmaz");
        assert!(s.seed([("a".to_owned(), 9)]));
        s.save().unwrap();
        assert_eq!(read(&backup_path(&p)), junk, "yedek önceki içerik");
        assert_eq!(seen_of(&p, "a"), Some(9));
        // `seen` tablo değilse de bozuk sayılır.
        std::fs::write(&p, "seen = 3\nfoo = \"x\"\n").unwrap();
        let mut s = SeenState::load(Some(p.clone()));
        assert!(!s.is_seeded());
        s.seed(Vec::new());
        s.save().unwrap();
        assert_eq!(read(&backup_path(&p)), "seen = 3\nfoo = \"x\"\n");
    }

    #[test]
    fn unreadable_file_is_never_overwritten() {
        let (_d, p) = tmp();
        // Dosya yerine dizin: okuma hatası (bozuk değil) → yazılmaz, yedek de alınmaz.
        std::fs::create_dir_all(&p).unwrap();
        let mut s = SeenState::load(Some(p.clone()));
        s.seed(Vec::new());
        assert!(s.save().is_err());
        assert!(p.is_dir() && !backup_path(&p).exists());
    }

    #[test]
    fn writes_only_the_state_file() {
        let d = tempfile::tempdir().unwrap();
        let config = d.path().join("config").join("nuxut");
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(config.join("shell.toml"), "config_version = 3\n").unwrap();
        std::fs::write(config.join("framework.toml"), "").unwrap();
        let state = d.path().join("state").join("nuxut").join(FILE);
        let mut s = SeenState::load(Some(state.clone()));
        s.seed([("bar.x".to_owned(), 3)]);
        s.mark("bar.y", 4);
        s.save().unwrap();
        assert!(state.exists());
        assert_eq!(read(&config.join("shell.toml")), "config_version = 3\n");
        assert_eq!(read(&config.join("framework.toml")), "");
        let names: Vec<_> = std::fs::read_dir(&config).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(names.len(), 2, "config dizinine dosya eklenmez: {names:?}");
    }

    #[test]
    fn default_path_follows_xdg_state_home() {
        // Ortamı değiştirmeden: yol `<state_dir>/nuxut/settings.toml` biçiminde.
        if let Some(p) = default_path() {
            assert!(p.ends_with("nuxut/settings.toml"), "{}", p.display());
        }
    }

    #[test]
    fn dwell_needs_one_continuous_second() {
        let t0 = Instant::now();
        let ms = |n| t0 + Duration::from_millis(n);
        let mut t = Tracker::default();
        t.sight("a", 2);
        let (done, wake) = t.end_frame(t0, true);
        assert!(done.is_empty());
        assert_eq!(wake, Some(ms(1000)), "uyanma eşiğin sonunda");
        t.sight("a", 2);
        assert!(t.end_frame(ms(500), true).0.is_empty());
        t.sight("a", 2);
        let (done, wake) = t.end_frame(ms(1000), true);
        assert_eq!(done, vec![("a".to_owned(), 2)]);
        assert_eq!(wake, None);
    }

    #[test]
    fn dwell_resets_when_hidden_unfocused_or_frames_stop() {
        let t0 = Instant::now();
        let ms = |n| t0 + Duration::from_millis(n);
        // Bir karede görünmedi (kaydırıldı / sayfa değişti): sayaç baştan.
        let mut t = Tracker::default();
        t.sight("a", 2);
        t.end_frame(t0, true);
        t.end_frame(ms(600), true);
        t.sight("a", 2);
        let (done, wake) = t.end_frame(ms(1100), true);
        assert!(done.is_empty());
        assert_eq!(wake, Some(ms(2100)));
        // Pencere odakta değil.
        let mut t = Tracker::default();
        t.sight("a", 2);
        t.end_frame(t0, true);
        t.sight("a", 2);
        assert_eq!(t.end_frame(ms(900), false), (Vec::new(), None));
        t.sight("a", 2);
        assert!(t.end_frame(ms(1200), true).0.is_empty(), "odak dönünce yeniden sayılır");
        // Kare gelmedi (pencere gizli): uzun boşluk kesinti sayılır.
        let mut t = Tracker::default();
        t.sight("a", 2);
        t.end_frame(t0, true);
        t.sight("a", 2);
        assert!(t.end_frame(ms(5000), true).0.is_empty());
    }

    #[test]
    fn within_requires_whole_rect() {
        let v = Rect::new(0.0, 100.0, 500.0, 300.0);
        assert!(within(Rect::new(10.0, 120.0, 100.0, 20.0), v));
        assert!(!within(Rect::new(10.0, 90.0, 100.0, 20.0), v), "üstten yarım");
        assert!(!within(Rect::new(10.0, 390.0, 100.0, 20.0), v), "alttan yarım");
        assert!(!within(Rect::new(10.0, 120.0, 0.0, 0.0), v), "boş");
    }
}
