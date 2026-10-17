//! Ekran okuyucu (framework ADR-0011): Ayarlar penceresinin AT-SPI ağacı ve AT eylemleri.
//!
//! Ağaç: başlık çubuğu düğmeleri, arama alanı, kenar çubuğu (sayfa listesi ya da arama
//! sonuçları), açık sayfanın görünen bölüm başlıkları ve blokları ([`crate::block::Block::access`]).
//!
//! AT_POLICY (ADR-0011 madde 7, 2026-10-17): a11y bus'ı oturumdaki her sürece (Flatpak dahil)
//! açık; AT eylemi kullanıcı niyeti değildir. AT yalnız okur, odak taşır ve sayfalar arasında
//! gezinir: `Click` yalnız kenar çubuğundaki sayfa satırlarında ve arama sonuçlarında sunulur ve
//! uygulanır (gerçek girdinin yolundan: düğüm ortasına işaretçi tıklaması). Anahtar, seçim,
//! kaydırıcı, renk, geri al, son değişiklikler ve pencere düğmeleri AT'ye eylemsiz
//! ([`gate_actions`]); değer değişimi yalnız gerçek klavye ve işaretçiyle. Hoş geldin penceresi
//! ağaç yayımlamaz ve AT eylemi almaz (Uygula yalnız gerçek girdiyle).

use std::collections::HashMap;
use std::ops::Range;

use nuxut::access::{AccessAction, AccessRequest, AccessTree, Action as AxAction, NodeId, Role};
use nuxut::i18n::t;
use nuxut::{Context, Point, PointerEvent, PointerKind, SurfaceId};

use super::Settings;

/// AT'nin değer ya da durum değiştiren eylemleri: gezinme aralığı dışında hiç sunulmaz.
const CHANGING: [AxAction; 6] =
    [AxAction::Click, AxAction::Increment, AxAction::Decrement, AxAction::SetValue, AxAction::Expand, AxAction::Collapse];

impl Settings {
    pub(super) fn accessibility_inner(&mut self, _cx: &mut Context<Self>, surface: SurfaceId, tree: &mut AccessTree) {
        if Some(surface) != self.window {
            self.access_points.clear();
            return;
        }
        let now = std::time::Instant::now();
        tree.root().label(self.app_title.text());
        let bar = tree.push("titlebar", Role::Toolbar).id();
        tree.within(bar, |tree| {
            self.undo_btn.access(tree, "undo").label(&t!("settings-a11y-undo"));
            self.history_btn.access(tree, "history").label(&t!("settings-a11y-history"));
            self.min_btn.access(tree, "minimize").label(&t!("settings-a11y-minimize"));
            self.max_btn.access(tree, "maximize").label(&t!("settings-a11y-maximize"));
            self.close_btn.access(tree, "close").label(&t!("settings-a11y-close"));
        });
        self.search.access(tree, "search").label(&t!("settings-a11y-search"));
        let nav_from = tree.len();
        if self.searching && self.no_results {
            let page_mascot = self.pages[Self::page_index(self.current)].as_ref().and_then(|p| p.mascot(&self.store));
            self.search_empty[usize::from(super::search_spot(page_mascot).is_none())].access(tree, "no-results");
        } else if self.searching {
            let ids = &self.result_ids;
            self.results.access(tree, "results", |i| ids.get(i).copied().unwrap_or_default()).label(&t!("settings-a11y-results"));
        } else {
            let pages = &self.nav_pages;
            let nav = self.nav.access(tree, "nav", |i| pages.get(i).map(|p| p.title())).label(&t!("settings-a11y-nav")).id();
            // Açık sayfa satırı seçili okunur.
            if let Some(i) = pages.iter().position(|p| *p == self.current)
                && let Some(n) = tree.get(nuxut::access::child_id(nav, pages[i].title()))
            {
                n.selected(true);
            }
        }
        // Boş durum bloğu (düğmeli olabilir) gezinme değil.
        let nav = if self.searching && self.no_results { nav_from..nav_from } else { nav_from..tree.len() };
        if let Some(page) = self.pages[Self::page_index(self.current)].as_ref() {
            let id = tree.push("page", Role::Pane).label(self.page_title.text()).id();
            tree.within(id, |tree| page.access(tree, now));
        }
        gate_actions(tree, nav);
        self.access_points = click_points(tree);
    }

