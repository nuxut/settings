//! Bildirimsel ayar kaydı: basit satırlar tek tabloda tanımlanır; çizim, arama, sıfırlama ve
//! geri al aynı tanımdan çıkar. Aralıklar framework/shell doğrulama sınırlarının içindedir
//! (birim testi). shell.toml satırlarının seçenekleri ve sınırları şema yüklüyse shell'in
//! şemasından gelir ([`SettingDesc::choices`], [`SettingDesc::bounds`]); buradaki tablolar
//! yalnızca Türkçe etiketleri ve şema yokken yedeği taşır. Tablodaki metinler kaynak Türkçedir;
//! görünen metin katalogdan, etkin dilde (erişimciler; ARCHITECTURE "Kayıt metinleri").
//! Bkz. `docs/ARCHITECTURE.md` §4.

/// Kaynak (Türkçe) metin tablosu işareti: ifadeyi aynen döner. Tablolar kataloğun yedeği ve
/// dilden bağımsız arama/kimliktir; görünen metin katalogdan (ARCHITECTURE "Kayıt metinleri").
/// i18n bekçisi yalnızca bu makronun ve `s!` satırlarının içini kabul eder (`i18n-allow.txt`).
macro_rules! source_text {
    ($e:expr) => {
        $e
    };
}

use nuxut::i18n::t;
use toml_edit::Value;

use crate::defaults::{DESKTOP_CURSOR_MAX, DESKTOP_CURSOR_MIN};
use crate::page_ui::PageUi;
use crate::pages::bar_preview::{BarEnd, end_widget, selected, selected_end};
use crate::services::default_apps::Role as DefaultRole;
use crate::services::widgets::{self, SettingKind, SettingSpec};
use crate::store::{FileId, KeyPath, Store};
use crate::typography::FontRole;

/// Ayarlar sayfaları. Bildirim sırası kenar çubuğu sırasıdır (`ALL`; arama sonuçları da bu
/// sırayla dizilir — `Ord`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PageId {
    Appearance,
    Wallpaper,
    Fonts,
    Motion,
    Accessibility,
    Bar,
    Panels,
    Windows,
    Displays,
    Keyboard,
    Pointer,
    Keybinds,
    Wifi,
    Vpn,
    Bluetooth,
    Notifications,
    Sound,
    Power,
    Printers,
    Region,
    DateTime,
    DefaultApps,
    User,
    Accounts,
    About,
}

impl PageId {
    /// Kenar çubuğu sırası (`docs/design/settings-ux.md` §2.2): Kişisel · Masaüstü · Bağlantılar · Sistem (Bağlantılar: karar, 2026-10-13).
    /// Grup sınırları [`Self::group`]'tan çıkar; aynı grubun sayfaları ardışık olmalı (testli).
    pub const ALL: [PageId; 25] = [
        PageId::Appearance,
        PageId::Wallpaper,
        PageId::Fonts,
        PageId::Motion,
        PageId::Accessibility,
        PageId::Bar,
        PageId::Panels,
        PageId::Windows,
        PageId::Displays,
        PageId::Keyboard,
        PageId::Pointer,
        PageId::Keybinds,
        PageId::Wifi,
        PageId::Vpn,
        PageId::Bluetooth,
        PageId::Notifications,
        PageId::Sound,
        PageId::Power,
        PageId::Printers,
        PageId::Region,
        PageId::DateTime,
        PageId::DefaultApps,
        PageId::User,
        PageId::Accounts,
        PageId::About,
    ];

    /// Sayfa adı, etkin dilde (`settings-page-<slug>`).
    pub fn title(self) -> &'static str {
        crate::i18n::text_or(&format!("settings-page-{}", self.slug()), self.source_title())
    }

    /// Kaynak (Türkçe) sayfa adı: kataloğun yedeği ve dile bağımsız arama/komut adı.
    pub fn source_title(self) -> &'static str {
        source_text!(match self {
            PageId::Appearance => "Görünüm",
            PageId::Wallpaper => "Duvar kağıdı",
            PageId::Fonts => "Yazı",
            PageId::Motion => "Hareket",
            PageId::Accessibility => "Erişilebilirlik",
            PageId::Bar => "Bar",
            PageId::Panels => "Paneller",
            PageId::Windows => "Pencereler",
            PageId::Displays => "Ekranlar",
            PageId::Keyboard => "Klavye",
            PageId::Pointer => "Fare ve dokunmatik yüzey",
            PageId::Keybinds => "Kısayollar",
            PageId::Wifi => "Wi-Fi",
            PageId::Vpn => "VPN",
            PageId::Bluetooth => "Bluetooth",
            PageId::Notifications => "Bildirimler",
            PageId::Sound => "Ses",
            PageId::Power => "Güç",
            PageId::Printers => "Yazıcılar",
            PageId::Region => "Bölge ve dil",
            PageId::DateTime => "Tarih ve saat",
            PageId::DefaultApps => "Varsayılan uygulamalar",
            PageId::User => "Kullanıcı hesabı",
            PageId::Accounts => "Hesaplar",
            PageId::About => "Hakkında",
        })
    }

    /// Kenar çubuğu ikonu.
    pub fn icon(self) -> nuxut::ui::IconName {
        use nuxut::ui::IconName;
        match self {
            PageId::Appearance => IconName::Palette,
            PageId::Wallpaper => IconName::Wallpaper,
            PageId::Fonts => IconName::Font,
            PageId::Motion => IconName::Animation,
            // Erişilebilirlik ikonu framework'te yok (istendi); en yakını göz.
            PageId::Accessibility => IconName::Accessibility,
            PageId::Bar => IconName::Panel,
            // Kenar çekmecesi ikonu (`dock_to_right`) framework'te yok (F5); en yakını.
            PageId::Panels => IconName::Widgets,
            PageId::Windows => IconName::Window,
            PageId::Displays => IconName::Monitor,
            // Kısayollar da `Keyboard` kullanıyor; framework'te ayrı kısayol/dokunmatik yüzey ikonu yok.
            PageId::Keyboard => IconName::Keyboard,
            PageId::Pointer => IconName::Mouse,
            PageId::Keybinds => IconName::Keyboard,
            PageId::Wifi => IconName::Wifi,
            PageId::Vpn => IconName::Vpn,
            PageId::Bluetooth => IconName::Bluetooth,
            PageId::Notifications => IconName::Notifications,
            PageId::Sound => IconName::VolumeHigh,
            PageId::Power => IconName::Power,
            PageId::Printers => IconName::Print,
            PageId::Region => IconName::Language,
            PageId::DateTime => IconName::Schedule,
            PageId::DefaultApps => IconName::Apps,
            PageId::User => IconName::Person,
            PageId::Accounts => IconName::AccountCircle,
            PageId::About => IconName::Info,
        }
    }

    /// Komut satırı / IPC adı (`nuxut-settings hareket`).
    pub fn slug(self) -> &'static str {
        match self {
            PageId::Appearance => "appearance",
            PageId::Wallpaper => "wallpaper",
            PageId::Fonts => "fonts",
            PageId::Motion => "motion",
            PageId::Accessibility => "accessibility",
            PageId::Bar => "bar",
            PageId::Panels => "panels",
            PageId::Windows => "windows",
            PageId::Displays => "displays",
            PageId::Keyboard => "keyboard",
            PageId::Pointer => "pointer",
            PageId::Keybinds => "keybinds",
            PageId::Wifi => "wifi",
            PageId::Vpn => "vpn",
            PageId::Bluetooth => "bluetooth",
            PageId::Notifications => "notifications",
            PageId::Sound => "sound",
            PageId::Power => "power",
            PageId::Printers => "printers",
            PageId::Region => "region",
            PageId::DateTime => "datetime",
            PageId::DefaultApps => "default-apps",
            PageId::User => "user",
            PageId::Accounts => "accounts",
            PageId::About => "about",
        }
    }

    /// Slug ya da başlık (etkin dilde ya da Türkçe, aksansız da). Eski adlar: `shell` (kaldırılan Shell
    /// sayfası; çerçevesi Bar'ın son kartı) → Bar.
    pub fn from_slug(s: &str) -> Option<Self> {
        let s = fold(s);
        if s == "shell" {
            return Some(PageId::Bar);
        }
        if matches!(s.as_str(), "mouse" | "touchpad" | "fare" | "dokunmatik") {
            return Some(PageId::Pointer);
        }
        // "hesap"/"account": Kullanıcı hesabı; çoğul "hesaplar"/"accounts" takvim hesapları.
        if matches!(s.as_str(), "account" | "hesap" | "kullanici" | "users") {
            return Some(PageId::User);
        }
        if matches!(s.as_str(), "defaults" | "default_apps" | "varsayilan") {
            return Some(PageId::DefaultApps);
        }
        if matches!(s.as_str(), "wi-fi" | "wlan" | "kablosuz" | "wireless" | "network" | "ag") {
            return Some(PageId::Wifi);
        }
        if matches!(s.as_str(), "a11y" | "erisilebilirlik" | "erisim" | "buyutec" | "magnifier" | "osk") {
            return Some(PageId::Accessibility);
        }
        if matches!(s.as_str(), "guc" | "pil" | "battery" | "enerji" | "energy") {
            return Some(PageId::Power);
        }
        if matches!(s.as_str(), "yazici" | "printer" | "cups" | "yazdir" | "print") {
            return Some(PageId::Printers);
        }
        Self::ALL.into_iter().find(|p| p.slug() == s || fold(p.title()) == s || fold(p.source_title()) == s)
    }

    /// Kenar çubuğu grubu (`docs/design/settings-ux.md` §2.2).
    pub fn group(self) -> NavGroup {
        match self {
            PageId::Appearance | PageId::Wallpaper | PageId::Fonts | PageId::Motion | PageId::Accessibility => NavGroup::Personal,
            PageId::Bar | PageId::Panels | PageId::Windows | PageId::Displays | PageId::Keyboard | PageId::Pointer | PageId::Keybinds => NavGroup::Desktop,
            PageId::Wifi | PageId::Vpn | PageId::Bluetooth => NavGroup::Connections,
            PageId::Notifications | PageId::Sound | PageId::Power | PageId::Printers | PageId::Region | PageId::DateTime | PageId::DefaultApps | PageId::User | PageId::Accounts | PageId::About => NavGroup::System,
        }
    }

    /// Ayna satırların sayfadaki kartı, sayfada kendinden önce birincil satır yoksa (kayıtta ayna
    /// satır önce geliyor): Erişilebilirlik'te Görünüm/Yazı/Hareket'in erişilebilirlik satırları
    /// "Görme" kartında toplanır. `None`: satırın kendi bölümü.
    pub fn mirror_home(self) -> Option<&'static str> {
        match self {
            PageId::Accessibility => Some(source_text!("Görme")),
            _ => None,
        }
    }

    /// Kenar çubuğunda (ve aramada) görünür mü. Donanıma/hizmete bağlı sayfalar veri yokken
    /// gizlenir: Güç (`services::power::page_listed`: yalnız servis açılamazsa; veri yoksa sayfada
    /// boş durum — `docs/design/power-page.md`).
    pub fn listed(self) -> bool {
        match self {
            PageId::Power => crate::services::power::page_listed() || lock_cards_known(),
            _ => true,
        }
    }
}

/// Kenar çubuğundaki sayfalar ([`PageId::listed`]), `ALL` sırasında. Liste pencere kurulurken
/// bir kez alınır (`App`).
pub fn nav_pages() -> Vec<PageId> {
    PageId::ALL.into_iter().filter(|p| p.listed()).collect()
}

/// Kenar çubuğu başlıkları: `(grubun ilk sayfasının `pages` indisi, grup adı)`, sırayla.
pub fn nav_groups(pages: &[PageId]) -> Vec<(usize, &'static str)> {
    let mut out = Vec::new();
    for (i, p) in pages.iter().enumerate() {
        if i == 0 || pages[i - 1].group() != p.group() {
            out.push((i, p.group().title()));
        }
    }
    out
}

/// Kenar çubuğu grupları.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NavGroup {
    Personal,
    Desktop,
    /// Bağlantılar (karar, 2026-10-13): Masaüstü ile Sistem arasında; Wi-Fi, VPN,
    /// Bluetooth (t115; ileride Paylaşım, Yazıcılar).
    Connections,
    System,
}

impl NavGroup {
    pub const ALL: [NavGroup; 4] = [NavGroup::Personal, NavGroup::Desktop, NavGroup::Connections, NavGroup::System];

    /// Grup adı, etkin dilde (`settings-nav-group-*`).
    pub fn title(self) -> &'static str {
        let (key, source) = source_text!(match self {
            NavGroup::Personal => ("settings-nav-group-personal", "Kişisel"),
            NavGroup::Desktop => ("settings-nav-group-desktop", "Masaüstü"),
            NavGroup::Connections => ("settings-nav-group-connections", "Bağlantılar"),
            NavGroup::System => ("settings-nav-group-system", "Sistem"),
        });
        crate::i18n::text_or(key, source)
    }
}

/// Dosyaya yazılmayan sayfa durumuna bağlı görünürlük (§3.3) ve onu kuran adım: arama gizli bir
/// satıra giderken `reveal` sayfa durumunu satırın görüneceği hâle getirir (ör. widget'ı seçer).
/// Sözleşme (testli): varsayılan durumdan `reveal` sonrası `shown` doğru. `Store` da verilir:
/// sayfa durumunun anlamı config'e bağlı olabilir (ör. seçili widget bar'ın ucunda mı).
#[derive(Debug, Clone, Copy)]
pub struct UiCond {
    pub shown: fn(&PageUi, &Store) -> bool,
    pub reveal: fn(&mut PageUi, &Store),
}

/// Ayarın kademesi (§3.1). Koşullu kademe ayrı değildir: `visible_if` / `ui_if`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tier {
    /// Hep açık.
    #[default]
    Basic,
    /// Kartın sonundaki açılır satırın içinde; ad [`DETAILS`]'teki bir kayıttır. Aranabilir:
    /// arama sonucu açılır satırı açar, satıra kaydırır, parlatır.
    Detail(&'static str),
}

/// Açılır satırın sunumu: başlık `name` (kaynak Türkçe; `Tier::Detail` kimliği), altında kısa
/// açıklama, kapalıyken özet. Görünen metin `settings-detail-<key>-label/-description`, etkin dilde
/// ("11 renk · 2 tanesi elle ayarlı").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetailGroup {
    /// Katalog anahtarının parçası.
    pub key: &'static str,
    pub name: &'static str,
    pub subtitle: &'static str,
    /// Özette sayılan şey ("renk", "ayar").
    pub noun: Counted,
}

impl DetailGroup {
    /// Başlık, etkin dilde.
    pub fn label(&self) -> &'static str {
        crate::i18n::text_or(&format!("settings-detail-{}-label", self.key), self.name)
    }

    /// Alt yazı, etkin dilde.
    pub fn description(&self) -> &'static str {
        crate::i18n::text_or(&format!("settings-detail-{}-description", self.key), self.subtitle)
    }
}

/// Açılır satır özetinde sayılan şey; her biri kendi tam cümle mesajını seçer (ad ve ek
/// yapıştırılmaz).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Counted {
    Settings,
    Colors,
}

/// Kayıttaki açılır satırlar (sayfa başına en çok bir; sayfa parçaları W1–W4 doldurur).
pub static DETAILS: &[DetailGroup] = source_text!(&[
    // ---- W2/W4 (Yazı, Hareket, Pencereler) ----
    DetailGroup { key: "typography", name: "Boyut oranları ve ikon yazı tipi", subtitle: "Rol başına boyut, ikonların yazı tipi ve boyutu", noun: Counted::Settings },
    DetailGroup { key: "motion", name: "Hareketi tek tek ayarla", subtitle: "Hareket türü ve her öğe grubunun yayı ya da eğrisi", noun: Counted::Settings },
    DetailGroup { key: "compositor", name: "Masaüstüyle ortaklık", subtitle: "Nuxut'un pencerelere ve masaüstü ayar dosyasına uyguladıkları", noun: Counted::Settings },
    // W3 Görünüm › Renkler: vurgu dışındaki on rol.
    DetailGroup { key: "colors", name: "Renkleri tek tek ayarla", subtitle: "Yüzey, yazı, hata ve çizgi renkleri", noun: Counted::Colors },
]);

pub fn detail_group(name: &str) -> Option<&'static DetailGroup> {
    DETAILS.iter().find(|d| d.name == name)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceStyle {
    /// Yan yana haplar (2–4 kısa seçenek).
    Segmented,
    /// Başlığın altında, satıra sarılan haplar (çok seçenek).
    Chips,
    /// Satır içi açılır seçim (5+ seçenek ya da uzun etiket, §5.1); menü kaplama katmanında
    /// açılır, satırın kırpmasında kalmaz.
    Dropdown,
}

/// Sayfaya özgü özel bileşenler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomId {
    /// Erişilebilirlik: büyütecin ya da ekran klavyesinin canlı aç/kapa satırı (`services::assist`).
    Assist(crate::services::assist::Tool),
    Preview,
    MotionRole(MotionRole),
    FrameEdges,
    /// Başka bir ayara götüren satır (hedef ayar kimliği); ayar başka yerde yaşıyorsa.
    Link(&'static str),
    /// Birden çok hedefe götüren tek yönlendirme satırı: `(düğme etiketi, hedef ayar)`.
    /// Bu satırı [`CustomId::Alias`] ile gösteren kimlikler de aramada bu satırı bulur.
    Links(&'static [(&'static str, &'static str)]),
    /// Kendi satırı yok; aramada adı geçen kimliğin satırına gider (ör. `windows.shadow` →
    /// Pencereler'deki tek "Köşe, gölge ve hareket" satırı, §3.5).
    Alias(&'static str),
    /// Kenar panelinin kartında "Bar'dan açma" satırı (bir uç eylemi bu paneli açıyorsa).
    PanelHint(&'static str),
    BarEdge,
    /// Bar sayfasının hero'su: tıklanabilir bar önizlemesi (widget seçimi).
    BarPreview,
    /// "Seçili widget" kartının başlığı (+ şemadaki diğer widget ayarları).
    SelectedWidget,
    BarSections,
    /// Kenar panelinin kart listesi (`panels.<kenar>.widgets`).
    PanelWidgets(&'static str),
    Panels,
    Wallpaper,
    WallpaperOutputs,
    /// Yerleşik Nuxut duvar kağıtlarının kart ızgarası (Duvar kağıdı sayfası ve Hoş geldin).
    WallpaperBuiltins,
    /// Duvar kağıdı › Renkler: renk kaynağının durumu ve Görünüm › Renkler'e giden düğme.
    WallpaperColors,
    /// Görünüm › Renkler: Nuxut renk temasının önizleme kartları (`[colors] theme`).
    ColorTheme,
    Displays,
    Palette,
    Accounts,
    NotificationApps,
    Notice,
    MediaKeys,
    /// Kısayol yakalayıcı satırı (`keybinds.<eylem>`).
    Keybind,
    TypePreview,
    MotionPreset,
    IconSet,
    FontFamily(FontRole),
    /// Koyu / açık kip (`[colors] mode`; duvar kağıdı paletinin kipiyle birlikte).
    DarkMode,
    /// Kurulu ikon temaları (`[desktop] icon_theme`).
    IconTheme,
    /// Kurulu imleç temaları (`[desktop] cursor_theme`).
    CursorTheme,
    /// Shell'in birleşik görünüm durumu (`msg look status`).
    LookStatus,
    /// Hyprland bağlantı satırı kaldırılmışsa not (`msg hypr status`).
    SourceNote,
    /// Klavye / Fare sayfasının üst notu: girdi eşitlemesi kapalı (+ "Aç") ya da kabuk durumu
    /// okunamadı (`msg input status`).
    InputNote,
    /// Klavye düzenleri listesi (`input.keyboard.layouts`): sırala, sil, ara-ekle.
    KeyboardLayouts,
    /// Düzenler arası geçiş seçimi (`input.keyboard.options` içindeki `grp:` öğesi).
    LayoutSwitch,
    /// Tekrar ayarını denemek için yerel metin alanı (hiçbir şey yazmaz).
    KeyTest,
    /// Varsayılan uygulamalar sayfasının üst notu: kabuk çalışmıyor ya da bu sürümü bildirmiyor
    /// (`msg defaults status`).
    DefaultAppsNote,
    /// Rolün varsayılan uygulaması (`msg defaults set`; geri alma snapshot `defaults.<rol>`).
    DefaultApp(crate::services::default_apps::Role),
    /// Gece ışığının konumu ve bugünün güneş olayları (salt okunur; `msg night-light status`).
    NightLightLocation,
    /// Gece ışığının başlangıç/bitiş saati (`night_light.from` / `.to`; TimePicker, dosyada 24 saat).
    NightLightTime,
    /// Kullanıcı hesabı sayfasının üst notu: AccountsService yok ya da okunamadı.
    UserNote,
    /// Profil resmi (büyük daire) + kullanıcı adı + hesap türü + "Değiştir…".
    UserAvatar,
    /// Görünen ad (`SetRealName`).
    UserName,
    About,
    /// Bölge ve dil hero'su: seçili biçimin canlı önizlemesi (karar 9).
    LocalePreview,
    /// `framework.toml [locale]` satırı (dil ya da biçimler; ADR-0010).
    Locale(LocaleField),
    /// Tarih ve saat sayfasının üst notu: timedated yok ya da donanım saati yerel saatte.
    TimedateNote,
    /// Otomatik saat (timedated `SetNTP`).
    AutoTime,
    /// Saat dilimi: aranabilir seçici (timedated `SetTimezone`).
    Timezone,
    /// Elle tarih ve saat (timedated `SetTime`; yalnızca otomatik saat kapalıyken).
    ManualTime,
    /// Bar saati açık kalıp kullanıyor notu + "Temizle" (iki biçim anahtarını siler).
    ClockFormatNote,
    /// Wi-Fi / VPN sayfasının üst notu: NetworkManager yok ya da servis açılamadı.
    NetworkNote,
    /// Wi-Fi durumu ve aç/kapa anahtarı.
    WifiStatus,
    /// Ağ listeleri ve satır eylemleri: bağlı ağ, görünen/gizli ağ, kayıtlı ağlar, kablolu, VPN, içe aktarma.
    NetList(NetList),
    /// Güç sayfasının boş durumu (kartsız): pil, şarj sınırı ve profil hizmeti yoksa maskotlu
    /// boş/hata durumu (`pages/power.rs`).
    PowerStatus,
    /// Pil durumu: yüzde, şarj durumu, kalan süre (UPower; `pages/power.rs`).
    BatteryStatus,
    /// Güç profili: üç seçenekli seçim (power-profiles-daemon; `pages/power.rs`).
    PowerProfile,
    /// Pil şarj sınırı anahtarı ("Pil ömrünü koru"; UPower).
    ChargeLimit,
    /// Bluetooth sayfasının durumu (kartsız): BlueZ/adaptör yoksa maskotlu hata, değilse son
    /// eylem hatasının notu (`pages/bluetooth.rs`).
    BluetoothStatus,
    /// Bluetooth aç/kapa (Powered; rfkill yumuşak/sert ayrı metin).
    BluetoothPower,
    /// Bu bilgisayarın Bluetooth adı (adaptör `Alias`).
    BluetoothName,
    /// Görünürlük (Discoverable, süreli; kalan süre alt metinde).
    BluetoothVisible,
    /// Cihaz listeleri: eşleşmiş (satır içi ayrıntı) ve yakındaki (eşleştir).
    BluetoothDevices(BtList),
    /// Yazıcılar sayfasının durumu (kartsız): CUPS yoksa maskotlu boş durum, değilse son eylem
    /// hatasının notu (`pages/printers.rs`).
    PrintersStatus,
    /// Yazıcı listesi (satır içi ayrıntı: varsayılan yap, deneme sayfası).
    PrinterList,
    /// "Yazıcı ekle…": CUPS web arayüzü bağlantısı (v1; keşif ve ekleme v2).
    PrintersAdmin,
    /// Kullanıcının bitmemiş yazdırma işleri (iptal).
    PrintJobs,
}

/// Bağlantılar sayfalarındaki liste türü (`pages/network.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetList {
    /// Bağlı Wi-Fi ağı (Wi-Fi kartında, durum satırının altında).
    Current,
    /// Görünen ağlar (sinyale göre; bağlı olan hariç).
    Visible,
    /// Kayıtlı Wi-Fi ağları.
    Saved,
    /// VPN ve WireGuard bağlantıları.
    Vpn,
    /// Kablolu aygıtlar (salt okunur: IP, DNS, hız).
    Wired,
    /// "Gizli ağa bağlan…" satırı (Görünen ağlar kartının sonunda).
    Hidden,
    /// "WireGuard içe aktar…" satırı (VPN kartının sonunda).
    Import,
}

/// Bluetooth sayfasındaki cihaz listesi (`pages/bluetooth.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtList {
    /// Eşleşmiş cihazlar (bağlı önce).
    Paired,
    /// Taramada görünen, eşleşmemiş cihazlar (sinyale göre).
    Nearby,
}

/// `[locale]` bölümünün iki ayarı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocaleField {
    /// `locale.language`: arayüz dili (BCP 47).
    Language,
    /// `locale.formats`: tarih/saat ve sayı biçimi (POSIX locale adı).
    Formats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MotionRole {
    Spatial,
    SpatialFast,
    Effects,
}

impl MotionRole {
    pub const ALL: [MotionRole; 3] = [MotionRole::Spatial, MotionRole::SpatialFast, MotionRole::Effects];

    pub fn key(self) -> &'static str {
        match self {
            MotionRole::Spatial => "spatial",
            MotionRole::SpatialFast => "spatial_fast",
            MotionRole::Effects => "effects",
        }
    }

    /// Rol kartının başlığı ve bölüm adı, etkin dilde (`settings-motion-role-<rol>`).
    pub fn title(self) -> &'static str {
        crate::i18n::text_or(&format!("settings-motion-role-{}", self.key().replace('_', "-")), self.source_title())
    }

    /// Kaynak (Türkçe) rol adı; kayıttaki rol bölümlerinin adı.
    pub fn source_title(self) -> &'static str {
        source_text!(match self {
            // Karar 6 (§13): teknik rol adı yerine neyi hareket ettirdiği.
            MotionRole::Spatial => "Paneller ve pencereler",
            MotionRole::SpatialFast => "Menüler ve küçük öğeler",
            MotionRole::Effects => "Solma ve renk",
        })
    }

    /// Rol kartının alt yazısı, etkin dilde (rol satırının `settings-motion-<rol>-description`'ı).
    pub fn subtitle(self) -> &'static str {
        let source = source_text!(match self {
            MotionRole::Spatial => "Açılma, boyut ve konum değişimi",
            MotionRole::SpatialFast => "Menüler, göstergeler, küçük düğmeler",
            MotionRole::Effects => "Saydamlık ve renk geçişleri; taşmaz",
        });
        crate::i18n::text_or(&format!("settings-motion-{}-description", self.key().replace('_', "-")), source)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    // Varsayılanlar burada YOK: tek kaynak config'in sahibidir (bkz. `defaults`).
    // `on`: dize değerli anahtar — açık = bu dize, kapalı = anahtar silinir (sahibin varsayılanı;
    // ör. `[colors] contrast`: "high" / yok = "standard"). `None` = mantıksal değer.
    Toggle { invert: bool, on: Option<&'static str> },
    Slider { min: f64, max: f64, step: f64, unit: &'static str, integer: bool, percent: bool },
    Choice { options: &'static [(&'static str, &'static str)], style: ChoiceStyle },
    Color,
    Text { placeholder: &'static str },
    Custom(CustomId),
}

#[derive(Debug, Clone, Copy)]
pub struct SettingDesc {
    pub id: &'static str,
    pub page: PageId,
    pub section: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub keywords: &'static [&'static str],
    pub file: FileId,
    /// Noktalı anahtar yolu; `Custom` için boş olabilir.
    pub path: &'static str,
    pub control: Control,
    pub visible_if: Option<fn(&Store) -> bool>,
    /// Kademe (§3.1); varsayılan `Basic`.
    pub tier: Tier,
    /// Ayna sayfalar (§3.4): aynı satır bu sayfalarda da çizilir. Tek anahtar, tek geri al
    /// kaydı, tek arama sonucu (birincil `page`). Birincil sayfayı içermez.
    pub mirror_on: &'static [PageId],
    /// Dosyaya yazılmayan sayfa durumuna bağlı görünürlük (ör. seçili widget, §3.3).
    pub ui_if: Option<UiCond>,
}

impl SettingDesc {
    pub fn key(&self) -> KeyPath {
        KeyPath::parse(self.path)
    }

    pub fn is_visible(&self, store: &Store) -> bool {
        self.visible_if.is_none_or(|f| f(store))
    }

    /// Sayfa durumuna göre görünür mü (`ui_if`).
    pub fn ui_visible(&self, ui: &PageUi, store: &Store) -> bool {
        self.ui_if.is_none_or(|c| (c.shown)(ui, store))
    }

    /// Bu sayfada çizilir mi (birincil ya da ayna).
    pub fn shown_on(&self, page: PageId) -> bool {
        self.page == page || self.mirror_on.contains(&page)
    }

    /// Açılır satırın adı (`Tier::Detail`).
    pub fn detail(&self) -> Option<&'static str> {
        match self.tier {
            Tier::Detail(name) => Some(name),
            Tier::Basic => None,
        }
    }

    /// Değer dosyada var ve sahibin varsayılanından farklı mı (açılır satırın nokta rozeti,
    /// "elle ayarlı" sayısı). Etkin değerli ([`Self::is_live`]) satırda yazılmış olması yeter.
    pub fn differs_from_default(&self, store: &Store) -> bool {
        let Some(v) = store.get(self.file, &self.key()) else { return false };
        self.is_live() || !self.default_value(store).is_some_and(|d| self.same_value(v, d))
    }

    /// Değeri çalışan masaüstünden okunan ayar mı (`shell.toml [input]`, sözleşme girdi-v1):
    /// "içe aktar, ezme" — yazılmamış anahtar Hyprland'in (kullanıcının) değerini korur. Böyle
    /// satırda yazılmış her değer (varsayılana eşit olsa da) dosyada kalır; ↺ anahtarı siler.
    pub fn is_live(&self) -> bool {
        self.file == FileId::Shell && self.path.starts_with("input.")
    }

    /// Gösterilen değer: dosyadaki; yoksa (etkin değerli satırda) çalışan masaüstündeki etkin
    /// değer ([`crate::services::input::effective`]); o da yoksa config sahibinin varsayılanı.
    pub fn value<'a>(&self, store: &'a Store) -> Option<std::borrow::Cow<'a, Value>> {
        self.value_with(store, crate::services::input::effective)
    }

    /// [`Self::value`], etkin değer kaynağı verilerek (saf; testler).
    pub fn value_with<'a>(&self, store: &'a Store, effective: impl Fn(&str) -> Option<Value>) -> Option<std::borrow::Cow<'a, Value>> {
        use std::borrow::Cow;
        if let Some(v) = store.get(self.file, &self.key()) {
            return Some(Cow::Borrowed(v));
        }
        if self.is_live()
            && let Some(v) = effective(self.path)
        {
            return Some(Cow::Owned(v));
        }
        self.default_value(store).map(Cow::Borrowed)
    }

    /// Sahibin varsayılanı (hareket anahtarlarında dosyadaki ön ayarınki).
    pub fn default_value(&self, store: &Store) -> Option<&'static Value> {
        crate::defaults::get(store, self.file, &self.key())
    }

    /// Belirli bir ön ayarla varsayılan (testler, ön ayar önizlemesi).
    pub fn default_for(&self, preset: nuxut::theme::MotionPreset) -> Option<&'static Value> {
        crate::defaults::get_for(preset, self.file, &self.key())
    }

    /// Dosyadaki değer, yoksa ya da tipi uymuyorsa varsayılan.
    pub fn bool(&self, store: &Store) -> bool {
        let Control::Toggle { invert, on } = self.control else { return false };
        if self.id == REDUCE_MOTION_ID {
            // Etkin durum: "Kapalı" ön ayarı da azaltılmış harekettir (2026-10-13).
            return reduced_motion(store);
        }
        let v = match on {
            // Dize değerli anahtar: yalnız açık değer "açık"tır (geçersiz/başka değer kapalı görünür).
            Some(on) => self.value(store).is_some_and(|v| v.as_str() == Some(on)),
            None => self.value(store).and_then(|v| v.as_bool()).unwrap_or(false),
        };
        v ^ invert
    }

    pub fn number(&self, store: &Store) -> f64 {
        let Some((min, max)) = self.bounds() else { return 0.0 };
        number(self.value(store).as_deref()).filter(|v| v.is_finite()).unwrap_or(min).clamp(min, max)
    }

    /// Shell şemasındaki tanımı (yalnızca shell.toml satırları; şema yoksa `None`).
    pub fn schema(&self) -> Option<std::sync::Arc<SettingSpec>> {
        (self.file == FileId::Shell && !self.path.is_empty()).then(|| widgets::shell_setting(self.path)).flatten()
    }

    /// Satırın katalog anahtarlarının ortak öneki: `settings-<kimlik>` (`.` ve `_` → `-`). Hareket
    /// rollerinin ortak satırları (`motion.<rol>.<x>`) tek anahtarı paylaşır:
    /// `settings-motion-role-<x>` (üç rolde aynı metin). Bkz. `docs/design/i18n.md` "Kayıt".
    pub fn text_key(&self) -> String {
        text_key(self.id)
    }

    /// Satır başlığı, etkin dilde (`<text_key>-label`); tabloda boşsa boş.
    pub fn label(&self) -> &'static str {
        if self.title.is_empty() { "" } else { crate::i18n::text_or(&format!("{}-label", self.text_key()), self.title) }
    }

    /// Satırın alt yazısı, etkin dilde (`<text_key>-description`); tabloda boşsa boş.
    pub fn description(&self) -> &'static str {
        if self.subtitle.is_empty() { "" } else { crate::i18n::text_or(&format!("{}-description", self.text_key()), self.subtitle) }
    }

    /// Kart (bölüm) adı, etkin dilde ([`section_title`]).
    pub fn section_title(&self) -> &'static str {
        section_title(self.section)
    }

    /// Arama sözcükleri, etkin dilde (`<text_key>-keywords`, virgüllü liste); yoksa boş.
    fn keywords_text(&self) -> &'static str {
        if self.keywords.is_empty() { "" } else { crate::i18n::text_or(&format!("{}-keywords", self.text_key()), "") }
    }

    /// Satırın görünen başlığı. Boş `title` = başlık shell şemasından (dinamik widget satırları,
    /// `bar_preview::dynamic_desc`; etkin dilde, dil değişince şemayla yenilenir), şemada yoksa yol.
    pub fn display_title(&self) -> String {
        if !self.title.is_empty() {
            return self.label().to_owned();
        }
        self.schema().map_or_else(|| self.path.to_owned(), |s| s.title.clone())
    }

    /// Satırın alt metni: shell şeması açıklama veriyorsa o (shell'in sözleşmesi, etkin dilde;
    /// yerel kopya onunla çakışmasın diye), yoksa kayıttaki metin (etkin dilde).
    pub fn display_subtitle(&self) -> String {
        self.schema().map(|s| s.description.clone()).filter(|d| !d.is_empty()).unwrap_or_else(|| self.description().to_owned())
    }

    /// Duruma bağlı alt metin (yoksa [`display_subtitle`](Self::display_subtitle)): kenar
    /// çizgileri yüksek karşıtlıkta "Yok"ken gerçekte ince çizildiğini söyler.
    pub fn subtitle_in(&self, store: &Store) -> String {
        match self.id {
            "shape.outlines" => outlines_subtitle(store).map_or_else(|| self.display_subtitle(), str::to_owned),
            "magnifier.keys" => keys_subtitle(store, &["keybinds.magnifier_in", "keybinds.magnifier_out", "keybinds.magnifier_reset"]),
            "osk.keys" => keys_subtitle(store, &["keybinds.osk_toggle"]),
            _ => self.display_subtitle(),
        }
    }

    /// Duruma bağlı uyarı (satırın altında, uyarı renginde): uykudan önce kilit açık ama boşta
    /// ana anahtarı kapalı — shell `enabled = false` iken onu da yapmaz (shell 7b3c80e).
    pub fn note_in(&self, store: &Store) -> Option<String> {
        match self.id {
            "idle.lock_before_sleep" if idle_known(store) && !idle_on(store) && self.bool(store) => Some(t!("settings-idle-lock-before-sleep-off-note")),
            _ => None,
        }
    }

    /// Anahtar satırının yazımları (tek geri al kaydı). "Hareketi azalt" kapatılırken
    /// `enabled` varsayılana (açık) döner ve "Kapalı" ön ayarı da silinir (varsayılan ön ayar);
    /// aksi hâlde azaltılmış hareket sürerdi. Diğer satırlar: tek yazım, varsayılan silinir.
    pub fn toggle_writes(&self, on: bool, store: &Store) -> Vec<(FileId, KeyPath, Option<Value>)> {
        let mut writes = vec![(self.file, self.key(), self.toggle_value(on).and_then(|v| self.stored(v, store)))];
        if self.id == REDUCE_MOTION_ID && !on {
            let preset = KeyPath::parse("motion.preset");
            if store.get(FileId::Framework, &preset).and_then(Value::as_str).and_then(nuxut::theme::MotionPreset::from_name) == Some(nuxut::theme::MotionPreset::Instant) {
                writes.push((FileId::Framework, preset, None));
            }
        }
        writes
    }

    /// Metin alanının yer tutucusu. Bar saati biçimleri boşken (yazılmamış ya da `""`) saat
    /// `[locale] hour_cycle`'a uyar (shell `docs/design/clock-locale.md`): yer tutucu bunu söyler,
    /// kalıp örneği değil. Shell'in varsayılanı boş değilse (eski shell) o varsayılan gösterilir.
    pub fn placeholder(&self, store: &Store) -> String {
        let Control::Text { placeholder } = self.control else { return String::new() };
        if CLOCK_FORMAT_KEYS.contains(&self.path) {
            return match self.default_value(store).and_then(Value::as_str) {
                Some(d) if !d.trim().is_empty() => d.to_owned(),
                _ => t!("settings-bar-clock-format-auto"),
            };
        }
        placeholder.to_owned()
    }

    /// Seçim satırının seçenekleri (değer, görünen ad): şema yüklüyse shell'in değerleri ve
    /// sırası, yoksa yerel tablo. Bkz. [`resolve_choices`].
    pub fn choices(&self) -> Vec<Choice> {
        let Control::Choice { options, .. } = self.control else { return Vec::new() };
        resolve_choices(options, self.schema().as_deref())
    }

    /// Kaydırıcının aralığı: yerel aralık, şema yüklüyse shell'in doğrulama sınırlarıyla
    /// kesişimi. Bkz. [`resolve_bounds`]. Kaydırıcı değilse `None`.
    pub fn bounds(&self) -> Option<(f64, f64)> {
        let Control::Slider { min, max, .. } = self.control else { return None };
        Some(resolve_bounds((min, max), self.schema().and_then(|s| s.bounds())))
    }

    /// Geldiği `config_version` (şemadaki `since`; yoksa ya da şema yoksa `None` = sürüm 1).
    pub fn since(&self) -> Option<i64> {
        self.schema().and_then(|s| s.since)
    }

    pub fn string(&self, store: &Store) -> String {
        self.value(store).and_then(|v| v.as_str().map(str::to_owned)).unwrap_or_default()
    }

    /// Dosyada açıkça yazılı mı ("varsayılana dön" yalnızca o zaman anlamlı).
    pub fn is_set(&self, store: &Store) -> bool {
        store.get(self.file, &self.key()).is_some()
    }

    /// UI değerini dosyaya yazılacak TOML değerine çevirir (yuvarlama, tamsayı, ters çevirme).
    /// Anahtar satırında `None` = anahtarı sil (dize değerli anahtarın kapalı hâli).
    pub fn toggle_value(&self, on: bool) -> Option<Value> {
        match self.control {
            Control::Toggle { invert, on: Some(value) } => (on ^ invert).then(|| Value::from(value)),
            Control::Toggle { invert, on: None } => Some(Value::from(on ^ invert)),
            _ => Some(Value::from(on)),
        }
    }

    /// Seçim satırının `i`. seçeneği; boş değer ("Genel") anahtarı siler.
    pub fn choice_value(&self, i: usize, store: &Store) -> Option<Value> {
        self.choice_stored(&self.choices().get(i)?.0, store)
    }

    /// Seçim değerinin yazılacak hâli: boş ("Genel") ya da varsayılan → `None` (anahtar silinir).
    pub fn choice_stored(&self, v: &str, store: &Store) -> Option<Value> {
        // Varsayılanı seçmek anahtarı siler: config kısa kalır, sahibin varsayılanı geçerli olur.
        // Etkin değerli satırda seçim hep yazılır (yazılmazsa Hyprland'in değeri kalırdı).
        let is_default = !self.is_live() && self.default_value(store).and_then(Value::as_str) == Some(v);
        (!v.is_empty() && !is_default).then(|| Value::from(v))
    }

    /// Satırın yazacağı değer: varsayılana eşitse `None` — anahtar silinir, config kısa kalır,
    /// sahibin varsayılanı geçerli olur ve sıfırla düğmesi gizlenir ([`choice_value`](Self::choice_value) gibi).
    pub fn stored(&self, value: Value, store: &Store) -> Option<Value> {
        let is_default = !self.is_live() && self.default_value(store).is_some_and(|d| self.same_value(&value, d));
        (!is_default).then_some(value)
    }

    /// İki değer bu ayar için aynı mı (sayılar toleransla, renkler büyük/küçük harf duyarsız).
    pub fn same_value(&self, a: &Value, b: &Value) -> bool {
        match self.control {
            Control::Slider { .. } => matches!((number(Some(a)), number(Some(b))), (Some(x), Some(y)) if (x - y).abs() < 1e-9),
            Control::Color => matches!((a.as_str(), b.as_str()), (Some(x), Some(y)) if x.eq_ignore_ascii_case(y)),
            _ => a.to_string().trim() == b.to_string().trim(),
        }
    }

    pub fn number_value(&self, v: f64) -> Value {
        let (Control::Slider { step, integer, .. }, Some((min, max))) = (self.control, self.bounds()) else { return Value::from(v) };
        let v = quantize(v.clamp(min, max), step);
        if integer { Value::from(v.round() as i64) } else { Value::from(v) }
    }

    /// Arama eşleşmesi (katlanmış sorgu, `fold` ile): etkin dildeki metinler, kaynak Türkçe
    /// metinler (Türkçe bilen kullanıcı ya da çevirisi eksik dil) ve ayar yolu.
    pub fn matches(&self, folded_query: &str) -> bool {
        if folded_query.is_empty() {
            return false;
        }
        let hay = [
            self.label(),
            self.description(),
            self.section_title(),
            self.page.title(),
            self.keywords_text(),
            self.title,
            self.subtitle,
            self.section,
            self.page.source_title(),
            self.path,
        ];
        hay.iter().any(|s| fold(s).contains(folded_query)) || self.keywords.iter().any(|k| fold(k).contains(folded_query))
    }
}

