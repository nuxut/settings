//! Varsayılan uygulamalar sayfası (Sistem grubu; sözleşme defaults-v1, kararlar
//! `docs/design/default-apps.md`).
//!
//! - [`DefaultAppsNote`]: sayfanın üstündeki sakin not (kartsız) — kabuk çalışmıyor / bu sürüm
//!   varsayılan uygulamaları bildirmiyor. Satırlar o sırada gizlidir (gösterecek veri yok).
//! - [`DefaultAppRow`]: rol başına satır. Solda başlık + açıklama, sağda ikonlu açılır seçici
//!   (adaylar uygulama ikonlarıyla; `NoDisplay` olanlar "Diğer" başlığı altında). Altında rol notları: karışık seçim,
//!   masaüstüne özgü dosyanın gölgelemesi, terminal aracı yok (seçici kapalı). "Önceki hâline
//!   dön" shell snapshot'ıdır ([`PreviousAction`], `SettingId` `defaults.<rol>`).
//!
//! Seçim → `msg defaults set` (arka planda) → durum yeniden okunur; hata şeritte gösterilir ve
//! satır shell'in gerçek durumuna döner. Ayarlar `mimeapps.list`'e hiç dokunmaz.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::render::{ImageData, Texture, TextureOptions};
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, Modifiers, PointerEvent};
use nuxut::ui::{Icon, IconName, IconView, Label, Select, SelectEntry, SelectEvent, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use super::fiyu::{FiyuEmpty, Spot};
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{PreviousAction, is_narrow, row_height_for, row_padding, side_text_width};
use crate::services::default_apps::{self, DefaultsState, Role, RoleStatus};
use crate::services::{Event, Request, app_icons};
use crate::store::Store;

// ---- Saf satır modeli -----------------------------------------------------------------------

/// Notun tonu: bilgi (ikincil renk) ya da uyarı (hata rengi).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Info,
    Warning,
}

/// Bir rol satırının gösterilecek hâli (UI'dan bağımsız; testli).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowModel {
    /// Seçicinin seçenekleri (başlık hariç): etiket, desktop-id (`None`: "seçilmemiş" yer
    /// tutucusu) ve tema ikon adı (`None`: ikon yok → seçicinin yer tutucusu). Üçü aynı uzunlukta;
    /// indeksler `Select`'in seçenek indeksleriyle aynı.
    pub labels: Vec<String>,
    pub ids: Vec<Option<String>>,
    pub icons: Vec<Option<String>>,
    /// İlk `NoDisplay` adayın seçenek indeksi: ondan önce "Diğer" başlığı. Görünür seçenek yoksa
    /// (başlığın ayıracağı bir şey yok) `None`.
    pub other_at: Option<usize>,
    pub selected: usize,
    /// Seçici açılabilir mi (terminal aracı yoksa kapalı).
    pub enabled: bool,
    pub notes: Vec<(String, Tone)>,
}

impl RowModel {
    pub fn from_status(rs: &RoleStatus) -> Self {
        let mut labels = Vec::new();
        let mut ids = Vec::new();
        let mut icons = Vec::new();
        let current = rs.current.as_ref();
        match current {
            None => {
                labels.push(t!("settings-defaults-none"));
                ids.push(None);
                icons.push(None);
            }
            // Kurulu değil (ya da artık bu rolün adayı değil): yine seçili gösterilir.
            Some(c) if !rs.current_installed() => {
                labels.push(t!("settings-defaults-not-installed", name = c.name.as_str()));
                ids.push(Some(c.id.clone()));
                icons.push(default_apps::icon_name(c).map(str::to_owned));
            }
            Some(_) => {}
        }
        let ordered = rs.ordered();
        let mut other_at = None;
        for a in &ordered {
            if a.hidden && other_at.is_none() && !labels.is_empty() {
                other_at = Some(labels.len());
            }
            // Aynı adlı iki uygulama: kimlikle ayrılır.
            let twin = ordered.iter().filter(|b| b.name == a.name).count() > 1;
            labels.push(if twin { t!("settings-defaults-app-with-id", name = a.name.as_str(), id = a.id.as_str()) } else { a.name.clone() });
            ids.push(Some(a.id.clone()));
            // Yalnızca düz tema adı çözülür (mutlak yol reddedilir).
            icons.push(default_apps::icon_name(a).map(str::to_owned));
        }
        let selected = current.and_then(|c| ids.iter().position(|i| i.as_deref() == Some(c.id.as_str()))).unwrap_or(0);
        let mut notes = Vec::new();
        if !rs.available {
            notes.push((t!("settings-defaults-note-terminal-unavailable"), Tone::Info));
        }
        if rs.mixed {
            notes.push((t!("settings-defaults-note-mixed"), Tone::Info));
        }
        if let Some(path) = &rs.shadowed_by {
            notes.push((t!("settings-defaults-note-shadowed", file = path.as_str()), Tone::Warning));
        }
        // Seçecek başka bir şey yoksa da açılır (tek seçenek: bilgi); kapalı yalnızca yazılamayan rol.
        Self { labels, ids, icons, other_at, selected, enabled: rs.available, notes }
    }

