//! Ayarlar'ın Fluent kataloğu (ADR-0010; `locales/<dil>.ftl`, anahtarlar `settings-…`).
//!
//! Metin `nuxut::i18n::t!("settings-…")` ile istenir; dil değişince süreç geneli katalog
//! yenilenir, metni önbelleğe alan bileşen yeniden kurulmalıdır.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};

use nuxut::i18n::{Catalog, LanguagePrefs, Source};

/// Gömülü `settings` kaynağı.
pub fn source() -> Source {
    nuxut::i18n::embed!("settings")
}

/// [`text_or`] önbelleği: hangi katalog için doldurulduğu ve anahtar → metin.
struct TextCache {
    catalog: Arc<Catalog>,
    texts: HashMap<String, &'static str>,
}

static TEXTS: Mutex<Option<TextCache>> = Mutex::new(None);

/// Kayıt tablosu gibi `&'static str` bekleyen yerler için katalog metni: `key` etkin dilde,
/// katalogda yoksa `source` (kaynak Türkçe metin). Dil başına önbellekli; katalog değişince
/// (dil değişimi) önbellek boşalır, eski dilin metinleri sızar — değişim başına bir kez, yalnızca
/// kaynaktan farklı değerler (Türkçede hiç). `source` ile aynı değer kaynağın kendisi döner.
pub fn text_or(key: &str, source: &'static str) -> &'static str {
    let catalog = nuxut::i18n::current();
    // Zehirlenme: önbellek yalnızca HashMap; panik yarım durum bırakmaz.
    let mut guard = TEXTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if guard.as_ref().is_none_or(|c| !Arc::ptr_eq(&c.catalog, &catalog)) {
        *guard = Some(TextCache { catalog: catalog.clone(), texts: HashMap::new() });
    }
    let Some(cache) = guard.as_mut() else { return source };
    if let Some(text) = cache.texts.get(key) {
        return text;
    }
    let text: &'static str = match catalog.has(key).then(|| catalog.text(key)) {
        Some(v) if v != source => Box::leak(v.into_owned().into_boxed_str()),
        _ => source,
    };
    cache.texts.insert(key.to_owned(), text);
    text
}

/// Testler için süreç geneli kataloğu Türkçeye, sayı biçimini tr_TR'ye sabitler ("%45", "1,5";
/// ortam ve kullanıcı çeviri dizini okunmaz). Metin karşılaştıran her test önce çağırır; tekrar
/// çağrı zararsız. Süreç genelinde tek kurucu: başka bir test kataloğu kurulursa sıra bozar.
pub fn use_turkish_for_tests() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let numeric = nuxut::locale::Locale::parse("tr_TR").expect("tr_TR");
        let prefs = LanguagePrefs::fixed(&["tr"]).with_numeric(numeric);
        nuxut::i18n::set_current(Arc::new(Catalog::load(&[source(), nuxut_wallpapers::i18n_source()], &prefs)));
    });
}