pub fn number(v: Option<&Value>) -> Option<f64> {
    match v? {
        Value::Float(f) => Some(*f.value()),
        Value::Integer(i) => Some(*i.value() as f64),
        _ => None,
    }
}

/// Seçim satırının bir seçeneği: (yazılan değer, görünen ad).
pub type Choice = (String, String);

/// Seçim satırının seçenekleri. Şema yoksa (ya da boşsa) yerel tablo olduğu gibi. Şema varsa
/// değerler ve sıra shell'inkidir (shell'in tanımadığı değer yazılmaz, yeni değer görünür).
/// Görünen ad: shell'in `option_labels`'ı (etkin dilde; Ayarlar'ın `settings-choice-*` adlarıyla
/// aynı), yoksa ya da uzunluk uymadıysa yerel tablodaki ad, o da yoksa ham değer (uyarı
/// [`audit_schema`]'da). Yereldeki boş değer ("Genel" = anahtar yok) başta kalır.
pub fn resolve_choices(local: &[(&'static str, &'static str)], schema: Option<&SettingSpec>) -> Vec<Choice> {
    let local: Vec<(&str, &str)> = local.iter().map(|(v, _)| (*v, choice_label(local, v))).collect();
    let local = local.as_slice();
    let own = |(v, l): &(&str, &str)| ((*v).to_owned(), (*l).to_owned());
    let Some((spec, values)) = schema.and_then(|s| Some((s, s.options()?))).filter(|(_, o)| !o.is_empty()) else {
        return local.iter().map(own).collect();
    };
    let unset = local.iter().filter(|(v, _)| v.is_empty()).map(own);
    let from_schema = values.iter().map(|v| {
        let label = match spec.option_label(v).or_else(|| local.iter().find(|(l, _)| *l == v.as_str()).map(|(_, l)| *l)) {
            Some(l) => l.to_owned(),
            None => catalog_choice_label(v).unwrap_or_else(|| v.clone()),
        };
        (v.clone(), label)
    });
    unset.chain(from_schema).collect()
}

/// Seçim tablolarının katalog önekleri: seçenek adı `settings-choice-<önek>-<değer>` (değerde
/// `:` ve `_` → `-`, boş değer → `default`). Yeni tablo buraya eklenir (kapsam testi).
const CHOICE_TABLES: &[(&[(&str, &str)], &str)] = &[
    (HOUR_CYCLES, "hour-cycle"),
    (COLOR_SOURCES, "color-source"),
    (MOTION_STYLES, "motion-style"),
    (WALLPAPER_FITS, "wallpaper-fit"),
    (WALLPAPER_BACKDROPS, "wallpaper-backdrop"),
    (WALLPAPER_BACKDROP_COLORS, "wallpaper-backdrop-color"),
    (OUTLINES, "outline"),
    (ACCEL_PROFILES, "accel"),
    (SCROLL_METHODS, "scroll-method"),
    (QT_BACKENDS, "qt-backend"),
    (EDGE_CLICK, "edge-click"),
    (WORKSPACE_STYLES, "workspace-style"),
    (WORKSPACE_SCROLLS, "workspace-scroll"),
    (EDGE_SCROLL, "edge-scroll"),
    (ROLE_STYLES, "role-style"),
    (EASINGS, "easing"),
    (NIGHT_SCHEDULES, "night-schedule"),
    (OSK_LAYOUTS, "osk-layout"),
];

/// Seçeneğin katalog anahtarı; tablo [`CHOICE_TABLES`]'ta değilse `None`. Tablo içerikle
/// tanınır (sabit dilimlerin adresi derlemeler arasında tek olmayabilir).
fn choice_key(table: &[(&str, &str)], value: &str) -> Option<String> {
    let (_, prefix) = CHOICE_TABLES.iter().find(|(t, _)| *t == table)?;
    let value = if value.is_empty() { "default".to_owned() } else { value.replace([':', '_'], "-") };
    Some(format!("settings-choice-{prefix}-{value}"))
}

/// Yerel seçim tablosundaki `value`'nun adı, etkin dilde; tabloda yoksa boş.
pub fn choice_label(table: &[(&'static str, &'static str)], value: &str) -> &'static str {
    let Some((_, source)) = table.iter().find(|(v, _)| *v == value) else { return "" };
    match choice_key(table, value) {
        Some(key) => crate::i18n::text_or(&key, source),
        None => source,
    }
}

/// Kayıttaki bölüm (kart) adlarının katalog anahtarları; adı kaynak Türkçe. Hareket rolü
/// bölümleri rol adını kullanır (`settings-motion-role-*`). Yeni bölüm buraya eklenir (testli).
const SECTION_KEYS: &[(&str, &str)] = source_text!(&[
    ("Kablolu", "settings-section-wifi-wired"),
    ("Güç profili", "settings-section-power-profile"),
    ("Kilit", "settings-section-lock"),
    ("Boşta kalınca", "settings-section-idle"),
    ("Pil", "settings-section-power-battery"),
    ("Yazıcılar", "settings-section-printers"),
    ("Yazdırma kuyruğu", "settings-section-printers-jobs"),
    ("Görünen ağlar", "settings-section-wifi-visible"),
    ("Kayıtlı ağlar", "settings-section-wifi-saved"),
    ("VPN bağlantıları", "settings-section-vpn-connections"),
    ("Bluetooth", "settings-section-bluetooth"),
    ("Eşleşmiş cihazlar", "settings-section-bluetooth-paired"),
    ("Yakındaki cihazlar", "settings-section-bluetooth-nearby"),
    ("Alt panel", "settings-section-panel-bottom"),
    ("Bar ve paneller", "settings-section-bar-panels"),
    ("Boyutlandırma", "settings-section-resize"),
    ("Çalışma alanları", "settings-section-workspaces"),
    ("Biçim", "settings-section-format"),
    ("Büyüteç", "settings-section-magnifier"),
    ("Çerçeve", "settings-section-frame"),
    ("Davranış", "settings-section-behavior"),
    ("Diğer", "settings-section-other"),
    ("Dil ve biçimler", "settings-section-language-formats"),
    ("Dokunmatik yüzey", "settings-section-touchpad"),
    ("Durum", "settings-section-status"),
    ("Düzenler", "settings-section-layouts"),
    ("Emoji", "settings-section-emoji"),
    ("Ekran görüntüsü", "settings-section-screenshot"),
    ("Ekran klavyesi", "settings-section-osk"),
    ("Fare", "settings-section-mouse"),
    ("Gelişmiş", "settings-section-advanced"),
    ("Gece ışığı", "settings-section-night-light"),
    ("Genel", "settings-section-general"),
    ("Gölge", "settings-section-shadow"),
    ("Görme", "settings-section-seeing"),
    ("Görünüm", "settings-section-appearance"),
    ("İkonlar", "settings-section-icons"),
    ("İnternet", "settings-section-internet"),
    ("Konum ve boyut", "settings-section-position-size"),
    ("Köşeler ve şekil", "settings-section-corners-shape"),
    ("Maskot", "settings-section-mascot"),
    ("Medya", "settings-section-media"),
    ("Medya tuşları", "settings-section-media-keys"),
    ("Medya ve belgeler", "settings-section-media-documents"),
    ("Menüler ve küçük öğeler", "settings-motion-role-spatial-fast"),
    ("Monitör başına", "settings-section-per-monitor"),
    ("Monitörler", "settings-section-displays"),
    ("Nuxut duvar kağıtları", "settings-section-builtin-wallpapers"),
    ("Odak ve taşıma", "settings-section-focus-move"),
    ("Oturum", "settings-section-session"),
    ("Pano", "settings-section-clipboard"),
    ("Paneller ve pencereler", "settings-motion-role-spatial"),
    ("Parlaklık", "settings-section-brightness"),
    ("Pencereler", "settings-section-windows"),
    ("Profil", "settings-section-profile"),
    ("Renkler", "settings-section-colors"),
    ("Saat", "settings-section-clock"),
    ("Sağ panel", "settings-section-panel-right"),
    ("Seçili widget", "settings-section-selected-widget"),
    ("Ses", "settings-section-sound"),
    ("Ses düzeyi", "settings-section-sound-level"),
    ("Sistem", "settings-section-system"),
    ("Solma ve renk", "settings-motion-role-effects"),
    ("Sol panel", "settings-section-panel-left"),
    ("Takvim hesapları", "settings-section-accounts"),
    ("Tuş tekrarı", "settings-section-key-repeat"),
    ("Uygulamalar", "settings-section-apps"),
    ("Üst panel", "settings-section-panel-top"),
    ("Widget'lar", "settings-section-widgets"),
    ("Yazı tipleri", "settings-section-font-families"),
    ("Yerleşim", "settings-section-layout"),
]);

/// Bölüm adı (kaynak Türkçe, kayıttaki `section`; kartları gruplayan kimlik de odur), etkin
/// dilde; tabloda yoksa olduğu gibi.
pub fn section_title(section: &str) -> &str {
    match SECTION_KEYS.iter().find(|(s, _)| *s == section) {
        Some((source, key)) => crate::i18n::text_or(key, source),
        None => section,
    }
}

/// Kimlikten katalog öneki ([`SettingDesc::text_key`]).
pub fn text_key(id: &str) -> String {
    let id = ["spatial_fast", "spatial", "effects"]
        .iter()
        .find_map(|role| id.strip_prefix("motion.")?.strip_prefix(role)?.strip_prefix('.'))
        .map_or_else(|| id.to_owned(), |rest| format!("motion.role.{rest}"));
    format!("settings-{}", id.replace(['.', '_'], "-"))
}

/// Shell adı vermediğinde ve yerel tabloda olmadığında Ayarlar kataloğundaki genel ad (bugün
/// kenar değerleri: dinamik widget satırlarındaki panel kenarı seçimi). Bilinmeyen değer `None`.
fn catalog_choice_label(value: &str) -> Option<String> {
    Some(match value {
        "left" => t!("settings-edge-left"),
        "right" => t!("settings-edge-right"),
        "top" => t!("settings-edge-top"),
        "bottom" => t!("settings-edge-bottom"),
        _ => return None,
    })
}

/// Kaydırıcı aralığı: yerel aralık (ürün kararı, doğrulama sınırının içinde) ile şemanın
/// doğrulama sınırının kesişimi — shell sınırı daraltırsa Ayarlar reddedilecek değer yazmaz,
/// genişletirse yerel aralık kalır. Kesişim boşsa ya da şema yoksa yerel aralık.
pub fn resolve_bounds(local: (f64, f64), schema: Option<(f64, f64)>) -> (f64, f64) {
    match schema {
        Some((lo, hi)) if local.0.max(lo) < local.1.min(hi) => (local.0.max(lo), local.1.min(hi)),
        _ => local,
    }
}

/// Yerel yedek ile şema arasındaki farklar (Türkçe açıklama): seçim satırında değer listesi ya
/// da sırası farklı, etiketi olmayan şema değeri; kaydırıcı aralığı doğrulama sınırı dışında ya da
/// tür (tam sayı) farklı; şemadaki kısayol eyleminin satırı yok ya da satırın eylemi şemada yok.
/// Şemada olmayan anahtar fark sayılmaz (eski shell). `lookup` şemadaki tanımı verir.
pub fn schema_mismatches<S: std::ops::Deref<Target = SettingSpec>>(lookup: impl Fn(&str) -> Option<S>, schema_keys: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for d in SETTINGS.iter().filter(|d| d.file == FileId::Shell && !d.path.is_empty()) {
        let Some(spec) = lookup(d.path) else { continue };
        match d.control {
            Control::Choice { options, .. } => {
                let Some(schema) = spec.options() else {
                    out.push(format!("{}: şemada seçim değil", d.path));
                    continue;
                };
                let local: Vec<&str> = options.iter().map(|(v, _)| *v).filter(|v| !v.is_empty()).collect();
                if local != schema.iter().map(String::as_str).collect::<Vec<_>>() {
                    out.push(format!("{}: yerel seçenekler {local:?}, şema {schema:?}", d.path));
                }
                for v in schema.iter().filter(|v| !local.contains(&v.as_str()) && spec.option_label(v).is_none()) {
                    out.push(format!("{}: \"{v}\" için etiket yok (ham adıyla gösterilir)", d.path));
                }
            }
            Control::Slider { min, max, integer, .. } => {
                match spec.bounds() {
                    Some((lo, hi)) if min >= lo && max <= hi => {}
                    Some((lo, hi)) => out.push(format!("{}: yerel aralık [{min}, {max}] şema sınırı [{lo}, {hi}] dışında", d.path)),
                    None => {}
                }
                if integer != matches!(spec.kind, SettingKind::Int { .. }) {
                    out.push(format!("{}: tam sayı türü şemayla uyuşmuyor", d.path));
                }
            }
            _ => {}
        }
    }
    let rows: Vec<&str> = keybind_rows().map(|d| d.path).collect();
    let schema_binds: Vec<&str> = schema_keys.iter().copied().filter(|k| k.starts_with("keybinds.")).collect();
    for k in schema_binds.iter().filter(|k| !rows.contains(k)) {
        out.push(format!("{k}: şemadaki kısayol eyleminin Ayarlar'da satırı yok"));
    }
    if !schema_binds.is_empty() {
        for k in rows.iter().filter(|k| !schema_binds.contains(k)) {
            out.push(format!("{k}: kısayol satırının eylemi şemada yok"));
        }
    }
    out
}

/// Şema yüklenince bir kez: yerel yedekle farkları uyarı olarak loglar (Ayarlar yine çalışır —
/// seçenekler şemadan, eksik etiket ham adla).
pub fn audit_schema() {
    static DONE: std::sync::Once = std::sync::Once::new();
    let settings = widgets::shell_settings();
    if settings.is_empty() {
        return;
    }
    DONE.call_once(|| {
        let keys: Vec<&str> = settings.iter().map(|(k, _)| k.as_str()).collect();
        for m in schema_mismatches(widgets::shell_setting, &keys) {
            tracing::warn!("shell şeması: {m}");
        }
    });
}

/// Adımın hassasiyetine yuvarlar; `0.30000000000000004` yerine `0.3` yazılır.
pub fn quantize(v: f64, step: f64) -> f64 {
    if step <= 0.0 {
        return v;
    }
    let snapped = (v / step).round() * step;
    let decimals = (-step.log10()).ceil().max(0.0) as usize;
    format!("{snapped:.decimals$}").parse().unwrap_or(snapped)
}

/// Türkçe duyarlı katlama: küçük harf, aksansız (ı→i, ş→s, ğ→g, ü→u, ö→o, ç→c, â→a…).
pub fn fold(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            'I' | 'İ' | 'ı' | 'î' | 'Î' => Some('i'),
            'Ş' | 'ş' => Some('s'),
            'Ğ' | 'ğ' => Some('g'),
            'Ü' | 'ü' | 'û' | 'Û' => Some('u'),
            'Ö' | 'ö' => Some('o'),
            'Ç' | 'ç' => Some('c'),
            'Â' | 'â' => Some('a'),
            c => c.to_lowercase().next(),
        })
        .collect()
}

/// Sorguya uyan görünür ayarlar, sayfa sırasıyla. Birleşik görünüm kapalıyken gizlenen bir
/// satır ([`UNIFIED_GATED`]) aranırsa sonuç onu açan anahtara (`desktop.unified`) gider —
/// kullanıcı ayarın neden görünmediğini ve nasıl açılacağını orada görür (tek sonuç).
pub fn search<'a>(store: &Store, query: &str) -> Vec<&'a SettingDesc>
where
    'static: 'a,
{
    let q = fold(query.trim());
    let mut out: Vec<&SettingDesc> = Vec::new();
    for s in SETTINGS.iter().filter(|s| s.matches(&q)) {
        let hit = if s.is_visible(store) {
            Some(s)
        } else if UNIFIED_GATED.contains(&s.id) {
            by_id("desktop.unified").filter(|u| u.is_visible(store))
        } else if s.is_live() && !input_synced(store) {
            // Girdi ayarları masaüstüne uygulanmıyor: sonuç sayfanın notuna (neden + "Aç").
            input_note_for(s.page)
        } else if s.page == PageId::DefaultApps {
            // Kabuk durumu okunamadı (satırlar gizli): sonuç sayfanın notuna (neden).
            by_id("defaults.note")
        } else {
            None
        };
        if let Some(hit) = hit
            && !out.iter().any(|o| o.id == hit.id)
        {
            out.push(hit);
        }
    }
    out.sort_by_key(|s| s.page);
    out
}

/// Yalnızca birleşik görünüm (`[desktop] unified`) açıkken görünen satırlar: kapalıyken shell
/// bunları uygulamaz (shell `look::plan::build` boş plan). Hepsinin `visible_if`'i
/// [`unified_on`]; Görünüm ve Hoş geldin aynı koşulu kullanır, aramada anahtara yönlenir.
pub const UNIFIED_GATED: [&str; 5] = ["desktop.status", "desktop.icon_theme", "desktop.cursor_theme", "desktop.cursor_size", "desktop.qt_backend"];

pub fn by_id(id: &str) -> Option<&'static SettingDesc> {
    SETTINGS.iter().find(|s| s.id == id)
}

/// Birincil sayfası `page` olan ayarlar.
pub fn for_page(page: PageId) -> impl Iterator<Item = &'static SettingDesc> {
    SETTINGS.iter().filter(move |s| s.page == page)
}

/// Sayfada çizilecek ayarlar (birincil + ayna), kayıt sırasıyla.
pub fn shown_on(page: PageId) -> impl Iterator<Item = &'static SettingDesc> {
    SETTINGS.iter().filter(move |s| s.shown_on(page))
}

/// Kısayol satırları, shell'in eylem sırasıyla (çakışmada sıradaki ilk eylem kazanır).
pub fn keybind_rows() -> impl Iterator<Item = &'static SettingDesc> {
    SETTINGS.iter().filter(|s| s.control == Control::Custom(CustomId::Keybind))
}

// ---- Kayıt ----------------------------------------------------------------------------------

const fn toggle() -> Control {
    Control::Toggle { invert: false, on: None }
}

const fn slider(min: f64, max: f64, step: f64, unit: &'static str) -> Control {
    Control::Slider { min, max, step, unit, integer: false, percent: false }
}

const fn int_slider(min: f64, max: f64, unit: &'static str) -> Control {
    Control::Slider { min, max, step: 1.0, unit, integer: true, percent: false }
}

const fn percent() -> Control {
    Control::Slider { min: 0.0, max: 1.0, step: 0.01, unit: "%", integer: false, percent: true }
}

const fn color() -> Control {
    Control::Color
}

/// Yazı kalınlığı (framework sınırı 100..=1000).
/// Rol boyut ölçeği (framework sınırı 0.5..=4).
const SCALE: Control = slider(0.5, 2.0, 0.05, "×");

const fn custom(id: CustomId) -> Control {
    Control::Custom(id)
}

