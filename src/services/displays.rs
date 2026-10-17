//! Ekranlar: shell'in donuk v1 `nuxut-shell msg displays` sözleşmesi (shell ADR-0005 §13) için
//! saf model — `list` cevabını ayrıştırma, sayfanın düzenlediği yerleşim (mantıksal
//! dikdörtgenler, sürüklerken yapışma, bırakınca en yakın kenara oturma), istemci tarafı
//! doğrulama önizlemesi ve `apply` isteği. Asıl doğrulama ve uygulama shell'dedir; burada
//! yalnızca kullanıcıya geçersiz bir yerleşimi göndermeden göstermek için tekrarlanır.

use serde::{Deserialize, Serialize};

pub mod shell;

/// Sözleşme sınırları (ADR §7.2): fazlası bozuk cevap sayılır.
pub const MAX_LIST_BYTES: usize = 64 * 1024;
pub const MAX_DISPLAYS: usize = 16;
pub const MAX_RESOLUTIONS: usize = 64;
pub const MAX_RATES: usize = 16;
pub const MAX_STRING: usize = 64;
/// `apply` JSON'unun üst sınırı (ADR-0005 §13.1: framework satırı 4096 bayt, JSON ≤ 4000).
pub const MAX_APPLY_BYTES: usize = 4000;

/// Ölçek × 120 (Hyprland/wp_fractional_scale adımı).
pub const SCALE_DENOM: u32 = 120;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct List {
    pub v: u32,
    #[serde(rename = "gen")]
    pub generation: u64,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub pending: Option<Pending>,
    #[serde(default)]
    pub primary: Option<String>,
    pub displays: Vec<Display>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Pending {
    pub token: String,
    #[serde(default)]
    pub remaining_ms: u64,
}

/// `status.last`: shell'in sonuçlandırdığı son işlem (ADR-0005 §13.2).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Last {
    pub token: String,
    pub result: Outcome,
    #[serde(default)]
    pub detail: Option<String>,
}

/// `last.result`. Sözleşmedekiler: applied, confirmed, reverted, timeout, hotplug,
/// not_applied; shell kodu ayrıca not_shown, reloaded, error üretebilir. Bilinmeyen değer
/// ayrıştırmayı bozmaz ([`Outcome::Other`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Applied,
    Confirmed,
    Reverted,
    Timeout,
    Hotplug,
    NotApplied,
    NotShown,
    Reloaded,
    #[serde(other)]
    Other,
}

