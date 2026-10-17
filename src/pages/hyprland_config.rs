//! Hoş geldin "Masaüstüyle ortak" adımında Hyprland ana ayarının devralınması seçimi (shell
//! ADR-0007; kullanıcı onayı 2026-10-17). Satır dosyaya yazmaz: seçim paylaşılan hücrede tutulur,
//! Bitir'e basılınca (shell `setup done`'u aldıysa) `nuxut-shell hypr takeover` çalışır. Shell
//! sonucu "Geri al" düğmeli bildirimle gösterir (`hypr restore-backup`).
//!
//! Adım yalnızca gerçek ilk kurulumda açılan Hoş geldin'dedir ve yalnızca ana ayar henüz
//! Nuxut'un değilse görünür ([`available`]); seçili gelir.

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent};
use nuxut::ui::{Interaction, Toggle, Ui};
use nuxut::{Corners, Point, Rect, Theme};

use super::datetime::RowText;
use crate::block::{Block, Env};
use crate::rows::{row_height_for, row_padding};

/// Shell'in devralınmış ana dosya işaretleri (shell `hypr_config::takeover::MARKER_*`; sözleşme
/// testi aynılığı denetler).
pub const MARKER_CONF: &str = "# nuxut: managed main config v1";
pub const MARKER_LUA: &str = "-- nuxut: managed main config v1";
/// Okunan ilk bayt sınırı (işaret ilk boş olmayan satırda).
const HEAD_BYTES: u64 = 4096;

/// Hyprland'in seçeceği ana dosya: `hyprland.lua` varsa o, yoksa `hyprland.conf` (shell
/// `SyncEnv::config_path` ile aynı sıra; çalışan oturumun `-c` değeri burada bilinmez).
fn main_config(config_home: &Path) -> Option<PathBuf> {
    let dir = config_home.join("hypr");
    ["hyprland.lua", "hyprland.conf"].into_iter().map(|n| dir.join(n)).find(|p| p.is_file())
}

/// Ana ayar devralınabilir mi: dosya var ve ilk boş olmayan satırı Nuxut işareti değil.
pub fn available_in(config_home: &Path) -> bool {
    use std::io::Read as _;
    let Some(path) = main_config(config_home) else { return false };
    let mut head = Vec::new();
    if std::fs::File::open(&path).and_then(|f| f.take(HEAD_BYTES).read_to_end(&mut head)).is_err() {
        return false;
    }
    let text = String::from_utf8_lossy(&head);
    let first = text.lines().map(str::trim).find(|l| !l.is_empty());
    !matches!(first, Some(MARKER_CONF | MARKER_LUA))
}

/// Bu oturumda gösterilsin mi: Hyprland oturumu ve devralınabilir ana ayar.
pub fn available() -> bool {
    let session = std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some_and(|s| !s.is_empty());
    let config_home = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")));
    session && config_home.is_some_and(|c| available_in(&c))
}

/// Bitir'den sonra devralmayı ayrık başlatır (beklemez; sonuç shell bildiriminde).
pub fn spawn_takeover() {
    let Some(bin) = crate::services::widgets::shell_bin() else {
        tracing::warn!("Hyprland ayarı devralınamadı: nuxut-shell bulunamadı");
        return;
    };
    let spawned = std::thread::Builder::new().name("hyprland-config".into()).spawn(move || {
        let out = std::process::Command::new(&bin).args(["hypr", "takeover"]).stdin(std::process::Stdio::null()).output();
        match out {
            Ok(o) if o.status.success() => tracing::info!("Hyprland ana ayarı devralındı"),
            Ok(o) => tracing::warn!(stdout = %String::from_utf8_lossy(&o.stdout).chars().take(2000).collect::<String>(), "Hyprland ana ayarı devralınamadı"),
            Err(e) => tracing::warn!(%e, "nuxut-shell hypr takeover çalıştırılamadı"),
        }
    });
    if let Err(e) = spawned {
        tracing::warn!(%e, "devralma iş parçacığı başlatılamadı");
    }
}

#[derive(Debug)]
pub struct HyprTakeover {
    choice: Rc<Cell<bool>>,
    text: RowText,
    toggle: Toggle,
    hover: Interaction,
    rect: Rect,
}

impl HyprTakeover {
    pub fn new(theme: &Theme, choice: Rc<Cell<bool>>) -> Self {
        let mut text = RowText::new(theme, &t!("settings-welcome-hyprland-config-title"));
        text.set_subtitle(&t!("settings-welcome-hyprland-config-body"), false);
        Self { toggle: Toggle::new(theme, choice.get()), choice, text, hover: Interaction::new(theme), rect: Rect::ZERO }
    }

    fn flip(&mut self, now: Instant) {
        let on = !self.choice.get();
        self.choice.set(on);
        self.toggle.set_on(on, now);
    }
}

impl Block for HyprTakeover {
    fn section(&self) -> Option<&str> {
        Some("")
    }

    fn id(&self) -> Option<&'static str> {
        None
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - ts.width).max(t.space(20)));
        row_height_for(t, text.max(ts.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        self.toggle.set_rect(Rect::new(rect.right() - px - ts.width, rect.center().y - ts.height * 0.5, ts.width, ts.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let tap = self.hover.handle(self.rect, e, env.now);
        if tap.clicked {
            self.flip(env.now);
            return true;
        }
        tap.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) {
            self.flip(env.now);
            return true;
        }
        false
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.toggle.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rect.contains(p).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.toggle.set_theme(theme, now);
        self.hover.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        self.hover.draw_layer(ui.painter, self.rect, Corners::uniform(t.shape.radius_medium * 0.5), t, ui.now);
        if self.hover.is_animating(ui.now) {
            ui.request_frame();
        }
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.center().y - h * 0.5);
        self.toggle.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Yalnızca Nuxut'u göstermeyen ana dosyada; Lua önce; dosya yoksa yok.
    #[test]
    fn availability_follows_main_file_marker() {
        let tmp = tempfile::tempdir().unwrap();
        let hypr = tmp.path().join("hypr");
        std::fs::create_dir_all(&hypr).unwrap();
        assert!(!available_in(tmp.path()), "ana dosya yok");
        std::fs::write(hypr.join("hyprland.conf"), "source = ~/.config/hypr/hyprland/keybinds.conf\n").unwrap();
        assert!(available_in(tmp.path()));
        std::fs::write(hypr.join("hyprland.conf"), format!("\n{MARKER_CONF}\nsource = x\n")).unwrap();
        assert!(!available_in(tmp.path()), "zaten Nuxut'un");
        std::fs::write(hypr.join("hyprland.lua"), "hl.config({})\n").unwrap();
        assert!(available_in(tmp.path()), "Lua seçilir");
    }

    /// İşaretler shell'in sözleşmesiyle aynı.
    #[test]
    fn markers_match_shell() {
        assert_eq!(MARKER_CONF, nuxut_shell::services::hypr_config::takeover::MARKER_CONF);
        assert_eq!(MARKER_LUA, nuxut_shell::services::hypr_config::takeover::MARKER_LUA);
    }

    /// Satıra tıklamak seçimi değiştirir; hücre Bitir'de okunur.
    #[test]
    fn toggle_flips_shared_choice() {
        let theme = Theme::default();
        let choice = Rc::new(Cell::new(true));
        let mut row = HyprTakeover::new(&theme, choice.clone());
        row.flip(Instant::now());
        assert!(!choice.get());
        row.flip(Instant::now());
        assert!(choice.get());
    }
}