/// Kayıt satırı. İsteğe bağlı sondakiler (bu sırayla): `if <fn(&Store)>` (dosyaya bağlı
/// görünürlük), `detail "<açılır satır>"` (kademe), `mirror [Sayfa, …]` (ayna sayfalar),
/// `ui_if <UiCond>` (sayfa durumuna bağlı görünürlük ve aramanın onu kuran adımı).
macro_rules! s {
    ($id:literal, $page:ident, $section:literal, $title:literal, $subtitle:literal, [$($kw:literal),*], $file:ident, $path:literal, $control:expr
        $(, if $vis:expr)? $(, detail $detail:literal)? $(, mirror [$($mirror:ident),*])? $(, ui_if $uif:expr)?) => {
        SettingDesc {
            id: $id,
            page: PageId::$page,
            section: $section,
            title: $title,
            subtitle: $subtitle,
            keywords: &[$($kw),*],
            file: FileId::$file,
            path: $path,
            control: $control,
            visible_if: s!(@opt $($vis)?),
            tier: s!(@tier $($detail)?),
            mirror_on: &[$($(PageId::$mirror),*)?],
            ui_if: s!(@opt $($uif)?),
        }
    };
    (@opt) => { None };
    (@opt $f:expr) => { Some($f) };
    (@tier) => { Tier::Basic };
    (@tier $name:literal) => { Tier::Detail($name) };
}

fn shadow_on(store: &Store) -> bool {
    by_id("shadow.enabled").is_some_and(|d| d.bool(store))
}

/// `framework.toml [locale] hour_cycle` seçenekleri (framework; değer ve varsayılan
/// framework şemasından, burada yalnızca etiket).
pub const HOUR_CYCLES: &[(&str, &str)] = &[("auto", "Otomatik"), ("h23", "24 saat"), ("h12", "12 saat")];

/// Framework şeması `[locale] hour_cycle`'ı tanıyor mu (varsayılanı var mı). Tanımayan
/// framework'te satır gizli; framework sürümü gelince kendiliğinden görünür (t119).
fn hour_cycle_supported(store: &Store) -> bool {
    crate::defaults::get(store, FileId::Framework, &KeyPath::parse("locale.hour_cycle")).is_some()
}

/// Bar saatinin iki biçim anahtarı (yatay, dikey). Shell sözleşmesi (`docs/design/clock-locale.md`):
/// yazılmamış ya da `""` (yalnız boşluk dahil) = otomatik (`[locale] hour_cycle`); dolu kalıp kazanır.
pub const CLOCK_FORMAT_KEYS: [&str; 2] = ["bar.widget.clock.format", "bar.widget.clock.vertical_format"];

/// shell.toml'da açıkça yazılmış (boş olmayan) saat kalıbı mı? Dize olmayan değer de
/// "yazılmış" sayılır: shell onu uyarıyla reddeder, ama kullanıcının elle yazdığı bir şey var ve
/// "Temizle" onu da kaldırır.
pub fn clock_format_explicit(store: &Store, path: &str) -> bool {
    store.get(FileId::Shell, &KeyPath::parse(path)).is_some_and(|v| v.as_str().is_none_or(|s| !s.trim().is_empty()))
}

/// Saat biçimi satırının altındaki not: bar saati 12/24 tercihine uymuyor, çünkü *etkili* kalıp
/// açıkça yazılmış. Etkili kalıp bar'ın yönüne bağlı: shell yatay bar'da yalnız `format`'ı,
/// dikey bar'da (sol/sağ kenar) yalnız `vertical_format`'ı okur (shell
/// `ClockSettings::pattern(vertical)`; diğeri o yönde hiç kullanılmaz). Dolayısıyla yatay bar'da
/// yalnız `vertical_format` yazılıysa saat yine tercihe uyar ve not çıkmaz (ve tersi).
fn clock_format_overrides(store: &Store) -> bool {
    // `bar.edge` yoksa üst (yatay), bkz. `pages::edges::bar_edge`.
    let vertical = matches!(store.get(FileId::Shell, &KeyPath::parse("bar.edge")).and_then(Value::as_str), Some("left" | "right"));
    hour_cycle_supported(store) && clock_format_explicit(store, CLOCK_FORMAT_KEYS[usize::from(vertical)])
}

/// `framework.toml [colors] contrast`'ın açık değeri (framework `accessibility.md` §1).
pub const CONTRAST_HIGH: &str = "high";

/// Framework şeması `[colors] contrast`'ı tanıyor mu (varsayılanı var mı). Tanımayan (eski)
/// framework'te "Yüksek karşıtlık" satırı gizli; `hour_cycle_supported` kalıbı.
fn contrast_supported(store: &Store) -> bool {
    contrast_supported_by(crate::defaults::get(store, FileId::Framework, &KeyPath::parse("colors.contrast")))
}

/// Framework şeması `[appearance] mascot`'u tanıyor mu (mantıksal varsayılanı var mı). Tanımayan
/// framework'te "Fiyu'yu gizle" satırı gizli; `hour_cycle_supported` kalıbı.
fn mascot_supported(store: &Store) -> bool {
    mascot_supported_by(crate::defaults::get(store, FileId::Framework, &KeyPath::parse("appearance.mascot")))
}

/// [`mascot_supported`], şema varsayılanı verilerek (saf; testli).
fn mascot_supported_by(default: Option<&Value>) -> bool {
    default.is_some_and(|v| v.as_bool().is_some())
}

/// [`contrast_supported`], şema varsayılanı verilerek (saf; testli).
fn contrast_supported_by(default: Option<&Value>) -> bool {
    default.is_some_and(|v| v.as_str().is_some())
}

/// Hareket "Kapalı" mı — framework `Theme::reduced_motion()` ile aynı kural, store'dan (kaydedilmemiş
/// seçim de yansır): `[motion] preset = "instant"` ya da `[motion] enabled = false`
/// (framework: `enabled = cfg.motion.enabled && preset.enabled`).
pub fn reduced_motion(store: &Store) -> bool {
    // Ham değer (satırın `bool`'u bu işlevi çağırır; özyineleme olmasın).
    let key = KeyPath::parse("motion.enabled");
    let enabled = store
        .get(FileId::Framework, &key)
        .and_then(Value::as_bool)
        .or_else(|| crate::defaults::get(store, FileId::Framework, &key).and_then(Value::as_bool))
        .unwrap_or(true);
    !(enabled && crate::defaults::preset(store).values().enabled)
}

/// "Hareketi azalt" satırı (`motion.enabled`, ters anahtar): `reduced_motion`'ı gösterir.
pub const REDUCE_MOTION_ID: &str = "motion.enabled";

/// "Kenar çizgileri" yüksek karşıtlıkta "Yok"ken (framework: `none` → `subtle`) satırın alt metni.
fn outlines_subtitle(store: &Store) -> Option<&'static str> {
    let high = by_id("colors.contrast").is_some_and(|d| d.is_visible(store) && d.bool(store));
    let none = by_id("shape.outlines").is_some_and(|d| d.string(store) == "none");
    (high && none).then(|| crate::i18n::text_or("settings-shape-outlines-high-contrast-note", "Yüksek karşıtlık açık: ince çiziliyor"))
}

