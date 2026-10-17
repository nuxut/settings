//! `org.nuxut.Calendar1` istemcisi (bkz. calendar `docs/ARCHITECTURE.md` §4).
//!
//! İki thread: **işçi** komutları sırayla yürütür (bloklayan zbus proxy'si), **dinleyici**
//! daemon sinyallerini alır (hesap/takvim listesi değişti → işçiye tazele; akış durumu →
//! doğrudan UI'ya). İkisi de ilk istekte açılır. Daemon yoksa (D-Bus etkinleştirme
//! başarısız) UI'ya `Unavailable` gider; sonraki istek yeniden bağlanmayı dener.

use std::sync::mpsc;
use std::sync::{Mutex, OnceLock};

use nuxut::i18n::t;
use nuxut_calendar_ipc::{AccountEntry, CalendarEntry, CalendarProxyBlocking, FlowInput, FlowState, INTERFACE, WriteFailureInfo};

use super::{Event, emit};

#[derive(Debug, Clone)]
pub enum Command {
    /// Hesap/takvim/sağlayıcı listesini yeniden oku.
    Refresh,
    /// Daemon'da süren akış varsa ona dön (`ListFlows`).
    Resume,
    /// Belirli akışı aç (`nuxut-settings --flow <id>`, webcal: aboneliği).
    Open(String),
    /// `""` = sağlayıcının rengine dön.
    SetCalendarColor { calendar: String, color: String },
    Begin { provider: String, reconnect: String },
    Submit { flow: String, input: FlowInput },
    Cancel(String),
    ReopenBrowser(String),
    SetCalendarEnabled { calendar: String, enabled: bool },
    SyncNow(String),
    Remove(String),
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub accounts: Vec<AccountEntry>,
    pub calendars: Vec<CalendarEntry>,
    pub providers: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Update {
    Snapshot(Box<Snapshot>),
    /// Akış başladı ya da (Ayarlar yeniden açılınca) bekleyen akış bulundu.
    FlowStarted(String),
    Flow { flow: String, state: Box<FlowState> },
    /// Akış artık yok (bitti/iptal/zaman aşımı).
    FlowGone(String),
    Unavailable(String),
    /// Bir işlem başarısız (kullanıcıya kısa mesaj).
    Failed(String),
}

static WORKER: OnceLock<Mutex<mpsc::Sender<Command>>> = OnceLock::new();

pub fn send(cmd: Command) {
    let tx = WORKER.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        let spawned = std::thread::Builder::new().name("calendar".into()).spawn(move || worker(rx));
        if let Err(e) = spawned {
            tracing::warn!("takvim thread'i açılamadı: {e}");
        }
        Mutex::new(tx)
    });
    if let Ok(tx) = tx.lock() {
        let _ = tx.send(cmd);
    }
}

fn update(u: Update) {
    emit(Event::Calendar(u));
}

struct Client {
    conn: zbus::blocking::Connection,
    proxy: CalendarProxyBlocking<'static>,
}

fn connect() -> zbus::Result<Client> {
    let conn = zbus::blocking::Connection::session()?;
    let proxy = CalendarProxyBlocking::new(&conn)?;
    Ok(Client { conn, proxy })
}

/// Kullanıcıya gösterilecek kısa, Türkçe hata (D-Bus ayrıntısı loga).
fn friendly(e: &zbus::Error) -> String {
    tracing::warn!("takvim servisi: {e}");
    let s = e.to_string();
    if s.contains("ServiceUnknown") || s.contains("NameHasNoOwner") || s.contains("was not provided by any .service files") {
        t!("settings-calendar-error-not-running")
    } else if s.contains("NoReply") || s.contains("Timeout") {
        t!("settings-calendar-error-timeout")
    } else if let Some(info) = WriteFailureInfo::from_zbus(e) {
        // Daemon iletisi "anahtar\nmetin" (calendar ADR-0007 §8): anahtar asla gösterilmez.
        if info.message.is_empty() { t!("settings-calendar-error-write") } else { info.message }
    } else {
        t!("settings-calendar-error-unreachable")
    }
}

