//! Hareket rolü editörü: rolün etkin stiline göre eğri grafiği (`nuxut_ui::CurveEditor`).
//! Yayda grafik zaman-konum eğrisidir; sürüklemek süreyi (yatay) ve sönümü (dikey) değiştirir.
//! Eğride cubic-bezier tutamaçları sürüklenir; adlandırılmış eğriden çıkınca değer
//! `cubic-bezier(x1, y1, x2, y2)` olarak yazılır. Grafiğin altında canlı oynayan önizleme topu.
//!
//! Boşta iş yok: önizleme yalnızca bir değişiklikten sonra ve imleç üstündeyken birkaç saniye
//! oynar; sonra kapanır ve kare istenmez.
//!
//! Hareket "Kapalı"yken (`registry::reduced_motion`, framework `Theme::reduced_motion()` ile aynı
//! kural, store'dan) grafik ve önizleme gizlenir; rol başlığı ve özet kalır
//! (2026-10-13: Kapalı'da eğri önizlemesi anlamsız).

use std::time::{Duration, Instant};

use nuxut::motion::CubicBezier;
use nuxut::ui::input::{CursorShape, PointerEvent, PointerKind};
use nuxut::ui::{CurveEditor, CurveEvent, CurveValue, Interaction, Label, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Spring, Theme};
use nuxut::i18n::t;
use toml_edit::Value;

use crate::block::{Action, Block, Env};
use crate::registry::{MotionRole, SettingDesc, by_id, quantize, reduced_motion};
use crate::rows::row_padding;
use crate::store::{FileId, KeyPath, Store};

/// Değişiklikten sonra önizlemenin oynadığı süre.
const PLAY_FOR: Duration = Duration::from_millis(4500);

#[derive(Debug)]
pub struct RolePreview {
    desc: &'static SettingDesc,
    role: MotionRole,
    /// Rol adı: açılır satırın içinde rol başına alt bölümün başlığı.
    title: Label,
    subtitle: Label,
    summary: Label,
    hint: Label,
    editor: CurveEditor,
    hover: Interaction,
    rect: Rect,
    /// Önizleme bu ana kadar açık.
    play_until: Option<Instant>,
    dragging: bool,
    /// Son eşitlenen değer (dış değişiklikte yeniden oynatmak için).
    last: Option<CurveValue>,
    duration: Duration,
    /// Hareket kapalı: eğri editörü (grafik + önizleme) gizli.
    hidden: bool,
}

fn role_value(role: MotionRole, store: &Store) -> (CurveValue, Duration) {
    let k = role.key();
    let num = |f: &str| by_id(&format!("motion.{k}.{f}")).map_or(0.0, |d| d.number(store)) as f32;
    let speed = by_id("motion.speed").map_or(1.0, |d| d.number(store)) as f32;
    if role.effective_style(store) == "spring" {
        (CurveValue::Spring(Spring::new(num("response"), num("damping"))), Duration::ZERO)
    } else {
        let easing = by_id(&format!("motion.{k}.easing")).map(|d| d.string(store)).unwrap_or_default();
        let b = CubicBezier::parse(&easing).unwrap_or(CubicBezier::STANDARD);
        let ms = (num("duration_ms") / speed.max(0.05)).clamp(60.0, 5000.0);
        (CurveValue::Bezier(b), Duration::from_millis(ms as u64))
    }
}

/// Yay değeri (en çok 2 kesir), etkin biçim locale'inin ondalık ayırıcısıyla.
fn fmt(v: f32) -> String {
    crate::numfmt::format(f64::from(v), 2)
}

fn bezier_text(b: CubicBezier) -> String {
    let f = |v: f32| {
        let s = format!("{:.2}", quantize(v as f64, 0.01));
        let s = s.trim_end_matches('0').trim_end_matches('.').to_owned();
        if s == "-0" { "0".to_owned() } else { s }
    };
    format!("cubic-bezier({}, {}, {}, {})", f(b.x1), f(b.y1), f(b.x2), f(b.y2))
}

