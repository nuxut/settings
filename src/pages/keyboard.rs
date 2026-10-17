//! Klavye sayfasının özel blokları (sözleşme girdi-v1; `shell.toml [input.keyboard]`):
//!
//! - [`KeyboardLayouts`]: etkin düzen listesi — sırala, sil (en az bir kalır), "Düzen ekle" ile
//!   arama alanlı seçici (XKB kataloğu, varyantlar dahil), en çok [`MAX_LAYOUTS`]. Liste dosyada
//!   yazılıysa dosyadan, değilse çalışan masaüstünün ETKİN listesinden gelir; ilk değişiklik o
//!   listeden başlar (kullanıcının `tr`'si korunur). ↺ anahtarı siler.
//! - [`LayoutSwitch`]: düzenler arası geçiş (`grp:` seçeneği). Yazarken etkin `kb_options`'taki
//!   `grp:` olmayan seçenekler korunur.
//! - [`KeyTest`]: tekrar ayarını hissetmek için yerel metin alanı; hiçbir şey yazmaz.
//!
//! Liste işlemleri saf işlevlerdir ([`current_layouts_with`], [`with_added`], [`options_with_switch`]…).

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::state_layer_under;
use nuxut::ui::{
    Button, ButtonVariant, Icon, IconButton, IconButtonVariant, IconName, Interaction, Label, Notice, Select, SelectEvent, TextInput, TextInputEvent,
    TextRole, Ui, text_style,
};
use nuxut::{Corners, Point, Rect, Size, Theme};
use toml_edit::{InlineTable, Value};

use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{is_narrow, override_notice, row_height_for, row_padding, side_text_width};
use crate::services::input::{self, KbLayout};
use crate::services::xkb::{self, XkbCatalog};
use crate::store::{FileId, KeyPath, Store};

pub use crate::services::input::MAX_LAYOUTS;

/// `input.keyboard.layouts`
pub const LAYOUTS_KEY: &str = "input.keyboard.layouts";
/// `input.keyboard.options`
pub const OPTIONS_KEY: &str = "input.keyboard.options";
/// Seçicide aynı anda gösterilen en çok sonuç.
const MAX_RESULTS: usize = 8;

// ---- Saf liste işlemleri --------------------------------------------------------------------

/// Dosyadaki `layouts` değeri: `[{ layout = "tr", variant = "" }, …]`. Biçim bozuksa ya da
/// geçersiz ad varsa `None` (shell de anahtarı yok sayar).
pub fn parse_layouts(v: &Value) -> Option<Vec<KbLayout>> {
    let arr = v.as_array()?;
    let mut out = Vec::new();
    for item in arr.iter() {
        let t = item.as_inline_table()?;
        let layout = t.get("layout")?.as_str()?;
        let variant = t.get("variant").and_then(Value::as_str).unwrap_or("");
        if !input::valid_name(layout) || !(variant.is_empty() || input::valid_name(variant)) {
            return None;
        }
        out.push(KbLayout::new(layout, variant));
    }
    (!out.is_empty() && out.len() <= MAX_LAYOUTS).then_some(out)
}

/// Gösterilen liste: dosyada (geçerli) yazılıysa o, yoksa etkin liste, o da yoksa boş.
pub fn current_layouts_with(file: Option<&Value>, effective: Option<&[KbLayout]>) -> Vec<KbLayout> {
    if let Some(l) = file.and_then(parse_layouts) {
        return l;
    }
    effective.map(|e| e.iter().take(MAX_LAYOUTS).cloned().collect()).unwrap_or_default()
}

/// [`current_layouts_with`], store ve süreç genelindeki girdi durumuyla.
pub fn current_layouts(store: &Store) -> Vec<KbLayout> {
    let state = input::status();
    current_layouts_with(store.get(FileId::Shell, &KeyPath::parse(LAYOUTS_KEY)), state.ready().and_then(|s| s.layouts.as_deref()))
}

/// Dosyaya yazılacak değer (satır içi tablolar dizisi).
pub fn layouts_value(list: &[KbLayout]) -> Value {
    let mut arr = toml_edit::Array::new();
    for l in list {
        let mut t = InlineTable::new();
        t.insert("layout", Value::from(l.layout.as_str()));
        t.insert("variant", Value::from(l.variant.as_str()));
        arr.push(Value::InlineTable(t));
    }
    Value::Array(arr)
}

/// Sona ekler; liste doluysa ya da aynısı varsa `None`.
pub fn with_added(list: &[KbLayout], new: KbLayout) -> Option<Vec<KbLayout>> {
    if list.len() >= MAX_LAYOUTS || list.contains(&new) || !input::valid_name(&new.layout) {
        return None;
    }
    let mut out = list.to_vec();
    out.push(new);
    Some(out)
}

/// `i`. düzeni siler; tek düzen kaldıysa `None`.
pub fn without(list: &[KbLayout], i: usize) -> Option<Vec<KbLayout>> {
    if list.len() <= 1 || i >= list.len() {
        return None;
    }
    let mut out = list.to_vec();
    out.remove(i);
    Some(out)
}

/// `i`. düzeni bir yukarı (`up`) ya da aşağı taşır; sınırdaysa `None`.
pub fn moved(list: &[KbLayout], i: usize, up: bool) -> Option<Vec<KbLayout>> {
    let j = if up { i.checked_sub(1)? } else { i + 1 };
    if i >= list.len() || j >= list.len() {
        return None;
    }
    let mut out = list.to_vec();
    out.swap(i, j);
    Some(out)
}

