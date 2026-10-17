//! XKB klavye düzeni kataloğu: `xkeyboard-config`'in `rules/evdev.xml` dosyasından düzenler,
//! varyantlar ve düzen değiştirme (`grp:`) seçenekleri. UI'dan bağımsızdır; Ayarlar'ın
//! Klavye sayfası `kb_layout`/`kb_variant`/`kb_options` değerlerini buradan seçtirir.
//!
//! Dosya sistem dosyasıdır ama güvenilmez girdi gibi ele alınır: boyut, sayı ve ad
//! sınırları aşağıda; sınırı aşan ya da config'e yazılamayacak adlar sessizce atlanır.

use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::registry::fold;

/// Sistem kural dosyası.
pub const EVDEV_XML: &str = "/usr/share/X11/xkb/rules/evdev.xml";

/// Dosya boyutu üst sınırı (gerçek dosya ~250 KiB).
const MAX_FILE: u64 = 4 * 1024 * 1024;
const MAX_LAYOUTS: usize = 2000;
const MAX_VARIANTS: usize = 200;
const MAX_OPTIONS: usize = 2000;
const MAX_DESC: usize = 128;
const MAX_LANGS: usize = 16;

/// Düzen varyantı (`tr` düzeninin `f` varyantı).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub description: String,
    /// `iso639Id` kodları (ISO 639-2, 3 harf).
    pub languages: Vec<String>,
    pub exotic: bool,
}

/// Klavye düzeni (`kb_layout` değeri = `name`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub name: String,
    pub description: String,
    /// Gösterge kısaltması (`shortDescription`); yoksa boş.
    pub short: String,
    pub languages: Vec<String>,
    pub exotic: bool,
    pub variants: Vec<Variant>,
}

/// `kb_options` seçeneği (`grp:alt_shift_toggle`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XkbOption {
    pub name: String,
    pub description: String,
}

/// Ayrıştırılmış katalog.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XkbCatalog {
    pub layouts: Vec<Layout>,
    /// `grp` grubundaki (düzen değiştirme) seçenekler.
    pub switch_options: Vec<XkbOption>,
}

/// Config'e (`kb_layout`/`kb_variant`/`kb_options`) yazılabilir ad: harf, rakam ve `_()+:-`;
/// 1..=64 bayt. Virgül, boşluk ve kabuk karakterleri yok.
pub fn valid_name(s: &str) -> bool {
    (1..=64).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'(' | b')' | b'+' | b':' | b'-'))
}

/// Sistem dosyasını okuyup ayrıştırır.
pub fn load_system() -> Result<XkbCatalog, String> {
    load_file(Path::new(EVDEV_XML))
}

/// Dosyadan yükler: normal dosya ve ≤ 4 MiB değilse hata.
pub fn load_file(path: &Path) -> Result<XkbCatalog, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let meta = file.metadata().map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.len() > MAX_FILE {
        return Err(format!("{}: not a regular file or too large", path.display()));
    }
    let mut text = String::new();
    // `take`: dosya okurken büyürse de sınır tutar.
    file.take(MAX_FILE + 1).read_to_string(&mut text).map_err(|e| format!("{}: {e}", path.display()))?;
    if text.len() as u64 > MAX_FILE {
        return Err(format!("{}: too large", path.display()));
    }
    parse(&text)
}

fn child<'a, 'i>(n: roxmltree::Node<'a, 'i>, tag: &str) -> Option<roxmltree::Node<'a, 'i>> {
    n.children().find(|c| c.has_tag_name(tag))
}

fn text_of(n: roxmltree::Node<'_, '_>, tag: &str) -> String {
    child(n, tag).and_then(|c| c.text()).map(str::trim).unwrap_or_default().to_owned()
}

fn clip(s: &str) -> String {
    s.chars().take(MAX_DESC).collect()
}

/// `exotic="true"` ya da `popularity="exotic"` (xkeyboard-config'in eski/yeni işareti);
/// `configItem` ya da sahibi üzerinde.
fn is_exotic(owner: roxmltree::Node<'_, '_>, item: roxmltree::Node<'_, '_>) -> bool {
    [owner, item].iter().any(|n| n.attribute("exotic") == Some("true") || n.attribute("popularity") == Some("exotic"))
}

