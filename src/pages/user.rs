//! Kullanıcı hesabı sayfası (t118, `docs/design/account.md`): AccountsService'teki görünen ad ve
//! profil resmi. Değerler dosyada değil AccountsService'te ([`crate::services::accounts`]);
//! bloklar servis anlık görüntüsünden çizer, yazma isteğini servise verir (polkit yetkisi
//! sorulabilir; bekleme sırasında "Yetki bekleniyor…").
//!
//! - [`UserNote`]: kartsız üst not — hesap hizmeti yok / okunamadı.
//! - [`UserAvatar`]: büyük daire profil resmi (framework `Avatar`; resim yoksa baş harfler),
//!   görünen ad, kullanıcı adı ve hesap türü (salt okunur) + "Değiştir…" (dosya seçici).
//! - [`UserName`]: görünen ad alanı; Enter'da ya da odak çıkınca yazılır, boşsa yazılmaz.

use std::sync::Arc;
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::render::{ImageData, TextureOptions};
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::{Avatar, AvatarSize, Button, ButtonVariant, Label, Notice, NoticeKind, TextInput, TextInputEvent, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use super::datetime::RowText;
use super::notice::{notice_height, place_notice};
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{is_narrow, row_height_for, row_padding};
use crate::services::accounts::{self, AccountState, AccountType, ErrorKind, Op, OpState, Snapshot, User};
use crate::store::Store;

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// Sayfa üstü not.
pub fn note_text(s: &Snapshot) -> Option<String> {
    match &s.state {
        AccountState::Missing => Some(t!("settings-user-missing")),
        AccountState::Failed => Some(t!("settings-user-read-failed")),
        _ => None,
    }
}

/// Yazma hatasının metni.
pub fn error_text(kind: ErrorKind) -> String {
    match kind {
        ErrorKind::Missing => t!("settings-user-missing-short"),
        ErrorKind::BadImage => t!("settings-user-avatar-bad-image"),
        ErrorKind::Portal => t!("settings-user-avatar-portal"),
        ErrorKind::InvalidName => t!("settings-user-name-invalid"),
        ErrorKind::Face => t!("settings-user-face-failed"),
        // Yetki reddi/iptali hata olarak gösterilmez (servis `Idle` yapar); buraya gelmez.
        ErrorKind::NotAuthorized | ErrorKind::Other => t!("settings-user-failed"),
    }
}

/// Bir işlemin durum metni: bekleniyor / hata (`true` = hata rengi).
pub fn op_status(op: &OpState, which: Op) -> Option<(String, bool)> {
    if op.waiting(which) {
        return Some((t!("settings-user-waiting"), false));
    }
    op.error(which).map(|k| (error_text(k), true))
}

/// Hesap türü.
pub fn type_text(kind: AccountType) -> Option<String> {
    match kind {
        AccountType::Administrator => Some(t!("settings-user-type-admin")),
        AccountType::Standard => Some(t!("settings-user-type-standard")),
        AccountType::Unknown => None,
    }
}

/// Kimlik satırı: "hakan · Yönetici" (tür bilinmiyorsa yalnızca kullanıcı adı).
pub fn identity_text(user: &User) -> String {
    match type_text(user.account_type) {
        Some(kind) => t!("settings-user-identity", user = user.user_name.as_str(), kind = kind.as_str()),
        None => user.user_name.clone(),
    }
}

/// Başlıkta gösterilen ad: görünen ad, yoksa kullanıcı adı.
pub fn shown_name(user: &User) -> &str {
    if user.real_name.trim().is_empty() { &user.user_name } else { &user.real_name }
}

fn is_account(event: &crate::services::Event) -> bool {
    matches!(event, crate::services::Event::Account | crate::services::Event::LocaleChanged)
}

/// Profil kartı görünür mü: hizmet yok/okunamadıysa yalnızca üst not kalır (boş daire ve alan
/// gösterilmez). Okunurken (kısa an) kart yerinde durur, sayfa zıplamaz.
pub fn profile_shown(s: &Snapshot) -> bool {
    !matches!(s.state, AccountState::Missing | AccountState::Failed)
}

/// Düzenleme açık mı: servis hazır ve başka yazma sürmüyor.
fn editable(s: &Snapshot) -> bool {
    s.state.user().is_some() && !s.op.busy()
}

// ---- Üst not ------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct UserNote {
    desc: &'static SettingDesc,
    notice: Notice,
}

