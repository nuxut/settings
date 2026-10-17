//! Hoş geldin akışı (shell ADR-0004 §9, ADR-0001 §10): karşılama + 4 kısa adım +
//! bitiş. Her adım kayıttaki mevcut ayar satırlarını gösterir (kontrol kopyası yok); seçimler
//! normal yoldan anında yazılır ve geri alınabilir.
//!
//! Bu dosya saf durum modelidir (ileri/geri/atla, ilerleme, shell'e `msg setup` bildirimi);
//! pencere `app.rs`'teki hoş geldin kipindedir.
//!
//! Shell v1 sözleşmesi: karşılamayı shell açar (`nuxut-settings --welcome`) ve
//! `$XDG_STATE_HOME/nuxut/setup.toml`'u yalnızca shell yazar. Ayarlar bitişte `msg setup done <n>`,
//! pencere bitmeden kapanırsa `msg setup skip <n>` gönderir (n = akış sürümü); dosyaya dokunmaz, kendiliğinden açılmaz.

use nuxut::i18n::t;

/// Bir adım: başlık, kısa açıklama (katalogdan, [`Step::title`]/[`Step::body`]) ve gösterilen
/// ayarlar (kayıt kimlikleri; kayıtta olmayan kimlik sessizce atlanır — diğer işçilerin
/// satırları gelince kendiliğinden görünür).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Step {
    pub kind: StepKind,
    pub settings: &'static [&'static str],
}

/// Adımın kimliği; metinleri etkin dilde buradan okunur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepKind {
    Look,
    Apps,
    Desktop,
    /// Açılış görünümü (Plymouth/GRUB/Limine; shell docs/design/boot-theme.md). Satırı kayıtta
    /// değil, özel kart (`pages::system_theme`); uygulamayı birincil düğme başlatır.
    System,
}

impl Step {
    pub fn title(&self) -> String {
        match self.kind {
            StepKind::Look => t!("settings-welcome-step1-title"),
            StepKind::Apps => t!("settings-welcome-step2-title"),
            StepKind::Desktop => t!("settings-welcome-step3-title"),
            StepKind::System => t!("settings-welcome-step4-title"),
        }
    }

    pub fn body(&self) -> String {
        match self.kind {
            StepKind::Look => t!("settings-welcome-step1-body"),
            StepKind::Apps => t!("settings-welcome-step2-body"),
            StepKind::Desktop => t!("settings-welcome-step3-body"),
            StepKind::System => t!("settings-welcome-step4-body"),
        }
    }
}

pub const STEPS: [Step; 4] = [
    Step { kind: StepKind::Look, settings: &["colors.mode", "wallpaper", "wallpaper.builtin", "colors.source"] },
    Step {
        kind: StepKind::Apps,
        settings: &["desktop.unified", "desktop.status", "desktop.icon_theme", "desktop.cursor_theme", "desktop.cursor_size"],
    },
    Step {
        kind: StepKind::Desktop,
        // ADR-0004 §9 adım 3. `compositor.ensure_source` (varsayılan açık) teknik açıklamalı olduğu
        // için burada değil; Pencereler › Gelişmiş'te.
        settings: &["compositor.sync_look", "compositor.sync_windows", "compositor.sync_motion", "compositor.sync_keybinds"],
    },
    Step { kind: StepKind::System, settings: &[] },
];

// "Uygulamalar" adımı birleşik görünüm anahtarını ve ona bağlı satırları gösterir; kapalıyken
// gizlenen satırlar ve yerlerine gelen açıklama Görünüm sayfasıyla ortaktır
// ([`crate::registry::UNIFIED_GATED`], [`crate::pages::unified_off_note`]).

/// Giriş ve bitiş ekranlarının (başlık, açıklama) metni, etkin dilde.
pub fn intro_text() -> (String, String) {
    (t!("settings-welcome-intro-title"), t!("settings-welcome-intro-body"))
}