    /// Seçicinin girdileri: seçenekler, `other_at`'ten önce "Diğer" başlığı. İkonlar sonradan
    /// ([`DefaultAppRow::load_icons`]); yuva baştan ayrılır.
    pub fn entries(&self) -> Vec<SelectEntry> {
        let mut out = Vec::with_capacity(self.labels.len() + 1);
        for (i, l) in self.labels.iter().enumerate() {
            if self.other_at == Some(i) {
                out.push(SelectEntry::header(t!("settings-defaults-other")));
            }
            out.push(SelectEntry::option(l.as_str()));
        }
        out
    }

    /// Seçicinin yeniden kurulması gerekir mi (seçenekler, ikon adları ya da başlık değişti).
    fn same_options(&self, other: &Self) -> bool {
        self.labels == other.labels && self.icons == other.icons && self.other_at == other.other_at
    }

    /// `i`. seçenek yeni bir seçim mi (yer tutucu ve mevcut seçim değildir).
    pub fn pick(&self, i: usize, current: Option<&str>) -> Option<&str> {
        let id = self.ids.get(i)?.as_deref()?;
        (Some(id) != current).then_some(id)
    }
}

// ---- Üst not --------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DefaultAppsNote {
    desc: &'static SettingDesc,
    icon: IconView,
    text: Label,
    /// Kabuk çalışmıyor / okunamadı: notun yerine maskotlu hata durumu ([`unavailable_text`]).
    fiyu: Option<FiyuEmpty>,
    rect: Rect,
}

/// Kabuk çalışmıyor ya da okuma başarısız: sayfanın satırları gizli, tek içerik bu olduğundan
/// maskotlu hata durumu ([`Spot::DefaultAppsUnavailable`]); başlık + açıklama. Yükleniyor ve
/// "sürüm desteklemiyor" sakin not olarak kalır (`None`).
pub fn unavailable_text(state: &DefaultsState) -> Option<(String, String)> {
    match state {
        DefaultsState::NotRunning | DefaultsState::TestInstance => Some((t!("settings-defaults-empty-not-running-title"), t!("settings-defaults-empty-not-running-body"))),
        DefaultsState::Failed(_) => Some((t!("settings-defaults-empty-failed-title"), t!("settings-defaults-empty-failed-body"))),
        DefaultsState::Pending | DefaultsState::Ready(_) | DefaultsState::Unsupported => None,
    }
}

impl DefaultAppsNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        default_apps::ensure();
        let mut n = Self { desc, icon: IconView::new(theme, Icon::named(IconName::Info)), text: Label::new("", text_style(theme, TextRole::Body)), fiyu: None, rect: Rect::ZERO };
        n.update(theme);
        n
    }

    fn update(&mut self, theme: &Theme) {
        self.apply(theme, &default_apps::status());
    }

    fn apply(&mut self, theme: &Theme, state: &DefaultsState) {
        self.text.set_text(&default_apps::status_note(state).unwrap_or_default());
        match (unavailable_text(state), &mut self.fiyu) {
            (Some((title, body)), Some(f)) => f.set_text(theme, &title, Some(&body)),
            (Some((title, body)), None) => self.fiyu = Some(FiyuEmpty::new(theme, Spot::DefaultAppsUnavailable, &title, Some(&body))),
            (None, _) => self.fiyu = None,
        }
    }
}

