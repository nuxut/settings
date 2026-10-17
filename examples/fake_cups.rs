//! Canlı deneme için sahte CUPS: `cargo run --example fake_cups -- <soket> [boş]`. Ayarlar
//! `CUPS_SERVER=<soket>` ile buna bağlanır (gerçek CUPS'a dokunulmaz). `boş`: yazıcısız sunucu.
//! Tablo `tests/support/fake_cups.rs` `demo` (iş sahibi `$USER`).

#[path = "../tests/support/fake_cups.rs"]
mod fake_cups;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("kullanım: fake_cups <soket> [boş]");
        std::process::exit(2);
    };
    let user = std::env::var("USER").unwrap_or_default();
    let state = if args.next().as_deref() == Some("boş") { fake_cups::FakeState::default() } else { fake_cups::demo(&user) };
    let _ = std::fs::remove_file(&path);
    let fake = fake_cups::FakeCups::new(state);
    fake.serve_unix(std::path::Path::new(&path));
    eprintln!("fake_cups: {path}");
    loop {
        std::thread::park();
    }
}
