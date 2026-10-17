//! `nuxut-settings [sayfa]` — Ayarlar penceresini açar. Zaten açıksa o örneğe sayfayı
//! açtırır ve çıkar.

use std::time::Instant;

use nuxut::wayland::{RunOptions, ipc, run_with};
use nuxut_settings::app::Settings;
use nuxut_settings::registry::PageId;
use nuxut_settings::store::Store;

fn main() {
    let start = Instant::now();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if matches!(args.first().map(String::as_str), Some("-h" | "--help")) {
        println!(
            "kullanım: nuxut-settings [sayfa]\n          nuxut-settings --flow <akış>   (takvim hesabı ekleme akışını aç)\n          nuxut-settings welcome         (ilk açılış: Hoş geldin)\nsayfalar: {}",
            PageId::ALL.map(|p| p.slug()).join(", ")
        );
        return;
    }
    // `--flow <id>`: nuxut-calendard (ör. webcal: bağlantısı) bir akış başlattı; Hesaplar
    // sayfasını o akışla aç.
    let flow = match args.as_slice() {
        [f, id] if f == "--flow" => match valid_flow_id(id) {
            true => Some(id.clone()),
            false => {
                eprintln!("nuxut-settings: geçersiz akış kimliği");
                std::process::exit(2);
            }
        },
        _ => None,
    };
    // `welcome` / `--welcome` (shell ilk açılışta başlatır; ADR-0001 §10.4): Hoş geldin akışı.
    let welcome = matches!(args.first().map(String::as_str), Some("welcome" | "--welcome"));
    let arg = if flow.is_some() { Some("accounts".to_owned()) } else if welcome { None } else { args.first().cloned() };
    let page = match arg.as_deref() {
        Some(s) => match PageId::from_slug(s) {
            Some(p) => p,
            None => {
                eprintln!("nuxut-settings: bilinmeyen sayfa '{s}'");
                std::process::exit(2);
            }
        },
        None => PageId::Appearance,
    };
    // Testler ayrı örnek adıyla çalışır (kullanıcının açık Ayarlar'ına hiçbir komut gitmesin):
    // `NUXUT_SETTINGS_INSTANCE=<ad>` → IPC adı `settings-<ad>`.
    let instance = match std::env::var("NUXUT_SETTINGS_INSTANCE") {
        Ok(n) if !n.is_empty() && n.len() <= 20 && n.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_') => format!("settings-{n}"),
        Ok(_) => {
            eprintln!("nuxut-settings: NUXUT_SETTINGS_INSTANCE yalnızca [a-z0-9_-], en çok 20 karakter");
            std::process::exit(2);
        }
        Err(_) => "settings".to_owned(),
    };
    let listener = match ipc::try_single_instance(&instance) {
        Ok(ipc::Instance::Primary(l)) => Some(l),
        Ok(ipc::Instance::Secondary(client)) => {
            let sent = match &flow {
                Some(id) => client.send_args(&["flow", id.as_str()]),
                None if welcome => client.send_args(&["welcome"]),
                None => client.send_args(&["open", page.slug()]),
            };
            match sent {
                Ok(r) if r.starts_with("error") => eprintln!("nuxut-settings: {r}"),
                Ok(_) => {}
                Err(e) => eprintln!("nuxut-settings: çalışan örneğe ulaşılamadı: {e}"),
            }
            return;
        }
        Err(e) => {
            tracing::warn!("tek örnek kilidi alınamadı ({e}); IPC'siz devam");
            None
        }
    };
    // Katalog: framework.toml [locale] + ortam; metin kuran her şeyden önce (ADR-0010). İkinci
    // örnek yalnızca komut iletip çıktığından yukarıda kurulmaz.
    nuxut::i18n::init(&[nuxut_settings::i18n::source(), nuxut_wallpapers::i18n_source()]);
    let Some(store) = Store::open_default() else {
        eprintln!("nuxut-settings: config dizini bulunamadı ($HOME / $XDG_CONFIG_HOME)");
        std::process::exit(1);
    };
    // "Yeni" rozetlerinin görülme durumu ($XDG_STATE_HOME/nuxut/settings.toml; config'e yazılmaz).
    nuxut_settings::services::seen::init(nuxut_settings::services::seen::default_path());
    let app = Settings::new(store, page, flow, start);
    let app = if welcome { app.with_welcome() } else { app };
    if let Err(e) = run_with(app, RunOptions { ipc: listener }) {
        eprintln!("nuxut-settings: {e}");
        std::process::exit(1);
    }
}

/// Daemon'un akış kimlikleri: kısa, yalnızca harf/rakam/`-`/`_`.
fn valid_flow_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 128 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
