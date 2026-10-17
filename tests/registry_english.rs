//! Kayıt metinleri İngilizce katalogla (`docs/design/i18n.md` "Kayıt"): başlıklar, bölümler,
//! sayfalar ve seçenekler etkin dilde; arama hem çeviride hem kaynak Türkçede hem ayar yolunda
//! bulur. Süreç geneli kataloğu İngilizceye çevirdiği için ayrı test ikilisinde durur.

use std::sync::Arc;

use nuxut::i18n::{Catalog, LanguagePrefs};
use nuxut_settings::registry::{self, EDGE_CLICK, PageId, SETTINGS, by_id, choice_label, search, section_title};
use nuxut_settings::store::Store;

/// Türkçeye özgü harfler: İngilizce metinde bulunmamalı (çevrilmemiş kaynak metin işareti).
fn has_turkish_letters(s: &str) -> bool {
    s.chars().any(|c| "çğıöşüÇĞİÖŞÜ".contains(c))
}

#[test]
fn registry_follows_the_english_catalog() {
    let prefs = LanguagePrefs::fixed(&["en"]);
    nuxut::i18n::set_current(Arc::new(Catalog::load(&[nuxut_settings::i18n::source()], &prefs)));

    // Satır metinleri, bölümler, sayfalar, seçenekler İngilizce.
    let mut turkish = Vec::new();
    for d in SETTINGS {
        for s in [d.label(), d.description(), d.section_title()] {
            if has_turkish_letters(s) {
                turkish.push(format!("{}: {s}", d.id));
            }
        }
    }
    for g in registry::DETAILS {
        for s in [g.label(), g.description()] {
            if has_turkish_letters(s) {
                turkish.push(format!("açılır satır {}: {s}", g.key));
            }
        }
    }
    for p in PageId::ALL {
        if has_turkish_letters(p.title()) {
            turkish.push(format!("sayfa {}: {}", p.slug(), p.title()));
        }
    }
    assert!(turkish.is_empty(), "İngilizce katalogda Türkçe kalan kayıt metinleri:\n{}", turkish.join("\n"));

    let colors = by_id("colors.mode").expect("colors.mode");
    assert_ne!(colors.label(), colors.title, "başlık çevrilmiş");
    assert_ne!(PageId::DefaultApps.title(), PageId::DefaultApps.source_title());
    assert_ne!(section_title("Renkler"), "Renkler");
    assert_ne!(choice_label(EDGE_CLICK, "panel:clock"), "Takvimi aç");
    assert_eq!(PageId::from_slug(PageId::DefaultApps.title()), Some(PageId::DefaultApps), "İngilizce sayfa adı da komut adı");
    assert_eq!(PageId::from_slug("Varsayılan uygulamalar"), Some(PageId::DefaultApps), "Türkçe ad da çalışır");

    // Arama: çeviri, kaynak Türkçe ve ayar yolu.
    let dir = tempfile::tempdir().expect("geçici dizin");
    let store = Store::open(dir.path().join("f.toml"), dir.path().join("s.toml"), dir.path().join("p.toml"));
    let finds = |q: &str, id: &str| search(&store, q).iter().any(|d| d.id == id);
    let wallpaper = by_id("wallpaper").expect("wallpaper");
    assert!(finds(wallpaper.label(), "wallpaper"), "İngilizce başlıkla");
    assert!(finds("duvar kağıdı", "wallpaper"), "Türkçe kaynakla");
    assert!(finds("bar.size", "bar.size"), "ayar yoluyla");
    assert!(registry::fold(PageId::Motion.title()) != registry::fold(PageId::Motion.source_title()));
}
