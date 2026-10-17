//! Sistem › Yazıcılar sayfası (t125; `docs/design/printers.md`). Değerler CUPS'ta
//! ([`crate::services::printers`]); bloklar servis anlık görüntüsünden çizer, eylemi servise
//! verir. Durum ancak yeniden okumayla değişir (iyimser değil).
//!
//! - [`PrintersStatus`]: kartsız üst blok — CUPS yoksa/okunamadıysa maskotlu durum (sayfanın tek
//!   içeriği), değilse son eylem hatasının notu.
//! - [`PrinterList`]: yazıcılar (durum, varsayılan); satır içi ayrıntı: model, konum, kuyruk adı,
//!   "Varsayılan yap", "Deneme sayfası".
//! - [`PrintersAdmin`]: "Yazıcı ekle…" — CUPS web arayüzü (v1; keşif ve ekleme v2).
//! - [`PrintJobs`]: kullanıcının bitmemiş işleri, "İptal".

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{Button as MouseButton, CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{FOCUS_RING, focus_ring};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, IconView, Interaction, Label, Notice, NoticeKind, TextRole, Ui, text_style};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, Theme};

use super::datetime::RowText;
use super::fiyu::{FiyuEmpty, Spot};
use super::notice::{notice_height, place_notice};
use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{row_height_for, row_padding};
use crate::services::printers::{self, CupsError, Failure, Job, JobState, Op, OpError, Printer, PrinterState, Snapshot, Status};
use crate::store::Store;

/// Alt metin parçalarının ayırıcısı.
const SEP: &str = " · ";

// ---- Saf yardımcılar (testli) -------------------------------------------------------------------

/// Sayfanın maskotlu durumu: (başlık, gövde). Hazırken ve okunurken `None`.
pub fn unavailable_text(status: Status) -> Option<(String, String)> {
    Some(match status {
        Status::Unavailable => (t!("settings-printers-unavailable"), t!("settings-printers-unavailable-body")),
        Status::Failed => (t!("settings-printers-failed"), t!("settings-printers-failed-body")),
        Status::Reading | Status::Ready => return None,
    })
}

/// Yazıcının durum metni.
pub fn state_text(p: &Printer) -> String {
    let state = match p.state {
        PrinterState::Idle => t!("settings-printers-state-idle"),
        PrinterState::Processing => t!("settings-printers-state-processing"),
        PrinterState::Stopped => t!("settings-printers-state-stopped"),
    };
    if p.accepting { state } else { format!("{state}{SEP}{}", t!("settings-printers-not-accepting")) }
}

/// Satırın alt metni: [Varsayılan ·] durum [· CUPS iletisi (durmuşken)] [· deneme sayfası gönderildi].
pub fn printer_subtitle(p: &Printer, is_default: bool, sent: bool) -> String {
    let mut parts = Vec::new();
    if is_default {
        parts.push(t!("settings-printers-default"));
    }
    parts.push(state_text(p));
    // CUPS iletisi sunucunun dilinde ve kısa ("Paused", "Waiting for printer…"): yalnız durmuşken.
    if p.state == PrinterState::Stopped && !p.message.trim().is_empty() {
        parts.push(p.message.trim().to_owned());
    }
    if sent {
        parts.push(t!("settings-printers-test-page-sent"));
    }
    parts.join(SEP)
}

pub fn job_state_text(s: JobState) -> String {
    match s {
        JobState::Pending => t!("settings-printers-job-pending"),
        JobState::Held => t!("settings-printers-job-held"),
        JobState::Processing => t!("settings-printers-job-processing"),
        JobState::Stopped => t!("settings-printers-job-stopped"),
        JobState::Canceled | JobState::Aborted | JobState::Completed => t!("settings-printers-job-done"),
    }
}

/// İş başlığı: belge adı, yoksa "İş <n>".
pub fn job_title(j: &Job) -> String {
    if j.name.trim().is_empty() { t!("settings-printers-job-untitled", id = i64::from(j.id)) } else { j.name.trim().to_owned() }
}

/// İş alt metni: yazıcının görünen adı · durum · boyut.
pub fn job_subtitle(j: &Job, printers: &[Printer]) -> String {
    let mut parts = Vec::new();
    let printer = printers.iter().find(|p| p.name == j.printer).map_or(j.printer.as_str(), Printer::display_name);
    if !printer.is_empty() {
        parts.push(printer.to_owned());
    }
    parts.push(job_state_text(j.state));
    if let Some(kb) = j.size_kb.filter(|k| *k > 0) {
        let size = crate::numfmt::format(f64::from(kb), 0);
        parts.push(t!("settings-printers-job-size", size = size.as_str()));
    }
    parts.join(SEP)
}

/// Eylem hatası → kullanıcı metni (üst not).
pub fn failure_text(f: &Failure) -> String {
    match &f.error {
        OpError::Write => t!("settings-printers-error-write"),
        OpError::NoPrinter => t!("settings-printers-error-gone"),
        OpError::Cups(e) if e.is_denied() => t!("settings-printers-error-denied"),
        OpError::Cups(CupsError::Unreachable | CupsError::Timeout) => t!("settings-printers-error-unreachable"),
        OpError::Cups(CupsError::Document) => t!("settings-printers-error-test-page-missing"),
        OpError::Cups(_) => match f.op {
            Op::SetDefault(_) => t!("settings-printers-error-default"),
            Op::TestPage(..) => t!("settings-printers-error-test-page"),
            Op::Cancel(_) => t!("settings-printers-error-cancel"),
        },
    }
}

