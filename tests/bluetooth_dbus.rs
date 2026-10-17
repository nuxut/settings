//! Bluetooth servisinin ve sayfa içeriğinin uçtan uca testi: `nuxut-bluez` crate'inin paylaşılan
//! sahte BlueZ'i (`tests/fake_bluez`; kopya yok) bu süreçte, YALNIZCA `dbus-run-session`'ın
//! oturum bus'ında barındırılır. Testler `#[ignore]`: düz `cargo test` çalıştırmaz;
//! `tools/bluetooth-dbus-test.sh` her senaryoyu ayrı süreçte çalıştırır (servis durumu süreç
//! geneli). `NUXUT_BLUEZ_BUS=session` yoksa ya da bus kullanıcının oturum bus'ıysa testler hiçbir
//! şey yapmadan döner: gerçek sistem BlueZ'ine asla gidilmez.
//!
//! Her senaryo sonunda: Ayarlar eşleştirme ajanı kaydetmedi (`RegisterAgent` /
//! `RequestDefaultAgent` yok — karar, 2026-10-13).

#[path = "../../shell/crates/nuxut-bluez/tests/fake_bluez/mod.rs"]
mod fake_bluez;

use std::time::{Duration, Instant};

use fake_bluez::{FakeBluez, Log, PairError, Scene};
use nuxut_bluez::{BluezError, PowerState, Unavailable};
use nuxut_settings::pages::bluetooth::{Content, RowKind, content, discoverable_text, unavailable_text};
use nuxut_settings::registry::BtList;
use nuxut_settings::services::bluetooth::{self, BtState, PageStatus, Power, Snapshot, page_status, power};

const HCI0: &str = "/org/bluez/hci0";

fn dev(id: &str) -> String {
    format!("{HCI0}/{id}")
}

/// Yalnız yalıtılmış test bus'ında (betik `NUXUT_BLUEZ_BUS=session` + dbus-run-session kurar).
fn isolated() -> bool {
    nuxut_settings::i18n::use_turkish_for_tests();
    let session = std::env::var("NUXUT_BLUEZ_BUS").is_ok_and(|v| v == "session");
    let address = std::env::var("DBUS_SESSION_BUS_ADDRESS").unwrap_or_default();
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/".into());
    let user_bus = address.is_empty() || address.contains(&format!("{runtime}/bus")) || address.contains("/run/user/");
    if !session || user_bus {
        eprintln!("atlandı: NUXUT_BLUEZ_BUS=session ve yalıtılmış bus yok (tools/bluetooth-dbus-test.sh ile çalıştırın)");
        return false;
    }
    // Eşleştirme kabuğa `msg bluetooth agent-status` sorar: yalnız test örneğine ve geçici
    // çalışma dizinine (kullanıcının çalışan kabuğuna asla).
    let instance = std::env::var(SHELL_INSTANCE).unwrap_or_default();
    let private_runtime = !runtime.starts_with("/run/user/");
    if instance.is_empty() || !private_runtime {
        eprintln!("atlandı: {SHELL_INSTANCE} ve geçici XDG_RUNTIME_DIR yok (tools/bluetooth-dbus-test.sh ile çalıştırın)");
        return false;
    }
    true
}

/// Test kabuğu örnek adı (betik verir; kullanıcının kabuğu `nuxut-shell` adını tutar).
const SHELL_INSTANCE: &str = "NUXUT_SHELL_INSTANCE";

