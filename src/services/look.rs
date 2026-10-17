//! Birleşik görünüm (shell ADR-0004): kurulu ikon/imleç temaları ve shell'in uygulama durumu.
//!
//! - [`scan`]: kurulu temaların listesi — framework `nuxut_image::icon::{icon_themes_in,
//!   cursor_themes_in}` (§13-4; `installed_*_themes()`'in dizin parametreli hâli) üzerine
//!   Ayarlar'ın seçici kuralları. Arka planda bir kez çalışır ([`themes`]).
//! - [`LookStatus`]: `nuxut-shell msg look status` yanıtı (ADR-0004 §9). Ayarlar dış hedeflere
//!   (GTK/Qt/ikon/imleç) yazmaz; yalnızca `framework.toml [desktop]` + `[colors] mode` yazar ve
//!   durumu buradan okur (ADR-0004 §12-1).
//!
//! Dosya sistemi kuralları (tek düzey, index.theme ≤ 64 KiB normal dosya, ≤ 512 tema, symlink
//! döngüsü, aynı adda ilk dizinin index'i, imleç = `cursors/` ya da hyprcursor manifesti)
//! framework'tedir; burada yalnızca seçici süzgeci kalır: `Hidden` ikon teması (yedek
//! `Adwaita` hariç) ve `hicolor` listelenmez, kimlik config'e yazılabilir olmalı
//! ([`valid_theme_name`]), ad [`MAX_THEME_NAME`] karakterle sınırlı, sıra Ayarlar'ın Türkçe
//! katlamasıyla.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use nuxut::i18n::t;
use nuxut::theme::{MAX_THEME_NAME, valid_theme_name};

use super::shell_ipc::{IpcLink, LinkError, ShellLink};

/// İkon ve imleç için yedek tema (shell `FALLBACK_*_THEME`); gizli işaretli olsa da listelenir.
pub const FALLBACK_THEME: &str = "Adwaita";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeKind {
    Icons,
    Cursors,
}

/// Kurulu tema: `id` dizin adıdır (config'e yazılan), `name` index.theme'deki görünen ad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledTheme {
    pub id: String,
    pub name: String,
}

fn installed(id: String, name: String) -> Option<InstalledTheme> {
    valid_theme_name(&id).then(|| InstalledTheme { name: name.chars().take(MAX_THEME_NAME).collect(), id })
}

/// `dirs` (öncelik sırasıyla) altındaki kurulu temalar (bkz. modül belgesi). Görünen ada göre
/// (Türkçe katlamayla, büyük/küçük harf duyarsız) sıralı.
pub fn scan(kind: ThemeKind, dirs: &[PathBuf]) -> Vec<InstalledTheme> {
    let mut out: Vec<InstalledTheme> = match kind {
        ThemeKind::Icons => nuxut_image::icon::icon_themes_in(dirs)
            .into_iter()
            // `hicolor` herkesin yedeği: seçenek değil. Adwaita `Hidden=true` taşır (GNOME
            // araçlarında gizli) ama ADR-0004'ün zararsız yedeğidir.
            .filter(|t| t.id != "hicolor" && (!t.hidden || t.id == FALLBACK_THEME))
            .filter_map(|t| installed(t.id, t.name))
            .collect(),
        // `Hidden=true` imleç temaları da seçenek değil (ikon süzgeciyle aynı kural).
        ThemeKind::Cursors => nuxut_image::icon::cursor_themes_in(dirs).into_iter().filter(|t| !t.hidden).filter_map(|t| installed(t.id, t.name)).collect(),
    };
    out.sort_by(|a, b| crate::registry::fold(&a.name).cmp(&crate::registry::fold(&b.name)).then_with(|| a.id.cmp(&b.id)));
    out
}

/// Kurulu temalar (her iki tür).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Themes {
    pub icons: Vec<InstalledTheme>,
    pub cursors: Vec<InstalledTheme>,
}

impl Themes {
    pub fn of(&self, kind: ThemeKind) -> &[InstalledTheme] {
        match kind {
            ThemeKind::Icons => &self.icons,
            ThemeKind::Cursors => &self.cursors,
        }
    }
}

static THEMES: Mutex<Option<Arc<Themes>>> = Mutex::new(None);
/// (tarama sürüyor mu, bittiğinde bir kez daha tara)
static SCANNING: Mutex<(bool, bool)> = Mutex::new((false, false));

/// Önbellekteki liste; yoksa taramayı arka planda başlatır, bitince
/// [`Event::Look`](super::Event::Look).
pub fn themes() -> Option<Arc<Themes>> {
    if let Some(t) = THEMES.lock().unwrap_or_else(|e| e.into_inner()).clone() {
        return Some(t);
    }
    let mut s = SCANNING.lock().unwrap_or_else(|e| e.into_inner());
    if !s.0 {
        start_scan(&mut s);
    }
    None
}

static THEMES_GENERATION: AtomicU64 = AtomicU64::new(0);

/// Kurulu tema kuşağı: her yeniden taramada artar. Önizleme ızgarası değişimi görünce kart
/// önizlemelerini yeniden ister (aynı adla güncellenen tema da yenilenir).
pub fn themes_generation() -> u64 {
    THEMES_GENERATION.load(Ordering::Relaxed)
}

/// Kurulu temaları yeniden tarar (shell `themes_rev` değişti). Eski liste yeni tarama bitene
/// kadar geçerli kalır (ızgara boşalıp dolmaz); tarama sürüyorsa bittiğinde bir kez daha.
fn rescan() {
    // Kurulum/güncelleme sırasında yarım dosyadan çözülmüş ("yok") önizlemeler kalmasın:
    // önbellek boşalır, ızgaralar kuşak değişiminde yuvalarını yeniden ister.
    super::previews::Previews::shared().cache().clear();
    THEMES_GENERATION.fetch_add(1, Ordering::Relaxed);
    let mut s = SCANNING.lock().unwrap_or_else(|e| e.into_inner());
    if s.0 {
        s.1 = true;
    } else {
        start_scan(&mut s);
    }
}

fn start_scan(s: &mut (bool, bool)) {
    s.0 = true;
    let spawned = std::thread::Builder::new().name("look-themes".into()).spawn(|| {
        loop {
            let dirs = nuxut_image::icon::default_base_dirs();
            let t = Themes { icons: scan(ThemeKind::Icons, &dirs), cursors: scan(ThemeKind::Cursors, &dirs) };
            *THEMES.lock().unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(t));
            super::emit(super::Event::Look);
            let mut s = SCANNING.lock().unwrap_or_else(|e| e.into_inner());
            if std::mem::take(&mut s.1) {
                continue;
            }
            s.0 = false;
            break;
        }
    });
    if spawned.is_err() {
        s.0 = false;
    }
}

/// Son görülen `themes_rev`; `None` = henüz görülmedi.
static THEMES_REV: Mutex<Option<u64>> = Mutex::new(None);

/// Yeni `themes_rev` değerini kaydeder; önceki görülen değerden farklıysa (azalma dahil: shell
/// her başlangıçta 0'dan sayar) `true` — kurulu liste yeniden okunmalı. İlk değer yalnızca
/// kaydedilir (liste zaten ilk açılışta taranır). Değer yoksa (sürüm < 4) hiçbir şey olmaz.
fn themes_rev_changed(last: &mut Option<u64>, now: Option<u64>) -> bool {
    let Some(now) = now else { return false };
    last.replace(now).is_some_and(|prev| prev != now)
}

// ---- msg look status ------------------------------------------------------------------------

