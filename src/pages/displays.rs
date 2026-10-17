//! Ekranlar: monitör yerleşimi (sürükle-bırak tuval) ve seçili monitörün ayarları. Değişiklik
//! shell'e `displays apply` ile tam istenen durum olarak gider (shell ADR-0005 §13). Onay
//! isteyen değişiklikte shell'in onay penceresi henüz yok: sayfa geri sayımla "Koru / Geri al"
//! gösterir; süre dolarsa shell kendisi geri döner, sonuç `status.last`'tan gösterilir.
//! Yerleşim mantığı (yapışma, oturma, doğrulama) `services::displays` modelindedir.
//! `displays identify` donuk v1 sözleşmesinde yok (§13.4): "Tanımla" düğmesi shell'e gelince eklenir.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::motion::Animated;
use nuxut::ui::input::{Button as MouseButton, CursorShape, KeyEvent, PointerEvent, PointerKind};
use nuxut::ui::style::{HAIRLINE, divider, field_fill, tonal_container};
use nuxut::ui::{Button, ButtonVariant, EmptyState, Icon, IconName, IconView, Label, MAX_SEGMENTS, MIN_SEGMENTS, Segment, SegmentedButton, Select, SelectEvent, TextRole, Toggle, Ui, text_style};
use nuxut::{Border, Point, Rect, RectStyle, Size, Theme};

use super::fiyu::{FiyuEmpty, Spot};
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{row_height_for, row_padding, should_stack, side_text_width};
use crate::services::displays::shell::{self, ApplyFlow, Command, Phase, Unavailable, Update, countdown_text, error_text, outcome_text};
use crate::services::displays::Outcome;
use crate::services::displays::{self as model, DisplayEdit, Issue, Layout, List, Mode, Vrr};
use crate::services::{Event, Request};

/// VRR seçenekleri (etiketler katalogdan: [`vrr_label`]).
const VRR: [Vrr; 4] = [Vrr::Off, Vrr::On, Vrr::Fullscreen, Vrr::FullscreenMedia];
/// Döndürme seçenek sayısı (0..=3 çeyrek tur; etiketler: [`rotation_label`]).
const ROTATION_STEPS: usize = 4;

/// Vrr'in kullanıcıya görünen etiketi (katalogdan).
fn vrr_label(v: Vrr) -> String {
    match v {
        Vrr::Off => t!("settings-choice-vrr-off"),
        Vrr::On => t!("settings-choice-vrr-on"),
        Vrr::Fullscreen => t!("settings-choice-vrr-fullscreen"),
        Vrr::FullscreenMedia => t!("settings-choice-vrr-fullscreen-media"),
    }
}

/// Döndürme adımının etiketi (katalogdan; ilk adım döndürme yok).
fn rotation_label(step: usize) -> String {
    if step == 0 { t!("settings-choice-rotation-none") } else { format!("{}°", step * 90) }
}

/// Katalogdaki `.description` özniteliği (yoksa boş).
fn description(id: &str) -> String {
    nuxut::i18n::current().attr(id, "description", None).map_or_else(String::new, |s| s.into_owned())
}

/// Tuvalde mantıksal monitör dikdörtgeni.
fn to_rect(r: model::Rect) -> Rect {
    Rect::new(r.x as f32, r.y as f32, r.w as f32, r.h as f32)
}

/// Yerleşimin tuvale sığan görünümü (mantıksal koordinat): kenarlarda boşluk, tuvalin en-boy
/// oranında, ortalı.
fn fit_view(rects: &[Rect], aspect: f32) -> Rect {
    let Some(first) = rects.first() else { return Rect::new(0.0, 0.0, 1920.0, 1920.0 / aspect.max(0.1)) };
    let b = rects.iter().skip(1).fold(*first, |a, r| a.union(r));
    let pad = b.width.max(b.height) * 0.08;
    let (mut w, mut h) = (b.width + pad * 2.0, b.height + pad * 2.0);
    if w / h < aspect {
        w = h * aspect;
    } else {
        h = w / aspect;
    }
    let c = b.center();
    Rect::new(c.x - w * 0.5, c.y - h * 0.5, w, h)
}

/// Yenileme hızı (mHz) etkin biçim locale'inin ondalık ayırıcısıyla: "59,997 Hz" / "59.997 Hz".
pub fn format_rate(mhz: u32) -> String {
    format_rate_in(mhz, &crate::numfmt::current())
}

/// [`format_rate`] verilen sayı locale'iyle (saf; testler).
pub fn format_rate_in(mhz: u32, locale: &nuxut::locale::Locale) -> String {
    format!("{} Hz", crate::numfmt::format_in(locale, f64::from(mhz) / 1000.0, 3))
}

/// Ölçek yüzde olarak (Ayarlar'ın tek yüzde biçimi, [`crate::numfmt::percent`]).
pub fn format_scale(s120: u32) -> String {
    format_scale_in(s120, &crate::numfmt::current())
}

/// [`format_scale`] verilen sayı locale'iyle (saf; testler).
pub fn format_scale_in(s120: u32, locale: &nuxut::locale::Locale) -> String {
    crate::numfmt::percent_in(locale, f64::from(s120) / f64::from(model::SCALE_DENOM), 0)
}

/// Ölçek denetiminin türü (§5.1 kontrol kuralı): 2–5 seçenek `SegmentedButton` (framework
/// sınırı `MIN_SEGMENTS..=MAX_SEGMENTS`), daha çoğu ya da tek seçenek açılır seçim. Yaygın
/// 1920×1080'de temiz ölçek 9 tanedir (%100…%400): segment ya okunmaz daralır ya da satırı
/// taşar; açılır seçim tüm temiz ölçekleri kayıpsız gösterir.
fn scale_kind(labels: Vec<String>, selected: Option<usize>) -> SpecKind {
    if (MIN_SEGMENTS..=MAX_SEGMENTS).contains(&labels.len()) {
        SpecKind::Segmented(labels, selected.unwrap_or(0))
    } else {
        SpecKind::Select(labels, selected.unwrap_or(0))
    }
}

/// Doğrulama bulgusunun kullanıcı metni.
fn issue_text(issue: &Issue, name: &dyn Fn(&str) -> String) -> String {
    match issue {
        Issue::NoActiveDisplay => t!("settings-displays-issue-no-active"),
        Issue::Overlap(a, b) => t!("settings-displays-issue-overlap", first = name(a), second = name(b)),
        Issue::Disconnected => t!("settings-displays-issue-disconnected"),
        Issue::InvalidScale(id) => t!("settings-displays-issue-invalid-scale", display = name(id)),
        Issue::BadMirror(id) => t!("settings-displays-issue-bad-mirror", display = name(id)),
    }
}

#[derive(Debug)]
struct Mon {
    id: String,
    /// Mantıksal dikdörtgen (sürüklerken anında, oturunca yayla).
    rect: Animated<Rect>,
    selected: Animated<f32>,
    name: Label,
    detail: Label,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Enabled,
    Primary,
    Resolution,
    Rate,
    Scale,
    Rotation,
    Flip,
    Mirror,
    Vrr,
}

#[derive(Debug)]
// En çok dokuz satır: varyant boyu farkı önemsiz, kutulamak yalnızca dolaylılık ekler.
#[allow(clippy::large_enum_variant)]
enum Ctl {
    Toggle(Toggle),
    Select { select: Select, labels: Vec<String> },
    /// Etiketler tutulur: aynı sayıda ama farklı seçenek gelirse (ör. çözünürlük değişip ölçek
    /// listesi değişti) denetim yeniden kurulur.
    Segmented { seg: SegmentedButton, labels: Vec<String> },
}

