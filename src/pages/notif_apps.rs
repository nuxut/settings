//! Uygulama başına bildirim ayarı: `[notifications.apps."<app_name>"] popups, history`
//! (shell şeması; anahtar D-Bus `app_name`'i birebir, büyük/küçük harf duyarlı).
//!
//! Liste config'te yazılı uygulamalardır; "Uygulama ekle" alanı yazdıkça kurulu
//! uygulamaların (.desktop `Name=`) adlarını önerir. Eklenen uygulamanın açılır bildirimleri
//! kapatılır (kullanıcı bunun için ekliyor); geçmiş açık kalır.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, IconButton, IconButtonVariant, Label, TextInput, TextInputEvent, TextRole, Toggle, Ui, text_style};
use nuxut::{Point, Rect, Theme};
use toml_edit::Value;

use super::fiyu::{FiyuEmpty, Spot};
use crate::block::{Action, Block, Env};
use crate::registry::{SettingDesc, fold};
use crate::rows::row_padding;
use crate::store::{FileId, KeyPath, Store};

/// En fazla gösterilen öneri.
const MAX_SUGGESTIONS: usize = 6;
/// Taranan en fazla .desktop dosyası (dev dizinlerde sınır).
const MAX_DESKTOP_FILES: usize = 4000;

fn app_key(name: &str, field: &str) -> KeyPath {
    KeyPath::new(["notifications", "apps", name, field])
}

/// Config'te yazılı uygulamalar (sıralı).
fn configured_apps(store: &Store) -> Vec<String> {
    let path = KeyPath::parse("notifications.apps");
    let file = store.file(FileId::Shell);
    let Some(item) = file.get(&path) else { return Vec::new() };
    let Some(t) = item.as_table_like() else { return Vec::new() };
    let mut names: Vec<String> = t.iter().map(|(k, _)| k.to_owned()).collect();
    names.sort_by_key(|n| fold(n));
    names
}

/// Kurulu uygulamaların görünen adları (`Name=`, `NoDisplay=true` hariç), tekrarsız.
fn installed_app_names() -> Vec<String> {
    let mut dirs: Vec<std::path::PathBuf> = Vec::new();
    if let Some(d) = std::env::var_os("XDG_DATA_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/share"))) {
        dirs.push(d.join("applications"));
    }
    let sys = std::env::var("XDG_DATA_DIRS").unwrap_or_else(|_| "/usr/local/share:/usr/share".into());
    dirs.extend(sys.split(':').filter(|s| !s.is_empty()).map(|s| std::path::PathBuf::from(s).join("applications")));
    let mut names = Vec::new();
    let mut seen = 0usize;
    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            seen += 1;
            if seen > MAX_DESKTOP_FILES {
                break;
            }
            let p = e.path();
            if p.extension().is_none_or(|x| x != "desktop") {
                continue;
            }
            let Ok(meta) = e.metadata() else { continue };
            if meta.len() > 256 * 1024 {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&p) else { continue };
            if let Some(name) = parse_desktop_name(&text) {
                names.push(name);
            }
        }
    }
    names.sort_by_key(|n| fold(n));
    names.dedup();
    names
}

/// `[Desktop Entry]` bölümündeki yerelleştirilmemiş `Name=`; gizli girişler `None`.
pub fn parse_desktop_name(text: &str) -> Option<String> {
    let mut in_entry = false;
    let mut name = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if in_entry {
                break;
            }
            in_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_entry {
            continue;
        }
        if let Some(v) = line.strip_prefix("Name=") {
            name = Some(v.trim().chars().take(128).collect::<String>());
        } else if line == "NoDisplay=true" || line == "Hidden=true" {
            return None;
        }
    }
    name.filter(|n| !n.is_empty())
}

#[derive(Debug)]
struct AppRow {
    name: String,
    label: Label,
    popups: Toggle,
    history: Toggle,
    remove: IconButton,
}

