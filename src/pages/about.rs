//! Hakkında: marka başlığı, sürüm ve Ayarlar'ın düzenlediği dosyalar (yolu göster, klasörü aç).
//!
//! Başlık (marka kilidi): solda Kara Sağan logosu, uygulama ikonuyla aynı karolu görünümde
//! (`BrandMark::tile`, [`LOGO_SPACE`]); blok kartın (`surface_container`) üstünde durduğundan
//! `surface` karo zeminde kaybolmaz. Yanında "Nuxut" (başlık rolü), slogan (ikincil) ve sürüm.
//! Slogan logonun yanına tek satır sığmıyorsa metin logonun altına iner ([`header_stacks`]).
//! Renkler yalnızca temadan.
//!
//! Dosya satırları standart satır kalıbında (§5.1, ayırıcısız): başlık + yol solda, "Klasörü
//! aç" sağda. Yol ev dizini `~` ile, sığmazsa baştan ("…/nuxut/shell.toml") kısaltılır ve tek
//! satırdır; başlık ya da yol düğmenin yanına sığmıyorsa ya da sütun darsa düğme metnin altına
//! iner ([`should_stack`]). Satır yüksekliği ölçülen metinden (sabit yükseklik uzun yolda
//! başlıklarla üst üste biniyordu).

use std::path::{Path, PathBuf};
use std::time::Instant;

use nuxut::ui::input::{CursorShape, PointerEvent};
use nuxut::i18n::t;
use nuxut::ui::{BrandMark, Button, ButtonVariant, Icon, IconName, Label, Mascot, TextRole, Ui, text_style};
use nuxut::{Point, Rect, Theme};

use super::fiyu::Spot;
use crate::block::{Block, Env};
use crate::registry::SettingDesc;
use crate::rows::{row_height_for, row_padding, should_stack, side_text_width};
use crate::store::{FileId, Store};

#[derive(Debug)]
struct FileRow {
    title: Label,
    path: Label,
    /// Kısaltılmamış gösterim (`~` ile); sığmazsa baştan kısaltılır ([`fit_path`]).
    full_path: String,
    dir: Option<PathBuf>,
    open: Button,
    /// Son yerleşim: satırın üstü (blok içinde göreli değil, mutlak) ve yüksekliği.
    y: f32,
    h: f32,
    /// Düğme metnin altında mı.
    stack: bool,
}

#[derive(Debug)]
pub struct About {
    desc: &'static SettingDesc,
    rect: Rect,
    logo: BrandMark,
    name: Label,
    slogan: Label,
    version: Label,
    blurb: Label,
    /// Son yerleşimde başlık metinlerinin konumları (ad, slogan, sürüm) ve yığılma.
    header_text: [Point; 3],
    header_stack: bool,
    logo_rect: Rect,
    /// Maskot Fiyu (selam), başlığın sağında; sütun darsa ya da gizliyse yok ([`Spot::About`]).
    mascot: Mascot,
    mascot_shown: bool,
    blurb_y: f32,
    files: Vec<FileRow>,
}

/// Logo kenarı (`space` birimi): 80 px — ad + slogan + sürüm üç satırının boyu.
pub const LOGO_SPACE: u32 = 20;

/// Marka adı: özel addır, hiçbir dilde çevrilmez.
const BRAND_NAME: &str = "Nuxut";

/// Logo ile metin arası (yan yana) ya da logo ile ad arası (yığılmış).
fn logo_gap(theme: &Theme) -> f32 {
    theme.space(5)
}

/// Başlık metni logonun altına iner mi: ad ya da slogan logonun yanındaki genişliğe tek satır
/// sığmıyorsa (slogan sarılıp üç satırlık kilidi bozmasın).
pub fn header_stacks(theme: &Theme, inner: f32, name_w: f32, slogan_w: f32) -> bool {
    let side = inner - theme.space(LOGO_SPACE) - logo_gap(theme);
    name_w.max(slogan_w) > side
}

