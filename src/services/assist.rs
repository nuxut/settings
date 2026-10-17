//! Yardımcı araçların canlı durumu ve aç/kapa eylemleri: büyüteç (`nuxut-shell msg magnifier …`,
//! sözleşme magnifier-v1) ve ekran klavyesi (`msg osk …`, osk-v1). Sözleşmeler shell
//! `docs/design/magnifier.md` ve `docs/design/osk.md` "Ayarlar sözleşmesi".
//!
//! - Ayarlar Hyprland'e kendisi yazmaz; büyütmenin sahipliği kabuktadır. Aç/kapa yalnızca
//!   `magnifier on|off` ve `osk show|hide` ile.
//! - Yanıtlar güvenilmez girdi gibi doğrulanır; bilinmeyen alanlar yok sayılır (alanlar yalnızca
//!   eklenir). Kabuk çalışmıyor, komutu tanımıyor (eski sürüm) ya da yanıt sözleşme dışıysa durum
//!   [`Live::Unavailable`]; kabuk yanıt verip `available:false` derse [`Live::Ready`] ama araç
//!   bu masaüstünde kullanılamaz (Hyprland yok, compositor sanal klavye sunmuyor).
//! - Sorgu: sayfa açılınca, pencere odağı dönünce ve sayfa görünürken [`POLL`] arayla (kısayolla
//!   yapılan büyütme sayfada görünsün). Eylemden sonra hemen yeniden sorulur.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use super::shell_ipc::{IpcLink, LinkError, ShellLink};

/// Sayfa görünürken yeniden sorma aralığı.
pub const POLL: Duration = Duration::from_secs(2);

/// Hangi araç.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Magnifier,
    Osk,
}

/// `msg magnifier status` yanıtının kullanılan kısmı.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Magnifier {
    pub available: bool,
    /// Hyprland'in canlı çarpanı (`available:false` iken `None`).
    pub factor: Option<f64>,
    pub active: bool,
}

/// Ekran klavyesinin düzeni (osk-v1 `layout`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OskLayout {
    Tr,
    Us,
}

/// `msg osk status` yanıtının kullanılan kısmı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Osk {
    pub available: bool,
    pub visible: bool,
    pub layout: Option<OskLayout>,
}

/// Bir aracın son bilinen durumu.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Live<T> {
    /// Henüz sorulmadı / yanıt bekleniyor.
    #[default]
    Pending,
    Ready(T),
    /// Kabuk çalışmıyor, komutu tanımıyor ya da yanıt sözleşme dışı.
    Unavailable,
}

impl<T: Copy> Live<T> {
    pub fn ready(&self) -> Option<T> {
        match self {
            Live::Ready(s) => Some(*s),
            _ => None,
        }
    }
}

/// İki aracın durumu ve süren eylem.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Snapshot {
    pub magnifier: Live<Magnifier>,
    pub osk: Live<Osk>,
    /// Eylem sürüyor (anahtar kullanıcının istediği konumda bekler).
    pub busy: Option<Tool>,
    /// Son eylem kabukta başarısız oldu (satırda kısa hata; sonraki sorguda silinir).
    pub failed: Option<Tool>,
}

fn object(text: &str) -> Option<serde_json::Map<String, serde_json::Value>> {
    match serde_json::from_str::<serde_json::Value>(text.trim()).ok()? {
        serde_json::Value::Object(o) => Some(o),
        _ => None,
    }
}

/// `magnifier status` (magnifier-v1). `available` zorunlu; açıkken çarpan sonlu ve 1..=10.
pub fn parse_magnifier(text: &str) -> Option<Magnifier> {
    let o = object(text)?;
    let available = o.get("available")?.as_bool()?;
    if !available {
        return Some(Magnifier { available, factor: None, active: false });
    }
    let factor = o.get("factor")?.as_f64().filter(|f| f.is_finite() && (1.0..=10.0).contains(f))?;
    let active = o.get("active").and_then(|a| a.as_bool()).unwrap_or(factor > 1.0);
    Some(Magnifier { available, factor: Some(factor), active })
}

/// `osk status` (osk-v1). Bilinmeyen düzen adı `None` (satır düzeni göstermez).
pub fn parse_osk(text: &str) -> Option<Osk> {
    let o = object(text)?;
    let available = o.get("available")?.as_bool()?;
    let visible = available && o.get("visible").and_then(|v| v.as_bool()).unwrap_or(false);
    let layout = match o.get("layout").and_then(|l| l.as_str()) {
        Some("tr") => Some(OskLayout::Tr),
        Some("us") => Some(OskLayout::Us),
        _ => None,
    };
    Some(Osk { available, visible, layout })
}

