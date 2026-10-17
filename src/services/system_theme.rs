//! Açılış görünümü (Plymouth, GRUB, Limine; shell docs/design/boot-theme.md "Hoş geldin
//! sözleşmesi"). Ayarlar sisteme yazmaz: `nuxut-shell system-theme status --json` ile planı okur,
//! `apply --json` / `revert --json` ile kök yardımcıyı (pkexec, yönetici parolası) çalıştırır ve
//! satır başına gelen olayları izler. Durum süreç genelinde tek (Hoş geldin adımı ve ileride
//! Görünüm sayfası aynı durumu görür); değişince [`super::Event::SystemTheme`].
//!
//! Ayrıştırıcılar saf ve sınırlı (dış girdi): her liste ve metin kırpılır, bilinmeyen değer
//! "bilinmiyor" olarak tutulur, sözleşme dışı yanıt `None`.

use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Hedef başına en çok not / değişiklik ve metin boyu.
const MAX_ITEMS: usize = 32;
const MAX_TEXT: usize = 512;
/// `status` sınırı (algılama ~50 ms; yavaş diskte pay).
const STATUS_TIMEOUT: Duration = Duration::from_secs(20);
/// `status` çıktı sınırı.
const MAX_STATUS_BYTES: u64 = 256 * 1024;
/// Uygulamada okunan en çok olay satırı (mkinitcpio çıktısı dahil) ve tutulan son günlük satırı.
const MAX_EVENT_LINES: usize = 20_000;
const LOG_TAIL: usize = 6;

/// Hedef: açılış ekranı ya da önyükleme menüsü.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Plymouth,
    Grub,
    Limine,
}

impl Target {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "plymouth" => Some(Self::Plymouth),
            "grub" => Some(Self::Grub),
            "limine" => Some(Self::Limine),
            _ => None,
        }
    }
}

/// Hedefin durumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetState {
    Ready,
    Applied,
    Absent,
    Blocked,
    Unknown,
}

/// Plan notu: tür (shell sözleşmesindeki `kind`) ve isteğe bağlı ayrıntı (komut/yol).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub kind: String,
    pub detail: Option<String>,
}

/// Değişiklik: `create` eklenir, `modify` değişir (önce yedeklenir), `run` çalıştırılır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub kind: String,
    pub what: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetPlan {
    pub target: Target,
    pub state: TargetState,
    pub notes: Vec<Note>,
    pub changes: Vec<Change>,
}

/// Kök yardımcının kurulumu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Helper {
    Installed,
    Missing,
    Unsafe,
}

/// `system-theme status --json` (sürüm 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub targets: Vec<TargetPlan>,
    pub systemd_boot: bool,
    pub helper: Helper,
    /// Kaynak kurulumu sistem dosyalarını hazırlamış (`helper_installable`): yardımcı yoksa
    /// uygulama önce onları yönetici onayıyla kurar (shell ADR-0008 ek, 2026-10-17).
    pub installable: bool,
    /// Kurulu sistem dosyaları hazırlıktakilerden eski (`helper_update`): "Güncelle".
    pub update: bool,
    pub actionable: Vec<Target>,
}

impl Status {
    /// Uygulanmış en az bir hedef var mı (geri al gösterilir)?
    pub fn any_applied(&self) -> bool {
        self.targets.iter().any(|t| t.state == TargetState::Applied)
    }
}

fn text(v: Option<&serde_json::Value>) -> Option<String> {
    v?.as_str().map(|s| s.chars().filter(|c| !c.is_control()).take(MAX_TEXT).collect())
}

