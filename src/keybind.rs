//! Kısayol dizgeleri (`shell.toml [keybinds]`, `eylem = "MOD+MOD+Tuş"`, `""` = atanmamış):
//! ayrıştırma, yazım biçimi, gösterim adları, tuş yakalama ve çakışma. Saf mantık; arayüz
//! `pages/keybinds.rs`'te.
//!
//! Sözdizimi shell'in `parse_setting_combo`'suyla aynıdır (shell `hypr_config/keybinds.rs`):
//! değiştiriciler SUPER/WIN/META, CTRL/CONTROL, ALT, SHIFT; tuş xkb keysym adı (harf/rakam/`_`,
//! en çok 32), `code:<n>` ya da `mouse:<n>`; karşılaştırma tuşta büyük/küçük harf duyarsız.
//!
//! Yakalama ([`capture`]) tuş adını framework'ün XKB çözümünden (`KeyEvent::xkb`) alır ve
//! Hyprland'in eşleyeceği keysym'i yazar: Hyprland 0.56 bind'i değiştiricisiz (seviye 0)
//! keysym'le eşler — varsayılan ilk düzenin, `input:resolve_binds_by_sym` açıkken etkin düzenin
//! ([`XkbKey::hyprland`]). Türkçe Q'da Shift+1 → `SHIFT+1`, Shift+ı → `SHIFT+idotless`; `us,tr`
//! ile ikinci düzende ı tuşu varsayılan olarak `I` (ilk düzenin adı) yazılır. Tek ASCII harf
//! varsayılanlarla aynı biçimde büyük yazılır (`SUPER+N`). Ad tablosu yok.

use std::fmt;

use nuxut::i18n::t;
use nuxut::ui::input::{Key, KeyEvent, KeyState, Modifiers, XkbKey};

pub const MOD_SHIFT: u8 = 1;
pub const MOD_CTRL: u8 = 2;
pub const MOD_ALT: u8 = 4;
pub const MOD_SUPER: u8 = 8;

/// Yazım ve gösterim sırası (shell'in `Display`'iyle aynı). Üçüncü öge başlıktır; "Super",
/// "Ctrl", "Alt", "Shift" evrensel değiştirici adlarıdır ve her dilde aynı yazılır
/// (// i18n: tanı — katalogda settings-keybind-cap-* ile aynı, çeviri gerektirmez).
const MODS: [(u8, &str, &str); 4] = [(MOD_SUPER, "SUPER", "Super"), (MOD_CTRL, "CTRL", "Ctrl"), (MOD_ALT, "ALT", "Alt"), (MOD_SHIFT, "SHIFT", "Shift")];

const MAX_KEY_LEN: usize = 32;

/// Tuş birleşimi. `key` yazıldığı (yakalandığı) biçimde durur; eşitlik büyük/küçük harf duyarsız.
#[derive(Debug, Clone)]
pub struct Combo {
    pub mods: u8,
    pub key: String,
}

impl PartialEq for Combo {
    fn eq(&self, other: &Self) -> bool {
        self.mods == other.mods && self.key.eq_ignore_ascii_case(&other.key)
    }
}

impl Eq for Combo {}

impl fmt::Display for Combo {
    /// `shell.toml`'a yazılan biçim: `SUPER+SHIFT+N`; dokunma/rakam ailesinde yalnız değiştirici: `SUPER`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts: Vec<&str> = MODS.iter().filter(|(b, _, _)| self.mods & b != 0).map(|(_, n, _)| *n).collect();
        if !self.key.is_empty() {
            parts.push(&self.key);
        }
        f.write_str(&parts.join("+"))
    }
}

impl Combo {
    /// Gösterim parçaları (tuş başlıkları): `["Super", "Shift", "N"]`.
    pub fn caps(&self) -> Vec<String> {
        let mut out = mod_caps(self.mods);
        if !self.key.is_empty() {
            out.push(key_label(&self.key));
        }
        out
    }

    /// Yazı tuşu (harf, rakam, boşluk, ok…) değiştiricisiz ya da yalnızca Shift ile mi?
    /// Öyleyse masaüstü genelinde yazmayı bozar; yakalayıcı kabul etmez.
    pub fn blocks_typing(&self) -> bool {
        self.mods & (MOD_SUPER | MOD_CTRL | MOD_ALT) == 0 && !is_special_key(&self.key)
    }
}

/// Değiştirici maskesinin tuş başlıkları.
pub fn mod_caps(mods: u8) -> Vec<String> {
    MODS.iter().filter(|(b, _, _)| mods & b != 0).map(|(_, _, l)| (*l).to_owned()).collect()
}

fn valid_key(key: &str) -> bool {
    let digits = |s: &str| !s.is_empty() && s.len() <= 4 && s.bytes().all(|b| b.is_ascii_digit());
    if let Some(code) = key.strip_prefix("code:") {
        return digits(code);
    }
    if let Some(button) = key.strip_prefix("mouse:") {
        return digits(button);
    }
    !key.is_empty() && key.len() <= MAX_KEY_LEN && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Hata metni (kullanıcıya).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    TooLong,
    UnknownModifier,
    InvalidKey,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // hata türü Display metni; kullanıcıya görünen uyarı (pages/keybinds.rs)
        // çevrilmiş anahtardan (settings-keybinds-error-*) kurar, bu metin yalnız tanıdır.
        f.write_str(match self {
            ParseError::Empty => "boş kısayol",
            ParseError::TooLong => "kısayol çok uzun",
            ParseError::UnknownModifier => "bilinmeyen değiştirici",
            ParseError::InvalidKey => "geçersiz tuş",
        })
    }
}