/// Yazıcı satırının verisi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrinterRow {
    pub name: String,
    pub title: String,
    pub subtitle: String,
    /// Durmuş yazıcı: alt metin hata renginde.
    pub error: bool,
    /// Yazdırıyor (ikon dolu, vurgu renginde).
    pub active: bool,
    pub is_default: bool,
    /// Ayrıntı satırları: (etiket, değer); boş değerler yok.
    pub details: Vec<(String, String)>,
}

/// Yazıcılar kartının içeriği.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListContent {
    /// Kart çizilmez (CUPS yok / okunamadı: sayfanın tek içeriği üst bloktaki maskot).
    Hidden,
    Reading,
    /// Kuyruk yok: maskotlu boş durum.
    Empty,
    Rows(Vec<PrinterRow>),
}

pub fn list_content(s: &Snapshot) -> ListContent {
    match s.status {
        Status::Reading => return ListContent::Reading,
        Status::Unavailable | Status::Failed => return ListContent::Hidden,
        Status::Ready => {}
    }
    if s.state.printers.is_empty() {
        return ListContent::Empty;
    }
    let rows = s
        .state
        .printers
        .iter()
        .map(|p| {
            let is_default = s.state.default.as_deref() == Some(p.name.as_str());
            let sent = s.sent.as_deref() == Some(p.name.as_str());
            let details = [
                (t!("settings-printers-detail-model"), p.model.trim()),
                (t!("settings-printers-detail-location"), p.location.trim()),
                (t!("settings-printers-detail-queue"), p.name.as_str()),
            ]
            .into_iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| (k, v.to_owned()))
            .collect();
            PrinterRow {
                name: p.name.clone(),
                title: p.display_name().to_owned(),
                subtitle: printer_subtitle(p, is_default, sent),
                error: p.state == PrinterState::Stopped,
                active: p.state == PrinterState::Processing,
                is_default,
                details,
            }
        })
        .collect();
    ListContent::Rows(rows)
}

/// İş kuyruğu kartının içeriği: `None` = kart gizli (CUPS hazır değil ya da yazıcı yok).
pub fn job_rows(s: &Snapshot) -> Option<Vec<(i32, String, String)>> {
    if s.status != Status::Ready || s.state.printers.is_empty() {
        return None;
    }
    Some(s.state.jobs.iter().map(|j| (j.id, job_title(j), job_subtitle(j, &s.state.printers))).collect())
}

fn is_printers(event: &crate::services::Event) -> bool {
    matches!(event, crate::services::Event::Printers | crate::services::Event::LocaleChanged)
}

fn icon_side(theme: &Theme) -> f32 {
    text_style(theme, TextRole::Icon).size
}

fn draw_ring(ui: &mut Ui, rect: Rect) {
    let t = ui.theme;
    let radii = Corners::uniform(t.shape.radius_medium * 0.5);
    let color = ui.fade(focus_ring(t, &[t.colors.surface_bright]));
    ui.painter.rect(rect, &RectStyle { fill: Color::TRANSPARENT, radii, border: Some(Border { width: FOCUS_RING, color }) });
}

/// İkon + başlık + alt metin satırının ortak çizimi.
#[derive(Debug)]
struct ItemUi {
    icon: IconView,
    title: Label,
    subtitle: Label,
    error: bool,
    active: bool,
    height: f32,
    rect: Rect,
}

impl ItemUi {
    fn new(theme: &Theme, icon: IconName, title: &str, subtitle: &str, error: bool, active: bool) -> Self {
        Self {
            icon: IconView::new(theme, Icon::named(icon)).filled(active),
            title: Label::new(title, text_style(theme, TextRole::Body).with_max_lines(1)),
            subtitle: Label::new(subtitle, text_style(theme, TextRole::Caption).with_max_lines(2)),
            error,
            active,
            height: 0.0,
            rect: Rect::ZERO,
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.icon.set_theme(theme, now);
        self.title.set_style(text_style(theme, TextRole::Body).with_max_lines(1));
        self.subtitle.set_style(text_style(theme, TextRole::Caption).with_max_lines(2));
    }

    fn measure(&mut self, env: &mut Env, width: f32, trailing: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let tw = (width - px * 2.0 - icon_side(t) - t.space(4) - trailing).max(t.space(20));
        self.title.set_max_width(Some(tw));
        self.subtitle.set_max_width(Some(tw));
        let mut text = self.title.measure(env.text).height;
        if !self.subtitle.text().is_empty() {
            text += t.space(1) + self.subtitle.measure(env.text).height;
        }
        self.height = row_height_for(t, text.max(icon_side(t)));
        self.height
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, _) = row_padding(t);
        let side = icon_side(t);
        let mid = self.rect.center().y;
        let icon_color = if self.active { c.primary } else { c.on_surface_variant };
        self.icon.draw_centered(ui.painter, Point::new(self.rect.x + px + side * 0.5, mid), ui.fade(icon_color));
        let tx = self.rect.x + px + side + t.space(4);
        let th = self.title.measure_in(ui.painter).height;
        let sh = if self.subtitle.text().is_empty() { 0.0 } else { t.space(1) + self.subtitle.measure_in(ui.painter).height };
        let top = mid - (th + sh) * 0.5;
        self.title.draw(ui.painter, Point::new(tx, top), ui.fade(c.on_surface));
        if sh > 0.0 {
            let color = if self.error { c.error } else { c.on_surface_variant };
            self.subtitle.draw(ui.painter, Point::new(tx, top + th + t.space(1)), ui.fade(color));
        }
    }
}

