//! Sayfanın dosyaya yazılmayan arayüz durumu (`docs/design/settings-ux.md` §3.3): açılır
//! satırların açık/kapalı hâli, seçili widget… Sayfa nesnesi oturum boyunca yaşadığı için bu
//! durum da oturumda hatırlanır; config'e, geçmişe ya da geri al'a girmez.
//!
//! Kayıttaki `ui_if` koşulları bunu okur (`visible_if` dosyadaki değere, `ui_if` bu duruma bakar).

use std::collections::BTreeSet;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageUi {
    /// Açık olan açılır satırlar (`Tier::Detail` adı).
    open: BTreeSet<&'static str>,
    /// Bar sayfasında seçili widget'ın adı (önizlemeden, Widget'lar listesinden ya da aramadan).
    /// `None`: varsayılan seçim — bar'ın sağ (dikeyde alt) ucundaki widget (karar 4); bkz.
    /// [`crate::pages::bar_preview::selected`].
    pub selected_widget: Option<String>,
}

impl PageUi {
    pub fn is_open(&self, disclosure: &str) -> bool {
        self.open.contains(disclosure)
    }

    /// `true` = durum değişti.
    pub fn set_open(&mut self, disclosure: &'static str, open: bool) -> bool {
        if open { self.open.insert(disclosure) } else { self.open.remove(disclosure) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_state_toggles_and_reports_change() {
        let mut ui = PageUi::default();
        assert!(!ui.is_open("Renkleri tek tek ayarla"));
        assert!(ui.set_open("Renkleri tek tek ayarla", true));
        assert!(!ui.set_open("Renkleri tek tek ayarla", true), "aynı durum değişiklik değil");
        assert!(ui.is_open("Renkleri tek tek ayarla"));
        assert!(ui.set_open("Renkleri tek tek ayarla", false));
        assert!(!ui.is_open("Renkleri tek tek ayarla"));
    }
}
