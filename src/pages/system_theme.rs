//! Açılış görünümü kartı (Hoş geldin "Açılış görünümü" adımı; shell docs/design/boot-theme.md
//! "Hoş geldin sözleşmesi"). Anahtar (varsayılan açık), altında plan / ilerleme / sonuç notu ve
//! uygulanmışsa "Geri al". Uygulamayı Hoş geldin'in birincil düğmesi başlatır
//! ([`crate::services::system_theme::wants_apply`]); kart yalnızca durumu gösterir.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Interaction, Notice, NoticeKind, Toggle, Ui};
use nuxut::{Corners, Point, Rect, Theme};

use super::datetime::RowText;
use super::notice::{notice_height, place_notice};
use crate::block::{Block, Env};
use crate::rows::{row_height_for, row_padding};
use crate::services::system_theme::{self as st, Helper, Op, Outcome, State, StatusError, Target, TargetState};
use crate::store::Store;

/// Hedefin kullanıcıya görünen adı.
fn target_name(t: Target) -> String {
    match t {
        Target::Plymouth => t!("settings-system-theme-target-plymouth"),
        Target::Grub => t!("settings-system-theme-target-grub"),
        Target::Limine => t!("settings-system-theme-target-limine"),
    }
}

/// Notun metni (shell sözleşmesindeki `kind`; bilinmeyen tür gösterilmez).
fn note_text(kind: &str, detail: Option<&str>) -> Option<String> {
    let d = detail.unwrap_or("");
    Some(match kind {
        "plymouth-not-in-initramfs" => t!("settings-system-theme-note-plymouth-not-in-initramfs"),
        "plymouth-no-splash" => t!("settings-system-theme-note-plymouth-no-splash"),
        "plymouth-manual-rebuild" => t!("settings-system-theme-note-plymouth-manual-rebuild", command = if d.is_empty() { "initramfs" } else { d }),
        "grub-text-terminal" => t!("settings-system-theme-note-grub-text-terminal"),
        "grub-hand-written" => t!("settings-system-theme-note-grub-hand-written"),
        "grub-cfg-unreadable" => t!("settings-system-theme-note-grub-cfg-unreadable"),
        "grub-no-dropins" => t!("settings-system-theme-note-grub-no-dropins"),
        "grub-overridden" => t!("settings-system-theme-note-grub-overridden", path = d),
        "limine-ambiguous" => t!("settings-system-theme-note-limine-ambiguous", paths = d),
        "limine-unreadable" => t!("settings-system-theme-note-limine-unreadable"),
        "limine-unsupported" => t!("settings-system-theme-note-limine-unsupported"),
        "limine-enrolled" => t!("settings-system-theme-note-limine-enrolled"),
        _ => return None,
    })
}

fn change_text(kind: &str, what: &str) -> Option<String> {
    Some(match kind {
        "create" => t!("settings-system-theme-change-create", what = what),
        "modify" => t!("settings-system-theme-change-modify", what = what),
        "run" => t!("settings-system-theme-change-run", what = what),
        _ => return None,
    })
}

/// Kartın gösterdiği: anahtar alt metni, anahtar etkin mi, not (tür + metin), geri al ve güncelle
/// görünür mü (ikisi aynı yerde; güncelleme varsa o gösterilir).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    pub subtitle: String,
    pub enabled: bool,
    pub note: Option<(NoticeKind, String)>,
    pub revert: bool,
    pub update: bool,
}