#[derive(Debug)]
pub struct NotificationApps {
    desc: &'static SettingDesc,
    rect: Rect,
    rows: Vec<AppRow>,
    col_popups: Label,
    col_history: Label,
    /// Liste boş: maskotlu boş durum ([`Spot::NotifAppsEmpty`]).
    empty: FiyuEmpty,
    input: TextInput,
    add: Button,
    installed: Option<Vec<String>>,
    suggestions: Vec<(String, Button)>,
    focused: bool,
}

impl NotificationApps {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut s = Self {
            desc,
            rect: Rect::ZERO,
            rows: Vec::new(),
            col_popups: Label::new(t!("settings-notif-apps-col-popups"), text_style(theme, TextRole::Caption)),
            col_history: Label::new(t!("settings-section-history"), text_style(theme, TextRole::Caption)),
            empty: FiyuEmpty::new(theme, Spot::NotifAppsEmpty, &t!("settings-notif-apps-empty-title"), Some(&t!("settings-notif-apps-empty-body"))),
            input: TextInput::new(theme).with_placeholder(t!("settings-notif-apps-name-placeholder")).with_max_bytes(128),
            add: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::Add)).with_label(theme, t!("settings-sections-add")),
            installed: None,
            suggestions: Vec::new(),
            focused: false,
        };
        s.rebuild(theme, store);
        s
    }

    fn rebuild(&mut self, theme: &Theme, store: &Store) {
        let now = Instant::now();
        let names = configured_apps(store);
        let get = |n: &str, f: &str| store.get(FileId::Shell, &app_key(n, f)).and_then(Value::as_bool).unwrap_or(true);
        if names.iter().eq(self.rows.iter().map(|r| &r.name)) {
            for r in &mut self.rows {
                r.popups.set_on(get(&r.name, "popups"), now);
                r.history.set_on(get(&r.name, "history"), now);
            }
            return;
        }
        self.rows = names
            .into_iter()
            .map(|name| AppRow {
                label: Label::new(name.clone(), text_style(theme, TextRole::Body)),
                popups: Toggle::new(theme, get(&name, "popups")),
                history: Toggle::new(theme, get(&name, "history")),
                remove: IconButton::new(theme, Icon::named(IconName::Close), IconButtonVariant::Standard),
                name,
            })
            .collect();
    }

    fn refresh_suggestions(&mut self, theme: &Theme) {
        let q = fold(self.input.text().trim());
        self.suggestions.clear();
        if q.is_empty() {
            return;
        }
        let installed = self.installed.get_or_insert_with(installed_app_names);
        let taken: Vec<&str> = self.rows.iter().map(|r| r.name.as_str()).collect();
        self.suggestions = installed
            .iter()
            .filter(|n| fold(n).contains(&q) && !taken.contains(&n.as_str()))
            .take(MAX_SUGGESTIONS)
            .map(|n| (n.clone(), Button::new(theme, ButtonVariant::Outlined).with_label(theme, n.as_str())))
            .collect();
    }

    fn add_app(&mut self, env: &mut Env, name: &str) {
        let name = name.trim();
        if name.is_empty() || name.len() > 128 || self.rows.iter().any(|r| r.name == name) {
            return;
        }
        env.push(Action::Set { label: t!("settings-notif-apps-notifications", name = name), file: FileId::Shell, path: app_key(name, "popups"), value: Some(Value::from(false)) });
        self.input.set_text("");
        self.suggestions.clear();
    }

    fn row_h(theme: &Theme) -> f32 {
        theme.space(12)
    }
}

