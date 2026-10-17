//! Bekçi: koda gömülü renk yok (kullanıcı kuralı; framework `crates/nuxut/tests/
//! no_hardcoded_colors.rs` ile aynı kural). Her renk temadan gelir; varsayılanların tek
//! yeri `nuxut-theme`'dir — Ayarlar onları kopyalamaz, çalışma zamanında okur
//! (`src/defaults.rs`).
//!
//! Taranan: `src/**` ve `examples/**`. İzinli: `#[cfg(test)]` satırından sonrası, test modülü
//! dosyaları (`tests.rs`) ve
//! `// renk-matematiği:` yorumu taşıyan (ya da hemen önünde bu yorum olan) satır.

use std::path::{Path, PathBuf};

const CONSTRUCTORS: &[&str] = &["Color::rgb(", "Color::rgba(", "from_rgba8(", "Color::from_hex("];
const NAMED: &[&str] = &["Color::WHITE", "Color::BLACK"];
const ALLOW_MARK: &str = "// renk-matematiği:";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_owned()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn scanned_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for sub in ["src", "examples"] {
        rust_files(&root().join(sub), &mut files);
    }
    // Ayrı dosyadaki test modülleri (`#[cfg(test)] mod tests;` → `tests.rs`) taranmaz.
    files.retain(|f| !f.ends_with("tests.rs"));
    files.sort();
    files
}

#[test]
fn no_hardcoded_colors() {
    let root = root();
    let mut hits = Vec::new();
    for f in scanned_files() {
        let text = std::fs::read_to_string(&f).unwrap();
        for (n, line) in production_lines(&text) {
            if is_color_literal(line) {
                hits.push(format!("{}:{n}: {}", f.strip_prefix(&root).unwrap_or(&f).display(), line.trim()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "koda gömülü renk bulundu (tema token'ı kullanın; gerçekten renk matematiğiyse satıra \
         `{ALLOW_MARK} <gerekçe>` ekleyin):\n{}",
        hits.join("\n")
    );
}

/// `"#rgb"`, `"#rgba"`, `"#rrggbb"`, `"#rrggbbaa"` dizgi sabiti.
fn has_hex_literal(line: &str) -> bool {
    let b = line.as_bytes();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b'"' && b[i + 1] == b'#' {
            let digits = b[i + 2..].iter().take_while(|c| c.is_ascii_hexdigit()).count();
            if matches!(digits, 3 | 4 | 6 | 8) && b.get(i + 2 + digits) == Some(&b'"') {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// Test modülünden önceki, yorum olmayan, izin işaretsiz satırlar: (satır no, metin).
fn production_lines(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut prev_allowed = false;
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("#[cfg(test)]") {
            break;
        }
        let allowed = line.contains(ALLOW_MARK);
        let is_comment = t.starts_with("//");
        if !allowed && !prev_allowed && !is_comment {
            out.push((i + 1, line));
        }
        prev_allowed = allowed && is_comment;
    }
    out
}

#[test]
fn detector_self_check() {
    assert!(has_hex_literal(r##"let c = "#12141a";"##));
    assert!(has_hex_literal(r##"x("#fff")"##));
    assert!(!has_hex_literal(r##"format!("#{}", n)"##));
    assert!(!has_hex_literal(r##""#1234567""##));
    let text = "a\n// renk-matematiği: beyaz uç\nColor::WHITE\nColor::BLACK\n#[cfg(test)]\nColor::WHITE\n";
    let lines: Vec<_> = production_lines(text).into_iter().map(|(_, l)| l).collect();
    assert_eq!(lines, vec!["a", "Color::BLACK"]);
}

fn is_color_literal(line: &str) -> bool {
    if NAMED.iter().any(|n| line.contains(n)) || has_hex_literal(line) {
        return true;
    }
    CONSTRUCTORS.iter().any(|p| {
        line.match_indices(p).any(|(i, _)| {
            let rest = line[i + p.len()..].trim_start();
            rest.starts_with(|c: char| c.is_ascii_digit() || c == '"' || c == '.')
        })
    })
}

#[test]
fn literal_detection() {
    assert!(is_color_literal("Color::rgba(0.5, 0.2, 0.1, 1.0)"));
    assert!(is_color_literal("Color::from_rgba8(0x42, 0x85, 0xF4, 255)"));
    assert!(is_color_literal("Color::from_hex(\"#fff\")"));
    assert!(!is_color_literal("Color::rgba(c[0], c[1], c[2], c[3])"));
    assert!(!is_color_literal("Color::from_rgba8(c.r(), c.g(), c.b(), c.a())"));
    assert!(!is_color_literal("Color::from_hex(&s)"));
}
