//! Shell'in widget kataloğu: `nuxut-shell schema` çıktısının `[[widget]]` tabloları (TOML).
//! Katalog shell'in widget kaydından üretilir; ayarlar burada sabit liste tutmaz. Shell yoksa
//! ya da çıktı okunamazsa katalog boştur ve bloklar yalnızca config'teki adları gösterir.
//! Aynı çıktının `[[setting]]` ve `[[widget.setting]]` tabloları ayar başına tür, seçenekler,
//! sınırlar ve `since` (geldiği `config_version`, "Yeni" rozeti) olarak [`shell_setting`] ile
//! okunur; Ayarlar'ın yerel tabloları yalnızca şema yokken (eski/bozuk/çalışmayan shell) yedektir.

use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Once, OnceLock, RwLock};
use std::time::{Duration, Instant};

use nuxut::i18n::t;
use toml_edit::{DocumentMut, Item, Table, Value};

use crate::store::KeyPath;

/// Bir widget ayarının denetim türü.
#[derive(Debug, Clone, PartialEq)]
pub enum SettingKind {
    Bool,
    Int { min: Option<f64>, max: Option<f64>, unit: String },
    Float { min: Option<f64>, max: Option<f64>, step: Option<f64>, unit: String },
    Text,
    Choice(Vec<String>),
}

/// `[bar.widget.<ad>].<key>` ayarı.
#[derive(Debug, Clone)]
pub struct SettingSpec {
    pub key: String,
    pub title: String,
    pub kind: SettingKind,
    /// Shell'in varsayılanı: bu değer seçilince anahtar silinir.
    pub default: Option<Value>,
    /// Geldiği `config_version` (shell ADR-0002 §3.6; yalnızca 2+ yazılır, yoksa 1).
    pub since: Option<i64>,
    /// Seçim değerlerinin görünen adları (shell'in `option_labels`'ı, etkin dilde): `options` ile
    /// aynı sıra ve uzunlukta; boş öğe = o değerin adı yok. Shell yazmadıysa ya da uzunluk
    /// uymadıysa boş vektör (çağıran kendi adına ya da ham değere düşer, bkz. [`Self::option_label`]).
    pub option_labels: Vec<String>,
    /// Shell'in açıklaması (`description`, etkin dilde; ör. saat biçiminin "boş = 12/24 saat
    /// tercihi" sözleşmesi). Yoksa boş; satır yerel alt metnine düşer
    /// ([`crate::registry::SettingDesc::display_subtitle`]).
    pub description: String,
}

impl SettingSpec {
    /// Seçim değerinin shell'deki görünen adı; yoksa `None`.
    pub fn option_label(&self, value: &str) -> Option<&str> {
        let i = self.options()?.iter().position(|o| o == value)?;
        self.option_labels.get(i).map(String::as_str).filter(|l| !l.is_empty())
    }

    /// Sayı ayarının sınırları (`int`/`float`; ikisi de verilmişse). Shell `f32` sınırları `f64`
    /// olarak yazabilir (0.1 → 0.10000000149…): 7 anlamlı basamağa (f32 kesinliği) yuvarlanır, yerel aralık
    /// karşılaştırması ve kaydırıcı ucu temiz kalır.
    pub fn bounds(&self) -> Option<(f64, f64)> {
        match self.kind {
            SettingKind::Int { min: Some(lo), max: Some(hi), .. } | SettingKind::Float { min: Some(lo), max: Some(hi), .. } => Some((tidy(lo), tidy(hi))),
            _ => None,
        }
    }