/// Son uygulamanın sonucu (`last_apply`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LastApply {
    pub ok: bool,
    pub errors: usize,
    pub skipped: usize,
    /// Başka bir aracın yönettiği, bu yüzden yazılmayan dosyalar (sürüm 5; shell
    /// `docs/design/look-external.md`). Ayarın geri kalanı yazıldı.
    pub external: Vec<ExternalFile>,
    /// Nuxut'un devraldığı, önceden başka aracın yönettiği dosyalar (`~/…`; sürüm 6). Birleşik
    /// görünümü kapatmak özgün hâllerini (bağ, izin) geri koyar.
    pub taken_over: Vec<String>,
    /// Yeniden yazım döngüsü bekçisinin geçici olarak durdurduğu ayar sayısı (sürüm 5).
    pub paused: usize,
}

/// Dokunulmayan dosyanın nedeni (`nuxut_apply::ExternalReason`, shell üzerinden), kullanıcıya söylenecek
/// kümelere indirgenmiş.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalReason {
    /// `read_only` | `hard_link` | `symlink_broken` | `unknown` (ve bilinmeyen yeni nedenler).
    /// Shell sürüm 6 bunları varsayılan olarak devralır; buraya yalnız devralma başarısızsa düşer.
    Managed,
    /// `not_owned`: başka kullanıcıya ait (çoğunlukla root); Nuxut değiştiremez.
    NotOwned,
    /// `store`: Nix/Guix deposu; değişiklik oradan yapılır.
    Store,
    /// `dir_not_writable`: dosyanın klasörüne yazılamıyor.
    DirNotWritable,
}

impl ExternalReason {
    fn parse(s: &str) -> Self {
        match s {
            "not_owned" => Self::NotOwned,
            "store" => Self::Store,
            "dir_not_writable" => Self::DirNotWritable,
            _ => Self::Managed,
        }
    }
}

/// `last_apply.external[]` öğesi: `{setting, path: "~/…", reason}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalFile {
    /// Shell ayar kimliği (`look.mode`, `look.icons`…); metindeki "neyi almıyor" buradan.
    pub setting: String,
    pub path: String,
    pub reason: ExternalReason,
}

/// `last_apply.taken_over[]` öğesinin yolu (`{setting, path}`; `parse_external` ile aynı sınırlar).
fn parse_taken_over(v: &serde_json::Value) -> Option<String> {
    let path = v.get("path")?.as_str()?;
    ((path.starts_with("~/") || path.starts_with('/')) && path.len() <= 256 && !path.chars().any(char::is_control)).then(|| path.to_owned())
}

/// Raporda tutulan dış dosya üst sınırı (shell `MAX_EXTERNAL` 32).
const MAX_EXTERNAL: usize = 32;
/// Durum satırında adıyla anılan dosya sayısı; kalanı "N dosya daha".
const SHOWN_EXTERNAL: usize = 2;

/// `{setting, path, reason}`; yol `~/` ya da `/` ile başlamalı, ≤ 256 bayt, denetim karakteri yok.
fn parse_external(v: &serde_json::Value) -> Option<ExternalFile> {
    let path = v.get("path")?.as_str().filter(|p| (p.starts_with("~/") || p.starts_with('/')) && p.len() <= 256 && !p.chars().any(char::is_control))?;
    Some(ExternalFile {
        setting: v.get("setting").and_then(|x| x.as_str()).filter(|s| s.len() <= 64).unwrap_or_default().to_owned(),
        path: path.to_owned(),
        reason: ExternalReason::parse(v.get("reason").and_then(|x| x.as_str()).unwrap_or_default()),
    })
}

/// Paketin deposu (shell `missing_packages[].source`, `*.catalog[].source`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageSource {
    /// Resmi depo (`pacman`).
    Repo,
    /// AUR (`paru`).
    Aur,
}

/// Eksik paketin ait olduğu görünüm hedefi (shell `missing_packages[].target`, sürüm 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageTarget {
    Icons,
    Cursors,
    Gtk,
    Portal,
    Qt,
}

impl PackageTarget {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "icons" => Self::Icons,
            "cursors" => Self::Cursors,
            "gtk" => Self::Gtk,
            "portal" => Self::Portal,
            "qt" => Self::Qt,
            _ => return None,
        })
    }

    /// İkon/imleç paketleri kendi ızgaralarında (katalogdan) gösterilir; genel not onları saymaz.
    fn is_theme(self) -> bool {
        matches!(self, Self::Icons | Self::Cursors)
    }
}

/// Eksik paket: Arch adı, deposu ve (sürüm 3) hedefi. Hepsi shell'den gelir; Ayarlar addan
/// tahmin etmez. `target` yoksa (eski shell) paket genel notta gösterilir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingPackage {
    pub name: String,
    pub source: PackageSource,
    pub target: Option<PackageTarget>,
}

/// Kurulabilir tema ailesi (shell `icons.catalog[]` / `cursors.catalog[]`, sürüm 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogFamily {
    /// Kararlı kimlik (`vimix`).
    pub family: String,
    /// Görünen ad (özel ad; çevrilmez).
    pub name: String,
    pub package: String,
    pub source: PackageSource,
    /// Bu aileye ait kurulu tema kimlikleri (dizin adları); boş = aile kurulu değil.
    pub installed: Vec<String>,
}

impl CatalogFamily {
    /// Bu ailenin düzeltmesi: depo paketi "Kur" düğmesi, AUR kurulu yardımcının komutu ([`package_fix`]).
    pub fn fix(&self, helper: Option<AurHelper>) -> Fix {
        package_fix(self.source, &[self.package.as_str()], helper)
    }
}

// ---- AUR yardımcısı -------------------------------------------------------------------------

/// Kurulu AUR yardımcısı (kurulum komutunu kopyalanabilir kılmak için; Ayarlar çalıştırmaz).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AurHelper {
    Paru,
    Yay,
}

impl AurHelper {
    /// Arama sırası: paru, sonra yay.
    pub const ORDER: [Self; 2] = [Self::Paru, Self::Yay];

    pub fn command(self) -> &'static str {
        match self {
            Self::Paru => "paru",
            Self::Yay => "yay",
        }
    }
}

/// Yardımcı bulunamadığında AUR komutunun yanındaki kısa not, etkin dilde.
pub fn aur_helper_note() -> String {
    t!("settings-look-status-aur-helper-note")
}

/// `path` (PATH biçiminde, `:` ayrımlı) dizinlerinde çalıştırılabilir `paru`, yoksa `yay`. Süreç
/// başlatılmaz; yalnızca dosya bilgisine bakılır. Göreli ve boş PATH öğeleri yok sayılır
/// (çalışma dizinine bağlı sonuç olmasın).
pub fn find_aur_helper(path: Option<&std::ffi::OsStr>) -> Option<AurHelper> {
    use std::os::unix::fs::PermissionsExt;
    let dirs: Vec<PathBuf> = std::env::split_paths(path?).filter(|d| d.is_absolute()).collect();
    AurHelper::ORDER.into_iter().find(|h| {
        dirs.iter().any(|d| std::fs::metadata(d.join(h.command())).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0))
    })
}

/// Bu oturumda kurulu AUR yardımcısı (ortamdaki PATH; ilk soruda bir kez aranır).
pub fn aur_helper() -> Option<AurHelper> {
    static HELPER: std::sync::OnceLock<Option<AurHelper>> = std::sync::OnceLock::new();
    *HELPER.get_or_init(|| find_aur_helper(std::env::var_os("PATH").as_deref()))
}