/// Süreç içi sahte kabuk `msg` sunucusu: yalnız `bluetooth agent-status`'a `reply` döner (diğer
/// komutlara hata). `$XDG_RUNTIME_DIR/nuxut/nuxut-shell-<örnek>.sock` (framework IPC: tek satır
/// `\t` ayrılmış argüman → tek satır yanıt). Gelen istekleri sayar.
struct FakeShell {
    reply: std::sync::Arc<std::sync::Mutex<String>>,
    asked: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl FakeShell {
    fn start(reply: &str) -> Self {
        use std::io::{BufRead, Write};
        use std::os::unix::fs::DirBuilderExt;
        let runtime = std::path::PathBuf::from(std::env::var("XDG_RUNTIME_DIR").expect("betik verir"));
        let dir = runtime.join("nuxut");
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir).unwrap();
        let path = dir.join(format!("nuxut-shell-{}.sock", std::env::var(SHELL_INSTANCE).unwrap()));
        let _ = std::fs::remove_file(&path);
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let reply = std::sync::Arc::new(std::sync::Mutex::new(reply.to_owned()));
        let asked = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let (r, a) = (reply.clone(), asked.clone());
        std::thread::spawn(move || {
            // Test süreci bitince thread de biter; yazmayan istemci sunucuyu kilitlemesin (2 sn).
            for stream in listener.incoming().flatten() {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
                let mut line = String::new();
                if std::io::BufReader::new(&stream).read_line(&mut line).is_err() {
                    continue;
                }
                let answer = if line.trim_end() == "bluetooth\tagent-status" {
                    a.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    r.lock().unwrap().clone()
                } else {
                    "error: bilinmeyen komut".to_owned()
                };
                let _ = writeln!(&stream, "{answer}");
            }
        });
        Self { reply, asked }
    }

    fn set(&self, reply: &str) {
        *self.reply.lock().unwrap() = reply.to_owned();
    }

    fn asked(&self) -> usize {
        self.asked.load(std::sync::atomic::Ordering::SeqCst)
    }
}

fn fake_conn() -> zbus::blocking::Connection {
    zbus::blocking::Connection::session().expect("test bus'ına bağlanılamadı")
}

fn wait_for(what: &str, f: impl Fn(&Snapshot) -> bool) -> Snapshot {
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        let s = bluetooth::snapshot();
        if f(&s) {
            return s;
        }
        assert!(Instant::now() < end, "zaman aşımı: {what}; durum {:?}, hata {:?}", page_status(&s), s.failure);
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn wait_log(log: &Log, entry: &str) {
    let end = Instant::now() + Duration::from_secs(20);
    while !log.has(entry) {
        assert!(Instant::now() < end, "sahte BlueZ'e çağrı gelmedi: {entry}");
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn device<'a>(s: &'a Snapshot, id: &str) -> &'a nuxut_bluez::Device {
    let path = dev(id);
    s.state.ready().and_then(|st| st.devices.iter().find(|d| d.path == path)).expect("cihaz yok")
}

fn ready(log: &Log) -> Snapshot {
    bluetooth::ensure();
    let s = wait_for("hazır", |s| page_status(s) == PageStatus::Ready && s.state.ready().is_some_and(|st| !st.devices.is_empty()));
    assert!(!log.has("RegisterAgent KeyboardDisplay"));
    s
}

/// Ajan kaydı yok (her senaryonun sonunda).
fn assert_agentless(log: &Log) {
    // Son olayların gelmesi için kısa bekleme (kayıt yapılsaydı ilk durumdan hemen sonra olurdu).
    std::thread::sleep(Duration::from_millis(200));
    for entry in ["RegisterAgent KeyboardDisplay", "RegisterAgent NoInputNoOutput", "RequestDefaultAgent"] {
        assert!(!log.has(entry), "Ayarlar eşleştirme ajanı kaydetti: {entry} (yasak)");
    }
}

/// BlueZ çalışmıyor: maskotlu hata metni, listeler gizli, panik yok.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn missing_service() {
    if !isolated() {
        return;
    }
    bluetooth::ensure();
    let s = wait_for("Missing", |s| s.state != BtState::Pending);
    assert_eq!(s.state, BtState::Missing(Unavailable::ServiceNotRunning));
    assert_eq!(page_status(&s), PageStatus::NoBluez);
    assert!(unavailable_text(page_status(&s)).is_some());
    for k in [BtList::Paired, BtList::Nearby] {
        assert_eq!(content(k, &s, false), Content::Hidden);
    }
}