// ---- Durum (kartsız) ----------------------------------------------------------------------------

/// CUPS yoksa / okunamadıysa maskotlu durum; değilse son eylem hatasının notu.
#[derive(Debug)]
pub struct PrintersStatus {
    desc: &'static SettingDesc,
    fiyu: Option<FiyuEmpty>,
    notice: Notice,
    note: bool,
}

impl PrintersStatus {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        printers::ensure();
        let mut b = Self { desc, fiyu: None, notice: Notice::new(theme, NoticeKind::Warning, ""), note: false };
        b.update(theme);
        b
    }

    fn update(&mut self, theme: &Theme) {
        let s = printers::snapshot();
        self.fiyu = match unavailable_text(s.status) {
            Some((title, body)) => Some(match self.fiyu.take() {
                Some(mut f) => {
                    f.set_text(theme, &title, Some(&body));
                    f
                }
                None => FiyuEmpty::new(theme, Spot::PrintersUnavailable, &title, Some(&body)),
            }),
            None => None,
        };
        let note = s.failure.as_ref().map(failure_text);
        self.note = note.is_some() && self.fiyu.is_none();
        self.notice.set_text(&note.unwrap_or_default());
    }
}

impl Block for PrintersStatus {
    fn section(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.fiyu.as_ref().map(FiyuEmpty::spot)
    }

    fn visible(&self, _store: &Store) -> bool {
        self.fiyu.is_some() || self.note
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        if let Some(f) = &mut self.fiyu {
            return f.height(env.text, env.theme, width);
        }
        notice_height(&mut self.notice, env, width, 0.0)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        if let Some(f) = &mut self.fiyu {
            f.set_rect(rect);
        } else {
            place_notice(&mut self.notice, env.theme, rect, 0.0);
        }
    }

    fn pointer(&mut self, _env: &mut Env, _e: &PointerEvent) -> bool {
        false
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_printers(event) {
            self.update(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.notice.set_theme(theme, now);
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        match &mut self.fiyu {
            Some(f) => f.draw(ui),
            None => self.notice.draw(ui),
        }
    }
}

// ---- Yazıcı listesi -----------------------------------------------------------------------------

/// Açık satırın ayrıntısı: bilgi satırları + düğmeler.
#[derive(Debug)]
struct DetailUi {
    name: String,
    lines: Vec<(Label, Label)>,
    make_default: Button,
    test_page: Option<Button>,
    /// Bilgi satırlarının toplam yüksekliği ve düğme satırının yüksekliği.
    info_h: f32,
    buttons_h: f32,
    key_w: f32,
    rect: Rect,
}

impl DetailUi {
    fn new(theme: &Theme, row: &PrinterRow, test_page: bool, busy: bool) -> Self {
        let lines = row
            .details
            .iter()
            .map(|(k, v)| {
                (Label::new(k.as_str(), text_style(theme, TextRole::Caption).with_max_lines(1)), Label::new(v.as_str(), text_style(theme, TextRole::Caption).with_max_lines(2)))
            })
            .collect();
        let make_default = Button::new(theme, ButtonVariant::Tonal).with_label(theme, t!("settings-printers-make-default"));
        let test_page = test_page.then(|| Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Print)).with_label(theme, t!("settings-printers-test-page")));
        let mut d = Self { name: row.name.clone(), lines, make_default, test_page, info_h: 0.0, buttons_h: 0.0, key_w: 0.0, rect: Rect::ZERO };
        d.update(row, busy, Instant::now());
        d
    }