impl Block for DefaultAppsNote {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        default_apps::status_note(&default_apps::status()).is_some()
    }

    fn mascot(&self) -> Option<Spot> {
        self.fiyu.as_ref().map(FiyuEmpty::spot)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        if let Some(f) = &mut self.fiyu {
            return f.height(env.text, t, width);
        }
        let (px, py) = row_padding(t);
        let is = self.icon.measure(env.text);
        self.text.set_max_width(Some((width - px * 2.0 - is.width - t.space(3)).max(t.space(20))));
        self.text.measure(env.text).height.max(is.height) + py * 2.0
    }

    fn place(&mut self, _env: &mut Env, rect: Rect) {
        self.rect = rect;
        if let Some(f) = &mut self.fiyu {
            f.set_rect(rect);
        }
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &Event) -> bool {
        match event {
            Event::DefaultApps => {
                self.update(env.theme);
                true
            }
            // İkon teması değişmiş olabilir: uygulama ikonları yeni temadan (sayfada tek not var,
            // önbellek bir kez boşalır).
            Event::Look => {
                app_icons::invalidate();
                false
            }
            _ => false,
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.text.set_style(text_style(theme, TextRole::Body));
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        if let Some(f) = &mut self.fiyu {
            f.draw(ui);
            return;
        }
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let cy = self.rect.center().y;
        let is = self.icon.measure(ui.painter.text_system());
        self.icon.draw_centered(ui.painter, Point::new(self.rect.x + px + is.width * 0.5, cy), ui.fade(c.on_surface_variant));
        let h = self.text.measure_in(ui.painter).height;
        self.text.draw(ui.painter, Point::new(self.rect.x + px + is.width + t.space(3), cy - h * 0.5), ui.fade(c.on_surface_variant));
    }
}

// ---- Rol satırı -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct DefaultAppRow {
    desc: &'static SettingDesc,
    role: Role,
    title: Label,
    subtitle: Label,
    model: Option<RowModel>,
    /// Shell'in bildirdiği mevcut seçim (seçimin yeni olup olmadığı buna göre).
    current: Option<String>,
    select: Box<Select>,
    /// Seçenek başına yüklü ikon dokusunun anahtarı (fiziksel piksel, önbellek kuşağı); seçici
    /// yeniden kurulunca boşalır. İkon seçicinin içinde (alan + liste), satırda ayrı ikon yok.
    icon_keys: Vec<Option<(u32, u64)>>,
    notes: Vec<(Label, Tone)>,
    previous: PreviousAction,
    /// Seçim gönderildi, durum bekleniyor: seçici yeni seçimi gösterir.
    pending: bool,
    /// Canlı test: seçiciyi bir sonraki çizimde aç.
    demo_open: bool,
    stack: bool,
    rect: Rect,
}

