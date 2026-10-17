//! Ağ servisinin, eylemlerinin ve Wi-Fi/VPN sayfa içeriğinin uçtan uca testi: crate'in sahte
//! NetworkManager'ı (`../shell/crates/nuxut-nm` `examples/fake_nm`) ile, YALNIZCA
//! `dbus-run-session` içindeki oturum bus'ında. `test-support` özelliği olmadan derlenmez; testler
//! `#[ignore]`: düz `cargo test` çalıştırmaz. `tools/network-dbus-test.sh` her senaryoyu ayrı
//! süreçte çalıştırır (servis durumu süreç geneli) ve sahte NM'nin çağrı logunu denetler.
//! `NUXUT_NM_BUS=session` yoksa testler hiçbir şey yapmadan döner: gerçek sistem NM'ine asla
//! gidilmez. Şifreler ve anahtarlar test sabitidir (gerçek gizli bilgi değil).
#![cfg(feature = "test-support")]

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use nuxut_nm::{Metered, Security, Unavailable};
use nuxut_settings::pages::network::{ListContent, PanelKind, Press, RowData, content};
use nuxut_settings::registry::NetList;
use nuxut_settings::services::network::{self, NetState, RowError, Snapshot, Target, WifiStatus, wifi_status};
use zeroize::Zeroizing;

fn isolated() -> bool {
    nuxut_settings::i18n::use_turkish_for_tests();
    let ok = std::env::var(nuxut_nm::BUS_ENV).is_ok_and(|v| v == "session");
    if !ok {
        eprintln!("atlandı: {}=session yok (dbus-run-session betiğiyle çalıştırın)", nuxut_nm::BUS_ENV);
    }
    ok
}

fn wait_for(what: &str, f: impl Fn(&Snapshot) -> bool) -> Snapshot {
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        let s = network::snapshot();
        if f(&s) {
            return s;
        }
        assert!(Instant::now() < end, "zaman aşımı: {what}; son durum {:?}", s.state);
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn rows(c: ListContent) -> Vec<RowData> {
    match c {
        ListContent::Rows(r) => r,
        other => panic!("satır yok: {other:?}"),
    }
}

fn titles(c: ListContent) -> Vec<String> {
    rows(c).into_iter().map(|r| r.title).collect()
}

/// Sahte NM `default` sahnesinin kayıtlı bağlantıları (`psk-flags` 1/2/4 ve WEP örnekleri dahil).
const DEMO_SAVED: [&str; 8] = ["Ev", "Ofis", "İş VPN", "Kablolu", "Ajanli", "Sormali", "Gereksiz", "EskiWep"];

/// Sahte NM'nin (`default` sahnesi) kayıtlı bağlantıları okunana dek bekler.
fn ready() -> Snapshot {
    network::ensure();
    wait_for("hazır", |s| s.state.ready().is_some_and(|st| DEMO_SAVED.iter().all(|id| st.saved.iter().any(|c| c.id == *id))))
}

fn saved_path(s: &Snapshot, id: &str) -> String {
    s.state.ready().unwrap().saved.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id} yok")).path.clone()
}

fn pw(s: &str) -> Option<Zeroizing<Vec<u8>>> {
    Some(Zeroizing::new(s.as_bytes().to_vec()))
}

fn idle(what: &str, target: Target) -> Snapshot {
    wait_for(what, move |s| !s.busy(&target))
}

/// Sahte NM çalışmıyorken: "NetworkManager çalışmıyor" notu, kartlar gizli, panik yok.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn missing_service() {
    if !isolated() {
        return;
    }
    network::ensure();
    let s = wait_for("kullanılamaz", |s| s.state != NetState::Pending);
    assert_eq!(s.state, NetState::Unavailable(Unavailable::ServiceNotRunning));
    for k in [NetList::Current, NetList::Visible, NetList::Saved, NetList::Vpn, NetList::Wired, NetList::Import] {
        assert_eq!(content(k, &s), ListContent::Hidden);
    }
}

/// Bus adresi var ama ulaşılamıyor: "NM yok"tan ayrı neden.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn bus_unreachable() {
    if !isolated() {
        return;
    }
    network::ensure();
    let s = wait_for("bus yok", |s| s.state != NetState::Pending);
    assert_eq!(s.state, NetState::Unavailable(Unavailable::BusUnreachable));
}

/// Oturum bus'ı istendi ama adresi yok: servis açılmaz (sisteme/varsayılan oturuma düşülmez).
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn no_session_address() {
    if !isolated() {
        return;
    }
    assert!(std::env::var_os("DBUS_SESSION_BUS_ADDRESS").is_none(), "betik adresi silmeli");
    network::ensure();
    assert_eq!(network::snapshot().state, NetState::Failed);
}