#[derive(Debug)]
struct Row {
    field: Field,
    title: Label,
    subtitle: Option<Label>,
    ctl: Ctl,
    rect: Rect,
    /// Son ölçümde yığılmış mı (denetim başlığın altında; [`should_stack`]).
    stack: bool,
}

/// Bir satırın istenen içeriği (seçenekler + seçili).
struct Spec {
    field: Field,
    title: String,
    subtitle: String,
    kind: SpecKind,
}

enum SpecKind {
    Toggle(bool),
    Select(Vec<String>, usize),
    Segmented(Vec<String>, usize),
}

#[derive(Debug, Clone, Copy)]
struct Drag {
    /// İmlecin monitörün sol üstüne göre mantıksal uzaklığı.
    grab: Point,
    start: Point,
    moved: bool,
    /// Sürükleme boyunca sabit görünüm (tuval kaymasın).
    view: Rect,
}

#[derive(Debug)]
pub struct Displays {
    desc: &'static SettingDesc,
    rect: Rect,
    list: Option<List>,
    base: Option<Layout>,
    edit: Option<Layout>,
    unavailable: Option<Unavailable>,
    flow: ApplyFlow,
    selected: Option<String>,
    /// Hotplug düzenlemeyi sıfırladı: bir kez söylenir.
    reset_note: bool,
    // Tuval.
    mons: Vec<Mon>,
    view: Animated<Rect>,
    canvas: Rect,
    drag: Option<(String, Drag)>,
    // Monitör seçici.
    tab_ids: Vec<String>,
    tabs: Vec<Button>,
    /// Seçici satırının alt kenarı (yerleşimden; ipucu ve uyarılar buna göre).
    tabs_bottom: f32,
    hint: Label,
    rows: Vec<Row>,
    issues: Vec<Label>,
    warn: IconView,
    status: Label,
    reset: Button,
    apply: Button,
    /// Kabuk yokken sayfanın tek kartı (boş/hata durumu, [`EmptyView`]).
    empty: Option<EmptyView>,
    loading: Label,
}

impl Displays {
    pub fn new(theme: &Theme, desc: &'static SettingDesc) -> Self {
        Self {
            desc,
            rect: Rect::ZERO,
            list: None,
            base: None,
            edit: None,
            unavailable: None,
            flow: ApplyFlow::default(),
            selected: None,
            reset_note: false,
            mons: Vec::new(),
            view: Animated::new(Rect::new(0.0, 0.0, 1920.0, 1080.0), theme.motion.spatial),
            canvas: Rect::ZERO,
            drag: None,
            tab_ids: Vec::new(),
            tabs: Vec::new(),
            tabs_bottom: 0.0,
            hint: Label::new("", text_style(theme, TextRole::Caption)),
            rows: Vec::new(),
            issues: Vec::new(),
            warn: IconView::with_role(theme, Icon::named(IconName::Warning), TextRole::IconSmall),
            status: Label::new("", text_style(theme, TextRole::Caption)),
            reset: Button::new(theme, ButtonVariant::Text).with_label(theme, t!("settings-displays-button-cancel")),
            apply: Button::new(theme, ButtonVariant::Filled).with_label(theme, t!("settings-displays-button-apply")),
            empty: None,
            loading: Label::new(t!("settings-displays-loading"), text_style(theme, TextRole::Caption)),
        }
    }

    fn ready(&self) -> bool {
        self.unavailable.is_none() && self.edit.is_some()
    }

    fn changed(&self) -> bool {
        matches!((&self.edit, &self.base), (Some(e), Some(b)) if e.differs_from(b))
    }

    fn display_info(&self, id: &str) -> Option<&model::Display> {
        self.list.as_ref()?.displays.iter().find(|d| d.id == id)
    }

    /// Kullanıcıya gösterilen ad; aynı adlı iki monitörde bağlayıcı eklenir.
    fn name_of(&self, id: &str) -> String {
        let Some(list) = &self.list else { return id.to_owned() };
        let Some(d) = list.displays.iter().find(|d| d.id == id) else { return id.to_owned() };
        let same = list.displays.iter().filter(|o| o.name() == d.name()).count();
        if same > 1 && !d.connector.is_empty() {
            t!("settings-displays-name-connector", name = d.name(), connector = d.connector.as_str())
        } else {
            d.name().to_owned()
        }
    }

    fn selected_edit(&self) -> Option<&DisplayEdit> {
        self.edit.as_ref()?.get(self.selected.as_deref()?)
    }

    // ---- Shell olayları ------------------------------------------------------------------------

    fn on_list(&mut self, list: List) {
        self.unavailable = None;
        let fresh = Layout::from_list(&list);
        // Sonuçlandı ya da değişiklik ekranda bekliyor: düzenleme shell'in durumuna eşitlenir.
        let resolved = matches!(self.flow.phase(), Phase::Done { .. } | Phase::Awaiting { .. } | Phase::Deciding { .. } | Phase::Resolving { .. });
        match &mut self.edit {
            Some(edit) if !resolved && self.base.as_ref().is_some_and(|b| edit.differs_from(b)) => {
                if !edit.rebase(&fresh) {
                    self.reset_note = true;
                }
            }
            _ => self.edit = Some(fresh.clone()),
        }
        self.base = Some(fresh);
        self.list = Some(list);
        let ids: Vec<String> = self.edit.as_ref().map(|l| l.displays.iter().map(|d| d.id.clone()).collect()).unwrap_or_default();
        if self.selected.as_ref().is_none_or(|s| !ids.contains(s)) {
            let primary = self.edit.as_ref().and_then(|l| l.primary.clone()).filter(|p| ids.contains(p));
            self.selected = primary.or_else(|| ids.first().cloned());
        }
    }

    fn handle(&mut self, env: &mut Env, update: &Update) {
        match update {
            Update::Unavailable(u) => {
                self.unavailable = Some(u.clone());
                self.flow.on_lost();
                self.drag = None;
            }
            Update::List(list) => self.on_list(list.as_ref().clone()),
            Update::Status(s) => self.flow.on_status(s),
            Update::Applied(r) => {
                self.flow.on_reply(r.clone());
                if let Err(e) = r
                    && e.needs_refresh()
                {
                    env.push(Action::Service(Request::Displays(Command::Refresh)));
                }
            }
            Update::Decided(r) => self.flow.on_decided(r.clone()),
        }
        self.rebuild(env.theme, env.now);
    }

    // ---- Yeniden kurulum -----------------------------------------------------------------------

    /// Durumdan tuval, sekmeler, satırlar ve metinler (bileşenler değişmediyse korunur).
    fn rebuild(&mut self, theme: &Theme, now: Instant) {
        self.rebuild_empty(theme);
        self.rebuild_mons(theme, now);
        self.rebuild_tabs(theme, now);
        self.rebuild_rows(theme, now);
        self.rebuild_footer(theme, now);
    }

    fn rebuild_empty(&mut self, theme: &Theme) {
        let Some(u) = &self.unavailable else {
            self.empty = None;
            return;
        };
        let (title, body) = match u {
            Unavailable::NotRunning => (t!("settings-displays-empty-not-running-title"), t!("settings-displays-empty-not-running-body")),
            Unavailable::Unsupported => (t!("settings-displays-empty-unsupported-title"), t!("settings-displays-empty-unsupported-body")),
            Unavailable::Other(m) => (t!("settings-displays-empty-error-title"), m.clone()),
        };
        let spot = empty_spot(u);
        match &mut self.empty {
            Some(e) if e.spot() == spot => e.set_text(theme, &title, Some(&body)),
            _ => self.empty = Some(EmptyView::new(theme, spot, &title, &body)),
        }
    }