    pub(super) fn accessibility_action_inner(&mut self, cx: &mut Context<Self>, surface: SurfaceId, request: AccessRequest) {
        // AT_POLICY: yalnız ağaçta `Click` sunan (gezinme) düğümler; geri kalan her eylem atılır.
        if Some(surface) != self.window || request.action != AccessAction::Click {
            return;
        }
        let Some(&at) = self.access_points.get(&request.node) else { return };
        let left = nuxut::Button::Left;
        for kind in [PointerKind::Motion, PointerKind::Press(left), PointerKind::Release(left), PointerKind::Leave] {
            nuxut::App::pointer(self, cx, surface, PointerEvent::new(at, kind));
        }
    }
}

/// Ağaç sırasında `nav` aralığı (kenar çubuğu sayfa listesi / arama sonuçları) dışındaki her
/// düğümden değiştiren eylemleri siler: Orca onları sunmaz, `click_points` onları içermez.
/// `Focus` ve `ScrollIntoView` kalır.
fn gate_actions(tree: &mut AccessTree, nav: Range<usize>) {
    let ids: Vec<NodeId> = tree
        .nodes()
        .enumerate()
        .filter(|(i, (_, n))| !nav.contains(i) && CHANGING.iter().any(|&a| n.supports_action(a)))
        .map(|(_, (id, _))| id)
        .collect();
    for id in ids {
        if let Some(mut n) = tree.get(id) {
            for a in CHANGING {
                n.node_mut().remove_action(a);
            }
        }
    }
}

/// Tıklanabilir düğümlerin orta noktaları (AT `Click` → işaretçi tıklaması).
fn click_points(tree: &AccessTree) -> HashMap<NodeId, Point> {
    tree.nodes()
        .filter(|(_, n)| n.supports_action(AxAction::Click))
        .filter_map(|(id, n)| n.bounds().map(|b| (id, Point::new(((b.x0 + b.x1) / 2.0) as f32, ((b.y0 + b.y1) / 2.0) as f32))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nuxut::Rect;

    /// AT yalnız gezinme satırlarına tıklayabilir: anahtar, kaydırıcı, seçim ve geri al
    /// düğmesi ağaçta değiştiren eylem sunmaz, tıklama noktası olmaz; odak kalır.
    #[test]
    fn assistive_actions_only_navigate() {
        let mut tree = AccessTree::new();
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        let undo = tree.push("undo", Role::Button).bounds(r).clickable().action(AxAction::Focus).id();
        let from = tree.len();
        let nav = tree.push("nav", Role::List).id();
        let page = tree.within(nav, |t| t.push("Görünüm", Role::ListItem).bounds(r).clickable().id());
        let to = tree.len();
        let switch = tree.push("dark", Role::Switch).bounds(r).clickable().action(AxAction::Focus).id();
        let slider = tree
            .push("scale", Role::Slider)
            .bounds(r)
            .action(AxAction::Increment)
            .action(AxAction::Decrement)
            .action(AxAction::SetValue)
            .id();
        let combo = tree.push("font", Role::ComboBox).bounds(r).clickable().action(AxAction::Expand).id();
        gate_actions(&mut tree, from..to);
        let points = click_points(&tree);
        assert_eq!(points.keys().copied().collect::<Vec<_>>(), vec![page]);
        let supports = |tree: &mut AccessTree, id, a| tree.get(id).is_some_and(|mut n| n.node_mut().supports_action(a));
        for id in [undo, switch, slider, combo] {
            for a in CHANGING {
                assert!(!supports(&mut tree, id, a), "{a:?}");
            }
        }
        assert!(supports(&mut tree, switch, AxAction::Focus));
        assert!(supports(&mut tree, undo, AxAction::Focus));
    }
}
