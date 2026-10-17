//! Erişilebilirlik: büyütecin ve ekran klavyesinin canlı aç/kapa satırı. Durum kabuktan
//! ([`crate::services::assist`]); satır dosyaya yazmaz, kabuğa `msg magnifier on|off` /
//! `msg osk show|hide` gönderir (Hyprland'e doğrudan değil — büyütmenin sahibi kabuk).
//!
//! Alt metin: açıkken "Açık · 2× büyütme" / "Açık · Türkçe Q", kapalıyken "Kapalı"; kabuk
//! yanıt verip aracın bu masaüstünde çalışmadığını söylerse ya da kabuk yoksa anahtar devre
//! dışı ve neden alt metinde. Eylem sürerken anahtar istenen konumda bekler.

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, Key, KeyEvent, KeyState, PointerEvent};
use nuxut::ui::{Interaction, Toggle, Ui};
use nuxut::{Corners, Point, Rect, Theme};

use super::datetime::RowText;
use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{row_height_for, row_padding};
use crate::services::assist::{self, Live, OskLayout, Snapshot, Tool};

/// Satırın gösterdiği durum (saf; testli).
#[derive(Debug, Clone, PartialEq)]
pub struct View {
    pub on: bool,
    pub enabled: bool,
    pub subtitle: String,
    pub error: bool,
}

fn layout_label(layout: OskLayout) -> String {
    match layout {
        OskLayout::Tr => t!("settings-choice-osk-layout-tr"),
        OskLayout::Us => t!("settings-choice-osk-layout-us"),
    }
}

/// `(çalışıyor mu, açık mı, açıkken alt metin)`; `None` = yanıt yok / kabuk yok.
fn tool_state(s: &Snapshot, tool: Tool) -> Live<(bool, bool, String)> {
    let map = |m: assist::Magnifier| {
        let on_text = t!("settings-assist-magnifier-on", factor = crate::numfmt::format(m.factor.unwrap_or(1.0), 2));
        (m.available, m.active, on_text)
    };
    let map_osk = |o: assist::Osk| {
        let on_text = match o.layout {
            Some(l) => t!("settings-assist-osk-on", layout = layout_label(l)),
            None => t!("settings-assist-on"),
        };
        (o.available, o.visible, on_text)
    };
    match tool {
        Tool::Magnifier => match s.magnifier {
            Live::Ready(m) => Live::Ready(map(m)),
            Live::Pending => Live::Pending,
            Live::Unavailable => Live::Unavailable,
        },
        Tool::Osk => match s.osk {
            Live::Ready(o) => Live::Ready(map_osk(o)),
            Live::Pending => Live::Pending,
            Live::Unavailable => Live::Unavailable,
        },
    }
}

/// Anlık görüntüden satırın görünümü. `wanted`: eylem sürerken kullanıcının istediği konum.
pub fn view(s: &Snapshot, tool: Tool, idle_subtitle: &str, wanted: bool) -> View {
    let busy = s.busy.is_some();
    match tool_state(s, tool) {
        Live::Pending => View { on: wanted, enabled: false, subtitle: idle_subtitle.to_owned(), error: false },
        Live::Unavailable => View { on: false, enabled: false, subtitle: t!("settings-assist-no-shell"), error: false },
        Live::Ready((false, _, _)) => View { on: false, enabled: false, subtitle: t!("settings-assist-unsupported"), error: false },
        Live::Ready((true, active, on_text)) => {
            if s.busy == Some(tool) {
                return View { on: wanted, enabled: false, subtitle: t!("settings-assist-waiting"), error: false };
            }
            if s.failed == Some(tool) {
                return View { on: active, enabled: !busy, subtitle: t!("settings-assist-failed"), error: true };
            }
            let subtitle = if active { on_text } else { t!("settings-assist-off") };
            View { on: active, enabled: !busy, subtitle, error: false }
        }
    }
}

#[derive(Debug)]
pub struct AssistLive {
    desc: &'static SettingDesc,
    tool: Tool,
    text: RowText,
    toggle: Toggle,
    hover: Interaction,
    enabled: bool,
    rect: Rect,
}

impl AssistLive {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, tool: Tool) -> Self {
        assist::ensure();
        let mut b = Self {
            desc,
            tool,
            text: RowText::new(theme, desc.label()),
            toggle: Toggle::new(theme, false),
            hover: Interaction::new(theme),
            enabled: false,
            rect: Rect::ZERO,
        };
        b.update(Instant::now());
        b
    }

    fn update(&mut self, now: Instant) {
        let v = view(&assist::snapshot(), self.tool, self.desc.description(), self.toggle.is_on());
        self.text.set_subtitle(&v.subtitle, v.error);
        self.toggle.set_on(v.on, now);
        self.toggle.set_enabled(v.enabled, now);
        self.enabled = v.enabled;
    }

    fn flip(&mut self, now: Instant) {
        if !self.enabled {
            return;
        }
        let on = !self.toggle.is_on();
        self.toggle.set_on(on, now);
        assist::set(self.tool, on);
    }
}