/// Erişilebilirlik'teki kısayol yönlendirmesinin alt metni: eylem adı ve tuşları (Kısayollar'daki
/// satırın değeri; atanmamışsa "Atanmamış"). Kısayollar tekrarlanmaz, yalnızca gösterilir.
fn keys_subtitle(store: &Store, ids: &[&str]) -> String {
    ids.iter()
        .filter_map(|id| by_id(id))
        .map(|d| {
            let keys = match crate::keybind::parse(&d.string(store)) {
                Ok(combo) => combo.caps().join("+"),
                Err(_) => nuxut::i18n::t!("settings-keybinds-unassigned"),
            };
            format!("{} {keys}", d.label())
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// Ekran klavyesi kısayolunun Kısayollar'da satırı var mı (yönlendirmenin hedefi).
fn osk_keybind_row(_: &Store) -> bool {
    by_id("keybinds.osk_toggle").is_some()
}

fn volume_boosted(store: &Store) -> bool {
    by_id("audio.max_volume").is_some_and(|d| d.number(store) > 100.0)
}

fn frame_on(store: &Store) -> bool {
    by_id("frame.enabled").is_some_and(|d| d.bool(store))
}

fn panel_on(store: &Store, edge: &str) -> bool {
    by_id(&format!("panels.{edge}.enabled")).is_some_and(|d| d.bool(store))
}

fn panel_right(store: &Store) -> bool {
    panel_on(store, "right")
}

fn panel_left(store: &Store) -> bool {
    panel_on(store, "left")
}

fn panel_top(store: &Store) -> bool {
    panel_on(store, "top")
}

fn panel_bottom(store: &Store) -> bool {
    panel_on(store, "bottom")
}

/// Panel kısayolu (Kısayollar'da birincil, panel kartında ayna): Nuxut kısayolları açık ve
/// panel kullanılıyorsa. Kapalı panelin kısayolu anlamsız; kapalı panel kartı tek satıra iner.
fn panel_right_bind(store: &Store) -> bool {
    keybinds_synced(store) && panel_right(store)
}

fn panel_left_bind(store: &Store) -> bool {
    keybinds_synced(store) && panel_left(store)
}

/// Bar'ın bir uç eylemi `panel:<kenar>` açıyor mu: `(uç, tık türü)` listesi, kayıt sırasıyla
/// (`start`/`end`, `left_click`/`right_click`). Bkz. `pages::panel_hint`.
pub fn bar_actions_opening(store: &Store, edge: &str) -> Vec<(&'static str, &'static str)> {
    let want = format!("panel:{edge}");
    let mut out = Vec::new();
    for (end, which) in [("start", BarEnd::Start), ("end", BarEnd::End)] {
        // Uçta widget yoksa eylem çalışmaz (`end_widget`: bölüm bazında uç).
        if end_widget(store, which).is_none() {
            continue;
        }
        for click in ["left_click", "right_click"] {
            if by_id(&format!("bar.edge_actions.{end}.{click}")).is_some_and(|d| d.string(store) == want) {
                out.push((end, click));
            }
        }
    }
    out
}

fn panel_hint_on(store: &Store, edge: &str) -> bool {
    panel_on(store, edge) && !bar_actions_opening(store, edge).is_empty()
}

fn panel_hint_right(store: &Store) -> bool {
    panel_hint_on(store, "right")
}

fn panel_hint_left(store: &Store) -> bool {
    panel_hint_on(store, "left")
}

fn panel_hint_top(store: &Store) -> bool {
    panel_hint_on(store, "top")
}

fn panel_hint_bottom(store: &Store) -> bool {
    panel_hint_on(store, "bottom")
}

// ---- Bar: seçili widget'a bağlı görünürlük (§6.1; sayfa durumu `PageUi::selected_widget`) ----

/// Bu uçta widget var ve tekerleği ses/mikrofon/parlaklık değiştiriyor: adım ayarı anlamlı.
fn end_scroll_level(store: &Store, end: BarEnd) -> bool {
    end_widget(store, end).is_some()
        && by_id(&format!("bar.edge_actions.{}.scroll", end.key())).is_some_and(|d| matches!(d.string(store).as_str(), "volume" | "mic" | "brightness"))
}

fn edge_scroll_level(store: &Store) -> bool {
    BarEnd::ALL.into_iter().any(|e| end_scroll_level(store, e))
}

fn has_start(store: &Store) -> bool {
    end_widget(store, BarEnd::Start).is_some()
}

fn has_end(store: &Store) -> bool {
    end_widget(store, BarEnd::End).is_some()
}

/// Uç satırları: seçili widget o uçta. Arama o uçtaki widget'ı seçer.
const SEL_START: UiCond = UiCond { shown: |ui, s| selected_end(ui, s) == Some(BarEnd::Start), reveal: |ui, s| ui.selected_widget = end_widget(s, BarEnd::Start) };
const SEL_END: UiCond = UiCond { shown: |ui, s| selected_end(ui, s) == Some(BarEnd::End), reveal: |ui, s| ui.selected_widget = end_widget(s, BarEnd::End) };
/// Tekerlek adımı: seçili widget, tekerleği seviye değiştiren uçta. Arama önce sağ ucu dener.
const SEL_LEVEL: UiCond = UiCond {
    shown: |ui, s| selected_end(ui, s).is_some_and(|e| end_scroll_level(s, e)),
    reveal: |ui, s| {
        let end = [BarEnd::End, BarEnd::Start].into_iter().find(|e| end_scroll_level(s, *e)).unwrap_or(BarEnd::End);
        ui.selected_widget = end_widget(s, end);
    },
};
/// Widget'ın kendi ayarları: o widget seçili (bar'da olmasa da — arama seçer, kart "Bar'da değil" der).
const SEL_WORKSPACES: UiCond = UiCond { shown: |ui, s| selected(ui, s).as_deref() == Some("workspaces"), reveal: |ui, _| ui.selected_widget = Some("workspaces".into()) };
const SEL_CLOCK: UiCond = UiCond { shown: |ui, s| selected(ui, s).as_deref() == Some("clock"), reveal: |ui, _| ui.selected_widget = Some("clock".into()) };
const SEL_TRAY: UiCond = UiCond { shown: |ui, s| selected(ui, s).as_deref() == Some("tray"), reveal: |ui, _| ui.selected_widget = Some("tray".into()) };

/// Birleşik görünüm açık mı (`[desktop] unified`; yoksa framework varsayılanı).
pub fn unified_on(store: &Store) -> bool {
    by_id("desktop.unified").is_some_and(|d| d.bool(store))
}

/// Kenar boşluğu var mı: genel ya da bir monitörün yerleşimi "Tamamı" (`contain`; shell zemini
/// yalnızca orada çizer). Zemin satırları yalnızca o zaman görünür (§3.1 koşullu kademe).
pub fn wallpaper_letterbox(store: &Store) -> bool {
    use crate::services::wallpaper_source::{self, WallSource};
    // Yerleşik (`nuxut:`) her zaman cover'dır (sözleşme §3): yalnızca dosya gösteren yüzeyler sayılır.
    let global = wallpaper_source::current(store);
    let general = by_id("wallpaper.fit").is_some_and(|d| d.string(store) == "contain");
    let per_output = store.file(FileId::Shell).get(&KeyPath::parse("wallpaper.outputs")).and_then(|i| i.as_table_like()).is_some_and(|t| {
        t.iter().any(|(_, o)| {
            let own = o.get("path").and_then(|p| p.as_str()).filter(|p| !p.is_empty()).map(WallSource::parse);
            let file = own.as_ref().unwrap_or(&global).file().is_some();
            let contain = o.get("fit").and_then(|f| f.as_str()).map_or(general, |f| f == "contain");
            file && contain
        })
    });
    (general && global.file().is_some()) || per_output
}

/// Yerleşim satırı anlamlı mı: genel duvar kağıdı ya da bir monitörün kendi görseli dosya.
/// Yerleşik (`nuxut:`) her zaman ekranı doldurur; yalnızca yerleşik varken satır gizli
/// (2026-10-13).
pub fn wallpaper_fit_matters(store: &Store) -> bool {
    use crate::services::wallpaper_source::{self, WallSource};
    wallpaper_source::current(store).file().is_some()
        || store.file(FileId::Shell).get(&KeyPath::parse("wallpaper.outputs")).and_then(|i| i.as_table_like()).is_some_and(|t| {
            t.iter().any(|(_, o)| o.get("path").and_then(|p| p.as_str()).filter(|p| !p.is_empty()).is_some_and(|p| WallSource::parse(p).file().is_some()))
        })
}

/// Kenar boşluğu zemini bulanık kopya mı (yarıçap yalnızca o zaman anlamlı). Zemin rengi her
/// zeminde görünür: düz zeminin rengi, bulanık kopya hazır olana dek de yedek.
fn wallpaper_backdrop_blur(store: &Store) -> bool {
    wallpaper_letterbox(store) && by_id("wallpaper.backdrop").is_some_and(|d| d.string(store) == "blur")
}

/// Renkler Nuxut temasından mı (duvar kağıdından değil).
fn static_colors(store: &Store) -> bool {
    !wallpaper_colors(store)
}

fn wallpaper_colors(store: &Store) -> bool {
    by_id("colors.source").is_some_and(|d| d.string(store) == "wallpaper")
}

fn ensure_source_on(store: &Store) -> bool {
    toggle_on(store, "compositor.ensure_source")
}

fn toggle_on(store: &Store, id: &str) -> bool {
    by_id(id).is_some_and(|d| d.bool(store))
}

/// Pencereler: köşe/gölge/bulanıklık masaüstüne uygulanıyor mu (`compositor.sync_look`).
fn look_synced(store: &Store) -> bool {
    toggle_on(store, "compositor.sync_look")
}

fn windows_synced(store: &Store) -> bool {
    toggle_on(store, "compositor.sync_windows")
}

fn motion_synced(store: &Store) -> bool {
    toggle_on(store, "compositor.sync_motion")
}

/// Pencereler › Görünüm "Köşe, gölge ve hareket" satırı: köşe/gölge ya da hareket temadan.
fn look_or_motion_synced(store: &Store) -> bool {
    look_synced(store) || motion_synced(store)
}

fn window_blur_on(store: &Store) -> bool {
    look_synced(store) && toggle_on(store, "windows.blur")
}

fn keybinds_synced(store: &Store) -> bool {
    toggle_on(store, "compositor.sync_keybinds")
}

fn keybinds_unsynced(store: &Store) -> bool {
    !keybinds_synced(store)
}

// ---- Kilit ve boşta (Güç) ----

/// Yüklü shell şeması `[idle]`'ı tanıyor mu (eski shell ya da şema yoksa kartlar gizli).
fn idle_known(_store: &Store) -> bool {
    widgets::shell_setting("idle.enabled").is_some()
}

/// Boşta ana anahtarı açık: kapalıyken shell boşta eylemlerini ve uykudan önce kilidi yapmaz.
fn idle_on(store: &Store) -> bool {
    idle_known(store) && toggle_on(store, "idle.enabled")
}

/// Uykudan önce kilit satırı: anahtar shell 7b3c80e ile geldi (daha eski shell'de yok). Ana anahtar
/// kapalıyken de görünür; çalışmadığını satırın uyarısı söyler ([`SettingDesc::note_in`]).
fn idle_lock_before_sleep_known(store: &Store) -> bool {
    idle_known(store) && widgets::shell_setting("idle.lock_before_sleep").is_some()
}

/// `[lock] listen_logind` satırı: anahtarı tanıyan shell'de (eski shell'de yok).
fn lock_listen_known(_store: &Store) -> bool {
    widgets::shell_setting("lock.listen_logind").is_some()
}

/// Bir aşamanın süre satırı yalnızca aşama açıkken.
fn idle_stage_on(store: &Store, id: &str) -> bool {
    idle_on(store) && toggle_on(store, id)
}

/// Kilit ya da boşta kartı sayfada mı (Güç sayfası kenar çubuğunda kalsın; boş durum maskotu
/// kartlar varken gösterilmez).
pub fn idle_shown(_store: &Store) -> bool {
    lock_cards_known()
}

/// Yüklü shell şeması `[idle]` ya da `[lock] listen_logind`'i tanıyor mu.
fn lock_cards_known() -> bool {
    widgets::shell_setting("idle.enabled").is_some() || widgets::shell_setting("lock.listen_logind").is_some()
}

/// Temel katman kısayolları (shell ADR-0006): kısayollar eşitleniyor ve temel katman açık.
fn base_binds(store: &Store) -> bool {
    keybinds_synced(store) && toggle_on(store, "compositor.base_layer")
}

/// Kabuk kısayollarının devralınması (shell ADR-0006 eki, 2026-10-16): kısayollar eşitleniyor ve
/// yüklü shell anahtarı tanıyor (eski shell'de satır yok).
fn take_over_known(store: &Store) -> bool {
    keybinds_synced(store) && widgets::shell_setting("compositor.take_over_keys").is_some()
}

// ---- Gece ışığı (Ekranlar) ----

/// Yüklü shell şeması `[night_light]`'ı tanıyor mu (eski shell ya da şema yoksa kart gizli:
/// yazılan anahtar etkisiz kalırdı).
fn night_light_known(_store: &Store) -> bool {
    widgets::shell_setting("night_light.enabled").is_some()
}

fn night_light_on(store: &Store) -> bool {
    night_light_known(store) && toggle_on(store, "night_light.enabled")
}

fn night_schedule(store: &Store) -> String {
    by_id("night_light.schedule").map(|d| d.string(store)).unwrap_or_default()
}

/// Saat aralığı satırları: yalnızca `schedule = "manual"`.
fn night_light_manual(store: &Store) -> bool {
    night_light_on(store) && night_schedule(store) == "manual"
}

/// Konum satırı: yalnızca `schedule = "sun"` (satırın kendisi shell durumu yoksa da gizlenir).
fn night_light_sun(store: &Store) -> bool {
    night_light_on(store) && night_schedule(store) == "sun"
}

/// Geçiş süresi: zamanlama bir sınır içeriyorsa (`always`'te geçiş yok).
fn night_light_scheduled(store: &Store) -> bool {
    night_light_on(store) && night_schedule(store) != "always"
}

pub const COLOR_SOURCES: &[(&str, &str)] = source_text!(&[("static", "Nuxut teması"), ("wallpaper", "Duvar kağıdından")]);
pub const MOTION_STYLES: &[(&str, &str)] = source_text!(&[("spring", "Yay"), ("curve", "Eğri")]);
pub const WALLPAPER_FITS: &[(&str, &str)] = source_text!(&[("cover", "Doldur ve kırp"), ("contain", "Tamamı"), ("fill", "Uzat")]);
/// Shell `night_light::ScheduleKind` (değerler ve adlar şemadan `option_labels`; bu tablo yedek).
pub const NIGHT_SCHEDULES: &[(&str, &str)] = source_text!(&[("always", "Her zaman"), ("manual", "Belirli saatlerde"), ("sun", "Gün batımından gün doğumuna")]);
/// Ekran klavyesi düzeni (shell `[osk] layout`, osk-v1; değerler ve adlar şemadan `option_labels`, bu
/// tablo yedek). `auto`: masaüstünde etkin klavye düzenini izler.
pub const OSK_LAYOUTS: &[(&str, &str)] = source_text!(&[("auto", "Otomatik"), ("tr", "Türkçe Q"), ("us", "İngilizce (ABD)")]);
/// Shell `WallpaperBackdropKind` (`[wallpaper] backdrop`, shell 88885da) etiketleri; değerler
/// şemadan ([`SettingDesc::choices`]), bu tablo şema yokken yedek (test şemayla karşılaştırır).
pub const WALLPAPER_BACKDROPS: &[(&str, &str)] = source_text!(&[("blur", "Bulanık görsel"), ("color", "Düz renk")]);
/// Shell `WallpaperBackdropColor` etiketleri (değerler şemadan, bu tablo yedek) — sabit renk yok,
/// yalnızca tema rolleri (tema değişince izler). Etiketler Görünüm › Renkler'deki rol adlarıyla aynı.
pub const WALLPAPER_BACKDROP_COLORS: &[(&str, &str)] = source_text!(&[("surface", "Yüzey"), ("surface_container", "Kart"), ("surface_bright", "Vurgulu yüzey")]);
/// Shell `WALLPAPER_BACKDROP_BLUR` yedeği (mantıksal px; dışı kırpılıp uyarılır); şema yüklüyse
/// aralık şemanın sınırıyla kesiştirilir ([`SettingDesc::bounds`]).
pub const WALLPAPER_BACKDROP_BLUR: (f64, f64) = (4.0, 128.0);
pub const OUTLINES: &[(&str, &str)] = source_text!(&[("none", "Yok"), ("subtle", "İnce"), ("full", "Tam")]);
/// Qt uygulamalarının görünüm yolu (`[desktop] qt_backend`; shell ADR-0004 §17-1). `auto`: kurulu olana göre.
/// Klavye ve Fare: girdi ayarları masaüstüne uygulanıyor mu (`compositor.sync_input`). Kapalıyken
/// satırlar gizlenir, sayfanın üstündeki not açıklar ve "Aç" sunar (bkz. `pages/input_note.rs`).
pub fn input_synced(store: &Store) -> bool {
    toggle_on(store, "compositor.sync_input")
}

/// Düzenler arası geçiş: yalnızca birden çok düzen varken (dosyadaki, yoksa etkin liste).
fn layout_switch_shown(store: &Store) -> bool {
    input_synced(store) && crate::pages::keyboard::current_layouts(store).len() > 1
}

/// Varsayılan uygulamalar: shell durumu okundu mu (`msg defaults status`). Okunmadıysa rol
/// satırları gizli, sayfanın notu nedenini söyler (bkz. `pages/default_apps.rs`). Değer dosyada
/// değil, servis durumunda: `Store` kullanılmaz.
fn defaults_ready(_store: &Store) -> bool {
    crate::services::default_apps::status().ready().is_some()
}

/// Girdi sayfasının üst notu (aramada gizli girdi satırı buraya yönlenir).
fn input_note_for(page: PageId) -> Option<&'static SettingDesc> {
    match page {
        PageId::Keyboard => by_id("keyboard.note"),
        PageId::Pointer => by_id("pointer.note"),
        _ => None,
    }
}

/// Fare ivmesi (`input.mouse.accel_profile`; şema sırası).
pub const ACCEL_PROFILES: &[(&str, &str)] = source_text!(&[("adaptive", "Uyarlanır"), ("flat", "Düz")]);

/// Dokunmatik yüzey kaydırma yöntemi (`input.touchpad.scroll_method`; şema sırası).
pub const SCROLL_METHODS: &[(&str, &str)] = source_text!(&[("2fg", "İki parmak"), ("edge", "Kenar"), ("on_button_down", "Düğmeyle"), ("no_scroll", "Kapalı")]);

pub const QT_BACKENDS: &[(&str, &str)] = source_text!(&[("auto", "Otomatik"), ("kde", "KDE"), ("qt6ct", "qt6ct")]);
/// Bar uç widget'ı tık eylemleri (shell `docs/design/bar-edge-actions.md`; şemadaki seçenekler).
/// Etiketler eylem kipinde (§8: "Sağ paneli aç"); değerler shell'in.
pub const EDGE_CLICK: &[(&str, &str)] = source_text!(&[
    ("widget", "Widget'ın kendi"),
    ("none", "Hiçbir şey"),
    ("panel:left", "Sol paneli aç"),
    ("panel:right", "Sağ paneli aç"),
    ("panel:top", "Üst paneli aç"),
    ("panel:bottom", "Alt paneli aç"),
    ("panel:clock", "Takvimi aç"),
    ("panel:launcher", "Uygulama başlatıcıyı aç"),
    ("volume", "Sesi kapat/aç"),
    ("mic", "Mikrofonu kapat/aç"),
]);
/// Workspace widget'ı (shell `docs/design/workspaces.md`; şemadaki seçenekler).
pub const WORKSPACE_STYLES: &[(&str, &str)] = source_text!(&[("icons", "İkon"), ("numbers", "Numara"), ("dots", "Nokta")]);
pub const WORKSPACE_SCROLLS: &[(&str, &str)] = source_text!(&[("all", "Tümü"), ("occupied", "Yalnızca dolu"), ("off", "Kapalı")]);
pub const EDGE_SCROLL: &[(&str, &str)] = source_text!(&[
    ("widget", "Widget'ın kendi"),
    ("none", "Hiçbir şey"),
    ("volume", "Ses"),
    ("mic", "Mikrofon"),
    ("brightness", "Parlaklık"),
    ("workspace", "Çalışma alanı"),
]);
pub const ROLE_STYLES: &[(&str, &str)] = source_text!(&[("", "Genel"), ("spring", "Yay"), ("curve", "Eğri")]);
/// Adlandırılmış eğriler (DESIGN.md); özel `cubic-bezier(…)` editörü framework bileşenini bekliyor.
pub const EASINGS: &[(&str, &str)] = source_text!(&[
    ("emphasized", "Vurgulu"),
    ("emphasized-decelerate", "Vurgulu yavaşlayan"),
    ("emphasized-accelerate", "Vurgulu hızlanan"),
    ("ease", "Yumuşak"),
    ("ease-out", "Yavaşlayan"),
    ("ease-in", "Hızlanan"),
    ("ease-in-out", "Yavaş-hızlı-yavaş"),
    ("linear", "Doğrusal"),
]);

impl MotionRole {
    const fn from_u8(v: u8) -> Self {
        match v {
            0 => MotionRole::Spatial,
            1 => MotionRole::SpatialFast,
            _ => MotionRole::Effects,
        }
    }

    /// Rolün etkin stili: rolde yazılıysa o, değilse `motion.style`.
    pub fn effective_style(self, store: &Store) -> &'static str {
        let own = KeyPath::new(["motion", self.key(), "style"]);
        let style = store.get(FileId::Framework, &own).and_then(Value::as_str).or_else(|| store.get(FileId::Framework, &KeyPath::parse("motion.style")).and_then(Value::as_str))
            .or_else(|| crate::defaults::get(store, FileId::Framework, &KeyPath::parse("motion.style")).and_then(Value::as_str));
        if style == Some("curve") { "curve" } else { "spring" }
    }
}

fn role_spring<const R: u8>(store: &Store) -> bool {
    MotionRole::from_u8(R).effective_style(store) == "spring"
}

fn role_curve<const R: u8>(store: &Store) -> bool {
    MotionRole::from_u8(R).effective_style(store) == "curve"
}

/// Renk rolleri: anahtar, başlık, açıklama, varsayılan (DESIGN.md).
pub static SETTINGS: &[SettingDesc] = &[
    // ---- Görünüm (§9: hero; Renkler; Köşeler ve şekil; Gölge; Uygulamalar) ----
    s!("preview", Appearance, "", "Önizleme", "", ["canlı", "örnek"], Framework, "", custom(CustomId::Preview)),
    // Renkler: koyu/açık (karar 5: duvar kağıdı paletinin kipi de bu satırda), kaynak, duvar
    // kağıdından ise şema + kontrast, vurgu; on rol tek açılır satırda (§3.5 A).
    s!("colors.mode", Appearance, "Renkler", "Koyu ya da açık", "Nuxut ve uygulamalar aynı kipte", ["karanlık", "aydınlık", "dark", "light", "kip", "gece", "tema"], Framework, "colors.mode", custom(CustomId::DarkMode)),
    // Eşanlamlı (§3.5): duvar kağıdı paletinin kipi `colors.mode` satırıyla tek adımda yazılır
    // (`look::mode_writes`); kendi satırı yok, arama koyu/açık satırına gider.
    s!("palette.mode", Appearance, "Renkler", "Renk şemasının kipi", "", ["karanlık", "aydınlık", "dark", "light", "palet", "kip"], Palette, "meta.mode", custom(CustomId::Alias("colors.mode"))),
    // Yüksek karşıtlık (framework t124, `accessibility.md` §1): açık = "high", kapalı = anahtar yok
    // ("standard"). Eski framework'te gizli.
    s!("colors.contrast", Appearance, "Renkler", "Yüksek karşıtlık", "Metin ve kenarlar daha belirgin olur",
        ["karşıtlık", "kontrast", "erişilebilirlik", "görme", "okunabilirlik", "high contrast"], Framework, "colors.contrast",
        Control::Toggle { invert: false, on: Some(CONTRAST_HIGH) }, if contrast_supported, mirror [Accessibility]),
    s!("colors.source", Appearance, "Renkler", "Renkler nereden gelsin", "Nuxut teması: hazır renk temaları. Duvar kağıdından: tüm renkler görsele uyar",
        ["renk kaynağı", "tema", "palet", "matugen", "material you", "duvar kağıdı", "otomatik renk"], Framework, "colors.source",
        Control::Choice { options: COLOR_SOURCES, style: ChoiceStyle::Segmented }),
    // Nuxut renk teması (framework `[colors] theme`, kullanıcı isteği 2026-10-16): renkler
    // duvar kağıdından gelmiyorsa altı önizleme kartı; varsayılan (mavi) anahtarı siler.
    s!("colors.theme", Appearance, "Renkler", "Renk teması", "Arayüz, uygulamalar ve Nuxut duvar kağıtları bu renklere boyanır",
        ["renk teması", "tema", "mavi", "nötr", "adaçayı", "gül kurusu", "kehribar", "grafit", "renk"], Framework, "colors.theme", custom(CustomId::ColorTheme), if static_colors),
    s!("palette", Appearance, "Renkler", "Şema", "Aynı görselden dokuz farklı palet", ["renk şeması", "matugen", "palet", "varyant", "tonal", "canlı"], Palette, "meta.scheme", custom(CustomId::Palette), if wallpaper_colors),
    s!("palette.contrast", Appearance, "Renkler", "Palet kontrastı", "Duvar kağıdından gelen renklerin birbirinden farkı; solda az, sağda çok", ["kontrast", "erişilebilirlik", "okunabilirlik"], Palette, "meta.contrast", slider(-1.0, 1.0, 0.1, ""), if wallpaper_colors),
    s!("colors.primary", Appearance, "Renkler", "Vurgu rengi", "Etkin çalışma alanı, seçim ve düğmeler", ["vurgu", "accent", "ana renk", "tema rengi"], Framework, "colors.primary", color()),
    s!("colors.on_primary", Appearance, "Renkler", "Vurgu üzerindeki yazı", "Vurgu renginin üstündeki yazı ve ikonlar", ["vurgu üstü metin"], Framework, "colors.on_primary", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.surface", Appearance, "Renkler", "Yüzey", "Çerçeve, bar ve panel zemini", ["arka plan", "zemin"], Framework, "colors.surface", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.surface_container", Appearance, "Renkler", "Kart", "Panel içindeki kartlar", [], Framework, "colors.surface_container", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.surface_bright", Appearance, "Renkler", "Vurgulu yüzey", "Fareyle üzerine gelince", ["hover", "parlak yüzey"], Framework, "colors.surface_bright", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.on_surface", Appearance, "Renkler", "Yazı", "Ana yazı rengi", ["metin", "yazı rengi"], Framework, "colors.on_surface", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.on_surface_variant", Appearance, "Renkler", "İkincil yazı", "Açıklamalar, soluk yazı", ["ikincil metin"], Framework, "colors.on_surface_variant", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.error", Appearance, "Renkler", "Hata", "Başarısız eylemler, uyarılar", ["kırmızı", "uyarı"], Framework, "colors.error", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.on_error", Appearance, "Renkler", "Hata üzerindeki yazı", "Hata renginin üstündeki yazı", ["hata üstü metin"], Framework, "colors.on_error", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.outline", Appearance, "Renkler", "Çizgi", "Ayırıcılar ve kenarlıklar", ["kenarlık"], Framework, "colors.outline", color(), detail "Renkleri tek tek ayarla"),
    s!("colors.shadow", Appearance, "Renkler", "Gölge rengi", "Gölgelerin rengi", [], Framework, "colors.shadow", color(), detail "Renkleri tek tek ayarla"),
    // Köşeler ve şekil (bugünkü Köşeler + Şekiller tek kart).
    s!("shape.radius_small", Appearance, "Köşeler ve şekil", "Küçük öğeler", "Çipler, çalışma alanı göstergeleri", ["köşe", "yarıçap", "radius", "yuvarlak"], Framework, "shape.radius_small", slider(0.0, 32.0, 1.0, "px")),
    s!("shape.radius_medium", Appearance, "Köşeler ve şekil", "Kartlar", "Panel içindeki kartlar", ["köşe", "yarıçap", "radius", "yuvarlak"], Framework, "shape.radius_medium", slider(0.0, 48.0, 1.0, "px")),
    s!("shape.radius_large", Appearance, "Köşeler ve şekil", "Paneller", "Paneller ve pencereler", ["köşe", "yarıçap", "radius", "yuvarlak"], Framework, "shape.radius_large", slider(0.0, 64.0, 1.0, "px")),
    s!("shape.outlines", Appearance, "Köşeler ve şekil", "Kenar çizgileri", "Bileşen sınırları; yüksek karşıtlık açıkken en az ince çizilir", ["kontrast", "erişilebilirlik", "kenarlık", "outline"], Framework, "shape.outlines",
        Control::Choice { options: OUTLINES, style: ChoiceStyle::Segmented }),
    s!("blob.smoothing", Appearance, "Köşeler ve şekil", "Birleşim kavisi", "Çerçevenin iç köşeleri ve panel birleşimleri", ["birleşim yumuşaklığı", "blob", "fileto", "puding", "smoothing", "iç kavis", "iç köşe", "çerçeve"], Framework, "blob.smoothing", slider(0.0, 40.0, 1.0, "px")),
    s!("blob.deform", Appearance, "Köşeler ve şekil", "Esneme", "Hızlı panellerin esneyip basılması", ["blob", "deform", "jöle", "puding"], Framework, "blob.deform", slider(0.0, 1.5, 0.05, "")),
    s!("shadow.enabled", Appearance, "Gölge", "Gölgeler", "Bar, panel ve menülerin altında yumuşak gölge", ["shadow", "derinlik"], Framework, "shadow.enabled", toggle()),
    s!("shadow.blur", Appearance, "Gölge", "Yumuşaklık", "Gölgenin bulanıklık yarıçapı", ["blur", "bulanık"], Framework, "shadow.blur", slider(0.0, 64.0, 1.0, "px"), if shadow_on),
    s!("shadow.offset_y", Appearance, "Gölge", "Dikey kayma", "Gölgenin aşağı kayması", ["offset"], Framework, "shadow.offset_y", slider(-16.0, 16.0, 0.5, "px"), if shadow_on),
    s!("shadow.opacity", Appearance, "Gölge", "Yoğunluk", "Gölgenin opaklığı", ["opacity", "koyuluk"], Framework, "shadow.opacity", percent(), if shadow_on),

    // ---- Görünüm › Uygulamalar (shell ADR-0004 birleşik görünüm; shell uygular) ----
    s!("desktop.unified", Appearance, "Uygulamalar", "Uygulamalar da Nuxut gibi görünsün", "Renk, ikon, imleç ve yazı tipleri Nuxut'tan; her an geri alınır", ["birleşik görünüm", "gtk", "qt", "kde", "libadwaita", "tema", "uyumlu"], Framework, "desktop.unified", toggle()),
    s!("desktop.status", Appearance, "Uygulamalar", "Durum", "", ["eksik paket", "kurulum"], Framework, "", custom(CustomId::LookStatus), if unified_on),
    s!("desktop.icon_theme", Appearance, "Uygulamalar", "İkon teması", "Uygulama, dosya ve klasör ikonları", ["simge", "icon", "papirus", "adwaita", "breeze"], Framework, "desktop.icon_theme", custom(CustomId::IconTheme), if unified_on),
    s!("desktop.cursor_theme", Appearance, "Uygulamalar", "İmleç", "Fare imlecinin görünümü", ["fare", "cursor", "işaretçi", "bibata", "adwaita", "vimix", "capitaine", "oreo", "volantes", "qogir", "sweet"], Framework, "desktop.cursor_theme", custom(CustomId::CursorTheme), if unified_on),
    s!("desktop.cursor_size", Appearance, "Uygulamalar", "İmleç boyutu", "Büyük imleç daha kolay görünür", ["fare", "cursor", "büyüklük", "erişilebilirlik"], Framework, "desktop.cursor_size",
        Control::Slider { min: DESKTOP_CURSOR_MIN, max: DESKTOP_CURSOR_MAX, step: 8.0, unit: "px", integer: true, percent: false }, if unified_on, mirror [Accessibility]),
    s!("desktop.qt_backend", Appearance, "Uygulamalar", "Qt uygulamaları", "Otomatik: kuruluysa KDE'nin tema eklentisi, yoksa qt6ct", ["qt", "kde", "plasma", "qt6ct", "breeze", "kvantum"], Framework, "desktop.qt_backend",
        Control::Choice { options: QT_BACKENDS, style: ChoiceStyle::Segmented }, if unified_on),

    // Maskot Fiyu (framework mascot.md; karar kullanıcı 2026-10-13): "gizle" anahtarı,
    // sayfanın son kartı — tek satırlık kart Gölge ile Uygulamalar arasını bölmesin (settings-ux §5.2),
    // varsayılan kapalı (= görünür). Açık → `[appearance] mascot = false`; kapalı → anahtar silinir.
    // Hoş geldin'de yok. Şeması tanımayan framework'te gizli (hour_cycle kalıbı).
    s!("appearance.mascot", Appearance, "Maskot", "Fiyu'yu gizle", "Boş ekranlarda ve hata mesajlarında görünen maskot",
        ["maskot", "fiyu", "kuş", "kırlangıç"], Framework, "appearance.mascot", Control::Toggle { invert: true, on: None }, if mascot_supported),

    // ---- Yazı (ADR-0006: rol başına aile + kalınlık + ölçek; §9: Yazı tipleri tek kart) ----
    s!("typography.preview", Fonts, "", "Önizleme", "", ["örnek", "font"], Framework, "typography", custom(CustomId::TypePreview)),
    s!("typography.size", Fonts, "Yazı tipleri", "Yazı boyutu", "Tüm yazıların taban boyutu", ["font", "punto", "büyüklük"], Framework, "typography.size", slider(9.0, 20.0, 0.5, "px"), mirror [Accessibility]),
    s!("typography.ui.family", Fonts, "Yazı tipleri", "Arayüz", "Düğmeler, menüler, ayar satırları", ["font", "aile", "yazı tipi", "kalınlık", "weight", "inter", "sans"], Framework, "typography.ui.family", custom(CustomId::FontFamily(FontRole::Ui))),
    s!("typography.display.family", Fonts, "Yazı tipleri", "Başlık", "Saat, büyük başlıklar", ["font", "aile", "yazı tipi", "kalınlık", "weight", "saat", "display"], Framework, "typography.display.family", custom(CustomId::FontFamily(FontRole::Display))),
    s!("typography.document.family", Fonts, "Yazı tipleri", "Belge", "Uzun metin: bildirim gövdesi, takvim notları", ["font", "aile", "yazı tipi", "kalınlık", "weight", "serif", "okuma"], Framework, "typography.document.family", custom(CustomId::FontFamily(FontRole::Document))),
    s!("typography.mono.family", Fonts, "Yazı tipleri", "Eş aralıklı", "Kod ve sayı tabloları; Nerd Font ikonlarının da yazı tipi", ["font", "aile", "yazı tipi", "kalınlık", "weight", "monospace", "kod", "nerd font"], Framework, "typography.mono.family", custom(CustomId::FontFamily(FontRole::Mono))),
    s!("typography.ui.scale", Fonts, "Yazı tipleri", "Arayüz boyut oranı", "", ["font", "büyüklük", "ölçek", "oran"], Framework, "typography.ui.scale", SCALE, detail "Boyut oranları ve ikon yazı tipi"),
    s!("typography.display.scale", Fonts, "Yazı tipleri", "Başlık boyut oranı", "", ["font", "büyüklük", "ölçek", "oran"], Framework, "typography.display.scale", SCALE, detail "Boyut oranları ve ikon yazı tipi"),
    s!("typography.document.scale", Fonts, "Yazı tipleri", "Belge boyut oranı", "", ["font", "büyüklük", "ölçek", "oran"], Framework, "typography.document.scale", SCALE, detail "Boyut oranları ve ikon yazı tipi"),
    s!("typography.mono.scale", Fonts, "Yazı tipleri", "Eş aralıklı boyut oranı", "", ["font", "büyüklük", "ölçek", "oran"], Framework, "typography.mono.scale", SCALE, detail "Boyut oranları ve ikon yazı tipi"),
    s!("typography.icon.family", Fonts, "Yazı tipleri", "İkon yazı tipi", "Boşsa ikon setinin kendi yazı tipi", ["font", "ikon fontu", "sembol", "kalınlık", "weight", "material symbols"], Framework, "typography.icon.family", custom(CustomId::FontFamily(FontRole::Icon)), detail "Boyut oranları ve ikon yazı tipi"),
    s!("typography.icon.scale", Fonts, "Yazı tipleri", "İkon boyut oranı", "", ["ikon", "büyüklük", "ölçek", "oran"], Framework, "typography.icon.scale", SCALE, detail "Boyut oranları ve ikon yazı tipi"),
    s!("typography.icon_set", Fonts, "İkonlar", "İkon seti", "Arayüzdeki tüm ikonların çizim seti", ["ikon", "simge", "material", "nerd font", "sembol"], Framework, "typography.icon_set", custom(CustomId::IconSet)),

    // ---- Hareket (§9: karakter kartları hero; genel; rol ayarları tek açılır satırda, rol başına
    // alt bölüm — bölüm adı aramada rolü ayırt eder; karar 6 rol adları) ----
    s!("motion.preset", Motion, "", "Karakter", "Tüm hareketlerin karakteri", ["ön ayar", "sakin", "canlı", "çevik", "kapalı", "puding", "preset", "animasyon"], Framework, "motion.preset", custom(CustomId::MotionPreset)),
    s!("motion.enabled", Motion, "Genel", "Hareketi azalt", "Animasyonlar kapanır; her şey anında değişir", ["animasyon", "erişilebilirlik", "reduce motion"], Framework, "motion.enabled", Control::Toggle { invert: true, on: None }, mirror [Accessibility]),
    s!("motion.speed", Motion, "Genel", "Hız", "Tüm animasyonların çarpanı; 2× iki kat hızlı", ["animasyon hızı", "speed", "süre"], Framework, "motion.speed", slider(0.25, 3.0, 0.05, "×")),
    s!("motion.style", Motion, "Genel", "Hareket türü", "Yay doğal ve kesilebilir; eğri sabit sürelidir", ["stil", "yay", "eğri", "spring", "curve", "bezier", "easing"], Framework, "motion.style",
        Control::Choice { options: MOTION_STYLES, style: ChoiceStyle::Segmented }, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial", Motion, "Paneller ve pencereler", "Paneller ve pencereler", "Açılma, boyut ve konum değişimi", ["mekânsal", "spatial", "animasyon", "önizleme"], Framework, "motion.spatial", custom(CustomId::MotionRole(MotionRole::Spatial)), detail "Hareketi tek tek ayarla"),
    s!("motion.spatial.style", Motion, "Paneller ve pencereler", "Hareket türü", "Genel: üstteki tür", ["stil", "yay", "eğri", "spring", "curve"], Framework, "motion.spatial.style",
        Control::Choice { options: ROLE_STYLES, style: ChoiceStyle::Segmented }, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial.response", Motion, "Paneller ve pencereler", "Süre", "Kısa = hızlı", ["response", "yay", "hız"], Framework, "motion.spatial.response", slider(0.05, 1.5, 0.01, "sn"), if role_spring::<{ MotionRole::Spatial as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial.damping", Motion, "Paneller ve pencereler", "Yaylanma", "Düşük değer hedefi aşıp geri oturur", ["sönüm", "damping", "zıplama", "puding"], Framework, "motion.spatial.damping", slider(0.2, 1.5, 0.01, ""), if role_spring::<{ MotionRole::Spatial as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial.easing", Motion, "Paneller ve pencereler", "Eğri", "Hızlanıp yavaşlama biçimi", ["easing", "bezier", "cubic-bezier"], Framework, "motion.spatial.easing",
        Control::Choice { options: EASINGS, style: ChoiceStyle::Dropdown }, if role_curve::<{ MotionRole::Spatial as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial.duration_ms", Motion, "Paneller ve pencereler", "Süre", "Eğrinin toplam süresi", ["duration", "ms"], Framework, "motion.spatial.duration_ms", Control::Slider { min: 0.0, max: 2000.0, step: 10.0, unit: "ms", integer: true, percent: false }, if role_curve::<{ MotionRole::Spatial as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial_fast", Motion, "Menüler ve küçük öğeler", "Menüler ve küçük öğeler", "Menüler, göstergeler, küçük düğmeler", ["hızlı mekânsal", "spatial_fast", "animasyon", "önizleme"], Framework, "motion.spatial_fast", custom(CustomId::MotionRole(MotionRole::SpatialFast)), detail "Hareketi tek tek ayarla"),
    s!("motion.spatial_fast.style", Motion, "Menüler ve küçük öğeler", "Hareket türü", "Genel: üstteki tür", ["stil", "yay", "eğri", "spring", "curve"], Framework, "motion.spatial_fast.style",
        Control::Choice { options: ROLE_STYLES, style: ChoiceStyle::Segmented }, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial_fast.response", Motion, "Menüler ve küçük öğeler", "Süre", "Kısa = hızlı", ["response", "yay", "hız"], Framework, "motion.spatial_fast.response", slider(0.05, 1.5, 0.01, "sn"), if role_spring::<{ MotionRole::SpatialFast as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial_fast.damping", Motion, "Menüler ve küçük öğeler", "Yaylanma", "Düşük değer hedefi aşıp geri oturur", ["sönüm", "damping", "zıplama", "puding"], Framework, "motion.spatial_fast.damping", slider(0.2, 1.5, 0.01, ""), if role_spring::<{ MotionRole::SpatialFast as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial_fast.easing", Motion, "Menüler ve küçük öğeler", "Eğri", "Hızlanıp yavaşlama biçimi", ["easing", "bezier", "cubic-bezier"], Framework, "motion.spatial_fast.easing",
        Control::Choice { options: EASINGS, style: ChoiceStyle::Dropdown }, if role_curve::<{ MotionRole::SpatialFast as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.spatial_fast.duration_ms", Motion, "Menüler ve küçük öğeler", "Süre", "Eğrinin toplam süresi", ["duration", "ms"], Framework, "motion.spatial_fast.duration_ms", Control::Slider { min: 0.0, max: 2000.0, step: 10.0, unit: "ms", integer: true, percent: false }, if role_curve::<{ MotionRole::SpatialFast as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.effects", Motion, "Solma ve renk", "Solma ve renk", "Saydamlık ve renk geçişleri; taşmaz", ["efektler", "effects", "opaklık", "animasyon", "önizleme"], Framework, "motion.effects", custom(CustomId::MotionRole(MotionRole::Effects)), detail "Hareketi tek tek ayarla"),
    s!("motion.effects.style", Motion, "Solma ve renk", "Hareket türü", "Genel: üstteki tür", ["stil", "yay", "eğri", "spring", "curve"], Framework, "motion.effects.style",
        Control::Choice { options: ROLE_STYLES, style: ChoiceStyle::Segmented }, detail "Hareketi tek tek ayarla"),
    s!("motion.effects.response", Motion, "Solma ve renk", "Süre", "Kısa = hızlı", ["response", "yay", "hız"], Framework, "motion.effects.response", slider(0.05, 1.5, 0.01, "sn"), if role_spring::<{ MotionRole::Effects as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.effects.damping", Motion, "Solma ve renk", "Yaylanma", "Düşük değer hedefi aşıp geri oturur", ["sönüm", "damping", "zıplama", "puding"], Framework, "motion.effects.damping", slider(0.2, 1.5, 0.01, ""), if role_spring::<{ MotionRole::Effects as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.effects.easing", Motion, "Solma ve renk", "Eğri", "Hızlanıp yavaşlama biçimi", ["easing", "bezier", "cubic-bezier"], Framework, "motion.effects.easing",
        Control::Choice { options: EASINGS, style: ChoiceStyle::Dropdown }, if role_curve::<{ MotionRole::Effects as u8 }>, detail "Hareketi tek tek ayarla"),
    s!("motion.effects.duration_ms", Motion, "Solma ve renk", "Süre", "Eğrinin toplam süresi", ["duration", "ms"], Framework, "motion.effects.duration_ms", Control::Slider { min: 0.0, max: 2000.0, step: 10.0, unit: "ms", integer: true, percent: false }, if role_curve::<{ MotionRole::Effects as u8 }>, detail "Hareketi tek tek ayarla"),

    // ---- Erişilebilirlik (Kişisel; settings-ux §13.2, 2026-10-15): Görme kartı Görünüm/Yazı/
    // Hareket'teki satırların aynası (`PageId::mirror_home`); büyüteç ve ekran klavyesi shell
    // sözleşmeleri magnifier-v1 / osk-v1 (shell docs/design/magnifier.md, osk.md). Kısayollar
    // Kısayollar sayfasında; burada yalnızca gösterilir ve oraya yönlendirilir.
    s!("magnifier.live", Accessibility, "Büyüteç", "Büyüteç", "Ekranın imleç çevresini büyütür", ["büyüteç", "yakınlaştır", "zoom", "magnifier", "görme", "büyüt"], Shell, "",
        custom(CustomId::Assist(crate::services::assist::Tool::Magnifier))),
    s!("magnifier.factor", Accessibility, "Büyüteç", "Açılış büyütmesi", "Büyüteç açılınca kullanılan büyütme", ["büyüteç", "çarpan", "zoom", "factor"], Shell, "magnifier.factor", slider(1.1, 10.0, 0.25, "×")),
    s!("magnifier.step", Accessibility, "Büyüteç", "Adım", "Her yakınlaştırma büyütmeyi bu sayıyla çarpar", ["büyüteç", "adım", "zoom", "step"], Shell, "magnifier.step", slider(1.05, 2.0, 0.05, "×")),
    s!("magnifier.max", Accessibility, "Büyüteç", "En fazla büyütme", "Yakınlaştırma bu sınırda durur", ["büyüteç", "sınır", "zoom", "max"], Shell, "magnifier.max", slider(1.5, 10.0, 0.5, "×")),
    s!("magnifier.keys", Accessibility, "Büyüteç", "Kısayollar", "", ["büyüteç", "kısayol", "tuş", "zoom"], Shell, "", custom(CustomId::Link("keybinds.magnifier_in"))),
    s!("osk.live", Accessibility, "Ekran klavyesi", "Ekran klavyesi", "Fareyle ya da dokunmatik ekranla yazmak için", ["ekran klavyesi", "sanal klavye", "dokunmatik", "osk", "on-screen keyboard"], Shell, "",
        custom(CustomId::Assist(crate::services::assist::Tool::Osk))),
    s!("osk.layout", Accessibility, "Ekran klavyesi", "Düzen", "Otomatik: masaüstündeki klavye düzenini izler", ["ekran klavyesi", "düzen", "türkçe q", "layout"], Shell, "osk.layout",
        Control::Choice { options: OSK_LAYOUTS, style: ChoiceStyle::Segmented }),
    s!("osk.key_height", Accessibility, "Ekran klavyesi", "Tuş yüksekliği", "Klavyenin boyu ve genişliği bundan", ["ekran klavyesi", "boyut", "tuş"], Shell, "osk.key_height", int_slider(32.0, 72.0, "px")),
    s!("osk.keys", Accessibility, "Ekran klavyesi", "Kısayol", "", ["ekran klavyesi", "kısayol", "tuş"], Shell, "", custom(CustomId::Link("keybinds.osk_toggle")), if osk_keybind_row),

    // ---- Kısayollar + Paneller (iç içe sıralı) ----------------------------------------------
    // Kısayollar (shell.toml [keybinds]; eylemler ve sıra shell `Action::ALL`): birincil yer
    // Kısayollar sayfası. Panel kısayolları (§3.4 ayna) aynı kayıtla panel kartında da çizilir;
    // ayna satır kayıt sırasıyla yerleştiği için sağ/sol panel kısayolu Paneller bloğunun
    // içinde, kendi panel kartının ikinci satırındadır (Kısayollar sayfasında sıra aynı kalır:
    // aradaki panel satırları orada çizilmez). Kısayol eylem sırası (`keybind_rows`) değişmez.
    // ---- Klavye (sözleşme girdi-v1; değer: dosya > etkin > şema varsayılanı) ----
    s!("keyboard.note", Keyboard, "Durum", "Klavye ayarlarının durumu", "", ["masaüstü", "kabuk", "uygulanmıyor"], Shell, "", custom(CustomId::InputNote)),
    s!("input.keyboard.layouts", Keyboard, "Düzenler", "Klavye düzenleri", "", ["klavye", "düzen", "layout", "xkb", "varyant", "variant", "kb_layout", "dil", "q klavye", "f klavye"], Shell, "input.keyboard.layouts", custom(CustomId::KeyboardLayouts), if input_synced),
    s!("input.keyboard.switch", Keyboard, "Düzenler", "Düzenler arası geçiş", "Düzeni değiştiren tuş birleşimi", ["klavye", "düzen", "geçiş", "değiştir", "grp", "kb_options", "alt shift", "layout"], Shell, "input.keyboard.options", custom(CustomId::LayoutSwitch), if layout_switch_shown),
    s!("input.keyboard.repeat_delay", Keyboard, "Tuş tekrarı", "Tekrar gecikmesi", "Basılı tutulan tuş bu süreden sonra tekrarlar", ["klavye", "tekrar", "gecikme", "repeat", "delay", "basılı tut"], Shell, "input.keyboard.repeat_delay", int_slider(100.0, 2000.0, "ms"), if input_synced),
    s!("input.keyboard.repeat_rate", Keyboard, "Tuş tekrarı", "Tekrar hızı", "Saniyedeki tekrar sayısı", ["klavye", "tekrar", "hız", "repeat", "rate"], Shell, "input.keyboard.repeat_rate", int_slider(1.0, 100.0, "tuş/sn"), if input_synced),
    s!("input.keyboard.try", Keyboard, "Tuş tekrarı", "Burada deneyin", "", ["deneme", "dene", "test", "tekrar"], Shell, "", custom(CustomId::KeyTest), if input_synced),
    s!("input.keyboard.numlock_by_default", Keyboard, "Diğer", "Açılışta Num Lock açık", "Oturum açılınca sayı tuşları etkin olur", ["numlock", "num lock", "sayı tuşları", "klavye"], Shell, "input.keyboard.numlock_by_default", toggle(), if input_synced),
    // ---- Fare ve dokunmatik yüzey ----
    s!("pointer.note", Pointer, "Durum", "Fare ve dokunmatik yüzey durumu", "", ["masaüstü", "kabuk", "uygulanmıyor"], Shell, "", custom(CustomId::InputNote)),
    s!("input.mouse.sensitivity", Pointer, "Genel", "İşaretçi hızı", "Fare ve dokunmatik yüzey için", ["fare", "işaretçi", "imleç", "hız", "hassasiyet", "sensitivity", "mouse", "touchpad", "dokunmatik"], Shell, "input.mouse.sensitivity", slider(-1.0, 1.0, 0.05, ""), if input_synced),
    s!("input.mouse.accel_profile", Pointer, "Genel", "İvme", "Uyarlanır: hızlı hareket işaretçiyi daha uzağa götürür", ["fare", "ivme", "hızlanma", "acceleration", "accel", "düz", "flat"], Shell, "input.mouse.accel_profile",
        Control::Choice { options: ACCEL_PROFILES, style: ChoiceStyle::Segmented }, if input_synced),
    s!("input.mouse.natural_scroll", Pointer, "Fare", "Doğal kaydırma", "Tekerlek içeriği kaydırma yönünde taşır", ["fare", "tekerlek", "doğal kaydırma", "natural scroll", "ters kaydırma", "mouse"], Shell, "input.mouse.natural_scroll", toggle(), if input_synced),
    s!("input.touchpad.natural_scroll", Pointer, "Dokunmatik yüzey", "Doğal kaydırma", "İçerik parmakların yönünde kayar", ["dokunmatik", "touchpad", "doğal kaydırma", "natural scroll", "ters kaydırma"], Shell, "input.touchpad.natural_scroll", toggle(), if input_synced),
    s!("input.touchpad.tap_to_click", Pointer, "Dokunmatik yüzey", "Dokunarak tıkla", "Hafif dokunuş tıklama sayılır", ["dokunmatik", "touchpad", "tap", "dokun", "tıkla"], Shell, "input.touchpad.tap_to_click", toggle(), if input_synced),
    s!("input.touchpad.disable_while_typing", Pointer, "Dokunmatik yüzey", "Yazarken devre dışı", "Yazarken avuç içi dokunuşları yok sayılır", ["dokunmatik", "touchpad", "yazarken", "avuç", "palm", "dwt"], Shell, "input.touchpad.disable_while_typing", toggle(), if input_synced),
    s!("input.touchpad.scroll_factor", Pointer, "Dokunmatik yüzey", "Kaydırma hızı", "", ["dokunmatik", "touchpad", "kaydırma", "hız", "scroll", "factor"], Shell, "input.touchpad.scroll_factor", slider(0.1, 3.0, 0.1, "×"), if input_synced),
    s!("input.touchpad.scroll_method", Pointer, "Dokunmatik yüzey", "Kaydırma yöntemi", "", ["dokunmatik", "touchpad", "kaydırma", "iki parmak", "kenar", "scroll method"], Shell, "input.touchpad.scroll_method",
        Control::Choice { options: SCROLL_METHODS, style: ChoiceStyle::Segmented }, if input_synced),
    s!("compositor.sync_keybinds", Keybinds, "Genel", "Nuxut kısayolları", "Aynı tuşa bağlı masaüstü kısayolu varsa o geçerli kalır", ["kısayol", "tuş", "bind", "eşitle"], Shell, "compositor.sync_keybinds", toggle()),
    s!("compositor.base_layer", Keybinds, "Genel", "Hazır pencere yönetimi", "Pencere kısayolları ve kuralları; Hyprland ayarı üstün gelir", ["kısayol", "pencere", "çalışma alanı", "temel katman", "hyprland", "kural"], Shell, "compositor.base_layer", toggle()),
    s!("compositor.take_over_keys", Keybinds, "Genel", "Medya ve ses tuşlarını Nuxut kullansın", "Hyprland'deki aynı tuşlar Nuxut'a geçer; dosya değişmez", ["kısayol", "tuş", "ses", "medya", "parlaklık", "ekran görüntüsü", "devral"], Shell, "compositor.take_over_keys", toggle(), if take_over_known),
    s!("keybinds.volume_up", Keybinds, "Ses", "Sesi aç", "", ["kısayol", "tuş", "volume", "xf86"], Shell, "keybinds.volume_up", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.volume_down", Keybinds, "Ses", "Sesi kıs", "", ["kısayol", "tuş", "volume", "xf86"], Shell, "keybinds.volume_down", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.volume_mute", Keybinds, "Ses", "Sesi kapat / aç", "", ["kısayol", "tuş", "sessiz", "mute", "xf86"], Shell, "keybinds.volume_mute", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.mic_mute", Keybinds, "Ses", "Mikrofonu kapat / aç", "", ["kısayol", "tuş", "mikrofon", "mute", "xf86"], Shell, "keybinds.mic_mute", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.brightness_up", Keybinds, "Parlaklık", "Parlaklığı artır", "", ["kısayol", "tuş", "ekran", "brightness", "xf86"], Shell, "keybinds.brightness_up", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.brightness_down", Keybinds, "Parlaklık", "Parlaklığı azalt", "", ["kısayol", "tuş", "ekran", "brightness", "xf86"], Shell, "keybinds.brightness_down", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.media_play", Keybinds, "Medya", "Oynat / duraklat", "", ["kısayol", "tuş", "müzik", "playerctl", "xf86"], Shell, "keybinds.media_play", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.media_pause", Keybinds, "Medya", "Duraklat tuşu", "", ["kısayol", "tuş", "müzik", "duraklat", "playerctl", "xf86"], Shell, "keybinds.media_pause", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.media_next", Keybinds, "Medya", "Sonraki parça", "", ["kısayol", "tuş", "müzik", "playerctl", "xf86"], Shell, "keybinds.media_next", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.media_prev", Keybinds, "Medya", "Önceki parça", "", ["kısayol", "tuş", "müzik", "playerctl", "xf86"], Shell, "keybinds.media_prev", custom(CustomId::Keybind), if keybinds_synced),

    // Paneller (§7): kenar başına tek kart — kullan → kısayol (ayna) → genişlik/yükseklik →
    // içerik → "Bar'dan açma". Kapalı panel tek satır. Sıra: Sağ, Sol, Üst, Alt. Hero yok
    // (karar 7: ikinci aşama).
    s!("panels.right.enabled", Panels, "Sağ panel", "Sağ paneli kullan", "Ekranın sağ kenarından açılan çekmece", ["sağ panel", "çekmece", "drawer", "kenar paneli"], Shell, "panels.right.enabled", toggle()),
    s!("keybinds.panel_right", Keybinds, "Bar ve paneller", "Sağ paneli aç / kapat", "", ["kısayol", "tuş", "çekmece", "hızlı ayarlar"], Shell, "keybinds.panel_right", custom(CustomId::Keybind), if panel_right_bind, mirror [Panels]),
    s!("panels.right.size", Panels, "Sağ panel", "Genişlik", "", ["sağ panel", "genişlik", "boyut", "derinlik"], Shell, "panels.right.size", slider(160.0, 700.0, 10.0, "px"), if panel_right),
    s!("panels.right.widgets", Panels, "Sağ panel", "İçerik", "", ["sağ panel", "kart", "widget", "hızlı ayarlar", "medya", "bildirim", "sıra"], Shell, "panels.right.widgets", custom(CustomId::PanelWidgets("right")), if panel_right),
    s!("panels.right.from_bar", Panels, "Sağ panel", "Bar'dan açma", "", ["sağ panel", "uç eylemi", "tık", "bar"], Shell, "", custom(CustomId::PanelHint("right")), if panel_hint_right),
    s!("panels.left.enabled", Panels, "Sol panel", "Sol paneli kullan", "Ekranın sol kenarından açılan çekmece", ["sol panel", "çekmece", "drawer", "kenar paneli"], Shell, "panels.left.enabled", toggle()),
    s!("keybinds.panel_left", Keybinds, "Bar ve paneller", "Sol paneli aç / kapat", "", ["kısayol", "tuş", "çekmece"], Shell, "keybinds.panel_left", custom(CustomId::Keybind), if panel_left_bind, mirror [Panels]),
    s!("panels.left.size", Panels, "Sol panel", "Genişlik", "", ["sol panel", "genişlik", "boyut", "derinlik"], Shell, "panels.left.size", slider(160.0, 700.0, 10.0, "px"), if panel_left),
    s!("panels.left.widgets", Panels, "Sol panel", "İçerik", "", ["sol panel", "kart", "widget", "hızlı ayarlar", "medya", "bildirim", "sıra"], Shell, "panels.left.widgets", custom(CustomId::PanelWidgets("left")), if panel_left),
    s!("panels.left.from_bar", Panels, "Sol panel", "Bar'dan açma", "", ["sol panel", "uç eylemi", "tık", "bar"], Shell, "", custom(CustomId::PanelHint("left")), if panel_hint_left),
    s!("panels.top.enabled", Panels, "Üst panel", "Üst paneli kullan", "Ekranın üst kenarından açılan çekmece", ["üst panel", "çekmece", "drawer", "kenar paneli"], Shell, "panels.top.enabled", toggle()),
    s!("panels.top.size", Panels, "Üst panel", "Yükseklik", "", ["üst panel", "yükseklik", "boyut", "derinlik"], Shell, "panels.top.size", slider(160.0, 700.0, 10.0, "px"), if panel_top),
    s!("panels.top.widgets", Panels, "Üst panel", "İçerik", "", ["üst panel", "kart", "widget", "hızlı ayarlar", "medya", "bildirim", "sıra"], Shell, "panels.top.widgets", custom(CustomId::PanelWidgets("top")), if panel_top),
    s!("panels.top.from_bar", Panels, "Üst panel", "Bar'dan açma", "", ["üst panel", "uç eylemi", "tık", "bar"], Shell, "", custom(CustomId::PanelHint("top")), if panel_hint_top),
    s!("panels.bottom.enabled", Panels, "Alt panel", "Alt paneli kullan", "Ekranın alt kenarından açılan çekmece", ["alt panel", "çekmece", "drawer", "kenar paneli"], Shell, "panels.bottom.enabled", toggle()),
    s!("panels.bottom.size", Panels, "Alt panel", "Yükseklik", "", ["alt panel", "yükseklik", "boyut", "derinlik"], Shell, "panels.bottom.size", slider(160.0, 700.0, 10.0, "px"), if panel_bottom),
    s!("panels.bottom.widgets", Panels, "Alt panel", "İçerik", "", ["alt panel", "kart", "widget", "hızlı ayarlar", "medya", "bildirim", "sıra"], Shell, "panels.bottom.widgets", custom(CustomId::PanelWidgets("bottom")), if panel_bottom),
    s!("panels.bottom.from_bar", Panels, "Alt panel", "Bar'dan açma", "", ["alt panel", "uç eylemi", "tık", "bar"], Shell, "", custom(CustomId::PanelHint("bottom")), if panel_hint_bottom),

    // Kısayollar (son): takvim (`panel:clock` panel değil: kısayolu yalnızca burada); bar kısayolu
    // aşağıda Bar bloğunun içinde.
    s!("keybinds.panel_clock", Keybinds, "Bar ve paneller", "Takvimi aç / kapat", "", ["kısayol", "tuş", "saat", "takvim"], Shell, "keybinds.panel_clock", custom(CustomId::Keybind), if keybinds_synced),

    // Bar bloğu kayıtta Kısayollar/Paneller'den sonra: bar kısayolu (eylem sırasında son) Bar'da
    // "Konum ve boyut" kartının sonunda aynalanır (§3.4); sayfa sırası değişmez.
    // ---- Bar (§6.1 öneri A, §9: önizleme → seçili widget → widget'lar → konum ve boyut → çerçeve) ----
    // Hero: tıklanabilir bar önizlemesi; seçim sayfa durumunda (`PageUi`), config'e yazılmaz.
    s!("bar.preview", Bar, "", "Bar önizlemesi", "", ["önizleme", "widget", "seç"], Shell, "", custom(CustomId::BarPreview)),
    // Seçili widget kartı: başlık (+ şemadaki diğer widget ayarları, dinamik), widget'ın kendi
    // ayarları, uçtaysa tıklama/tekerlek. Satırların görünürlüğü seçime bağlı (`ui_if`).
    s!("bar.selected", Bar, "Seçili widget", "Seçili widget", "", ["widget", "ayar"], Shell, "", custom(CustomId::SelectedWidget)),
    s!("bar.widget.workspaces.count", Bar, "Seçili widget", "Görünen çalışma alanı", "Fazlası oklarla sonraki sayfada", ["çalışma alanı", "workspace", "sayı", "sayfa başına"], Shell, "bar.widget.workspaces.count", int_slider(1.0, 30.0, ""), ui_if SEL_WORKSPACES),
    s!("bar.widget.workspaces.style", Bar, "Seçili widget", "Görünüm", "Uygulama ikonu, numara ya da nokta", ["çalışma alanı", "workspace", "ikon", "numara", "nokta"], Shell, "bar.widget.workspaces.style",
        Control::Choice { options: WORKSPACE_STYLES, style: ChoiceStyle::Segmented }, ui_if SEL_WORKSPACES),
    s!("bar.widget.workspaces.scroll", Bar, "Seçili widget", "Tekerlekle geçiş", "Widget'ın üstünde tekerlek çalışma alanını değiştirir", ["çalışma alanı", "workspace", "tekerlek", "kaydırma", "scroll"], Shell, "bar.widget.workspaces.scroll",
        Control::Choice { options: WORKSPACE_SCROLLS, style: ChoiceStyle::Segmented }, ui_if SEL_WORKSPACES),
    // Yer tutucu `SettingDesc::placeholder` (boş = otomatik); alt metin şemanın `description`'ı,
    // yoksa buradaki aynı sözleşme metni (shell `docs/design/clock-locale.md`).
    s!("bar.widget.clock.format", Bar, "Seçili widget", "Biçim", "Boş bırakılırsa 12/24 saat tercihine uyar; bir biçim kalıbı yazılırsa o kullanılır.", ["saat", "saat biçimi", "tarih", "format", "strftime", "12 saat", "24 saat", "otomatik"], Shell, "bar.widget.clock.format", Control::Text { placeholder: "" }, ui_if SEL_CLOCK),
    s!("bar.widget.tray.icon_size", Bar, "Seçili widget", "İkon boyutu", "", ["tepsi", "sistem tepsisi", "tray", "ikon"], Shell, "bar.widget.tray.icon_size", slider(12.0, 32.0, 1.0, "px"), ui_if SEL_TRAY),
    s!("bar.edge_actions.start.left_click", Bar, "Seçili widget", "Sol tık", "", ["uç", "baştaki", "sol uç", "kenar", "tık", "eylem", "panel aç"], Shell, "bar.edge_actions.start.left_click",
        Control::Choice { options: EDGE_CLICK, style: ChoiceStyle::Dropdown }, if has_start, ui_if SEL_START),
    s!("bar.edge_actions.start.right_click", Bar, "Seçili widget", "Sağ tık", "", ["uç", "baştaki", "sol uç", "kenar", "tık", "eylem"], Shell, "bar.edge_actions.start.right_click",
        Control::Choice { options: EDGE_CLICK, style: ChoiceStyle::Dropdown }, if has_start, ui_if SEL_START),
    s!("bar.edge_actions.start.scroll", Bar, "Seçili widget", "Tekerlek", "Sistem eyleminde widget'ın yanında ikon belirir", ["uç", "baştaki", "sol uç", "kenar", "kaydırma", "scroll", "ses", "parlaklık", "çalışma alanı"], Shell, "bar.edge_actions.start.scroll",
        Control::Choice { options: EDGE_SCROLL, style: ChoiceStyle::Dropdown }, if has_start, ui_if SEL_START),
    s!("bar.edge_actions.end.left_click", Bar, "Seçili widget", "Sol tık", "", ["uç", "sondaki", "sağ uç", "kenar", "tık", "eylem", "panel aç"], Shell, "bar.edge_actions.end.left_click",
        Control::Choice { options: EDGE_CLICK, style: ChoiceStyle::Dropdown }, if has_end, ui_if SEL_END),
    s!("bar.edge_actions.end.right_click", Bar, "Seçili widget", "Sağ tık", "", ["uç", "sondaki", "sağ uç", "kenar", "tık", "eylem"], Shell, "bar.edge_actions.end.right_click",
        Control::Choice { options: EDGE_CLICK, style: ChoiceStyle::Dropdown }, if has_end, ui_if SEL_END),
    s!("bar.edge_actions.end.scroll", Bar, "Seçili widget", "Tekerlek", "Sistem eyleminde widget'ın yanında ikon belirir", ["uç", "sondaki", "sağ uç", "kenar", "kaydırma", "scroll", "ses", "parlaklık", "çalışma alanı"], Shell, "bar.edge_actions.end.scroll",
        Control::Choice { options: EDGE_SCROLL, style: ChoiceStyle::Dropdown }, if has_end, ui_if SEL_END),
    s!("bar.edge_actions.scroll_step", Bar, "Seçili widget", "Tekerlek adımı", "Bir çentikte değişen yüzde", ["adım", "kaydırma adımı", "tekerlek", "ses", "parlaklık"], Shell, "bar.edge_actions.scroll_step", int_slider(1.0, 25.0, "%"), if edge_scroll_level, ui_if SEL_LEVEL),
    s!("bar.sections", Bar, "Widget'lar", "Bar bölümleri", "Widget ekle, çıkar ve sürükleyerek sırala", ["widget", "saat", "tepsi", "çalışma alanı", "workspace", "sıra"], Shell, "bar", custom(CustomId::BarSections)),
    s!("bar.edge", Bar, "Konum ve boyut", "Ekran kenarı", "", ["konum", "üst", "alt", "sol", "sağ", "dikey", "panel"], Shell, "bar.edge", custom(CustomId::BarEdge)),
    s!("bar.size", Bar, "Konum ve boyut", "Kalınlık", "Bar'ın kenarındaki çerçeve de bu kalınlıktadır", ["bar kalınlığı", "yükseklik", "boyut", "kenar"], Shell, "bar.size", slider(16.0, 128.0, 1.0, "px")),
    s!("bar.padding", Bar, "Konum ve boyut", "Uçlardaki boşluk", "", ["uç boşluğu", "padding", "iç boşluk"], Shell, "bar.padding", slider(0.0, 48.0, 1.0, "px")),
    s!("bar.spacing", Bar, "Konum ve boyut", "Widget aralığı", "", ["spacing", "boşluk"], Shell, "bar.spacing", slider(0.0, 48.0, 1.0, "px")),
    s!("keybinds.bar_toggle", Keybinds, "Bar ve paneller", "Barı göster / gizle", "", ["kısayol", "tuş", "bar", "gizle"], Shell, "keybinds.bar_toggle", custom(CustomId::Keybind), if keybinds_synced, mirror [Bar]),
    // Ekran okuyucu ve yalnız klavye: bar'da Tab/ok gezinmesine giriş (shell 713be21, 2026-10-17).
    s!("keybinds.bar_focus", Keybinds, "Bar ve paneller", "Bar'da klavyeyle gezin", "Ekran okuyucu ve klavye için; Escape gezinmeyi bitirir", ["kısayol", "tuş", "bar", "odak", "ekran okuyucu", "orca", "klavye"], Shell, "keybinds.bar_focus", custom(CustomId::Keybind), if keybinds_synced, mirror [Accessibility]),
    s!("keybinds.shot_region", Keybinds, "Ekran görüntüsü", "Bölgenin görüntüsünü al", "", ["kısayol", "tuş", "ekran görüntüsü", "print", "bölge"], Shell, "keybinds.shot_region", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.shot_output", Keybinds, "Ekran görüntüsü", "Tüm ekranın görüntüsünü al", "", ["kısayol", "tuş", "ekran görüntüsü", "print", "monitör"], Shell, "keybinds.shot_output", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.shot_window", Keybinds, "Ekran görüntüsü", "Pencerenin görüntüsünü al", "", ["kısayol", "tuş", "ekran görüntüsü", "print", "pencere"], Shell, "keybinds.shot_window", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.clipboard", Keybinds, "Pano", "Pano geçmişini aç", "", ["kısayol", "tuş", "pano", "kopyala", "yapıştır", "geçmiş"], Shell, "keybinds.clipboard", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.magnifier_in", Keybinds, "Büyüteç", "Yakınlaştır", "", ["kısayol", "tuş", "büyüteç", "yakınlaştır", "zoom", "erişilebilirlik"], Shell, "keybinds.magnifier_in", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.magnifier_out", Keybinds, "Büyüteç", "Uzaklaştır", "", ["kısayol", "tuş", "büyüteç", "uzaklaştır", "zoom", "erişilebilirlik"], Shell, "keybinds.magnifier_out", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.magnifier_reset", Keybinds, "Büyüteç", "Büyütmeyi kapat", "", ["kısayol", "tuş", "büyüteç", "sıfırla", "kapat", "zoom", "erişilebilirlik"], Shell, "keybinds.magnifier_reset", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.emoji", Keybinds, "Emoji", "Emoji seçici", "", ["kısayol", "tuş", "emoji", "ifade", "simge", "başlatıcı"], Shell, "keybinds.emoji", custom(CustomId::Keybind), if keybinds_synced),
    s!("keybinds.osk_toggle", Keybinds, "Ekran klavyesi", "Ekran klavyesini aç / kapat", "", ["kısayol", "tuş", "ekran klavyesi", "sanal klavye", "dokunmatik", "erişilebilirlik"], Shell, "keybinds.osk_toggle", custom(CustomId::Keybind), if keybinds_synced),
    // Temel katman (shell ADR-0006): pencere ve çalışma alanı yönetimi, uygulama açma.
    s!("keybinds.launcher", Keybinds, "Uygulamalar", "Uygulama başlatıcısı", "", ["kısayol", "tuş", "başlatıcı", "super", "uygulama", "arama", "launcher"], Shell, "keybinds.launcher", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.settings", Keybinds, "Uygulamalar", "Ayarlar", "", ["kısayol", "tuş", "ayarlar", "settings", "tercihler"], Shell, "keybinds.settings", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.terminal", Keybinds, "Uygulamalar", "Terminal", "", ["kısayol", "tuş", "terminal", "konsol", "komut satırı"], Shell, "keybinds.terminal", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.file_manager", Keybinds, "Uygulamalar", "Dosya yöneticisi", "", ["kısayol", "tuş", "dosya", "klasör", "dosya yöneticisi", "files"], Shell, "keybinds.file_manager", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.browser", Keybinds, "Uygulamalar", "Web tarayıcısı", "", ["kısayol", "tuş", "tarayıcı", "internet", "web", "browser"], Shell, "keybinds.browser", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.lock", Keybinds, "Oturum", "Ekranı kilitle", "", ["kısayol", "tuş", "kilit", "kilitle", "lock", "ekran"], Shell, "keybinds.lock", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.session_menu", Keybinds, "Oturum", "Oturum menüsü", "", ["kısayol", "tuş", "oturum", "kapat", "yeniden başlat", "çıkış", "ctrl alt delete"], Shell, "keybinds.session_menu", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.close_window", Keybinds, "Pencereler", "Pencereyi kapat", "", ["kısayol", "tuş", "pencere", "kapat", "close"], Shell, "keybinds.close_window", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.close_window_alt", Keybinds, "Pencereler", "Pencereyi kapat (ikinci kısayol)", "", ["kısayol", "tuş", "pencere", "kapat", "alt f4", "close"], Shell, "keybinds.close_window_alt", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.force_kill", Keybinds, "Pencereler", "Yanıt vermeyen pencereyi zorla kapat", "", ["kısayol", "tuş", "pencere", "zorla", "kapat", "öldür", "kill"], Shell, "keybinds.force_kill", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.fullscreen", Keybinds, "Pencereler", "Tam ekran", "", ["kısayol", "tuş", "pencere", "tam ekran", "fullscreen"], Shell, "keybinds.fullscreen", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.maximize", Keybinds, "Pencereler", "Ekranı kapla", "", ["kısayol", "tuş", "pencere", "büyüt", "ekranı kapla", "maximize"], Shell, "keybinds.maximize", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.toggle_floating", Keybinds, "Pencereler", "Yüzen pencere", "", ["kısayol", "tuş", "pencere", "yüzen", "floating", "döşeme"], Shell, "keybinds.toggle_floating", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.pin", Keybinds, "Pencereler", "Pencereyi sabitle", "", ["kısayol", "tuş", "pencere", "sabitle", "her zaman üstte", "pin"], Shell, "keybinds.pin", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.cycle_windows", Keybinds, "Pencereler", "Pencereler arasında geçiş", "", ["kısayol", "tuş", "pencere", "geçiş", "alt tab", "değiştir"], Shell, "keybinds.cycle_windows", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.cycle_windows_back", Keybinds, "Pencereler", "Pencereler arasında geri geçiş", "", ["kısayol", "tuş", "pencere", "geçiş", "alt shift tab"], Shell, "keybinds.cycle_windows_back", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.toggle_split", Keybinds, "Pencereler", "Bölme yönünü çevir", "", ["kısayol", "tuş", "pencere", "bölme", "yan yana", "alt alta", "split"], Shell, "keybinds.toggle_split", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.focus_left", Keybinds, "Odak ve taşıma", "Odak: sol", "", ["kısayol", "tuş", "odak", "sol", "pencere", "focus"], Shell, "keybinds.focus_left", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.focus_right", Keybinds, "Odak ve taşıma", "Odak: sağ", "", ["kısayol", "tuş", "odak", "sağ", "pencere", "focus"], Shell, "keybinds.focus_right", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.focus_up", Keybinds, "Odak ve taşıma", "Odak: yukarı", "", ["kısayol", "tuş", "odak", "yukarı", "pencere", "focus"], Shell, "keybinds.focus_up", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.focus_down", Keybinds, "Odak ve taşıma", "Odak: aşağı", "", ["kısayol", "tuş", "odak", "aşağı", "pencere", "focus"], Shell, "keybinds.focus_down", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_left", Keybinds, "Odak ve taşıma", "Pencereyi sola taşı", "", ["kısayol", "tuş", "taşı", "sol", "pencere", "move"], Shell, "keybinds.move_left", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_right", Keybinds, "Odak ve taşıma", "Pencereyi sağa taşı", "", ["kısayol", "tuş", "taşı", "sağ", "pencere", "move"], Shell, "keybinds.move_right", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_up", Keybinds, "Odak ve taşıma", "Pencereyi yukarı taşı", "", ["kısayol", "tuş", "taşı", "yukarı", "pencere", "move"], Shell, "keybinds.move_up", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_down", Keybinds, "Odak ve taşıma", "Pencereyi aşağı taşı", "", ["kısayol", "tuş", "taşı", "aşağı", "pencere", "move"], Shell, "keybinds.move_down", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.shrink_width", Keybinds, "Boyutlandırma", "Pencereyi daralt", "", ["kısayol", "tuş", "boyut", "daralt", "genişlik", "resize"], Shell, "keybinds.shrink_width", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.grow_width", Keybinds, "Boyutlandırma", "Pencereyi genişlet", "", ["kısayol", "tuş", "boyut", "genişlet", "genişlik", "resize"], Shell, "keybinds.grow_width", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.shrink_height", Keybinds, "Boyutlandırma", "Pencereyi kısalt", "", ["kısayol", "tuş", "boyut", "kısalt", "yükseklik", "resize"], Shell, "keybinds.shrink_height", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.grow_height", Keybinds, "Boyutlandırma", "Pencereyi uzat", "", ["kısayol", "tuş", "boyut", "uzat", "yükseklik", "resize"], Shell, "keybinds.grow_height", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.workspace", Keybinds, "Çalışma alanları", "Çalışma alanına geç (1…0)", "", ["kısayol", "tuş", "çalışma alanı", "masaüstü", "workspace", "rakam"], Shell, "keybinds.workspace", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_to_workspace", Keybinds, "Çalışma alanları", "Pencereyi çalışma alanına taşı (1…0)", "", ["kısayol", "tuş", "çalışma alanı", "taşı", "workspace", "rakam"], Shell, "keybinds.move_to_workspace", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.workspace_next", Keybinds, "Çalışma alanları", "Sonraki çalışma alanı", "", ["kısayol", "tuş", "çalışma alanı", "sonraki", "workspace", "page down"], Shell, "keybinds.workspace_next", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.workspace_prev", Keybinds, "Çalışma alanları", "Önceki çalışma alanı", "", ["kısayol", "tuş", "çalışma alanı", "önceki", "workspace", "page up"], Shell, "keybinds.workspace_prev", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_to_workspace_next", Keybinds, "Çalışma alanları", "Pencereyi sonraki alana taşı", "", ["kısayol", "tuş", "çalışma alanı", "taşı", "sonraki", "workspace"], Shell, "keybinds.move_to_workspace_next", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_to_workspace_prev", Keybinds, "Çalışma alanları", "Pencereyi önceki alana taşı", "", ["kısayol", "tuş", "çalışma alanı", "taşı", "önceki", "workspace"], Shell, "keybinds.move_to_workspace_prev", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.workspace_last", Keybinds, "Çalışma alanları", "Son çalışma alanına dön", "", ["kısayol", "tuş", "çalışma alanı", "son", "önceki", "super tab"], Shell, "keybinds.workspace_last", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.scratchpad", Keybinds, "Çalışma alanları", "Karalama alanını göster/gizle", "", ["kısayol", "tuş", "karalama", "gizli", "scratchpad", "özel alan"], Shell, "keybinds.scratchpad", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.move_to_scratchpad", Keybinds, "Çalışma alanları", "Pencereyi karalama alanına gönder", "", ["kısayol", "tuş", "karalama", "gizli", "scratchpad", "taşı"], Shell, "keybinds.move_to_scratchpad", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.window_drag", Keybinds, "Fare", "Fareyle pencere taşı", "", ["kısayol", "tuş", "fare", "sürükle", "taşı", "pencere"], Shell, "keybinds.window_drag", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.window_resize_drag", Keybinds, "Fare", "Fareyle pencere boyutlandır", "", ["kısayol", "tuş", "fare", "sürükle", "boyut", "pencere"], Shell, "keybinds.window_resize_drag", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.workspace_scroll_next", Keybinds, "Fare", "Tekerlekle sonraki çalışma alanı", "", ["kısayol", "tuş", "fare", "tekerlek", "çalışma alanı", "kaydır"], Shell, "keybinds.workspace_scroll_next", custom(CustomId::Keybind), if base_binds),
    s!("keybinds.workspace_scroll_prev", Keybinds, "Fare", "Tekerlekle önceki çalışma alanı", "", ["kısayol", "tuş", "fare", "tekerlek", "çalışma alanı", "kaydır"], Shell, "keybinds.workspace_scroll_prev", custom(CustomId::Keybind), if base_binds),
    // Çerçeve son kart: bar, çerçevenin kalın kenarıdır (§2.2, karar 1).
    s!("frame.enabled", Bar, "Çerçeve", "Ekran çerçevesi", "Ekranı saran ince kenar; kapalıyken yalnızca bar kalır", ["kenar", "border", "frame"], Shell, "frame.enabled", toggle()),
    s!("frame.thickness", Bar, "Çerçeve", "Kalınlık", "Bar dışındaki kenarlar", ["çerçeve kalınlığı", "kenar", "border"], Shell, "frame.thickness", slider(0.0, 64.0, 1.0, "px"), if frame_on),
    s!("frame.edges", Bar, "Çerçeve", "Kenarlar", "Bar kenarı bar kalınlığıdır; bir kenar ayrı ayarlanıp sonra genele bağlanabilir", ["üst", "alt", "sol", "sağ", "kalınlık", "bar"], Shell, "frame", custom(CustomId::FrameEdges), if frame_on),
    s!("frame.inner_corners", Bar, "Çerçeve", "İç köşeler", "Kavis temadan gelir", ["iç kavis", "köşe", "radius", "yuvarlak", "inner_radius"], Framework, "", custom(CustomId::Link("blob.smoothing"))),

    // ---- Ekranlar ----
    s!("displays", Displays, "Monitörler", "Monitörler", "Yerleşim, çözünürlük, yenileme hızı, ölçek, döndürme, yansıtma", ["ekran", "monitör", "çözünürlük", "yenileme", "hz", "ölçek", "döndür", "yansıt", "vrr", "birincil", "hdmi", "dp"], Shell, "", custom(CustomId::Displays)),
    // ---- Ekranlar › Gece ışığı (shell.toml [night_light]; shell docs/design/night-light.md) ----
    // Shell şeması `night_light`'ı tanımıyorsa (eski shell) kart yok. Başlıklar yerel; seçenek
    // adları ve sınırlar şemadan (`option_labels`, birim "K" / "min").
    s!("night_light.enabled", Displays, "Gece ışığı", "Gece ışığını kullan", "Akşamları ekran renklerini sıcaklaştırır", ["gece", "gece ışığı", "mavi ışık", "sıcaklık", "göz", "night light", "hyprsunset", "renk sıcaklığı"], Shell, "night_light.enabled", toggle(), if night_light_known),
    s!("night_light.temperature", Displays, "Gece ışığı", "Gece sıcaklığı", "Düşük değer daha turuncu", ["gece", "sıcaklık", "kelvin", "renk sıcaklığı", "turuncu"], Shell, "night_light.temperature",
        Control::Slider { min: 1500.0, max: 6000.0, step: 100.0, unit: "K", integer: true, percent: false }, if night_light_on),
    s!("night_light.schedule", Displays, "Gece ışığı", "Zamanlama", "", ["gece", "zamanlama", "gün batımı", "gün doğumu", "saat"], Shell, "night_light.schedule",
        Control::Choice { options: NIGHT_SCHEDULES, style: ChoiceStyle::Dropdown }, if night_light_on),
    // Saat aralığı: TimePicker (12/24 saat `[locale] hour_cycle`'a uyar); dosyaya hep 24 saat
    // "SS:DD" yazılır (shell `parse_hhmm`; inceleme 2026-10-13).
    s!("night_light.from", Displays, "Gece ışığı", "Başlangıç saati", "Bitiş daha erkense gece yarısını aşar", ["gece", "başlangıç", "saat", "akşam"], Shell, "night_light.from",
        custom(CustomId::NightLightTime), if night_light_manual),
    s!("night_light.to", Displays, "Gece ışığı", "Bitiş saati", "", ["gece", "bitiş", "saat", "sabah"], Shell, "night_light.to",
        custom(CustomId::NightLightTime), if night_light_manual),
    s!("night_light.location", Displays, "Gece ışığı", "Konum", "", ["gece", "konum", "enlem", "boylam", "gün batımı", "gün doğumu", "latitude", "longitude"], Shell, "", custom(CustomId::NightLightLocation), if night_light_sun),
    s!("night_light.transition", Displays, "Gece ışığı", "Geçiş süresi", "Renk bu sürede yavaşça değişir", ["gece", "geçiş", "yumuşak", "süre", "dakika"], Shell, "night_light.transition",
        Control::Slider { min: 0.0, max: 120.0, step: 5.0, unit: "min", integer: true, percent: false }, if night_light_scheduled),
    // ---- Pencereler (shell.toml [windows], [compositor]; masaüstü ortaklığı ADR-0003) ----
    // §9: üç kart — Yerleşim, Görünüm (köşe/gölge/hareket tek satır), Gelişmiş (kapalı açılır satır).
    s!("windows.gap_inner", Windows, "Yerleşim", "Pencereler arası boşluk", "Yan yana iki pencere arasında", ["boşluk", "gap", "aralık", "döşeme"], Shell, "windows.gap_inner", int_slider(0.0, 64.0, "px"), if windows_synced),
    s!("windows.gap_outer", Windows, "Yerleşim", "Ekran kenarına boşluk", "Pencerelerle ekran kenarı ve bar arasında", ["boşluk", "gap", "aralık", "kenar"], Shell, "windows.gap_outer", int_slider(0.0, 128.0, "px"), if windows_synced),
    s!("windows.border", Windows, "Yerleşim", "Kenarlık", "Etkin pencerede vurgu renginde; 0 kapatır", ["border", "çerçeve", "çizgi", "etkin pencere", "kenarlık kalınlığı"], Shell, "windows.border", int_slider(0.0, 16.0, "px"), if windows_synced),
    s!("windows.blur", Windows, "Görünüm", "Saydam pencerelerin ardı bulanık", "Buzlu cam görünümü", ["blur", "bulanıklık", "saydam", "cam", "arka plan"], Shell, "windows.blur", toggle(), if look_synced),
    s!("windows.blur_size", Windows, "Görünüm", "Yoğunluk", "Büyük değer daha yumuşak", ["blur", "bulanık", "yarıçap"], Shell, "windows.blur_size", int_slider(1.0, 32.0, "px"), if window_blur_on),
    s!("windows.corners", Windows, "Görünüm", "Köşe, gölge ve hareket", "Bar ve panellerle ortak", ["köşe", "yuvarlak", "radius", "rounding", "gölge", "animasyon", "temayla ortak"], Framework, "",
        custom(CustomId::Links(&[("Görünüm", "shape.radius_large"), ("Hareket", "motion.preset")])), if look_or_motion_synced),
    // Kendi satırları yok: aramada yukarıdaki tek satırı bulurlar (§3.5).
    s!("windows.shadow", Windows, "Görünüm", "Pencere gölgesi", "Bar ve panellerinkiyle aynı", ["gölge", "shadow", "derinlik"], Framework, "", custom(CustomId::Alias("windows.corners")), if look_synced),
    s!("windows.motion", Windows, "Görünüm", "Pencere hareketleri", "Açılıp kapanma ve çalışma alanı geçişleri Hareket'i izler", ["animasyon", "hareket", "geçiş", "workspace"], Framework, "", custom(CustomId::Alias("windows.corners")), if motion_synced),
    s!("compositor.sync_look", Windows, "Gelişmiş", "Pencereler temaya uysun", "Köşe, gölge ve bulanıklık Nuxut'tan gelir", ["tema", "köşe", "gölge", "bulanıklık", "eşitle"], Shell, "compositor.sync_look", toggle(), detail "Masaüstüyle ortaklık"),
    s!("compositor.sync_windows", Windows, "Gelişmiş", "Boşluk ve kenarlığı Nuxut belirlesin", "Kapalıyken masaüstünün kendi ayarı geçerli", ["aralık", "kenarlık", "gap", "eşitle"], Shell, "compositor.sync_windows", toggle(), detail "Masaüstüyle ortaklık"),
    s!("compositor.sync_motion", Windows, "Gelişmiş", "Pencere hareketleri temaya uysun", "Kapalıyken masaüstünün kendi animasyonları geçerli", ["animasyon", "hareket", "eşitle"], Shell, "compositor.sync_motion", toggle(), detail "Masaüstüyle ortaklık"),
    s!("compositor.sync_input", Windows, "Gelişmiş", "Klavye ve fareyi Nuxut belirlesin", "Kapalıyken masaüstünün kendi ayarı geçerli", ["klavye", "fare", "dokunmatik", "girdi", "input", "eşitle"], Shell, "compositor.sync_input", toggle(), detail "Masaüstüyle ortaklık"),
    s!("compositor.ensure_source", Windows, "Gelişmiş", "Masaüstü ayarlarına Nuxut'u ekle", "Ayar dosyasına tek satır eklenir, önce yedek alınır", ["hyprland", "source", "hyprland.conf", "hyprland.lua", "dahil et", "bağlan"], Shell, "compositor.ensure_source", toggle(), detail "Masaüstüyle ortaklık"),
    s!("compositor.source_note", Windows, "Gelişmiş", "Bağlantı satırı", "", ["source", "bağlantı", "kaldırıldı"], Shell, "", custom(CustomId::SourceNote), if ensure_source_on, detail "Masaüstüyle ortaklık"),

    // ---- Duvar kağıdı (§9: hero; Yerleşim; Monitör başına; renkler Görünüm'de — karar 5) ----
    s!("wallpaper", Wallpaper, "", "Duvar kağıdı", "Görsel ya da hareketli GIF/WebP/APNG", ["arka plan", "resim", "görsel", "gif", "webp"], Shell, "wallpaper", custom(CustomId::Wallpaper)),
    // Yerleşik Nuxut duvar kağıtları (sözleşme wallpaper-builtins-v1): hero'nun altında kart ızgarası (önce seçili olan görünür).
    s!("wallpaper.builtin", Wallpaper, "Nuxut duvar kağıtları", "Nuxut duvar kağıtları", "Koyu ve açık kipe kendiliğinden uyar; renkler de buradan gelir", ["nuxut", "yerleşik", "hazır", "varsayılan", "duvar kağıdı", "eş uçuş", "kanat ucu", "tırmanış", "göç", "aurora", "büyük silüet", "bulut molası", "uzak süzülüş", "kovalamaca"], Shell, "wallpaper.path", custom(CustomId::WallpaperBuiltins)),
    s!("wallpaper.fit", Wallpaper, "Yerleşim", "Yerleşim", "Görsel ekrana nasıl sığsın", ["sığdır", "doldur", "kırp", "ölçek"], Shell, "wallpaper.fit",
        Control::Choice { options: WALLPAPER_FITS, style: ChoiceStyle::Segmented }, if wallpaper_fit_matters),
    s!("wallpaper.backdrop", Wallpaper, "Yerleşim", "Kenarlardaki boşluk", "Görsel ekranı kaplamadığında", ["kenar boşluğu zemini", "letterbox", "kenar", "bant", "siyah", "zemin", "bulanık"], Shell, "wallpaper.backdrop",
        Control::Choice { options: WALLPAPER_BACKDROPS, style: ChoiceStyle::Segmented }, if wallpaper_letterbox),
    s!("wallpaper.backdrop_blur", Wallpaper, "Yerleşim", "Bulanıklık", "Kenarlardaki kopyanın yumuşaklığı", ["zemin bulanıklığı", "bulanık", "blur", "yarıçap", "kenar"], Shell, "wallpaper.backdrop_blur",
        int_slider(WALLPAPER_BACKDROP_BLUR.0, WALLPAPER_BACKDROP_BLUR.1, "px"), if wallpaper_backdrop_blur),
    s!("wallpaper.backdrop_color", Wallpaper, "Yerleşim", "Zemin rengi", "Temayla birlikte değişir", ["renk", "kenar", "zemin", "yüzey"], Shell, "wallpaper.backdrop_color",
        Control::Choice { options: WALLPAPER_BACKDROP_COLORS, style: ChoiceStyle::Segmented }, if wallpaper_letterbox),
    s!("wallpaper.outputs", Wallpaper, "Monitör başına", "Monitör başına", "Bir monitöre ayrı görsel ya da yerleşim", ["monitör", "ekran", "çoklu", "bağlayıcı", "hdmi", "dp"], Shell, "wallpaper.outputs", custom(CustomId::WallpaperOutputs)),
    // Yönlendirme (§3.5): renk kaynağı Görünüm › Renkler'de yaşar; burada durumu ve oraya giden düğme.
    s!("palette.source", Wallpaper, "Renkler", "Renkleri duvar kağıdından al", "Şema ve kontrast Görünüm'de", ["matugen", "material you", "otomatik renk", "tema", "renk şeması"], Framework, "colors.source", custom(CustomId::WallpaperColors)),

    // ---- Varsayılan uygulamalar (sözleşme defaults-v1) ----
    // Değer shell'de (`mimeapps.list`; `msg defaults status/set`), satırlar shell durumu okununca
    // görünür; okunamazsa arama sayfanın notuna gider. Ortak anahtar sözcükler: varsayılan, aç.
    s!("defaults.note", DefaultApps, "Durum", "Varsayılan uygulamaların durumu", "", ["varsayılan uygulama", "default app", "kabuk", "masaüstü"], Shell, "", custom(CustomId::DefaultAppsNote)),
    s!("defaults.browser", DefaultApps, "İnternet", "Tarayıcı", "Web sayfaları ve bağlantılar", ["tarayıcı", "browser", "web", "internet", "bağlantı", "link", "http", "html", "firefox", "chrome", "chromium", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Browser)), if defaults_ready),
    s!("defaults.email", DefaultApps, "İnternet", "E-posta", "E-posta bağlantıları (mailto)", ["e-posta", "eposta", "posta", "mail", "email", "mailto", "thunderbird", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Email)), if defaults_ready),
    s!("defaults.file_manager", DefaultApps, "Sistem", "Dosya yöneticisi", "Klasörleri açar", ["dosya yöneticisi", "dosyalar", "klasör", "dizin", "file manager", "folder", "nautilus", "thunar", "dolphin", "nemo", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::FileManager)), if defaults_ready),
    s!("defaults.terminal", DefaultApps, "Sistem", "Terminal", "Terminalde açılan uygulamalar için", ["terminal", "konsol", "console", "komut satırı", "xdg-terminal-exec", "kitty", "foot", "alacritty", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Terminal)), if defaults_ready),
    s!("defaults.text_editor", DefaultApps, "Sistem", "Metin düzenleyici", "Düz metin dosyaları", ["metin", "düzenleyici", "editör", "editor", "text editor", "txt", "not defteri", "gedit", "kate", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::TextEditor)), if defaults_ready),
    s!("defaults.image", DefaultApps, "Medya ve belgeler", "Resim görüntüleyici", "PNG, JPEG ve diğer resimler", ["resim", "görüntü", "fotoğraf", "image", "photo", "png", "jpg", "jpeg", "görüntüleyici", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Image)), if defaults_ready),
    s!("defaults.video", DefaultApps, "Medya ve belgeler", "Video oynatıcı", "MP4, MKV, WebM ve diğer videolar", ["video", "film", "oynatıcı", "player", "mp4", "mkv", "webm", "mpv", "vlc", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Video)), if defaults_ready),
    s!("defaults.music", DefaultApps, "Medya ve belgeler", "Müzik çalar", "MP3, FLAC, OGG ve diğer ses dosyaları", ["müzik", "ses dosyası", "şarkı", "music", "audio", "mp3", "flac", "çalar", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Music)), if defaults_ready),
    s!("defaults.pdf", DefaultApps, "Medya ve belgeler", "PDF görüntüleyici", "PDF belgeleri", ["pdf", "belge", "doküman", "document", "okuyucu", "reader", "evince", "okular", "zathura", "varsayılan", "default app"], Shell, "", custom(CustomId::DefaultApp(DefaultRole::Pdf)), if defaults_ready),
    // Kullanıcı hesabı (t118, docs/design/account.md): sistem ayarı, dosyaya değil AccountsService'e yazılır.
    s!("user.note", User, "", "Hesap durumu", "", ["accountsservice", "hesap hizmeti"], Framework, "", custom(CustomId::UserNote)),
    s!("user.avatar", User, "Profil", "Profil resmi", "", ["avatar", "profil resmi", "fotoğraf", "resim", "face", "kullanıcı adı", "hesap türü", "yönetici"], Framework, "", custom(CustomId::UserAvatar)),
    s!("user.real_name", User, "Profil", "Görünen ad", "", ["ad", "isim", "ad soyad", "gerçek ad", "real name", "full name"], Framework, "", custom(CustomId::UserName)),

    // ---- Bağlantılar (t114, docs/design/network-pages.md): NetworkManager durumu ve eylemleri ----
    // Değer dosyada değil NM'de (`services/network.rs`); eylemler geri al geçmişine girmez.
    // İlk kart başlıksız: sayfa başlığı ve satır zaten "Wi-Fi" (review notu, 2026-10-14).
    s!("wifi.note", Wifi, "", "Ağ durumu", "", ["networkmanager", "ağ hizmeti"], Framework, "", custom(CustomId::NetworkNote)),
    s!("wifi.enabled", Wifi, "", "Wi-Fi", "", ["kablosuz", "wireless", "wlan", "aç", "kapat", "uçak modu"], Framework, "", custom(CustomId::WifiStatus)),
    s!("wifi.current", Wifi, "", "Bağlı ağ", "", ["bağlı", "ssid", "sinyal", "internet", "bağlantıyı kes", "ip adresi", "dns"], Framework, "", custom(CustomId::NetList(NetList::Current))),
    s!("wifi.networks", Wifi, "Görünen ağlar", "Görünen ağlar", "", ["ağlar", "tarama", "erişim noktası", "ssid", "sinyal", "bağlan", "şifre"], Framework, "", custom(CustomId::NetList(NetList::Visible))),
    s!("wifi.hidden", Wifi, "Görünen ağlar", "Gizli ağa bağlan…", "", ["gizli ağ", "hidden network", "ssid"], Framework, "", custom(CustomId::NetList(NetList::Hidden))),
    s!("wifi.saved", Wifi, "Kayıtlı ağlar", "Kayıtlı ağlar", "", ["kayıtlı", "bilinen ağlar", "unut", "otomatik bağlan", "ölçülü", "metered"], Framework, "", custom(CustomId::NetList(NetList::Saved))),
    s!("wifi.wired", Wifi, "Kablolu", "Kablolu bağlantı", "", ["ethernet", "kablolu", "lan", "ip adresi", "dns", "hız"], Framework, "", custom(CustomId::NetList(NetList::Wired))),
    s!("vpn.note", Vpn, "", "VPN durumu", "", ["networkmanager", "ağ hizmeti"], Framework, "", custom(CustomId::NetworkNote)),
    s!("vpn.connections", Vpn, "VPN bağlantıları", "VPN bağlantıları", "", ["vpn", "wireguard", "openvpn", "tünel", "sil"], Framework, "", custom(CustomId::NetList(NetList::Vpn))),
    s!("vpn.import", Vpn, "VPN bağlantıları", "WireGuard içe aktar…", "", ["içe aktar", "import", "wireguard", "conf", "dosya"], Framework, "", custom(CustomId::NetList(NetList::Import))),
    // ---- Bağlantılar › Bluetooth (t115, docs/design/bluetooth-page.md): BlueZ, ajansız ----
    // Değer dosyada değil BlueZ'de (`services/bluetooth.rs`); eşleştirme onayı kabuğun ajanında.
    s!("bluetooth.status", Bluetooth, "", "Bluetooth durumu", "", ["bluez", "adaptör", "bluetooth hizmeti"], Framework, "", custom(CustomId::BluetoothStatus)),
    s!("bluetooth.enabled", Bluetooth, "Bluetooth", "Bluetooth", "", ["bluetooth", "aç", "kapat", "uçak modu", "rfkill"], Framework, "", custom(CustomId::BluetoothPower)),
    s!("bluetooth.name", Bluetooth, "Bluetooth", "Bu bilgisayarın adı", "", ["ad", "isim", "adaptör adı", "cihaz adı"], Framework, "", custom(CustomId::BluetoothName)),
    s!("bluetooth.discoverable", Bluetooth, "Bluetooth", "Görünür", "", ["görünürlük", "keşfedilebilir", "discoverable", "bulunabilir"], Framework, "", custom(CustomId::BluetoothVisible)),
    s!("bluetooth.paired", Bluetooth, "Eşleşmiş cihazlar", "Eşleşmiş cihazlar", "", ["cihaz", "kulaklık", "klavye", "fare", "bağlan", "unut", "güvenilir", "engelle", "pil"], Framework, "", custom(CustomId::BluetoothDevices(BtList::Paired))),
    s!("bluetooth.nearby", Bluetooth, "Yakındaki cihazlar", "Yakındaki cihazlar", "", ["tara", "eşleştir", "yeni cihaz", "pair", "ara"], Framework, "", custom(CustomId::BluetoothDevices(BtList::Nearby))),

    // ---- Sistem › Güç (t117, docs/design/power-page.md): power-profiles-daemon + UPower ----
    // Değer dosyada değil sistem hizmetlerinde (`services/power.rs`, `nuxut-power`); veri yoksa satırlar
    // gizli, sayfada boş durum (power.status). Kart sırası: Pil, Güç profili.
    s!("power.status", Power, "", "Güç bilgisi", "", ["güç", "upower", "power-profiles-daemon"], Framework, "", custom(CustomId::PowerStatus), if |_| crate::services::power::page_listed()),
    s!("power.battery", Power, "Pil", "Pil durumu", "", ["pil", "şarj", "yüzde", "kalan süre", "battery"], Framework, "", custom(CustomId::BatteryStatus), if |_| crate::services::power::battery_listed()),
    s!("power.charge_limit", Power, "Pil", "Pil ömrünü koru", "", ["şarj", "şarj sınırı", "pil sağlığı", "pil", "battery", "charge limit"], Framework, "", custom(CustomId::ChargeLimit), if |_| crate::services::power::charge_listed()),
    s!("power.profile", Power, "Güç profili", "Güç profili", "", ["güç", "profil", "performans", "güç tasarrufu", "dengeli", "power-profiles-daemon", "pil"], Framework, "", custom(CustomId::PowerProfile), if |_| crate::services::power::profiles_listed()),
    // Kilit (shell.toml [idle] lock_before_sleep, [lock] listen_logind; shell lock-trigger.md) ve
    // Boşta kalınca ([idle]; shell services/idle). Boşta ana anahtarı kapalıyken boşta satırları
    // gizli; uykudan önce kilit görünür kalır, çalışmadığını uyarısı söyler (`note_in`). Süreler
    // saniye; yerel aralık shell STAGE_SECS (30..=14400) içinde, 1 dk adımlı.
    s!("idle.lock_before_sleep", Power, "Kilit", "Uykudan önce kilitle", "Bilgisayar uyandığında masaüstü kilitli gelir", ["kilit", "uyku", "askıya alma", "kapak", "suspend", "lock", "güvenlik"], Shell, "idle.lock_before_sleep", toggle(), if idle_lock_before_sleep_known),
    s!("lock.listen_logind", Power, "Kilit", "Nuxut kilit ekranını kullan", "Kilitle düğmesi, kısayolu ve otomatik kilit Nuxut kilit ekranını açar; başka bir kilit aracıyla birlikte kapatılır", ["kilit ekranı", "kilitleyici", "hyprlock", "swaylock", "loginctl", "logind", "lock"], Shell, "lock.listen_logind", toggle(), if lock_listen_known),
    s!("idle.enabled", Power, "Boşta kalınca", "Boşta eylemlerini Nuxut yönetsin", "Kapalıyken uykudan önce kilit de çalışmaz; başka bir araç yönetebilir", ["boşta", "kilit", "hypridle", "idle", "uyku", "otomatik"], Shell, "idle.enabled", toggle(), if idle_known),
    s!("idle.lock", Power, "Boşta kalınca", "Boşta kilitle", "Belirlenen süre boşta kalınca oturum kilitlenir", ["kilit", "boşta", "otomatik kilit", "ekran kilidi", "lock"], Shell, "idle.lock", toggle(), if idle_on),
    s!("idle.lock_after", Power, "Boşta kalınca", "Kilitleme süresi", "", ["kilit", "süre", "boşta"], Shell, "idle.lock_after",
        Control::Slider { min: 60.0, max: 3600.0, step: 60.0, unit: "s", integer: true, percent: false }, if |s| idle_stage_on(s, "idle.lock")),
    s!("idle.dim", Power, "Boşta kalınca", "Ekranı karart", "", ["karart", "boşta", "ekran", "parlaklık", "dim"], Shell, "idle.dim", toggle(), if idle_on),
    s!("idle.dim_after", Power, "Boşta kalınca", "Karartma süresi", "", ["karart", "süre", "boşta"], Shell, "idle.dim_after",
        Control::Slider { min: 60.0, max: 3600.0, step: 60.0, unit: "s", integer: true, percent: false }, if |s| idle_stage_on(s, "idle.dim")),
    s!("idle.screen_off", Power, "Boşta kalınca", "Ekranı kapat", "", ["ekranı kapat", "boşta", "monitör", "dpms", "ekran"], Shell, "idle.screen_off", toggle(), if idle_on),
    s!("idle.screen_off_after", Power, "Boşta kalınca", "Ekranı kapatma süresi", "", ["ekranı kapat", "süre", "boşta"], Shell, "idle.screen_off_after",
        Control::Slider { min: 60.0, max: 3600.0, step: 60.0, unit: "s", integer: true, percent: false }, if |s| idle_stage_on(s, "idle.screen_off")),
    s!("idle.suspend", Power, "Boşta kalınca", "Uyu", "Varsayılan olarak yalnızca pildeyken", ["uyku", "askıya alma", "boşta", "suspend", "otomatik uyku"], Shell, "idle.suspend", toggle(), if idle_on),
    s!("idle.suspend_after", Power, "Boşta kalınca", "Uyku süresi", "", ["uyku", "süre", "boşta"], Shell, "idle.suspend_after",
        Control::Slider { min: 300.0, max: 7200.0, step: 300.0, unit: "s", integer: true, percent: false }, if |s| idle_stage_on(s, "idle.suspend")),
    s!("idle.suspend_on_ac", Power, "Boşta kalınca", "Prizdeyken de uyu", "", ["uyku", "priz", "şarj", "ac"], Shell, "idle.suspend_on_ac", toggle(), if |s| idle_stage_on(s, "idle.suspend")),
    s!("idle.media_inhibits", Power, "Boşta kalınca", "Medya oynarken bekle", "Müzik ya da video çalarken ekran kararmaz, kilitlenmez", ["medya", "müzik", "video", "film", "boşta", "engelle"], Shell, "idle.media_inhibits", toggle(), if idle_on),
    // ---- Sistem › Yazıcılar (t125, docs/design/printers.md): CUPS (IPP) ----
    s!("printers.status", Printers, "", "Yazdırma servisi", "", ["cups", "yazdırma servisi", "yazdırma hizmeti"], Framework, "", custom(CustomId::PrintersStatus)),
    s!("printers.list", Printers, "Yazıcılar", "Yazıcılar", "", ["yazıcı", "varsayılan yazıcı", "deneme sayfası", "test sayfası", "printer"], Framework, "", custom(CustomId::PrinterList)),
    s!("printers.add", Printers, "Yazıcılar", "Yazıcı ekle", "Yazıcı ekleme ve gelişmiş ayarlar CUPS web arayüzünde", ["yazıcı ekle", "yeni yazıcı", "cups web", "yönetim"], Framework, "", custom(CustomId::PrintersAdmin)),
    s!("printers.jobs", Printers, "Yazdırma kuyruğu", "Yazdırma kuyruğu", "", ["kuyruk", "iş", "yazdırma işi", "iptal", "bekleyen"], Framework, "", custom(CustomId::PrintJobs)),

    // ---- Hesaplar ----
    s!("accounts", Accounts, "Takvim hesapları", "Takvim hesapları", "Google, Microsoft, Nextcloud, CalDAV, ICS", ["takvim", "google", "outlook", "nextcloud", "icloud", "caldav"], Shell, "", custom(CustomId::Accounts)),

    // ---- Bildirimler (§9: Genel, Davranış, Uygulamalar) ----
    s!("notifications.enabled", Notifications, "Genel", "Bildirimleri Nuxut göstersin", "Kapalıyken başka bir uygulama gösterebilir", ["bildirimler", "bildirim sunucusu"], Shell, "notifications.enabled", toggle()),
    s!("notifications.dnd", Notifications, "Genel", "Rahatsız etme", "Açılır bildirim yok; hepsi merkezde birikir", ["dnd", "sessiz", "sustur"], Shell, "notifications.dnd", toggle()),
    s!("notifications.popup_timeout_ms", Notifications, "Davranış", "Görünme süresi", "Açılır bildirimin ekranda kalma süresi", ["popup", "süre", "zaman aşımı", "açılır bildirimler"], Shell, "notifications.popup_timeout_ms", Control::Slider { min: 1000.0, max: 20000.0, step: 500.0, unit: "ms", integer: true, percent: false }),
    s!("notifications.max_popups", Notifications, "Davranış", "Aynı anda en çok", "Üst üste görünen açılır bildirim sayısı", ["popup", "sayı", "yığın"], Shell, "notifications.max_popups", int_slider(1.0, 10.0, "")),
    s!("notifications.history_limit", Notifications, "Davranış", "Saklanan bildirim", "Bildirim merkezinde tutulan en çok bildirim", ["geçmiş", "merkez", "history"], Shell, "notifications.history_limit", Control::Slider { min: 10.0, max: 500.0, step: 10.0, unit: "", integer: true, percent: false }),
    s!("notifications.apps", Notifications, "Uygulamalar", "Uygulama başına", "Bir uygulamanın açılır bildirimlerini ya da geçmişini kapat", ["popup", "uygulama"], Shell, "notifications.apps", custom(CustomId::NotificationApps)),

    // ---- Ses (§9: Ses düzeyi kartı — sayfa adını tekrarlamaz 2026-10-15 — en yüksek ses + uyarı + gösterge; Medya tuşları) ----
    s!("audio.max_volume", Sound, "Ses düzeyi", "En yüksek ses", "Kısayol ve kaydırıcıların üst sınırı", ["volume", "yükseltme", "boost", "amplify"], Shell, "audio.max_volume", Control::Slider { min: 100.0, max: 150.0, step: 5.0, unit: "%", integer: true, percent: false }),
    s!("audio.max_volume.warning", Sound, "Ses düzeyi", "Uyarı", "100 üstü yükseltme sesi bozabilir ve hoparlöre zarar verebilir", ["volume"], Shell, "", custom(CustomId::Notice), if volume_boosted),
    s!("osd.timeout_ms", Sound, "Ses düzeyi", "Ses ve parlaklık göstergesi", "Ekranda kalma süresi", ["osd", "ekran göstergesi", "gösterge", "süre", "görünme süresi"], Shell, "osd.timeout_ms", Control::Slider { min: 500.0, max: 5000.0, step: 100.0, unit: "ms", integer: true, percent: false }),
    s!("media_keys.link", Sound, "Medya tuşları", "Ses ve medya tuşları", "Klavyedeki ses, parlaklık ve oynatma tuşları", ["xf86", "kısayol", "tuş", "bind", "parlaklık", "mikrofon"], Shell, "", custom(CustomId::Link("keybinds.volume_up")), if keybinds_synced),
    s!("media_keys", Sound, "Medya tuşları", "Masaüstü ayar dosyası için örnek", "Nuxut kısayolları kapalı; tuşlar elle bağlanabilir", ["hyprland bind örneği", "xf86", "kısayol", "bind", "playerctl", "parlaklık", "brightness", "mikrofon"], Shell, "", custom(CustomId::MediaKeys), if keybinds_unsynced),

    // ---- Hakkında ----
    s!("about", About, "", "Hakkında", "Sürüm, config dosyaları", ["sürüm", "versiyon", "lisans"], Shell, "", custom(CustomId::About)),

    // ---- Bölge ve dil (ADR-0010 §5 adım 3; karar 9: ayrı sayfa, Sistem grubu; ileride klavye düzeni ve saat dilimi) ----
    // Hero: seçili biçimin canlı önizlemesi (yazılı ayar değil; başlıksız kart).
    s!("locale.preview", Region, "", "Biçim önizlemesi", "", ["önizleme", "örnek", "tarih", "saat", "sayı", "yüzde"], Framework, "", custom(CustomId::LocalePreview)),
    s!("locale.language", Region, "Dil ve biçimler", "Dil", "Menüler ve metinler bu dilde gösterilir",
        ["dil", "language", "lisan", "çeviri", "türkçe", "english", "locale"], Framework, "locale.language", custom(CustomId::Locale(LocaleField::Language))),
    s!("locale.formats", Region, "Dil ve biçimler", "Biçimler", "Tarih, saat ve sayıların yazılışı",
        ["tarih", "saat", "sayı", "ondalık", "bölge", "region", "format", "locale"], Framework, "locale.formats", custom(CustomId::Locale(LocaleField::Formats))),
    // Tarih ve saat (t119, docs/design/datetime.md): sistem ayarı, dosyaya değil timedated'e yazılır.
    s!("datetime.note", DateTime, "Saat", "Tarih ve saat durumu", "", ["timedated", "rtc", "donanım saati"], Framework, "", custom(CustomId::TimedateNote)),
    s!("datetime.ntp", DateTime, "Saat", "Otomatik saat", "", ["ntp", "ağ saati", "eşitle", "senkron", "internet saati", "automatic time"], Framework, "", custom(CustomId::AutoTime)),
    s!("datetime.timezone", DateTime, "Saat", "Saat dilimi", "", ["saat dilimi", "zaman dilimi", "timezone", "utc", "gmt", "şehir"], Framework, "", custom(CustomId::Timezone)),
    s!("datetime.manual", DateTime, "Saat", "Elle tarih ve saat", "", ["tarih", "saat", "elle", "manual", "ayarla"], Framework, "", custom(CustomId::ManualTime)),
    s!("locale.hour_cycle", DateTime, "Biçim", "Saat biçimi", "Saatin 24 ya da 12 saatlik gösterimi",
        ["24 saat", "12 saat", "öö", "ös", "am", "pm", "saat biçimi", "hour"], Framework, "locale.hour_cycle", Control::Choice { options: HOUR_CYCLES, style: ChoiceStyle::Segmented }, if hour_cycle_supported),
    s!("datetime.clock_format_note", DateTime, "Biçim", "Bar saati biçimi", "Bar saati özel biçim kullanıyor",
        ["saat biçimi", "bar saati", "özel biçim", "temizle"], Shell, "", custom(CustomId::ClockFormatNote), if clock_format_overrides),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    /// Girdi-v1: satır değeri dosya > çalışan masaüstündeki etkin değer > şema varsayılanı.
    /// Yazılmamış satır "değiştirilmemiş" (↺ yok); yazılmış her değer (varsayılana eşit de)
    /// dosyada kalır; ↺ anahtarı siler.
    #[test]
    fn input_value_prefers_file_then_effective_then_schema() {
        let (_d, mut store) = temp_store();
        let rate = by_id("input.keyboard.repeat_rate").unwrap();
        assert!(rate.is_live());
        let none = |_: &str| None;
        let eff = |p: &str| (p == "input.keyboard.repeat_rate").then(|| Value::from(70));
        assert_eq!(number(rate.value_with(&store, none).as_deref()), Some(25.0), "durum yok: şema varsayılanı");
        assert_eq!(number(rate.value_with(&store, eff).as_deref()), Some(70.0), "etkin değer (Hyprland varsayılanı değil)");
        assert!(!rate.is_set(&store) && !rate.differs_from_default(&store), "yazılmamış = değiştirilmemiş");
        // Varsayılana eşit değer de yazılır ve "değiştirilmiş" sayılır.
        let v = rate.stored(rate.number_value(25.0), &store).expect("yazılmalı");
        store.set(FileId::Shell, &rate.key(), Some(v));
        assert_eq!(number(rate.value_with(&store, eff).as_deref()), Some(25.0), "dosya önce gelir");
        assert!(rate.differs_from_default(&store));
        // ↺ = anahtarı sil: etkin değere döner.
        store.set(FileId::Shell, &rate.key(), None);
        assert_eq!(number(rate.value_with(&store, eff).as_deref()), Some(70.0));
        // Seçimler de varsayılanı yazar (Uyarlanır seçmek Hyprland'in "flat"ını ezer).
        let accel = by_id("input.mouse.accel_profile").unwrap();
        assert_eq!(accel.choice_stored("adaptive", &store).and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some("adaptive"));
        // Etkin değerli olmayan satır etkin kaynağa bakmaz.
        let gap = by_id("windows.gap_inner").unwrap();
        assert!(!gap.is_live());
        assert_eq!(number(gap.value_with(&store, |_| Some(Value::from(99))).as_deref()), Some(4.0));
    }

    /// Girdi eşitlemesi kapalıyken satırlar gizli; aramada sonuç sayfanın notuna (neden + "Aç").
    #[test]
    fn hidden_input_rows_lead_to_page_note() {
        let (_d, mut store) = temp_store();
        let hit = |store: &Store, q: &str| search(store, q).iter().map(|d| d.id).collect::<Vec<_>>();
        assert!(hit(&store, "tekrar gecikmesi").contains(&"input.keyboard.repeat_delay"));
        assert!(hit(&store, "dokunarak").contains(&"input.touchpad.tap_to_click"));
        store.set(FileId::Shell, &KeyPath::parse("compositor.sync_input"), Some(Value::from(false)));
        assert_eq!(hit(&store, "tekrar gecikmesi"), vec!["keyboard.note"]);
        assert_eq!(hit(&store, "dokunarak"), vec!["pointer.note"]);
        // Arama terimleri.
        store.set(FileId::Shell, &KeyPath::parse("compositor.sync_input"), None);
        for (q, id) in [
            ("xkb", "input.keyboard.layouts"),
            ("layout", "input.keyboard.layouts"),
            ("numlock", "input.keyboard.numlock_by_default"),
            ("touchpad", "input.touchpad.tap_to_click"),
            ("natural scroll", "input.mouse.natural_scroll"),
            ("tap", "input.touchpad.tap_to_click"),
            ("fare", "input.mouse.sensitivity"),
            ("klavye", "input.keyboard.repeat_rate"),
        ] {
            assert!(hit(&store, q).contains(&id), "{q} → {id}");
        }
        // Geçiş seçimi yalnızca birden çok düzen varken.
        assert!(!by_id("input.keyboard.switch").unwrap().is_visible(&store));
        store.set(FileId::Shell, &KeyPath::parse("input.keyboard.layouts"), Some(crate::pages::keyboard::layouts_value(&[
            crate::services::input::KbLayout::new("tr", ""),
            crate::services::input::KbLayout::new("us", ""),
        ])));
        assert!(by_id("input.keyboard.switch").unwrap().is_visible(&store));
    }

    /// Framework doğrulama sınırları (nuxut-theme `from_config`): UI aralığı bunların içinde.
    fn framework_limits(path: &str) -> Option<(f64, f64)> {
        Some(match path {
            "shape.radius_small" | "shape.radius_medium" | "shape.radius_large" => (0.0, 256.0),
            "shadow.opacity" => (0.0, 1.0),
            "shadow.blur" => (0.0, 128.0),
            "shadow.offset_y" => (-64.0, 64.0),
            "motion.speed" => (0.05, 20.0),
            "blob.deform" => (0.0, 2.0),
            "blob.smoothing" => (0.0, 128.0),
            "typography.size" => (4.0, 128.0),
            // shell `services/look/selection.rs` MIN/MAX_CURSOR_SIZE (ADR-0004).
            "desktop.cursor_size" => (16.0, 96.0),
            p if p.starts_with("typography.") && p.ends_with(".weight") => (100.0, 1000.0),
            p if p.starts_with("typography.") && p.ends_with(".scale") => (0.5, 4.0),
            _ => return None,
        })
    }

    /// Shell doğrulama sınırlarının çevrimdışı yedeği (shell `docs/ARCHITECTURE.md`); gerçek şema
    /// okunabiliyorsa `local_fallbacks_match_real_schema` bu tablonun şemayla aynı olduğunu sınar.
    fn shell_limits(path: &str) -> Option<(f64, f64)> {
        Some(match path {
            "frame.thickness" => (0.0, 64.0),
            "bar.size" => (16.0, 128.0),
            "bar.padding" | "bar.spacing" => (0.0, 64.0),
            "bar.widget.workspaces.count" => (1.0, 30.0),
            "bar.edge_actions.scroll_step" => (1.0, 25.0),
            p if p.starts_with("panels.") && p.ends_with(".size") => (160.0, 900.0),
            "notifications.popup_timeout_ms" => (1000.0, 60_000.0),
            "notifications.max_popups" => (1.0, 10.0),
            "notifications.history_limit" => (0.0, 500.0),
            "bar.widget.tray.icon_size" => (10.0, 64.0),
            "bar.widget.active_window.max_width" => (40.0, 1200.0),
            "audio.max_volume" => (100.0, 150.0),
            "osd.timeout_ms" => (500.0, 10_000.0),
            // shell `config.rs` WALLPAPER_BACKDROP_BLUR (88885da).
            "wallpaper.backdrop_blur" => (4.0, 128.0),
            // shell `hypr_config/settings.rs` GAP_INNER/GAP_OUTER/BORDER/BLUR_SIZE.
            "windows.gap_inner" => (0.0, 64.0),
            "windows.gap_outer" => (0.0, 128.0),
            "windows.border" => (0.0, 16.0),
            "windows.blur_size" => (1.0, 32.0),
            // shell `services/night_light/config.rs` TEMPERATURE/TRANSITION_MINUTES.
            "night_light.temperature" => (1500.0, 6000.0),
            "night_light.transition" => (0.0, 120.0),
            // shell `services/idle/config.rs` STAGE_SECS (lock_after 0 = boşta kilitleme yok).
            "idle.dim_after" | "idle.screen_off_after" | "idle.suspend_after" => (30.0, 14_400.0),
            "idle.lock_after" => (0.0, 14_400.0),
            _ => return None,
        })
    }

    #[test]
    fn sliders_are_within_validation_limits() {
        for s in SETTINGS {
            if let Control::Slider { min, max, step, .. } = s.control {
                assert!(min < max && step > 0.0, "{}", s.id);
                if let Some(d) = number(s.default_for(Default::default())) {
                    assert!((min..=max).contains(&d), "{}: varsayılan {d} aralık dışı", s.id);
                }
                let limits = match s.file {
                    FileId::Framework => framework_limits(s.path),
                    FileId::Shell => shell_limits(s.path),
                    FileId::Palette => Some((-1.0, 1.0)),
                };
                if let Some((lo, hi)) = limits {
                    assert!(min >= lo && max <= hi, "{}: [{min}, {max}] ⊄ [{lo}, {hi}]", s.id);
                }
            }
        }
    }

    #[test]
    fn choosing_default_removes_key() {
        let store = Store::open("/nonexistent/f.toml", "/nonexistent/s.toml", "/nonexistent/p.toml");
        let fit = by_id("wallpaper.fit").expect("wallpaper.fit");
        assert!(fit.choice_value(0, &store).is_none(), "cover varsayılan: anahtar yazılmaz");
        assert_eq!(fit.choice_value(1, &store).and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some("contain"));
    }

    #[test]
    fn writing_default_value_removes_key() {
        let store = Store::open("/nonexistent/f.toml", "/nonexistent/s.toml", "/nonexistent/p.toml");
        for s in SETTINGS {
            if let Some(d) = s.default_value(&store) {
                if s.is_live() {
                    // Girdi-v1 "içe aktar, ezme": varsayılana eşit değer de yazılır (yazılmazsa
                    // kullanıcının Hyprland değeri geçerli kalırdı).
                    assert!(s.stored(d.clone(), &store).is_some(), "{}: etkin değerli satır varsayılanı da yazmalı", s.id);
                    continue;
                }
                assert!(s.stored(d.clone(), &store).is_none(), "{}: varsayılan yazılmamalı", s.id);
            }
        }
        let speed = by_id("motion.speed").expect("motion.speed");
        assert!(speed.stored(Value::from(1), &store).is_none(), "1 ile 1.0 aynı");
        assert!(speed.stored(speed.number_value(1.5), &store).is_some());
        let reduce = by_id("motion.enabled").expect("motion.enabled");
        assert!(reduce.stored(reduce.toggle_value(false).unwrap(), &store).is_none(), "ters anahtar: kapalı = enabled varsayılanı");
        assert!(reduce.stored(reduce.toggle_value(true).unwrap(), &store).is_some());
        let clock = by_id("bar.widget.clock.format").expect("clock.format");
        assert!(clock.stored(Value::from("%a %H:%M"), &store).is_some());
        let color = SETTINGS.iter().find(|s| matches!(s.control, Control::Color) && s.default_value(&store).is_some()).expect("renk ayarı");
        let hex = color.default_value(&store).and_then(Value::as_str).unwrap().to_ascii_uppercase();
        assert!(color.stored(Value::from(hex), &store).is_none(), "renk büyük/küçük harf duyarsız");
    }

    #[test]
    fn ids_unique_and_defaults_come_from_owner() {
        let mut ids: Vec<_> = SETTINGS.iter().map(|s| s.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SETTINGS.len());
        for s in SETTINGS {
            match s.control {
                Control::Choice { options, .. } => {
                    // Varsayılanı yoksa ("Genel" = anahtar yok) boş seçenek olmalı.
                    let d = s.default_for(Default::default()).and_then(Value::as_str).unwrap_or("");
                    assert!(options.iter().any(|(v, _)| *v == d), "{}: varsayılan '{d}' seçeneklerde yok", s.id);
                }
                Control::Color => {
                    let d = s.default_for(Default::default()).and_then(Value::as_str);
                    assert!(d.is_some_and(|h| nuxut::Color::from_hex(h).is_ok()), "{}", s.id);
                }
                Control::Toggle { .. } | Control::Slider { .. } | Control::Text { .. } => {
                    assert!(s.default_for(Default::default()).is_some(), "{}: varsayılan bulunamadı", s.id);
                }
                Control::Custom(_) => {}
            }
        }
    }

    #[test]
    fn color_defaults_track_theme() {
        let t = nuxut::Theme::default();
        for (name, c) in crate::defaults::palette_roles(&t.colors) {
            let d = by_id(&format!("colors.{name}")).and_then(|d| d.default_for(Default::default())).and_then(Value::as_str);
            assert_eq!(d, Some(c.to_hex().as_str()), "{name}");
        }
    }

    #[test]
    fn quantize_writes_clean_numbers() {
        assert_eq!(quantize(0.30000000000000004, 0.05), 0.3);
        assert_eq!(quantize(1.2345, 0.05).to_string(), "1.25");
        assert_eq!(quantize(13.26, 0.5), 13.5);
        assert_eq!(quantize(0.424, 0.01), 0.42);
        let d = by_id("bar.widget.workspaces.count").unwrap();
        assert_eq!(d.number_value(7.4).to_string(), "7");
        let d = by_id("motion.speed").unwrap();
        assert_eq!(d.number_value(1.0 / 3.0).to_string(), "0.35");
    }

    #[test]
    fn search_is_turkish_and_accent_insensitive() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let ids = |q: &str| search(&store, q).iter().map(|s| s.id).collect::<Vec<_>>();
        assert!(ids("golge").contains(&"shadow.enabled"));
        assert!(ids("GÖLGE").contains(&"shadow.enabled"));
        assert!(ids("animasyon").contains(&"motion.speed"));
        assert!(ids("kosE").contains(&"shape.radius_small"));
        assert!(ids("").is_empty());
        // Gizli satırlar aranmaz (gölge kapalıyken bulanıklık).
        let mut store = store;
        store.set(FileId::Framework, &KeyPath::parse("shadow.enabled"), Some(Value::from(false)));
        assert!(!ids_with(&store, "bulanık").contains(&"shadow.blur"));
    }

    fn ids_with(store: &Store, q: &str) -> Vec<&'static str> {
        search(store, q).iter().map(|s| s.id).collect()
    }

    #[test]
    fn high_contrast_row_visibility_follows_schema() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let d = by_id("colors.contrast").expect("colors.contrast");
        assert_eq!((d.page, d.section, d.file), (PageId::Appearance, "Renkler", FileId::Framework));
        // Bu framework şeması anahtarı tanıyor (varsayılan "standard") → satır görünür.
        assert_eq!(d.default_value(&store).and_then(Value::as_str), Some("standard"));
        assert!(d.is_visible(&store));
        // Şema tanımıyorsa (eski framework: varsayılan yok) gizli.
        assert!(!contrast_supported_by(None));
        assert!(!contrast_supported_by(Some(&Value::from(true))), "dize olmayan varsayılan tanınmış sayılmaz");
        assert!(contrast_supported_by(Some(&Value::from("standard"))));
        for q in ["karşıtlık", "kontrast", "erişilebilirlik", "görme", "okunabilirlik", "high contrast"] {
            assert!(ids_with(&store, q).contains(&"colors.contrast"), "{q}");
        }
    }

    /// "Fiyu'yu gizle" (kullanıcı kararı 2026-10-13): varsayılan kapalı (maskot
    /// görünür); açınca `[appearance] mascot = false` yazılır, kapatınca anahtar silinir.
    #[test]
    fn hide_mascot_toggle_writes_false_or_removes_key() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let d = by_id("appearance.mascot").unwrap();
        assert_eq!(d.page, PageId::Appearance);
        assert!(d.is_visible(&store), "framework şeması anahtarı tanıyor");
        assert_eq!(d.default_value(&store).and_then(Value::as_bool), Some(true));
        assert!(!d.bool(&store), "varsayılan: gizle kapalı");
        let w = d.toggle_writes(true, &store);
        assert_eq!(w.len(), 1);
        assert_eq!((w[0].0, &w[0].1), (FileId::Framework, &KeyPath::parse("appearance.mascot")));
        assert_eq!(w[0].2.as_ref().and_then(Value::as_bool), Some(false));
        store.set(FileId::Framework, &d.key(), w[0].2.clone());
        assert!(d.bool(&store) && d.is_set(&store));
        let w = d.toggle_writes(false, &store);
        assert!(w[0].2.is_none(), "kapalı = varsayılan: anahtar silinir");
        store.set(FileId::Framework, &d.key(), None);
        assert!(!d.is_set(&store) && !d.bool(&store));
        // Arama sözcükleri.
        for q in ["maskot", "fiyu", "kuş", "kırlangıç"] {
            assert!(search(&store, q).iter().any(|r| r.id == "appearance.mascot"), "{q}");
        }
        // Hoş geldin adımlarında yok.
        assert!(crate::welcome::STEPS.iter().all(|s| !s.settings.contains(&"appearance.mascot")));
    }

    /// Şeması `[appearance] mascot`'u tanımayan framework'te satır gizli.
    #[test]
    fn hide_mascot_row_needs_framework_schema() {
        assert!(mascot_supported_by(Some(&Value::from(true))));
        assert!(!mascot_supported_by(None));
        assert!(!mascot_supported_by(Some(&Value::from("yes"))));
    }

    #[test]
    fn high_contrast_toggle_writes_high_or_removes_key() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let d = by_id("colors.contrast").unwrap();
        assert!(!d.bool(&store), "varsayılan standard → kapalı");
        // Açık → "high" (varsayılan değil, yazılır).
        let on = d.toggle_value(true).and_then(|v| d.stored(v, &store));
        assert_eq!(on.as_ref().and_then(Value::as_str), Some("high"));
        store.set(FileId::Framework, &d.key(), on);
        assert!(d.bool(&store));
        // Kapalı → anahtar silinir (sahibin varsayılanı "standard").
        let off = d.toggle_value(false);
        assert!(off.is_none());
        store.set(FileId::Framework, &d.key(), off);
        assert!(!d.is_set(&store) && !d.bool(&store));
        // Elle yazılmış "standard" ya da tanınmayan değer kapalı görünür.
        store.set(FileId::Framework, &d.key(), Some(Value::from("standard")));
        assert!(!d.bool(&store));
        store.set(FileId::Framework, &d.key(), Some(Value::from("HIGH")));
        assert!(!d.bool(&store));
    }

    #[test]
    fn reduced_motion_matches_framework_rule() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let set = |store: &mut Store, p: &str, v: Option<Value>| store.set(FileId::Framework, &KeyPath::parse(p), v);
        // Framework'ün kendi çözümüyle karşılaştır: aynı dosya içeriği → aynı karar.
        let framework = |store: &Store| {
            let text = store.file(FileId::Framework).doc().to_string();
            nuxut::Theme::parse(&text).unwrap().0.reduced_motion()
        };
        assert!(!reduced_motion(&store) && !framework(&store));
        for (preset, enabled, want) in [
            (Some("instant"), None, true),
            (Some("lively"), Some(false), true),
            (Some("calm"), Some(true), false),
            (Some("instant"), Some(true), true),
            (None, Some(false), true),
            (Some("bilinmeyen"), None, false),
        ] {
            set(&mut store, "motion.preset", preset.map(Value::from));
            set(&mut store, "motion.enabled", enabled.map(Value::from));
            assert_eq!(reduced_motion(&store), want, "{preset:?} {enabled:?}");
            assert_eq!(framework(&store), want, "framework: {preset:?} {enabled:?}");
        }
    }

    #[test]
    fn inverted_toggle_reads_and_writes() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let d = by_id("motion.enabled").unwrap();
        assert!(!d.bool(&store), "varsayılan: hareket açık → 'azalt' kapalı");
        let v = d.toggle_value(true).unwrap();
        assert_eq!(v.as_bool(), Some(false));
        store.set(FileId::Framework, &d.key(), Some(v));
        assert!(d.bool(&store));
    }

    #[test]
    fn reduce_motion_row_shows_effective_state_and_clears_instant_preset() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let d = by_id(REDUCE_MOTION_ID).unwrap();
        let apply = |store: &mut Store, w: Vec<(FileId, KeyPath, Option<Value>)>| {
            for (f, p, v) in w {
                store.set(f, &p, v);
            }
        };
        let preset = KeyPath::parse("motion.preset");
        // "Kapalı" ön ayarı: anahtar açık görünür.
        store.set(FileId::Framework, &preset, Some(Value::from("instant")));
        assert!(d.bool(&store), "Kapalı ön ayarı = azaltılmış hareket");
        // Kapatmak: enabled açık (varsayılan) + ön ayar silinir, tek grup.
        let w = d.toggle_writes(false, &store);
        assert_eq!(w.len(), 2);
        assert!(w.iter().any(|(_, p, v)| *p == preset && v.is_none()));
        let en = w.iter().find(|(_, p, _)| p.segments() == ["motion", "enabled"]).unwrap();
        assert!(en.2.as_ref().is_none_or(|v| v.as_bool() == Some(true)), "enabled açık ya da varsayılan (silinir)");
        apply(&mut store, w);
        assert!(!d.bool(&store) && !reduced_motion(&store));
        assert!(store.get(FileId::Framework, &preset).is_none(), "varsayılan ön ayara döndü");
        // Açmak: bugünkü gibi enabled = false, ön ayara dokunmaz.
        store.set(FileId::Framework, &preset, Some(Value::from("lively")));
        let w = d.toggle_writes(true, &store);
        assert_eq!(w.len(), 1);
        assert_eq!(w[0].2.as_ref().and_then(Value::as_bool), Some(false));
        apply(&mut store, w);
        assert!(d.bool(&store));
        // Ön ayar Kapalı değilken kapatmak yalnız enabled'a dokunur.
        let w = d.toggle_writes(false, &store);
        assert_eq!(w.len(), 1);
        apply(&mut store, w);
        assert!(!d.bool(&store));
        assert_eq!(store.get(FileId::Framework, &preset).and_then(Value::as_str), Some("lively"));
    }

    #[test]
    fn outlines_subtitle_notes_high_contrast_none() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let d = by_id("shape.outlines").unwrap();
        let base = "Bileşen sınırları; yüksek karşıtlık açıkken en az ince çizilir";
        let note = "Yüksek karşıtlık açık: ince çiziliyor";
        assert_eq!(d.subtitle_in(&store), base, "varsayılan: standard + Yok");
        store.set(FileId::Framework, &KeyPath::parse("colors.contrast"), Some(Value::from("high")));
        assert_eq!(d.subtitle_in(&store), note, "high + Yok (varsayılan)");
        store.set(FileId::Framework, &d.key(), Some(Value::from("full")));
        assert_eq!(d.subtitle_in(&store), base, "high + Tam");
        store.set(FileId::Framework, &d.key(), Some(Value::from("none")));
        assert_eq!(d.subtitle_in(&store), note);
        // Palet kontrastı yeniden adlandırıldı; "kontrast" ikisini de bulur.
        let p = by_id("palette.contrast").unwrap();
        assert_eq!(p.title, "Palet kontrastı");
        let mut store2 = store;
        store2.set(FileId::Framework, &KeyPath::parse("colors.source"), Some(Value::from("wallpaper")));
        let found = ids_with(&store2, "kontrast");
        assert!(found.contains(&"palette.contrast") && found.contains(&"colors.contrast"), "{found:?}");
    }

    #[test]
    fn edge_actions_default_and_step_visibility() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let scroll = by_id("bar.edge_actions.end.scroll").unwrap();
        assert_eq!(scroll.string(&store), "widget", "varsayılan nötr");
        let step = by_id("bar.edge_actions.scroll_step").unwrap();
        assert!(!step.is_visible(&store), "adım yalnızca seviye kaydırmasında");
        let i = EDGE_SCROLL.iter().position(|(v, _)| *v == "volume").unwrap();
        store.set(FileId::Shell, &scroll.key(), scroll.choice_value(i, &store));
        assert!(step.is_visible(&store));
        assert_eq!(step.number(&store), 5.0);
        // Varsayılanı seçmek anahtarı siler.
        assert!(scroll.choice_value(0, &store).is_none());
    }

    /// Yerel yedekler (seçim tabloları, kaydırıcı aralıkları, kısayol satırları) gerçek shell
    /// şemasıyla aynı (bkz. `widgets::real_shell_schema`: shell crate'inden süreç içinde, her zaman var).
    #[test]
    fn local_fallbacks_match_real_schema() {
        assert!(WALLPAPER_BACKDROP_COLORS.iter().all(|(v, _)| !v.starts_with('#')), "sabit renk yok, yalnızca tema rolü");
        let text = widgets::real_shell_schema();
        let schema = widgets::parse_settings(&text);
        let keys: Vec<&str> = schema.keys().map(String::as_str).collect();
        let m = schema_mismatches(|k| schema.get(k), &keys);
        assert!(m.is_empty(), "yerel yedek şemadan farklı:\n{}", m.join("\n"));
        // Testteki doğrulama sınırı kopyası da şemayla aynı.
        for (k, s) in &schema {
            if let (Some(want), Some(got)) = (shell_limits(k), s.bounds()) {
                assert_eq!(want, got, "{k}: shell_limits şemadan farklı");
            }
        }
        // Şema okunabiliyor: kayıttaki her shell.toml kaydırıcı/seçim satırı şemada var.
        for d in SETTINGS.iter().filter(|d| d.file == FileId::Shell && matches!(d.control, Control::Choice { .. } | Control::Slider { .. })) {
            assert!(schema.contains_key(d.path), "{}: şemada yok", d.path);
        }
    }

    fn owned(c: &[(&str, &str)]) -> Vec<Choice> {
        c.iter().map(|(v, l)| ((*v).to_owned(), (*l).to_owned())).collect()
    }

    /// Seçim ayarı şeması: değerler ve (varsa) shell'in görünen adları.
    fn choice_spec(values: &[&str], labels: Option<&[&str]>) -> SettingSpec {
        let quote = |a: &[&str]| a.iter().map(|v| format!("\"{v}\"")).collect::<Vec<_>>().join(", ");
        let labels = labels.map_or(String::new(), |l| format!("option_labels = [{}]\n", quote(l)));
        let text = format!("version = 1\n[[setting]]\nkey = \"x.y\"\nkind = \"choice\"\noptions = [{}]\n{labels}", quote(values));
        widgets::parse_settings(&text).remove("x.y").unwrap()
    }

    #[test]
    fn choices_follow_schema_with_local_labels() {
        let local: &[(&str, &str)] = &[("blur", "Bulanık görsel"), ("color", "Düz renk")];
        assert_eq!(resolve_choices(local, None), owned(local), "şema yok: yerel");
        let text = choice_spec(&["a"], None);
        let text = SettingSpec { kind: SettingKind::Text, ..text };
        assert_eq!(resolve_choices(local, Some(&text)), owned(local), "seçim olmayan şema: yerel");
        let schema = choice_spec(&["color", "gradient", "blur"], None);
        assert_eq!(resolve_choices(local, Some(&schema)), owned(&[("color", "Düz renk"), ("gradient", "gradient"), ("blur", "Bulanık görsel")]), "shell sırası, etiketsiz değer ham adla");
        let only = choice_spec(&["blur"], None);
        assert_eq!(resolve_choices(local, Some(&only)), owned(&[("blur", "Bulanık görsel")]), "shell'in tanımadığı değer düşer");
        let with_unset: &[(&str, &str)] = &[("", "Genel"), ("spring", "Yay")];
        assert_eq!(resolve_choices(with_unset, Some(&choice_spec(&["spring"], None))), owned(&[("", "Genel"), ("spring", "Yay")]), "\"Genel\" başta kalır");
    }

    /// Shell'in `option_labels`'ı (etkin dilde) yerel addan önce gelir; eksik ya da boş ad yerel
    /// ada, o da yoksa kataloğa (kenar adları) ya da ham değere düşer; uzunluğu uymayan dizi yok sayılır.
    #[test]
    fn choices_prefer_schema_option_labels() {
        let local: &[(&str, &str)] = &[("blur", "Bulanık görsel"), ("color", "Düz renk")];
        let en = choice_spec(&["blur", "color", "gradient"], Some(&["Blurred image", "Solid color", "Gradient"]));
        assert_eq!(resolve_choices(local, Some(&en)), owned(&[("blur", "Blurred image"), ("color", "Solid color"), ("gradient", "Gradient")]));
        let partial = choice_spec(&["blur", "color", "gradient"], Some(&["", "Solid color", " "]));
        assert_eq!(resolve_choices(local, Some(&partial)), owned(&[("blur", "Bulanık görsel"), ("color", "Solid color"), ("gradient", "gradient")]), "boş ad: yerel ya da ham");
        let short = choice_spec(&["blur", "color"], Some(&["Blurred image"]));
        assert!(short.option_labels.is_empty(), "uzunluk uymuyor: adlar yok sayılır");
        assert_eq!(resolve_choices(local, Some(&short)), owned(local));
        let sides = choice_spec(&["left", "nowhere"], None);
        assert_eq!(resolve_choices(&[], Some(&sides)), vec![("left".to_owned(), t!("settings-edge-left")), ("nowhere".to_owned(), "nowhere".to_owned())], "katalog adı, yoksa ham");
        // Tekrarlı ve dize olmayan değerler atlanırken adlar doğru değere kalır.
        let messy = widgets::parse_settings("version = 1\n[[setting]]\nkey = \"x.y\"\nkind = \"choice\"\noptions = [\"a\", 3, \"a\", \"b\"]\noption_labels = [\"A\", \"?\", \"A2\", \"B\"]\n").remove("x.y").unwrap();
        assert_eq!(resolve_choices(&[], Some(&messy)), owned(&[("a", "A"), ("b", "B")]));
    }

    #[test]
    fn bounds_intersect_schema() {
        assert_eq!(resolve_bounds((4.0, 128.0), None), (4.0, 128.0));
        assert_eq!(resolve_bounds((4.0, 128.0), Some((8.0, 64.0))), (8.0, 64.0), "shell daralttı");
        assert_eq!(resolve_bounds((200.0, 600.0), Some((160.0, 900.0))), (200.0, 600.0), "yerel ürün aralığı kalır");
        assert_eq!(resolve_bounds((4.0, 128.0), Some((200.0, 300.0))), (4.0, 128.0), "kesişim boş: yerel");
        assert_eq!(resolve_bounds((4.0, 128.0), Some((128.0, 300.0))), (4.0, 128.0), "tek nokta: yerel");
    }

    #[test]
    fn mismatches_report_drift() {
        let text = r#"version = 1
[[setting]]
key = "wallpaper.backdrop"
kind = "choice"
options = ["blur", "color", "gradient"]
[[setting]]
key = "wallpaper.backdrop_blur"
kind = "int"
min = 8
max = 64
[[setting]]
key = "keybinds.screenshot"
kind = "text"
since = 3
"#;
        let schema = widgets::parse_settings(text);
        let keys: Vec<&str> = schema.keys().map(String::as_str).collect();
        let m = schema_mismatches(|k| schema.get(k), &keys).join("\n");
        assert!(m.contains("wallpaper.backdrop: yerel seçenekler") && m.contains("\"gradient\" için etiket yok"), "{m}");
        assert!(m.contains("wallpaper.backdrop_blur: yerel aralık [4, 128] şema sınırı [8, 64] dışında"), "{m}");
        assert!(m.contains("keybinds.screenshot: şemadaki kısayol eyleminin Ayarlar'da satırı yok"), "{m}");
        assert!(m.contains("keybinds.volume_up: kısayol satırının eylemi şemada yok"), "{m}");
        assert!(!m.contains("wallpaper.fit"), "şemada olmayan anahtar fark değil (eski shell)");
    }

    #[test]
    fn rows_without_schema_have_no_since() {
        // Testte şema yüklü değil: rozet yok, seçenekler/aralıklar yerel yedekten.
        let kind = by_id("wallpaper.backdrop").unwrap();
        assert_eq!(kind.since(), None);
        assert_eq!(kind.choices(), owned(WALLPAPER_BACKDROPS));
        assert_eq!(by_id("wallpaper.backdrop_blur").unwrap().bounds(), Some(WALLPAPER_BACKDROP_BLUR));
        assert_eq!(by_id("wallpaper.backdrop").unwrap().bounds(), None);
    }

    #[test]
    fn wallpaper_backdrop_rows() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let kind = by_id("wallpaper.backdrop").unwrap();
        let blur = by_id("wallpaper.backdrop_blur").unwrap();
        let color = by_id("wallpaper.backdrop_color").unwrap();
        // Varsayılanlar shell'inki: bulanık kopya, 48 px, tema yüzeyi.
        assert_eq!(kind.string(&store), "blur");
        assert_eq!(blur.number(&store), 48.0);
        assert_eq!(color.string(&store), "surface");
        // Zemin yalnızca kenar boşluğu varken (Tamamı) anlamlı: o zaman görünür.
        assert!(!kind.is_visible(&store) && !blur.is_visible(&store) && !color.is_visible(&store));
        // Dosya görseli: boş yolda varsayılan yerleşik görünür, o her zaman doldurur.
        store.set(FileId::Shell, &KeyPath::parse("wallpaper.path"), Some(Value::from("/a.png")));
        store.set(FileId::Shell, &KeyPath::parse("wallpaper.fit"), Some(Value::from("contain")));
        assert!(kind.is_visible(&store) && blur.is_visible(&store) && color.is_visible(&store));
        // Düz renkte yarıçap gizlenir (aramada da yok), renk kalır.
        store.set(FileId::Shell, &kind.key(), kind.choice_value(1, &store));
        assert_eq!(store.get(FileId::Shell, &KeyPath::parse("wallpaper.backdrop")).and_then(Value::as_str), Some("color"));
        assert!(!blur.is_visible(&store) && color.is_visible(&store));
        assert!(!ids_with(&store, "zemin bulanıklığı").contains(&"wallpaper.backdrop_blur"));
        // Yazım: tam sayı yarıçap, rol adı.
        assert_eq!(blur.number_value(16.4).as_integer(), Some(16));
        assert_eq!(blur.number_value(1000.0).as_integer(), Some(128), "shell sınırına kırpılır");
        assert_eq!(color.choice_value(1, &store).and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some("surface_container"));
    }

    #[test]
    fn panel_content_rows_follow_enabled() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        assert!(by_id("panels.right.widgets").unwrap().is_visible(&store));
        let left = by_id("panels.left.widgets").unwrap();
        assert!(!left.is_visible(&store), "sol panel varsayılan kapalı");
        store.set(FileId::Shell, &KeyPath::parse("panels.left.enabled"), Some(Value::from(true)));
        assert!(left.is_visible(&store));
    }

    #[test]
    fn keybind_rows_match_shell_actions() {
        // Shell `hypr_config/keybinds.rs` `actions!` sırası ve varsayılanları.
        let shell = [
            ("volume_up", "XF86AudioRaiseVolume"),
            ("volume_down", "XF86AudioLowerVolume"),
            ("volume_mute", "XF86AudioMute"),
            ("mic_mute", "XF86AudioMicMute"),
            ("brightness_up", "XF86MonBrightnessUp"),
            ("brightness_down", "XF86MonBrightnessDown"),
            ("media_play", "XF86AudioPlay"),
            ("media_pause", "XF86AudioPause"),
            ("media_next", "XF86AudioNext"),
            ("media_prev", "XF86AudioPrev"),
            ("panel_right", ""),
            ("panel_left", ""),
            ("panel_clock", ""),
            ("bar_toggle", ""),
            ("bar_focus", "CTRL+ALT+Tab"),
            ("shot_region", "Print"),
            ("shot_output", "SHIFT+Print"),
            ("shot_window", "ALT+Print"),
            ("clipboard", "SUPER+SHIFT+V"),
            ("magnifier_in", "SUPER+ALT+Up"),
            ("magnifier_out", "SUPER+ALT+Down"),
            ("magnifier_reset", "SUPER+ALT+Escape"),
            ("emoji", "SUPER+period"),
            ("osk_toggle", "SUPER+ALT+K"),
            ("launcher", "SUPER"),
            ("settings", "SUPER+I"),
            ("terminal", "CTRL+ALT+T"),
            ("file_manager", "SUPER+E"),
            ("browser", "SUPER+B"),
            ("lock", "SUPER+L"),
            ("session_menu", "CTRL+ALT+Delete"),
            ("close_window", "SUPER+Q"),
            ("close_window_alt", "ALT+F4"),
            ("force_kill", "CTRL+ALT+Escape"),
            ("fullscreen", "SUPER+F"),
            ("maximize", "SUPER+M"),
            ("toggle_floating", "SUPER+SHIFT+space"),
            ("pin", "SUPER+P"),
            ("cycle_windows", "ALT+Tab"),
            ("cycle_windows_back", "ALT+SHIFT+Tab"),
            ("toggle_split", "SUPER+J"),
            ("focus_left", "SUPER+Left"),
            ("focus_right", "SUPER+Right"),
            ("focus_up", "SUPER+Up"),
            ("focus_down", "SUPER+Down"),
            ("move_left", "SUPER+SHIFT+Left"),
            ("move_right", "SUPER+SHIFT+Right"),
            ("move_up", "SUPER+SHIFT+Up"),
            ("move_down", "SUPER+SHIFT+Down"),
            ("shrink_width", "SUPER+CTRL+Left"),
            ("grow_width", "SUPER+CTRL+Right"),
            ("shrink_height", "SUPER+CTRL+Up"),
            ("grow_height", "SUPER+CTRL+Down"),
            ("workspace", "SUPER"),
            ("move_to_workspace", "SUPER+SHIFT"),
            ("workspace_next", "SUPER+Page_Down"),
            ("workspace_prev", "SUPER+Page_Up"),
            ("move_to_workspace_next", "SUPER+SHIFT+Page_Down"),
            ("move_to_workspace_prev", "SUPER+SHIFT+Page_Up"),
            ("workspace_last", "SUPER+Tab"),
            ("scratchpad", "SUPER+S"),
            ("move_to_scratchpad", "SUPER+SHIFT+S"),
            ("window_drag", "SUPER+mouse:272"),
            ("window_resize_drag", "SUPER+mouse:273"),
            ("workspace_scroll_next", "SUPER+mouse_down"),
            ("workspace_scroll_prev", "SUPER+mouse_up"),
        ];
        let rows: Vec<_> = keybind_rows().collect();
        assert_eq!(rows.len(), shell.len());
        for (d, (name, default)) in rows.iter().zip(shell) {
            assert_eq!(d.path, format!("keybinds.{name}"));
            assert_eq!(d.id, d.path);
            assert_eq!(d.page, PageId::Keybinds);
            assert_eq!(d.default_for(Default::default()).and_then(Value::as_str), Some(default), "{name}");
        }
    }

    #[test]
    fn window_rows_follow_sync_toggles() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let blur_size = by_id("windows.blur_size").unwrap();
        assert!(blur_size.is_visible(&store), "varsayılan: eşitleme ve bulanıklık açık");
        assert_eq!(blur_size.number(&store), 8.0);
        assert_eq!(by_id("windows.gap_outer").unwrap().number(&store), 8.0);
        store.set(FileId::Shell, &KeyPath::parse("compositor.sync_look"), Some(Value::from(false)));
        assert!(!blur_size.is_visible(&store));
        // Köşe/gölge temadan gelmiyor; tek bağlantı satırı hareket için kalır.
        assert!(!by_id("windows.shadow").unwrap().is_visible(&store));
        assert!(by_id("windows.motion").unwrap().is_visible(&store));
        assert!(by_id("windows.corners").unwrap().is_visible(&store));
        store.set(FileId::Shell, &KeyPath::parse("compositor.sync_motion"), Some(Value::from(false)));
        assert!(!by_id("windows.corners").unwrap().is_visible(&store));
        assert!(by_id("windows.gap_inner").unwrap().is_visible(&store));
        store.set(FileId::Shell, &KeyPath::parse("compositor.sync_keybinds"), Some(Value::from(false)));
        assert!(!by_id("keybinds.volume_up").unwrap().is_visible(&store));
        assert!(by_id("media_keys").unwrap().is_visible(&store), "kısayollar kapalıyken elle bind örneği");
        assert!(!by_id("media_keys.link").unwrap().is_visible(&store));
        assert!(ids_with(&Store::open(dir.path().join("x.toml"), dir.path().join("y.toml"), dir.path().join("z.toml")), "kisayol").contains(&"keybinds.panel_right"));
    }

    #[test]
    fn every_group_has_pages_and_defaults_are_basic() {
        for g in NavGroup::ALL {
            assert!(PageId::ALL.iter().any(|p| p.group() == g), "{g:?} boş");
        }
        // Kayıt makrosunun varsayılanları: sondaki seçenekler yazılmazsa temel, aynasız, sayfa
        // durumundan bağımsız. Sayfa durumuna bağlı olanlar yalnızca Bar'ın Seçili widget
        // kartında (§6.1).
        let about = by_id("about").unwrap();
        assert!(about.tier == Tier::Basic && about.mirror_on.is_empty() && about.ui_if.is_none());
        let bind = by_id("keybinds.panel_right").unwrap();
        assert_eq!((bind.page, bind.mirror_on), (PageId::Keybinds, &[PageId::Panels][..]));
        assert!(SETTINGS.iter().filter(|s| s.ui_if.is_some()).all(|s| s.page == PageId::Bar && s.section == "Seçili widget"));
        assert!(SETTINGS.iter().all(|s| s.detail().is_none_or(|n| detail_group(n).is_some())));
    }

    #[test]
    fn page_slugs() {
        assert_eq!(PageId::from_slug("kısayollar"), Some(PageId::Keybinds));
        assert_eq!(PageId::from_slug("windows"), Some(PageId::Windows));
        assert_eq!(PageId::from_slug("motion"), Some(PageId::Motion));
        assert_eq!(PageId::from_slug("Duvar Kağıdı"), Some(PageId::Wallpaper));
        assert_eq!(PageId::from_slug("hareket"), Some(PageId::Motion));
        assert_eq!(PageId::from_slug("panels"), Some(PageId::Panels));
        assert_eq!(PageId::from_slug("paneller"), Some(PageId::Panels));
        assert_eq!(PageId::from_slug("region"), Some(PageId::Region));
        assert_eq!(PageId::from_slug("Bölge ve dil"), Some(PageId::Region));
        // Geri uyum: kaldırılan Shell sayfasının slug'ı Bar'a gider.
        assert_eq!(PageId::from_slug("shell"), Some(PageId::Bar));
        assert_eq!(PageId::from_slug("Shell"), Some(PageId::Bar));
        // Her sayfa kendi slug'ıyla ve başlığıyla bulunur.
        for p in PageId::ALL {
            assert_eq!(PageId::from_slug(p.slug()), Some(p));
            assert_eq!(PageId::from_slug(p.title()), Some(p));
        }
    }

    /// Varsayılan uygulamalar (defaults-v1): her rolün kayıtta satırı, snapshot kimliği ve
    /// aranabilir anahtar sözcükleri var. Arama sonucu, shell durumu ne olursa olsun bu sayfaya
    /// gider (satır ya da durum okunamadıysa sayfanın notu).
    #[test]
    fn default_app_rows_are_complete_and_searchable() {
        use crate::services::default_apps::Role;
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        for role in Role::ALL {
            let d = by_id(role.setting_id()).unwrap_or_else(|| panic!("{role:?}: kayıtta yok"));
            assert_eq!(d.page, PageId::DefaultApps);
            assert_eq!(d.control, Control::Custom(CustomId::DefaultApp(role)));
            assert_eq!(crate::services::snapshots::setting_for(d.id), Some(role.setting_id()), "geri alma kimliği");
            assert!(crate::services::snapshots::SNAPSHOT_SETTINGS.contains(&role.setting_id()));
        }
        assert_eq!(PageId::from_slug("default-apps"), Some(PageId::DefaultApps));
        assert_eq!(PageId::from_slug("varsayılan uygulamalar"), Some(PageId::DefaultApps));
        for (q, id) in [
            ("tarayıcı", "defaults.browser"),
            ("browser", "defaults.browser"),
            ("mailto", "defaults.email"),
            ("e-posta", "defaults.email"),
            ("dosya yöneticisi", "defaults.file_manager"),
            ("klasör", "defaults.file_manager"),
            ("xdg-terminal-exec", "defaults.terminal"),
            ("metin düzenleyici", "defaults.text_editor"),
            ("fotoğraf", "defaults.image"),
            ("mkv", "defaults.video"),
            ("mp3", "defaults.music"),
            ("pdf", "defaults.pdf"),
        ] {
            assert!(by_id(id).unwrap().matches(&fold(q)), "{q} → {id}");
            let hits = search(&store, q);
            assert!(hits.iter().any(|h| h.id == id || h.id == "defaults.note"), "{q}: {:?}", hits.iter().map(|h| h.id).collect::<Vec<_>>());
        }
    }

    /// Kenar çubuğu §2.2 (+ Bağlantılar, 2026-10-13): dört grup, her grubun sayfaları ardışık; `ALL` bildirim sırasında
    /// (arama sonuçları `Ord` ile kenar çubuğu sırasında dizilir).
    #[test]
    fn sidebar_groups_follow_design() {
        let groups = nav_groups(&nav_pages());
        assert_eq!(groups, vec![(0, "Kişisel"), (5, "Masaüstü"), (12, "Bağlantılar"), (15, "Sistem")]);
        let all = nav_groups(&PageId::ALL);
        assert_eq!(all, vec![(0, "Kişisel"), (5, "Masaüstü"), (12, "Bağlantılar"), (15, "Sistem")], "gizli sayfa grup başını kaydırmaz");
        assert!(!PageId::Power.listed() && !nav_pages().contains(&PageId::Power), "güç verisi yok (birim testi) → Güç kenar çubuğunda yok");
        let names = |g: NavGroup| PageId::ALL.iter().filter(|p| p.group() == g).map(|p| p.title()).collect::<Vec<_>>();
        assert_eq!(names(NavGroup::Personal), ["Görünüm", "Duvar kağıdı", "Yazı", "Hareket", "Erişilebilirlik"], "Erişilebilirlik (2026-10-16)");
        assert_eq!(names(NavGroup::Desktop), ["Bar", "Paneller", "Pencereler", "Ekranlar", "Klavye", "Fare ve dokunmatik yüzey", "Kısayollar"]);
        assert_eq!(names(NavGroup::Connections), ["Wi-Fi", "VPN", "Bluetooth"], "karar, 2026-10-13 (Bluetooth t115)");
        assert_eq!(names(NavGroup::System), ["Bildirimler", "Ses", "Güç", "Yazıcılar", "Bölge ve dil", "Tarih ve saat", "Varsayılan uygulamalar", "Kullanıcı hesabı", "Hesaplar", "Hakkında"]);
        assert!(PageId::ALL.windows(2).all(|w| w[0] < w[1]), "ALL bildirim sırasında değil");
    }

    /// t119: Tarih ve saat sayfası. Saat biçimi framework şemasından (`[locale] hour_cycle`
    /// varsayılanı varsa görünür); bar saati notu yalnızca shell.toml'da `clock.format` yazılıyken.
    #[test]
    fn datetime_page_rows() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let ids: Vec<_> = for_page(PageId::DateTime).map(|s| s.id).collect();
        assert_eq!(ids, ["datetime.note", "datetime.ntp", "datetime.timezone", "datetime.manual", "locale.hour_cycle", "datetime.clock_format_note"]);
        let hc = by_id("locale.hour_cycle").unwrap();
        assert_eq!(hc.default_value(&store).and_then(Value::as_str), Some("auto"), "framework şeması hour_cycle'ı tanıyor");
        assert!(hc.is_visible(&store));
        let note = by_id("datetime.clock_format_note").unwrap();
        assert!(!note.is_visible(&store));
        store.set(FileId::Shell, &KeyPath::parse("bar.widget.clock.format"), Some(Value::from("%H:%M")));
        assert!(note.is_visible(&store), "açık clock.format kazanır notu");
        // Seçim "Otomatik" anahtarı siler; açık değer yazılır.
        assert!(hc.choice_stored("auto", &store).is_none());
        assert_eq!(hc.choice_stored("h12", &store).and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some("h12"));
        assert_eq!(PageId::from_slug("datetime"), Some(PageId::DateTime));
    }

    /// Shell saat sözleşmesi (`clock-locale.md`): yazılmamış, `""` ya da yalnız boşluk =
    /// otomatik (not yok); dolu kalıp not gösterir. Etkili anahtar bar'ın yönüne bağlı.
    #[test]
    fn clock_note_follows_effective_pattern() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let note = by_id("datetime.clock_format_note").unwrap();
        let set = |s: &mut Store, k: &str, v: Option<&str>| s.set(FileId::Shell, &KeyPath::parse(k), v.map(Value::from));
        for empty in ["", " ", "\t "] {
            set(&mut store, "bar.widget.clock.format", Some(empty));
            assert!(!note.is_visible(&store), "boş/boşluk {empty:?} otomatik sayılır");
        }
        set(&mut store, "bar.widget.clock.format", Some("%a %H:%M"));
        assert!(note.is_visible(&store), "açık kalıp kazanır");
        // Yatay bar'da yalnız dikey kalıp yazılıysa saat yine otomatik.
        set(&mut store, "bar.widget.clock.format", None);
        set(&mut store, "bar.widget.clock.vertical_format", Some("%H\n%M"));
        assert!(!note.is_visible(&store), "yatay bar dikey kalıbı okumaz");
        // Dikey bar'da tersi.
        set(&mut store, "bar.edge", Some("left"));
        assert!(note.is_visible(&store), "dikey bar vertical_format'ı okur");
        set(&mut store, "bar.widget.clock.vertical_format", Some("  "));
        set(&mut store, "bar.widget.clock.format", Some("%H:%M"));
        assert!(!note.is_visible(&store), "dikey bar format'ı okumaz; boşluk otomatik");
        // Dize olmayan değer (shell reddeder) de elle yazılmış sayılır ve temizlenebilir.
        store.set(FileId::Shell, &KeyPath::parse("bar.widget.clock.vertical_format"), Some(Value::from(5)));
        assert!(clock_format_explicit(&store, "bar.widget.clock.vertical_format"));
    }

    /// Yedek varsayılanlar shell şemasıyla eş (`""`); boş alanın yer tutucusu otomatik olduğunu
    /// söyler, alt metin sözleşme metni (şema yokken yerel).
    #[test]
    fn clock_format_defaults_are_empty_and_described() {
        crate::i18n::use_turkish_for_tests();
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        for k in CLOCK_FORMAT_KEYS {
            assert_eq!(crate::defaults::get(&store, FileId::Shell, &KeyPath::parse(k)).and_then(Value::as_str), Some(""), "{k}");
        }
        let d = by_id("bar.widget.clock.format").unwrap();
        assert_eq!(d.placeholder(&store), "Otomatik (12/24 saat tercihine uyar)");
        assert_eq!(d.display_subtitle(), "Boş bırakılırsa 12/24 saat tercihine uyar; bir biçim kalıbı yazılırsa o kullanılır.");
        // Boş değer varsayılana eşit: ↺ gösterilmez.
        assert!(d.same_value(&Value::from(""), d.default_value(&store).unwrap()));
        // Şema açıklaması yerel metnin önüne geçer. (Eski shell'in dolu varsayılanı yer tutucuda
        // kalıp olarak görünür; testte şema varsayılanları okunmaz, `widgets::shell_default`.)
        widgets::set_test_schema(
            "version = 1\n[[widget]]\nname = \"clock\"\ntitle = \"Saat\"\nbar = true\n[[widget.setting]]\nkey = \"format\"\ntitle = \"Biçim\"\nkind = \"text\"\ndefault = \"%H:%M\"\ndescription = \"Şemadan\"\n",
        );
        assert_eq!(d.display_subtitle(), "Şemadan");
        widgets::set_test_schema("version = 1\n");
    }

    /// t094 taşımaları: çerçeve Bar'ın son kartı, paneller kendi sayfasında panel başına kart,
    /// dil ve biçimler Bölge ve dil'de. Kimlikler ve anahtarlar değişmez.
    #[test]
    fn moved_settings_live_on_their_new_pages() {
        let bar: Vec<_> = for_page(PageId::Bar).collect();
        let first_frame = bar.iter().position(|s| s.section == "Çerçeve").unwrap();
        assert!(bar[first_frame..].iter().all(|s| s.section == "Çerçeve"), "çerçeve Bar'ın son kartı değil");
        for id in ["frame.enabled", "frame.thickness", "frame.edges", "frame.inner_corners"] {
            assert_eq!(by_id(id).unwrap().page, PageId::Bar, "{id}");
        }
        assert_eq!(by_id("frame.enabled").unwrap().path, "frame.enabled");
        for (edge, section) in [("right", "Sağ panel"), ("left", "Sol panel"), ("top", "Üst panel"), ("bottom", "Alt panel")] {
            for key in ["enabled", "size", "widgets"] {
                let d = by_id(&format!("panels.{edge}.{key}")).unwrap();
                assert_eq!((d.page, d.section), (PageId::Panels, section), "{}", d.id);
                assert_eq!(d.path, format!("panels.{edge}.{key}"));
            }
        }
        let sections: Vec<_> = for_page(PageId::Panels).map(|s| s.section).collect::<Vec<_>>();
        let mut order = sections.clone();
        order.dedup();
        assert_eq!(order, ["Sağ panel", "Sol panel", "Üst panel", "Alt panel"]);
        for id in ["locale.language", "locale.formats"] {
            let d = by_id(id).unwrap();
            assert_eq!((d.page, d.section, d.path), (PageId::Region, "Dil ve biçimler", id));
        }
        assert!(for_page(PageId::About).all(|s| s.id == "about"), "Hakkında yalnızca sürüm kartı");
    }

    /// Kilit ve boşta kartları: shell şeması tanımıyorsa yok; sıra: uykudan önce kilit ve kilit
    /// ekranı anahtarı en üstte, sonra boşta ana anahtarı ve aşamaları. Ana anahtar kapalıyken
    /// boşta satırları gizli; uykudan önce kilit kalır ve çalışmadığını uyarısı söyler.
    #[test]
    fn idle_rows_follow_master_and_stages() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let vis = |store: &Store, id: &str| by_id(id).unwrap().is_visible(store);
        let shown = |store: &Store| {
            for_page(PageId::Power).filter(|d| (d.id.starts_with("idle.") || d.id.starts_with("lock.")) && vis(store, d.id)).map(|d| d.id).collect::<Vec<_>>()
        };
        let schema = |keys: &[&str]| {
            let rows: String = keys.iter().map(|k| format!("[[setting]]\nkey = \"{k}\"\ntitle = \"x\"\nkind = \"bool\"\ndefault = true\n")).collect();
            widgets::set_test_schema(&format!("version = {}\n{rows}", widgets::SCHEMA_VERSION));
        };
        let note = |store: &Store| by_id("idle.lock_before_sleep").unwrap().note_in(store);
        widgets::set_test_schema("");
        assert!(shown(&store).is_empty(), "eski shell: kart yok");
        assert!(!idle_shown(&store));
        schema(&["idle.enabled"]);
        assert_eq!(shown(&store), ["idle.enabled", "idle.lock", "idle.dim", "idle.screen_off", "idle.suspend", "idle.media_inhibits"], "lock_before_sleep'i bilmeyen shell");
        schema(&["lock.listen_logind"]);
        assert!(PageId::Power.listed() && idle_shown(&store));
        assert_eq!(shown(&store), ["lock.listen_logind"], "[idle] bilmeyen shell");
        schema(&["idle.enabled", "idle.lock_before_sleep", "lock.listen_logind"]);
        assert!(PageId::Power.listed(), "güç verisi yok ama kilit kartı var → Güç kenar çubuğunda");
        assert_eq!(shown(&store), ["idle.lock_before_sleep", "lock.listen_logind", "idle.enabled", "idle.lock", "idle.dim", "idle.screen_off", "idle.suspend", "idle.media_inhibits"]);
        assert!(by_id("idle.lock_before_sleep").unwrap().bool(&store), "varsayılan açık (shell 7b3c80e)");
        assert_eq!(note(&store), None);
        for stage in ["idle.lock", "idle.dim", "idle.screen_off", "idle.suspend"] {
            store.set(FileId::Shell, &KeyPath::parse(stage), Some(Value::from(true)));
        }
        assert_eq!(
            shown(&store),
            [
                "idle.lock_before_sleep",
                "lock.listen_logind",
                "idle.enabled",
                "idle.lock",
                "idle.lock_after",
                "idle.dim",
                "idle.dim_after",
                "idle.screen_off",
                "idle.screen_off_after",
                "idle.suspend",
                "idle.suspend_after",
                "idle.suspend_on_ac",
                "idle.media_inhibits"
            ]
        );
        store.set(FileId::Shell, &KeyPath::parse("idle.enabled"), Some(Value::from(false)));
        assert_eq!(shown(&store), ["idle.lock_before_sleep", "lock.listen_logind", "idle.enabled"], "ana anahtar kapalı: boşta eylemi yok");
        assert!(note(&store).is_some(), "shell enabled=false iken uykudan önce kilitlemez: açıkça söylenir");
        store.set(FileId::Shell, &KeyPath::parse("idle.lock_before_sleep"), Some(Value::from(false)));
        assert_eq!(note(&store), None, "anahtar kapalıysa uyarı gereksiz");
        widgets::set_test_schema("");
    }

    /// Gece ışığı kartı: shell şeması tanımıyorsa yok; ana anahtar kapalıyken yalnızca anahtar;
    /// saat aralığı yalnızca `manual`, konum yalnızca `sun`, geçiş `always` dışında.
    #[test]
    fn night_light_rows_follow_schedule() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        let vis = |store: &Store, id: &str| by_id(id).unwrap().is_visible(store);
        widgets::set_test_schema("");
        assert!(!vis(&store, "night_light.enabled"), "eski shell: kart yok");
        widgets::set_test_schema(&format!("version = {}\n[[setting]]\nkey = \"night_light.enabled\"\ntitle = \"Gece ışığı\"\nkind = \"bool\"\ndefault = false\n", widgets::SCHEMA_VERSION));
        assert!(vis(&store, "night_light.enabled"));
        assert!(!vis(&store, "night_light.temperature"), "kapalıyken ayrıntı yok");
        store.set(FileId::Shell, &KeyPath::parse("night_light.enabled"), Some(Value::from(true)));
        let shown = |store: &Store| {
            ["night_light.temperature", "night_light.schedule", "night_light.from", "night_light.to", "night_light.location", "night_light.transition"]
                .into_iter()
                .filter(|id| vis(store, id))
                .collect::<Vec<_>>()
        };
        // Varsayılan zamanlama sun.
        assert_eq!(shown(&store), ["night_light.temperature", "night_light.schedule", "night_light.location", "night_light.transition"]);
        store.set(FileId::Shell, &KeyPath::parse("night_light.schedule"), Some(Value::from("manual")));
        assert_eq!(shown(&store), ["night_light.temperature", "night_light.schedule", "night_light.from", "night_light.to", "night_light.transition"]);
        store.set(FileId::Shell, &KeyPath::parse("night_light.schedule"), Some(Value::from("always")));
        assert_eq!(shown(&store), ["night_light.temperature", "night_light.schedule"]);
        // Birim: geçiş "min" → "dk", sıcaklık "K" aynen.
        crate::i18n::use_turkish_for_tests();
        let locale = nuxut::locale::Locale::parse("tr_TR").unwrap();
        assert_eq!(crate::rows::format_number_in(by_id("night_light.transition").unwrap(), 30.0, &locale), "30 dk");
        assert_eq!(crate::rows::format_number_in(by_id("night_light.temperature").unwrap(), 4000.0, &locale), "4000 K");
    }
}