/// Durumdan görünüm (saf; testler).
pub fn view(s: &State) -> View {
    let mut v = View { subtitle: String::new(), enabled: false, note: None, revert: false, update: false };
    let status = match &s.status {
        None => {
            v.subtitle = t!("settings-system-theme-reading");
            return v;
        }
        Some(Err(StatusError::NoShell | StatusError::Unavailable)) => {
            v.subtitle = t!("settings-system-theme-unavailable");
            return v;
        }
        Some(Ok(status)) => status,
    };
    // Alt metin: değişecek hedefler.
    let names: Vec<String> = status.actionable.iter().map(|t| target_name(*t)).collect();
    v.subtitle = if names.is_empty() { t!("settings-system-theme-nothing") } else { names.join(" · ") };
    v.update = status.update && !s.running();
    v.revert = status.any_applied() && !s.running() && !v.update;
    // Yardımcı yoksa ama kaynak kurulumu sistem dosyalarını hazırlamışsa uygulama önce onları
    // kurar; kullanıcıya komut yazdırılmaz (kullanıcı kararı 2026-10-17).
    match (status.helper, status.installable) {
        (Helper::Installed, _) | (_, true) => v.enabled = !status.actionable.is_empty() && !s.running(),
        (Helper::Missing, false) => v.note = Some((NoticeKind::Info, t!("settings-system-theme-helper-missing"))),
        (Helper::Unsafe, false) => v.note = Some((NoticeKind::Warning, t!("settings-system-theme-helper-unsafe"))),
    }
    if let Some(run) = &s.run {
        v.note = Some(run_note(run));
        return v;
    }
    if v.note.is_some() {
        return v;
    }
    // Plan: hedef başına notlar ve (seçiliyse) değişecek yollar.
    let mut lines: Vec<String> = Vec::new();
    if v.update {
        lines.push(t!("settings-system-theme-helper-update"));
    } else if status.helper != Helper::Installed && v.enabled && s.wanted() {
        lines.push(t!("settings-system-theme-helper-install"));
    }
    for tp in &status.targets {
        let notes: Vec<String> = tp.notes.iter().filter_map(|n| note_text(&n.kind, n.detail.as_deref())).collect();
        if tp.state == TargetState::Blocked || !notes.is_empty() {
            lines.push(format!("{}: {}", target_name(tp.target), if notes.is_empty() { t!("settings-system-theme-blocked") } else { notes.join(" ") }));
        }
        if s.wanted() && tp.state == TargetState::Ready {
            lines.extend(tp.changes.iter().filter_map(|c| change_text(&c.kind, &c.what)));
        }
    }
    if status.systemd_boot {
        lines.push(t!("settings-system-theme-systemd-boot"));
    }
    if !lines.is_empty() {
        v.note = Some((NoticeKind::Info, lines.join("\n")));
    } else if status.any_applied() && status.actionable.is_empty() {
        v.note = Some((NoticeKind::Success, t!("settings-system-theme-applied")));
    }
    v
}

/// İşlem notu: sürerken adım + son günlük satırı; bitince sonuç.
fn run_note(run: &st::Run) -> (NoticeKind, String) {
    if !run.finished() {
        let mut text = match &run.step {
            Some((t, step)) => t!("settings-system-theme-running-step", target = target_name(*t), step = step.as_str()),
            None => t!("settings-system-theme-waiting"),
        };
        if let Some(l) = run.log.last() {
            text = format!("{text}\n{l}");
        }
        return (NoticeKind::Info, text);
    }
    if run.cancelled() {
        return (NoticeKind::Info, t!("settings-system-theme-cancelled"));
    }
    let failed: Vec<String> = run
        .outcomes
        .iter()
        .filter(|(_, o, _)| matches!(o, Outcome::RolledBack | Outcome::Inconsistent))
        .map(|(t, o, m)| {
            let (target, message) = (target_name(*t), m.as_deref().unwrap_or(""));
            if *o == Outcome::Inconsistent {
                t!("settings-system-theme-inconsistent", target = target.as_str(), message = message)
            } else {
                t!("settings-system-theme-rolled-back", target = target.as_str(), message = message)
            }
        })
        .collect();
    if !failed.is_empty() {
        return (NoticeKind::Warning, failed.join("\n"));
    }
    if !run.ok() {
        return (NoticeKind::Warning, t!("settings-system-theme-failed"));
    }
    match run.op {
        Op::Apply => (NoticeKind::Success, t!("settings-system-theme-done")),
        Op::Revert => (NoticeKind::Success, t!("settings-system-theme-reverted")),
        Op::Update => (NoticeKind::Success, t!("settings-system-theme-updated")),
    }
}

#[derive(Debug)]
pub struct SystemThemeCard {
    text: RowText,
    toggle: Toggle,
    hover: Interaction,
    note: Notice,
    revert: Button,
    view: View,
    row: Rect,
    note_rect: Rect,
    /// Odak geri al düğmesinde (anahtar kilitliyken).
    revert_focus: bool,
}

impl SystemThemeCard {
    pub fn new(theme: &Theme) -> Self {
        let mut c = Self {
            text: RowText::new(theme, &t!("settings-system-theme-switch")),
            toggle: Toggle::new(theme, true),
            hover: Interaction::new(theme),
            note: Notice::new(theme, NoticeKind::Info, ""),
            revert: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-system-theme-revert")),
            view: view(&State::default()),
            row: Rect::ZERO,
            note_rect: Rect::ZERO,
            revert_focus: false,
        };
        st::refresh();
        c.update(Instant::now());
        c
    }

