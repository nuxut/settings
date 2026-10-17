//! Hesaplar: takvim hesapları listesi ve "Hesap ekle" akışı (calendar `ARCHITECTURE` §4.3).
//!
//! Kullanıcıyı uğraştırmamak için her ekranda **tek soru** ve tek büyük düğme vardır.
//! Akış daemon'da yaşar; bu blok yalnızca durumu çizer ve girdiyi iletir.

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut::motion::Animated;
use nuxut::ui::input::{CursorShape, Button as MouseButton, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{HAIRLINE, state_layer_under};
use nuxut::ui::IconName;
use nuxut::ui::{
    Button, ButtonVariant, ColorEvent, ColorPicker, Icon, IconButton, IconButtonVariant, IconView, Interaction, Label, Spinner, TextInput, TextInputEvent, TextRole, Toggle, Ui,
    text_style,
};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, Theme};
use nuxut_calendar_ipc::{AccountEntry, CalendarEntry, FlowInput, FlowState, step};

use super::fiyu::{FiyuEmpty, Spot};
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::row_padding;
use crate::services::calendar::{Command, Update};
use crate::services::{Event, Request};

/// Hesaplar bloğunun maskot yeri: hiç yüklenemeden servis hatası → hata maskotu; yüklendi,
/// listede ve hesap yok → boş maskotu; akış ve sağlayıcı ekranlarında yok. İkisi aynı anda olmaz.
pub fn accounts_spot(unavailable: bool, loaded: bool, list_view: bool, no_accounts: bool) -> Option<Spot> {
    if unavailable && !loaded {
        Some(Spot::AccountsUnavailable)
    } else if loaded && list_view && no_accounts {
        Some(Spot::AccountsEmpty)
    } else {
        None
    }
}

/// "Kaldır"a ikinci tıklamanın beklendiği süre.
const CONFIRM: Duration = Duration::from_secs(4);

fn provider_info(id: &str) -> (String, String, IconName) {
    let description = |key: &str| nuxut::i18n::current().attr(key, "description", None).map_or_else(String::new, |s| s.into_owned());
    match id {
        "google" => ("Google".to_owned(), t!("settings-accounts-provider-google-description"), IconName::Mail),
        "microsoft" => ("Microsoft".to_owned(), t!("settings-accounts-provider-microsoft-description"), IconName::Mail),
        "nextcloud" => ("Nextcloud".to_owned(), t!("settings-accounts-provider-nextcloud-description"), IconName::Server),
        "caldav" => (t!("settings-accounts-provider-caldav"), description("settings-accounts-provider-caldav"), IconName::CalendarMonth),
        "ics-subscription" => (t!("settings-accounts-provider-subscription"), description("settings-accounts-provider-subscription"), IconName::Link),
        _ => (t!("settings-accounts-provider-unknown"), String::new(), IconName::Help),
    }
}

fn relative_time(unix: i64) -> String {
    if unix <= 0 {
        return t!("settings-accounts-sync-never");
    }
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
    let d = (now - unix).max(0);
    match d {
        0..=59 => t!("settings-accounts-sync-just-now"),
        60..=3599 => t!("settings-accounts-sync-minutes-ago", count = d / 60),
        3600..=86_399 => t!("settings-accounts-sync-hours-ago", count = d / 3600),
        _ => t!("settings-accounts-sync-days-ago", count = d / 86_400),
    }
}

/// Hesap kartının durum satırı. `state_reason`/`last_error` yalnızca sabit anahtardır
/// (calendar ADR-0007 §8); gösterilen metin daemon'un `*_message` alanlarından gelir. Anahtar
/// asla gösterilmez: metni boş (eski daemon) anahtar genel metne düşer.
fn account_status(entry: &AccountEntry) -> String {
    match entry.state.as_str() {
        "needs-reconnect" if !entry.state_reason_message.is_empty() => entry.state_reason_message.clone(),
        "needs-reconnect" => t!("settings-accounts-state-needs-reconnect"),
        "disabled" => t!("settings-accounts-state-disabled"),
        _ if entry.syncing => t!("settings-accounts-state-syncing"),
        _ if !entry.last_error_message.is_empty() => entry.last_error_message.clone(),
        _ if !entry.last_error.is_empty() => t!("settings-accounts-sync-failed"),
        _ => relative_time(entry.last_sync),
    }
}

#[derive(Debug)]
struct CalRow {
    id: String,
    color: Color,
    name: Label,
    toggle: Toggle,
    /// Renk noktası (tıklayınca altında renk seçici açılır).
    swatch: Rect,
    overridden: bool,
    open: bool,
    picker: Box<ColorPicker>,
    reset: Button,
}

fn picker_width(theme: &Theme, card_w: f32) -> f32 {
    (card_w - theme.space(18)).min(theme.space(80))
}

fn cal_height(theme: &Theme, c: &CalRow, card_w: f32) -> f32 {
    let base = theme.space(10);
    if c.open { base + c.picker.preferred_size(theme, picker_width(theme, card_w)).height + theme.space(14) } else { base }
}

#[derive(Debug)]
struct AccountCard {
    entry: AccountEntry,
    icon: IconView,
    title: Label,
    sub: Label,
    status: Label,
    sync: IconButton,
    remove: IconButton,
    reconnect: Option<Button>,
    confirm_remove: Option<Instant>,
    calendars: Vec<CalRow>,
    rect: Rect,
}

#[derive(Debug)]
struct ProviderCard {
    id: String,
    icon: IconView,
    title: Label,
    sub: Label,
    hover: Interaction,
    rect: Rect,
}

#[derive(Debug, Clone, PartialEq)]
enum View {
    List,
    Providers,
    Flow,
}

#[derive(Debug)]
pub struct Accounts {
    desc: &'static SettingDesc,
    rect: Rect,
    view: View,
    status: Option<String>,
    loaded: bool,
    accounts: Vec<AccountCard>,
    providers: Vec<ProviderCard>,
    /// Hesap yok (yüklendikten sonra): maskotlu boş durum.
    empty: FiyuEmpty,
    /// Takvim servisine ulaşılamadı (yüklenmeden): maskotlu hata; gövde servis mesajı.
    down: FiyuEmpty,
    add: Button,
    back: Button,
    retry: Button,
    // Akış.
    flow: Option<String>,
    state: FlowState,
    provider: String,
    question: Label,
    hint: Label,
    error: Label,
    input: TextInput,
    input2: TextInput,
    second_field: bool,
    primary: Button,
    secondary: Button,
    link: Button,
    code: Label,
    busy: bool,
    pulse: Animated<f32>,
    spinner: Spinner,
    focus: u8,
    toast: Label,
}