    fn rebuild_mons(&mut self, theme: &Theme, now: Instant) {
        let Some(edit) = self.edit.clone() else {
            self.mons.clear();
            return;
        };
        let placed: Vec<&DisplayEdit> = edit.displays.iter().filter(|d| d.placed()).collect();
        self.mons.retain(|m| placed.iter().any(|d| d.id == m.id));
        for d in &placed {
            let r = to_rect(d.rect());
            let sel = if self.selected.as_deref() == Some(&d.id) { 1.0 } else { 0.0 };
            let name = self.name_of(&d.id);
            let detail = if d.scale_120 == model::SCALE_DENOM { format!("{} × {}", d.mode.w, d.mode.h) } else { format!("{} × {} · {}", d.mode.w, d.mode.h, format_scale(d.scale_120)) };
            match self.mons.iter_mut().find(|m| m.id == d.id) {
                Some(m) => {
                    let dragging = self.drag.as_ref().is_some_and(|(id, _)| *id == d.id);
                    if !dragging && m.rect.target() != r {
                        m.rect.set_target(r, now);
                    }
                    m.selected.set_target(sel, now);
                    m.name.set_text(&name);
                    m.detail.set_text(&detail);
                }
                None => {
                    let mut selected = Animated::new(sel, theme.motion.effects);
                    selected.jump_to(sel);
                    self.mons.push(Mon {
                        id: d.id.clone(),
                        rect: Animated::new(r, theme.motion.spatial),
                        selected,
                        name: Label::new(name, text_style(theme, TextRole::Label).with_max_lines(1)),
                        detail: Label::new(detail, text_style(theme, TextRole::Caption).with_max_lines(1)),
                    });
                }
            }
        }
        // Seçili en üstte çizilir.
        self.mons.sort_by_key(|m| self.selected.as_deref() == Some(&m.id));
        if self.drag.is_none() {
            self.refit_view(now);
        }
    }

    fn refit_view(&mut self, now: Instant) {
        if self.canvas.is_empty() {
            return;
        }
        let rects: Vec<Rect> = self.mons.iter().map(|m| m.rect.target()).collect();
        let v = fit_view(&rects, self.canvas.width / self.canvas.height);
        if self.view.target() != v {
            // İlk yerleşimde atlanır; sonra yayla.
            if self.view.target().width == 1920.0 && self.view.target().height == 1080.0 {
                self.view.jump_to(v);
            } else {
                self.view.set_target(v, now);
            }
        }
    }

    fn rebuild_tabs(&mut self, theme: &Theme, now: Instant) {
        let ids: Vec<String> = self.edit.as_ref().map(|l| l.displays.iter().map(|d| d.id.clone()).collect()).unwrap_or_default();
        let ids = if ids.len() < 2 { Vec::new() } else { ids };
        if ids != self.tab_ids {
            self.tabs = ids.iter().map(|id| Button::new(theme, ButtonVariant::Outlined).with_label(theme, self.name_of(id))).collect();
            self.tab_ids = ids;
        }
        for (id, b) in self.tab_ids.iter().zip(&mut self.tabs) {
            b.set_variant(chip_variant(self.selected.as_deref() == Some(id.as_str())));
            b.set_enabled(true, now);
        }
        let hint = self.selected.as_deref().and_then(|s| self.display_info(s)).filter(|d| !d.id_stable).map(|d| {
            t!("settings-displays-hint-connector", connector = d.connector.as_str())
        });
        self.hint.set_text(hint.as_deref().unwrap_or(""));
    }

    fn specs(&self) -> Vec<Spec> {
        let (Some(edit), Some(id)) = (&self.edit, self.selected.as_deref()) else { return Vec::new() };
        let Some(d) = edit.get(id) else { return Vec::new() };
        let info = self.display_info(id);
        let mut out = vec![Spec { field: Field::Enabled, title: t!("settings-displays-enabled"), subtitle: description("settings-displays-enabled"), kind: SpecKind::Toggle(d.enabled) }];
        if !d.enabled {
            return out;
        }
        let others: Vec<&DisplayEdit> = edit.displays.iter().filter(|o| o.id != id).collect();
        if !others.is_empty() {
            let mut labels = vec![t!("settings-choice-mirror-none")];
            labels.extend(others.iter().map(|o| self.name_of(&o.id)));
            let sel = d.mirror_of.as_ref().and_then(|m| others.iter().position(|o| &o.id == m)).map_or(0, |i| i + 1);
            out.push(Spec { field: Field::Mirror, title: t!("settings-displays-mirror"), subtitle: description("settings-displays-mirror"), kind: SpecKind::Select(labels, sel) });
        }
        let mirroring = d.mirror_of.is_some();
        if !mirroring && others.iter().any(|o| o.enabled) {
            out.push(Spec {
                field: Field::Primary,
                title: t!("settings-displays-primary"),
                subtitle: description("settings-displays-primary"),
                kind: SpecKind::Toggle(edit.primary.as_deref() == Some(id)),
            });
        }
        let resolutions = self.resolutions(d, info);
        let res_labels = resolutions
            .iter()
            .map(|(w, h, pref)| {
                let r = format!("{w} × {h}");
                if *pref { t!("settings-choice-resolution-preferred", resolution = r) } else { r }
            })
            .collect();
        let res_sel = resolutions.iter().position(|(w, h, _)| *w == d.mode.w && *h == d.mode.h).unwrap_or(0);
        out.push(Spec { field: Field::Resolution, title: t!("settings-displays-resolution"), subtitle: String::new(), kind: SpecKind::Select(res_labels, res_sel) });
        let rates = self.rates(d, info);
        let rate_sel = rates.iter().position(|r| *r == d.mode.mhz).unwrap_or(0);
        out.push(Spec { field: Field::Rate, title: t!("settings-displays-rate"), subtitle: String::new(), kind: SpecKind::Select(rates.iter().map(|r| format_rate(*r)).collect(), rate_sel) });
        if !mirroring {
            let scales = self.scales(d, info);
            let sel = scales.iter().position(|s| *s == d.scale_120);
            out.push(Spec {
                field: Field::Scale,
                title: t!("settings-displays-scale"),
                subtitle: description("settings-displays-scale"),
                kind: scale_kind(scales.iter().map(|s| format_scale(*s)).collect(), sel),
            });
            out.push(Spec {
                field: Field::Rotation,
                title: t!("settings-displays-rotation"),
                subtitle: String::new(),
                kind: SpecKind::Segmented((0..ROTATION_STEPS).map(rotation_label).collect(), model::rotation(d.transform) as usize),
            });
            out.push(Spec { field: Field::Flip, title: t!("settings-displays-flip"), subtitle: description("settings-displays-flip"), kind: SpecKind::Toggle(model::flipped(d.transform)) });
        }
        let vrr = VRR.iter().position(|v| *v == d.vrr).unwrap_or(0);
        out.push(Spec {
            field: Field::Vrr,
            title: t!("settings-displays-vrr"),
            subtitle: description("settings-displays-vrr"),
            kind: SpecKind::Select(VRR.iter().map(|v| vrr_label(*v)).collect(), vrr),
        });
        out
    }

