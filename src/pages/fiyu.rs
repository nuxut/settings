//! Maskot Fiyu'nun Ayarlar'daki yerleri (framework `docs/design/mascot.md`; kararlar
//! kullanıcı, 2026-10-13). Tek tablo: [`Spot`] → ruh hâli ve kenar. Yerler `docs/ARCHITECTURE.md`
//! "Maskot Fiyu" bölümünde listelidir.
//!
//! - Yalnız sayfa/kart düzeyinde boş ve hata durumları, arama sonucu yok, Hakkında ve Hoş geldin
//!   bitiş ekranı ("Hazır"). Hoş geldin ilk ekranı marka logosunundur, maskot yok
//!   (2026-10-14). Satır içi tek satırlık notlarda maskot yok.
//! - Ekranda aynı anda en çok bir maskot: bir sayfada iki maskotlu durum aynı anda oluşmaz (yer
//!   seçimi buna göre); arama sonucu yokken sayfada maskot varsa aramadaki maskot çizilmez
//!   ([`crate::block::Block::mascot`], [`crate::page::Page::mascot`]).
//! - Gizleme (`framework.toml [appearance] mascot = false`) bileşenin işidir: `Mascot::measure`
//!   ve `EmptyState` görseli sıfır yer kaplar; burada ek iş yok.

use std::time::Instant;

use nuxut::Theme;
use nuxut::TextSystem;
use nuxut::ui::{EMPTY_MASCOT_SIZE, EmptyState, Mood, Ui};

/// Maskotun Ayarlar'daki yeri.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spot {
    /// Hakkında sayfasının marka başlığı.
    About,
    /// Hoş geldin bitiş ekranı ("Hazır"). İlk ekranda maskot yok: orada marka logosu her zaman
    /// görünür (aynı ekranda iki kırlangıç olmasın; 2026-10-14).
    WelcomeDone,
    /// Kenar çubuğu araması, sonuç yok.
    SearchNoResults,
    /// Ekranlar: kabuk çalışmıyor / yanıt vermiyor.
    DisplaysUnavailable,
    /// Varsayılan uygulamalar: kabuk çalışmıyor / okunamadı.
    DefaultAppsUnavailable,
    /// Takvim hesapları: takvim servisine ulaşılamadı.
    AccountsUnavailable,
    /// Takvim hesapları: hesap yok.
    AccountsEmpty,
    /// Bildirimler › uygulama listesi boş.
    NotifAppsEmpty,
    /// Wi-Fi › kayıtlı ağ yok.
    WifiSavedEmpty,
    /// VPN › kayıtlı bağlantı yok.
    VpnEmpty,
    /// Bluetooth: BlueZ çalışmıyor / adaptör yok / servis açılamadı (sayfanın tek içeriği).
    BluetoothUnavailable,
    /// Bluetooth › eşleşmiş cihaz yok.
    BluetoothPairedEmpty,
    /// Yazıcılar: CUPS kurulu değil / çalışmıyor / okunamadı (sayfanın tek içeriği).
    PrintersUnavailable,
    /// Güç: UPower ve power-profiles-daemon çalışmıyor (sayfanın tek içeriği).
    PowerUnavailable,
    /// Güç: pil yok ve profil hizmeti yok (masaüstü, VM; sayfanın tek içeriği).
    PowerEmpty,
    /// Yazıcılar › yazıcı yok.
    PrintersEmpty,
}

/// Hakkında ve Hoş geldin'deki büyük maskotun kenarı (mascot.md: 128–160).
pub const FEATURE_MASCOT_SIZE: f32 = 128.0;

impl Spot {
    pub const ALL: [Spot; 16] = [
        Spot::About,
        Spot::WelcomeDone,
        Spot::SearchNoResults,
        Spot::DisplaysUnavailable,
        Spot::DefaultAppsUnavailable,
        Spot::AccountsUnavailable,
        Spot::AccountsEmpty,
        Spot::NotifAppsEmpty,
        Spot::WifiSavedEmpty,
        Spot::VpnEmpty,
        Spot::BluetoothUnavailable,
        Spot::BluetoothPairedEmpty,
        Spot::PrintersUnavailable,
        Spot::PrintersEmpty,
        Spot::PowerUnavailable,
        Spot::PowerEmpty,
    ];

    /// Ruh hâli (mascot.md tablosu: boş → `Empty`, sonuç yok → `NoResults`, hata → `Error`).
    pub fn mood(self) -> Mood {
        match self {
            Spot::About => Mood::Hello,
            Spot::WelcomeDone => Mood::Celebrate,
            Spot::SearchNoResults => Mood::NoResults,
            Spot::DisplaysUnavailable | Spot::DefaultAppsUnavailable | Spot::AccountsUnavailable | Spot::BluetoothUnavailable | Spot::PrintersUnavailable | Spot::PowerUnavailable => Mood::Error,
            Spot::AccountsEmpty | Spot::NotifAppsEmpty | Spot::WifiSavedEmpty | Spot::VpnEmpty | Spot::BluetoothPairedEmpty | Spot::PrintersEmpty | Spot::PowerEmpty => Mood::Empty,
        }
    }