/// Sahne `no-adapter`: BlueZ var, adaptör yok.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn no_adapter() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let _fake = FakeBluez::start_scene(fake_conn(), &log, Scene::NoAdapter);
    bluetooth::ensure();
    let s = wait_for("adaptör yok", |s| s.state.ready().is_some());
    assert_eq!(page_status(&s), PageStatus::NoAdapter);
    assert!(unavailable_text(PageStatus::NoAdapter).is_some());
    assert_eq!(content(BtList::Paired, &s, false), Content::Hidden);
    assert_eq!(content(BtList::Nearby, &s, false), Content::Hidden);
    assert_agentless(&log);
}

/// Sahne `off`: adaptör kapalı başlar — eşleşmiş cihaz listede, yakındakiler "kapalı" notu,
/// sayfa görünse de tarama yok; açınca tarama başlar.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scene_off() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let _fake = FakeBluez::start_scene(fake_conn(), &log, Scene::Off);
    bluetooth::set_page_visible(true);
    let s = ready(&log);
    assert_eq!(s.state.ready().and_then(power), Some(Power::Off));
    assert_eq!(content(BtList::Nearby, &s, false), Content::Empty(nuxut::i18n::t!("settings-bluetooth-nearby-off")));
    assert!(matches!(content(BtList::Paired, &s, false), Content::Rows(rows, None) if rows.iter().any(|r| r.title == "Kulaklık")));
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(log.count("StartDiscovery"), 0, "kapalıyken tarama yok");
    bluetooth::set_powered(true);
    wait_for("açık + tarama", |s| s.state.ready().is_some_and(|st| st.own_discovery));
    assert_eq!(log.count("StartDiscovery"), 1);
    bluetooth::set_page_visible(false);
    wait_for("tarama durdu", |s| s.state.ready().is_some_and(|st| !st.own_discovery));
    assert_agentless(&log);
}

/// Sahne `empty`: adaptör açık, cihaz yok — eşleşmiş yok (maskot), yakındakiler boş.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scene_empty() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let _fake = FakeBluez::start_scene(fake_conn(), &log, Scene::Empty);
    bluetooth::ensure();
    let s = wait_for("hazır", |s| page_status(s) == PageStatus::Ready);
    assert_eq!(s.state.ready().and_then(power), Some(Power::On));
    assert_eq!(content(BtList::Paired, &s, false), Content::NoPaired);
    match content(BtList::Nearby, &s, false) {
        Content::Rows(rows, Some(info)) => assert!(rows.is_empty() && info.hidden == 0, "{rows:?}"),
        other => panic!("{other:?}"),
    }
    assert_agentless(&log);
}

/// Sahne `rfkill`: adaptör engelli başlar (sysfs okunamazsa yumuşak sayılır); açma reddi notta.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scene_rfkill() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let _fake = FakeBluez::start_scene(fake_conn(), &log, Scene::Rfkill);
    let s = ready(&log);
    assert_eq!(s.state.ready().and_then(power), Some(Power::SoftBlocked));
    bluetooth::set_powered(true);
    let s = wait_for("açma reddi", |s| s.failure.is_some());
    assert!(matches!(s.failure.unwrap().error, BluezError::RfkillBlocked { .. }));
    assert_agentless(&log);
}

/// Yakındakiler sinyale (RSSI) göre: Klavye (-48) > Hoparlör (-60) > Saat (-75); adsız LE
/// (-90) varsayılan gizli, gösterilince en sonda.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn nearby_sorted_by_rssi() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let _fake = FakeBluez::start(fake_conn(), &log, true);
    let s = ready(&log);
    let order = |unnamed| match content(BtList::Nearby, &s, unnamed) {
        Content::Rows(rows, _) => rows.into_iter().map(|r| r.path).collect::<Vec<_>>(),
        other => panic!("{other:?}"),
    };
    assert_eq!(order(false), [dev("dev_2"), dev("dev_3"), dev("dev_4")]);
    assert_eq!(order(true), [dev("dev_2"), dev("dev_3"), dev("dev_4"), dev("dev_5")]);
    assert_eq!(device(&s, "dev_2").rssi, Some(-48));
    assert_agentless(&log);
}

