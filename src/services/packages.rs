//! Önerilen resmî depo paketlerini kurma ("Kur" düğmesi; kullanıcı kararı 2026-10-17: kök
//! gerektiren düzeltme bir düğmeyle, kullanıcıya `sudo pacman` yazdırılmaz). Ayarlar kurmaz:
//! `nuxut-shell packages install <ad>…` kök yardımcının `packages` alanını pkexec ile çalıştırır
//! (Nuxut polkit penceresi; yardımcı yalnız kendi sabit listesindeki adları kabul eder). Süreç
//! genelinde tek iş; bitince görünüm durumu yeniden sorulur ([`super::look::refresh`]) ve
//! [`super::Event::Look`] yayılır.

use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;

/// Son işin sonucu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Installed,
    /// Yönetici onayı verilmedi (hiçbir şey değişmedi).
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct State {
    running: bool,
    last: Option<Outcome>,
}

static STATE: Mutex<State> = Mutex::new(State { running: false, last: None });

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    f(&mut STATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner))
}

pub fn running() -> bool {
    with(|s| s.running)
}

/// Son işin sonucu (yeni iş başlayınca silinir).
pub fn last() -> Option<Outcome> {
    with(|s| s.last)
}

/// Çıkış kodunun anlamı (`nuxut-shell` sözleşmesi: 0 kuruldu, 3 onay verilmedi).
pub fn outcome(code: Option<i32>) -> Outcome {
    match code {
        Some(0) => Outcome::Installed,
        Some(3) => Outcome::Cancelled,
        _ => Outcome::Failed,
    }
}

/// Kurar (arka planda). İş sürüyorsa ya da ad yoksa etkisiz.
pub fn start(names: &[String]) {
    if names.is_empty() || names.len() > 16 {
        return;
    }
    let go = with(|s| {
        let go = !s.running;
        if go {
            s.running = true;
            s.last = None;
        }
        go
    });
    if !go {
        return;
    }
    super::emit(super::Event::Look);
    let names = names.to_vec();
    let spawned = std::thread::Builder::new().name("packages-install".into()).spawn(move || {
        let code = super::widgets::shell_bin().and_then(|bin| {
            Command::new(bin).args(["packages", "install"]).args(&names).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status().ok()?.code()
        });
        with(|s| {
            s.running = false;
            s.last = Some(outcome(code));
        });
        super::look::refresh(Duration::ZERO);
        super::emit(super::Event::Look);
    });
    if spawned.is_err() {
        with(|s| {
            s.running = false;
            s.last = Some(Outcome::Failed);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes() {
        assert_eq!(outcome(Some(0)), Outcome::Installed);
        assert_eq!(outcome(Some(3)), Outcome::Cancelled);
        assert_eq!(outcome(Some(2)), Outcome::Failed);
        assert_eq!(outcome(None), Outcome::Failed);
    }
}
