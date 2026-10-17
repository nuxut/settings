//! Sayfa içerikleri: kayıttaki standart satırlar + sayfaya özgü özel bloklar.

mod about;
pub mod fiyu;
mod datetime;
mod user;
pub mod network;
pub mod bluetooth;
pub mod power;
mod printers;
mod accounts;
mod default_apps;
pub(crate) mod bar_preview;
pub mod disclosure;
mod displays;
mod edges;
mod fonts;
mod input_note;
mod keybinds;
pub(crate) mod keyboard;
mod link;
mod links;
mod theme_card_preview;
mod theme_grid;
pub(crate) mod locale;
mod assist;
pub(crate) mod hyprland_config;
mod mirror;
mod night_light;
mod panel_hint;
#[cfg(test)]
mod text_pages_tests;
mod color_theme;
mod look;
mod source_note;
mod motion;
mod media_keys;
mod motion_preset;
mod notice;
mod notif_apps;
mod placeholder;
mod sections;
pub mod system_theme;
mod wallpaper;
mod wallpaper_outputs;
pub mod wallpaper_grid;
mod preview;

use nuxut::Theme;

use crate::block::Block;
use crate::registry::{self, Control, CustomId, PageId, SettingDesc};
use crate::rows::SettingRow;
use crate::store::Store;

/// Sayfanın bloklarını kayıt sırasıyla üretir (birincil + ayna satırlar).
pub fn build(theme: &Theme, page: PageId, store: &Store) -> Vec<Box<dyn Block>> {
    build_from(theme, page, store, registry::SETTINGS)
}

/// `settings` tablosundan sayfa blokları. Aynı açılır satıra (`Tier::Detail`) ait ayarlar,
/// ilkinin yerinde tek [`disclosure::Disclosure`] bloğunda toplanır (kayıt sırasıyla).
pub fn build_from(theme: &Theme, page: PageId, store: &Store, settings: &'static [SettingDesc]) -> Vec<Box<dyn Block>> {
    let shown: Vec<&'static SettingDesc> = settings.iter().filter(|d| d.shown_on(page)).collect();
    let mut done: Vec<&'static str> = Vec::new();
    let mut out: Vec<Box<dyn Block>> = Vec::new();
    for (i, desc) in shown.iter().enumerate() {
        let Some(name) = desc.detail() else {
            let block = block_for(theme, desc, store);
            if desc.page != page {
                // Ayna satır (§3.4): sayfada kendinden önceki satırın kartına girer; önünde satır
                // yoksa sayfanın ayna kartına (`PageId::mirror_home`), o da yoksa kendi bölümüne.
                let section = shown[..i].iter().rev().find(|d| d.page == page).map(|d| d.section).or(page.mirror_home());
                out.extend(block.map(|b| Box::new(mirror::Mirror::new(b, section)) as Box<dyn Block>));
            } else {
                out.extend(block);
                // Birleşik görünüm kapalıyken gizlenen satırların yerine açıklama + "aç" eylemi.
                if desc.id == "desktop.unified" {
                    out.push(unified_off_note(theme, desc.section, true));
                }
            }
            continue;
        };
        if done.contains(&name) {
            continue;
        }
        done.push(name);
        let children = shown
            .iter()
            .filter(|d| d.detail() == Some(name))
            .filter_map(|d| block_for(theme, d, store).map(|b| (b, *d)))
            .collect();
        out.push(Box::new(disclosure::Disclosure::new(theme, desc.section, name, children, store)));
    }
    out
}