/// Eşleştirmeden önce kabuğa ajan durumu sorulur: varsayılan ajan yoksa satırda "kabuk
/// çalışmalı" uyarısı, `Pair` yine gönderilir; varsayılansa bekleme metni.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn pairing_agent_warning() {
    if !isolated() {
        return;
    }
    let shell = FakeShell::start(r#"{"registered":false,"default":false}"#);
    let log = Log::default();
    let _fake = FakeBluez::start(fake_conn(), &log, true);
    ready(&log);
    let nearby_subtitle = |s: &Snapshot, id: &str| match content(BtList::Nearby, s, false) {
        Content::Rows(rows, _) => rows.into_iter().find(|r| r.path == dev(id)).map(|r| r.subtitle),
        _ => None,
    };
    // Saat (`dev_4`) CancelPairing bekler: eşleştirme sürerken satır gözlenebilir.
    bluetooth::pair(&dev("dev_4"));
    wait_log(&log, &format!("PairWaiting {}", dev("dev_4")));
    let s = wait_for("uyarı", |s| s.pairing.as_ref().is_some_and(|p| p.no_agent && p.request.is_some()));
    assert_eq!(nearby_subtitle(&s, "dev_4").as_deref(), Some(nuxut::i18n::t!("settings-bluetooth-pairing-no-agent").as_str()));
    assert_eq!(shell.asked(), 1);
    bluetooth::cancel_pairing();
    wait_for("iptal", |s| s.pairing.is_none());

    // Kabuğun ajanı varsayılan: bekleme metni.
    shell.set(r#"{"registered":true,"default":true}"#);
    bluetooth::pair(&dev("dev_4"));
    let s = wait_for("gönderildi", |s| s.pairing.as_ref().is_some_and(|p| p.request.is_some()));
    assert!(!s.pairing.as_ref().unwrap().no_agent);
    assert_eq!(nearby_subtitle(&s, "dev_4").as_deref(), Some(nuxut::i18n::t!("settings-bluetooth-pairing-wait").as_str()));
    assert_eq!(shell.asked(), 2);
    bluetooth::cancel_pairing();
    wait_for("iptal", |s| s.pairing.is_none());

    // Kabuk yanıtı sözleşme dışı (eski kabuk): eski davranış, uyarı yok.
    shell.set("error: bilinmeyen komut");
    bluetooth::pair(&dev("dev_2"));
    let s = wait_for("eşleşti", |s| s.pairing.is_none());
    assert_eq!(s.pair_failed, None);
    assert!(device(&s, "dev_2").paired, "onaysız cihaz yine eşleşir");
    assert_agentless(&log);
}

/// Aç/kapa ve rfkill (yumuşak → sert); hata türü sysfs'ten.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn power_and_rfkill() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let fake = FakeBluez::start(fake_conn(), &log, true);
    ready(&log);
    bluetooth::set_powered(false);
    wait_for("kapalı", |s| s.state.ready().and_then(power) == Some(Power::Off));
    assert!(log.has("Powered false"));
    let s = bluetooth::snapshot();
    assert_eq!(content(BtList::Nearby, &s, false), Content::Empty(nuxut::i18n::t!("settings-bluetooth-nearby-off")));
    bluetooth::set_powered(true);
    wait_for("açık", |s| s.state.ready().and_then(power) == Some(Power::On));

    // Yumuşak engel (uçak kipi): sysfs'te soft=1.
    let dir = std::path::PathBuf::from(std::env::var("NUXUT_BLUEZ_RFKILL_DIR").expect("betik rfkill dizinini verir"));
    let entry = dir.join("rfkill0");
    std::fs::create_dir_all(&entry).unwrap();
    std::fs::write(entry.join("type"), "bluetooth\n").unwrap();
    std::fs::write(entry.join("soft"), "1\n").unwrap();
    std::fs::write(entry.join("hard"), "0\n").unwrap();
    fake.set_rfkill(true);
    let s = wait_for("yumuşak engel", |s| s.state.ready().and_then(power) == Some(Power::SoftBlocked));
    assert_eq!(s.state.ready().unwrap().adapters[0].power_state, PowerState::Blocked);
    bluetooth::set_powered(true);
    let s = wait_for("açma reddi", |s| s.failure.is_some());
    assert_eq!(s.failure.unwrap().error, BluezError::RfkillBlocked { hard: Some(false) });

    // Sert engel: durum yeniden okununca (herhangi bir değişiklik) donanım anahtarı görünür.
    std::fs::write(entry.join("hard"), "1\n").unwrap();
    fake.set_battery(&dev("dev_1"), 70);
    wait_for("sert engel", |s| s.state.ready().and_then(power) == Some(Power::HardBlocked));
    bluetooth::set_powered(true);
    let s = wait_for("açma reddi (sert)", |s| s.failure.as_ref().is_some_and(|f| f.error == BluezError::RfkillBlocked { hard: Some(true) }));
    assert!(s.failure.is_some());
    fake.set_rfkill(false);
    assert_agentless(&log);
}

