//! `nuxut::App`: pencere, kenar çubuğu, sayfalar, config yazımı, geri al, IPC.

mod a11y;

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{HAIRLINE, hairline, state_layer_under};
use nuxut::ui::IconName;
use nuxut::ui::{
    Button, ButtonVariant, EmptyState, Icon, IconButton, IconButtonVariant, IconView, Label, List, ListDensity, ListEvent, ListItem, ListTrailing, Menu, MenuAnchor,
    MenuEvent, MenuItem, MenuModel, TextInput, TextInputEvent, TextRole, Ui, text_style,
};
use nuxut::wayland::{
    ImeEvent, ResizeEdge, Selection, SurroundingText, TextInputState, TextPurpose, TimerId, WindowSpec, WindowState, ipc,
};
use nuxut::{App, Context, Corners, OutputId, OutputInfo, Painter, Point, Rect, RectStyle, Size, SurfaceId, Theme};

use crate::block::{Action, Env};
use crate::history::{Change, History};
use crate::live_theme::LiveTheme;
use crate::page::Page;
use crate::pages::fiyu::Spot;
use crate::registry::{self, PageId, SettingDesc};
use crate::store::{DiskChange, FileId, Store};

/// Ardışık değişiklikler bu süre içinde tek yazımda birleşir.
pub const FLUSH_DELAY: Duration = Duration::from_millis(150);
/// Geri al şeridinin görünme süresi.
const SNACKBAR_TIME: Duration = Duration::from_secs(4);
/// Kenarlardan boyutlandırma payı.
const RESIZE_EDGE: f32 = 6.0;
const MENU_RECENT_BASE: u64 = 1000;

/// Ayarlar penceresinin sınıfı (Hyprland pencere kuralları, `.desktop` eşlemesi).
pub const SETTINGS_APP_ID: &str = "nuxut-settings";
/// Hoş geldin penceresinin sınıfı: Ayarlar'dan ayrı, compositor kuralı (float + ortala) buna yazılır.
pub const WELCOME_APP_ID: &str = "nuxut-welcome";
/// Hoş geldin penceresinin sabit mantıksal boyutu: metin sütunu (`space(150)`) iki yanda
/// `space(8)` payla sığar; adım satırları sayfa içinde kayar.
pub const WELCOME_SIZE: (u32, u32) = (720, 640);

/// Ayarlar penceresi: boyutlandırılabilir, döşenebilir.
fn settings_window_spec(page_title: &str) -> WindowSpec {
    WindowSpec { size: (1040, 720), min_size: Some((720, 480)), ..WindowSpec::new(t!("settings-app-window-title", page = page_title), SETTINGS_APP_ID) }
}

/// Hoş geldin penceresi: sabit boyut, `min_size = max_size` ipucu — compositor'lar (sway dahil)
/// böyle pencereyi yüzer açar; Hyprland kuralı shell'de (`nuxut-welcome` sınıfı).
fn welcome_window_spec() -> WindowSpec {
    WindowSpec { size: WELCOME_SIZE, min_size: Some(WELCOME_SIZE), max_size: Some(WELCOME_SIZE), ..WindowSpec::new(t!("settings-welcome-intro-title"), WELCOME_APP_ID) }
}

#[derive(Debug)]
pub enum Msg {
    Service(crate::services::Event),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    None,
    Search,
    Page,
}

#[derive(Debug)]
struct Snackbar {
    text: Label,
    undo: Button,
    /// "Geri al" düğmesi görünür mü (kopyalama gibi bildirimlerde gizli).
    undoable: bool,
    shown: nuxut::Animated<f32>,
    timer: Option<TimerId>,
    rect: Rect,
}

#[derive(Debug)]
pub struct Settings {
    /// Ayarlar penceresi (kenar çubuğu + sayfalar). Hoş geldin ayrı penceredir; süreç yalnızca
    /// Hoş geldin ile başlamış olabilir (`nuxut-settings welcome`), Ayarlar sonra açılabilir.
    window: Option<SurfaceId>,
    /// Servisler ve shell izleme bir kez başlatıldı (ilk çıkışta).
    started: bool,
    /// `nuxut-settings welcome` ile başladı: ilk açılışta Ayarlar penceresi açılmaz.
    welcome_only_start: bool,
    size: Size,
    state: WindowState,
    store: Store,
    history: History,
    live: LiveTheme,
    /// Bileşenlere son verilen tema (renk geçişi hariç token'lar).
    applied: Theme,
    flush_timer: Option<TimerId>,
    /// Bileşenlerin zamanlı uyanması (`Ui::wake_deadline`: değer balonunun sönmesi, eğri önizleme
    /// beklemesi); karede tek zamanlayıcı, beklerken kare çizilmez.
    wake_timer: Option<TimerId>,
    /// Hoş geldin penceresinin uyanma zamanlayıcısı (pencere başına bir tane).
    welcome_wake_timer: Option<TimerId>,
    /// "Yeni" rozeti görülme kayıtlarının toplu yazımı (`seen::FLUSH_DELAY`).
    seen_timer: Option<TimerId>,
    save_error: Option<String>,
    /// Bağlı monitörler (duvar kağıdı monitör başına satırları için).
    outputs: Vec<(OutputId, crate::services::outputs::Output)>,
    // Kenar çubuğu.
    app_title: Label,
    search: TextInput,
    search_icon: IconView,
    /// İkon şeridinde arama alanının yerini alan düğme (basınca kenar çubuğu açılır, alan odaklanır).
    search_btn: IconButton,
    nav: List,
    /// Kenar çubuğundaki sayfalar (`nav` öğeleriyle aynı sırada; veri yokken gizli sayfalar yok —
    /// `registry::nav_pages`). Pencere kurulurken bir kez alınır.
    nav_pages: Vec<PageId>,
    /// Kenar çubuğunun bu anki genişliği (şerit geçişinde `List::animated_width`); 0 = henüz
    /// yerleşim yok (ilk yerleşimde kip animasyonsuz kurulur).
    side_w: f32,
    results: List,
    result_ids: Vec<&'static str>,
    /// Arama sonuç vermedi: listenin yerine boş durum. Maskotlu ([`Spot::SearchNoResults`]) ve
    /// maskotsuz (sayfada zaten maskot varken; ekranda en çok bir maskot, [`search_spot`]).
    no_results: bool,
    search_empty: [EmptyState; 2],
    searching: bool,
    // İçerik.
    pages: Vec<Option<Page>>,
    current: PageId,
    page_title: Label,
    transition: nuxut::Animated<f32>,
    banner: Option<Label>,
    // Başlık çubuğu düğmeleri.
    undo_btn: IconButton,
    history_btn: IconButton,
    min_btn: IconButton,
    max_btn: IconButton,
    close_btn: IconButton,
    recent_menu: Menu,
    snackbar: Snackbar,
    focus: Focus,
    pending_actions: Vec<Action>,
    start: Instant,
    first_frame_logged: bool,
    /// Ekran okuyucu: tıklanabilir düğüm → işaretçi noktası (son ağaçtan; `app/a11y.rs`).
    access_points: std::collections::HashMap<nuxut::access::NodeId, Point>,
    /// `NUXUT_SETTINGS_DEBUG`: canlı testlerde başlangıç durumu (bkz. `run_debug_steps`).
    debug_steps: std::collections::VecDeque<String>,
    debug_timer: Option<TimerId>,
    /// `--flow <id>`: pencere açılınca Hesaplar sayfasında bu akış gösterilir.
    start_flow: Option<String>,
    debug_done: bool,
    /// Son karede kaplamaların girdi bölgeleri (sayfadaki açık menü).
    overlay: nuxut::ui::OverlayRegions,
    /// Hoş geldin akışı (`welcome_ui`); açıkken kendi penceresi var (`welcome_window`).
    welcome: Option<crate::welcome_ui::WelcomeView>,
    /// Hoş geldin penceresi: ayrı xdg_toplevel (`nuxut-welcome`, sabit 720×640). Kapanınca
    /// yalnızca o gider; Ayarlar açık kalır.
    welcome_window: Option<SurfaceId>,
    welcome_size: Size,
    /// Hoş geldin başlık çubuğundaki tek düğme (kapatmak akışı atlar).
    welcome_close: IconButton,
    /// Hoş geldin "Kapat" ile bitti (kapanışta `setup done`, yoksa `setup skip`).
    welcome_done: bool,
    /// Sonuç shell'e bir kez bildirildi.
    welcome_reported: bool,
}

/// Kenar çubuğunun tam genişliği (şerit değilken).
fn sidebar_width(theme: &Theme) -> f32 {
    theme.space(68)
}

/// Bu mantıksal genişliğin altında kenar çubuğu ikon şeridine iner (§4: `space(225)` = 900 px;
/// ölçekten bağımsız, mantıksal genişliğe bakar).
fn rail_below(theme: &Theme) -> f32 {
    theme.space(225)
}

pub(crate) fn header_height(theme: &Theme) -> f32 {
    theme.space(16)
}

impl Settings {
    pub fn new(store: Store, start_page: PageId, start_flow: Option<String>, start: Instant) -> Self {
        // Sayı gösterimleri (kaydırıcı değerleri vb.) satırlar kurulmadan önce etkin biçim
        // locale'ini bilsin.
        crate::numfmt::configure(&store);
        let live = LiveTheme::new(&store);
        let theme = live.theme().clone();
        let now = Instant::now();
        let nav_pages = registry::nav_pages();
        // Gizli sayfayla açıldı (`nuxut-settings power`, güç verisi yok): ilk sayfa.
        let start_page = if nav_pages.contains(&start_page) { start_page } else { nav_pages.first().copied().unwrap_or(PageId::Appearance) };
        let nav_items = nav_pages
            .iter()
            .map(|p| {
                let mut item = ListItem::new(&theme, p.title()).with_leading(&theme, Icon::named(p.icon()));
                item.set_selected(*p == start_page);
                item
            })
            .collect();
        let mut snackbar_shown = nuxut::Animated::new(0.0, theme.motion.spatial_fast);
        snackbar_shown.jump_to(0.0);
        let mut transition = nuxut::Animated::new(1.0, theme.motion.spatial_fast);
        transition.jump_to(1.0);
        let _ = now;
        Self {
            window: None,
            started: false,
            welcome_only_start: false,
            size: Size::ZERO,
            state: WindowState::default(),
            history: History::new(),
            applied: theme.clone(),
            flush_timer: None,
            wake_timer: None,
            welcome_wake_timer: None,
            seen_timer: None,
            save_error: None,
            outputs: Vec::new(),
            app_title: Label::new(t!("settings-app-title"), text_style(&theme, TextRole::Title)),
            search: TextInput::new(&theme).with_placeholder(t!("settings-app-search-placeholder")).with_max_bytes(128),
            search_icon: IconView::with_role(&theme, Icon::named(IconName::Search), TextRole::IconSmall),
            search_btn: IconButton::new(&theme, Icon::named(IconName::Search), IconButtonVariant::Standard),
            nav: List::new(&theme).with_items(nav_items).with_groups(&theme, &registry::nav_groups(&nav_pages)).with_density(ListDensity::Compact),
            nav_pages,
            side_w: 0.0,
            results: List::new(&theme),
            result_ids: Vec::new(),
            searching: false,
            no_results: false,
            search_empty: [
                EmptyState::new(&theme, t!("settings-app-search-empty")).with_mascot(&theme, Spot::SearchNoResults.mood()),
                EmptyState::new(&theme, t!("settings-app-search-empty")),
            ],
            pages: PageId::ALL.iter().map(|_| None).collect(),
            current: start_page,
            page_title: Label::new(start_page.title(), text_style(&theme, TextRole::Title)),
            transition,
            banner: None,
            undo_btn: IconButton::new(&theme, Icon::named(IconName::Undo), IconButtonVariant::Standard),
            history_btn: IconButton::new(&theme, Icon::named(IconName::History), IconButtonVariant::Standard),
            min_btn: IconButton::new(&theme, Icon::named(IconName::Minimize), IconButtonVariant::Standard),
            max_btn: IconButton::new(&theme, Icon::named(IconName::Maximize), IconButtonVariant::Standard),
            close_btn: IconButton::new(&theme, Icon::named(IconName::Close), IconButtonVariant::Standard),
            recent_menu: Menu::new(&theme),
            snackbar: Snackbar {
                text: Label::new("", text_style(&theme, TextRole::Body)),
                undo: Button::new(&theme, ButtonVariant::Text).with_label(&theme, t!("settings-app-snackbar-undo")),
                undoable: true,
                shown: snackbar_shown,
                timer: None,
                rect: Rect::ZERO,
            },
            focus: Focus::None,
            pending_actions: Vec::new(),
            start,
            first_frame_logged: false,
            access_points: Default::default(),
            debug_steps: std::env::var("NUXUT_SETTINGS_DEBUG").map(|s| s.split(';').map(str::to_owned).collect()).unwrap_or_default(),
            debug_timer: None,
            start_flow,
            debug_done: std::env::var_os("NUXUT_SETTINGS_DEBUG").is_none(),
            overlay: nuxut::ui::OverlayRegions::default(),
            welcome: None,
            welcome_window: None,
            welcome_size: Size::ZERO,
            welcome_close: IconButton::new(&theme, Icon::named(IconName::Close), IconButtonVariant::Standard),
            welcome_done: false,
            welcome_reported: false,
            store,
            live,
        }
    }