impl DefaultAppRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, role: Role) -> Self {
        default_apps::ensure();
        let mut s = Self {
            desc,
            role,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(desc.description(), text_style(theme, TextRole::Caption)),
            model: None,
            current: None,
            select: Box::new(Select::new(theme, vec![String::new()], 0)),
            icon_keys: Vec::new(),
            notes: Vec::new(),
            previous: PreviousAction::new(theme, desc),
            pending: false,
            demo_open: false,
            stack: false,
            rect: Rect::ZERO,
        };
        s.refresh(theme, Instant::now());
        s
    }

    fn role_status(&self) -> Option<RoleStatus> {
        default_apps::status().ready().and_then(|s| s.role(self.role).cloned())
    }

    /// Durumu yeniden okur; seçici yalnızca seçenekler değiştiyse yeniden kurulur.
    fn refresh(&mut self, theme: &Theme, now: Instant) {
        self.pending = false;
        let Some(rs) = self.role_status() else {
            self.model = None;
            return;
        };
        let model = RowModel::from_status(&rs);
        self.current = rs.current.as_ref().map(|c| c.id.clone());
        if !self.model.as_ref().is_some_and(|m| m.same_options(&model)) {
            // İkonlar sonradan gelir: yuva baştan ayrılır (yerleşim zıplamaz).
            *self.select = Select::with_entries(theme, model.entries(), model.selected).with_icon_slot(true);
            self.icon_keys.clear();
        } else if !self.select.is_open() {
            self.select.set_selected(model.selected);
        }
        self.select.set_enabled(model.enabled, now);
        self.notes = model.notes.iter().map(|(n, tone)| (Label::new(n.as_str(), text_style(theme, TextRole::Caption)), *tone)).collect();
        self.model = Some(model);
    }

    fn choose(&mut self, env: &mut Env, i: usize) {
        let Some(model) = &self.model else { return };
        let Some(id) = model.pick(i, self.current.as_deref()) else { return };
        self.pending = true;
        env.push(Action::Service(Request::DefaultApps(default_apps::Command::Set { role: self.role, id: id.to_owned() })));
    }

    fn select_width(theme: &Theme, width: f32) -> f32 {
        (width * 0.4).clamp(theme.space(40), theme.space(70))
    }

    /// Seçeneklerin uygulama ikonları: yüklenmemiş olanı önbellekten ister, geleni dokuya
    /// yükler (çizimde; ölçek belli). Gelmeyen/temada olmayan ikon seçicinin yer tutucusunda
    /// kalır. İkon teması değişince ([`app_icons::generation`]) yeniden istenir.
    fn load_icons(&mut self, ui: &mut Ui) {
        let Some(model) = &self.model else { return };
        let px = (text_style(ui.theme, TextRole::Icon).size * ui.painter.scale().max(1.0)).round() as u32;
        let generation = app_icons::generation();
        self.icon_keys.resize(model.icons.len(), None);
        for (i, name) in model.icons.iter().enumerate() {
            let Some(name) = name else { continue };
            if self.icon_keys[i] == Some((px, generation)) {
                continue;
            }
            if let Some(img) = app_icons::get(name, px) {
                let (width, height) = img.size();
                let tex: Texture = ui.painter.upload_texture(ImageData { width, height, pixels: std::sync::Arc::from(img.pixels()) }, TextureOptions::default());
                self.select.set_option_icon(i, Some(Icon::image(tex)));
                self.icon_keys[i] = Some((px, generation));
            }
        }
    }

    fn notes_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let (px, py) = row_padding(env.theme);
        let mut h = 0.0;
        for (n, _) in &mut self.notes {
            n.set_max_width(Some(width - px * 2.0));
            h += n.measure(env.text).height + py * 0.5;
        }
        h
    }
}