/// Yanıtı ayrıştırır; sözleşme dışı (`version` 1 değil, `plan` yok) → `None`.
pub fn parse_status(input: &str) -> Option<Status> {
    let v: serde_json::Value = serde_json::from_str(input.trim()).ok()?;
    let o = v.as_object()?;
    if o.get("version")?.as_u64()? != 1 {
        return None;
    }
    let plan = o.get("plan")?.as_object()?;
    let targets = plan
        .get("targets")?
        .as_array()?
        .iter()
        .take(8)
        .filter_map(|t| {
            let target = Target::parse(t.get("target")?.as_str()?)?;
            let state = match t.get("state").and_then(|s| s.as_str()) {
                Some("ready") => TargetState::Ready,
                Some("applied") => TargetState::Applied,
                Some("absent") => TargetState::Absent,
                Some("blocked") => TargetState::Blocked,
                _ => TargetState::Unknown,
            };
            let list = |k: &str| t.get(k).and_then(|l| l.as_array()).map(|l| l.iter().take(MAX_ITEMS).collect::<Vec<_>>()).unwrap_or_default();
            let notes = list("notes")
                .into_iter()
                .filter_map(|n| {
                    let kind = text(n.get("kind"))?;
                    // Ayrıntı alanı türe göre değişir (`command`, `path`, `paths`, `binary`, `reason`).
                    let detail = ["command", "path", "binary", "reason"].iter().find_map(|k| text(n.get(*k))).or_else(|| {
                        n.get("paths").and_then(|p| p.as_array()).map(|p| p.iter().take(8).filter_map(|x| text(Some(x))).collect::<Vec<_>>().join(", "))
                    });
                    Some(Note { kind, detail })
                })
                .collect();
            let changes = list("changes").into_iter().filter_map(|c| Some(Change { kind: text(c.get("kind"))?, what: text(c.get("what"))? })).collect();
            Some(TargetPlan { target, state, notes, changes })
        })
        .collect();
    let helper = match o.get("helper").and_then(|h| h.as_str()) {
        Some("installed") => Helper::Installed,
        Some("unsafe") => Helper::Unsafe,
        _ => Helper::Missing,
    };
    let actionable = o
        .get("actionable")
        .and_then(|a| a.as_array())
        .map(|a| a.iter().take(8).filter_map(|t| Target::parse(t.as_str()?)).collect())
        .unwrap_or_default();
    let flag = |k: &str| o.get(k).and_then(|b| b.as_bool()).unwrap_or(false);
    Some(Status {
        targets,
        systemd_boot: plan.get("systemd_boot").and_then(|b| b.as_bool()).unwrap_or(false),
        helper,
        installable: flag("helper_installable"),
        update: flag("helper_update"),
        actionable,
    })
}

/// Uygulama, geri alma ya da sistem dosyalarını güncelleme (`system-theme install-helper`; olay
/// akışı yok, sonuç çıkış kodundan).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Apply,
    Revert,
    Update,
}

/// Bir hedefin sonucu (yardımcının `target` olayı).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Applied,
    Reverted,
    RolledBack,
    Skipped,
    Inconsistent,
}

/// Çalışan ya da biten işlem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub op: Op,
    /// Şu anki adım (hedef, adım adı).
    pub step: Option<(Target, String)>,
    pub log: Vec<String>,
    pub outcomes: Vec<(Target, Outcome, Option<String>)>,
    /// `done` olayı geldi: (tamam mı, yeniden başlatma gerekli mi).
    pub done: Option<(bool, bool)>,
    /// Süreç bitti: çıkış kodu (3 = yönetici onayı verilmedi; `None` = başlatılamadı/sinyal).
    pub exit: Option<Option<i32>>,
}

impl Run {
    pub fn new(op: Op) -> Self {
        Self { op, step: None, log: Vec::new(), outcomes: Vec::new(), done: None, exit: None }
    }

    pub fn finished(&self) -> bool {
        self.exit.is_some()
    }

    /// Bitti ve her hedef başarılı (ya da atlandı).
    pub fn ok(&self) -> bool {
        self.exit == Some(Some(0)) && (self.op == Op::Update || self.done.is_some_and(|(ok, _)| ok))
    }

    /// Yönetici onayı verilmedi (hiçbir şey değişmedi).
    pub fn cancelled(&self) -> bool {
        self.exit == Some(Some(3))
    }

