//! Arka plan servisleri: UI thread'ini bloklamayan işler (D-Bus, portal, palet üretimi).
//! İstekler [`Request`] olarak gider, sonuçlar [`Event`] olarak `App::message`'a döner.
//! Thread'ler ilk istekte açılır (runtime'ın sinyal maskesini miras alsınlar diye olay
//! işleyicisinden).

pub mod accounts;
pub mod app_icons;
pub mod assist;
pub mod bluetooth;
pub mod calendar;
pub mod default_apps;
pub mod displays;
pub mod hypr_status;
pub mod hyprland;
pub mod input;
pub mod look;
pub mod network;
pub mod night_light;
pub mod outputs;
pub mod power;
pub mod printers;
pub mod palette;
pub mod portal;
pub mod previews;
pub mod seen;
pub mod shell_ipc;
pub mod snapshots;
pub mod system_theme;
pub mod timedate;
pub mod wallpaper;
pub mod wallpaper_source;
pub mod widgets;
pub mod xkb;

use std::sync::OnceLock;

use nuxut::wayland::MessageSender;

use crate::app::Msg;

/// UI → servis.
#[derive(Debug, Clone)]
pub enum Request {
    Calendar(calendar::Command),
    Palette(palette::Command),
    /// Ekranlar: shell'e `displays apply/confirm/revert`, liste tazeleme.
    Displays(displays::shell::Command),
    /// Duvar kağıdı seçtir (`tag` yanıtta döner).
    PickWallpaper { tag: String },
    /// Ayar başına "önceki hâline dön" ([`snapshots`]).
    Snapshots(snapshots::Command),
    /// Varsayılan uygulama seçimi ([`default_apps`]).
    DefaultApps(default_apps::Command),
}

/// Servis → UI.
#[derive(Debug, Clone)]
pub enum Event {
    /// Arka planda bir şey hazır (görsel çözüldü): yalnızca yeniden çiz.
    Wake,
    Calendar(calendar::Update),
    Palette(palette::Update),
    Portal(portal::Update),
    /// Shell widget kataloğu yüklendi ([`widgets::catalog`]).
    Widgets,
    /// Bağlı monitör listesi değişti ([`outputs::list`]).
    Outputs,
    /// Shell'in ekran durumu ([`displays::shell`]).
    Displays(displays::shell::Update),
    /// `framework.toml [locale]` değişti (`App::locale_changed`): dil/biçim gösterenler
    /// (Dil ve biçimler satırları, önizleme) yeniden kurulur. Arka plandan gelmez; `App` yayar.
    LocaleChanged,
    /// Kurulu temalar tarandı ya da shell'in görünüm durumu geldi ([`look`]).
    Look,
    /// Hyprland bağlantı satırının durumu geldi ([`hypr_status::source`]).
    Hypr,
    /// Girdi ayarlarının etkin durumu geldi ([`input::refresh`]).
    Input,
    /// Gece ışığının konumu / bugünün güneş olayları geldi ([`night_light::refresh`]).
    NightLight,
    /// Büyüteç ya da ekran klavyesinin canlı durumu ya da eylem sonucu değişti ([`assist`]).
    Assist,
    /// XKB düzen kataloğu yüklendi ([`xkb::catalog`]).
    Xkb,
    /// Tarih ve saat servisinin (timedated) durumu ya da yazma sonucu değişti ([`timedate`]).
    Timedate,
    /// Kullanıcı hesabı (AccountsService) durumu ya da yazma sonucu değişti ([`accounts`]).
    Account,
    /// NetworkManager durumu değişti ([`network`]; Wi-Fi ve VPN sayfaları).
    Network,
    /// BlueZ durumu, eşleştirme sonucu ya da görünürlük sayacı ([`bluetooth`]; Bluetooth sayfası).
    Bluetooth,
    /// Güç durumu ya da yazma sonucu değişti ([`power`]; Sistem › Güç).
    Power,
    /// Yazıcı durumu ya da eylem sonucu değişti ([`printers`]; Sistem › Yazıcılar).
    Printers,
    /// Snapshot listesi yenilendi ([`snapshots::latest_for`]).
    Snapshots,
    /// Geri yükleme bitti; `text` kullanıcıya şeritte gösterilir.
    SnapshotRestored { text: String },
    /// Varsayılan uygulamaların durumu geldi ([`default_apps::refresh`]).
    DefaultApps,
    /// Varsayılan uygulama seçimi başarısız; `text` şeritte gösterilir.
    DefaultAppSet { text: String },
    /// Açılış görünümünün durumu ya da uygulama olayı geldi ([`system_theme`]).
    SystemTheme,
}