impl Block for NotificationApps {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.rows.is_empty().then_some(Spot::NotifAppsEmpty)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let list = if self.rows.is_empty() {
            self.empty.height(env.text, t, width - px * 2.0)
        } else {
            t.space(6) + Self::row_h(t) * self.rows.len() as f32
        };
        let sugg = if self.suggestions.is_empty() { 0.0 } else { t.space(12) };
        py + list + t.space(3) + t.space(12) + sugg + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let right = rect.right() - px;
        let ts = self.rows.first().map(|r| r.popups.preferred_size(t)).unwrap_or_default();
        let col_w = t.space(18);
        let mut y = rect.y + py;
        if self.rows.is_empty() {
            let eh = self.empty.height(env.text, t, rect.width - px * 2.0);
            self.empty.set_rect(Rect::new(rect.x + px, y, rect.width - px * 2.0, eh));
            y += eh;
        } else {
            y += t.space(6);
            let rb = t.space(8);
            for r in &mut self.rows {
                let mid = y + Self::row_h(t) * 0.5;
                r.remove.set_rect(Rect::new(right - rb, mid - rb * 0.5, rb, rb));
                let hx = right - rb - t.space(3) - col_w;
                let px_ = hx - col_w;
                r.history.set_rect(Rect::new(hx + (col_w - ts.width) * 0.5, mid - ts.height * 0.5, ts.width, ts.height));
                r.popups.set_rect(Rect::new(px_ + (col_w - ts.width) * 0.5, mid - ts.height * 0.5, ts.width, ts.height));
                r.label.set_max_width(Some((px_ - rect.x - px * 2.0).max(t.space(20))));
                y += Self::row_h(t);
            }
            y += t.space(3);
        }
        let bs = self.add.preferred_size(env.text, t);
        let iw = (rect.width - px * 2.0 - bs.width - t.space(3)).min(t.space(90));
        let ih = self.input.preferred_size(t, iw).height;
        self.input.set_rect(Rect::new(rect.x + px, y, iw, ih));
        self.add.set_rect(Rect::new(rect.x + px + iw + t.space(3), y + (ih - bs.height) * 0.5, bs.width, bs.height));
        let mut x = rect.x + px;
        let sy = y + ih + t.space(2);
        for (_, b) in &mut self.suggestions {
            let s = b.preferred_size(env.text, t);
            if x + s.width > right {
                break;
            }
            b.set_rect(Rect::new(x, sy, s.width, s.height));
            x += s.width + t.space(2);
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let t = env.theme;
        let mut repaint = false;
        let mut action = None;
        for r in &mut self.rows {
            let rp = r.popups.pointer(e, now);
            repaint |= rp.repaint;
            if let Some(on) = rp.event {
                action = Some(Action::Set { label: t!("settings-notif-apps-popups-toggled", name = r.name.as_str()), file: FileId::Shell, path: app_key(&r.name, "popups"), value: Some(Value::from(on)) });
            }
            let rh = r.history.pointer(e, now);
            repaint |= rh.repaint;
            if let Some(on) = rh.event {
                action = Some(Action::Set { label: t!("settings-notif-apps-history-toggled", name = r.name.as_str()), file: FileId::Shell, path: app_key(&r.name, "history"), value: Some(Value::from(on)) });
            }
            let rr = r.remove.pointer(e, now);
            repaint |= rr.repaint;
            if rr.event.is_some() {
                action = Some(Action::Group {
                    label: t!("settings-notif-apps-notifications", name = r.name.as_str()),
                    writes: vec![(FileId::Shell, app_key(&r.name, "popups"), None), (FileId::Shell, app_key(&r.name, "history"), None)],
                });
            }
        }
        if let Some(a) = action {
            env.push(a);
            return true;
        }
        let ri = self.input.pointer(env.text, t, e, now);
        repaint |= ri.repaint;
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && self.input.rect().contains(e.position)
        {
            env.push(Action::Focus);
        }
        let ra = self.add.pointer(e, now);
        repaint |= ra.repaint;
        if ra.event.is_some() {
            let name = self.input.text().to_owned();
            self.add_app(env, &name);
            return true;
        }
        let mut picked = None;
        for (n, b) in &mut self.suggestions {
            let r = b.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                picked = Some(n.clone());
            }
        }
        if let Some(n) = picked {
            self.add_app(env, &n);
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }
        if e.state == KeyState::Pressed && e.key == Key::Escape && !self.input.text().is_empty() {
            self.input.set_text("");
            self.suggestions.clear();
            return true;
        }
        let r = self.input.key(e, env.now);
        match r.event {
            Some(TextInputEvent::Submit) => {
                let name = self.input.text().to_owned();
                self.add_app(env, &name);
                true
            }
            Some(TextInputEvent::Changed) => {
                self.refresh_suggestions(env.theme);
                true
            }
            _ => r.repaint,
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        self.focused = focused;
        self.input.set_focused(focused, now);
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        Some(&mut self.input)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rows
            .iter()
            .find_map(|r| r.popups.cursor(p).or_else(|| r.history.cursor(p)).or_else(|| r.remove.cursor(p)))
            .or_else(|| self.input.cursor(p))
            .or_else(|| self.add.cursor(p))
            .or_else(|| self.suggestions.iter().find_map(|(_, b)| b.cursor(p)))
    }

