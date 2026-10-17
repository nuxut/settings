//! Gece ışığının konum ve bugünün güneş olayları (`nuxut-shell msg night-light status`, shell
//! `docs/design/night-light.md`).
//!
//! Ayarlar yalnızca iki alanı okur (v1 = salt okunur konum satırı,
//! kararı 2026-10-13; bkz. `docs/design/night-light.md`):
//!
//! - `"location"`: `{"source": "manual"|"timezone"|"none", "lat", "lon"}` — yalnızca shell'in
//!   etkin zamanlaması `sun` iken dolu, yoksa `null`; `none` → `lat`/`lon` `null` (shell
//!   20:00–07:00 kullanıyor).
//! - `"today"`: `{"sunrise", "sunset" (unix sn ya da null), "polar": "day"|"night"|null}` —
//!   koordinat yoksa `null`.
//!
//! Diğer alanlar (`supported`, `status`, `next_change`…) yok sayılır. Yanıt güvenilmez girdi gibi
//! doğrulanır (enlem/boylam aralığı, sonlu sayı, bilinen kaynak adı). Shell çalışmıyor, komutu
//! tanımıyor (eski sürüm) ya da yanıt sözleşme dışıysa durum [`NightState::Unavailable`]: satır
//! gizlenir.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::shell_ipc::{IpcLink, LinkError, ShellLink};

/// Konumun kaynağı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationSource {
    /// `shell.toml [night_light] latitude/longitude`.
    Manual,
    /// Saat diliminin şehrinden yaklaşık (tzdata `zone1970.tab`).
    Timezone,
    /// Bulunamadı: shell 20:00–07:00 kullanıyor.
    None,
}

/// Shell'in kullandığı konum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub source: LocationSource,
    /// (enlem, boylam) derece; `None` yalnızca [`LocationSource::None`] ile.
    pub coords: Option<(f64, f64)>,
}

/// Bugünün güneş olayları.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SunToday {
    /// Unix saniye; olayın bugün olmadığı (ör. yüksek enlemde geçiş günü) uç `None` olabilir.
    Events { sunrise: Option<i64>, sunset: Option<i64> },
    /// Kutup günü: güneş batmıyor.
    PolarDay,
    /// Kutup gecesi: güneş doğmuyor.
    PolarNight,
}

/// `msg night-light status` yanıtının Ayarlar'ın kullandığı kısmı.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NightStatus {
    /// Shell'in etkin zamanlaması `sun` değilse `None`.
    pub location: Option<Location>,
    pub today: Option<SunToday>,
}

/// Shell'den okunan son durum.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum NightState {
    /// Henüz sorulmadı / yanıt bekleniyor.
    #[default]
    Pending,
    Ready(Arc<NightStatus>),
    /// Shell çalışmıyor, komutu tanımıyor (eski sürüm), test örneği ya da yanıt sözleşme dışı.
    Unavailable,
}

impl NightState {
    pub fn ready(&self) -> Option<&NightStatus> {
        match self {
            NightState::Ready(s) => Some(s),
            _ => None,
        }
    }
}

/// Unix saniyenin kabul edilen aralığı (1970..=2200; sözleşme dışı büyük sayı tarih
/// biçimleyiciye gitmez).
const MAX_UNIX: i64 = 7_258_118_400;

fn coord(v: Option<&serde_json::Value>, limit: f64) -> Option<f64> {
    v?.as_f64().filter(|x| x.is_finite() && x.abs() <= limit)
}

fn unix(v: Option<&serde_json::Value>) -> Result<Option<i64>, ()> {
    match v {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(x) => x.as_i64().filter(|t| (0..=MAX_UNIX).contains(t)).map(Some).ok_or(()),
    }
}