pub fn done_text() -> (String, String) {
    (t!("settings-welcome-done-title"), t!("settings-welcome-done-body"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Intro,
    Step(usize),
    Done,
}

/// Bir adımın sonucu (ilerleme göstergesi ve bitiş özeti için).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visit {
    NotYet,
    Seen,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Welcome {
    screen: Screen,
    visits: [Visit; STEPS.len()],
}

impl Default for Welcome {
    fn default() -> Self {
        Self::new()
    }
}

impl Welcome {
    pub fn new() -> Self {
        Self { screen: Screen::Intro, visits: [Visit::NotYet; STEPS.len()] }
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn step(&self) -> Option<&'static Step> {
        match self.screen {
            Screen::Step(i) => STEPS.get(i),
            _ => None,
        }
    }

    pub fn visits(&self) -> &[Visit] {
        &self.visits
    }

    fn enter(&mut self, screen: Screen) {
        self.screen = screen;
        if let Screen::Step(i) = screen
            && self.visits[i] == Visit::NotYet
        {
            self.visits[i] = Visit::Seen;
        }
    }

    /// Birincil düğme: Başla / İleri / Bitir.
    pub fn next(&mut self) {
        let to = match self.screen {
            Screen::Intro => Screen::Step(0),
            Screen::Step(i) if i + 1 < STEPS.len() => Screen::Step(i + 1),
            Screen::Step(_) | Screen::Done => Screen::Done,
        };
        self.enter(to);
    }

    /// Geri: önceki ekran (girişte etkisiz).
    pub fn back(&mut self) {
        let to = match self.screen {
            Screen::Intro | Screen::Step(0) => Screen::Intro,
            Screen::Step(i) => Screen::Step(i - 1),
            Screen::Done => Screen::Step(STEPS.len() - 1),
        };
        self.enter(to);
    }

    /// Atla: bu adımı atlanmış say ve ilerle (seçilen değerler olduğu gibi kalır).
    pub fn skip(&mut self) {
        if let Screen::Step(i) = self.screen {
            self.visits[i] = Visit::Skipped;
        }
        self.next();
    }

    pub fn can_back(&self) -> bool {
        self.screen != Screen::Intro
    }

    pub fn can_skip(&self) -> bool {
        matches!(self.screen, Screen::Step(_))
    }

    pub fn is_done(&self) -> bool {
        self.screen == Screen::Done
    }

    pub fn primary_label(&self) -> String {
        match self.screen {
            Screen::Intro => t!("settings-welcome-button-start"),
            Screen::Step(i) if i + 1 < STEPS.len() => t!("settings-welcome-button-next"),
            Screen::Step(_) => t!("settings-welcome-button-finish"),
            Screen::Done => t!("settings-welcome-button-close"),
        }
    }

    /// Üstteki ilerleme ("Adım 2 / 3").
    pub fn progress_text(&self) -> Option<String> {
        match self.screen {
            Screen::Step(i) => Some(t!("settings-welcome-progress", current = (i + 1) as i64, total = STEPS.len() as i64)),
            _ => None,
        }
    }

    /// İlerleme noktaları: (etkin mi, ziyaret edildi mi).
    pub fn dots(&self) -> [(bool, bool); STEPS.len()] {
        std::array::from_fn(|i| (self.screen == Screen::Step(i), self.visits[i] != Visit::NotYet))
    }
}

// ---- Shell'e bildirim (`msg setup`) ---------------------------------------------------------

// Shell ADR-0001 §10.5 (donuk v1): `setup.toml`'u YALNIZCA shell yazar. Ayarlar sonucu
// bildirir: bitince `setup done <n>`, bitmeden kapatılınca `setup skip <n>` (n = akış sürümü);
// yanıt `ok` | `error: …`. Karşılamayı shell açar (`nuxut-settings --welcome`, ayrık, `NUXUT_*`
// ortamı aktarılmaz → normal örnek, varsayılan shell'e konuşur).

use crate::services::shell_ipc::{LinkError, ShellLink};

/// Hoş geldin AKIŞININ sürümü (1..=65535): adımlar anlamlı biçimde değişince artar; shell daha
/// küçük sürümün büyüğü düşürmesine izin vermez. Uygulama sürümü değildir.
/// 2 (2026-10-17): "Açılış görünümü" adımı eklendi.
pub const WELCOME_FLOW_VERSION: u16 = 2;
const _: () = assert!(WELCOME_FLOW_VERSION >= 1, "akış sürümü 1..=65535");

/// Hoş geldin sonucu için shell komutu (`finished`: Bitir'e basıldı).
pub fn setup_args(finished: bool) -> [String; 3] {
    ["setup".to_owned(), if finished { "done" } else { "skip" }.to_owned(), WELCOME_FLOW_VERSION.to_string()]
}

/// Sonucu shell'e bildirir. Shell çalışmıyor / test örneği → bildirilmez (shell bir sonraki
/// açılışta karşılamayı yeniden önerir; hata değil).
pub fn report_setup(link: &dyn ShellLink, finished: bool) -> Result<(), LinkError> {
    let args = setup_args(finished);
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    link.request(&args).map(drop)
}

/// `setup_state`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetupState {
    Done,
    Skipped,
    /// Sözleşmede olmayan (yeni sürüm) değer.
    Other(String),
}

/// `hypr` alt nesnesi. `source` `msg hypr status`'un değeri olduğu gibi (off, present, added,
/// disabled_by_user, removed_by_user, missing_config, unsupported, pending, inactive, unknown,
/// error, undone …) — bilinmeyen değerde kırılmamak için metin tutulur.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SetupHypr {
    pub session: bool,
    pub source: Option<String>,
    /// `exec_once` | `external` | `systemd`; ilk eşitleme bitmeden `None`.
    pub launch: Option<String>,
}