fn languages(item: roxmltree::Node<'_, '_>) -> Vec<String> {
    let Some(list) = child(item, "languageList") else { return Vec::new() };
    list.children()
        .filter(|c| c.has_tag_name("iso639Id"))
        .filter_map(|c| c.text())
        .map(str::trim)
        .filter(|s| (2..=3).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_alphabetic()))
        .take(MAX_LANGS)
        .map(str::to_owned)
        .collect()
}

/// `evdev.xml` metnini ayrıştırır.
pub fn parse(xml: &str) -> Result<XkbCatalog, String> {
    // DOCTYPE (xkb.dtd) var; roxmltree harici DTD yüklemez, varlık genişletmesi sınırlıdır.
    let opts = roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() };
    let doc = roxmltree::Document::parse_with_options(xml, opts).map_err(|e| format!("evdev.xml: {e}"))?;
    let root = doc.root_element();
    let mut cat = XkbCatalog::default();

    if let Some(list) = child(root, "layoutList") {
        for l in list.children().filter(|c| c.has_tag_name("layout")) {
            if cat.layouts.len() >= MAX_LAYOUTS {
                break;
            }
            let Some(item) = child(l, "configItem") else { continue };
            let name = text_of(item, "name");
            if !valid_name(&name) {
                continue;
            }
            let mut variants = Vec::new();
            if let Some(vl) = child(l, "variantList") {
                for v in vl.children().filter(|c| c.has_tag_name("variant")) {
                    if variants.len() >= MAX_VARIANTS {
                        break;
                    }
                    let Some(vi) = child(v, "configItem") else { continue };
                    let vname = text_of(vi, "name");
                    if !valid_name(&vname) {
                        continue;
                    }
                    variants.push(Variant {
                        name: vname,
                        description: clip(&text_of(vi, "description")),
                        languages: languages(vi),
                        exotic: is_exotic(v, vi),
                    });
                }
            }
            cat.layouts.push(Layout {
                name,
                description: clip(&text_of(item, "description")),
                short: clip(&text_of(item, "shortDescription")),
                languages: languages(item),
                exotic: is_exotic(l, item),
                variants,
            });
        }
    }

    if let Some(list) = child(root, "optionList") {
        let grp = list
            .children()
            .filter(|c| c.has_tag_name("group"))
            .find(|g| child(*g, "configItem").map(|i| text_of(i, "name")).as_deref() == Some("grp"));
        if let Some(g) = grp {
            for o in g.children().filter(|c| c.has_tag_name("option")) {
                if cat.switch_options.len() >= MAX_OPTIONS {
                    break;
                }
                let Some(oi) = child(o, "configItem") else { continue };
                let name = text_of(oi, "name");
                if valid_name(&name) {
                    cat.switch_options.push(XkbOption { name, description: clip(&text_of(oi, "description")) });
                }
            }
        }
    }
    Ok(cat)
}

/// ISO 639-2 (3 harf) → ISO 639-1 (2 harf): `nuxut-locale` `Locale::parse` 2 harfli dil ister.
/// Yalnızca klavye düzenlerinde yaygın diller; tabloda olmayan kod `Locale::parse`'a olduğu
/// gibi verilir (`fil` gibi 3 harfli adlar orada zaten var), o da bilmezse `None`.
fn iso1(code: &str) -> &str {
    match code {
        "ara" => "ar", "bul" => "bg", "cat" => "ca", "ces" | "cze" => "cs", "dan" => "da",
        "deu" | "ger" => "de", "ell" | "gre" => "el", "eng" => "en", "spa" => "es", "est" => "et",
        "eus" | "baq" => "eu", "fas" | "per" => "fa", "fin" => "fi", "fra" | "fre" => "fr",
        "heb" => "he", "hin" => "hi", "hrv" => "hr", "hun" => "hu", "hye" | "arm" => "hy",
        "ind" => "id", "isl" | "ice" => "is", "ita" => "it", "jpn" => "ja", "kat" | "geo" => "ka",
        "kaz" => "kk", "kor" => "ko", "lit" => "lt", "lav" => "lv", "mkd" | "mac" => "mk",
        "nld" | "dut" => "nl", "nob" => "nb", "nor" => "nb", "pol" => "pl", "por" => "pt",
        "ron" | "rum" => "ro", "rus" => "ru", "slk" | "slo" => "sk", "slv" => "sl", "sqi" | "alb" => "sq",
        "srp" => "sr", "swe" => "sv", "tha" => "th", "tur" => "tr", "ukr" => "uk", "vie" => "vi",
        "zho" | "chi" => "zh", "bel" => "be", "aze" => "az", "bos" => "bs", "mon" => "mn",
        c => c,
    }
}