    /// Çözünürlükler (büyükten küçüğe) + seçilen kipin listede olmadığı durumda o da.
    fn resolutions(&self, d: &DisplayEdit, info: Option<&model::Display>) -> Vec<(u32, u32, bool)> {
        let mut v: Vec<(u32, u32, bool)> = info.map(|i| i.resolutions.iter().map(|r| (r.w, r.h, r.preferred)).collect()).unwrap_or_default();
        if !v.iter().any(|(w, h, _)| *w == d.mode.w && *h == d.mode.h) {
            v.push((d.mode.w, d.mode.h, false));
        }
        v.sort_by_key(|(w, h, _)| std::cmp::Reverse((*w as u64) * (*h as u64)));
        v
    }

    fn rates(&self, d: &DisplayEdit, info: Option<&model::Display>) -> Vec<u32> {
        let mut v: Vec<u32> = info.and_then(|i| i.resolution(d.mode.w, d.mode.h)).map(|r| r.rates_mhz.clone()).unwrap_or_default();
        if !v.contains(&d.mode.mhz) {
            v.push(d.mode.mhz);
        }
        v.sort_unstable_by(|a, b| b.cmp(a));
        v
    }

    fn scales(&self, d: &DisplayEdit, info: Option<&model::Display>) -> Vec<u32> {
        let res = info.and_then(|i| i.resolution(d.mode.w, d.mode.h));
        let mut v = model::scale_choices(res, d.mode);
        if !v.contains(&d.scale_120) {
            v.push(d.scale_120);
            v.sort_unstable();
        }
        v
    }

    fn rebuild_rows(&mut self, theme: &Theme, now: Instant) {
        let specs = self.specs();
        let reuse = specs.len() == self.rows.len() && specs.iter().zip(&self.rows).all(|(s, r)| s.field == r.field);
        if !reuse {
            self.rows = specs.into_iter().map(|s| Self::make_row(theme, s)).collect();
            return;
        }
        for (s, row) in specs.into_iter().zip(self.rows.iter_mut()) {
            let same = match (&s.kind, &mut row.ctl) {
                (SpecKind::Toggle(on), Ctl::Toggle(t)) => {
                    t.set_on(*on, now);
                    true
                }
                (SpecKind::Select(l, i), Ctl::Select { select, labels }) if l == labels => {
                    select.set_selected(*i);
                    true
                }
                (SpecKind::Segmented(l, i), Ctl::Segmented { seg, labels }) if l == labels => {
                    seg.set_selected(*i, now);
                    true
                }
                _ => false,
            };
            if !same {
                *row = Self::make_row(theme, s);
            }
        }
    }

    fn make_row(theme: &Theme, s: Spec) -> Row {
        let ctl = match s.kind {
            SpecKind::Toggle(on) => Ctl::Toggle(Toggle::new(theme, on)),
            SpecKind::Select(labels, i) => Ctl::Select { select: Select::new(theme, labels.clone(), i), labels },
            SpecKind::Segmented(labels, i) => Ctl::Segmented { seg: SegmentedButton::new(theme, labels.iter().map(|l| Segment::new(l.clone())).collect(), i), labels },
        };
        Row {
            field: s.field,
            title: Label::new(s.title, text_style(theme, TextRole::Body)),
            subtitle: (!s.subtitle.is_empty()).then(|| Label::new(s.subtitle, text_style(theme, TextRole::Caption))),
            ctl,
            rect: Rect::ZERO,
            stack: false,
        }
    }

    fn rebuild_footer(&mut self, theme: &Theme, now: Instant) {
        let issues = self.edit.as_ref().map(Layout::validate).unwrap_or_default();
        let texts: Vec<String> = issues.iter().map(|i| issue_text(i, &|id| self.name_of(id))).collect();
        if !texts.iter().eq(self.issues.iter().map(|l| l.text())) {
            self.issues = texts.into_iter().map(|t| Label::new(t, text_style(theme, TextRole::Body))).collect();
        }
        let changed = self.changed();
        let name = |id: &str| self.display_info(id).map(|_| self.name_of(id));
        let status = match self.flow.phase() {
            Phase::Sending => t!("settings-displays-status-applying"),
            Phase::Awaiting { remaining_ms, .. } => countdown_text(*remaining_ms),
            Phase::Deciding { keep: true, .. } => t!("settings-displays-status-keeping"),
            Phase::Deciding { keep: false, .. } => t!("settings-displays-status-reverting"),
            Phase::Resolving { .. } => t!("settings-displays-status-resolving"),
            Phase::Done { outcome, detail } => outcome_text(*outcome, detail.as_deref()),
            Phase::Failed(e) => error_text(e, &name),
            Phase::Idle if self.reset_note => t!("settings-displays-status-reset"),
            Phase::Idle if changed => t!("settings-displays-status-unsaved"),
            Phase::Idle => t!("settings-displays-status-hint"),
        };
        self.status.set_text(&status);
        let busy = self.flow.busy();
        // Onay beklerken alt çubuk "Geri al / Koru" olur (shell'in onay penceresi gelene dek).
        let awaiting = matches!(self.flow.phase(), Phase::Awaiting { .. });
        let apply_label = if awaiting { t!("settings-displays-button-keep") } else { t!("settings-displays-button-apply") };
        let reset_label = if awaiting { t!("settings-displays-button-revert") } else { t!("settings-displays-button-cancel") };
        self.apply.set_label(&apply_label);
        self.reset.set_label(&reset_label);
        self.apply.set_enabled(awaiting || (changed && issues.is_empty() && !busy && self.unavailable.is_none()), now);
        self.reset.set_enabled(awaiting || (changed && !busy), now);
    }

    /// Durum satırı hata tonunda mı (başarısız ya da kullanıcı istemeden geri dönüldü).
    fn status_is_error(&self) -> bool {
        match self.flow.phase() {
            Phase::Failed(_) => true,
            Phase::Done { outcome, .. } => !outcome.kept() && *outcome != Outcome::Reverted,
            _ => false,
        }
    }

    // ---- Düzenleme -----------------------------------------------------------------------------

    fn edited(&mut self, theme: &Theme, now: Instant) {
        self.flow.dismiss();
        self.reset_note = false;
        self.rebuild(theme, now);
    }

    fn set_field(&mut self, field: Field, index: usize, on: bool) {
        let Some(id) = self.selected.clone() else { return };
        let info = self.display_info(&id).cloned();
        let Some(d) = self.selected_edit().cloned() else { return };
        let others: Vec<String> = self.edit.as_ref().map(|l| l.displays.iter().filter(|o| o.id != id).map(|o| o.id.clone()).collect()).unwrap_or_default();
        let resolutions = self.resolutions(&d, info.as_ref());
        let rates = self.rates(&d, info.as_ref());
        let scales = self.scales(&d, info.as_ref());
        let Some(edit) = self.edit.as_mut() else { return };
        match field {
            Field::Enabled => edit.set_enabled(&id, on),
            Field::Primary => edit.primary = if on { Some(id) } else { None },
            Field::Mirror => edit.set_mirror(&id, index.checked_sub(1).and_then(|i| others.get(i).cloned())),
            Field::Resolution => {
                let Some(&(w, h, _)) = resolutions.get(index) else { return };
                let rates: Vec<u32> = info.as_ref().and_then(|i| i.resolution(w, h)).map(|r| r.rates_mhz.clone()).unwrap_or_default();
                let mhz = model::nearest_rate(&rates, d.mode.mhz).unwrap_or(d.mode.mhz);
                edit.set_mode(&id, Mode { w, h, mhz });
            }
            Field::Rate => {
                let Some(&mhz) = rates.get(index) else { return };
                edit.set_mode(&id, Mode { mhz, ..d.mode });
            }
            Field::Scale => {
                let Some(&s) = scales.get(index) else { return };
                edit.set_scale(&id, s);
            }
            Field::Rotation => edit.set_transform(&id, model::transform_of(index as u8, model::flipped(d.transform))),
            Field::Flip => edit.set_transform(&id, model::transform_of(model::rotation(d.transform), on)),
            Field::Vrr => {
                if let Some(e) = edit.get_mut(&id) {
                    e.vrr = VRR.get(index).map_or(Vrr::Off, |v| *v);
                }
            }
        }
    }