    /// Tercih edilen kenar (alan darsa `EmptyState` küçültür).
    pub fn size(self) -> f32 {
        match self {
            Spot::About | Spot::WelcomeDone => FEATURE_MASCOT_SIZE,
            _ => EMPTY_MASCOT_SIZE,
        }
    }
}

/// Maskotlu boş/hata durumu: `EmptyState` + yeri (testler ve "tek maskot" kuralı için).
#[derive(Debug)]
pub struct FiyuEmpty {
    spot: Spot,
    state: EmptyState,
}

impl FiyuEmpty {
    pub fn new(theme: &Theme, spot: Spot, title: &str, body: Option<&str>) -> Self {
        let mut state = EmptyState::new(theme, title).with_mascot_size(spot.size()).with_mascot(theme, spot.mood());
        state.set_body(theme, body);
        Self { spot, state }
    }

    pub fn spot(&self) -> Spot {
        self.spot
    }

    pub fn title(&self) -> &str {
        self.state.title()
    }

    /// Metinleri günceller (değişmediyse yerleşim korunur).
    pub fn set_text(&mut self, theme: &Theme, title: &str, body: Option<&str>) {
        self.state.set_title(title);
        self.state.set_body(theme, body);
    }

    /// `width` genişlikteki yükseklik (dolgu dahil; maskot gizliyse onun yeri yok).
    pub fn height(&mut self, text: &mut TextSystem, theme: &Theme, width: f32) -> f32 {
        self.state.measure(text, theme, width).height
    }

    pub fn set_rect(&mut self, rect: nuxut::Rect) {
        self.state.set_rect(rect);
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.state.set_theme(theme, now);
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        self.state.draw(ui);
    }

    /// Son yerleşimde maskot çizildi mi (gizliyse ya da sığmadıysa `false`).
    pub fn art_shown(&self) -> bool {
        self.state.last_layout().is_some_and(|l| l.art.is_some())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// `[appearance] mascot = false` teması.
    pub(crate) fn hidden_theme() -> Theme {
        Theme::parse("[appearance]\nmascot = false\n").unwrap().0
    }

    /// Yerleşim tablosu (kullanıcı kararı 2026-10-13; Hoş geldin ilk ekranı çıktı
    /// 2026-10-14; mascot.md ruh hâli tablosu).
    #[test]
    fn every_spot_has_the_decided_mood_and_size() {
        let want = [
            (Spot::About, Mood::Hello, 128.0),
            (Spot::WelcomeDone, Mood::Celebrate, 128.0),
            (Spot::SearchNoResults, Mood::NoResults, 96.0),
            (Spot::DisplaysUnavailable, Mood::Error, 96.0),
            (Spot::DefaultAppsUnavailable, Mood::Error, 96.0),
            (Spot::AccountsUnavailable, Mood::Error, 96.0),
            (Spot::AccountsEmpty, Mood::Empty, 96.0),
            (Spot::NotifAppsEmpty, Mood::Empty, 96.0),
            (Spot::WifiSavedEmpty, Mood::Empty, 96.0),
            (Spot::VpnEmpty, Mood::Empty, 96.0),
            (Spot::BluetoothUnavailable, Mood::Error, 96.0),
            (Spot::BluetoothPairedEmpty, Mood::Empty, 96.0),
            (Spot::PrintersUnavailable, Mood::Error, 96.0),
            (Spot::PrintersEmpty, Mood::Empty, 96.0),
            (Spot::PowerUnavailable, Mood::Error, 96.0),
            (Spot::PowerEmpty, Mood::Empty, 96.0),
        ];
        assert_eq!(want.len(), Spot::ALL.len());
        for (spot, mood, size) in want {
            assert_eq!(spot.mood(), mood, "{spot:?}");
            assert_eq!(spot.size(), size, "{spot:?}");
            assert!((nuxut::ui::MASCOT_MIN_SIZE..=nuxut::ui::MASCOT_MAX_SIZE).contains(&spot.size()));
        }
    }

    /// Gizliyken maskot yer kaplamaz: yükseklik tam maskot payı (kenar + aralık) kadar küçülür.
    #[test]
    fn hidden_mascot_takes_no_space() {
        let shown = Theme::default();
        let hidden = hidden_theme();
        assert!(shown.appearance.mascot && !hidden.appearance.mascot);
        let mut text = TextSystem::new();
        let w = shown.space(150);
        for spot in Spot::ALL {
            let mut a = FiyuEmpty::new(&shown, spot, "Başlık", Some("Gövde"));
            let mut b = FiyuEmpty::new(&hidden, spot, "Başlık", Some("Gövde"));
            let ha = a.height(&mut text, &shown, w);
            let hb = b.height(&mut text, &hidden, w);
            let art = spot.size() + shown.space(4);
            assert!((ha - hb - art).abs() <= 1.0, "{spot:?}: {ha} - {hb} ≠ {art}");
        }
    }
}