impl Outcome {
    /// Yeni ayar geçerli mi (değilse önceki ayarlara dönülmüştür).
    pub fn kept(self) -> bool {
        matches!(self, Self::Applied | Self::Confirmed)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Display {
    pub id: String,
    #[serde(default = "yes")]
    pub id_stable: bool,
    #[serde(default)]
    pub connector: String,
    #[serde(default)]
    pub make: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub physical_mm: Option<[u32; 2]>,
    #[serde(default = "yes")]
    pub connected: bool,
    pub enabled: bool,
    pub mode: Mode,
    pub pos: [i32; 2],
    pub scale: f64,
    /// Ölçek × 120 (sözleşmenin asıl ölçeği; `scale` yalnızca gösterim kolaylığı).
    #[serde(default)]
    pub scale_120: Option<u32>,
    #[serde(default)]
    pub transform: u8,
    #[serde(default)]
    pub vrr: Vrr,
    #[serde(default)]
    pub mirror_of: Option<String>,
    #[serde(default)]
    pub resolutions: Vec<Resolution>,
}

fn yes() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mode {
    pub w: u32,
    pub h: u32,
    pub mhz: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Resolution {
    pub w: u32,
    pub h: u32,
    pub rates_mhz: Vec<u32>,
    #[serde(default)]
    pub preferred: bool,
    #[serde(default)]
    pub scales: Vec<f64>,
    /// Önerilen temiz ölçekler × 120 (varsa `scales` yerine bu kullanılır).
    #[serde(default)]
    pub scales_120: Vec<u32>,
}

/// Sözleşme değerleri: `off|on|fullscreen|fullscreen-media`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Vrr {
    #[default]
    Off,
    On,
    Fullscreen,
    FullscreenMedia,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("cevap çok büyük")]
    TooLarge,
    #[error("cevap okunamadı: {0}")]
    Json(String),
    #[error("desteklenmeyen sürüm {0}")]
    Version(u32),
    #[error("sınır aşıldı: {0}")]
    Limit(&'static str),
}

impl List {
    /// `displays list` cevabı. Sınır dışı cevap reddedilir (yarım gösterilmez).
    pub fn parse(json: &str) -> Result<Self, ParseError> {
        if json.len() > MAX_LIST_BYTES {
            return Err(ParseError::TooLarge);
        }
        let list: List = serde_json::from_str(json).map_err(|e| ParseError::Json(e.to_string()))?;
        if list.v != 1 {
            return Err(ParseError::Version(list.v));
        }
        list.check_limits()?;
        Ok(list)
    }

    fn check_limits(&self) -> Result<(), ParseError> {
        let long = |s: &str| s.chars().count() > MAX_STRING || s.chars().any(char::is_control);
        if self.displays.len() > MAX_DISPLAYS {
            return Err(ParseError::Limit("monitör sayısı"));
        }
        for d in &self.displays {
            if [&d.id, &d.connector, &d.make, &d.model, &d.description].into_iter().any(|s| long(s)) || d.mirror_of.as_deref().is_some_and(long) {
                return Err(ParseError::Limit("dizgi"));
            }
            if d.resolutions.len() > MAX_RESOLUTIONS || d.resolutions.iter().any(|r| r.rates_mhz.len() > MAX_RATES || r.scales.len() > MAX_RATES || r.scales_120.len() > MAX_RATES) {
                return Err(ParseError::Limit("kip listesi"));
            }
            if d.mode.w == 0 || d.mode.h == 0 || !(0.25..=8.0).contains(&d.scale) || d.scale_120.is_some_and(|s| !(30..=960).contains(&s)) || d.transform > 7 {
                return Err(ParseError::Limit("kip"));
            }
        }
        if self.primary.as_deref().is_some_and(long) {
            return Err(ParseError::Limit("dizgi"));
        }
        Ok(())
    }
}

/// Ekran düzleminde mantıksal dikdörtgen (sol üst + boy).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn overlaps(&self, o: &Rect) -> bool {
        self.x < o.right() && o.x < self.right() && self.y < o.bottom() && o.y < self.bottom()
    }

    /// Kenar ya da köşe teması (iç içe geçmeden). Fare köşeden de geçer (ADR §3.3).
    pub fn touches(&self, o: &Rect) -> bool {
        if self.overlaps(o) {
            return false;
        }
        let x_meet = self.right() == o.x || o.right() == self.x;
        let y_meet = self.bottom() == o.y || o.bottom() == self.y;
        let x_span = self.x <= o.right() && o.x <= self.right();
        let y_span = self.y <= o.bottom() && o.y <= self.bottom();
        (x_meet && y_span) || (y_meet && x_span)
    }
}

/// Kip + ölçek + döndürmeden mantıksal boy (ADR §3.4; 90°/270° döndürmede en-boy takas).
pub fn logical_size(mode: Mode, scale_120: u32, transform: u8) -> (i32, i32) {
    let s = scale_120.max(1) as u64;
    let w = ((mode.w as u64 * SCALE_DENOM as u64 + s / 2) / s) as i32;
    let h = ((mode.h as u64 * SCALE_DENOM as u64 + s / 2) / s) as i32;
    if transform % 2 == 1 { (h, w) } else { (w, h) }
}