/// Dosyadaki `options` (geçerli öğeler); dizi değilse `None`.
pub fn parse_options(v: &Value) -> Option<Vec<String>> {
    Some(v.as_array()?.iter().filter_map(Value::as_str).filter(|s| input::valid_option(s)).map(str::to_owned).collect())
}

/// Temel `kb_options`: dosyada yazılıysa o, yoksa etkin liste, o da yoksa boş.
pub fn current_options_with(file: Option<&Value>, effective: Option<&[String]>) -> Vec<String> {
    file.and_then(parse_options).or_else(|| effective.map(<[String]>::to_vec)).unwrap_or_default()
}

pub fn current_options(store: &Store) -> Vec<String> {
    let state = input::status();
    current_options_with(store.get(FileId::Shell, &KeyPath::parse(OPTIONS_KEY)), state.ready().and_then(|s| s.options.as_deref()))
}

/// Seçili geçiş seçeneği (ilk `grp:` öğesi).
pub fn current_switch(options: &[String]) -> Option<&str> {
    options.iter().map(String::as_str).find(|o| input::is_switch_option(o))
}

/// Geçiş seçeneği değişince yazılacak tam liste: `grp:` olmayanlar sırasıyla korunur, seçilen
/// `grp:` (varsa) sona eklenir. `None` = "Yok".
pub fn options_with_switch(base: &[String], grp: Option<&str>) -> Vec<String> {
    let mut out: Vec<String> = base.iter().filter(|o| !input::is_switch_option(o)).cloned().collect();
    if let Some(g) = grp.filter(|g| input::is_switch_option(g) && input::valid_option(g)) {
        out.push(g.to_owned());
    }
    out.truncate(input::MAX_OPTIONS);
    out
}

pub fn options_value(list: &[String]) -> Value {
    Value::Array(list.iter().map(String::as_str).collect())
}

/// Düzenin görünen adı ve kodu: katalogda varsa `display_name` ("Türkçe — Turkish"), kod
/// `tr` ya da `us · intl`.
pub fn layout_label(cat: Option<&XkbCatalog>, l: &KbLayout) -> (String, String) {
    let code = if l.variant.is_empty() { l.layout.clone() } else { format!("{} · {}", l.layout, l.variant) };
    let name = cat.and_then(|c| c.find(&l.layout, &l.variant).map(|(li, vi)| c.display_name(li, vi))).filter(|n| !n.is_empty());
    // Sağdan sola yazılan dil adı ("العربية — Arabic") satırın başından taşmasın: paragraf yönü
    // soldan sağa sabitlenir (U+200E), ad bir öğe olarak sola hizalı kalır.
    let name = name.map(|n| if n.chars().any(is_rtl) { format!("\u{200E}{n}") } else { n });
    (name.unwrap_or_else(|| code.clone()), code)
}

/// Sağdan sola yazılan betik (İbranice, Arapça, Süryanice, Thaana, N'Ko…).
fn is_rtl(c: char) -> bool {
    matches!(c as u32, 0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF)
}

/// "Düzen ekle" / "Vazgeç" düğmesi (ikonu da değişir).
fn add_button(theme: &Theme, open: bool) -> Button {
    let (icon, label) = if open {
        (IconName::Close, t!("settings-sections-cancel"))
    } else {
        (IconName::Add, t!("settings-keyboard-add"))
    };
    Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(icon)).with_label(theme, label)
}

/// Seçici sonuçları: sorguya uyan, listede olmayan düzen/varyantlar (boş sorguda nadir
/// düzenler hariç), en çok [`MAX_RESULTS`].
pub fn picker_results(cat: &XkbCatalog, query: &str, present: &[KbLayout]) -> Vec<KbLayout> {
    let empty = query.trim().is_empty();
    cat.search(query)
        .into_iter()
        .filter(|(li, vi)| {
            let l = &cat.layouts[*li];
            let exotic = match vi {
                Some(v) => l.variants[*v].exotic,
                None => l.exotic,
            };
            !(empty && exotic)
        })
        .map(|(li, vi)| {
            let l = &cat.layouts[li];
            KbLayout::new(l.name.clone(), vi.map(|v| l.variants[v].name.clone()).unwrap_or_default())
        })
        .filter(|k| !present.contains(k))
        .take(MAX_RESULTS)
        .collect()
}

// ---- Düzen listesi bloğu --------------------------------------------------------------------

#[derive(Debug)]
struct LayoutRow {
    name: Label,
    code: Label,
    up: IconButton,
    down: IconButton,
    remove: IconButton,
    rect: Rect,
}

#[derive(Debug)]
struct ResultRow {
    item: KbLayout,
    name: Label,
    code: Label,
    hover: Interaction,
    rect: Rect,
}

#[derive(Debug)]
pub struct KeyboardLayouts {
    desc: &'static SettingDesc,
    list: Vec<KbLayout>,
    rows: Vec<LayoutRow>,
    /// Liste boşken (etkin değer okunamadı) gösterilen açıklama.
    empty: Label,
    add: Button,
    /// Liste dolu: "Düğme yerine" açıklama.
    full: Label,
    reset: IconButton,
    show_reset: bool,
    /// Geçersiz kılma uyarısı (`overrides`).
    note: Option<Notice>,
    /// Seçici açık mı.
    open: bool,
    search: TextInput,
    results: Vec<ResultRow>,
    /// Klavyeyle seçili sonuç.
    cursor: usize,
    /// Seçicide sonuç yokken ya da katalog yüklenirken.
    status: Label,
    focused: bool,
    focus_visible: bool,
    /// Canlı test açılışı bekliyor (`demo_expand`).
    demo: bool,
    rect: Rect,
    add_rect: Rect,
}