    fn select_display(&mut self, id: &str, theme: &Theme, now: Instant) {
        if self.selected.as_deref() != Some(id) {
            self.selected = Some(id.to_owned());
            self.rebuild(theme, now);
        }
    }

    fn send_apply(&mut self, env: &mut Env) {
        let Some(edit) = &self.edit else { return };
        if let Ok(json) = self.flow.begin(edit) {
            env.push(Action::Service(Request::Displays(Command::Apply(json))));
        }
        self.rebuild_footer(env.theme, env.now);
    }

    // ---- Tuval ---------------------------------------------------------------------------------

    /// Mantıksal → tuval dönüşümü (`view` mantıksal görünüm).
    fn to_screen(&self, view: Rect, r: Rect) -> Rect {
        let s = self.canvas.width / view.width.max(1.0);
        Rect::new(self.canvas.x + (r.x - view.x) * s, self.canvas.y + (r.y - view.y) * s, r.width * s, r.height * s)
    }

    fn to_logical(&self, view: Rect, p: Point) -> Point {
        let s = view.width.max(1.0) / self.canvas.width.max(1.0);
        Point::new(view.x + (p.x - self.canvas.x) * s, view.y + (p.y - self.canvas.y) * s)
    }

    fn canvas_pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        match e.kind {
            PointerKind::Press(MouseButton::Left) if self.canvas.contains(e.position) => {
                let view = self.view.value(now);
                let hit = self.mons.iter().rev().find(|m| self.to_screen(view, m.rect.value(now)).contains(e.position)).map(|m| (m.id.clone(), m.rect.value(now)));
                let Some((id, r)) = hit else { return false };
                self.view.jump_to(view);
                let p = self.to_logical(view, e.position);
                let busy = self.flow.busy();
                self.select_display(&id, env.theme, now);
                if !busy {
                    self.drag = Some((id, Drag { grab: Point::new(p.x - r.x, p.y - r.y), start: e.position, moved: false, view }));
                }
                true
            }
            PointerKind::Motion => {
                let Some((id, mut drag)) = self.drag.clone() else { return false };
                if !drag.moved && (e.position.x - drag.start.x).abs().max((e.position.y - drag.start.y).abs()) < env.theme.space(1) {
                    return false;
                }
                drag.moved = true;
                let p = self.to_logical(drag.view, e.position);
                let s = drag.view.width / self.canvas.width.max(1.0);
                let threshold = (env.theme.space(3) * s).round() as i32;
                let Some(edit) = self.edit.as_mut() else { return false };
                edit.drag_to(&id, (p.x - drag.grab.x).round() as i32, (p.y - drag.grab.y).round() as i32, threshold);
                let r = edit.get(&id).map(|d| to_rect(d.rect()));
                if let (Some(m), Some(r)) = (self.mons.iter_mut().find(|m| m.id == id), r) {
                    m.rect.jump_to(r);
                }
                self.drag = Some((id, drag));
                true
            }
            PointerKind::Release(MouseButton::Left) | PointerKind::Leave => {
                let Some((id, drag)) = self.drag.take() else { return false };
                if drag.moved
                    && let Some(edit) = self.edit.as_mut()
                {
                    edit.settle(&id);
                    self.edited(env.theme, now);
                }
                true
            }
            _ => false,
        }
    }

    fn select_open(&self) -> bool {
        self.rows.iter().any(|r| matches!(&r.ctl, Ctl::Select { select, .. } if select.is_open()))
    }

    // ---- Yerleşim ------------------------------------------------------------------------------

    fn canvas_height(theme: &Theme, width: f32) -> f32 {
        (width * 0.36).clamp(theme.space(44), theme.space(72))
    }

    fn row_height(&mut self, env: &mut Env, i: usize, width: f32) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let row = &mut self.rows[i];
        let ctl = control_size(&mut row.ctl, env.text, t, width);
        // Anahtar yerinde kalır; seçim ve segmentli dar/kalabalık satırda başlığın altına iner.
        row.stack = !matches!(row.ctl, Ctl::Toggle(_)) && should_stack(t, env.text, width, ctl.width, &mut row.title, row.subtitle.as_mut());
        let text_w = if row.stack { width - px * 2.0 } else { side_text_width(t, width, ctl.width).max(t.space(20)) };
        let h = text_height(row, env.text, t, text_w);
        if row.stack {
            return h + t.space(2) + ctl.height + py * 2.0;
        }
        row_height_for(t, h.max(ctl.height))
    }

    /// Yükseklik; `place` ise bileşenler yerleştirilir.
    fn arrange(&mut self, env: &mut Env, rect: Rect, place: bool) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let inner_w = rect.width - px * 2.0;
        if let Some(e) = &mut self.empty {
            // Boş durum kendi iç boşluğunu (space(4)) taşır; kartın tamamını kaplar.
            let h = e.height(env.text, t, rect.width) + py * 2.0;
            if place {
                e.set_rect(Rect::new(rect.x, rect.y, rect.width, h));
            }
            return h;
        }
        let mut y = rect.y + py;
        if self.edit.is_none() {
            return self.loading.measure(env.text).height + py * 2.0;
        }
        // Tuval.
        let ch = Self::canvas_height(t, inner_w);
        if place {
            let canvas = Rect::new(rect.x + px, y, inner_w, ch);
            if canvas.width != self.canvas.width || canvas.height != self.canvas.height {
                self.canvas = canvas;
                self.refit_view(env.now);
            }
            self.canvas = canvas;
        }
        y += ch + t.space(3);
        // Monitör seçici.
        let (_, tabs_h) = flow_buttons(&mut self.tabs, env.text, t, Rect::new(rect.x + px, y, inner_w, 0.0), place);
        y += tabs_h;
        if place {
            self.tabs_bottom = y;
        }
        if !self.hint.text().is_empty() {
            self.hint.set_max_width(Some(inner_w));
            y += t.space(2) + self.hint.measure(env.text).height;
        }
        y += t.space(2);
        // Satırlar.
        for i in 0..self.rows.len() {
            let h = self.row_height(env, i, rect.width);
            if place {
                let r = Rect::new(rect.x, y, rect.width, h);
                place_row(&mut self.rows[i], env, r, rect);
            }
            y += h;
        }
        // Uyarılar.
        let icon_w = self.warn.size().width + t.space(2);
        if !self.issues.is_empty() {
            y += t.space(2);
            for l in &mut self.issues {
                l.set_max_width(Some(inner_w - icon_w));
                y += l.measure(env.text).height + t.space(1);
            }
        }
        // Alt çubuk.
        y += t.space(3);
        let as_ = self.apply.preferred_size(env.text, t);
        let rs = self.reset.preferred_size(env.text, t);
        self.status.set_max_width(Some((inner_w - as_.width - rs.width - t.space(6)).max(t.space(20))));
        let bar_h = as_.height.max(self.status.measure(env.text).height);
        if place {
            let cy = y + bar_h * 0.5;
            self.apply.set_rect(Rect::new(rect.right() - px - as_.width, cy - as_.height * 0.5, as_.width, as_.height));
            self.reset.set_rect(Rect::new(rect.right() - px - as_.width - t.space(2) - rs.width, cy - rs.height * 0.5, rs.width, rs.height));
        }
        y += bar_h + py;
        y - rect.y
    }
}