fn block_for(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Option<Box<dyn Block>> {
    match desc.control {
        // Kendi satırı yok: hedefin satırı onu da taşır (`links::Links::contains_id`).
        Control::Custom(CustomId::Alias(_)) => None,
        Control::Custom(id) => Some(custom(theme, id, desc, store)),
        _ => SettingRow::new(theme, desc, store).map(|r| Box::new(r) as Box<dyn Block>),
    }
}

/// Belirli ayarların blokları (Hoş geldin adımları): sayfadakiyle aynı satırlar, kopya yok.
/// Kayıtta olmayan kimlik atlanır.
pub fn build_ids(theme: &Theme, ids: &[&str], store: &Store) -> Vec<Box<dyn Block>> {
    ids.iter().filter_map(|id| registry::by_id(id)).filter_map(|desc| block_for(theme, desc, store)).collect()
}

/// Birleşik görünüm kapalıyken anahtarın altındaki satır (Görünüm ve Hoş geldin ortak):
/// açıklama, `action` ise "Birleşik görünümü aç" düğmesi. Bkz. [`look::UnifiedOffNote`].
pub fn unified_off_note(theme: &Theme, section: &'static str, action: bool) -> Box<dyn Block> {
    Box::new(look::UnifiedOffNote::new(theme, section, action))
}

fn custom(theme: &Theme, id: CustomId, desc: &'static registry::SettingDesc, store: &Store) -> Box<dyn Block> {
    match id {
        CustomId::About => Box::new(about::About::new(theme, desc, store)),
        CustomId::Accounts => Box::new(accounts::Accounts::new(theme, desc)),
        CustomId::BarSections => Box::new(sections::WidgetLists::new(theme, desc, store, sections::Target::Bar)),
        CustomId::PanelWidgets(edge) => Box::new(sections::WidgetLists::new(theme, desc, store, sections::Target::Panel(edge))),
        CustomId::BarEdge => Box::new(edges::BarEdgePicker::new(theme, desc, store)),
        CustomId::BarPreview => Box::new(bar_preview::BarPreview::new(theme, desc, store)),
        CustomId::SelectedWidget => Box::new(bar_preview::SelectedWidget::new(theme, desc, store)),
        CustomId::FrameEdges => Box::new(edges::FrameEdges::new(theme, desc, store)),
        CustomId::Link(target) => Box::new(link::Link::new(theme, desc, target, store)),
        CustomId::Links(targets) => Box::new(links::Links::new(theme, desc, targets)),
        CustomId::PanelHint(edge) => Box::new(panel_hint::PanelHint::new(theme, desc, edge, store)),
        CustomId::Wallpaper => Box::new(wallpaper::WallpaperPicker::new(theme, desc, store)),
        CustomId::WallpaperBuiltins => Box::new(wallpaper_grid::WallpaperGrid::new(theme, desc, store)),
        CustomId::Displays => Box::new(displays::Displays::new(theme, desc)),
        CustomId::WallpaperOutputs =>Box::new(wallpaper_outputs::WallpaperOutputs::new(theme, desc, store)),
        CustomId::Palette => Box::new(wallpaper::SchemePicker::new(theme, desc, store)),
        CustomId::WallpaperColors => Box::new(wallpaper::ColorsLink::new(theme, desc, store)),
        CustomId::ColorTheme => Box::new(color_theme::ColorThemePicker::new(theme, desc, store)),
        CustomId::NotificationApps => Box::new(notif_apps::NotificationApps::new(theme, desc, store)),
        CustomId::Notice => Box::new(notice::Notice::new(theme, desc)),
        CustomId::MediaKeys => Box::new(media_keys::MediaKeys::new(theme, desc)),
        CustomId::Keybind => Box::new(keybinds::KeybindRow::new(theme, desc, store)),
        CustomId::TypePreview => Box::new(fonts::TypePreview::new(theme, desc, store)),
        CustomId::IconSet => Box::new(fonts::IconSetPicker::new(theme, desc, store)),
        CustomId::FontFamily(role) => Box::new(fonts::FontFamilyPicker::new(theme, desc, role, store)),
        CustomId::MotionPreset => Box::new(motion_preset::PresetPicker::new(theme, desc, store)),
        CustomId::Preview => Box::new(preview::Preview::new(theme, desc, store)),
        CustomId::MotionRole(role) => Box::new(motion::RolePreview::new(theme, desc, role, store)),
        CustomId::Locale(field) => Box::new(locale::LocaleRow::new(theme, desc, field, store)),
        CustomId::LocalePreview => Box::new(locale::LocalePreview::new(theme, desc, store)),
        CustomId::DarkMode => Box::new(look::ModeChoice::new(theme, desc, store)),
        CustomId::IconTheme => Box::new(theme_grid::ThemeGrid::new(theme, desc, crate::services::look::ThemeKind::Icons, store)),
        CustomId::CursorTheme => Box::new(theme_grid::ThemeGrid::new(theme, desc, crate::services::look::ThemeKind::Cursors, store)),
        CustomId::LookStatus => Box::new(look::LookStatus::new(theme, desc)),
        CustomId::SourceNote => Box::new(source_note::SourceNote::new(theme, desc)),
        CustomId::InputNote => Box::new(input_note::InputNote::new(theme, desc, store)),
        CustomId::KeyboardLayouts => Box::new(keyboard::KeyboardLayouts::new(theme, desc, store)),
        CustomId::LayoutSwitch => Box::new(keyboard::LayoutSwitch::new(theme, desc, store)),
        CustomId::KeyTest => Box::new(keyboard::KeyTest::new(theme, desc)),
        CustomId::TimedateNote => Box::new(datetime::TimedateNote::new(theme, desc)),
        CustomId::AutoTime => Box::new(datetime::AutoTime::new(theme, desc)),
        CustomId::Timezone => Box::new(datetime::TimezoneRow::new(theme, desc)),
        CustomId::ManualTime => Box::new(datetime::ManualTime::new(theme, desc, store)),
        CustomId::ClockFormatNote => Box::new(datetime::ClockFormatNote::new(theme, desc)),
        CustomId::DefaultAppsNote => Box::new(default_apps::DefaultAppsNote::new(theme, desc)),
        CustomId::DefaultApp(role) => Box::new(default_apps::DefaultAppRow::new(theme, desc, role)),
        CustomId::NightLightLocation => Box::new(night_light::NightLocation::new(theme, desc, store)),
        CustomId::NightLightTime => Box::new(night_light::NightTime::new(theme, desc, store)),
        CustomId::Assist(tool) => Box::new(assist::AssistLive::new(theme, desc, tool)),
        CustomId::UserNote => Box::new(user::UserNote::new(theme, desc)),
        CustomId::UserAvatar => Box::new(user::UserAvatar::new(theme, desc)),
        CustomId::UserName => Box::new(user::UserName::new(theme, desc)),
        CustomId::NetworkNote => Box::new(network::NetworkNote::new(theme, desc)),
        CustomId::WifiStatus => Box::new(network::WifiStatusRow::new(theme, desc)),
        CustomId::NetList(kind) => Box::new(network::NetListBlock::new(theme, desc, kind)),
        CustomId::PowerStatus => Box::new(power::PowerStatus::new(theme, desc)),
        CustomId::BatteryStatus => Box::new(power::BatteryRow::new(theme, desc)),
        CustomId::PowerProfile => Box::new(power::PowerProfileRow::new(theme, desc)),
        CustomId::ChargeLimit => Box::new(power::ChargeLimitRow::new(theme, desc)),
        CustomId::BluetoothStatus => Box::new(bluetooth::BtStatus::new(theme, desc)),
        CustomId::BluetoothPower => Box::new(bluetooth::BtPowerRow::new(theme, desc)),
        CustomId::BluetoothName => Box::new(bluetooth::BtNameRow::new(theme, desc)),
        CustomId::BluetoothVisible => Box::new(bluetooth::BtVisibleRow::new(theme, desc)),
        CustomId::BluetoothDevices(kind) => Box::new(bluetooth::BtDeviceList::new(theme, desc, kind)),
        CustomId::PrintersStatus => Box::new(printers::PrintersStatus::new(theme, desc)),
        CustomId::PrinterList => Box::new(printers::PrinterList::new(theme, desc)),
        CustomId::PrintersAdmin => Box::new(printers::PrintersAdmin::new(theme, desc)),
        CustomId::PrintJobs => Box::new(printers::PrintJobs::new(theme, desc)),
        _ => Box::new(placeholder::Placeholder::new(theme, desc)),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::Instant;

    use nuxut::{Rect, TextSystem, Theme};
    use toml_edit::Value;

    use super::*;
    use crate::block::Env;
    use crate::page::Page;
    use crate::registry::{DETAILS, SETTINGS, Tier, detail_group, fold, search};
    use crate::store::{FileId, KeyPath};

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
        (dir, s)
    }

    /// Kayıt tablosunun yapısal sözleşmesi (§3.5): her ayar bir sayfada erişilebilir.
    fn assert_homes(settings: &'static [SettingDesc], require_details: bool) {
        let (_d, store) = store();
        let theme = Theme::default();
        let mut ids = HashSet::new();
        for s in settings {
            assert!(ids.insert(s.id), "kimlik iki kez: {}", s.id);
            assert!(PageId::ALL.contains(&s.page), "{}: sayfası kenar çubuğunda yok", s.id);
            assert!(!s.mirror_on.contains(&s.page), "{}: ayna birincil sayfayı içeremez", s.id);
            for m in s.mirror_on {
                assert!(PageId::ALL.contains(m), "{}: ayna sayfası kenar çubuğunda yok", s.id);
            }
            // Config'e göre görünür satır (aramanın bulabileceği) sayfa durumundan gösterilebilir.
            if let Some(c) = s.ui_if
                && s.is_visible(&store)
            {
                let mut ui = crate::page_ui::PageUi::default();
                (c.reveal)(&mut ui, &store);
                assert!((c.shown)(&ui, &store), "{}: ui_if.reveal satırı göstermiyor (arama bulamaz)", s.id);
            }
            if let Tier::Detail(name) = s.tier {
                assert!(!require_details || detail_group(name).is_some(), "{}: açılır satır '{name}' DETAILS'te yok", s.id);
                // Bir sayfada en çok bir açılır satır kartı (§3.1).
                let other = settings.iter().find(|o| o.page == s.page && o.detail().is_some_and(|n| n != name));
                assert!(other.is_none(), "{}: sayfada ikinci açılır satır", s.id);
            }
            // Arama başlığıyla bulur (kademe ve sayfa durumu aramayı etkilemez).
            assert!(s.matches(&fold(s.title)), "{}: başlığıyla aranamıyor", s.id);
        }
        // Sayfa blokları kaydın her satırını taşır (açılır satırın içindekiler dahil).
        for page in PageId::ALL {
            let blocks = build_from(&theme, page, &store, settings);
            for s in settings.iter().filter(|s| s.shown_on(page)) {
                let n = blocks.iter().filter(|b| b.contains_id(s.id)).count();
                assert_eq!(n, 1, "{}: {:?} sayfasında {n} blokta", s.id, page);
            }
        }
    }

    #[test]
    fn every_setting_has_a_home() {
        assert_homes(SETTINGS, true);
        // DETAILS kayıtları kullanılıyor ve benzersiz.
        let names: HashSet<_> = DETAILS.iter().map(|d| d.name).collect();
        assert_eq!(names.len(), DETAILS.len());
        for d in DETAILS {
            assert!(SETTINGS.iter().any(|s| s.detail() == Some(d.name)), "kullanılmayan açılır satır: {}", d.name);
        }
        // Arama görünür her ayarı bulur (varsayılan config).
        let (_d, store) = store();
        for s in SETTINGS.iter().filter(|s| s.is_visible(&store)) {
            assert!(search(&store, s.title).iter().any(|r| r.id == s.id), "{}: aramada yok", s.id);
        }
        assert_homes(TEST_SETTINGS, false);
        assert_homes(UI_SETTINGS, false);
    }

    /// t094 taşımaları: arama taşınan her ayarı yeni sayfasında bulur ve o sayfanın blokları
    /// satırı taşır (arama sonucu oraya götürür). Panel satırları panel açıkken görünür.
    #[test]
    fn search_finds_moved_settings_on_their_new_pages() {
        let (_d, mut store) = store();
        let theme = Theme::default();
        for edge in ["right", "left", "top", "bottom"] {
            store.set(FileId::Shell, &KeyPath::parse(&format!("panels.{edge}.enabled")), Some(Value::from(true)));
        }
        let moved = [
            ("frame.enabled", PageId::Bar),
            ("frame.thickness", PageId::Bar),
            ("frame.edges", PageId::Bar),
            ("frame.inner_corners", PageId::Bar),
            ("panels.right.enabled", PageId::Panels),
            ("panels.left.size", PageId::Panels),
            ("panels.top.widgets", PageId::Panels),
            ("panels.bottom.enabled", PageId::Panels),
            ("locale.language", PageId::Region),
            ("locale.formats", PageId::Region),
        ];
        for (id, page) in moved {
            let desc = registry::by_id(id).unwrap();
            let hit = search(&store, desc.title).into_iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{id}: aramada yok"));
            assert_eq!(hit.page, page, "{id}");
            let blocks = build(&theme, page, &store);
            assert!(blocks.iter().any(|b| b.contains_id(id)), "{id}: {page:?} sayfasında blok yok");
        }
        // Hakkında'da artık dil satırı yok; Bölge ve dil'de üstte başlıksız önizleme kartı,
        // altında iki satır tek kartta (karar 9).
        let about = build(&theme, PageId::About, &store);
        assert!(!about.iter().any(|b| b.contains_id("locale.language")));
        let region = build(&theme, PageId::Region, &store);
        assert_eq!(region.len(), 3);
        assert!(region[0].contains_id("locale.preview") && region[0].section() == Some(""));
        assert!(region[1..].iter().all(|b| b.section() == Some("Dil ve biçimler")));
    }

    const DETAIL: &str = "Renkleri tek tek ayarla";

    const fn color(id: &'static str, title: &'static str, tier: Tier) -> SettingDesc {
        SettingDesc {
            id,
            page: PageId::Appearance,
            section: "Renkler",
            title,
            subtitle: "",
            keywords: &[],
            file: FileId::Framework,
            path: id,
            control: Control::Color,
            visible_if: None,
            tier,
            mirror_on: &[],
            ui_if: None,
        }
    }

    /// Test kaydı: bir temel satır, ardından açılır satırda üç renk; biri Bar'a aynalı.
    static TEST_SETTINGS: &[SettingDesc] = &[
        color("colors.primary", "Vurgu", Tier::Basic),
        color("colors.on_primary", "Vurgu üzerindeki yazı", Tier::Detail(DETAIL)),
        color("colors.surface", "Yüzey", Tier::Detail(DETAIL)),
        SettingDesc { mirror_on: &[PageId::Bar], ..color("colors.outline", "Çizgi", Tier::Detail(DETAIL)) },
    ];

    #[test]
    fn detail_settings_collapse_into_one_disclosure() {
        let (_d, store) = store();
        let theme = Theme::default();
        let blocks = build_from(&theme, PageId::Appearance, &store, TEST_SETTINGS);
        assert_eq!(blocks.len(), 2, "temel satır + tek açılır satır");
        assert_eq!(blocks[0].id(), Some("colors.primary"));
        assert_eq!(blocks[1].disclosure(), Some((DETAIL, false)), "kapalı başlar");
        for id in ["colors.on_primary", "colors.surface", "colors.outline"] {
            assert!(blocks[1].contains_id(id));
        }
        assert_eq!(blocks[1].section(), Some("Renkler"), "ilk satırının kartında");
        // Ayna sayfada da aynı kimlik (tek satır).
        let bar = build_from(&theme, PageId::Bar, &store, TEST_SETTINGS);
        assert_eq!(bar.len(), 1);
        assert!(bar[0].contains_id("colors.outline"));
    }

    #[test]
    fn search_reveal_opens_disclosure_and_remembers_it() {
        let (_d, store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let blocks = build_from(&theme, PageId::Appearance, &store, TEST_SETTINGS);
        let mut page = Page::new(&theme, PageId::Appearance, blocks, &store);
        page.set_viewport(Rect::new(0.0, 0.0, 900.0, 600.0));
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        assert!(!page.ui().is_open(DETAIL));
        page.reveal(&mut env, "colors.outline");
        assert!(page.ui().is_open(DETAIL), "arama açılır satırı açtı; sayfa durumunda");
        // Kapatınca durum da kapanır (oturumda hatırlanan bu).
        page.update_ui(&mut env, |ui| {
            ui.set_open(DETAIL, false);
        });
        assert!(!page.ui().is_open(DETAIL));
    }

    /// Seçili widget'a bağlı test satırı (§3.3): arama onu seçerek gösterir.
    static UI_SETTINGS: &[SettingDesc] = &[
        color("colors.primary", "Vurgu", Tier::Basic),
        SettingDesc {
            ui_if: Some(crate::registry::UiCond {
                shown: |ui, _| ui.selected_widget.as_deref() == Some("clock"),
                reveal: |ui, _| ui.selected_widget = Some("clock".into()),
            }),
            ..color("colors.secondary", "Saat rengi", Tier::Basic)
        },
        SettingDesc {
            ui_if: Some(crate::registry::UiCond { shown: |ui, _| ui.is_open("x"), reveal: |ui, _| _ = ui.set_open("x", true) }),
            ..color("colors.on_surface", "Açılır içi", Tier::Detail(DETAIL))
        },
    ];

    #[test]
    fn search_reveal_sets_up_page_state() {
        let (_d, store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let blocks = build_from(&theme, PageId::Appearance, &store, UI_SETTINGS);
        let mut page = Page::with_settings(&theme, PageId::Appearance, blocks, &store, UI_SETTINGS);
        page.set_viewport(Rect::new(0.0, 0.0, 900.0, 600.0));
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        assert!(!UI_SETTINGS[1].ui_visible(page.ui(), &store), "başta gizli");
        page.reveal(&mut env, "colors.secondary");
        assert_eq!(page.ui().selected_widget.as_deref(), Some("clock"), "arama sayfa durumunu kurdu");
        assert!(UI_SETTINGS[1].ui_visible(page.ui(), &store));
        // Açılır satırın içindeki koşullu satır: hem durum hem açılır satır.
        page.reveal(&mut env, "colors.on_surface");
        assert!(page.ui().is_open("x") && page.ui().is_open(DETAIL));
    }

    #[test]
    fn disclosure_reveal_offset_and_dirty_summary() {
        let (_d, mut store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let mut blocks = build_from(&theme, PageId::Appearance, &store, TEST_SETTINGS);
        let mut d = blocks.remove(1);
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        let closed = d.height(&mut env, 700.0);
        d.place(&mut env, Rect::new(0.0, 0.0, 700.0, closed));
        let (first, own) = d.reveal(&mut env, "colors.on_primary").unwrap();
        let (third, _) = d.reveal(&mut env, "colors.outline").unwrap();
        assert!(own, "satırı açılır satır kendisi parlatır");
        assert!(first >= closed - 0.5, "ilk satır başlığın altında: {first} / {closed}");
        assert!(third > first, "sıra korunur");
        assert_eq!(d.disclosure(), Some((DETAIL, true)));
        let open = d.height(&mut env, 700.0);
        assert!(open > closed + 100.0, "açıkken üç satır: {open} / {closed}");
        assert!(d.reveal(&mut env, "colors.primary").is_none(), "içinde olmayan kimlik");

        // Varsayılandan farklı bir değer: özet ve rozet.
        let disc = disclosure::summary_text(3, 0, crate::registry::Counted::Settings);
        assert_eq!(disc, "3 ayar");
        store.set(FileId::Framework, &KeyPath::parse("colors.surface"), Some(Value::from("#123456")));
        assert!(TEST_SETTINGS[2].differs_from_default(&store));
        assert!(!TEST_SETTINGS[1].differs_from_default(&store));
    }

    #[test]
    fn disclosure_click_and_keyboard() {
        use nuxut::Point;
        use nuxut::ui::input::{Button, Key, KeyEvent, KeyState, Modifiers, PointerEvent, PointerKind};
        let (_d, store) = store();
        let theme = Theme::default();
        let mut text = TextSystem::new();
        let mut actions = Vec::new();
        let now = Instant::now();
        let mut d = build_from(&theme, PageId::Appearance, &store, TEST_SETTINGS).remove(1);
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
        let h = d.height(&mut env, 700.0);
        d.place(&mut env, Rect::new(0.0, 0.0, 700.0, h));
        let p = Point::new(100.0, h * 0.5);
        for kind in [PointerKind::Motion, PointerKind::Press(Button::Left), PointerKind::Release(Button::Left)] {
            d.pointer(&mut env, &PointerEvent::new(p, kind));
        }
        assert_eq!(d.disclosure(), Some((DETAIL, true)), "başlığa tıklama açar");
        let key = |k| KeyEvent::new(k, KeyState::Pressed, None, Modifiers::default());
        d.set_focus(true, true, now);
        assert!(d.key(&mut env, &key(Key::Enter)));
        assert_eq!(d.disclosure(), Some((DETAIL, false)), "Enter kapatır");
        assert!(!d.focus_step(true, now), "kapalıyken Tab sayfaya");
        assert!(d.key(&mut env, &key(Key::Space)));
        let full = d.height(&mut env, 700.0);
        // Yay oturmadan yerleşim yarım; testte anında açılmış say.
        let _ = d.reveal(&mut env, "colors.on_primary");
        let full = d.height(&mut env, 700.0).max(full);
        d.place(&mut env, Rect::new(0.0, 0.0, 700.0, full));
        assert!(d.focus_step(true, now), "açıkken Tab ilk satıra");
        assert!(d.focus_step(true, now));
        assert!(d.focus_step(true, now));
        assert!(!d.focus_step(true, now), "son satırdan sonra sayfaya");
    }
}