/// `"SUPER+SHIFT+N"` → `Combo`. Katı (shell'in kabul etmediğini kabul etmez). Boş dizge
/// `Empty` döner; "atanmamış" anlamı çağırana aittir.
pub fn parse(text: &str) -> Result<Combo, ParseError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(ParseError::Empty);
    }
    if text.len() > 96 {
        return Err(ParseError::TooLong);
    }
    let parts: Vec<&str> = text.split('+').map(str::trim).collect();
    let (key, mods) = parts.split_last().ok_or(ParseError::Empty)?;
    let mut mask = 0;
    for m in mods {
        mask |= match m.to_ascii_uppercase().as_str() {
            "SUPER" | "WIN" | "META" => MOD_SUPER,
            "CTRL" | "CONTROL" => MOD_CTRL,
            "ALT" => MOD_ALT,
            "SHIFT" => MOD_SHIFT,
            _ => return Err(ParseError::UnknownModifier),
        };
    }
    if !valid_key(key) {
        return Err(ParseError::InvalidKey);
    }
    Ok(Combo { mods: mask, key: (*key).to_owned() })
}

/// Eylem değerinin biçimi (shell `hypr_config/keybinds.rs` `Shape` ile aynı anlam; shell ADR-0006).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `MOD+…+Tuş`.
    Key,
    /// Tek değiştiriciye dokunup bırakma (`SUPER`: Super_L/Super_R; başlatıcı).
    Tap,
    /// Değiştirici + rakam satırı 1…0 (tuş kodlarıyla; değer yalnız değiştirici).
    Digits,
    /// Fare düğmesi/tekerleği: yakalanmaz, yalnızca temizlenir ya da varsayılana döner.
    Pointer,
}

/// Eylem adına göre biçim. Shell'deki biçimin aynısı (`shapes_match_shell_defaults` testli).
pub fn shape_of(action: &str) -> Shape {
    match action {
        "launcher" => Shape::Tap,
        "workspace" | "move_to_workspace" => Shape::Digits,
        "window_drag" | "window_resize_drag" | "workspace_scroll_next" | "workspace_scroll_prev" => Shape::Pointer,
        _ => Shape::Key,
    }
}

fn mod_bit(word: &str) -> Option<u8> {
    Some(match word.to_ascii_uppercase().as_str() {
        "SUPER" | "WIN" | "META" => MOD_SUPER,
        "CTRL" | "CONTROL" => MOD_CTRL,
        "ALT" => MOD_ALT,
        "SHIFT" => MOD_SHIFT,
        _ => return None,
    })
}

/// Biçime göre ayrıştırma: dokunma tek değiştirici, rakam ailesi yalnız değiştiriciler (en az
/// bir), diğerleri [`parse`].
pub fn parse_shaped(shape: Shape, text: &str) -> Result<Combo, ParseError> {
    match shape {
        Shape::Key | Shape::Pointer => parse(text),
        Shape::Tap | Shape::Digits => {
            let text = text.trim();
            if text.is_empty() {
                return Err(ParseError::Empty);
            }
            if text.len() > 96 {
                return Err(ParseError::TooLong);
            }
            let mut mods = 0;
            for p in text.split('+').map(str::trim) {
                mods |= mod_bit(p).ok_or(ParseError::InvalidKey)?;
            }
            if shape == Shape::Tap && mods.count_ones() != 1 {
                return Err(ParseError::InvalidKey);
            }
            Ok(Combo { mods, key: String::new() })
        }
    }
}

pub fn binding_shaped(shape: Shape, text: &str) -> Binding {
    if text.trim().is_empty() {
        return Binding::Unassigned;
    }
    match parse_shaped(shape, text) {
        Ok(c) => Binding::Assigned(c),
        Err(e) => Binding::Invalid(e),
    }
}

/// Hyprland'e gidecek somut tuşlar (çakışma karşılaştırması için, küçük harf ve eş adlar
/// birleşmiş): rakam ailesi 1…0, dokunma sol/sağ değiştirici.
fn expand(shape: Shape, c: &Combo) -> Vec<Combo> {
    let canon = |key: &str| match key.to_ascii_lowercase().as_str() {
        "prior" => "page_up".to_owned(),
        "next" => "page_down".to_owned(),
        k => match k.strip_prefix("code:").and_then(|n| n.parse::<u16>().ok()) {
            Some(n @ 10..=19) => ((n - 9) % 10).to_string(),
            _ => k.to_owned(),
        },
    };
    match shape {
        Shape::Key | Shape::Pointer => vec![Combo { mods: c.mods, key: canon(&c.key) }],
        Shape::Tap => {
            let names: &[&str] = match c.mods {
                MOD_SUPER => &["super_l", "super_r"],
                MOD_CTRL => &["control_l", "control_r"],
                MOD_ALT => &["alt_l", "alt_r"],
                MOD_SHIFT => &["shift_l", "shift_r"],
                _ => &[],
            };
            names.iter().map(|k| Combo { mods: c.mods, key: (*k).to_owned() }).collect()
        }
        Shape::Digits => (1..=10).map(|i| Combo { mods: c.mods, key: (i % 10).to_string() }).collect(),
    }
}