impl KeyboardLayouts {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        input::ensure();
        let mut b = Self {
            desc,
            list: Vec::new(),
            rows: Vec::new(),
            empty: Label::new(t!("settings-keyboard-empty-effective"), text_style(theme, TextRole::Body)),
            add: add_button(theme, false),
            full: Label::new(t!("settings-keyboard-max", count = MAX_LAYOUTS), text_style(theme, TextRole::Caption)),
            reset: IconButton::new(theme, Icon::named(IconName::Restart), IconButtonVariant::Standard),
            show_reset: false,
            note: None,
            open: false,
            search: TextInput::new(theme).with_placeholder(t!("settings-keyboard-search-placeholder")).with_max_bytes(64),
            results: Vec::new(),
            cursor: 0,
            status: Label::new("", text_style(theme, TextRole::Body)),
            focused: false,
            focus_visible: false,
            rect: Rect::ZERO,
            add_rect: Rect::ZERO,
            demo: false,
        };
        b.refresh(theme, store, true);
        b
    }

    /// Listeyi, düğmeleri ve uyarıyı store + durumdan yeniden kurar.
    fn refresh(&mut self, theme: &Theme, store: &Store, force: bool) {
        let list = current_layouts(store);
        self.show_reset = store.get(FileId::Shell, &KeyPath::parse(LAYOUTS_KEY)).is_some();
        let state = input::status();
        let note = state.ready().and_then(|s| s.override_for(LAYOUTS_KEY)).map(input::override_note);
        match (note, &mut self.note) {
            (None, n) => *n = None,
            (Some(s), Some(l)) => {
                l.set_text(&s);
            }
            (Some(s), n) => *n = Some(override_notice(theme, &s)),
        }
        if !force && list == self.list && self.rows.len() == list.len() {
            return;
        }
        let cat = xkb::catalog();
        self.rows = list
            .iter()
            .map(|l| {
                let (name, code) = layout_label(cat.as_deref(), l);
                LayoutRow {
                    name: Label::new(name, text_style(theme, TextRole::Body)),
                    code: Label::new(code, text_style(theme, TextRole::Caption)),
                    up: IconButton::new(theme, Icon::named(IconName::ChevronUp), IconButtonVariant::Standard),
                    down: IconButton::new(theme, Icon::named(IconName::ChevronDown), IconButtonVariant::Standard),
                    remove: IconButton::new(theme, Icon::named(IconName::Close), IconButtonVariant::Standard),
                    rect: Rect::ZERO,
                }
            })
            .collect();
        self.list = list;
        if self.list.len() >= MAX_LAYOUTS {
            self.open = false;
        }
        self.refresh_results(theme);
    }

    fn refresh_results(&mut self, theme: &Theme) {
        if !self.open {
            self.results.clear();
            return;
        }
        let Some(cat) = xkb::catalog() else {
            self.results.clear();
            self.status.set_text(&t!("settings-keyboard-loading"));
            return;
        };
        let items = picker_results(&cat, self.search.text(), &self.list);
        let status = if items.is_empty() { t!("settings-keyboard-no-match") } else { String::new() };
        self.status.set_text(&status);
        self.results = items
            .into_iter()
            .map(|item| {
                let (name, code) = layout_label(Some(&cat), &item);
                ResultRow {
                    item,
                    name: Label::new(name, text_style(theme, TextRole::Body)),
                    code: Label::new(code, text_style(theme, TextRole::Caption)),
                    hover: Interaction::new(theme),
                    rect: Rect::ZERO,
                }
            })
            .collect();
        self.cursor = self.cursor.min(self.results.len().saturating_sub(1));
    }

    fn row_h(theme: &Theme) -> f32 {
        nuxut::ui::style::row_height(theme)
    }

    fn write(&self, env: &mut Env, list: Option<Vec<KbLayout>>) {
        env.set(self.desc.label(), FileId::Shell, KeyPath::parse(LAYOUTS_KEY), list.map(|l| layouts_value(&l)));
    }

    fn set_open(&mut self, env: &mut Env, open: bool) {
        self.open = open && self.list.len() < MAX_LAYOUTS;
        self.add = add_button(env.theme, self.open);
        self.search.set_text("");
        self.cursor = 0;
        self.search.set_focused(self.open && self.focused, env.now);
        self.refresh_results(env.theme);
        if self.open {
            env.push(Action::Focus);
        }
    }

    fn pick(&mut self, env: &mut Env, i: usize) {
        let Some(item) = self.results.get(i).map(|r| r.item.clone()) else { return };
        if let Some(list) = with_added(&self.list, item) {
            self.write(env, Some(list));
        }
        self.set_open(env, false);
    }

    fn picker_height(&mut self, env: &mut Env, width: f32) -> f32 {
        if !self.open {
            return 0.0;
        }
        // Arama alanı ekle satırında (Vazgeç'in solunda); burada yalnızca sonuçlar.
        let t = env.theme;
        let (px, py) = row_padding(t);
        let list_h = if self.results.is_empty() {
            self.status.set_max_width(Some(width - px * 2.0));
            self.status.measure(env.text).height + t.space(2)
        } else {
            self.results.len() as f32 * Self::row_h(t)
        };
        list_h + py
    }

    fn note_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let (px, py) = row_padding(env.theme);
        let Some(n) = &mut self.note else { return 0.0 };
        n.measure(env.text, env.theme, width - px * 2.0).height + py
    }
}