/// Dil kodundan (ISO 639-2 ya da 639-1) dilin kendi dilindeki adı ("tur" → "Türkçe").
pub fn language_name(code: &str) -> Option<String> {
    let l = nuxut::locale::Locale::parse(iso1(code))?;
    Some(l.language_name())
}

impl XkbCatalog {
    fn desc(&self, layout: usize, variant: Option<usize>) -> Option<&str> {
        let l = self.layouts.get(layout)?;
        Some(match variant {
            Some(v) => &l.variants.get(v)?.description,
            None => &l.description,
        })
    }

    fn langs(&self, layout: usize, variant: Option<usize>) -> &[String] {
        let Some(l) = self.layouts.get(layout) else { return &[] };
        match variant.and_then(|v| l.variants.get(v)) {
            // Varyantın kendi dili yoksa düzenin dili geçerli.
            Some(v) if !v.languages.is_empty() => &v.languages,
            _ => &l.languages,
        }
    }

    /// Config adlarından (`kb_layout` öğesi, `kb_variant` öğesi; boş = varsayılan varyant)
    /// katalog indisleri. Bilinmeyen düzen ya da varyant `None`.
    pub fn find(&self, layout: &str, variant: &str) -> Option<(usize, Option<usize>)> {
        let li = self.layouts.iter().position(|l| l.name == layout)?;
        if variant.is_empty() {
            return Some((li, None));
        }
        let vi = self.layouts[li].variants.iter().position(|v| v.name == variant)?;
        Some((li, Some(vi)))
    }

    /// Listede gösterilecek ad: dilin kendi adı + evdev açıklaması ("Türkçe — Turkish (F)").
    /// Dil bilinmiyorsa ya da ad açıklamada zaten geçiyorsa yalnızca açıklama; indeks
    /// geçersizse boş.
    pub fn display_name(&self, layout_idx: usize, variant_idx: Option<usize>) -> String {
        let Some(desc) = self.desc(layout_idx, variant_idx) else { return String::new() };
        let native = self.langs(layout_idx, variant_idx).iter().find_map(|c| language_name(c));
        match native {
            Some(n) if fold(&n) != fold(desc) && !fold(desc).starts_with(&fold(&n)) => format!("{n} — {desc}"),
            _ => desc.to_owned(),
        }
    }

    /// Arama: `(düzen, varyant)` eşleşmeleri. Açıklama, ad ve dil kodu (ve dilin kendi adı)
    /// büyük/küçük harf ve Türkçe katlamadan bağımsız alt dize olarak aranır. Boş sorgu =
    /// her düzen (varyantsız). Sıralama: tam ad eşleşmesi, sonra ad önekiyle, sonra dilin kendi
    /// düzeni (adı dil kodu olan düzen: "german" → `de`, `at`/`ch`den önce), sonra diğerleri;
    /// eşitlikte düzen satırı varyanttan önce, sonra katalog sırası. Varyant eşleşirse düzeni de ayrıca eşleşmiş olabilir; düzen
    /// satırı yalnızca kendisi eşleşince gelir.
    pub fn search(&self, query: &str) -> Vec<(usize, Option<usize>)> {
        let q = fold(query.trim());
        let mut hits: Vec<(u8, usize, Option<usize>)> = Vec::new();
        for (li, l) in self.layouts.iter().enumerate() {
            if let Some(rank) = self.rank(&q, &l.name, li, None) {
                hits.push((rank, li, None));
            }
            for vi in 0..l.variants.len() {
                if q.is_empty() {
                    break;
                }
                if let Some(rank) = self.rank(&q, &l.variants[vi].name, li, Some(vi)) {
                    hits.push((rank, li, Some(vi)));
                }
            }
        }
        hits.sort_by_key(|h| (h.0, h.2.is_some())); // kararlı: eşitte düzen önce, sonra katalog sırası
        hits.into_iter().map(|(_, l, v)| (l, v)).collect()
    }

