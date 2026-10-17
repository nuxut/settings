use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::*;
use crate::services::displays::{DisplayEdit, Vrr};

/// ADR-0005 §13.2 biçiminde `list` (shell'in ürettiği alanlarla).
const LIST: &str = r#"{"v":1,"gen":12,"provider":"lua","available":true,"pending":null,"primary":"A","displays":[
 {"id":"A","id_stable":true,"connector":"DP-1","make":"M","model":"A","serial":"1","description":"M A 1","physical_mm":[600,340],
  "enabled":true,"mode":{"w":1920,"h":1080,"mhz":60000},"pos":[0,0],"scale":1.0,"scale_120":120,"transform":0,
  "vrr":"fullscreen-media","vrr_active":false,"vrr_capable":null,"mirror_of":null,"logical":[1920,1080],"primary":true,
  "resolutions":[{"w":1920,"h":1080,"rates_mhz":[60000],"preferred":true,"scales":[1.0,1.25,1.5,2.0],"scales_120":[120,150,180,240]}]},
 {"id":"B","id_stable":true,"connector":"HDMI-A-1","make":"M","model":"B","serial":"2","description":"M B 2","physical_mm":[600,340],
  "enabled":true,"mode":{"w":1920,"h":1080,"mhz":60000},"pos":[1920,0],"scale":1.0,"scale_120":120,"transform":0,
  "vrr":"off","vrr_active":false,"vrr_capable":null,"mirror_of":null,"logical":[1920,1080],"primary":false,"resolutions":[]}]}"#;

/// Sahte shell: sıradaki cevapları verir, gelen istekleri kaydeder.
#[derive(Clone, Default)]
struct Fake {
    replies: Arc<Mutex<VecDeque<Result<String, LinkError>>>>,
    seen: Arc<Mutex<Vec<Vec<String>>>>,
}

impl Fake {
    fn push(&self, r: Result<&str, LinkError>) {
        self.replies.lock().unwrap().push_back(r.map(str::to_owned));
    }

    fn seen(&self) -> Vec<String> {
        self.seen.lock().unwrap().iter().map(|a| a[..a.len().min(2)].join(" ")).collect()
    }
}

impl ShellLink for Fake {
    fn request(&mut self, args: &[&str]) -> Result<String, LinkError> {
        self.seen.lock().unwrap().push(args.iter().map(|s| (*s).to_owned()).collect());
        self.replies.lock().unwrap().pop_front().unwrap_or(Err(LinkError::NotRunning))
    }
}