impl Block for KeyboardLayouts {
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
        let (px, py) = row_padding(t);
        let row_h = Self::row_h(t);
        if std::mem::take(&mut self.demo) {
            // `demo_expand` temasızdır: düğme ve sonuçlar burada kurulur.
            self.add = add_button(t, self.open);
            self.refresh_results(t);
        }
        let list_h = if self.rows.is_empty() {
            self.empty.set_max_width(Some(width - px * 2.0));
            self.empty.measure(env.text).height + py * 2.0
        } else {
            self.rows.len() as f32 * row_h
        };
        list_h + row_h + self.picker_height(env, width) + self.note_height(env, width)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let row_h = Self::row_h(t);
        let bs = self.reset.preferred_size(t);
        let bs = Size::new(bs.width * 0.8, bs.height * 0.8);
        let mut y = rect.y;
        let n = self.rows.len();
        for (i, r) in self.rows.iter_mut().enumerate() {
            r.rect = Rect::new(rect.x, y, rect.width, row_h);
            let cy = y + row_h * 0.5;
            let mut x = rect.right() - px - bs.width;
            for (shown, b) in [(n > 1, &mut r.remove), (i + 1 < n, &mut r.down), (i > 0, &mut r.up)] {
                b.set_rect(Rect::new(x, cy - bs.height * 0.5, bs.width, bs.height));
                if shown {
                    x -= bs.width + t.space(1);
                }
            }
            y += row_h;
        }
        if n == 0 {
            y += self.empty.measure(env.text).height + py * 2.0;
        }
        // Ekle satırı: solda düğme (ya da "en fazla" notu), sağda ↺. Seçici açıkken aynı satır:
        // solda arama alanı, sağında Vazgeç (↺ gizli).
        self.add_rect = Rect::new(rect.x, y, rect.width, row_h);
        let cy = y + row_h * 0.5;
        let s = self.add.preferred_size(env.text, t);
        if self.open {
            // Metin düğmesinin iç boşluğu kadar taşar: etiketi içerik kenarına hizalı.
            let bx = rect.right() - px - s.width + t.space(3);
            self.add.set_rect(Rect::new(bx, cy - s.height * 0.5, s.width, s.height));
            let w = (bx - t.space(2) - (rect.x + px)).max(0.0);
            let ih = self.search.preferred_size(t, w).height;
            self.search.set_rect(Rect::new(rect.x + px, cy - ih * 0.5, w, ih));
        } else {
            self.add.set_rect(Rect::new(rect.x + px - t.space(3), cy - s.height * 0.5, s.width, s.height));
        }
        self.reset.set_rect(Rect::new(rect.right() - px - bs.width, cy - bs.height * 0.5, bs.width, bs.height));
        y += row_h;
        if self.open {
            for r in &mut self.results {
                r.rect = Rect::new(rect.x, y, rect.width, row_h);
                y += row_h;
            }
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        // Satır düğmeleri: işlem anında yazılır (geri al şeridi App'te).
        let mut write: Option<Option<Vec<KbLayout>>> = None;
        let n = self.rows.len();
        for (i, r) in self.rows.iter_mut().enumerate() {
            if i > 0 {
                let x = r.up.pointer(e, now);
                repaint |= x.repaint;
                if x.event.is_some() {
                    write = Some(moved(&self.list, i, true));
                }
            }
            if i + 1 < n {
                let x = r.down.pointer(e, now);
                repaint |= x.repaint;
                if x.event.is_some() {
                    write = Some(moved(&self.list, i, false));
                }
            }
            if n > 1 {
                let x = r.remove.pointer(e, now);
                repaint |= x.repaint;
                if x.event.is_some() {
                    write = Some(without(&self.list, i));
                }
            }
        }
        if let Some(Some(list)) = write {
            self.write(env, Some(list));
            return true;
        }
        if self.list.len() < MAX_LAYOUTS || self.open {
            let r = self.add.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                let open = !self.open;
                self.set_open(env, open);
                return true;
            }
        }
        if self.show_reset && !self.open {
            let r = self.reset.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.write(env, None);
                return true;
            }
        }
        if self.open {
            let r = self.search.pointer(env.text, env.theme, e, now);
            repaint |= r.repaint;
            if let PointerKind::Press(MouseButton::Left) = e.kind
                && self.search.rect().contains(e.position)
            {
                env.push(Action::Focus);
            }
            let mut picked = None;
            for (i, row) in self.results.iter_mut().enumerate() {
                let tap = row.hover.handle(row.rect, e, now);
                repaint |= tap.repaint;
                if tap.clicked {
                    picked = Some(i);
                }
            }
            if let Some(i) = picked {
                self.pick(env, i);
                return true;
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released {
            return false;
        }
        if !self.open {
            if matches!(e.key, Key::Enter | Key::Space) && self.list.len() < MAX_LAYOUTS {
                self.set_open(env, true);
                return true;
            }
            return false;
        }
        match e.key {
            Key::Down => {
                self.cursor = (self.cursor + 1).min(self.results.len().saturating_sub(1));
                return true;
            }
            Key::Up => {
                self.cursor = self.cursor.saturating_sub(1);
                return true;
            }
            _ => {}
        }
        let r = self.search.key(e, env.now);
        match r.event {
            Some(TextInputEvent::Submit) => {
                let i = self.cursor;
                self.pick(env, i);
                true
            }
            Some(TextInputEvent::Cancel) => {
                self.set_open(env, false);
                true
            }
            Some(TextInputEvent::Changed) => {
                self.cursor = 0;
                self.refresh_results(env.theme);
                true
            }
            _ => r.repaint,
        }
    }

    fn captures_keys(&self) -> bool {
        // Seçici açıkken oklar ve Escape bloğun (sayfa gezinmesine gitmez).
        self.open
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, now: Instant) {
        self.focused = focused;
        self.focus_visible = focused && visible;
        self.add.set_focus_visible(self.focus_visible && !self.open);
        self.search.set_focused(focused && self.open, now);
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        self.open.then_some(&mut self.search)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        let n = self.rows.len();
        for (i, r) in self.rows.iter().enumerate() {
            let c = [(i > 0, &r.up), (i + 1 < n, &r.down), (n > 1, &r.remove)].into_iter().filter(|(s, _)| *s).find_map(|(_, b)| b.cursor(p));
            if c.is_some() {
                return c;
            }
        }
        if let Some(c) = self.add.cursor(p) {
            return Some(c);
        }
        if self.show_reset && !self.open && let Some(c) = self.reset.cursor(p) {
            return Some(c);
        }
        if self.open {
            if let Some(c) = self.search.cursor(p) {
                return Some(c);
            }
            if self.results.iter().any(|r| r.rect.contains(p)) {
                return Some(CursorShape::Pointer);
            }
        }
        None
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.theme, env.store, false);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        match event {
            // Katalog geldi: adlar ve sonuçlar; durum geldi: etkin liste ve uyarı.
            crate::services::Event::Xkb | crate::services::Event::Input => {
                self.refresh(env.theme, env.store, true);
                true
            }
            _ => false,
        }
    }

    fn demo_expand(&mut self, _now: Instant) {
        // Canlı test (`expand=input.keyboard.layouts`): seçiciyi aç, ilk sonuç vurgulu.
        if self.list.len() < MAX_LAYOUTS {
            self.open = true;
            self.demo = true;
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for r in &mut self.rows {
            r.name.set_style(text_style(theme, TextRole::Body));
            r.code.set_style(text_style(theme, TextRole::Caption));
            r.up.set_theme(theme, now);
            r.down.set_theme(theme, now);
            r.remove.set_theme(theme, now);
        }
        for r in &mut self.results {
            r.name.set_style(text_style(theme, TextRole::Body));
            r.code.set_style(text_style(theme, TextRole::Caption));
            r.hover.set_theme(theme, now);
        }
        self.empty.set_style(text_style(theme, TextRole::Body));
        self.full.set_style(text_style(theme, TextRole::Caption));
        self.status.set_style(text_style(theme, TextRole::Body));
        if let Some(n) = &mut self.note {
            n.set_theme(theme, now);
        }
        self.add.set_theme(theme, now);
        self.reset.set_theme(theme, now);
        self.search.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let n = self.rows.len();
        for (i, r) in self.rows.iter_mut().enumerate() {
            let cy = r.rect.center().y;
            let ns = r.name.measure_in(ui.painter);
            r.name.draw(ui.painter, Point::new(r.rect.x + px, cy - ns.height * 0.5), ui.fade(c.on_surface));
            let cs = r.code.measure_in(ui.painter);
            r.code.draw(ui.painter, Point::new(r.rect.x + px + ns.width + t.space(3), cy - cs.height * 0.5), ui.fade(c.on_surface_variant));
            if i > 0 {
                r.up.draw(ui);
            }
            if i + 1 < n {
                r.down.draw(ui);
            }
            if n > 1 {
                r.remove.draw(ui);
            }
        }
        if n == 0 {
            self.empty.draw(ui.painter, Point::new(self.rect.x + px, self.rect.y + py), ui.fade(c.on_surface_variant));
        }
        if self.list.len() < MAX_LAYOUTS || self.open {
            self.add.draw(ui);
        } else {
            let s = self.full.measure_in(ui.painter);
            self.full.draw(ui.painter, Point::new(self.add_rect.x + px, self.add_rect.center().y - s.height * 0.5), ui.fade(c.on_surface_variant));
        }
        if self.show_reset && !self.open {
            self.reset.draw(ui);
        }
        let mut bottom = self.add_rect.bottom();
        if self.open {
            self.search.draw(ui);
            if self.results.is_empty() {
                self.status.draw(ui.painter, Point::new(self.rect.x + px, bottom), ui.fade(c.on_surface_variant));
                bottom += self.status.measure_in(ui.painter).height + t.space(2);
            }
            let now = ui.now;
            for (i, r) in self.results.iter_mut().enumerate() {
                let layer = Rect::new(r.rect.x + t.space(1), r.rect.y, r.rect.width - t.space(2), r.rect.height);
                r.hover.draw_layer(ui.painter, layer, Corners::uniform(t.shape.radius_medium * 0.5), t, now);
                if r.hover.is_animating(now) {
                    ui.request_frame();
                }
                if i == self.cursor && self.focus_visible {
                    ui.painter.rect(layer, &nuxut::RectStyle {
                        fill: ui.fade(state_layer_under(t, c.primary, 0.12, &[], &[c.on_surface, c.on_surface_variant])),
                        radii: Corners::uniform(t.shape.radius_medium * 0.5),
                        border: None,
                    });
                }
                let cy = r.rect.center().y;
                let ns = r.name.measure_in(ui.painter);
                r.name.draw(ui.painter, Point::new(r.rect.x + px, cy - ns.height * 0.5), ui.fade(c.on_surface));
                let cs = r.code.measure_in(ui.painter);
                r.code.draw(ui.painter, Point::new(r.rect.x + px + ns.width + t.space(3), cy - cs.height * 0.5), ui.fade(c.on_surface_variant));
                bottom = r.rect.bottom();
            }
            bottom += py;
        }
        if let Some(note) = &mut self.note {
            let w = self.rect.width - px * 2.0;
            let h = note.measure(ui.painter.text_system(), t, w).height;
            note.set_rect(Rect::new(self.rect.x + px, bottom, w, h));
            note.draw(ui);
        }
    }
}

