//! Görünüm › Uygulamalar: ikon ve imleç teması kart ızgarası (t103; açılır seçimin yerini aldı).
//! Hoş geldin'in "Uygulamalar" adımı da aynı bloğu gösterir (kayıttaki satır; orada yalnızca
//! birleşik görünüm açıkken).
//!
//! Veriler shell'in `msg look status` yanıtından (sürüm 3, shell ADR-0004 §17-5); Ayarlar aile,
//! paket ya da dizin adı tahmin etmez.
//!
//! - İlk kart **Önerilen**: anahtar silinir, shell önerdiğini uygular. Adı shell'in
//!   `icons.recommended` / `cursors.recommended`'ından; shell yoksa adsız "Önerilen". Önerilen tema
//!   listede başka bir kart olarak tekrar görünmez. Önerilen kurulu ve ailesinin birden çok kurulu
//!   varyantı varsa önerilen kart o ailenin kartıdır (başlık aile adı).
//! - Ardından katalogdaki (`icons.catalog` / `cursors.catalog`) kurulu aileler, katalog sırasıyla:
//!   **aile başına tek kart**. Birden çok kurulu varyantı olan ailenin kartı ([`CardSpec::variants`])
//!   seçiliyken ızgaranın altında varyant seçici açılır (2–5 varyant bölümlü düğme, sığmazsa ya da
//!   daha çoksa açılır seçim); varyant seçmek config'e o dizin adını yazar (önerilen varyant
//!   anahtarı siler). Tek varyantlı aile ve katalog dışındaki kurulu temalar tekil karttır; sonra
//!   dosyada yazılı ama kurulu olmayan seçim (soluk).
//! - En sonda **kurulu olmayan aileler** soluk kart olarak ("Kurulu değil", AUR ise "· AUR"):
//!   seçilmez — config'e var olmayan tema adı yazılmaz. Karta tık/Enter ızgaranın altında o ailenin
//!   kurulum komutunu gösterir ve kopyalama düğmesi sunar: repo `sudo pacman -S`, AUR kurulu
//!   yardımcıyla (paru, yoksa yay; ikisi de yoksa `paru -S` + "AUR yardımcısı gerekir" notu —
//!   [`look::aur_helper`]).
//! - Seçili tema kurulu değilse (önerilen ya da dosyadaki) ızgaranın altında not; önerilen ailenin
//!   paketi katalogda biliniyorsa kurulum komutu da.
//! - Eski shell (katalog yok) ya da shell yok: yalnızca önerilen + kurulu temalar; komut yok.
//! - Önizleme yuvası: kurulu temanın kartında framework `CursorPreview` / `IconThemePreview`
//!   ([`super::theme_card_preview`]); aile kartında etkin varyantın (seçiliyse dosyadaki, önerilen
//!   kartta öneri, yoksa ilk) önizlemesi — varyant değişince kart yeniden kurulur ve yuva o
//!   varyantı ister (aynı kimliğin önizlemesi taşınır). Kurulu olmayan kartta tür/indirme ikonlu
//!   yer tutucu. Kart `surface_bright` dolgulu, yuva `surface` kuyusu.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent, PointerKind};
use nuxut::ui::style::{FOCUS_RING, HAIRLINE, faded_mark, faded_text, focus_ring, hairline};
use nuxut::ui::{
    Button, ButtonVariant, Icon, IconName, IconView, Interaction, Label, MAX_SEGMENTS, MIN_SEGMENTS, Segment, SegmentedButton, Select, SelectEvent, TextRole, Ui,
    text_style,
};
use nuxut::{Border, Color, Corners, Point, Rect, RectStyle, TextSystem, Theme};
use toml_edit::Value;

use super::look::{APPLY_SETTLE, Head};
use super::theme_card_preview::{self as card_preview, CardPreview};
use crate::block::{Action, Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{PreviousAction, row_padding};
use crate::services::look::{self, AurHelper, CatalogFamily, InstalledTheme, PackageSource, ThemeKind};
use crate::services::previews::{self, Inbox, Previews};
use crate::store::Store;

/// En fazla sütun ve kartın en dar hâli (`space` birimi): 160 px'lik kartın kuyusuna 24 px'lik
/// üç önizleme öğesi sığar (720 px'lik sütunda dört kart yan yana).
const MAX_COLS: usize = 6;
const MIN_CARD_SPACE: u32 = 40;
/// Varyantın ortak adı çıkarılınca boş kalan (ailenin temel) varyantının etiketi.
fn base_variant_label() -> String {
    t!("settings-theme-grid-variant-base")
}

/// Karta tıklanınca ne olur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardValue {
    /// Önerilen: anahtar silinir.
    Recommended,
    /// Bu tema kimliği yazılır (kurulu ya da dosyada zaten yazılı). Aile kartında ailenin ilk
    /// varyantı.
    Theme(String),
    /// Kurulu olmayan katalog ailesi: hiçbir şey yazılmaz, kurulum komutu gösterilir.
    Install,
}

/// Aile kartındaki bir kurulu varyant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    /// Dizin adı (config'e yazılan).
    pub id: String,
    /// Seçicideki kısa ad ([`variant_labels`]).
    pub label: String,
}

/// Bir kartın verisi (saf; testli).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardSpec {
    pub value: CardValue,
    pub name: String,
    pub installed: bool,
    /// Kurulum komutu (yalnızca kurulu olmayan ve paketi katalogda bilinen kartlarda).
    pub install: Option<String>,
    /// Paket AUR'da (açıklamada belirtilir).
    pub aur: bool,
    /// Aile kartı: ailenin kurulu varyantları (katalog sırası, en az 2); tekil kartta boş.
    pub variants: Vec<Variant>,
}

impl CardSpec {
    fn new(value: CardValue, name: String, installed: bool) -> Self {
        Self { value, name, installed, install: None, aur: false, variants: Vec::new() }
    }

    #[cfg(test)]
    fn theme_id(&self) -> Option<&str> {
        match &self.value {
            CardValue::Theme(id) => Some(id),
            _ => None,
        }
    }

    fn has_variant(&self, id: &str) -> bool {
        self.variants.iter().any(|v| v.id == id)
    }

    /// Kart dosyadaki seçimi temsil ediyor mu (`current = None` = önerilen). Aile kartı,
    /// varyantlarından biri yazılıysa seçilidir.
    pub fn matches(&self, current: Option<&str>) -> bool {
        match (&self.value, current) {
            (CardValue::Install, _) => false,
            (CardValue::Recommended, None) => true,
            (CardValue::Theme(id), Some(c)) if id == c => true,
            (_, Some(c)) => self.has_variant(c),
            (CardValue::Theme(_), None) => false,
        }
    }

    /// Aile kartının etkin varyantı: dosyadaki seçim ailedeyse o, önerilen kartta öneri, yoksa
    /// ilki. Tekil kartta `None`.
    pub fn active_variant(&self, rec: Option<&str>, current: Option<&str>) -> Option<usize> {
        if self.variants.is_empty() {
            return None;
        }
        let pos = |id: Option<&str>| id.and_then(|id| self.variants.iter().position(|v| v.id == id));
        Some(match self.value {
            CardValue::Recommended => pos(current).or_else(|| pos(rec)).unwrap_or(0),
            _ => pos(current).unwrap_or(0),
        })
    }

    /// `i`. varyant seçilince yazılacak değer: önerilen tema → anahtar silinir (`Some(None)`),
    /// diğeri adıyla. Varyant yoksa `None`.
    pub fn variant_value(&self, i: usize, rec: Option<&str>) -> Option<Option<String>> {
        let v = self.variants.get(i)?;
        Some((Some(v.id.as_str()) != rec).then(|| v.id.clone()))
    }
}

