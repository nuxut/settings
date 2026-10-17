//! t094 W2 (Paneller) ve W4 (metin sayfaları: Yazı, Hareket, Pencereler, Bildirimler, Ses)
//! yerleşim sözleşmeleri: kartlar, açılır satırlar, ayna kısayol, tek bağlantı satırı.

use std::time::Instant;

use nuxut::{Rect, TextSystem, Theme};
use toml_edit::Value;

use super::build;
use crate::block::{Block, Env};
use crate::page::Page;
use crate::registry::{self, MotionRole, PageId, SETTINGS, search};
use crate::store::{FileId, KeyPath, Store};

fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let s = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
    (dir, s)
}

/// Görünür blokların bölümleri, ardışık tekrarlar tek (= kart sırası).
fn cards(blocks: &[Box<dyn Block>], store: &Store) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for b in blocks.iter().filter(|b| b.visible(store)) {
        let s = b.section().unwrap_or("").to_owned();
        if out.last() != Some(&s) {
            out.push(s);
        }
    }
    out
}

const PAGES: [PageId; 6] = [PageId::Panels, PageId::Fonts, PageId::Motion, PageId::Windows, PageId::Notifications, PageId::Sound];

#[test]
fn text_pages_have_merged_cards() {
    let (_d, store) = store();
    let theme = Theme::default();
    let c = |p| cards(&build(&theme, p, &store), &store);
    assert_eq!(c(PageId::Fonts), ["", "Yazı tipleri", "İkonlar"]);
    assert_eq!(c(PageId::Motion), ["", "Genel"]);
    assert_eq!(c(PageId::Windows), ["Yerleşim", "Görünüm", "Gelişmiş"], "5 kart → 3");
    assert_eq!(c(PageId::Notifications), ["Genel", "Davranış", "Uygulamalar"]);
    assert_eq!(c(PageId::Sound), ["Ses düzeyi", "Medya tuşları"], "3 kart → 2; ilk kart sayfa adını tekrarlamaz");
    // Açılır satırlar kapalı başlar ve kartlarının sonundadır.
    for (page, name) in [(PageId::Fonts, "Boyut oranları ve ikon yazı tipi"), (PageId::Motion, "Hareketi tek tek ayarla"), (PageId::Windows, "Masaüstüyle ortaklık")] {
        let blocks = build(&theme, page, &store);
        let i = blocks.iter().position(|b| b.disclosure().is_some()).unwrap_or_else(|| panic!("{page:?}: açılır satır yok"));
        assert_eq!(blocks[i].disclosure(), Some((name, false)));
        let sec = blocks[i].section();
        assert!(blocks[i + 1..].iter().all(|b| b.section() != sec), "{page:?}: açılır satır kartın sonunda değil");
    }
}

#[test]
fn search_opens_closed_disclosures_on_text_pages() {
    let (_d, store) = store();
    let theme = Theme::default();
    let mut text = TextSystem::new();
    let mut actions = Vec::new();
    let now = Instant::now();
    let mut n = 0;
    for page in PAGES {
        for desc in SETTINGS.iter().filter(|d| d.page == page && d.detail().is_some() && d.is_visible(&store)) {
            let name = desc.detail().unwrap();
            assert!(search(&store, desc.title).iter().any(|r| r.id == desc.id), "{}: aramada yok", desc.id);
            let mut p = Page::new(&theme, page, build(&theme, page, &store), &store);
            p.set_viewport(Rect::new(0.0, 0.0, 900.0, 600.0));
            let mut env = Env { theme: &theme, store: &store, text: &mut text, now, actions: &mut actions };
            assert!(!p.ui().is_open(name));
            p.reveal(&mut env, desc.id);
            assert!(p.ui().is_open(name), "{}: arama '{name}' açılır satırını açmadı", desc.id);
            n += 1;
        }
    }
    assert!(n >= 20, "açılır satırdaki ayarlar sınandı: {n}");
}

