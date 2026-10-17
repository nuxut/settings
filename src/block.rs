//! Sayfa yapı taşı: bir sayfa dikey olarak dizilmiş [`Block`]'lardan oluşur (ayar satırları,
//! önizleme kartı, easing editörü…). Bloklar değerleri her zaman `Store`'dan okur ve
//! değişiklikleri [`Action`] olarak bildirir; uygulamayı (yazım, geçmiş) `App` yapar.

use std::time::Instant;

use nuxut::ui::input::{KeyEvent, PointerEvent};
use nuxut::ui::Ui;
use nuxut::ui::input::CursorShape;
use nuxut::{Point, Rect, TextSystem, Theme};
use toml_edit::Value;

use crate::registry::PageId;
use crate::store::{FileId, KeyPath, Store};

/// Bloğun istediği yan etki.
#[derive(Debug, Clone)]
pub enum Action {
    /// Tek anahtar. `label` geçmişte ve geri al şeridinde görünür.
    Set { label: String, file: FileId, path: KeyPath, value: Option<Value> },
    /// Birlikte geri alınan çok anahtarlı eylem.
    Group { label: String, writes: Vec<(FileId, KeyPath, Option<Value>)> },
    /// Kaydırıcı sürüklemesi bitti: sonraki değişiklik yeni geçmiş kaydı olur.
    Seal,
    /// Bir sayfaya git (ör. "Duvar kağıdı" bağlantısı).
    Navigate(PageId),
    /// Kayıttaki ayara git: sayfasını aç, satıra kaydır ve vurgula (ör. bar widget satırı tıklandı).
    Reveal(&'static str),
    /// Bloğa özgü istek (D-Bus, portal, palet); `App` yönlendirir.
    Service(crate::services::Request),
    /// Klavye odağını iste (metin alanı tıklandı).
    Focus,
    /// Metni sistem panosuna kopyala. Compositor girdi serial'i istediği için yalnızca
    /// tıklama/tuş işleyicisinden; sonucu `App` şeritte bildirir.
    Copy(String),
}

/// Olay ve yerleşim bağlamı.
pub struct Env<'a> {
    pub theme: &'a Theme,
    pub store: &'a Store,
    pub text: &'a mut TextSystem,
    pub now: Instant,
    pub actions: &'a mut Vec<Action>,
}

impl std::fmt::Debug for Env<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Env").field("now", &self.now).finish_non_exhaustive()
    }
}

impl Env<'_> {
    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn set(&mut self, label: impl Into<String>, file: FileId, path: KeyPath, value: Option<Value>) {
        self.actions.push(Action::Set { label: label.into(), file, path, value });
    }
}

pub trait Block: std::fmt::Debug {
    /// Bölüm başlığı (aynı bölümdeki ardışık bloklar tek kartta toplanır). `None`: kartsız.
    fn section(&self) -> Option<&str>;