/// Kayıt metinlerinin katalog kapsamı (`docs/design/i18n.md` "Kayıt"): her başlık, alt yazı,
/// arama sözcüğü, bölüm, sayfa ve seçenek adının anahtarı var ve Türkçe değeri kaynak metnin
/// aynısı (kayıtta metin değişirse katalog da güncellenmeli — İngilizcesi yeniden çevrilir).
#[cfg(test)]
mod catalog_tests {
    use nuxut::i18n::{Catalog, LanguagePrefs};

    use super::*;

    fn turkish() -> Catalog {
        Catalog::load(&[crate::i18n::source()], &LanguagePrefs::fixed(&["tr"]))
    }

    /// `key` var ve Türkçesi `source`; değilse sorun satırı.
    fn check(tr: &Catalog, key: &str, source: &str, out: &mut Vec<String>) {
        if out.iter().any(|o| o.starts_with(&format!("{key}:"))) {
            // Hareket rollerinin ortak satırları aynı anahtarı paylaşır; bir kez yazılır.
        } else if !tr.has(key) {
            out.push(format!("{key}: katalogda yok (kaynak: {source})"));
        } else if tr.text(key) != source {
            out.push(format!("{key}: katalog '{}', kaynak '{source}'", tr.text(key)));
        }
    }