impl Accounts {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        crate::services::dispatch(Request::Calendar(Command::Refresh));
        crate::services::dispatch(Request::Calendar(Command::Resume));
        let mut pulse = Animated::new(0.0, theme.motion.spatial);
        pulse.jump_to(0.0);
        Self {
            desc,
            rect: Rect::ZERO,
            view: View::List,
            status: None,
            loaded: false,
            accounts: Vec::new(),
            providers: Vec::new(),
            empty: FiyuEmpty::new(theme, Spot::AccountsEmpty, &t!("settings-accounts-empty-title"), Some(&t!("settings-accounts-empty-body"))),
            down: FiyuEmpty::new(theme, Spot::AccountsUnavailable, &t!("settings-accounts-unavailable-title"), None),
            add: Button::new(theme, ButtonVariant::Filled).with_icon(theme, Icon::named(IconName::Add)).with_label(theme, t!("settings-accounts-add")),
            back: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::ArrowBack)).with_label(theme, t!("settings-accounts-back")),
            retry: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::Refresh)).with_label(theme, t!("settings-accounts-retry")),
            flow: None,
            state: FlowState::default(),
            provider: String::new(),
            question: Label::new("", text_style(theme, TextRole::Title)),
            hint: Label::new("", text_style(theme, TextRole::Body)),
            error: Label::new("", text_style(theme, TextRole::Body)),
            input: TextInput::new(theme).with_max_bytes(512),
            input2: TextInput::new(theme).with_max_bytes(512),
            second_field: false,
            primary: Button::new(theme, ButtonVariant::Filled).with_label(theme, t!("settings-accounts-continue")),
            secondary: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-accounts-cancel")),
            link: Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::OpenInNew)).with_label(theme, t!("settings-accounts-reopen-link")),
            code: Label::new("", text_style(theme, TextRole::Display)),
            busy: false,
            pulse,
            spinner: Spinner::new(theme),
            focus: 0,
            toast: Label::new("", text_style(theme, TextRole::Caption)),
        }
    }

    fn send(env: &mut Env, c: Command) {
        env.push(Action::Service(Request::Calendar(c)));
    }

    fn set_snapshot(&mut self, theme: &Theme, accounts: Vec<AccountEntry>, calendars: Vec<CalendarEntry>, providers: Vec<String>) {
        self.loaded = true;
        self.status = None;
        let old: Vec<(String, Option<Instant>)> = self.accounts.iter().map(|a| (a.entry.id.clone(), a.confirm_remove)).collect();
        // Renk seçicisi açık takvimler yenilemede açık kalır (renk değişince CalendarsChanged gelir).
        let open_cals: Vec<String> = self.accounts.iter().flat_map(|a| a.calendars.iter().filter(|c| c.open).map(|c| c.id.clone())).collect();
        self.accounts = accounts
            .into_iter()
            .map(|entry| {
                let (pname, _, icon) = provider_info(&entry.provider);
                let title = if entry.display_name.is_empty() { pname.clone() } else { entry.display_name.clone() };
                let sub = if entry.login.is_empty() { pname.clone() } else { t!("settings-accounts-card-sub", provider = pname.as_str(), login = entry.login.as_str()) };
                let status = account_status(&entry);
                let mut cals: Vec<&CalendarEntry> = calendars.iter().filter(|c| c.account == entry.id).collect();
                cals.sort_by_key(|c| c.order);
                let reconnect = (entry.state == "needs-reconnect")
                    .then(|| Button::new(theme, ButtonVariant::Filled).with_icon(theme, Icon::named(IconName::Link)).with_label(theme, t!("settings-accounts-button-reconnect")));
                let confirm_remove = old.iter().find(|(id, _)| *id == entry.id).and_then(|(_, c)| *c);
                AccountCard {
                    icon: IconView::with_role(theme, Icon::named(icon), TextRole::Icon),
                    title: Label::new(title, text_style(theme, TextRole::Label)),
                    sub: Label::new(sub, text_style(theme, TextRole::Caption)),
                    status: Label::new(status, text_style(theme, TextRole::Caption)),
                    sync: IconButton::new(theme, Icon::named(IconName::Sync), IconButtonVariant::Standard),
                    remove: IconButton::new(theme, Icon::named(IconName::Delete), IconButtonVariant::Standard),
                    reconnect,
                    confirm_remove,
                    calendars: cals
                        .into_iter()
                        .map(|c| {
                            let color = Color::from_hex(&c.color).unwrap_or(theme.colors.primary);
                            CalRow {
                                id: c.id.clone(),
                                color,
                                name: Label::new(if c.name.is_empty() { t!("settings-accounts-calendar-default-name") } else { c.name.clone() }, text_style(theme, TextRole::Body)),
                                toggle: Toggle::new(theme, c.enabled),
                                swatch: Rect::ZERO,
                                overridden: c.color_overridden,
                                open: open_cals.contains(&c.id),
                                picker: Box::new(ColorPicker::new(theme, color)),
                                reset: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Restart)).with_label(theme, t!("settings-accounts-calendar-color-reset")),
                            }
                        })
                        .collect(),
                    entry,
                    rect: Rect::ZERO,
                }
            })
            .collect();
        self.providers = providers
            .into_iter()
            .map(|id| {
                let (title, sub, icon) = provider_info(&id);
                ProviderCard {
                    icon: IconView::with_role(theme, Icon::named(icon), TextRole::IconLarge),
                    title: Label::new(title, text_style(theme, TextRole::Label)),
                    sub: Label::new(sub, text_style(theme, TextRole::Caption)),
                    hover: Interaction::new(theme),
                    id,
                    rect: Rect::ZERO,
                }
            })
            .collect();
    }

    /// Akış durumunu ekrana çevirir: soru, ipucu, alanlar, düğmeler.
    fn apply_flow(&mut self, theme: &Theme, state: FlowState, now: Instant) {
        self.busy = false;
        let (pname, _, _) = provider_info(if state.provider.is_empty() { &self.provider } else { &state.provider });
        let prev = std::mem::replace(&mut self.state, state);
        let s = &self.state;
        let step_changed = prev.step != s.step;
        // Hatadan "Tekrar dene" ile dönülünce kullanıcının yazdığı korunur.
        let fresh = step_changed && prev.step != step::FAILED;
        self.error.set_text("");
        self.second_field = false;
        self.primary.set_label(&t!("settings-accounts-continue"));
        self.secondary.set_label(&t!("settings-accounts-cancel"));
        let field = |input: &mut TextInput, placeholder: &str, password: bool, keep: bool| {
            *input = TextInput::new(theme).with_placeholder(placeholder).password(password).with_max_bytes(512);
            let _ = keep;
        };
        match s.step.as_str() {
            step::NEED_SERVER => {
                self.question.set_text(&t!("settings-accounts-flow-server-question"));
                self.hint.set_text(&if s.hint.is_empty() { t!("settings-accounts-flow-server-hint") } else { s.hint.clone() });
                if fresh {
                    field(&mut self.input, &t!("settings-accounts-flow-server-placeholder"), false, false);
                }
            }
            step::NEED_EMAIL => {
                self.question.set_text(&t!("settings-accounts-flow-email-question"));
                self.hint.set_text(&t!("settings-accounts-flow-email-hint"));
                if fresh {
                    field(&mut self.input, &t!("settings-accounts-flow-email-placeholder"), false, false);
                }
            }
            step::NEED_PASSWORD => {
                let name = if s.provider_name.is_empty() { pname.clone() } else { s.provider_name.clone() };
                self.question.set_text(&t!("settings-accounts-flow-password-question", provider = name.as_str()));
                self.hint.set_text(&t!("settings-accounts-flow-password-hint"));
                if fresh {
                    field(&mut self.input, &t!("settings-accounts-flow-password-placeholder"), true, false);
                }
                self.second_field = s.server.is_empty();
                if self.second_field && fresh {
                    field(&mut self.input2, &t!("settings-accounts-flow-password-server-placeholder"), false, false);
                }
                self.link.set_label(&t!("settings-accounts-flow-password-link"));
            }
            step::NEED_URL => {
                self.question.set_text(&t!("settings-accounts-flow-url-question"));
                self.hint.set_text(&t!("settings-accounts-flow-url-hint"));
                if fresh {
                    field(&mut self.input, &t!("settings-accounts-flow-url-placeholder"), false, false);
                }
            }
            step::WAITING_BROWSER => {
                self.question.set_text(&t!("settings-accounts-flow-browser-question"));
                self.hint.set_text(&t!("settings-accounts-flow-browser-hint", provider = pname.as_str()));
                self.code.set_style(text_style(theme, TextRole::Display));
                self.code.set_text(&s.user_code);
                self.link.set_label(&t!("settings-accounts-reopen-link"));
            }
            step::CONFIRM_SERVER => {
                self.question.set_text(&t!("settings-accounts-flow-confirm-server-question"));
                self.hint.set_text(&if s.message.is_empty() { t!("settings-accounts-flow-confirm-server-hint") } else { s.message.clone() });
                self.code.set_style(text_style(theme, TextRole::Title));
                self.code.set_text(&s.server);
                self.primary.set_label(&t!("settings-accounts-flow-confirm-yes"));
                self.secondary.set_label(&t!("settings-sections-cancel"));
            }
            step::CONFIRM_SUBSCRIPTION => {
                // Adres yoksa ayrı cümle: Türkçe ek ($host + "takvimine") bir yedek sözcüğe yapıştırılmaz.
                let question = if s.server.is_empty() {
                    t!("settings-accounts-flow-subscribe-question-unnamed")
                } else {
                    t!("settings-accounts-flow-subscribe-question", host = s.server.as_str())
                };
                self.question.set_text(&question);
                self.hint.set_text(&t!("settings-accounts-flow-subscribe-hint"));
                self.code.set_style(text_style(theme, TextRole::Title));
                self.code.set_text(&s.url);
                self.primary.set_label(&t!("settings-accounts-flow-subscribe-confirm"));
                self.secondary.set_label(&t!("settings-sections-cancel"));
            }
            step::CONNECTING => {
                self.question.set_text(&t!("settings-accounts-flow-connecting"));
                self.hint.set_text(&t!("settings-accounts-flow-connecting-hint"));
            }
            step::DISCOVERING => {
                self.question.set_text(&t!("settings-accounts-flow-discovering"));
                self.hint.set_text(&if s.found > 0 { t!("settings-accounts-flow-found", count = s.found) } else { t!("settings-accounts-flow-discovering-hint") });
            }
            step::DONE => {
                self.question.set_text(&t!("settings-accounts-flow-done"));
                let n = s.found;
                self.hint.set_text(&t!("settings-accounts-flow-done-hint", count = n));
                self.primary.set_label(&t!("settings-accounts-flow-done-button"));
            }
            step::FAILED => {
                self.question.set_text(&t!("settings-accounts-flow-failed"));
                self.error.set_text(&if s.message.is_empty() { t!("settings-accounts-flow-error-generic") } else { s.message.clone() });
                self.hint.set_text("");
                self.primary.set_label(&t!("settings-accounts-retry"));
            }
            other => {
                self.question.set_text("…");
                self.hint.set_text(other);
            }
        }
        if step_changed {
            self.focus = 0;
            self.input.set_focused(self.has_input(), now);
            self.pulse.jump_to(0.0);
            self.pulse.set_target(1.0, now);
        }
    }

    /// Büyük metin: cihaz kodu ya da onaylanacak sunucu.
    fn show_code(&self) -> bool {
        (self.state.step == step::WAITING_BROWSER && !self.state.user_code.is_empty()) || matches!(self.state.step.as_str(), step::CONFIRM_SERVER | step::CONFIRM_SUBSCRIPTION)
    }

    fn has_input(&self) -> bool {
        matches!(self.state.step.as_str(), step::NEED_SERVER | step::NEED_EMAIL | step::NEED_PASSWORD | step::NEED_URL)
    }

    /// Kısa bekleme (bağlanma, keşif, daemon yanıtı): nokta animasyonu. Tarayıcı beklemesi
    /// dakikalar sürebilir; orada animasyon yok (boşta iş yok).
    fn animating_wait(&self) -> bool {
        matches!(self.state.step.as_str(), step::CONNECTING | step::DISCOVERING) || self.busy
    }

    fn waiting(&self) -> bool {
        matches!(self.state.step.as_str(), step::WAITING_BROWSER | step::CONNECTING | step::DISCOVERING) || self.busy
    }

    fn submit(&mut self, env: &mut Env) {
        let Some(flow) = self.flow.clone() else { return };
        match self.state.step.as_str() {
            step::CONFIRM_SERVER | step::CONFIRM_SUBSCRIPTION => {
                self.busy = true;
                Self::send(env, Command::Submit { flow, input: FlowInput { confirm: true, ..FlowInput::default() } });
            }
            step::DONE => {
                self.flow = None;
                self.view = View::List;
                Self::send(env, Command::Refresh);
            }
            step::FAILED => {
                // retry_step'e dön: aynı ekran, girdi korunur, hata metni görünür. Boşsa akış
                // bitti; aynı sağlayıcıyla yeniden başlat.
                let msg = self.state.message.clone();
                if self.state.retry_step.is_empty() {
                    let p = if self.state.provider.is_empty() { self.provider.clone() } else { self.state.provider.clone() };
                    self.busy = true;
                    Self::send(env, Command::Begin { provider: p, reconnect: String::new() });
                } else {
                    let mut s = self.state.clone();
                    s.step = s.retry_step.clone();
                    self.apply_flow(env.theme, s, env.now);
                    self.error.set_text(&msg);
                }
            }
            _ if self.has_input() => {
                let text = self.input.text().trim().to_owned();
                if text.is_empty() {
                    return;
                }
                let mut input = FlowInput::default();
                match self.state.step.as_str() {
                    step::NEED_SERVER => input.server = text,
                    step::NEED_EMAIL => input.email = text,
                    step::NEED_PASSWORD => {
                        input.password = text;
                        if self.second_field {
                            input.server = self.input2.text().trim().to_owned();
                        }
                    }
                    _ => input.url = text,
                }
                self.busy = true;
                Self::send(env, Command::Submit { flow, input });
            }
            _ => {}
        }
    }

    fn cancel(&mut self, env: &mut Env) {
        // Yönlendirme onayında "Vazgeç" akışı bitirmez: şifre adımına döner.
        if matches!(self.state.step.as_str(), step::CONFIRM_SERVER | step::CONFIRM_SUBSCRIPTION)
            && let Some(flow) = self.flow.clone()
        {
            self.busy = true;
            Self::send(env, Command::Submit { flow, input: FlowInput { confirm: false, ..FlowInput::default() } });
            return;
        }
        if let Some(flow) = self.flow.take() {
            Self::send(env, Command::Cancel(flow));
        }
        self.view = View::List;
        self.state = FlowState::default();
        self.busy = false;
    }

    // ---- Yerleşim yardımcıları -------------------------------------------------------------

    fn card_height(theme: &Theme, a: &AccountCard, card_w: f32) -> f32 {
        let head = nuxut::ui::style::row_height(theme);
        let cals: f32 = a.calendars.iter().map(|c| cal_height(theme, c, card_w)).sum();
        let extra = if a.reconnect.is_some() { theme.space(12) } else { 0.0 };
        head + cals + extra + if a.calendars.is_empty() { 0.0 } else { theme.space(2) }
    }

    /// Hesap listesi boşken listenin yeri: yüklendiyse maskotlu boş durum, yüklenirken
    /// "yükleniyor" satırı.
    fn list_empty_height(&mut self, env: &mut Env, w: f32) -> f32 {
        let t = env.theme;
        if self.loaded {
            self.empty.height(env.text, t, w)
        } else {
            t.space(10)
        }
    }

    /// Ekranda maskotlu durum ([`Block::mascot`], [`accounts_spot`]).
    fn mascot_spot(&self) -> Option<Spot> {
        accounts_spot(self.status.is_some(), self.loaded, self.view == View::List, self.accounts.is_empty())
    }

    fn content_height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let w = width - px * 2.0;
        if let Some(s) = &self.status
            && !self.loaded
        {
            // Maskotlu hata (başlık + servis mesajı), altında ortalı "Tekrar dene".
            self.down.set_text(t, &t!("settings-accounts-unavailable-title"), Some(s));
            let b = self.retry.preferred_size(env.text, t);
            return self.down.height(env.text, t, width) + b.height + py * 2.0;
        }
        match self.view {
            View::List => {
                let mut h = py;
                if self.accounts.is_empty() {
                    h += self.list_empty_height(env, w);
                }
                for a in &self.accounts {
                    h += Self::card_height(t, a, w) + t.space(3);
                }
                h + t.space(10) + py
            }
            View::Providers => {
                let cols = if w > t.space(120) { 2 } else { 1 };
                let rows = self.providers.len().div_ceil(cols) as f32;
                py + t.space(10) + t.space(3) + rows * (t.space(20) + t.space(3)) + py
            }
            View::Flow => {
                self.question.set_max_width(Some(w));
                self.hint.set_max_width(Some(w));
                self.error.set_max_width(Some(w));
                self.code.set_max_width(Some(w));
                let mut h = py + t.space(10) + t.space(4);
                h += self.question.measure(env.text).height + t.space(2);
                if !self.hint.text().is_empty() {
                    h += self.hint.measure(env.text).height + t.space(4);
                }
                if !self.error.text().is_empty() {
                    h += self.error.measure(env.text).height + t.space(4);
                }
                if self.has_input() {
                    h += t.space(12) + t.space(3);
                    if self.second_field {
                        h += t.space(12) + t.space(3);
                    }
                }
                if self.show_code() {
                    h += self.code.measure(env.text).height + t.space(4);
                }
                if self.animating_wait() {
                    h += t.space(9);
                }
                h + t.space(10) + py
            }
        }
    }
}