// ---- Düzenler arası geçiş -------------------------------------------------------------------

#[derive(Debug)]
pub struct LayoutSwitch {
    desc: &'static SettingDesc,
    title: Label,
    subtitle: Label,
    /// `(seçenek adı, etiket)`; ilki "Yok" (`""`).
    choices: Vec<(String, String)>,
    select: Box<Select>,
    reset: IconButton,
    show_reset: bool,
    note: Option<Notice>,
    stack: bool,
    rect: Rect,
}

/// Seçim listesi: "Yok", katalogdaki `grp:` seçenekleri, katalogda olmayan etkin seçenek.
fn switch_choices(cat: Option<&XkbCatalog>, current: Option<&str>) -> Vec<(String, String)> {
    let mut out = vec![(String::new(), t!("settings-keyboard-switch-none"))];
    if let Some(cat) = cat {
        out.extend(cat.switch_options.iter().map(|o| (o.name.clone(), if o.description.is_empty() { o.name.clone() } else { o.description.clone() })));
    }
    if let Some(c) = current
        && !out.iter().any(|(n, _)| n == c)
    {
        out.push((c.to_owned(), c.to_owned()));
    }
    out
}

impl LayoutSwitch {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut s = Self {
            desc,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(desc.description(), text_style(theme, TextRole::Caption)),
            choices: Vec::new(),
            select: Box::new(Select::new(theme, vec![t!("settings-keyboard-switch-none")], 0)),
            reset: IconButton::new(theme, Icon::named(IconName::Restart), IconButtonVariant::Standard),
            show_reset: false,
            note: None,
            stack: false,
            rect: Rect::ZERO,
        };
        s.refresh(theme, store);
        s
    }

    fn refresh(&mut self, theme: &Theme, store: &Store) {
        let options = current_options(store);
        let current = current_switch(&options).map(str::to_owned);
        let choices = switch_choices(xkb::catalog().as_deref(), current.as_deref());
        let selected = choices.iter().position(|(n, _)| Some(n.as_str()) == current.as_deref()).unwrap_or(0);
        if choices != self.choices {
            *self.select = Select::new(theme, choices.iter().map(|(_, l)| l.clone()).collect(), selected);
            self.choices = choices;
        } else if !self.select.is_open() {
            self.select.set_selected(selected);
        }
        self.show_reset = store.get(FileId::Shell, &KeyPath::parse(OPTIONS_KEY)).is_some();
        let state = input::status();
        let note = state.ready().and_then(|s| s.override_for(OPTIONS_KEY)).map(input::override_note);
        self.note = note.map(|n| override_notice(theme, &n));
    }

    fn choose(&mut self, env: &mut Env, i: usize) {
        let Some((name, _)) = self.choices.get(i) else { return };
        let grp = (!name.is_empty()).then_some(name.as_str());
        let list = options_with_switch(&current_options(env.store), grp);
        env.set(self.desc.label(), FileId::Shell, KeyPath::parse(OPTIONS_KEY), Some(options_value(&list)));
    }

    fn select_width(theme: &Theme, width: f32) -> f32 {
        (width * 0.4).clamp(theme.space(40), theme.space(70))
    }
}

