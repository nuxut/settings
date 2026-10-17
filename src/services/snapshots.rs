//! Ayar başına "önceki hâline dön" (shell ADR-0001 §7, ADR-0004 §8-4).
//!
//! **Oturum içi geri al ile ilişkisi** (`crate::history`):
//! - `History` bu oturumda Ayarlar'ın kendi yazdığı TOML anahtarlarını tutar (bellekte, ≤ 100,
//!   Ctrl+Z / "Son değişiklikler"); uygulama kapanınca biter. Normal framework/shell.toml
//!   düzenlemeleri snapshot ALMAZ (ADR-0001 §9.1) — o işi `History` yapar.
//! - Snapshot'lar kalıcıdır ve shell'indir: shell dış hedeflere (GTK/Qt/ikon/imleç dosyaları,
//!   dconf, Hyprland) dokunurken, dokunduğu parçaların önceki hâlini `$XDG_STATE_HOME/nuxut/
//!   snapshots`'a yazar. Ayarlar yalnızca listeyi okur (`msg snapshot list`) ve geri yüklemeyi
//!   ister (`msg snapshot restore <id>`); dosyalara dokunmaz (ADR-0004 §12-1).
//! - Bu yüzden ikisi çakışmaz: satırın "varsayılana dön"ü ve Ctrl+Z config anahtarını değiştirir,
//!   "önceki hâline dön" oturumlar arası ve dış hedeflerle birlikte döner. Geri yükleme de bir
//!   snapshot'tır (ADR-0001 §7): düğmeye yeniden basmak geri yüklemeyi geri alır.
//!
//! SÖZLEŞME (karar, 2026-10-11): geri yükleme TEK işlemdir — shell dış dosyalarla
//! birlikte `framework.toml`'daki seçim anahtarını da (ör. `[desktop] icon_theme`, ADR-0001
//! `TomlKey` parçası) geri koyar. Ayarlar bu sırada framework.toml'a YAZMAZ: istekten önce
//! bekleyen yazımını gönderir (`App::apply_actions`, yarış olmasın), sonra yalnızca ister;
//! satırın yeni değeri dosya izleyicisinin yeniden yüklemesinden gelir (dış değişiklik,
//! `History`'ye girmez). İstek sürerken düğme devre dışıdır ve şeritte "geri yükleniyor…"
//! görünür; `partial`/`conflict` sonuçlarını metin açıkça söyler.
//!
//! Shell sözleşmesi (shell ADR-0001 §9.3/§9.5; framework `nuxut_apply` tipleri):
//! - `snapshot list [--setting <id>] [--limit <1..=200>]` → `[{id, at, setting, label, source,
//!   state, restored_by?, restores?, pinned, fragments, conflicts}]` (yeniden eskiye; `at` ISO
//!   8601 UTC; `state` `pending|applied|rolled_back|interrupted|restored`; `restored_by` yalnızca
//!   `restored` kayıtta: onu geri alan işlem).
//! - `snapshot show <id>` → aynı alanlar + `fragments: [{kind, target, key?, before, after}]`.
//! - `snapshot restore <id> [--force] [--setting <id>]` → `RestoreOutcome`:
//!   `{"result":"restored","id":<yeni kimlik>|null}` · `{"result":"partial","id",conflicts:[…]}` ·
//!   `{"result":"conflict","conflicts":[…]}` · `{"result":"not_found"}` · `{"result":"busy"}` ·
//!   `{"result":"skipped","reason":"test_instance|no_session|disabled"}` ·
//!   `{"result":"error","kind":<ad>}`. `--setting` verilip snapshot o ayarın değilse hiçbir şey
//!   yazılmaz (`not_found`). Ayarlar her zaman `--setting` gönderir.
//! - Hata satırı `error: …` (sözdizimi, `skipped(…)`, `busy`, `not_found` …); `nuxut-apply` öncesi
//!   shell `snapshot`'ı tanımaz (`error: bilinmeyen komut`): liste boş, düğme görünmez.
//! - Zaman aşımı: shell `snapshot` gövdelerini ayrı thread'de çalıştırır (depo kilidi ≤ 5 sn) ve
//!   yanıt penceresi 15 sn'dir (shell `ipc::REPLY_TIMEOUT`); "Shell'e `snapshot` gönderen başka
//!   istemci de en az bu kadar beklemeli" — Ayarlar her `snapshot` isteğinde [`SNAPSHOT_TIMEOUT`]
//!   bekler (arka plan thread'i; UI beklemez). Pencere dolarsa yanıt düşer ama geri yükleme yine
//!   yapılmış olabilir ya da hâlâ sürebilir: sonuç ~2 sn ve ~5 sn sonra `snapshot list` ile
//!   doğrulanır ([`verify_restore`]); görünürse "geri yüklendi (ayrıntı doğrulanamadı)", yoksa
//!   "doğrulanamadı — durumu yenile". Restore isteği asla yeniden gönderilmez.
//!
//! Ayar başına geri al = o `SettingId`'nin en son `applied` kaydı.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use nuxut::i18n::t;

use super::shell_ipc::{IpcLink, LinkError, ShellLink};

/// Her `snapshot` isteğinin bekleme süresi: shell yanıt penceresi (15 sn) + pay. Pencereden
/// kısa beklemek, shell'in yaptığı geri yüklemeyi "başarısız" göstermek olur.
pub const SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(16);

/// Ayarlar satırı olan snapshot `SettingId`'leri (ADR-0001 §9.3 donuk v1 kapalı listesinden).
/// `migration.shell` göç bildirimidir (satır değil); `hypr.<grup>` ve `displays` kimlik değildir.
pub const SNAPSHOT_SETTINGS: [&str; 16] = [
    "look.mode",
    "look.icons",
    "look.cursor",
    "look.fonts",
    "look.qt_backend",
    "look.unified",
    "hypr.source",
    // Varsayılan uygulamalar (sözleşme defaults-v1): `defaults.<rol>`, kayıttaki satır kimliğiyle aynı.
    "defaults.browser",
    "defaults.email",
    "defaults.file_manager",
    "defaults.terminal",
    "defaults.text_editor",
    "defaults.image",
    "defaults.video",
    "defaults.music",
    "defaults.pdf",
];