impl Block for Accounts {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.mascot_spot()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.content_height(env, width)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let x = rect.x + px;
        let w = rect.width - px * 2.0;
        let mut y = rect.y + py;
        if self.status.is_some() && !self.loaded {
            let dh = self.down.height(env.text, t, rect.width);
            self.down.set_rect(Rect::new(rect.x, y, rect.width, dh));
            let s = self.retry.preferred_size(env.text, t);
            self.retry.set_rect(Rect::new((rect.center().x - s.width * 0.5).round(), y + dh, s.width, s.height));
            return;
        }
        match self.view {
            View::List => {
                if self.accounts.is_empty() {
                    let eh = self.list_empty_height(env, w);
                    self.empty.set_rect(Rect::new(x, y, w, eh));
                    y += eh;
                }
                for a in &mut self.accounts {
                    let h = Self::card_height(t, a, w);
                    a.rect = Rect::new(x, y, w, h);
                    let bs = a.sync.preferred_size(t);
                    let by = y + (nuxut::ui::style::row_height(t) - bs.height) * 0.5;
                    a.remove.set_rect(Rect::new(a.rect.right() - t.space(2) - bs.width, by, bs.width, bs.height));
                    a.sync.set_rect(Rect::new(a.rect.right() - t.space(2) - bs.width * 2.0, by, bs.width, bs.height));
                    let mut cy = y + nuxut::ui::style::row_height(t);
                    if let Some(b) = &mut a.reconnect {
                        let s = b.preferred_size(env.text, t);
                        b.set_rect(Rect::new(x + t.space(14), cy, s.width, s.height));
                        cy += t.space(12);
                    }
                    for c in &mut a.calendars {
                        let ts = c.toggle.preferred_size(t);
                        c.toggle.set_rect(Rect::new(a.rect.right() - t.space(4) - ts.width, cy + (t.space(10) - ts.height) * 0.5, ts.width, ts.height));
                        let d = t.space(5);
                        c.swatch = Rect::new(x + t.space(13), cy + (t.space(10) - d) * 0.5, d, d);
                        if c.open {
                            let pw = picker_width(t, w);
                            let ph = c.picker.preferred_size(t, pw).height;
                            let px0 = x + t.space(14);
                            c.picker.set_rect(t, Rect::new(px0, cy + t.space(11), pw, ph));
                            let rs = c.reset.preferred_size(env.text, t);
                            c.reset.set_rect(Rect::new(px0 - t.space(2), cy + t.space(11) + ph + t.space(1), rs.width, rs.height));
                        }
                        cy += cal_height(t, c, w);
                    }
                    y += h + t.space(3);
                }
                let s = self.add.preferred_size(env.text, t);
                self.add.set_rect(Rect::new(x, y, s.width, s.height));
            }
            View::Providers => {
                let s = self.back.preferred_size(env.text, t);
                self.back.set_rect(Rect::new(x - t.space(2), y, s.width, s.height));
                y += t.space(10) + t.space(3);
                let cols = if w > t.space(120) { 2 } else { 1 };
                let gap = t.space(3);
                let cw = (w - gap * (cols as f32 - 1.0)) / cols as f32;
                for (i, p) in self.providers.iter_mut().enumerate() {
                    let (r, c) = (i / cols, i % cols);
                    p.rect = Rect::new(x + c as f32 * (cw + gap), y + r as f32 * (t.space(20) + gap), cw, t.space(20));
                }
            }
            View::Flow => {
                let s = self.back.preferred_size(env.text, t);
                self.back.set_rect(Rect::new(x - t.space(2), y, s.width, s.height));
                y += t.space(10) + t.space(4);
                y += self.question.measure(env.text).height + t.space(2);
                if !self.hint.text().is_empty() {
                    y += self.hint.measure(env.text).height + t.space(4);
                }
                if !self.error.text().is_empty() {
                    y += self.error.measure(env.text).height + t.space(4);
                }
                let fw = w.min(t.space(110));
                if self.has_input() {
                    let h = self.input.preferred_size(t, fw).height;
                    self.input.set_rect(Rect::new(x, y, fw, h));
                    y += t.space(12) + t.space(3);
                    if self.second_field {
                        self.input2.set_rect(Rect::new(x, y, fw, h));
                        y += t.space(12) + t.space(3);
                    }
                }
                if self.show_code() {
                    y += self.code.measure(env.text).height + t.space(4);
                }
                if self.animating_wait() {
                    y += t.space(9);
                }
                let ps = self.primary.preferred_size(env.text, t);
                let ss = self.secondary.preferred_size(env.text, t);
                let ls = self.link.preferred_size(env.text, t);
                self.primary.set_rect(Rect::new(x, y, ps.width, ps.height));
                let mut bx = x + ps.width + t.space(2);
                let show_link = self.state.step == step::WAITING_BROWSER || (self.state.step == step::NEED_PASSWORD && !self.state.app_password_url.is_empty());
                if self.waiting() {
                    bx = x;
                }
                if show_link {
                    self.link.set_rect(Rect::new(bx, y, ls.width, ls.height));
                    bx += ls.width + t.space(2);
                }
                self.secondary.set_rect(Rect::new(bx, y, ss.width, ss.height));
            }
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let t = env.theme;
        let mut repaint = false;
        if self.status.is_some() && !self.loaded {
            let r = self.retry.pointer(e, now);
            if r.event.is_some() {
                Self::send(env, Command::Refresh);
            }
            return r.repaint;
        }
        match self.view {
            View::List => {
                let r = self.add.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some() {
                    self.view = View::Providers;
                    return true;
                }
                let mut cmd = None;
                for a in &mut self.accounts {
                    let r = a.sync.pointer(e, now);
                    repaint |= r.repaint;
                    if r.event.is_some() {
                        cmd = Some(Command::SyncNow(a.entry.id.clone()));
                        a.status.set_text(&t!("settings-accounts-state-syncing"));
                    }
                    let r = a.remove.pointer(e, now);
                    repaint |= r.repaint;
                    if r.event.is_some() {
                        match a.confirm_remove {
                            Some(at) if now.saturating_duration_since(at) < CONFIRM => {
                                cmd = Some(Command::Remove(a.entry.id.clone()));
                                a.confirm_remove = None;
                            }
                            _ => {
                                a.confirm_remove = Some(now);
                                a.status.set_text(&t!("settings-accounts-remove-confirm"));
                            }
                        }
                    }
                    if let Some(b) = &mut a.reconnect {
                        let r = b.pointer(e, now);
                        repaint |= r.repaint;
                        if r.event.is_some() {
                            self.provider = a.entry.provider.clone();
                            cmd = Some(Command::Begin { provider: a.entry.provider.clone(), reconnect: a.entry.id.clone() });
                        }
                    }
                    for c in &mut a.calendars {
                        let r = c.toggle.pointer(e, now);
                        repaint |= r.repaint;
                        if let Some(on) = r.event {
                            cmd = Some(Command::SetCalendarEnabled { calendar: c.id.clone(), enabled: on });
                        }
                        if let PointerKind::Press(MouseButton::Left) = e.kind
                            && c.swatch.expand(t.space(2)).contains(e.position)
                        {
                            c.open = !c.open;
                            repaint = true;
                        }
                        if c.open {
                            let r = c.picker.pointer(env.text, t, e, now);
                            repaint |= r.repaint;
                            match r.event {
                                Some(ColorEvent::Changed(col)) => c.color = col,
                                Some(ColorEvent::Committed(col)) => {
                                    c.color = col;
                                    cmd = Some(Command::SetCalendarColor { calendar: c.id.clone(), color: col.to_hex() });
                                }
                                None => {}
                            }
                            if c.overridden {
                                let r = c.reset.pointer(e, now);
                                repaint |= r.repaint;
                                if r.event.is_some() {
                                    cmd = Some(Command::SetCalendarColor { calendar: c.id.clone(), color: String::new() });
                                }
                            }
                        }
                    }
                }
                if let Some(c) = cmd {
                    if matches!(c, Command::Begin { .. }) {
                        self.view = View::Flow;
                        self.busy = true;
                        self.state = FlowState::default();
                        self.question.set_text(&t!("settings-accounts-flow-starting"));
                        self.hint.set_text("");
                    }
                    Self::send(env, c);
                    repaint = true;
                }
            }
            View::Providers => {
                let r = self.back.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some() {
                    self.view = View::List;
                    return true;
                }
                let mut chosen = None;
                for p in &mut self.providers {
                    let tap = p.hover.handle(p.rect, e, now);
                    repaint |= tap.repaint;
                    if tap.clicked {
                        chosen = Some(p.id.clone());
                    }
                }
                if let Some(id) = chosen {
                    self.provider = id.clone();
                    self.view = View::Flow;
                    self.busy = true;
                    self.state = FlowState::default();
                    let (name, _, _) = provider_info(&id);
                    self.question.set_text(&t!("settings-accounts-flow-adding", provider = name.as_str()));
                    self.hint.set_text("");
                    self.error.set_text("");
                    Self::send(env, Command::Begin { provider: id, reconnect: String::new() });
                    repaint = true;
                }
            }
            View::Flow => {
                let r = self.back.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some() {
                    self.cancel(env);
                    return true;
                }
                if self.has_input() {
                    let r = self.input.pointer(env.text, t, e, now);
                    repaint |= r.repaint;
                    if self.second_field {
                        repaint |= self.input2.pointer(env.text, t, e, now).repaint;
                    }
                    if let PointerKind::Press(MouseButton::Left) = e.kind {
                        if self.input.rect().contains(e.position) {
                            self.focus = 0;
                            env.push(Action::Focus);
                        } else if self.second_field && self.input2.rect().contains(e.position) {
                            self.focus = 1;
                            env.push(Action::Focus);
                        }
                        self.input.set_focused(self.focus == 0, now);
                        self.input2.set_focused(self.focus == 1, now);
                    }
                }
                if !self.waiting() || self.state.step == step::DONE {
                    let r = self.primary.pointer(e, now);
                    repaint |= r.repaint;
                    if r.event.is_some() {
                        self.submit(env);
                        return true;
                    }
                }
                let r = self.link.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some()
                    && let Some(flow) = self.flow.clone()
                {
                    if self.state.step == step::WAITING_BROWSER {
                        Self::send(env, Command::ReopenBrowser(flow));
                    } else if !self.state.app_password_url.is_empty() {
                        crate::services::open_url(&self.state.app_password_url);
                    }
                }
                if self.state.step != step::DONE {
                    let r = self.secondary.pointer(e, now);
                    repaint |= r.repaint;
                    if r.event.is_some() {
                        self.cancel(env);
                        return true;
                    }
                }
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if self.view != View::Flow {
            return false;
        }
        let now = env.now;
        // Yalnızca ilk basış: tuş tekrarı akışı ikinci kez göndermesin.
        if e.state == KeyState::Pressed {
            match e.key {
                Key::Enter => {
                    self.submit(env);
                    return true;
                }
                Key::Escape => {
                    self.cancel(env);
                    return true;
                }
                Key::Tab if self.second_field => {
                    self.focus = 1 - self.focus;
                    self.input.set_focused(self.focus == 0, now);
                    self.input2.set_focused(self.focus == 1, now);
                    return true;
                }
                _ => {}
            }
        }
        if !self.has_input() {
            return false;
        }
        let input = if self.focus == 1 && self.second_field { &mut self.input2 } else { &mut self.input };
        let r = input.key(e, now);
        if let Some(TextInputEvent::Submit) = r.event {
            self.submit(env);
            return true;
        }
        r.repaint || r.event.is_some()
    }

    fn focusable(&self) -> bool {
        self.view == View::Flow && self.has_input()
    }

    fn set_focus(&mut self, focused: bool, _visible: bool, now: Instant) {
        let on = focused && self.view == View::Flow && self.has_input();
        self.input.set_focused(on && self.focus == 0, now);
        self.input2.set_focused(on && self.focus == 1, now);
    }

    fn text_input(&mut self) -> Option<&mut TextInput> {
        if self.view != View::Flow || !self.has_input() {
            return None;
        }
        Some(if self.focus == 1 && self.second_field { &mut self.input2 } else { &mut self.input })
    }

    fn service_event(&mut self, env: &mut Env, event: &Event) -> bool {
        let Event::Calendar(u) = event else { return false };
        let t = env.theme;
        match u.clone() {
            Update::Snapshot(s) => {
                let s = *s;
                self.set_snapshot(t, s.accounts, s.calendars, s.providers);
            }
            Update::FlowStarted(id) => {
                self.flow = Some(id);
                self.view = View::Flow;
            }
            Update::Flow { flow, state } => {
                if self.flow.as_deref() == Some(flow.as_str()) {
                    if state.step == step::DONE {
                        Self::send(env, Command::Refresh);
                    }
                    self.apply_flow(t, *state, env.now);
                }
            }
            Update::FlowGone(flow) => {
                if self.flow.as_deref() == Some(flow.as_str()) {
                    self.flow = None;
                    self.view = View::List;
                }
            }
            Update::Unavailable(msg) => {
                self.status = Some(msg);
                self.busy = false;
                if self.view == View::Flow {
                    self.error.set_text(self.status.as_deref().unwrap_or(""));
                    self.state.step = step::FAILED.into();
                    self.question.set_text(&t!("settings-accounts-flow-failed"));
                    self.primary.set_label(&t!("settings-accounts-retry"));
                }
            }
            Update::Failed(msg) => {
                self.busy = false;
                if self.view == View::Flow {
                    self.error.set_text(&msg);
                } else {
                    self.toast.set_text(&msg);
                }
            }
        }
        true
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.status.is_some() && !self.loaded {
            return self.retry.cursor(p);
        }
        match self.view {
            View::List => self.add.cursor(p).or_else(|| {
                self.accounts.iter().find_map(|a| {
                    a.sync.cursor(p).or_else(|| a.remove.cursor(p)).or_else(|| a.reconnect.as_ref().and_then(|b| b.cursor(p))).or_else(|| a.calendars.iter().find_map(|c| c.toggle.cursor(p).or_else(|| c.swatch.contains(p).then_some(CursorShape::Pointer)).or_else(|| c.open.then(|| c.picker.cursor(p).or_else(|| c.reset.cursor(p))).flatten())))
                })
            }),
            View::Providers => self.back.cursor(p).or_else(|| self.providers.iter().any(|c| c.rect.contains(p)).then_some(CursorShape::Pointer)),
            View::Flow => self
                .back
                .cursor(p)
                .or_else(|| self.has_input().then(|| self.input.cursor(p).or_else(|| self.second_field.then(|| self.input2.cursor(p)).flatten())).flatten())
                .or_else(|| self.primary.cursor(p))
                .or_else(|| self.link.cursor(p))
                .or_else(|| self.secondary.cursor(p)),
        }
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for b in [&mut self.add, &mut self.back, &mut self.retry, &mut self.primary, &mut self.secondary, &mut self.link] {
            b.set_theme(theme, now);
        }
        self.input.set_theme(theme, now);
        self.input2.set_theme(theme, now);
        self.question.set_style(text_style(theme, TextRole::Title));
        self.empty.set_theme(theme, now);
        self.down.set_theme(theme, now);
        for l in [&mut self.hint, &mut self.error] {
            l.set_style(text_style(theme, TextRole::Body));
        }
        self.code.set_style(text_style(theme, TextRole::Display));
        self.pulse.set_motion(theme.motion.spatial, now);
        self.spinner.set_theme(theme, now);
        for a in &mut self.accounts {
            a.sync.set_theme(theme, now);
            a.remove.set_theme(theme, now);
            a.icon.set_theme(theme, now);
            if let Some(b) = &mut a.reconnect {
                b.set_theme(theme, now);
            }
            for c in &mut a.calendars {
                c.toggle.set_theme(theme, now);
                c.picker.set_theme(theme, now);
                c.reset.set_theme(theme, now);
            }
        }
        for p in &mut self.providers {
            p.hover.set_theme(theme, now);
            p.icon.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let r = self.rect;
        let x = r.x + px;
        if self.status.is_some() && !self.loaded {
            self.down.draw(ui);
            self.retry.draw(ui);
            return;
        }
        match self.view {
            View::List => {
                if self.accounts.is_empty() && self.loaded {
                    self.empty.draw(ui);
                } else if !self.loaded {
                    let mut l = Label::new(t!("settings-accounts-loading"), text_style(t, TextRole::Body));
                    l.draw(ui.painter, Point::new(x, r.y + py), ui.fade(c.on_surface_variant));
                }
                for a in &mut self.accounts {
                    let ar = a.rect;
                    ui.painter.rect(ar, &RectStyle { fill: ui.fade(c.surface), radii: t.shape.medium(), border: Some(Border { width: HAIRLINE, color: ui.fade(c.outline) }) });
                    let head_mid = ar.y + t.space(8);
                    a.icon.draw_centered(ui.painter, Point::new(ar.x + t.space(7), head_mid), ui.fade(c.primary));
                    let tx = ar.x + t.space(14);
                    let th = a.title.measure_in(ui.painter).height;
                    a.title.draw(ui.painter, Point::new(tx, head_mid - th - 1.0), ui.fade(c.on_surface));
                    let warn = a.entry.state == "needs-reconnect" || a.confirm_remove.is_some() || !a.entry.last_error.is_empty();
                    let sub_text = t!("settings-accounts-card-sub-status", sub = a.sub.text(), status = a.status.text());
                    let mut sub = Label::new(sub_text, text_style(t, TextRole::Caption));
                    sub.set_max_width(Some(ar.width - t.space(40)));
                    sub.draw(ui.painter, Point::new(tx, head_mid + 1.0), ui.fade(if warn { c.primary } else { c.on_surface_variant }));
                    a.sync.draw(ui);
                    a.remove.draw(ui);
                    if let Some(b) = &mut a.reconnect {
                        b.draw(ui);
                    }
                    for cal in &mut a.calendars {
                        let mid = cal.toggle.rect().center().y;
                        let sw = cal.swatch;
                        ui.painter.rect(sw, &RectStyle { fill: ui.fade(cal.color), radii: Corners::uniform(sw.height * 0.5), border: Some(Border { width: HAIRLINE, color: ui.fade(c.outline) }) });
                        let ns = cal.name.measure_in(ui.painter);
                        cal.name.draw(ui.painter, Point::new(sw.right() + t.space(3), mid - ns.height * 0.5), ui.fade(c.on_surface));
                        cal.toggle.draw(ui);
                        if cal.open {
                            cal.picker.draw(ui);
                            if cal.overridden {
                                cal.reset.draw(ui);
                            }
                        }
                    }
                }
                self.add.draw(ui);
            }
            View::Providers => {
                self.back.draw(ui);
                for p in &mut self.providers {
                    let pr = p.rect;
                    ui.painter.rect(pr, &RectStyle { fill: ui.fade(c.surface), radii: t.shape.medium(), border: Some(Border { width: HAIRLINE, color: ui.fade(c.outline) }) });
                    p.hover.draw_layer(ui.painter, pr, t.shape.medium(), t, now);
                    if p.hover.is_animating(now) {
                        ui.request_frame();
                    }
                    p.icon.draw_centered(ui.painter, Point::new(pr.x + t.space(10), pr.center().y), ui.fade(c.primary));
                    let tx = pr.x + t.space(19);
                    p.sub.set_max_width(Some(pr.right() - tx - t.space(3)));
                    let th = p.title.measure_in(ui.painter).height;
                    let sh = p.sub.measure_in(ui.painter).height;
                    let ty = pr.center().y - (th + sh + t.space(1)) * 0.5;
                    p.title.draw(ui.painter, Point::new(tx, ty), ui.fade(c.on_surface));
                    p.sub.draw(ui.painter, Point::new(tx, ty + th + t.space(1)), ui.fade(c.on_surface_variant));
                }
            }
            View::Flow => {
                self.back.draw(ui);
                let mut y = self.back.rect().bottom() + t.space(4);
                // Yeni adım hafifçe yukarı kayarak gelir.
                let p = self.pulse.value(now);
                if !self.pulse.is_settled(now) {
                    ui.request_frame();
                }
                y += (1.0 - p) * t.space(2);
                let qh = self.question.measure_in(ui.painter).height;
                self.question.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
                y += qh + t.space(2);
                if !self.hint.text().is_empty() {
                    let hh = self.hint.measure_in(ui.painter).height;
                    self.hint.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface_variant));
                    y += hh + t.space(4);
                }
                if !self.error.text().is_empty() {
                    let eh = self.error.measure_in(ui.painter).height;
                    let er = Rect::new(x - t.space(2), y - t.space(1), r.width - px * 2.0 + t.space(4), eh + t.space(2));
                    ui.painter.rect(er, &RectStyle::filled(ui.fade(state_layer_under(t, c.primary, 0.14, &[], &[c.on_surface])), t.shape.small()));
                    self.error.draw(ui.painter, Point::new(x, y), ui.fade(c.on_surface));
                }
                if self.has_input() {
                    self.input.draw(ui);
                    if self.second_field {
                        self.input2.draw(ui);
                    }
                }
                if self.show_code() {
                    let cy = self.primary.rect().y - t.space(4) - self.code.measure_in(ui.painter).height;
                    self.code.draw(ui.painter, Point::new(x, cy), ui.fade(c.primary));
                }
                if self.animating_wait() {
                    // Kısa bekleme: dönen gösterge (kendisi kare ister; tarayıcı beklemesinde yok).
                    let d = t.space(6);
                    self.spinner.set_rect(Rect::new(x, self.primary.rect().y - t.space(2) - d, d, d));
                    self.spinner.draw(ui);
                }
                if !self.waiting() || self.state.step == step::DONE {
                    self.primary.draw(ui);
                }
                let show_link = self.state.step == step::WAITING_BROWSER || (self.state.step == step::NEED_PASSWORD && !self.state.app_password_url.is_empty());
                if show_link {
                    self.link.draw(ui);
                }
                if self.state.step != step::DONE {
                    self.secondary.draw(ui);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(state: &str) -> AccountEntry {
        AccountEntry { state: state.into(), last_sync: 0, ..AccountEntry::default() }
    }

    #[test]
    fn status_shows_daemon_message_never_the_key() {
        crate::i18n::use_turkish_for_tests();
        let mut e = entry("needs-reconnect");
        e.state_reason = "calendar-err-auth-rejected".into();
        assert_eq!(account_status(&e), "Yeniden bağlanması gerekiyor");
        e.state_reason_message = "Sunucu giriş bilgisini kabul etmedi".into();
        assert_eq!(account_status(&e), "Sunucu giriş bilgisini kabul etmedi");

        let mut e = entry("active");
        e.last_error = "calendar-err-rate-limited".into();
        assert_eq!(account_status(&e), "Son senkron başarısız oldu");
        e.last_error_message = "Sağlayıcı istekleri sınırladı".into();
        assert_eq!(account_status(&e), "Sağlayıcı istekleri sınırladı");
        e.syncing = true;
        assert_eq!(account_status(&e), "Senkronlanıyor…");

        assert_eq!(account_status(&entry("active")), "henüz senkronlanmadı");
        for e in [entry("needs-reconnect"), entry("active")] {
            assert!(!account_status(&e).contains("calendar-err-"));
        }
    }

    /// Maskot yerleşimi: servis hatası (yüklenmeden) → Error; yüklenmiş boş liste → Empty;
    /// yüklenirken, hesap varken, akış/sağlayıcı ekranında → yok.
    #[test]
    fn mascot_spots_follow_state() {
        assert_eq!(accounts_spot(true, false, true, true), Some(Spot::AccountsUnavailable));
        assert_eq!(accounts_spot(true, false, false, true), Some(Spot::AccountsUnavailable));
        assert_eq!(accounts_spot(false, true, true, true), Some(Spot::AccountsEmpty));
        assert_eq!(accounts_spot(false, false, true, true), None, "yükleniyor");
        assert_eq!(accounts_spot(false, true, true, false), None, "hesap var");
        assert_eq!(accounts_spot(false, true, false, true), None, "akış ekranı");
        assert_eq!(Spot::AccountsUnavailable.mood(), nuxut::ui::Mood::Error);
        assert_eq!(Spot::AccountsEmpty.mood(), nuxut::ui::Mood::Empty);
    }
}
