//! Bekçi (güvenlik, framework ADR-0011 madde 7, 2026-10-17): AT eylemi Ayarlar'da yalnız gezinir.
//!
//! a11y bus'ı oturumdaki her sürece (Flatpak'te varsayılan izinli) açık; AT eylemi gerçek
//! kullanıcı niyeti değildir. AT ağacı ve eylemleri yalnız `src/app/a11y.rs`'te kurulur: orada
//! gezinme dışındaki düğümlerden değiştiren eylemler silinir (birim testi
//! `assistive_actions_only_navigate`). Hoş geldin penceresi ağaç yayımlamaz ve AT eylemi almaz.
//! Başka bir yere ağaç ya da eylem işleyicisi eklenirse bu test kırılır: eklenen kod AT kuralını
//! `AT_POLICY:` işaretiyle belgelemeli (ARCHITECTURE "Ekran okuyucu").

use std::path::{Path, PathBuf};

fn sources(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push((p.clone(), std::fs::read_to_string(&p).unwrap()));
        }
    }
}

#[test]
fn assistive_actions_are_gated() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    sources(&root.join("src"), &mut files);
    assert!(files.len() > 20, "kaynaklar okunamadı: {}", files.len());
    for (path, text) in &files {
        for hook in ["fn accessibility(", "fn accessibility_action(", "fn accessibility_action_inner("] {
            assert!(
                !text.contains(hook) || text.contains("AT_POLICY:"),
                "{}: {hook} eklendi ama AT kuralı belgelenmedi (AT_POLICY:)",
                path.display()
            );
        }
        // Hoş geldin (Uygula, Bitir) AT'ye kapalı: ağaç ya da eylem işleyicisi yok.
        let name = path.file_name().unwrap().to_string_lossy();
        if name.starts_with("welcome") {
            assert!(
                !text.contains(".access(tree") && !text.contains("AccessRequest"),
                "{}: Hoş geldin AT'ye açıldı",
                path.display()
            );
        }
    }
    let a11y = std::fs::read_to_string(root.join("src/app/a11y.rs")).unwrap();
    // Eylem yalnız ağaçta `Click` kalan (gezinme) düğümlere: tıklama noktaları kapıdan sonra.
    let gate = a11y.find("gate_actions(tree, nav)").expect("kapı çağrılmıyor");
    let points = a11y.find("self.access_points = click_points(tree)").expect("tıklama noktaları yok");
    assert!(gate < points, "tıklama noktaları kapıdan önce toplanıyor");
    // Pencere dışı yüzeyde (Hoş geldin) eski tıklama noktaları kalmaz.
    assert!(a11y.contains("self.access_points.clear()"));
}
