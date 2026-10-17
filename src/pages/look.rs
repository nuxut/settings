//! Görünüm › Renkler (koyu/açık) ve › Uygulamalar (shell ADR-0004 birleşik görünüm): ikon ve imleç teması,
//! shell'in uygulama durumu. Ayarlar yalnızca `framework.toml` yazar (`[colors] mode`,
//! `[desktop]`); GTK/Qt/ikon/imleç hedeflerine shell uygular (§12-1).
//!
//! - [`ModeChoice`]: Otomatik | Koyu | Açık. Otomatik varsayılandır (anahtar silinir); açık seçimde
//!   renkler duvar kağıdından geliyorsa paletin kipi de aynı geri al adımında yazılır
//!   ([`mode_writes`]).
//! - İkon ve imleç teması: [`super::theme_grid::ThemeGrid`] kart ızgarası (t103).
//! - [`LookStatus`]: `msg look status`'tan satır içi durum (uygulandı / eksik paket + kurulum
//!   komutunu kopyala / kabuk çalışmıyor).

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Label, Notice, NoticeKind, Segment, SegmentedButton, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Size, TextSystem, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{PreviousAction, is_narrow, row_height_for, row_padding};
use crate::services::look::{self, LookState, Tone};
use crate::store::{FileId, KeyPath, Store};

/// Kip seçenekleri (`[colors] mode`). İlki varsayılan: "Otomatik" seçilince anahtar silinir.
/// Etiketler katalogdan geldiği için burada yalnızca anahtar ve ikon tutulur (etiket: [`mode_label`]).
const MODES: [(&str, IconName); 3] =
    [("auto", IconName::Brightness), ("dark", IconName::DarkMode), ("light", IconName::LightMode)];

/// Kipin kullanıcıya görünen etiketi (katalogdan).
fn mode_label(mode: &str) -> String {
    match mode {
        "dark" => t!("settings-look-mode-dark"),
        "light" => t!("settings-look-mode-light"),
        _ => t!("settings-look-mode-auto"),
    }
}

/// Seçim yapıldıktan sonra shell'in uygulaması için beklenen süre (durum yeniden sorulur).
pub(super) const APPLY_SETTLE: Duration = Duration::from_millis(700);

/// Sol başlık + açıklama (standart satırla aynı tipografi).
#[derive(Debug)]
pub(super) struct Head {
    pub(super) title: Label,
    pub(super) subtitle: Option<Label>,
}

impl Head {
    pub(super) fn new(theme: &Theme, desc: &SettingDesc) -> Self {
        Self {
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: (!desc.subtitle.is_empty()).then(|| Label::new(desc.description(), text_style(theme, TextRole::Caption))),
        }
    }

    pub(super) fn measure(&mut self, text: &mut TextSystem, theme: &Theme, max_w: f32) -> f32 {
        self.title.set_max_width(Some(max_w));
        let mut h = self.title.measure(text).height;
        if let Some(s) = &mut self.subtitle {
            s.set_max_width(Some(max_w));
            h += theme.space(1) + s.measure(text).height;
        }
        h
    }

    pub(super) fn set_theme(&mut self, theme: &Theme) {
        self.title.set_style(text_style(theme, TextRole::Body));
        if let Some(s) = &mut self.subtitle {
            s.set_style(text_style(theme, TextRole::Caption));
        }
    }

    /// `top` yüksekliğinden çizer; çizilen yükseklik.
    pub(super) fn draw(&mut self, ui: &mut Ui, x: f32, top: f32) -> f32 {
        let t = ui.theme;
        let c = &t.colors;
        let th = self.title.measure_in(ui.painter).height;
        self.title.draw(ui.painter, Point::new(x, top), ui.fade(c.on_surface));
        let mut h = th;
        if let Some(s) = &mut self.subtitle {
            let sh = s.measure_in(ui.painter).height;
            s.draw(ui.painter, Point::new(x, top + th + t.space(1)), ui.fade(c.on_surface_variant));
            h += t.space(1) + sh;
        }
        h
    }
}