fn parse_location(v: &serde_json::Value) -> Option<Location> {
    let o = v.as_object()?;
    let source = match o.get("source")?.as_str()? {
        "manual" => LocationSource::Manual,
        "timezone" => LocationSource::Timezone,
        "none" => LocationSource::None,
        _ => return None,
    };
    let coords = match source {
        LocationSource::None => None,
        _ => Some((coord(o.get("lat"), 90.0)?, coord(o.get("lon"), 180.0)?)),
    };
    Some(Location { source, coords })
}

fn parse_today(v: &serde_json::Value) -> Option<SunToday> {
    let o = v.as_object()?;
    match o.get("polar").and_then(|p| p.as_str()) {
        Some("day") => return Some(SunToday::PolarDay),
        Some("night") => return Some(SunToday::PolarNight),
        Some(_) => return None,
        None => {}
    }
    let sunrise = unix(o.get("sunrise")).ok()?;
    let sunset = unix(o.get("sunset")).ok()?;
    Some(SunToday::Events { sunrise, sunset })
}

/// Yanıtı ayrıştırır; JSON nesnesi değilse `None`. `location`/`today` alanı yoksa (eski shell)
/// ya da sözleşme dışıysa o alan `None` (satır gizlenir).
pub fn parse_status(text: &str) -> Option<NightStatus> {
    let v: serde_json::Value = serde_json::from_str(text.trim()).ok()?;
    let o = v.as_object()?;
    let location = o.get("location").and_then(parse_location);
    // Konum yoksa bugünün olayları anlamsız (sözleşme: koordinat yoksa null).
    let today = location.filter(|l| l.coords.is_some()).and_then(|_| o.get("today")).and_then(parse_today);
    Some(NightStatus { location, today })
}

/// Tek sorgu (bloklar; yalnızca arka plan thread'inden).
pub fn query_status(link: &dyn ShellLink) -> NightState {
    match link.request(&["night-light", "status"]) {
        Ok(reply) => parse_status(&reply).map_or(NightState::Unavailable, |s| NightState::Ready(Arc::new(s))),
        Err(e) => {
            if matches!(e, LinkError::Timeout | LinkError::Io(_)) {
                tracing::debug!("night-light status yanıtı alınamadı: {e}");
            }
            NightState::Unavailable
        }
    }
}

static STATUS: Mutex<NightState> = Mutex::new(NightState::Pending);
/// (sorgu sürüyor, bittiğinde bir kez daha sor).
static QUERY: Mutex<(bool, bool)> = Mutex::new((false, false));
static ASKED: AtomicBool = AtomicBool::new(false);