/// Varyantların seçicideki kısa adları: görünen adların ortak baş ve son sözcükleri atılır
/// (`Vimix Cursors` / `Vimix Cursors - White` → `Standart` / `White`). Sonuç benzersiz değilse
/// görünen adlar olduğu gibi.
pub fn variant_labels(names: &[String]) -> Vec<String> {
    let toks: Vec<Vec<&str>> = names.iter().map(|n| n.split(|c: char| c == '-' || c == '_' || c.is_whitespace()).filter(|t| !t.is_empty()).collect()).collect();
    let min = toks.iter().map(Vec::len).min().unwrap_or(0);
    let same = |a: &str, b: &str| crate::registry::fold(a) == crate::registry::fold(b);
    let mut pre = 0;
    while pre < min && toks.iter().all(|t| same(t[pre], toks[0][pre])) {
        pre += 1;
    }
    let mut suf = 0;
    while pre + suf < min && toks.iter().all(|t| same(t[t.len() - 1 - suf], toks[0][toks[0].len() - 1 - suf])) {
        suf += 1;
    }
    let labels: Vec<String> = toks
        .iter()
        .map(|t| {
            let mid = t[pre..t.len() - suf].join(" ");
            let mut chars = mid.chars();
            match chars.next() {
                None => base_variant_label(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        })
        .collect();
    let unique = labels.iter().collect::<HashSet<_>>().len() == labels.len();
    if unique { labels } else { names.to_vec() }
}

/// Kartlar (bkz. modül belgesi): önerilen, katalogdaki kurulu aileler (aile başına tek kart),
/// kurulu diğerleri, dosyadaki kurulu olmayan seçim, kurulu olmayan aileler. `catalog = None`
/// (eski shell / shell yok) iken aile kartı yoktur. `helper`: AUR komutunun yardımcısı.
pub fn theme_cards(
    rec: Option<&str>,
    catalog: Option<&[CatalogFamily]>,
    installed: &[InstalledTheme],
    current: Option<&str>,
    helper: Option<AurHelper>,
) -> Vec<CardSpec> {
    let cat = catalog.unwrap_or_default();
    // index.theme adı kimliğin aynısıysa okunur hâli (`Bibata-Modern-Classic` → boşluklu).
    let name_of = |id: &str| match installed.iter().find(|t| t.id == id) {
        Some(t) if t.name != t.id => t.name.clone(),
        _ => pretty(id),
    };
    let variants_of = |ids: &[&String]| -> Vec<Variant> {
        let names: Vec<String> = ids.iter().map(|id| name_of(id)).collect();
        ids.iter().zip(variant_labels(&names)).map(|(id, label)| Variant { id: (*id).clone(), label }).collect()
    };
    let is_installed = |id: &str| installed.iter().any(|t| t.id == id) || cat.iter().any(|f| f.installed.iter().any(|i| i == id));
    // Önerilenin ailesi: varyantı kurulu olan aile, yoksa katalogun ilki (sözleşme: önerilen başta).
    let rec_family = rec.and_then(|r| cat.iter().find(|f| f.installed.iter().any(|i| i == r)).or(cat.first()));
    // Bir ailenin kurulu varyantları (tekrarsız, katalog sırası).
    let unique = |f: &'_ CatalogFamily| -> Vec<String> {
        let mut seen = HashSet::new();
        f.installed.iter().filter(|i| seen.insert(i.as_str())).cloned().collect()
    };
    let first = match rec {
        Some(r) if is_installed(r) => {
            let fam = cat.iter().find(|f| f.installed.iter().any(|i| i == r)).map(|f| (f, unique(f))).filter(|(_, ids)| ids.len() > 1);
            match fam {
                Some((f, ids)) => {
                    let mut c = CardSpec::new(CardValue::Recommended, f.name.clone(), true);
                    c.variants = variants_of(&ids.iter().collect::<Vec<_>>());
                    c
                }
                None => CardSpec::new(CardValue::Recommended, name_of(r), true),
            }
        }
        Some(r) => {
            let mut c = CardSpec::new(CardValue::Recommended, rec_family.map_or_else(|| pretty(r), |f| f.name.clone()), false);
            // Ailenin başka varyantı kuruluysa paket zaten kurulu: komut yardımcı olmaz.
            if let Some(f) = rec_family.filter(|f| f.installed.is_empty()) {
                c.install = Some(f.install_command(helper));
                c.aur = f.source == PackageSource::Aur;
            }
            c
        }
        // Shell'e ulaşılamadı: önerinin adı bilinmiyor, kart soluk değil.
        None => CardSpec::new(CardValue::Recommended, t!("settings-theme-grid-recommended"), true),
    };
    // Bir kartta yer alan kimlikler (önerilen dahil): ikinci kez kart olmaz.
    let mut covered: HashSet<String> = first.variants.iter().map(|v| v.id.clone()).chain(rec.map(str::to_owned)).collect();
    let mut out = vec![first];
    for f in cat {
        let ids: Vec<String> = unique(f).into_iter().filter(|i| !covered.contains(i)).collect();
        match ids.as_slice() {
            [] => {}
            [id] => out.push(CardSpec::new(CardValue::Theme(id.clone()), name_of(id), true)),
            [id, ..] => {
                let mut c = CardSpec::new(CardValue::Theme(id.clone()), f.name.clone(), true);
                c.variants = variants_of(&ids.iter().collect::<Vec<_>>());
                out.push(c);
            }
        }
        covered.extend(ids);
    }
    for t in installed {
        if covered.insert(t.id.clone()) {
            out.push(CardSpec::new(CardValue::Theme(t.id.clone()), name_of(&t.id), true));
        }
    }
    if let Some(c) = current
        && covered.insert(c.to_owned())
    {
        out.push(CardSpec::new(CardValue::Theme(c.to_owned()), name_of(c), false));
    }
    // Önerilen kart kurulu olmayan önerilen aileyi zaten temsil ediyor.
    let rec_card_family = rec_family.filter(|_| !out[0].installed).map(|f| f.family.as_str());
    for f in cat.iter().filter(|f| f.installed.is_empty() && Some(f.family.as_str()) != rec_card_family) {
        let mut c = CardSpec::new(CardValue::Install, f.name.clone(), false);
        c.install = Some(f.install_command(helper));
        c.aur = f.source == PackageSource::Aur;
        out.push(c);
    }
    out
}

/// Tema kimliğinin okunur hâli (`Bibata-Modern-Classic` → `Bibata Modern Classic`).
pub fn pretty(id: &str) -> String {
    id.replace(['-', '_'], " ")
}

/// Kartın önizlediği tema: aile kartında etkin varyant, kurulu tema kartında kendi kimliği,
/// kurulu önerilende önerinin adı. Kurulu olmayan kart, adı bilinmeyen öneri (shell yok) ve
/// kurulum kartı önizlemesizdir.
pub fn preview_id<'a>(spec: &'a CardSpec, rec: Option<&'a str>, current: Option<&str>) -> Option<&'a str> {
    if !spec.installed {
        return None;
    }
    if let Some(i) = spec.active_variant(rec, current) {
        return Some(&spec.variants[i].id);
    }
    match &spec.value {
        CardValue::Recommended => rec,
        CardValue::Theme(id) => Some(id),
        CardValue::Install => None,
    }
}

/// Kartın alt yazısı. Aile kartı: seçiliyken etkin varyantın adı, değilken "n varyant"
/// (önerilen kartta "Önerilen · " önekiyle, etkin varyant öneri değilse öneksiz).
pub fn caption(spec: &CardSpec, rec: Option<&str>, current: Option<&str>, rec_known: bool) -> String {
    let recommended = spec.value == CardValue::Recommended;
    if let Some(i) = spec.active_variant(rec, current) {
        let v = &spec.variants[i];
        return match (spec.matches(current), recommended) {
            (true, true) if Some(v.id.as_str()) == rec => t!("settings-theme-grid-caption-recommended-variant", name = v.label.as_str()),
            (true, _) => v.label.clone(),
            (false, true) => t!("settings-theme-grid-caption-recommended-variants", count = spec.variants.len()),
            (false, false) => t!("settings-theme-grid-caption-variants", count = spec.variants.len()),
        };
    }
    match (recommended, spec.installed, spec.aur) {
        (true, true, _) if rec_known => t!("settings-theme-grid-recommended"),
        (true, true, _) => t!("settings-theme-grid-caption-shell-picks"),
        (true, false, false) => t!("settings-theme-grid-caption-recommended-missing"),
        (true, false, true) => t!("settings-theme-grid-caption-recommended-missing-aur"),
        (false, false, true) => t!("settings-theme-grid-caption-missing-aur"),
        (false, false, false) => t!("settings-theme-grid-caption-missing"),
        (false, true, _) => String::new(),
    }
}

#[derive(Debug)]
struct Card {
    spec: CardSpec,
    name: Label,
    caption: Label,
    glyph: IconView,
    hover: Interaction,
    /// Kurulu temanın önizlemesi ([`preview_id`]); yoksa `glyph` yer tutucusu.
    preview: Option<CardPreview>,
    rect: Rect,
}

impl Card {
    /// `reuse`: yeniden kurulumdan önceki önizlemeler (aynı tema yeniden yüklenmez, belirmez).
    fn with_preview(theme: &Theme, kind: ThemeKind, spec: CardSpec, rec: Option<&str>, current: Option<&str>, reuse: &mut Vec<CardPreview>) -> Self {
        let id = preview_id(&spec, rec, current).map(str::to_owned);
        let cap = caption(&spec, rec, current, rec.is_some());
        let mut card = Self::new(theme, kind, spec, &cap);
        card.preview = id.map(|id| match reuse.iter().position(|p| p.id() == id) {
            Some(i) => reuse.swap_remove(i),
            None => CardPreview::new(theme, kind, &id),
        });
        card
    }

    fn new(theme: &Theme, kind: ThemeKind, spec: CardSpec, caption: &str) -> Self {
        // Kurulu olmayan aile seçilemez, kurulabilir: indirme ikonu.
        let glyph = match (&spec.value, kind) {
            (CardValue::Install, _) => IconName::Download,
            (_, ThemeKind::Icons) => IconName::Apps,
            (_, ThemeKind::Cursors) => IconName::Mouse,
        };
        Self {
            name: Label::new(spec.name.clone(), text_style(theme, TextRole::Label).with_max_lines(1)),
            caption: Label::new(caption, text_style(theme, TextRole::Caption).with_max_lines(1)),
            glyph: IconView::with_role(theme, Icon::named(glyph), TextRole::IconSmall),
            hover: Interaction::new(theme),
            preview: None,
            rect: Rect::ZERO,
            spec,
        }
    }
}

/// Kartların kurulduğu girdi (`ThemeGrid::rebuild`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct BuildKey {
    installed: Vec<InstalledTheme>,
    catalog: Option<Vec<CatalogFamily>>,
    /// Dosyadaki seçim (`None` = önerilen).
    current: Option<String>,
    rec: Option<String>,
}

/// Izgaranın klavye bölümü: kartlar, varyant seçici ya da kurulum komutu düğmesi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Cards,
    Variant,
    Copy,
}

/// Seçili aile kartının varyant seçicisi (ızgaranın altında).
#[derive(Debug)]
struct VariantPicker {
    /// Seçicinin ait olduğu kartın varyant kimlikleri (aynıysa yeniden kurulmaz).
    ids: Vec<String>,
    label: Label,
    /// 2–5 varyantta bölümlü düğme; sığmazsa [`Self::select`] kullanılır.
    seg: Option<SegmentedButton>,
    select: Box<Select>,
    /// Son yerleşimde bölümlü düğme mi.
    use_seg: bool,
    /// Etiketin altına yığılmış mı.
    stacked: bool,
    rect: Rect,
}

impl VariantPicker {
    fn new(theme: &Theme, spec: &CardSpec, active: usize) -> Self {
        let labels: Vec<String> = spec.variants.iter().map(|v| v.label.clone()).collect();
        let seg = (MIN_SEGMENTS..=MAX_SEGMENTS)
            .contains(&labels.len())
            .then(|| SegmentedButton::new(theme, labels.iter().map(|l| Segment::new(l.clone())).collect(), active));
        Self {
            ids: spec.variants.iter().map(|v| v.id.clone()).collect(),
            label: Label::new(t!("settings-theme-grid-variant-picker-label", name = spec.name.as_str()), text_style(theme, TextRole::Body)),
            seg,
            select: Box::new(Select::new(theme, labels, active)),
            use_seg: false,
            stacked: false,
            rect: Rect::ZERO,
        }
    }

    fn set_selected(&mut self, i: usize, now: Instant) {
        if let Some(s) = &mut self.seg {
            s.set_selected(i, now);
        }
        self.select.set_selected(i);
    }