impl RolePreview {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, role: MotionRole, store: &Store) -> Self {
        let (value, duration) = role_value(role, store);
        let mut s = Self {
            desc,
            role,
            title: Label::new(role.title(), text_style(theme, TextRole::Body)),
            subtitle: Label::new(role.subtitle(), text_style(theme, TextRole::Caption)),
            summary: Label::new("", text_style(theme, TextRole::Caption)),
            hint: Label::new("", text_style(theme, TextRole::Caption)),
            editor: CurveEditor::new(value),
            hover: Interaction::new(theme),
            rect: Rect::ZERO,
            play_until: None,
            dragging: false,
            last: Some(value),
            duration,
            hidden: reduced_motion(store),
        };
        s.refresh_texts(store);
        s.play(Instant::now());
        s
    }

    fn refresh_texts(&mut self, store: &Store) {
        let k = self.role.key();
        let (text, hint) = match role_value(self.role, store).0 {
            CurveValue::Spring(sp) => (
                t!("settings-motion-role-summary-spring", response = fmt(sp.response), damping = fmt(sp.damping)),
                t!("settings-motion-role-hint-spring"),
            ),
            CurveValue::Bezier(_) => {
                let e = by_id(&format!("motion.{k}.easing")).map(|d| d.string(store)).unwrap_or_default();
                let name = Some(crate::registry::choice_label(crate::registry::EASINGS, &e)).filter(|l| !l.is_empty()).map_or_else(|| t!("settings-motion-easing-custom"), str::to_owned);
                let ms = by_id(&format!("motion.{k}.duration_ms")).map_or(0.0, |d| d.number(store));
                (t!("settings-motion-role-summary-bezier", name = name, ms = ms.round().to_string()), t!("settings-motion-role-hint-bezier"))
            }
        };
        self.summary.set_text(&text);
        self.hint.set_text(&hint);
    }

    fn play(&mut self, now: Instant) {
        if self.hidden {
            return;
        }
        self.play_until = Some(now + PLAY_FOR);
        let d = if self.duration.is_zero() { Duration::from_millis(600) } else { self.duration };
        self.editor.set_preview(Some(d));
    }

    /// Başlık (rol adı) + açıklama yüksekliği.
    fn head_height(&mut self, text: &mut nuxut::TextSystem, theme: &Theme) -> f32 {
        self.title.measure(text).height + theme.space(1) + self.subtitle.measure(text).height
    }

    fn editor_width(&self, theme: &Theme, width: f32) -> f32 {
        let (px, _) = row_padding(theme);
        (width - px * 2.0).min(theme.space(84))
    }
}

impl Block for RolePreview {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (_, py) = row_padding(t);
        let ew = self.editor_width(t, width);
        // Önizleme açık/kapalı fark etmeksizin aynı yükseklik (satırlar zıplamasın).
        let editor_h = (ew * 0.7).round() + t.space(10);
        let head = self.head_height(env.text, t);
        if self.hidden {
            // Yalnız rol başlığı (alt bölümün adı kalır; altındaki satırlar ona ait).
            return t.space(3) + py + head + py;
        }
        // Üstte alt bölüm boşluğu: açılır satırın içinde rol, önceki satırdan ayrışsın.
        t.space(3) + py + head + t.space(3) + editor_h * 0.62 + py
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        let t = env.theme;
        let (px, py) = row_padding(t);
        self.rect = rect;
        let head = self.head_height(env.text, t);
        let ew = self.editor_width(t, rect.width) * 0.62;
        let top = rect.y + t.space(3) + py + head + t.space(3);
        let eh = rect.bottom() - py - top;
        self.editor.set_rect(Rect::new(rect.x + px, top, ew.max(t.space(30)), eh.max(t.space(20))));
        let hint_x = self.editor.rect().right() + t.space(6);
        self.hint.set_max_width(Some((rect.right() - px - hint_x).max(t.space(20))));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        if self.hidden {
            return false;
        }
        let now = env.now;
        let tap = self.hover.handle(self.rect, e, now);
        let mut repaint = tap.repaint;
        if self.hover.is_hovered() {
            self.play(now);
            repaint = true;
        }
        if e.kind == PointerKind::Leave {
            repaint |= self.hover.reset(now);
        }
        let was = self.dragging;
        let r = self.editor.pointer(env.theme, e);
        repaint |= r.repaint;
        let k = self.role.key();
        match r.event {
            Some(CurveEvent::Spring(sp)) => {
                self.dragging = true;
                let resp = quantize(sp.response as f64, 0.01).clamp(0.05, 1.5);
                let damp = quantize(sp.damping as f64, 0.01).clamp(0.2, 1.5);
                env.push(Action::Group {
                    label: t!("settings-motion-role-drag-spring", role = self.role.title()),
                    writes: vec![
                        (FileId::Framework, KeyPath::new(["motion", k, "response"]), Some(Value::from(resp))),
                        (FileId::Framework, KeyPath::new(["motion", k, "damping"]), Some(Value::from(damp))),
                    ],
                });
            }
            Some(CurveEvent::Bezier(b)) => {
                self.dragging = true;
                env.push(Action::Set {
                    label: t!("settings-motion-role-drag-bezier", role = self.role.title()),
                    file: FileId::Framework,
                    path: KeyPath::new(["motion", k, "easing"]),
                    value: Some(Value::from(bezier_text(b))),
                });
            }
            None => {}
        }
        if was && matches!(e.kind, PointerKind::Release(_)) {
            self.dragging = false;
            env.push(Action::Seal);
        }
        repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        if self.hidden { None } else { self.editor.cursor(p) }
    }

    fn sync(&mut self, env: &mut Env) {
        self.refresh_texts(env.store);
        let hidden = reduced_motion(env.store);
        if hidden != self.hidden {
            self.hidden = hidden;
            self.dragging = false;
            self.play_until = None;
            self.editor.set_preview(None);
            if hidden {
                let _ = self.hover.reset(env.now);
            } else {
                self.play(env.now);
            }
        }
        let (value, duration) = role_value(self.role, env.store);
        self.duration = duration;
        if self.last != Some(value) {
            self.last = Some(value);
            // Sürüklerken editör kendi değerini tutar; dışarıdan gelen değeri yalnızca o
            // zaman uygula (geri al, stil değişimi, dış düzenleme).
            if !self.dragging {
                self.editor.set_value(value);
            }
            self.play(env.now);
        }
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.title.set_style(text_style(theme, TextRole::Body));
        self.subtitle.set_style(text_style(theme, TextRole::Caption));
        self.summary.set_style(text_style(theme, TextRole::Caption));
        self.hint.set_style(text_style(theme, TextRole::Caption));
        self.hover.set_theme(theme, now);
        // Hız çarpanı ve stil temadan gelir: önizlemeyi yeni hareketle oynat.
        self.play(now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let now = ui.now;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let r = self.rect;
        if let Some(until) = self.play_until
            && now >= until
            && !self.hover.is_hovered()
        {
            self.play_until = None;
            self.editor.set_preview(None);
        }
        let th = self.title.measure_in(ui.painter).height;
        let y = r.y + t.space(3) + py;
        self.title.draw(ui.painter, Point::new(r.x + px, y), ui.fade(c.on_surface));
        self.subtitle.draw(ui.painter, Point::new(r.x + px, y + th + t.space(1)), ui.fade(c.on_surface_variant));
        let ss = self.summary.measure_in(ui.painter);
        self.summary.draw(ui.painter, Point::new(r.right() - px - ss.width, y + (th - ss.height) * 0.5), ui.fade(c.primary));
        if self.hidden {
            return;
        }
        self.editor.draw(ui);
        let er = self.editor.rect();
        self.hint.draw(ui.painter, Point::new(er.right() + t.space(6), er.y + t.space(4)), ui.fade(c.on_surface_variant));
        if self.hover.is_animating(now) {
            ui.request_frame();
        }
    }
}