    /// Bir olay satırını işler (yardımcının `events.rs` biçimi). Tanınmayan satır yok sayılır.
    pub fn feed(&mut self, line: &str) {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line.trim()) else { return };
        let target = || v.get("target").and_then(|t| t.as_str()).and_then(Target::parse);
        match v.get("event").and_then(|e| e.as_str()) {
            Some("step") => {
                if let (Some(t), Some(s)) = (target(), text(v.get("step"))) {
                    self.step = Some((t, s));
                }
            }
            Some("log") => {
                if let Some(l) = text(v.get("line")) {
                    self.log.push(l);
                    if self.log.len() > LOG_TAIL {
                        self.log.remove(0);
                    }
                }
            }
            Some("target") => {
                let outcome = match v.get("outcome").and_then(|o| o.as_str()) {
                    Some("applied") => Outcome::Applied,
                    Some("reverted") => Outcome::Reverted,
                    Some("rolled-back") => Outcome::RolledBack,
                    Some("skipped") => Outcome::Skipped,
                    Some("inconsistent") => Outcome::Inconsistent,
                    _ => return,
                };
                if let Some(t) = target()
                    && self.outcomes.len() < 8
                {
                    self.outcomes.push((t, outcome, text(v.get("message"))));
                }
            }
            Some("done") => {
                let ok = v.get("ok").and_then(|b| b.as_bool()).unwrap_or(false);
                self.done = Some((ok, v.get("reboot_hint").and_then(|b| b.as_bool()).unwrap_or(false)));
            }
            _ => {}
        }
    }
}

/// Süreç genelindeki durum.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    /// `None`: henüz sorulmadı ya da okunuyor.
    pub status: Option<Result<Status, StatusError>>,
    pub querying: bool,
    pub run: Option<Run>,
    /// Kullanıcının seçimi (Hoş geldin anahtarı); ilk kurulumda varsayılan açık (kullanıcı ilkesi
    /// "Nuxut yapabiliyorsa varsayılan Nuxut").
    pub wanted: Option<bool>,
}

impl State {
    pub fn wanted(&self) -> bool {
        self.wanted.unwrap_or(true)
    }

    /// Uygulanabilir mi: yardımcı kurulu (ya da uygulamada kurulabilir), uygulanacak hedef var, iş
    /// sürmüyor.
    pub fn can_apply(&self) -> bool {
        matches!(&self.status, Some(Ok(s)) if (s.helper == Helper::Installed || s.installable) && !s.actionable.is_empty())
            && self.run.as_ref().is_none_or(Run::finished)
    }

    pub fn running(&self) -> bool {
        self.run.as_ref().is_some_and(|r| !r.finished())
    }
}

/// Durum okunamadı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusError {
    /// `nuxut-shell` bulunamadı.
    NoShell,
    /// Bu shell sürümünde yok, zaman aşımı ya da sözleşme dışı yanıt.
    Unavailable,
}

static STATE: Mutex<State> = Mutex::new(State { status: None, querying: false, run: None, wanted: None });

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    f(&mut STATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner))
}

/// Anlık durum.
pub fn snapshot() -> State {
    with(|s| s.clone())
}

pub fn set_wanted(on: bool) {
    with(|s| s.wanted = Some(on));
    super::emit(super::Event::SystemTheme);
}

/// Durumu (arka planda) sorar; zaten soruluyorsa ya da iş sürüyorsa etkisiz.
pub fn refresh() {
    let start = with(|s| {
        let go = !s.querying && !s.running();
        s.querying |= go;
        go
    });
    if !start {
        return;
    }
    let spawned = std::thread::Builder::new().name("system-theme-status".into()).spawn(|| {
        let result = query();
        with(|s| {
            s.querying = false;
            s.status = Some(result);
        });
        super::emit(super::Event::SystemTheme);
    });
    if spawned.is_err() {
        with(|s| {
            s.querying = false;
            s.status = Some(Err(StatusError::Unavailable));
        });
    }
}