    fn rank(&self, q: &str, name: &str, li: usize, vi: Option<usize>) -> Option<u8> {
        if q.is_empty() {
            return Some(2);
        }
        let n = fold(name);
        if n == q {
            return Some(0);
        }
        if n.starts_with(q) {
            return Some(1);
        }
        let hay = |s: &str| fold(s).contains(q);
        let in_desc = self.desc(li, vi).is_some_and(hay);
        let in_lang = self.langs(li, vi).iter().any(|c| fold(c) == q || language_name(c).is_some_and(|n| hay(&n)));
        if !(n.contains(q) || in_desc || in_lang) {
            return None;
        }
        // Dilin "ev" düzeni (adı kendi dil kodu olan düzen satırı: de ↔ deu) aynı dili taşıyan
        // bölge düzenlerinden (at, be, ch) önce: "deutsch"/"german" → önce `de`.
        let home = vi.is_none() && (in_desc || in_lang) && self.langs(li, None).iter().any(|c| iso1(c) == name);
        Some(if home { 2 } else { 3 })
    }
}

// ---- arka planda bir kez yükleme ------------------------------------------------------------

enum State {
    Idle,
    Loading,
    Done(Option<Arc<XkbCatalog>>),
}

static CATALOG: Mutex<State> = Mutex::new(State::Idle);