fn wallpaper_source(store: &Store) -> bool {
    store.get(FileId::Framework, &KeyPath::parse("colors.source")).and_then(Value::as_str) == Some("wallpaper")
}

/// Dosyadaki kip seçimi (`[colors] mode`); anahtar yok ya da bilinmeyen değer → `auto`
/// (framework varsayılanı; geçersiz değeri framework de uyarıyla varsayılana çevirir).
pub fn current_mode(desc: &SettingDesc, store: &Store) -> &'static str {
    let v = store.get(desc.file, &desc.key()).and_then(Value::as_str).unwrap_or_default();
    MODES.iter().find(|(k, ..)| *k == v).map_or(MODES[0].0, |(k, ..)| k)
}

// ---- Koyu / açık ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct ModeChoice {
    desc: &'static SettingDesc,
    head: Head,
    seg: SegmentedButton,
    previous: PreviousAction,
    rect: Rect,
}

impl ModeChoice {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let segments = MODES.iter().map(|(k, i)| Segment::new(mode_label(k)).with_icon(Icon::named(*i))).collect();
        let i = MODES.iter().position(|(k, ..)| *k == current_mode(desc, store)).unwrap_or(0);
        Self { desc, head: Head::new(theme, desc), seg: SegmentedButton::new(theme, segments, i), previous: PreviousAction::new(theme, desc), rect: Rect::ZERO }
    }

    /// Seçimi yazar, tek geri al adımı.
    fn write(&self, env: &mut Env, i: usize) {
        let Some((mode, ..)) = MODES.get(i) else { return };
        let d = self.desc;
        env.push(Action::Group { label: d.label().to_owned(), writes: mode_writes(d, env.store, mode) });
    }
}

/// Kip seçiminin yazımları (saf; testli).
///
/// - `auto` (varsayılan): `[colors] mode` silinir. Duvar kağıdı paletine dokunulmaz — renkler
///   duvar kağıdından geliyorsa otomatik kip paletin kaydettiği kipi (`palette.toml [meta] mode`)
///   izler (framework çözüm sırası: palet kipi → tema yüzeyinin parlaklığı → koyu).
/// - `dark` / `light`: `[colors] mode` yazılır; renkler duvar kağıdından geliyorsa palet de aynı
///   kipte yeniden üretilsin diye `[meta] mode` aynı adımda yazılır (koyu yüzey açık kipe girmez).
pub fn mode_writes(desc: &SettingDesc, store: &Store, mode: &str) -> Vec<(FileId, KeyPath, Option<Value>)> {
    // Otomatik anahtarı siler; Koyu/Açık her zaman açıkça yazılır (varsayılan otomatiktir,
    // varsayılana eşitlik denetimi koyu seçimi sessizce otomatiğe çevirebilirdi).
    let value = (mode != MODES[0].0).then(|| Value::from(mode));
    let mut writes = vec![(desc.file, desc.key(), value)];
    if mode != MODES[0].0 && wallpaper_source(store) {
        let meta = KeyPath::parse("meta.mode");
        let default = crate::defaults::get(store, FileId::Palette, &meta).and_then(Value::as_str);
        writes.push((FileId::Palette, meta, (default != Some(mode)).then(|| Value::from(mode))));
    }
    writes
}