    fn update(&mut self, now: Instant) {
        let s = st::snapshot();
        self.view = view(&s);
        self.text.set_subtitle(&self.view.subtitle, false);
        self.toggle.set_on(s.wanted() && self.view.enabled, now);
        self.toggle.set_enabled(self.view.enabled, now);
        if let Some((kind, text)) = &self.view.note {
            self.note.set_kind(*kind);
            self.note.set_text(text);
        }
        self.revert.set_label(&if self.view.update { t!("settings-system-theme-update") } else { t!("settings-system-theme-revert") });
    }

    fn flip(&self) {
        if self.view.enabled {
            st::set_wanted(!self.toggle.is_on());
        }
    }

    fn revert_inset(&mut self, env: &mut Env) -> f32 {
        if self.action().is_some() { self.revert.preferred_size(env.text, env.theme).width + env.theme.space(3) } else { 0.0 }
    }

    /// Not yanındaki düğmenin işi: güncelleme varsa "Güncelle", uygulanmışsa "Geri al".
    fn action(&self) -> Option<Op> {
        if self.view.update {
            Some(Op::Update)
        } else if self.view.revert {
            Some(Op::Revert)
        } else {
            None
        }
    }
}

impl Block for SystemThemeCard {
    fn section(&self) -> Option<&str> {
        Some("")
    }