/// Yolun gösterimi: ev dizini altındaysa `~/…`.
pub fn display_path(path: &Path, home: Option<&Path>) -> String {
    match home.and_then(|h| path.strip_prefix(h).ok()) {
        Some(rest) if !rest.as_os_str().is_empty() => format!("~/{}", rest.display()),
        _ => path.display().to_string(),
    }
}

/// Yolun `skip` baştaki bileşeni atılmış gösterimi: "…/nuxut/shell.toml". Dosya adı hep kalır.
pub fn shortened_path(full: &str, skip: usize) -> String {
    if skip == 0 {
        return full.to_owned();
    }
    let parts: Vec<&str> = full.split('/').collect();
    let keep = parts.len().saturating_sub(skip).max(1);
    format!("…/{}", parts[parts.len() - keep..].join("/"))
}

/// Yolu `max_w`'ye sığdırır: önce tam hâli, sığmazsa baştan bileşen atarak (dosya adı ve
/// yakın klasörler görünür kalır; sondan kesmek dosya adını gizliyordu). Son çare olarak
/// etiketin tek satır kesmesi.
fn fit_path(label: &mut Label, full: &str, text: &mut nuxut::TextSystem, max_w: f32) {
    label.set_max_width(None);
    let parts = full.split('/').count();
    for skip in 0..parts {
        label.set_text(&shortened_path(full, skip));
        if label.measure(text).width <= max_w {
            break;
        }
    }
    label.set_max_width(Some(max_w));
}

fn path_style(theme: &Theme) -> nuxut::TextStyle {
    text_style(theme, TextRole::Mono).with_max_lines(1)
}