/// Kayıttaki ayar kimliği → shell'in snapshot ayar kimliği (`SettingId`, ADR-0004 §6).
pub fn setting_for(desc_id: &str) -> Option<&'static str> {
    Some(match desc_id {
        "colors.mode" => "look.mode",
        "desktop.unified" => "look.unified",
        "desktop.icon_theme" => "look.icons",
        "desktop.cursor_theme" | "desktop.cursor_size" => "look.cursor",
        "desktop.qt_backend" => "look.qt_backend",
        // Kullanıcının Hyprland ana dosyasındaki `source` satırı.
        "compositor.ensure_source" => "hypr.source",
        "typography.ui.family" | "typography.document.family" | "typography.mono.family" => "look.fonts",
        // Varsayılan uygulama satırı: kimlik shell `SettingId`'siyle aynı.
        id => return super::default_apps::Role::from_setting_id(id).map(super::default_apps::Role::setting_id),
    })
}

/// `SettingId` biçimi (`[a-z0-9_.]`, 1..=64, nokta ile başlamaz/bitmez, ardışık nokta yok).
pub fn valid_setting_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'.')
        && !s.starts_with('.')
        && !s.ends_with('.')
        && !s.contains("..")
}

/// İşlem durumu (`TxState`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapState {
    Pending,
    Applied,
    RolledBack,
    Interrupted,
    /// Bu işlem sonradan bir geri yüklemeyle geri alındı (geri yüklemenin kendisi ayrı,
    /// `applied` bir işlemdir).
    Restored,
    Unknown,
}

impl SnapState {
    fn parse(s: &str) -> Self {
        match s.split(['(', ':']).next().unwrap_or("").trim() {
            "pending" => Self::Pending,
            "applied" => Self::Applied,
            "rolled_back" => Self::RolledBack,
            "interrupted" => Self::Interrupted,
            "restored" => Self::Restored,
            _ => Self::Unknown,
        }
    }
}

/// `snapshot list` girdisi. Değerler (dosya içerikleri) IPC'den geçmez.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotEntry {
    pub id: String,
    /// Shell'in ayar kimliği (`look.icons`).
    pub setting: String,
    /// Kim başlattı (`settings` | `shell` | `welcome` | `migration` | `cli`).
    pub source: String,
    pub state: SnapState,
    /// Parça sayısı.
    pub fragments: usize,
    /// Sonradan değişmiş (geri yüklemede çakışacak) parça sayısı.
    pub conflicts: usize,
    /// `restored` kayıtta: onu geri alan işlemin kimliği.
    pub restored_by: Option<String>,
    /// Kayıt anı (`at`; yoksa kimliğin UTC damgası).
    pub at: Option<SystemTime>,
}

/// `20261010T120000Z-3fa1`: UTC zaman + 4 onaltılık.
pub fn valid_snapshot_id(id: &str) -> bool {
    let b = id.as_bytes();
    id.len() == 21
        && b[..8].iter().all(u8::is_ascii_digit)
        && b[8] == b'T'
        && b[9..15].iter().all(u8::is_ascii_digit)
        && b[15] == b'Z'
        && b[16] == b'-'
        && b[17..].iter().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

/// Gregoryen tarihten Unix günü (Howard Hinnant, `days_from_civil`).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_time(y: i64, mo: i64, d: i64, h: i64, mi: i64, s: i64) -> Option<SystemTime> {
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) || !(0..=23).contains(&h) || !(0..=59).contains(&mi) || !(0..=60).contains(&s) {
        return None;
    }
    let secs = days_from_civil(y, mo, d) * 86_400 + h * 3600 + mi * 60 + s;
    Some(UNIX_EPOCH + Duration::from_secs(u64::try_from(secs).ok()?))
}

/// Kimlikteki UTC damgası.
pub fn id_time(id: &str) -> Option<SystemTime> {
    if !valid_snapshot_id(id) {
        return None;
    }
    let n = |r: std::ops::Range<usize>| id[r].parse::<i64>().ok();
    civil_time(n(0..4)?, n(4..6)?, n(6..8)?, n(9..11)?, n(11..13)?, n(13..15)?)
}

/// `at` alanı: ISO 8601 UTC (`2026-10-10T12:00:00Z`).
fn parse_time(v: &serde_json::Value) -> Option<SystemTime> {
    let s = v.as_str()?.as_bytes();
    if s.len() != 20 || s[4] != b'-' || s[7] != b'-' || s[10] != b'T' || s[13] != b':' || s[16] != b':' || s[19] != b'Z' {
        return None;
    }
    let n = |r: std::ops::Range<usize>| -> Option<i64> {
        let part = &s[r];
        part.iter().all(u8::is_ascii_digit).then(|| part.iter().fold(0i64, |a, d| a * 10 + i64::from(d - b'0')))
    };
    civil_time(n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?)
}

/// "az önce", "5 dk önce", "3 sa önce", "dün", "4 gün önce", "3 hafta önce".
pub fn ago(at: SystemTime, now: SystemTime) -> String {
    let s = now.duration_since(at).map_or(0, |d| d.as_secs());
    match s {
        0..60 => t!("settings-snapshots-ago-now"),
        60..3600 => t!("settings-snapshots-ago-minutes", count = s / 60),
        3600..86_400 => t!("settings-snapshots-ago-hours", count = s / 3600),
        86_400..172_800 => t!("settings-snapshots-ago-yesterday"),
        _ if s < 14 * 86_400 => t!("settings-snapshots-ago-days", count = s / 86_400),
        _ => t!("settings-snapshots-ago-weeks", count = s / (7 * 86_400)),
    }
}

fn bounded(v: Option<&serde_json::Value>, max: usize) -> Option<String> {
    v?.as_str().filter(|s| !s.is_empty() && s.len() <= max).map(str::to_owned)
}

/// En çok okunan liste girdisi.
pub const MAX_ENTRIES: usize = 200;