impl Block for ModeChoice {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store)
    }

    /// Eşanlamlılar (duvar kağıdı paletinin kipi) da bu satırdadır (§3.5).
    fn contains_id(&self, id: &str) -> bool {
        id == self.desc.id || super::links::is_alias_of(self.desc.id, id)
    }

    fn reveal(&mut self, _env: &mut Env, id: &str) -> Option<(f32, bool)> {
        self.contains_id(id).then_some((0.0, false))
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let ss = self.seg.preferred_size(env.text, t);
        let prev_w = if self.previous.visible() { self.previous.size(t).width + t.space(2) } else { 0.0 };
        if is_narrow(t, width) {
            // Dar kart: seçim başlığın altında (standart satırın yığılan düzeni).
            let head = self.head.measure(env.text, t, (width - px * 2.0 - prev_w).max(t.space(20)));
            return py * 2.0 + head + t.space(2) + ss.height;
        }
        let text_w = (width - px * 2.0 - ss.width - t.space(4) - prev_w).max(t.space(20));
        row_height_for(t, self.head.measure(env.text, t, text_w).max(ss.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let ss = self.seg.preferred_size(env.text, t);
        let ps = self.previous.size(t);
        if is_narrow(t, rect.width) {
            let w = rect.width - px * 2.0;
            self.seg.set_rect(t, Rect::new(rect.x + px, rect.bottom() - py - ss.height, w, ss.height));
            let th = self.head.title.measure(env.text).height;
            self.previous.set_rect(Rect::new(rect.right() - px - ps.width, rect.y + py + th * 0.5 - ps.height * 0.5, ps.width, ps.height));
            return;
        }
        self.seg.set_rect(t, Rect::new(rect.right() - px - ss.width, rect.center().y - ss.height * 0.5, ss.width, ss.height));
        let s = self.seg.rect();
        self.previous.set_rect(Rect::new(s.x - t.space(2) - ps.width, rect.center().y - ps.height * 0.5, ps.width, ps.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let (prev, fired) = self.previous.pointer(env, e);
        if fired {
            return true;
        }
        let r = self.seg.pointer(e, env.now);
        if let Some(i) = r.event {
            self.write(env, i);
        }
        r.repaint || prev
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.seg.key(e, env.now);
        if let Some(i) = r.event {
            self.write(env, i);
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.seg.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.seg.cursor(p).or_else(|| self.previous.cursor(p))
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        matches!(event, crate::services::Event::Snapshots) && self.previous.refresh()
    }

    fn sync(&mut self, env: &mut Env) {
        let i = MODES.iter().position(|(k, ..)| *k == current_mode(self.desc, env.store)).unwrap_or(0);
        self.seg.set_selected(i, env.now);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.head.set_theme(theme);
        self.seg.set_theme(theme, now);
        self.previous.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        let th = self.head.title.measure_in(ui.painter).height
            + self.head.subtitle.as_mut().map_or(0.0, |s| s.measure_in(ui.painter).height + t.space(1));
        let (_, py) = row_padding(t);
        let top = if is_narrow(t, self.rect.width) { self.rect.y + py } else { self.rect.center().y - th * 0.5 };
        self.head.draw(ui, self.rect.x + px, top);
        self.seg.draw(ui);
        self.previous.draw(ui);
    }
}

// ---- Durum ---------------------------------------------------------------------------------

#[derive(Debug)]
pub struct LookStatus {
    desc: &'static SettingDesc,
    /// Durum satırı: framework notu (uygulandı = başarı, eksik paket/hata = uyarı, diğerleri
    /// sakin bilgi).
    note: Notice,
    copy: Button,
    command: Option<String>,
    rect: Rect,
    /// Son görülen seçim (değişince durum yeniden sorulur).
    seen: Option<String>,
}

/// Durumu etkileyen seçimlerin özeti. `colors.contrast` da içinde: shell GTK/Qt renklerini
/// `Theme::contrast` ile yeniden yazar (framework `accessibility.md` §1), koyu/açık gibi.
fn selection_key(store: &Store) -> String {
    ["desktop.unified", "desktop.icon_theme", "desktop.cursor_theme", "desktop.cursor_size", "desktop.qt_backend", "colors.mode", "colors.source", "colors.contrast"]
        .iter()
        .map(|p| store.get(FileId::Framework, &KeyPath::parse(p)).map(|v| v.to_string()).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("|")
}

impl LookStatus {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        let mut s = Self {
            desc,
            note: Notice::new(theme, NoticeKind::Info, ""),
            copy: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Copy)).with_label(theme, t!("settings-look-copy-command")),
            command: None,
            rect: Rect::ZERO,
            seen: None,
        };
        s.update();
        s
    }

    fn update(&mut self) {
        let state = look::status();
        let line = look::status_line(&state);
        let (tone, text, command) = line.map_or((Tone::Info, String::new(), None), |l| (l.tone, l.text, l.command));
        self.apply(tone, &text, command);
        if matches!(state, LookState::Pending) && self.seen.is_none() {
            look::refresh(Duration::ZERO);
        }
    }

    /// Durum satırını nota yansıtır (saf; testler).
    fn apply(&mut self, tone: Tone, text: &str, command: Option<String>) {
        self.note.set_kind(notice_kind(tone));
        self.note.set_text(text);
        self.command = command;
    }

    /// Notun genişliği (sağda kopyala düğmesi varsa onun payı düşülür).
    fn note_width(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let cw = if self.command.is_some() { self.copy.preferred_size(text, theme).width + theme.space(3) } else { 0.0 };
        (width - px * 2.0 - cw).max(theme.space(24))
    }
}

/// Durum tonunun not düzeyi: uygulandı → başarı (`CheckCircle`, `success` rengi).
fn notice_kind(tone: Tone) -> NoticeKind {
    match tone {
        Tone::Ok => NoticeKind::Success,
        Tone::Info => NoticeKind::Info,
        Tone::Warning => NoticeKind::Warning,
    }
}

impl Block for LookStatus {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let w = self.note_width(env.text, t, width);
        let bh = if self.command.is_some() { self.copy.preferred_size(env.text, t).height } else { 0.0 };
        self.note.measure(env.text, t, w).height.max(bh) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let w = self.note_width(env.text, t, rect.width);
        let nh = self.note.measure(env.text, t, w).height;
        self.note.set_rect(Rect::new(rect.x + px, rect.center().y - nh * 0.5, w, nh));
        let bs: Size = self.copy.preferred_size(env.text, t);
        self.copy.set_rect(Rect::new(rect.right() - px - bs.width + t.space(2), rect.center().y - bs.height * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        if self.command.is_none() {
            return false;
        }
        let r = self.copy.pointer(e, env.now);
        if r.event.is_some()
            && let Some(cmd) = &self.command
        {
            env.push(Action::Copy(cmd.clone()));
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let Some(cmd) = self.command.clone() else { return false };
        let r = self.copy.key(e, env.now);
        if r.event.is_some() {
            env.push(Action::Copy(cmd));
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        self.command.is_some()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.copy.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.command.as_ref().and_then(|_| self.copy.cursor(p))
    }

    fn sync(&mut self, env: &mut Env) {
        let key = selection_key(env.store);
        if self.seen.as_ref() != Some(&key) {
            // İlk eşitleme yalnızca kaydeder; sonraki değişimler shell uygulayınca yeniden sorar.
            if self.seen.is_some() {
                look::refresh(APPLY_SETTLE);
            }
            self.seen = Some(key);
        }
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::Look) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.note.set_theme(theme, now);
        self.copy.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.note.draw(ui);
        if self.command.is_some() {
            self.copy.draw(ui);
        }
    }
}

/// Birleşik görünüm kapalıyken anahtarın altında, gizlenen satırların
/// ([`crate::registry::UNIFIED_GATED`]) yerine görünen açıklama.
pub fn unified_off_note() -> String {
    t!("settings-look-unified-off-note")
}

/// Açıklamanın yanındaki tek tık eylemi (Görünüm sayfası).
pub fn unified_on_action() -> String {
    t!("settings-look-unified-turn-on")
}

/// Birleşik görünüm kapalıyken anahtarın hemen altındaki satır: kısa açıklama ve isteğe bağlı
/// "Birleşik görünümü aç" düğmesi. Hoş geldin ve Görünüm aynı bloğu kullanır; görünürlük
/// [`crate::registry::unified_on`]'dan (gizlenen satırlarla tek kaynak).
#[derive(Debug)]
pub struct UnifiedOffNote {
    section: &'static str,
    text: Label,
    action: Option<Button>,
    rect: Rect,
}

impl UnifiedOffNote {
    /// `section`: içinde durduğu kart (anahtarınki); `action`: düğme gösterilsin mi.
    pub fn new(theme: &Theme, section: &'static str, action: bool) -> Self {
        Self {
            section,
            text: Label::new(unified_off_note(), text_style(theme, TextRole::Body)),
            action: action.then(|| Button::new(theme, ButtonVariant::Tonal).with_label(theme, unified_on_action())),
            rect: Rect::ZERO,
        }
    }

    fn action_size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        self.action.as_mut().map_or(Size::ZERO, |b| b.preferred_size(text, theme))
    }

    /// Düğmeye basıldı: anahtar satırının yazdığı değerle (`toggle_value(true)`), aynı geri al
    /// etiketiyle.
    fn turn_on(env: &mut Env) {
        if let Some(d) = crate::registry::by_id("desktop.unified") {
            env.set(d.label(), d.file, d.key(), d.toggle_value(true));
        }
    }
}

impl Block for UnifiedOffNote {
    fn section(&self) -> Option<&str> {
        Some(self.section)
    }

    fn visible(&self, store: &Store) -> bool {
        !crate::registry::unified_on(store)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let bs = self.action_size(env.text, t);
        let gap = if self.action.is_some() { t.space(3) } else { 0.0 };
        self.text.set_max_width(Some((width - px * 2.0 - bs.width - gap).max(t.space(20))));
        self.text.measure(env.text).height.max(bs.height) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let bs = self.action_size(env.text, t);
        if let Some(b) = &mut self.action {
            b.set_rect(Rect::new(rect.right() - px - bs.width, rect.center().y - bs.height * 0.5, bs.width, bs.height));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let Some(b) = &mut self.action else { return false };
        let r = b.pointer(e, env.now);
        if r.event.is_some() {
            Self::turn_on(env);
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let Some(b) = &mut self.action else { return false };
        let r = b.key(e, env.now);
        if r.event.is_some() {
            Self::turn_on(env);
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        self.action.is_some()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        if let Some(b) = &mut self.action {
            b.set_focus_visible(focused && visible);
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.action.as_ref().and_then(|b| b.cursor(p))
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_style(text_style(theme, TextRole::Body));
        if let Some(b) = &mut self.action {
            b.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let (px, _) = row_padding(ui.theme);
        let color = ui.fade(ui.theme.colors.on_surface_variant);
        let h = self.text.measure_in(ui.painter).height;
        self.text.draw(ui.painter, Point::new(self.rect.x + px, self.rect.center().y - h * 0.5), color);
        if let Some(b) = &mut self.action {
            b.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::by_id;

    fn store(framework: &str) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let fw = dir.path().join("framework.toml");
        std::fs::write(&fw, framework).unwrap();
        let s = Store::open(&fw, dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        (dir, s)
    }

    fn key(writes: &[(FileId, KeyPath, Option<Value>)], file: FileId, path: &str) -> Option<Option<String>> {
        writes
            .iter()
            .find(|(f, k, _)| *f == file && *k == KeyPath::parse(path))
            .map(|(.., v)| v.as_ref().and_then(Value::as_str).map(str::to_owned))
    }

    #[test]
    fn mode_defaults_to_auto_and_auto_removes_the_key() {
        let d = by_id("colors.mode").unwrap();
        let (_t, s) = store("");
        assert_eq!(current_mode(d, &s), "auto", "anahtar yok → otomatik");
        let (_t, s) = store("[colors]\nmode = \"light\"\n");
        assert_eq!(current_mode(d, &s), "light");
        let (_t, s) = store("[colors]\nmode = \"bozuk\"\n");
        assert_eq!(current_mode(d, &s), "auto");

        let (_t, s) = store("[colors]\nmode = \"dark\"\n");
        let w = mode_writes(d, &s, "auto");
        assert_eq!(w.len(), 1);
        assert_eq!(key(&w, FileId::Framework, "colors.mode"), Some(None), "otomatik anahtarı siler");
        // Koyu, varsayılan ne olursa olsun açıkça yazılır.
        let (_t, s) = store("");
        assert_eq!(key(&mode_writes(d, &s, "dark"), FileId::Framework, "colors.mode"), Some(Some("dark".into())));
    }

    #[test]
    fn wallpaper_palette_mode_follows_explicit_choice_only() {
        let d = by_id("colors.mode").unwrap();
        let (_t, s) = store("[colors]\nsource = \"wallpaper\"\nmode = \"dark\"\n");
        // Otomatik: palete dokunulmaz (otomatik kip paletin kipini izler).
        let w = mode_writes(d, &s, "auto");
        assert_eq!(key(&w, FileId::Palette, "meta.mode"), None);
        // Açık: tek geri al adımında paletin kipi de yazılır.
        let w = mode_writes(d, &s, "light");
        assert_eq!(key(&w, FileId::Framework, "colors.mode"), Some(Some("light".into())));
        assert_eq!(key(&w, FileId::Palette, "meta.mode"), Some(Some("light".into())));
        // Sabit renklerde palet yazılmaz.
        let (_t, s) = store("[colors]\nmode = \"dark\"\n");
        assert_eq!(key(&mode_writes(d, &s, "light"), FileId::Palette, "meta.mode"), None);
    }

    #[test]
    fn palette_mode_lives_in_the_dark_mode_row() {
        // Karar 5 / §3.5: paletin kipi ayrı satır değil; koyu/açık satırı onu da taşır, arama oraya gider.
        let theme = Theme::default();
        let (_t, s) = store("[colors]\nsource = \"wallpaper\"\n");
        let row = ModeChoice::new(&theme, by_id("colors.mode").unwrap(), &s);
        assert!(row.contains_id("colors.mode") && row.contains_id("palette.mode"));
        let blocks = crate::pages::build(&theme, crate::registry::PageId::Appearance, &s);
        assert_eq!(blocks.iter().filter(|b| b.contains_id("palette.mode")).count(), 1, "tek satır");
        let hits = crate::registry::search(&s, "renk şemasının kipi");
        assert!(hits.iter().any(|d| d.id == "palette.mode" && d.page == crate::registry::PageId::Appearance));
    }

    /// §9 Görünüm: Renkler kartı koyu/açık ile başlar; on renk rolü tek açılır satırda; arama
    /// her rolü bulur ve açılır satırı açar.
    #[test]
    fn appearance_colors_card_and_role_search() {
        use crate::page::Page;
        use crate::registry::{PageId, search};
        use nuxut::TextSystem;
        const DETAIL: &str = "Renkleri tek tek ayarla";
        let theme = Theme::default();
        let (_t, s) = store("");
        let blocks = crate::pages::build(&theme, PageId::Appearance, &s);
        let colors: Vec<_> = blocks.iter().filter(|b| b.section() == Some("Renkler")).collect();
        assert_eq!(colors[0].id(), Some("colors.mode"), "kartın ilk satırı koyu/açık");
        assert_eq!(colors.iter().filter(|b| b.disclosure().is_some()).count(), 1);
        assert_eq!(colors.last().and_then(|b| b.disclosure()), Some((DETAIL, false)), "kartın sonunda, kapalı");
        let roles = ["colors.on_primary", "colors.surface", "colors.surface_container", "colors.surface_bright", "colors.on_surface", "colors.on_surface_variant", "colors.error", "colors.on_error", "colors.outline", "colors.shadow"];
        for id in roles {
            assert_eq!(by_id(id).unwrap().detail(), Some(DETAIL), "{id}");
        }
        assert_eq!(by_id("colors.primary").unwrap().detail(), None, "vurgu hep açık");
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        for id in roles {
            let hit = search(&s, by_id(id).unwrap().title).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id} aramada yok"));
            assert_eq!(hit.page, PageId::Appearance);
            let mut page = Page::new(&theme, PageId::Appearance, crate::pages::build(&theme, PageId::Appearance, &s), &s);
            page.set_viewport(Rect::new(0.0, 0.0, 1000.0, 700.0));
            let mut env = Env { theme: &theme, store: &s, text: &mut text, now: Instant::now(), actions: &mut actions };
            page.reveal(&mut env, id);
            assert!(page.ui().is_open(DETAIL), "{id}: arama açılır satırı açtı");
        }
    }

    /// Görünüm › Uygulamalar: birleşik görünüm kapalıyken ikon/imleç ızgaraları, imleç boyutu,
    /// Qt ve durum satırı yok; anahtarın hemen altında açıklama + "aç" düğmesi. Açıkken
    /// (varsayılan dahil) sayfa eskisi gibi.
    #[test]
    fn apps_card_hides_gated_rows_when_unified_is_off() {
        let theme = Theme::default();
        let visible = |s: &Store| -> Vec<Option<&'static str>> {
            crate::pages::build(&theme, crate::registry::PageId::Appearance, s)
                .iter()
                .filter(|b| b.section() == Some("Uygulamalar") && b.visible(s))
                .map(|b| b.id())
                .collect()
        };
        let (_t, off) = store("[desktop]\nunified = false\n");
        assert_eq!(visible(&off), vec![Some("desktop.unified"), None], "kapalı: anahtar + açıklama");
        for on in ["", "[desktop]\nunified = true\n"] {
            let (_t, s) = store(on);
            let ids = visible(&s);
            assert!(!ids.contains(&None), "açıkken açıklama yok ({on:?})");
            for id in crate::registry::UNIFIED_GATED {
                assert!(ids.contains(&Some(id)), "açıkken {id} görünür ({on:?})");
            }
        }
    }

    /// Düğmeye tık: anahtar satırının yazdığı değer (`true`), aynı geri al etiketiyle.
    #[test]
    fn unified_off_action_turns_unified_on() {
        use nuxut::ui::input::{Button as MouseButton, PointerKind};
        let theme = Theme::default();
        let (_t, s) = store("[desktop]\nunified = false\n");
        let mut note = UnifiedOffNote::new(&theme, "Uygulamalar", true);
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &s, text: &mut text, now: Instant::now(), actions: &mut actions };
        let h = note.height(&mut env, 600.0);
        note.place(&mut env, Rect::new(0.0, 0.0, 600.0, h));
        assert!(note.focusable());
        let at = note.action.as_ref().unwrap().rect().center();
        for kind in [PointerKind::Enter, PointerKind::Motion, PointerKind::Press(MouseButton::Left), PointerKind::Release(MouseButton::Left)] {
            note.pointer(&mut env, &PointerEvent::new(at, kind));
        }
        let d = by_id("desktop.unified").unwrap();
        match actions.as_slice() {
            [Action::Set { label, file, path, value }] => {
                assert_eq!((label.as_str(), *file, path), (d.title, FileId::Framework, &KeyPath::parse("desktop.unified")));
                assert_eq!(value.as_ref().and_then(Value::as_bool), Some(true));
            }
            other => panic!("tek yazım beklenirdi: {other:?}"),
        }
        // Hoş geldin: düğmesiz (anahtar hemen üstünde), odaklanmaz.
        assert!(!UnifiedOffNote::new(&theme, "", false).focusable());
    }

    /// Aramada gizli satırlar birleşik görünüm anahtarına gider (tek sonuç); açıkken kendileri.
    #[test]
    fn search_for_gated_rows_leads_to_unified_toggle() {
        use crate::registry::search;
        let ids = |s: &Store, q: &str| search(s, q).iter().map(|d| d.id).collect::<Vec<_>>();
        let (_t, off) = store("[desktop]\nunified = false\n");
        for q in ["imleç", "papirus", "qt6ct", "eksik paket"] {
            let found = ids(&off, q);
            assert!(found.contains(&"desktop.unified"), "{q}: {found:?}");
            assert!(!found.iter().any(|id| crate::registry::UNIFIED_GATED.contains(id)), "{q}: {found:?}");
            assert_eq!(found.iter().filter(|id| **id == "desktop.unified").count(), 1, "{q}: tekrarsız");
        }
        let (_t, on) = store("[desktop]\nunified = true\n");
        let found = ids(&on, "papirus");
        assert!(found.contains(&"desktop.icon_theme") && !found.contains(&"desktop.unified"), "{found:?}");
    }

    #[test]
    fn qt_backend_row_writes_schema_values() {
        let d = by_id("desktop.qt_backend").unwrap();
        let (_t, s) = store("[desktop]\nunified = true\n");
        assert!(d.is_visible(&s));
        assert!(d.choice_value(0, &s).is_none(), "otomatik (varsayılan) anahtarı siler");
        assert_eq!(d.choice_value(1, &s).as_ref().and_then(Value::as_str), Some("kde"));
        assert_eq!(d.choice_value(2, &s).as_ref().and_then(Value::as_str), Some("qt6ct"));
        let (_t, s) = store("[desktop]\nunified = false\n");
        assert!(!d.is_visible(&s), "birleşik görünüm kapalıyken gizli");
    }
}

#[cfg(test)]
mod schema_tests {
    use nuxut::theme::{ChoiceValue, ColorMode, QtBackend};

    #[test]
    fn choices_match_framework_names() {
        let modes: Vec<&str> = super::MODES.iter().map(|(k, ..)| *k).collect();
        assert_eq!(modes, ColorMode::ALL.map(ColorMode::name), "Otomatik/Koyu/Açık = framework auto/dark/light");
        assert_eq!(ColorMode::default().name(), super::MODES[0].0, "ilk seçenek framework varsayılanı");
        let qt: Vec<&str> = crate::registry::QT_BACKENDS.iter().map(|(k, _)| *k).collect();
        assert_eq!(qt, QtBackend::ALL.map(|b| b.name()));
    }

    /// Durum satırı framework notuyla çizilir: uygulandı → başarı, eksik/hata → uyarı, diğerleri
    /// sakin bilgi; metin aynen nota geçer.
    #[test]
    fn status_tone_maps_to_notice_kind() {
        use super::*;
        use crate::registry::by_id;
        let theme = Theme::default();
        let d = by_id("desktop.status").unwrap();
        let mut s = LookStatus::new(&theme, d);
        s.apply(Tone::Ok, "Uygulandı", None);
        assert_eq!((s.note.kind(), s.note.text()), (NoticeKind::Success, "Uygulandı"));
        assert_eq!(s.note.kind().icon(), IconName::CheckCircle);
        s.apply(Tone::Warning, "Eksik paket", Some("paru -S x".into()));
        assert_eq!((s.note.kind(), s.command.as_deref()), (NoticeKind::Warning, Some("paru -S x")));
        s.apply(Tone::Info, "Uygulanıyor…", None);
        assert_eq!(s.note.kind(), NoticeKind::Info);
        let mut text = TextSystem::new();
        assert!(s.note.measure(&mut text, &theme, 600.0).height > 0.0, "not görünür yükseklikte");
    }
}
