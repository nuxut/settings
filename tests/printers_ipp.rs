//! Yazıcılar servisi uçtan uca: istemci ↔ süreç içi sahte IPP sunucusu
//! (`tests/support/fake_cups.rs`). Sunucu her testte açıkça verilir; ortamdaki `CUPS_SERVER` ya
//! da gerçek CUPS kullanılmaz (deneme sayfası ve iptal yalnız sahte sunucuda).

#[path = "support/fake_cups.rs"]
mod fake_cups;

use fake_cups::{Addr, FakeCups, FakeState, demo};
use nuxut_settings::services::printers::cups::{self, CupsError, JobState, PrinterState, Server};
use nuxut_settings::services::printers::{Config, Op, OpError, lpoptions, perform, read_state};

const USER: &str = "deneme";

fn server(addr: &Addr) -> Server {
    match addr {
        Addr::Unix(p) => Server::Unix(p.clone()),
        Addr::Tcp(port) => Server::Tcp { host: "127.0.0.1".into(), port: *port },
    }
}

struct Env {
    dir: tempfile::TempDir,
    fake: FakeCups,
    cfg: Config,
}

fn setup(state: FakeState) -> Env {
    let dir = tempfile::tempdir().unwrap();
    let fake = FakeCups::new(state);
    let addr = fake.serve_unix(&dir.path().join("cups.sock"));
    let page = dir.path().join("testprint");
    std::fs::write(&page, b"#CUPS-BANNER\nTemplate standard\nShow printer-name printer-info\n").unwrap();
    let cfg = Config { server: server(&addr), user: USER.into(), lpoptions: Some(dir.path().join("home/.cups/lpoptions")), test_page: page };
    Env { dir, fake, cfg }
}

#[test]
fn lists_printers_default_and_own_jobs() {
    let env = setup(demo(USER));
    let st = read_state(&env.cfg).unwrap();
    let names: Vec<_> = st.printers.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["Etiket", "Ev_Renkli", "Ofis"], "görünen ada göre (Etiket'in açıklaması yok)");
    let ofis = st.printers.iter().find(|p| p.name == "Ofis").unwrap();
    assert_eq!((ofis.display_name(), ofis.state, ofis.model.as_str()), ("Ofis yazıcısı", PrinterState::Processing, "HP LaserJet Pro M404"));
    assert_eq!(ofis.uri, "ipp://localhost/printers/Ofis");
    let etiket = st.printers.iter().find(|p| p.name == "Etiket").unwrap();
    assert_eq!((etiket.state, etiket.accepting, etiket.reasons.as_slice()), (PrinterState::Stopped, false, &["paused".to_owned()][..]));
    assert_eq!(st.default.as_deref(), Some("Ofis"), "sunucu varsayılanı");
    let ids: Vec<_> = st.jobs.iter().map(|j| (j.id, j.state)).collect();
    assert_eq!(ids, [(41, JobState::Processing), (42, JobState::Pending)], "yalnız kendi işleri (chunked yanıt)");
    assert_eq!(st.jobs[1].printer, "Ofis");
    assert_eq!(st.jobs[1].size_kb, Some(2310));
}

#[test]
fn user_default_wins_and_is_written_to_lpoptions() {
    let env = setup(demo(USER));
    perform(&env.cfg, &Op::SetDefault("Ev_Renkli".into()), &read_state(&env.cfg).unwrap().printers).unwrap();
    let path = env.cfg.lpoptions.as_ref().unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), "Default Ev_Renkli\n");
    assert_eq!(read_state(&env.cfg).unwrap().default.as_deref(), Some("Ev_Renkli"));
    // Kullanıcının varsayılanı silinmiş kuyruğu gösterirse sunucununki.
    lpoptions::write_default(path, "Silinmis").unwrap();
    assert_eq!(read_state(&env.cfg).unwrap().default.as_deref(), Some("Ofis"));
    // Varsayılan yazımı CUPS'a gitmez (yönetici işlemi yok).
    assert!(!env.fake.state.lock().unwrap().ops.contains(&0x400A));
}

#[test]
fn test_page_reaches_fake_server_only() {
    let env = setup(demo(USER));
    let printers = read_state(&env.cfg).unwrap().printers;
    perform(&env.cfg, &Op::TestPage("Ev_Renkli".into(), "Deneme sayfası".into()), &printers).unwrap();
    let st = env.fake.state.lock().unwrap();
    let p = st.printed.last().unwrap();
    assert_eq!((p.printer.as_str(), p.title.as_str(), p.format.as_str(), p.user.as_str()), ("Ev_Renkli", "Deneme sayfası", cups::TEST_PAGE_FORMAT, USER));
    assert_eq!(p.bytes, std::fs::metadata(&env.cfg.test_page).unwrap().len() as usize, "belge gövdede, eksiksiz");
    drop(st);
    let jobs = read_state(&env.cfg).unwrap().jobs;
    assert!(jobs.iter().any(|j| j.id == 101 && j.name == "Deneme sayfası"), "yeni iş kuyrukta");
    // Kaldırılmış yazıcıya ya da belge yokken gönderilmez.
    assert_eq!(perform(&env.cfg, &Op::TestPage("Yok".into(), "t".into()), &printers), Err(OpError::NoPrinter));
    let mut cfg = env.cfg.clone();
    cfg.test_page = env.dir.path().join("yok");
    assert_eq!(perform(&cfg, &Op::TestPage("Ofis".into(), "t".into()), &printers), Err(OpError::Cups(CupsError::Document)));
}

