//! Monitör başına duvar kağıdı: `[wallpaper.outputs."<bağlayıcı>"] path, fit` (shell şeması;
//! ikisi de opsiyonel, yoksa genel `[wallpaper]` değeri). En fazla 32 monitör.
//!
//! Satırlar: bağlı monitörler (Wayland çıkış adları) ∪ config'te yazılı olanlar ∪ elle
//! eklenenler (bağlı olmayan bir monitör için). Satır başlığı Ekranlar'daki insan adıdır
//! (shell `msg displays list`: model; yoksa Wayland açıklaması; yoksa bağlayıcı) — bağlayıcı
//! adı yalnızca ad çakışırsa görünür (§8). Her satırda monitör oranlı küçük resim (o monitörde
//! gerçekten görünen görsel ve yerleşim), yerleşim açılır seçimi (dar pencerede kırpılmasın
//! diye segment değil; §4), görsel seç ve sıfırla.
//!
//! Bağlı olmayan monitör ekleme seyrek bir iş: kartın sonunda framework `Disclosure`'ının
//! içinde (§3.5 "A"). Elle eklenen satır, görsel ya da yerleşim seçilene dek yalnızca ekranda
//! durur; dosyaya bir şey yazılmaz.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::render::ImageFit;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::{
    Button, ButtonVariant, Disclosure, DisclosureEvent, Icon, IconButton, IconButtonVariant, IconName, Label, Select, SelectEvent, TextInput, TextInputEvent,
    TextRole, Ui, text_style,
};
use nuxut::{Point, Rect, RectStyle, Theme};
use super::wallpaper_grid::WallThumb;
use crate::services::wallpaper_source::WallSource;
use toml_edit::Value;

use super::wallpaper::{Backdrop, current_source, general_fit, image_fit};
use crate::block::{Action, Block, Env};
use crate::registry::{SettingDesc, WALLPAPER_FITS, choice_label};
use crate::rows::{is_narrow, row_padding};
use crate::services::displays::shell::{Command as DisplaysCommand, Update as DisplaysUpdate};
use crate::services::displays::{self as model, List};
use crate::services::outputs::{self, Output};
use crate::services::portal::Update as PortalUpdate;
use crate::services::{Event, Request};
use crate::store::{FileId, KeyPath, Store};

/// Portal etiketi öneki; ardından bağlayıcı adı gelir.
const PICK_PREFIX: &str = "wallpaper-output:";
/// Shell'in sınırı.
const MAX_OUTPUTS: usize = 32;
const MAX_NAME: usize = 64;
/// Ekran bilgisi yokken varsayılan mantıksal boy (16:9).
pub(super) const FALLBACK_SCREEN: (f32, f32) = (1920.0, 1080.0);

// ---- Ekranlar'dan bilinen monitörler ------------------------------------------------------

/// Ekranlar servisinin (`msg displays list`) bir monitörü: bağlayıcı → insan adı ve mantıksal
/// boy (küçük resmin oranı).
#[derive(Debug, Clone, PartialEq)]
pub struct KnownDisplay {
    pub connector: String,
    pub name: String,
    pub size: (f32, f32),
    pub primary: bool,
}

static KNOWN: Mutex<Vec<KnownDisplay>> = Mutex::new(Vec::new());
static ASKED: AtomicBool = AtomicBool::new(false);

/// Ekranlar listesini bir kez ister (Ekranlar sayfası açılmamış olsa da adlar gelsin). Cevap
/// her sayfaya `Event::Displays` olarak yayılır; [`on_displays`] önbelleğe yazar.
pub(super) fn ask_displays() {
    if !ASKED.swap(true, Ordering::Relaxed) {
        crate::services::dispatch(Request::Displays(DisplaysCommand::Refresh));
    }
}

/// Liste geldi: önbelleği günceller; değiştiyse `true`.
pub(super) fn on_displays(event: &Event) -> bool {
    let Event::Displays(DisplaysUpdate::List(list)) = event else { return false };
    let known = known_from(list);
    let mut cur = KNOWN.lock().unwrap_or_else(|e| e.into_inner());
    if *cur == known {
        return false;
    }
    *cur = known;
    true
}