fn query() -> Result<Status, StatusError> {
    let bin = super::widgets::shell_bin().ok_or(StatusError::NoShell)?;
    let mut child = Command::new(bin)
        .args(["system-theme", "status", "--json"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| StatusError::Unavailable)?;
    let out = child.stdout.take();
    let reader = std::thread::spawn(move || {
        let mut buf = String::new();
        if let Some(o) = out {
            let _ = o.take(MAX_STATUS_BYTES).read_to_string(&mut buf);
        }
        buf
    });
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if start.elapsed() < STATUS_TIMEOUT => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(StatusError::Unavailable);
            }
        }
    }
    let text = reader.join().unwrap_or_default();
    parse_status(&text).ok_or(StatusError::Unavailable)
}

/// Hoş geldin birincil düğmesi uygulamayı başlatmalı mı: anahtar açık ve uygulanabilir (yardımcı
/// kurulu, uygulanacak hedef var, iş sürmüyor). Başarıdan sonra hedefler `applied` olur, bu yanlışa
/// döner ve düğme ilerler.
pub fn wants_apply() -> bool {
    with(|s| s.wanted() && s.can_apply())
}

/// İş sürüyor mu (Hoş geldin ilerlemez).
pub fn running() -> bool {
    with(|s| s.running())
}

/// Uygular ya da geri alır (pkexec: nuxut-polkit penceresi parola sorar). İş sürüyorsa etkisiz.
/// Bitince durum yeniden sorulur.
pub fn start(op: Op) {
    let go = with(|s| {
        if s.running() {
            return false;
        }
        s.run = Some(Run::new(op));
        true
    });
    if !go {
        return;
    }
    super::emit(super::Event::SystemTheme);
    let spawned = std::thread::Builder::new().name("system-theme-run".into()).spawn(move || {
        let exit = run(op);
        with(|s| {
            if let Some(r) = &mut s.run {
                r.exit = Some(exit);
            }
        });
        super::emit(super::Event::SystemTheme);
        refresh();
    });
    if spawned.is_err() {
        with(|s| {
            if let Some(r) = &mut s.run {
                r.exit = Some(None);
            }
        });
    }
}