    fn theme(&self) -> &Theme {
        &self.applied
    }

    fn page_index(id: PageId) -> usize {
        PageId::ALL.iter().position(|p| *p == id).unwrap_or(0)
    }

    fn content_rect(&self) -> Rect {
        let t = self.theme();
        let x = self.side_width();
        let y = header_height(t);
        Rect::new(x, y, (self.size.width - x).max(0.0), (self.size.height - y).max(0.0))
    }

    fn ensure_page(&mut self, id: PageId) {
        let i = Self::page_index(id);
        if self.pages[i].is_none() {
            let theme = self.applied.clone();
            let blocks = crate::pages::build(&theme, id, &self.store);
            let mut page = Page::new(&theme, id, blocks, &self.store);
            page.set_viewport(self.content_rect());
            self.pages[i] = Some(page);
        }
    }

    fn page_mut(&mut self) -> &mut Page {
        self.ensure_page(self.current);
        self.pages[Self::page_index(self.current)].as_mut().expect("sayfa az önce yaratıldı")
    }

    fn navigate(&mut self, cx: &mut Context<Self>, id: PageId, now: Instant) {
        // Gizli sayfaya (ör. IPC ile `power`, güç verisi yok) gidilmez.
        if !self.nav_pages.contains(&id) {
            return;
        }
        // Ekranlar yalnızca görünürken shell'i yoklar (sayfa kendisi `place`'te açar).
        crate::services::displays::shell::watch(id == PageId::Displays);
        // Wi-Fi taraması yalnız Wi-Fi sayfası görünürken (açılınca bir kez; docs/design/network-pages.md).
        crate::services::network::set_wifi_visible(id == PageId::Wifi);
        // Bluetooth taraması yalnız Bluetooth sayfası görünürken (docs/design/bluetooth-page.md).
        crate::services::bluetooth::set_page_visible(id == PageId::Bluetooth);
        // Yazıcılar yalnız sayfa görünürken yoklanır (docs/design/printers.md §3).
        crate::services::printers::set_page_visible(id == PageId::Printers);
        // Büyüteç ve ekran klavyesi yalnız Erişilebilirlik görünürken yoklanır (kısayolla değişen durum görünsün).
        crate::services::assist::set_page_visible(id == PageId::Accessibility);
        // Tarih ve saat: NTPSynchronized/TimeUSec sinyal vermez; sayfa açılınca yeniden okunur.
        if id == PageId::DateTime {
            crate::services::timedate::recheck();
        }
        // Kullanıcı hesabı: başka araçla (ör. GNOME, usermod) değişmiş olabilir.
        if id == PageId::User {
            crate::services::accounts::recheck();
        }
        // Gece ışığı konum satırı (Ekranlar): sayfa her açılışta durumu bir kez sorar (yoklama yok).
        if id == PageId::Displays {
            crate::services::night_light::refresh(std::time::Duration::ZERO);
        }
        if id != self.current {
            if let Some(p) = self.pages[Self::page_index(self.current)].as_mut() {
                p.clear_focus(now);
            }
            // Sayfadan ayrılındı: orada görülen satırların "Yeni" rozeti kalkar.
            crate::services::seen::retire();
            self.current = id;
            self.page_title.set_text(id.title());
            self.transition.jump_to(0.0);
            self.transition.set_target(1.0, now);
            for (i, item) in self.nav.items_mut().iter_mut().enumerate() {
                item.set_selected(self.nav_pages.get(i) == Some(&id));
            }
            let r = self.nav_rect();
            let theme = self.applied.clone();
            self.nav.set_rect(&theme, r, now);
            if let Some(w) = self.window {
                cx.set_window_title(w, &t!("settings-app-window-title", page = id.title()));
            }
        }
        self.ensure_page(id);
        if self.focus == Focus::Page {
            self.focus = Focus::None;
        }
        self.request_frame(cx);
    }

    /// İki pencereye de kare ister (ortak durum: tema, kayıt, servis olayları).
    fn request_frame(&self, cx: &mut Context<Self>) {
        for w in [self.window, self.welcome_window].into_iter().flatten() {
            cx.request_frame(w);
        }
    }

    fn track_output(&mut self, cx: &mut Context<Self>, info: &OutputInfo) {
        let o = crate::services::outputs::Output { name: info.name.clone(), description: info.description.clone() };
        match self.outputs.iter_mut().find(|(id, _)| *id == info.id) {
            Some((_, cur)) => *cur = o,
            None => self.outputs.push((info.id, o)),
        }
        self.publish_outputs(cx);
    }

    /// Monitör listesini yayımlar; değiştiyse açık sayfalara bildirir.
    fn publish_outputs(&mut self, cx: &mut Context<Self>) {
        if crate::services::outputs::set(self.outputs.iter().map(|(_, o)| o.clone()).collect()) && self.started {
            self.message(cx, Msg::Service(crate::services::Event::Outputs));
        }
    }

    // ---- Yerleşim ---------------------------------------------------------------------------

    /// Kenar çubuğunun bu anki genişliği (ilk yerleşimden önce tam genişlik).
    fn side_width(&self) -> f32 {
        if self.side_w > 0.0 { self.side_w } else { sidebar_width(self.theme()) }
    }

    /// Şerit geçişinin ilerlemesi (0 = tam kenar çubuğu, 1 = ikon şeridi).
    fn rail_progress(&self, now: Instant) -> f32 {
        self.nav.rail_progress(now)
    }

    /// İkon şeridi isteniyor mu (§4): pencere dar ve arama kullanılmıyor. Arama odaklanınca ya
    /// da metin varken kenar çubuğu tam genişliğe açılır (alan ve sonuçlar okunsun), bitince
    /// şeride döner.
    fn wants_rail(&self) -> bool {
        self.size.width > 0.0
            && self.size.width < rail_below(self.theme())
            && self.focus != Focus::Search
            && self.search.text().is_empty()
    }

    /// Şerit kipini ve kenar çubuğu genişliğini günceller; genişlik değiştiyse yeniden yerleştirir.
    /// İlk yerleşimde kip animasyonsuz kurulur (dar açılan pencere şeritle başlar).
    fn sync_rail(&mut self, now: Instant) {
        let theme = self.applied.clone();
        let want = self.wants_rail();
        if self.side_w <= 0.0 {
            let nav = std::mem::replace(&mut self.nav, List::new(&theme));
            self.nav = nav.with_rail(want);
        } else {
            self.nav.set_rail(want, now);
        }
        let w = self.nav.animated_width(&theme, sidebar_width(&theme), now);
        if w != self.side_w {
            self.side_w = w;
            self.layout_sidebar(now);
        }
    }

    fn search_rect(&self) -> Rect {
        let t = self.theme();
        let pad = t.space(3);
        let w = (self.side_width() - pad * 2.0).max(0.0);
        let h = self.search.preferred_size(t, sidebar_width(t) - pad * 2.0).height;
        Rect::new(pad, header_height(t), w, h)
    }

    fn nav_rect(&self) -> Rect {
        let t = self.theme();
        let s = self.search_rect();
        let top = s.bottom() + t.space(3);
        // Tam kipte iki yanda `space(2)` boşluk; şeritte liste kenar çubuğunun tamamıdır
        // (`List::rail_width`).
        let full = sidebar_width(t);
        let rail = List::rail_width(t);
        let p = ((full - self.side_width()) / (full - rail)).clamp(0.0, 1.0);
        let inset = (t.space(2) * (1.0 - p)).round();
        Rect::new(inset, top, (self.side_width() - inset * 2.0).max(0.0), (self.size.height - top - t.space(2)).max(0.0))
    }

    /// Kenar çubuğu parçaları + içerik alanı (genişlik değişince).
    fn layout_sidebar(&mut self, now: Instant) {
        let theme = self.applied.clone();
        let t = &theme;
        self.search.set_rect(self.search_rect());
        let s = self.search_rect();
        let bs = self.search_btn.preferred_size(t);
        self.search_btn.set_rect(Rect::new(((self.side_width() - bs.width) * 0.5).round(), (s.center().y - bs.height * 0.5).round(), bs.width, bs.height));
        let nav = self.nav_rect();
        self.nav.set_rect(t, nav, now);
        self.results.set_rect(t, nav, now);
        let content = self.content_rect();
        for p in self.pages.iter_mut().flatten() {
            p.set_viewport(content);
        }
    }

    fn layout(&mut self, cx: &mut Context<Self>, now: Instant) {
        let theme = self.applied.clone();
        let t = &theme;
        self.sync_rail(now);
        self.layout_sidebar(now);
        let hh = header_height(t);
        let bs = self.close_btn.preferred_size(t);
        let mut x = self.size.width - t.space(2) - bs.width;
        let y = (hh - bs.height) * 0.5;
        for b in [&mut self.close_btn, &mut self.max_btn, &mut self.min_btn] {
            b.set_rect(Rect::new(x, y, bs.width, bs.height));
            x -= bs.width;
        }
        x -= t.space(3);
        for b in [&mut self.history_btn, &mut self.undo_btn] {
            b.set_rect(Rect::new(x, y, bs.width, bs.height));
            x -= bs.width;
        }
        let _ = cx;
    }