/// `snapshot list` yanıtı (JSON dizi). Geçersiz girdiler atlanır; sözleşme dışı → `None`.
pub fn parse_list(text: &str) -> Option<Vec<SnapshotEntry>> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let items = v.as_array()?;
    Some(
        items
            .iter()
            .take(MAX_ENTRIES)
            .filter_map(|e| {
                let id = bounded(e.get("id"), 64).filter(|i| valid_snapshot_id(i))?;
                let count = |key: &str| {
                    let n = e.get(key).and_then(|f| f.as_u64().or_else(|| f.as_array().map(|a| a.len() as u64)));
                    n.unwrap_or(0).min(10_000) as usize
                };
                Some(SnapshotEntry {
                    setting: bounded(e.get("setting"), 64).filter(|s| valid_setting_id(s))?,
                    source: bounded(e.get("source"), 32).unwrap_or_default(),
                    state: e.get("state").and_then(|s| s.as_str()).map_or(SnapState::Unknown, SnapState::parse),
                    fragments: count("fragments"),
                    conflicts: count("conflicts"),
                    restored_by: bounded(e.get("restored_by"), 64).filter(|i| valid_snapshot_id(i)),
                    at: e.get("at").and_then(parse_time).or_else(|| id_time(&id)),
                    id,
                })
            })
            .collect(),
    )
}

/// Liste isteği; `setting` verilirse yalnızca o ayarınki (geçersiz kimlik → `None`).
pub fn list_args(setting: Option<&str>) -> Option<Vec<String>> {
    let mut args = vec!["snapshot".to_owned(), "list".to_owned()];
    if let Some(s) = setting {
        if !valid_setting_id(s) {
            return None;
        }
        args.extend(["--setting".to_owned(), s.to_owned()]);
    }
    Some(args)
}

/// Ayrıntı isteği (`snapshot show <id>`); geçersiz kimlikte `None`.
pub fn show_args(id: &str) -> Option<Vec<String>> {
    valid_snapshot_id(id).then(|| ["snapshot", "show", id].map(str::to_owned).to_vec())
}

/// Geri yükleme isteği (ADR-0001 §9.3 sırası: `<id> [--force] [--setting <ayar>]`); geçersiz
/// kimlikte `None` (shell'e hiç gitmez). `setting`: snapshot bu ayarın değilse shell hiçbir şey
/// yazmaz (`not_found`) — eski bir liste başka ayarın kaydını geri yükletemez. `force`: sonradan
/// değişmiş parçaların da üzerine yaz (Ayarlar şimdilik kullanmaz).
pub fn restore_args(id: &str, setting: &str, force: bool) -> Option<Vec<String>> {
    if !valid_snapshot_id(id) || !valid_setting_id(setting) {
        return None;
    }
    let mut args = ["snapshot", "restore", id].map(str::to_owned).to_vec();
    if force {
        args.push("--force".to_owned());
    }
    args.extend(["--setting".to_owned(), setting.to_owned()]);
    Some(args)
}

/// Shell'in atlama nedeni (`{"result":"skipped","reason":…}`; framework `Skip`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
    /// Test/ikincil shell örneği: kullanıcının dosyalarına yazmaz.
    TestInstance,
    /// Gerekli oturum yok.
    NoSession,
    /// Shell yazımı kapattı (kilidi alamamış ikinci örnek).
    Disabled,
    Unknown,
}

impl SkipReason {
    fn parse(s: Option<&str>) -> Self {
        match s {
            Some("test_instance") => Self::TestInstance,
            Some("no_session") => Self::NoSession,
            Some("disabled") => Self::Disabled,
            _ => Self::Unknown,
        }
    }
}

/// `snapshot restore` sonucu (framework `RestoreOutcome`; ADR-0001 §9.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreOutcome {
    /// `wrote`: geri yükleme bir şey yazdı (`id` dolu); `false`: her parça zaten öncesindeydi
    /// (`"id": null`).
    Restored { wrote: bool },
    /// Bazı parçalar sonradan başka biri tarafından değiştirilmiş; onlar atlandı.
    Partial { conflicts: usize },
    Conflict { conflicts: usize },
    NotFound,
    Busy,
    /// Shell bilerek uygulamadı.
    Skipped(SkipReason),
    /// Shell tarafında hata (framework `ApplyError::kind_name`; `[a-z_]`).
    Error(String),
    /// Yanıt zamanında gelmedi; `snapshot list` geri yüklemenin olduğunu gösteriyor ama tamamı
    /// mı (`restored`) kısmı mı (`partial`) bilinmiyor.
    RestoredUnverified,
    /// Yanıt zamanında gelmedi ve `snapshot list` iki denemede de geri yüklemeyi göstermedi.
    Unconfirmed,
    /// İstek gitmedi ya da yanıt sözleşme dışı (kullanıcıya gösterilir; shell metni geçmez).
    Failed(String),
}

/// `kind`/`reason` gibi kısa ad alanı: `[a-z_]`, 1..=32; değilse `None`.
fn short_name(v: Option<&serde_json::Value>) -> Option<String> {
    bounded(v, 32).filter(|s| s.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'))
}

pub fn parse_restore(text: &str) -> RestoreOutcome {
    let bad = || RestoreOutcome::Failed(t!("settings-snapshots-error-unparsable"));
    let Ok(v) = serde_json::from_str::<serde_json::Value>(text.trim()) else {
        return bad();
    };
    let conflicts = v.get("conflicts").and_then(|c| c.as_array()).map_or(0, Vec::len);
    let new_id = || bounded(v.get("id"), 64).filter(|i| valid_snapshot_id(i));
    match v.get("result").and_then(|r| r.as_str()) {
        Some("restored") => RestoreOutcome::Restored { wrote: new_id().is_some() },
        Some("partial") => RestoreOutcome::Partial { conflicts },
        Some("conflict") => RestoreOutcome::Conflict { conflicts },
        Some("not_found") => RestoreOutcome::NotFound,
        Some("busy") => RestoreOutcome::Busy,
        Some("skipped") => RestoreOutcome::Skipped(SkipReason::parse(v.get("reason").and_then(|r| r.as_str()))),
        Some("error") => RestoreOutcome::Error(short_name(v.get("kind")).unwrap_or_else(|| t!("settings-snapshots-error-unknown-kind"))),
        // Bilinmeyen sonuç (daha yeni shell): yanıt metni gösterilmez.
        _ => bad(),
    }
}

/// Kullanıcıya sonuç metni (`label` = ayarın başlığı).
pub fn outcome_text(label: &str, outcome: &RestoreOutcome) -> String {
    match outcome {
        RestoreOutcome::Restored { wrote: true } => t!("settings-snapshots-outcome-restored", label = label),
        RestoreOutcome::Restored { wrote: false } => t!("settings-snapshots-outcome-already", label = label),
        RestoreOutcome::Partial { conflicts } => t!("settings-snapshots-outcome-partial", label = label, count = conflicts),
        RestoreOutcome::Conflict { .. } => t!("settings-snapshots-outcome-conflict", label = label),
        RestoreOutcome::NotFound => t!("settings-snapshots-outcome-not-found"),
        RestoreOutcome::Busy => t!("settings-snapshots-outcome-busy"),
        RestoreOutcome::Skipped(SkipReason::TestInstance) => t!("settings-snapshots-outcome-skipped-test", label = label),
        RestoreOutcome::Skipped(SkipReason::NoSession) => t!("settings-snapshots-outcome-skipped-no-session", label = label),
        RestoreOutcome::Skipped(SkipReason::Disabled | SkipReason::Unknown) => {
            t!("settings-snapshots-outcome-skipped-disabled", label = label)
        }
        RestoreOutcome::Error(kind) => t!("settings-snapshots-outcome-error", label = label, kind = kind),
        RestoreOutcome::RestoredUnverified => t!("settings-snapshots-outcome-restored-unverified", label = label),
        RestoreOutcome::Unconfirmed => t!("settings-snapshots-outcome-unconfirmed", label = label),
        RestoreOutcome::Failed(e) => t!("settings-snapshots-outcome-failed", label = label, error = e),
    }
}

/// Ayar başına en son geri yüklenebilir kayıt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotIndex {
    latest: HashMap<String, SnapshotEntry>,
}