impl UserNote {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        accounts::ensure();
        let mut n = Self { desc, notice: Notice::new(theme, NoticeKind::Warning, "") };
        n.update();
        n
    }

    fn update(&mut self) {
        self.notice.set_text(&note_text(&accounts::snapshot()).unwrap_or_default());
    }
}

impl Block for UserNote {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        note_text(&accounts::snapshot()).is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        notice_height(&mut self.notice, env, width, 0.0)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        place_notice(&mut self.notice, env.theme, rect, 0.0);
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, _env: &mut Env) {
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if is_account(event) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.notice.draw(ui);
    }
}

// ---- Profil resmi ve kimlik ---------------------------------------------------------------------

/// Profil kartı avatar boyu: framework `XLarge` (96 px = `space(24)`; baş harf oranı korunur).
const PROFILE_AVATAR: AvatarSize = AvatarSize::XLarge;

#[derive(Debug)]
pub struct UserAvatar {
    desc: &'static SettingDesc,
    avatar: Avatar,
    name: Label,
    identity: Label,
    status: Label,
    status_error: bool,
    change: Button,
    /// Dokuya yüklenecek yeni resim (`draw`'da; boyacı orada). `Some(None)` = resmi kaldır.
    pending: Option<Option<Arc<nuxut_image::Image>>>,
    /// Son yüklenen resim (aynı `Arc` ise yeniden yüklenmez).
    shown: Option<Arc<nuxut_image::Image>>,
    rect: Rect,
}

impl UserAvatar {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        accounts::ensure();
        let mut b = Self {
            desc,
            avatar: Avatar::new(theme, "", PROFILE_AVATAR),
            name: Label::new("", text_style(theme, TextRole::Title).with_max_lines(1)),
            identity: Label::new("", text_style(theme, TextRole::Body).with_max_lines(1)),
            status: Label::new("", text_style(theme, TextRole::Caption)),
            status_error: false,
            change: Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-user-avatar-change")),
            pending: None,
            shown: None,
            rect: Rect::ZERO,
        };
        b.update(Instant::now());
        b
    }

    fn update(&mut self, now: Instant) {
        let s = accounts::snapshot();
        let (name, identity) = s.state.user().map(|u| (shown_name(u).to_owned(), identity_text(u))).unwrap_or_default();
        self.name.set_text(&name);
        self.identity.set_text(&identity);
        self.avatar.set_name(&name);
        let (status, error) = op_status(&s.op, Op::Avatar).unwrap_or_default();
        self.status.set_text(&status);
        self.status_error = error;
        self.change.set_enabled(editable(&s), now);
        let same = match (&s.avatar, &self.shown) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        if !same {
            self.pending = Some(s.avatar.clone());
        }
    }

    fn pick(&mut self) {
        if self.change.is_enabled() {
            accounts::pick_avatar();
        }
    }

    fn side(theme: &Theme) -> f32 {
        PROFILE_AVATAR.side(theme)
    }

    /// Metin sütunu genişliği (avatarın sağı, düğmenin solu).
    fn text_width(&mut self, env_text: &mut nuxut::TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let bs = self.change.preferred_size(env_text, theme);
        let button = if is_narrow(theme, width) { 0.0 } else { bs.width + theme.space(4) };
        (width - px * 2.0 - Self::side(theme) - theme.space(4) - button).max(theme.space(20))
    }
}