/// Ölçek temiz bir bölen mi (Hyprland kuralının tam sayı karşılığı, ADR §3.3).
pub fn scale_fits(mode: Mode, scale_120: u32) -> bool {
    (60..=480).contains(&scale_120) && (mode.w * SCALE_DENOM).is_multiple_of(scale_120) && (mode.h * SCALE_DENOM).is_multiple_of(scale_120)
}

pub fn scale_to_120(scale: f64) -> u32 {
    (scale * SCALE_DENOM as f64).round().clamp(1.0, 8.0 * SCALE_DENOM as f64) as u32
}

/// Kullanıcıya anlamlı ölçekler × 120 (ADR-0005 §13.2): 1, 1.25, 1.333…, 1.5, 1.6, 1.75, 2, 2.25, 2.5, 3, 4.
pub const SCALE_CHOICES_120: [u32; 11] = [120, 150, 160, 180, 192, 210, 240, 270, 300, 360, 480];

/// Bu kip için sunulacak ölçekler (× 120, artan): shell'in önerisi (`scales_120`, yoksa
/// `scales`) varsa o (temiz olmayanlar ayıklanır), yoksa [`SCALE_CHOICES_120`]'nin temiz bölenleri.
pub fn scale_choices(res: Option<&Resolution>, mode: Mode) -> Vec<u32> {
    let mut out: Vec<u32> = match res {
        Some(r) if !r.scales_120.is_empty() => r.scales_120.clone(),
        Some(r) if !r.scales.is_empty() => r.scales.iter().filter(|s| s.is_finite()).map(|s| scale_to_120(*s)).collect(),
        _ => SCALE_CHOICES_120.to_vec(),
    };
    out.retain(|s| scale_fits(mode, *s));
    out.sort_unstable();
    out.dedup();
    out
}

/// `scale_120`'ye en yakın temiz ölçek (önce önerilenler, yoksa 1/120 adımlı tüm temiz bölenler).
pub fn nearest_scale(mode: Mode, scale_120: u32) -> u32 {
    let near = |c: &mut dyn Iterator<Item = u32>| c.min_by_key(|c| (c.abs_diff(scale_120), *c));
    near(&mut SCALE_CHOICES_120.into_iter().filter(|s| scale_fits(mode, *s)))
        .or_else(|| near(&mut (60..=480).filter(|s| scale_fits(mode, *s))))
        .unwrap_or(SCALE_DENOM)
}

/// Döndürme (0..=3, 90° adımı) ve yatay çevirme `transform`'dan (Hyprland 0..7; 4.. çevrilmiş).
pub fn rotation(transform: u8) -> u8 {
    transform % 4
}

pub fn flipped(transform: u8) -> bool {
    transform % 8 >= 4
}

pub fn transform_of(rotation: u8, flipped: bool) -> u8 {
    rotation % 4 + if flipped { 4 } else { 0 }
}

/// İstenen yenilemeye en yakın hız (eşitlikte yüksek olan); liste boşsa `None`.
pub fn nearest_rate(rates: &[u32], mhz: u32) -> Option<u32> {
    rates.iter().copied().min_by_key(|r| (r.abs_diff(mhz), u32::MAX - r))
}

impl Display {
    /// Ölçek × 120: sözleşmenin `scale_120`'si; yoksa ondalık `scale`'den yuvarlanır.
    pub fn scale_120(&self) -> u32 {
        self.scale_120.unwrap_or_else(|| scale_to_120(self.scale))
    }

    pub fn resolution(&self, w: u32, h: u32) -> Option<&Resolution> {
        self.resolutions.iter().find(|r| r.w == w && r.h == h)
    }

    /// Kullanıcıya gösterilecek ad: model, yoksa açıklama, yoksa bağlayıcı.
    pub fn name(&self) -> &str {
        [&self.model, &self.description, &self.connector, &self.id].into_iter().find(|s| !s.is_empty()).map_or("", |s| s.as_str())
    }
}