    /// Kayıttaki kimlik (arama sonucundan kaydırma ve vurgulama için).
    fn id(&self) -> Option<&'static str> {
        None
    }

    /// Görünür mü (ör. gölge kapalıyken bulanıklık satırı gizli).
    fn visible(&self, _store: &Store) -> bool {
        true
    }

    /// `width` genişlikteki yükseklik (çağrı ucuz olmalı; her karede çağrılabilir).
    fn height(&mut self, env: &mut Env, width: f32) -> f32;

    /// Yüzey koordinatında yerleşim (kaydırma ile her karede değişebilir).
    fn place(&mut self, env: &mut Env, rect: Rect);

    /// İşaretçi; `true` = yeniden çizim gerekir.
    fn pointer(&mut self, env: &mut Env, event: &PointerEvent) -> bool;

    /// Tekerlek bu noktada bloğun kendi kaydırılan alanına mı gider (ör. font listesi)?
    /// `true` ise sayfa kaymaz, olay [`Block::pointer`]'a iletilir.
    fn wants_scroll(&self, _p: Point) -> bool {
        false
    }

    /// Klavye (blok odaktayken). `true` = tüketildi.
    fn key(&mut self, _env: &mut Env, _event: &KeyEvent) -> bool {
        false
    }

    /// Tüm tuşları kendisi mi istiyor (kısayol yakalanırken: Tab, Ctrl+Z, Ctrl+W… uygulamaya
    /// gitmez, yakalanır)?
    fn captures_keys(&self) -> bool {
        false
    }

    fn focusable(&self) -> bool {
        false
    }

    /// Klavye odağı (Tab gezinmesi ya da tıklama).
    fn set_focus(&mut self, _focused: bool, _visible: bool, _now: Instant) {}

    /// Store değişti (dış düzenleme, geri al): bileşenler değere akar.
    fn sync(&mut self, env: &mut Env);

    fn set_theme(&mut self, theme: &Theme, now: Instant);

    fn draw(&mut self, ui: &mut Ui);

    /// IME / metin girişi için (yalnızca metin alanı olan bloklar).
    fn text_input(&mut self) -> Option<&mut nuxut::ui::TextInput> {
        None
    }

    /// İmleç şekli (yüzey koordinatı); `None` = bu blok söz sahibi değil.
    fn cursor(&self, _p: Point) -> Option<CursorShape> {
        None
    }

    /// Test senaryosu (`NUXUT_SETTINGS_DEBUG=expand=<id>`): bloğun açılabilir kısmını aç.
    fn demo_expand(&mut self, _now: Instant) {}

    /// Bu kimlik bu blokta (ya da içindeki bir satırda) mı? Açılır satır içindekileri de sayar.
    fn contains_id(&self, id: &str) -> bool {
        self.id() == Some(id)
    }

    /// Arama sonucundan: `id`'yi göster (gerekirse açılır satırı aç) ve bloğun üstüne göre
    /// satırın dikey konumunu döndür (açılmış hâline göre). Kendi içinde parlatan blok
    /// `(konum, true)` döndürür; `false` ise sayfa bloğun tamamını parlatır.
    fn reveal(&mut self, _env: &mut Env, id: &str) -> Option<(f32, bool)> {
        (self.id() == Some(id)).then_some((0.0, false))
    }

    /// Açılır satır: `(ad, açık mı)`; sayfa durumunu ([`PageUi`](crate::page_ui::PageUi)) eşlemek için.
    fn disclosure(&self) -> Option<(&'static str, bool)> {
        None
    }

    /// Açılır satırı aç/kapat (sayfa durumundan).
    fn set_disclosed(&mut self, _open: bool, _now: Instant) {}

    /// Sayfa durumu değişti (içinde `ui_if`'li satır taşıyan bloklar için; sayfanın kendi
    /// satırlarını [`Page`](crate::page::Page) süzer).
    fn sync_ui(&mut self, _ui: &crate::page_ui::PageUi, _now: Instant) {}

    /// Bloğun istediği sayfa durumu değişikliğini (ör. Bar önizlemesinde tıklanan widget) `ui`'a
    /// yazar; sayfa işaretçi/tuş olayından sonra sorar, değiştiyse tüm bloklara
    /// ([`Block::sync_ui`]) ve görünürlüklere akıtır. `true` = bir şey yazıldı.
    fn take_ui(&mut self, _ui: &mut crate::page_ui::PageUi) -> bool {
        false
    }

    /// Tab: odak bloğun içinde bir sonraki parçaya geçebildiyse `true` (açılır satırın
    /// satırları); `false` ise sayfa sonraki bloğa geçer. Döndürülen konum yok: sayfa bloğu
    /// görünür tutar.
    fn focus_step(&mut self, _forward: bool, _now: Instant) -> bool {
        false
    }

    /// Kaplama katmanında açık bir parça (açılır seçimin menüsü) var mı? Varken işaretçi
    /// olayları yalnızca bu bloğa gider (menü dışına tıklama onu kapatır, alttaki satırı değil).
    fn overlay_open(&self) -> bool {
        false
    }

    /// Blok şu an maskotlu bir durum mu gösteriyor (boş/hata; `[appearance] mascot`'tan
    /// bağımsız)? Ekranda en çok bir maskot kuralı ve yerleşim testleri için
    /// ([`crate::pages::fiyu`]).
    fn mascot(&self) -> Option<crate::pages::fiyu::Spot> {
        None
    }

    /// Ekran okuyucu düğümleri (framework ADR-0011): blok kendi alt ağacını framework
    /// bileşenlerinin `access` yöntemleriyle `tree`'nin şu anki ebeveyni altına ekler (son
    /// `place` dikdörtgenleri). AT tıklaması düğüm ortasına işaretçi tıklaması olarak gelir
    /// (`app/a11y.rs`); ayrı eylem kodu gerekmez. Varsayılan: blok ağaçta yok (henüz benimsenmedi).
    fn access(&self, _tree: &mut nuxut::access::AccessTree) {}

    /// Arka plan servisinden gelen yanıt.
    fn service_event(&mut self, _env: &mut Env, _event: &crate::services::Event) -> bool {
        false
    }
}