    // ---- Değişiklik akışı ------------------------------------------------------------------

    /// Blok eylemlerini uygular: belleğe yaz, geçmişe kaydet, yazım timer'ı, tema güncelle.
    fn apply_actions(&mut self, cx: &mut Context<Self>, now: Instant) {
        let actions = std::mem::take(&mut self.pending_actions);
        if actions.is_empty() {
            return;
        }
        let mut changed = false;
        let mut last_label = None;
        for a in actions {
            tracing::debug!(?a, "eylem");
            match a {
                Action::Set { label, file, path, value } => {
                    let before = self.store.set(file, &path, value.clone());
                    self.history.record(label.clone(), vec![Change { file, path, before, after: value }], now);
                    last_label = Some(label);
                    changed = true;
                }
                Action::Group { label, writes } => {
                    let mut changes = Vec::new();
                    for (file, path, value) in writes {
                        let before = self.store.set(file, &path, value.clone());
                        changes.push(Change { file, path, before, after: value });
                    }
                    self.history.record(label.clone(), changes, now);
                    last_label = Some(label);
                    changed = true;
                }
                Action::Seal => self.history.seal(),
                Action::Navigate(p) => self.navigate(cx, p, now),
                Action::Reveal(id) => self.reveal(cx, id, now),
                Action::Focus => {
                    self.set_focus(cx, Focus::Page, now);
                }
                Action::Service(crate::services::Request::Snapshots(crate::services::snapshots::Command::Restore { id, setting, label })) => {
                    // Karar: geri yüklemeyi shell tek işlemde yapar, framework.toml
                    // seçimi dahil. Ayarlar bu sırada dosyaya yazmaz; bekleyen yazım önce gider ki
                    // shell'in yazımıyla yarışmasın. Yeni değer dosya izleyicisinden gelir.
                    if let Some(t) = self.flush_timer.take() {
                        cx.cancel_timer(t);
                    }
                    self.flush(cx);
                    self.show_notice(cx, &t!("settings-app-restoring", label = label.as_str()), now);
                    crate::services::dispatch(crate::services::Request::Snapshots(crate::services::snapshots::Command::Restore { id, setting, label }));
                }
                Action::Service(req) => crate::services::dispatch(req),
                Action::Copy(text) => match cx.set_clipboard(Selection::Clipboard, &text) {
                    Ok(()) => self.show_notice(cx, &t!("settings-app-copied"), now),
                    Err(e) => {
                        tracing::warn!(%e, "kopyalanamadı");
                        self.show_notice(cx, &t!("settings-app-error-copy", error = e.to_string()), now);
                    }
                },
            }
        }
        if changed {
            self.after_store_change(cx, now, true);
            if let Some(label) = last_label {
                self.show_snackbar(cx, &t!("settings-app-changed", label = label.as_str()), now);
            }
        }
    }

    /// Store değişti (kullanıcı, geri al ya da dış düzenleme).
    fn after_store_change(&mut self, cx: &mut Context<Self>, now: Instant, schedule_flush: bool) {
        if schedule_flush && self.flush_timer.is_none() && self.store.is_dirty() {
            self.flush_timer = Some(cx.set_timer(FLUSH_DELAY));
        }
        if self.live.update(&self.store, now) {
            self.apply_theme(cx, now);
        }
        self.sync_pages(cx, now);
        self.update_banner();
        self.request_frame(cx);
    }