#[cfg(test)]
mod tests {
    use nuxut::locale::Locale;
    use nuxut::{TextSystem, Theme};
    use toml_edit::Value;

    use super::RolePreview;
    use crate::block::{Block, Env};
    use crate::registry::{MotionRole, by_id};
    use crate::store::{FileId, KeyPath, Store};

    #[test]
    fn curve_editor_hidden_when_motion_off() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let desc = by_id("motion.spatial").unwrap();
        let mut row = RolePreview::new(&theme, desc, MotionRole::Spatial, &store);
        let now = std::time::Instant::now();
        let shown = {
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            row.height(&mut env, 900.0)
        };
        assert!(!row.hidden && row.play_until.is_some(), "açıkken önizleme oynar");
        // Kaydedilmemiş seçim (store) "Kapalı" → gizli, önizleme durur, daha kısa.
        store.set(FileId::Framework, &KeyPath::parse("motion.preset"), Some(Value::from("instant")));
        let hidden = {
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            row.sync(&mut env);
            row.height(&mut env, 900.0)
        };
        assert!(row.hidden && row.play_until.is_none());
        assert!(hidden < shown * 0.6, "{hidden} / {shown}");
        assert!(row.cursor(row.editor.rect().center()).is_none());
        // Ana anahtar da aynı sonucu verir; ön ayar açıkken enabled = false → gizli.
        store.set(FileId::Framework, &KeyPath::parse("motion.preset"), Some(Value::from("lively")));
        store.set(FileId::Framework, &KeyPath::parse("motion.enabled"), Some(Value::from(false)));
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        row.sync(&mut env);
        assert!(row.hidden);
        // Geri açılınca görünür ve yeniden oynar.
        store.set(FileId::Framework, &KeyPath::parse("motion.enabled"), None);
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        row.sync(&mut env);
        assert!(!row.hidden && row.play_until.is_some());
        assert_eq!(row.height(&mut env, 900.0), shown);
        // Kapalıyken açılan sayfa da gizli başlar.
        store.set(FileId::Framework, &KeyPath::parse("motion.preset"), Some(Value::from("instant")));
        assert!(RolePreview::new(&theme, desc, MotionRole::Spatial, &store).hidden);
    }

    #[test]
    fn spring_values_use_locale_separator() {
        // `fmt`'in biçimi: f32 → f64 dönüşümü yuvarlamayı bozmaz.
        let f = |v: f32, l: &str| crate::numfmt::format_in(&Locale::parse(l).unwrap(), f64::from(v), 2);
        assert_eq!(f(0.35, "tr_TR"), "0,35");
        assert_eq!(f(0.8, "tr_TR"), "0,8");
        assert_eq!(f(1.0, "tr_TR"), "1");
        assert_eq!(f(0.35, "en_US"), "0.35");
    }
}