/// Katalog öğesi sınırı (shell 8 aile verir).
pub const MAX_CATALOG: usize = 32;
/// Aile başına kurulu varyant sınırı (shell `MAX_VARIANTS`).
pub const MAX_VARIANTS: usize = 64;

/// Shell'in dış yazım arka ucu (`writer`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Writer {
    /// Shell uygular (alan yok ya da bilinen etkin değer).
    #[default]
    Active,
    /// Dış yazım bu shell sürümünde kapalı (`"disabled"`): seçimler kaydedilir, uygulanmaz.
    Disabled,
}

/// `msg look status` yanıtının Ayarlar'ın kullandığı kısmı (bilinmeyen alanlar yok sayılır).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LookStatus {
    pub unified: bool,
    /// Çözülmüş kip (`dark` | `light`; `[colors] mode = "auto"` iken de çözülmüş hâli).
    pub mode: Option<String>,
    pub icon_theme: Option<String>,
    pub cursor_theme: Option<String>,
    pub cursor_size: Option<u32>,
    /// Shell'in önerdiği ikon/imleç teması (`icons.recommended`, `cursors.recommended`; ADR-0004
    /// §12-10). Öneri shell'indir: Ayarlar kopya tutmaz; shell çalışmıyorsa adsız "Önerilen".
    pub icon_recommended: Option<String>,
    pub cursor_recommended: Option<String>,
    /// Kurulabilir aileler (sürüm 3); `None` = eski shell (katalog yok, tahmin de yok).
    pub icon_catalog: Option<Vec<CatalogFamily>>,
    pub cursor_catalog: Option<Vec<CatalogFamily>>,
    pub missing_packages: Vec<MissingPackage>,
    pub conflicts: usize,
    pub restart_needed: bool,
    pub writer: Writer,
    pub last_apply: Option<LastApply>,
    /// Kurulu tema sayacı (sürüm 4; shell `look::watch`): değişince kurulu liste yeniden okunur.
    pub themes_rev: Option<u64>,
}

/// Arch paket adı gibi mi (komut satırına kopyalanacak; kabuk özel karakteri yok).
fn valid_package(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"-_.+@".contains(&b))
}

fn parse_source(v: &serde_json::Value) -> Option<PackageSource> {
    match v.get("source")?.as_str()? {
        "repo" => Some(PackageSource::Repo),
        "aur" => Some(PackageSource::Aur),
        _ => None,
    }
}

/// `{name, source: "repo"|"aur", target?}`; geçersiz ad ya da bilinmeyen depo → atlanır.
/// Bilinmeyen hedef (gelecekteki shell) → hedefsiz sayılır, genel notta görünür.
fn parse_package(v: &serde_json::Value) -> Option<MissingPackage> {
    let name = v.get("name")?.as_str().filter(|n| valid_package(n))?;
    let source = parse_source(v)?;
    let target = v.get("target").and_then(|t| t.as_str()).and_then(PackageTarget::parse);
    Some(MissingPackage { name: name.to_owned(), source, target })
}

/// `{family, name, package, source, installed: [id…]}`; eksik/geçersiz alan → atlanır.
/// Geçersiz tema adı taşıyan kurulu kimlikler düşülür (config'e yazılabilir olmalı).
fn parse_family(v: &serde_json::Value) -> Option<CatalogFamily> {
    let field = |k: &str| v.get(k).and_then(|x| x.as_str()).filter(|s| !s.is_empty() && s.len() <= 64).map(str::to_owned);
    let installed = v.get("installed")?.as_array()?;
    Some(CatalogFamily {
        family: field("family")?,
        name: field("name")?,
        package: field("package").filter(|p| valid_package(p))?,
        source: parse_source(v)?,
        installed: installed.iter().filter_map(|x| x.as_str()).filter(|id| valid_theme_name(id)).take(MAX_VARIANTS).map(str::to_owned).collect(),
    })
}

/// `icons.catalog` / `cursors.catalog`; alan yoksa `None` (sürüm < 3).
fn parse_catalog(v: Option<&serde_json::Value>) -> Option<Vec<CatalogFamily>> {
    let a = v?.get("catalog")?.as_array()?;
    Some(a.iter().take(MAX_CATALOG).filter_map(parse_family).collect())
}

fn short(v: &serde_json::Value) -> Option<String> {
    v.as_str().filter(|s| s.len() <= 256).map(str::to_owned)
}

/// JSON yanıtı ayrıştırır. Sözleşme dışı (JSON değil, kök nesne değil) → `None`.
pub fn parse_status(text: &str) -> Option<LookStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    let list_len = |k: &str| o.get(k).and_then(|x| x.as_array()).map_or(0, |a| a.len().min(256));
    let last_apply = o.get("last_apply").and_then(|x| x.as_object()).map(|a| LastApply {
        ok: a.get("ok").and_then(|x| x.as_bool()).unwrap_or(false),
        errors: a.get("errors").and_then(|x| x.as_array()).map_or(0, Vec::len),
        skipped: a.get("skipped").and_then(|x| x.as_array()).map_or(0, Vec::len),
        external: a.get("external").and_then(|x| x.as_array()).map(|l| l.iter().filter_map(parse_external).take(MAX_EXTERNAL).collect()).unwrap_or_default(),
        paused: a.get("paused").and_then(|x| x.as_array()).map_or(0, |l| l.len().min(256)),
        taken_over: a.get("taken_over").and_then(|x| x.as_array()).map(|l| l.iter().filter_map(parse_taken_over).take(MAX_EXTERNAL).collect()).unwrap_or_default(),
    });
    Some(LookStatus {
        unified: o.get("unified").and_then(|x| x.as_bool()).unwrap_or(false),
        mode: o.get("mode").and_then(short),
        icon_theme: o.get("icons").and_then(|i| i.get("current")).and_then(short),
        cursor_theme: o.get("cursors").and_then(|i| i.get("current")).and_then(short),
        cursor_size: o.get("cursors").and_then(|i| i.get("size")).and_then(|x| x.as_u64()).and_then(|n| u32::try_from(n).ok()),
        icon_recommended: o.get("icons").and_then(|i| i.get("recommended")).and_then(short).filter(|n| valid_theme_name(n)),
        cursor_recommended: o.get("cursors").and_then(|i| i.get("recommended")).and_then(short).filter(|n| valid_theme_name(n)),
        icon_catalog: parse_catalog(o.get("icons")),
        cursor_catalog: parse_catalog(o.get("cursors")),
        missing_packages: o
            .get("missing_packages")
            .and_then(|x| x.as_array())
            .map(|a| a.iter().take(32).filter_map(parse_package).collect())
            .unwrap_or_default(),
        conflicts: list_len("conflicts"),
        restart_needed: list_len("restart_needed") > 0,
        writer: match o.get("writer").and_then(|x| x.as_str()) {
            Some("disabled") => Writer::Disabled,
            _ => Writer::Active,
        },
        last_apply,
        themes_rev: o.get("themes_rev").and_then(|x| x.as_u64()),
    })
}

/// Shell'den okunan son durum.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LookState {
    /// Henüz sorulmadı / yanıt bekleniyor.
    #[default]
    Pending,
    Ready(Box<LookStatus>),
    /// Shell çalışmıyor.
    NotRunning,
    /// Shell bu komutu tanımıyor (eski sürüm) ya da yanıt sözleşme dışı.
    Unsupported,
    /// Test örneği: bilerek sorulmadı.
    TestInstance,
    Failed(String),
}