    fn sync_pages(&mut self, cx: &mut Context<Self>, now: Instant) {
        let theme = self.applied.clone();
        let mut sink = Vec::new();
        let store = &self.store;
        for p in self.pages.iter_mut().flatten() {
            let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut sink };
            p.sync(&mut env);
        }
        if let Some(w) = &mut self.welcome {
            let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut sink };
            w.sync(&mut env);
        }
    }

    fn apply_theme(&mut self, cx: &mut Context<Self>, now: Instant) {
        let theme = self.live.theme().clone();
        let t = &theme;
        self.app_title.set_style(text_style(t, TextRole::Title));
        self.page_title.set_style(text_style(t, TextRole::Title));
        self.search.set_theme(t, now);
        self.search_icon.set_theme(t, now);
        self.nav.set_theme(t, now);
        self.results.set_theme(t, now);
        for e in &mut self.search_empty {
            e.set_theme(t, now);
        }
        for b in [&mut self.search_btn, &mut self.undo_btn, &mut self.history_btn, &mut self.min_btn, &mut self.max_btn, &mut self.close_btn] {
            b.set_theme(t, now);
        }
        self.recent_menu.set_theme(cx.text(), t, now);
        self.snackbar.text.set_style(text_style(t, TextRole::Body));
        self.snackbar.undo.set_theme(t, now);
        self.snackbar.shown.set_motion(t.motion.spatial_fast, now);
        self.transition.set_motion(t.motion.spatial_fast, now);
        for p in self.pages.iter_mut().flatten() {
            p.set_theme(t, now);
        }
        if let Some(w) = &mut self.welcome {
            w.set_theme(t, now);
        }
        self.welcome_close.set_theme(t, now);
        self.applied = theme;
        self.layout(cx, now);
    }

    fn flush(&mut self, cx: &mut Context<Self>) {
        self.flush_timer = None;
        self.store.set_shell_config_version(crate::services::widgets::shell_config_version());
        let errors = self.store.flush();
        // Kapanışta (ve her config yazımında) bekleyen görülme kayıtları da yazılır.
        crate::services::seen::save();
        if let Some(req) = self.store.take_palette_request() {
            crate::services::dispatch(crate::services::Request::Palette(crate::services::palette::Command::Apply(req)));
        }
        self.save_error = errors.first().map(|(f, e)| t!("settings-app-error-save", file = f.name(), error = e.to_string()));
        if let Some(msg) = self.save_error.clone() {
            self.show_snackbar(cx, &msg, Instant::now());
        }
    }

    fn update_banner(&mut self) {
        let broken = FileId::ALL
            .into_iter()
            .find_map(|f| self.store.file(f).broken().map(|e| t!("settings-app-error-broken-file", file = f.name(), error = e.to_string())));
        match (broken, &mut self.banner) {
            (Some(text), Some(label)) => {
                label.set_text(&text);
            }
            (Some(text), None) => self.banner = Some(Label::new(text, text_style(&self.applied, TextRole::Body))),
            (None, _) => self.banner = None,
        }
    }

    fn show_snackbar(&mut self, cx: &mut Context<Self>, text: &str, now: Instant) {
        self.snackbar.undoable = true;
        self.show_strip(cx, text, now);
    }

    /// Geri alınamayan bildirim (ör. "Panoya kopyalandı"): şerit "Geri al" düğmesiz.
    fn show_notice(&mut self, cx: &mut Context<Self>, text: &str, now: Instant) {
        self.snackbar.undoable = false;
        self.show_strip(cx, text, now);
    }

    fn show_strip(&mut self, cx: &mut Context<Self>, text: &str, now: Instant) {
        self.snackbar.text.set_text(text);
        self.snackbar.shown.set_target(1.0, now);
        if let Some(t) = self.snackbar.timer.take() {
            cx.cancel_timer(t);
        }
        self.snackbar.timer = Some(cx.set_timer(SNACKBAR_TIME));
        self.request_frame(cx);
    }

    fn undo(&mut self, cx: &mut Context<Self>, redo: bool, now: Instant) {
        let entry = if redo { self.history.redo().cloned() } else { self.history.undo().cloned() };
        let Some(entry) = entry else { return };
        let writes: Vec<_> =
            if redo { entry.redo_writes().map(|(f, p, v)| (f, p.clone(), v)).collect() } else { entry.undo_writes().map(|(f, p, v)| (f, p.clone(), v)).collect() };
        for (f, p, v) in writes {
            self.store.set(f, &p, v);
        }
        self.after_store_change(cx, now, true);
        let text = if redo {
            t!("settings-app-history-redone", label = entry.label.as_str())
        } else {
            t!("settings-app-history-undone", label = entry.label.as_str())
        };
        self.show_snackbar(cx, &text, now);
    }

    fn revert_recent(&mut self, cx: &mut Context<Self>, index: usize, now: Instant) {
        let Some(inverse) = self.history.revert(index, now) else { return };
        for (f, p, v) in inverse.redo_writes() {
            let p = p.clone();
            self.store.set(f, &p, v);
        }
        self.after_store_change(cx, now, true);
        self.show_snackbar(cx, &inverse.label, now);
    }

    fn open_recent_menu(&mut self, cx: &mut Context<Self>, now: Instant) {
        let mut items: Vec<MenuItem> = self
            .history
            .recent(10)
            .map(|(i, e)| MenuItem::action(MENU_RECENT_BASE + i as u64, e.label.clone()).with_icon(Icon::named(IconName::ArrowBack)))
            .collect();
        if items.is_empty() {
            items.push(MenuItem::action(1, t!("settings-app-no-changes")).enabled(false));
        }
        let theme = self.applied.clone();
        let anchor = MenuAnchor::below(self.history_btn.rect());
        let bounds = Rect::new(0.0, 0.0, self.size.width, self.size.height).inset(nuxut::Insets::uniform(theme.space(2)));
        self.recent_menu.open(cx.text(), &theme, MenuModel::new(items), anchor, bounds, now);
    }

    // ---- Arama ------------------------------------------------------------------------------

    fn update_search(&mut self, now: Instant) {
        let q = self.search.text().trim().to_owned();
        self.searching = !q.is_empty();
        if !self.searching {
            return;
        }
        let theme = self.applied.clone();
        let found: Vec<&'static SettingDesc> = registry::search(&self.store, &q);
        self.result_ids = found.iter().map(|d| d.id).collect();
        let mut items: Vec<ListItem> = found
            .iter()
            .take(50)
            .map(|d| {
                ListItem::new(&theme, d.label())
                    .with_subtitle(&theme, t!("settings-app-result-subtitle", page = d.page.title(), section = d.section_title()))
                    .with_leading(&theme, Icon::named(d.page.icon()))
                    // "Yeni" (görülme sayılmaz; satırın kendisi görülünce kalkar).
                    .with_trailing(&theme, if crate::services::seen::badge(d.path, d.since()) { ListTrailing::Pill(t!("settings-badge-new")) } else { ListTrailing::None })
            })
            .collect();
        // Sayfa adları da aranır.
        let fq = registry::fold(&q);
        for &p in &self.nav_pages {
            if registry::fold(p.title()).contains(&fq) && !found.iter().any(|d| d.page == p && d.label() == p.title()) {
                items.insert(0, ListItem::new(&theme, p.title()).with_subtitle(&theme, t!("settings-app-result-page")).with_leading(&theme, Icon::named(p.icon())));
                self.result_ids.insert(0, p.slug());
            }
        }
        // Sonuç yok: liste boş kalır, yerine boş durum çizilir (maskot: sonuç yok).
        self.no_results = items.is_empty();
        if self.no_results {
            self.result_ids.clear();
            for e in &mut self.search_empty {
                e.set_title(&t!("settings-app-search-empty"));
                e.set_body(&theme, Some(&t!("settings-app-search-empty-hint")));
            }
        }
        self.results = List::new(&theme).with_items(items);
        let r = self.nav_rect();
        self.results.set_rect(&theme, r, now);
    }

    fn open_result(&mut self, cx: &mut Context<Self>, index: usize, now: Instant) {
        let Some(id) = self.result_ids.get(index).copied() else { return };
        self.reveal(cx, id, now);
    }

    /// Sayfa kimliği o sayfayı, ayar kimliği sayfasını açıp satıra kaydırır ve vurgular.
    fn reveal(&mut self, cx: &mut Context<Self>, id: &'static str, now: Instant) {
        if let Some(p) = PageId::from_slug(id) {
            self.navigate(cx, p, now);
            return;
        }
        let Some(desc) = registry::by_id(id) else { return };
        self.navigate(cx, desc.page, now);
        let theme = self.applied.clone();
        let mut sink = Vec::new();
        let store = &self.store;
        let i = Self::page_index(desc.page);
        if let Some(p) = self.pages[i].as_mut() {
            let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut sink };
            p.reveal(&mut env, id);
        }
    }

    fn set_focus(&mut self, cx: &mut Context<Self>, focus: Focus, now: Instant) {
        if self.focus == focus {
            self.update_ime(cx);
            return;
        }
        if self.focus == Focus::Search {
            self.search.set_focused(false, now);
        }
        if self.focus == Focus::Page && focus != Focus::Page {
            self.page_mut().clear_focus(now);
        }
        self.focus = focus;
        if focus == Focus::Search {
            self.search.set_focused(true, now);
        }
        // Dar pencerede arama kenar çubuğunu açar, bitince şeride döner.
        self.sync_rail(now);
        self.update_ime(cx);
    }

    fn update_ime(&mut self, cx: &mut Context<Self>) {
        let Some(w) = self.window else { return };
        let state = match self.focus {
            Focus::Search => Some(self.search.ime_state()),
            Focus::Page => self.page_mut().focused_text_input().map(|i| i.ime_state()),
            Focus::None => None,
        };
        cx.set_text_input(w, state.map(text_input_state));
    }

    /// Canlı test senaryosu (`NUXUT_SETTINGS_DEBUG`, adımlar `;` ile): `search=<metin>`,
    /// `reveal=<ayar>`, `expand=<ayar>`, `page=<sayfa>`, `click=<x>,<y>` (pencereye göre),
    /// `hover=<x>,<y>` (yalnızca hareket),
    /// `type=<metin>`, `key=enter|escape|tab`, `wait=<ms>`, `avatar=<yol>` (profil resmi seçimi), `wgimport=<yol>` (WireGuard içe aktarma). Girdi uygulamanın içinde
    /// üretilir: gerçek fare/klavye kullanılmaz, kullanıcının odağına dokunulmaz. Bitince
    /// stdout'a `SETTINGS_SCRIPT_DONE` yazar (ekran görüntüsü zamanlaması).
    fn run_debug_steps(&mut self, cx: &mut Context<Self>) {
        // Hoş geldin ile başlamış süreçte (Ayarlar penceresi yok) adımlar Hoş geldin penceresine.
        let Some(surface) = self.window.or(self.welcome_window) else { return };
        while let Some(step) = self.debug_steps.pop_front() {
            let now = Instant::now();
            let (cmd, arg) = step.split_once('=').unwrap_or((step.as_str(), ""));
            match cmd {
                "search" => {
                    self.search.set_text(arg);
                    self.set_focus(cx, Focus::Search, now);
                    self.update_search(now);
                }
                "reveal" => {
                    if let Some(i) = self.result_index_for(arg) {
                        self.open_result(cx, i, now);
                    } else if let Some(d) = registry::by_id(arg) {
                        self.result_ids = vec![d.id];
                        self.open_result(cx, 0, now);
                    }
                }
                "expand" => {
                    if let Some(d) = registry::by_id(arg) {
                        self.navigate(cx, d.page, now);
                        self.page_mut().demo_expand(arg, now);
                    }
                }
                "page" => {
                    if let Some(p) = PageId::from_slug(arg) {
                        self.navigate(cx, p, now);
                    }
                }
                "click" => {
                    tracing::debug!(arg, "test tıklaması");
                    let mut it = arg.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
                    if let (Some(x), Some(y)) = (it.next(), it.next()) {
                        let p = Point::new(x, y);
                        for kind in [PointerKind::Motion, PointerKind::Press(MouseButton::Left), PointerKind::Release(MouseButton::Left)] {
                            self.pointer(cx, surface, PointerEvent::new(p, kind));
                        }
                    }
                }
                "hover" => {
                    // Yalnızca işaretçi hareketi (ör. imleç önizlemesinin animasyonu).
                    let mut it = arg.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
                    if let (Some(x), Some(y)) = (it.next(), it.next()) {
                        self.pointer(cx, surface, PointerEvent::new(Point::new(x, y), PointerKind::Motion));
                    }
                }
                "drag" => {
                    let v: Vec<f32> = arg.split(',').filter_map(|v| v.trim().parse().ok()).collect();
                    if let [x1, y1, x2, y2] = v[..] {
                        self.pointer(cx, surface, PointerEvent::new(Point::new(x1, y1), PointerKind::Motion));
                        self.pointer(cx, surface, PointerEvent::new(Point::new(x1, y1), PointerKind::Press(MouseButton::Left)));
                        for i in 1..=12 {
                            let t = i as f32 / 12.0;
                            let p = Point::new(x1 + (x2 - x1) * t, y1 + (y2 - y1) * t);
                            self.pointer(cx, surface, PointerEvent::new(p, PointerKind::Motion));
                        }
                        self.pointer(cx, surface, PointerEvent::new(Point::new(x2, y2), PointerKind::Release(MouseButton::Left)));
                    }
                }
                "type" => {
                    for ch in arg.chars() {
                        let ev = KeyEvent::new(Key::Char(ch), KeyState::Pressed, Some(ch.to_string()), Default::default());
                        self.keyboard(cx, surface, ev);
                    }
                }
                "key" => {
                    let key = match arg {
                        "enter" => Key::Enter,
                        "escape" => Key::Escape,
                        "tab" => Key::Tab,
                        "down" => Key::Down,
                        "up" => Key::Up,
                        _ => continue,
                    };
                    for state in [KeyState::Pressed, KeyState::Released] {
                        self.keyboard(cx, surface, KeyEvent::new(key, state, None, Default::default()));
                    }
                }
                "wallpaper" => {
                    // Portal seçimini taklit eder (gerçek dosya seçici açılmaz).
                    let ev = crate::services::Event::Portal(crate::services::portal::Update::Picked { tag: "wallpaper".into(), path: arg.into() });
                    self.message(cx, Msg::Service(ev));
                }
                "avatar" => {
                    // Profil resmi dosya seçicisinin seçimini taklit eder (portal açılmaz).
                    crate::services::accounts::set_avatar_file(arg.into());
                }
                "wgimport" => {
                    // WireGuard dosya seçicisinin seçimini taklit eder (portal açılmaz).
                    crate::services::network::import_file(std::path::Path::new(arg));
                }
                "wait" => {
                    let ms = arg.parse().unwrap_or(500);
                    self.debug_timer = Some(cx.set_timer(Duration::from_millis(ms)));
                    return;
                }
                _ => tracing::warn!("bilinmeyen test adımı: {step}"),
            }
        }
        if !self.debug_done {
            self.debug_done = true;
            println!("SETTINGS_SCRIPT_DONE");
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        cx.request_frame(surface);
    }

    fn result_index_for(&self, id: &str) -> Option<usize> {
        self.result_ids.iter().position(|r| *r == id)
    }

    fn edge_at(&self, p: Point) -> Option<ResizeEdge> {
        if self.state.fullscreen {
            return None;
        }
        let (l, r) = (p.x < RESIZE_EDGE, p.x > self.size.width - RESIZE_EDGE);
        let (t, b) = (p.y < RESIZE_EDGE, p.y > self.size.height - RESIZE_EDGE);
        Some(match (t, b, l, r) {
            (true, _, true, _) => ResizeEdge::TopLeft,
            (true, _, _, true) => ResizeEdge::TopRight,
            (_, true, true, _) => ResizeEdge::BottomLeft,
            (_, true, _, true) => ResizeEdge::BottomRight,
            (true, ..) => ResizeEdge::Top,
            (_, true, ..) => ResizeEdge::Bottom,
            (_, _, true, _) => ResizeEdge::Left,
            (_, _, _, true) => ResizeEdge::Right,
            _ => return None,
        })
    }

    /// Şeridin yeri: verilen pencerenin içerik alanında ortalı, altta.
    fn snackbar_layout(&mut self, painter: &mut Painter, content: Rect, height: f32) {
        let t = &self.applied;
        let ts = self.snackbar.text.measure_in(painter);
        let bs = if self.snackbar.undoable { self.snackbar.undo.preferred_size(painter.text_system(), t) } else { Size::ZERO };
        let pad = t.space(4);
        // Düğmesiz şeritte metin ortada: iki yanda aynı iç boşluk (space(5)).
        let w = if self.snackbar.undoable { ts.width + t.space(6) + bs.width + pad } else { ts.width + t.space(10) };
        let h = t.space(12);
        let x = content.x + (content.width - w) * 0.5;
        let y = height - t.space(5) - h;
        self.snackbar.rect = Rect::new(x, y, w, h);
        self.snackbar.undo.set_rect(Rect::new(x + w - bs.width - t.space(1), y + (h - bs.height) * 0.5, bs.width, bs.height));
    }

    fn handle_pointer(&mut self, cx: &mut Context<Self>, surface: SurfaceId, e: PointerEvent) {
        let now = Instant::now();
        let theme = self.applied.clone();
        let t = &theme;
        let mut repaint = false;

        // Açık menü her şeyin önünde.
        if self.recent_menu.is_open() {
            let r = self.recent_menu.pointer(cx.text(), t, &e, now);
            if let Some(MenuEvent::Activated(id)) = r.event
                && id >= MENU_RECENT_BASE
            {
                self.revert_recent(cx, (id - MENU_RECENT_BASE) as usize, now);
            }
            cx.request_frame(surface);
            return;
        }

        // Sayfada açık kaplama (açılır seçimin menüsü): olay yalnızca sayfaya; menü dışına
        // tıklama menüyü kapatır, kenar çubuğuna/başlığa gitmez.
        let i = Self::page_index(self.current);
        if let Some(p) = self.pages[i].as_mut()
            && (p.overlay_open() || self.overlay.contains(e.position))
        {
            let mut actions = std::mem::take(&mut self.pending_actions);
            let mut env = Env { theme: t, store: &self.store, text: cx.text(), now, actions: &mut actions };
            let _ = p.pointer(&mut env, &e);
            self.pending_actions = actions;
            self.apply_actions(cx, now);
            cx.request_frame(surface);
            return;
        }

        // Pencere kenarı → boyutlandırma.
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && let Some(edge) = self.edge_at(e.position)
        {
            cx.begin_resize(surface, edge);
            return;
        }

        // Başlık çubuğu düğmeleri.
        let mut clicked = None;
        for (i, b) in [&mut self.undo_btn, &mut self.history_btn, &mut self.min_btn, &mut self.max_btn, &mut self.close_btn].into_iter().enumerate() {
            let r = b.pointer(&e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                clicked = Some(i);
            }
        }
        if let Some(i) = clicked {
            {
                match i {
                    0 => self.undo(cx, false, now),
                    1 => self.open_recent_menu(cx, now),
                    2 => cx.set_minimized(surface),
                    3 => cx.set_maximized(surface, !self.state.maximized),
                    _ => {
                        self.close_window(cx, surface);
                        return;
                    }
                }
                cx.request_frame(surface);
                return;
            }
        }

        // Geri al şeridi.
        if self.snackbar.undoable && self.snackbar.shown.target() > 0.5 {
            let r = self.snackbar.undo.pointer(&e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.undo(cx, false, now);
                self.snackbar.shown.set_target(0.0, now);
                cx.request_frame(surface);
                return;
            }
        }

        // Kenar çubuğu. İkon şeridinde arama alanı yerine düğme: basınca alan odaklanır,
        // kenar çubuğu açılır (`sync_rail`).
        if self.rail_progress(now) >= 0.5 {
            let r = self.search_btn.pointer(&e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                self.set_focus(cx, Focus::Search, now);
                cx.request_frame(surface);
                return;
            }
        } else {
            let r = self.search.pointer(cx.text(), t, &e, now);
            repaint |= r.repaint;
        }
        if let PointerKind::Press(MouseButton::Left) = e.kind {
            if self.rail_progress(now) < 0.5 && self.search.rect().contains(e.position) {
                self.set_focus(cx, Focus::Search, now);
            } else if self.focus == Focus::Search {
                self.set_focus(cx, Focus::None, now);
            }
        }
        if self.searching {
            let r = self.results.pointer(t, &e, now);
            repaint |= r.repaint;
            if let Some((i, ListEvent::Clicked)) = r.event {
                self.open_result(cx, i, now);
            }
        } else {
            let r = self.nav.pointer(t, &e, now);
            repaint |= r.repaint;
            if let Some((i, ListEvent::Clicked)) = r.event
                && let Some(&p) = self.nav_pages.get(i)
            {
                self.navigate(cx, p, now);
            }
        }

        // Taşıma: başlık çubuğunun ve kenar çubuğu üstünün boş alanı.
        if let PointerKind::Press(MouseButton::Left) = e.kind {
            let hh = header_height(t);
            if e.position.y < hh {
                cx.begin_move(surface);
                return;
            }
        }
        if let PointerKind::Press(MouseButton::Right) = e.kind
            && e.position.y < header_height(t)
        {
            cx.set_maximized(surface, !self.state.maximized);
            return;
        }

        // İçerik.
        let mut actions = std::mem::take(&mut self.pending_actions);
        self.ensure_page(self.current);
        let i = Self::page_index(self.current);
        let content = self.content_rect();
        let store = &self.store;
        if let Some(p) = self.pages[i].as_mut() {
            let mut env = Env { theme: t, store, text: cx.text(), now, actions: &mut actions };
            let in_content = content.contains(e.position);
            let ev = if in_content || !matches!(e.kind, PointerKind::Press(_)) { e } else { PointerEvent::new(e.position, PointerKind::Leave) };
            repaint |= p.pointer(&mut env, &ev);
            if let PointerKind::Press(MouseButton::Left) = e.kind
                && in_content
                && self.focus == Focus::Search
            {
                // Arama alanı dışına tıklandı: odak sayfaya.
            }
        }
        self.pending_actions = actions;
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && content.contains(e.position)
        {
            let has = self.page_mut().has_focus();
            self.set_focus(cx, if has { Focus::Page } else { Focus::None }, now);
        }
        self.apply_actions(cx, now);
        if repaint {
            cx.request_frame(surface);
        }
    }

    // ---- Hoş geldin penceresi --------------------------------------------------------------

    /// `nuxut-settings welcome`: süreç yalnızca Hoş geldin penceresiyle başlar.
    pub fn with_welcome(mut self) -> Self {
        self.welcome_only_start = true;
        self.welcome = Some(crate::welcome_ui::WelcomeView::new(&self.applied, &self.store));
        self
    }

    /// Ayarlar penceresi yoksa açar (Hoş geldin'le başlamış süreçte `open`/`flow`).
    fn ensure_settings_window(&mut self, cx: &mut Context<Self>) {
        self.welcome_only_start = false;
        if self.window.is_none() && self.started {
            self.size = Size::ZERO;
            self.state = WindowState::default();
            self.window = Some(cx.create_window(settings_window_spec(self.current.title())));
        }
    }

    /// Hoş geldin akışını ve penceresini açar; açıksa bir şey yapmaz. Ayarlar penceresi
    /// (varsa) olduğu gibi kalır.
    fn open_welcome(&mut self, cx: &mut Context<Self>) {
        if self.welcome_window.is_some() {
            return;
        }
        if self.welcome.is_none() {
            self.welcome = Some(crate::welcome_ui::WelcomeView::new(&self.applied, &self.store));
        }
        self.welcome_done = false;
        self.welcome_reported = false;
        self.welcome_size = Size::ZERO;
        self.welcome_window = Some(cx.create_window(welcome_window_spec()));
    }

    fn welcome_content_rect(&self) -> Rect {
        let y = header_height(self.theme());
        Rect::new(0.0, y, self.welcome_size.width, (self.welcome_size.height - y).max(0.0))
    }

    fn layout_welcome(&mut self, cx: &mut Context<Self>) {
        let theme = self.applied.clone();
        let t = &theme;
        let content = self.welcome_content_rect();
        if let Some(w) = &mut self.welcome {
            w.set_rect(t, cx.text(), content);
        }
        let bs = self.welcome_close.preferred_size(t);
        let hh = header_height(t);
        self.welcome_close.set_rect(Rect::new(self.welcome_size.width - t.space(2) - bs.width, (hh - bs.height) * 0.5, bs.width, bs.height));
    }

    /// Hoş geldin sonucunu shell'e bildirir (bitti: `setup done`, kapatıldı: `setup skip`).
    /// `setup.toml`'u yalnızca shell yazar. Pencere hemen kapandığından istek eşzamanlıdır
    /// (yerel soket; en çok IPC zaman aşımı kadar).
    fn record_welcome(&mut self, finished: bool) {
        if self.welcome.is_none() || self.welcome_reported {
            return;
        }
        self.welcome_reported = true;
        let link = crate::services::shell_ipc::IpcLink::from_env();
        match crate::welcome::report_setup(&link, finished) {
            // Hyprland ana ayarı: yalnız Bitir'le, seçiliyse ve gerçek shell sonucu aldıysa (test
            // örneğinde ve shell yokken çalışmaz). Sonuç shell bildiriminde, "Geri al" düğmeli.
            Ok(()) if finished && self.welcome.as_ref().is_some_and(|w| w.takeover_chosen()) => crate::pages::hyprland_config::spawn_takeover(),
            Ok(()) => {}
            Err(e @ (crate::services::shell_ipc::LinkError::NotRunning | crate::services::shell_ipc::LinkError::TestInstance)) => tracing::info!(%e, "Hoş geldin sonucu shell'e bildirilmedi"),
            Err(e) => tracing::warn!(%e, "Hoş geldin sonucu shell'e bildirilemedi"),
        }
    }

    /// Bir pencereyi kapatır; son pencereyse süreç çıkar. Önce bekleyen yazımlar diske iner,
    /// sonra (Hoş geldin penceresinde) sonuç bildirilir — shell `setup done`'u aldığında
    /// seçimler diskte olsun. Ayarlar penceresinin sayfa durumu süreçte kalır.
    fn close_window(&mut self, cx: &mut Context<Self>, surface: SurfaceId) {
        self.flush(cx);
        if self.welcome_window == Some(surface) {
            self.record_welcome(self.welcome_done);
            self.welcome_window = None;
            self.welcome = None;
        } else if self.window == Some(surface) {
            self.window = None;
            self.recent_menu.close(Instant::now());
            crate::services::displays::shell::watch(false);
        }
        cx.destroy_surface(surface);
        if cx.window_count() == 0 {
            cx.quit();
        }
    }

    fn welcome_pointer(&mut self, cx: &mut Context<Self>, surface: SurfaceId, e: PointerEvent, now: Instant) {
        let theme = self.applied.clone();
        // Başlık çubuğunda yalnızca kapat: kapatmak akışı atlar (`skipped`).
        let r = self.welcome_close.pointer(&e, now);
        let mut repaint = r.repaint;
        if r.event.is_some() {
            self.close_window(cx, surface);
            return;
        }
        self.layout_welcome(cx);
        let mut actions = std::mem::take(&mut self.pending_actions);
        let store = &self.store;
        let mut outcome = crate::welcome_ui::Outcome::None;
        if let Some(w) = &mut self.welcome {
            let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut actions };
            let (r, o) = w.pointer(&mut env, &e);
            repaint |= r;
            outcome = o;
        }
        self.pending_actions = actions;
        if outcome == crate::welcome_ui::Outcome::Finished {
            self.welcome_done = true;
            self.apply_actions(cx, now);
            self.close_window(cx, surface);
            return;
        }
        if let PointerKind::Press(MouseButton::Left) = e.kind
            && e.position.y < header_height(&theme)
            && !self.welcome_close.rect().contains(e.position)
        {
            cx.begin_move(surface);
            return;
        }
        self.apply_actions(cx, now);
        if repaint {
            cx.request_frame(surface);
        }
    }

    fn welcome_key(&mut self, cx: &mut Context<Self>, surface: SurfaceId, e: &KeyEvent, now: Instant) {
        if e.state != KeyState::Released && e.modifiers.ctrl {
            match e.key {
                Key::Char('z' | 'Z') if e.modifiers.shift => return self.undo(cx, true, now),
                Key::Char('z') => return self.undo(cx, false, now),
                Key::Char('y') => return self.undo(cx, true, now),
                Key::Char('w' | 'q') => return self.close_window(cx, surface),
                _ => {}
            }
        }
        let theme = self.applied.clone();
        self.layout_welcome(cx);
        let mut actions = std::mem::take(&mut self.pending_actions);
        let store = &self.store;
        let mut outcome = crate::welcome_ui::Outcome::None;
        if let Some(w) = &mut self.welcome {
            let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut actions };
            outcome = w.key(&mut env, e).1;
        }
        self.pending_actions = actions;
        self.apply_actions(cx, now);
        if outcome == crate::welcome_ui::Outcome::Finished {
            self.welcome_done = true;
            self.close_window(cx, surface);
            return;
        }
        self.update_welcome_ime(cx);
        cx.request_frame(surface);
    }

    fn update_welcome_ime(&mut self, cx: &mut Context<Self>) {
        let Some(surface) = self.welcome_window else { return };
        let state = self.welcome.as_mut().and_then(|w| w.focused_text_input()).map(|i| i.ime_state());
        cx.set_text_input(surface, state.map(text_input_state));
    }

    fn welcome_cursor(&self, p: Point) -> CursorShape {
        CursorShape::pick([self.welcome_close.cursor(p), self.welcome.as_ref().and_then(|w| w.cursor(p))])
    }

    fn draw_welcome(&mut self, cx: &mut Context<Self>, surface: SurfaceId, now: Instant, painter: &mut Painter) {
        let mut theme = self.applied.clone();
        theme.colors = self.live.current(now).colors;
        let theme_animating = self.live.is_animating(now);
        let t = &theme;
        let size = self.welcome_size;
        painter.rect(Rect::new(0.0, 0.0, size.width, size.height), &RectStyle::filled(t.colors.surface, Corners::ZERO));
        let content = self.welcome_content_rect();
        if let Some(w) = &mut self.welcome {
            w.set_rect(&self.applied, painter.text_system(), content);
        }
        // Ayarlar penceresi yokken geri al şeridi burada (yazım hatası, geri yükleme bildirimi).
        let snackbar = self.window.is_none();
        if snackbar {
            self.snackbar_layout(painter, content, size.height);
        }
        let mut ui = Ui::new(painter, t, now);
        if let Some(w) = &mut self.welcome {
            w.draw(&mut ui, &self.store);
        }
        self.welcome_close.draw(&mut ui);
        self.draw_overlays(cx, surface, &mut ui, theme_animating, now, snackbar);
    }

    /// İmleç şekli: menü > pencere kenarı > başlık düğmeleri > kenar çubuğu > sayfa.
    fn cursor_at(&self, p: Point) -> CursorShape {
        if self.recent_menu.is_open() {
            return CursorShape::pick([self.recent_menu.cursor(p)]);
        }
        if let Some(edge) = self.edge_at(p) {
            return match edge {
                ResizeEdge::Top => CursorShape::ResizeN,
                ResizeEdge::Bottom => CursorShape::ResizeS,
                ResizeEdge::Left => CursorShape::ResizeW,
                ResizeEdge::Right => CursorShape::ResizeE,
                ResizeEdge::TopLeft => CursorShape::ResizeNw,
                ResizeEdge::TopRight => CursorShape::ResizeNe,
                ResizeEdge::BottomLeft => CursorShape::ResizeSw,
                ResizeEdge::BottomRight => CursorShape::ResizeSe,
            };
        }
        let page = self.pages[Self::page_index(self.current)].as_ref().and_then(|pg| pg.cursor(p));
        if self.overlay.contains(p) || self.pages[Self::page_index(self.current)].as_ref().is_some_and(Page::overlay_open) {
            return CursorShape::pick([page]);
        }
        let buttons = [&self.undo_btn, &self.history_btn, &self.min_btn, &self.max_btn, &self.close_btn];
        let snack = (self.snackbar.undoable && self.snackbar.shown.target() > 0.5).then(|| self.snackbar.undo.cursor(p)).flatten();
        let list = if self.searching { self.results.cursor(p) } else { self.nav.cursor(p) };
        let search = if self.rail_progress(Instant::now()) >= 0.5 { self.search_btn.cursor(p) } else { self.search.cursor(p) };
        CursorShape::pick(buttons.iter().map(|b| b.cursor(p)).chain([snack, search, list, page]))
    }

    fn handle_page_key(&mut self, cx: &mut Context<Self>, e: &KeyEvent, now: Instant) -> bool {
        let theme = self.applied.clone();
        let mut actions = std::mem::take(&mut self.pending_actions);
        let i = Self::page_index(self.current);
        self.ensure_page(self.current);
        let store = &self.store;
        let used = match self.pages[i].as_mut() {
            Some(p) => {
                let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut actions };
                if e.key == Key::Tab && e.state != KeyState::Released && !p.captures_keys() {
                    p.move_focus(&mut env, !e.modifiers.shift)
                } else {
                    p.key(&mut env, e)
                }
            }
            None => false,
        };
        self.pending_actions = actions;
        if e.key == Key::Tab && used {
            self.focus = Focus::Page;
        }
        self.apply_actions(cx, now);
        self.update_ime(cx);
        used
    }
}

