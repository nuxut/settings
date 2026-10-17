//! Kurulum dosyaları (`dist/`): .desktop girdisi pencere app-id'siyle ve ikon adıyla tutarlı,
//! ikon framework'ün marka SVG'sinin birebir kopyası (geometri kayarsa test kırılır).

const DESKTOP: &str = include_str!("../dist/nuxut-settings.desktop");
const ICON: &str = include_str!("../dist/icons/hicolor/scalable/apps/nuxut-settings.svg");

fn entry(key: &str) -> Option<&'static str> {
    DESKTOP.lines().find_map(|l| l.strip_prefix(key)?.strip_prefix('='))
}

#[test]
fn desktop_entry_matches_window_and_icon() {
    assert!(DESKTOP.contains("[Desktop Entry]"));
    assert_eq!(entry("Type"), Some("Application"));
    assert_eq!(entry("StartupWMClass"), Some(nuxut_settings::app::SETTINGS_APP_ID));
    assert_eq!(entry("Icon"), Some("nuxut-settings"));
    // Ayarlar URL/dosya almaz (yalnızca sayfa adı): alan kodu yok.
    assert_eq!(entry("Exec"), Some("nuxut-settings"));
    assert!(entry("NoDisplay").is_none(), "menüde görünen tek girdi");
    assert!(entry("Categories").is_some_and(|c| c.split(';').any(|c| c == "Settings")));
    assert!(entry("Name").is_some() && entry("Name[tr]").is_some());
}

#[test]
fn icon_is_verbatim_brand_svg() {
    let svg = &ICON[ICON.find("<svg").expect("svg kökü")..];
    assert_eq!(svg.as_bytes(), nuxut::ui::BRAND_SVG, "dist ikonu framework'ün kara-sagan.svg'si ile aynı olmalı");
}

/// Hoş geldin penceresinin gizli girdisi: görev çubuğu pencereyi Ayarlar ikonuyla tanır, menüde
/// görünmez.
#[test]
fn welcome_entry_is_hidden_and_matches_window() {
    const WELCOME: &str = include_str!("../dist/nuxut-welcome.desktop");
    let entry = |key: &str| WELCOME.lines().find_map(|l| l.strip_prefix(key)?.strip_prefix('='));
    assert_eq!(entry("StartupWMClass"), Some(nuxut_settings::app::WELCOME_APP_ID));
    assert_eq!(entry("NoDisplay"), Some("true"));
    assert_eq!(entry("Icon"), Some("nuxut-settings"));
    assert_eq!(entry("Exec"), Some("nuxut-settings --welcome"));
}