impl Block for LayoutSwitch {
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
        let (px, py) = row_padding(t);
        let sw = Self::select_width(t, width);
        let ss = self.select.preferred_size(t, sw);
        self.stack = crate::rows::should_stack(t, env.text, width, ss.width, &mut self.title, Some(&mut self.subtitle));
        let text_w = if self.stack { crate::rows::stacked_text_width(t, width) } else { side_text_width(t, width, ss.width) };
        self.title.set_max_width(Some(text_w));
        self.subtitle.set_max_width(Some(text_w));
        let text_h = self.title.measure(env.text).height + t.space(1) + self.subtitle.measure(env.text).height;
        let head = if self.stack { text_h + t.space(2) + ss.height + py * 2.0 } else { row_height_for(t, text_h.max(ss.height)) };
        let note_h = match &mut self.note {
            Some(n) => n.measure(env.text, t, width - px * 2.0).height + py * 0.5,
            None => 0.0,
        };
        head + note_h
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let sw = Self::select_width(t, rect.width);
        let ss = self.select.preferred_size(t, sw);
        let bs = self.reset.preferred_size(t);
        let bs = Size::new(bs.width * 0.8, bs.height * 0.8);
        if self.stack || is_narrow(t, rect.width) {
            let th = self.title.measure(env.text).height + t.space(1) + self.subtitle.measure(env.text).height;
            self.select.set_rect(t, Rect::new(rect.x + px, rect.y + py + th + t.space(2), rect.width - px * 2.0, ss.height));
            let ty = rect.y + py + self.title.measure(env.text).height * 0.5;
            self.reset.set_rect(Rect::new(rect.right() - px - bs.width, ty - bs.height * 0.5, bs.width, bs.height));
        } else {
            let th = self.title.measure(env.text).height + t.space(1) + self.subtitle.measure(env.text).height;
            let head = row_height_for(t, th.max(ss.height));
            let cy = rect.y + head * 0.5;
            let sel = Rect::new(rect.right() - px - ss.width, cy - ss.height * 0.5, ss.width, ss.height);
            self.select.set_rect(t, sel);
            self.reset.set_rect(Rect::new(sel.x - t.space(2) - bs.width, cy - bs.height * 0.5, bs.width, bs.height));
        }
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
        let mut repaint = false;
        if self.show_reset {
            let r = self.reset.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                env.set(self.desc.label(), FileId::Shell, KeyPath::parse(OPTIONS_KEY), None);
                return true;
            }
        }
        let r = self.select.pointer(env.text, env.theme, e, now);
        repaint |= r.repaint;
        if self.select.is_open() {
            env.push(Action::Focus);
        }
        repaint
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
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.select.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.show_reset && let Some(c) = self.reset.cursor(p) {
            return Some(c);
        }
        self.select.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.theme, env.store);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        match event {
            crate::services::Event::Xkb | crate::services::Event::Input => {
                self.refresh(env.theme, env.store);
                true
            }
            _ => false,
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        if let Some(n) = &mut self.note {
            n.set_theme(theme, now);
        }
        self.select.set_theme(theme, now);
        self.reset.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let rect = self.rect;
        let th = self.title.measure_in(ui.painter).height;
        let sh = self.subtitle.measure_in(ui.painter).height;
        let ty = if self.stack {
            rect.y + py
        } else {
            let sel = self.select.rect();
            let head = (sel.center().y - rect.y) * 2.0;
            rect.y + (head - th - sh - t.space(1)) * 0.5
        };
        self.title.draw(ui.painter, Point::new(rect.x + px, ty), ui.fade(c.on_surface));
        self.subtitle.draw(ui.painter, Point::new(rect.x + px, ty + th + t.space(1)), ui.fade(c.on_surface_variant));
        if self.show_reset {
            self.reset.draw(ui);
        }
        if let Some(n) = &mut self.note {
            let w = rect.width - px * 2.0;
            let nh = n.measure(ui.painter.text_system(), t, w).height;
            n.set_rect(Rect::new(rect.x + px, rect.bottom() - py - nh, w, nh));
            n.draw(ui);
        }
        self.select.draw(ui);
        // Menü kaplamada (satırın kırpmasından bağımsız).
        self.select.set_bounds(ui.surface_rect());
        self.select.draw_overlay(ui);
    }
}