impl SnapshotIndex {
    /// "Önceki hâl" = ayarın en son `applied` işlemi. Geri yükleme de yeni bir `applied`
    /// işlemdir (düğmeye yeniden basmak onu geri alır); `restored` = zaten geri alınmış, yarım
    /// ya da geri sarılmış işlemler sunulmaz. Kimlikler zaman sıralı: en büyük kimlik en yeni.
    pub fn from_entries(entries: impl IntoIterator<Item = SnapshotEntry>) -> Self {
        let mut latest: HashMap<String, SnapshotEntry> = HashMap::new();
        for e in entries {
            if e.state != SnapState::Applied {
                continue;
            }
            match latest.get(&e.setting) {
                Some(cur) if cur.id >= e.id => {}
                _ => {
                    latest.insert(e.setting.clone(), e);
                }
            }
        }
        Self { latest }
    }

    pub fn latest(&self, setting: &str) -> Option<&SnapshotEntry> {
        self.latest.get(setting)
    }

    pub fn is_empty(&self) -> bool {
        self.latest.is_empty()
    }
}

/// Saf sorgu: her görünüm ayarının listesini (`--setting`) ister ve dizine çevirir — ayar
/// başına istek, böylece başka ayarların çok sayıdaki kaydı birini listeden itmez. Shell yok /
/// desteklemiyor → boş dizin (düğme görünmez; hata değil); ilk bağlantı hatasında durur.
pub fn query_index(link: &dyn ShellLink) -> SnapshotIndex {
    let mut entries = Vec::new();
    for setting in SNAPSHOT_SETTINGS {
        let Some(args) = list_args(Some(setting)) else { continue };
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        match link.request(&args) {
            // Yanıt başka ayarın girdisini taşıyorsa (sözleşme dışı) yok sayılır.
            Ok(reply) => entries.extend(parse_list(&reply).unwrap_or_default().into_iter().filter(|e| e.setting == setting)),
            Err(e) => {
                if !matches!(e, LinkError::NotRunning | LinkError::TestInstance) {
                    tracing::debug!(%e, "snapshot listesi alınamadı");
                }
                break;
            }
        }
    }
    SnapshotIndex::from_entries(entries)
}

/// Zaman aşımından sonra doğrulama listesinin istenme gecikmeleri (kabuk: restore thread'i
/// pencere dolduktan sonra da bitebilir — ~2 sn sonra bir kez, görünmezse ~5 sn sonra bir kez daha).
pub const VERIFY_DELAYS: [Duration; 2] = [Duration::from_secs(2), Duration::from_secs(5)];

/// Saf geri yükleme (`link` [`SNAPSHOT_TIMEOUT`] bekleyen bağlantı; arka plan thread'inde).
/// Yanıt zamanında gelmezse sonuç [`verify_restore`] ile listeden doğrulanır.
pub fn restore(link: &dyn ShellLink, id: &str, setting: &str) -> RestoreOutcome {
    restore_with(link, id, setting, &std::thread::sleep)
}

/// [`restore`]; `sleep` testte sahte (gecikmeler kaydedilir, beklenmez).
pub fn restore_with(link: &dyn ShellLink, id: &str, setting: &str, sleep: &dyn Fn(Duration)) -> RestoreOutcome {
    let Some(args) = restore_args(id, setting, false) else { return RestoreOutcome::NotFound };
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    // `restore` isteği bir kez gönderilir; hiçbir yolda yeniden gönderilmez (çift geri yükleme
    // ilkini geri alır).
    match link.request(&args) {
        Ok(reply) => parse_restore(&reply),
        // Zaman aşımı ya da boş kapanan bağlantı (pencere doldu / bağlantı koptu): ikisi de
        // doğrulama yolu.
        Err(LinkError::Timeout) => verify_restore(link, id, setting, sleep),
        Err(LinkError::NotRunning) => RestoreOutcome::Failed(t!("settings-snapshots-error-not-running")),
        Err(LinkError::TestInstance) => RestoreOutcome::Failed(t!("settings-snapshots-error-test-instance")),
        // `error: …` satırı: kimlik/ayar reddi ya da `nuxut-apply` öncesi shell (`snapshot`'ı
        // tanımaz). Shell metni kullanıcıya geçmez.
        Err(LinkError::Shell(msg)) => {
            tracing::debug!(%msg, "snapshot restore reddedildi");
            RestoreOutcome::Failed(t!("settings-snapshots-error-rejected"))
        }
        Err(e) => RestoreOutcome::Failed(e.to_string()),
    }
}