impl Block for UserAvatar {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        profile_shown(&accounts::snapshot())
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let tw = self.text_width(env.text, t, width);
        for l in [&mut self.name, &mut self.identity, &mut self.status] {
            l.set_max_width(Some(tw));
        }
        let mut text = self.name.measure(env.text).height + t.space(1) + self.identity.measure(env.text).height;
        if !self.status.text().is_empty() {
            text += t.space(1) + self.status.measure(env.text).height;
        }
        let mut h = row_height_for(t, text.max(Self::side(t)));
        if is_narrow(t, width) {
            // Dar: düğme avatarın altında, kendi satırında.
            h += self.change.preferred_size(env.text, t).height + row_padding(t).1;
        }
        h
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let side = Self::side(t);
        let bs = self.change.preferred_size(env.text, t);
        let narrow = is_narrow(t, rect.width);
        let head = if narrow { rect.height - bs.height - py } else { rect.height };
        self.avatar.set_center(Point::new(rect.x + px + side * 0.5, rect.y + head * 0.5));
        let br = if narrow {
            Rect::new(rect.x + px, rect.y + head - py * 0.5, bs.width, bs.height)
        } else {
            Rect::new(rect.right() - px - bs.width, rect.y + (head - bs.height) * 0.5, bs.width, bs.height)
        };
        self.change.set_rect(br);
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.change.pointer(e, env.now);
        if r.event.is_some() {
            self.pick();
            return true;
        }
        r.repaint
    }

    fn key(&mut self, _env: &mut Env, e: &KeyEvent) -> bool {
        if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) {
            self.pick();
            return true;
        }
        false
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.change.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.change.cursor(p)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_account(event) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.avatar.set_theme(theme, now);
        self.name.set_style(text_style(theme, TextRole::Title).with_max_lines(1));
        self.identity.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        self.status.set_style(text_style(theme, TextRole::Caption));
        self.change.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        if let Some(next) = self.pending.take() {
            let tex = next.as_ref().map(|img| {
                let (width, height) = img.size();
                ui.painter.upload_texture(ImageData { width, height, pixels: Arc::from(img.pixels()) }, TextureOptions::default())
            });
            self.avatar.set_image(tex);
            self.shown = next;
        }
        self.avatar.draw(ui);
        let ar = self.avatar.rect(t);
        let x = ar.right() + t.space(4);
        let nh = self.name.measure_in(ui.painter).height;
        let ih = self.identity.measure_in(ui.painter).height;
        let sh = if self.status.text().is_empty() { 0.0 } else { t.space(1) + self.status.measure_in(ui.painter).height };
        let total = nh + t.space(1) + ih + sh;
        let top = ar.center().y - total * 0.5;
        self.name.draw(ui.painter, Point::new(x, top), ui.fade(c.on_surface));
        self.identity.draw(ui.painter, Point::new(x, top + nh + t.space(1)), ui.fade(c.on_surface_variant));
        if sh > 0.0 {
            let col = if self.status_error { c.error } else { c.on_surface_variant };
            self.status.draw(ui.painter, Point::new(x, top + nh + t.space(1) + ih + t.space(1)), ui.fade(col));
        }
        self.change.draw(ui);
    }
}

// ---- Görünen ad ---------------------------------------------------------------------------------

#[derive(Debug)]
pub struct UserName {
    desc: &'static SettingDesc,
    text: RowText,
    input: TextInput,
    focused: bool,
    /// Kullanıcı alana yazdı (servisten gelen değer üzerine yazılmaz).
    edited: bool,
    rect: Rect,
    narrow: bool,
}

impl UserName {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        accounts::ensure();
        // Bayt sınırı karakter sınırının üstünde (Türkçe harf 2 bayt); asıl denetim servis tarafında.
        let max_bytes = accounts::MAX_NAME_CHARS * 4;
        let mut b = Self {
            desc,
            text: RowText::new(theme, desc.label()),
            input: TextInput::new(theme).with_placeholder(t!("settings-user-name-placeholder")).with_max_bytes(max_bytes),
            focused: false,
            edited: false,
            rect: Rect::ZERO,
            narrow: false,
        };
        b.update();
        b
    }

    fn update(&mut self) {
        let s = accounts::snapshot();
        let (text, error) = op_status(&s.op, Op::RealName).unwrap_or_default();
        self.text.set_subtitle(&text, error);
        self.input.set_enabled(editable(&s));
        // Yazılan ad yanıt gelene dek alanda kalır (iyimser); ret/iptalde servisin değerine döner.
        let current = s.state.user().map(|u| u.real_name.as_str()).unwrap_or_default();
        if !self.edited && !s.op.waiting(Op::RealName) && self.input.text() != current {
            self.input.set_text(current);
        }
    }

    /// Yazılanı servise verir (boş/değişmemişse servis yazmaz, alan geçerli değere döner).
    fn commit(&mut self) {
        if self.edited {
            self.edited = false;
            accounts::set_real_name(self.input.text());
            self.update();
        }
    }

    fn input_width(theme: &Theme, width: f32) -> f32 {
        (width * 0.4).clamp(theme.space(40), theme.space(64))
    }
}

impl Block for UserName {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        profile_shown(&accounts::snapshot())
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        if is_narrow(t, width) {
            let text = self.text.height(env.text, t, width - px * 2.0);
            let ih = self.input.preferred_size(t, width - px * 2.0).height;
            return text + ih + py * 3.0;
        }
        let iw = Self::input_width(t, width);
        let ih = self.input.preferred_size(t, iw).height;
        let text = self.text.height(env.text, t, (width - px * 3.0 - iw).max(t.space(20)));
        row_height_for(t, text.max(ih))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.narrow = is_narrow(t, rect.width);
        if self.narrow {
            let w = rect.width - px * 2.0;
            let ih = self.input.preferred_size(t, w).height;
            self.input.set_rect(Rect::new(rect.x + px, rect.bottom() - py - ih, w, ih));
        } else {
            let iw = Self::input_width(t, rect.width);
            let ih = self.input.preferred_size(t, iw).height;
            self.input.set_rect(Rect::new(rect.right() - px - iw, rect.center().y - ih * 0.5, iw, ih));
        }
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