/// Liste → bilinen monitörler (saf; testli). Kapalı monitörün de adı bilinir.
pub fn known_from(list: &List) -> Vec<KnownDisplay> {
    list.displays
        .iter()
        .filter(|d| !d.connector.is_empty())
        .map(|d| {
            let (w, h) = model::logical_size(d.mode, d.scale_120(), d.transform);
            KnownDisplay { connector: d.connector.clone(), name: d.name().to_owned(), size: (w.max(1) as f32, h.max(1) as f32), primary: list.primary.as_deref() == Some(d.id.as_str()) }
        })
        .collect()
}

pub(super) fn known() -> Vec<KnownDisplay> {
    KNOWN.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Ana monitörün (yoksa ilk bilinenin) mantıksal boyu; hiç bilgi yoksa 16:9.
pub(super) fn primary_screen() -> (f32, f32) {
    let k = KNOWN.lock().unwrap_or_else(|e| e.into_inner());
    k.iter().find(|d| d.primary).or(k.first()).map_or(FALLBACK_SCREEN, |d| d.size)
}

/// Wayland açıklamasından ad: sondaki " (<bağlayıcı>)" atılır ("Dell Inc. U2720Q (DP-1)").
fn from_description(desc: &str, connector: &str) -> String {
    let suffix = format!(" ({connector})");
    desc.strip_suffix(&suffix).unwrap_or(desc).trim().to_owned()
}

/// Satır başlıkları (saf; testli): Ekranlar'daki ad, yoksa Wayland açıklaması, yoksa bağlayıcı.
/// Aynı ad iki kez çıkarsa ikisine de bağlayıcı eklenir (iki aynı model monitör).
pub fn titles(rows: &[(String, Option<Output>)], known: &[KnownDisplay]) -> Vec<String> {
    let base: Vec<String> = rows
        .iter()
        .map(|(conn, out)| {
            known
                .iter()
                .find(|k| &k.connector == conn)
                .map(|k| k.name.clone())
                .or_else(|| out.as_ref().map(|o| from_description(&o.description, conn)))
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| conn.clone())
        })
        .collect();
    base.iter()
        .zip(rows)
        .map(|(n, (conn, _))| if n != conn && base.iter().filter(|m| *m == n).count() > 1 { format!("{n} · {conn}") } else { n.clone() })
        .collect()
}

// ---- Config ---------------------------------------------------------------------------------

fn key(name: &str, field: &str) -> KeyPath {
    KeyPath::new(["wallpaper", "outputs", name, field])
}

/// Bağlayıcı adı geçerli mi (Wayland adları kısa ASCII; tırnak ve kontrol karakteri yok).
pub fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= MAX_NAME && name.chars().all(|c| c.is_ascii_graphic() && c != '"' && c != '\\')
}

/// Config'te yazılı monitörler.
fn configured(store: &Store) -> Vec<String> {
    let file = store.file(FileId::Shell);
    let Some(t) = file.get(&KeyPath::parse("wallpaper.outputs")).and_then(|i| i.as_table_like()) else { return Vec::new() };
    t.iter().map(|(k, _)| k.to_owned()).collect()
}

/// Yerleşim seçenekleri: ilki "Genel · <genel yerleşim>" (anahtar yok), sonra `WALLPAPER_FITS`.
fn fit_labels(general: &str) -> Vec<String> {
    let g = Some(choice_label(WALLPAPER_FITS, general)).filter(|l| !l.is_empty()).unwrap_or_else(|| choice_label(WALLPAPER_FITS, WALLPAPER_FITS[0].0));
    std::iter::once(t!("settings-outputs-fit-general-option", fit = g)).chain(WALLPAPER_FITS.iter().map(|(k, _)| choice_label(WALLPAPER_FITS, k).to_owned())).collect()
}

/// Seçili yerleşim: 0 = genel (anahtar yok), 1.. = `WALLPAPER_FITS` sırası.
fn fit_index(store: &Store, name: &str) -> usize {
    let v = store.get(FileId::Shell, &key(name, "fit")).and_then(Value::as_str);
    v.and_then(|v| WALLPAPER_FITS.iter().position(|(k, _)| *k == v)).map_or(0, |i| i + 1)
}

fn path_of(store: &Store, name: &str) -> Option<String> {
    store.get(FileId::Shell, &key(name, "path")).and_then(Value::as_str).filter(|p| !p.is_empty()).map(str::to_owned)
}