impl App for Settings {
    type Message = Msg;

    fn output_added(&mut self, cx: &mut Context<Self>, output: &OutputInfo) {
        self.track_output(cx, output);
        if self.started {
            return;
        }
        self.started = true;
        // `nuxut-settings welcome`: yalnızca Hoş geldin penceresi; Ayarlar istenince açılır.
        if !self.welcome_only_start {
            self.ensure_settings_window(cx);
        }
        if self.welcome.is_some() {
            self.open_welcome(cx);
        }
        cx.watch_config(FileId::Shell.name());
        crate::services::init(cx.sender());
        // Shell şeması (katalog + varsayılanlar) hemen yüklenmeye başlar; hangi sayfa açık olursa olsun.
        let _ = crate::services::widgets::catalog();
        if let Some(flow) = self.start_flow.take() {
            crate::services::dispatch(crate::services::Request::Calendar(crate::services::calendar::Command::Open(flow)));
        }
        self.ensure_page(self.current);
        // Doğrudan Wi-Fi sayfasıyla açıldı (`nuxut-settings wifi`): gezinme olmadığı için burada.
        crate::services::network::set_wifi_visible(self.current == PageId::Wifi);
        crate::services::bluetooth::set_page_visible(self.current == PageId::Bluetooth);
        crate::services::printers::set_page_visible(self.current == PageId::Printers);
        crate::services::assist::set_page_visible(self.current == PageId::Accessibility);
    }

