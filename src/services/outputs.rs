//! Bağlı monitörler: Wayland çıkışlarının bağlayıcı adı ("DP-1") ve açıklaması. `App`
//! çıkış olaylarında günceller ve [`Event::Outputs`](super::Event::Outputs) yayar; bloklar
//! (sonradan kurulanlar dahil) [`list`] ile okur.

use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    /// Bağlayıcı adı; shell.toml `[wallpaper.outputs."<ad>"]` anahtarı.
    pub name: String,
    pub description: String,
}

static OUTPUTS: Mutex<Vec<Output>> = Mutex::new(Vec::new());

/// Ada göre sıralı, tekrarsız ve adı boş olmayan çıkışlar.
pub fn set(mut outputs: Vec<Output>) -> bool {
    outputs.retain(|o| !o.name.is_empty());
    outputs.sort_by(|a, b| a.name.cmp(&b.name));
    outputs.dedup_by(|a, b| a.name == b.name);
    let mut cur = OUTPUTS.lock().unwrap_or_else(|e| e.into_inner());
    if *cur == outputs {
        return false;
    }
    *cur = outputs;
    true
}

pub fn list() -> Vec<Output> {
    OUTPUTS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}