impl Block for DefaultAppRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        default_apps::role_ready(self.role)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let sw = Self::select_width(t, width);
        let ss = self.select.preferred_size(t, sw);
        let ctl = ss.width;
        self.stack = crate::rows::should_stack(t, env.text, width, ctl, &mut self.title, Some(&mut self.subtitle));
        let text_w = if self.stack { crate::rows::stacked_text_width(t, width) } else { side_text_width(t, width, ctl) };
        self.title.set_max_width(Some(text_w));
        self.subtitle.set_max_width(Some(text_w));
        let sub_h = if self.desc.subtitle.is_empty() { 0.0 } else { t.space(1) + self.subtitle.measure(env.text).height };
        let text_h = self.title.measure(env.text).height + sub_h;
        let head = if self.stack { text_h + t.space(2) + ss.height + py * 2.0 } else { row_height_for(t, text_h.max(ss.height)) };
        head + self.notes_height(env, width)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let sw = Self::select_width(t, rect.width);
        let ss = self.select.preferred_size(t, sw);
        let ps = self.previous.size(t);
        let sub_h = if self.desc.subtitle.is_empty() { 0.0 } else { t.space(1) + self.subtitle.measure(env.text).height };
        let th = self.title.measure(env.text).height + sub_h;
        let (sel, cy_head) = if self.stack || is_narrow(t, rect.width) {
            let x = rect.x + px;
            let sel = Rect::new(x, rect.y + py + th + t.space(2), rect.right() - px - x, ss.height);
            (sel, rect.y + py + self.title.measure(env.text).height * 0.5)
        } else {
            let head = row_height_for(t, th.max(ss.height));
            let cy = rect.y + head * 0.5;
            (Rect::new(rect.right() - px - ss.width, cy - ss.height * 0.5, ss.width, ss.height), cy)
        };
        self.select.set_rect(t, sel);
        // "Önceki hâline dön": başlığın hizasında, sağ kenarda (dar) ya da seçicinin solunda.
        let prev_right = if self.stack || is_narrow(t, rect.width) { rect.right() - px } else { sel.x - t.space(3) };
        self.previous.set_rect(Rect::new(prev_right - ps.width, cy_head - ps.height * 0.5, ps.width, ps.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        if self.select.is_open() {
            let r = self.select.pointer(env.text, env.theme, e, now);
            if let Some(SelectEvent::Changed(i)) = r.event {
                self.choose(env, i);
            }
            return true;
        }
        let (repaint, fired) = self.previous.pointer(env, e);
        if fired {
            return true;
        }
        let r = self.select.pointer(env.text, env.theme, e, now);
        if self.select.is_open() {
            env.push(Action::Focus);
        }
        repaint || r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let was_open = self.select.is_open();
        let r = self.select.key(env.text, env.theme, e, env.now);
        if let Some(SelectEvent::Changed(i)) = r.event {
            self.choose(env, i);
            return true;
        }
        was_open || r.repaint
    }

    fn captures_keys(&self) -> bool {
        self.select.is_open()
    }

    fn overlay_open(&self) -> bool {
        self.select.is_open()
    }

    fn focusable(&self) -> bool {
        self.model.as_ref().is_some_and(|m| m.enabled)
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.select.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.previous.cursor(p).or_else(|| self.select.cursor(p))
    }

    fn sync(&mut self, env: &mut Env) {
        // Store bu satırı etkilemez (değer shell'de); yalnızca bekleyen seçim yoksa yenile.
        if !self.pending {
            self.refresh(env.theme, env.now);
        }
    }

    fn service_event(&mut self, env: &mut Env, event: &Event) -> bool {
        match event {
            Event::DefaultApps => {
                self.refresh(env.theme, env.now);
                true
            }
            Event::Snapshots => self.previous.refresh(),
            _ => false,
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        for (n, _) in &mut self.notes {
            n.set_style(text_style(theme, TextRole::Caption));
        }
        self.select.set_theme(theme, now);
        self.previous.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.load_icons(ui);
        if std::mem::take(&mut self.demo_open) {
            let enter = KeyEvent::new(Key::Enter, KeyState::Pressed, None, Modifiers::default());
            let _ = self.select.key(ui.painter.text_system(), ui.theme, &enter, ui.now);
        }
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let rect = self.rect;
        let sel = self.select.rect();
        let th = self.title.measure_in(ui.painter).height;
        let sh = if self.desc.subtitle.is_empty() { 0.0 } else { self.subtitle.measure_in(ui.painter).height + t.space(1) };
        let ty = if self.stack || is_narrow(t, rect.width) {
            rect.y + py
        } else {
            let head = (sel.center().y - rect.y) * 2.0;
            rect.y + (head - th - sh) * 0.5
        };
        self.title.draw(ui.painter, Point::new(rect.x + px, ty), ui.fade(c.on_surface));
        if sh > 0.0 {
            self.subtitle.draw(ui.painter, Point::new(rect.x + px, ty + th + t.space(1)), ui.fade(c.on_surface_variant));
        }
        self.previous.draw(ui);
        let mut y = sel.bottom() + py;
        if !(self.stack || is_narrow(t, rect.width)) {
            y = y.max(rect.y + row_height_for(t, (th + sh).max(sel.height)) - py * 0.5);
        }
        for (n, tone) in &mut self.notes {
            let color = match tone {
                Tone::Info => c.on_surface_variant,
                Tone::Warning => c.error,
            };
            n.draw(ui.painter, Point::new(rect.x + px, y), ui.fade(color));
            y += n.measure_in(ui.painter).height + py * 0.5;
        }
        self.select.draw(ui);
        // Menü kaplamada (satırın kırpmasından bağımsız).
        self.select.set_bounds(ui.surface_rect());
        self.select.draw_overlay(ui);
    }

    fn demo_expand(&mut self, _now: Instant) {
        // Canlı test (`NUXUT_SETTINGS_DEBUG=expand=defaults.<rol>`): seçici bir sonraki çizimde açılır
        // (açmak metin sistemi ister).
        self.demo_open = true;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::services::default_apps::{App, DefaultsState, DefaultsStatus, Source, parse_status};

    fn app(id: &str, name: &str, hidden: bool) -> App {
        App { id: id.into(), name: name.into(), icon: Some(id.trim_end_matches(".desktop").into()), hidden }
    }

    fn role(current: Option<App>, candidates: Vec<App>) -> RoleStatus {
        RoleStatus { role: Role::Browser, current, source: Source::User, mixed: false, shadowed_by: None, available: true, candidates }
    }

    #[test]
    fn row_lists_visible_then_hidden_and_selects_current() {
        crate::i18n::use_turkish_for_tests();
        let rs = role(Some(app("firefox.desktop", "Firefox", false)), vec![
            app("chromium.desktop", "Chromium", false),
            app("handler.desktop", "İşleyici", true),
            app("firefox.desktop", "Firefox", false),
        ]);
        let m = RowModel::from_status(&rs);
        assert_eq!(m.labels, ["Chromium", "Firefox", "İşleyici"]);
        assert_eq!(m.selected, 1);
        assert!(m.enabled && m.notes.is_empty());
        assert_eq!(m.icons, [Some("chromium".to_owned()), Some("firefox".to_owned()), Some("handler".to_owned())], "seçenek başına ikon");
        assert_eq!(m.other_at, Some(2), "NoDisplay adaydan önce Diğer başlığı");
        let e = m.entries();
        assert_eq!(e.len(), 4);
        assert_eq!(e[2], SelectEntry::header("Diğer"));
        assert_eq!(e[3], SelectEntry::option("İşleyici"));
        assert_eq!(m.pick(1, Some("firefox.desktop")), None, "mevcut seçim yeniden gönderilmez");
        assert_eq!(m.pick(2, Some("firefox.desktop")), Some("handler.desktop"));
        assert_eq!(m.pick(9, None), None);
    }

    #[test]
    fn row_placeholder_when_nothing_or_not_installed() {
        crate::i18n::use_turkish_for_tests();
        let none = RowModel::from_status(&role(None, vec![app("a.desktop", "A", false)]));
        assert_eq!(none.ids[0], None, "yer tutucu");
        assert_eq!(none.selected, 0);
        assert_eq!(none.pick(0, None), None, "yer tutucu seçilemez");
        assert_eq!(none.pick(1, None), Some("a.desktop"));
        assert_eq!(none.icons, [None, Some("a".to_owned())], "yer tutucunun ikonu yok");
        assert_eq!(none.other_at, None);
        let only_hidden = RowModel::from_status(&role(None, vec![app("h.desktop", "H", true)]));
        assert_eq!(only_hidden.other_at, Some(1), "yer tutucudan sonra gizli aday: başlık var");
        let all_hidden = RowModel::from_status(&role(Some(app("h.desktop", "H", true)), vec![app("h.desktop", "H", true)]));
        assert_eq!(all_hidden.other_at, None, "ayıracak görünür seçenek yok: başlık yok");
        let gone = RowModel::from_status(&role(Some(App { id: "gone.desktop".into(), name: "gone.desktop".into(), icon: None, hidden: false }), vec![app("a.desktop", "A", false)]));
        assert!(gone.labels[0].contains("gone.desktop") && gone.labels[0] != "gone.desktop", "kurulu değil eki: {}", gone.labels[0]);
        assert_eq!(gone.selected, 0);
        assert_eq!(gone.pick(0, Some("gone.desktop")), None);
    }

    #[test]
    fn row_notes_for_mixed_shadowed_and_unavailable() {
        crate::i18n::use_turkish_for_tests();
        let mut rs = role(Some(app("eog.desktop", "Görüntüleyici", false)), vec![app("eog.desktop", "Görüntüleyici", false)]);
        rs.mixed = true;
        rs.shadowed_by = Some("~/.config/hyprland-mimeapps.list".into());
        let m = RowModel::from_status(&rs);
        assert_eq!(m.notes.len(), 2);
        assert_eq!(m.notes[0].1, Tone::Info);
        assert!(m.notes[0].0.contains("başka uygulamada"), "{}", m.notes[0].0);
        assert_eq!(m.notes[1].1, Tone::Warning);
        assert!(m.notes[1].0.contains("~/.config/hyprland-mimeapps.list"));
        let mut term = role(None, vec![app("kitty.desktop", "kitty", false)]);
        term.role = Role::Terminal;
        term.available = false;
        let m = RowModel::from_status(&term);
        assert!(!m.enabled, "seçici kapalı");
        assert!(m.notes[0].0.contains("xdg-terminal-exec"));
    }

    #[test]
    fn twin_names_are_disambiguated_and_bad_icons_dropped() {
        crate::i18n::use_turkish_for_tests();
        let mut a = app("org.a.Editor.desktop", "Editor", false);
        a.icon = Some("/opt/editor.png".into());
        let rs = role(Some(a.clone()), vec![a, app("b-editor.desktop", "Editor", false)]);
        let m = RowModel::from_status(&rs);
        assert!(m.labels.iter().all(|l| l.starts_with("Editor") && l.contains(".desktop")), "{:?}", m.labels);
        assert_eq!(m.icons[m.ids.iter().position(|i| i.as_deref() == Some("org.a.Editor.desktop")).unwrap()], None, "mutlak yollu ikon çözülmez");
    }

    #[test]
    fn rows_follow_service_state() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let desc = crate::registry::by_id("defaults.browser").expect("kayıtta");
        let s = parse_status(r#"{"v":1,"roles":[{"role":"browser","current":{"id":"firefox.desktop","name":"Firefox","icon":"firefox"},"source":"user","mixed":false,"shadowed_by":null,"available":true,"candidates":[{"id":"firefox.desktop","name":"Firefox","icon":"firefox","hidden":false},{"id":"chromium.desktop","name":"Chromium","icon":"chromium","hidden":false}]}]}"#).unwrap();
        default_apps::set_status_for_tests(DefaultsState::Ready(Arc::new(s)));
        let row = DefaultAppRow::new(&theme, desc, Role::Browser);
        assert_eq!(row.select.options(), ["Firefox", "Chromium"], "shell sırası korunur");
        assert_eq!(row.select.selected(), 0);
        assert!(row.select.has_icon_slot(), "ikonlar sonradan gelir; yuva baştan ayrılır");
        assert_eq!(row.current.as_deref(), Some("firefox.desktop"));
        assert!(default_apps::role_ready(Role::Browser));
        assert!(!default_apps::role_ready(Role::Pdf), "yanıtta olmayan rol gizli");
        default_apps::set_status_for_tests(DefaultsState::Ready(Arc::new(DefaultsStatus::default())));
        assert!(!default_apps::role_ready(Role::Browser));
    }

    /// Maskot yerleşimi: kabuk çalışmıyor / test örneği / okuma hatası → hata maskotu (notun
    /// yerine); yükleniyor, sürüm desteklemiyor ve hazır → maskot yok.
    #[test]
    fn unavailable_shell_shows_error_mascot() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let mut n = DefaultAppsNote::new(&theme, crate::registry::by_id("defaults.note").unwrap());
        let ready = DefaultsState::Ready(std::sync::Arc::new(default_apps::DefaultsStatus::default()));
        for (state, want) in [
            (DefaultsState::NotRunning, Some(Spot::DefaultAppsUnavailable)),
            (DefaultsState::TestInstance, Some(Spot::DefaultAppsUnavailable)),
            (DefaultsState::Failed("x".into()), Some(Spot::DefaultAppsUnavailable)),
            (DefaultsState::Pending, None),
            (DefaultsState::Unsupported, None),
            (ready, None),
        ] {
            n.apply(&theme, &state);
            assert_eq!(n.mascot(), want, "{state:?}");
            assert_eq!(n.mascot().map(Spot::mood), want.map(|_| nuxut::ui::Mood::Error));
        }
        n.apply(&theme, &DefaultsState::NotRunning);
        assert!(n.fiyu.as_ref().unwrap().title().contains("çalışmıyor"));
    }
}