    fn keyboard_focus(&mut self, _cx: &mut Context<Self>, _surface: SurfaceId, focused: bool) {
        // Kullanıcı başka pencerede (ör. terminalde tema kurup) döndü: görünüm durumu yeniden
        // sorulur; `themes_rev` değiştiyse kurulu tema listesi yeniden okunur.
        if focused {
            crate::services::look::recheck();
            // Girdi ayarları: kullanıcı Hyprland dosyasını elle düzenleyip dönmüş olabilir (yalnızca
            // Klavye ya da Fare sayfası daha önce açıldıysa sorar).
            crate::services::input::recheck();
            // Tarih ve saat: eşitlenme durumu ve saat sinyal vermez (yalnızca sayfa açıldıysa sorar).
            crate::services::timedate::recheck();
            // Kullanıcı hesabı (yalnızca sayfa açıldıysa sorar).
            crate::services::accounts::recheck();
            // Wi-Fi: sayfa görünüyorsa yeniden tara (kullanıcı başka yerde ağ değiştirmiş olabilir).
            if self.current == PageId::Wifi {
                crate::services::network::rescan();
            }
            // Bluetooth: sayfa görünüyorsa ve tarama durmuşsa yeniden ara (≥ 10 sn arayla).
            if self.current == PageId::Bluetooth {
                crate::services::bluetooth::rescan(false);
            }
            // Yazıcılar: sayfa görünüyorsa hemen yeniden oku (yazıcı eklenmiş/iş bitmiş olabilir).
            crate::services::printers::recheck();
            // Varsayılan uygulamalar: başka bir araçla değişmiş ya da uygulama kurulmuş olabilir.
            crate::services::default_apps::recheck();
            // Gece ışığı konumu: kullanıcı shell.toml'a elle konum yazıp dönmüş olabilir.
            crate::services::night_light::recheck();
            // Büyüteç / ekran klavyesi: kısayolla ya da kabuktan değişmiş olabilir.
            crate::services::assist::recheck();
        }
    }

    fn output_changed(&mut self, cx: &mut Context<Self>, output: &OutputInfo) {
        self.track_output(cx, output);
    }

    fn output_removed(&mut self, cx: &mut Context<Self>, output: OutputId) {
        self.outputs.retain(|(id, _)| *id != output);
        self.publish_outputs(cx);
    }

    fn surface_resized(&mut self, cx: &mut Context<Self>, surface: SurfaceId, size: Size, _scale: f32) {
        if self.welcome_window == Some(surface) {
            self.welcome_size = size;
            self.layout_welcome(cx);
        } else {
            self.size = size;
            self.layout(cx, Instant::now());
        }
        if !self.debug_done && self.debug_timer.is_none() {
            self.run_debug_steps(cx);
        }
        cx.request_frame(surface);
    }

    fn window_state_changed(&mut self, cx: &mut Context<Self>, surface: SurfaceId, state: WindowState) {
        if self.window != Some(surface) {
            return;
        }
        self.state = state;
        self.max_btn.set_icon(Icon::named(if state.maximized { IconName::Restore } else { IconName::Maximize }));
        cx.request_frame(surface);
    }

    fn close_requested(&mut self, cx: &mut Context<Self>, surface: SurfaceId) {
        self.close_window(cx, surface);
    }

    fn terminate(&mut self, cx: &mut Context<Self>) {
        self.flush(cx);
        cx.quit();
    }

    fn pointer(&mut self, cx: &mut Context<Self>, surface: SurfaceId, e: PointerEvent) {
        let welcome = self.welcome_window == Some(surface);
        if welcome {
            self.welcome_pointer(cx, surface, e, Instant::now());
        } else if self.window == Some(surface) {
            self.handle_pointer(cx, surface, e);
        } else {
            return;
        }
        // Olay pencereyi kapatmış olabilir.
        if !matches!(e.kind, PointerKind::Leave) && (self.window == Some(surface) || self.welcome_window == Some(surface)) {
            let shape = if welcome { self.welcome_cursor(e.position) } else { self.cursor_at(e.position) };
            cx.set_cursor(surface, shape);
        }
    }