    #[test]
    fn registry_texts_are_in_catalog() {
        let tr = turkish();
        let mut out = Vec::new();
        for d in SETTINGS {
            let k = d.text_key();
            if !d.title.is_empty() {
                check(&tr, &format!("{k}-label"), d.title, &mut out);
            }
            if !d.subtitle.is_empty() {
                check(&tr, &format!("{k}-description"), d.subtitle, &mut out);
            }
            if !d.keywords.is_empty() {
                check(&tr, &format!("{k}-keywords"), &d.keywords.join(", "), &mut out);
            }
            if !d.section.is_empty() && !SECTION_KEYS.iter().any(|(s, _)| *s == d.section) {
                out.push(format!("{}: bölüm '{}' SECTION_KEYS'te yok", d.id, d.section));
            }
            if let Control::Choice { options, .. } = d.control
                && !options.is_empty()
                && !CHOICE_TABLES.iter().any(|(t, _)| *t == options)
            {
                out.push(format!("{}: seçim tablosu CHOICE_TABLES'ta yok", d.id));
            }
        }
        for (source, key) in SECTION_KEYS {
            check(&tr, key, source, &mut out);
        }
        for (table, _) in CHOICE_TABLES {
            for (v, l) in *table {
                check(&tr, &choice_key(table, v).unwrap_or_default(), l, &mut out);
            }
        }
        for p in PageId::ALL {
            check(&tr, &format!("settings-page-{}", p.slug()), p.source_title(), &mut out);
        }
        for r in MotionRole::ALL {
            check(&tr, &format!("settings-motion-role-{}", r.key().replace('_', "-")), r.source_title(), &mut out);
        }
        for g in DETAILS {
            check(&tr, &format!("settings-detail-{}-label", g.key), g.name, &mut out);
            check(&tr, &format!("settings-detail-{}-description", g.key), g.subtitle, &mut out);
        }
        assert!(out.is_empty(), "kayıt metinleri katalogla uyuşmuyor:\n{}", out.join("\n"));
    }