/// Bu monitörde görünen yerleşim (kendi değeri, yoksa genel).
fn effective_fit(store: &Store, name: &str) -> String {
    store.get(FileId::Shell, &key(name, "fit")).and_then(Value::as_str).map_or_else(|| general_fit(store), str::to_owned)
}

// ---- Blok -----------------------------------------------------------------------------------

#[derive(Debug)]
struct Row {
    name: String,
    title: Label,
    detail: Label,
    /// Küçük resim (monitörün kendi görseli, yoksa genel); dosya `draw`'da istenir.
    thumb: WallThumb,
    thumb_rect: Rect,
    /// Monitörün mantıksal boyu (küçük resmin oranı).
    screen: (f32, f32),
    fit: Box<Select>,
    /// Seçicinin kurulduğu genel yerleşim (etiketi "Genel · …").
    fit_general: String,
    fit_value: ImageFit,
    /// Bu monitörde yerleşik (`nuxut:`) görünüyor: her zaman cover, yerleşim seçicisi gizli (yeri ayrılır).
    builtin: bool,
    pick: IconButton,
    reset: IconButton,
    /// Dosyada bir değeri var mı (sıfırlama düğmesi yalnızca o zaman).
    set: bool,
    hover: bool,
}

#[derive(Debug)]
pub struct WallpaperOutputs {
    desc: &'static SettingDesc,
    rect: Rect,
    rows: Vec<Row>,
    /// Elle eklenen, henüz yazılmamış adlar.
    manual: Vec<String>,
    empty: Label,
    add_header: Disclosure,
    input: TextInput,
    add: Button,
    error: Label,
    focused: bool,
    focus_visible: bool,
    /// Odak kartın sonundaki açılır satırda mı (başlık ya da içindeki alan).
    add_focus: AddFocus,
    backdrop: Backdrop,
    theme: Theme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AddFocus {
    Header,
    Input,
}

impl WallpaperOutputs {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        ask_displays();
        let mut s = Self {
            desc,
            rect: Rect::ZERO,
            rows: Vec::new(),
            manual: Vec::new(),
            empty: Label::new(t!("settings-outputs-no-monitors"), text_style(theme, TextRole::Body)),
            add_header: Disclosure::new(theme, &t!("settings-outputs-add-unconnected")).with_description(theme, &t!("settings-outputs-add-unconnected-description")),
            input: TextInput::new(theme).with_placeholder(t!("settings-outputs-connector-placeholder")).with_max_bytes(MAX_NAME),
            add: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::Add)).with_label(theme, t!("settings-outputs-add-button")),
            error: Label::new("", text_style(theme, TextRole::Caption)),
            focused: false,
            focus_visible: false,
            add_focus: AddFocus::Header,
            backdrop: Backdrop::read(store),
            theme: theme.clone(),
        };
        s.rebuild(store);
        s
    }

    fn names(&self, store: &Store) -> Vec<(String, Option<Output>)> {
        let connected = outputs::list();
        let mut names: Vec<String> = connected.iter().map(|o| o.name.clone()).collect();
        names.extend(configured(store));
        names.extend(self.manual.iter().cloned());
        names.sort();
        names.dedup();
        let mut rows: Vec<(String, Option<Output>)> = names
            .into_iter()
            .map(|n| {
                let o = connected.iter().find(|o| o.name == n).cloned();
                (n, o)
            })
            .collect();
        // Bağlı monitörler önce.
        rows.sort_by_key(|(_, o)| o.is_none());
        rows
    }

    fn new_row(theme: &Theme, name: &str, general: &str) -> Row {
        Row {
            name: name.to_owned(),
            title: Label::new("", text_style(theme, TextRole::Body).with_max_lines(1)),
            detail: Label::new("", text_style(theme, TextRole::Caption).with_max_lines(1)),
            thumb: WallThumb::new(ImageFit::Cover, theme.motion.effects),
            thumb_rect: Rect::ZERO,
            screen: FALLBACK_SCREEN,
            fit: Box::new(Select::new(theme, fit_labels(general), 0)),
            fit_general: general.to_owned(),
            fit_value: ImageFit::Cover,
            builtin: false,
            pick: IconButton::new(theme, Icon::named(IconName::Image), IconButtonVariant::Tonal),
            reset: IconButton::new(theme, Icon::named(IconName::Close), IconButtonVariant::Standard),
            set: false,
            hover: false,
        }
    }

    fn rebuild(&mut self, store: &Store) {
        let theme = self.theme.clone();
        let names = self.names(store);
        let general = general_fit(store);
        if !names.iter().map(|(n, _)| n).eq(self.rows.iter().map(|r| &r.name)) {
            self.rows = names.iter().map(|(name, _)| Self::new_row(&theme, name, &general)).collect();
        }
        let known = known();
        let titles = titles(&names, &known);
        let global = current_source(store);
        for ((row, (_, out)), title) in self.rows.iter_mut().zip(&names).zip(titles) {
            if row.fit_general != general {
                *row.fit = Select::new(&theme, fit_labels(&general), 0);
                row.fit_general.clone_from(&general);
            }
            row.fit.set_selected(fit_index(store, &row.name));
            let path = path_of(store, &row.name);
            row.set = path.is_some() || store.get(FileId::Shell, &key(&row.name, "fit")).is_some();
            let own = path.as_deref().map(WallSource::parse);
            let image = match &own {
                Some(WallSource::Builtin { builtin, .. }) => builtin.name(),
                Some(WallSource::File(p)) => p.file_name().map_or_else(|| p.display().to_string(), |f| f.to_string_lossy().into_owned()),
                None => t!("settings-outputs-general-wallpaper"),
            };
            let detail = if out.is_some() { image } else { t!("settings-outputs-detail-disconnected", image = image) };
            row.title.set_text(&title);
            row.detail.set_text(&detail);
            row.screen = known.iter().find(|k| k.connector == row.name).map_or_else(primary_screen, |k| k.size);
            row.fit_value = image_fit(&effective_fit(store, &row.name));
            row.thumb.set_fit(row.fit_value);
            // `nuxut:` adresi yerleşiktir: çözücüye gitmez, süreç içinde çizilir.
            let src = own.unwrap_or_else(|| global.clone());
            row.builtin = src.builtin().is_some();
            if row.thumb.source() != Some(&src) {
                row.thumb.set_source(&src);
            }
        }
        self.backdrop = Backdrop::read(store);
    }

    fn add_manual(&mut self, env: &mut Env) {
        let name = self.input.text().trim().to_owned();
        if !valid_name(&name) {
            self.error.set_text(&t!("settings-outputs-error-name-rules"));
            return;
        }
        if self.rows.len() >= MAX_OUTPUTS {
            self.error.set_text(&t!("settings-outputs-error-too-many"));
            return;
        }
        self.error.set_text("");
        if !self.rows.iter().any(|r| r.name == name) {
            self.manual.push(name);
        }
        self.input.set_text("");
        self.rebuild(env.store);
    }

    fn thumb_size(theme: &Theme, screen: (f32, f32)) -> (f32, f32) {
        let w = theme.space(18);
        let aspect = (screen.0 / screen.1).clamp(0.5, 3.5);
        (w, (w / aspect).round())
    }

    fn select_width(theme: &Theme) -> f32 {
        theme.space(50)
    }

    /// Sıfırlama düğmesinin yeri (+ boşluk): yalnızca bir satır elle ayarlıysa ayrılır; o zaman
    /// her satırda ayrılır ki seçiciler satırlar arasında hizalı kalsın. Hiçbiri ayarlı değilken
    /// seçici kart kenarına yaslanır (diğer satırlar gibi).
    fn reset_slot(&self, theme: &Theme) -> f32 {
        match self.rows.first() {
            Some(r) if self.rows.iter().any(|r| r.set) => r.reset.preferred_size(theme).width + theme.space(1),
            _ => 0.0,
        }
    }

    /// Bir satırın yüksekliği (`width` = kart genişliği). Dar kartta seçici alt satırda.
    fn row_height(t: &Theme, r: &mut Row, text: &mut nuxut::TextSystem, width: f32, reset_slot: f32) -> f32 {
        let narrow = is_narrow(t, width);
        let (_, th) = Self::thumb_size(t, r.screen);
        let bs = r.pick.preferred_size(t);
        let sh = r.fit.preferred_size(t, Self::select_width(t)).height;
        let text_w = Self::text_width(t, width, narrow, bs.width, reset_slot);
        r.title.set_max_width(Some(text_w));
        r.detail.set_max_width(Some(text_w));
        let text_h = r.title.measure(text).height + r.detail.measure(text).height;
        let line = th.max(text_h).max(bs.height).max(if narrow { 0.0 } else { sh });
        if narrow { line + t.space(2) + sh } else { line }
    }

    /// Ad sütununun genişliği: küçük resim ile (seçici +) düğmeler arası.
    fn text_width(theme: &Theme, width: f32, narrow: bool, button_w: f32, reset_slot: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let right = button_w + reset_slot + if narrow { 0.0 } else { Self::select_width(theme) + theme.space(3) };
        (width - px * 2.0 - theme.space(18) - theme.space(3) - right - theme.space(3)).max(theme.space(16))
    }

    fn add_content_height(&mut self, t: &Theme, text: &mut nuxut::TextSystem, width: f32) -> f32 {
        let (px, py) = row_padding(t);
        let ih = self.input.preferred_size(t, width).height;
        let mut h = ih + py;
        if !self.error.text().is_empty() {
            self.error.set_max_width(Some(width - px * 2.0));
            h += t.space(2) + self.error.measure(text).height;
        }
        h
    }

    fn set_focus_state(&mut self) {
        let fv = self.focused && self.focus_visible;
        self.add_header.set_focus_visible(fv && self.add_focus == AddFocus::Header);
    }
}