    fn keyboard(&mut self, cx: &mut Context<Self>, surface: SurfaceId, e: KeyEvent) {
        let now = Instant::now();
        if self.welcome_window == Some(surface) {
            return self.welcome_key(cx, surface, &e, now);
        }
        if self.window != Some(surface) {
            return;
        }
        let theme = self.applied.clone();
        if self.recent_menu.is_open() {
            let r = self.recent_menu.key(cx.text(), &theme, &e, now);
            if let Some(MenuEvent::Activated(id)) = r.event
                && id >= MENU_RECENT_BASE
            {
                self.revert_recent(cx, (id - MENU_RECENT_BASE) as usize, now);
            }
            cx.request_frame(surface);
            return;
        }
        let down = e.state != KeyState::Released;
        let m = e.modifiers;
        // Kısayol yakalanıyor: uygulama kısayolları (Ctrl+Z, Ctrl+W, Tab…) dahil her tuş satıra.
        if self.focus != Focus::Search && self.pages[Self::page_index(self.current)].as_ref().is_some_and(|p| p.captures_keys()) {
            self.handle_page_key(cx, &e, now);
            cx.request_frame(surface);
            return;
        }
        if down && m.ctrl {
            match e.key {
                Key::Char('z' | 'Z') if m.shift => return self.undo(cx, true, now),
                Key::Char('z') => return self.undo(cx, false, now),
                Key::Char('y') => return self.undo(cx, true, now),
                Key::Char('f') => {
                    self.set_focus(cx, Focus::Search, now);
                    cx.request_frame(surface);
                    return;
                }
                Key::Char('w' | 'q') => {
                    self.close_window(cx, surface);
                    return;
                }
                Key::PageDown | Key::PageUp => {
                    // Kenar çubuğundaki sırayla (gizli sayfalar atlanır).
                    let n = self.nav_pages.len().max(1) as i32;
                    let cur = self.nav_pages.iter().position(|p| *p == self.current).unwrap_or(0) as i32;
                    let i = cur + if e.key == Key::PageDown { 1 } else { -1 };
                    if let Some(&p) = self.nav_pages.get(i.rem_euclid(n) as usize) {
                        self.navigate(cx, p, now);
                    }
                    return;
                }
                _ => {}
            }
        }
        match self.focus {
            Focus::Search => {
                if down && e.key == Key::Escape {
                    if self.search.text().is_empty() {
                        self.set_focus(cx, Focus::None, now);
                    } else {
                        self.search.set_text("");
                        self.update_search(now);
                    }
                    cx.request_frame(surface);
                    return;
                }
                if down && matches!(e.key, Key::Down | Key::Up) {
                    let list = if self.searching { &mut self.results } else { &mut self.nav };
                    let _ = list.key(&theme, &e, now);
                    cx.request_frame(surface);
                    return;
                }
                if down && e.key == Key::Enter && self.searching {
                    // İlk sonuca git (ya da klavyeyle seçilene).
                    let r = self.results.key(&theme, &e, now);
                    match r.event {
                        Some((i, ListEvent::Clicked)) => self.open_result(cx, i, now),
                        _ => self.open_result(cx, 0, now),
                    }
                    self.set_focus(cx, Focus::None, now);
                    cx.request_frame(surface);
                    return;
                }
                if down && e.key == Key::Tab {
                    self.set_focus(cx, Focus::None, now);
                    self.handle_page_key(cx, &e, now);
                    cx.request_frame(surface);
                    return;
                }
                let r = self.search.key(&e, now);
                if matches!(r.event, Some(TextInputEvent::Changed)) {
                    self.update_search(now);
                }
                self.update_ime(cx);
                cx.request_frame(surface);
            }
            Focus::Page | Focus::None => {
                let used = self.handle_page_key(cx, &e, now);
                if !used && down && e.key == Key::Escape {
                    self.set_focus(cx, Focus::None, now);
                    self.page_mut().clear_focus(now);
                } else if !used && down && self.focus == Focus::None && e.text.is_some() && !m.ctrl && !m.alt && e.key != Key::Space {
                    // Yazmaya başlamak aramayı açar.
                    self.set_focus(cx, Focus::Search, now);
                    let _ = self.search.key(&e, now);
                    self.update_search(now);
                }
                cx.request_frame(surface);
            }
        }
    }

    fn ime(&mut self, cx: &mut Context<Self>, surface: SurfaceId, event: ImeEvent) {
        if self.welcome_window == Some(surface) {
            if let Some(i) = self.welcome.as_mut().and_then(|w| w.focused_text_input()) {
                apply_ime(i, event);
            }
            self.update_welcome_ime(cx);
            cx.request_frame(surface);
            return;
        }
        if self.window != Some(surface) {
            return;
        }
        let input = match self.focus {
            Focus::Search => Some(&mut self.search),
            Focus::Page => None,
            Focus::None => None,
        };
        let changed = match input {
            Some(i) => apply_ime(i, event),
            None => {
                let Some(i) = self.page_mut().focused_text_input() else { return };
                apply_ime(i, event);
                false
            }
        };
        if changed && self.focus == Focus::Search {
            self.update_search(Instant::now());
        }
        self.update_ime(cx);
        cx.request_frame(surface);
    }

    fn timer(&mut self, cx: &mut Context<Self>, id: TimerId) {
        let now = Instant::now();
        if self.debug_timer == Some(id) {
            self.debug_timer = None;
            self.run_debug_steps(cx);
        }
        if self.welcome_wake_timer == Some(id) {
            self.welcome_wake_timer = None;
            self.request_frame(cx);
        }
        if self.wake_timer == Some(id) {
            self.wake_timer = None;
            self.request_frame(cx);
        }
        if self.seen_timer == Some(id) {
            self.seen_timer = None;
            crate::services::seen::save();
        }
        if self.flush_timer == Some(id) {
            self.flush(cx);
        }
        if self.snackbar.timer == Some(id) {
            self.snackbar.timer = None;
            self.snackbar.shown.set_target(0.0, now);
            self.request_frame(cx);
        }
    }

    fn theme_changed(&mut self, cx: &mut Context<Self>) {
        // framework.toml ya da palette.toml değişti. Kendi yazımımızsa store değişmez.
        let now = Instant::now();
        let change = self.store.on_disk_changed(FileId::Framework);
        log_change(FileId::Framework, &change);
        let palette_change = self.store.on_disk_changed(FileId::Palette);
        log_change(FileId::Palette, &palette_change);
        if self.live.reload_palette(&self.store, now) {
            self.apply_theme(cx, now);
        }
        if change != DiskChange::Unchanged || palette_change != DiskChange::Unchanged {
            self.after_store_change(cx, now, false);
        }
    }

    fn locale_changed(&mut self, cx: &mut Context<Self>) {
        // `[locale]` değişti (runtime süreç geneli katalogu zaten yeniledi). Önce store dosyayı
        // görsün (dış düzenleme), sonra dil/biçim gösterenler değer aynı olsa da yeniden kurulur.
        // Ayarlar metinleri henüz Fluent'te değil (ADR-0010 adım 2); yalnızca bu satırlar etkilenir.
        let now = Instant::now();
        let change = self.store.on_disk_changed(FileId::Framework);
        log_change(FileId::Framework, &change);
        // Sayı gösterimlerinin locale'i yeni `[locale] formats`'la; kaydırıcı/eğri metinleri
        // değer aynı olsa da yeniden yazılsın diye sayfalar her durumda eşitlenir.
        crate::numfmt::configure(&self.store);
        if change != DiskChange::Unchanged {
            self.after_store_change(cx, now, false);
        } else {
            self.sync_pages(cx, now);
            self.request_frame(cx);
        }
        // Shell şeması (widget/ayar başlıkları, seçenek adları) dile bağlı: arka planda yeniden
        // okunur, bitince `Event::Widgets` şemaya bağlı satırları yeni dilde kurar.
        crate::services::widgets::reload();
        self.message(cx, Msg::Service(crate::services::Event::LocaleChanged));
    }

    fn config_changed(&mut self, cx: &mut Context<Self>, name: &str) {
        let Some(file) = FileId::from_name(name) else { return };
        let now = Instant::now();
        let change = self.store.on_disk_changed(file);
        log_change(file, &change);
        if change != DiskChange::Unchanged {
            self.after_store_change(cx, now, false);
        }
    }