/// `msg setup status` yanıtı (ADR-0001 §10.2 + §10.5).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SetupStatus {
    /// Yalnızca `setup.toml` yoksa `true` (bozuk dosyada `false`).
    pub first_run: bool,
    /// Kaydedilen akış sürümü.
    pub setup_version: Option<u16>,
    pub setup_state: Option<SetupState>,
    pub hypr: SetupHypr,
    /// Geri alınabilir son göçün opak etiketi (`migration.shell:1-2`; geri alma
    /// `msg config undo-migration`). Şimdilik yalnızca ayrıştırılır.
    pub pending_notice: Option<String>,
}

fn short(v: Option<&serde_json::Value>, max: usize) -> Option<String> {
    v?.as_str().filter(|s| !s.is_empty() && s.len() <= max).map(str::to_owned)
}

/// `setup status` yanıtı. Sözleşme dışı (JSON nesnesi değil, `first_run` yok) → `None`;
/// bilinmeyen alanlar ve değerler yok sayılır / metin olarak tutulur.
pub fn parse_setup_status(text: &str) -> Option<SetupStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    let first_run = o.get("first_run")?.as_bool()?;
    let setup_state = short(o.get("setup_state"), 32).map(|s| match s.as_str() {
        "done" => SetupState::Done,
        "skipped" => SetupState::Skipped,
        _ => SetupState::Other(s),
    });
    let hypr = o.get("hypr").and_then(|h| h.as_object()).map_or_else(SetupHypr::default, |h| SetupHypr {
        session: h.get("session").and_then(|x| x.as_bool()).unwrap_or(false),
        source: short(h.get("source"), 32),
        launch: short(h.get("launch"), 32),
    });
    Some(SetupStatus {
        first_run,
        setup_version: o.get("setup_version").and_then(|x| x.as_u64()).and_then(|n| u16::try_from(n).ok()).filter(|n| *n >= 1),
        setup_state,
        hypr,
        pending_notice: o.get("migration").and_then(|m| short(m.get("pending_notice"), 128)),
    })
}