fn chip_variant(selected: bool) -> ButtonVariant {
    if selected { ButtonVariant::Tonal } else { ButtonVariant::Outlined }
}

/// Başlık + açıklama yüksekliği `text_w` genişliğinde.
fn text_height(row: &mut Row, text: &mut nuxut::TextSystem, t: &Theme, text_w: f32) -> f32 {
    row.title.set_max_width(Some(text_w));
    let mut h = row.title.measure(text).height;
    if let Some(s) = &mut row.subtitle {
        s.set_max_width(Some(text_w));
        h += t.space(1) + s.measure(text).height;
    }
    h
}

fn control_size(ctl: &mut Ctl, text: &mut nuxut::TextSystem, t: &Theme, width: f32) -> Size {
    match ctl {
        Ctl::Toggle(tg) => tg.preferred_size(t),
        Ctl::Select { select, .. } => select.preferred_size(t, (width * 0.42).clamp(t.space(44), t.space(80))),
        Ctl::Segmented { seg, .. } => seg.preferred_size(text, t),
    }
}

/// Satırı yerleştirir (`row.stack` son ölçümden): yan yana düzende denetim sağda dikey
/// ortada; yığılmışta metnin altında, satır genişliğinde.
fn place_row(row: &mut Row, env: &mut Env, r: Rect, block: Rect) {
    let t = env.theme;
    let (px, py) = row_padding(t);
    row.rect = r;
    let ctl = control_size(&mut row.ctl, env.text, t, r.width);
    let cr = if row.stack {
        let text = text_height(row, env.text, t, r.width - px * 2.0);
        Rect::new(r.x + px, r.y + py + text + t.space(2), r.width - px * 2.0, ctl.height)
    } else {
        let cy = r.y + r.height * 0.5;
        Rect::new(r.right() - px - ctl.width, cy - ctl.height * 0.5, ctl.width, ctl.height)
    };
    match &mut row.ctl {
        Ctl::Toggle(tg) => tg.set_rect(cr),
        Ctl::Select { select, .. } => {
            select.set_rect(t, cr);
            // Menü bloğun içinde kalır (sayfa blokları kendi alanına kırpar).
            select.set_bounds(block);
        }
        Ctl::Segmented { seg, .. } => seg.set_rect(t, cr),
    }
}

/// Düğmeleri `area` genişliğinde soldan sağa sarar; `(son satır y, toplam yükseklik)`.
fn flow_buttons(buttons: &mut [Button], text: &mut nuxut::TextSystem, theme: &Theme, area: Rect, place: bool) -> (f32, f32) {
    let gap = theme.space(2);
    let (mut x, mut y, mut row_h) = (area.x, area.y, 0.0f32);
    for b in buttons.iter_mut() {
        let s = b.preferred_size(text, theme);
        if x > area.x && x + s.width > area.right() {
            x = area.x;
            y += row_h + gap;
            row_h = 0.0;
        }
        if place {
            b.set_rect(Rect::new(x, y, s.width, s.height));
        }
        x += s.width + gap;
        row_h = row_h.max(s.height);
    }
    (y, y + row_h - area.y)
}