    /// Yerleşim kararı (`width`: satırın iç genişliği): bölümlü düğme etiketin yanına, olmazsa
    /// altına sığıyorsa o; yoksa açılır seçim (yanına sığmazsa altına). Yükseklik döner.
    fn layout(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let gap = theme.space(4);
        self.label.set_max_width(None);
        let lw = self.label.measure(text).width;
        let seg_w = self.seg.as_mut().map(|s| s.preferred_size(text, theme).width);
        let select_w = theme.space(64).min(width);
        (self.use_seg, self.stacked) = match seg_w {
            Some(w) if lw + gap + w <= width => (true, false),
            Some(w) if w <= width => (true, true),
            _ => (false, lw + gap + theme.space(40).min(select_w) > width),
        };
        self.label.set_max_width(Some(width));
        let lh = self.label.measure(text).height;
        let ch = if self.use_seg { theme.space(10) } else { self.select.preferred_size(theme, select_w).height };
        if self.stacked { lh + theme.space(2) + ch } else { lh.max(ch) }
    }

    fn place(&mut self, text: &mut TextSystem, theme: &Theme, rect: Rect) {
        self.rect = rect;
        let gap = theme.space(4);
        let lw = self.label.measure(text).width;
        let lh = self.label.measure(text).height;
        let ctl = if self.use_seg {
            let w = self.seg.as_mut().map_or(0.0, |s| s.preferred_size(text, theme).width);
            let w = if self.stacked { w.min(rect.width) } else { w };
            (w, theme.space(10))
        } else {
            let w = if self.stacked { rect.width } else { theme.space(64).min(rect.width - lw - gap) };
            (w, self.select.preferred_size(theme, w).height)
        };
        let r = if self.stacked {
            Rect::new(rect.x, rect.y + lh + theme.space(2), ctl.0, ctl.1)
        } else {
            Rect::new(rect.right() - ctl.0, rect.center().y - ctl.1 * 0.5, ctl.0, ctl.1)
        };
        match (&mut self.seg, self.use_seg) {
            (Some(s), true) => s.set_rect(theme, r),
            _ => self.select.set_rect(theme, r),
        }
    }

    fn set_focus_visible(&mut self, on: bool) {
        if let Some(s) = &mut self.seg {
            s.set_focus_visible(on && self.use_seg);
        }
        self.select.set_focus_visible(on && !self.use_seg);
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.label.set_style(text_style(theme, TextRole::Body));
        if let Some(s) = &mut self.seg {
            s.set_theme(theme, now);
        }
        self.select.set_theme(theme, now);
    }

    fn is_open(&self) -> bool {
        !self.use_seg && self.select.is_open()
    }

