//! Kısayol satırı (`keybinds.<eylem>`): solda eylem adı, sağda tuş başlıklarıyla kısayol
//! alanı. Alana tıkla (ya da odakta Enter) → "Tuş birleşimine bas" → basılan birleşim
//! yazılır; Escape iptal, Geri silme atanmamış yapar. Yanında temizle ve varsayılana dön.
//! Aynı birleşim iki eylemdeyse satır altında uyarı (shell sıradaki ilkini tutar).
//! Ayrıştırma ve yakalama: `crate::keybind`.
//!
//! Hyprland ayarında aynı tuş bağlıysa (`msg hypr status`, [`hypr_status::key`]) satır bunu
//! söyler ve tek tıkla seçtirir (2026-10-17): kullanıcınınki çalışıyorsa "Nuxut'unkini kullan",
//! Nuxut'unki çalışıyorsa ya da devralınamıyorsa "Benimkini koru" → `[keybind_owner] <eylem>`.
//! Düğmeye Tab ile ulaşılır (alandan sonra).

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{FOCUS_RING, focus_ring, state_layer_under};
use nuxut::ui::{Button, ButtonVariant, Icon, IconButton, IconButtonVariant, IconName, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{Border, Corners, Point, Rect, RectStyle, Size, TextSystem, Theme};
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::keybind::{self, Binding, Capture, Shape};
use crate::registry::{self, SettingDesc};
use crate::rows::row_padding;
use crate::services::hypr_status::{self, KeyRelation};
use crate::store::{KeyPath, Store};

/// Shell'in `shell.toml` değişimini uygulayıp Hyprland'e yazması için beklenen süre.
const APPLY_SETTLE: Duration = Duration::from_millis(700);

/// Çakışan tuşta seçilecek taraf (`[keybind_owner]` değeri).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    Nuxut,
    User,
}

impl Owner {
    fn value(self) -> &'static str {
        match self {
            Owner::Nuxut => "nuxut",
            Owner::User => "user",
        }
    }
}

/// Bu tuşta kimin kısayolu çalışıyor (ekran okuyucu açıklaması).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Holder {
    /// Nuxut'unki (çakışma yok ya da devralındı).
    Nuxut,
    /// Kullanıcının Hyprland ayarındaki.
    User,
    /// Nuxut'unki isteniyor ama devralınamıyor.
    Blocked,
}

impl Holder {
    fn text(self) -> String {
        match self {
            Holder::Nuxut => t!("settings-a11y-keybind-holder-nuxut"),
            Holder::User => t!("settings-a11y-keybind-holder-user"),
            Holder::Blocked => t!("settings-a11y-keybind-holder-blocked"),
        }
    }
}