    fn sync(&mut self, env: &mut Env) {
        self.rebuild(env.theme, env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for r in &mut self.rows {
            r.label.set_style(text_style(theme, TextRole::Body));
            r.popups.set_theme(theme, now);
            r.history.set_theme(theme, now);
            r.remove.set_theme(theme, now);
        }
        self.col_popups.set_style(text_style(theme, TextRole::Caption));
        self.col_history.set_style(text_style(theme, TextRole::Caption));
        self.empty.set_theme(theme, now);
        self.input.set_theme(theme, now);
        self.add.set_theme(theme, now);
        for (_, b) in &mut self.suggestions {
            b.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let r = self.rect;
        if self.rows.is_empty() {
            self.empty.draw(ui);
        } else {
            let first = &self.rows[0];
            let (pr, hr) = (first.popups.rect(), first.history.rect());
            let ph = self.col_popups.measure_in(ui.painter);
            self.col_popups.draw(ui.painter, Point::new(pr.center().x - ph.width * 0.5, r.y + py), ui.fade(c.on_surface_variant));
            let hh = self.col_history.measure_in(ui.painter);
            self.col_history.draw(ui.painter, Point::new(hr.center().x - hh.width * 0.5, r.y + py), ui.fade(c.on_surface_variant));
            for row in &mut self.rows {
                let mid = row.popups.rect().center().y;
                let ls = row.label.measure_in(ui.painter);
                row.label.draw(ui.painter, Point::new(r.x + px, mid - ls.height * 0.5), ui.fade(c.on_surface));
                row.popups.draw(ui);
                row.history.draw(ui);
                row.remove.draw(ui);
            }
        }
        self.input.draw(ui);
        self.add.draw(ui);
        for (_, b) in &mut self.suggestions {
            b.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_name_parsing() {
        let t = "[Desktop Entry]\nName=Firefox\nName[tr]=Firefox Tarayıcı\nExec=firefox\n[Desktop Action new]\nName=Yeni pencere\n";
        assert_eq!(parse_desktop_name(t).as_deref(), Some("Firefox"));
        assert_eq!(parse_desktop_name("[Desktop Entry]\nName=Gizli\nNoDisplay=true\n"), None);
        assert_eq!(parse_desktop_name("[Other]\nName=x\n"), None);
    }

    /// Maskot yerleşimi: liste boşken boş maskotu, uygulama varken yok.
    #[test]
    fn empty_list_shows_empty_mascot() {
        let dir = tempfile::tempdir().unwrap();
        let shell = dir.path().join("s.toml");
        let theme = Theme::default();
        let desc = crate::registry::by_id("notifications.apps").unwrap();
        let store = Store::open(dir.path().join("f.toml"), &shell, dir.path().join("p.toml"));
        let b = NotificationApps::new(&theme, desc, &store);
        assert_eq!(b.mascot(), Some(Spot::NotifAppsEmpty));
        assert_eq!(Spot::NotifAppsEmpty.mood(), nuxut::ui::Mood::Empty);
        std::fs::write(&shell, "[notifications.apps.\"Firefox\"]\npopups = false\n").unwrap();
        let store = Store::open(dir.path().join("f.toml"), &shell, dir.path().join("p.toml"));
        assert_eq!(NotificationApps::new(&theme, desc, &store).mascot(), None);
    }
}