/// Bağlı senaryo: listeler, kablolu ayrıntı (IP/DNS/hız), sayfa görünürken tek tarama.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn connected() {
    if !isolated() {
        return;
    }
    let s = ready();
    let st = s.state.ready().unwrap();
    assert_eq!(wifi_status(st), WifiStatus::On);
    assert_eq!(titles(content(NetList::Current, &s)), ["Ev"]);
    assert_eq!(titles(content(NetList::Visible, &s)), ["Misafir", "Kafe", "Komsu"], "sinyale göre, bağlı ve gizli yok");
    assert_eq!(titles(content(NetList::Saved, &s)), ["Ajanli", "EskiWep", "Ev", "Gereksiz", "Ofis", "Sormali"], "yalnız Wi-Fi, ada göre");
    assert_eq!(rows(content(NetList::Vpn, &s)).iter().map(|r| (r.title.as_str(), r.switch)).collect::<Vec<_>>(), [("İş VPN", Some(false))]);
    let s = wait_for("kablolu ayrıntı", |s| rows(content(NetList::Wired, s)).first().is_some_and(|r| !r.details.is_empty()));
    let wired = &rows(content(NetList::Wired, &s))[0];
    let labels: Vec<&str> = wired.details.iter().map(|(k, _)| k.as_str()).collect();
    assert!(labels.contains(&t!("settings-network-detail-ipv4").as_str()) && labels.contains(&t!("settings-network-detail-dns").as_str()), "{labels:?}");
    assert!(labels.contains(&t!("settings-network-detail-speed").as_str()));
    // Sayfa görünmüyorken tarama yok; görünür olunca bir kez.
    network::try_scan();
    std::thread::sleep(Duration::from_millis(300));
    network::set_wifi_visible(true);
    network::set_wifi_visible(true);
    network::try_scan();
    std::thread::sleep(Duration::from_millis(300));
    network::set_wifi_visible(false);
}

/// Uygulama doğrudan Wi-Fi sayfasıyla açıldığında: tarama ilk durum gelince bir kez.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scan_when_opened_on_wifi() {
    if !isolated() {
        return;
    }
    network::set_wifi_visible(true);
    let _ = ready();
    std::thread::sleep(Duration::from_millis(500));
}

/// Wi-Fi kapat/aç: iyimser değil, gerçek durum gelince biter; kapalıyken liste notu.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn wifi_toggle() {
    if !isolated() {
        return;
    }
    let _ = ready();
    network::set_wifi(false);
    assert!(network::snapshot().busy(&Target::Wifi), "beklerken meşgul");
    let s = wait_for("Wi-Fi kapandı", |s| s.state.ready().is_some_and(|st| !st.wifi_enabled) && !s.busy(&Target::Wifi));
    assert_eq!(content(NetList::Visible, &s), ListContent::Empty(t!("settings-wifi-visible-off")));
    assert_eq!(content(NetList::Hidden, &s), ListContent::Hidden);
    network::set_wifi(true);
    wait_for("Wi-Fi açıldı", |s| s.state.ready().is_some_and(|st| st.wifi_enabled) && !s.busy(&Target::Wifi));
}

/// Açık ağ: doğrudan bağlan (şifresiz yeni bağlantı).
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn connect_open() {
    if !isolated() {
        return;
    }
    let s = ready();
    let row = rows(content(NetList::Visible, &s)).into_iter().find(|r| r.title == "Misafir").unwrap();
    assert_eq!(row.press, Press::Connect);
    network::connect(b"Misafir", None);
    assert!(network::snapshot().busy(&Target::Ap(b"Misafir".to_vec())));
    std::thread::sleep(Duration::from_millis(400));
}

/// Şifreli ağ, doğru biçimde şifre: yeni bağlantı şifreyle.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn connect_password() {
    if !isolated() {
        return;
    }
    let s = ready();
    let row = rows(content(NetList::Visible, &s)).into_iter().find(|r| r.title == "Kafe").unwrap();
    assert_eq!(row.press, Press::Open(PanelKind::Password { security: Security::WpaPsk }));
    network::connect(b"Kafe", pw("test-sifre-1"));
    std::thread::sleep(Duration::from_millis(400));
}

/// Şifreli ağ, geçersiz şifre: istemci doğrulaması, NM'ye çağrı yok; satırda hata, alan açık.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn connect_password_invalid() {
    if !isolated() {
        return;
    }
    let _ = ready();
    network::connect(b"Kafe", pw("kisa"));
    let s = network::snapshot();
    assert_eq!(s.error(&Target::Ap(b"Kafe".to_vec())), Some(&RowError::InvalidPassword));
    let row = rows(content(NetList::Visible, &s)).into_iter().find(|r| r.title == "Kafe").unwrap();
    assert!(row.wants_password && row.error.is_some());
    std::thread::sleep(Duration::from_millis(300));
}