/// Hyprland ilişkisinden durum satırı ve düğmenin seçeceği taraf.
fn relation_status(k: &hypr_status::KeyConflict) -> (Status, Option<Owner>) {
    let place = k.place().map(|p| format!(" ({p})")).unwrap_or_default();
    match &k.relation {
        KeyRelation::UserKept { can_take, .. } => {
            (Status::Hint(t!("settings-keybinds-user-kept", place = place)), can_take.then_some(Owner::Nuxut))
        }
        KeyRelation::Taken => (Status::Hint(t!("settings-keybinds-taken", place = place)), Some(Owner::User)),
        KeyRelation::Blocked { reason } => {
            let why = match reason.as_deref() {
                Some("read_after") => t!("settings-keybinds-blocked-read-after"),
                Some("not_loaded") => t!("settings-keybinds-blocked-not-loaded"),
                Some("special_mods") => t!("settings-keybinds-blocked-special-mods"),
                _ => t!("settings-keybinds-blocked-other"),
            };
            (Status::Warn(t!("settings-keybinds-blocked", place = place, why = why)), Some(Owner::User))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    None,
    Hint(String),
    Warn(String),
}

#[derive(Debug)]
pub struct KeybindRow {
    desc: &'static SettingDesc,
    /// Değer biçimi (dokunma, rakam ailesi, fare: yakalama ve gösterim buna göre).
    shape: Shape,
    /// Yakalamada şimdiye dek basılı tutulan değiştiricilerin birleşimi (yalnız değiştirici
    /// biçimlerinde hepsi bırakılınca yazılır).
    peak: u8,
    title: Label,
    status_label: Label,
    status: Status,
    /// Gösterilen tuş başlıkları (atanmış birleşim ya da yakalanırken basılı değiştiriciler).
    caps: Vec<Label>,
    /// Başlık yoksa alanın metni ("Atanmamış", "Tuşlara bas…").
    empty: Label,
    field: Rect,
    field_hover: Interaction,
    clear: IconButton,
    reset: IconButton,
    show_clear: bool,
    show_reset: bool,
    capturing: bool,
    held: u8,
    /// Yakalama sırasında son reddedilen tuşun açıklaması.
    capture_note: Option<String>,
    focus_visible: bool,
    /// İmleç satırın üstünde: temizle düğmesi yalnızca o zaman (on üç × gürültü olmasın).
    hovered: bool,
    rect: Rect,
    /// Hyprland çakışmasında "Nuxut'unkini kullan" / "Benimkini koru" ve seçeceği taraf.
    owner_button: Button,
    owner: Option<Owner>,
    /// Bu tuşta kimin kısayolu çalışıyor (ekran okuyucu açıklaması).
    holder: Holder,
    /// Klavye odağı bu satırda ve düğmede mi (değilse alanda).
    focused: bool,
    on_button: bool,
}

impl KeybindRow {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let mut row = Self {
            desc,
            shape: keybind::shape_of(desc.path.strip_prefix("keybinds.").unwrap_or(desc.path)),
            peak: 0,
            title: Label::new(desc.label(), text_style(theme, TextRole::Body)),
            status_label: Label::new("", text_style(theme, TextRole::Caption)),
            status: Status::None,
            caps: Vec::new(),
            empty: Label::new("", text_style(theme, TextRole::Label)),
            field: Rect::ZERO,
            field_hover: Interaction::new(theme),
            clear: IconButton::new(theme, Icon::named(IconName::Close), IconButtonVariant::Standard),
            reset: IconButton::new(theme, Icon::named(IconName::Restart), IconButtonVariant::Standard),
            show_clear: false,
            show_reset: false,
            capturing: false,
            held: 0,
            capture_note: None,
            focus_visible: false,
            hovered: false,
            rect: Rect::ZERO,
            owner_button: Button::new(theme, ButtonVariant::Tonal).with_label(theme, ""),
            owner: None,
            holder: Holder::Nuxut,
            focused: false,
            on_button: false,
        };
        row.refresh(theme, store);
        row
    }

    fn action_name(&self) -> &'static str {
        self.desc.path.strip_prefix("keybinds.").unwrap_or(self.desc.path)
    }

    /// "Nuxut'unkini kullan" / "Benimkini koru": `[keybind_owner] <eylem>` yazılır, shell uygulayınca
    /// durum yeniden sorulur.
    fn choose(&mut self, env: &mut Env, owner: Owner) {
        let path = KeyPath::parse(&format!("keybind_owner.{}", self.action_name()));
        env.set(self.desc.label(), self.desc.file, path, Some(Value::from(owner.value())));
        env.push(Action::Seal);
        hypr_status::refresh(APPLY_SETTLE);
    }

    fn sync_rings(&mut self) {
        if self.owner.is_none() {
            self.on_button = false;
        }
        self.owner_button.set_focus_visible(self.focus_visible && self.on_button);
    }

    fn clear_visible(&self) -> bool {
        self.show_clear && (self.hovered || self.focus_visible)
    }

    fn binding(&self, store: &Store) -> Binding {
        keybind::binding_shaped(self.shape, &self.desc.string(store))
    }

    fn default_text(&self, store: &Store) -> &'static str {
        self.desc.default_value(store).and_then(Value::as_str).unwrap_or("")
    }

    /// Store'dan (ya da yakalama durumundan) başlıkları, düğmeleri ve durum satırını kurar.
    fn refresh(&mut self, theme: &Theme, store: &Store) {
        let binding = self.binding(store);
        let style = text_style(theme, TextRole::Label);
        let (caps, empty) = if self.capturing {
            let caps = keybind::mod_caps(self.held);
            let empty = if caps.is_empty() { t!("settings-keybinds-capturing") } else { "…".to_owned() };
            (caps, empty)
        } else {
            match &binding {
                Binding::Assigned(c) => {
                    let mut caps = c.caps();
                    if self.shape == Shape::Digits {
                        caps.push("1…0".to_owned());
                    }
                    (caps, String::new())
                }
                Binding::Unassigned | Binding::Invalid(_) => (Vec::new(), t!("settings-keybinds-unassigned")),
            }
        };
        if self.caps.len() != caps.len() {
            self.caps = caps.iter().map(|c| Label::new(c.as_str(), style.clone())).collect();
        } else {
            for (l, c) in self.caps.iter_mut().zip(&caps) {
                l.set_text(c);
            }
        }
        self.empty.set_text(&empty);
        self.show_clear = !self.capturing && matches!(binding, Binding::Assigned(_) | Binding::Invalid(_));
        let is_default = store.get(self.desc.file, &self.desc.key()).is_none() || same_combo(self.shape, &self.desc.string(store), self.default_text(store));
        self.show_reset = !self.capturing && !is_default;
        self.owner = None;
        self.holder = Holder::Nuxut;
        self.status = if self.capturing {
            match &self.capture_note {
                Some(n) => Status::Warn(n.clone()),
                None => Status::Hint(match self.shape {
                    Shape::Tap => t!("settings-keybinds-hint-tap"),
                    Shape::Digits => t!("settings-keybinds-hint-digits"),
                    Shape::Key | Shape::Pointer => t!("settings-keybinds-hint"),
                }),
            }
        } else if let Binding::Invalid(e) = &binding {
            Status::Warn(invalid_text(e))
        } else if let Some(c) = clash_text(self.desc, store) {
            Status::Warn(c)
        } else if let Some(k) = hypr_status::key(self.action_name()).filter(|_| matches!(binding, Binding::Assigned(_))) {
            let (status, owner) = relation_status(&k);
            self.owner = owner;
            self.holder = match k.relation {
                KeyRelation::UserKept { .. } => Holder::User,
                KeyRelation::Taken => Holder::Nuxut,
                KeyRelation::Blocked { .. } => Holder::Blocked,
            };
            status
        } else {
            Status::None
        };
        if let Some(o) = self.owner {
            // Sakin bilgi satırında metin düğmesi; uyarıda (tuş çalışmıyor olabilir) tonal.
            self.owner_button.set_variant(if matches!(self.status, Status::Warn(_)) { ButtonVariant::Tonal } else { ButtonVariant::Text });
            self.owner_button.set_label(&match o {
                Owner::Nuxut => t!("settings-keybinds-use-nuxut"),
                Owner::User => t!("settings-keybinds-keep-mine"),
            });
        }
        self.sync_rings();
        match &self.status {
            Status::None => {
                self.status_label.set_text("");
            }
            Status::Hint(s) | Status::Warn(s) => {
                self.status_label.set_text(s);
            }
        }
    }

    fn start(&mut self, env: &mut Env) {
        // Fare kısayolu klavyeden yakalanmaz: yalnızca temizlenir ya da varsayılana döner.
        if self.shape == Shape::Pointer {
            self.status = Status::Hint(t!("settings-keybinds-pointer-hint"));
            self.status_label.set_text(&t!("settings-keybinds-pointer-hint"));
            return;
        }
        // Bind tuşunun hangi düzenden adlandırılacağı Hyprland ayarına bağlı: arka planda tazele.
        crate::services::hyprland::refresh();
        self.capturing = true;
        self.held = 0;
        self.peak = 0;
        self.capture_note = None;
        env.push(Action::Focus);
        self.refresh(env.theme, env.store);
    }

    fn stop(&mut self, theme: &Theme, store: &Store) {
        self.capturing = false;
        self.held = 0;
        self.capture_note = None;
        self.refresh(theme, store);
    }

    /// Dosyaya yazar: varsayılana eşitse anahtar silinir.
    fn write(&mut self, env: &mut Env, text: Option<String>) {
        let value = text.filter(|t| !same_combo(self.shape, t, self.default_text(env.store))).map(Value::from);
        env.set(self.desc.label(), self.desc.file, self.desc.key(), value);
        env.push(Action::Seal);
        hypr_status::refresh(APPLY_SETTLE);
    }

    fn finish(&mut self, env: &mut Env, combo: keybind::Combo) {
        self.stop(env.theme, env.store);
        if !matches!(self.binding(env.store), Binding::Assigned(ref cur) if *cur == combo) {
            self.write(env, Some(combo.to_string()));
        }
    }

    fn on_capture(&mut self, env: &mut Env, e: &KeyEvent) {
        let Some(c) = keybind::capture(e, crate::services::hyprland::resolve_binds_by_sym()) else { return };
        match c {
            Capture::Held(m) => {
                self.held = m;
                self.peak |= m;
                // Yalnız değiştirici biçimleri: hepsi bırakılınca basılı tutulanlar yazılır.
                if m == 0 && self.peak != 0 && matches!(self.shape, Shape::Tap | Shape::Digits) {
                    if self.shape == Shape::Tap && self.peak.count_ones() != 1 {
                        self.peak = 0;
                        self.capture_note = Some(t!("settings-keybinds-tap-single"));
                    } else {
                        let combo = keybind::Combo { mods: self.peak, key: String::new() };
                        return self.finish(env, combo);
                    }
                }
            }
            Capture::Cancel => return self.stop(env.theme, env.store),
            Capture::Clear => {
                self.stop(env.theme, env.store);
                return self.write(env, Some(String::new()));
            }
            // Rakam ailesi: basılan rakamdan yalnız değiştiriciler alınır; dokunmada tuş birleşimi olmaz.
            Capture::Done(combo) if self.shape == Shape::Digits && combo.mods != 0 => {
                return self.finish(env, keybind::Combo { mods: combo.mods, key: String::new() });
            }
            Capture::Done(_) if self.shape == Shape::Tap => {
                self.peak = 0;
                self.capture_note = Some(t!("settings-keybinds-tap-single"));
            }
            Capture::Done(combo) => return self.finish(env, combo),
            Capture::NeedsModifier(_) => self.capture_note = Some(t!("settings-keybinds-need-modifier")),
            Capture::Unknown => self.capture_note = Some(t!("settings-keybinds-unknown-key")),
        }
        self.refresh(env.theme, env.store);
    }

    fn caps_size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        let pad = theme.space(2);
        let gap = theme.space(1);
        let mut w = 0.0f32;
        let mut h = 0.0f32;
        for (i, l) in self.caps.iter_mut().enumerate() {
            let s = l.measure(text);
            let cap_h = s.height + theme.space(1) * 2.0;
            w += (s.width + pad * 2.0).max(cap_h) + if i > 0 { gap } else { 0.0 };
            h = h.max(cap_h);
        }
        if !self.empty.text().is_empty() {
            let s = self.empty.measure(text);
            w += s.width + if self.caps.is_empty() { 0.0 } else { gap };
            h = h.max(s.height);
        }
        Size::new(w, h)
    }

    fn field_size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        let c = self.caps_size(text, theme);
        Size::new((c.width + theme.space(4) * 2.0).max(theme.space(40)), theme.space(10))
    }

    fn icon_size(&self, theme: &Theme) -> Size {
        let s = self.reset.preferred_size(theme);
        Size::new(s.width * 0.8, s.height * 0.8)
    }

    fn text_width(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let field = self.field_size(env.text, t);
        let icons = (self.icon_size(t).width + t.space(1)) * 2.0;
        (width - px * 2.0 - field.width - icons - t.space(4)).max(t.space(20))
    }

    /// Seçim düğmesinin boyutu (düğme yoksa sıfır).
    fn button_size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        if self.owner.is_some() { self.owner_button.preferred_size(text, theme) } else { Size::ZERO }
    }

    /// Metin sütunu: başlık, (varsa) durum satırı ve seçim düğmesi.
    fn text_height(&mut self, text: &mut TextSystem, theme: &Theme) -> f32 {
        let mut h = self.title.measure(text).height;
        if self.status != Status::None {
            h += theme.space(1) + self.status_label.measure(text).height;
        }
        let b = self.button_size(text, theme);
        if b.height > 0.0 {
            h += theme.space(2) + b.height;
        }
        h
    }
}