impl About {
    pub fn new(theme: &Theme, desc: &'static SettingDesc, store: &Store) -> Self {
        let home = std::env::var_os("HOME").map(PathBuf::from);
        let files = [
            (FileId::Framework, t!("settings-about-file-look")),
            (FileId::Shell, t!("settings-about-file-shell")),
            (FileId::Palette, t!("settings-about-file-palette")),
        ]
            .into_iter()
            .map(|(id, title)| {
                let p = store.file(id).path().to_owned();
                FileRow {
                    title: Label::new(title, text_style(theme, TextRole::Body)),
                    path: Label::new(display_path(&p, home.as_deref()), path_style(theme)),
                    full_path: display_path(&p, home.as_deref()),
                    dir: p.parent().map(PathBuf::from),
                    open: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::FolderOpen)).with_label(theme, t!("settings-about-open-folder")),
                    y: 0.0,
                    h: 0.0,
                    stack: false,
                }
            })
            .collect();
        Self {
            desc,
            rect: Rect::ZERO,
            logo: BrandMark::new().tile().with_size(theme.space(LOGO_SPACE)),
            name: Label::new(BRAND_NAME, text_style(theme, TextRole::Title)),
            slogan: Label::new(t!("settings-brand-slogan"), text_style(theme, TextRole::Body)),
            version: Label::new(t!("settings-about-version", version = env!("CARGO_PKG_VERSION")), text_style(theme, TextRole::Caption)),
            header_text: [Point::ZERO; 3],
            header_stack: false,
            logo_rect: Rect::ZERO,
            mascot: Mascot::new(theme, Spot::About.mood(), Spot::About.size()),
            mascot_shown: false,
            blurb_y: 0.0,
            blurb: Label::new(t!("settings-about-blurb"), text_style(theme, TextRole::Body)),
            files,
        }
    }

    /// Yükseklik; `place` ise bileşenler `rect`'e yerleştirilir.
    fn arrange(&mut self, env: &mut Env, rect: Rect, place: bool) -> f32 {
        let t = env.theme;
        let (px, py) = row_padding(t);
        let inner = rect.width - px * 2.0;
        let header = self.arrange_header(env, Point::new(rect.x + px, rect.y + py), inner, place);
        self.blurb.set_max_width(Some(inner));
        if place {
            self.blurb_y = rect.y + py + header + t.space(6);
        }
        let mut y = rect.y + py + header + t.space(6) + self.blurb.measure(env.text).height + t.space(4);
        for f in &mut self.files {
            let s = f.open.preferred_size(env.text, t);
            // Yol sarılmaz: sığmıyorsa satır yığılır (başlık da aynı ölçüte bakar).
            let avail = side_text_width(t, rect.width, s.width);
            f.path.set_text(&f.full_path);
            f.path.set_max_width(None);
            let path_w = f.path.measure(env.text).width;
            f.stack = should_stack(t, env.text, rect.width, s.width, &mut f.title, None) || path_w > avail;
            let text_w = if f.stack { inner } else { avail.max(t.space(20)) };
            f.title.set_max_width(Some(text_w));
            fit_path(&mut f.path, &f.full_path, env.text, text_w);
            let text_h = f.title.measure(env.text).height + t.space(1) + f.path.measure(env.text).height;
            f.h = if f.stack { py * 2.0 + text_h + t.space(1) + s.height } else { row_height_for(t, text_h.max(s.height)) };
            if place {
                f.y = y;
                // Metin düğmesinin iç boşluğu metin sütununa hizalanır (yığılmışta solda).
                let r = if f.stack {
                    Rect::new(rect.x + px - t.space(2), y + py + text_h + t.space(1), s.width, s.height)
                } else {
                    Rect::new(rect.right() - px - s.width + t.space(2), y + (f.h - s.height) * 0.5, s.width, s.height)
                };
                f.open.set_rect(r);
            }
            y += f.h;
        }
        y + py - rect.y
    }

    /// Marka başlığı: logo + ad/slogan/sürüm, sağda maskot; yüksekliği döner. `origin` içerik
    /// sol üstü. Maskot ([`mascot_fits`]) sığmıyorsa ya da gizliyse yer kaplamaz; varken
    /// logo + metin grubu maskotun boyunda dikeyde ortalanır.
    fn arrange_header(&mut self, env: &mut Env, origin: Point, inner: f32, place: bool) -> f32 {
        let t = env.theme;
        let logo = t.space(LOGO_SPACE);
        let gap = logo_gap(t);
        let ms = self.mascot.measure().width;
        let shown = mascot_fits(t, inner, ms);
        let inner = if shown { inner - ms - gap } else { inner };
        for l in [&mut self.name, &mut self.slogan, &mut self.version] {
            l.set_max_width(None);
        }
        let name = self.name.measure(env.text);
        let slogan_w = self.slogan.measure(env.text).width;
        let stack = header_stacks(t, inner, name.width, slogan_w);
        // Yığılmışta metin sütun genişliğinde sarılır; yan yanayken tek satır sığar.
        let text_w = if stack { inner } else { inner - logo - gap };
        for l in [&mut self.name, &mut self.slogan, &mut self.version] {
            l.set_max_width(Some(text_w.max(1.0)));
        }
        let nh = self.name.measure(env.text).height;
        let sh = self.slogan.measure(env.text).height;
        let vh = self.version.measure(env.text).height;
        let line = t.space(1);
        let text_h = nh + line + sh + t.space(2) + vh;
        let group_h = if stack { logo + gap + text_h } else { logo.max(text_h) };
        let h = if shown { group_h.max(ms) } else { group_h };
        let origin = Point::new(origin.x, origin.y + ((h - group_h) * 0.5).round());
        let text_origin = if stack {
            Point::new(origin.x, origin.y + logo + gap)
        } else {
            Point::new(origin.x + logo + gap, origin.y + ((logo - text_h) * 0.5).max(0.0))
        };
        if place {
            self.header_stack = stack;
            self.logo_rect = Rect::new(origin.x, origin.y, logo, logo);
            self.logo.set_rect(self.logo_rect);
            let p = text_origin;
            self.header_text = [p, Point::new(p.x, p.y + nh + line), Point::new(p.x, p.y + nh + line + sh + t.space(2))];
            self.mascot_shown = shown;
            let top = origin.y - ((h - group_h) * 0.5).round();
            self.mascot.set_rect(if shown { Rect::new(origin.x + inner + gap, top + ((h - ms) * 0.5).round(), ms, ms) } else { Rect::ZERO });
        }
        h
    }
}