    fn message(&mut self, cx: &mut Context<Self>, message: Msg) {
        let now = Instant::now();
        match message {
            Msg::Service(crate::services::Event::Wake) => self.request_frame(cx),
            Msg::Service(crate::services::Event::SnapshotRestored { text } | crate::services::Event::DefaultAppSet { text }) => self.show_notice(cx, &text, now),
            Msg::Service(event) => {
                let theme = self.applied.clone();
                let mut actions = std::mem::take(&mut self.pending_actions);
                let store = &self.store;
                let mut any = false;
                for p in self.pages.iter_mut().flatten() {
                    let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut actions };
                    any |= p.service_event(&mut env, &event);
                }
                if let Some(w) = &mut self.welcome {
                    let mut env = Env { theme: &theme, store, text: cx.text(), now, actions: &mut actions };
                    any |= w.service_event(&mut env, &event);
                }
                self.pending_actions = actions;
                // Şema geldi: shell varsayılanları, seçenekleri ve sınırları değişmiş olabilir; satırlar
                // yeni değere/denetime akar, yerel yedekle farklar bir kez loglanır.
                if matches!(event, crate::services::Event::Widgets) {
                    crate::registry::audit_schema();
                    // İlk açılış: şemadaki tüm "since"li öğeler görüldü yazılır (rozetsiz başlangıç).
                    crate::services::seen::seed_from_schema();
                    self.sync_pages(cx, now);
                    any = true;
                }
                // Girdi durumu geldi: dosyada yazılmamış satırlar etkin değere akar, uyarılar yenilenir.
                if matches!(event, crate::services::Event::Input | crate::services::Event::NightLight) {
                    self.sync_pages(cx, now);
                    any = true;
                }
                self.apply_actions(cx, now);
                if any {
                    self.request_frame(cx);
                }
            }
        }
    }

    fn command(&mut self, cx: &mut Context<Self>, args: Vec<String>, reply: ipc::Reply) {
        let now = Instant::now();
        match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
            // Hoş geldin'le başlamış süreçte Ayarlar penceresi burada açılır.
            ["open"] | [] => {
                self.ensure_settings_window(cx);
                reply.send("ok");
            }
            ["open", page] => match PageId::from_slug(page) {
                Some(p) => {
                    self.navigate(cx, p, now);
                    self.ensure_settings_window(cx);
                    reply.send("ok");
                }
                None => reply.send(&format!("error: bilinmeyen sayfa '{page}'")),
            },
            ["flow", id] if !id.is_empty() && id.len() <= 128 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') => {
                self.navigate(cx, PageId::Accounts, now);
                self.ensure_settings_window(cx);
                crate::services::dispatch(crate::services::Request::Calendar(crate::services::calendar::Command::Open((*id).to_owned())));
                reply.send("ok");
            }
            ["ping"] => reply.send("pong"),
            // Hoş geldin ayrı pencerede açılır; Ayarlar penceresi (varsa) açık kalır.
            ["welcome"] => {
                if self.started {
                    self.open_welcome(cx);
                } else if self.welcome.is_none() {
                    self.welcome = Some(crate::welcome_ui::WelcomeView::new(&self.applied, &self.store));
                }
                reply.send("ok");
            }
            _ => reply.send("error: komutlar: open [sayfa] | flow <akış> | welcome | ping"), // i18n: tanı IPC kullanım iletisi
        }
    }

    // AT_POLICY: ağaç ve eylemler yalnız Ayarlar penceresinde; AT yalnız gezinir (`app/a11y.rs`).
    fn accessibility(&mut self, cx: &mut Context<Self>, surface: SurfaceId, tree: &mut nuxut::access::AccessTree) {
        self.accessibility_inner(cx, surface, tree);
    }

    fn accessibility_action(&mut self, cx: &mut Context<Self>, surface: SurfaceId, request: nuxut::access::AccessRequest) {
        self.accessibility_action_inner(cx, surface, request);
    }

    fn draw(&mut self, cx: &mut Context<Self>, surface: SurfaceId, now: Instant, painter: &mut Painter) {
        if !self.first_frame_logged {
            self.first_frame_logged = true;
            tracing::info!(ms = self.start.elapsed().as_millis() as u64, "ilk kare");
        }
        if self.welcome_window == Some(surface) {
            return self.draw_welcome(cx, surface, now, painter);
        }
        if self.window != Some(surface) {
            return;
        }
        self.sync_rail(now);
        let mut theme = self.applied.clone();
        theme.colors = self.live.current(now).colors;
        let theme_animating = self.live.is_animating(now);
        let t = &theme;
        let c = &t.colors;
        let size = self.size;
        let full = Rect::new(0.0, 0.0, size.width, size.height);
        let sw = self.side_width();
        let hh = header_height(t);

        // Zemin: içerik `surface`, kenar çubuğu bir ton ayrışır.
        painter.rect(full, &RectStyle::filled(c.surface, Corners::ZERO));
        let side = Rect::new(0.0, 0.0, sw, size.height);
        painter.rect(side, &RectStyle::filled(c.surface.mix(&c.surface_container, 0.55), Corners::ZERO));
        painter.rect(Rect::new(sw - HAIRLINE, 0.0, HAIRLINE, size.height), &RectStyle::filled(hairline(t, c.outline, 0.5, &[]), Corners::ZERO));

        let content = self.content_rect();
        self.snackbar_layout(painter, content, size.height);
        let mut ui = Ui::new(painter, t, now);

        // Kenar çubuğu başlığı ve arama alanı: şerit geçişinin ilk yarısında söner (listenin
        // metinleriyle aynı eğri), ikinci yarısında yerini arama düğmesi alır.
        let rail = self.rail_progress(now);
        let text_alpha = (1.0 - rail * 2.0).clamp(0.0, 1.0);
        if self.nav.is_rail_animating(now) {
            ui.request_frame();
        }
        if text_alpha > 0.0 {
            let ts = self.app_title.measure_in(ui.painter);
            ui.painter.push_clip(Rect::new(0.0, 0.0, sw, hh));
            self.app_title.draw(ui.painter, Point::new(t.space(5), (hh - ts.height) * 0.5), c.on_surface.multiply_alpha(text_alpha));
            ui.painter.pop_clip();
            let search = &mut self.search;
            let icon = &mut self.search_icon;
            ui.with_alpha(text_alpha, |ui| {
                search.draw(ui);
                if search.text().is_empty() && !search.is_focused() {
                    // Placeholder'ın sağında arama ikonu.
                    let r = search.rect();
                    let is = icon.measure(ui.painter.text_system());
                    icon.draw_centered(ui.painter, Point::new(r.right() - t.space(3) - is.width * 0.5, r.center().y), ui.fade(c.on_surface_variant));
                }
            });
        }
        if rail >= 0.5 {
            let a = ((rail - 0.5) * 2.0).clamp(0.0, 1.0);
            let btn = &mut self.search_btn;
            ui.with_alpha(a, |ui| btn.draw(ui));
        }
        if self.searching && self.no_results {
            // Listenin üstünde, kenar çubuğu genişliğinde; sayfada maskot varken maskotsuz.
            let nav = self.nav_rect();
            let page_mascot = self.pages[Self::page_index(self.current)].as_ref().and_then(|p| p.mascot(&self.store));
            let e = &mut self.search_empty[usize::from(search_spot(page_mascot).is_none())];
            let h = e.measure(ui.painter.text_system(), t, nav.width).height.min(nav.height);
            e.set_rect(Rect::new(nav.x, nav.y, nav.width, h));
            e.draw(&mut ui);
        } else if self.searching {
            self.results.draw(&mut ui)
        } else {
            self.nav.draw(&mut ui)
        }

        // İçerik başlığı.
        let ps = self.page_title.measure_in(ui.painter);
        self.page_title.draw(ui.painter, Point::new(sw + t.space(6), (hh - ps.height) * 0.5), c.on_surface);
        self.undo_btn.set_enabled(self.history.can_undo(), now);
        for b in [&mut self.undo_btn, &mut self.history_btn, &mut self.min_btn, &mut self.max_btn, &mut self.close_btn] {
            b.draw(&mut ui);
        }

        // Bozuk dosya şeridi.
        let mut content = self.content_rect();
        if let Some(b) = &mut self.banner {
            let pad = t.space(3);
            b.set_max_width(Some(content.width - pad * 4.0));
            let bs = b.measure_in(ui.painter);
            let r = Rect::new(content.x + pad, content.y, content.width - pad * 2.0, bs.height + pad * 2.0);
            ui.painter.rect(r, &RectStyle::filled(state_layer_under(t, c.primary, 0.16, &[], &[c.on_surface]), t.shape.medium()));
            b.draw(ui.painter, Point::new(r.x + pad, r.y + pad), c.on_surface);
            content.y += r.height + pad;
            content.height -= r.height + pad;
        }

        // Sayfa (geçiş: yukarı kayarak belirir).
        let tr = self.transition.value(now);
        if !self.transition.is_settled(now) {
            ui.request_frame();
        }
        let i = Self::page_index(self.current);
        let store = &self.store;
        // "Yeni" rozetleri: pencere etkinken sayfanın görünür alanında çizilen rozetli öğeler
        // izlenir; 1 sn kesintisiz görünen görüldü yazılır (bkz. `services::seen`).
        crate::services::seen::begin_frame(content, self.state.activated);
        if let Some(p) = self.pages[i].as_mut() {
            let offset = (1.0 - tr) * t.space(4);
            p.set_viewport(Rect::new(content.x, content.y + offset, content.width, content.height - offset));
            ui.painter.push_clip(content);
            ui.with_alpha(tr.clamp(0.0, 1.0), |ui| p.draw(ui, store));
            ui.painter.pop_clip();
        }
        if let Some(at) = crate::services::seen::end_frame(now) {
            ui.wake_at(at);
        }
        if crate::services::seen::is_dirty() && self.seen_timer.is_none() {
            self.seen_timer = Some(cx.set_timer(crate::services::seen::FLUSH_DELAY));
        }

        self.draw_overlays(cx, surface, &mut ui, theme_animating, now, true);
    }
}

impl Settings {
    /// Karenin son katmanları (iki pencerede): geri al şeridi (`snackbar` ise), menü ve kaplama
    /// bölgeleri (yalnız Ayarlar), kare/uyanma isteği (pencere başına zamanlayıcı).
    fn draw_overlays(&mut self, cx: &mut Context<Self>, surface: SurfaceId, ui: &mut Ui, theme_animating: bool, now: Instant, snackbar: bool) {
        let t = ui.theme;
        let c = &t.colors;
        let settings = self.window == Some(surface);
        // Geri al şeridi.
        let shown = if snackbar { self.snackbar.shown.value(now) } else { 0.0 };
        if snackbar && !self.snackbar.shown.is_settled(now) {
            ui.request_frame();
        }
        if shown > 0.01 {
            let base = self.snackbar.rect;
            let r = base.translate(Point::new(0.0, (1.0 - shown) * t.space(6)));
            ui.with_alpha(shown.clamp(0.0, 1.0), |ui| {
                if let Some(s) = t.shadow.enabled.then(|| nuxut::ShadowStyle { color: t.shadow.color, blur: t.shadow.blur, offset: Point::new(0.0, t.shadow.offset_y), spread: 0.0 }) {
                    ui.painter.shadow(r, Corners::uniform(r.height * 0.5), &nuxut::ShadowStyle { color: ui.fade(s.color), ..s });
                }
                ui.painter.rect(r, &RectStyle::filled(ui.fade(c.surface_bright), Corners::uniform(r.height * 0.5)));
                let ts = self.snackbar.text.measure_in(ui.painter);
                let color = ui.fade(c.on_surface);
                self.snackbar.text.draw(ui.painter, Point::new(r.x + t.space(5), r.center().y - ts.height * 0.5), color);
                if self.snackbar.undoable {
                    let ub = self.snackbar.undo.rect();
                    self.snackbar.undo.set_rect(Rect::new(ub.x, r.center().y - ub.height * 0.5, ub.width, ub.height));
                    self.snackbar.undo.draw(ui);
                    self.snackbar.undo.set_rect(ub);
                }
            });
        }

        // Kaplamaların girdi bölgeleri (açık açılır seçim): sonraki işaretçi olayları önce sayfaya.
        let overlay = ui.take_overlay_regions();
        if settings {
            self.overlay = overlay;
        }

        // Menü en üstte.
        if settings && self.recent_menu.is_visible() {
            self.recent_menu.draw(ui);
        }

        if ui.wants_frame() || theme_animating {
            cx.request_frame(surface);
        }
        // Zamanlı uyanma: öncekini iptal edip en erken ana tek zamanlayıcı.
        let wake = ui.wake_deadline();
        let slot = if settings { &mut self.wake_timer } else { &mut self.welcome_wake_timer };
        if let Some(t) = slot.take() {
            cx.cancel_timer(t);
        }
        if let Some(at) = wake {
            *slot = Some(cx.set_timer(at.saturating_duration_since(now)));
        }
    }
}

fn text_input_state(s: nuxut::ui::ImeState) -> TextInputState {
    TextInputState {
        cursor_rect: s.cursor_rect,
        surrounding: Some(SurroundingText { text: s.surrounding, cursor: s.cursor, anchor: s.anchor }),
        purpose: if s.password { TextPurpose::Password } else { TextPurpose::Normal },
        multiline: false,
    }
}

fn apply_ime(input: &mut TextInput, event: ImeEvent) -> bool {
    let r = match event {
        ImeEvent::Preedit { text, cursor } => input.ime_preedit(&text, cursor),
        ImeEvent::Commit(text) => input.ime_commit(&text),
        ImeEvent::DeleteSurrounding { before, after } => input.ime_delete_surrounding(before, after),
    };
    matches!(r.event, Some(TextInputEvent::Changed))
}

fn log_change(file: FileId, change: &DiskChange) {
    match change {
        DiskChange::Unchanged => tracing::debug!("{}.toml: kendi yazımımız", file.name()),
        DiskChange::Reloaded => tracing::info!("{}.toml dışarıdan değişti; yeniden yüklendi", file.name()),
        DiskChange::Removed => tracing::info!("{}.toml silindi; varsayılanlar", file.name()),
        DiskChange::Broken(e) => tracing::warn!("{}.toml bozuk: {e}", file.name()),
    }
}

/// Arama sonuç vermediğinde maskot yeri: sonuç yok maskotu; açık sayfa zaten maskot gösteriyorsa
/// yok (ekranda en çok bir maskot; framework `docs/design/mascot.md`).
pub fn search_spot(page_mascot: Option<Spot>) -> Option<Spot> {
    page_mascot.is_none().then_some(Spot::SearchNoResults)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_window_is_fixed_size_with_own_app_id() {
        let w = welcome_window_spec();
        assert_eq!(w.app_id, "nuxut-welcome");
        assert_eq!(w.size, WELCOME_SIZE);
        // min = max: compositor'lar sabit boyutlu pencereyi yüzer açar.
        assert_eq!(w.min_size, Some(w.size));
        assert_eq!(w.max_size, Some(w.size));
    }

    #[test]
    fn settings_window_stays_resizable() {
        crate::i18n::use_turkish_for_tests();
        let s = settings_window_spec("Görünüm");
        assert_eq!(s.app_id, "nuxut-settings");
        assert_eq!(s.title, "Ayarlar — Görünüm");
        assert_eq!(s.max_size, None);
        assert_ne!(s.app_id, welcome_window_spec().app_id);
    }

    /// Arama sonuç yok: maskot (sonuç yok); açık sayfada maskot varken yok (ekranda tek maskot).
    #[test]
    fn search_no_results_mascot_yields_to_page_mascot() {
        assert_eq!(search_spot(None), Some(Spot::SearchNoResults));
        assert_eq!(Spot::SearchNoResults.mood(), nuxut::ui::Mood::NoResults);
        for page in [Spot::About, Spot::AccountsEmpty, Spot::DisplaysUnavailable] {
            assert_eq!(search_spot(Some(page)), None, "{page:?}");
        }
    }
}