#[test]
fn panel_cards_hold_their_shortcut_mirror() {
    let (_d, mut store) = store();
    let theme = Theme::default();
    for edge in ["right", "left", "top", "bottom"] {
        store.set(FileId::Shell, &KeyPath::parse(&format!("panels.{edge}.enabled")), Some(Value::from(true)));
    }
    let blocks = build(&theme, PageId::Panels, &store);
    assert_eq!(cards(&blocks, &store), ["Sağ panel", "Sol panel", "Üst panel", "Alt panel"], "ayna kendi kartını açmaz");
    let ids: Vec<_> = blocks.iter().filter(|b| b.visible(&store)).filter_map(|b| b.id()).collect();
    assert_eq!(&ids[..4], ["panels.right.enabled", "keybinds.panel_right", "panels.right.size", "panels.right.widgets"]);
    assert_eq!(&ids[4..8], ["panels.left.enabled", "keybinds.panel_left", "panels.left.size", "panels.left.widgets"]);
    let mirror = blocks.iter().find(|b| b.id() == Some("keybinds.panel_right")).unwrap();
    assert_eq!(mirror.section(), Some("Sağ panel"));
    // Birincil yer Kısayollar: orada kendi kartında, tek arama sonucu birincil sayfaya.
    let binds = build(&theme, PageId::Keybinds, &store);
    let row = binds.iter().find(|b| b.id() == Some("keybinds.panel_right")).unwrap();
    assert_eq!(row.section(), Some("Bar ve paneller"));
    assert_eq!(cards(&binds, &store), ["Genel", "Ses", "Parlaklık", "Medya", "Bar ve paneller", "Ekran görüntüsü", "Pano", "Büyüteç", "Emoji", "Ekran klavyesi", "Uygulamalar", "Oturum", "Pencereler", "Odak ve taşıma", "Boyutlandırma", "Çalışma alanları", "Fare"]);
    let hits: Vec<_> = search(&store, "Sağ paneli aç").into_iter().filter(|r| r.id == "keybinds.panel_right").collect();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].page, PageId::Keybinds);
    // Kapalı panel tek satır: kısayol, genişlik, içerik ve ipucu gizli.
    store.set(FileId::Shell, &KeyPath::parse("panels.left.enabled"), Some(Value::from(false)));
    let left: Vec<_> = blocks.iter().filter(|b| b.visible(&store) && b.section() == Some("Sol panel")).filter_map(|b| b.id()).collect();
    assert_eq!(left, ["panels.left.enabled"]);
}

#[test]
fn window_look_links_are_one_row_for_three_ids() {
    let (_d, store) = store();
    let theme = Theme::default();
    let blocks = build(&theme, PageId::Windows, &store);
    let holders: Vec<_> = ["windows.corners", "windows.shadow", "windows.motion"]
        .iter()
        .map(|id| blocks.iter().position(|b| b.contains_id(id)).unwrap_or_else(|| panic!("{id}: satır yok")))
        .collect();
    assert!(holders.iter().all(|&i| i == holders[0]), "üç kimlik aynı satırda");
    for (q, id) in [("pencere gölgesi", "windows.shadow"), ("pencere hareketleri", "windows.motion"), ("köşe, gölge", "windows.corners")] {
        let hit = search(&store, q).into_iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{q}: aramada yok"));
        assert_eq!(hit.page, PageId::Windows);
    }
    // Pencereler tek bağlantı satırının iki hedefi var olan ayarlar.
    let registry::Control::Custom(registry::CustomId::Links(targets)) = registry::by_id("windows.corners").unwrap().control else { panic!() };
    assert!(targets.iter().all(|(_, t)| registry::by_id(t).is_some()));
}

#[test]
fn motion_roles_use_plain_names() {
    assert_eq!(MotionRole::ALL.map(MotionRole::title), ["Paneller ve pencereler", "Menüler ve küçük öğeler", "Solma ve renk"]);
    for role in MotionRole::ALL {
        let d = registry::by_id(&format!("motion.{}", role.key())).unwrap();
        assert_eq!((d.title, d.section), (role.title(), role.title()), "arama sonucunda rol ayırt edilir");
    }
}