    /// Seçim ayarının değerleri, shell'in sırasıyla.
    pub fn options(&self) -> Option<&[String]> {
        match &self.kind {
            SettingKind::Choice(o) => Some(o),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WidgetSpec {
    pub name: String,
    pub title: String,
    /// Framework `IconName` adı (`schedule`).
    pub icon: String,
    pub bar: bool,
    pub panel: bool,
    pub settings: Vec<SettingSpec>,
    /// Geldiği `config_version` (ekleme menüsünde "Yeni" rozeti; bkz. [`super::seen`]).
    pub since: Option<i64>,
}

fn str_of(t: &Table, key: &str) -> Option<String> {
    t.get(key).and_then(Item::as_str).map(str::to_owned)
}

fn num_of(t: &Table, key: &str) -> Option<f64> {
    t.get(key).and_then(|i| i.as_float().or_else(|| i.as_integer().map(|n| n as f64)))
}

/// `[[a]]` tabloları ya da satır içi `a = [{ … }]` dizisi (serileştirici ikisini de üretebilir).
fn tables(item: Option<&Item>) -> Option<Vec<Table>> {
    match item? {
        Item::ArrayOfTables(a) => Some(a.iter().cloned().collect()),
        Item::Value(Value::Array(a)) => Some(a.iter().filter_map(|v| v.as_inline_table().map(|t| t.clone().into_table())).collect()),
        _ => None,
    }
}

/// `min`/`max`: sonlu değilse ya da `min > max` ise ikisi de yok sayılır (bozuk şemada yerel
/// aralık geçerli kalır).
fn bounds_of(t: &Table) -> (Option<f64>, Option<f64>) {
    let (min, max) = (num_of(t, "min").filter(|v| v.is_finite()), num_of(t, "max").filter(|v| v.is_finite()));
    match (min, max) {
        (Some(lo), Some(hi)) if lo > hi => (None, None),
        b => b,
    }
}

fn setting(t: &Table) -> Option<SettingSpec> {
    let key = str_of(t, "key").filter(|k| !k.is_empty())?;
    let (min, max) = bounds_of(t);
    let mut option_labels: Vec<String> = Vec::new();
    let kind = match t.get("kind").and_then(Item::as_str)? {
        "bool" => SettingKind::Bool,
        "int" => SettingKind::Int { min, max, unit: str_of(t, "unit").unwrap_or_default() },
        "float" => SettingKind::Float { min, max, step: num_of(t, "step").filter(|s| s.is_finite() && *s > 0.0), unit: str_of(t, "unit").unwrap_or_default() },
        "text" => SettingKind::Text,
        "choice" => {
            let raw = t.get("options").and_then(Item::as_array)?;
            // Adlar ham `options` dizisiyle indis indis eşlenir; uzunluk uymazsa hiçbiri
            // kullanılmaz (hangi adın hangi değere ait olduğu bilinemez).
            let labels = t.get("option_labels").and_then(Item::as_array).filter(|l| l.len() == raw.len());
            let mut options: Vec<String> = Vec::new();
            for (i, v) in raw.iter().enumerate() {
                let Some(v) = v.as_str() else { continue };
                if !v.is_empty() && !options.iter().any(|o| o == v) {
                    options.push(v.to_owned());
                    option_labels.push(labels.and_then(|l| l.get(i)?.as_str()).unwrap_or_default().trim().to_owned());
                }
            }
            if options.is_empty() {
                return None;
            }
            if labels.is_none() {
                option_labels.clear();
            }
            SettingKind::Choice(options)
        }
        // Bilinmeyen tür (daha yeni shell): ayar gösterilmez.
        _ => return None,
    };
    Some(SettingSpec {
        title: str_of(t, "title").unwrap_or_else(|| key.clone()),
        key,
        kind,
        default: t.get("default").and_then(Item::as_value).cloned(),
        since: since_of(t),
        option_labels,
        description: str_of(t, "description").map(|d| d.trim().to_owned()).unwrap_or_default(),
    })
}

/// `since`: 2 ve sonrası anlamlıdır (shell 1'i yazmaz); başka her şey yok sayılır.
fn since_of(t: &Table) -> Option<i64> {
    t.get("since").and_then(Item::as_integer).filter(|v| *v >= 2)
}

/// Şemadaki her ayar, `shell.toml` noktalı yoluyla: genel `[[setting]]` (`bar.size`) ve
/// `[[widget.setting]]` (`bar.widget.<ad>.<key>`). Bozuk girdi ve bilinmeyen tür atlanır,
/// yol tekrarında ilki kalır; sürüm uymazsa boş.
pub fn parse_settings(text: &str) -> HashMap<String, SettingSpec> {
    let mut out = HashMap::new();
    let Ok(doc) = text.parse::<DocumentMut>() else { return out };
    if doc.get("version").and_then(Item::as_integer) != Some(SCHEMA_VERSION) {
        return out;
    }
    for s in tables(doc.get("setting")).unwrap_or_default().iter().filter_map(setting) {
        out.entry(s.key.clone()).or_insert(s);
    }
    for w in tables(doc.get("widget")).unwrap_or_default() {
        let Some(name) = str_of(&w, "name").filter(|n| !n.is_empty() && !n.contains('.')) else { continue };
        for s in tables(w.get("setting")).unwrap_or_default().iter().filter_map(setting) {
            if !s.key.contains('.') {
                out.entry(format!("bar.widget.{name}.{}", s.key)).or_insert(s);
            }
        }
    }
    out
}

/// Şema metnini çözer; sürüm [`SCHEMA_VERSION`] değilse boş. Bozuk girdiler atlanır, ad
/// tekrarında ilki kalır.
pub fn parse(text: &str) -> Vec<WidgetSpec> {
    let Ok(doc) = text.parse::<DocumentMut>() else { return Vec::new() };
    if doc.get("version").and_then(Item::as_integer) != Some(SCHEMA_VERSION) {
        return Vec::new();
    }
    let Some(widgets) = tables(doc.get("widget")) else { return Vec::new() };
    let mut out: Vec<WidgetSpec> = Vec::new();
    for w in &widgets {
        let Some(name) = str_of(w, "name").filter(|n| !n.is_empty()) else { continue };
        if out.iter().any(|o| o.name == name) {
            continue;
        }
        let settings = tables(w.get("setting")).map(|a| a.iter().filter_map(setting).collect()).unwrap_or_default();
        out.push(WidgetSpec {
            title: str_of(w, "title").unwrap_or_else(|| name.clone()),
            icon: str_of(w, "icon").unwrap_or_default(),
            bar: w.get("bar").and_then(Item::as_bool).unwrap_or(true),
            panel: w.get("panel").and_then(Item::as_bool).unwrap_or(false),
            name,
            settings,
            since: since_of(w),
        });
    }
    out
}

/// Şemadaki her ayarın varsayılanı, `shell.toml` biçiminde tek belge: genel `[[setting]]`
/// (`key` noktalı yol) ve `[[widget.setting]]` (`bar.widget.<ad>.<key>`). Varsayılanı olmayan
/// ya da bozuk girdiler atlanır; sürüm uymazsa boş.
pub fn parse_defaults(text: &str) -> DocumentMut {
    let mut out = DocumentMut::new();
    let Ok(doc) = text.parse::<DocumentMut>() else { return out };
    if doc.get("version").and_then(Item::as_integer) != Some(SCHEMA_VERSION) {
        return out;
    }
    let mut put = |path: KeyPath, value: Value| {
        let Some((last, parents)) = path.segments().split_last() else { return };
        let mut table = out.as_table_mut();
        for seg in parents {
            let item = table.entry(seg).or_insert_with(|| Item::Table(Table::new()));
            let Some(t) = item.as_table_mut() else { return };
            table = t;
        }
        table.insert(last, Item::Value(value.decorated("", "")));
    };
    for s in tables(doc.get("setting")).unwrap_or_default() {
        if let (Some(key), Some(v)) = (str_of(&s, "key").filter(|k| !k.is_empty()), s.get("default").and_then(Item::as_value)) {
            put(KeyPath::parse(&key), v.clone());
        }
    }
    for w in tables(doc.get("widget")).unwrap_or_default() {
        let Some(name) = str_of(&w, "name").filter(|n| !n.is_empty()) else { continue };
        for s in tables(w.get("setting")).unwrap_or_default() {
            if let (Some(key), Some(v)) = (str_of(&s, "key").filter(|k| !k.is_empty()), s.get("default").and_then(Item::as_value)) {
                put(KeyPath::new(["bar", "widget", name.as_str(), key.as_str()]), v.clone());
            }
        }
    }
    out
}

/// Şemanın kökündeki `config_version` (shell.toml'un güncel sürümü, shell ADR-0002). Eski shell
/// yazmaz ya da sürüm uymazsa `None` — Ayarlar o zaman dosyaya sürüm damgası basmaz.
pub fn parse_config_version(text: &str) -> Option<i64> {
    let doc = text.parse::<DocumentMut>().ok()?;
    if doc.get("version").and_then(Item::as_integer) != Some(SCHEMA_VERSION) {
        return None;
    }
    doc.get("config_version").and_then(Item::as_integer).filter(|v| *v >= 2)
}

/// Çalışan shell'in shell.toml sürümü; şema henüz yüklenmediyse ya da shell eskiyse `None`.
pub fn shell_config_version() -> Option<i64> {
    current()?.config_version
}

/// Yüklenmiş şema: katalog + ayar tanımları (noktalı yol → tanım). Başlıklar ve seçenek adları
/// shell'in etkin dilinde; dil değişince bütünüyle yenisiyle değiştirilir ([`reload`]).
#[derive(Debug, Default)]
struct Schema {
    widgets: Vec<WidgetSpec>,
    settings: HashMap<String, Arc<SettingSpec>>,
    config_version: Option<i64>,
    error: Option<LoadError>,
}

/// Desteklenen şema sürümü (`version = 1`).
pub const SCHEMA_VERSION: i64 = 1;
const TIMEOUT: Duration = Duration::from_millis(1500);
const MAX_OUTPUT: u64 = 256 * 1024;

/// Geçerli şema. Okuyucular `Arc`'ı kopyalayıp kilidi hemen bırakır (yazma yalnızca yükleme
/// bitince, oturumda birkaç kez); ayrıştırma hep arka planda, kare yolunda hiç yapılmaz.
#[cfg_attr(test, allow(dead_code))]
static SCHEMA: RwLock<Option<Arc<Schema>>> = RwLock::new(None);
/// Shell varsayılanları: ilk başarılı yüklemeden, oturum boyunca sabit. Dile bağlı değiller;
/// `&'static` dönebilmeleri için yeniden yüklemede değiştirilmezler.
static DEFAULTS: OnceLock<DocumentMut> = OnceLock::new();
static STARTED: Once = Once::new();
/// Son istenen yüklemenin sırası: eski bir yükleme (hızlı tr→en→tr) yenisinin üstüne yazmaz.
static GENERATION: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
thread_local! {
    /// Testlerde şema iş parçacığına özel: paralel testler birbirinin şemasını görmez.
    static TEST_SCHEMA: std::cell::RefCell<Option<Arc<Schema>>> = const { std::cell::RefCell::new(None) };
}

#[cfg(not(test))]
fn current() -> Option<Arc<Schema>> {
    SCHEMA.read().ok()?.clone()
}

#[cfg(test)]
fn current() -> Option<Arc<Schema>> {
    TEST_SCHEMA.with(|s| s.borrow().clone())
}

#[cfg(not(test))]
fn store(schema: Schema) {
    if let Ok(mut w) = SCHEMA.write() {
        *w = Some(Arc::new(schema));
    }
}

#[cfg(test)]
fn store(schema: Schema) {
    TEST_SCHEMA.with(|s| *s.borrow_mut() = Some(Arc::new(schema)));
}

/// Yüklenen şemayı yerleştirir. Yeniden yüklemede shell okunamadıysa eski şema (önceki dilde de
/// olsa geçerli değerler) kalır; `seq` artık son istek değilse sonuç atılır. `true`: yerleşti.
fn install(schema: Schema, seq: u64) -> bool {
    // Denetim ve yerleştirme birlikte: eski yükleme denetimden sonra yenisinin üstüne yazamaz
    // (yeni istek sırayı önce artırır, sonucu bu kilidi sonra alır).
    static INSTALL: Mutex<()> = Mutex::new(());
    let _guard = INSTALL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if GENERATION.load(Ordering::SeqCst) != seq {
        return false;
    }
    let keep_old = schema.error.is_some() && current().is_some_and(|old| old.error.is_none());
    if keep_old {
        tracing::warn!("shell şeması yeniden okunamadı; önceki şema kullanılmaya devam ediyor");
        return false;
    }
    store(schema);
    true
}

/// Önbellekteki katalog (o anki şemanın görüntüsü; tutulduğu sürece değişmez). Henüz
/// yüklenmediyse boş. İlk çağrı yüklemeyi arka planda başlatır, bitince
/// [`Event::Widgets`](super::Event::Widgets); dil değişince [`reload`].
pub fn catalog() -> Catalog {
    // Birim testleri yerel yedeklerle çalışır: ortamdaki (PATH/NUXUT_SHELL_BIN) bir shell'in
    // şeması testin ortasında yüklenip varsayılanları/seçenekleri değiştirmesin. Gerçek şema
    // testleri onu açıkça okur (`real_shell_schema`); yeniden yükleme testleri kendi sahte
    // shell'iyle `load_from` + `install` çağırır.
    if !cfg!(test) {
        STARTED.call_once(|| spawn_load(GENERATION.load(Ordering::SeqCst)));
    }
    Catalog(current())
}

/// Şemayı shell'den yeniden okur (arka planda; bitince [`Event::Widgets`](super::Event::Widgets)).
/// Ayarlar'ın dili değişince çağrılır: `nuxut-shell schema` başlıkları ve seçenek adlarını
/// framework.toml `[locale]`'e göre üretir. Yükleme hiç başlamadıysa ilk yüklemeyi başlatır.
pub fn reload() {
    if cfg!(test) {
        return;
    }
    let seq = GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
    // İlk yükleme henüz başlamadıysa bu yükleme onun yerine geçer.
    STARTED.call_once(|| {});
    spawn_load(seq);
}

fn spawn_load(seq: u64) {
    std::thread::Builder::new()
        .name("widget-schema".into())
        .spawn(move || {
            if install(load(), seq) {
                super::emit(super::Event::Widgets);
            }
        })
        .ok();
}

/// [`catalog`]'un döndürdüğü görüntü: dilimi gibi kullanılır (`iter`, `is_empty`, `find`).
#[derive(Debug, Clone, Default)]
pub struct Catalog(Option<Arc<Schema>>);

impl std::ops::Deref for Catalog {
    type Target = [WidgetSpec];
    fn deref(&self) -> &[WidgetSpec] {
        self.0.as_ref().map_or(&[], |s| s.widgets.as_slice())
    }
}

/// Shell şemasındaki varsayılan; şema henüz yüklenmediyse, shell yoksa ya da anahtar şemada
/// yoksa `None` (çağıran [`crate::defaults::SHELL_DEFAULTS`]'a düşer). Yüklemeyi başlatmaz.
pub fn shell_default(path: &KeyPath) -> Option<&'static Value> {
    if cfg!(test) {
        return None;
    }
    let (last, parents) = path.segments().split_last()?;
    let mut table: &dyn toml_edit::TableLike = DEFAULTS.get()?.as_table();
    for seg in parents {
        table = table.get(seg)?.as_table_like()?;
    }
    table.get(last)?.as_value()
}

/// Shell şemasındaki ayar tanımı (`shell.toml` noktalı yolu: `wallpaper.backdrop`,
/// `bar.widget.workspaces.style`); şema henüz yüklenmediyse, shell yoksa ya da anahtar şemada
/// yoksa `None` — çağıran kendi yerel tablosuna düşer. Yüklemeyi başlatmaz. Paylaşılan tanım
/// (`Arc`): şema dil değişiminde yenilense de tutan için geçerli kalır.
pub fn shell_setting(path: &str) -> Option<Arc<SettingSpec>> {
    current()?.settings.get(path).cloned()
}

/// Şemadaki tüm genel ve widget ayarları (yol, tanım); şema yoksa boş. Denetim/uyarı için.
pub fn shell_settings() -> Vec<(String, Arc<SettingSpec>)> {
    current().map(|s| s.settings.iter().map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default()
}

/// Şema neden alınamadı (Ayarlar'da kullanıcıya gösterilir).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// `NUXUT_SHELL_BIN`, Ayarlar'ın yanı ve PATH'te `nuxut-shell` yok.
    NotFound,
    /// Bulundu ama başlatılamadı.
    Spawn(String),
    /// Süre aşıldı.
    Timeout,
    /// Çalıştı, başarısız çıkış kodu.
    Failed(Option<i32>),
    /// Çıktı çok büyük, UTF-8 değil ya da şema sürümü uymuyor.
    Unparsable,
}

impl LoadError {
    /// Kullanıcıya gösterilen neden (etkin dilde; katalog anahtarları `settings-sections-error-*`
    /// ve `settings-widgets-error-*`).
    pub fn message(&self) -> String {
        match self {
            LoadError::NotFound => t!("settings-sections-error-not-found"),
            LoadError::Spawn(e) => t!("settings-sections-error-spawn", error = e),
            LoadError::Timeout => t!("settings-sections-error-timeout"),
            LoadError::Failed(Some(c)) => t!("settings-widgets-error-exit-code", code = c),
            LoadError::Failed(None) => t!("settings-widgets-error-signal"),
            LoadError::Unparsable => t!("settings-sections-error-unparsable"),
        }
    }
}

/// Katalog yüklemesinin durumu: `None` = sürüyor, `Some(Ok)` = katalog geldi, `Some(Err)` = neden.
pub fn load_state() -> Option<Result<(), LoadError>> {
    let s = current()?;
    Some(s.error.clone().map_or(Ok(()), Err))
}

/// Testler: bu iş parçacığının şemasını verilen metinle kurar (yüklenmiş gibi).
#[cfg(test)]
pub fn set_test_schema(text: &str) {
    store(Schema::from_text(text));
}

/// `nuxut-shell` ikilisi: `NUXUT_SHELL_BIN` → Ayarlar'ın kendi dizini (install.sh hepsini aynı
/// yere, ör. ~/.local/bin, koyar; oturum PATH'inde olmayabilir) → PATH. Ayarlar'ın shell'i
/// çalıştırdığı her çağrı bunu kullanır.
pub fn shell_bin() -> Option<PathBuf> {
    resolve_shell_bin(std::env::var_os("NUXUT_SHELL_BIN"), std::env::current_exe().ok(), std::env::var_os("PATH"))
}

fn resolve_shell_bin(env: Option<OsString>, exe: Option<PathBuf>, path: Option<OsString>) -> Option<PathBuf> {
    if let Some(p) = env.filter(|p| !p.is_empty()) {
        return Some(PathBuf::from(p));
    }
    let beside = exe.and_then(|e| Some(e.parent()?.join(SHELL_BIN)));
    let in_path = path.into_iter().flat_map(|p| std::env::split_paths(&p).collect::<Vec<_>>()).filter(|d| d.is_absolute()).map(|d| d.join(SHELL_BIN));
    beside.into_iter().chain(in_path).find(|p| is_executable(p))
}

const SHELL_BIN: &str = "nuxut-shell";

fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(p).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

/// `nuxut-shell schema`, zaman aşımı ve çıktı sınırıyla; stderr debug log'a.
fn load() -> Schema {
    match shell_bin() {
        Some(bin) => load_from(&bin),
        None => failed(LoadError::NotFound),
    }
}

/// `bin schema` çıktısından şema. Varsayılanlar ilk başarılı yüklemede bir kez saklanır.
fn load_from(bin: &Path) -> Schema {
    match run_schema(bin) {
        Ok(text) => {
            let schema = Schema::from_text(&text);
            if schema.error.is_none() && !cfg!(test) {
                DEFAULTS.get_or_init(|| parse_defaults(&text));
            }
            schema
        }
        Err(e) => failed(e),
    }
}

fn failed(e: LoadError) -> Schema {
    tracing::warn!("{}", e.message());
    Schema { error: Some(e), ..Default::default() }
}

impl Schema {
    fn from_text(text: &str) -> Self {
        Schema {
            widgets: parse(text),
            settings: parse_settings(text).into_iter().map(|(k, v)| (k, Arc::new(v))).collect(),
            config_version: parse_config_version(text),
            error: None,
        }
        .checked()
    }

    /// Çıktı geldi ama hiçbir widget ayrıştırılamadıysa (sürüm uyumsuz) bu da bir hatadır.
    fn checked(mut self) -> Self {
        if self.widgets.is_empty() {
            tracing::warn!("{}", LoadError::Unparsable.message());
            self.error = Some(LoadError::Unparsable);
        }
        self
    }
}

fn run_schema(bin: &Path) -> Result<String, LoadError> {
    let mut child = Command::new(bin)
        .arg("schema")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound { LoadError::NotFound } else { LoadError::Spawn(e.to_string()) })?;
    let read = |r: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            r.map(|r| r.take(MAX_OUTPUT + 1).read_to_end(&mut buf));
            buf
        })
    };
    let stdout = read(child.stdout.take().map(|s| Box::new(s) as Box<dyn Read + Send>));
    let stderr = read(child.stderr.take().map(|s| Box::new(s) as Box<dyn Read + Send>));
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break Some(s),
            Ok(None) if start.elapsed() < TIMEOUT => std::thread::sleep(Duration::from_millis(20)),
            _ => break None,
        }
    };
    let Some(status) = status else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(LoadError::Timeout);
    };
    let err = stderr.join().unwrap_or_default();
    if !err.is_empty() {
        tracing::debug!("{} schema stderr: {}", bin.display(), String::from_utf8_lossy(&err).trim());
    }
    if !status.success() {
        return Err(LoadError::Failed(status.code()));
    }
    let buf = stdout.join().unwrap_or_default();
    if buf.len() as u64 > MAX_OUTPUT {
        return Err(LoadError::Unparsable);
    }
    String::from_utf8(buf).map_err(|_| LoadError::Unparsable)
}

