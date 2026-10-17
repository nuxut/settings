//! i18n kapıları (ADR-0010 §4): katalog (tr/en tam, yorum, değişken, [max], önek) ve gömülü
//! kullanıcı metni bekçisi (`i18n-allow.txt` geçici listesi aşama B'de küçülür).

#[test]
fn i18n_catalog() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/locales");
    nuxut::i18n::check::catalog_embedded(&nuxut::i18n::embed!("settings"), dir, "settings").assert_ok();
}

#[test]
fn i18n_source_guard() {
    nuxut::i18n::check::source_guard(env!("CARGO_MANIFEST_DIR")).assert_ok();
}

/// Marka sloganı her dilde çevrilir (kullanıcı kararı, 2026-10-13); Hakkında ve Hoş geldin
/// katalogdan okur. Hiçbir dilde boş kalmaz.
#[test]
fn brand_slogan_is_present_in_every_language() {
    use nuxut::i18n::{Catalog, LanguagePrefs};
    let src = nuxut::i18n::embed!("settings");
    for lang in ["tr", "en"] {
        let c = Catalog::load(&[src], &LanguagePrefs::fixed(&[lang]));
        assert!(c.has("settings-brand-slogan"), "{lang}");
        assert!(!c.text("settings-brand-slogan").trim().is_empty(), "{lang}");
    }
}