    /// İşaretçi: `(yeniden çiz, seçilen varyant)`.
    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> (bool, Option<usize>) {
        match (&mut self.seg, self.use_seg) {
            (Some(s), true) => {
                let r = s.pointer(e, env.now);
                (r.repaint || r.event.is_some(), r.event)
            }
            _ => {
                let r = self.select.pointer(env.text, env.theme, e, env.now);
                let chosen = match r.event {
                    Some(SelectEvent::Changed(i)) => Some(i),
                    _ => None,
                };
                (r.repaint || chosen.is_some(), chosen)
            }
        }
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> (bool, Option<usize>) {
        match (&mut self.seg, self.use_seg) {
            (Some(s), true) => {
                let r = s.key(e, env.now);
                (r.repaint || r.event.is_some(), r.event)
            }
            _ => {
                let was_open = self.select.is_open();
                let r = self.select.key(env.text, env.theme, e, env.now);
                match r.event {
                    Some(SelectEvent::Changed(i)) => (true, Some(i)),
                    // Açıkken her tuş menünündür (Escape kapatır).
                    _ => (was_open || r.repaint, None),
                }
            }
        }
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        match (&self.seg, self.use_seg) {
            (Some(s), true) => s.cursor(p),
            _ => self.select.cursor(p),
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let lh = self.label.measure_in(ui.painter).height;
        let ly = if self.stacked { self.rect.y } else { self.rect.center().y - lh * 0.5 };
        self.label.draw(ui.painter, Point::new(self.rect.x, ly), ui.fade(t.colors.on_surface));
        match (&mut self.seg, self.use_seg) {
            (Some(s), true) => s.draw(ui),
            _ => {
                self.select.draw(ui);
                // Menü kaplamada: sayfanın ve satırın kırpmasından bağımsız, yüzeye sığdırılır.
                self.select.set_bounds(ui.surface_rect());
                self.select.draw_overlay(ui);
            }
        }
    }
}

/// İkon ya da imleç teması kart ızgarası (bkz. modül belgesi).
#[derive(Debug)]
pub struct ThemeGrid {
    desc: &'static SettingDesc,
    kind: ThemeKind,
    head: Head,
    cards: Vec<Card>,
    /// Kartların kurulduğu girdi; değişmediyse yeniden kurulmaz.
    built_for: Option<BuildKey>,
    /// Kurulum komutu gösterilen kurulu olmayan aile kartı (config'e yazılmaz).
    shown_family: Option<usize>,
    /// Seçili aile kartının varyant seçicisi.
    picker: Option<VariantPicker>,
    /// Tarama sürerken / hiç tema yokken başlığın altında.
    note: Label,
    footer: Label,
    copy: Button,
    command: Option<String>,
    previous: PreviousAction,
    /// AUR komutlarının yardımcısı (oturumda bir kez aranır).
    helper: Option<AurHelper>,
    cols: usize,
    /// Klavyede odaktaki kart.
    focus_i: usize,
    part: Part,
    focused: bool,
    focus_visible: bool,
    rect: Rect,
    /// Son yerleşimde ızgaranın alt kenarı ve alt notun üstü.
    grid_bottom: f32,
    footer_top: f32,
    theme: Theme,
    /// Paylaşılan önizleme önbelleği + çözücü ve bu ızgaranın teslim kutusu.
    previews: Arc<Previews>,
    inbox: Inbox,
    /// Önizlemelerin yüklendiği ölçek (değişince yeniden istenir).
    scale: f32,
    /// Görülen kurulu tema kuşağı (`look::themes_generation`); değişince yuvalar yeniden istenir.
    themes_gen: u64,
}

impl ThemeGrid {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, kind: ThemeKind, store: &Store) -> Self {
        Self::with_previews(theme, desc, kind, store, Previews::shared())
    }

    fn with_previews(theme: &Theme, desc: &'static SettingDesc, kind: ThemeKind, store: &Store, previews: Arc<Previews>) -> Self {
        let mut s = Self {
            desc,
            kind,
            head: Head::new(theme, desc),
            cards: Vec::new(),
            built_for: None,
            shown_family: None,
            picker: None,
            note: Label::new("", text_style(theme, TextRole::Caption)),
            footer: Label::new("", text_style(theme, TextRole::Caption)),
            copy: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Copy)).with_label(theme, t!("settings-theme-grid-button-copy-command")),
            command: None,
            previous: PreviousAction::new(theme, desc),
            helper: look::aur_helper(),
            cols: 1,
            focus_i: 0,
            part: Part::Cards,
            focused: false,
            focus_visible: false,
            rect: Rect::ZERO,
            grid_bottom: 0.0,
            footer_top: 0.0,
            theme: theme.clone(),
            previews,
            inbox: previews::inbox(),
            scale: 1.0,
            themes_gen: look::themes_generation(),
        };
        s.rebuild(store);
        s
    }

    /// Dosyadaki seçim (`None` / boş / önerilenin adı = önerilen; shell için de aynıdır).
    fn current(&self, store: &Store, rec: Option<&str>) -> Option<String> {
        Some(self.desc.string(store)).filter(|s| !s.is_empty() && Some(s.as_str()) != rec)
    }

    fn rec(&self) -> Option<&str> {
        self.built_for.as_ref().and_then(|k| k.rec.as_deref())
    }

    fn built_current(&self) -> Option<&str> {
        self.built_for.as_ref().and_then(|k| k.current.as_deref())
    }

    fn selected(&self) -> Option<usize> {
        let k = self.built_for.as_ref()?;
        self.cards.iter().position(|c| c.spec.matches(k.current.as_deref()))
    }

    /// Kartları kurulu listeye, kataloğa, seçime ve öneriye göre kurar (değişmediyse dokunmaz).
    fn rebuild(&mut self, store: &Store) {
        let (rec, catalog) = look::offer(self.kind);
        let installed = look::themes().map(|t| t.of(self.kind).to_vec());
        let scanning = installed.is_none();
        let installed = installed.unwrap_or_default();
        let current = self.current(store, rec.as_deref());
        let key = BuildKey { installed, catalog, current, rec };
        if self.built_for.as_ref() != Some(&key) {
            let none_installed = key.installed.is_empty() && key.catalog.iter().flatten().all(|f| f.installed.is_empty());
            let note = match (scanning, none_installed) {
                (true, _) => t!("settings-look-themes-loading"),
                (false, true) => t!("settings-look-themes-empty"),
                _ => String::new(),
            };
            self.note.set_text(&note);
            self.build(key);
        }
        self.update_footer();
    }

    /// Kartları `key`'den kurar; aynı temanın önizlemesi yeni karta taşınır.
    fn build(&mut self, key: BuildKey) {
        let specs = theme_cards(key.rec.as_deref(), key.catalog.as_deref(), &key.installed, key.current.as_deref(), self.helper);
        let mut reuse: Vec<CardPreview> = self.cards.iter_mut().filter_map(|c| c.preview.take()).collect();
        self.cards = specs
            .into_iter()
            .map(|s| Card::with_preview(&self.theme, self.kind, s, key.rec.as_deref(), key.current.as_deref(), &mut reuse))
            .collect();
        self.built_for = Some(key);
        self.shown_family = None;
        self.focus_i = self.selected().unwrap_or(0);
        self.sync_picker();
    }

    /// Seçili kart bir aileyse seçicisini kurar (aynı aileyse yalnızca seçimi günceller).
    fn sync_picker(&mut self) {
        let (rec, current) = (self.rec().map(str::to_owned), self.built_current().map(str::to_owned));
        let card = self.selected().and_then(|i| self.cards.get(i)).filter(|c| !c.spec.variants.is_empty());
        let Some(card) = card else {
            self.picker = None;
            if self.part == Part::Variant {
                self.part = Part::Cards;
            }
            return;
        };
        let active = card.spec.active_variant(rec.as_deref(), current.as_deref()).unwrap_or(0);
        let ids: Vec<String> = card.spec.variants.iter().map(|v| v.id.clone()).collect();
        match &mut self.picker {
            Some(p) if p.ids == ids => p.set_selected(active, Instant::now()),
            _ => self.picker = Some(VariantPicker::new(&self.theme, &card.spec, active)),
        }
    }

    /// Izgara altı: gösterilen aile kartının kurulum komutu, yoksa seçili tema kurulu değilse not
    /// (paketi katalogda biliniyorsa komutla). AUR komutunda yardımcı yoksa kısa not eklenir.
    fn update_footer(&mut self) {
        let family = self.shown_family.and_then(|i| self.cards.get(i)).filter(|c| c.spec.value == CardValue::Install);
        let missing = || self.selected().and_then(|i| self.cards.get(i)).filter(|c| !c.spec.installed);
        let (mut text, command, aur) = match (family, family.is_none().then(missing).flatten()) {
            (Some(c), _) => {
                let cmd = c.spec.install.clone();
                let text = match &cmd {
                    Some(cmd) => t!("settings-theme-grid-footer-missing-command", name = c.spec.name.as_str(), command = cmd.as_str()),
                    None => t!("settings-theme-grid-footer-missing", name = c.spec.name.as_str()),
                };
                (text, cmd, c.spec.aur)
            }
            (None, Some(c)) => {
                let cmd = c.spec.install.clone();
                let text = match &cmd {
                    Some(cmd) => t!("settings-theme-grid-footer-fallback-command", name = c.spec.name.as_str(), command = cmd.as_str()),
                    None => t!("settings-theme-grid-footer-fallback", name = c.spec.name.as_str()),
                };
                (text, cmd, c.spec.aur)
            }
            (None, None) => (String::new(), None, false),
        };
        if command.is_some() && look::needs_helper_note(aur, self.helper) {
            text = format!("{text} — {}", look::aur_helper_note());
        }
        self.footer.set_text(&text);
        self.command = command;
        if self.command.is_none() && self.part == Part::Copy {
            self.part = Part::Cards;
        }
    }

    /// Yazım: anahtar `value` (`None` = önerilen, anahtar silinir); dosyadakiyle aynıysa yazılmaz.
    fn write(&mut self, env: &mut Env, value: Option<String>) {
        let rec = self.rec().map(str::to_owned);
        if value == self.current(env.store, rec.as_deref()) {
            return;
        }
        let d = self.desc;
        env.set(d.label(), d.file, d.key(), value.map(Value::from));
        look::refresh(APPLY_SETTLE);
    }

    /// Kart eylemi: tema/önerilen yazılır; kurulu olmayan aile yalnızca komutunu gösterir. Zaten
    /// seçili aile kartı yazmaz (seçili varyant korunur).
    fn choose(&mut self, env: &mut Env, i: usize) {
        let Some(spec) = self.cards.get(i).map(|c| c.spec.clone()) else { return };
        if spec.value == CardValue::Install {
            self.shown_family = Some(i);
            self.update_footer();
            return;
        }
        if self.shown_family.take().is_some() {
            self.update_footer();
        }
        if !spec.variants.is_empty() && spec.matches(self.built_current()) {
            return;
        }
        let value = match spec.value {
            CardValue::Theme(id) => Some(id),
            _ => None,
        };
        self.write(env, value);
    }

    /// Seçicide `i`. varyant seçildi: seçili aile kartının o varyantı yazılır.
    fn choose_variant(&mut self, env: &mut Env, i: usize) {
        let rec = self.rec().map(str::to_owned);
        let Some(value) = self.selected().and_then(|s| self.cards.get(s)).and_then(|c| c.spec.variant_value(i, rec.as_deref())) else { return };
        self.write(env, value);
    }

    fn cols_for(theme: &Theme, width: f32) -> usize {
        let gap = theme.space(3);
        (((width + gap) / (theme.space(MIN_CARD_SPACE) + gap)).floor() as usize).clamp(1, MAX_COLS)
    }

    /// Önizleme yuvasının yüksekliği (her kartta aynı: önizleme şeridinin yüksekliği).
    fn slot_h(theme: &Theme) -> f32 {
        CardPreview::height(theme)
    }

    fn slot_rect(theme: &Theme, card: Rect) -> Rect {
        let pad = theme.space(2);
        Rect::new(card.x + pad, card.y + pad, card.width - pad * 2.0, Self::slot_h(theme))
    }

    fn card_h(&mut self, text: &mut TextSystem, theme: &Theme) -> f32 {
        let Some(c) = self.cards.first_mut() else { return 0.0 };
        let name = c.name.measure(text).height;
        let cap = c.caption.measure(text).height.max(name * 0.8);
        theme.space(2) * 2.0 + Self::slot_h(theme) + theme.space(2) + name + cap
    }

    /// Teslim edilen önizlemeleri yükler; ölçek ya da kurulu tema kuşağı değiştiyse önce tüm
    /// yuvaları sıfırlar.
    fn apply_previews(&mut self, painter: &mut nuxut::render::Painter, theme: &Theme, now: Instant) {
        let scale = painter.scale().max(1.0);
        let generation = look::themes_generation();
        if scale != self.scale || generation != self.themes_gen {
            self.scale = scale;
            self.themes_gen = generation;
            for p in self.cards.iter_mut().filter_map(|c| c.preview.as_mut()) {
                p.reset(theme);
            }
        }
        for (key, entry) in previews::drain(&self.inbox) {
            let mut upload = |img: &nuxut_image::PreviewImage| card_preview::upload(painter, img);
            for p in self.cards.iter_mut().filter_map(|c| c.preview.as_mut()) {
                p.deliver(&key, &entry, scale, now, &mut upload);
            }
        }
    }

    /// Çizimden sonra: yeni görünen yuvaları önbellekten ya da çözücüden ister. Önbellekten
    /// gelen varsa `true` (bir sonraki karede yüklenir).
    fn request_previews(&mut self) -> bool {
        let mut cached = false;
        for p in self.cards.iter_mut().filter_map(|c| c.preview.as_mut()) {
            for (key, request) in p.take_jobs(self.scale) {
                cached |= self.previews.request(key, request, &self.inbox);
            }
        }
        cached
    }

    fn text_width(&self, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let prev = if self.previous.visible() { self.previous.size(theme).width + theme.space(2) } else { 0.0 };
        (width - px * 2.0 - prev).max(theme.space(20))
    }

    fn head_height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let w = self.text_width(theme, width);
        let mut h = self.head.measure(text, theme, w);
        if !self.note.text().is_empty() {
            self.note.set_max_width(Some(w));
            h += theme.space(1) + self.note.measure(text).height;
        }
        h
    }

    fn picker_height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        self.picker.as_mut().map_or(0.0, |p| theme.space(4) + p.layout(text, theme, width - px * 2.0))
    }

    fn footer_height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        if self.footer.text().is_empty() {
            return 0.0;
        }
        let (px, _) = row_padding(theme);
        let bw = if self.command.is_some() { self.copy.preferred_size(text, theme).width + theme.space(3) } else { 0.0 };
        self.footer.set_max_width(Some((width - px * 2.0 - bw).max(theme.space(20))));
        let bh = if self.command.is_some() { self.copy.preferred_size(text, theme).height } else { 0.0 };
        theme.space(3) + self.footer.measure(text).height.max(bh)
    }

    fn grid_height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        let w = width - px * 2.0;
        let cols = Self::cols_for(theme, w);
        let rows = self.cards.len().div_ceil(cols) as f32;
        let gap = theme.space(3);
        rows * (self.card_h(text, theme) + gap) - gap
    }

    fn move_focus(&mut self, key: Key) -> bool {
        let n = self.cards.len();
        if n == 0 {
            return false;
        }
        let i = self.focus_i.min(n - 1);
        let cols = self.cols.max(1);
        self.focus_i = match key {
            Key::Left => i.saturating_sub(1),
            Key::Right => (i + 1).min(n - 1),
            Key::Up => i.saturating_sub(cols),
            Key::Down => (i + cols).min(n - 1),
            Key::Home => 0,
            Key::End => n - 1,
            _ => return false,
        };
        true
    }

    /// Klavye bölümleri sırasıyla (kartlar, varsa seçici, varsa komut düğmesi).
    fn parts(&self) -> Vec<Part> {
        let mut v = vec![Part::Cards];
        if self.picker.is_some() {
            v.push(Part::Variant);
        }
        if self.command.is_some() {
            v.push(Part::Copy);
        }
        v
    }

    fn sync_focus_rings(&mut self) {
        let on = self.focus_visible;
        self.copy.set_focus_visible(on && self.part == Part::Copy);
        if let Some(p) = &mut self.picker {
            p.set_focus_visible(on && self.part == Part::Variant);
        }
    }
}