/// Adaptör adı ve görünürlük (süre önce, sonra açılış; kalan süre metni).
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn name_and_visibility() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let _fake = FakeBluez::start(fake_conn(), &log, true);
    ready(&log);
    bluetooth::set_name("  Masaüstü  ");
    wait_for("ad", |s| s.state.ready().and_then(|st| st.adapter()).is_some_and(|a| a.name == "Masaüstü"));
    assert!(log.has(&format!("Alias {HCI0} Masaüstü")), "ad kırpılarak yazılır");

    bluetooth::set_discoverable(true);
    let s = wait_for("görünür", |s| s.state.ready().and_then(|st| st.adapter()).is_some_and(|a| a.discoverable));
    let (t, d) = (log.index("DiscoverableTimeout 180").expect("süre"), log.index("Discoverable true").expect("açılış"));
    assert!(t < d, "süre önce yazılır");
    assert!(s.discoverable_since.is_some(), "açılış görüldü: kalan süre gösterilir");
    let a = s.state.ready().unwrap().adapter().unwrap().clone();
    let text = discoverable_text(&a, s.discoverable_since, Instant::now());
    assert!(text.contains("3:00") || text.contains("2:5"), "kalan süre metinde: {text}");
    bluetooth::set_discoverable(false);
    let s = wait_for("görünmez", |s| s.state.ready().and_then(|st| st.adapter()).is_some_and(|a| !a.discoverable));
    assert_eq!(s.discoverable_since, None);
    assert_agentless(&log);
}