/// `status` JSON'u: bekleyen (jeton, kalan ms) ve son sonuç (jeton, sonuç).
fn status_json(g: u64, pending: Option<(&str, u64)>, last: Option<(&str, &str)>) -> String {
    let pending = pending.map_or("null".to_owned(), |(t, ms)| format!(r#"{{"token":"{t}","remaining_ms":{ms},"confirm":true}}"#));
    let last = last.map_or("null".to_owned(), |(t, r)| format!(r#"{{"token":"{t}","result":"{r}"}}"#));
    format!(r#"{{"v":1,"gen":{g},"provider":"lua","available":true,"pending":{pending},"last":{last}}}"#)
}

fn status(g: u64, pending: Option<(&str, u64)>, last: Option<(&str, &str)>) -> Status {
    Status::parse(&status_json(g, pending, last)).unwrap()
}

fn layout() -> Layout {
    Layout::from_list(&List::parse(LIST).unwrap())
}

fn edited() -> Layout {
    let mut l = layout();
    l.get_mut("A").unwrap().scale_120 = 240;
    l
}

/// Onay bekleyen akış (jeton `t1`).
fn awaiting() -> ApplyFlow {
    let mut flow = ApplyFlow::default();
    flow.on_status(&status(12, None, Some(("old", "confirmed"))));
    flow.begin(&edited()).unwrap();
    flow.on_reply(Ok(ApplyReply { token: "t1".into(), confirm: true, deadline_ms: 15000 }));
    flow
}

fn no_name(_: &str) -> Option<String> {
    None
}

// ---- Ayrıştırma -----------------------------------------------------------------------------

#[test]
fn reply_errors_are_split_into_code_and_message() {
    assert_eq!(split_reply("ok"), Ok("ok"));
    assert_eq!(split_reply("error: invalid_scale: A: 1.37 geçersiz, en yakın: 1.6"), Err(ShellError { code: "invalid_scale".into(), message: "A: 1.37 geçersiz, en yakın: 1.6".into() }));
    assert_eq!(split_reply("error: unavailable"), Err(ShellError { code: "unavailable".into(), message: String::new() }), "mesajsız kod");
    let old = split_reply("error: bilinmeyen komut \"displays\" (help ile listele)").unwrap_err();
    assert_eq!(old.code, "", "kodsuz hata: eski shell");
}

#[test]
fn status_follows_frozen_contract() {
    let s = Status::parse(r#"{"v":1,"gen":4,"provider":null,"available":false,"pending":{"token":"ab","remaining_ms":900,"confirm":true},"last":{"token":"9f","result":"timeout","detail":"x"}}"#).unwrap();
    assert_eq!((s.generation, s.provider.as_deref(), s.available), (4, None, false));
    assert_eq!(s.pending, Some(Pending { token: "ab".into(), remaining_ms: 900 }));
    assert_eq!(s.last, Some(Last { token: "9f".into(), result: Outcome::Timeout, detail: Some("x".into()) }));
    let results = ["applied", "confirmed", "reverted", "timeout", "hotplug", "not_applied", "not_shown", "reloaded", "error", "gelecekte"];
    let parsed: Vec<Outcome> = results.iter().map(|r| status(1, None, Some(("t", r))).last.unwrap().result).collect();
    use Outcome::*;
    assert_eq!(parsed, [Applied, Confirmed, Reverted, Timeout, Hotplug, NotApplied, NotShown, Reloaded, Other, Other], "bilinmeyen sonuç ayrıştırmayı bozmaz");
    assert!(Applied.kept() && Confirmed.kept() && !Timeout.kept() && !Hotplug.kept());
    assert!(Status::parse(r#"{"v":2,"gen":1}"#).is_none());
}

#[test]
fn list_uses_scale_120_and_kebab_vrr() {
    let list = List::parse(LIST).unwrap();
    assert_eq!(list.displays[0].vrr, Vrr::FullscreenMedia);
    assert_eq!(list.displays[0].scale_120(), 120);
    assert_eq!(list.displays[0].resolutions[0].scales_120, [120, 150, 180, 240]);
    let json = edited().apply_json().unwrap();
    assert!(json.contains(r#""vrr":"fullscreen-media""#), "{json}");
    assert!(json.contains(r#""scale_120":240"#) && !json.contains(r#""scale":"#), "ölçek her zaman scale_120: {json}");
    // Sözleşmedeki alanlar dışında alan yok (shell `deny_unknown_fields`).
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    let keys = |o: &serde_json::Value| o.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
    assert_eq!(keys(&v), ["base_gen", "displays", "primary", "v"]);
    assert_eq!(keys(&v["displays"][0]), ["enabled", "id", "mirror_of", "mode", "pos", "scale_120", "transform", "vrr"]);
}

#[test]
fn shell_instance_name_follows_shell_rule() {
    assert_eq!(shell_app_name(None).unwrap(), "nuxut-shell");
    assert_eq!(shell_app_name(Some("")).unwrap(), "nuxut-shell");
    assert_eq!(shell_app_name(Some("test-displays")).unwrap(), "nuxut-shell-test-displays");
    assert!(shell_app_name(Some("Kötü ad")).is_err(), "geçersiz ad varsayılan shell'e düşmez");
}

// ---- Yoklama --------------------------------------------------------------------------------

#[test]
fn poller_lists_on_generation_change_only() {
    let fake = Fake::default();
    let mut p = Poller::new(Box::new(fake.clone()));
    fake.push(Ok(&status_json(12, None, None)));
    fake.push(Ok(LIST));
    let u = p.tick(false);
    assert!(matches!(u[..], [Update::Status(_), Update::List(_)]), "{u:?}");
    fake.push(Ok(&status_json(12, None, None)));
    assert!(p.tick(false).is_empty(), "değişiklik yok: olay yok");
    fake.push(Ok(&status_json(13, None, None)));
    fake.push(Ok(LIST));
    assert_eq!(p.tick(false).len(), 2);
    assert_eq!(fake.seen(), ["displays status", "displays list", "displays status", "displays status", "displays list"]);
}

#[test]
fn poller_reports_missing_old_or_unavailable_shell_once() {
    let fake = Fake::default();
    let mut p = Poller::new(Box::new(fake.clone()));
    assert_eq!(p.tick(false), vec![Update::Unavailable(Unavailable::NotRunning)]);
    assert!(p.tick(false).is_empty(), "aynı durum tekrar yayılmaz");
    fake.push(Ok("error: bilinmeyen komut \"displays\" (help ile listele)"));
    assert_eq!(p.tick(false), vec![Update::Unavailable(Unavailable::Unsupported)]);
    fake.push(Ok(r#"{"v":2,"gen":1}"#));
    assert!(p.tick(false).is_empty(), "farklı sürüm de desteklenmiyor sayılır (değişmedi)");
    fake.push(Ok("error: unavailable: Hyprland yok"));
    assert_eq!(p.tick(false), vec![Update::Unavailable(Unavailable::Other("Hyprland yok".into()))]);
    fake.push(Ok(&status_json(12, None, None).replace(r#""available":true"#, r#""available":false"#)));
    assert_eq!(p.tick(false), vec![Update::Unavailable(Unavailable::Other(not_available()))], "available:false: ekranlar yönetilemez");
    fake.push(Err(LinkError::Timeout));
    assert!(matches!(&p.tick(false)[..], [Update::Unavailable(Unavailable::Other(m))] if m.contains("zamanında")));
    // Shell geri geldi: aynı gen olsa da liste alınır.
    fake.push(Ok(&status_json(12, None, None)));
    fake.push(Ok(LIST));
    assert!(matches!(p.tick(false).last(), Some(Update::List(_))));
}

#[test]
fn apply_request_is_one_argument_and_followed_by_status() {
    let fake = Fake::default();
    let mut p = Poller::new(Box::new(fake.clone()));
    let mut flow = ApplyFlow::default();
    let json = flow.begin(&edited()).unwrap();
    fake.push(Ok(r#"{"token":"t1","confirm":true,"deadline_ms":15000}"#));
    fake.push(Ok(&status_json(13, Some(("t1", 15000)), None)));
    fake.push(Ok(LIST));
    let u = p.apply(&json);
    let args = fake.seen.lock().unwrap()[0].clone();
    assert_eq!(args.len(), 3, "JSON tek argüman");
    assert_eq!(args[..2], ["displays", "apply"]);
    assert!(!args[2].contains(['\n', '\t']));
    assert!(args[2].len() <= 4000);
    assert!(args[2].starts_with(r#"{"v":1,"base_gen":12,"#));
    assert!(matches!(&u[0], Update::Applied(Ok(r)) if r.confirm && r.token == "t1"));
    assert!(matches!(&u[1], Update::Status(s) if s.pending.is_some()));
}

#[test]
fn decide_sends_token_and_always_reports_status() {
    let fake = Fake::default();
    let mut p = Poller::new(Box::new(fake.clone()));
    fake.push(Ok(&status_json(13, None, Some(("t1", "confirmed")))));
    fake.push(Ok(LIST));
    p.tick(false);
    // Durum değişmemiş olsa da `confirm` sonrası yayılır (akış sonucu ondan okur).
    fake.push(Ok("ok"));
    fake.push(Ok(&status_json(13, None, Some(("t1", "confirmed")))));
    let u = p.decide(true, "t1");
    assert_eq!(u[0], Update::Decided(Ok(())));
    assert!(matches!(&u[1], Update::Status(_)));
    fake.push(Ok("error: stale: onay bekleyen değişiklik yok"));
    fake.push(Ok(&status_json(13, None, Some(("t1", "confirmed")))));
    assert!(matches!(&p.decide(false, "t1")[0], Update::Decided(Err(e)) if e.code == "stale"));
    let seen = fake.seen.lock().unwrap().clone();
    assert_eq!(seen[2], ["displays", "confirm", "t1"]);
    assert_eq!(seen[4], ["displays", "revert", "t1"]);
}

#[test]
fn apply_errors_reach_the_page() {
    let fake = Fake::default();
    let mut p = Poller::new(Box::new(fake.clone()));
    fake.push(Ok("error: stale: güncel gen 13"));
    let u = p.apply("{}");
    assert!(matches!(&u[0], Update::Applied(Err(e)) if e.code == "stale" && e.needs_refresh()));
    let u = p.apply("{}");
    assert!(matches!(&u[0], Update::Applied(Err(e)) if e.code == "not_running"));
    fake.push(Err(LinkError::Timeout));
    let u = p.apply("{}");
    assert!(matches!(&u[0], Update::Applied(Err(e)) if e.code == "timeout"));
}

// ---- Akış (geri sayım durum makinesi) -------------------------------------------------------

#[test]
fn flow_keep_confirms_with_token() {
    let mut flow = awaiting();
    assert_eq!(flow.phase(), &Phase::Awaiting { token: "t1".into(), remaining_ms: 15000 }, "geri sayım deadline_ms'ten başlar");
    assert!(flow.begin(&edited()).is_err(), "meşgulken ikinci istek yok");
    flow.on_status(&status(13, Some(("t1", 9000)), Some(("old", "confirmed"))));
    assert_eq!(flow.phase(), &Phase::Awaiting { token: "t1".into(), remaining_ms: 9000 }, "geri sayım status'tan güncellenir");
    assert_eq!(flow.keep().as_deref(), Some("t1"));
    assert_eq!(flow.phase(), &Phase::Deciding { token: "t1".into(), keep: true });
    assert_eq!(flow.keep(), None, "ikinci tık bir şey göndermez");
    flow.on_decided(Ok(()));
    assert_eq!(flow.phase(), &Phase::Resolving { token: Some("t1".into()) });
    flow.on_status(&status(14, None, Some(("t1", "confirmed"))));
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::Confirmed, detail: None });
    assert!(!flow.busy());
    flow.dismiss();
    assert_eq!(flow.phase(), &Phase::Idle);
}

#[test]
fn flow_revert_and_shell_side_endings() {
    let mut flow = awaiting();
    assert_eq!(flow.revert().as_deref(), Some("t1"));
    flow.on_decided(Ok(()));
    flow.on_status(&status(14, None, Some(("t1", "reverted"))));
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::Reverted, detail: None });

    // Süre doldu: shell kendisi geri döndü; sonuç last'tan, ek açıklamasıyla.
    let mut flow = awaiting();
    let s = Status::parse(&status_json(14, None, None).replace(r#""last":null"#, r#""last":{"token":"t1","result":"timeout","detail":"bekçi dönecek"}"#)).unwrap();
    flow.on_status(&s);
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::Timeout, detail: Some("bekçi dönecek".into()) });

    // Hotplug.
    let mut flow = awaiting();
    flow.on_status(&status(14, None, Some(("t1", "hotplug"))));
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::Hotplug, detail: None });
}

#[test]
fn flow_late_keep_after_timeout_reads_last() {
    // Kullanıcı Koru'ya bastı ama süre o an doldu: confirm `stale` döner, sonuç last'tan.
    let mut flow = awaiting();
    flow.keep().unwrap();
    flow.on_status(&status(14, None, Some(("t1", "timeout"))));
    assert!(matches!(flow.phase(), Phase::Deciding { .. }), "karar cevabı gelmeden durum işlenmez");
    flow.on_decided(Err(ShellError { code: "stale".into(), message: "onay bekleyen değişiklik yok".into() }));
    flow.on_status(&status(14, None, Some(("t1", "timeout"))));
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::Timeout, detail: None });
}

#[test]
fn flow_decision_lost_in_transit_restores_buttons() {
    let mut flow = awaiting();
    flow.keep().unwrap();
    flow.on_decided(Err(ShellError { code: "timeout".into(), message: String::new() }));
    flow.on_status(&status(13, Some(("t1", 4000)), None));
    assert_eq!(flow.phase(), &Phase::Awaiting { token: "t1".into(), remaining_ms: 4000 }, "hâlâ bekliyor: Koru/Geri al yeniden");
    flow.keep().unwrap();
    flow.on_decided(Err(ShellError { code: "not_running".into(), message: String::new() }));
    assert!(matches!(flow.phase(), Phase::Failed(e) if e.code == "not_running"));
}

#[test]
fn flow_without_confirmation_and_errors() {
    let mut flow = ApplyFlow::default();
    flow.begin(&edited()).unwrap();
    flow.on_reply(Ok(ApplyReply { token: "t2".into(), confirm: false, deadline_ms: 0 }));
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::Applied, detail: None }, "confirm:false: uygulandı ve kalıcı");

    let mut flow = ApplyFlow::default();
    flow.begin(&edited()).unwrap();
    flow.on_reply(Err(ShellError { code: "invalid_scale".into(), message: "A: en yakın: 1.5".into() }));
    assert!(matches!(flow.phase(), Phase::Failed(e) if e.code == "invalid_scale"));
    assert!(!flow.busy());
    flow.begin(&edited()).unwrap();
    flow.on_lost();
    assert!(matches!(flow.phase(), Phase::Failed(e) if e.code == "not_running"), "shell kayboldu");
}

#[test]
fn flow_apply_timeout_is_resolved_from_status() {
    let timeout = || Err(ShellError { code: "timeout".into(), message: String::new() });
    // Shell isteği işlemiş, onay bekliyor: düğmeler gelir.
    let mut flow = ApplyFlow::default();
    flow.on_status(&status(12, None, Some(("old", "confirmed"))));
    flow.begin(&edited()).unwrap();
    flow.on_reply(timeout());
    assert_eq!(flow.phase(), &Phase::Resolving { token: None });
    flow.on_status(&status(13, Some(("t9", 14000)), Some(("old", "confirmed"))));
    assert_eq!(flow.phase(), &Phase::Awaiting { token: "t9".into(), remaining_ms: 14000 });

    // Onaysız değişiklik işlenmiş: yeni last.
    let mut flow = ApplyFlow::default();
    flow.on_status(&status(12, None, Some(("old", "confirmed"))));
    flow.begin(&edited()).unwrap();
    flow.on_reply(timeout());
    flow.on_status(&status(13, None, Some(("t9", "not_applied"))));
    assert_eq!(flow.phase(), &Phase::Done { outcome: Outcome::NotApplied, detail: None });

    // Hiçbir şey olmamış: eski last aynı → uygulanmadı.
    let mut flow = ApplyFlow::default();
    flow.on_status(&status(12, None, Some(("old", "confirmed"))));
    flow.begin(&edited()).unwrap();
    flow.on_reply(timeout());
    flow.on_status(&status(12, None, Some(("old", "confirmed"))));
    assert!(matches!(flow.phase(), Phase::Failed(e) if e.code == "timeout"));
}

#[test]
fn flow_is_limited_to_contract_size() {
    let mut big = layout();
    let tmpl = big.displays[0].clone();
    big.displays = (0..40).map(|i| DisplayEdit { id: format!("Manufacturer {i}|Model Name XYZ|SERIAL{i:06}"), ..tmpl.clone() }).collect();
    let mut flow = ApplyFlow::default();
    assert!(matches!(flow.begin(&big), Err(e) if e.code == "too_large"));
}

// ---- Kullanıcı metinleri --------------------------------------------------------------------

#[test]
fn every_contract_error_has_its_own_text() {
    let codes = [
        "bad_json", "too_large", "stale", "unknown_display", "missing_display", "invalid_mode", "invalid_scale", "invalid_position", "overlap",
        "disconnected_layout", "no_active_display", "mirror_chain", "invalid_primary", "pending_exists", "not_applied", "unavailable", "locked",
        "not_running", "timeout", "ipc", "bad_reply",
    ];
    let texts: Vec<String> = codes.iter().map(|c| error_text(&ShellError { code: (*c).into(), message: "ham mesaj".into() }, &no_name)).collect();
    for (c, t) in codes.iter().zip(&texts) {
        assert!(!t.contains("ham mesaj") && !t.contains(c), "{c}: kullanıcı metni ham değil: {t}");
    }
    assert_eq!(error_text(&ShellError { code: "gelecek_kod".into(), message: "shell açıklaması".into() }, &no_name), "shell açıklaması");
    assert_eq!(error_text(&ShellError { code: "gelecek_kod".into(), message: String::new() }, &no_name), "Uygulanamadı (gelecek_kod).");
    for c in ["stale", "unknown_display", "missing_display"] {
        assert!(ShellError { code: c.into(), message: String::new() }.needs_refresh());
    }
}

#[test]
fn error_text_names_the_display_and_suggests_scale() {
    let name = |id: &str| (id == "Dell Inc.|DELL U2720Q|ABC").then(|| "DELL U2720Q".to_owned());
    let e = ShellError { code: "invalid_scale".into(), message: "Dell Inc.|DELL U2720Q|ABC: ölçek 1.37 temiz değil, en yakın: 1.6".into() };
    assert_eq!(error_text(&e, &name), "DELL U2720Q: bu ölçek çözünürlüğü tam bölmüyor; en yakın uygun ölçek %160.");
    let e = ShellError { code: "invalid_mode".into(), message: "Bilinmeyen|X|1: kip yok".into() };
    assert_eq!(error_text(&e, &name), "Ekran: bu çözünürlük ya da yenileme hızı desteklenmiyor.");
    let e = ShellError { code: "unknown_display".into(), message: "Dell Inc.|DELL U2720Q|ABC: bağlı değil".into() };
    assert_eq!(error_text(&e, &name), "DELL U2720Q artık bağlı değil; liste yenilendi.");
}

#[test]
fn outcome_and_countdown_texts() {
    assert_eq!(countdown_text(15000), "Yeni ayarlar korunsun mu? 15 sn içinde geri alınacak.");
    assert_eq!(countdown_text(14001), "Yeni ayarlar korunsun mu? 15 sn içinde geri alınacak.", "yukarı yuvarlanır");
    assert_eq!(countdown_text(0), "Yeni ayarlar korunsun mu?");
    assert!(outcome_text(Outcome::Timeout, None).contains("Süre doldu"));
    assert!(outcome_text(Outcome::Hotplug, None).contains("takıldı"));
    assert!(outcome_text(Outcome::Reverted, Some("x")).ends_with("(x)"));
    assert_eq!(outcome_text(Outcome::Confirmed, Some("")), "Yeni ekran ayarları korundu.");
}

// ---- Sahte shell ----------------------------------------------------------------------------

#[test]
fn fake_shell_follows_contract_without_ipc() {
    let mut f = FakeShell::new(LIST).unwrap();
    let st = Status::parse(&f.request(&["displays", "status"]).unwrap()).unwrap();
    assert_eq!((st.generation, st.pending, st.last), (12, None, None));
    let r: ApplyReply = serde_json::from_str(&f.request(&["displays", "apply", &edited().apply_json().unwrap()]).unwrap()).unwrap();
    assert!(r.confirm && r.deadline_ms == 15000);
    let list = List::parse(&f.request(&["displays", "list"]).unwrap()).unwrap();
    assert_eq!((list.generation, list.displays[0].scale_120()), (13, 240), "değişiklik ekranda");
    assert!(f.request(&["displays", "apply", &edited().apply_json().unwrap()]).unwrap().starts_with("error: pending_exists"));
    assert_eq!(f.request(&["displays", "revert", &r.token]).unwrap(), "ok");
    let st = Status::parse(&f.request(&["displays", "status"]).unwrap()).unwrap();
    assert_eq!(st.last.map(|l| (l.token, l.result)), Some((r.token.clone(), Outcome::Reverted)));
    let list = List::parse(&f.request(&["displays", "list"]).unwrap()).unwrap();
    assert_eq!((list.generation, list.displays[0].scale_120()), (14, 120), "önceki duruma dönüldü");
    assert!(f.request(&["displays", "confirm", &r.token]).unwrap().starts_with("error: stale"));
    assert!(f.request(&["displays", "apply", &edited().apply_json().unwrap()]).unwrap().starts_with("error: stale"), "eski base_gen");

    // Süre dolması.
    let mut l = Layout::from_list(&list);
    l.get_mut("A").unwrap().scale_120 = 240;
    let r: ApplyReply = serde_json::from_str(&f.request(&["displays", "apply", &l.apply_json().unwrap()]).unwrap()).unwrap();
    f.pending.as_mut().unwrap().2 -= FAKE_CONFIRM;
    let st = Status::parse(&f.request(&["displays", "status"]).unwrap()).unwrap();
    assert_eq!((st.pending, st.last.map(|l| (l.token, l.result))), (None, Some((r.token, Outcome::Timeout))));
}