fn status_args(tool: Tool) -> [&'static str; 2] {
    match tool {
        Tool::Magnifier => ["magnifier", "status"],
        Tool::Osk => ["osk", "status"],
    }
}

/// Aç/kapa komutu: büyüteç `on|off`, klavye `show|hide`.
pub fn action_args(tool: Tool, on: bool) -> [&'static str; 2] {
    match (tool, on) {
        (Tool::Magnifier, true) => ["magnifier", "on"],
        (Tool::Magnifier, false) => ["magnifier", "off"],
        (Tool::Osk, true) => ["osk", "show"],
        (Tool::Osk, false) => ["osk", "hide"],
    }
}

fn query<T>(link: &dyn ShellLink, tool: Tool, parse: fn(&str) -> Option<T>) -> Live<T> {
    match link.request(&status_args(tool)) {
        Ok(reply) => parse(&reply).map_or(Live::Unavailable, Live::Ready),
        Err(e) => {
            if matches!(e, LinkError::Timeout | LinkError::Io(_)) {
                tracing::debug!("{:?} durumu alınamadı: {e}", tool);
            }
            Live::Unavailable
        }
    }
}

/// İki aracı sorar (bloklar; yalnızca arka plan thread'inden).
pub fn query_both(link: &dyn ShellLink) -> (Live<Magnifier>, Live<Osk>) {
    (query(link, Tool::Magnifier, parse_magnifier), query(link, Tool::Osk, parse_osk))
}

/// Eylemi gönderir; kabuk `ok` dışında yanıt verirse `false` (tanı loglanır).
pub fn send_action(link: &dyn ShellLink, tool: Tool, on: bool) -> bool {
    match link.request(&action_args(tool, on)) {
        Ok(_) => true,
        Err(e) => {
            tracing::warn!("{:?} eylemi başarısız: {e}", tool);
            false
        }
    }
}

static STATE: Mutex<Snapshot> = Mutex::new(Snapshot { magnifier: Live::Pending, osk: Live::Pending, busy: None, failed: None });
/// (sorgu sürüyor, bittiğinde bir kez daha sor).
static QUERY: Mutex<(bool, bool)> = Mutex::new((false, false));
static ASKED: AtomicBool = AtomicBool::new(false);
static VISIBLE: AtomicBool = AtomicBool::new(false);
static POLLING: AtomicBool = AtomicBool::new(false);

fn state() -> std::sync::MutexGuard<'static, Snapshot> {
    STATE.lock().unwrap_or_else(|e| e.into_inner())
}

/// Son durum.
pub fn snapshot() -> Snapshot {
    *state()
}