impl Block for Displays {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn mascot(&self) -> Option<Spot> {
        self.empty.as_ref().and_then(EmptyView::spot)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.arrange(env, Rect::new(0.0, 0.0, width, 0.0), false)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        // Sayfa görünür: yoklama açık (ayrılınca `App::navigate` kapatır).
        shell::watch(true);
        self.rect = rect;
        self.arrange(env, rect, true);
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        let theme = env.theme;
        if !self.ready() {
            return false;
        }
        // Açık menü kalıcı: olay yalnızca ona.
        if let Some(i) = self.rows.iter().position(|r| matches!(&r.ctl, Ctl::Select { select, .. } if select.is_open())) {
            let Ctl::Select { select, .. } = &mut self.rows[i].ctl else { return false };
            let r = select.pointer(env.text, theme, e, now);
            if let Some(SelectEvent::Changed(idx)) = r.event {
                let field = self.rows[i].field;
                self.set_field(field, idx, false);
                self.edited(theme, now);
                return true;
            }
            return r.repaint;
        }
        let mut repaint = false;
        if self.canvas_pointer(env, e) {
            return true;
        }
        let mut picked = None;
        for (id, b) in self.tab_ids.iter().zip(&mut self.tabs) {
            let r = b.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                picked = Some(id.clone());
            }
        }
        if let Some(id) = picked {
            self.select_display(&id, theme, now);
            return true;
        }
        let busy = self.flow.busy();
        let mut change: Option<(Field, usize, bool)> = None;
        for row in &mut self.rows {
            let field = row.field;
            match &mut row.ctl {
                Ctl::Toggle(tg) => {
                    let r = tg.pointer(e, now);
                    repaint |= r.repaint;
                    if let Some(on) = r.event {
                        change = Some((field, 0, on));
                    }
                }
                Ctl::Select { select, .. } => {
                    let r = select.pointer(env.text, theme, e, now);
                    repaint |= r.repaint;
                    if let Some(SelectEvent::Changed(i)) = r.event {
                        change = Some((field, i, false));
                    }
                }
                Ctl::Segmented { seg, .. } => {
                    let r = seg.pointer(e, now);
                    repaint |= r.repaint;
                    if let Some(i) = r.event {
                        change = Some((field, i, false));
                    }
                }
            }
        }
        if let Some((field, i, on)) = change {
            if !busy {
                self.set_field(field, i, on);
            }
            // Meşgulken de denetimler modele geri döner.
            self.edited(theme, now);
            return true;
        }
        let r = self.reset.pointer(e, now);
        repaint |= r.repaint;
        if r.event.is_some() {
            if let Some(token) = self.flow.revert() {
                env.push(Action::Service(Request::Displays(Command::Revert(token))));
                self.rebuild_footer(theme, now);
            } else {
                self.edit = self.base.clone();
                self.edited(theme, now);
            }
            return true;
        }
        let r = self.apply.pointer(e, now);
        repaint |= r.repaint;
        if r.event.is_some() {
            if let Some(token) = self.flow.keep() {
                env.push(Action::Service(Request::Displays(Command::Confirm(token))));
                self.rebuild_footer(theme, now);
            } else {
                self.send_apply(env);
            }
            return true;
        }
        repaint
    }

    fn wants_scroll(&self, p: Point) -> bool {
        self.rows.iter().any(|r| matches!(&r.ctl, Ctl::Select { select, .. } if select.input_rect().is_some_and(|ir| ir.contains(p))))
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        let now = env.now;
        for i in 0..self.rows.len() {
            let Ctl::Select { select, .. } = &mut self.rows[i].ctl else { continue };
            if !select.is_open() {
                continue;
            }
            let r = select.key(env.text, env.theme, e, now);
            if let Some(SelectEvent::Changed(idx)) = r.event {
                let field = self.rows[i].field;
                self.set_field(field, idx, false);
                self.edited(env.theme, now);
                return true;
            }
            return r.repaint || self.select_open();
        }
        false
    }

    fn focusable(&self) -> bool {
        self.ready()
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if !self.ready() {
            return None;
        }
        for r in &self.rows {
            if let Ctl::Select { select, .. } = &r.ctl
                && select.is_open()
            {
                return select.cursor(p);
            }
        }
        if self.drag.as_ref().is_some_and(|(_, d)| d.moved) {
            return Some(CursorShape::Grabbing);
        }
        if self.canvas.contains(p) {
            let now = Instant::now();
            let view = self.view.value(now);
            if self.mons.iter().any(|m| self.to_screen(view, m.rect.value(now)).contains(p)) {
                return Some(CursorShape::Grab);
            }
        }
        self.tabs
            .iter()
            .find_map(|b| b.cursor(p))
            .or_else(|| {
                self.rows.iter().find_map(|r| match &r.ctl {
                    Ctl::Toggle(t) => t.cursor(p),
                    Ctl::Select { select, .. } => select.cursor(p),
                    Ctl::Segmented { seg, .. } => seg.cursor(p),
                })
            })
            .or_else(|| self.reset.cursor(p))
            .or_else(|| self.apply.cursor(p))
    }

    fn service_event(&mut self, env: &mut Env, event: &Event) -> bool {
        match event {
            Event::Displays(u) => self.handle(env, u),
            // Biçim locale'i değişti: yenileme hızı etiketleri yeni ayırıcıyla kurulur.
            Event::LocaleChanged => self.rebuild(env.theme, env.now),
            _ => return false,
        }
        true
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.view.set_motion(theme.motion.spatial, now);
        for m in &mut self.mons {
            m.rect.set_motion(theme.motion.spatial, now);
            m.selected.set_motion(theme.motion.effects, now);
            m.name.set_style(text_style(theme, TextRole::Label).with_max_lines(1));
            m.detail.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
        }
        for b in &mut self.tabs {
            b.set_theme(theme, now);
        }
        self.hint.set_style(text_style(theme, TextRole::Caption));
        // Açılır seçimlerin teması metin sistemi ister: satırlar yeniden kurulur.
        self.rows.clear();
        self.rebuild_rows(theme, now);
        for l in &mut self.issues {
            l.set_style(text_style(theme, TextRole::Body));
        }
        self.warn.set_theme(theme, now);
        self.status.set_style(text_style(theme, TextRole::Caption));
        self.reset.set_theme(theme, now);
        self.apply.set_theme(theme, now);
        if let Some(e) = &mut self.empty {
            e.set_theme(theme, now);
        }
        self.loading.set_style(text_style(theme, TextRole::Caption));
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let rect = self.rect;
        if let Some(e) = &mut self.empty {
            e.draw(ui);
            return;
        }
        if self.edit.is_none() {
            self.loading.draw(ui.painter, Point::new(rect.x + px, rect.y + py), ui.fade(c.on_surface_variant));
            return;
        }
        self.draw_canvas(ui);
        for b in &mut self.tabs {
            b.draw(ui);
        }
        if !self.hint.text().is_empty() {
            let y = self.rows.first().map_or(self.tabs_bottom, |r| r.rect.y) - t.space(2) - self.hint.measure_in(ui.painter).height;
            self.hint.draw(ui.painter, Point::new(rect.x + px, y), ui.fade(c.on_surface_variant));
        }
        for row in &mut self.rows {
            let r = row.rect;
            let th = row.title.measure_in(ui.painter).height;
            let sh = row.subtitle.as_mut().map(|s| s.measure_in(ui.painter).height + t.space(1)).unwrap_or(0.0);
            let ty = if row.stack { r.y + py } else { r.y + (r.height - th - sh) * 0.5 };
            row.title.draw(ui.painter, Point::new(r.x + px, ty), ui.fade(c.on_surface));
            if let Some(s) = &mut row.subtitle {
                s.draw(ui.painter, Point::new(r.x + px, ty + th + t.space(1)), ui.fade(c.on_surface_variant));
            }
            match &mut row.ctl {
                Ctl::Toggle(tg) => tg.draw(ui),
                Ctl::Select { select, .. } => select.draw(ui),
                Ctl::Segmented { seg, .. } => seg.draw(ui),
            }
        }
        let mut y = self.rows.last().map_or(self.tabs_bottom, |r| r.rect.bottom()) + t.space(2);
        let ws = self.warn.size();
        for l in &mut self.issues {
            let h = l.measure_in(ui.painter).height;
            self.warn.draw_centered(ui.painter, Point::new(rect.x + px + ws.width * 0.5, y + h * 0.5), ui.fade(c.error));
            l.draw(ui.painter, Point::new(rect.x + px + ws.width + t.space(2), y), ui.fade(c.on_surface));
            y += h + t.space(1);
        }
        let ar = self.apply.rect();
        let sh = self.status.measure_in(ui.painter).height;
        let tone = if self.status_is_error() { c.error } else { c.on_surface_variant };
        self.status.draw(ui.painter, Point::new(rect.x + px, ar.center().y - sh * 0.5), ui.fade(tone));
        self.reset.draw(ui);
        self.apply.draw(ui);
        // Açık menü her şeyin üstünde (blok alanı içinde).
        for row in &mut self.rows {
            if let Ctl::Select { select, .. } = &mut row.ctl
                && select.is_open()
            {
                select.draw_overlay(ui);
            }
        }
    }
}

impl Displays {
    fn draw_canvas(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let canvas = self.canvas;
        ui.painter.rect(canvas, &RectStyle::filled(ui.fade(c.surface), t.shape.medium()));
        let view = self.view.value(now);
        let mut animating = !self.view.is_settled(now);
        ui.painter.push_clip(canvas);
        let gap = t.space(1) * 0.5;
        for i in 0..self.mons.len() {
            let r = self.to_screen(view, self.mons[i].rect.value(now));
            let m = &mut self.mons[i];
            animating |= !m.rect.is_settled(now) || !m.selected.is_settled(now);
            let sel = m.selected.value(now).clamp(0.0, 1.0);
            // Komşular arasında ince boşluk: kenar teması görünür kalsın.
            let r = r.inset(nuxut::Insets::uniform(gap));
            let fill = field_fill(t).lerp(&tonal_container(t), sel);
            let border = Border { width: HAIRLINE + sel * 1.5, color: divider(t).lerp(&c.primary, sel) };
            ui.painter.rect(r, &RectStyle { fill: ui.fade(fill), radii: t.shape.small(), border: Some(Border { color: ui.fade(border.color), ..border }) });
            let pad = t.space(2);
            m.name.set_max_width(Some((r.width - pad * 2.0).max(0.0)));
            m.detail.set_max_width(Some((r.width - pad * 2.0).max(0.0)));
            let nh = m.name.measure_in(ui.painter);
            let dh = m.detail.measure_in(ui.painter);
            if r.height > nh.height + dh.height + pad && r.width > t.space(10) {
                ui.painter.push_clip(r);
                let top = r.center().y - (nh.height + dh.height) * 0.5;
                m.name.draw(ui.painter, Point::new(r.center().x - nh.width * 0.5, top), ui.fade(c.on_surface));
                m.detail.draw(ui.painter, Point::new(r.center().x - dh.width * 0.5, top + nh.height), ui.fade(c.on_surface_variant));
                ui.painter.pop_clip();
            }
        }
        ui.painter.pop_clip();
        if animating {
            ui.request_frame();
        }
    }
}

// ---- Kabuk yokken ------------------------------------------------------------------------------

/// Kabuk yokken kartın maskot yeri: çalışmıyor / yanıt vermiyor → hata maskotu
/// ([`Spot::DisplaysUnavailable`]); sürüm desteklemiyor bir arıza değil → maskot yok, ekran ikonu.
pub fn empty_spot(u: &Unavailable) -> Option<Spot> {
    match u {
        Unavailable::NotRunning | Unavailable::Other(_) => Some(Spot::DisplaysUnavailable),
        Unavailable::Unsupported => None,
    }
}