impl Block for AssistLive {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        let text = self.text.height(env.text, t, (width - px * 3.0 - ts.width).max(t.space(20)));
        row_height_for(t, text.max(ts.height))
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        let t = env.theme;
        let (px, _) = row_padding(t);
        let ts = self.toggle.preferred_size(t);
        self.toggle.set_rect(Rect::new(rect.right() - px - ts.width, rect.center().y - ts.height * 0.5, ts.width, ts.height));
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        // Satırın tamamı tıklanabilir (standart anahtar satırı gibi).
        let tap = self.hover.handle(self.rect, e, env.now);
        if tap.clicked {
            self.flip(env.now);
            return true;
        }
        tap.repaint
    }

    fn key(&mut self, env: &mut Env, e: &KeyEvent) -> bool {
        if e.state != KeyState::Released && matches!(e.key, Key::Enter | Key::Space) {
            self.flip(env.now);
            return true;
        }
        false
    }

    fn focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool, visible: bool, _now: Instant) {
        self.toggle.set_focus_visible(focused && visible);
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        (self.enabled && self.rect.contains(p)).then_some(CursorShape::Pointer)
    }

    fn sync(&mut self, env: &mut Env) {
        self.update(env.now);
    }

    fn service_event(&mut self, env: &mut Env, event: &crate::services::Event) -> bool {
        if matches!(event, crate::services::Event::Assist | crate::services::Event::LocaleChanged) {
            self.update(env.now);
            return true;
        }
        false
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.text.set_theme(theme);
        self.toggle.set_theme(theme, now);
        self.hover.set_theme(theme, now);
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let (px, _) = row_padding(t);
        if self.enabled {
            self.hover.draw_layer(ui.painter, self.rect, Corners::uniform(t.shape.radius_medium * 0.5), t, ui.now);
            if self.hover.is_animating(ui.now) {
                ui.request_frame();
            }
        }
        let h = self.text.drawn_height(ui);
        self.text.draw(ui, self.rect.x + px, self.rect.center().y - h * 0.5);
        self.toggle.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::assist::{Magnifier, Osk};

    fn snap(m: Live<Magnifier>, o: Live<Osk>) -> Snapshot {
        Snapshot { magnifier: m, osk: o, busy: None, failed: None }
    }

    #[test]
    fn magnifier_view_follows_live_state() {
        crate::i18n::use_turkish_for_tests();
        let on = snap(Live::Ready(Magnifier { available: true, factor: Some(2.5), active: true }), Live::Pending);
        let v = view(&on, Tool::Magnifier, "", false);
        assert!(v.on && v.enabled && !v.error);
        assert!(v.subtitle.contains("2,5"), "{}", v.subtitle);
        let off = snap(Live::Ready(Magnifier { available: true, factor: Some(1.0), active: false }), Live::Pending);
        assert_eq!(view(&off, Tool::Magnifier, "", true).subtitle, "Kapalı");
        let no_hypr = snap(Live::Ready(Magnifier { available: false, factor: None, active: false }), Live::Pending);
        let v = view(&no_hypr, Tool::Magnifier, "", false);
        assert!(!v.enabled && !v.on && v.subtitle == "Bu masaüstünde kullanılamıyor");
        let v = view(&snap(Live::Unavailable, Live::Unavailable), Tool::Magnifier, "", false);
        assert!(!v.enabled && v.subtitle == "Masaüstü kabuğu çalışmıyor");
        // Henüz yanıt yok: kayıttaki açıklama, anahtar kapalı değil ama dokunulmaz.
        let v = view(&Snapshot::default(), Tool::Magnifier, "açıklama", false);
        assert!(!v.enabled && v.subtitle == "açıklama");
    }

    #[test]
    fn osk_view_shows_layout_and_busy_state() {
        crate::i18n::use_turkish_for_tests();
        let mut s = snap(Live::Pending, Live::Ready(Osk { available: true, visible: true, layout: Some(OskLayout::Tr) }));
        let v = view(&s, Tool::Osk, "", false);
        assert!(v.on && v.subtitle.contains("Türkçe Q"), "{}", v.subtitle);
        // Eylem sürerken anahtar istenen konumda, dokunulmaz.
        s.busy = Some(Tool::Osk);
        let v = view(&s, Tool::Osk, "", false);
        assert!(!v.on && !v.enabled && v.subtitle == "Bekleniyor…");
        // Başka aracın eylemi sürerken bu satır da beklemede (tek eylem).
        let mut m = snap(Live::Ready(Magnifier { available: true, factor: Some(1.0), active: false }), s.osk);
        m.busy = Some(Tool::Osk);
        assert!(!view(&m, Tool::Magnifier, "", false).enabled);
        // Başarısız eylem: hata metni, gerçek durum.
        s.busy = None;
        s.failed = Some(Tool::Osk);
        let v = view(&s, Tool::Osk, "", false);
        assert!(v.error && v.on && v.subtitle == "Değiştirilemedi");
    }
}