    fn update(&mut self, row: &PrinterRow, busy: bool, now: Instant) {
        self.make_default.set_label(&if row.is_default { t!("settings-printers-is-default") } else { t!("settings-printers-make-default") });
        self.make_default.set_enabled(!row.is_default && !busy, now);
        if let Some(b) = &mut self.test_page {
            b.set_enabled(!busy, now);
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        for (k, v) in &mut self.lines {
            k.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
            v.set_style(text_style(theme, TextRole::Caption).with_max_lines(2));
        }
        self.make_default.set_theme(theme, now);
        if let Some(b) = &mut self.test_page {
            b.set_theme(theme, now);
        }
    }

    fn indent(theme: &Theme) -> f32 {
        let (px, _) = row_padding(theme);
        px + icon_side(theme) + theme.space(4)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let inner = (width - Self::indent(t) - px).max(t.space(20));
        self.key_w = self.lines.iter_mut().map(|(k, _)| k.measure(env.text).width).fold(0.0, f32::max);
        let vw = (inner - self.key_w - t.space(4)).max(t.space(20));
        self.info_h = 0.0;
        for (k, v) in &mut self.lines {
            v.set_max_width(Some(vw));
            self.info_h += k.measure(env.text).height.max(v.measure(env.text).height) + t.space(1);
        }
        let mut bh = self.make_default.preferred_size(env.text, t).height;
        if let Some(b) = &mut self.test_page {
            bh = bh.max(b.preferred_size(env.text, t).height);
        }
        self.buttons_h = bh + py * 2.0;
        self.info_h + self.buttons_h
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (_, py) = row_padding(t);
        let left = rect.x + Self::indent(t);
        let y = rect.y + self.info_h + py;
        let s = self.make_default.preferred_size(env.text, t);
        self.make_default.set_rect(Rect::new(left, y, s.width, s.height));
        if let Some(b) = &mut self.test_page {
            let bs = b.preferred_size(env.text, t);
            b.set_rect(Rect::new(left + s.width + t.space(3), y, bs.width, bs.height));
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let left = self.rect.x + Self::indent(t);
        let mut y = self.rect.y;
        for (k, v) in &mut self.lines {
            let kh = k.measure_in(ui.painter).height;
            let vh = v.measure_in(ui.painter).height;
            k.draw(ui.painter, Point::new(left, y), ui.fade(c.on_surface_variant));
            v.draw(ui.painter, Point::new(left + self.key_w + t.space(4), y), ui.fade(c.on_surface));
            y += kh.max(vh) + t.space(1);
        }
        self.make_default.draw(ui);
        if let Some(b) = &mut self.test_page {
            b.draw(ui);
        }
    }
}

/// Liste içindeki klavye hedefi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListTarget {
    Row(usize),
    MakeDefault,
    TestPage,
}

#[derive(Debug)]
struct PrinterUi {
    data: PrinterRow,
    item: ItemUi,
    hover: Interaction,
}

/// Yazıcılar kartı. Satırlar yalnız veri değişince yeniden kurulur; açık ayrıntı korunur.
#[derive(Debug)]
pub struct PrinterList {
    desc: &'static SettingDesc,
    data: ListContent,
    rows: Vec<PrinterUi>,
    expanded: Option<String>,
    detail: Option<DetailUi>,
    reading: Label,
    fiyu: Option<FiyuEmpty>,
    focus: Option<ListTarget>,
    focus_visible: bool,
    rect: Rect,
}

impl PrinterList {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        printers::ensure();
        let mut b = Self {
            desc,
            data: ListContent::Hidden,
            rows: Vec::new(),
            expanded: None,
            detail: None,
            reading: Label::new(t!("settings-printers-reading"), text_style(theme, TextRole::Body)),
            fiyu: None,
            focus: None,
            focus_visible: false,
            rect: Rect::ZERO,
        };
        b.update(theme);
        b
    }

    fn update(&mut self, theme: &Theme) {
        let s = printers::snapshot();
        let busy = s.op.is_some();
        let data = list_content(&s);
        let now = Instant::now();
        if data != self.data {
            self.rows = match &data {
                ListContent::Rows(rows) => rows
                    .iter()
                    .map(|r| PrinterUi { item: ItemUi::new(theme, IconName::Print, &r.title, &r.subtitle, r.error, r.active), hover: Interaction::new(theme), data: r.clone() })
                    .collect(),
                _ => Vec::new(),
            };
            self.fiyu = match &data {
                ListContent::Empty => {
                    let (title, body) = (t!("settings-printers-empty"), t!("settings-printers-empty-body"));
                    Some(match self.fiyu.take() {
                        Some(mut f) => {
                            f.set_text(theme, &title, Some(&body));
                            f
                        }
                        None => FiyuEmpty::new(theme, Spot::PrintersEmpty, &title, Some(&body)),
                    })
                }
                _ => None,
            };
            self.data = data;
        }
        // Açık ayrıntı: yazıcı hâlâ listedeyse korunur (düğmeler güncellenir).
        let open = self.expanded.as_deref().and_then(|n| self.rows.iter().find(|r| r.data.name == n)).map(|r| r.data.clone());
        match (open, &mut self.detail) {
            (Some(row), Some(d)) if d.name == row.name => d.update(&row, busy, now),
            (Some(row), _) => self.detail = Some(DetailUi::new(theme, &row, s.test_page, busy)),
            (None, _) => {
                self.expanded = None;
                self.detail = None;
            }
        }
        if let Some(f) = self.focus
            && !self.targets().contains(&f)
        {
            self.focus = self.targets().first().copied();
            self.sync_focus();
        }
    }

    fn targets(&self) -> Vec<ListTarget> {
        let mut v = Vec::new();
        for (i, r) in self.rows.iter().enumerate() {
            v.push(ListTarget::Row(i));
            if self.expanded.as_deref() == Some(r.data.name.as_str())
                && let Some(d) = &self.detail
            {
                if d.make_default.is_enabled() {
                    v.push(ListTarget::MakeDefault);
                }
                if d.test_page.as_ref().is_some_and(Button::is_enabled) {
                    v.push(ListTarget::TestPage);
                }
            }
        }
        v
    }