/// Tablodaki bir eylemin durumu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binding {
    Unassigned,
    Assigned(Combo),
    /// Dosyada elle yazılmış geçersiz değer (shell atanmamış sayar).
    Invalid(ParseError),
}

pub fn binding(text: &str) -> Binding {
    if text.trim().is_empty() {
        return Binding::Unassigned;
    }
    match parse(text) {
        Ok(c) => Binding::Assigned(c),
        Err(e) => Binding::Invalid(e),
    }
}

/// Aynı kısayolun başka bir eylemde de olması.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clash {
    /// Diğer eylemin sırası.
    pub other: usize,
    /// Bu eylem kazanıyor mu? Shell eylem sırasıyla ilkini tutar, sonrakini atar.
    pub wins: bool,
}

/// Eylem sırasıyla (shell'in `Action::ALL` sırası) verilen dizgelerdeki çakışmalar. Her eylem
/// için en çok bir kayıt: kaybeden kazananı, kazanan ilk kaybedeni gösterir.
pub fn clashes(texts: &[&str]) -> Vec<Option<Clash>> {
    let items: Vec<(Shape, &str)> = texts.iter().map(|t| (Shape::Key, *t)).collect();
    clashes_shaped(&items)
}

/// [`clashes`]'in biçimli hâli: aynı fiziksel tuşa giden somut bind'ler çakışır (rakam ailesi
/// `SUPER` ile `SUPER+1`, dokunma `SUPER` ile `SUPER+Super_L`).
pub fn clashes_shaped(items: &[(Shape, &str)]) -> Vec<Option<Clash>> {
    let combos: Vec<Option<Vec<Combo>>> = items.iter().map(|(s, t)| parse_shaped(*s, t).ok().map(|c| expand(*s, &c))).collect();
    let hit = |a: &Vec<Combo>, b: &Vec<Combo>| a.iter().any(|x| b.contains(x));
    let mut out = vec![None; items.len()];
    for (i, ci) in combos.iter().enumerate() {
        let Some(ci) = ci else { continue };
        if let Some(j) = combos[..i].iter().position(|c| c.as_ref().is_some_and(|c| hit(c, ci))) {
            out[i] = Some(Clash { other: j, wins: false });
            if out[j].is_none() {
                out[j] = Some(Clash { other: i, wins: true });
            }
        }
    }
    out
}

// ---- Tuş yakalama ---------------------------------------------------------------------------

/// Yakalayıcıya gelen bir tuş basışının sonucu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capture {
    /// Değiştirici basıldı/bırakıldı; basılı olanlar (göstermek için). Beklemeye devam.
    Held(u8),
    /// Escape (değiştiricisiz): eski değer kalır.
    Cancel,
    /// Geri silme (değiştiricisiz): atanmamış.
    Clear,
    Done(Combo),
    /// Yazı tuşu değiştiricisiz/yalnızca Shift ile: masaüstünde yazmayı bozar.
    NeedsModifier(Combo),
    /// Adlandırılamayan tuş: olayda XKB çözümü yok (sentetik olay, derlenemeyen keymap).
    Unknown,
}

/// Framework olayının basılı değiştiricileri.
pub fn mods_of(m: Modifiers) -> u8 {
    let mut mask = 0;
    if m.shift {
        mask |= MOD_SHIFT;
    }
    if m.ctrl {
        mask |= MOD_CTRL;
    }
    if m.alt {
        mask |= MOD_ALT;
    }
    if m.logo {
        mask |= MOD_SUPER;
    }
    mask
}

/// Değiştirici tuşunun keysym'i → biti (`0`: kilit tuşları gibi yok sayılanlar).
fn modifier_keysym(sym: u32) -> Option<u8> {
    Some(match sym {
        0xffe1 | 0xffe2 => MOD_SHIFT,               // Shift_L/R
        0xffe3 | 0xffe4 => MOD_CTRL,                // Control_L/R
        0xffe7..=0xffea => MOD_ALT,                 // Alt_L/R, Meta_L/R
        0xffeb..=0xffee => MOD_SUPER,               // Super_L/R, Hyper_L/R
        0xffe5 | 0xffe6 | 0xff7f | 0xfe03 | 0xfe11 | 0xff7e => 0, // Caps/Shift_Lock, Num_Lock, ISO_Level3/5_Shift, Mode_switch
        _ => return None,
    })
}