/// Kayıtlı ağın şifresi NM'de yok (ajansız Ayarlar): NoSecrets → şifre alanı → kayıtlı şifre
/// yerinde güncellenir (`set_wifi_password`: `Update`, bağlantı yolu ve ayarları aynı; unut +
/// yeni bağlantı YOK) → etkinleştirilir. Yanlış yeni şifre yine NoSecrets.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn saved_needs_password() {
    if !isolated() {
        return;
    }
    let s = ready();
    let ev = saved_path(&s, "Ev");
    let t = Target::Ap(b"Ev".to_vec());
    network::connect(b"Ev", None);
    let s = wait_for("NoSecrets", |s| s.error(&t).is_some());
    assert_eq!(s.error(&t), Some(&RowError::NoSecrets));
    let mut st = s.state.ready().unwrap().clone();
    // Ev bağlı olduğu için görünen listede yok; içerik eşlemesini bağlı olmayan hâliyle denetle.
    for a in &mut st.access_points {
        a.active = false;
    }
    let s2 = Snapshot { state: NetState::Ready(st), ..s.clone() };
    let row = rows(content(NetList::Visible, &s2)).into_iter().find(|r| r.title == "Ev").unwrap();
    assert_eq!(row.press, Press::Open(PanelKind::Password { security: Security::WpaPsk }), "alan yeniden açılır");
    // Yanlış yeni şifre: güncellenir, etkinleştirme NO_SECRETS ile düşer → alan yine açılır.
    network::connect(b"Ev", pw("yeni-sifre-1"));
    let s = wait_for("yanlış şifre", |s| s.error(&t).is_some() && !s.busy(&t));
    assert_eq!(s.error(&t), Some(&RowError::NoSecrets));
    // Doğru şifre: güncelle → etkinleştir → biter, hata yok.
    network::connect(b"Ev", pw("ev-sifre-123"));
    let s = idle("şifre güncellendi ve bağlandı", t.clone());
    assert_eq!(s.error(&t), None);
    let st = s.state.ready().unwrap();
    let c = st.saved.iter().find(|c| c.id == "Ev").expect("Ev kayıtlı kalır");
    assert_eq!(c.path, ev, "aynı kayıt (unut + yeni bağlantı yok)");
    assert!(c.autoconnect, "otomatik bağlanma korunur");
    std::thread::sleep(Duration::from_millis(300));
}

/// Gizli ağ: geçersiz SSID istemcide reddedilir; geçerli form `connect_hidden` → Done.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn hidden_network() {
    if !isolated() {
        return;
    }
    let _ = ready();
    network::connect_hidden("", Security::WpaPsk, pw("gizli-sifre"));
    assert_eq!(network::snapshot().error(&Target::Hidden), Some(&RowError::InvalidSsid));
    network::connect_hidden(&"a".repeat(33), Security::WpaPsk, pw("gizli-sifre"));
    assert_eq!(network::snapshot().error(&Target::Hidden), Some(&RowError::InvalidSsid));
    network::connect_hidden("Gizli", Security::WpaPsk, pw("gizli-sifre"));
    let s = idle("gizli ağ bitti", Target::Hidden);
    assert_eq!(s.error(&Target::Hidden), None);
    wait_for("gizli kayıt", |s| s.state.ready().is_some_and(|st| st.saved.iter().any(|c| c.id == "Gizli")));
}

/// Bağlı ağ: Bağlantıyı kes.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn disconnect() {
    if !isolated() {
        return;
    }
    let s = ready();
    let ev = saved_path(&s, "Ev");
    assert_eq!(rows(content(NetList::Current, &s))[0].press, Press::Open(PanelKind::Current { connection: Some(ev.clone()) }));
    network::disconnect(&ev);
    assert!(network::snapshot().busy(&Target::Conn(ev)));
    std::thread::sleep(Duration::from_millis(400));
}