#[test]
fn cancel_own_job_and_errors() {
    let env = setup(demo(USER));
    let printers = read_state(&env.cfg).unwrap().printers;
    perform(&env.cfg, &Op::Cancel(42), &printers).unwrap();
    let ids: Vec<_> = read_state(&env.cfg).unwrap().jobs.iter().map(|j| j.id).collect();
    assert_eq!(ids, [41]);
    // Zaten bitmiş / olmayan iş: amaç gerçekleşmiş sayılır.
    assert_eq!(perform(&env.cfg, &Op::Cancel(42), &printers), Ok(()));
    assert_eq!(perform(&env.cfg, &Op::Cancel(999), &printers), Ok(()));
    // Başkasının işi ve yetki reddi: "izin verilmedi".
    let e = perform(&env.cfg, &Op::Cancel(43), &printers).unwrap_err();
    assert!(matches!(&e, OpError::Cups(c) if c.is_denied()), "{e:?}");
    env.fake.state.lock().unwrap().deny_cancel = true;
    assert!(matches!(perform(&env.cfg, &Op::Cancel(41), &printers), Err(OpError::Cups(c)) if c.is_denied()));
}

#[test]
fn no_printers_and_no_default() {
    let env = setup(FakeState::default());
    let st = read_state(&env.cfg).unwrap();
    assert!(st.printers.is_empty() && st.default.is_none() && st.jobs.is_empty(), "not-found = boş liste, hata değil");
}

#[test]
fn tcp_transport() {
    let fake = FakeCups::new(demo(USER));
    let addr = fake.serve_tcp();
    let s = server(&addr);
    assert_eq!(cups::printers(&s).unwrap().len(), 3);
    assert_eq!(cups::server_default(&s).unwrap().as_deref(), Some("Ofis"));
    assert_eq!(cups::jobs(&s, "baska").unwrap().iter().map(|j| j.id).collect::<Vec<_>>(), [43]);
}

#[test]
fn missing_server_is_unreachable() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = Config { server: Server::Unix(dir.path().join("yok.sock")), user: USER.into(), lpoptions: None, test_page: dir.path().join("tp") };
    assert_eq!(read_state(&cfg), Err(CupsError::Unreachable));
    // Port dinlenmiyor (bağlantı reddedildi).
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    assert_eq!(cups::printers(&Server::Tcp { host: "127.0.0.1".into(), port }), Err(CupsError::Unreachable));
}

/// IPP gövdesini HTTP yanıtına sarar.
fn http(body: &[u8]) -> Vec<u8> {
    let mut v = format!("HTTP/1.1 200 OK\r\nContent-Type: application/ipp\r\nContent-Length: {}\r\n\r\n", body.len()).into_bytes();
    v.extend_from_slice(body);
    v
}

fn ipp_value(out: &mut Vec<u8>, tag: u8, name: &[u8], v: &[u8]) {
    out.push(tag);
    out.extend_from_slice(&(name.len() as u16).to_be_bytes());
    out.extend_from_slice(name);
    out.extend_from_slice(&(v.len() as u16).to_be_bytes());
    out.extend_from_slice(v);
}

/// Düşmanca sunucu (güvenlik bulgusu "resource-cap-defeat / integer overflow",
/// 2026-10-15): her yanıt hatayla biter; süreç düşmez, sınırsız bellek ayrılmaz.
#[test]
fn hostile_server_replies_are_rejected() {
    let env = setup(FakeState::default());
    let s = env.cfg.server.clone();
    let reply = |raw: Vec<u8>| {
        env.fake.state.lock().unwrap().raw = Some(raw);
        cups::printers(&s)
    };
    let m = Err(CupsError::Malformed);
    assert_eq!(reply(b"HTTP/1.1 200 OK\r\nContent-Length: 18446744073709551615\r\n\r\n".to_vec()), m);
    assert_eq!(reply(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nffffffffffffffff\r\nab".to_vec()), m);
    // Toplamı sınırı aşan parça akışı (2 MiB, 64 KiB'lık parçalar).
    let mut big = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n".to_vec();
    for _ in 0..32 {
        big.extend_from_slice(b"10000\r\n");
        big.extend(std::iter::repeat_n(b'x', 0x10000));
        big.extend_from_slice(b"\r\n");
    }
    assert_eq!(reply(big), m);
    let header = [1u8, 1, 0, 0, 0, 0, 0, 1];
    // u16::MAX değer uzunluğu, eksik gövde.
    let mut b = header.to_vec();
    b.extend_from_slice(&[0x04, 0x42, 0, 1, b'x', 0xff, 0xff, b'a', 0x03]);
    assert_eq!(reply(http(&b)), m);
    // Çok sayıda öznitelik.
    let mut b = header.to_vec();
    b.push(0x04);
    for _ in 0..=cups::MAX_IPP_VALUES {
        ipp_value(&mut b, 0x21, b"", &[0, 0, 0, 1]);
    }
    b.push(0x03);
    assert_eq!(reply(http(&b)), m);
    // Derin iç içe koleksiyon (özyinelemeli Drop yığını taşırırdı).
    let mut b = header.to_vec();
    b.push(0x04);
    ipp_value(&mut b, 0x34, b"c", b"");
    for _ in 0..50_000 {
        ipp_value(&mut b, 0x4a, b"", b"m");
        ipp_value(&mut b, 0x34, b"", b"");
    }
    for _ in 0..50_001 {
        ipp_value(&mut b, 0x37, b"", b"");
    }
    b.push(0x03);
    assert_eq!(reply(http(&b)), m);
    // Sınırın altındaki çok kuyruklu yanıt kesilir.
    let mut b = header.to_vec();
    for i in 0..(cups::MAX_PRINTERS + 5) {
        b.push(0x04);
        ipp_value(&mut b, 0x42, b"printer-name", format!("p{i}").as_bytes());
    }
    b.push(0x03);
    assert_eq!(reply(http(&b)).map(|p| p.len()), Ok(cups::MAX_PRINTERS));
}
