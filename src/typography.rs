//! Tipografi rolleri (framework ADR-0006): Ayarlar'ın gösterdiği başlık, açıklama ve örnek
//! metin; bilinen ailelerin Arch paket adları ("kurulu değil" bilgisi için).
//!
//! Varsayılan aileler burada YOK: tek kaynak `FrameworkConfig::default()` (bkz. `defaults`).

use nuxut::render::FamilyInfo;
use nuxut::theme::IconSet;

use crate::i18n::text_or;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontRole {
    Ui,
    Display,
    Document,
    Mono,
    Icon,
}

impl FontRole {
    pub const ALL: [FontRole; 5] = [FontRole::Ui, FontRole::Display, FontRole::Document, FontRole::Mono, FontRole::Icon];

    /// `[typography.<key>]`.
    pub fn key(self) -> &'static str {
        match self {
            FontRole::Ui => "ui",
            FontRole::Display => "display",
            FontRole::Document => "document",
            FontRole::Mono => "mono",
            FontRole::Icon => "icon",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            FontRole::Ui => text_or("settings-section-interface", "Arayüz"),
            FontRole::Display => text_or("settings-section-display", "Başlık"),
            FontRole::Document => text_or("settings-section-document", "Belge"),
            FontRole::Mono => text_or("settings-section-mono", "Eş aralıklı"),
            FontRole::Icon => text_or("settings-typography-role-icon", "İkon"),
        }
    }

    pub fn subtitle(self) -> &'static str {
        match self {
            FontRole::Ui => text_or("settings-typography-ui-description", "Düğmeler, menüler, ayar satırları"),
            FontRole::Display => text_or("settings-typography-display-description", "Saat, büyük başlıklar"),
            FontRole::Document => text_or("settings-typography-document-description", "Uzun metin: bildirim gövdesi, takvim notları"),
            FontRole::Mono => text_or("settings-typography-mono-subtitle", "Kod, terminal çıktısı, sayılar"),
            FontRole::Icon => text_or("settings-typography-icon-subtitle", "Sembol fontu (ligatürlü)"),
        }
    }

    /// Satırda ve önizlemede ailenin kendi fontuyla çizilen örnek.
    pub fn sample(self) -> &'static str {
        match self {
            FontRole::Ui => text_or("settings-typography-sample-ui", "Ağ bağlantısı · Ses düzeyi %60"),
            FontRole::Display => "14:32",
            FontRole::Document => text_or("settings-typography-sample-document", "Pijamalı hasta yağız şoföre çabucak güvendi."),
            FontRole::Mono => "fn main() { 0O 1lI }",
            FontRole::Icon => "",
        }
    }
}

/// Bilinen ailelerin Arch paketi (`(AUR)` işaretli olanlar resmi depoda değil).
pub fn arch_package(family: &str) -> Option<&'static str> {
    const KNOWN: &[(&str, &str)] = &[
        ("Inter", "inter-font"),
        ("Inter Display", "inter-font"),
        ("Source Serif 4", "adobe-source-serif-fonts"),
        ("JetBrainsMono Nerd Font", "ttf-jetbrains-mono-nerd"),
        ("JetBrainsMono Nerd Font Mono", "ttf-jetbrains-mono-nerd"),
        ("JetBrains Mono", "ttf-jetbrains-mono"),
        ("Material Symbols Rounded", "ttf-material-symbols-variable"),
        ("Material Symbols Outlined", "ttf-material-symbols-variable"),
        ("Material Symbols Sharp", "ttf-material-symbols-variable"),
        ("Noto Sans", "noto-fonts"),
        ("Noto Serif", "noto-fonts"),
    ];
    KNOWN.iter().find(|(f, _)| f.eq_ignore_ascii_case(family)).map(|(_, p)| *p)
}

/// Ligatürlü ikon fontu (Material Symbols / Material Icons): harf taşıdığı için `latin`
/// süzgecinden geçer, adıyla ayıklanır.
pub fn is_icon_font(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n.starts_with("material symbols") || n.starts_with("material icons")
}

/// Aile bu rolün listesinde gösterilir mi? Metin rollerinde ikon fontları ve harf taşımayan
/// sembol fontları yok; eş aralıklıda yalnızca eş aralıklılar; ikon rolünde yalnızca seçili
/// setin fontları (Material → Material Symbols, Nerd → Nerd Font aileleri).
pub fn listed(info: &FamilyInfo, role: FontRole, set: IconSet) -> bool {
    match role {
        FontRole::Icon => match set {
            IconSet::Material => is_icon_font(&info.name),
            IconSet::Nerd => info.name.to_ascii_lowercase().contains("nerd font"),
        },
        FontRole::Mono => info.monospace && info.latin && !is_icon_font(&info.name),
        _ => info.latin && !is_icon_font(&info.name),
    }
}

/// Rolün listesinde gösterilen kurulu aileler (framework önbelleğinden; ikinci tarama yok).
pub fn installed_families(text: &mut nuxut::TextSystem, role: FontRole, set: IconSet) -> Vec<String> {
    text.families().iter().filter(|f| listed(f, role, set)).map(|f| f.name.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roles_have_unique_keys_and_text() {
        for (i, a) in FontRole::ALL.iter().enumerate() {
            assert!(!a.title().is_empty() && !a.subtitle().is_empty());
            assert!(FontRole::ALL[i + 1..].iter().all(|b| b.key() != a.key()));
        }
    }

    fn info(name: &str, monospace: bool, latin: bool) -> FamilyInfo {
        FamilyInfo { name: name.into(), weights: vec![400], weight_range: None, italic: false, monospace, latin }
    }

    #[test]
    fn icon_fonts_only_in_icon_role() {
        let material = info("Material Symbols Rounded", false, true);
        let nerd = info("JetBrainsMono Nerd Font", true, true);
        let symbols = info("Symbols Nerd Font", true, false);
        let inter = info("Inter", false, true);
        for role in [FontRole::Ui, FontRole::Display, FontRole::Document, FontRole::Mono] {
            assert!(!listed(&material, role, IconSet::Material));
            assert!(!listed(&symbols, role, IconSet::Material));
        }
        assert!(listed(&inter, FontRole::Ui, IconSet::Material));
        assert!(!listed(&inter, FontRole::Mono, IconSet::Material));
        assert!(listed(&nerd, FontRole::Mono, IconSet::Material));
        assert!(listed(&material, FontRole::Icon, IconSet::Material));
        assert!(!listed(&nerd, FontRole::Icon, IconSet::Material));
        assert!(!listed(&inter, FontRole::Icon, IconSet::Material));
        assert!(listed(&nerd, FontRole::Icon, IconSet::Nerd));
        assert!(listed(&symbols, FontRole::Icon, IconSet::Nerd));
        assert!(!listed(&material, FontRole::Icon, IconSet::Nerd));
    }

    #[test]
    fn known_packages() {
        assert_eq!(arch_package("inter"), Some("inter-font"));
        assert_eq!(arch_package("Bilinmeyen"), None);
    }
}