/// Kayıtlı ağ: otomatik bağlan, ölçülü (Done ile biter, durumda görünür), unut.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn saved_settings_and_forget() {
    if !isolated() {
        return;
    }
    let s = ready();
    let ofis = saved_path(&s, "Ofis");
    let t = Target::Conn(ofis.clone());
    network::set_autoconnect(&ofis, false);
    let s = idle("otomatik", t.clone());
    assert_eq!(s.error(&t), None);
    wait_for("autoconnect=false", |s| s.state.ready().is_some_and(|st| st.saved.iter().any(|c| c.path == ofis && !c.autoconnect)));
    network::set_metered(&ofis, Metered::Yes);
    idle("ölçülü", t.clone());
    let s = wait_for("metered=yes", |s| s.state.ready().is_some_and(|st| st.saved.iter().any(|c| c.path == ofis && c.metered == Metered::Yes)));
    let row = rows(content(NetList::Saved, &s)).into_iter().find(|r| r.title == "Ofis").unwrap();
    assert_eq!(row.press, Press::Open(PanelKind::Saved { connection: ofis.clone(), autoconnect: false, metered: Metered::Yes }));
    network::forget(&ofis);
    let s = wait_for("unutuldu", |s| s.state.ready().is_some_and(|st| st.saved.iter().all(|c| c.path != ofis)) && !s.busy(&t));
    assert_eq!(titles(content(NetList::Saved, &s)), ["Ajanli", "EskiWep", "Ev", "Gereksiz", "Sormali"]);
}

/// VPN aç (sahte NM etkinleştirmeyi kabul eder, durumu değiştirmez); etkin değilken kapatma
/// hiçbir şey göndermez (betik logda `Deactivate` olmadığını denetler).
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn vpn_toggle() {
    if !isolated() {
        return;
    }
    let s = ready();
    let vpn = saved_path(&s, "İş VPN");
    network::set_vpn(&vpn, false);
    assert!(!network::snapshot().busy(&Target::Conn(vpn.clone())), "etkin değil: istek yok");
    network::set_vpn(&vpn, true);
    assert!(network::snapshot().busy(&Target::Conn(vpn.clone())));
    network::forget(&vpn);
    assert!(!network::snapshot().ops.iter().any(|o| o.kind == network::OpKind::Forget), "hedef meşgulken ikinci eylem yok");
    std::thread::sleep(Duration::from_millis(400));
}

#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn vpn_delete() {
    if !isolated() {
        return;
    }
    let s = ready();
    let vpn = saved_path(&s, "İş VPN");
    network::forget(&vpn);
    let s = wait_for("VPN silindi", |s| s.state.ready().is_some_and(|st| st.vpns().next().is_none()));
    assert_eq!(content(NetList::Vpn, &s), ListContent::Empty(t!("settings-vpn-empty")));
}

/// WireGuard içe aktarma: geçerli dosya → AddConnection + Done; geçersiz dosya ve ad → satır hatası.
/// Anahtarlar wg(8) kılavuzunun yayımlanmış örnekleridir (gerçek gizli bilgi değil).
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn wireguard_import() {
    if !isolated() {
        return;
    }
    let _ = ready();
    let dir = tempfile::tempdir().unwrap();
    let bad = dir.path().join("wg-bozuk.conf");
    std::fs::write(&bad, "[Interface]\nPrivateKey = kisa\n").unwrap();
    network::import_file(&bad);
    let s = idle("bozuk", Target::Import);
    assert!(matches!(s.error(&Target::Import), Some(RowError::Import(network::ImportError::Parse(_)))), "{:?}", s.error(&Target::Import));
    // wg-quick'e özgü PostUp/Table içe aktarılmaz (çalıştırılmaz); satırda adlarıyla not.
    let conf = "[Interface]\nPrivateKey = MDEyMzQ1Njc4OWFiY2RlZjAxMjM0NTY3ODlhYmNkZWY=\nAddress = 10.0.0.2/32\nDNS = 10.0.0.1\n\
                Table = off\nPostUp = echo gizli-komut\n\n\
                [Peer]\nPublicKey = ZmVkY2JhOTg3NjU0MzIxMGZlZGNiYTk4NzY1NDMyMTA=\nAllowedIPs = 0.0.0.0/0\nEndpoint = vpn.example.com:51820\n";
    let long = dir.path().join("bu-ad-fazla-uzun-bir-ad.conf");
    std::fs::write(&long, conf).unwrap();
    network::import_file(&long);
    let s = idle("uzun ad", Target::Import);
    assert!(matches!(s.error(&Target::Import), Some(RowError::Import(network::ImportError::Parse(e))) if e.kind == nuxut_nm::WireGuardErrorKind::InvalidName));
    let ok = dir.path().join("wg-test.conf");
    std::fs::write(&ok, conf).unwrap();
    network::import_file(&ok);
    let s = idle("içe aktarıldı", Target::Import);
    assert_eq!(s.error(&Target::Import), None);
    let s = wait_for("yeni VPN", |s| s.state.ready().is_some_and(|st| st.vpns().any(|c| c.id == "wg-test")));
    assert!(titles(content(NetList::Vpn, &s)).contains(&"wg-test".to_owned()));
    let mut ignored = s.import_ignored.clone();
    ignored.sort_unstable();
    assert_eq!(ignored, ["PostUp", "Table"], "yalnız anahtar adları");
    let row = rows(content(NetList::Import, &s)).remove(0);
    assert!(row.subtitle.contains("PostUp") && row.subtitle.contains("Table"), "bilgi notu: {}", row.subtitle);
    assert!(!row.subtitle.contains("gizli-komut"), "değer asla");
    assert!(row.error.is_none());
}

