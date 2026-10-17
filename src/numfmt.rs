//! Ayarlar'daki sayı gösterimlerinin locale'i (kaydırıcı değerleri, yenileme hızı, eğri
//! özetleri): ondalık ayırıcı sabit `,` değil, etkin biçim locale'inin `LC_NUMERIC`'i.
//!
//! - Etkin locale `framework.toml [locale] formats`'tan, framework'ün sayı çözümüyle
//!   ([`LanguagePrefs::resolve`] → `numeric()`): açık ve tanınan ad o; `auto` ya da tanınmayan
//!   ad → süreç ortamı (`LC_ALL` > `LC_NUMERIC` > `LANG`). Yerel kopya kural yok.
//! - Süreç geneli tek değer: açılışta ve `App::locale_changed`'de [`configure`] ile kurulur;
//!   gösterenler değeri o anda [`current`] ile okur (kurulmadıysa ortamdan çözülür).
//! - Biçimleyici framework'ün [`Locale::format_number_with`]'idir (gruplama kapalı: ayarlar
//!   değerleri küçük ve kısa; Türkçe sistemde görünüm eskisiyle aynı kalır).

use std::sync::RwLock;

use nuxut::i18n::{LanguagePrefs, LocaleEnv, LocaleSetting};
use nuxut::locale::{Locale, NumberOptions};

use crate::registry::by_id;
use crate::store::Store;

static CURRENT: RwLock<Option<Locale>> = RwLock::new(None);

/// `[locale] formats` ayarından sayı locale'i (framework çözümü; ortam `env`).
pub fn resolve(formats: &LocaleSetting, env: &LocaleEnv) -> Locale {
    LanguagePrefs::resolve(&LocaleSetting::AUTO, formats, env).numeric().clone()
}

/// Store'daki `[locale] formats` ile süreç geneli sayı locale'ini kurar.
pub fn configure(store: &Store) {
    let formats = by_id("locale.formats").map_or(LocaleSetting::AUTO, |d| crate::pages::locale::setting(d, store));
    let locale = resolve(&formats, &LocaleEnv::from_process());
    *CURRENT.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(locale);
}

/// Etkin sayı locale'i; [`configure`] çağrılmadıysa ortamdan.
pub fn current() -> Locale {
    if let Some(l) = CURRENT.read().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref() {
        return l.clone();
    }
    resolve(&LocaleSetting::AUTO, &LocaleEnv::from_process())
}

/// En çok `max_fraction` kesir basamağı (yuvarlanır, sondaki sıfırlar atılır), gruplamasız;
/// `-0` yazılmaz.
pub fn format_in(locale: &Locale, value: f64, max_fraction: usize) -> String {
    let mut o = NumberOptions::default();
    o.max_fraction_digits = Some(max_fraction);
    o.grouping = false;
    locale.format_number_with(value, &o)
}

/// En az `min_fraction`, en çok `max_fraction` kesir basamağı (sondaki sıfırlar en aza dek
/// kalır: `1,0`), gruplamasız.
pub fn format_range_in(locale: &Locale, value: f64, min_fraction: usize, max_fraction: usize) -> String {
    let mut o = NumberOptions::default();
    o.min_fraction_digits = min_fraction.min(max_fraction);
    o.max_fraction_digits = Some(max_fraction);
    o.grouping = false;
    locale.format_number_with(value, &o)
}

/// [`format_in`] etkin locale'le.
pub fn format(value: f64, max_fraction: usize) -> String {
    format_in(&current(), value, max_fraction)
}

/// Kesir `fraction`'ı (0.05 = yüzde beş) yüzde olarak: Ayarlar'daki TEK yüzde biçimi.
/// İşaret ve konumu framework'ün [`NumberOptions::percent`]'inden (locale'e göre; burada
/// yerel kural yok). En çok `max_fraction` kesir basamağı, gruplamasız.
pub fn percent_in(locale: &Locale, fraction: f64, max_fraction: usize) -> String {
    let mut o = NumberOptions::default();
    o.max_fraction_digits = Some(max_fraction);
    o.grouping = false;
    o.percent = true;
    locale.format_number_with(fraction, &o)
}

/// [`percent_in`] etkin locale'le.
pub fn percent(fraction: f64, max_fraction: usize) -> String {
    percent_in(&current(), fraction, max_fraction)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc(name: &str) -> Locale {
        Locale::parse(name).unwrap()
    }

    #[test]
    fn separator_follows_locale() {
        assert_eq!(format_in(&loc("tr_TR"), 1.25, 2), "1,25");
        assert_eq!(format_in(&loc("en_US"), 1.25, 2), "1.25");
        assert_eq!(format_in(&loc("de_DE"), 1.25, 2), "1,25");
        // Sondaki sıfırlar atılır, gruplama yok, -0 yok.
        assert_eq!(format_in(&loc("tr_TR"), 2.50, 2), "2,5");
        assert_eq!(format_in(&loc("tr_TR"), 4.0, 2), "4");
        assert_eq!(format_in(&loc("tr_TR"), 12345.0, 0), "12345");
        assert_eq!(format_in(&loc("en_US"), -0.001, 2), "0");
    }

    /// Yüzde framework'ün biçimiyle birebir (işaret konumu orada); sayı kısmı locale ayırıcılı,
    /// yuvarlanmış, gruplamasız.
    #[test]
    fn percent_is_frameworks_percent() {
        for name in ["tr_TR", "en_US", "de_DE", "fr_FR"] {
            let l = loc(name);
            let mut o = NumberOptions::default();
            o.max_fraction_digits = Some(0);
            o.grouping = false;
            o.percent = true;
            assert_eq!(percent_in(&l, 0.05, 0), l.format_number_with(0.05, &o), "{name}");
            assert!(percent_in(&l, 0.05, 0).contains('5') && percent_in(&l, 0.05, 0).contains('%'), "{name}");
        }
        assert!(percent_in(&loc("tr_TR"), 0.125, 1).contains("12,5"));
        assert!(percent_in(&loc("en_US"), 0.125, 1).contains("12.5"));
        assert!(percent_in(&loc("tr_TR"), 12.0, 0).contains("1200"), "gruplama yok");
        assert!(!percent_in(&loc("tr_TR"), 0.05, 0).contains(' '), "sayı ile işaret arasında boşluk yok (tr)");
        assert_eq!(percent_in(&loc("tr_TR"), 0.05, 0), "%5", "tr: işaret önde");
        assert_eq!(percent_in(&loc("en_US"), 0.05, 0), "5%", "en: işaret sonda");
    }

    #[test]
    fn resolve_uses_formats_then_env() {
        let env = LocaleEnv { lc_numeric: Some("tr_TR.UTF-8".into()), ..LocaleEnv::default() };
        assert_eq!(resolve(&LocaleSetting::AUTO, &env).name(), "tr_TR");
        assert_eq!(resolve(&LocaleSetting::parse("en_US.UTF-8"), &env).name(), "en_US");
        // Tanınmayan ad ortama düşer.
        assert_eq!(resolve(&LocaleSetting::parse("xx_YY"), &env).name(), "tr_TR");
    }
}