/// Bir tuş olayını yorumlar. Basma/tekrarda birleşim üretir; bırakmada yalnızca değiştirici
/// gösterimi güncellenir. `resolve_binds_by_sym`: Hyprland `input:resolve_binds_by_sym`
/// (`crate::services::hyprland`); bind tuşunun hangi düzenden adlandırılacağını seçer.
///
/// Tuş adı yalnızca olayın XKB çözümünden ([`bind_key`]) gelir. `xkb: None` (sentetik olay ya
/// da derlenemeyen keymap) adlandırılamaz → [`Capture::Unknown`]; Escape/Geri silme ve
/// değiştiriciler yine çalışır.
pub fn capture(e: &KeyEvent, resolve_binds_by_sym: bool) -> Option<Capture> {
    let pressed = e.state != KeyState::Released;
    let mut mods = mods_of(e.modifiers);
    let produced = match (&e.xkb, e.key) {
        (Some(x), _) => Some(x.sym.raw),
        (None, Key::Other(sym)) => Some(sym),
        (None, _) => None,
    };
    if let Some(bit) = produced.and_then(modifier_keysym) {
        // Değiştirici durumu olaydan sonra güncellenir: tuşun kendisini elle ekle/çıkar.
        if pressed { mods |= bit } else { mods &= !bit }
        return Some(Capture::Held(mods));
    }
    if !pressed {
        return None;
    }
    if mods == 0 {
        match e.key {
            Key::Escape => return Some(Capture::Cancel),
            Key::Backspace => return Some(Capture::Clear),
            _ => {}
        }
    }
    let Some(key) = e.xkb.as_ref().and_then(|x| bind_key(x, resolve_binds_by_sym)) else { return Some(Capture::Unknown) };
    let combo = Combo { mods, key };
    Some(if combo.blocks_typing() { Capture::NeedsModifier(combo) } else { Capture::Done(combo) })
}

/// Hyprland'in bu tuşu eşleyeceği bind tuşu: seviye-0 keysym'in xkb adı (Türkçe Q'da Shift+1 →
/// `1`, Shift+ı → `idotless`; değiştiriciler ayrı maskedir). Keysym yoksa (`NoSymbol`) ya da
/// adı bind sözdizimine sığmıyorsa tuş kodu: `code:<n>` (Hyprland bunu düzenden bağımsız eşler).
pub fn bind_key(xkb: &XkbKey, resolve_binds_by_sym: bool) -> Option<String> {
    let sym = xkb.hyprland(resolve_binds_by_sym);
    if !sym.is_none() && valid_key(&sym.name) {
        // Tek ASCII harf varsayılanlarla aynı biçimde büyük yazılır (`SUPER+N`); shell tuşu
        // küçültüp çözer, eşitlik büyük/küçük harf duyarsız.
        if sym.name.len() == 1 && sym.name.as_bytes()[0].is_ascii_alphabetic() {
            return Some(sym.name.to_ascii_uppercase());
        }
        return Some(sym.name.clone());
    }
    let code = format!("code:{}", xkb.keycode);
    valid_key(&code).then_some(code)
}

/// Değiştiricisiz atanabilen tuşlar: yazmada kullanılmayan özel tuşlar.
fn is_special_key(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    k.starts_with("xf86")
        || k.starts_with("mouse:")
        || (k.len() >= 2 && k.starts_with('f') && k[1..].bytes().all(|b| b.is_ascii_digit()))
        || matches!(k.as_str(), "print" | "pause" | "scroll_lock" | "insert" | "menu")
}