/// Bir bağlantıdan durumu sorar (saf; testte sahte bağlantı).
pub fn query_status(link: &dyn ShellLink) -> LookState {
    match link.request(&["look", "status"]) {
        Ok(reply) => parse_status(&reply).map_or(LookState::Unsupported, |s| LookState::Ready(Box::new(s))),
        Err(LinkError::NotRunning) => LookState::NotRunning,
        Err(LinkError::TestInstance) => LookState::TestInstance,
        Err(LinkError::Shell(_) | LinkError::BadReply) => LookState::Unsupported,
        Err(e) => LookState::Failed(e.to_string()),
    }
}

/// Satır içi bildirimin tonu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Ok,
    Info,
    Warning,
}

/// Kullanıcıya gösterilecek durum: ton, metin ve (varsa) düğmeyle yapılacak düzeltme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLine {
    pub tone: Tone,
    pub text: String,
    pub fix: Option<Fix>,
}

/// Eksik paketin düzeltmesi (kullanıcı kararı 2026-10-17: kök komutu yazdırılmaz, kök gerektiren
/// düzeltme bir düğmeyle). Resmî depo paketleri "Kur" düğmesiyle kök yardımcının `packages`
/// alanından kurulur ([`super::packages`], yönetici onayı); AUR paketi kök olarak derlenmediği için
/// kopyalanacak yardımcı komutu (`paru -S` / `yay -S`) kalır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fix {
    Install(Vec<String>),
    Copy(String),
}

/// AUR paketini kuran satır: kurulu yardımcıyla (`paru -S` / `yay -S`; hiçbiri yoksa `paru -S` ve
/// arayüz [`aur_helper_note`]'u gösterir). Adlar `valid_package`'tan geçmiştir (kabuk özel
/// karakteri yok).
pub fn aur_line(names: &str, helper: Option<AurHelper>) -> String {
    format!("{} -S {names}", helper.unwrap_or(AurHelper::Paru).command())
}

/// Bir paketin (ya da aynı depodaki paketlerin) düzeltmesi.
pub fn package_fix(source: PackageSource, names: &[&str], helper: Option<AurHelper>) -> Fix {
    match source {
        PackageSource::Repo => Fix::Install(names.iter().map(|n| (*n).to_owned()).collect()),
        PackageSource::Aur => Fix::Copy(aur_line(&names.join(" "), helper)),
    }
}

/// AUR komutu var ama yardımcı kurulu değil: komutun yanında [`aur_helper_note`] gösterilir.
pub fn needs_helper_note(aur: bool, helper: Option<AurHelper>) -> bool {
    aur && helper.is_none()
}

/// Eksik paketlerin düzeltmesi: depo paketi varsa önce onlar ("Kur"; AUR olanlar metinde adıyla
/// kalır, depo paketleri kurulunca kopyalanacak komut görünür), yoksa AUR komutu (depo bilgisi
/// shell'den; addan tahmin yok).
pub fn missing_fix<'a>(packages: impl IntoIterator<Item = &'a MissingPackage> + Clone, helper: Option<AurHelper>) -> Option<Fix> {
    [PackageSource::Repo, PackageSource::Aur].into_iter().find_map(|src| {
        let names: Vec<&str> = packages.clone().into_iter().filter(|p| p.source == src).map(|p| p.name.as_str()).collect();
        (!names.is_empty()).then(|| package_fix(src, &names, helper))
    })
}

/// Genel notta gösterilecek eksik paketler: ikon/imleç hedefli olanlar ızgaralarda katalogdan
/// gösterildiği için düşülür; hedefsizler (sürüm < 3) kalır.
pub fn general_packages(s: &LookStatus) -> Vec<&MissingPackage> {
    s.missing_packages.iter().filter(|p| !p.target.is_some_and(PackageTarget::is_theme)).collect()
}

/// Eksik paket cümlesi ("2 ek bileşen gerekli: a, b."); AUR paketi var ve yardımcı yoksa not eklenir.
fn missing_text(packages: &[&MissingPackage], helper: Option<AurHelper>) -> String {
    let names: Vec<&str> = packages.iter().map(|p| p.name.as_str()).collect();
    let mut text = t!("settings-look-status-missing-packages", count = names.len(), names = names.join(", "));
    if needs_helper_note(packages.iter().any(|p| p.source == PackageSource::Aur), helper) {
        text = format!("{text} {}", t!("settings-look-status-aur-helper-note"));
    }
    text
}

/// Ayarın hangi parçasının uygulamalara ulaşmadığı (cümlenin nesnesi, etkin dilde).
fn external_what(setting: &str) -> String {
    match setting {
        "look.mode" => t!("settings-look-external-what-colors"),
        "look.icons" => t!("settings-look-external-what-icons"),
        "look.cursor" => t!("settings-look-external-what-cursor"),
        "look.fonts" => t!("settings-look-external-what-fonts"),
        _ => t!("settings-look-external-what-look"),
    }
}

/// Dış dosyaların cümlesi: ilk [`SHOWN_EXTERNAL`] dosya (aynı yol bir kez) neden + etkiyle,
/// kalanı "N dosya daha". Hitapsız; teknik ad yalnızca dosya yolu (docs/design/settings-ux.md).
/// Devralınan dosyalar: ilk dosya adıyla, kalanı sayıyla; geri alma yolu söylenir.
pub fn taken_over_text(paths: &[String]) -> String {
    let mut unique: Vec<&str> = Vec::new();
    for p in paths {
        if !unique.contains(&p.as_str()) {
            unique.push(p);
        }
    }
    match unique.as_slice() {
        [] => String::new(),
        [one] => t!("settings-look-taken-over-one", path = *one),
        [first, rest @ ..] => t!("settings-look-taken-over-many", path = *first, count = rest.len()),
    }
}

pub fn external_text(files: &[ExternalFile]) -> String {
    let mut unique: Vec<&ExternalFile> = Vec::new();
    for f in files {
        if !unique.iter().any(|u| u.path == f.path) {
            unique.push(f);
        }
    }
    let mut parts: Vec<String> = unique
        .iter()
        .take(SHOWN_EXTERNAL)
        .map(|f| {
            let path = f.path.as_str();
            let reason = match f.reason {
                ExternalReason::Managed => t!("settings-look-external-managed", path = path),
                ExternalReason::NotOwned => t!("settings-look-external-not-owned", path = path),
                ExternalReason::Store => t!("settings-look-external-store", path = path),
                ExternalReason::DirNotWritable => t!("settings-look-external-dir", path = path),
            };
            format!("{reason} {}", t!("settings-look-external-effect", what = external_what(&f.setting)))
        })
        .collect();
    let rest = unique.len().saturating_sub(SHOWN_EXTERNAL);
    if rest > 0 {
        parts.push(t!("settings-look-external-more", count = rest));
    }
    parts.join(" ")
}

/// Durumdan tek satırlık kullanıcı metni (teknik ad yok; ADR-0004 §9). AUR komutu bu oturumda
/// kurulu yardımcıyla ([`aur_helper`]).
pub fn status_line(state: &LookState) -> Option<StatusLine> {
    status_line_with(state, aur_helper())
}