/// Tarama yalnız sayfa görünürken; ayrılınca yalnız kendi taramamız durur.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn discovery_follows_page() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let fake = FakeBluez::start(fake_conn(), &log, true);
    ready(&log);
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(log.count("StartDiscovery"), 0, "sayfa görünmüyorken tarama yok");

    bluetooth::set_page_visible(true);
    wait_for("kendi taramamız", |s| s.state.ready().is_some_and(|st| st.own_discovery));
    assert_eq!(log.count("StartDiscovery"), 1);
    assert!(log.has("SetDiscoveryFilter auto false"));
    let s = bluetooth::snapshot();
    match content(BtList::Nearby, &s, false) {
        Content::Rows(rows, Some(info)) => {
            assert!(info.scanning);
            assert_eq!(info.hidden, 1, "adsız LE cihaz varsayılan gizli");
            assert!(rows.iter().all(|r| matches!(r.kind, RowKind::Nearby { can_pair: true, .. })));
            assert_eq!(rows.len(), 3, "Klavye, Hoparlör, Saat");
        }
        other => panic!("yakındakiler yok: {other:?}"),
    }
    match content(BtList::Nearby, &s, true) {
        Content::Rows(rows, Some(info)) => assert_eq!((rows.len(), info.hidden, info.unnamed_shown), (4, 0, 1)),
        other => panic!("{other:?}"),
    }
    // Yeniden görünür olmak (aynı durum) ikinci tarama başlatmaz.
    bluetooth::set_page_visible(true);
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(log.count("StartDiscovery"), 1);

    bluetooth::set_page_visible(false);
    wait_log(&log, "StopDiscovery");
    wait_for("tarama durdu", |s| s.state.ready().is_some_and(|st| !st.own_discovery));

    // Başka istemcinin taraması: sayfadan çıkınca ona dokunulmaz.
    bluetooth::set_page_visible(true);
    wait_for("yeniden tarama", |s| s.state.ready().is_some_and(|st| st.own_discovery));
    assert_eq!(log.count("StartDiscovery"), 2);
    bluetooth::set_page_visible(false);
    wait_for("durdu", |s| s.state.ready().is_some_and(|st| !st.own_discovery));
    let stops = log.count("StopDiscovery");
    fake.set_external_discovering(true);
    std::thread::sleep(Duration::from_millis(200));
    bluetooth::set_page_visible(true);
    // Dış tarama sürüyor olsa da biz kendi taramamızı isteriz (BlueZ istemci başına sayar).
    wait_for("kendi", |s| s.state.ready().is_some_and(|st| st.own_discovery));
    bluetooth::set_page_visible(false);
    wait_for("kendi durdu", |s| s.state.ready().is_some_and(|st| !st.own_discovery));
    assert_eq!(log.count("StopDiscovery"), stops + 1, "yalnız kendi taramamız durdu");
    assert_agentless(&log);
}

/// Eşleştirme: başarı (ajan yok → just works; crate güvenir ve bağlanır), başarısız, ret, iptal.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn pairing_results() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let fake = FakeBluez::start(fake_conn(), &log, true);
    fake.add_failing_device("dev_7", "Ret", PairError::Rejected);
    ready(&log);
    wait_for("Ret görünür", |s| s.state.ready().is_some_and(|st| st.devices.iter().any(|d| d.path == dev("dev_7"))));

    // Başarı.
    bluetooth::pair(&dev("dev_2"));
    let s = bluetooth::snapshot();
    assert_eq!(s.pairing.as_ref().map(|p| p.device.as_str()), Some(dev("dev_2").as_str()));
    if let Content::Rows(rows, _) = content(BtList::Nearby, &s, false) {
        let row = rows.iter().find(|r| r.path == dev("dev_2")).unwrap();
        assert_eq!(row.subtitle, nuxut::i18n::t!("settings-bluetooth-pairing-wait"), "kabuğa yönlendiren bekleme metni");
        assert!(matches!(row.kind, RowKind::Nearby { pairing: true, .. }));
        assert!(rows.iter().filter(|r| r.path != dev("dev_2")).all(|r| matches!(r.kind, RowKind::Nearby { can_pair: false, .. })), "aynı anda tek eşleştirme");
    }
    let s = wait_for("eşleşme bitti", |s| s.pairing.is_none());
    assert_eq!(s.pair_failed, None);
    wait_log(&log, &format!("Paired {}", dev("dev_2")));
    wait_log(&log, &format!("Connect {}", dev("dev_2")));
    assert!(log.has(&format!("Trusted {} true", dev("dev_2"))));
    // Pair → Paired=true (sahte BlueZ cevaptan önce yayınlar): eşleşmiş listeye geçer.
    let s = wait_for("eşleşmiş", |s| device(s, "dev_2").paired);
    assert!(matches!(content(BtList::Paired, &s, false), Content::Rows(rows, None) if rows.iter().any(|r| r.path == dev("dev_2"))));

    // Başarısız (AuthenticationFailed) ve ret (AuthenticationRejected): satırda hata metni.
    for (id, error) in [("dev_3", BluezError::AuthenticationFailed), ("dev_7", BluezError::AuthenticationRejected)] {
        bluetooth::pair(&dev(id));
        let s = wait_for("eşleştirme hatası", |s| s.pairing.is_none() && s.pair_failed.is_some());
        assert_eq!(s.pair_failed, Some((dev(id), error.clone())));
        assert_eq!(s.failure, None, "eşleştirme hatası üst nota gitmez");
        if let Content::Rows(rows, _) = content(BtList::Nearby, &s, false) {
            let row = rows.iter().find(|r| r.path == dev(id)).unwrap();
            assert!(row.error);
            assert_eq!(row.subtitle, nuxut_settings::pages::bluetooth::error_text(&error));
        }
    }

    // İptal: CancelPairing gönderilir, AuthenticationCanceled gösterilmez.
    bluetooth::pair(&dev("dev_4"));
    wait_log(&log, &format!("PairWaiting {}", dev("dev_4")));
    bluetooth::cancel_pairing();
    wait_log(&log, &format!("CancelPairing {}", dev("dev_4")));
    let s = wait_for("iptal", |s| s.pairing.is_none());
    assert_eq!(s.pair_failed, None, "kullanıcı iptali hata değil");
    assert_agentless(&log);
}