    fn sync_focus(&mut self) {
        let f = if self.focus_visible { self.focus } else { None };
        if let Some(d) = &mut self.detail {
            d.make_default.set_focus_visible(f == Some(ListTarget::MakeDefault));
            if let Some(b) = &mut d.test_page {
                b.set_focus_visible(f == Some(ListTarget::TestPage));
            }
        }
    }

    fn toggle_expand(&mut self, theme: &Theme, i: usize) {
        let Some(row) = self.rows.get(i).map(|r| r.data.clone()) else { return };
        if self.expanded.as_deref() == Some(row.name.as_str()) {
            self.expanded = None;
            self.detail = None;
        } else {
            let s = printers::snapshot();
            self.detail = Some(DetailUi::new(theme, &row, s.test_page, s.op.is_some()));
            self.expanded = Some(row.name);
        }
        self.sync_focus();
    }

    fn make_default(&self) {
        if let Some(n) = &self.expanded {
            printers::set_default(n);
        }
    }

    fn test_page(&self) {
        if let Some(n) = &self.expanded {
            printers::print_test_page(n, &t!("settings-printers-test-page-title"));
        }
    }

    fn activate(&mut self, theme: &Theme, target: ListTarget) -> bool {
        match target {
            ListTarget::Row(i) => self.toggle_expand(theme, i),
            ListTarget::MakeDefault => self.make_default(),
            ListTarget::TestPage => self.test_page(),
        }
        true
    }
}