/// Yanıtı düşmüş geri yüklemenin sonucu: shell pencere dolunca yanıtı düşürür ama iş sürer
/// (ADR-0001 §9.5). [`VERIFY_DELAYS`] sırasıyla bekleyip `snapshot list --setting` ister; kayıt
/// `restored` görünürse (framework onu geri yükleme işlemi yazılınca işaretler)
/// [`RestoreOutcome::RestoredUnverified`] — `partial` ile ayrımı listeden bilinemez, "tamamı
/// döndü" denmez. İki denemede de görünmezse [`RestoreOutcome::Unconfirmed`]. Yalnızca liste
/// ister; geri yükleme isteğini yeniden göndermez.
pub fn verify_restore(link: &dyn ShellLink, id: &str, setting: &str, sleep: &dyn Fn(Duration)) -> RestoreOutcome {
    let Some(args) = list_args(Some(setting)) else { return RestoreOutcome::Unconfirmed };
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    for delay in VERIFY_DELAYS {
        sleep(delay);
        match link.request(&args) {
            Ok(reply) => {
                let entries = parse_list(&reply).unwrap_or_default();
                if entries.iter().any(|e| e.id == id && e.state == SnapState::Restored) {
                    return RestoreOutcome::RestoredUnverified;
                }
            }
            Err(e) => tracing::debug!(%e, "geri yükleme doğrulanamadı"),
        }
    }
    RestoreOutcome::Unconfirmed
}

// ---- Arka plan ------------------------------------------------------------------------------

/// UI → servis.
#[derive(Debug, Clone)]
pub enum Command {
    Refresh,
    /// `setting`: kaydın shell ayar kimliği (istekte `--setting` olarak gider).
    Restore { id: String, setting: String, label: String },
}

static INDEX: Mutex<Option<SnapshotIndex>> = Mutex::new(None);
static STARTED: Mutex<bool> = Mutex::new(false);

/// Ayarın geri yüklenebilir son kaydı (`desc_id` kayıttaki kimlik). İlk çağrı listeyi
/// arka planda ister; gelince [`Event::Snapshots`](super::Event::Snapshots).
pub fn latest_for(desc_id: &str) -> Option<SnapshotEntry> {
    let setting = setting_for(desc_id)?;
    let guard = INDEX.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        drop(guard);
        let mut started = STARTED.lock().unwrap_or_else(|e| e.into_inner());
        if !*started {
            *started = true;
            send(Command::Refresh);
        }
        return None;
    }
    guard.as_ref()?.latest(setting).cloned()
}

