//! Kullanıcı hesabı servisinin uçtan uca testi: sahte AccountsService (`examples/fake_accounts.rs`)
//! ile, YALNIZCA `dbus-run-session` içindeki oturum bus'ında. Testler `#[ignore]`: düz
//! `cargo test` çalıştırmaz; betik (`NUXUT_ACCOUNTS_BUS=session` + geçici `HOME`) her senaryoyu ayrı
//! süreçte çalıştırır (servis durumu süreç geneli):
//!
//! ```sh
//! dbus-run-session -- tools/accounts-dbus-test.sh   # bkz. docs/design/account.md "Test"
//! ```
//!
//! Gerçek sistem bus'ına ya da AccountsService'e asla gidilmez: `NUXUT_ACCOUNTS_BUS=session`
//! yoksa testler hiçbir şey yapmadan döner.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use nuxut_settings::services::accounts::{self, AVATAR_SIDE, AccountState, ErrorKind, Op, OpState, Snapshot};

fn isolated() -> bool {
    let ok = std::env::var(accounts::BUS_ENV).is_ok_and(|v| v == "session");
    if !ok {
        eprintln!("atlandı: {}=session yok (dbus-run-session betiğiyle çalıştırın)", accounts::BUS_ENV);
    }
    ok
}

fn wait_for(what: &str, f: impl Fn(&Snapshot) -> bool) -> Snapshot {
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        let s = accounts::snapshot();
        if f(&s) {
            return s;
        }
        assert!(Instant::now() < end, "zaman aşımı: {what}; son durum {s:?}");
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn ready() -> Snapshot {
    accounts::ensure();
    wait_for("hazır", |s| matches!(s.state, AccountState::Ready(_)))
}

fn real_name(s: &Snapshot) -> &str {
    s.state.user().map_or("", |u| u.real_name.as_str())
}

fn home() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").expect("HOME"))
}

/// `w×h` düz renkli PNG (sol yarı kırmızı, sağ yarı mavi).
fn write_png(path: &Path, w: u32, h: u32) {
    let mut px = Vec::with_capacity((w * h * 4) as usize);
    for _y in 0..h {
        for x in 0..w {
            px.extend_from_slice(if x < w / 2 { &[255, 0, 0, 255] } else { &[0, 0, 255, 255] });
        }
    }
    let f = std::fs::File::create(path).expect("png");
    let mut enc = png::Encoder::new(f, w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().expect("başlık").write_image_data(&px).expect("veri");
}

fn png_size(path: &Path) -> (u32, u32) {
    let d = png::Decoder::new(std::io::BufReader::new(std::fs::File::open(path).expect("aç"))).read_info().expect("png");
    (d.info().width, d.info().height)
}

/// Sahte servis çalışmıyorken: sayfa "hesap hizmeti yok" der, panik yok.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn missing_service() {
    if !isolated() {
        return;
    }
    accounts::ensure();
    let s = wait_for("Missing", |s| s.state != AccountState::Pending);
    assert_eq!(s.state, AccountState::Missing);
    // Yazma istekleri sessizce yok sayılır.
    accounts::set_real_name("Yeni");
    assert_eq!(accounts::snapshot().op, OpState::Idle);
}

/// Okuma, ad yazma (yetki bayrağıyla), geçersiz ad, profil resmi hattı ve `~/.face` yedeği.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn read_and_write() {
    if !isolated() {
        return;
    }
    let s = ready();
    let user = s.state.user().expect("kullanıcı");
    assert_eq!(user.user_name, "deniz");
    assert_eq!(user.real_name, "Deniz Yılmaz");

    accounts::set_real_name("  Deniz Kaya ");
    let s = wait_for("ad yazıldı", |s| real_name(s) == "Deniz Kaya" && s.op == OpState::Idle);
    assert_eq!(s.op, OpState::Idle);

    accounts::set_real_name("Kaya, Deniz");
    assert_eq!(accounts::snapshot().op, OpState::Failed(Op::RealName, ErrorKind::InvalidName));
    accounts::set_real_name("   ");
    assert_eq!(real_name(&accounts::snapshot()), "Deniz Kaya", "boş ad yazılmaz");

    // Kullanıcının kendi ~/.face'i: bir kez yedeklenir.
    let h = home();
    std::fs::write(h.join(".face"), b"eski resim").expect(".face");
    let src = h.join("foto.png");
    write_png(&src, 480, 320);
    accounts::set_avatar_file(src);
    let s = wait_for("resim yazıldı", |s| s.op == OpState::Idle && s.state.user().is_some_and(|u| u.icon_file.is_some()) && s.avatar.is_some());
    let icon = s.state.user().and_then(|u| u.icon_file.clone()).expect("IconFile");
    assert_eq!(png_size(&icon), (AVATAR_SIDE, AVATAR_SIDE), "kare 256² kopya");
    assert_eq!(s.avatar.as_ref().map(|a| a.size()), Some((AVATAR_SIDE, AVATAR_SIDE)));
    let face = std::fs::read(h.join(".face")).expect(".face yazıldı");
    assert!(accounts::is_ours(&face));
    assert_eq!(std::fs::read(h.join(accounts::FACE_BACKUP)).expect("yedek"), b"eski resim");
    let cache = accounts::avatar_cache_path().expect("önbellek");
    assert!(cache.starts_with(&h) || std::env::var_os("XDG_CACHE_HOME").is_some(), "önbellek geçici dizinde");
    assert_eq!(std::fs::read(&cache).expect("önbellek dosyası"), face);

    // Bozuk dosya: hata durumu, servis değişmez.
    let bad = h.join("bozuk.png");
    std::fs::write(&bad, b"resim degil").expect("bozuk");
    accounts::set_avatar_file(bad);
    let s = wait_for("bozuk resim", |s| !s.op.busy());
    assert_eq!(s.op, OpState::Failed(Op::Avatar, ErrorKind::BadImage));
    assert_eq!(s.state.user().and_then(|u| u.icon_file.clone()), Some(icon));
}

/// polkit reddi: sessiz (hata yok), değer değişmez.
#[test]
#[ignore = "dbus-run-session betiği gerekir"]
fn denied_is_silent() {
    if !isolated() {
        return;
    }
    let s = ready();
    let before = real_name(&s).to_owned();
    accounts::set_real_name("Başka Ad");
    assert_eq!(accounts::snapshot().op, OpState::Waiting(Op::RealName));
    let s = wait_for("ret", |s| !s.op.busy());
    assert_eq!(s.op, OpState::Idle, "ret/iptal hata göstermez");
    assert_eq!(real_name(&s), before);
    let src = home().join("foto.png");
    write_png(&src, 64, 64);
    accounts::set_avatar_file(src);
    let s = wait_for("resim reddi", |s| !s.op.busy());
    assert_eq!(s.op, OpState::Idle);
    assert!(!home().join(".face").exists(), "reddedilen resim ~/.face'e yazılmaz");
}