impl Block for PrinterList {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.fiyu.as_ref().map(FiyuEmpty::spot)
    }

    fn visible(&self, _store: &Store) -> bool {
        self.data != ListContent::Hidden
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            return f.height(env.text, t, width);
        }
        if self.data == ListContent::Reading {
            self.reading.set_max_width(Some((width - px * 2.0).max(t.space(20))));
            return row_height_for(t, self.reading.measure(env.text).height);
        }
        // `demo_expand` ayrıntıyı temasız açar; bileşen burada kurulur.
        if self.detail.is_none()
            && let Some(row) = self.expanded.as_deref().and_then(|n| self.rows.iter().find(|r| r.data.name == n)).map(|r| r.data.clone())
        {
            let s = printers::snapshot();
            self.detail = Some(DetailUi::new(t, &row, s.test_page, s.op.is_some()));
        }
        let mut total = 0.0;
        for r in &mut self.rows {
            total += r.item.measure(env, width, 0.0);
            if self.expanded.as_deref() == Some(r.data.name.as_str())
                && let Some(d) = &mut self.detail
            {
                total += d.height(env, width);
            }
        }
        total
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        if let Some(f) = &mut self.fiyu {
            f.set_rect(rect);
        }
        let mut y = rect.y;
        let expanded = self.expanded.clone();
        for r in &mut self.rows {
            r.item.rect = Rect::new(rect.x, y, rect.width, r.item.height);
            y += r.item.height;
            if expanded.as_deref() == Some(r.data.name.as_str())
                && let Some(d) = &mut self.detail
            {
                let h = d.info_h + d.buttons_h;
                d.place(env, Rect::new(rect.x, y, rect.width, h));
                y += h;
            }
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let mut repaint = false;
        let mut act = None;
        if let Some(d) = &mut self.detail {
            let r = d.make_default.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                act = Some(ListTarget::MakeDefault);
            }
            if let Some(b) = &mut d.test_page {
                let r = b.pointer(e, now);
                repaint |= r.repaint;
                if r.event.is_some() {
                    act = Some(ListTarget::TestPage);
                }
            }
        }
        let mut row_click = None;
        for (i, r) in self.rows.iter_mut().enumerate() {
            let tap = r.hover.handle(r.item.rect, e, now);
            repaint |= tap.repaint;
            if tap.clicked {
                row_click = Some(i);
            }
        }
        if let Some(target) = act {
            self.activate(env.theme, target);
            return true;
        }
        if let Some(i) = row_click {
            self.toggle_expand(env.theme, i);
            self.focus = Some(ListTarget::Row(i));
            self.sync_focus();
            return true;
        }
        if matches!(e.kind, PointerKind::Press(MouseButton::Left)) && !self.rect.contains(e.position) {
            self.focus_visible = false;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state == KeyState::Released {
            return false;
        }
        match (self.focus, e.key) {
            (Some(target), Key::Enter | Key::Space) => self.activate(env.theme, target),
            (Some(_), Key::Escape) if self.expanded.is_some() => {
                let i = self.rows.iter().position(|r| Some(r.data.name.as_str()) == self.expanded.as_deref());
                if let Some(i) = i {
                    self.toggle_expand(env.theme, i);
                    self.focus = Some(ListTarget::Row(i));
                    self.sync_focus();
                }
                true
            }
            _ => false,
        }
    }

    fn focusable(&self) -> bool {
        !self.targets().is_empty()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focus_visible = visible;
        if !focused {
            self.focus = None;
        } else if self.focus.is_none() && visible {
            self.focus = self.targets().first().copied();
        }
        self.sync_focus();
    }

    fn focus_step(&mut self, forward: bool, _now: Instant) -> bool {
        let targets = self.targets();
        let Some(cur) = self.focus.and_then(|f| targets.iter().position(|&x| x == f)) else { return false };
        let next = if forward { targets.get(cur + 1) } else { cur.checked_sub(1).and_then(|i| targets.get(i)) };
        match next {
            Some(&n) => {
                self.focus_visible = true;
                self.focus = Some(n);
                self.sync_focus();
                true
            }
            None => false,
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if let Some(d) = &self.detail {
            let c = d.make_default.cursor(p).or_else(|| d.test_page.as_ref().and_then(|b| b.cursor(p)));
            if c.is_some() {
                return c;
            }
        }
        self.rows.iter().any(|r| r.item.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn demo_expand(&mut self, _now: Instant) {
        // Canlı test (`NUXUT_SETTINGS_DEBUG=expand=printers.list`): ilk yazıcının ayrıntısı;
        // ayrıntı bileşeni sonraki `update`'te kurulur.
        if self.expanded.is_none() {
            self.expanded = self.rows.first().map(|r| r.data.name.clone());
            self.detail = None;
        }
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_printers(event) {
            if matches!(event, crate::services::Event::LocaleChanged) {
                // Metinler yeni dilde: satırlar ve ayrıntı yeniden kurulur.
                self.data = ListContent::Hidden;
                self.detail = None;
                self.reading.set_text(&t!("settings-printers-reading"));
            }
            self.update(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.reading.set_style(text_style(theme, TextRole::Body));
        if let Some(f) = &mut self.fiyu {
            f.set_theme(theme, now);
        }
        for r in &mut self.rows {
            r.item.set_theme(theme, now);
            r.hover.set_theme(theme, now);
        }
        if let Some(d) = &mut self.detail {
            d.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        if let Some(f) = &mut self.fiyu {
            f.draw(ui);
            return;
        }
        if self.data == ListContent::Reading {
            let h = self.reading.measure_in(ui.painter).height;
            self.reading.draw(ui.painter, Point::new(self.rect.x + px, self.rect.center().y - h * 0.5), ui.fade(t.colors.on_surface_variant));
            return;
        }
        let focus = if self.focus_visible { self.focus } else { None };
        for (i, r) in self.rows.iter_mut().enumerate() {
            r.hover.draw_layer(ui.painter, r.item.rect, Corners::uniform(t.shape.radius_medium * 0.5), t, ui.now);
            if r.hover.is_animating(ui.now) {
                ui.request_frame();
            }
            if focus == Some(ListTarget::Row(i)) {
                draw_ring(ui, r.item.rect);
            }
            r.item.draw(ui);
        }
        if let Some(d) = &mut self.detail
            && self.expanded.is_some()
        {
            d.draw(ui);
        }
    }
}

// ---- Yazıcı ekle (CUPS web arayüzü) -------------------------------------------------------------

/// "Yazıcı ekle" satırı: açıklama + "Aç" düğmesi (`services::open_url`). Yalnız CUPS hazır ve
/// sunucu yerelken.
#[derive(Debug)]
pub struct PrintersAdmin {
    desc: &'static SettingDesc,
    text: RowText,
    open: Button,
    url: Option<String>,
    ready: bool,
    rect: Rect,
}

impl PrintersAdmin {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        printers::ensure();
        let mut text = RowText::new(theme, desc.label());
        text.set_subtitle(desc.description(), false);
        let open = Button::new(theme, ButtonVariant::Tonal).with_icon(theme, Icon::named(IconName::OpenInNew)).with_label(theme, t!("settings-printers-open-web"));
        let mut b = Self { desc, text, open, url: None, ready: false, rect: Rect::ZERO };
        b.update();
        b
    }

    fn update(&mut self) {
        let s = printers::snapshot();
        self.ready = s.status == Status::Ready;
        self.url = s.admin_url;
    }

    fn open(&self) {
        if let Some(u) = &self.url {
            crate::services::open_url(u);
        }
    }
}

impl Block for PrintersAdmin {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        self.ready && self.url.is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let bs = self.open.preferred_size(env.text, t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - bs.width).max(t.space(20)));
        row_height_for(t, text.max(bs.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let bs = self.open.preferred_size(env.text, t);
        self.open.set_rect(Rect::new(rect.right() - px - bs.width, rect.center().y - bs.height * 0.5, bs.width, bs.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let r = self.open.pointer(e, env.now);
        if r.event.is_some() {
            self.open();
            return true;
        }
        r.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let r = self.open.key(e, env.now);
        if r.event.is_some() {
            self.open();
            return true;
        }
        r.repaint
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.open.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.open.cursor(p)
    }

    fn sync(&mut self, _env: &mut Env) {
        self.update();
    }

    fn service_event(&mut self, _env: &mut Env, event: &crate::services::Event) -> bool {
        if is_printers(event) {
            if matches!(event, crate::services::Event::LocaleChanged) {
                self.text.set_subtitle(self.desc.description(), false);
                self.open.set_label(&t!("settings-printers-open-web"));
            }
            self.update();
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.open.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let (px, _) = row_padding(ui.theme);
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.center().y - h * 0.5);
        self.open.draw(ui);
    }
}

// ---- İş kuyruğu ---------------------------------------------------------------------------------

#[derive(Debug)]
struct JobUi {
    id: i32,
    item: ItemUi,
    cancel: Button,
}

/// Kullanıcının bitmemiş işleri; satırda "İptal".
#[derive(Debug)]
pub struct PrintJobs {
    desc: &'static SettingDesc,
    data: Option<Vec<(i32, String, String)>>,
    rows: Vec<JobUi>,
    empty: Label,
    focus: Option<usize>,
    focus_visible: bool,
    rect: Rect,
}

impl PrintJobs {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        printers::ensure();
        let mut b = Self { desc, data: None, rows: Vec::new(), empty: Label::new(t!("settings-printers-jobs-empty"), text_style(theme, TextRole::Body)), focus: None, focus_visible: false, rect: Rect::ZERO };
        b.update(theme);
        b
    }

    fn update(&mut self, theme: &Theme) {
        let s = printers::snapshot();
        let data = job_rows(&s);
        let now = Instant::now();
        if data != self.data {
            self.rows = data
                .iter()
                .flatten()
                .map(|(id, title, sub)| JobUi {
                    id: *id,
                    item: ItemUi::new(theme, IconName::File, title, sub, false, false),
                    cancel: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-printers-job-cancel")),
                })
                .collect();
            self.data = data;
            // İptal edilen satır gitti: odak son satıra.
            if self.focus.is_some_and(|i| i >= self.rows.len()) {
                self.focus = self.rows.len().checked_sub(1);
            }
        }
        let cancelling = match s.op {
            Some(Op::Cancel(id)) => Some(id),
            _ => None,
        };
        let busy = s.op.is_some();
        for r in &mut self.rows {
            r.cancel.set_enabled(!busy, now);
            r.cancel.set_label(&if cancelling == Some(r.id) { t!("settings-printers-job-cancelling") } else { t!("settings-printers-job-cancel") });
        }
        self.sync_focus();
    }

    fn sync_focus(&mut self) {
        let f = if self.focus_visible { self.focus } else { None };
        for (i, r) in self.rows.iter_mut().enumerate() {
            r.cancel.set_focus_visible(f == Some(i));
        }
    }
}

impl Block for PrintJobs {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn visible(&self, _store: &Store) -> bool {
        self.data.is_some()
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        if self.rows.is_empty() {
            self.empty.set_max_width(Some((width - px * 2.0).max(t.space(20))));
            return row_height_for(t, self.empty.measure(env.text).height);
        }
        let mut total = 0.0;
        for r in &mut self.rows {
            let bw = r.cancel.preferred_size(env.text, t).width + t.space(3);
            total += r.item.measure(env, width, bw);
        }
        total
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let mut y = rect.y;
        for r in &mut self.rows {
            r.item.rect = Rect::new(rect.x, y, rect.width, r.item.height);
            let s = r.cancel.preferred_size(env.text, t);
            r.cancel.set_rect(Rect::new(rect.right() - px - s.width, r.item.rect.center().y - s.height * 0.5, s.width, s.height));
            y += r.item.height;
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        let mut cancel = None;
        for r in &mut self.rows {
            let res = r.cancel.pointer(e, env.now);
            repaint |= res.repaint;
            if res.event.is_some() {
                cancel = Some(r.id);
            }
        }
        if let Some(id) = cancel {
            printers::cancel_job(id);
            return true;
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let Some(r) = self.focus.and_then(|i| self.rows.get_mut(i)) else { return false };
        let res = r.cancel.key(e, env.now);
        if res.event.is_some() {
            printers::cancel_job(r.id);
            return true;
        }
        res.repaint
    }

    fn focusable(&self) -> bool {
        self.rows.iter().any(|r| r.cancel.is_enabled())
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.focus_visible = visible;
        if !focused {
            self.focus = None;
        } else if self.focus.is_none() {
            self.focus = (!self.rows.is_empty()).then_some(0);
        }
        self.sync_focus();
    }

    fn focus_step(&mut self, forward: bool, _now: Instant) -> bool {
        let Some(cur) = self.focus else { return false };
        let next = if forward { cur + 1 } else { cur.wrapping_sub(1) };
        if next < self.rows.len() {
            self.focus = Some(next);
            self.focus_visible = true;
            self.sync_focus();
            return true;
        }
        false
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.rows.iter().find_map(|r| r.cancel.cursor(p))
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.theme);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if is_printers(event) {
            if matches!(event, crate::services::Event::LocaleChanged) {
                self.data = None;
                self.empty.set_text(&t!("settings-printers-jobs-empty"));
            }
            self.update(env.theme);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.empty.set_style(text_style(theme, TextRole::Body));
        for r in &mut self.rows {
            r.item.set_theme(theme, now);
            r.cancel.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        if self.rows.is_empty() {
            let h = self.empty.measure_in(ui.painter).height;
            self.empty.draw(ui.painter, Point::new(self.rect.x + px, self.rect.center().y - h * 0.5), ui.fade(t.colors.on_surface_variant));
            return;
        }
        for r in &mut self.rows {
            r.item.draw(ui);
            r.cancel.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use nuxut::i18n::{Catalog, LanguagePrefs};

    use super::*;
    use crate::services::printers::{State, cups};

    fn printer(name: &str, info: &str, state: PrinterState) -> Printer {
        Printer { name: name.into(), info: info.into(), state, accepting: true, uri: cups::printer_uri(name), ..Printer::default() }
    }

    fn ready(printers: Vec<Printer>, default: Option<&str>, jobs: Vec<Job>) -> Snapshot {
        Snapshot { status: Status::Ready, state: State { printers, default: default.map(str::to_owned), jobs }, test_page: true, ..Snapshot::default() }
    }

    #[test]
    fn status_and_visibility() {
        for st in [Status::Unavailable, Status::Failed] {
            let s = Snapshot { status: st, ..Snapshot::default() };
            assert!(unavailable_text(st).is_some());
            assert_eq!(list_content(&s), ListContent::Hidden, "maskot sayfanın tek içeriği");
            assert_eq!(job_rows(&s), None);
        }
        assert_eq!(unavailable_text(Status::Ready), None);
        assert_eq!(list_content(&Snapshot::default()), ListContent::Reading);
        let empty = ready(vec![], None, vec![]);
        assert_eq!(list_content(&empty), ListContent::Empty);
        assert_eq!(job_rows(&empty), None, "yazıcı yokken kuyruk kartı yok");
        assert_eq!(job_rows(&ready(vec![printer("a", "", PrinterState::Idle)], None, vec![])), Some(vec![]));
    }

    #[test]
    fn printer_rows() {
        let mut stopped = printer("depo", "", PrinterState::Stopped);
        stopped.message = "Paused".into();
        stopped.accepting = false;
        stopped.model = "HP LaserJet".into();
        let mut s = ready(vec![printer("ofis", "Ofis yazıcısı", PrinterState::Processing), stopped], Some("ofis"), vec![]);
        s.sent = Some("depo".into());
        let ListContent::Rows(rows) = list_content(&s) else { panic!() };
        assert_eq!(rows[0].title, "Ofis yazıcısı");
        assert!(rows[0].is_default && rows[0].active && !rows[0].error);
        assert_eq!(rows[0].subtitle, format!("{}{SEP}{}", t!("settings-printers-default"), t!("settings-printers-state-processing")));
        assert_eq!(rows[0].details, [(t!("settings-printers-detail-queue"), "ofis".to_owned())], "boş model/konum gösterilmez");
        assert!(!rows[1].is_default && rows[1].error);
        assert_eq!(
            rows[1].subtitle,
            [t!("settings-printers-state-stopped"), t!("settings-printers-not-accepting"), "Paused".into(), t!("settings-printers-test-page-sent")].join(SEP)
        );
        assert_eq!(rows[1].details.len(), 2);
    }

    #[test]
    fn job_texts() {
        let ps = vec![printer("ofis", "Ofis yazıcısı", PrinterState::Idle)];
        let j = Job { id: 7, name: "".into(), printer: "ofis".into(), state: JobState::Held, size_kb: Some(12), created: None };
        let s = ready(ps, None, vec![j]);
        let rows = job_rows(&s).unwrap();
        assert_eq!(rows[0].0, 7);
        assert_eq!(rows[0].1, t!("settings-printers-job-untitled", id = 7i64));
        let size = crate::numfmt::format(12.0, 0);
        assert_eq!(rows[0].2, ["Ofis yazıcısı".to_owned(), t!("settings-printers-job-held"), t!("settings-printers-job-size", size = size.as_str())].join(SEP));
    }

    #[test]
    fn failure_texts() {
        let f = |op: Op, error: OpError| failure_text(&Failure { op, error });
        assert_eq!(f(Op::Cancel(1), OpError::Cups(CupsError::Ipp(0x0401))), t!("settings-printers-error-denied"));
        assert_eq!(f(Op::Cancel(1), OpError::Cups(CupsError::Ipp(0x0500))), t!("settings-printers-error-cancel"));
        assert_eq!(f(Op::TestPage("a".into(), "t".into()), OpError::Cups(CupsError::Malformed)), t!("settings-printers-error-test-page"));
        assert_eq!(f(Op::TestPage("a".into(), "t".into()), OpError::Cups(CupsError::Document)), t!("settings-printers-error-test-page-missing"));
        assert_eq!(f(Op::SetDefault("a".into()), OpError::Write), t!("settings-printers-error-write"));
        assert_eq!(f(Op::SetDefault("a".into()), OpError::NoPrinter), t!("settings-printers-error-gone"));
        assert_eq!(f(Op::SetDefault("a".into()), OpError::Cups(CupsError::Timeout)), t!("settings-printers-error-unreachable"));
    }

    /// Türkçe kaynak metinler (2026-10-15).
    #[test]
    fn turkish_texts() {
        let tr = Catalog::load(&[crate::i18n::source()], &LanguagePrefs::fixed(&["tr"]));
        assert_eq!(tr.text("settings-page-printers"), "Yazıcılar");
        assert_eq!(tr.text("settings-printers-unavailable"), "Yazdırma servisi yok");
        assert_eq!(tr.text("settings-printers-empty"), "Yazıcı yok");
        let mut a = nuxut::i18n::Args::new();
        a.set("size", "1.250");
        assert_eq!(tr.format("settings-printers-job-size", &a), "1.250 KB");
    }
}