/// VPN aç → etkinleşince biter; kapat → `Deactivate`, etkin değil.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn vpn_off() {
    if !isolated() {
        return;
    }
    let s = ready();
    let vpn = saved_path(&s, "İş VPN");
    let t = Target::Conn(vpn.clone());
    network::set_vpn(&vpn, true);
    let s = wait_for("VPN etkin", |s| !s.busy(&t) && rows(content(NetList::Vpn, s)).iter().any(|r| r.title == "İş VPN" && r.switch == Some(true)));
    assert_eq!(s.error(&t), None);
    network::set_vpn(&vpn, false);
    assert!(network::snapshot().busy(&t));
    let s = wait_for("VPN kapandı", |s| !s.busy(&t) && rows(content(NetList::Vpn, s)).iter().any(|r| r.title == "İş VPN" && r.switch == Some(false)));
    assert_eq!(s.error(&t), None);
}

/// Sahne `no-device`: Wi-Fi cihazı yok — durum satırı metni, görünen/gizli ağ kartları yok, kayıtlı
/// Wi-Fi boş (maskot metni). Tarama istenmez.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scene_no_device() {
    if !isolated() {
        return;
    }
    network::set_wifi_visible(true);
    network::ensure();
    let s = wait_for("hazır", |s| s.state.ready().is_some());
    let st = s.state.ready().unwrap();
    assert_eq!(wifi_status(st), WifiStatus::NoDevice);
    for k in [NetList::Current, NetList::Visible, NetList::Hidden] {
        assert_eq!(content(k, &s), ListContent::Hidden, "{k:?}");
    }
    assert_eq!(content(NetList::Saved, &s), ListContent::Empty(t!("settings-wifi-saved-empty")));
    std::thread::sleep(Duration::from_millis(300));
    network::set_wifi_visible(false);
}

/// Sahne `empty`: Wi-Fi açık, ağ ve kayıt yok — görünen ağ yok notu, kayıtlı/VPN boş.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scene_empty() {
    if !isolated() {
        return;
    }
    network::set_wifi_visible(true);
    network::ensure();
    let s = wait_for("hazır", |s| s.state.ready().is_some());
    assert_eq!(wifi_status(s.state.ready().unwrap()), WifiStatus::On);
    assert_eq!(content(NetList::Visible, &s), ListContent::Empty(t!("settings-wifi-visible-empty")));
    assert_eq!(content(NetList::Current, &s), ListContent::Hidden);
    assert_eq!(content(NetList::Saved, &s), ListContent::Empty(t!("settings-wifi-saved-empty")));
    assert_eq!(content(NetList::Vpn, &s), ListContent::Empty(t!("settings-vpn-empty")));
    assert_eq!(titles(content(NetList::Hidden, &s)), [t!("settings-wifi-hidden-label")], "gizli ağ formu yine var");
    std::thread::sleep(Duration::from_millis(300));
    network::set_wifi_visible(false);
}

/// Sahne `wifi-off`: Wi-Fi kapalı başlar — kapalı notu, tarama yok; açınca liste gelir ve
/// sayfa görünürken bir kez taranır.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn scene_wifi_off() {
    if !isolated() {
        return;
    }
    network::set_wifi_visible(true);
    network::ensure();
    let s = wait_for("hazır", |s| s.state.ready().is_some_and(|st| st.saved.iter().any(|c| c.id == "Ev")));
    assert_eq!(wifi_status(s.state.ready().unwrap()), WifiStatus::Off);
    assert_eq!(content(NetList::Visible, &s), ListContent::Empty(t!("settings-wifi-visible-off")));
    assert_eq!(content(NetList::Hidden, &s), ListContent::Hidden);
    std::thread::sleep(Duration::from_millis(300));
    network::set_wifi(true);
    wait_for("Wi-Fi açıldı", |s| s.state.ready().is_some_and(|st| st.wifi_enabled) && !s.busy(&Target::Wifi));
    std::thread::sleep(Duration::from_millis(400));
    network::set_wifi_visible(false);
}