impl Block for WallpaperOutputs {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let mut h = py;
        let slot = self.reset_slot(t);
        if self.rows.is_empty() {
            self.empty.set_max_width(Some(width - px * 2.0));
            h += self.empty.measure(env.text).height + t.space(3);
        }
        for r in &mut self.rows {
            h += Self::row_height(t, r, env.text, width, slot) + t.space(4);
        }
        let content = self.add_content_height(t, env.text, width);
        self.add_header.set_content_height(content);
        self.add_header.header_height(env.text, t, width);
        h + self.add_header.height(env.now) - t.space(1)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let narrow = is_narrow(t, rect.width);
        let right = rect.right() - px;
        let mut y = rect.y + py;
        let slot = self.reset_slot(t);
        if self.rows.is_empty() {
            y += self.empty.measure(env.text).height + t.space(3);
        }
        for r in &mut self.rows {
            let h = Self::row_height(t, r, env.text, rect.width, slot);
            let (tw, th) = Self::thumb_size(t, r.screen);
            let bs = r.pick.preferred_size(t);
            let sh = r.fit.preferred_size(t, Self::select_width(t)).height;
            let line = if narrow { h - t.space(2) - sh } else { h };
            let mid = y + line * 0.5;
            r.thumb_rect = Rect::new(rect.x + px, mid - th * 0.5, tw, th);
            // Seç düğmesi kart kenarında; sıfırlama (ayrıldıysa) solunda — kayıt satırlarındaki
            // ↺ gibi kontrolün yanında.
            r.pick.set_rect(Rect::new(right - bs.width, mid - bs.height * 0.5, bs.width, bs.height));
            let rs = r.reset.preferred_size(t);
            r.reset.set_rect(Rect::new(r.pick.rect().x - t.space(1) - rs.width, mid - rs.height * 0.5, rs.width, rs.height));
            let fit = if narrow {
                Rect::new(rect.x + px, y + line + t.space(2), rect.width - px * 2.0, sh)
            } else {
                let w = Self::select_width(t);
                Rect::new(r.pick.rect().x - slot - t.space(3) - w, mid - sh * 0.5, w, sh)
            };
            r.fit.set_rect(t, fit);
            y += h + t.space(4);
        }
        // Açılır satır: kart kenarına yaslı (sayfadaki açılır satır başlığıyla aynı ritim).
        let add_h = self.add_header.height(env.now);
        self.add_header.set_rect(Rect::new(rect.x, y - t.space(1), rect.width, add_h));
        let c = self.add_header.content_rect();
        let bs = self.add.preferred_size(env.text, t);
        let iw = (c.width - px * 2.0 - bs.width - t.space(3)).min(t.space(90));
        let ih = self.input.preferred_size(t, iw).height;
        self.input.set_rect(Rect::new(c.x + px, c.y, iw, ih));
        self.add.set_rect(Rect::new(c.x + px + iw + t.space(3), c.y + (ih - bs.height) * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let t = env.theme;
        // Açık menü olayın tek sahibi (dışına tıklama yalnızca kapatır).
        if let Some(r) = self.rows.iter_mut().find(|r| r.fit.is_open()) {
            let res = r.fit.pointer(env.text, t, e, now);
            if let Some(SelectEvent::Changed(i)) = res.event {
                let value = i.checked_sub(1).and_then(|i| WALLPAPER_FITS.get(i)).map(|(k, _)| Value::from(*k));
                env.push(Action::Set { label: t!("settings-outputs-fit", name = r.title.text()), file: FileId::Shell, path: key(&r.name, "fit"), value });
            }
            return true;
        }
        let mut repaint = false;
        let mut action = None;
        for r in &mut self.rows {
            // Hareketli görsel yalnızca imleç üstündeyken oynar: boşta iş yok.
            let hover = e.kind != PointerKind::Leave && r.thumb_rect.contains(e.position);
            if hover != r.hover {
                r.hover = hover;
                r.thumb.set_visible(hover, now);
                repaint = true;
            }
            if !r.builtin {
                let rf = r.fit.pointer(env.text, t, e, now);
                repaint |= rf.repaint;
            }
            if r.fit.is_open() {
                env.push(Action::Focus);
                return true;
            }
            let rp = r.pick.pointer(e, now);
            repaint |= rp.repaint;
            if rp.event.is_some() {
                action = Some(Action::Service(Request::PickWallpaper { tag: format!("{PICK_PREFIX}{}", r.name) }));
            }
            if r.set {
                let rr = r.reset.pointer(e, now);
                repaint |= rr.repaint;
                if rr.event.is_some() {
                    action = Some(Action::Group {
                        label: t!("settings-outputs-wallpaper", name = r.title.text()),
                        writes: vec![(FileId::Shell, key(&r.name, "path"), None), (FileId::Shell, key(&r.name, "fit"), None)],
                    });
                }
            }
        }
        if let Some(a) = action {
            env.push(a);
            return true;
        }
        let rh = self.add_header.pointer(e, now);
        repaint |= rh.repaint;
        if let Some(DisclosureEvent::Toggled(open)) = rh.event {
            self.add_focus = AddFocus::Header;
            if !open {
                self.input.set_focused(false, now);
            }
            return true;
        }
        if !self.add_header.is_open() {
            return repaint;
        }
        repaint |= self.input.pointer(env.text, t, e, now).repaint;
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && self.input.rect().contains(e.position)
        {
            self.add_focus = AddFocus::Input;
            env.push(Action::Focus);
        }
        let ra = self.add.pointer(e, now);
        repaint |= ra.repaint;
        if ra.event.is_some() {
            self.add_manual(env);
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let now = env.now;
        if let Some(r) = self.rows.iter_mut().find(|r| r.fit.is_open()) {
            let res = r.fit.key(env.text, env.theme, e, now);
            if let Some(SelectEvent::Changed(i)) = res.event {
                let value = i.checked_sub(1).and_then(|i| WALLPAPER_FITS.get(i)).map(|(k, _)| Value::from(*k));
                env.push(Action::Set { label: t!("settings-outputs-fit", name = r.title.text()), file: FileId::Shell, path: key(&r.name, "fit"), value });
            }
            return true;
        }
        if !self.focused {
            return false;
        }
        match self.add_focus {
            AddFocus::Header => {
                let r = self.add_header.key(e, now);
                if let Some(DisclosureEvent::Toggled(_)) = r.event {
                    return true;
                }
                // Açıkken Aşağı alana iner.
                if self.add_header.is_open() && e.state == KeyState::Pressed && e.key == Key::Down {
                    self.add_focus = AddFocus::Input;
                    self.input.set_focused(true, now);
                    self.set_focus_state();
                    return true;
                }
                r.repaint
            }
            AddFocus::Input => {
                if e.state == KeyState::Pressed && e.key == Key::Escape && !self.input.text().is_empty() {
                    self.input.set_text("");
                    return true;
                }
                let r = self.input.key(e, now);
                match r.event {
                    Some(TextInputEvent::Submit) => {
                        self.add_manual(env);
                        true
                    }
                    _ => r.repaint,
                }
            }
        }
    }

    fn captures_keys(&self) -> bool {
        self.rows.iter().any(|r| r.fit.is_open())
    }

    fn overlay_open(&self) -> bool {
        self.rows.iter().any(|r| r.fit.is_open())
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        self.focused = focused;
        self.focus_visible = visible;
        if !focused {
            self.add_focus = AddFocus::Header;
        }
        self.input.set_focused(focused && self.add_focus == AddFocus::Input, now);
        self.set_focus_state();
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        (self.add_focus == AddFocus::Input).then_some(&mut self.input)
    }

    fn demo_expand(&mut self, now: Instant) {
        self.add_header.set_open(true, false, now);
    }

    fn service_event(&mut self, env: &mut Env, event: &Event) -> bool {
        match event {
            Event::Outputs => {
                self.rebuild(env.store);
                true
            }
            Event::Displays(_) => {
                if on_displays(event) {
                    self.rebuild(env.store);
                    return true;
                }
                // Önbelleği başka bir blok güncellemiş olabilir: adlar her listede yeniden okunur.
                if matches!(event, Event::Displays(DisplaysUpdate::List(_))) {
                    self.rebuild(env.store);
                    return true;
                }
                false
            }
            Event::Portal(PortalUpdate::Picked { tag, path }) => {
                let Some(name) = tag.strip_prefix(PICK_PREFIX) else { return false };
                let label = self.rows.iter().find(|r| r.name == name).map_or_else(|| name.to_owned(), |r| r.title.text().to_owned());
                env.push(Action::Set { label: t!("settings-outputs-wallpaper", name = label), file: FileId::Shell, path: key(name, "path"), value: Some(Value::from(path.display().to_string())) });
                true
            }
            Event::Portal(PortalUpdate::Failed { tag, message }) if tag.starts_with(PICK_PREFIX) => {
                self.error.set_text(message);
                self.add_header.set_open(true, true, env.now);
                true
            }
            _ => false,
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if let Some(r) = self.rows.iter().find(|r| r.fit.is_open()) {
            return r.fit.cursor(p);
        }
        self.rows
            .iter()
            .find_map(|r| (!r.builtin).then(|| r.fit.cursor(p)).flatten().or_else(|| r.pick.cursor(p)).or_else(|| r.set.then(|| r.reset.cursor(p)).flatten()))
            .or_else(|| self.add_header.cursor(p))
            .or_else(|| self.add_header.is_open().then(|| self.input.cursor(p).or_else(|| self.add.cursor(p))).flatten())
    }

    fn sync(&mut self, env: &mut Env) {
        // Yazılan elle eklenmiş adlar artık config'ten gelir.
        let conf = configured(env.store);
        self.manual.retain(|n| !conf.contains(n));
        self.rebuild(env.store);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.theme = theme.clone();
        for r in &mut self.rows {
            r.title.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
            r.detail.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
            r.fit.set_theme(theme, now);
            r.pick.set_theme(theme, now);
            r.reset.set_theme(theme, now);
            r.thumb.set_motion(theme.motion.effects, now);
        }
        self.empty.set_style(text_style(theme, TextRole::Body));
        self.error.set_style(text_style(theme, TextRole::Caption));
        self.add_header.set_theme(theme, now);
        self.input.set_theme(theme, now);
        self.add.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let r = self.rect;
        if self.rows.is_empty() {
            self.empty.draw(ui.painter, Point::new(r.x + px, r.y + py), ui.fade(c.on_surface_variant));
        }
        let backdrop = self.backdrop;
        for row in &mut self.rows {
            // Küçük resim: monitör oranlı, o monitörde görünen görsel ve yerleşim.
            let tr = row.thumb_rect;
            let radii = t.shape.small();
            ui.painter.rect(tr, &RectStyle::filled(ui.fade(c.surface), radii));
            // Dosya: yalıtılmış çözücüye istek, 192×120 × ölçek (aynı istek yinelenmez).
            let target = crate::services::wallpaper::row_target(ui.painter.scale());
            row.thumb.set_radii(radii);
            row.thumb.set_backdrop(backdrop.resolve(t, row.fit_value, tr.width / row.screen.0));
            let st = row.thumb.draw(ui, tr, Some(target));
            if st.redraw || (row.hover && st.next_frame_in.is_some()) {
                ui.request_frame();
            }
            let tx = tr.right() + t.space(3);
            let mid = row.pick.rect().center().y;
            let th = row.title.measure_in(ui.painter).height;
            let dh = row.detail.measure_in(ui.painter).height;
            let top = mid - (th + dh) * 0.5;
            row.title.draw(ui.painter, Point::new(tx, top), ui.fade(c.on_surface));
            row.detail.draw(ui.painter, Point::new(tx, top + th), ui.fade(c.on_surface_variant));
            if !row.builtin {
                row.fit.draw(ui);
            }
            row.pick.draw(ui);
            if row.set {
                row.reset.draw(ui);
            }
        }
        self.add_header.draw(ui);
        if self.add_header.reveal(now) > 0.001 {
            let (input, add, error) = (&mut self.input, &mut self.add, &mut self.error);
            self.add_header.draw_content(ui, |ui, _| {
                input.draw(ui);
                add.draw(ui);
                if !error.text().is_empty() {
                    let y = input.rect().bottom() + ui.theme.space(2);
                    error.draw(ui.painter, Point::new(input.rect().x, y), ui.fade(ui.theme.colors.error));
                }
            });
        }
        // Açık yerleşim menüsü en üstte, kırpılmadan.
        for row in &mut self.rows {
            if row.fit.is_open() {
                row.fit.set_bounds(ui.surface_rect());
                row.fit.draw_overlay(ui);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connector_names() {
        assert!(valid_name("DP-1") && valid_name("HDMI-A-1") && valid_name("eDP-1"));
        assert!(!valid_name("") && !valid_name("DP 1") && !valid_name("a\"b") && !valid_name(&"x".repeat(65)));
    }

    fn kd(conn: &str, name: &str) -> KnownDisplay {
        KnownDisplay { connector: conn.into(), name: name.into(), size: FALLBACK_SCREEN, primary: false }
    }

    fn out(conn: &str, desc: &str) -> (String, Option<Output>) {
        (conn.into(), Some(Output { name: conn.into(), description: desc.into() }))
    }

    #[test]
    fn titles_prefer_display_names_and_hide_connectors() {
        let rows = [out("DP-1", "Dell Inc. DELL U2720Q ABC (DP-1)"), out("HEADLESS-1", "Headless output 1"), ("HDMI-A-9".into(), None)];
        let known = [kd("DP-1", "DELL U2720Q")];
        assert_eq!(titles(&rows, &known), ["DELL U2720Q", "Headless output 1", "HDMI-A-9"], "Ekranlar adı > Wayland açıklaması > bağlayıcı");
        let rows = [out("DP-1", ""), out("DP-2", "")];
        let known = [kd("DP-1", "LG 27GL"), kd("DP-2", "LG 27GL")];
        assert_eq!(titles(&rows, &known), ["LG 27GL · DP-1", "LG 27GL · DP-2"], "aynı model iki monitör ayrışır");
    }

    #[test]
    fn known_displays_come_from_the_displays_list() {
        let json = r#"{"v":1,"gen":3,"primary":"Dell|U2720Q|ABC","displays":[
            {"id":"Dell|U2720Q|ABC","connector":"DP-1","make":"Dell Inc.","model":"DELL U2720Q","enabled":true,"mode":{"w":3840,"h":2160,"mhz":60000},"pos":[0,0],"scale":2.0,"transform":0},
            {"id":"LG|27GL|","connector":"HDMI-A-1","model":"","description":"LG 27GL","enabled":true,"mode":{"w":2560,"h":1440,"mhz":60000},"pos":[1920,0],"scale":1.0,"transform":1}]}"#;
        let list = List::parse(json).expect("liste");
        let k = known_from(&list);
        assert_eq!(k[0], KnownDisplay { connector: "DP-1".into(), name: "DELL U2720Q".into(), size: (1920.0, 1080.0), primary: true });
        assert_eq!(k[1].name, "LG 27GL", "model yoksa açıklama (Ekranlar'daki ad)");
        assert_eq!(k[1].size, (1440.0, 2560.0), "döndürülmüş monitör dikey");
    }

    #[test]
    fn fit_labels_name_the_general_choice() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(fit_labels("contain")[0], "Genel · Tamamı");
        assert_eq!(fit_labels("bilinmeyen")[0], "Genel · Doldur ve kırp");
        assert_eq!(fit_labels("cover").len(), WALLPAPER_FITS.len() + 1);
    }
}