/// Eşleşmiş cihaz: bağlan/kopar, güven/engelle, yeniden adlandır, unut, pil yüzdesi.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn device_actions_and_battery() {
    if !isolated() {
        return;
    }
    let log = Log::default();
    let fake = FakeBluez::start(fake_conn(), &log, true);
    let s = ready(&log);
    let row = |s: &Snapshot| match content(BtList::Paired, s, false) {
        Content::Rows(rows, None) => rows.into_iter().find(|r| r.path == dev("dev_1")).expect("Kulaklık"),
        other => panic!("eşleşmiş liste yok: {other:?}"),
    };
    assert_eq!(row(&s).battery, Some(80));
    assert_eq!(row(&s).icon, nuxut::ui::IconName::Headphones, "Icon=audio-headset");
    fake.set_battery(&dev("dev_1"), 55);
    let s = wait_for("pil", |s| device(s, "dev_1").battery == Some(55));
    assert_eq!(row(&s).battery, Some(55));

    bluetooth::connect(&dev("dev_1"));
    let s = wait_for("bağlı", |s| device(s, "dev_1").connected);
    assert!(row(&s).active);
    assert!(row(&s).subtitle.starts_with(&nuxut::i18n::t!("settings-bluetooth-connected")));
    bluetooth::disconnect(&dev("dev_1"));
    wait_for("koptu", |s| !device(s, "dev_1").connected);
    assert!(log.has(&format!("Disconnect {}", dev("dev_1"))));

    bluetooth::set_trusted(&dev("dev_1"), false);
    wait_for("güvenilmez", |s| !device(s, "dev_1").trusted);
    bluetooth::set_blocked(&dev("dev_1"), true);
    let s = wait_for("engelli", |s| device(s, "dev_1").blocked);
    assert!(matches!(row(&s).kind, RowKind::Paired { blocked: true, trusted: false, .. }));
    assert!(row(&s).subtitle.contains(&nuxut::i18n::t!("settings-bluetooth-blocked")));
    bluetooth::set_blocked(&dev("dev_1"), false);
    wait_for("engel kalktı", |s| !device(s, "dev_1").blocked);

    bluetooth::rename(&dev("dev_1"), " Kafa ");
    let s = wait_for("ad", |s| device(s, "dev_1").name == "Kafa");
    assert_eq!(row(&s).title, "Kafa");
    assert!(log.has(&format!("Alias {} Kafa", dev("dev_1"))));

    bluetooth::forget(&dev("dev_1"));
    wait_log(&log, &format!("RemoveDevice {}", dev("dev_1")));
    // RemoveDevice cihazı (pil dahil) siler: listeden düşer.
    let s = wait_for("silindi", |s| s.state.ready().is_some_and(|st| st.devices.iter().all(|d| d.path != dev("dev_1"))));
    assert!(!matches!(content(BtList::Paired, &s, false), Content::Rows(rows, _) if rows.iter().any(|r| r.path == dev("dev_1"))));
    assert_eq!(s.failure, None);
    assert_agentless(&log);
}