    #[test]
    fn text_keys_follow_the_id() {
        assert_eq!(text_key("colors.mode"), "settings-colors-mode");
        assert_eq!(text_key("bar.widget.clock.format"), "settings-bar-widget-clock-format");
        assert_eq!(text_key("defaults.file_manager"), "settings-defaults-file-manager");
        // Hareket rollerinin ortak satırları tek anahtar; rol satırının kendisi kendi kimliğiyle.
        assert_eq!(text_key("motion.spatial_fast.damping"), "settings-motion-role-damping");
        assert_eq!(text_key("motion.effects.easing"), "settings-motion-role-easing");
        assert_eq!(text_key("motion.spatial"), "settings-motion-spatial");
        assert_eq!(choice_key(EDGE_CLICK, "panel:left").as_deref(), Some("settings-choice-edge-click-panel-left"));
        assert_eq!(choice_key(ROLE_STYLES, "").as_deref(), Some("settings-choice-role-style-default"));
        assert_eq!(choice_key(SCROLL_METHODS, "on_button_down").as_deref(), Some("settings-choice-scroll-method-on-button-down"));
    }

    /// Türkçe katalogda erişimciler kaynak metni verir (önbellek kaynağın kendisini döndürür).
    #[test]
    fn accessors_give_source_text_in_turkish() {
        crate::i18n::use_turkish_for_tests();
        let d = by_id("colors.mode").expect("colors.mode");
        assert_eq!((d.label(), d.description(), d.section_title()), (d.title, d.subtitle, d.section));
        assert_eq!(PageId::DefaultApps.title(), "Varsayılan uygulamalar");
        assert_eq!(choice_label(EDGE_CLICK, "panel:clock"), "Takvimi aç");
        assert_eq!(choice_label(EDGE_CLICK, "bilinmeyen"), "");
        assert_eq!(section_title("Kayıtta olmayan"), "Kayıtta olmayan");
    }
}