/// [`status_line`], AUR yardımcısı verilerek (saf; testli).
pub fn status_line_with(state: &LookState, helper: Option<AurHelper>) -> Option<StatusLine> {
    let line = |tone, text: String| Some(StatusLine { tone, text, fix: None });
    match state {
        LookState::Pending => line(Tone::Info, t!("settings-look-status-pending")),
        LookState::NotRunning => line(Tone::Info, t!("settings-look-status-not-running")),
        LookState::Unsupported => line(Tone::Info, t!("settings-look-status-unsupported")),
        LookState::TestInstance => line(Tone::Info, t!("settings-look-status-test-instance")),
        LookState::Failed(_) => line(Tone::Warning, t!("settings-look-status-failed")),
        LookState::Ready(s) => {
            let missing = general_packages(s);
            // Shell'in dış yazımı bu sürümde kapalı: hata değil, sakin bir not. Eksik paketler
            // yine de söylenir (kurulunca hazır olur), ama uyarı tonunda değil.
            if s.writer == Writer::Disabled {
                let mut text = t!("settings-look-status-writer-disabled");
                if !missing.is_empty() {
                    text = format!("{text} {}", missing_text(&missing, helper));
                }
                return Some(StatusLine { tone: Tone::Info, text, fix: missing_fix(missing.iter().copied(), helper) });
            }
            if !missing.is_empty() {
                return Some(StatusLine { tone: Tone::Warning, text: missing_text(&missing, helper), fix: missing_fix(missing.iter().copied(), helper) });
            }
            if let Some(a) = &s.last_apply {
                // Gerçek hata önce (genel metin); yoksa dosya başına neden (sürüm 5), sonra
                // durdurulan ayarlar. Eski shell `external` göndermez: genel metne düşer.
                if a.errors == 0 && !a.external.is_empty() {
                    // Kök komutu (chown) önerilmez (kullanıcı kararı 2026-10-17).
                    return Some(StatusLine { tone: Tone::Warning, text: external_text(&a.external), fix: None });
                }
                if a.errors == 0 && a.paused > 0 {
                    return line(Tone::Warning, t!("settings-look-status-paused"));
                }
                if !a.ok || a.errors > 0 {
                    return line(Tone::Warning, t!("settings-look-status-apply-failed"));
                }
            }
            if !s.unified {
                return line(Tone::Info, t!("settings-look-status-applying"));
            }
            let mut text = t!("settings-look-status-applied");
            if s.restart_needed {
                text = format!("{text} {}", t!("settings-welcome-done-body"));
            }
            if let Some(a) = s.last_apply.as_ref().filter(|a| !a.taken_over.is_empty()) {
                text = format!("{text} {}", taken_over_text(&a.taken_over));
            }
            if s.conflicts > 0 {
                text = format!("{text} {}", t!("settings-look-status-conflicts"));
            }
            line(Tone::Ok, text)
        }
    }
}

static STATUS: Mutex<LookState> = Mutex::new(LookState::Pending);
/// (çalışan sorgu var mı, bittiğinde bir kez daha sor)
static QUERY: Mutex<(bool, bool)> = Mutex::new((false, false));