    fn visible(&self, _store: &Store) -> bool {
        true
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - ts.width).max(t.space(20)));
        let row = row_height_for(t, text.max(ts.height));
        let inset = self.revert_inset(env);
        let note = if self.view.note.is_some() || self.action().is_some() { notice_height(&mut self.note, env, width, inset) } else { 0.0 };
        row + note
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (rect.width - px * 3.0 - ts.width).max(t.space(20)));
        let row_h = row_height_for(t, text.max(ts.height));
        self.row = Rect::new(rect.x, rect.y, rect.width, row_h);
        self.toggle.set_rect(Rect::new(rect.right() - px - ts.width, self.row.center().y - ts.height * 0.5, ts.width, ts.height));
        let inset = self.revert_inset(env);
        self.note_rect = Rect::new(rect.x, rect.y + row_h, rect.width, rect.height - row_h);
        place_notice(&mut self.note, t, self.note_rect, inset);
        let bs = self.revert.preferred_size(env.text, t);
        self.revert.set_rect(Rect::new(rect.right() - px - bs.width + t.space(2), self.note_rect.center().y - bs.height * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        if self.action().is_some() {
            let r = self.revert.pointer(e, env.now);
            repaint |= r.repaint;
            if r.event.is_some() {
                if let Some(op) = self.action() {
                    st::start(op);
                }
                return true;
            }
        }
        if self.view.enabled {
            let tap = self.hover.handle(self.row, e, env.now);
            repaint |= tap.repaint;
            if tap.clicked {
                self.flip();
                return true;
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released {
            return false;
        }
        if self.revert_focus && self.action().is_some() {
            let r = self.revert.key(e, env.now);
            if r.event.is_some() {
                if let Some(op) = self.action() {
                    st::start(op);
                }
                return true;
            }
            return r.repaint;
        }
        if self.view.enabled && matches!(e.key, Key::Enter | Key::Space) {
            self.flip();
            return true;
        }
        false
    }

    fn focusable(&self) -> bool {
        self.view.enabled || self.action().is_some()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        // Anahtar etkinse odak onda; değilse (uygulanmış) geri al düğmesinde.
        self.toggle.set_focus_visible(focused && visible && self.view.enabled);
        self.revert_focus = focused && !self.view.enabled && self.action().is_some();
        self.revert.set_focus_visible(self.revert_focus && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.action().is_some()
            && let Some(c) = self.revert.cursor(p)
        {
            return Some(c);
        }
        (self.view.enabled && self.row.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::SystemTheme) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.toggle.set_theme(theme, now);
        self.hover.set_theme(theme, now);
        self.note.set_theme(theme, now);
        self.revert.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        self.hover.draw_layer(ui.painter, self.row, Corners::uniform(t.shape.radius_medium * 0.5), t, ui.now);
        if self.hover.is_animating(ui.now) {
            ui.request_frame();
        }
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.row.x + px, self.row.center().y - h * 0.5);
        self.toggle.draw(ui);
        if self.view.note.is_some() {
            self.note.draw(ui);
        }
        if self.action().is_some() {
            self.revert.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::system_theme::{Change, Note, Run, Status, TargetPlan};

    fn status(helper: Helper, grub: TargetState) -> Status {
        Status {
            targets: vec![
                TargetPlan { target: Target::Plymouth, state: TargetState::Blocked, notes: vec![Note { kind: "plymouth-manual-rebuild".into(), detail: Some("dracut -f".into()) }], changes: vec![] },
                TargetPlan { target: Target::Grub, state: grub, notes: vec![], changes: vec![Change { kind: "modify".into(), what: "/boot/grub/grub.cfg".into() }] },
            ],
            systemd_boot: false,
            helper,
            installable: false,
            update: false,
            actionable: if grub == TargetState::Ready { vec![Target::Grub] } else { vec![] },
        }
    }

    #[test]
    fn view_follows_state() {
        crate::i18n::use_turkish_for_tests();
        let reading = view(&State::default());
        assert!(!reading.enabled && reading.note.is_none() && !reading.subtitle.is_empty());
        let s = State { status: Some(Ok(status(Helper::Installed, TargetState::Ready))), ..State::default() };
        let v = view(&s);
        assert!(v.enabled && !v.revert);
        let (kind, text) = v.note.unwrap();
        assert_eq!(kind, NoticeKind::Info);
        assert!(text.contains("dracut -f") && text.contains("/boot/grub/grub.cfg"), "{text}");
        // Anahtar kapalıyken değişiklik listesi yok, engel notu kalır.
        let off = view(&State { wanted: Some(false), ..s.clone() });
        let text = off.note.unwrap().1;
        assert!(!text.contains("/boot/grub/grub.cfg") && text.contains("dracut -f"));
        // Yardımcı yok: anahtar kapalı, açıklama.
        let missing = view(&State { status: Some(Ok(status(Helper::Missing, TargetState::Ready))), ..State::default() });
        assert!(!missing.enabled && missing.note.is_some());
        assert!(!missing.note.unwrap().1.contains("install.sh"), "komut yazdırılmaz");
        // Hazırlanmış sistem dosyaları: anahtar açık, uygulamanın onları da kuracağı söylenir.
        let installable = view(&State { status: Some(Ok(Status { installable: true, ..status(Helper::Missing, TargetState::Ready) })), ..State::default() });
        assert!(installable.enabled && !installable.update);
        assert!(installable.note.unwrap().1.contains(&t!("settings-system-theme-helper-install")));
        // Kurulu dosyalar eski: düğme "Güncelle" (geri al yerine), not söyler.
        let outdated = view(&State { status: Some(Ok(Status { update: true, ..status(Helper::Installed, TargetState::Applied) })), ..State::default() });
        assert!(outdated.update && !outdated.revert);
        assert!(outdated.note.unwrap().1.contains(&t!("settings-system-theme-helper-update")));
        let mut done = Run::new(Op::Update);
        done.exit = Some(Some(0));
        assert_eq!(view(&State { run: Some(done), ..s.clone() }).note.unwrap(), (NoticeKind::Success, t!("settings-system-theme-updated")));
        // Uygulanmış: geri al görünür.
        let applied = view(&State { status: Some(Ok(status(Helper::Installed, TargetState::Applied))), ..State::default() });
        assert!(applied.revert && !applied.enabled);
        // Sürerken: anahtar ve geri al kilitli, adım görünür.
        let mut run = Run::new(Op::Apply);
        run.feed(r#"{"event":"step","target":"grub","step":"mkconfig","state":"start"}"#);
        let running = view(&State { run: Some(run), ..s.clone() });
        assert!(!running.enabled && !running.revert);
        assert!(running.note.unwrap().1.contains("mkconfig"));
        // Onay verilmedi.
        let mut c = Run::new(Op::Apply);
        c.exit = Some(Some(3));
        assert_eq!(view(&State { run: Some(c), ..s.clone() }).note.unwrap().0, NoticeKind::Info);
        // Geri alındı (rolled-back) uyarı.
        let mut f = Run::new(Op::Apply);
        f.feed(r#"{"event":"target","target":"plymouth","outcome":"rolled-back","message":"mkinitcpio"}"#);
        f.exit = Some(Some(1));
        let (k, text) = view(&State { run: Some(f), ..s }).note.unwrap();
        assert!(k == NoticeKind::Warning && text.contains("mkinitcpio"));
    }

    #[test]
    fn every_contract_note_has_text() {
        crate::i18n::use_turkish_for_tests();
        for k in [
            "plymouth-not-in-initramfs",
            "plymouth-no-splash",
            "plymouth-manual-rebuild",
            "grub-text-terminal",
            "grub-hand-written",
            "grub-cfg-unreadable",
            "grub-no-dropins",
            "grub-overridden",
            "limine-ambiguous",
            "limine-unreadable",
            "limine-unsupported",
            "limine-enrolled",
        ] {
            assert!(note_text(k, Some("x")).is_some_and(|t| !t.is_empty()), "{k}");
        }
        assert!(note_text("gelecekte", None).is_none());
    }
}