/// SÖZLEŞME testleri için gerçek shell şeması: kardeş shell crate'inin `schema::generate`'i süreç
/// içinde (dev-bağımlılık `nuxut-shell`; `nuxut-shell schema` aynı işlevi çağırır). Derlenmiş
/// ikiliye ya da PATH'e gidilmez: cargo shell kaynağı değişince bu testi yeniden derler, bayat
/// ikiliyle (ya da PATH'teki kurulu shell'le) yanlışlıkla geçme ve şema bulunamayınca sessizce
/// atlama olmaz (e413de1/86dc342 sınıfı; 2026-10-15). Başlıklar o anki süreç
/// kataloğuyla çözülür; sözleşme testleri başlık karşılaştırmaz.
#[cfg(test)]
pub fn real_shell_schema() -> String {
    static TEXT: OnceLock<String> = OnceLock::new();
    TEXT.get_or_init(|| nuxut_shell::schema::generate().expect("shell şeması üretilemedi")).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
version = 1

[[widget]]
name = "clock"
title = "Saat"
icon = "schedule"
bar = true
panel = false
[[widget.setting]]
key = "format"
title = "Saat biçimi"
kind = "text"
default = "%H:%M"
[[widget.setting]]
key = "future"
kind = "color"

[[widget]]
name = "active_window"
title = "Etkin pencere"
icon = "window"
[[widget.setting]]
key = "max_width"
title = "En çok genişlik"
kind = "float"
min = 40
max = 1200.0
unit = "px"
default = 360.0

[[widget]]
name = "panel-toggle"
[[widget.setting]]
key = "panel"
kind = "choice"
options = ["left", "right"]
default = "right"

[[widget]]
name = "clock"
title = "Tekrar"

[[widget]]
title = "Adsız"
"#;

    #[test]
    fn parses_catalog() {
        let w = parse(SAMPLE);
        assert_eq!(w.iter().map(|w| w.name.as_str()).collect::<Vec<_>>(), ["clock", "active_window", "panel-toggle"]);
        assert_eq!(w[0].title, "Saat");
        assert_eq!(w[0].settings.len(), 1, "bilinmeyen tür atlanır");
        assert_eq!(w[0].settings[0].default.as_ref().and_then(Value::as_str), Some("%H:%M"));
        assert_eq!(w[1].settings[0].kind, SettingKind::Float { min: Some(40.0), max: Some(1200.0), step: None, unit: "px".into() });
        assert!(w[1].bar && !w[1].panel);
        assert_eq!(w[2].title, "panel-toggle");
        assert_eq!(w[2].settings[0].kind, SettingKind::Choice(vec!["left".into(), "right".into()]));
    }

    #[test]
    fn inline_arrays_and_general_settings() {
        let text = r#"
version = 1
widget = [{ name = "clock", title = "Saat", icon = "schedule", bar = true, panel = false, setting = [{ key = "seconds", title = "Saniye", kind = "bool", default = false }] }, { name = "tray", setting = [] }]

[[setting]]
key = "bar.size"
title = "Bar boyu"
kind = "int"
default = 36
min = 16.0
max = 128.0
step = 1.0
"#;
        let w = parse(text);
        assert_eq!(w.iter().map(|w| w.name.as_str()).collect::<Vec<_>>(), ["clock", "tray"]);
        assert_eq!(w[0].settings[0].kind, SettingKind::Bool);
        assert!(w[1].settings.is_empty());
    }

    /// Gerçek `nuxut-shell schema` çıktısından kesit (shell b76384f): tam sayı sınırları
    /// ondalık (`1.0`) yazılıyor, çok satırlı varsayılan, genel `[[setting]]` widget'lardan sonra.
    #[test]
    fn real_shell_output() {
        let text = r#"version = 1

[[widget]]
name = "workspaces"
title = "Çalışma alanları"
icon = "grid"
bar = true
panel = false

[[widget.setting]]
key = "count"
title = "Workspace sayısı"
kind = "int"
default = 10
min = 1.0
max = 50.0
step = 1.0

[[widget]]
name = "clock"
title = "Saat"
icon = "schedule"
bar = true
panel = false

[[widget.setting]]
key = "vertical_format"
title = "Dikey bar biçimi"
kind = "text"
default = """
%H
%M"""

[[widget]]
name = "volume"
title = "Ses"
icon = "volume_high"
bar = false
panel = true

[[setting]]
key = "bar.size"
kind = "int"
min = 16.0
"#;
        let w = parse(text);
        assert_eq!(w.iter().map(|w| w.name.as_str()).collect::<Vec<_>>(), ["workspaces", "clock", "volume"]);
        assert_eq!(w[0].settings[0].kind, SettingKind::Int { min: Some(1.0), max: Some(50.0), unit: String::new() });
        assert_eq!(w[1].settings[0].default.as_ref().and_then(Value::as_str), Some("%H\n%M"));
        assert!(!w[2].bar && w[2].panel && w[2].settings.is_empty());
        assert!(w.iter().all(|w| w.icon.parse::<nuxut::ui::IconName>().is_ok()), "ikonlar framework adları");
    }

    #[test]
    fn defaults_document() {
        let text = r#"version = 1

[[widget]]
name = "panel-toggle"
[[widget.setting]]
key = "panel"
kind = "choice"
options = ["left", "right"]
default = "right"
[[widget.setting]]
key = "nodefault"
kind = "text"

[[setting]]
key = "bar.edge_actions.end.scroll"
kind = "choice"
default = "volume"
options = ["widget", "volume"]

[[setting]]
key = "bar.edge_actions.scroll_step"
kind = "int"
default = 5
min = 1.0
"#;
        let d = parse_defaults(text);
        let get = |p: &str| {
            let path = KeyPath::parse(p);
            let (last, parents) = path.segments().split_last().unwrap();
            let mut t: &dyn toml_edit::TableLike = d.as_table();
            for s in parents {
                t = t.get(s)?.as_table_like()?;
            }
            t.get(last)?.as_value().cloned()
        };
        assert_eq!(get("bar.edge_actions.end.scroll").as_ref().and_then(Value::as_str), Some("volume"));
        assert_eq!(get("bar.edge_actions.scroll_step").as_ref().and_then(Value::as_integer), Some(5));
        assert_eq!(get("bar.widget.panel-toggle.panel").as_ref().and_then(Value::as_str), Some("right"));
        assert!(get("bar.widget.panel-toggle.nodefault").is_none());
        assert!(parse_defaults(&text.replace("version = 1", "version = 2")).as_table().is_empty());
    }

    #[test]
    fn broken_input_is_empty() {
        assert!(parse("not = [toml").is_empty());
        assert!(parse("widget = 3").is_empty());
        assert!(parse(&SAMPLE.replace("version = 1", "version = 2")).is_empty(), "bilinmeyen sürüm");
        assert!(parse(&SAMPLE.replace("version = 1", "")).is_empty(), "sürümsüz");
    }

    #[test]
    fn config_version_from_schema() {
        assert_eq!(parse_config_version("version = 1\nconfig_version = 2\n"), Some(2));
        assert_eq!(parse_config_version(SAMPLE), None, "eski shell");
        assert_eq!(parse_config_version("version = 2\nconfig_version = 2\n"), None, "bilinmeyen şema sürümü");
        assert_eq!(parse_config_version("version = 1\nconfig_version = 1\n"), None, "v1 damgalanmaz");
    }

    #[test]
    fn settings_index_with_validation_fields() {
        let text = r#"version = 1

[[widget]]
name = "workspaces"
since = 2
[[widget.setting]]
key = "style"
kind = "choice"
default = "icons"
options = ["icons", "numbers", "icons", "", 3, "dots"]
since = 2
[[widget.setting]]
key = "count"
kind = "int"
default = 10
min = 1
max = 30
step = 1
since = 1

[[widget]]
name = "clock"

[[setting]]
key = "wallpaper.backdrop_blur"
kind = "int"
default = 48
min = 4
max = 128
since = 2

[[setting]]
key = "bar.size"
kind = "float"
min = 64.0
max = 16.0

[[setting]]
key = "osd.timeout_ms"
kind = "int"
unit = "ms"
min = 500
max = nan

[[setting]]
key = "wallpaper.backdrop_blur"
kind = "int"
min = 0
max = 1

[[setting]]
key = "future.thing"
kind = "color"
since = "2"
"#;
        let s = parse_settings(text);
        let style = &s["bar.widget.workspaces.style"];
        assert_eq!(style.options(), Some(&["icons".to_owned(), "numbers".into(), "dots".into()][..]), "tekrar, boş ve dize olmayan atlanır");
        assert_eq!(style.since, Some(2));
        let count = &s["bar.widget.workspaces.count"];
        assert_eq!((count.bounds(), count.since), (Some((1.0, 30.0)), None), "since = 1 yazılmaz, yok sayılır");
        let blur = &s["wallpaper.backdrop_blur"];
        assert_eq!((blur.bounds(), blur.since), (Some((4.0, 128.0)), Some(2)), "yol tekrarında ilki");
        assert_eq!(s["bar.size"].bounds(), None, "min > max: sınır yok");
        assert_eq!(s["osd.timeout_ms"].kind, SettingKind::Int { min: Some(500.0), max: None, unit: "ms".into() }, "sonsuz/NaN sınır yok sayılır");
        assert!(!s.contains_key("future.thing"), "bilinmeyen tür");
        assert_eq!(parse(text)[0].since, Some(2));
        assert_eq!(parse(text)[1].since, None);
        assert!(parse_settings(&text.replace("version = 1", "version = 2")).is_empty());
        assert!(parse_settings("not = [toml").is_empty());
    }


    /// Gerçek şemanın (bkz. [`real_shell_schema`]) her ayarı ayrıştırılır ve
    /// widget kataloğuyla tutarlıdır.
    #[test]
    fn real_schema_parses_completely() {
        let text = real_shell_schema();
        let s = parse_settings(&text);
        let catalog = parse(&text);
        assert!(!catalog.is_empty() && !s.is_empty());
        for w in &catalog {
            for ws in &w.settings {
                assert!(s.contains_key(&format!("bar.widget.{}.{}", w.name, ws.key)), "{}.{}", w.name, ws.key);
            }
        }
        for (k, v) in &s {
            if let SettingKind::Int { .. } | SettingKind::Float { .. } = v.kind {
                assert!(v.bounds().is_some(), "{k}: sayı ayarı sınırsız");
            }
        }
    }

    fn fake_bin(dir: &Path, mode: u32) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        std::fs::create_dir_all(dir).unwrap();
        let p = dir.join(SHELL_BIN);
        std::fs::write(&p, "#!/bin/sh\n").unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(mode)).unwrap();
        p
    }

    /// Sahte shell: `schema` çağrısında `out` dosyasını basar (dil değişimini dosyayı
    /// değiştirerek taklit eder; gerçek shell framework.toml `[locale]`'e göre üretir).
    fn fake_shell(dir: &Path) -> (PathBuf, PathBuf) {
        use std::os::unix::fs::PermissionsExt;
        let out = dir.join("schema.toml");
        let bin = dir.join(SHELL_BIN);
        std::fs::write(&bin, format!("#!/bin/sh\n[ \"$1\" = schema ] || exit 2\nexec cat '{}'\n", out.display())).unwrap();
        std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).unwrap();
        (bin, out)
    }

    fn lang_schema(widget: &str, setting: &str, labels: &str) -> String {
        format!(
            "version = 1\n[[widget]]\nname = \"session\"\ntitle = \"{widget}\"\n[[widget.setting]]\nkey = \"panel\"\ntitle = \"{setting}\"\nkind = \"choice\"\noptions = [\"left\", \"right\"]\noption_labels = {labels}\n"
        )
    }

    /// Dil değişince şema shell'den yeniden okunur ve yenisiyle değiştirilir; eski görüntüyü
    /// tutan (`Catalog`, `Arc<SettingSpec>`) kendi kopyasıyla geçerli kalır.
    #[test]
    fn reload_swaps_schema_per_locale() {
        let d = tempfile::tempdir().unwrap();
        let (bin, out) = fake_shell(d.path());
        let seq = || GENERATION.load(Ordering::SeqCst);
        std::fs::write(&out, lang_schema("Oturum", "Açtığı panel", r#"["Sol", "Sağ"]"#)).unwrap();
        assert!(install(load_from(&bin), seq()));
        let tr_catalog = catalog();
        let tr_panel = shell_setting("bar.widget.session.panel").unwrap();
        assert_eq!(tr_catalog[0].title, "Oturum");
        assert_eq!(tr_panel.option_label("left"), Some("Sol"));

        std::fs::write(&out, lang_schema("Session", "Opens panel", r#"["Left", "Right"]"#)).unwrap();
        assert!(install(load_from(&bin), seq()));
        assert_eq!(catalog()[0].title, "Session");
        let en_panel = shell_setting("bar.widget.session.panel").unwrap();
        assert_eq!((en_panel.title.as_str(), en_panel.option_label("right")), ("Opens panel", Some("Right")));
        assert_eq!((tr_catalog[0].title.as_str(), tr_panel.option_label("left")), ("Oturum", Some("Sol")), "eski görüntü değişmez");
        assert_eq!(load_state(), Some(Ok(())));

        // Yeniden okuma başarısız (shell bozuldu): önceki şema kalır.
        std::fs::write(&out, "bozuk = [").unwrap();
        assert!(!install(load_from(&bin), seq()));
        assert_eq!(catalog()[0].title, "Session");
        std::fs::remove_file(&bin).unwrap();
        assert!(!install(load_from(&bin), seq()));
        assert_eq!(shell_setting("bar.widget.session.panel").unwrap().option_label("left"), Some("Left"));

        // Eski sıra numarasıyla gelen sonuç (araya yeni istek girdi) atılır.
        assert!(!install(Schema::from_text(&lang_schema("Eski", "Eski", r#"["S", "S2"]"#)), seq().wrapping_add(1)));
        assert_eq!(catalog()[0].title, "Session");
    }

    #[test]
    fn option_labels_parsed_with_options() {
        let s = parse_settings(&lang_schema("W", "S", r#"["Sol", "Sağ"]"#));
        let p = &s["bar.widget.session.panel"];
        assert_eq!(p.option_labels, ["Sol", "Sağ"]);
        assert_eq!((p.option_label("left"), p.option_label("nope")), (Some("Sol"), None));
        let s = parse_settings(&lang_schema("W", "S", r#"["Sol"]"#));
        assert!(s["bar.widget.session.panel"].option_labels.is_empty(), "uzunluk uymuyor");
        assert_eq!(s["bar.widget.session.panel"].option_label("left"), None);
        let s = parse_settings(&lang_schema("W", "S", "3"));
        assert!(s["bar.widget.session.panel"].option_labels.is_empty(), "dizi değil");
    }

    #[test]
    fn shell_bin_beside_settings_without_path() {
        let d = tempfile::tempdir().unwrap();
        let bin = fake_bin(&d.path().join("bin"), 0o755);
        let exe = d.path().join("bin/nuxut-settings");
        assert_eq!(resolve_shell_bin(None, Some(exe.clone()), None), Some(bin.clone()), "PATH yok");
        assert_eq!(resolve_shell_bin(None, Some(exe), Some(OsString::new())), Some(bin), "PATH boş");
    }

    #[test]
    fn shell_bin_order_env_then_beside_then_path() {
        let d = tempfile::tempdir().unwrap();
        let beside = fake_bin(&d.path().join("a"), 0o755);
        let in_path = fake_bin(&d.path().join("b"), 0o755);
        let exe = Some(d.path().join("a/nuxut-settings"));
        let path = Some(std::env::join_paths([d.path().join("b")]).unwrap());
        assert_eq!(resolve_shell_bin(Some("/özel/nuxut-shell".into()), exe.clone(), path.clone()), Some(PathBuf::from("/özel/nuxut-shell")));
        assert_eq!(resolve_shell_bin(Some(OsString::new()), exe.clone(), path.clone()), Some(beside), "boş değişken yok sayılır");
        assert_eq!(resolve_shell_bin(None, Some(d.path().join("c/nuxut-settings")), path), Some(in_path), "yanında yoksa PATH");
    }

    #[test]
    fn shell_bin_skips_non_executable_and_relative_path_entries() {
        let d = tempfile::tempdir().unwrap();
        fake_bin(&d.path().join("a"), 0o644);
        let exe = Some(d.path().join("a/nuxut-settings"));
        assert_eq!(resolve_shell_bin(None, exe.clone(), Some(".:bin".into())), None);
        assert_eq!(resolve_shell_bin(None, exe, None), None);
    }
}

/// `f32` kökenli sınırı 7 anlamlı basamağa (f32 kesinliği) yuvarlar (`0.10000000149011612` → `0.1`); tam sayılar
/// ve zaten temiz değerler değişmez.
fn tidy(x: f64) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let scale = 10f64.powi(6 - x.abs().log10().floor() as i32);
    (x * scale).round() / scale
}

#[cfg(test)]
mod tidy_tests {
    #[test]
    fn f32_bounds_are_tidied() {
        assert_eq!(super::tidy(f64::from(0.1f32)), 0.1);
        assert_eq!(super::tidy(f64::from(3.0f32)), 3.0);
        assert_eq!(super::tidy(f64::from(0.05f32)), 0.05);
        assert_eq!(super::tidy(4000.0), 4000.0);
        assert_eq!(super::tidy(-1.5), -1.5);
    }
}