/// Sayfanın düzenlediği monitör ayarı (`apply` isteğinin bir öğesi).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DisplayEdit {
    pub id: String,
    pub enabled: bool,
    pub mode: Mode,
    pub pos: [i32; 2],
    pub scale_120: u32,
    pub transform: u8,
    pub vrr: Vrr,
    pub mirror_of: Option<String>,
}

impl DisplayEdit {
    pub fn rect(&self) -> Rect {
        let (w, h) = logical_size(self.mode, self.scale_120, self.transform);
        Rect { x: self.pos[0], y: self.pos[1], w, h }
    }

    /// Yerleşimde yer kaplıyor mu (etkin ve yansıtma değil).
    pub fn placed(&self) -> bool {
        self.enabled && self.mirror_of.is_none()
    }
}

/// Doğrulama bulgusu (sayfa satırın altında kullanıcı diliyle gösterir).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Issue {
    /// Hiç etkin, yansıtma olmayan monitör yok.
    NoActiveDisplay,
    Overlap(String, String),
    /// Yerleşim tek parça değil (fare bir monitörden ötekine geçemez).
    Disconnected,
    /// Ölçek bu çözünürlüğü tam bölmüyor.
    InvalidScale(String),
    /// Yansıtılan monitör yok, kapalı ya da kendisi yansıtma.
    BadMirror(String),
}

/// Sayfanın üzerinde çalıştığı tam istenen durum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub base_gen: u64,
    pub primary: Option<String>,
    pub displays: Vec<DisplayEdit>,
}

impl Layout {
    /// `list`'teki bağlı monitörlerden.
    pub fn from_list(list: &List) -> Self {
        let displays = list
            .displays
            .iter()
            .filter(|d| d.connected)
            .map(|d| DisplayEdit {
                id: d.id.clone(),
                enabled: d.enabled,
                mode: d.mode,
                pos: d.pos,
                scale_120: d.scale_120(),
                transform: d.transform,
                vrr: d.vrr,
                mirror_of: d.mirror_of.clone(),
            })
            .collect();
        Self { base_gen: list.generation, primary: list.primary.clone(), displays }
    }