/// Kabuk yokken kartın içeriği: maskotlu ya da ikonlu boş durum (framework `EmptyState`).
#[derive(Debug)]
enum EmptyView {
    Mascot(FiyuEmpty),
    Icon(EmptyState),
}

impl EmptyView {
    fn new(theme: &Theme, spot: Option<Spot>, title: &str, body: &str) -> Self {
        match spot {
            Some(s) => Self::Mascot(FiyuEmpty::new(theme, s, title, Some(body))),
            None => Self::Icon(EmptyState::new(theme, title).with_icon(theme, Icon::named(IconName::Monitor)).with_body(theme, body)),
        }
    }

    fn spot(&self) -> Option<Spot> {
        match self {
            Self::Mascot(m) => Some(m.spot()),
            Self::Icon(_) => None,
        }
    }

    fn set_text(&mut self, theme: &Theme, title: &str, body: Option<&str>) {
        match self {
            Self::Mascot(m) => m.set_text(theme, title, body),
            Self::Icon(e) => {
                e.set_title(title);
                e.set_body(theme, body);
            }
        }
    }

    fn height(&mut self, text: &mut nuxut::TextSystem, theme: &Theme, width: f32) -> f32 {
        match self {
            Self::Mascot(m) => m.height(text, theme, width),
            Self::Icon(e) => e.measure(text, theme, width).height,
        }
    }

    fn set_rect(&mut self, r: Rect) {
        match self {
            Self::Mascot(m) => m.set_rect(r),
            Self::Icon(e) => e.set_rect(r),
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        match self {
            Self::Mascot(m) => m.set_theme(theme, now),
            Self::Icon(e) => e.set_theme(theme, now),
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        match self {
            Self::Mascot(m) => m.draw(ui),
            Self::Icon(e) => e.draw(ui),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_fits_layout_with_canvas_aspect() {
        let v = fit_view(&[Rect::new(0.0, 0.0, 2560.0, 1440.0), Rect::new(2560.0, 0.0, 1920.0, 1080.0)], 2.0);
        assert!((v.width / v.height - 2.0).abs() < 1e-3);
        assert!(v.x < 0.0 && v.right() > 4480.0, "kenarlarda boşluk");
        assert!((v.center().x - 2240.0).abs() < 1e-3, "ortalı");
    }

    #[test]
    fn labels_are_readable() {
        let tr = nuxut::locale::Locale::parse("tr_TR.UTF-8").unwrap();
        let en = nuxut::locale::Locale::parse("en_US.UTF-8").unwrap();
        assert_eq!(format_rate_in(60000, &tr), "60 Hz");
        assert_eq!(format_rate_in(59997, &tr), "59,997 Hz");
        assert_eq!(format_rate_in(143850, &tr), "143,85 Hz");
        assert_eq!(format_rate_in(59997, &en), "59.997 Hz");
        assert_eq!(format_rate_in(60000, &en), "60 Hz");
        // Ölçek Ayarlar'ın tek yüzde biçiminden (framework; işaret konumu locale'den).
        assert_eq!(format_scale_in(180, &tr), crate::numfmt::percent_in(&tr, 1.5, 0));
        assert!(format_scale_in(160, &tr).contains("133") && !format_scale_in(160, &tr).contains(','));
        assert!(format_scale_in(120, &en).contains("100%"));
    }

    #[test]
    fn scale_uses_segments_only_when_few() {
        let labels = |n: usize| (0..n).map(|i| format!("{i}")).collect::<Vec<_>>();
        assert!(matches!(scale_kind(labels(4), Some(2)), SpecKind::Segmented(_, 2)));
        assert!(matches!(scale_kind(labels(2), None), SpecKind::Segmented(_, 0)));
        // 1920×1080'in temiz ölçekleri 9 tane: açılır seçim.
        let fhd = model::scale_choices(None, Mode { w: 1920, h: 1080, mhz: 60000 });
        assert!(fhd.len() > MAX_SEGMENTS, "{fhd:?}");
        assert!(matches!(scale_kind(labels(fhd.len()), Some(0)), SpecKind::Select(..)));
        assert!(matches!(scale_kind(labels(1), Some(0)), SpecKind::Select(..)), "tek seçenek segment olmaz");
    }

    fn row(theme: &Theme, kind: SpecKind) -> Row {
        Displays::make_row(theme, Spec { field: Field::Rotation, title: "Döndürme".into(), subtitle: "Ekranın yönü".into(), kind })
    }

    /// Dar sütunda segmentli satır yığılır: denetim başlık ve açıklamanın altında, satır
    /// içinde; geniş sütunda sağda.
    #[test]
    fn narrow_rows_stack_control_below_text() {
        crate::i18n::use_turkish_for_tests();
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let dir = tempfile::tempdir().unwrap();
        let store = crate::store::Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let desc = crate::registry::by_id("displays").unwrap();
        let mut d = Displays::new(&theme, desc);
        d.rows = vec![row(&theme, SpecKind::Segmented((0..ROTATION_STEPS).map(rotation_label).collect(), 0))];
        let (px, py) = row_padding(&theme);
        for (w, stacked) in [(theme.space(260), false), (theme.space(110), true)] {
            let h = d.row_height(&mut env, 0, w);
            assert_eq!(d.rows[0].stack, stacked, "{w}");
            place_row(&mut d.rows[0], &mut env, Rect::new(0.0, 0.0, w, h), Rect::new(0.0, 0.0, w, 1000.0));
            let Ctl::Segmented { seg, .. } = &d.rows[0].ctl else { unreachable!() };
            let s = seg.rect();
            assert!(s.bottom() <= h + 0.5 && s.right() <= w - px + 0.5 && s.x >= px - 0.5, "{w}: satırın içinde {s:?}");
            if stacked {
                let title = d.rows[0].title.measure(env.text).height;
                assert!(s.y >= py + title, "kontrol başlığın altında: {s:?}");
            } else {
                assert!(s.x > w * 0.4, "geniş: sağda {s:?}");
            }
        }
    }

    /// Maskot yerleşimi: kabuk çalışmıyor / yanıt vermiyor → hata maskotu; sürüm desteklemiyor →
    /// ikonlu boş durum (maskot yok); liste gelince boş durum kalkar. Gizliyken yer kaplamaz.
    #[test]
    fn unavailable_shell_shows_error_mascot() {
        crate::i18n::use_turkish_for_tests();
        let dir = tempfile::tempdir().unwrap();
        let store = crate::store::Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let desc = crate::registry::by_id("displays").unwrap();
        let mut heights = Vec::new();
        for theme in [Theme::default(), crate::pages::fiyu::tests::hidden_theme()] {
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
            let mut d = Displays::new(&theme, desc);
            for (u, want) in [
                (Unavailable::Unsupported, None),
                (Unavailable::NotRunning, Some(Spot::DisplaysUnavailable)),
                (Unavailable::Other("x".into()), Some(Spot::DisplaysUnavailable)),
            ] {
                assert_eq!(empty_spot(&u), want);
                d.handle(&mut env, &Update::Unavailable(u));
                assert_eq!(d.mascot(), want);
                assert_eq!(want.map(Spot::mood), want.map(|_| nuxut::ui::Mood::Error));
            }
            heights.push(d.height(&mut env, theme.space(200)));
        }
        assert!((heights[0] - heights[1] - (96.0 + Theme::default().space(4))).abs() <= 1.0, "gizliyken maskot yeri kapanır: {heights:?}");
    }
}