fn worker(rx: mpsc::Receiver<Command>) {
    let mut client: Option<Client> = None;
    let mut listening = false;
    while let Ok(cmd) = rx.recv() {
        if client.is_none() {
            match connect() {
                Ok(c) => client = Some(c),
                Err(e) => {
                    update(Update::Unavailable(friendly(&e)));
                    continue;
                }
            }
        }
        let c = client.as_ref().expect("bağlandı");
        if !listening {
            listening = start_listener(&c.conn);
        }
        if let Err(e) = run(c, cmd) {
            let unavailable = matches!(&e, zbus::Error::MethodError(name, ..) if name.as_str().contains("ServiceUnknown"))
                || e.to_string().contains("ServiceUnknown");
            let msg = friendly(&e);
            if unavailable {
                update(Update::Unavailable(msg));
            } else {
                update(Update::Failed(msg));
            }
        }
    }
}

fn run(c: &Client, cmd: Command) -> zbus::Result<()> {
    let p = &c.proxy;
    match cmd {
        Command::Refresh => {
            let accounts = p.list_accounts()?;
            let calendars = p.list_calendars()?;
            let providers = p.available_providers()?;
            update(Update::Snapshot(Box::new(Snapshot { accounts, calendars, providers })));
        }
        Command::Resume => {
            // Ayarlar kapanıp açıldıysa daemon'da süren en yeni akışa dön (bitmişler hariç).
            for flow in p.list_flows()? {
                let state = p.get_flow(&flow)?;
                if state.step != nuxut_calendar_ipc::step::DONE {
                    update(Update::FlowStarted(flow.clone()));
                    update(Update::Flow { flow, state: Box::new(state) });
                    break;
                }
            }
        }
        Command::Open(flow) => {
            let state = p.get_flow(&flow)?;
            update(Update::FlowStarted(flow.clone()));
            update(Update::Flow { flow, state: Box::new(state) });
        }
        Command::SetCalendarColor { calendar, color } => p.set_calendar_color(&calendar, &color)?,
        Command::Begin { provider, reconnect } => {
            let flow = p.begin_add_account(&provider, &reconnect)?;
            update(Update::FlowStarted(flow.clone()));
            let state = p.get_flow(&flow)?;
            update(Update::Flow { flow, state: Box::new(state) });
        }
        Command::Submit { flow, input } => {
            p.submit_flow(&flow, input)?;
            let state = p.get_flow(&flow)?;
            update(Update::Flow { flow, state: Box::new(state) });
        }
        Command::Cancel(flow) => {
            let _ = p.cancel_flow(&flow);
            update(Update::FlowGone(flow));
        }
        Command::ReopenBrowser(flow) => p.reopen_browser(&flow)?,
        Command::SetCalendarEnabled { calendar, enabled } => p.set_calendar_enabled(&calendar, enabled)?,
        Command::SyncNow(account) => p.sync_now(&account)?,
        Command::Remove(account) => p.remove_account(&account)?,
    }
    Ok(())
}

/// Sinyal dinleyicisi: hesap/takvim değişimleri listeyi tazeler, akış değişimi UI'ya gider.
fn start_listener(conn: &zbus::blocking::Connection) -> bool {
    let rule = match zbus::MatchRule::builder().msg_type(zbus::message::Type::Signal).interface(INTERFACE).map(|b| b.build()) {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("takvim sinyal kuralı: {e}");
            return false;
        }
    };
    let iter = match zbus::blocking::MessageIterator::for_match_rule(rule, conn, Some(64)) {
        Ok(i) => i,
        Err(e) => {
            tracing::warn!("takvim sinyalleri dinlenemiyor: {e}");
            return false;
        }
    };
    let spawned = std::thread::Builder::new().name("calendar-signals".into()).spawn(move || {
        for msg in iter {
            let Ok(msg) = msg else { continue };
            let header = msg.header();
            let Some(member) = header.member() else { continue };
            match member.as_str() {
                "AccountsChanged" | "CalendarsChanged" => send(Command::Refresh),
                "FlowChanged" => {
                    if let Ok((flow, state)) = msg.body().deserialize::<(String, FlowState)>() {
                        update(Update::Flow { flow, state: Box::new(state) });
                    }
                }
                _ => {}
            }
        }
    });
    spawned.is_ok()
}