    pub fn get(&self, id: &str) -> Option<&DisplayEdit> {
        self.displays.iter().find(|d| d.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut DisplayEdit> {
        self.displays.iter_mut().find(|d| d.id == id)
    }

    fn others(&self, id: &str) -> Vec<Rect> {
        self.displays.iter().filter(|d| d.id != id && d.placed()).map(DisplayEdit::rect).collect()
    }

    /// Sürüklerken: `id`'yi `(x, y)`'ye koyar, kenarları `threshold` içindeki komşu kenarlarına
    /// yapıştırır (mıknatıs). Çakışmayı düzeltmez; o, bırakınca [`settle`](Self::settle).
    pub fn drag_to(&mut self, id: &str, x: i32, y: i32, threshold: i32) {
        let others = self.others(id);
        let Some(d) = self.get_mut(id) else { return };
        let r = Rect { x, y, ..d.rect() };
        let pick = |cands: &mut dyn Iterator<Item = i32>| cands.filter(|dv| dv.abs() <= threshold).min_by_key(|dv| dv.abs()).unwrap_or(0);
        let dx = pick(&mut others.iter().flat_map(|o| [o.x - r.x, o.right() - r.x, o.x - r.right(), o.right() - r.right()]));
        let dy = pick(&mut others.iter().flat_map(|o| [o.y - r.y, o.bottom() - r.y, o.y - r.bottom(), o.bottom() - r.bottom()]));
        d.pos = [x + dx, y + dy];
    }

    /// Bırakınca (ve kip/ölçek/döndürme değişince): `id` çakışıyor ya da kopuksa, istenen yere
    /// en yakın, kimseyle çakışmayan ve bir komşuya değen konuma oturur. Komşular itilmez.
    pub fn settle(&mut self, id: &str) {
        let others = self.others(id);
        let Some(d) = self.get_mut(id) else { return };
        if !d.placed() || others.is_empty() {
            return;
        }
        let r = d.rect();
        if !others.iter().any(|o| o.overlaps(&r)) && others.iter().any(|o| o.touches(&r)) {
            return;
        }
        let mut best: Option<((i64, i64), [i32; 2])> = None;
        for o in &others {
            let ys = (o.y - r.h, o.bottom());
            let xs = (o.x - r.w, o.right());
            let cands = [
                (o.right(), r.y.clamp(ys.0, ys.1)),
                (o.x - r.w, r.y.clamp(ys.0, ys.1)),
                (r.x.clamp(xs.0, xs.1), o.bottom()),
                (r.x.clamp(xs.0, xs.1), o.y - r.h),
            ];
            for (x, y) in cands {
                let c = Rect { x, y, ..r };
                if others.iter().any(|p| p.overlaps(&c)) {
                    continue;
                }
                let (dx, dy) = ((x - r.x) as i64, (y - r.y) as i64);
                // Eşit uzaklıkta kenar temasını köşe temasına tercih et.
                let corner_only = !others.iter().any(|p| edge_contact(p, &c));
                let key = (dx * dx + dy * dy, corner_only as i64);
                if best.is_none_or(|(k, _)| key < k) {
                    best = Some((key, [x, y]));
                }
            }
        }
        if let Some((_, pos)) = best {
            d.pos = pos;
        }
    }

    /// En küçük x ve y 0'a (ADR §3.3: negatif koordinatlar bazı istemcilerde sorun).
    pub fn normalize(&mut self) {
        let placed: Vec<Rect> = self.displays.iter().filter(|d| d.placed()).map(DisplayEdit::rect).collect();
        let (Some(min_x), Some(min_y)) = (placed.iter().map(|r| r.x).min(), placed.iter().map(|r| r.y).min()) else { return };
        for d in &mut self.displays {
            d.pos = [d.pos[0] - min_x, d.pos[1] - min_y];
        }
    }

    /// Shell'in reddedeceği durumlar (ADR §3.3), göndermeden önce gösterilsin diye.
    pub fn validate(&self) -> Vec<Issue> {
        let mut issues = Vec::new();
        let placed: Vec<&DisplayEdit> = self.displays.iter().filter(|d| d.placed()).collect();
        if placed.is_empty() {
            issues.push(Issue::NoActiveDisplay);
        }
        for d in self.displays.iter().filter(|d| d.enabled) {
            if !scale_fits(d.mode, d.scale_120) {
                issues.push(Issue::InvalidScale(d.id.clone()));
            }
            if let Some(m) = &d.mirror_of
                && !placed.iter().any(|p| &p.id == m)
            {
                issues.push(Issue::BadMirror(d.id.clone()));
            }
        }
        for (i, a) in placed.iter().enumerate() {
            for b in &placed[i + 1..] {
                if a.rect().overlaps(&b.rect()) {
                    issues.push(Issue::Overlap(a.id.clone(), b.id.clone()));
                }
            }
        }
        if placed.len() > 1 && !connected(&placed.iter().map(|d| d.rect()).collect::<Vec<_>>()) {
            issues.push(Issue::Disconnected);
        }
        issues
    }

    /// Konumları normalleştirilmiş kopya (karşılaştırma ve gönderim için).
    pub fn normalized(&self) -> Self {
        let mut l = self.clone();
        l.normalize();
        l
    }

    /// Kullanıcı `base`'e göre bir şey değiştirdi mi (yerleşimin yalnızca ötelenmesi değişiklik
    /// sayılmaz; shell zaten normalleştirir).
    pub fn differs_from(&self, base: &Layout) -> bool {
        let (a, b) = (self.normalized(), base.normalized());
        a.primary != b.primary || a.displays != b.displays
    }

    /// Shell'den yeni durum geldi (`gen` değişti) ama kullanıcı düzenliyor: bağlı monitör kümesi
    /// aynıysa düzenleme korunur ve yeni `gen`'e taşınır (`true`); değiştiyse (hotplug) düzenleme
    /// yeni duruma sıfırlanır (`false`).
    pub fn rebase(&mut self, fresh: &Layout) -> bool {
        let ids = |l: &Layout| {
            let mut v: Vec<String> = l.displays.iter().map(|d| d.id.clone()).collect();
            v.sort();
            v
        };
        if ids(self) == ids(fresh) {
            self.base_gen = fresh.base_gen;
            true
        } else {
            *self = fresh.clone();
            false
        }
    }

    /// Kip değişti: yenileme kipten gelir; ölçek bu çözünürlüğü tam bölmüyorsa en yakın temiz
    /// ölçeğe iner; yeni boy komşularla çakışırsa monitör oturur (komşular itilmez).
    pub fn set_mode(&mut self, id: &str, mode: Mode) {
        let Some(d) = self.get_mut(id) else { return };
        d.mode = mode;
        if !scale_fits(mode, d.scale_120) {
            d.scale_120 = nearest_scale(mode, d.scale_120);
        }
        self.settle(id);
    }

    pub fn set_scale(&mut self, id: &str, scale_120: u32) {
        let Some(d) = self.get_mut(id) else { return };
        d.scale_120 = scale_120;
        self.settle(id);
    }

    pub fn set_transform(&mut self, id: &str, transform: u8) {
        let Some(d) = self.get_mut(id) else { return };
        d.transform = transform % 8;
        self.settle(id);
    }

    /// Açma/kapama. Kapanan birincil monitör birincilliği bırakır (shell sol üsttekini seçer);
    /// açılan monitör çakışıyor ya da kopuksa en yakın kenara oturur.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) {
        let Some(d) = self.get_mut(id) else { return };
        d.enabled = enabled;
        if !enabled && self.primary.as_deref() == Some(id) {
            self.primary = None;
        }
        if enabled {
            self.settle(id);
        }
    }

    /// Yansıtma hedefi (`None`: kendi görüntüsü). Yansıtmadan çıkan monitör kaynağın üstünde
    /// kalmasın diye oturur; yansıtan monitör birincil olamaz.
    pub fn set_mirror(&mut self, id: &str, target: Option<String>) {
        if target.as_deref() == Some(id) {
            return;
        }
        let Some(d) = self.get_mut(id) else { return };
        let mirroring = target.is_some();
        d.mirror_of = target;
        if mirroring {
            if self.primary.as_deref() == Some(id) {
                self.primary = None;
            }
        } else {
            self.settle(id);
        }
    }

    /// `displays apply <json>` argümanı (tek satır). Satır sınırını aşarsa `None`.
    pub fn apply_json(&self) -> Option<String> {
        #[derive(Serialize)]
        struct Wire<'a> {
            v: u32,
            base_gen: u64,
            primary: &'a Option<String>,
            displays: &'a [DisplayEdit],
        }
        let s = serde_json::to_string(&Wire { v: 1, base_gen: self.base_gen, primary: &self.primary, displays: &self.displays }).ok()?;
        (s.len() <= MAX_APPLY_BYTES).then_some(s)
    }
}

fn edge_contact(a: &Rect, b: &Rect) -> bool {
    let x_meet = a.right() == b.x || b.right() == a.x;
    let y_meet = a.bottom() == b.y || b.bottom() == a.y;
    (x_meet && a.y < b.bottom() && b.y < a.bottom()) || (y_meet && a.x < b.right() && b.x < a.right())
}

/// Dikdörtgenler temasla tek parça mı.
fn connected(rects: &[Rect]) -> bool {
    let Some(_) = rects.first() else { return true };
    let mut seen = vec![false; rects.len()];
    let mut stack = vec![0];
    seen[0] = true;
    while let Some(i) = stack.pop() {
        for j in 0..rects.len() {
            if !seen[j] && rects[i].touches(&rects[j]) {
                seen[j] = true;
                stack.push(j);
            }
        }
    }
    seen.into_iter().all(|s| s)
}

#[cfg(test)]
mod tests;