/// İki dizge aynı kısayol mu (ikisi de boş ya da ayrıştırılmış birleşimleri eşit).
fn same_combo(shape: Shape, a: &str, b: &str) -> bool {
    match (keybind::binding_shaped(shape, a), keybind::binding_shaped(shape, b)) {
        (Binding::Unassigned, Binding::Unassigned) => true,
        (Binding::Assigned(x), Binding::Assigned(y)) => x == y,
        _ => a.trim() == b.trim(),
    }
}

/// Geçersiz kayıt uyarısı: her `ParseError` türü için ayrı katalog anahtarı.
fn invalid_text(e: &keybind::ParseError) -> String {
    match e {
        keybind::ParseError::Empty => t!("settings-keybinds-error-empty"),
        keybind::ParseError::TooLong => t!("settings-keybinds-error-too-long"),
        keybind::ParseError::UnknownModifier => t!("settings-keybinds-error-unknown-modifier"),
        keybind::ParseError::InvalidKey => t!("settings-keybinds-error-invalid-key"),
    }
}

/// Bu satırın çakışma uyarısı (shell sırasıyla: ilk eylem kazanır, sonraki çalışmaz).
fn clash_text(desc: &SettingDesc, store: &Store) -> Option<String> {
    let rows: Vec<&SettingDesc> = registry::keybind_rows().collect();
    let texts: Vec<String> = rows.iter().map(|d| d.string(store)).collect();
    let refs: Vec<(Shape, &str)> =
        rows.iter().zip(&texts).map(|(d, t)| (keybind::shape_of(d.path.strip_prefix("keybinds.").unwrap_or(d.path)), t.as_str())).collect();
    let me = rows.iter().position(|d| d.id == desc.id)?;
    let clash = keybind::clashes_shaped(&refs).get(me).copied().flatten()?;
    let other = rows.get(clash.other)?.title;
    Some(if clash.wins {
        t!("settings-keybinds-clash-wins", other = other)
    } else {
        t!("settings-keybinds-clash-loses", other = other)
    })
}