fn run(op: Op) -> Option<i32> {
    let bin = super::widgets::shell_bin()?;
    let args: &[&str] = match op {
        Op::Apply => &["system-theme", "apply", "--json"],
        Op::Revert => &["system-theme", "revert", "--json"],
        Op::Update => &["system-theme", "install-helper"],
    };
    let mut child = Command::new(bin).args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    if let Some(out) = child.stdout.take() {
        for line in BufReader::new(out.take(64 * 1024 * 1024)).lines().take(MAX_EVENT_LINES) {
            let Ok(line) = line else { break };
            let changed = with(|s| {
                let Some(r) = &mut s.run else { return false };
                r.feed(&line);
                true
            });
            if changed {
                super::emit(super::Event::SystemTheme);
            }
        }
    }
    child.wait().ok()?.code()
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATUS: &str = r#"{"version":1,"plan":{"targets":[
        {"target":"plymouth","state":"blocked","notes":[{"kind":"plymouth-manual-rebuild","command":"dracut -f"}],"changes":[]},
        {"target":"grub","state":"ready","notes":[],"changes":[{"kind":"create","what":"/boot/grub/themes/nuxut"},{"kind":"modify","what":"/boot/grub/grub.cfg"}]},
        {"target":"limine","state":"absent","notes":[{"kind":"limine-ambiguous","paths":["/boot/limine.conf","/efi/limine.conf"]}],"changes":[]},
        {"target":"yeni","state":"ready"}],"systemd_boot":true},
      "helper":"installed","helper_path":"/usr/lib/nuxut/nuxut-system-helper","helper_update":true,"actionable":["grub","bilinmeyen"],"scale":1,"lang":"tr","colors":[]}"#;

    #[test]
    fn status_contract() {
        let s = parse_status(STATUS).unwrap();
        assert_eq!(s.targets.len(), 3, "bilinmeyen hedef atlanır");
        assert_eq!(s.targets[0].notes[0], Note { kind: "plymouth-manual-rebuild".into(), detail: Some("dracut -f".into()) });
        assert_eq!(s.targets[1].changes[1], Change { kind: "modify".into(), what: "/boot/grub/grub.cfg".into() });
        assert_eq!(s.targets[2].notes[0].detail.as_deref(), Some("/boot/limine.conf, /efi/limine.conf"));
        assert_eq!((s.helper, s.systemd_boot, s.installable, s.update), (Helper::Installed, true, false, true));
        assert_eq!(s.actionable, [Target::Grub]);
        assert!(!s.any_applied());
        assert!(parse_status(r#"{"version":2,"plan":{"targets":[]}}"#).is_none(), "yeni sürüm sözleşme dışı");
        assert!(parse_status("ok").is_none());
        // Denetim karakterleri ve uzun metin kırpılır.
        let evil = STATUS.replace("dracut -f", &format!("a\\u001b[31m{}", "x".repeat(2000)));
        let n = &parse_status(&evil).unwrap().targets[0].notes[0];
        let d = n.detail.as_deref().unwrap();
        assert!(!d.contains('\u{1b}') && d.chars().count() == MAX_TEXT);
    }

    #[test]
    fn events_drive_the_run() {
        let mut r = Run::new(Op::Apply);
        for l in [
            r#"{"event":"plan","plan":{}}"#,
            r#"{"event":"step","target":"grub","step":"mkconfig","state":"start"}"#,
            r#"{"event":"log","source":"grub-mkconfig","line":"Generating grub configuration file ..."}"#,
            "çöp satır",
            r#"{"event":"target","target":"grub","outcome":"applied","message":null}"#,
            r#"{"event":"target","target":"plymouth","outcome":"rolled-back","message":"mkinitcpio başarısız"}"#,
            r#"{"event":"done","ok":false,"reboot_hint":true}"#,
        ] {
            r.feed(l);
        }
        assert_eq!(r.step, Some((Target::Grub, "mkconfig".into())));
        assert_eq!(r.log, ["Generating grub configuration file ..."]);
        assert_eq!(r.outcomes[1], (Target::Plymouth, Outcome::RolledBack, Some("mkinitcpio başarısız".into())));
        assert_eq!(r.done, Some((false, true)));
        assert!(!r.finished() && !r.ok());
        r.exit = Some(Some(1));
        assert!(r.finished() && !r.ok() && !r.cancelled());
        let mut c = Run::new(Op::Apply);
        c.exit = Some(Some(3));
        assert!(c.cancelled());
        // Günlük kuyruğu sınırlı.
        let mut long = Run::new(Op::Revert);
        for i in 0..50 {
            long.feed(&format!(r#"{{"event":"log","source":"x","line":"{i}"}}"#));
        }
        assert_eq!(long.log.len(), LOG_TAIL);
        assert_eq!(long.log.last().map(String::as_str), Some("49"));
    }

    #[test]
    fn can_apply_needs_helper_targets_and_idle() {
        let st = parse_status(STATUS).unwrap();
        let mut s = State { status: Some(Ok(st.clone())), ..State::default() };
        assert!(s.wanted() && s.can_apply(), "varsayılan seçili");
        s.run = Some(Run::new(Op::Apply));
        assert!(!s.can_apply() && s.running());
        s.run.as_mut().unwrap().exit = Some(Some(0));
        assert!(s.can_apply());
        let missing = Status { helper: Helper::Missing, ..st.clone() };
        assert!(!State { status: Some(Ok(missing.clone())), ..State::default() }.can_apply());
        // Hazırlanmış sistem dosyaları: uygulama önce onları kurar (2026-10-17).
        let installable = Status { installable: true, ..missing };
        assert!(State { status: Some(Ok(installable)), ..State::default() }.can_apply());
        let mut u = Run::new(Op::Update);
        u.exit = Some(Some(0));
        assert!(u.ok(), "güncelleme olay akışsız: çıkış 0 yeter");
        let none = Status { actionable: vec![], ..st };
        assert!(!State { status: Some(Ok(none)), ..State::default() }.can_apply());
    }
}