/// Tuş başlığında görünen ad (Türkçe, kısa).
pub fn key_label(key: &str) -> String {
    let lower = key.to_ascii_lowercase();
    let named = match lower.as_str() {
        "xf86audioraisevolume" => t!("settings-keybind-cap-volume-up"),
        "xf86audiolowervolume" => t!("settings-keybind-cap-volume-down"),
        "xf86audiomute" => t!("settings-keybind-cap-mute"),
        "xf86audiomicmute" => t!("settings-keybind-cap-mic-mute"),
        "xf86monbrightnessup" => t!("settings-keybind-cap-brightness-up"),
        "xf86monbrightnessdown" => t!("settings-keybind-cap-brightness-down"),
        "xf86audioplay" => t!("settings-keybind-cap-play"),
        "xf86audiopause" => t!("settings-keybind-cap-pause"),
        "xf86audiostop" => t!("settings-keybind-cap-stop"),
        "xf86audionext" => t!("settings-keybind-cap-next"),
        "xf86audioprev" => t!("settings-keybind-cap-previous"),
        "xf86calculator" => t!("settings-keybind-cap-calculator"),
        "xf86search" => t!("settings-keybind-cap-search"),
        "xf86mail" => t!("settings-keybind-cap-mail"),
        "xf86homepage" => t!("settings-keybind-cap-home-page"),
        "xf86eject" => t!("settings-keybind-cap-eject"),
        "xf86sleep" => t!("settings-keybind-cap-sleep"),
        "xf86display" => t!("settings-keybind-cap-display"),
        "xf86wlan" => t!("settings-keybind-cap-wlan"),
        "xf86touchpadtoggle" => t!("settings-keybind-cap-touchpad"),
        "mouse_down" => t!("settings-keybind-cap-wheel-down"),
        "mouse_up" => t!("settings-keybind-cap-wheel-up"),
        "mouse:272" => t!("settings-keybind-cap-mouse-left"),
        "mouse:273" => t!("settings-keybind-cap-mouse-right"),
        "space" => t!("settings-keybind-cap-space"),
        "backspace" => t!("settings-keybind-cap-backspace"),
        // tuş üstü simgeleri ve evrensel tuş adları (Enter, Esc, Delete, oklar,
        // noktalama, Ç/Ö/Ş…) fiziksel tuşu gösterir; her dilde aynı yazılır, çevrilmez.
        "return" => "Enter".to_owned(),
        "escape" => "Esc".to_owned(),
        "delete" => "Delete".to_owned(),
        "prior" | "page_up" => "Page Up".to_owned(),
        "next" | "page_down" => "Page Down".to_owned(),
        "left" => "←".to_owned(),
        "right" => "→".to_owned(),
        "up" => "↑".to_owned(),
        "down" => "↓".to_owned(),
        "print" => "Print Screen".to_owned(),
        "scroll_lock" => "Scroll Lock".to_owned(),
        "comma" => ",".to_owned(),
        "period" => ".".to_owned(),
        "slash" => "/".to_owned(),
        "backslash" => "\\".to_owned(),
        "minus" => "-".to_owned(),
        "equal" => "=".to_owned(),
        "plus" => "+".to_owned(),
        "asterisk" => "*".to_owned(),
        "semicolon" => ";".to_owned(),
        "colon" => ":".to_owned(),
        "apostrophe" => "'".to_owned(),
        "quotedbl" => "\"".to_owned(),
        "grave" => "`".to_owned(),
        "bracketleft" => "[".to_owned(),
        "bracketright" => "]".to_owned(),
        "less" => "<".to_owned(),
        "greater" => ">".to_owned(),
        "ccedilla" => "Ç".to_owned(), // i18n: tanı tuş üstü harfi
        "gbreve" => "Ğ".to_owned(), // i18n: tanı tuş üstü harfi
        "idotless" => "ı".to_owned(), // i18n: tanı tuş üstü harfi
        "odiaeresis" => "Ö".to_owned(), // i18n: tanı tuş üstü harfi
        "scedilla" => "Ş".to_owned(), // i18n: tanı tuş üstü harfi
        "udiaeresis" => "Ü".to_owned(), // i18n: tanı tuş üstü harfi
        _ => String::new(),
    };
    if !named.is_empty() {
        return named;
    }
    if lower == "i" {
        return "i".to_owned();
    }
    if let Some(b) = lower.strip_prefix("mouse:") {
        return t!("settings-keybind-cap-mouse", button = b);
    }
    if let Some(c) = lower.strip_prefix("code:") {
        return t!("settings-keybind-cap-code", code = c);
    }
    if let Some(rest) = key.strip_prefix("XF86").or_else(|| key.strip_prefix("xf86")) {
        return rest.to_owned();
    }
    if key.chars().count() == 1 {
        return key.to_uppercase();
    }
    key.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nuxut::ui::input::NamedKeysym;

    fn m(logo: bool, ctrl: bool, alt: bool, shift: bool) -> Modifiers {
        Modifiers { logo, ctrl, alt, shift, ..Default::default() }
    }

    #[test]
    fn parse_and_format_round_trip() {
        let c = parse("super+Shift+n").unwrap();
        assert_eq!(c.mods, MOD_SUPER | MOD_SHIFT);
        assert_eq!(c.to_string(), "SUPER+SHIFT+n");
        assert_eq!(parse(&c.to_string()).unwrap(), c);
        assert_eq!(parse("XF86AudioMute").unwrap().mods, 0);
        assert_eq!(parse("CTRL + ALT + Delete").unwrap().to_string(), "CTRL+ALT+Delete");
        assert_eq!(parse("WIN+code:10").unwrap().key, "code:10");
        // Sıra yazımda sabittir.
        assert_eq!(parse("SHIFT+ALT+CTRL+SUPER+x").unwrap().to_string(), "SUPER+CTRL+ALT+SHIFT+x");
    }

    #[test]
    fn parse_is_strict_like_shell() {
        assert_eq!(parse(""), Err(ParseError::Empty));
        assert_eq!(parse("HYPER+N"), Err(ParseError::UnknownModifier));
        assert_eq!(parse("SUPER+"), Err(ParseError::InvalidKey));
        assert_eq!(parse("SUPER+N, exec, rm"), Err(ParseError::InvalidKey));
        assert_eq!(parse("SUPER+N;reload"), Err(ParseError::InvalidKey));
        assert_eq!(parse(&"A".repeat(200)), Err(ParseError::TooLong));
        assert_eq!(parse("code:12345"), Err(ParseError::InvalidKey));
        assert_eq!(binding("  "), Binding::Unassigned);
        assert!(matches!(binding("SUPER+ş"), Binding::Invalid(ParseError::InvalidKey)));
    }

    #[test]
    fn equality_ignores_key_case() {
        assert_eq!(parse("SUPER+N").unwrap(), parse("super+n").unwrap());
        assert_ne!(parse("SUPER+N").unwrap(), parse("SUPER+SHIFT+N").unwrap());
    }

    #[test]
    fn clashes_follow_shell_order() {
        let c = clashes(&["SUPER+N", "", "super+n", "XF86AudioMute", "SUPER+N", "bozuk+"]);
        assert_eq!(c[0], Some(Clash { other: 2, wins: true }));
        assert_eq!(c[1], None);
        assert_eq!(c[2], Some(Clash { other: 0, wins: false }));
        assert_eq!(c[3], None);
        assert_eq!(c[4], Some(Clash { other: 0, wins: false }));
        assert_eq!(c[5], None);
        assert!(clashes(&["", ""]).iter().all(Option::is_none), "boşlar çakışmaz");
    }

    // ---- Yakalama: XKB test değerleri (keysym'ler `xkbcommon-keysyms.h`, tuş kodları evdev+8;
    // Türkçe Q değerleri framework `keysym.rs` testleriyle aynı) ----

    fn ks(raw: u32, name: &str) -> NamedKeysym {
        NamedKeysym { raw, name: name.into() }
    }

    /// Tek düzen: `base` = `first_layout_base`.
    fn xk(keycode: u32, sym: NamedKeysym, base: NamedKeysym) -> XkbKey {
        XkbKey { keycode, sym, first_layout_base: base.clone(), base }
    }

    fn ev(key: Key, modifiers: Modifiers, xkb: Option<XkbKey>, pressed: bool) -> KeyEvent {
        let state = if pressed { KeyState::Pressed } else { KeyState::Released };
        KeyEvent { key, state, text: None, modifiers, xkb }
    }

    fn down(key: Key, modifiers: Modifiers, xkb: XkbKey) -> Option<Capture> {
        capture(&ev(key, modifiers, Some(xkb), true), false)
    }

    fn done(c: Option<Capture>) -> String {
        match c {
            Some(Capture::Done(c)) => c.to_string(),
            other => panic!("Done beklendi: {other:?}"),
        }
    }

    const SUP: Modifiers = Modifiers { logo: true, ctrl: false, alt: false, shift: false, caps_lock: false, num_lock: false };
    const SUP_SHIFT: Modifiers = Modifiers { logo: true, ctrl: false, alt: false, shift: true, caps_lock: false, num_lock: false };

    #[test]
    fn capture_writes_level0_keysym() {
        let n = || xk(57, ks(0x6e, "n"), ks(0x6e, "n"));
        assert_eq!(done(down(Key::Char('n'), SUP, n())), "SUPER+N");
        // Shift'li harf N üretir; bind seviye 0 adıyla.
        let big_n = XkbKey { sym: ks(0x4e, "N"), ..n() };
        assert_eq!(done(down(Key::Char('N'), SUP_SHIFT, big_n)), "SUPER+SHIFT+N");
        // Medya ve F tuşu değiştiricisiz kabul edilir.
        let vol = xk(123, ks(0x1008_ff13, "XF86AudioRaiseVolume"), ks(0x1008_ff13, "XF86AudioRaiseVolume"));
        assert_eq!(done(down(Key::Other(0x1008_ff13), Modifiers::default(), vol)), "XF86AudioRaiseVolume");
        let f5 = xk(71, ks(0xffc2, "F5"), ks(0xffc2, "F5"));
        assert_eq!(done(down(Key::Other(0xffc2), Modifiers::default(), f5)), "F5");
        let ret = xk(36, ks(0xff0d, "Return"), ks(0xff0d, "Return"));
        assert_eq!(done(down(Key::Enter, m(false, true, false, false), ret)), "CTRL+Return");
    }

    #[test]
    fn capture_turkish_q_dotted_and_dotless_i() {
        // ı tuşu (evdev 23): ı / I.
        let dotless = || xk(31, ks(0x2b9, "idotless"), ks(0x2b9, "idotless"));
        assert_eq!(done(down(Key::Char('ı'), SUP, dotless())), "SUPER+idotless");
        let shifted = XkbKey { sym: ks(0x49, "I"), ..dotless() };
        assert_eq!(done(down(Key::Char('I'), SUP_SHIFT, shifted)), "SUPER+SHIFT+idotless");
        // i tuşu (evdev 40): i / İ.
        let dotted = || xk(48, ks(0x69, "i"), ks(0x69, "i"));
        assert_eq!(done(down(Key::Char('i'), SUP, dotted())), "SUPER+I");
        let big_dotted = XkbKey { sym: ks(0x2a9, "Iabovedot"), ..dotted() };
        assert_eq!(done(down(Key::Char('İ'), SUP_SHIFT, big_dotted)), "SUPER+SHIFT+I");
        // ı ile i ayrı kısayollardır.
        assert_ne!(parse("SUPER+idotless").unwrap(), parse("SUPER+i").unwrap());
        // ö (evdev 51) Shift'le Ö üretir.
        let o = XkbKey { sym: ks(0xd6, "Odiaeresis"), ..xk(59, ks(0xf6, "odiaeresis"), ks(0xf6, "odiaeresis")) };
        assert_eq!(done(down(Key::Char('Ö'), SUP_SHIFT, o)), "SUPER+SHIFT+odiaeresis");
    }

    #[test]
    fn capture_turkish_q_shifted_digit_and_punctuation() {
        // Shift+1 → ! üretir; Hyprland `SHIFT, 1` eşler.
        let one = XkbKey { sym: ks(0x21, "exclam"), ..xk(10, ks(0x31, "1"), ks(0x31, "1")) };
        assert_eq!(done(down(Key::Char('!'), SUP_SHIFT, one)), "SUPER+SHIFT+1");
        // Türkçe Q'da . tuşu (evdev 53) Shift'le :.
        let colon = XkbKey { sym: ks(0x3a, "colon"), ..xk(61, ks(0x2e, "period"), ks(0x2e, "period")) };
        assert_eq!(done(down(Key::Char(':'), SUP_SHIFT, colon)), "SUPER+SHIFT+period");
        // * tuşu (evdev 12) Shift'le ?.
        let question = XkbKey { sym: ks(0x3f, "question"), ..xk(20, ks(0x2a, "asterisk"), ks(0x2a, "asterisk")) };
        assert_eq!(done(down(Key::Char('?'), SUP_SHIFT, question)), "SUPER+SHIFT+asterisk");
        // Değiştiricisiz/yalnızca Shift'li rakam yazmayı bozar.
        let bare = XkbKey { sym: ks(0x21, "exclam"), ..xk(10, ks(0x31, "1"), ks(0x31, "1")) };
        let c = down(Key::Char('!'), m(false, false, false, true), bare);
        assert!(matches!(c, Some(Capture::NeedsModifier(ref c)) if c.to_string() == "SHIFT+1"), "{c:?}");
    }

    #[test]
    fn capture_second_layout_follows_resolve_binds_by_sym() {
        // us,tr; tr (grup 1) etkin, ı tuşuna Shift'le basıldı: I üretir, etkin düzende seviye 0
        // `idotless`, ilk (us) düzende `i`.
        let key = XkbKey { keycode: 31, sym: ks(0x49, "I"), base: ks(0x2b9, "idotless"), first_layout_base: ks(0x69, "i") };
        let e = ev(Key::Char('I'), SUP_SHIFT, Some(key.clone()), true);
        assert_eq!(done(capture(&e, false)), "SUPER+SHIFT+I", "varsayılan: ilk düzen");
        assert_eq!(done(capture(&e, true)), "SUPER+SHIFT+idotless", "resolve_binds_by_sym");
        assert_eq!(bind_key(&key, false).as_deref(), Some("I"));
    }

    #[test]
    fn single_ascii_letters_are_written_uppercase() {
        // Varsayılanlarla (`SUPER+N`) aynı biçim; yalnızca tek ASCII harf büyütülür.
        let one = |raw: u32, name: &str| xk(0, ks(raw, name), ks(raw, name));
        assert_eq!(bind_key(&one(0x6e, "n"), false).as_deref(), Some("N"));
        assert_eq!(bind_key(&one(0x69, "i"), false).as_deref(), Some("I"));
        assert_eq!(bind_key(&one(0x31, "1"), false).as_deref(), Some("1"));
        assert_eq!(bind_key(&one(0x2b9, "idotless"), false).as_deref(), Some("idotless"));
        assert_eq!(bind_key(&one(0xff0d, "Return"), false).as_deref(), Some("Return"));
        assert_eq!(bind_key(&one(0x1008_ff12, "XF86AudioMute"), false).as_deref(), Some("XF86AudioMute"));
        // Yazılan değer varsayılanla aynı dizge; gösterimde Türkçe noktalı i korunur.
        let c = done(down(Key::Char('n'), SUP, one(0x6e, "n")));
        assert_eq!(c, "SUPER+N");
        assert_eq!(key_label("I"), "i");
    }

    #[test]
    fn capture_falls_back_to_keycode() {
        // Seviye 0'da simge yok: tuş kodu.
        let none = XkbKey { keycode: 255, sym: ks(0x61, "a"), base: ks(0, "NoSymbol"), first_layout_base: ks(0, "NoSymbol") };
        assert_eq!(done(down(Key::Char('a'), SUP, none)), "SUPER+code:255");
        // Bind sözdizimine sığmayan ad (ve 4 haneden uzun kod): tanınamaz.
        let odd = XkbKey { keycode: 123_456, sym: ks(1, "a-b"), base: ks(1, "a-b"), first_layout_base: ks(1, "a-b") };
        assert_eq!(bind_key(&odd, false), None);
        assert_eq!(down(Key::Other(1), SUP, odd), Some(Capture::Unknown));
    }

    #[test]
    fn capture_guards() {
        let none = Modifiers::default();
        let esc = || xk(9, ks(0xff1b, "Escape"), ks(0xff1b, "Escape"));
        let bksp = xk(22, ks(0xff08, "BackSpace"), ks(0xff08, "BackSpace"));
        assert_eq!(down(Key::Escape, none, esc()), Some(Capture::Cancel));
        assert_eq!(down(Key::Backspace, none, bksp), Some(Capture::Clear));
        // XKB'siz (sentetik) olayda da iptal/temizle çalışır.
        assert_eq!(capture(&ev(Key::Escape, none, None, true), false), Some(Capture::Cancel));
        assert_eq!(capture(&ev(Key::Backspace, none, None, true), false), Some(Capture::Clear));
        // Değiştiricili Escape bir kısayoldur.
        assert_eq!(done(down(Key::Escape, SUP, esc())), "SUPER+Escape");
        // Yazı tuşu tek başına: yazmayı bozar.
        let a = xk(38, ks(0x61, "a"), ks(0x61, "a"));
        assert!(matches!(down(Key::Char('a'), none, a), Some(Capture::NeedsModifier(_))));
        let space = xk(65, ks(0x20, "space"), ks(0x20, "space"));
        assert!(matches!(down(Key::Space, none, space), Some(Capture::NeedsModifier(_))));
        // XKB çözümü olmayan tuş adlandırılamaz.
        assert_eq!(capture(&ev(Key::Char('n'), SUP, None, true), false), Some(Capture::Unknown));
        // Bırakma yalnızca değiştirici gösterimini değiştirir.
        assert_eq!(capture(&ev(Key::Char('n'), none, Some(xk(57, ks(0x6e, "n"), ks(0x6e, "n"))), false), false), None);
    }

    #[test]
    fn capture_tracks_held_modifiers() {
        let key = |code, raw: u32, name: &str| xk(code, ks(raw, name), ks(raw, name));
        // Super_L basıldı: olaydaki durumda henüz yok, tuşun kendisi eklenir.
        assert_eq!(down(Key::Other(0xffeb), Modifiers::default(), key(133, 0xffeb, "Super_L")), Some(Capture::Held(MOD_SUPER)));
        assert_eq!(down(Key::Other(0xffe1), SUP, key(50, 0xffe1, "Shift_L")), Some(Capture::Held(MOD_SUPER | MOD_SHIFT)));
        let up = ev(Key::Other(0xffe1), SUP_SHIFT, Some(key(50, 0xffe1, "Shift_L")), false);
        assert_eq!(capture(&up, false), Some(Capture::Held(MOD_SUPER)));
        assert_eq!(down(Key::Other(0xffe5), Modifiers::default(), key(66, 0xffe5, "Caps_Lock")), Some(Capture::Held(0)), "Caps Lock yok sayılır");
        // Sentetik olayda keysym `Key::Other`'dan.
        assert_eq!(capture(&ev(Key::Other(0xffe3), Modifiers::default(), None, true), false), Some(Capture::Held(MOD_CTRL)));
    }

    #[test]
    fn labels_are_readable() {
        crate::i18n::use_turkish_for_tests();
        assert_eq!(parse("SUPER+SHIFT+n").unwrap().caps(), ["Super", "Shift", "N"]);
        assert_eq!(key_label("XF86AudioRaiseVolume"), "Ses +");
        assert_eq!(key_label("xf86audiomicmute"), "Mikrofon");
        assert_eq!(key_label("XF86Tools"), "Tools");
        assert_eq!(key_label("Return"), "Enter");
        assert_eq!(key_label("comma"), ",");
        assert_eq!(key_label("i"), "i");
        assert_eq!(key_label("idotless"), "ı");
        assert_eq!(key_label("F12"), "F12");
        assert_eq!(key_label("code:10"), "Tuş kodu 10");
    }

    #[test]
    fn special_keys_need_no_modifier() {
        assert!(!parse("XF86AudioMute").unwrap().blocks_typing());
        assert!(!parse("F11").unwrap().blocks_typing());
        assert!(!parse("Print").unwrap().blocks_typing());
        assert!(parse("f").unwrap().blocks_typing());
        assert!(parse("SHIFT+Return").unwrap().blocks_typing());
        assert!(!parse("ALT+f").unwrap().blocks_typing());
    }

    /// Shell ADR-0006 biçimleri: dokunma tek değiştirici, rakam ailesi yalnız değiştirici; yazım
    /// biçimi `SUPER` (sonda `+` yok); çakışma somut tuşlar üzerinden.
    #[test]
    fn shapes_parse_display_and_clash() {
        assert_eq!(shape_of("launcher"), Shape::Tap);
        assert_eq!(shape_of("move_to_workspace"), Shape::Digits);
        assert_eq!(shape_of("window_drag"), Shape::Pointer);
        assert_eq!(shape_of("lock"), Shape::Key);
        let tap = parse_shaped(Shape::Tap, "super").unwrap();
        assert_eq!(tap.to_string(), "SUPER");
        assert_eq!(tap.caps(), ["Super"]);
        assert_eq!(parse_shaped(Shape::Tap, "SUPER+SHIFT"), Err(ParseError::InvalidKey));
        assert_eq!(parse_shaped(Shape::Digits, "SUPER+1"), Err(ParseError::InvalidKey));
        assert_eq!(parse_shaped(Shape::Digits, "SUPER+SHIFT").unwrap().to_string(), "SUPER+SHIFT");
        assert!(matches!(binding_shaped(Shape::Digits, ""), Binding::Unassigned));
        // Rakam ailesi SUPER ile tek tuş SUPER+3 çakışır; dokunma SUPER ile aile SUPER çakışmaz.
        let c = clashes_shaped(&[(Shape::Tap, "SUPER"), (Shape::Digits, "SUPER"), (Shape::Key, "SUPER+3"), (Shape::Key, "SUPER+code:14")]);
        assert_eq!(c[0], None);
        assert_eq!(c[1], Some(Clash { other: 2, wins: true }));
        assert_eq!(c[2], Some(Clash { other: 1, wins: false }));
        assert_eq!(c[3], Some(Clash { other: 1, wins: false }), "code:14 = 5");
        assert_eq!(key_label("mouse_down"), t!("settings-keybind-cap-wheel-down"));
    }
}