impl Block for KeybindRow {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    // Ekran okuyucu (framework ADR-0011): alan = eylem adı, değer = tuş, açıklama = kimin kısayolu
    // çalıştığı + durum satırı; seçim düğmesi ve geri al ayrı düğüm. AT eylemleri `app/a11y.rs`
    // kapısında silinir: düğmeler yalnız gerçek Tab/Enter ve işaretçiyle çalışır (AT kuralı).
    fn access(&self, tree: &mut nuxut::access::AccessTree) {
        use nuxut::access::Role;
        let title = self.title.text();
        let keys = if self.caps.is_empty() {
            self.empty.text().to_owned()
        } else {
            self.caps.iter().map(|l| l.text()).collect::<Vec<_>>().join(" + ")
        };
        let status = self.status_label.text();
        // Atanmamış tuşun sahibi yok: yalnız durum.
        let holder = if self.caps.is_empty() { String::new() } else { self.holder.text() };
        let desc = [holder, status.to_owned()].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(". ");
        let key = self.desc.id;
        tree.push(key, Role::Button)
            .label(title)
            .value(&keys)
            .description(&desc)
            .bounds(self.field)
            .clickable()
            .focused_if(self.focused && !self.on_button);
        if self.owner.is_some() {
            self.owner_button
                .access(tree, (key, "owner"))
                .description(&t!("settings-a11y-keybind-owner", setting = title))
                .focused_if(self.focused && self.on_button);
        }
        if self.clear_visible() {
            self.clear.access(tree, (key, "clear")).label(&t!("settings-a11y-keybind-clear", setting = title));
        }
        if self.show_reset {
            self.reset.access(tree, (key, "reset")).label(&t!("settings-a11y-reset", setting = title));
        }
    }