// ---- Deneme alanı ---------------------------------------------------------------------------

/// Tekrar gecikmesi ve hızını denemek için yerel metin alanı. Yazılan hiçbir yere gitmez;
/// tekrarı compositor üretir, yani alan çalışan masaüstünün gerçek ayarını gösterir.
#[derive(Debug)]
pub struct KeyTest {
    desc: &'static SettingDesc,
    input: TextInput,
    rect: Rect,
}

impl KeyTest {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        Self { desc, input: TextInput::new(theme).with_placeholder(t!("settings-keyboard-test-placeholder")).with_max_bytes(512), rect: Rect::ZERO }
    }
}

impl Block for KeyTest {
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
        let (px, py) = row_padding(env.theme);
        self.input.preferred_size(env.theme, width - px * 2.0).height + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let (px, py) = row_padding(env.theme);
        let h = self.input.preferred_size(env.theme, rect.width - px * 2.0).height;
        self.input.set_rect(Rect::new(rect.x + px, rect.y + py, rect.width - px * 2.0, h));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.input.pointer(env.text, env.theme, e, env.now);
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && self.input.rect().contains(e.position)
        {
            env.push(Action::Focus);
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.input.key(e, env.now);
        match r.event {
            // Esc temizler; Enter bir şey yazmaz.
            Some(TextInputEvent::Cancel) => {
                self.input.set_text("");
                true
            }
            Some(_) => true,
            None => r.repaint,
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        self.input.set_focused(focused, now);
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        Some(&mut self.input)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.input.cursor(p)
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.input.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.input.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kb(l: &str, v: &str) -> KbLayout {
        KbLayout::new(l, v)
    }

    #[test]
    fn layouts_start_from_effective_unless_written() {
        let tr = vec![kb("tr", "")];
        // Yazılmamış: etkin liste (kullanıcının `tr`'si).
        assert_eq!(current_layouts_with(None, Some(&tr)), tr);
        // Dosya önce gelir.
        let written = layouts_value(&[kb("us", "intl"), kb("tr", "f")]);
        assert_eq!(current_layouts_with(Some(&written), Some(&tr)), vec![kb("us", "intl"), kb("tr", "f")]);
        // Bozuk dosya değeri: etkin listeye düşer (shell de yok sayar).
        let bad: Value = Value::from("tr");
        assert_eq!(current_layouts_with(Some(&bad), Some(&tr)), tr);
        assert!(current_layouts_with(None, None).is_empty());
        // İlk ekleme etkin listeden başlar: ["tr","us"].
        let added = with_added(&current_layouts_with(None, Some(&tr)), kb("us", "")).unwrap();
        assert_eq!(added, vec![kb("tr", ""), kb("us", "")]);
        let v = layouts_value(&added);
        assert_eq!(v.to_string().trim(), r#"[{ layout = "tr", variant = "" }, { layout = "us", variant = "" }]"#);
        assert_eq!(parse_layouts(&v), Some(added));
    }

    #[test]
    fn at_most_four_layouts_at_least_one() {
        let mut list = vec![kb("tr", "")];
        for l in ["us", "de", "fr"] {
            list = with_added(&list, kb(l, "")).unwrap();
        }
        assert_eq!(list.len(), MAX_LAYOUTS);
        assert_eq!(with_added(&list, kb("ru", "")), None, "beşinci yok");
        assert_eq!(with_added(&list[..2], kb("us", "")), None, "aynısı iki kez yok");
        assert_eq!(with_added(&list[..1], kb("tr", "f")).unwrap().len(), 2, "aynı düzenin varyantı ayrı");
        assert_eq!(with_added(&[], kb("bad name", "")), None);
        assert_eq!(without(&list[..1], 0), None, "son düzen silinmez");
        assert_eq!(without(&list, 1).unwrap(), vec![kb("tr", ""), kb("de", ""), kb("fr", "")]);
        assert_eq!(moved(&list, 1, true).unwrap()[0], kb("us", ""));
        assert_eq!(moved(&list, 0, true), None);
        assert_eq!(moved(&list, 3, false), None);
        assert_eq!(moved(&list, 2, false).unwrap()[3], kb("de", ""));
        // Dosyada dörtten fazlası geçersiz.
        let five = layouts_value(&[kb("a", ""), kb("b", ""), kb("c", ""), kb("d", ""), kb("e", "")]);
        assert_eq!(parse_layouts(&five), None);
    }

    #[test]
    fn switch_option_merges_and_keeps_other_options() {
        let base = vec!["caps:escape".to_owned(), "grp:alt_shift_toggle".to_owned(), "compose:ralt".to_owned()];
        assert_eq!(current_switch(&base), Some("grp:alt_shift_toggle"));
        assert_eq!(options_with_switch(&base, Some("grp:win_space_toggle")), vec!["caps:escape", "compose:ralt", "grp:win_space_toggle"]);
        assert_eq!(options_with_switch(&base, None), vec!["caps:escape", "compose:ralt"], "Yok: yalnızca grp: gider");
        assert_eq!(options_with_switch(&[], Some("grp:alt_shift_toggle")), vec!["grp:alt_shift_toggle"]);
        assert_eq!(options_with_switch(&base, Some("caps:swapescape")), vec!["caps:escape", "compose:ralt"], "grp: olmayan seçim yazılmaz");
        // Temel liste: dosya > etkin.
        let eff = vec!["caps:escape".to_owned()];
        assert_eq!(current_options_with(None, Some(&eff)), eff);
        let file = options_value(&["grp:caps_toggle".to_owned()]);
        assert_eq!(current_options_with(Some(&file), Some(&eff)), vec!["grp:caps_toggle"]);
        assert!(current_options_with(None, None).is_empty());
    }

    const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<xkbConfigRegistry version="1.1">
  <layoutList>
    <layout><configItem><name>tr</name><shortDescription>tr</shortDescription><description>Turkish</description>
      <languageList><iso639Id>tur</iso639Id></languageList></configItem>
      <variantList><variant><configItem><name>f</name><description>Turkish (F)</description></configItem></variant></variantList></layout>
    <layout><configItem><name>us</name><shortDescription>en</shortDescription><description>English (US)</description>
      <languageList><iso639Id>eng</iso639Id></languageList></configItem>
      <variantList><variant><configItem><name>intl</name><description>English (US, intl., with dead keys)</description></configItem></variant></variantList></layout>
    <layout><configItem popularity="exotic"><name>brai</name><description>Braille</description></configItem></layout>
  </layoutList>
  <optionList><group><configItem><name>grp</name><description>Switching to another layout</description></configItem>
    <option><configItem><name>grp:alt_shift_toggle</name><description>Alt+Shift</description></configItem></option></group></optionList>
</xkbConfigRegistry>"#;

    #[test]
    fn picker_lists_catalog_minus_present_and_labels_by_language() {
        crate::i18n::use_turkish_for_tests();
        let cat = xkb::parse(XML).unwrap();
        let present = vec![kb("tr", "")];
        let all = picker_results(&cat, "", &present);
        assert_eq!(all, vec![kb("us", "")], "boş sorgu: mevcut ve nadir düzen yok");
        let f = picker_results(&cat, "turk", &present);
        assert!(f.contains(&kb("tr", "f")) && !f.contains(&kb("tr", "")), "{f:?}");
        assert_eq!(layout_label(Some(&cat), &kb("us", "intl")).1, "us · intl");
        assert!(layout_label(Some(&cat), &kb("tr", "")).0.contains("Turkish"));
        assert_eq!(layout_label(None, &kb("tr", "")), ("tr".to_owned(), "tr".to_owned()), "katalog yokken kod");
        let choices = switch_choices(Some(&cat), Some("grp:menu_toggle"));
        assert_eq!(choices[0].1, "Yok");
        assert!(choices.iter().any(|(n, l)| n == "grp:alt_shift_toggle" && l == "Alt+Shift"));
        assert!(choices.iter().any(|(n, _)| n == "grp:menu_toggle"), "katalogda olmayan etkin seçenek de seçilebilir");
    }
}