/// Son durum.
pub fn status() -> NightState {
    STATUS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Henüz sorulmadıysa sorar (konum satırı kurulurken; sayfa doğrudan Ekranlar'da açıldıysa).
pub fn ensure() {
    if !ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Pencere odağı geri geldi: durum daha önce sorulduysa yeniden sorar (kullanıcı shell.toml'u
/// elle düzenleyip dönmüş olabilir). Yoklama yok.
pub fn recheck() {
    if ASKED.load(Ordering::Relaxed) {
        refresh(Duration::ZERO);
    }
}

/// Durumu `delay` sonra sorar (shell `shell.toml` değişimini uygulasın diye beklenir); bitince
/// [`Event::NightLight`](super::Event::NightLight). Sorgu sürüyorsa bittiğinde bir kez daha
/// sorulur. Bağlantı zaman aşımı `IpcLink` varsayılanı (2 sn).
pub fn refresh(delay: Duration) {
    // Birim testleri shell'e gitmez (ve arka plan thread'i açmaz).
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
    let spawned = std::thread::Builder::new().name("night-status".into()).spawn(move || {
        let link = IpcLink::from_env();
        let mut delay = delay;
        loop {
            std::thread::sleep(delay);
            let state = query_status(&link);
            *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = state;
            super::emit(super::Event::NightLight);
            let mut q = QUERY.lock().unwrap_or_else(|e| e.into_inner());
            if q.1 {
                q.1 = false;
                delay = Duration::from_millis(300);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::shell_ipc::FakeLink;

    #[test]
    fn parses_timezone_location_and_events() {
        let s = parse_status(
            r#"{"supported":true,"enabled":true,"schedule":"sun","status":"off","location":{"source":"timezone","lat":41.0,"lon":29.0},"today":{"sunrise":1760328840,"sunset":1760369520,"polar":null}}"#,
        )
        .unwrap();
        assert_eq!(s.location, Some(Location { source: LocationSource::Timezone, coords: Some((41.0, 29.0)) }));
        assert_eq!(s.today, Some(SunToday::Events { sunrise: Some(1760328840), sunset: Some(1760369520) }));
    }

    #[test]
    fn parses_none_polar_and_absent() {
        let none = parse_status(r#"{"location":{"source":"none","lat":null,"lon":null},"today":null}"#).unwrap();
        assert_eq!(none.location, Some(Location { source: LocationSource::None, coords: None }));
        assert_eq!(none.today, None);
        let polar = parse_status(r#"{"location":{"source":"manual","lat":78.2,"lon":15.6},"today":{"sunrise":null,"sunset":null,"polar":"night"}}"#).unwrap();
        assert_eq!(polar.location.unwrap().source, LocationSource::Manual);
        assert_eq!(polar.today, Some(SunToday::PolarNight));
        let day = parse_status(r#"{"location":{"source":"manual","lat":78.2,"lon":15.6},"today":{"sunrise":null,"sunset":null,"polar":"day"}}"#).unwrap();
        assert_eq!(day.today, Some(SunToday::PolarDay));
        // sun dışı zamanlama ya da eski shell (alan yok): konum yok.
        assert_eq!(parse_status(r#"{"schedule":"manual","location":null,"today":null}"#), Some(NightStatus::default()));
        assert_eq!(parse_status(r#"{"supported":false}"#), Some(NightStatus::default()));
    }

    #[test]
    fn rejects_out_of_contract_values() {
        // Aralık dışı koordinat, bilinmeyen kaynak, eksik koordinat: konum yok sayılır.
        for bad in [
            r#"{"location":{"source":"manual","lat":91.0,"lon":0.0}}"#,
            r#"{"location":{"source":"manual","lat":0.0,"lon":-181.0}}"#,
            r#"{"location":{"source":"gps","lat":1.0,"lon":1.0}}"#,
            r#"{"location":{"source":"timezone","lat":null,"lon":1.0}}"#,
        ] {
            assert_eq!(parse_status(bad).unwrap().location, None, "{bad}");
        }
        let bad_time = parse_status(r#"{"location":{"source":"manual","lat":1.0,"lon":1.0},"today":{"sunrise":-5,"sunset":null,"polar":null}}"#).unwrap();
        assert!(bad_time.location.is_some() && bad_time.today.is_none());
        // Konumsuzken today yok sayılır.
        let s = parse_status(r#"{"location":{"source":"none","lat":null,"lon":null},"today":{"sunrise":1,"sunset":2,"polar":null}}"#).unwrap();
        assert_eq!(s.today, None);
        assert_eq!(parse_status("error: bilinmeyen komut"), None);
        assert_eq!(parse_status("[1,2]"), None);
    }

    #[test]
    fn query_maps_errors_to_unavailable() {
        let link = FakeLink::with([Err(LinkError::Shell("bilinmeyen komut: night-light".into()))]);
        assert_eq!(query_status(&link), NightState::Unavailable);
        assert_eq!(link.sent(), vec![vec!["night-light".to_owned(), "status".to_owned()]]);
        assert_eq!(query_status(&FakeLink::with([])), NightState::Unavailable, "shell çalışmıyor");
        let ok = FakeLink::with([Ok(r#"{"location":{"source":"timezone","lat":41.0,"lon":29.0},"today":null}"#.to_owned())]);
        assert!(query_status(&ok).ready().is_some_and(|s| s.location.is_some()));
    }
}