/// Durumu sorar (saf; testte sahte bağlantı).
pub fn query_setup_status(link: &dyn ShellLink) -> Result<SetupStatus, LinkError> {
    parse_setup_status(&link.request(&["setup", "status"])?).ok_or(LinkError::BadReply)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walks_forward_back_and_skip() {
        crate::i18n::use_turkish_for_tests();
        let mut w = Welcome::new();
        assert_eq!(w.screen(), Screen::Intro);
        assert!(!w.can_back() && !w.can_skip());
        assert_eq!(w.primary_label(), "Başla");
        w.back();
        assert_eq!(w.screen(), Screen::Intro, "girişte geri etkisiz");
        w.next();
        assert_eq!(w.screen(), Screen::Step(0));
        assert_eq!(w.progress_text().as_deref(), Some("Adım 1 / 4"));
        assert_eq!(w.primary_label(), "İleri");
        w.skip();
        assert_eq!(w.screen(), Screen::Step(1));
        assert_eq!(w.visits()[0], Visit::Skipped);
        w.next();
        assert_eq!(w.screen(), Screen::Step(2));
        assert_eq!(w.primary_label(), "İleri");
        w.next();
        assert_eq!(w.screen(), Screen::Step(3));
        assert_eq!(w.primary_label(), "Bitir");
        assert_eq!(w.dots(), [(false, true), (false, true), (false, true), (true, true)]);
        w.back();
        assert_eq!(w.screen(), Screen::Step(2));
        w.next();
        w.next();
        assert!(w.is_done());
        assert_eq!(w.primary_label(), "Kapat");
        w.back();
        assert_eq!(w.screen(), Screen::Step(3), "bitişten geri son adıma");
        assert_eq!(w.visits()[0], Visit::Skipped, "geri dönmek atlanmışı silmez");
    }

    #[test]
    fn steps_reference_registry_rows() {
        crate::i18n::use_turkish_for_tests();
        // Her adımın satırı kayıtta (adı değişirse test kırılsın, adım sessizce boşalmasın).
        for s in STEPS {
            for id in s.settings {
                {
                    let d = crate::registry::by_id(id).unwrap_or_else(|| panic!("{id} kayıtta yok"));
                    for word in ["GTK", "Qt", "Hyprland", "dconf"] {
                        assert!(!d.title.contains(word) && !d.subtitle.contains(word), "{id}: {word}");
                    }
                }
            }
            let (title, body) = (s.title(), s.body());
            assert!(!title.is_empty() && !body.is_empty());
            // Teknik ad görünmez (ADR-0004 §9).
            for word in ["GTK", "Qt", "Hyprland", "dconf"] {
                assert!(!title.contains(word) && !body.contains(word), "{word}");
            }
        }
    }

    #[test]
    fn unified_only_rows_belong_to_apps_step() {
        for id in ["desktop.icon_theme", "desktop.cursor_theme", "desktop.cursor_size"] {
            assert!(crate::registry::UNIFIED_GATED.contains(&id), "{id}");
            assert!(STEPS[1].settings.contains(&id), "{id}");
        }
        assert!(STEPS[1].settings.contains(&"desktop.unified"));
    }

    #[test]
    fn setup_is_reported_to_shell_with_flow_version() {
        use crate::services::shell_ipc::FakeLink;
        assert_eq!(setup_args(true), ["setup", "done", "2"]);
        assert_eq!(setup_args(false), ["setup", "skip", "2"]);
        let link = FakeLink::with([Ok("ok".to_owned()), Ok("ok".to_owned()), Ok("error: yazılamadı".to_owned()), Err(LinkError::NotRunning)]);
        assert!(report_setup(&link, true).is_ok());
        assert!(report_setup(&link, false).is_ok());
        assert_eq!(report_setup(&link, true), Err(LinkError::Shell("yazılamadı".into())));
        assert_eq!(report_setup(&link, true), Err(LinkError::NotRunning));
        assert_eq!(link.sent()[0], vec!["setup", "done", "2"]);
        assert_eq!(link.sent()[1], vec!["setup", "skip", "2"]);
    }

    #[test]
    fn setup_status_contract() {
        use crate::services::shell_ipc::FakeLink;
        // ADR-0001 §10.5 örneği.
        let s = parse_setup_status(
            r#"{"first_run":true,"setup_version":null,"setup_state":null,"hypr":{"session":true,"source":"present","launch":"exec_once"},"migration":{"pending_notice":null}}"#,
        )
        .unwrap();
        assert_eq!(
            s,
            SetupStatus {
                first_run: true,
                setup_version: None,
                setup_state: None,
                hypr: SetupHypr { session: true, source: Some("present".into()), launch: Some("exec_once".into()) },
                pending_notice: None,
            }
        );
        let s = parse_setup_status(
            r#"{"first_run":false,"setup_version":3,"setup_state":"skipped","hypr":{"session":false,"source":"yepyeni","launch":null},"migration":{"pending_notice":"migration.shell:1-2"},"gelecek":1}"#,
        )
        .unwrap();
        assert_eq!(s.setup_version, Some(3));
        assert_eq!(s.setup_state, Some(SetupState::Skipped));
        assert_eq!(s.hypr.source.as_deref(), Some("yepyeni"), "bilinmeyen değerde kırılmaz");
        assert_eq!(s.hypr.launch, None, "ilk eşitlemeden önce null");
        assert_eq!(s.pending_notice.as_deref(), Some("migration.shell:1-2"));
        assert_eq!(parse_setup_status(r#"{"first_run":false,"setup_state":"done"}"#).unwrap().setup_state, Some(SetupState::Done));
        assert_eq!(parse_setup_status(r#"{"first_run":false,"setup_version":70000}"#).unwrap().setup_version, None);
        assert_eq!(parse_setup_status(r#"{"v":1,"state":"done"}"#), None, "eski varsayım sözleşme dışı");
        assert_eq!(parse_setup_status("ok"), None);
        let link = FakeLink::with([Ok(r#"{"first_run":false,"setup_version":1,"setup_state":"done"}"#.to_owned()), Ok("[]".to_owned())]);
        assert_eq!(query_setup_status(&link).unwrap().setup_state, Some(SetupState::Done));
        assert_eq!(query_setup_status(&link), Err(LinkError::BadReply));
        assert!(link.sent().iter().all(|a| a == &["setup", "status"]));
    }
}