/// Henüz sorulmadıysa sorar (satır kurulurken).
pub fn ensure() {
    if !ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Pencere odağı geri geldi: daha önce sorulduysa yeniden sorar.
pub fn recheck() {
    if ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Erişilebilirlik sayfası görünür mü: görünürken [`POLL`] arayla sorulur (kısayolla yapılan
/// büyütme ya da kabuktan kapanan klavye sayfada görünsün); gizliyken yoklama durur.
pub fn set_page_visible(visible: bool) {
    VISIBLE.store(visible, Ordering::Relaxed);
    if !visible || cfg!(test) || POLLING.swap(true, Ordering::Relaxed) {
        return;
    }
    let spawned = std::thread::Builder::new().name("assist-poll".into()).spawn(|| {
        while VISIBLE.load(Ordering::Relaxed) {
            std::thread::sleep(POLL);
            if VISIBLE.load(Ordering::Relaxed) {
                refresh(Duration::ZERO);
            }
        }
        POLLING.store(false, Ordering::Relaxed);
    });
    if spawned.is_err() {
        POLLING.store(false, Ordering::Relaxed);
    }
}

/// Durumu `delay` sonra sorar; bitince [`Event::Assist`](super::Event::Assist) — yalnızca
/// durum değiştiyse. Sorgu sürüyorsa bittiğinde bir kez daha sorulur.
pub fn refresh(delay: Duration) {
    // Birim testleri kabuğa gitmez (ve arka plan thread'i açmaz).
    if cfg!(test) {
        return;
    }
    ASKED.store(true, Ordering::Relaxed);
    let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
    if q.0 {
        q.1 = true;
        return;
    }
    q.0 = true;
    let spawned = std::thread::Builder::new().name("assist-status".into()).spawn(move || {
        let link = IpcLink::from_env();
        let mut delay = delay;
        loop {
            std::thread::sleep(delay);
            let (magnifier, osk) = query_both(&link);
            let changed = {
                let mut s = state();
                let before = *s;
                s.magnifier = magnifier;
                s.osk = osk;
                if s.busy.is_none() {
                    s.failed = None;
                }
                *s != before
            };
            if changed {
                super::emit(super::Event::Assist);
            }
            let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
            if q.1 {
                q.1 = false;
                delay = Duration::from_millis(150);
                continue;
            }
            q.0 = false;
            break;
        }
    });
    if spawned.is_err() {
        q.0 = false;
    }
}

/// Aracı açar/kapatır (arka planda); bitince durum yeniden sorulur. Başka eylem sürüyorsa yok sayılır.
pub fn set(tool: Tool, on: bool) {
    if cfg!(test) {
        return;
    }
    {
        let mut s = state();
        if s.busy.is_some() {
            return;
        }
        s.busy = Some(tool);
        s.failed = None;
    }
    super::emit(super::Event::Assist);
    let spawned = std::thread::Builder::new().name("assist-act".into()).spawn(move || {
        let ok = send_action(&IpcLink::from_env(), tool, on);
        {
            let mut s = state();
            s.busy = None;
            s.failed = (!ok).then_some(tool);
        }
        super::emit(super::Event::Assist);
        refresh(Duration::ZERO);
    });
    if spawned.is_err() {
        state().busy = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::shell_ipc::FakeLink;

    #[test]
    fn parses_magnifier_v1() {
        let on = parse_magnifier(r#"{"v":1,"available":true,"factor":2.5,"active":true,"managed":true,"step":1.25,"max":8,"reason":null,"new_field":1}"#).unwrap();
        assert_eq!(on, Magnifier { available: true, factor: Some(2.5), active: true });
        let off = parse_magnifier(r#"{"v":1,"available":true,"factor":1.0,"active":false,"managed":false,"step":1.25,"max":8,"reason":null}"#).unwrap();
        assert!(!off.active && off.factor == Some(1.0));
        let none = parse_magnifier(r#"{"v":1,"available":false,"factor":null,"active":false,"managed":false,"step":1.25,"max":8,"reason":"no hyprland"}"#).unwrap();
        assert_eq!(none, Magnifier { available: false, factor: None, active: false });
        // `active` yoksa çarpandan.
        assert!(parse_magnifier(r#"{"available":true,"factor":3}"#).unwrap().active);
    }

    #[test]
    fn rejects_out_of_contract_magnifier() {
        for bad in [r#"{"factor":2}"#, r#"{"available":true,"factor":null}"#, r#"{"available":true,"factor":11}"#, r#"{"available":true,"factor":0.5}"#, "ok", "[1]", "error: bilinmeyen komut"] {
            assert_eq!(parse_magnifier(bad), None, "{bad}");
        }
    }

    #[test]
    fn parses_osk_v1() {
        assert_eq!(parse_osk(r#"{"available":true,"visible":true,"layout":"tr","output":"DP-1"}"#), Some(Osk { available: true, visible: true, layout: Some(OskLayout::Tr) }));
        assert_eq!(parse_osk(r#"{"available":true,"visible":false,"layout":"us","output":null}"#).unwrap().layout, Some(OskLayout::Us));
        assert_eq!(parse_osk(r#"{"available":true,"visible":false,"layout":"de"}"#).unwrap().layout, None);
        // Kullanılamıyorsa görünür sayılmaz.
        assert!(!parse_osk(r#"{"available":false,"visible":true}"#).unwrap().visible);
        assert_eq!(parse_osk(r#"{"visible":true}"#), None);
    }

    #[test]
    fn queries_both_and_maps_errors() {
        let link = FakeLink::with([Ok(r#"{"available":true,"factor":2,"active":true}"#.to_owned()), Err(LinkError::Shell("bilinmeyen komut: osk".into()))]);
        let (m, o) = query_both(&link);
        assert!(m.ready().is_some_and(|m| m.active));
        assert_eq!(o, Live::Unavailable, "eski kabuk");
        assert_eq!(link.sent(), vec![vec!["magnifier".to_owned(), "status".to_owned()], vec!["osk".to_owned(), "status".to_owned()]]);
        assert_eq!(query_both(&FakeLink::with([])), (Live::Unavailable, Live::Unavailable), "kabuk çalışmıyor");
    }

    #[test]
    fn actions_use_contract_commands() {
        assert_eq!(action_args(Tool::Magnifier, true), ["magnifier", "on"]);
        assert_eq!(action_args(Tool::Magnifier, false), ["magnifier", "off"]);
        assert_eq!(action_args(Tool::Osk, true), ["osk", "show"]);
        assert_eq!(action_args(Tool::Osk, false), ["osk", "hide"]);
        let link = FakeLink::with([Ok("ok".to_owned()), Err(LinkError::Shell("error: no hyprland".into()))]);
        assert!(send_action(&link, Tool::Magnifier, true));
        assert!(!send_action(&link, Tool::Osk, false));
    }
}