    fn key(&mut self, _env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.input.key(e, _env.now);
        match r.event {
            Some(TextInputEvent::Submit) => {
                self.edited = true;
                self.commit();
                true
            }
            Some(TextInputEvent::Cancel) => {
                self.edited = false;
                self.update();
                true
            }
            Some(TextInputEvent::Changed) => {
                self.edited = true;
                true
            }
            _ => r.repaint,
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        let was = self.focused;
        self.focused = focused;
        self.input.set_focused(focused, now);
        if was && !focused {
            // Odak giderken yazılanı kaydet (Enter'a basılmasa da).
            self.commit();
        }
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        Some(&mut self.input)
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.input.cursor(p)
    }

    fn sync(&mut self, _env: &mut Env) {
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if is_account(event) {
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.input.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, py) = row_padding(t);
        let h = self.text.drawn_height(ui);
        let top = if self.narrow { self.rect.y + py } else { self.rect.center().y - h * 0.5 };
        self.text.draw(ui, self.rect.x + px, top);
        self.input.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(kind: AccountType) -> User {
        User { real_name: "Ayşe Yılmaz".into(), user_name: "ayse".into(), icon_file: None, account_type: kind }
    }

    #[test]
    fn profile_avatar_is_xlarge_96px() {
        let theme = Theme::default();
        assert_eq!(UserAvatar::side(&theme), 96.0);
        assert_eq!(UserAvatar::side(&theme), AvatarSize::XLarge.side(&theme));
        assert!(UserAvatar::side(&theme) > AvatarSize::Large.side(&theme));
    }

    #[test]
    fn note_only_when_service_is_unusable() {
        let mut s = Snapshot::default();
        assert_eq!(note_text(&s), None, "okunurken not yok");
        assert!(profile_shown(&s), "okunurken kart yerinde");
        s.state = AccountState::Missing;
        assert_eq!(note_text(&s).as_deref(), Some(t!("settings-user-missing").as_str()));
        s.state = AccountState::Failed;
        assert!(note_text(&s).is_some());
        assert!(!profile_shown(&s), "hizmet yokken yalnızca not");
        s.state = AccountState::Ready(user(AccountType::Standard));
        assert_eq!(note_text(&s), None);
    }

    #[test]
    fn identity_and_shown_name() {
        let u = user(AccountType::Administrator);
        let id = identity_text(&u);
        let admin = t!("settings-user-type-admin");
        assert_eq!(id, t!("settings-user-identity", user = "ayse", kind = admin.as_str()));
        assert_eq!(identity_text(&user(AccountType::Unknown)), "ayse");
        assert_eq!(shown_name(&u), "Ayşe Yılmaz");
        let mut blank = u.clone();
        blank.real_name = "  ".into();
        assert_eq!(shown_name(&blank), "ayse", "görünen ad yoksa kullanıcı adı");
    }

    #[test]
    fn status_texts() {
        assert_eq!(op_status(&OpState::Idle, Op::Avatar), None);
        assert_eq!(op_status(&OpState::Picking, Op::Avatar), None, "seçici açıkken metin yok");
        assert_eq!(op_status(&OpState::Waiting(Op::RealName), Op::RealName), Some((t!("settings-user-waiting"), false)));
        assert_eq!(op_status(&OpState::Waiting(Op::RealName), Op::Avatar), None);
        let (text, error) = op_status(&OpState::Failed(Op::Avatar, ErrorKind::BadImage), Op::Avatar).unwrap();
        assert!(error && text == t!("settings-user-avatar-bad-image"));
        assert_eq!(error_text(ErrorKind::NotAuthorized), error_text(ErrorKind::Other));
    }

    #[test]
    fn editing_needs_ready_and_idle() {
        let mut s = Snapshot::default();
        assert!(!editable(&s));
        s.state = AccountState::Ready(user(AccountType::Standard));
        assert!(editable(&s));
        s.op = OpState::Picking;
        assert!(!editable(&s));
        s.op = OpState::Failed(Op::Avatar, ErrorKind::Face);
        assert!(editable(&s), "hata sonrası yeniden denenebilir");
    }
}