/// Önbellekteki katalog; ilk çağrıda yüklemeyi arka planda başlatır ve `None` döner, bitince
/// [`Event::Xkb`](super::Event::Xkb). Dosya okunamazsa kalıcı olarak `None` (sayfa boş durum
/// gösterir; yeniden denenmez).
pub fn catalog() -> Option<Arc<XkbCatalog>> {
    let mut s = CATALOG.lock().unwrap_or_else(|e| e.into_inner());
    match &*s {
        State::Done(c) => return c.clone(),
        State::Loading => return None,
        State::Idle => {}
    }
    *s = State::Loading;
    let spawned = std::thread::Builder::new().name("xkb-catalog".into()).spawn(|| {
        let cat = match load_system() {
            Ok(c) => Some(Arc::new(c)),
            Err(e) => {
                tracing::warn!("klavye düzeni kataloğu yüklenemedi: {e}");
                None
            }
        };
        *CATALOG.lock().unwrap_or_else(|e| e.into_inner()) = State::Done(cat);
        super::emit(super::Event::Xkb);
    });
    if spawned.is_err() {
        *s = State::Done(None);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE xkbConfigRegistry SYSTEM "xkb.dtd">
<xkbConfigRegistry version="1.1">
  <layoutList>
    <layout>
      <configItem>
        <name>tr</name>
        <shortDescription>tr</shortDescription>
        <description>Turkish</description>
        <languageList><iso639Id>tur</iso639Id></languageList>
      </configItem>
      <variantList>
        <variant><configItem><name>f</name><description>Turkish (F)</description></configItem></variant>
        <variant><configItem><name>bad name</name><description>x</description></configItem></variant>
      </variantList>
    </layout>
    <layout>
      <configItem>
        <name>us</name>
        <shortDescription>en</shortDescription>
        <description>English (US)</description>
        <languageList><iso639Id>eng</iso639Id></languageList>
      </configItem>
      <variantList>
        <variant><configItem><name>intl</name><description>English (intl., with AltGr dead keys)</description>
          <languageList><iso639Id>eng</iso639Id></languageList></configItem></variant>
      </variantList>
    </layout>
    <layout>
      <configItem><name>a,b</name><description>virgüllü</description></configItem>
    </layout>
    <layout>
      <configItem popularity="exotic"><name>zz</name><description>Zzish</description></configItem>
    </layout>
    <layout exotic="true">
      <configItem><name>yy</name><description>Yyish</description></configItem>
    </layout>
  </layoutList>
  <optionList>
    <group>
      <configItem><name>ctrl</name><description>Ctrl</description></configItem>
      <option><configItem><name>ctrl:nocaps</name><description>Caps as Ctrl</description></configItem></option>
    </group>
    <group allowMultipleSelection="true">
      <configItem><name>grp</name><description>Switching</description></configItem>
      <option><configItem><name>grp:alt_shift_toggle</name><description>Alt+Shift</description></configItem></option>
      <option><configItem><name>grp:$(rm)</name><description>kötü</description></configItem></option>
      <option><configItem><name>grp:win_space_toggle</name><description>Super+Space</description></configItem></option>
    </group>
  </optionList>
</xkbConfigRegistry>"#;

    fn sample() -> XkbCatalog {
        parse(SAMPLE).unwrap()
    }

    #[test]
    fn parses_layouts_variants_and_options() {
        let c = sample();
        let names: Vec<_> = c.layouts.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, ["tr", "us", "zz", "yy"], "geçersiz ad atlanır");
        assert_eq!(c.layouts[0].short, "tr");
        assert_eq!(c.layouts[0].languages, ["tur"]);
        assert_eq!(c.layouts[0].variants.len(), 1, "boşluklu varyant atlanır");
        assert_eq!(c.layouts[0].variants[0].name, "f");
        assert_eq!(c.layouts[1].variants[0].languages, ["eng"]);
        let opts: Vec<_> = c.switch_options.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(opts, ["grp:alt_shift_toggle", "grp:win_space_toggle"]);
    }

    #[test]
    fn exotic_flag_is_carried() {
        let c = sample();
        let ex: Vec<_> = c.layouts.iter().map(|l| l.exotic).collect();
        assert_eq!(ex, [false, false, true, true]);
    }

    #[test]
    fn name_validation() {
        for ok in ["tr", "us", "grp:alt_shift_toggle", "ctrl:nocaps", "a-b_c(d)+e"] {
            assert!(valid_name(ok), "{ok}");
        }
        for bad in ["", "a,b", "a b", "a;b", "$(x)", "a`b", "ş", &"x".repeat(65)] {
            assert!(!valid_name(bad), "{bad}");
        }
    }

    #[test]
    fn limits_are_enforced() {
        let mut xml = String::from("<r><layoutList>");
        for i in 0..2100 {
            xml.push_str(&format!("<layout><configItem><name>l{i}</name><description>d</description></configItem></layout>"));
        }
        xml.push_str("<layout><configItem><name>big</name><description>");
        xml.push_str(&"ü".repeat(500));
        xml.push_str("</description></configItem><variantList>");
        for i in 0..300 {
            xml.push_str(&format!("<variant><configItem><name>v{i}</name></configItem></variant>"));
        }
        xml.push_str("</variantList></layout></layoutList></r>");
        let c = parse(&xml).unwrap();
        assert_eq!(c.layouts.len(), 2000);
        let mut small = String::from("<r><layoutList><layout><configItem><name>big</name><description>");
        small.push_str(&"ü".repeat(500));
        small.push_str("</description></configItem><variantList>");
        for i in 0..300 {
            small.push_str(&format!("<variant><configItem><name>v{i}</name></configItem></variant>"));
        }
        small.push_str("</variantList></layout></layoutList></r>");
        let c = parse(&small).unwrap();
        assert_eq!(c.layouts[0].variants.len(), 200);
        assert_eq!(c.layouts[0].description.chars().count(), 128);
    }

    #[test]
    fn rejects_bad_xml_and_big_files() {
        assert!(parse("<a><b></a>").is_err());
        let dir = tempfile::tempdir().unwrap();
        let big = dir.path().join("big.xml");
        std::fs::write(&big, vec![b' '; (MAX_FILE + 1) as usize]).unwrap();
        assert!(load_file(&big).is_err());
        assert!(load_file(dir.path()).is_err(), "dizin normal dosya değil");
        assert!(load_file(&dir.path().join("yok.xml")).is_err());
    }

    #[test]
    fn search_ranks_exact_name_first_and_folds() {
        let c = sample();
        // "us": tam ad önce.
        assert_eq!(c.search("US")[0], (1, None));
        // Türkçe katlama: "TÜRKÇE" ve "turkish" aynı düzeni bulur.
        assert!(c.search("TURKISH").contains(&(0, None)));
        assert!(c.search("türkçe").contains(&(0, None)), "dilin kendi adı");
        // Dil kodu.
        assert_eq!(c.search("tur")[0], (0, None));
        // Varyant: açıklamadan.
        assert!(c.search("dead keys").contains(&(1, Some(0))));
        assert_eq!(c.search("intl")[0], (1, Some(0)), "varyant adı");
        // Boş sorgu: tüm düzenler, varyantsız.
        assert_eq!(c.search("  ").len(), 4);
        assert!(c.search("hiçbirşey").is_empty());
    }

    #[test]
    fn exact_name_beats_description_hit() {
        let mut c = sample();
        // "tr" başka bir düzenin açıklamasında geçse de tam ad önce gelir.
        c.layouts.insert(0, Layout {
            name: "aa".into(), description: "Contains tr word".into(), short: String::new(),
            languages: vec![], exotic: false, variants: vec![],
        });
        let r = c.search("tr");
        assert_eq!(r[0], (1, None));
    }

    #[test]
    fn language_search_puts_home_layout_first() {
        let lay = |name: &str, desc: &str, langs: &[&str], variants: Vec<Variant>| Layout {
            name: name.into(), description: desc.into(), short: String::new(),
            languages: langs.iter().map(|s| s.to_string()).collect(), exotic: false, variants,
        };
        let var = |name: &str, desc: &str| Variant { name: name.into(), description: desc.into(), languages: vec![], exotic: false };
        // evdev.xml sırası alfabetik: at, be, ch, de — hepsi Almanca taşır.
        let c = XkbCatalog {
            layouts: vec![
                lay("at", "German (Austria)", &["deu"], vec![]),
                lay("be", "Belgian", &["deu", "nld", "fra"], vec![]),
                lay("ch", "German (Switzerland)", &["deu", "gsw"], vec![var("fr", "French (Switzerland)")]),
                lay("de", "German", &["deu"], vec![var("nodeadkeys", "German (no dead keys)")]),
            ],
            switch_options: vec![],
        };
        for q in ["deutsch", "german", "GERMAN", "deu"] {
            let r = c.search(q);
            assert_eq!(r[0], (3, None), "{q}: önce de");
            // Diğer düzen satırları varyanttan önce, katalog sırasıyla.
            assert_eq!(r[1], (0, None), "{q}");
        }
        assert!(c.search("german").contains(&(3, Some(0))));
        // Ad eşleşmesi yine önce: "ch" sorgusunda ch.
        assert_eq!(c.search("ch")[0], (2, None));
    }

    #[test]
    fn display_name_uses_native_language() {
        let c = sample();
        assert_eq!(language_name("tur").as_deref(), Some("Türkçe"));
        assert_eq!(language_name("eng").as_deref(), Some("English"));
        assert_eq!(language_name("deu").as_deref(), Some("Deutsch"));
        assert_eq!(c.display_name(0, None), "Türkçe — Turkish");
        assert_eq!(c.display_name(0, Some(0)), "Türkçe — Turkish (F)", "varyant düzenin dilini miras alır");
        // "English" açıklamayla başlıyor: tekrar yok.
        assert_eq!(c.display_name(1, None), "English (US)");
        assert_eq!(c.display_name(9, None), "");
        assert_eq!(language_name("qqq"), None);
    }

    #[test]
    fn system_file_has_tr_us_and_switch_option() {
        if !Path::new(EVDEV_XML).is_file() {
            return;
        }
        let c = load_system().unwrap();
        assert!(c.layouts.iter().any(|l| l.name == "tr"));
        assert!(c.layouts.iter().any(|l| l.name == "us"));
        assert!(c.switch_options.iter().any(|o| o.name == "grp:alt_shift_toggle"));
        let tr = c.layouts.iter().position(|l| l.name == "tr").unwrap();
        assert!(c.display_name(tr, None).starts_with("Türkçe"));
        assert_eq!(c.search("tr")[0], (tr, None));
        if let Some(de) = c.layouts.iter().position(|l| l.name == "de") {
            assert_eq!(c.search("deutsch")[0], (de, None));
            assert_eq!(c.search("german")[0], (de, None));
        }
    }
}