/// İsteği kısa ömürlü bir thread'de yürütür (istekler seyrek: düğme tıklaması, sayfa açılışı).
pub fn send(cmd: Command) {
    let _ = std::thread::Builder::new().name("snapshots".into()).spawn(move || {
        // Her `snapshot` isteği shell'in yanıt penceresi kadar bekler (bu thread'de; UI değil).
        let link = IpcLink::from_env().with_timeout(SNAPSHOT_TIMEOUT);
        match cmd {
            Command::Refresh => {}
            Command::Restore { id, setting, label } => {
                let outcome = restore(&link, &id, &setting);
                super::emit(super::Event::SnapshotRestored { text: outcome_text(&label, &outcome) });
                // `hypr.source` geri yüklendiyse shell satırı kaldırmış olabilir.
                super::hypr_status::refresh(Duration::from_millis(300));
                // Varsayılan uygulama geri yüklendiyse `mimeapps.list` değişti: satır shell'in yeni durumunu gösterir.
                if super::default_apps::Role::from_setting_id(&setting).is_some() {
                    super::default_apps::refresh(Duration::from_millis(100));
                }
            }
        }
        let index = query_index(&link);
        *INDEX.lock().unwrap_or_else(|e| e.into_inner()) = Some(index);
        super::emit(super::Event::Snapshots);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::shell_ipc::FakeLink;

    /// `snapshot list --setting look.icons` / `look.cursor` yanıtları: `nuxut-apply`'ın gerçek
    /// biçimi (framework `SnapshotSummary` serde sırası: id, at, setting, label, source, state,
    /// restored_by?, restores?, pinned, fragments, conflicts) + sözleşme dışı girdiler.
    const ICONS: &str = r#"[
      {"id":"20261010T130000Z-0001","at":"2026-10-10T13:00:00Z","setting":"look.icons","label":"İkon teması","source":"settings","state":"rolled_back","pinned":false,"fragments":3,"conflicts":0},
      {"id":"20261010T120000Z-3fa1","at":"2026-10-10T12:00:00Z","setting":"look.icons","label":"İkon teması","source":"settings","state":"applied","pinned":false,"fragments":3,"conflicts":1},
      {"id":"20261010T110000Z-0002","at":"2026-10-10T11:00:00Z","setting":"look.icons","label":"İkon teması","source":"welcome","state":"applied","pinned":false,"fragments":2,"conflicts":0},
      {"id":"kötü","setting":"look.icons","state":"applied"},
      {"id":"20261010T120000Z-3fa2","state":"applied"},
      {"id":"20261010T120000Z-3fa3","setting":"Look/../x","state":"applied"}
    ]"#;
    const CURSOR: &str = r#"[
      {"id":"20261009T090000Z-bbbb","at":"2026-10-09T09:00:00Z","setting":"look.cursor","label":"İmleç","source":"cli","state":"applied","restores":"20261009T080000Z-aaaa","pinned":false,"fragments":2,"conflicts":0},
      {"id":"20261009T080000Z-aaaa","at":"2026-10-09T08:00:00Z","setting":"look.cursor","label":"İmleç","source":"welcome","state":"restored","restored_by":"20261009T090000Z-bbbb","pinned":false,"fragments":2,"conflicts":0}
    ]"#;

    #[test]
    fn list_parses_v1_fields() {
        let entries = parse_list(ICONS).expect("dizi");
        assert_eq!(entries.len(), 3, "geçersiz kimlik, ayarsız ve geçersiz ayar kimlikli girdi atlanır");
        assert_eq!(entries[1].source, "settings");
        assert_eq!((entries[1].fragments, entries[1].conflicts), (3, 1));
        assert_eq!(entries[1].at, id_time("20261010T120000Z-3fa1"));
        assert_eq!(entries[0].state, SnapState::RolledBack);
        let c = parse_list(CURSOR).unwrap();
        assert_eq!(c[0].at, Some(UNIX_EPOCH + Duration::from_secs(1_791_536_400)), "ISO 8601");
        assert_eq!(c[1].state, SnapState::Restored);
        assert_eq!(c[1].restored_by.as_deref(), Some("20261009T090000Z-bbbb"));
        assert_eq!(c[0].restored_by, None);
        // `at` bozuksa kimliğin damgası.
        let e = parse_list(r#"[{"id":"20261010T120000Z-3fa1","setting":"look.mode","at":"dün","state":"applied","fragments":[{},{}]}]"#).unwrap();
        assert_eq!(e[0].at, id_time("20261010T120000Z-3fa1"));
        assert_eq!(e[0].fragments, 2, "liste biçimli parçalar da sayılır");
        assert!(parse_list("ok").is_none());
        assert_eq!(parse_list("[]"), Some(vec![]));
    }

    #[test]
    fn index_keeps_latest_applied_per_setting() {
        let mut entries = parse_list(ICONS).unwrap();
        entries.extend(parse_list(CURSOR).unwrap());
        let idx = SnapshotIndex::from_entries(entries);
        assert_eq!(idx.latest("look.icons").expect("icons").id, "20261010T120000Z-3fa1", "geri sarılan sunulmaz, en yeni applied");
        // 08:00 kaydı geri alınmış (restored); geri yüklemenin kendisi 09:00 applied → onu geri alır.
        assert_eq!(idx.latest("look.cursor").expect("cursor").id, "20261009T090000Z-bbbb");
        assert!(idx.latest("look.mode").is_none());
    }

    #[test]
    fn query_asks_each_setting_and_stops_when_shell_is_gone() {
        let link = FakeLink::with([Ok("[]".to_owned()), Ok(ICONS.to_owned()), Ok(CURSOR.to_owned()), Err(LinkError::NotRunning)]);
        let idx = query_index(&link);
        assert!(idx.latest("look.icons").is_some() && idx.latest("look.cursor").is_some());
        let sent = link.sent();
        assert_eq!(sent.len(), 4, "ilk bağlantı hatasında durur");
        assert_eq!(sent[0], vec!["snapshot", "list", "--setting", "look.mode"]);
        assert_eq!(sent[1], vec!["snapshot", "list", "--setting", "look.icons"]);
        // Yanıt istenen ayarın dışındaki girdiyi taşıyorsa yok sayılır.
        let link = FakeLink::with([Ok(ICONS.to_owned())]);
        assert!(query_index(&link).is_empty(), "look.mode isteğine gelen look.icons girdileri sayılmaz");
        let link = FakeLink::with([Ok("error: bilinmeyen komut".to_owned())]);
        assert!(query_index(&link).is_empty());
        assert_eq!(link.sent().len(), 1);
    }

    #[test]
    fn registry_ids_map_to_shell_settings() {
        assert_eq!(setting_for("desktop.icon_theme"), Some("look.icons"));
        assert_eq!(setting_for("desktop.cursor_size"), Some("look.cursor"));
        assert_eq!(setting_for("colors.mode"), Some("look.mode"));
        assert_eq!(setting_for("desktop.qt_backend"), Some("look.qt_backend"));
        assert_eq!(setting_for("desktop.unified"), Some("look.unified"));
        assert_eq!(setting_for("compositor.ensure_source"), Some("hypr.source"));
        assert_eq!(setting_for("compositor.sync_look"), None, "hypr.<grup> kimlik değil");
        assert_eq!(setting_for("shadow.blur"), None, "normal düzenlemeler snapshot almaz");
        for id in ["colors.mode", "desktop.unified", "desktop.icon_theme", "desktop.cursor_theme", "desktop.cursor_size", "desktop.qt_backend", "typography.ui.family", "compositor.ensure_source"] {
            assert!(crate::registry::by_id(id).is_some(), "{id} kayıtta yok");
            let s = setting_for(id).unwrap();
            assert!(SNAPSHOT_SETTINGS.contains(&s) && valid_setting_id(s), "{s}");
        }
        assert!(!SNAPSHOT_SETTINGS.contains(&"displays"), "displays kimlik değil");
        for bad in ["", ".a", "a.", "a..b", "Look.x", "a b", "a;b"] {
            assert!(!valid_setting_id(bad), "{bad}");
        }
        assert_eq!(list_args(Some("a;b")), None);
        assert_eq!(list_args(None).unwrap(), vec!["snapshot", "list"]);
    }

    #[test]
    fn snapshot_ids_and_times() {
        crate::i18n::use_turkish_for_tests();
        assert!(valid_snapshot_id("20261010T120000Z-3fa1"));
        assert!(!valid_snapshot_id("20261010T120000Z-3FA1"));
        assert!(!valid_snapshot_id("20261010T120000Z-3fa1; rm"));
        assert!(!valid_snapshot_id("../../etc/passwd"));
        let t = id_time("20261010T120000Z-3fa1").unwrap();
        assert_eq!(t.duration_since(UNIX_EPOCH).unwrap().as_secs(), 1_791_633_600);
        assert_eq!(parse_time(&serde_json::json!("2026-10-10T12:00:00Z")), Some(t));
        assert_eq!(parse_time(&serde_json::json!("2026-13-10T12:00:00Z")), None);
        assert_eq!(parse_time(&serde_json::json!("2026-10-10 12:00:00")), None);
        assert!(id_time("20261310T120000Z-3fa1").is_none());
        let now = t + Duration::from_secs(3 * 3600 + 5);
        assert_eq!(ago(t, now), "3 sa önce");
        assert_eq!(ago(t, t + Duration::from_secs(30)), "az önce");
        assert_eq!(ago(t, t + Duration::from_secs(26 * 3600)), "dün");
        assert_eq!(ago(t, t + Duration::from_secs(5 * 86_400)), "5 gün önce");
        assert_eq!(ago(t, t - Duration::from_secs(10)), "az önce", "saat kayması");
        assert_eq!(show_args("20261010T120000Z-3fa1").unwrap(), vec!["snapshot", "show", "20261010T120000Z-3fa1"]);
        assert_eq!(show_args("x"), None);
    }

    const ID: &str = "20261010T120000Z-3fa1";

    #[test]
    fn restore_sends_only_valid_ids_with_setting_and_reads_outcome() {
        let link = FakeLink::with([
            Ok(r#"{"result":"restored","id":"20261010T140000Z-0003"}"#.to_owned()),
            Ok(r#"{"result":"partial","id":"20261010T140000Z-0004","conflicts":[{"kind":"ini_key","target":"~/.config/gtk-4.0/settings.ini","key":"Settings.gtk-icon-theme-name","before":"set","after":"set"}]}"#.to_owned()),
            Ok("error: invalid: \"x\" bilinmeyen ayar kimliği".to_owned()),
        ]);
        assert_eq!(restore(&link, ID, "look.icons"), RestoreOutcome::Restored { wrote: true });
        assert_eq!(restore(&link, ID, "look.icons"), RestoreOutcome::Partial { conflicts: 1 });
        assert_eq!(restore(&link, "geçersiz", "look.icons"), RestoreOutcome::NotFound);
        assert_eq!(restore(&link, ID, "Look/x"), RestoreOutcome::NotFound, "geçersiz ayar kimliği gitmez");
        // `error: …` satırı: shell metni kullanıcıya geçmez.
        let RestoreOutcome::Failed(text) = restore(&link, ID, "look.icons") else { panic!() };
        assert!(!text.contains("invalid") && !text.contains('"'), "{text}");
        let sent = link.sent();
        assert_eq!(sent.len(), 3, "geçersiz kimlik shell'e gitmez");
        assert_eq!(sent[0], vec!["snapshot", "restore", ID, "--setting", "look.icons"]);
        assert_eq!(restore_args(ID, "look.cursor", true).unwrap(), vec!["snapshot", "restore", ID, "--force", "--setting", "look.cursor"]);
    }

    /// Framework `RestoreOutcome`'un her serde biçimi (shell `services::apply` testlerindeki
    /// yanıtlar) ve bilinmeyen/bozuk sonuç.
    #[test]
    fn every_restore_result_kind_parses() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(parse_restore(r#"{"result":"restored","id":"20261010T140000Z-0003"}"#), RestoreOutcome::Restored { wrote: true });
        assert_eq!(parse_restore(r#"{"result":"restored","id":null}"#), RestoreOutcome::Restored { wrote: false });
        assert_eq!(parse_restore(r#"{"result":"partial","id":"20261010T140000Z-0004","conflicts":[{},{}]}"#), RestoreOutcome::Partial { conflicts: 2 });
        assert_eq!(parse_restore(r#"{"result":"conflict","conflicts":[{}]}"#), RestoreOutcome::Conflict { conflicts: 1 });
        assert_eq!(parse_restore(r#"{"result":"not_found"}"#), RestoreOutcome::NotFound);
        assert_eq!(parse_restore(r#"{"result":"busy"}"#), RestoreOutcome::Busy);
        assert_eq!(parse_restore(r#"{"result":"skipped","reason":"test_instance"}"#), RestoreOutcome::Skipped(SkipReason::TestInstance));
        assert_eq!(parse_restore(r#"{"result":"skipped","reason":"no_session"}"#), RestoreOutcome::Skipped(SkipReason::NoSession));
        assert_eq!(parse_restore(r#"{"result":"skipped","reason":"disabled"}"#), RestoreOutcome::Skipped(SkipReason::Disabled));
        assert_eq!(parse_restore(r#"{"result":"skipped"}"#), RestoreOutcome::Skipped(SkipReason::Unknown));
        assert_eq!(parse_restore(r#"{"result":"error","kind":"io"}"#), RestoreOutcome::Error("io".into()));
        assert_eq!(parse_restore(r#"{"result":"error","kind":"<b>x</b>"}"#), RestoreOutcome::Error("bilinmeyen".into()), "kısa ad değil");
        for bad in [r#"{"result":"yeni_sonuc","detail":"gizli"}"#, r#"{"id":"x"}"#, "ok", "", "[]"] {
            let out = parse_restore(bad);
            let RestoreOutcome::Failed(text) = &out else { panic!("{bad}: {out:?}") };
            assert!(!text.contains("yeni_sonuc") && !text.contains("gizli"), "yanıt metni geçmez: {text}");
        }
        let texts = [
            RestoreOutcome::Restored { wrote: true },
            RestoreOutcome::Restored { wrote: false },
            RestoreOutcome::Skipped(SkipReason::TestInstance),
            RestoreOutcome::Skipped(SkipReason::NoSession),
            RestoreOutcome::Unconfirmed,
            RestoreOutcome::Error("io".into()),
            RestoreOutcome::RestoredUnverified,
        ]
        .map(|o| outcome_text("İmleç", &o));
        assert!(texts[0].starts_with("İmleç önceki") && texts[1].contains("zaten"));
        assert!(texts[2].contains("test örneği") && texts[3].contains("oturum"));
        assert!(texts[4].contains("doğrulanamadı") && texts[4].contains("yenile") && texts[5].ends_with("(io)"));
        assert_eq!(texts[6], "İmleç geri yüklendi (ayrıntı doğrulanamadı)", "tamamı döndü iddiası yok");
        assert!(texts.iter().all(|t| !t.contains("Hyprland")));
    }

    /// Yanıt penceresi doldu (zaman aşımı ya da boş kapanan bağlantı): sonuç listeden doğrulanır.
    #[test]
    fn timeout_is_verified_with_list() {
        let restored = r#"[{"id":"20261010T140000Z-0003","at":"2026-10-10T14:00:00Z","setting":"look.icons","label":"İkon teması","source":"cli","state":"applied","restores":"20261010T120000Z-3fa1","pinned":false,"fragments":3,"conflicts":0},
            {"id":"20261010T120000Z-3fa1","at":"2026-10-10T12:00:00Z","setting":"look.icons","label":"İkon teması","source":"settings","state":"restored","restored_by":"20261010T140000Z-0003","pinned":false,"fragments":3,"conflicts":0}]"#;
        let list = vec!["snapshot", "list", "--setting", "look.icons"];
        let restore_req = vec!["snapshot", "restore", ID, "--setting", "look.icons"];
        let run = |replies: Vec<Result<String, LinkError>>| {
            let link = FakeLink::with(replies);
            let slept = Mutex::new(Vec::new());
            let out = restore_with(&link, ID, "look.icons", &|d| slept.lock().unwrap().push(d));
            (out, link.sent(), slept.into_inner().unwrap())
        };

        // İlk doğrulamada (~2 sn) görünür: tek liste isteği.
        let (out, sent, slept) = run(vec![Err(LinkError::Timeout), Ok(restored.to_owned())]);
        assert_eq!(out, RestoreOutcome::RestoredUnverified, "partial/restored ayrımı yok");
        assert_eq!(sent, vec![restore_req.clone(), list.clone()]);
        assert_eq!(slept, vec![Duration::from_secs(2)]);

        // Boş yanıt (pencere doldu / bağlantı koptu) da doğrulama yolu; ilkinde hâlâ applied,
        // ~5 sn sonra ikincide restored.
        let (out, sent, slept) = run(vec![Ok(String::new()), Ok(ICONS.to_owned()), Ok(restored.to_owned())]);
        assert_eq!(out, RestoreOutcome::RestoredUnverified);
        assert_eq!(sent, vec![restore_req.clone(), list.clone(), list.clone()]);
        assert_eq!(slept, VERIFY_DELAYS.to_vec());

        // İki denemede de görünmez (applied / boş liste / yanıtsız): doğrulanamadı; başarı
        // uydurulmaz, restore yeniden GÖNDERİLMEZ, üçüncü liste istenmez.
        for second in [Ok(ICONS.to_owned()), Ok("[]".to_owned()), Err(LinkError::Timeout)] {
            let (out, sent, slept) = run(vec![Err(LinkError::Timeout), Err(LinkError::Timeout), second]);
            assert_eq!(out, RestoreOutcome::Unconfirmed);
            assert_eq!(sent, vec![restore_req.clone(), list.clone(), list.clone()]);
            assert_eq!(sent.iter().filter(|a| a[1] == "restore").count(), 1, "restore yalnızca bir kez");
            assert_eq!(slept, VERIFY_DELAYS.to_vec());
        }

        // Yanıt geldiyse doğrulama yok, bekleme yok.
        let (out, sent, slept) = run(vec![Ok(r#"{"result":"busy"}"#.to_owned())]);
        assert_eq!((out, sent.len(), slept.len()), (RestoreOutcome::Busy, 1, 0));
        assert!(SNAPSHOT_TIMEOUT >= Duration::from_secs(15), "shell yanıt penceresinden kısa beklenmez");
    }

    /// `nuxut-apply` öncesi shell `snapshot`'ı tanımaz: liste boş (düğme yok), geri yükleme sakin
    /// bir hata metniyle biter, panik/yanlış başarı yok.
    #[test]
    fn pre_sh_apply_shell_is_calm() {
        crate::i18n::use_turkish_for_tests();
        let old = "error: bilinmeyen komut: snapshot (nuxut-shell msg help)";
        let link = FakeLink::with([Ok(old.to_owned())]);
        assert!(query_index(&link).is_empty());
        assert_eq!(link.sent().len(), 1, "ilk hatada durur");
        let link = FakeLink::with([Ok(old.to_owned())]);
        let RestoreOutcome::Failed(text) = restore(&link, ID, "look.icons") else { panic!() };
        assert!(!text.contains("bilinmeyen komut"), "{text}");
        assert_eq!(link.sent().len(), 1, "zaman aşımı değil: doğrulama listesi istenmez");
    }

    /// Shell gibi davranan sahte bağlantı: restore'da framework.toml'daki seçim anahtarını
    /// (TomlKey parçası) dosyada geri koyar — dosyanın geri kalanına dokunmadan.
    #[derive(Debug)]
    struct RestoringShell {
        framework: std::path::PathBuf,
    }

    impl ShellLink for RestoringShell {
        fn request(&self, args: &[&str]) -> Result<String, LinkError> {
            assert_eq!(&args[..2], ["snapshot", "restore"]);
            let text = std::fs::read_to_string(&self.framework).unwrap();
            std::fs::write(&self.framework, text.replace("icon_theme = \"Breeze\"", "icon_theme = \"Adwaita\"")).unwrap();
            Ok(r#"{"result":"restored","id":"20261010T140000Z-0003"}"#.into())
        }
    }

    #[test]
    fn restore_value_comes_from_disk_and_settings_does_not_write() {
        use crate::store::{DiskChange, FileId, KeyPath, Store};
        let dir = tempfile::tempdir().unwrap();
        let fw = dir.path().join("framework.toml");
        std::fs::write(&fw, "# kullanıcı yorumu\n[desktop]\nunified = true\nicon_theme = \"Breeze\"\n").unwrap();
        let mut store = Store::open(&fw, dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let icons = crate::registry::by_id("desktop.icon_theme").unwrap();
        assert_eq!(icons.string(&store), "Breeze");

        // Bekleyen bir düzenleme var: App restore isteğinden önce yazar (yarış yok).
        store.set(FileId::Framework, &KeyPath::parse("desktop.cursor_size"), Some(toml_edit::Value::from(32)));
        assert!(store.flush().is_empty());
        assert!(!store.is_dirty());

        let link = RestoringShell { framework: fw.clone() };
        assert_eq!(restore(&link, ID, "look.icons"), RestoreOutcome::Restored { wrote: true });
        let after_shell = std::fs::read_to_string(&fw).unwrap();

        // Değer dosya izleyicisinin yeniden yüklemesinden gelir (dış değişiklik).
        assert_eq!(store.on_disk_changed(FileId::Framework), DiskChange::Reloaded);
        assert_eq!(icons.string(&store), "Adwaita");
        assert_eq!(crate::registry::by_id("desktop.cursor_size").unwrap().number(&store), 32.0, "önceki yazım korunur");
        // Ayarlar hiçbir şey yazmaz: belge temiz, flush dosyaya dokunmaz.
        assert!(!store.is_dirty());
        assert!(store.flush().is_empty());
        assert_eq!(std::fs::read_to_string(&fw).unwrap(), after_shell);
        assert!(after_shell.starts_with("# kullanıcı yorumu"));
    }

    #[test]
    fn partial_and_conflict_texts_say_so() {
        crate::i18n::use_turkish_for_tests();
        assert!(outcome_text("İmleç", &RestoreOutcome::Partial { conflicts: 2 }).contains("kısmen"));
        assert!(outcome_text("İmleç", &RestoreOutcome::Partial { conflicts: 2 }).contains("2 ayara dokunulmadı"));
        assert!(outcome_text("İmleç", &RestoreOutcome::Conflict { conflicts: 1 }).contains("döndürülmedi"));
    }
}