impl Block for ThemeGrid {
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
        let (_, py) = row_padding(t);
        let head = self.head_height(env.text, t, width);
        let grid = self.grid_height(env.text, t, width);
        let picker = self.picker_height(env.text, t, width);
        let footer = self.footer_height(env.text, t, width);
        py * 2.0 + head + t.space(3) + grid + picker + footer
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, py) = row_padding(t);
        let head = self.head_height(env.text, t, rect.width);
        let ps = self.previous.size(t);
        let th = self.head.title.measure(env.text).height;
        self.previous.set_rect(Rect::new(rect.right() - px - ps.width, rect.y + py + th * 0.5 - ps.height * 0.5, ps.width, ps.height));
        let w = rect.width - px * 2.0;
        let cols = Self::cols_for(t, w);
        self.cols = cols;
        let gap = t.space(3);
        let cw = (w - gap * (cols as f32 - 1.0)) / cols as f32;
        let ch = self.card_h(env.text, t);
        let top = rect.y + py + head + t.space(3);
        for (i, card) in self.cards.iter_mut().enumerate() {
            let (r, c) = (i / cols, i % cols);
            card.rect = Rect::new(rect.x + px + c as f32 * (cw + gap), top + r as f32 * (ch + gap), cw, ch);
            card.name.set_max_width(Some(cw - t.space(4)));
            card.caption.set_max_width(Some(cw - t.space(4)));
            let slot = Self::slot_rect(t, card.rect);
            if let Some(p) = &mut card.preview {
                p.set_rect(slot);
            }
        }
        let rows = self.cards.len().div_ceil(cols) as f32;
        self.grid_bottom = top + rows * (ch + gap) - gap;
        let picker_h = self.picker_height(env.text, t, rect.width);
        if let Some(p) = &mut self.picker {
            let y = self.grid_bottom + t.space(4);
            p.place(env.text, t, Rect::new(rect.x + px, y, w, picker_h - t.space(4)));
        }
        self.footer_top = self.grid_bottom + picker_h;
        self.footer_height(env.text, t, rect.width);
        if self.command.is_some() {
            let bs = self.copy.preferred_size(env.text, t);
            let fh = self.footer.measure(env.text).height.max(bs.height);
            let cy = self.footer_top + t.space(3) + fh * 0.5;
            self.copy.set_rect(Rect::new(rect.right() - px - bs.width + t.space(2), cy - bs.height * 0.5, bs.width, bs.height));
        }
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let now = env.now;
        // Açık açılır seçim olayın tek sahibi (dışına tıklama yalnızca kapatır).
        if let Some(p) = self.picker.as_mut().filter(|p| p.is_open()) {
            let (_, chosen) = p.pointer(env, e);
            if let Some(i) = chosen {
                self.choose_variant(env, i);
            }
            return true;
        }
        let (mut repaint, fired) = self.previous.pointer(env, e);
        if fired {
            return true;
        }
        if let Some(p) = &mut self.picker {
            let (r, chosen) = p.pointer(env, e);
            repaint |= r;
            let opened = p.is_open();
            if chosen.is_some() || opened {
                self.part = Part::Variant;
                env.push(Action::Focus);
            }
            if let Some(i) = chosen {
                self.choose_variant(env, i);
                return true;
            }
            if opened {
                return true;
            }
        }
        let mut chosen = None;
        for (i, card) in self.cards.iter_mut().enumerate() {
            let tap = card.hover.handle(card.rect, e, now);
            repaint |= tap.repaint;
            if e.kind == PointerKind::Leave {
                repaint |= card.hover.reset(now);
            }
            if tap.clicked {
                chosen = Some(i);
            }
            // Animasyonlu imleç kartın her yerinde oynar (bileşen yalnızca kendi şeridine bakar).
            if let Some(p) = &mut card.preview {
                let over = card.rect.contains(e.position) && e.kind != PointerKind::Leave;
                let at = if over { PointerEvent::new(Self::slot_rect(&self.theme, card.rect).center(), PointerKind::Motion) } else { PointerEvent::new(Point::new(f32::MIN, f32::MIN), PointerKind::Leave) };
                repaint |= p.pointer(&at, now);
            }
        }
        if let Some(i) = chosen {
            self.focus_i = i;
            self.part = Part::Cards;
            env.push(Action::Focus);
            self.choose(env, i);
            return true;
        }
        if let Some(cmd) = self.command.clone() {
            let r = self.copy.pointer(e, now);
            repaint |= r.repaint;
            if r.event.is_some() {
                env.push(Action::Copy(cmd));
                return true;
            }
        }
        repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        match self.part {
            Part::Copy => {
                let Some(cmd) = self.command.clone() else { return false };
                let r = self.copy.key(e, env.now);
                if r.event.is_some() {
                    env.push(Action::Copy(cmd));
                    return true;
                }
                return r.repaint;
            }
            Part::Variant => {
                let Some(p) = &mut self.picker else { return false };
                let (used, chosen) = p.key(env, e);
                if let Some(i) = chosen {
                    self.choose_variant(env, i);
                }
                return used;
            }
            Part::Cards => {}
        }
        if e.state == KeyState::Released {
            return false;
        }
        match e.key {
            Key::Enter | Key::Space => {
                let i = self.focus_i;
                self.focus_visible = self.focused;
                self.choose(env, i);
                true
            }
            k => {
                let moved = self.move_focus(k);
                if moved {
                    self.focus_visible = self.focused;
                }
                moved
            }
        }
    }

    fn captures_keys(&self) -> bool {
        self.picker.as_ref().is_some_and(VariantPicker::is_open)
    }

    fn overlay_open(&self) -> bool {
        self.picker.as_ref().is_some_and(VariantPicker::is_open)
    }

    fn focusable(&self) -> bool {
        !self.cards.is_empty()
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        if focused && !self.focused {
            self.focus_i = self.selected().unwrap_or(0);
            self.part = Part::Cards;
        }
        self.focused = focused;
        self.focus_visible = focused && visible;
        self.sync_focus_rings();
    }

    fn focus_step(&mut self, forward: bool, _now: Instant) -> bool {
        let parts = self.parts();
        let i = parts.iter().position(|p| *p == self.part).unwrap_or(0);
        let next = if forward { parts.get(i + 1) } else { i.checked_sub(1).and_then(|j| parts.get(j)) };
        let Some(next) = next.copied() else { return false };
        self.part = next;
        self.sync_focus_rings();
        true
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if let Some(c) = self.previous.cursor(p) {
            return Some(c);
        }
        if let Some(c) = self.picker.as_ref().and_then(|pk| pk.cursor(p)) {
            return Some(c);
        }
        if self.command.is_some()
            && let Some(c) = self.copy.cursor(p)
        {
            return Some(c);
        }
        self.cards.iter().any(|c| c.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.rebuild(env.store);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        match event {
            crate::services::Event::Snapshots => self.previous.refresh(),
            crate::services::Event::Look => {
                self.rebuild(env.store);
                true
            }
            _ => false,
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.theme = theme.clone();
        self.head.set_theme(theme);
        self.note.set_style(text_style(theme, TextRole::Caption));
        self.footer.set_style(text_style(theme, TextRole::Caption));
        self.copy.set_theme(theme, now);
        self.previous.set_theme(theme, now);
        if let Some(p) = &mut self.picker {
            p.set_theme(theme, now);
        }
        for c in &mut self.cards {
            c.name.set_style(text_style(theme, TextRole::Label).with_max_lines(1));
            c.caption.set_style(text_style(theme, TextRole::Caption).with_max_lines(1));
            c.glyph.set_theme(theme, now);
            c.hover.set_theme(theme, now);
            if let Some(p) = &mut c.preview {
                p.set_theme(theme, now);
            }
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let x = self.rect.x + px;
        let h = self.head.draw(ui, x, self.rect.y + py);
        if !self.note.text().is_empty() {
            self.note.draw(ui.painter, Point::new(x, self.rect.y + py + h + t.space(1)), ui.fade(c.on_surface_variant));
        }
        self.previous.draw(ui);
        self.apply_previews(ui.painter, t, now);
        // Yalnızca yüzeyde görünen kartın önizlemesi çizilir (dolayısıyla istenir): kaydırılıp
        // görünmeyen kartlar hiçbir şey yüklemez.
        let surface_h = ui.painter.size().height;
        let selected = self.selected();
        let ring_on = self.focus_visible && self.part == Part::Cards;
        for (i, card) in self.cards.iter_mut().enumerate() {
            let r = card.rect;
            let is_sel = Some(i) == selected;
            // Kurulu olmayan kart soluk (bkz. modül belgesi).
            let dim = if card.spec.installed { 1.0 } else { 0.5 };
            let tone = |ui: &Ui, col: Color| ui.fade(col.multiply_alpha(dim));
            // Soluk metin/işaret yüksek karşıtlıkta eşikte kalır (framework yardımcıları).
            let text_tone = |ui: &Ui, col: Color| ui.fade(faded_text(t, col, dim, &[c.surface_bright]));
            // Komutu gösterilen aile kartı: seçim değil (primary değil), tam opak kenar.
            let border = if is_sel {
                Border { width: 2.0, color: ui.fade(c.primary) }
            } else if Some(i) == self.shown_family {
                Border { width: 2.0, color: ui.fade(c.outline) }
            } else {
                Border { width: HAIRLINE, color: ui.fade(hairline(t, c.outline, dim, &[])) }
            };
            ui.painter.rect(r, &RectStyle { fill: ui.fade(c.surface_bright), radii: t.shape.medium(), border: Some(border) });
            card.hover.draw_layer(ui.painter, r, t.shape.medium(), t, now);
            if card.hover.is_animating(now) {
                ui.request_frame();
            }
            let pad = t.space(2);
            let slot = Self::slot_rect(t, r);
            match &mut card.preview {
                Some(p) if slot.bottom() >= 0.0 && slot.y <= surface_h => p.draw(ui),
                Some(_) => {}
                None => {
                    // Önizlemesiz kart: bileşenin kuyusuyla aynı ton ve yarıçapta yer tutucu.
                    let radius = (t.shape.radius_medium - t.space(1)).max(t.shape.radius_small);
                    ui.painter.rect(slot, &RectStyle::filled(tone(ui, c.surface), Corners::uniform(radius)));
                    let gc = ui.fade(faded_mark(t, c.on_surface_variant, 0.6 * dim, &[c.surface_bright, c.surface.multiply_alpha(dim)]));
                    card.glyph.draw_centered(ui.painter, slot.center(), gc);
                }
            }
            let ty = slot.bottom() + t.space(2);
            let nh = card.name.measure_in(ui.painter).height;
            let nc = text_tone(ui, if is_sel { c.primary } else { c.on_surface });
            card.name.draw(ui.painter, Point::new(r.x + pad, ty), nc);
            if !card.caption.text().is_empty() {
                let cc = text_tone(ui, c.on_surface_variant);
                card.caption.draw(ui.painter, Point::new(r.x + pad, ty + nh), cc);
            }
            if ring_on && i == self.focus_i {
                let ring = r.expand(FOCUS_RING);
                let m = t.shape.medium();
                let radii = Corners::new(m.top_left + FOCUS_RING, m.top_right + FOCUS_RING, m.bottom_right + FOCUS_RING, m.bottom_left + FOCUS_RING);
                ui.painter.rect(ring, &RectStyle { fill: Color::TRANSPARENT, radii, border: Some(Border { width: FOCUS_RING, color: ui.fade(focus_ring(t, &[c.surface_bright])) }) });
            }
        }
        if self.request_previews() {
            ui.request_frame();
        }
        if !self.footer.text().is_empty() {
            let fh = self.footer.measure_in(ui.painter).height;
            let line_h = if self.command.is_some() { self.copy.rect().height.max(fh) } else { fh };
            let cy = self.footer_top + t.space(3) + line_h * 0.5;
            self.footer.draw(ui.painter, Point::new(x, cy - fh * 0.5), ui.fade(c.on_surface_variant));
            if self.command.is_some() {
                self.copy.draw(ui);
            }
        }
        // Seçici en son: açık menüsü kartların ve notun üstünde.
        if let Some(p) = &mut self.picker {
            p.draw(ui);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use nuxut::render::Recorder;
    use nuxut::ui::PreviewSlot;
    use nuxut_image::{CursorRole, PreviewImage, PreviewKey};

    fn t(id: &str) -> InstalledTheme {
        InstalledTheme { id: id.into(), name: pretty(id) }
    }

    fn fam(family: &str, name: &str, package: &str, source: PackageSource, installed: &[&str]) -> CatalogFamily {
        CatalogFamily { family: family.into(), name: name.into(), package: package.into(), source, installed: installed.iter().map(|s| s.to_string()).collect() }
    }

    /// Shell sürüm 3 imleç kataloğu (kısaltılmış; önerilen aile başta).
    fn cursor_catalog() -> Vec<CatalogFamily> {
        vec![
            fam("vimix", "Vimix", "vimix-cursors", PackageSource::Repo, &[]),
            fam("capitaine", "Capitaine", "capitaine-cursors", PackageSource::Repo, &["capitaine-cursors", "capitaine-cursors-light"]),
            fam("oreo", "Oreo", "oreo-cursors-bin", PackageSource::Aur, &[]),
            fam("sweet", "Sweet", "sweet-cursors-git", PackageSource::Aur, &[]),
        ]
    }

    fn theme(id: &str) -> CardValue {
        CardValue::Theme(id.into())
    }

    /// Testlerde AUR yardımcısı paru (makinedeki kurulumdan bağımsız).
    fn tc(rec: Option<&str>, cat: Option<&[CatalogFamily]>, installed: &[InstalledTheme], current: Option<&str>) -> Vec<CardSpec> {
        theme_cards(rec, cat, installed, current, Some(AurHelper::Paru))
    }

    fn ids(c: &CardSpec) -> Vec<&str> {
        c.variants.iter().map(|v| v.id.as_str()).collect()
    }

    #[test]
    fn catalog_families_then_others_then_installable_families() {
        let installed = [t("Adwaita"), t("capitaine-cursors"), t("capitaine-cursors-light"), t("Bibata-Modern-Classic")];
        let cat = cursor_catalog();
        let cards = tc(Some("Vimix-cursors"), Some(&cat), &installed, None);
        let values: Vec<&CardValue> = cards.iter().map(|c| &c.value).collect();
        assert_eq!(
            values,
            [&CardValue::Recommended, &theme("capitaine-cursors"), &theme("Adwaita"), &theme("Bibata-Modern-Classic"), &CardValue::Install, &CardValue::Install],
            "önerilen, aile başına tek kart, katalog dışı kurulular, en sonda kurulabilir aileler"
        );
        // Önerilen aile kurulu değil: önerilen kart onu temsil eder (ayrı Vimix aile kartı yok).
        assert_eq!(cards[0].name, "Vimix");
        assert!(!cards[0].installed);
        assert_eq!(cards[0].install.as_deref(), Some("sudo pacman -S vimix-cursors"));
        // Çok varyantlı aile: başlık aile adı, varyantlar katalog sırasıyla.
        assert_eq!(cards[1].name, "Capitaine");
        assert_eq!(ids(&cards[1]), ["capitaine-cursors", "capitaine-cursors-light"]);
        assert!(cards[2].variants.is_empty() && cards[3].variants.is_empty(), "katalog dışı tekil");
        assert_eq!(cards[4].name, "Oreo");
        assert_eq!(cards[4].install.as_deref(), Some("paru -S oreo-cursors-bin"));
        assert!(cards[4].aur && !cards[4].installed);
        assert!(cards[1..4].iter().all(|c| c.installed && c.install.is_none()));
        // Future katalogda yok: gösterilmez; yerel aile listesi yok.
        assert!(!cards.iter().any(|c| c.name.contains("Future")));
    }

    /// Çok varyantlı aile (~40 kurulu Oreo varyantı) tek kart; tek varyantlı aile tekil kart ve
    /// seçicisiz; aynı kimlik iki kez listelenmez.
    #[test]
    fn many_variants_group_into_one_card() {
        let oreo: Vec<String> = (0..40).map(|i| format!("oreo_c{i}_cursors")).collect();
        let mut cat = cursor_catalog();
        cat[2].installed = oreo.clone();
        cat[3].installed = vec!["Sweet-cursors".into()];
        let mut installed: Vec<InstalledTheme> = oreo.iter().map(|i| t(i)).collect();
        installed.push(t("Sweet-cursors"));
        let cards = tc(Some("Vimix-cursors"), Some(&cat), &installed, Some("oreo_c7_cursors"));
        let oreo_cards: Vec<_> = cards.iter().filter(|c| c.name == "Oreo").collect();
        assert_eq!(oreo_cards.len(), 1);
        assert_eq!(oreo_cards[0].variants.len(), 40);
        assert_eq!(oreo_cards[0].variants[7].label, "C7", "ortak baş/son sözcükler atılır");
        assert!(oreo_cards[0].matches(Some("oreo_c7_cursors")), "varyantı yazılı aile kartı seçili");
        assert!(!cards.iter().any(|c| c.theme_id() == Some("oreo_c7_cursors") && c.variants.is_empty()), "varyant ayrı kart olmaz");
        let sweet = cards.iter().find(|c| c.theme_id() == Some("Sweet-cursors")).unwrap();
        assert!(sweet.variants.is_empty(), "tek varyantlı aile: seçici yok");
        assert_eq!(sweet.name, "Sweet cursors");
        let all: Vec<&str> = cards.iter().flat_map(|c| if c.variants.is_empty() { c.theme_id().into_iter().collect() } else { ids(c) }).collect();
        assert_eq!(all.len(), all.iter().collect::<HashSet<_>>().len(), "{all:?}");
    }

    #[test]
    fn variant_labels_strip_common_words() {
        let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(variant_labels(&s(&["Vimix Cursors", "Vimix Cursors - White"])), ["Standart", "White"]);
        assert_eq!(variant_labels(&s(&["capitaine cursors", "capitaine cursors light"])), ["Standart", "Light"]);
        assert_eq!(variant_labels(&s(&["Breeze", "Breeze Dark", "Breeze Plus Dark"])), ["Standart", "Dark", "Plus Dark"]);
        assert_eq!(variant_labels(&s(&["oreo_blue_cursors", "oreo_red_cursors"])), ["Blue", "Red"]);
        // Ayırt edilemiyorsa görünen adlar.
        assert_eq!(variant_labels(&s(&["A x", "a-x"])), ["A x", "a-x"]);
    }

    /// Önerilen kurulu ve ailesinin birden çok varyantı var: önerilen kart ailenin kartıdır;
    /// önerilen varyant anahtarı siler, diğeri adını yazar.
    #[test]
    fn recommended_family_card_and_variant_writes() {
        let mut cat = cursor_catalog();
        cat[0].installed = vec!["Vimix-cursors".into(), "Vimix-white-cursors".into()];
        let installed = [InstalledTheme { id: "Vimix-cursors".into(), name: "Vimix Cursors".into() }, InstalledTheme { id: "Vimix-white-cursors".into(), name: "Vimix Cursors - White".into() }];
        let rec = Some("Vimix-cursors");
        let cards = tc(rec, Some(&cat), &installed, None);
        let c = &cards[0];
        assert_eq!((&c.value, c.name.as_str()), (&CardValue::Recommended, "Vimix"));
        assert_eq!(ids(c), ["Vimix-cursors", "Vimix-white-cursors"]);
        assert!(!cards[1..].iter().any(|c| c.name.starts_with("Vimix")), "aile ikinci kez kart olmaz");
        // Önerilen seçili (anahtar yok): etkin varyant öneri, önizleme onu gösterir.
        assert!(c.matches(None));
        assert_eq!(c.active_variant(rec, None), Some(0));
        assert_eq!(preview_id(c, rec, None), Some("Vimix-cursors"));
        assert_eq!(caption(c, rec, None, true), "Önerilen · Standart");
        // Beyaz varyant yazılı: kart yine seçili, önizleme ve alt yazı o varyantta.
        let w = Some("Vimix-white-cursors");
        assert!(c.matches(w));
        assert_eq!(preview_id(c, rec, w), Some("Vimix-white-cursors"));
        assert_eq!(caption(c, rec, w, true), "White");
        // Yazımlar: önerilen → anahtar silinir; diğeri adıyla.
        assert_eq!(c.variant_value(0, rec), Some(None));
        assert_eq!(c.variant_value(1, rec), Some(Some("Vimix-white-cursors".into())));
        assert_eq!(c.variant_value(2, rec), None);
        // Başka bir tema seçiliyken: "n varyant".
        assert!(!c.matches(Some("Adwaita")));
        assert_eq!(caption(c, rec, Some("Adwaita"), true), "Önerilen · 2 varyant");
        // Tek varyantlı ailede önerilen tekil kart (seçici yok).
        cat[0].installed = vec!["Vimix-cursors".into()];
        let cards = tc(rec, Some(&cat), &installed[..1], None);
        assert!(cards[0].variants.is_empty());
        assert_eq!(cards[0].variant_value(0, rec), None);
    }

    #[test]
    fn uninstalled_family_never_carries_a_theme_name() {
        // Kurulu olmayan ailenin kartı config'e yazılacak bir kimlik taşımaz (tahmin yok).
        let cat = cursor_catalog();
        for c in tc(Some("Vimix-cursors"), Some(&cat), &[], None) {
            if !c.installed {
                assert!(c.theme_id().is_none(), "{c:?}");
            }
        }
    }

    #[test]
    fn installed_recommendation_is_not_repeated() {
        let mut cat = cursor_catalog();
        cat[0].installed = vec!["Vimix-cursors".into(), "Vimix-white-cursors".into()];
        let installed = [t("Vimix-cursors"), t("Vimix-white-cursors")];
        let cards = tc(Some("Vimix-cursors"), Some(&cat), &installed, None);
        assert_eq!(cards[0].value, CardValue::Recommended);
        assert_eq!(ids(&cards[0]), ["Vimix-cursors", "Vimix-white-cursors"], "ailenin diğer varyantı önerilen kartın seçicisinde");
        assert_eq!(cards.iter().filter(|c| c.theme_id() == Some("Vimix-cursors")).count(), 0);
        // Ailenin başka varyantı kurulu ama önerilen değil: paket kurulu, komut gösterilmez.
        let cards = tc(Some("Vimix-cursors"), Some(&cat), &installed[1..], None);
        let mut cat2 = cat.clone();
        cat2[0].installed = vec!["Vimix-white-cursors".into()];
        let cards2 = tc(Some("Vimix-cursors"), Some(&cat2), &installed[1..], None);
        assert!(cards[0].installed, "kurulu varyant listesinde varsa kurulu sayılır");
        assert!(!cards2[0].installed && cards2[0].install.is_none());
        assert!(!cards2.iter().any(|c| c.value == CardValue::Install && c.name == "Vimix"));
    }

    #[test]
    fn old_shell_without_catalog_lists_only_installed() {
        // Sürüm 2: katalog yok → aile kartı, kurulum komutu ve tahmin edilen dizin adı yok.
        let cards = tc(Some("Papirus"), None, &[t("Adwaita"), t("breeze")], Some("Yaru"));
        let values: Vec<&CardValue> = cards.iter().map(|c| &c.value).collect();
        assert_eq!(values, [&CardValue::Recommended, &theme("Adwaita"), &theme("breeze"), &theme("Yaru")]);
        assert!(!cards[0].installed && cards[0].install.is_none(), "önerilen kurulu değil, komut bilinmiyor");
        assert!(!cards[3].installed, "dosyadaki kurulu olmayan seçim soluk");
        assert!(cards.iter().all(|c| c.install.is_none()));
        // Shell yok: adsız öneri.
        let cards = tc(None, None, &[t("Papirus")], None);
        assert_eq!(cards[0], CardSpec::new(CardValue::Recommended, "Önerilen".into(), true));
    }

    #[test]
    fn missing_current_comes_before_installable_families() {
        let cat = cursor_catalog();
        let cards = tc(Some("Vimix-cursors"), Some(&cat), &[], Some("Eski-tema"));
        let i = cards.iter().position(|c| c.value == theme("Eski-tema")).unwrap();
        assert!(cards[i + 1..].iter().all(|c| c.value == CardValue::Install));
    }

    /// Klavye: oklar odağı ızgarada gezdirir (yukarı/aşağı sütun kadar), Enter seçer ve tek
    /// anahtar yazar; kurulu olmayan aile hiçbir şey yazmaz, kurulum komutunu gösterir.
    #[test]
    fn keyboard_moves_chooses_and_family_shows_command() {
        let theme = Theme::default();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let desc = crate::registry::by_id("desktop.cursor_theme").unwrap();
        let mut g = ThemeGrid::new(&theme, desc, ThemeKind::Cursors, &store);
        let cat = cursor_catalog();
        let specs = tc(Some("Vimix-cursors"), Some(&cat), &[t("Adwaita")], None);
        g.cards = specs.into_iter().map(|s| Card::new(&theme, ThemeKind::Cursors, s, "")).collect();
        g.built_for = Some(BuildKey { installed: vec![t("Adwaita")], catalog: Some(cat), current: None, rec: Some("Vimix-cursors".into()) });
        // [Önerilen Vimix] [Capitaine (2 varyant)] [Adwaita] [Oreo] / [Sweet]
        g.cols = 4;
        g.focus_i = 0;
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let press = |k| KeyEvent::new(k, KeyState::Pressed, None, Default::default());
        assert!(g.key(&mut env, &press(Key::Down)));
        assert_eq!(g.focus_i, 4);
        assert!(g.key(&mut env, &press(Key::Enter)), "Sweet: kurulu değil");
        assert_eq!(g.command.as_deref(), Some("paru -S sweet-cursors-git"));
        assert!(g.footer.text().contains("paru -S sweet-cursors-git"));
        assert!(g.key(&mut env, &press(Key::Up)));
        assert!(g.key(&mut env, &press(Key::Right)));
        assert!(g.key(&mut env, &press(Key::Enter)));
        assert_eq!(g.shown_family, None, "tema seçimi aile notunu kapatır");
        assert!(g.key(&mut env, &press(Key::End)));
        assert_eq!(g.focus_i, g.cards.len() - 1);
        let written: Vec<Option<String>> = actions
            .iter()
            .filter_map(|a| match a {
                Action::Set { value, .. } => Some(value.as_ref().and_then(Value::as_str).map(str::to_owned)),
                _ => None,
            })
            .collect();
        assert_eq!(written, [Some("capitaine-cursors".to_owned())], "yalnızca kurulu tema yazıldı; aile kartı hiçbir şey yazmadı");
    }

    /// Önerilen seçili ve kurulu değil: not + katalogdaki paketin komutu.
    #[test]
    fn recommended_not_installed_footer_uses_catalog_package() {
        let theme = Theme::default();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let desc = crate::registry::by_id("desktop.cursor_theme").unwrap();
        let mut g = ThemeGrid::new(&theme, desc, ThemeKind::Cursors, &store);
        let cat = cursor_catalog();
        let specs = tc(Some("Vimix-cursors"), Some(&cat), &[], None);
        g.cards = specs.into_iter().map(|s| Card::new(&theme, ThemeKind::Cursors, s, "")).collect();
        g.built_for = Some(BuildKey { installed: vec![], catalog: Some(cat), current: None, rec: Some("Vimix-cursors".into()) });
        g.update_footer();
        assert_eq!(g.command.as_deref(), Some("sudo pacman -S vimix-cursors"));
        assert!(g.footer.text().starts_with("Vimix kurulu değil"));
        assert!(!g.footer.text().contains(&look::aur_helper_note()), "depo paketi: not yok");
    }

    fn grid_with(theme: &Theme, store: &Store, key: BuildKey, helper: Option<AurHelper>) -> ThemeGrid {
        let desc = crate::registry::by_id("desktop.cursor_theme").unwrap();
        let mut g = ThemeGrid::with_previews(theme, desc, ThemeKind::Cursors, store, Previews::recording(1 << 20));
        g.helper = helper;
        g.build(key);
        g.update_footer();
        g
    }

    fn written(actions: &[Action]) -> Vec<Option<String>> {
        actions
            .iter()
            .filter_map(|a| match a {
                Action::Set { value, .. } => Some(value.as_ref().and_then(Value::as_str).map(str::to_owned)),
                _ => None,
            })
            .collect()
    }

    /// Seçili aile kartında seçici açılır (2 varyant → bölümlü düğme); varyant seçmek o dizin
    /// adını yazar; seçili tekil kartta seçici yok. Seçici klavyede kartlardan sonra gelir.
    #[test]
    fn variant_picker_follows_selected_family() {
        let theme = Theme::default();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let key = |current: Option<&str>| BuildKey { installed: vec![t("Adwaita")], catalog: Some(cursor_catalog()), current: current.map(str::to_owned), rec: Some("Vimix-cursors".into()) };
        let mut g = grid_with(&theme, &store, key(Some("capitaine-cursors-light")), Some(AurHelper::Paru));
        assert_eq!(g.selected(), Some(1), "varyantı yazılı aile kartı seçili");
        let p = g.picker.as_ref().expect("seçici");
        assert_eq!(p.ids, ["capitaine-cursors", "capitaine-cursors-light"]);
        assert_eq!(p.seg.as_ref().map(SegmentedButton::selected), Some(1), "2 varyant: bölümlü düğme, etkin varyant seçili");
        assert_eq!(g.cards[1].preview.as_ref().map(CardPreview::id), Some("capitaine-cursors-light"), "önizleme seçili varyantta");
        assert_eq!(g.cards[1].caption.text(), "Light");
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        // Yerleşim: seçici ızgara ile alt not arasında.
        let h = g.height(&mut env, 720.0);
        g.place(&mut env, Rect::new(0.0, 0.0, 720.0, h));
        let pr = g.picker.as_ref().unwrap().rect;
        assert!(pr.y > g.grid_bottom && pr.bottom() <= g.footer_top + 0.5, "{pr:?}");
        assert!(g.picker.as_ref().unwrap().use_seg, "geniş satırda bölümlü düğme");
        // Klavye: Tab kartlardan seçiciye; Sol önceki varyantı yazar.
        g.set_focus(true, true, env.now);
        assert!(g.focus_step(true, env.now));
        assert_eq!(g.part, Part::Variant);
        let press = |k| KeyEvent::new(k, KeyState::Pressed, None, Default::default());
        assert!(g.key(&mut env, &press(Key::Left)));
        // Aynı kartta tekrar tık seçili varyantı değiştirmez.
        g.choose(&mut env, 1);
        assert_eq!(written(env.actions), [Some("capitaine-cursors".to_owned())]);
        // Tekil kart seçili: seçici yok; Tab kartlardan sonra bölüm bulamaz.
        let mut g = grid_with(&theme, &store, key(Some("Adwaita")), Some(AurHelper::Paru));
        assert!(g.picker.is_none());
        g.set_focus(true, true, Instant::now());
        assert!(!g.focus_step(true, Instant::now()));
        // Aynı aile içinde seçim değişince seçici yeniden kurulmaz, yalnızca seçimi kayar.
        let mut g = grid_with(&theme, &store, key(Some("capitaine-cursors")), Some(AurHelper::Paru));
        assert_eq!(g.cards[1].preview.as_ref().map(CardPreview::id), Some("capitaine-cursors"));
        g.build(key(Some("capitaine-cursors-light")));
        assert_eq!(g.picker.as_ref().and_then(|p| p.seg.as_ref()).map(SegmentedButton::selected), Some(1));
        assert_eq!(g.cards[1].preview.as_ref().map(CardPreview::id), Some("capitaine-cursors-light"), "varyant değişince önizleme o varyanta geçer");
    }

    /// Çok varyant (> 5): açılır seçim; dar satırda seçim etiketin altına iner.
    #[test]
    fn many_variants_use_select_and_stack_when_narrow() {
        let theme = Theme::default();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let mut cat = cursor_catalog();
        cat[2].installed = (0..12).map(|i| format!("oreo_c{i}_cursors")).collect();
        let key = BuildKey { installed: vec![], catalog: Some(cat), current: Some("oreo_c3_cursors".into()), rec: Some("Vimix-cursors".into()) };
        let mut g = grid_with(&theme, &store, key, Some(AurHelper::Paru));
        let p = g.picker.as_mut().unwrap();
        assert!(p.seg.is_none());
        assert_eq!(p.select.selected(), 3);
        let mut text = TextSystem::new();
        let wide = p.layout(&mut text, &theme, 700.0);
        assert!(!p.use_seg && !p.stacked);
        let narrow = p.layout(&mut text, &theme, theme.space(60));
        assert!(!p.use_seg && p.stacked && narrow > wide);
    }

    /// AUR komutu kurulu yardımcıyla; yardımcı yoksa paru + kısa not.
    #[test]
    fn install_footer_uses_detected_aur_helper() {
        let theme = Theme::default();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let key = || BuildKey { installed: vec![t("Adwaita")], catalog: Some(cursor_catalog()), current: None, rec: Some("Vimix-cursors".into()) };
        let oreo = |g: &ThemeGrid| g.cards.iter().position(|c| c.spec.name == "Oreo").unwrap();
        for (helper, cmd, note) in [(Some(AurHelper::Yay), "yay -S oreo-cursors-bin", false), (None, "paru -S oreo-cursors-bin", true)] {
            let mut g = grid_with(&theme, &store, key(), helper);
            let i = oreo(&g);
            assert_eq!(g.cards[i].spec.install.as_deref(), Some(cmd));
            g.shown_family = Some(i);
            g.update_footer();
            assert_eq!(g.command.as_deref(), Some(cmd));
            assert!(g.footer.text().contains(cmd));
            assert_eq!(g.footer.text().ends_with(&look::aur_helper_note()), note, "{}", g.footer.text());
        }
    }

    #[test]
    fn only_installed_cards_preview_and_recommended_shows_its_theme() {
        let cat = cursor_catalog();
        let installed = [t("Adwaita"), t("capitaine-cursors")];
        let mut cat2 = cat.clone();
        cat2[0].installed = vec!["Vimix-cursors".into()];
        let ids = |cards: &[CardSpec], rec| cards.iter().map(|c| preview_id(c, rec, None).map(str::to_owned)).collect::<Vec<_>>();
        // Önerilen kurulu: önerinin kendisi önizlenir.
        let cards = tc(Some("Vimix-cursors"), Some(&cat2), &installed, None);
        assert_eq!(ids(&cards, Some("Vimix-cursors"))[0].as_deref(), Some("Vimix-cursors"));
        // Önerilen kurulu değil, aile kartları, dosyadaki kurulu olmayan seçim: önizleme yok.
        let cards = tc(Some("Vimix-cursors"), Some(&cat), &installed, Some("Eski"));
        let got = ids(&cards, Some("Vimix-cursors"));
        for (c, id) in cards.iter().zip(&got) {
            assert_eq!(id.is_some(), c.installed && c.value != CardValue::Recommended, "{c:?}");
        }
        // Shell yok: adsız öneri kurulu sayılır ama hangi tema olduğu bilinmez.
        let cards = tc(None, None, &installed, None);
        assert_eq!(ids(&cards, None)[0], None);
    }

    #[test]
    fn preview_keys_carry_scale_and_card_role_order() {
        let theme = Theme::default();
        let c = CardPreview::new(&theme, ThemeKind::Cursors, "Adwaita");
        let i = CardPreview::new(&theme, ThemeKind::Icons, "Papirus");
        let cur = |slot, scale| match c.key(slot, scale) {
            Some(PreviewKey::Cursor { theme, role, px }) => (theme, role, px),
            k => panic!("{k:?}"),
        };
        assert_eq!(cur(0, 1.0), ("Adwaita".into(), CursorRole::Default, 24));
        assert_eq!(cur(2, 1.0).1, CursorRole::Wait, "animasyonlu imleç dar kartta da görünür");
        assert_eq!(cur(0, 1.5).2, 36, "imleç: mantıksal boyut × ölçek");
        assert_eq!(cur(0, 2.0).2, 48);
        let icon_px = |scale| match i.key(0, scale) {
            Some(PreviewKey::Icon { px, .. }) => px,
            k => panic!("{k:?}"),
        };
        assert_eq!((icon_px(1.0), icon_px(1.5), icon_px(2.0)), (24, 48, 48), "ikon: kesirli ölçek bir üst tamsayıya");
        assert_eq!(c.key(9, 1.0), None);
        assert_eq!(i.key(8, 1.0), None);
        assert_ne!(c.key(0, 1.0), c.key(0, 1.5), "ölçek anahtarda: eski boyutun sonucu eşleşmez");
    }

    fn still(side: u32) -> PreviewImage {
        PreviewImage::from_decoded(nuxut_image::Decoded::Still(nuxut_image::Image::from_premultiplied(side, side, vec![255; (side * side * 4) as usize]).unwrap()))
    }

    /// Tembel istek, önbellekten teslim, "yok" durumu, ölçek değişiminde yeniden istek,
    /// yeniden kurulumda önizlemenin taşınması.
    #[test]
    fn grid_requests_visible_slots_and_reloads_on_scale_change() {
        let theme = Theme::default();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let desc = crate::registry::by_id("desktop.cursor_theme").unwrap();
        let service = Previews::recording(1 << 20);
        let mut g = ThemeGrid::with_previews(&theme, desc, ThemeKind::Cursors, &store, service.clone());
        let installed = vec![t("Adwaita"), t("Bibata")];
        let cat = cursor_catalog();
        let rec = Some("Adwaita");
        let mut reuse = Vec::new();
        let specs = tc(rec, Some(&cat), &installed, None);
        g.cards = specs.into_iter().map(|s| Card::with_preview(&theme, ThemeKind::Cursors, s, rec, None, &mut reuse)).collect();
        g.built_for = Some(BuildKey { installed, catalog: Some(cat), current: None, rec: rec.map(str::to_owned) });
        // [Önerilen Adwaita] [Capitaine (2 varyant)] [Bibata] [Vimix] [Oreo] [Sweet]
        assert_eq!(g.cards.iter().filter(|c| c.preview.is_some()).count(), 3);
        assert!(g.cards.iter().filter(|c| !c.spec.installed).all(|c| c.preview.is_none()));

        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        {
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            let h = g.height(&mut env, 720.0);
            g.place(&mut env, Rect::new(0.0, 0.0, 720.0, h));
        }
        // Bir yuva önbellekte: istenmez, kutuya hemen düşer.
        let first = g.cards[0].preview.as_ref().unwrap();
        let cached_key = first.key(0, 1.0).unwrap();
        let missing_key = first.key(1, 1.0).unwrap();
        service.cache().insert(cached_key.clone(), Some(still(24)));
        service.cache().insert(missing_key.clone(), None);
        let mut rec = Recorder::new();
        let mut draw = |g: &mut ThemeGrid, text: &mut TextSystem, scale: f32| {
            rec.record(text, ((1000.0 * scale) as u32, (1000.0 * scale) as u32), scale, |p| {
                let mut ui = Ui::new(p, &theme, now);
                g.draw(&mut ui);
            });
        };
        draw(&mut g, &mut text, 1.0);
        let asked = service.take_requests();
        let per_card = asked.iter().filter(|k| matches!(k, PreviewKey::Cursor { theme, .. } if theme == "Adwaita")).count();
        assert!(per_card >= 1 && per_card < card_preview::CURSOR_ROLES.len() - 2, "dar kart: yalnızca sığan yuvalar ({per_card})");
        assert!(!asked.contains(&cached_key) && !asked.contains(&missing_key), "önbellektekiler çözülmez");
        assert!(asked.iter().all(|k| matches!(k, PreviewKey::Cursor { px: 24, .. })));
        // İkinci çizim: önbellekten gelenler yüklenir, yeni istek yok.
        draw(&mut g, &mut text, 1.0);
        assert!(service.take_requests().is_empty(), "her yuva bir kez istenir");
        let p = g.cards[0].preview.as_ref().unwrap();
        assert!(matches!(p.slot(0), Some(PreviewSlot::Ready(f)) if f.len() == 1));
        assert_eq!(p.slot(1), Some(&PreviewSlot::Missing), "temada yok: soluk yer tutucu");
        assert_eq!(p.slot(2), Some(&PreviewSlot::Loading));

        // Seçim değişti → kartlar yeniden kurulur, aynı temanın yüklü yuvaları korunur.
        g.built_for.as_mut().unwrap().current = Some("Bibata".into());
        let key = g.built_for.clone().unwrap();
        g.built_for = None;
        let specs = tc(key.rec.as_deref(), key.catalog.as_deref(), &key.installed, key.current.as_deref());
        let mut reuse: Vec<CardPreview> = g.cards.iter_mut().filter_map(|c| c.preview.take()).collect();
        g.cards = specs.into_iter().map(|s| Card::with_preview(&theme, ThemeKind::Cursors, s, key.rec.as_deref(), key.current.as_deref(), &mut reuse)).collect();
        assert!(matches!(g.cards[0].preview.as_ref().unwrap().slot(0), Some(PreviewSlot::Ready(_))), "önizleme taşındı");

        // Ölçek 1,5: yuvalar sıfırlanır, görünenler 36 px ile yeniden istenir.
        draw(&mut g, &mut text, 1.5);
        let p = g.cards[0].preview.as_ref().unwrap();
        assert_eq!(p.slot(0), Some(&PreviewSlot::Loading), "eski boyutun dokusu bırakıldı");
        let asked = service.take_requests();
        assert!(!asked.is_empty() && asked.iter().all(|k| matches!(k, PreviewKey::Cursor { px: 36, .. })), "{asked:?}");
        // Eski ölçeğin geç gelen sonucu eşleşmez.
        g.inbox.lock().unwrap().push((cached_key, Some(still(24))));
        draw(&mut g, &mut text, 1.5);
        assert_eq!(g.cards[0].preview.as_ref().unwrap().slot(0), Some(&PreviewSlot::Loading));
    }
}