/// Son durum.
pub fn status() -> LookState {
    STATUS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

impl LookStatus {
    /// Bu türün kurulabilir aileleri (`None` = eski shell, katalog yok).
    pub fn catalog(&self, kind: ThemeKind) -> Option<&[CatalogFamily]> {
        match kind {
            ThemeKind::Icons => self.icon_catalog.as_deref(),
            ThemeKind::Cursors => self.cursor_catalog.as_deref(),
        }
    }
}

/// Shell'in bu tür için bildirdiği öneri ve katalog (durum hazır değilse ikisi de yok).
pub fn offer(kind: ThemeKind) -> (Option<String>, Option<Vec<CatalogFamily>>) {
    match status() {
        LookState::Ready(s) => {
            let rec = match kind {
                ThemeKind::Icons => s.icon_recommended.clone(),
                ThemeKind::Cursors => s.cursor_recommended.clone(),
            };
            (rec, s.catalog(kind).map(<[_]>::to_vec))
        }
        _ => (None, None),
    }
}

/// Bu süreçte durum en az bir kez soruldu mu (görünüm sayfası ya da Hoş geldin açıldı).
static ASKED: AtomicBool = AtomicBool::new(false);

/// Pencere odağı geri geldi: durum daha önce sorulduysa hemen yeniden sorar. Shell kurulu
/// temaları izler (`themes_rev`); kullanıcı terminalde tema kurup Ayarlar'a dönünce yeni tema
/// yeniden açmadan görünür. Shell'e tek istek; görünüm hiç açılmadıysa hiçbir şey yapmaz
/// (boşta iş yok, yoklama yok).
pub fn recheck() {
    if ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Durumu `delay` sonra yeniden sorar (shell framework.toml değişimini uygulasın diye
/// beklenir). Sorgu sürüyorsa bittiğinde bir kez daha sorulur; aynı anda tek thread.
pub fn refresh(delay: Duration) {
    ASKED.store(true, Ordering::Relaxed);
    let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
    if q.0 {
        q.1 = true;
        return;
    }
    q.0 = true;
    let spawned = std::thread::Builder::new().name("look-status".into()).spawn(move || {
        let link = IpcLink::from_env();
        let mut delay = delay;
        loop {
            std::thread::sleep(delay);
            let state = query_status(&link);
            if let LookState::Ready(s) = &state
                && themes_rev_changed(&mut THEMES_REV.lock().unwrap_or_else(|e| e.into_inner()), s.themes_rev)
            {
                rescan();
            }
            *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = state;
            super::emit(super::Event::Look);
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
    use std::path::Path;
    use crate::services::shell_ipc::FakeLink;

    fn theme_dir(base: &Path, id: &str, index: Option<&str>, cursors: bool) {
        let d = base.join(id);
        std::fs::create_dir_all(&d).unwrap();
        if let Some(i) = index {
            std::fs::write(d.join("index.theme"), i).unwrap();
        }
        if cursors {
            std::fs::create_dir_all(d.join("cursors")).unwrap();
        }
    }

    #[test]
    fn scans_icon_and_cursor_themes_in_priority_order() {
        let tmp = tempfile::tempdir().unwrap();
        let user = tmp.path().join("user");
        let sys = tmp.path().join("sys");
        theme_dir(&sys, "Papirus-Dark", Some("[Icon Theme]\nName=Papirus-Dark\nName[tr]=Papirus Koyu\nDirectories=16x16/apps\n"), false);
        theme_dir(&sys, "Adwaita", Some("[Icon Theme]\nName=Adwaita\nHidden=true\nDirectories=scalable/apps\n"), true);
        theme_dir(&sys, "hicolor", Some("[Icon Theme]\nName=Hicolor\nDirectories=48x48/apps\n"), false);
        theme_dir(&sys, "default", Some("[Icon Theme]\nInherits=Adwaita\n"), false);
        theme_dir(&sys, "Gizli", Some("[Icon Theme]\nName=Gizli\nHidden=true\nDirectories=a\n"), false);
        theme_dir(&sys, "Bibata-Modern-Classic", Some("[Icon Theme]\nName=Bibata Modern Classic\nInherits=hicolor\n"), true);
        theme_dir(&sys, "Gizli-imlec", Some("[Icon Theme]\nName=Gizli imleç\nHidden=TRUE\n"), true);
        theme_dir(&sys, "Hypr-only", None, false);
        std::fs::write(sys.join("Hypr-only/manifest.hl"), "name = x\n").unwrap();
        theme_dir(&sys, ".gizli", Some("[Icon Theme]\nDirectories=a\n"), false);
        std::fs::write(sys.join("dosya.theme"), "değil").unwrap();
        // Kullanıcı dizinindeki aynı ad önce gelir.
        theme_dir(&user, "Adwaita", Some("[Icon Theme]\nName=Benim Adwaita\nDirectories=x\n"), false);

        let dirs = vec![user.clone(), tmp.path().join("yok"), sys.clone()];
        let icons = scan(ThemeKind::Icons, &dirs);
        let names: Vec<_> = icons.iter().map(|t| (t.id.as_str(), t.name.as_str())).collect();
        assert_eq!(names, vec![("Adwaita", "Benim Adwaita"), ("Papirus-Dark", "Papirus-Dark")]);

        let cursors = scan(ThemeKind::Cursors, &dirs);
        let ids: Vec<_> = cursors.iter().map(|t| t.id.as_str()).collect();
        // Kullanıcının Adwaita'sında cursors/ yok: imleç araması (libXcursor) sonraki dizine
        // geçer, sistemdeki Adwaita imleci listelenir.
        assert_eq!(ids, vec!["Adwaita", "Bibata-Modern-Classic", "Hypr-only"]);
        assert_eq!(cursors[1].name, "Bibata Modern Classic");
    }

    #[test]
    fn oversized_index_is_ignored() {
        let tmp = tempfile::tempdir().unwrap();
        let big = format!("[Icon Theme]\nName=Büyük\nDirectories=a\n#{}\n", "x".repeat(nuxut_image::icon::MAX_INDEX_BYTES as usize));
        theme_dir(tmp.path(), "Big", Some(&big), false);
        assert!(scan(ThemeKind::Icons, &[tmp.path().to_path_buf()]).is_empty());
    }

    /// Shell `look` `msg look status` sürüm 2 (`status_json_contract` testindeki biçim).
    const STATUS_JSON: &str = r#"{ "version": 2, "unified": true, "mode": "light", "wallpaper_colors": false,
      "icons":   { "requested": "Papirus", "current": "Papirus-Light", "fallback": false, "recommended": "Papirus", "explicit": false,
                   "installed": [{"id": "Papirus", "name": "Papirus", "hidden": false}] },
      "cursors": { "requested": "Bibata-Modern-Classic", "current": "Adwaita", "fallback": true, "recommended": "Bibata-Modern-Classic",
                   "explicit": false, "size": 32, "installed": [{"id": "Adwaita", "name": "Adwaita", "xcursor": true, "hyprcursor": false}] },
      "gtk_theme": "adw-gtk3",
      "qt_backend": { "selected": "auto", "active": "kde", "available": ["kde", "qt6ct"], "style": "Breeze" },
      "missing_packages": [ {"name": "plasma-integration", "source": "repo"}, {"name": "bibata-cursor-theme-bin", "source": "aur"},
                            {"name": "kötü; rm -rf", "source": "repo"}, {"name": "adw-gtk-theme", "source": "flatpak"}, "eski-dize" ],
      "conflicts": [ {"kind": "env", "name": "QT_STYLE_OVERRIDE", "value": "kvantum", "source": "session_env"} ],
      "restart_needed": ["gtk3_colors"],
      "writer": "active",
      "last_apply": null,
      "warnings": ["x"],
      "gelecekte": 1 }"#;

    fn pkg(name: &str, source: PackageSource) -> MissingPackage {
        MissingPackage { name: name.into(), source, target: None }
    }

    #[test]
    fn parses_shell_status_v2() {
        let s = parse_status(STATUS_JSON).expect("geçerli");
        assert!(s.unified);
        assert_eq!(s.mode.as_deref(), Some("light"));
        assert_eq!(s.icon_theme.as_deref(), Some("Papirus-Light"));
        assert_eq!(s.cursor_theme.as_deref(), Some("Adwaita"));
        assert_eq!(s.cursor_size, Some(32));
        assert_eq!(s.icon_recommended.as_deref(), Some("Papirus"));
        assert_eq!(s.cursor_recommended.as_deref(), Some("Bibata-Modern-Classic"));
        assert_eq!(
            s.missing_packages,
            vec![pkg("plasma-integration", PackageSource::Repo), pkg("bibata-cursor-theme-bin", PackageSource::Aur)],
            "geçersiz ad, bilinmeyen depo ve nesne olmayan girdi atlanır"
        );
        assert_eq!(s.conflicts, 1);
        assert!(s.restart_needed);
        assert_eq!(s.writer, Writer::Active);
        assert_eq!(s.last_apply, None);
        assert!(parse_status("ok").is_none());
        assert!(parse_status("[1,2]").is_none());
        assert_eq!(parse_status("{}"), Some(LookStatus::default()));
        let empty = parse_status(r#"{"restart_needed": [], "writer": "disabled"}"#).unwrap();
        assert!(!empty.restart_needed);
        assert_eq!(empty.writer, Writer::Disabled);
    }

    #[test]
    fn package_source_comes_from_shell_not_name() {
        // Ad soneki tahmini yok: "-git" depo paketi olabilir, "-bin" olmayan AUR paketi olabilir.
        let pk = [pkg("foo-git", PackageSource::Repo), pkg("papirus-folders", PackageSource::Aur)];
        assert_eq!(missing_fix(&pk, Some(AurHelper::Paru)), Some(Fix::Install(vec!["foo-git".into()])), "depo paketi önce, düğmeyle");
        assert_eq!(missing_fix(&pk[1..], Some(AurHelper::Paru)), Some(Fix::Copy("paru -S papirus-folders".into())));
        assert_eq!(missing_fix(&[], None), None);
    }

    /// Shell sürüm 3 (shell 5263e5f `status_json_contract`): katalog + `missing_packages[].target`.
    const STATUS_V3: &str = r#"{ "version": 3, "unified": true, "mode": "dark", "wallpaper_colors": false,
      "icons":   { "requested": "Papirus", "current": "Papirus-Dark", "fallback": false, "recommended": "Papirus", "explicit": false,
                   "installed": [{"id": "Papirus", "name": "Papirus", "hidden": false}],
                   "catalog": [ {"family": "papirus", "name": "Papirus", "package": "papirus-icon-theme", "source": "repo", "installed": ["Papirus", "Papirus-Dark"]},
                                {"family": "kora", "name": "Kora", "package": "kora-icon-theme", "source": "aur", "installed": []},
                                {"family": "kötü", "name": "Kötü", "package": "x; rm -rf /", "source": "aur", "installed": []},
                                {"family": "eksik", "name": "Eksik", "package": "eksik", "source": "repo"} ] },
      "cursors": { "requested": "Vimix-cursors", "current": "Adwaita", "fallback": true, "recommended": "Vimix-cursors",
                   "explicit": false, "size": 24, "installed": [],
                   "catalog": [ {"family": "vimix", "name": "Vimix", "package": "vimix-cursors", "source": "repo", "installed": ["bad/../id"]},
                                {"family": "oreo", "name": "Oreo", "package": "oreo-cursors-bin", "source": "aur", "installed": []} ] },
      "missing_packages": [ {"name": "vimix-cursors", "source": "repo", "target": "cursors"},
                            {"name": "plasma-integration", "source": "repo", "target": "qt"},
                            {"name": "yeni-hedef", "source": "aur", "target": "gelecek"} ],
      "conflicts": [], "restart_needed": [], "writer": "active", "last_apply": null, "warnings": [] }"#;

    #[test]
    fn parses_v3_catalog_and_targets() {
        let s = parse_status(STATUS_V3).expect("geçerli");
        let icons = s.catalog(ThemeKind::Icons).expect("v3 katalog");
        assert_eq!(icons.len(), 2, "geçersiz paket adı ve `installed` alanı olmayan aile atlanır");
        assert_eq!(icons[0].family, "papirus");
        assert_eq!(icons[0].installed, ["Papirus", "Papirus-Dark"]);
        assert_eq!(icons[1].source, PackageSource::Aur);
        let cursors = s.catalog(ThemeKind::Cursors).unwrap();
        assert_eq!(cursors[0].name, "Vimix");
        assert!(cursors[0].installed.is_empty(), "geçersiz tema adı kurulu sayılmaz");
        assert_eq!(s.cursor_recommended.as_deref(), Some("Vimix-cursors"));
        let targets: Vec<_> = s.missing_packages.iter().map(|p| p.target).collect();
        assert_eq!(targets, [Some(PackageTarget::Cursors), Some(PackageTarget::Qt), None], "bilinmeyen hedef = hedefsiz");
        // v2 yanıtında katalog yok: tahmin edilmez.
        let v2 = parse_status(STATUS_JSON).unwrap();
        assert_eq!(v2.catalog(ThemeKind::Cursors), None);
        assert_eq!(v2.catalog(ThemeKind::Icons), None);
        assert!(v2.missing_packages.iter().all(|p| p.target.is_none()));
    }

    #[test]
    fn themes_rev_parsed_and_change_detected() {
        assert_eq!(parse_status(STATUS_V3).unwrap().themes_rev, None, "sürüm 3: alan yok");
        let v4 = STATUS_V3.replacen(r#""version": 3,"#, r#""version": 4, "themes_rev": 7,"#, 1);
        assert_eq!(parse_status(&v4).unwrap().themes_rev, Some(7));
        let mut last = None;
        assert!(!themes_rev_changed(&mut last, None), "eski shell: yeniden okuma yok");
        assert!(!themes_rev_changed(&mut last, Some(3)), "ilk değer yalnızca kaydedilir");
        assert!(!themes_rev_changed(&mut last, Some(3)));
        assert!(themes_rev_changed(&mut last, Some(4)));
        assert!(themes_rev_changed(&mut last, Some(0)), "shell yeniden başladı: azalma da değişimdir");
        assert!(!themes_rev_changed(&mut last, None), "alan kayboldu: son değer korunur");
        assert_eq!(last, Some(0));
    }

    #[test]
    fn family_fix_by_source() {
        let s = parse_status(STATUS_V3).unwrap();
        let c = s.catalog(ThemeKind::Cursors).unwrap();
        assert_eq!(c[0].fix(None), Fix::Install(vec!["vimix-cursors".into()]));
        assert_eq!(c[1].fix(Some(AurHelper::Paru)), Fix::Copy("paru -S oreo-cursors-bin".into()));
    }

    /// PATH test içinde geçici dizinlerle verilir (süreç ortamı değişmez): paru önce, sonra yay;
    /// çalıştırılabilir olmayan dosya, dizin ve göreli PATH öğesi sayılmaz.
    #[test]
    fn aur_helper_detection_order() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let (a, b) = (tmp.path().join("a"), tmp.path().join("b"));
        std::fs::create_dir_all(&a).unwrap();
        std::fs::create_dir_all(&b).unwrap();
        let exe = |p: &Path, mode| {
            std::fs::write(p, "#!/bin/sh\n").unwrap();
            std::fs::set_permissions(p, std::fs::Permissions::from_mode(mode)).unwrap();
        };
        let path = |dirs: &[&Path]| std::env::join_paths(dirs).unwrap();
        assert_eq!(find_aur_helper(None), None);
        assert_eq!(find_aur_helper(Some(&path(&[&a, &b]))), None, "ikisi de yok");
        exe(&a.join("paru"), 0o644);
        std::fs::create_dir_all(b.join("yay")).unwrap();
        assert_eq!(find_aur_helper(Some(&path(&[&a, &b]))), None, "çalıştırılamayan dosya ve dizin sayılmaz");
        std::fs::remove_dir(b.join("yay")).unwrap();
        exe(&b.join("yay"), 0o755);
        assert_eq!(find_aur_helper(Some(&path(&[&a, &b]))), Some(AurHelper::Yay));
        exe(&a.join("paru"), 0o755);
        assert_eq!(find_aur_helper(Some(&path(&[&b, &a]))), Some(AurHelper::Paru), "PATH sırasından bağımsız: paru önce");
        // Göreli öğe yok sayılır.
        assert_eq!(find_aur_helper(Some(std::ffi::OsStr::new("a:b"))), None);
    }

    #[test]
    fn aur_command_follows_helper_and_notes_missing_one() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(aur_line("oreo-cursors-bin", Some(AurHelper::Paru)), "paru -S oreo-cursors-bin");
        assert_eq!(aur_line("oreo-cursors-bin", Some(AurHelper::Yay)), "yay -S oreo-cursors-bin");
        assert_eq!(aur_line("oreo-cursors-bin", None), "paru -S oreo-cursors-bin", "yardımcı yok: paru önerilir");
        assert_eq!(package_fix(PackageSource::Repo, &["vimix-cursors"], Some(AurHelper::Yay)), Fix::Install(vec!["vimix-cursors".into()]), "depo: kök komutu değil, düğme");
        assert!(needs_helper_note(true, None));
        assert!(!needs_helper_note(true, Some(AurHelper::Yay)) && !needs_helper_note(false, None));
        // Genel durum notu da aynı yardımcıyı ve notu kullanır.
        let s = parse_status(STATUS_V3).unwrap();
        let state = LookState::Ready(Box::new(s));
        let yay = status_line_with(&state, Some(AurHelper::Yay)).unwrap();
        assert_eq!(yay.fix, Some(Fix::Install(vec!["plasma-integration".into()])));
        assert!(!yay.text.contains(&aur_helper_note()));
        let none = status_line_with(&state, None).unwrap();
        assert_eq!(none.fix, Some(Fix::Install(vec!["plasma-integration".into()])));
        assert!(none.text.ends_with(&aur_helper_note()), "{}", none.text);
        // Yalnızca depo paketi: not yok.
        let repo_only = LookStatus { missing_packages: vec![pkg("plasma-integration", PackageSource::Repo)], ..Default::default() };
        assert!(!status_line_with(&LookState::Ready(Box::new(repo_only)), None).unwrap().text.contains(&aur_helper_note()));
    }

    #[test]
    fn general_note_skips_icon_and_cursor_packages() {
        // İmleç paketi ızgarada (katalogdan); genel not yalnızca diğer hedefleri ve hedefsizleri sayar.
        let s = parse_status(STATUS_V3).unwrap();
        let names: Vec<&str> = general_packages(&s).iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["plasma-integration", "yeni-hedef"]);
        let line = status_line_with(&LookState::Ready(Box::new(s.clone())), Some(AurHelper::Paru)).unwrap();
        assert_eq!(line.tone, Tone::Warning);
        assert!(!line.text.contains("vimix-cursors"));
        assert_eq!(line.fix, Some(Fix::Install(vec!["plasma-integration".into()])));
        // Yalnızca tema paketleri eksikse genel not "Uygulandı" der.
        let only_theme = LookStatus { missing_packages: s.missing_packages[..1].to_vec(), ..s };
        let line = status_line_with(&LookState::Ready(Box::new(only_theme)), Some(AurHelper::Paru)).unwrap();
        assert_eq!((line.tone, line.fix), (Tone::Ok, None));
    }

    #[test]
    fn status_line_prefers_missing_packages_with_install_button() {
        crate::i18n::use_turkish_for_tests();
        let s = parse_status(STATUS_JSON).unwrap();
        let line = status_line_with(&LookState::Ready(Box::new(s.clone())), Some(AurHelper::Paru)).unwrap();
        assert_eq!(line.tone, Tone::Warning);
        assert_eq!(line.fix, Some(Fix::Install(vec!["plasma-integration".into()])));
        let ok = LookStatus { missing_packages: vec![], ..s };
        let line = status_line_with(&LookState::Ready(Box::new(ok.clone())), Some(AurHelper::Paru)).unwrap();
        assert_eq!(line.tone, Tone::Ok);
        assert!(line.text.starts_with("Uygulandı."));
        assert!(line.text.contains("yeniden açılınca"));
        let failed = LookStatus { last_apply: Some(LastApply { ok: false, errors: 2, ..Default::default() }), ..ok };
        assert_eq!(status_line_with(&LookState::Ready(Box::new(failed)), Some(AurHelper::Paru)).unwrap().tone, Tone::Warning);
        assert_eq!(status_line(&LookState::NotRunning).unwrap().tone, Tone::Info);
    }

    /// Sürüm 5 `last_apply.external`: genel "aktarılamadı" yerine dosya başına neden ve etki.
    #[test]
    fn external_files_are_named_with_reason() {
        crate::i18n::use_turkish_for_tests();
        let v5 = r#"{ "version": 5, "unified": true, "mode": "dark", "missing_packages": [], "conflicts": [], "restart_needed": [],
          "last_apply": { "ok": false, "written": [], "unchanged": [], "skipped": [], "errors": [], "live": [], "paused": [],
            "external": [
              {"setting": "look.mode", "path": "~/.config/gtk-3.0/gtk.css", "reason": "read_only"},
              {"setting": "look.fonts", "path": "~/.config/gtk-3.0/gtk.css", "reason": "read_only"},
              {"setting": "look.icons", "path": "~/.config/kdeglobals", "reason": "not_owned"},
              {"setting": "look.mode", "path": "/nix/store/x/gtk-4.0/gtk.css", "reason": "store"},
              {"setting": "look.mode", "path": "göreli/yol", "reason": "read_only"},
              {"setting": "look.cursor", "path": "~/.icons/default/index.theme", "reason": "gelecekte_yeni"} ] } }"#;
        let s = parse_status(v5).unwrap();
        let a = s.last_apply.clone().unwrap();
        assert_eq!(a.external.len(), 5, "göreli yol atlanır");
        assert_eq!(a.external[4].reason, ExternalReason::Managed, "bilinmeyen neden yönetiliyor sayılır");
        let line = status_line_with(&LookState::Ready(Box::new(s)), Some(AurHelper::Paru)).unwrap();
        assert_eq!(line.tone, Tone::Warning);
        assert!(!line.text.contains("günlüğünde"), "genel metin değil: {}", line.text);
        assert!(line.text.starts_with("~/.config/gtk-3.0/gtk.css başka bir araç tarafından yönetiliyor."), "{}", line.text);
        assert!(line.text.contains("renklerini almıyor"));
        assert!(line.text.contains("~/.config/kdeglobals başka bir kullanıcıya ait"));
        assert!(line.text.contains("ikonlarını almıyor"));
        assert!(!line.text.contains("/nix/store"), "ilk iki benzersiz dosya adıyla anılır");
        assert!(line.text.ends_with("2 dosya daha aynı durumda."), "{}", line.text);
        assert_eq!(line.fix, None, "sahiplik komutu önerilmez (kullanıcı kararı 2026-10-17)");

        // Gerçek hata varsa genel metin; yalnızca durdurulan ayar varsa kendi notu.
        let failed = LastApply { errors: 1, ..a.clone() };
        let st = |la| LookState::Ready(Box::new(LookStatus { unified: true, last_apply: Some(la), ..Default::default() }));
        assert!(status_line_with(&st(failed), None).unwrap().text.contains("günlüğünde"));
        let paused = LastApply { paused: 1, ..Default::default() };
        let line = status_line_with(&st(paused), None).unwrap();
        assert_eq!((line.tone, line.text.contains("durduruldu")), (Tone::Warning, true));
        // Eski shell (sürüm 4): `external` yok, `ok: false` → genel metin.
        assert!(status_line_with(&st(LastApply::default()), None).unwrap().text.contains("günlüğünde"));
    }

    /// Sürüm 6 `last_apply.taken_over`: devralınan dosyalar "uygulandı" metninde görünür
    /// (sessiz kalmaz), geri alma yolu söylenir; uyarı değil.
    #[test]
    fn taken_over_files_are_shown() {
        crate::i18n::use_turkish_for_tests();
        let v6 = r#"{ "version": 6, "unified": true, "mode": "dark", "missing_packages": [], "conflicts": [], "restart_needed": [],
          "last_apply": { "ok": true, "written": [], "unchanged": [], "skipped": [], "errors": [], "live": [], "paused": [], "external": [],
            "taken_over": [
              {"setting": "look.mode", "path": "~/.config/gtk-4.0/gtk.css"},
              {"setting": "look.mode", "path": "~/.config/gtk-4.0/gtk-dark.css"},
              {"setting": "look.mode", "path": "~/.config/gtk-4.0/gtk.css"},
              {"setting": "look.mode", "path": "göreli"} ] } }"#;
        let s = parse_status(v6).unwrap();
        assert_eq!(s.last_apply.as_ref().unwrap().taken_over.len(), 3, "göreli yol atlanır");
        let line = status_line_with(&LookState::Ready(Box::new(s)), None).unwrap();
        assert_eq!(line.tone, Tone::Ok);
        assert!(line.text.contains("~/.config/gtk-4.0/gtk.css ve 1 dosya daha önceden başka araçlardaydı"), "{}", line.text);
        assert_eq!(taken_over_text(&["~/a".into()]), "~/a önceden başka bir araçtaydı, Nuxut devraldı; birleşik görünümü kapatmak eskisini geri koyar.");
        assert_eq!(taken_over_text(&[]), "");
    }

    #[test]
    fn disabled_writer_is_a_calm_note_not_an_error() {
        crate::i18n::use_turkish_for_tests();
        let s = LookStatus { unified: true, writer: Writer::Disabled, last_apply: Some(LastApply::default()), ..Default::default() };
        let line = status_line_with(&LookState::Ready(Box::new(s.clone())), Some(AurHelper::Paru)).unwrap();
        assert_eq!(line.tone, Tone::Info);
        assert!(line.text.contains("kaydedildi") && !line.text.contains("Uygulandı"));
        assert_eq!(line.fix, None);
        let with_pkg = LookStatus { missing_packages: vec![pkg("papirus-icon-theme", PackageSource::Repo)], ..s };
        let line = status_line_with(&LookState::Ready(Box::new(with_pkg)), Some(AurHelper::Paru)).unwrap();
        assert_eq!(line.tone, Tone::Info, "eksik paket de sakin not içinde");
        assert!(line.text.contains("papirus-icon-theme"));
        assert_eq!(line.fix, Some(Fix::Install(vec!["papirus-icon-theme".into()])));
    }

    #[test]
    fn query_uses_look_status_and_classifies_errors() {
        let link = FakeLink::with([
            Ok(STATUS_JSON.to_owned()),
            Ok("error: bilinmeyen komut: look".to_owned()),
            Err(LinkError::NotRunning),
            Err(LinkError::TestInstance),
            Ok("ok".to_owned()),
        ]);
        assert!(matches!(query_status(&link), LookState::Ready(_)));
        assert_eq!(query_status(&link), LookState::Unsupported);
        assert_eq!(query_status(&link), LookState::NotRunning);
        assert_eq!(query_status(&link), LookState::TestInstance);
        assert_eq!(query_status(&link), LookState::Unsupported, "JSON değil");
        assert!(link.sent().iter().all(|a| a == &["look", "status"]));
    }
}