static SENDER: OnceLock<MessageSender<Msg>> = OnceLock::new();

/// Runtime başladıktan sonra bir kez çağrılır.
pub fn init(sender: MessageSender<Msg>) {
    let _ = SENDER.set(sender);
}

/// Görsel çözücülerin `notify` geri çağrısı.
pub fn wake() {
    emit(Event::Wake);
}

pub(crate) fn emit(event: Event) {
    if let Some(s) = SENDER.get() {
        let _ = s.send(Msg::Service(event));
    }
}

/// İsteği ilgili servis thread'ine iletir.
pub fn dispatch(request: Request) {
    match request {
        Request::Calendar(c) => calendar::send(c),
        Request::Palette(c) => palette::send(c),
        Request::Displays(c) => displays::shell::send(c),
        Request::PickWallpaper { tag } => portal::pick_wallpaper(tag, nuxut::i18n::t!("settings-services-wallpaper-dialog-title")),
        Request::Snapshots(c) => snapshots::send(c),
        Request::DefaultApps(c) => default_apps::send(c),
    }
}

/// Bağlantıyı varsayılan tarayıcıda açar: xdg-desktop-portal `OpenURI`, olmazsa `xdg-open`
/// (kabuk yok; argüman olarak). Yalnızca http(s) adresleri.
pub fn open_url(url: &str) {
    if !(url.starts_with("https://") || url.starts_with("http://")) || url.len() > 2048 {
        tracing::warn!("açılmayan adres: {url}");
        return;
    }
    let url = url.to_owned();
    let _ = std::thread::Builder::new().name("open-url".into()).spawn(move || {
        let portal = (|| -> zbus::Result<()> {
            let conn = zbus::blocking::Connection::session()?;
            let opts: std::collections::HashMap<&str, zbus::zvariant::Value> = std::collections::HashMap::new();
            conn.call_method(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                Some("org.freedesktop.portal.OpenURI"),
                "OpenURI",
                &("", url.as_str(), opts),
            )?;
            Ok(())
        })();
        if let Err(e) = portal {
            tracing::info!("portal OpenURI yok ({e}); xdg-open");
            // Bu thread zaten arka planda: çocuğu bekle (zombi kalmasın).
            match std::process::Command::new("xdg-open").arg(&url).spawn() {
                Ok(mut child) => {
                    let _ = child.wait();
                }
                Err(e) => tracing::warn!("xdg-open çalıştırılamadı: {e}"),
            }
        }
    });
}

/// Klasörü dosya yöneticisinde açar (`xdg-open <dizin>`; kabuk yok, argüman olarak).
pub fn open_dir(dir: &std::path::Path) {
    if !dir.is_absolute() {
        return;
    }
    let _ = std::fs::create_dir_all(dir);
    match std::process::Command::new("xdg-open").arg(dir).spawn() {
        // Çocuğu arka planda bekle (zombi kalmasın).
        Ok(mut child) => {
            let _ = std::thread::Builder::new().name("xdg-open".into()).spawn(move || child.wait());
        }
        Err(e) => tracing::warn!("xdg-open çalıştırılamadı: {e}"),
    }
}