/// §8: açıklama en çok bir satır (720 sütununda ~60 karakter), hitapsız (siz/sen eki yok).
#[test]
fn texts_are_short_and_address_free() {
    // Sözcük sonundaki iyelik/kişi ekleri ("dosyanız", "dosyanıza", "kendiniz") ve zamirler.
    let suffixes = ["nız", "niz", "nuz", "nüz", "nıza", "nize", "nuza", "nüze", "nızı", "nizi"];
    let pronouns = ["siz", "sizin", "sen", "senin"];
    let formal = |t: &str| {
        t.split(|c: char| !c.is_alphabetic()).filter(|w| !w.is_empty()).any(|w| {
            let w = w.to_lowercase();
            pronouns.contains(&w.as_str()) || suffixes.iter().any(|s| w.ends_with(s))
        })
    };
    assert!(formal("Ayar dosyanıza eklenir") && formal("kendiniz") && !formal("Önizleme"));
    for d in SETTINGS.iter().filter(|d| PAGES.contains(&d.page) || d.page == PageId::Keybinds) {
        if d.control != registry::Control::Custom(registry::CustomId::Notice) {
            assert!(d.subtitle.chars().count() <= 60, "{}: açıklama uzun ({})", d.id, d.subtitle.chars().count());
        }
        for t in [d.title, d.subtitle] {
            assert!(!formal(t), "{}: hitaplı metin: {t}", d.id);
        }
    }
}

/// Bar kısayolu Bar'da "Konum ve boyut" kartının son satırı (ayna); birincil yer Kısayollar.
#[test]
fn bar_toggle_mirror_ends_position_card() {
    let (_d, store) = store();
    let theme = Theme::default();
    let blocks = build(&theme, PageId::Bar, &store);
    let card: Vec<_> = blocks.iter().filter(|b| b.section() == Some("Konum ve boyut")).filter_map(|b| b.id()).collect();
    assert_eq!(card.last(), Some(&"keybinds.bar_toggle"), "{card:?}");
    assert_eq!(card[card.len() - 2], "bar.spacing");
    assert_eq!(registry::by_id("keybinds.bar_toggle").unwrap().page, PageId::Keybinds);
    // Kısayol eylem sırası shell'inkiyle aynı: kabuk eylemlerinden sonra temel katman (ADR-0006), en sonda fare.
    assert_eq!(registry::keybind_rows().last().map(|d| d.id), Some("keybinds.workspace_scroll_prev"));
}

/// Erişilebilirlik (2026-10-16): Görme kartı Görünüm/Yazı/Hareket satırlarının aynası
/// (`PageId::mirror_home`), sonra Büyüteç ve Ekran klavyesi; kısayollar yalnızca gösterilir,
/// birincil yerleri Kısayollar'da.
#[test]
fn accessibility_page_cards_and_mirrors() {
    crate::i18n::use_turkish_for_tests();
    let (_d, mut store) = store();
    // İmleç boyutu birleşik görünüm açıkken görünür.
    store.set(FileId::Framework, &KeyPath::parse("desktop.unified"), Some(Value::from(true)));
    let theme = Theme::default();
    let blocks = build(&theme, PageId::Accessibility, &store);
    assert_eq!(cards(&blocks, &store), ["Görme", "Büyüteç", "Ekran klavyesi"]);
    let ids: Vec<_> = blocks.iter().filter(|b| b.visible(&store) && b.section() == Some("Görme")).filter_map(|b| b.id()).collect();
    for id in ["desktop.cursor_size", "typography.size", "motion.enabled"] {
        assert!(ids.contains(&id), "{id} Görme kartında yok: {ids:?}");
    }
    // Aynalar birincil sayfayı değiştirmez: arama birincil yere gider.
    for (q, id, page) in [("hareketi azalt", "motion.enabled", PageId::Motion), ("büyüteç", "magnifier.live", PageId::Accessibility), ("ekran klavyesi", "osk.live", PageId::Accessibility)] {
        let hit = search(&store, q).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{q}"));
        assert_eq!(hit.page, page, "{q}");
    }
    // Kısayol yönlendirmesi tuşları Kısayollar'daki değerden gösterir; atanmamış eylem adıyla.
    let keys = registry::by_id("magnifier.keys").unwrap();
    let text = keys.subtitle_in(&store);
    assert!(text.contains("Yakınlaştır") && text.contains("Alt"), "{text}");
    store.set(FileId::Shell, &KeyPath::parse("keybinds.magnifier_out"), Some(Value::from("")));
    assert!(keys.subtitle_in(&store).contains("Atanmamış"));
    // Ekran klavyesi kısayolunun yönlendirmesi yalnızca Kısayollar'da satırı varken görünür.
    assert_eq!(registry::by_id("osk.keys").unwrap().is_visible(&store), registry::by_id("keybinds.osk_toggle").is_some());
}