/// Maskot başlığın sağına sığıyor mu: yanında logo + en az `space(20)` metin kalmalı. `side` 0
/// (gizli) ise hayır.
pub fn mascot_fits(theme: &Theme, inner: f32, side: f32) -> bool {
    side > 0.0 && inner - side - logo_gap(theme) >= theme.space(LOGO_SPACE) + theme.space(20)
}

impl Block for About {
    fn section(&self) -> Option<&str> {
        Some(self.desc.section)
    }

    fn id(&self) -> Option<&'static str> {
        Some(self.desc.id)
    }

    fn height(&mut self, env: &mut Env, width: f32) -> f32 {
        self.arrange(env, Rect::new(0.0, 0.0, width, 0.0), false)
    }

    fn place(&mut self, env: &mut Env, rect: Rect) {
        self.rect = rect;
        self.arrange(env, rect, true);
    }

    fn pointer(&mut self, env: &mut Env, e: &PointerEvent) -> bool {
        let mut repaint = false;
        for f in &mut self.files {
            let r = f.open.pointer(e, env.now);
            repaint |= r.repaint;
            if r.event.is_some()
                && let Some(dir) = &f.dir
            {
                crate::services::open_dir(dir);
            }
        }
        repaint
    }

    fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.files.iter().find_map(|f| f.open.cursor(p))
    }

    fn sync(&mut self, _env: &mut Env) {}

    fn mascot(&self) -> Option<Spot> {
        Some(Spot::About)
    }

    fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.logo.set_theme(theme, now);
        self.logo.set_size(theme.space(LOGO_SPACE));
        self.mascot.set_theme(theme, now);
        self.name.set_style(text_style(theme, TextRole::Title));
        self.slogan.set_style(text_style(theme, TextRole::Body));
        self.version.set_style(text_style(theme, TextRole::Caption));
        self.blurb.set_style(text_style(theme, TextRole::Body));
        for f in &mut self.files {
            f.title.set_style(text_style(theme, TextRole::Body));
            f.path.set_style(path_style(theme));
            f.open.set_theme(theme, now);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        let t = ui.theme;
        let c = &t.colors;
        let (px, py) = row_padding(t);
        let r = self.rect;
        self.logo.draw(ui);
        if self.mascot_shown {
            self.mascot.draw(ui);
        }
        let [np, sp, vp] = self.header_text;
        self.name.draw(ui.painter, np, ui.fade(c.on_surface));
        self.slogan.draw(ui.painter, sp, ui.fade(c.on_surface_variant));
        self.version.draw(ui.painter, vp, ui.fade(c.on_surface_variant));
        self.blurb.draw(ui.painter, Point::new(r.x + px, self.blurb_y), ui.fade(c.on_surface_variant));
        for f in &mut self.files {
            let th = f.title.measure_in(ui.painter).height;
            let ph = f.path.measure_in(ui.painter).height;
            let ty = if f.stack { f.y + py } else { f.y + (f.h - th - ph - t.space(1)) * 0.5 };
            f.title.draw(ui.painter, Point::new(r.x + px, ty), ui.fade(c.on_surface));
            f.path.draw(ui.painter, Point::new(r.x + px, ty + th + t.space(1)), ui.fade(c.on_surface_variant));
            f.open.draw(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_is_shortened() {
        let home = Path::new("/home/ayse");
        assert_eq!(display_path(Path::new("/home/ayse/.config/nuxut/shell.toml"), Some(home)), "~/.config/nuxut/shell.toml");
        assert_eq!(display_path(Path::new("/etc/nuxut/shell.toml"), Some(home)), "/etc/nuxut/shell.toml");
        assert_eq!(display_path(Path::new("/home/ayse"), Some(home)), "/home/ayse");
        assert_eq!(display_path(Path::new("/home/ayse/x"), None), "/home/ayse/x");
        assert_eq!(shortened_path("~/.config/nuxut/shell.toml", 0), "~/.config/nuxut/shell.toml");
        assert_eq!(shortened_path("~/.config/nuxut/shell.toml", 2), "…/nuxut/shell.toml");
        assert_eq!(shortened_path("~/.config/nuxut/shell.toml", 9), "…/shell.toml", "dosya adı hep kalır");
    }

    /// Uzun yollar ve dar sütun: satırlar üst üste binmez, düğme metnin altına iner, her şey
    /// bloğun içinde kalır.
    #[test]
    fn long_paths_never_overlap() {
        let dir = tempfile::tempdir().unwrap();
        let deep = dir.path().join("çok/uzun/bir/klasör/yolu/ki/dar/pencerede/sığmaz/hiç/nuxut");
        let store = Store::open(deep.join("framework.toml"), deep.join("shell.toml"), deep.join("palette.toml"));
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut about = About::new(&theme, crate::registry::by_id("about").unwrap(), &store);
        let (px, _) = row_padding(&theme);
        for w in [theme.space(180), theme.space(110)] {
            let h = about.height(&mut env, w);
            about.place(&mut env, Rect::new(0.0, 0.0, w, h));
            let mut prev_bottom = 0.0f32;
            for f in &mut about.files {
                assert!(f.y >= prev_bottom - 0.5, "{w}: satırlar sırayla");
                let text_bottom = f.y + f.title.measure(env.text).height + f.path.measure(env.text).height;
                assert!(text_bottom <= f.y + f.h + 0.5, "{w}: metin satırın içinde");
                let b = f.open.rect();
                assert!(b.y >= f.y - 0.5 && b.bottom() <= f.y + f.h + 0.5, "{w}: düğme satırın içinde {b:?}");
                assert!(f.path.measure(env.text).width <= w - px * 2.0 + 0.5, "{w}: yol sütunda");
                if !f.stack {
                    assert!(f.path.measure(env.text).width <= b.x, "{w}: yol düğmeye değmez");
                } else {
                    assert!(b.y >= text_bottom - 0.5, "{w}: yığılmışta düğme metnin altında");
                }
                prev_bottom = f.y + f.h;
            }
            assert!(prev_bottom <= h + 0.5, "{w}: blok yüksekliği satırları kapsar");
        }
        assert!(about.files.iter().any(|f| f.stack), "dar sütunda uzun yol yığılır");
        // Sığmayan yol baştan kısalır: dosya adı görünür.
        let f = &about.files[1];
        assert!(f.path.text().starts_with("…/") && f.path.text().ends_with("/shell.toml"), "{}", f.path.text());
    }

    /// Marka başlığı: geniş sütunda logo solda, ad/slogan/sürüm yanında ve logoya ortalı; dar
    /// sütunda metin logonun altına iner. Her durumda başlık dosya satırlarının üstünde ve
    /// bloğun içinde; logo kenarı `LOGO_SPACE`.
    #[test]
    fn brand_header_sits_beside_or_stacks_under_logo() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let theme = Theme::default();
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let mut env = Env { theme: &theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
        let mut about = About::new(&theme, crate::registry::by_id("about").unwrap(), &store);
        // Katalog testte kurulmaz (t! anahtarı döner): gerçek uzunluktaki metinle ölç.
        about.slogan.set_text("The penguin can't fly. This one can.");
        let (px, py) = row_padding(&theme);
        let side = theme.space(LOGO_SPACE);
        for (w, stacked) in [(crate::page::content_width(&theme), false), (theme.space(70), true)] {
            let h = about.height(&mut env, w);
            about.place(&mut env, Rect::new(10.0, 20.0, w, h));
            let logo = about.logo_rect;
            assert_eq!((logo.x, logo.width, logo.height), (10.0 + px, side, side), "{w}: logo solda");
            assert!(logo.y >= 20.0 + py, "{w}: logo başlığın içinde");
            assert_eq!(about.header_stack, stacked, "{w}");
            let [name, slogan, version] = about.header_text;
            let version_bottom = version.y + about.version.measure(env.text).height;
            if stacked {
                assert!(name.y >= logo.bottom(), "{w}: yığılmışta ad logonun altında");
                assert!((name.x - logo.x).abs() < 0.5, "{w}: yığılmışta metin logoyla hizalı");
            } else {
                assert!(name.x >= logo.right() + theme.space(4), "{w}: metin logonun sağında");
                assert!(name.y >= logo.y && version_bottom <= logo.bottom() + 0.5, "{w}: metin logo boyunda");
                let mid = (name.y + version_bottom) * 0.5;
                assert!((mid - logo.center().y).abs() < 1.0, "{w}: metin bloğu logoya ortalı");
                assert!(slogan.x + about.slogan.measure(env.text).width <= 10.0 + w - px + 0.5, "{w}: slogan sütunda");
            }
            assert!(name.y < slogan.y && slogan.y < version.y, "{w}: ad, slogan, sürüm sırası");
            let header_bottom = version_bottom.max(logo.bottom());
            assert!(about.blurb_y > header_bottom, "{w}: tanıtım başlığın altında");
            assert!(about.files[0].y > about.blurb_y, "{w}: dosyalar tanıtımın altında");
            let last = about.files.last().unwrap();
            assert!(last.y + last.h <= 20.0 + h + 0.5, "{w}: blok yüksekliği her şeyi kapsar");
        }
    }

    #[test]
    fn header_stacks_only_when_text_does_not_fit_beside_logo() {
        let t = Theme::default();
        let side = t.space(LOGO_SPACE) + logo_gap(&t);
        assert!(!header_stacks(&t, side + 300.0, 80.0, 300.0));
        assert!(header_stacks(&t, side + 299.0, 80.0, 300.0));
        assert!(header_stacks(&t, side + 100.0, 120.0, 90.0), "uzun ad da yığar");
    }

    /// Maskot (selam, 128): geniş sütunda başlığın sağında, logo/metinle çakışmaz, blokta kalır;
    /// dar sütunda ya da `[appearance] mascot = false` iken yok ve yer kaplamaz.
    #[test]
    fn mascot_greets_beside_header_and_hides_without_space() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("framework.toml"), dir.path().join("shell.toml"), dir.path().join("palette.toml"));
        let mut text = nuxut::TextSystem::new();
        let mut actions = Vec::new();
        let shown = Theme::default();
        let hidden = crate::pages::fiyu::tests::hidden_theme();
        let w = crate::page::content_width(&shown);
        let mut heights = Vec::new();
        for theme in [&shown, &hidden] {
            let mut env = Env { theme, store: &store, text: &mut text, now: Instant::now(), actions: &mut actions };
            let mut about = About::new(theme, crate::registry::by_id("about").unwrap(), &store);
            assert_eq!(about.mascot(), Some(Spot::About));
            assert_eq!(about.mascot.mood(), nuxut::ui::Mood::Hello);
            assert_eq!(about.mascot.size(), 128.0);
            let h = about.height(&mut env, w);
            about.place(&mut env, Rect::new(0.0, 0.0, w, h));
            heights.push((h, about.blurb_y));
            if theme.appearance.mascot {
                assert!(about.mascot_shown);
                let (m, _) = about.mascot.device_rect(1.0);
                let [_, slogan, _] = about.header_text;
                assert!(m.x >= slogan.x + about.slogan.measure(env.text).width, "maskot metnin sağında: {m:?}");
                assert!(m.right() <= w + 0.5 && m.y >= 0.0 && m.bottom() < about.blurb_y, "maskot başlıkta: {m:?}");
                // Dar sütunda düşer.
                let narrow = shown.space(70);
                let nh = about.height(&mut env, narrow);
                about.place(&mut env, Rect::new(0.0, 0.0, narrow, nh));
                assert!(!about.mascot_shown, "dar sütunda maskot yok");
            } else {
                assert!(!about.mascot_shown, "gizli");
            }
        }
        // Gizliyken başlık maskot boyundan logo boyuna iner (boşluk kalmaz). Yalnız başlık
        // (blurb'ün yeri) karşılaştırılır: alttaki yol satırları süreç ortamından (XDG_*) gelir
        // ve paralel testler onu değiştirebilir — toplam yükseklik kararlı değil.
        let ((_, bs), (_, bh)) = (heights[0], heights[1]);
        let drop = 128.0 - shown.space(LOGO_SPACE);
        assert!((bs - bh - drop).abs() < 1.0, "{bs} {bh}");
    }
}