    fn visible(&self, store: &Store) -> bool {
        self.desc.is_visible(store)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let tw = self.text_width(env, width);
        self.title.set_max_width(Some(tw));
        self.status_label.set_max_width(Some(tw));
        self.text_height(env.text, t).max(self.field_size(env.text, t).height) + py * 2.0
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, _) = row_padding(t);
        self.rect = rect;
        let tw = self.text_width(env, rect.width);
        self.title.set_max_width(Some(tw));
        self.status_label.set_max_width(Some(tw));
        let fs = self.field_size(env.text, t);
        let cy = rect.center().y;
        self.field = Rect::new(rect.right() - px - fs.width, cy - fs.height * 0.5, fs.width, fs.height);
        // Seçim düğmesi metin sütununun altında (başlık + durum satırından sonra).
        let bs = self.button_size(env.text, t);
        if bs.width > 0.0 {
            let bottom = cy + self.text_height(env.text, t) * 0.5;
            // Metin düğmesinin yazısı satır metniyle hizalı (düğmenin iç boşluğu `space(3)`).
            let inset = if self.owner_button.variant() == ButtonVariant::Text { t.space(3) } else { 0.0 };
            self.owner_button.set_rect(Rect::new(rect.x + px - inset, bottom - bs.height, bs.width, bs.height));
        } else {
            self.owner_button.set_rect(Rect::ZERO);
        }
        let is = self.icon_size(t);
        let mut x = self.field.x - t.space(1);
        if self.show_clear {
            x -= is.width;
            self.clear.set_rect(Rect::new(x, cy - is.height * 0.5, is.width, is.height));
            x -= t.space(1);
        } else {
            self.clear.set_rect(Rect::ZERO);
        }
        if self.show_reset {
            x -= is.width;
            self.reset.set_rect(Rect::new(x, cy - is.height * 0.5, is.width, is.height));
        } else {
            self.reset.set_rect(Rect::ZERO);
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let hovered = !matches!(e.kind, PointerKind::Leave) && self.rect.contains(e.position);
        if hovered != self.hovered {
            self.hovered = hovered;
            repaint = true;
        }
        if self.clear_visible() {
            let r = self.clear.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.write(env, Some(String::new()));
                return true;
            }
        }
        if self.show_reset {
            let r = self.reset.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.write(env, None);
                return true;
            }
        }
        if let Some(o) = self.owner.filter(|_| !self.capturing) {
            let r = self.owner_button.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.choose(env, o);
                return true;
            }
        }
        // Yakalarken başka yere tıklamak iptal eder.
        if self.capturing
            && let PointerKind::Press(MouseButton::Left) = e.kind
            && !self.field.contains(e.position)
        {
            self.stop(env.theme, env.store);
            return true;
        }
        let tap = self.field_hover.handle(self.field, e, now);
        repaint |= tap.repaint;
        if tap.clicked && !self.capturing {
            self.start(env);
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if self.capturing {
            self.on_capture(env, e);
            return true;
        }
        if self.on_button && let Some(o) = self.owner {
            let r = self.owner_button.key(e, env.now);
            if r.event.is_some() {
                self.choose(env, o);
                return true;
            }
            return r.repaint;
        }
        if e.state != KeyState::Released && e.modifiers == Default::default() && matches!(e.key, Key::Enter | Key::Space) {
            self.start(env);
            return true;
        }
        false
    }

    fn captures_keys(&self) -> bool {
        self.capturing
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        // Yeni odak alanda başlar; Tab seçim düğmesine geçer ([`Block::focus_step`]).
        if focused && !self.focused {
            self.on_button = false;
        }
        self.focused = focused;
        self.focus_visible = focused && visible;
        self.sync_rings();
        if !focused && self.capturing {
            self.capturing = false;
            self.held = 0;
            self.capture_note = None;
            // Görünüm bir sonraki `sync`/`refresh`'te; burada store yok.
            self.empty.set_text("");
            self.status = Status::None;
            self.status_label.set_text("");
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.clear_visible() && let Some(c) = self.clear.cursor(p) {
            return Some(c);
        }
        if self.show_reset && let Some(c) = self.reset.cursor(p) {
            return Some(c);
        }
        if self.owner.is_some() && let Some(c) = self.owner_button.cursor(p) {
            return Some(c);
        }
        self.field.contains(p).then_some(CursorShape::Pointer)
    }

    fn focus_step(&mut self, forward: bool, _now: Instant) -> bool {
        if self.owner.is_none() || self.capturing || forward == self.on_button {
            return false;
        }
        self.on_button = forward;
        self.sync_rings();
        true
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh(env.theme, env.store);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::Hypr) {
            self.refresh(env.theme, env.store);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.status_label.set_style(text_style(theme, TextRole::Caption));
        self.empty.set_style(text_style(theme, TextRole::Label));
        for l in &mut self.caps {
            l.set_style(text_style(theme, TextRole::Label));
        }
        self.field_hover.set_theme(theme, now);
        self.clear.set_theme(theme, now);
        self.reset.set_theme(theme, now);
        self.owner_button.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let now = ui.now;
        let (px, _) = row_padding(t);
        // Başlık + durum satırı (+ seçim düğmesi) dikeyde ortalı.
        let th = self.title.measure_in(ui.painter).height;
        let sh = if self.status == Status::None { 0.0 } else { t.space(1) + self.status_label.measure_in(ui.painter).height };
        let bh = if self.owner.is_some() { t.space(2) + self.owner_button.rect().height } else { 0.0 };
        let y = self.rect.center().y - (th + sh + bh) * 0.5;
        self.title.draw(ui.painter, Point::new(self.rect.x + px, y), ui.fade(c.on_surface));
        let status_color = match self.status {
            Status::Warn(_) => c.error,
            _ => c.on_surface_variant,
        };
        if self.status != Status::None {
            self.status_label.draw(ui.painter, Point::new(self.rect.x + px, y + th + t.space(1)), ui.fade(status_color));
        }
        if self.owner.is_some() {
            self.owner_button.draw(ui);
        }
        if self.show_reset {
            self.reset.draw(ui);
        }
        if self.clear_visible() {
            self.clear.draw(ui);
        }
        // Alan: kenarsız tonal kuyu (kartın üstünde `surface`, §5.2); yakalarken vurgu katmanı,
        // yakalarken ya da klavye odağında (düğmede değilken) vurgu halkası.
        let f = self.field;
        let radii = Corners::uniform(f.height * 0.5);
        let active = self.capturing || (self.focus_visible && !self.on_button);
        let border = active.then(|| Border { width: FOCUS_RING, color: ui.fade(focus_ring(t, &[c.surface])) });
        ui.painter.rect(f, &RectStyle { fill: ui.fade(c.surface), radii, border });
        if self.capturing {
            ui.painter.rect(f, &RectStyle { fill: ui.fade(state_layer_under(t, c.primary, 0.08, &[c.surface], &[c.on_surface, c.on_surface_variant])), radii, border: None });
        }
        self.field_hover.draw_layer(ui.painter, f, radii, t, now);
        if self.field_hover.is_animating(now) {
            ui.request_frame();
        }
        // Tuş başlıkları alanda ortalı.
        let size = self.caps_size(ui.painter.text_system(), t);
        let mut x = f.center().x - size.width * 0.5;
        let cy = f.center().y;
        let pad = t.space(2);
        for l in &mut self.caps {
            let s = l.measure_in(ui.painter);
            let h = s.height + t.space(1) * 2.0;
            let w = (s.width + pad * 2.0).max(h);
            let cap = Rect::new(x, cy - h * 0.5, w, h);
            ui.painter.rect(cap, &RectStyle { fill: ui.fade(c.surface_bright), radii: t.shape.small(), border: None });
            l.draw(ui.painter, Point::new(cap.center().x - s.width * 0.5, cy - s.height * 0.5), ui.fade(c.on_surface));
            x += w + t.space(1);
        }
        if !self.empty.text().is_empty() {
            let s = self.empty.measure_in(ui.painter);
            let color = if self.capturing { c.primary } else { c.on_surface_variant };
            self.empty.draw(ui.painter, Point::new(x, cy - s.height * 0.5), ui.fade(color));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{FileId, KeyPath};

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    #[test]
    fn clash_warning_names_the_other_action() {
        crate::i18n::use_turkish_for_tests();
        let (_d, mut store) = store();
        let right = registry::by_id("keybinds.panel_right").unwrap();
        let left = registry::by_id("keybinds.panel_left").unwrap();
        assert!(clash_text(right, &store).is_none());
        store.set(FileId::Shell, &KeyPath::parse("keybinds.panel_right"), Some(Value::from("SUPER+N")));
        store.set(FileId::Shell, &KeyPath::parse("keybinds.panel_left"), Some(Value::from("super+n")));
        let r = clash_text(right, &store).unwrap();
        let l = clash_text(left, &store).unwrap();
        assert!(r.contains(left.title) && r.contains("orada"), "{r}");
        assert!(l.contains(right.title) && l.contains("burada"), "{l}");
        // Varsayılan medya tuşunu panele atamak çakışma doğurur.
        store.set(FileId::Shell, &KeyPath::parse("keybinds.panel_left"), Some(Value::from("XF86AudioMute")));
        assert!(clash_text(left, &store).unwrap().contains("Sesi kapat"));
    }

    /// Hyprland çakışması (2026-10-17): kullanıcınınki geçerliyse "Nuxut'unkini kullan", devralınmış
    /// ya da devralınamıyorsa "Benimkini koru"; düğme `[keybind_owner]` yazar ve Tab ile ulaşılır.
    #[test]
    fn hypr_conflict_offers_one_click_choice() {
        crate::i18n::use_turkish_for_tests();
        let text = r#"{"source":"present","conflicts":[
            {"action":"close_window","combo":"SUPER+q","file":"~/.config/hypr/hyprland/keybinds.conf","line":40,"reason":"user_wins","can_take":true},
            {"action":"clipboard","combo":"SUPER+SHIFT+v","file":"~/.config/hypr/custom/late.conf","line":1,"reason":"read_after","can_take":false}
        ],"taken_over":[{"action":"volume_up","combo":"xf86audioraisevolume","file":"~/k.conf","line":7,"reason":"default","can_take":true}]}"#;
        hypr_status::set_for_test(hypr_status::parse_status(text));
        let (_d, store) = store();
        let theme = Theme::default();
        let row = |id: &str| KeybindRow::new(&theme, registry::by_id(id).unwrap(), &store);

        let mut q = row("keybinds.close_window");
        assert_eq!(q.owner, Some(Owner::Nuxut));
        assert!(matches!(&q.status, Status::Hint(s) if s.contains("sizinki geçerli (keybinds.conf:40)")), "{:?}", q.status);
        let v = row("keybinds.clipboard");
        assert_eq!(v.owner, Some(Owner::User));
        assert!(matches!(&v.status, Status::Warn(s) if s.contains("sonra okunduğu") && s.contains("late.conf:1")), "{:?}", v.status);
        let up = row("keybinds.volume_up");
        assert_eq!(up.owner, Some(Owner::User));
        assert!(matches!(&up.status, Status::Hint(s) if s.contains("yerine Nuxut'unki")), "{:?}", up.status);
        assert_eq!(row("keybinds.media_next").owner, None, "çakışma yok: düğme yok");

        // Tab alandan düğmeye; Enter "Nuxut'unkini kullan" yazar.
        let now = Instant::now();
        q.set_focus(true, true, now);
        assert!(!q.on_button);
        assert!(q.focus_step(true, now) && q.on_button);
        assert!(!q.focus_step(true, now), "sonraki blok");
        let mut text_sys = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text_sys, now, actions: &mut actions };
        let enter = |state| KeyEvent { key: Key::Enter, state, text: None, modifiers: Default::default(), xkb: None };
        q.key(&mut env, &enter(KeyState::Pressed));
        q.key(&mut env, &enter(KeyState::Released));
        let written: Vec<(String, Option<String>)> = actions
            .iter()
            .filter_map(|a| match a {
                Action::Set { path, value, .. } => Some((path.to_string(), value.as_ref().and_then(Value::as_str).map(str::to_owned))),
                _ => None,
            })
            .collect();
        assert_eq!(written, [("keybind_owner.close_window".to_owned(), Some("nuxut".to_owned()))]);

        // Ekran okuyucu: eylem adı, tuş, kimin kısayolu ve durum; seçim düğmesi ayrı düğüm ve odakta.
        let mut tree = nuxut::access::AccessTree::new();
        q.access(&mut tree);
        let field = tree.node(tree.id_for("keybinds.close_window")).unwrap();
        assert_eq!(field.label(), Some("Pencereyi kapat"));
        assert_eq!(field.value(), Some("Super + Q"));
        let d = field.description().unwrap();
        assert!(d.starts_with("Hyprland ayarınızdaki kısayol geçerli. ") && d.contains("keybinds.conf:40"), "{d}");
        let owner = tree.node(tree.id_for(("keybinds.close_window", "owner"))).unwrap();
        assert_eq!(owner.label(), Some("Nuxut'unkini kullan"));
        assert_eq!(tree.focus(), tree.id_for(("keybinds.close_window", "owner")));
        let mut tree = nuxut::access::AccessTree::new();
        up.access(&mut tree);
        assert!(tree.node(tree.id_for("keybinds.volume_up")).unwrap().description().unwrap().starts_with("Nuxut'un kısayolu. "));
        hypr_status::set_for_test(None);
    }

    #[test]
    fn same_combo_compares_parsed() {
        assert!(same_combo(Shape::Key, "", " "));
        assert!(same_combo(Shape::Key, "xf86audiomute", "XF86AudioMute"));
        assert!(!same_combo(Shape::Key, "SUPER+N", ""));
    }

    #[test]
    fn writing_default_removes_key() {
        let (_d, store) = store();
        let theme = Theme::default();
        let desc = registry::by_id("keybinds.volume_up").unwrap();
        let mut row = KeybindRow::new(&theme, desc, &store);
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        row.write(&mut env, Some("xf86audioraisevolume".into()));
        row.write(&mut env, Some(String::new()));
        let values: Vec<Option<String>> = actions
            .iter()
            .filter_map(|a| match a {
                Action::Set { value, .. } => Some(value.as_ref().and_then(Value::as_str).map(str::to_owned)),
                _ => None,
            })
            .collect();
        assert_eq!(values, [None, Some(String::new())], "varsayılan silinir; boş = atanmamış yazılır");
    }
}
