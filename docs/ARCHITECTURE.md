# nuxut-settings — Mimari

Nuxut'un Ayarlar uygulaması. `nuxut-framework` üzerine kurulu tek pencereli (xdg_toplevel)
bir uygulama; `framework.toml` ve `shell.toml`'u düzenler, takvim hesaplarını
`org.nuxut.Calendar1` üzerinden yönetir, duvar kağıdından renk paleti üretir.

**Kullanıcı sözü:** kolay ve güzel. Her değişiklik **anında** uygulanır ("Uygula" yok); her
şey geri alınabilir; renk seçerken palet açılır; hesap eklemek tek soru/tek tık.
Öncelik sırası: estetik, akıcılık, hız.

## 1. Süreç ve bağımlılıklar

```
nuxut-settings (tek paket: lib + bin)
├── nuxut (framework cephesi, path)      pencere, çizim, bileşenler, tema
├── nuxut-palette (path, yalnız `image`)  duvar kağıdından palet (arka plan thread'i)
├── nuxut-image (path, yalnız `render`)  duvar kağıdı küçük resmi (WallpaperDecoder + doku; jxl yok)
├── nuxut-image-sandbox (path)           yalıtılmış işçi: tema SVG'leri + duvar kağıdı (§7)
├── nuxut-calendar-ipc (path)            takvim D-Bus sözleşmesi (zbus, serde; ağ yok)
├── nuxut-nm (path, ../shell/crates)     NetworkManager istemcisi (kabukla ortak, UI'sız; ajansız)
├── toml_edit                            biçim/yorum koruyan config yazımı
├── png                                  profil resmi PNG kodlama (Kullanıcı hesabı)
├── ipp (yalnız protokol)                CUPS IPP istek/yanıtı (Yazıcılar; HTTP/1.1 yerel sokete kendimiz)
└── zbus                                 takvim + xdg-desktop-portal FileChooser + systemd-timedated + AccountsService
```

- Tek örnek: `nuxut::wayland::ipc::try_single_instance("settings")`. İkinci çağrı
  (`nuxut-settings [sayfa]`) birincile `open <sayfa>` gönderir ve çıkar.
- Shell'e ya da takvim daemon'una hiçbir kod bağımlılığı yoktur: shell ile sözleşme
  `shell.toml` dosyasıdır (shell inotify ile izler), takvimle D-Bus.
- `#![forbid(unsafe_code)]`.

**Kurulum dosyaları (`dist/`)** — Ayarlar kendisi kurmaz; shell `install.sh` kopyalar:
- `dist/nuxut-settings.desktop` → `~/.local/share/applications/nuxut-settings.desktop`
  (`Exec=nuxut-settings`; kurulumda mutlak yola çevrilir:
  `sed "s#^Exec=nuxut-settings#Exec=$BIN/nuxut-settings#"`).
  `StartupWMClass=nuxut-settings` = pencere app-id'si (`SETTINGS_APP_ID`), `Icon=nuxut-settings`,
  `Categories=Settings;DesktopSettings;`, `NoDisplay` yok (menüde tek girdi). Hoş geldin
  penceresi (`nuxut-welcome`) için gizli girdi `dist/nuxut-welcome.desktop` (`NoDisplay=true`,
  `StartupWMClass=nuxut-welcome`, aynı ikon; aynı yere, aynı `sed` ile). `desktop-file-validate` temiz.
- `dist/icons/hicolor/scalable/apps/nuxut-settings.svg` →
  `~/.local/share/icons/hicolor/scalable/apps/nuxut-settings.svg` (ardından varsa
  `gtk-update-icon-cache -qtf ~/.local/share/icons/hicolor`). Framework'ün renkli karolu marka
  SVG'sinin (`crates/nuxut-ui/assets/brand/kara-sagan.svg`, `nuxut::ui::BRAND_SVG`) birebir
  kopyası + başlık yorumu; renkler sabit (uygulama ikonu temadan bağımsız; mono `currentColor`
  sürümü `-symbolic` içindir). `tests/dist.rs` kopyanın kaymadığını ve .desktop alanlarını sınar.
- Kaldırmada ikisi de silinir.

## 2. Pencere ve gezinme

```
┌──────────────────────────────────────────────────────────────────────┐
│ Ayarlar               │  Görünüm                    ↶ ⟲  ─  □  ✕   │  ← başlık çubuğu (CSD,
│ ⌕ Ayarlarda ara       │                                              │    begin_move)
│ Kişisel               │  ┌ Önizleme (hero) ─────────────────────┐    │
│ ◐ Görünüm        ●    │  │ canlı kart: bar, panel, düğme, metin  │    │
│ ▣ Duvar kağıdı        │  └───────────────────────────────────────┘    │
│ Tт Yazı · ≈ Hareket   │  Renkler                                      │
│ Masaüstü              │  ┌───────────────────────────────────────┐    │
│ ▭ Bar · ▤ Paneller    │  │ Kaynak        [ Sabit | Duvar kağıdı ] │    │
│ ◻ Pencereler · ▢ Ekr. │  │ Vurgu rengi                       ■ ⌄  │    │
│ ⌘ Kısayollar          │  └───────────────────────────────────────┘    │
│ Sistem                │  ← sütun ortalı, tüm sayfalarda azami 1040    │
│ ◔ Bildirimler · ♪ Ses │                                               │
│ ◎ Bölge ve dil · …    │                                              │
└───────────────────────┴───────────────────────────────────────────────┘
```

- Pencere: `WindowSpec { app_id: "nuxut-settings", size: 1040×720, min 720×480 }`. Başlık
  çubuğu ve kenar boyutlandırma uygulamanın (framework CSD çizmez). `fullscreen` dışında
  kenarlardan 6 px boyutlandırma.
- Hoş geldin penceresi ayrı sınıfta: `WindowSpec { app_id: "nuxut-welcome", size 720×640,
  min = max = 720×640 }` (`app::WELCOME_APP_ID`, `WELCOME_SIZE`). `min_size = max_size`
  ipucu sabit boyut demektir; sway gibi compositor'lar böyle pencereyi yüzer açar (canlı:
  sway `floating_con`, ortalı). Hyprland'de yüzer + ortala kuralı shell'de bu sınıfa yazılır.
  Kenardan boyutlandırma yok. Hoş geldin **ayrı ikinci pencere**dir (framework çoklu
  toplevel): `nuxut-settings welcome` süreci yalnızca Hoş geldin'le başlatır; IPC `open
  [sayfa]`/`flow` Ayarlar penceresini yanına açar, IPC `welcome` çalışan Ayarlar'ın yanına
  Hoş geldin'i açar (açıksa bir şey yapmaz). Pencereler bağımsız kapanır (bekleyen yazımlar
  önce diske iner; Hoş geldin kapanınca sonuç shell'e bildirilir); son pencere kapanınca
  süreç 0 ile çıkar. Ayarlar penceresi yokken geri al şeridi Hoş geldin'de gösterilir.
- **Hyprland ana ayarı devralma** (shell ADR-0007 eki, kullanıcı onayı 2026-10-17): Hoş geldin
  "Masaüstüyle ortak" adımında en üstte "Hyprland ayarımı Nuxut yönetsin" anahtarı
  (`pages/hyprland_config.rs`), yalnızca Hyprland oturumunda ve ana dosya (`hyprland.lua` →
  `hyprland.conf`) henüz Nuxut işaretini taşımıyorsa; seçili gelir. Seçim dosyaya yazılmaz
  (paylaşılan hücre); Bitir'de shell `setup done`'u aldıysa `nuxut-shell hypr takeover` ayrık
  çalışır. Sonuç ve tek tıkla geri alma shell bildiriminde ("Geri al" = `hypr restore-backup`).
  Test örneğinde ve shell yokken çalışmaz.
- **Kenar çubuğu** (sol, standart dikey gezinme — eski projenin `StackSidebar` dersi):
  üstte arama alanı, altında sayfa listesi (`nuxut_ui::List`, seçili öğe vurgulu). Sıra ve
  gruplar `docs/design/settings-ux.md` §2.2 (t094): **Kişisel** (Görünüm, Duvar kağıdı, Yazı,
  Hareket) · **Masaüstü** (Bar, Paneller, Pencereler, Ekranlar, Klavye, Fare ve dokunmatik yüzey, Kısayollar) · **Bağlantılar** (Wi-Fi, VPN, Bluetooth — karar, 2026-10-13: Sistem grubu uzadı, bağlantılar ayrı bir zihinsel küme; `NavGroup::Connections`) · **Sistem**
  (Bildirimler, Ses, Güç, Yazıcılar, Bölge ve dil, Tarih ve saat, Varsayılan uygulamalar, Kullanıcı hesabı, Hesaplar, Hakkında). Sıra `PageId::ALL` (enum bildirim
  sırasıyla aynı; arama sonuçları `Ord` ile aynı sırada), grup `PageId::group`, başlıklar
  `registry::nav_groups(&nav_pages)` → framework `List::with_groups`; yoğun satır `ListDensity::Compact`
  (`space(10)`). Grup başlığı ve satır ölçüleri framework'ündür, Ayarlar'da kopya yok.
  Veriye bağlı sayfalar veri yokken kenar çubuğunda ve aramada görünmez (`PageId::listed`,
  `registry::nav_pages`; Güç yalnız servis açılamazsa gizli — veri yoksa sayfada boş durum,
  2026-10-15): gezinme ve PageUp/Down atlar,
  gizli sayfayla açılış ilk sayfaya düşer. Liste pencere kurulurken bir kez alınır.
- **İkon şeridi** (§4): pencerenin mantıksal genişliği `space(225)` (900 px) altındaysa kenar
  çubuğu framework `List` şerit kipine (`set_rail`, `space(16)` = 64 px, ipucunda sayfa adı)
  `spatial` yayıyla iner; genişlik her karede `List::animated_width` (`Settings::sync_rail`,
  `side_w`), içerik alanı onu izler. Başlık ("Ayarlar") ve arama alanı geçişin ilk yarısında
  söner, yerine arama düğmesi belirir. Düğmeye basmak (ya da Ctrl+F, yazmaya başlamak) aramayı
  odaklar ve kenar çubuğunu tam genişliğe açar (alan ve sonuçlar okunsun); odak çıkıp metin boşken
  şeride döner. Ölçek bağımsız: 1.5× 1140 px fiziksel = 760 mantıksal → şerit.
- **İçerik**: `ScrollView` içinde bölüm başlıkları (`TextRole::Label`, `on_surface_variant`;
  vurgu rengi yalnızca seçili/etkin öğeye) + bölüm kartları (`surface_container`,
  `radius_medium`, kenarsız), kart içinde satırlar; **satırlar arasında ayırıcı yok** (boşlukla
  ayrılır, açılır satırın içinde de). Sütun **yatayda ortalı** ve TÜM sayfalarda tek azami
  genişlik `page::content_width` = `space(260)` = 1040 px (kullanıcı kararı; t094 karar 3'ün
  sola hizası ve metin/görsel sayfa ayrımı geri alındı): sayfa değişince sütun kenarı
  oynamaz. Dar viewport'ta iki yanda en az `space(6)` (`page::column`). Sayfa geçişi: eski içerik `motion.effects` ile
  söner, yenisi `spatial_fast` yayıyla 12 px aşağıdan yerine oturur.
- **Arama**: kenar çubuğundaki alana yazınca sayfa listesi yerini sonuçlara bırakır (bkz. §4
  kayıt). Sonuca tıklama → sayfaya git, satıra kaydır, satır `primary` %12 ile kısa süre
  parlar. Escape aramayı temizler.
- Klavye: Ctrl+F arama, Ctrl+Z / Ctrl+Shift+Z geri al / yinele, Ctrl+W ya da Escape (arama
  boşken) kapat, Ctrl+PageUp/PageDown sayfa değiştir, Tab ile odak gezinmesi.

### Sayfalar

| Sayfa | Dosya | İçerik |
|---|---|---|
| Görünüm | framework.toml + palette.toml | hero: canlı önizleme (gerçek duvar kağıdı, yerleşimi ve kenar zeminiyle; seçilmemişse temadan sahte geçiş). **Renkler** (t094 W3, §9): koyu/açık (`[colors] mode`, `CustomId::DarkMode`: Otomatik varsayılan = anahtar yok, renkler duvar kağıdından geliyorsa paletin kipini izler; Koyu/Açık duvar kağıdı paletinde `meta.mode` ile aynı geri al adımında — `palette.mode` kimliği ayrı satır değil, bu satırın eşanlamlısı `CustomId::Alias`, karar 5), **Yüksek karşıtlık** (`[colors] contrast`, framework t124 `accessibility.md` §1; dize değerli anahtar satırı `Control::Toggle { on: Some("high") }`: açık = `"high"`, kapalı = anahtar silinir → varsayılan `"standard"`; framework şeması anahtarı tanımıyorsa — `defaults` varsayılanı yok — satır gizli, `hour_cycle` kalıbı; tema değişimi çalışan her Nuxut sürecine canlı akar; Hoş geldin'e eklenmedi; `look::selection_key`'e girer — shell GTK/Qt renklerini `Theme::contrast` ile yeniden yazar, durum yeniden sorulur; snapshot kimliği yok: shell `SettingId` kapalı listesinde `look.contrast` bulunmuyor, ⟲ gösterilmez, ↺ ve geri al yeterli), "Renkler nereden gelsin" (`colors.source` Nuxut teması / duvar kağıdından), Nuxut temasındaysa **Renk teması** (`colors.theme`, `CustomId::ColorTheme`, `pages/color_theme.rs`; kullanıcı isteği 2026-10-16: altı önizleme kartı Nuxut mavisi · Nötr · Adaçayı · Gül kurusu · Kehribar · Grafit, kart = temanın etkin kipteki paletiyle bar + panel, zemin seçili Nuxut yerleşiği o temaya boyanmış hâliyle — shell masaüstüyle aynı `render_preview_themed` — dosya duvar kağıdında temanın yüzeyi; varsayılan mavi anahtarı siler; oklar/Home/End/Enter; Duvar kağıdı sayfasındaki yerleşik kartları ve önizleme de etkin temaya boyanır), duvar kağıdından ise **Şema** (9 varyant kartı, dengeli 5+4 / 3×3 ızgara) ve **Palet kontrastı** (`palette`, `palette.contrast`; Duvar kağıdı sayfasından taşındı; 2026-10-13: "Yüksek karşıtlık"tan ayrışsın diye "Kontrast" adı değişti, "kontrast" araması ikisini de bulur), vurgu rengi; diğer 10 rol tek açılır satırda "Renkleri tek tek ayarla" (`Tier::Detail`; özet "10 renk · n'i elle ayarlı", nokta rozeti; arama açar). **Köşeler ve şekil** (3 yarıçap, kenar çizgileri — alt metin "Bileşen sınırları; yüksek karşıtlık açıkken en az ince çizilir", yüksek karşıtlıkta "Yok" seçiliyken duruma bağlı alt metin "Yüksek karşıtlık açık: ince çiziliyor" (`SettingDesc::subtitle_in`; framework `none → subtle`; 2026-10-13), birleşim kavsi, esneme); **Gölge** (aç/kapa + koşullu 3); **Uygulamalar** (shell ADR-0004 birleşik görünüm): "Uygulamalar da Nuxut gibi görünsün" (`[desktop] unified`), shell durumu (`msg look status` v3, shell ADR-0004 §17-5: uygulandı / eksik paket + düzeltme düğmesi — ikon/imleç hedefli paketler (`missing_packages[].target`) ızgaralarda gösterildiği için genel notta sayılmaz, hedefsizler (v2) sayılır; resmî depo paketi "Kur" düğmesi (`pages/fix_button.rs`, `services/packages.rs`: `nuxut-shell packages install`, kök yardımcının `packages` alanı, yönetici onayı; kullanıcı kararı 2026-10-17 — kök komutu yazdırılmaz), AUR paketi kopyalanacak komut, kurulu yardımcıyla (`look::aur_helper`: PATH dizinlerinde çalıştırılabilir paru, yoksa yay; süreç başlatılmaz, oturumda bir kez aranır; ikisi de yoksa `paru -S` + "AUR yardımcısı gerekir (paru ya da yay)." notu — genel notta ve ızgara altında aynı) / `writer = "disabled"` iken sakin not / kabuk çalışmıyor / v5 `last_apply.external`: genel "aktarılamadı" yerine dosya başına yol + neden (başka araç yönetiyor · başka kullanıcıya ait — kök komutu önerilmez, komut düğmesi yok (kullanıcı kararı 2026-10-17) · Nix/Guix · klasör yazılamıyor) + etki ("Bu dosyayı okuyan uygulamalar Nuxut renklerini almıyor"), ilk iki benzersiz dosya, kalanı "N dosya daha"; gerçek hata (`errors`) varsa yine genel metin; yalnızca `paused` varsa "birkaç dakikalığına durduruldu"; "Nuxut yönetsin" düğmesi framework `take_over` bekliyor, shell `docs/design/look-external.md`), ikon ve imleç teması **kart ızgarası** (`theme_grid.rs`; açılır seçimin yerini aldı; aile, paket ve dizin adı tahmini yok): ilk kart "Önerilen" = anahtar yok, adı shell `icons/cursors.recommended`'dan (Ayarlar kopya tutmaz; shell yoksa adsız), önerilen tema ayrıca kart olmaz (önerilen kurulu ve ailesinin birden çok kurulu varyantı varsa önerilen kart o ailenin kartıdır); sonra `icons/cursors.catalog` ailelerinden **aile başına tek kart** katalog sırasıyla — çok varyantlı ailenin kartı başlıkta aile adı, alt yazıda seçiliyken etkin varyant, değilken "n varyant"; önizleme etkin varyantta (dosyadaki, önerilen kartta öneri, yoksa ilk); seçiliyken ızgaranın altında "‹Aile› varyantı" seçicisi (2–5 varyant `SegmentedButton`, sığmazsa ya da daha çoksa `Select`, dar satırda etiketin altına iner; varyant ortak baş/son sözcükleri atılmış kısa adla, temel varyant "Standart"); varyant seçmek o dizin adını yazar (önerilen varyant anahtarı siler), seçili aile kartına yeniden tık varyantı değiştirmez; tek varyantlı aile tekil kart; katalog dışı kurulu temalar, dosyada yazılı ama kurulu olmayan seçim (soluk); en sonda kurulu olmayan aileler soluk "Kurulu değil[ · AUR]" kartı (indirme ikonu) — seçilmez, config'e yazmaz; tık/Enter ızgara altında ailenin düzeltmesini gösterir (depo: "Kur", AUR: komut ve "Kurulum komutunu kopyala"); seçili tema kurulu değilse ızgara altında not (önerilen ailenin paketi katalogda biliniyorsa komutuyla); eski shell (katalog yok) ya da shell yok: yalnızca önerilen + kurulu temalar, komut yok; oklar/Home/End/Enter, Tab varyant seçicisine, sonra kopyalama düğmesine; önizleme yuvası (`theme_card_preview.rs`, t103): kurulu temanın kartında framework `CursorPreview`/`IconThemePreview` (öğe 24 px; şerit kaç öğe sığıyorsa yalnızca onları ister; imleç sırası `wait` öne alınmış — dar kartta da animasyonlu bir öğe olsun; animasyon yalnızca işaretçi kartın üzerindeyken, `wake_at` ile), kurulu önerilende önerinin kendisi; kurulu olmayan kart tür/indirme ikonlu yer tutucu; kart `surface_bright`, yuva `surface` kuyusu. Yükleme `services/previews.rs`: süreç geneli `PreviewCache` (8 MiB LRU) + düşük öncelikli `Loader` (2 thread, nice 19, ilk istekte başlar), ızgara başına teslim kutusu, sonuç `services::wake` ile; önce önbellek. Ölçek değişince (`painter.scale()`) yuvalar `Loading`'e döner ve yeni px ile yeniden istenir (anahtar px taşır, eski boyutun geç sonucu eşleşmez); seçim değişip kartlar yeniden kurulunca aynı temanın önizlemesi taşınır; yüzey dışındaki kartın önizlemesi çizilmez ve istenmez. Kullanıcı dizinindeki SVG tema dosyaları (ADR-0007) framework'ün `nuxut-image-worker` yalıtılmış işçisinde çözülür (`Loader::with_sandbox` + `locate`; işçi tek kullanımlık, el sıkışma sürümlü — Ayarlar ile aynı framework'ten derlenmeli; Ayarlar'da `[[bin]]` yok, kurulum shell install.sh ile ortak `~/.local/lib/nuxut`). İşçi yoksa o yuvalar yer tutucu kalır; SVG süreç içinde asla çözülmez. İş anahtarları (`load_with_keys`): iptal = tema+rol+ızgara — aynı ızgaradaki yeni istek eskisini iptal eder, Hoş geldin ile Görünüm birbirini iptal etmez; kaynak = tema dizini (kimlik) — susturma, negatif kova ve hız sınırı tema başına ve ızgaralar arasında ortak: art arda bozuk dosya veren tema tüm rolleriyle susturulur, başka tema etkilenmez; iptal/geçici yalıtım hatası (`preview::outcome` → None) önbelleğe yazılmaz. Kurulu tema taraması (`services/look.rs::scan`) framework `icon_themes_in`/`cursor_themes_in` üzerine yalnızca seçici süzgecidir (`Hidden=true` ikon ve imleç temaları ve `hicolor` dışarıda, Adwaita yedek olarak içeride, kimlik `valid_theme_name`, Türkçe katlamalı sıra). Liste arka planda bir kez taranır; shell `msg look status` sürüm 4 `themes_rev` (kurulu tema izleyicisi) son görülenden farklıysa (azalma dahil: shell her başlangıçta 0) yeniden taranır — eski liste yenisi gelene dek kalır, önizleme önbelleği boşalır ve ızgaralar kuşak değişiminde yuvalarını yeniden ister (yarım kurulumdan kalan "yok" sonuçlar kalmaz). Durum, görünüm bir kez sorulduysa pencere klavye odağını geri alınca yeniden sorulur (`look::recheck`; yoklama yok): terminalde kurulan tema Ayarlar yeniden açılmadan görünür. Birleşik görünüm kapalıyken durum satırı, ikon/imleç ızgaraları, imleç boyutu ve Qt uygulamaları (`registry::UNIFIED_GATED`, hepsinin `visible_if`'i `unified_on`; shell kapalıyken boş plan, hiçbirini uygulamaz) gizlenir; anahtarın hemen altında "Kapalıyken uygulamalar kendi temalarını kullanır." + tek tık "Birleşik görünümü aç" düğmesi (`look::UnifiedOffNote`, `pages::unified_off_note`; anahtar satırının yazdığı `true`, aynı geri al etiketiyle). Hoş geldin Uygulamalar adımı aynı koşul ve aynı açıklama bloğunu kullanır (düğmesiz: anahtar hemen üstünde). Aramada gizli satır bulunursa sonuç `desktop.unified` satırına gider (tek sonuç, tekrar yok); açıkken satırların kendisi. Birleşik görünümün varsayılanı framework şemasından (`DesktopConfig::default().unified`; yerel sabit yok; framework.toml bozuksa çalışma zamanıyla aynı `Theme::broken_config()` değeri — kapalı), imleç boyutu, Qt uygulamaları (birleşik görünüm açıkken). ⟲ "önceki hâline dön" satır kalıbında ↺'nın yanında. Ayarlar yalnızca framework.toml yazar; GTK/Qt/ikon/imleç hedeflerini shell uygular |
| Yazı | framework.toml | ADR-0006: canlı tipografi önizlemesi (her rolün örneği + ikonlar); **Yazı tipleri** tek kart (t094 W4): taban boyut + rol başına (arayüz, başlık, belge, eş aralıklı) yazı tipi seçici (aile kendi fontuyla; kurulu değilse "X kullanılıyor · Arch paketi"; açılınca framework `FontPicker`: arama, Tümü/Eş aralıklı süzgeci, sanal aile listesi, ailenin bulunan ağırlıkları ya da değişken ailede `wght` kaydırıcısı, önizleme — aile ve kalınlık tek geri alma adımında yazılır, varsayılana eşit olan silinir; liste `typography::listed` ile rol başına süzülür, ikon rolünde seçili setin fontları; liste tekerleği sayfayı kaydırmaz — `Block::wants_scroll`); kartın sonunda kapalı açılır satır "Boyut oranları ve ikon yazı tipi" (rol boyut oranları, ikon yazı tipi ve boyut oranı); **İkonlar** kartı: ikon seti (Material Symbols / Nerd Font kartları, aynı ikonlar yan yana, font kurulu mu). "Varsayılana dön" rolün üç anahtarını siler |
| Hareket | framework.toml | hero: karakter kartları (`motion.preset`: Sakin/Canlı/Çevik/Kapalı, her birinde imleç girince bir kez oynayan mini panel önizlemesi; rol/tür elle yazılmışsa "Özelleştirilmiş" rozeti + "Karaktere dön": `motion.style` ve rol anahtarlarını tek geri al adımında siler, hız/aç-kapa kalır); **Genel**: hareketi azalt (`motion.enabled`; anahtar etkin durumu gösterir — `registry::reduced_motion`, "Kapalı" ön ayarında da açık; açmak `enabled = false`, kapatmak `enabled` varsayılana (açık; varsayılan yazılmaz) döner ve ön ayar `instant` ise `motion.preset`'i de aynı geri al kaydında siler → varsayılan ön ayar (`SettingDesc::toggle_writes`, `Action::Group`); 2026-10-13), hız çarpanı; kartın sonunda kapalı açılır satır "Hareketi tek tek ayarla": hareket türü (yay/eğri) ve rol başına alt bölüm (karar 6 adları: Paneller ve pencereler = `spatial`, Menüler ve küçük öğeler = `spatial_fast`, Solma ve renk = `effects`; rol adı editörün başlığı, kayıtta rolün bölümü — arama sonucunda rol ayırt edilir): tür, yay (süre/yaylanma) ya da eğri (adlandırılmış eğri açılır seçimle + cubic-bezier editörü, süre); eğri grafiği + canlı oynayan önizleme topu. Hareket "Kapalı"yken (`registry::reduced_motion`: `[motion] preset = "instant"` ya da `enabled = false` — framework `Theme::reduced_motion()` ile aynı kural, testte framework çözümüyle karşılaştırılır; store'dan, kaydedilmemiş seçim/geri al hemen yansır) grafik, önizleme topu ve sürükleme ipucu gizlenir; rol başlığı ve özet ("Yay · 0,38 sn …") alt bölümün adı olarak kalır, rol satırları (tür, süre, yaylanma) düzenlenebilir kalır. Karar: 2026-10-13 — Kapalı'da eğri önizlemesi anlamsız, gizli (framework `accessibility.md` §2 "Bilinçli istisnalar") |
| Erişilebilirlik | framework.toml + shell.toml + kabuk IPC (`msg magnifier`, `msg osk`) | 2026-10-16, ayrıntı `docs/design/accessibility-page.md`: Kişisel grubunun sonunda. **Görme**: Yüksek karşıtlık, İmleç boyutu, Yazı boyutu, Hareketi azalt — kendi sayfalarındaki satırların aynası (`mirror [Accessibility]`; önünde birincil satır olmayan ayna `PageId::mirror_home` kartına girer). **Büyüteç**: canlı aç/kapa (`services::assist`, magnifier-v1; `on`/`off`, Hyprland'e doğrudan yazılmaz), `[magnifier] factor/step/max` kaydırıcıları, Kısayollar'a yönlendirme (alt metinde tuşlar, `SettingDesc::subtitle_in`). **Ekran klavyesi**: canlı göster/gizle (osk-v1 `show`/`hide`), `[osk] layout` (segmentli) ve `key_height`, kısayol yönlendirmesi (`keybinds.osk_toggle`; satırı Kısayollar sayfasında). Durum sayfa açılınca, odak dönünce ve sayfa görünürken 2 sn arayla sorulur; değişince `Event::Assist` |
| Bar | shell.toml | settings-ux §6.1 öneri A: **hero = bar önizlemesi** (`bar_preview.rs`; masaüstü küçük resmi, bar config'teki kenarda, dikeyde dikey; widget hapları bar sırasıyla, tıkla/oklar = seç; sığmazsa yalnızca ikon; uca özel eylem atanmış uç hapında nokta); **Seçili widget** kartı (seçim `PageUi::selected_widget`, config'e yazılmaz; yokken sağ uç — karar 4): başlık (ikon, ad, bar'daki yeri, hitapsız) + şemada olup kayıtta satırı olmayan widget ayarları (`WidgetSpec.settings`'ten dinamik `SettingRow`, `vertical_*` yalnızca dikey bar'da) + kayıttaki widget ayarları (`ui_if`: o widget seçili) + widget uçtaysa sol/sağ tık/tekerlek açılır seçimle (`ui_if`: seçili widget o uçta; uç shell gibi `bar.start`'ın ilki / `bar.end`'in sonuncusu) + tekerlek adımı (yalnız o ucun tekerleği ses/mik/parlaklıkken); Widget'lar (bölümler: ekle/çıkar/sürükle; kenarsız kuyular; adlar sığmazsa sütunlar alt alta; satıra tık = seç ve karta kay); Konum ve boyut (kompakt kenar şeması tek satırda + kalınlık/uç boşluğu/aralık); **son kart Çerçeve** (karar 1; bar çerçevenin kalın kenarıdır): çerçeve aç/kapa, genel kalınlık (bar dışındaki kenarlar), dört kenar (şema + kaydırıcı: bar kenarı `bar.size`, elle ayarlı kenar `frame.<kenar>` + "Genele bağla"); iç köşeler temanın `blob.smoothing`'i (ADR-0008) — Görünüm › Köşeler ve şekil'e yönlendiren satır (`CustomId::Link`). Eski `nuxut-settings shell` slug'ı buraya gelir (geri uyum) |
| Paneller | shell.toml | kenar paneli başına bir kart (§7; Sağ, Sol, Üst, Alt; hero yok — karar 7): "<Kenar> paneli kullan" → kısayol (yalnızca sağ/sol; Kısayollar'daki satırın **aynası**, §3.4) → genişlik/yükseklik → içerik (widget listesi) → "Bar'dan açma" (`pages/panel_hint.rs`: bar'ın bir uç eylemi `panel:<kenar>` ise "Sağ uçtaki widget (Oturum) · sol tık" + "Ayarla" o uç eyleminin satırına gider; değilse gizli). Kapalı panel tek satır |
| Duvar kağıdı | shell.toml + palette.toml | hero (§5.4, en üstte: önce seçili olan görünür): ana monitörün oranında büyük önizleme — görsel, yerleşim ve kenar zemini ayar değişince anında görünür; ad/klasör tek satır, "n monitörde ayrı görsel var", "Değiştir…" (portal FileChooser) → `wallpaper.path` + palet görseli. **Nuxut duvar kağıtları** (hero'nun altında; bölüm başlığı kartın başlığıdır, kartta yalnızca açıklama): yerleşik duvar kağıtları (`nuxut-wallpapers`), 9 kart 16:9, etkin kipin varyantı, seçim `nuxut:<id>` → yol + palet görseli tek geri al; boş yol = varsayılan kart seçili, yazılmaz (§7). **Yerleşim** kartı (yalnızca bir dosya görseli gösteriliyorsa; yerleşik her zaman doldurur): `wallpaper.fit` (cover/contain/fill); kenar boşluğu zemini (`wallpaper.backdrop` blur/color, `backdrop_blur`, `backdrop_color` tema rolü) yalnızca bir yerleşim "Tamamı" iken (`wallpaper_letterbox`: genel ya da bir monitörün `fit = contain`). **Monitör başına** (Ekranlar'daki adlarla, §7). **Renkler**: yönlendirme satırı (`palette.source`, `CustomId::WallpaperColors`) "Renkler bu görselden alınıyor / Renkler sabit" + Görünüm › Renkler'e giden düğme — renk şeması tek yerde Görünüm'de (karar 5) |
| Hesaplar | D-Bus | hesap listesi (durum, son senkron, "yeniden bağlan", "şimdi senkronla", kaldır); "Hesap ekle" → sağlayıcı kartları → akış (§6); takvimler: renk noktası + göster/gizle |
| Bildirimler | shell.toml | **Genel**: "Bildirimleri Nuxut göstersin", rahatsız etme; **Davranış**: görünme süresi, aynı anda en çok, saklanan bildirim; **Uygulamalar**: uygulama başına açılır/geçmiş kapatma |
| Pencereler | shell.toml | üç kart (t094 W4): **Yerleşim** `[windows]` iç/dış boşluk, kenarlık; **Görünüm** bulanıklık + yoğunluk ve tek "Köşe, gölge ve hareket" satırı (`CustomId::Links`: Görünüm ve Hareket düğmeleri; `windows.shadow`/`windows.motion` kendi satırı olmayan `CustomId::Alias` kayıtlarıdır, aramada bu satırı bulur); **Gelişmiş** kapalı açılır satır "Masaüstüyle ortaklık": `[compositor]` `sync_look/sync_windows/sync_motion/sync_input/ensure_source` (+ bağlantı satırı notu) kullanıcı diliyle (kapalı grubun satırları gizlenir) |
| Klavye | shell.toml `[input.keyboard]` | sözleşme girdi-v1 (nuxut-shell `docs/design/input.md`; kalıcı kopya orada, değişiklik orada), "içe aktar, ezme": satır değeri dosyada yazılıysa dosya, değilse çalışan masaüstündeki **etkin** değer (`msg input status`, `services/input.rs`), o da yoksa şema varsayılanı (`SettingDesc::value`; `is_live` = `input.*`). Yazılmamış satır değiştirilmemiş sayılır (↺ yok); yazılmış her değer — varsayılana eşit de — dosyada kalır; ↺ anahtarı siler (Hyprland/kullanıcı değerine döner). **Düzenler** (`pages/keyboard.rs::KeyboardLayouts`): etkin liste `xkb::display_name` ve kodla ("Türkçe — Turkish  tr"; sağdan sola dil adı U+200E ile sola hizalı), yukarı/aşağı, sil (en az bir), "Düzen ekle" → arama alanlı satır içi seçici (arama alanı ekle satırında, sağında "Vazgeç"; seçici açıkken ↺ gizli; `xkb::search` — ad eşleşmesinden sonra dilin kendi düzeni (`de` ↔ deu) aynı dili taşıyan bölge düzenlerinden önce, "german" → `de`, sonra at/be/ch; varyantlar; boş sorguda nadir düzenler yok; mevcutlar hariç; en çok 8 sonuç; Enter/oklar/Esc), en çok 4; ilk değişiklik etkin listeden başlar (`tr` + eklenen → `[{tr},{us}]`); "Düzenler arası geçiş" (`LayoutSwitch`, yalnızca birden çok düzen varken): "Yok" + `grp:` seçenekleri (açılır seçim; katalogda olmayan etkin seçenek de listede), yazarken `options` = etkin/dosyadaki `grp:` olmayan seçenekler korunur + seçilen. **Tuş tekrarı**: gecikme (100–2000 ms), hız (1–100 tuş/sn), "Burada deneyin" yerel metin alanı (`KeyTest`; hiçbir şey yazmaz, tekrarı compositor üretir). **Diğer**: açılışta Num Lock. `overrides` içindeki anahtarın satırında (ve Düzenler/geçiş bloklarının altında) framework sıkı uyarı notuyla (`Notice` Warning + `compact`: Caption metin, küçük `Warning` ikonu `error` renginde; `rows::override_notice`) "Hyprland ayar dosyan bu değeri geçersiz kılıyor: <dosya>:<satır>"; shell dosya/satır bilmiyorsa (`file`/`line` null — Lua config, `getoption` yedeği) "Hyprland ayarın bu değeri geçersiz kılıyor (<hypr anahtarı>)." Üstte kartsız not (`pages/input_note.rs`; framework `Notice` Info, "Aç" notun sağında): `compositor.sync_input` kapalıysa neden + "Aç" (satırlar gizli; aramada gizli girdi satırı bu nota gider), kabuk çalışmıyor/eski/Hyprland'e ulaşılamıyorsa "değerler kabuk açılınca uygulanır" (satırlar şema varsayılanını gösterir). `[input]` değişince durum 700 ms sonra yeniden sorulur; pencere odağı dönünce de (`input::recheck`, yalnızca daha önce sorulduysa) |
| Fare ve dokunmatik yüzey | shell.toml `[input.mouse]`, `[input.touchpad]` | aynı değer kaynağı ve not. **Genel**: işaretçi hızı (−1..1; fare ve dokunmatik yüzey için ortak), ivme (Uyarlanır/Düz); **Fare**: doğal kaydırma; **Dokunmatik yüzey**: doğal kaydırma, dokunarak tıkla, yazarken devre dışı, kaydırma hızı (0,1–3×), kaydırma yöntemi (İki parmak/Kenar/Düğmeyle/Kapalı). Hyprland'in boş `accel_profile`/`scroll_method` değeri (libinput varsayılanı) etkin değer sayılmaz → şema varsayılanı gösterilir. Eşitleme anahtarı `compositor.sync_input` Pencereler › "Masaüstüyle ortaklık"ta |
| Kısayollar | shell.toml | `compositor.sync_keybinds` ("Nuxut kısayolları"); `[keybinds]` eylemleri shell `Action::ALL` sırasıyla (kayıtta; şemadaki `keybinds.*` anahtarlarıyla denetlenir ve testlidir) Ses/Parlaklık/Medya/Bar ve paneller gruplarında; satır = kısayol yakalayıcı (tıkla → tuşlara bas; yazı tuşu Super/Ctrl/Alt'sız kabul edilmez; tuş Hyprland'in eşleyeceği seviye-0 keysym adıyla yazılır: varsayılan ilk düzenin, `input:resolve_binds_by_sym` açıkken etkin düzenin — Türkçe Q'da Shift+1 → `SHIFT+1`; simgesiz tuş `code:<n>`); aynı birleşim iki eylemdeyse ikisinde de uyarı (shell ilkini tutar); tuş alanı kenarsız tonal kuyu. Sağ/sol panel kısayolu yalnızca o panel kullanılırken görünür ve Paneller'de panel kartında aynalanır (`mirror [Panels]`; kayıtta Paneller bloğunun içinde sıralı, eylem sırası değişmez). Bar kısayolu Bar'da "Konum ve boyut" kartının son satırı (`mirror [Bar]`; Bar bloğu bu yüzden kayıtta Kısayollar/Paneller'den sonra). Hyprland ayarında aynı tuş bağlıysa (2026-10-17, `msg hypr status` `conflicts`/`taken_over`, `services::hypr_status::key`) satırın altında durum ve tek düğme: kullanıcınınki çalışıyorsa "Nuxut'unkini kullan", Nuxut'unki çalışıyorsa ya da devralınamıyorsa (uyarı, nedeniyle) "Benimkini koru" → `[keybind_owner] <eylem>`; düğmeye Tab ile alandan sonra ulaşılır; ekran okuyucu düğümleri: alan (eylem adı, değer = tuş, açıklama = kimin kısayolu + durum), seçim, temizle ve geri al düğmeleri — AT eylemleri `app/a11y.rs` kapısında silinir, düğmeler yalnız gerçek Tab/Enter ve işaretçiyle; yazınca ve kısayol değişince durum yeniden sorulur (shell §3.5, `docs/design/hyprland-base.md`) |
| Ses | shell.toml | **Ses düzeyi** (bölüm başlığı sayfa adını tekrarlamaz; 2026-10-15): en yüksek ses (+ 100 üstü uyarısı), ses ve parlaklık göstergesinin süresi; **Medya tuşları**: Kısayollar'a yönlendirme ya da kısayollar kapalıyken masaüstü ayar dosyası için bind örneği (Kopyala) |
| Yazıcılar | CUPS (IPP; `services/printers`) + `~/.cups/lpoptions` | t125, kararlar `docs/design/printers.md` (2026-10-15): Sistem grubunda Güç'ten sonra (aygıt kümesi; slug `printers`). İstemci `ipp` crate'i (yalnız protokol) + kendi küçük HTTP/1.1'imiz; sunucu libcups gibi `CUPS_SERVER` → `/run/cups/cups.sock` → `localhost:631`; libcups FFI yok (`forbid(unsafe_code)`). **Yazıcılar** kartı: satır = açıklama (yoksa kuyruk adı), alt metin "[Varsayılan ·] Hazır/Yazdırıyor/Durduruldu [· İş kabul etmiyor] [· CUPS iletisi]" (durmuşta hata renginde); tıklayınca ayrıntı: model, konum, kuyruk adı, "Varsayılan yap" (kullanıcı düzeyi `~/.cups/lpoptions` `Default` satırı, atomik, biçim korur; yönetici yetkisi gerekmez), "Deneme sayfası" (CUPS `testprint`, `application/vnd.cups-banner`; dosya yoksa gizli); kartın sonunda "Yazıcı ekle" → CUPS web arayüzü (`/admin`, yalnız yerel sunucu). **Yazdırma kuyruğu**: kullanıcının bitmemiş işleri (`my-jobs`), "İptal" (`Cancel-Job`, `job-uri`; bitmiş/olmayan iş başarı sayılır). Yoklama yalnız sayfa görünürken (iş varken 2 sn, yokken 5 sn; odak dönünce hemen). Eylemler sırayla, tek bekleyen, iyimser değil; hata kartsız üst notta. CUPS yoksa sayfa kenar çubuğunda kalır, tek içerik maskotlu "Yazdırma servisi yok"; yazıcı yoksa kartta maskotlu boş durum + "Yazıcı ekle". v2: keşif/sürücüsüz ekleme (yönetici yetkisi), duraklat/sil, sistem varsayılanı, mürekkep düzeyi. Test: `tests/printers_ipp.rs` + süreç içi sahte IPP sunucusu (`tests/support/fake_cups.rs`; canlı deneme `examples/fake_cups.rs` + `CUPS_SERVER`) — gerçek CUPS'a yazma işlemi gönderilmez |
| Bölge ve dil | framework.toml | karar 9 (Sistem grubu): **hero = biçim önizlemesi** (`locale::LocalePreview`, başlıksız kart; hangi biçim — açık ad ya da "… · sistem" —, saat, haftanın günüyle uzun tarih, kısa tarih, sayı, yüzde; `Locale::format_system_time`/`format_number`, `numfmt::percent_in`; ayar/ortam değişince ve dakika dönünce `Ui::wake_at` ile yenilenir); "Dil ve biçimler" kartı: Dil ve Biçimler açılır seçimleri (`[locale] language`/`formats`, ADR-0010), açıklama tek satır (Otomatik'te çözülen değer, tanınmayan adda uyarı, yoksa kısa açıklama). Önizlemedeki saat `[locale] hour_cycle`'a uyar; saat dilimi Tarih ve saat sayfasında |
| Tarih ve saat | systemd-timedated (D-Bus, sistem bus'ı) + framework.toml `[locale] hour_cycle` | t119, ayrıntı ve kararlar `docs/design/datetime.md`: Ayarlar timedated'i doğrudan çağırır (sistem ayarı; dosyaya/geri al'a girmez), yazmalar `interactive=true` → polkit; **Saat** kartı: otomatik saat (`SetNTP`, eşitlenme alt metni, `CanNTP=false` kapalı), saat dilimi ("Şehir, Bölge — UTC±hh:mm" + satır içi aranabilir seçici, `ListTimezones`), elle tarih ve saat (yalnız otomatik kapalıyken; `DatePicker::with_locale` (`locale::date_locale`: store'daki biçim, `hour_cycle`'sız) + `TimePicker::with_locale` 1 dk adım, 12/24 saat store'daki `hour_cycle`'a uyar (`locale::time_locale`), biçim/`hour_cycle` değişiminde ikisi de yeniden kurulmadan yerinde `set_locale` (seçim, odak, açık takvim korunur), odak kaybında serbest giriş `cancel_entry` + Uygula → `SetTime`); **Biçim** kartı: saat biçimi (`hour_cycle`, framework şeması tanıyorsa görünür) + bar saati özel biçim notu ve "Temizle" (shell `clock-locale.md` sözleşmesi, 2026-10-13: `format`/`vertical_format` yazılmamış ya da `""` = otomatik; etkili anahtar bar yönüne göre; Temizle iki anahtarı tek geri al kaydıyla siler). Bar saati biçim alanlarının varsayılanı `""`, yer tutucu "Otomatik (12/24 saat tercihine uyar)", alt metin shell şemasının `description`'ı (`SettingDesc::display_subtitle`: şema açıklaması yerel alt metnin önüne geçer). Üstte kartsız not (framework `Notice`, Warning): timedated yok / okunamadı / LocalRTC uyarısı; bar saati notu `Notice` Info. Bekleme "Yetki bekleniyor…"; polkit reddi/iptali sessiz (satır önceki değere döner, karar), diğer hata "Değiştirilemedi" (ayrıntı gösterilmez). `PropertiesChanged` + sayfa açılınca/odak dönünce yeniden okuma; `Event::Timedate` Bölge ve dil önizlemesini de yeniler. Test: `NUXUT_TIMEDATE_BUS=session` + `examples/fake_timedated.rs` (gerçek timedated'e yazma yasak) |
| Kullanıcı hesabı | AccountsService (D-Bus, sistem bus'ı) + `~/.face` | t118, kararlar `docs/design/account.md` (2026-10-13): `PageId::User` (slug `user`; takvim "Hesaplar"ından ayrı ad). **Profil** kartı: framework `Avatar` (Large, resim yoksa baş harfler) + görünen ad + "kullanıcı adı · hesap türü" (salt okunur) + "Değiştir…" (framework `nuxut::portal` seçicisi → yalıtılmış işçide çözüm → kare 256² PNG → `$XDG_CACHE_HOME/nuxut-settings/avatar.png` → `SetIconFile` → başarılıysa `~/.face` atomik, yabancı `.face` bir kez `.face.nuxut-bak`); Görünen ad (`SetRealName`, Enter/odak kaybı, boş yazılmaz, `:`/`,` reddi, 64 karakter). Yazmalar `ALLOW_INTERACTIVE_AUTHORIZATION` bayrağıyla (polkit), 300 sn; ret/iptal sessiz (timedate kararı). Hizmet yok → yalnız üst not. `Changed`/`PropertiesChanged` + sayfa açılınca/odak dönünce yeniden okuma. Parola: v2. Test: `NUXUT_ACCOUNTS_BUS=session` + `examples/fake_accounts.rs` + `tools/accounts-dbus-test.sh` (gerçek AccountsService'e çağrı yasak) |
| Wi-Fi | NetworkManager (D-Bus, sistem bus'ı; `nuxut-nm`) | t114 v1, kararlar `docs/design/network-pages.md` (salt okuma 2026-10-13, eylemler 2026-10-14): istemci kabukla ortak `../shell/crates/nuxut-nm`, **ajansız** (Ayarlar asla NM gizli bilgi ajanı olmaz). İlk kart başlıksız: **Wi-Fi** aç/kapa (canlı, iyimser değil: bekleme ilerlemeyle, hata satırda) + bağlı ağ (tıklayınca IP/DNS `device_details` + Bağlantıyı kes). **Görünen ağlar**: açık ağa doğrudan bağlan, şifreli ağda satır içi parola alanı (`TextInput` parola kipi; tek `Zeroizing` kopya → `Secret`, ara `String`/log yok; `valid_secret` istemci doğrulaması; `NoSecrets`/`AuthFailed`/geçersiz → satırda hata, alan yeniden açılır), kurumsal ağda "desteklenmiyor (v2)" notu; "Gizli ağa bağlan…" (SSID 1–32 bayt, Açık/WPA/WPA3). **Kayıtlı ağlar**: otomatik bağlan, ölçülü (Otomatik/Evet/Hayır), Unut (`ConfirmButton`); kapsama dışında nötr ikon. **Kablolu** kartı salt okunur (IP, DNS, hız). Eylem durum makinesi `services::network::{Op, Target}` (2026-10-14: crate isteği `RequestId` ile izlenir, sonuç `Done`/`Failed { request }` kimlikle eşlenir; etkinleştirmeler kabulden sonra `Activated` ya da `ActivationFailed` ile biter; zaman aşımı yalnız güvenlik ağı); polkit reddi (`NmError::Denied`) sessiz; sistem eylemleri geri al geçmişine girmez. Kayıtlı ağ şifresi yenileme = `set_wifi_password` (yerinde; otomatik/ölçülü korunur) + etkinleştir (K3); WEP/kurumsal → "değiştirilemiyor", "her seferinde sor" → açıklayıcı metin. WireGuard içe aktarmada çalıştırılmayan wg-quick anahtarları (`PostUp`…) satırda adlarıyla not. Tarama yalnız sayfa görünürken. Üst not: bus yok / NM yok ayrı metin (`Unavailable`). Test: `test-support` özelliği + crate'in `examples/fake_nm` + `tools/network-dbus-test.sh` (gerçek NM'e çağrı yasak; özelliksiz yapıda bus hep sistem) |
| VPN | NetworkManager (`nuxut-nm`) | t114 v1: VPN ve WireGuard bağlantıları (etkin önce; satırda aç/kapa anahtarı, tıklayınca Sil — `ConfirmButton`), "WireGuard içe aktar…" (`nuxut::portal` OpenFile, `*.conf`; dosya 64 KB sınırıyla tek sıfırlanan tampona, ad = dosya adı, `WireGuardError` satır numarasıyla metin, değer/yol loglanmaz). OpenVPN içe aktarma ve ad düzenleme v2 |
| Bluetooth | BlueZ (D-Bus, sistem bus'ı; `nuxut-bluez`) | t115 v1, kararlar `docs/design/bluetooth-page.md` (2026-10-14): istemci kabukla ortak, **ajansız** (PIN/onay kabuğun ajanında; Ayarlar yalnız `Pair` sonucunu ve "onayı kabuk penceresinde verin" bekleme metnini gösterir; `Pair` öncesi `msg bluetooth agent-status` sorulur, kabuğun ajanı varsayılan değilse "Eşleştirme onayı için masaüstü kabuğu çalışmalı" uyarısı ve `Pair` yine denenir, yanıt yoksa eski davranış; sonuç `RequestId` ile eşlenir). **Bluetooth** kartı: aç/kapa (rfkill yumuşak "Uçak kipi açık" / sert "Donanım anahtarıyla kapatılmış", sertte anahtar devre dışı), bu bilgisayarın adı (Enter/odak kaybında yazılır), görünür (180 sn; kalan süre saniyede bir). **Eşleşmiş cihazlar**: tür ikonu, ad, Bağlı/Bağlı değil, pil; satır içi ayrıntı (ad, güvenilir, engelle, bağlan/kes, unut — iki basışlı onay). **Yakındaki cihazlar**: RSSI'ye göre, adsızlar gizli (alt satırda sayı + Göster/Gizle), Eşleştir/İptal; tarama yalnız sayfa görünürken (60 sn, "Yeniden ara"). BlueZ yok / bus yok / adaptör yok / servis açılamadı → sayfanın tek içeriği maskotlu hata. v1 yalnız ilk adaptör. Test: `--features test-support` + `tools/bluetooth-dbus-test.sh` (gerçek BlueZ'e çağrı yasak) |
| Güç | power-profiles-daemon + UPower (paylaşılan `nuxut-power` crate'i, `../shell/crates/nuxut-power`) | t117, `docs/design/power-page.md` (iskelet 2026-10-14; canlı kaynak 2026-10-15): veri yolu B (crate; `msg power`'a bağlanmaz, kabuk kapalıyken de çalışır, canlı `PropertiesChanged`). Kart sırası: **Pil** (durum satırı: düzey ikonu, "%64", "Pilden çalışıyor · 2 sa 10 dk kaldı" / "Şarj oluyor · … içinde dolar" / "Tam dolu" / "Prizde, şarj olmuyor"; "Pil ömrünü koru" yalnız destekleyende, alt metin salt okunur eşiklerden, Türkçe ek `tr.ftl` seçimiyle), **Güç profili** (Güç tasarrufu · Dengeli · Performans bölümlü düğmesi; üçü hep çizilir, ppd'nin sunmadığı tek başına devre dışı — framework 86f0125; kısıt nedeni notu; ppd yoksa devre dışı + "power-profiles-daemon gerekli"). Pil, şarj sınırı ve ppd yoksa kartsız maskotlu boş durum (UPower da yoksa hata). Yazma iyimser değil (istek kimliğiyle sonuç; yeni durum sonuçtan önce); hata alt metinde, `Unsupported` anahtarı gizler. Kaynak `services::power::Source`: `Live` (sistem bus'ı; `test-support` ile `NUXUT_POWER_BUS=session`), açılamazsa `None` (sayfa gizli), test/deneme `NUXUT_POWER_FAKE=<toml|json>`. **Kilit** ve **Boşta kalınca** kartları (shell.toml `[idle]`, `[lock]`, 2026-10-16): en üstte uykudan önce kilit (shell 7b3c80e, varsayılan açık) ve "Nuxut kilit ekranını kullan" (`lock.listen_logind`); sonra boşta ana anahtarı `idle.enabled` ("Boşta eylemlerini Nuxut yönetsin"; kapalıyken aşama satırları gizli, uykudan önce kilit satırı kalır ve altında "çalışmıyor" uyarısı — `SettingDesc::note_in`), boşta kilit/karart/ekranı kapat/uyu + aşama açıkken süresi (1 dk adım, ≥60 sn dakikayla gösterilir), prizdeyken de uyu, medya oynarken bekle; şema `[idle]` bilmiyorsa yok; kartlar varken boş durum maskotu yok ve sayfa güç hizmeti olmadan da listelenir (`docs/design/power-page.md`) |
| Varsayılan uygulamalar | shell IPC (`mimeapps.list` shell'de) | sözleşme defaults-v1 (Ayarlar tarafı kararları `docs/design/default-apps.md`): Sistem grubunda; İnternet (Tarayıcı, E-posta) · Sistem (Dosya yöneticisi, Terminal, Metin düzenleyici) · Medya ve belgeler (Resim, Video, Müzik, PDF). Satır: başlık + açıklama, sağda ikonlu açılır seçici (her seçenekte uygulama ikonu temadan, `services/app_icons.rs`; yoksa yer tutucu; görünür adaylar, sonra "Diğer" başlığı altında `NoDisplay` olanlar; hiçbir şey açmıyorsa "Seçilmemiş", kurulu olmayan mevcut seçim "<ad> (kurulu değil)"). Notlar satırın altında: `mixed` bilgi, `shadowed_by` hata renginde uyarı, terminal `available:false` → not + seçici kapalı. Seçim → `msg defaults set` (arka planda, 5 sn) → durum yeniden okunur; hata kodu katalog metniyle şeritte, satır shell'in gerçek durumuna döner. ↺ = shell snapshot'ı (`defaults.<rol>`). Durum okunamadıysa (kabuk yok / eski / hata) satırlar gizli, üstte kartsız not; arama sonucu o nota gider. Durum: `msg defaults status` 2 sn, ayrı thread; odak dönünce yeniden (`default_apps::recheck`, yalnızca sayfa açıldıysa) |
| Ekranlar | shell IPC | tuval (sürükle-bırak yerleşim) + monitör seçici + seçili monitörün satırları (ayırıcısız, dar/kalabalık satırda denetim alta iner); ölçek 2–5 temiz seçenekte `SegmentedButton`, daha çoğunda açılır seçim (1920×1080'de 9 temiz ölçek); değerler Ayarlar'ın tek yüzde biçimiyle |
| Ekranlar › Gece ışığı | shell.toml `[night_light]` + `nuxut-shell msg night-light status` | kart, yüklü shell şeması `night_light`'ı tanımıyorsa (eski shell / şema yok) tümüyle gizli (yazılan anahtar etkisiz kalırdı). Görünürlük: ayrıntı satırları ana anahtara (`enabled`), başlangıç/bitiş saati `schedule = "manual"`'e, konum `sun`'a, geçiş süresi `always` dışına bağlı. Konum satırı salt okunur: "Konum: saat diliminden (≈ …)" / "elle (…)" / "bulunamadı — 20:00–07:00" + bugünün gün batımı/doğumu (ya da kutup günü/gecesi); kaynak `msg night-light status` `location`/`today`, shell yanıt vermezse satır gizli. Elle konum shell.toml `latitude`/`longitude`'dan; Ayarlar'da düzenlenebilir alan v2 (karar, onay, 2026-10-13, v1 seçenek b). Başlangıç/bitiş saati framework `TimePicker` (`with_locale(locale::time_locale(store))`: 12/24 saat `[locale] hour_cycle`'a uyar, 1 dk adım, serbest giriş); dosyaya her zaman 24 saat `"SS:DD"` yazılır (shell `parse_hhmm`), dosyadaki geçersiz değer yerine shell varsayılanı 20:00 / 07:00 gösterilir ve dosya kullanıcı seçene dek değişmez (inceleme, 2026-10-13). Birimler şemadan: sıcaklık "K" (çevrilmez), geçiş "min" → "dk" |
| Hakkında | — | marka başlığı: Kara Sağan logosu (`nuxut::ui::BrandMark::tile`, 80 px; blok kartın `surface_container` zemininde, karo `surface` — uygulama ikonuyla aynı görünüm) solda, yanında "Nuxut" (Title), slogan (`settings-brand-slogan`, ikincil) ve sürüm logoya ortalı; ad ya da slogan logonun yanına tek satır sığmıyorsa metin logonun altına iner (`about::header_stacks`); kısa açıklama, config dosya yolları (ev dizini `~`, sığmazsa baştan "…/nuxut/shell.toml" kısaltılır, tek satır) + "Klasörü aç"; satır yüksekliği ölçülen metinden, sığmazsa düğme alta iner |

### Maskot Fiyu (`src/pages/fiyu.rs`)

Karar: kullanıcı, 2026-10-13 (framework `docs/design/mascot.md` yerleşim kuralı;
uygulama). Yerler tek tabloda (`Spot` → ruh hâli, kenar); boş/hata durumları framework
`EmptyState` ile çizilir.

| Yer | Durum | Ruh hâli | Kenar |
|---|---|---|---|
| Hakkında (`pages/about.rs`) | marka başlığının sağı; sütun darsa düşer | `Hello` | 128 |
| Hoş geldin bitiş "Hazır" (`welcome_ui.rs`) | onay ikonunun yerinde | `Celebrate` | 128 |
| Kenar çubuğu araması (`app.rs`) | sonuç yok | `NoResults` | 96 |
| Ekranlar (`pages/displays.rs`) | kabuk çalışmıyor / yanıt vermiyor | `Error` | 96 |
| Varsayılan uygulamalar (`pages/default_apps.rs`) | kabuk çalışmıyor / okunamadı | `Error` | 96 |
| Takvim hesapları (`pages/accounts.rs`) | servise ulaşılamadı (yüklenmeden) | `Error` | 96 |
| Takvim hesapları | hesap yok | `Empty` | 96 |
| Bildirimler › Uygulamalar (`pages/notif_apps.rs`) | liste boş | `Empty` | 96 |
| Wi-Fi › Kayıtlı ağlar (`pages/network.rs`) | kayıtlı ağ yok | `Empty` | 96 |
| VPN | kayıtlı bağlantı yok | `Empty` | 96 |
| Bluetooth (`pages/bluetooth.rs`) | BlueZ çalışmıyor / bus yok / adaptör yok / servis açılamadı (sayfanın tek içeriği) | `Error` | 96 |
| Bluetooth › Eşleşmiş cihazlar | eşleşmiş cihaz yok | `Empty` | 96 |
| Yazıcılar (`pages/printers.rs`) | CUPS kurulu değil / çalışmıyor / okunamadı (sayfanın tek içeriği) | `Error` | 96 |
| Yazıcılar › Yazıcılar kartı | yazıcı yok | `Empty` | 96 |
| Güç (`pages/power.rs`) | UPower ve power-profiles-daemon çalışmıyor (sayfanın tek içeriği) | `Error` | 96 |
| Güç | pil yok, şarj sınırı yok, power-profiles-daemon yok (masaüstü, VM; sayfanın tek içeriği) | `Empty` | 96 |

- Satır içi tek satırlık notlarda (girdi/görünüm "kabuk çalışmıyor" notları, ağ notu, Wi-Fi
  kapalı / görünen ağ yok) maskot yok. "Ekran ayarlarını bu kabuk sürümü desteklemiyor" bir
  arıza değil: ekran ikonlu `EmptyState`, maskotsuz. Varsayılan uygulamalar "yükleniyor" ve
  "desteklemiyor" sakin not olarak kalır.
- Ekranda en çok bir maskot: aynı sayfada iki maskotlu durum oluşmaz (Wi-Fi'de yalnız kayıtlı
  ağlar listesi maskotlu); bloklar `Block::mascot()` ile maskotlu durumlarını bildirir, arama
  sonuç vermediğinde açık sayfada maskot varsa (`Page::mascot`) arama maskotsuz çizilir.
- Gizleme: Görünüm › Maskot (sayfanın son kartı; tek satırlık kart Gölge ile Uygulamalar arasını bölmesin — 2026-10-15) › "Fiyu'yu gizle" (`appearance.mascot`, ters anahtar; varsayılan
  kapalı = görünür). Açık → `framework.toml [appearance] mascot = false`; kapalı → anahtar
  silinir. Hoş geldin'de yok; şeması anahtarı tanımayan framework'te satır gizli
  (`hour_cycle` kalıbı). Gizliyken maskot yer kaplamaz (`Mascot::measure` sıfır); Hoş geldin'de
  bitişte onay ikonu geri gelir, Hakkında başlığı logo boyuna iner. Değişim tema yeniden
  yüklemesiyle canlıdır.
- Hoş geldin ilk ekranı: marka logosu (Kara Sağan, iki tonlu) + slogan HER ZAMAN görünür,
  maskot ayarından bağımsız; Fiyu ilk ekranda yok — aynı ekranda iki kırlangıç olmasın, ilk
  izlenim marka. Fiyu yalnız son ekranda ("Hazır", bitiş) `Celebrate`. Hakkında değişmez (logo
  başlıkta, Fiyu küçük selam). Karar: 2026-10-14 (2026-10-13 tablosundaki
  "karşılamada logonun yerinde `Hello`" satırını kaldırır).

## 3. Config yazma katmanı (`src/store/`)

Tek doğruluk kaynağı diskteki dosyadır; uygulama bellekte bir `toml_edit::DocumentMut`
tutar ve yalnızca değişen anahtarı düzenler.

```rust
pub enum FileId { Framework, Shell }
pub struct KeyPath(Vec<String>);              // "motion.spatial.damping" → ["motion","spatial","damping"]

pub struct ConfigFile {
    path: PathBuf,
    doc: DocumentMut,                          // yorumlar, boşluklar, anahtar sırası korunur
    disk_text: Option<String>,                 // diskte bildiğimiz son içerik (okunan ya da yazılan)
    dirty: bool,                               // bellekte yazılmamış değişiklik var
    broken: Option<String>,                    // dış düzenleme bozuk TOML bıraktı → yazım durur
}
impl ConfigFile {
    pub fn open(path) -> Self;                 // yoksa boş belge (dosya ilk yazımda oluşur)
    pub fn get(&self, &KeyPath) -> Option<&toml_edit::Item>;
    pub fn set(&mut self, &KeyPath, Option<toml_edit::Value>) -> Option<Value> /* önceki */;
    pub fn flush(&mut self) -> io::Result<bool>;          // atomik yazım; değişmediyse false
    pub fn on_disk_changed(&mut self) -> DiskChange;      // OwnWrite | Reloaded | Broken(msg) | Removed
}
```

Kurallar:
- **Yalnızca değişen anahtar.** `set` var olan değeri yerinde değiştirir (değerin
  önündeki/arkasındaki süsleme — satır sonu yorumu dahil — korunur). Anahtar yoksa ilgili
  tabloya eklenir; tablo yoksa dosyanın sonuna açık `[a.b]` başlığıyla yaratılır. `None`
  anahtarı siler ("varsayılana dön"); boşalan, yorumsuz tablo da silinir. Bilinmeyen
  anahtarlara ve diğer tablolara dokunulmaz.
- **Sayılar** adımın hassasiyetine yuvarlanır (`0.30000000000000004` yazılmaz); tamsayı
  şemalı anahtar tamsayı yazılır.
- **Atomik yazım:** hedefin dizininde `.<ad>.toml.<pid>-<n>.tmp` (`create_new`, 0600) →
  `write_all` → izinler eski dosyadan (yoksa 0644) → `fsync` → `rename` → dizin `fsync`.
  Config bir **symlink** ise (dotfile deposu) zincir izlenir, symlink korunur ve gösterdiği
  dosya değiştirilir (kırık symlink'in hedefi oluşturulur; döngü hata). Hedef düz dosya
  değilse yazılmaz. Dizin yoksa oluşturulur. Hata olursa geçici dosya silinir, değişiklik
  bellekte kalır ve kullanıcıya "kaydedilemedi" gösterilir.
- **Yazım çakışması:** `rename`'den hemen önce diskteki içerik son okuduğumuz/yazdığımızla
  karşılaştırılır (`write_atomic_if`). Farklıysa (dış düzenleme, inotify henüz işlenmedi)
  üstüne yazılmaz: dış hâl yüklenir, bekleyen değişiklikler üstüne uygulanır, bir kez daha
  denenir ve `revision` artar (sayfalar dış değerlere eşitlenir). Dosya bu arada bozulduysa
  yazım durur.
- **shell.toml sürümü (shell ADR-0002 §11):** sürümsüz dosya v1 sayılır. Ayarlar shell.toml'u
  **yaratan** yazımda çalışan shell'in şemasındaki sürümü (`nuxut-shell schema` kökü, ≥2) en üste
  yazar. Var olan sürümsüz dosyaya yazmaz: onu shell ilk açılışta damgalar (tek yazan; v1 dosyanın
  göçü de shell'de). Var olan sürüme dokunmaz; şema yoksa ya da eski shell ise damga yok.
- **Birleştirme (150 ms):** `set` belleği hemen değiştirir (UI anında), ilk değişiklik
  150 ms'lik tek atımlık timer kurar; timer'a kadar gelen tüm değişiklikler tek yazımda
  gider. Kaydırıcı sürüklerken saniyede en çok ~7 yazım; shell ve tema değerlere yayla
  aktığı için ara adımlar yumuşak görünür. Pencere kapanırken bekleyen yazım hemen yapılır.
- **Kendi yazımını ayırt etme (save-on-parse döngüsü dersi):** dosya değişim bildirimi
  gelince içerik okunur; `disk_text` ile **aynıysa** kendi yazımımızdır → hiçbir şey
  yapılmaz (yeniden yükleme yok, yazım yok). Farklıysa dış değişikliktir: yeni belge
  ayrıştırılır, bekleyen (henüz yazılmamış) değişikliklerimiz üstüne yeniden uygulanır, UI
  yenilenir. **Yükleme asla yazım tetiklemez**; yazım yalnızca kullanıcı eylemiyle olur.
- **Bozuk dış düzenleme:** ayrıştırılamayan içerikte belge korunur, `broken` dolar, dosyaya
  yazım durur (kullanıcının yarım düzenlemesinin üstüne yazılmaz) ve sayfanın üstünde
  "shell.toml'da sözdizimi hatası: … — düzeltilene kadar değişiklikler kaydedilmiyor"
  şeridi görünür. Dosya düzelince normal akış.
- **Silinen dosya:** belge boşalır (varsayılanlar), UI yenilenir.
- Dosya boyutu sınırı 1 MiB (daha büyüğü bozuk sayılır).

Değişiklik bildirimi: `framework.toml` için runtime `App::theme_changed` çağırır (tema
zaten yenidir — Ayarlar penceresi de yeni temayla akar); `shell.toml` için
`cx.watch_config("shell")` → `App::config_changed("shell")`. Her ikisinde de
`ConfigFile::on_disk_changed` çağrılır. `[locale]` değişince runtime ayrıca `App::locale_changed`
çağırır (süreç geneli Fluent katalogu yenidir): Ayarlar `services::Event::LocaleChanged` yayar,
"Dil ve biçimler" satırları değer aynı olsa da çözülen dili ve biçim önizlemesini yeniden kurar.

### Geri al (`src/history.rs`)

```rust
pub struct Change { file: FileId, path: KeyPath, before: Option<Value>, after: Option<Value>,
                    label: String /* "Vurgu rengi" */, at: Instant }
pub struct History { undo: Vec<Change> /* ≤ 100 */, redo: Vec<Change> }
```

- Aynı anahtara 1 sn içindeki ardışık değişiklikler (kaydırıcı sürükleme, renk alanında
  gezinme) tek kayıtta birleşir (`after` güncellenir).
- Geri al = `before`'u aynı yazım yolundan uygula (kayıt üretmez, redo'ya taşır). Dış
  değişiklik aynı anahtarı değiştirdiyse kayıt yine anahtar tabanlı uygulanır (son yazan kazanır).
- Her değişiklikten sonra alt kenarda 4 sn'lik şerit: "Vurgu rengi değişti · Geri al".
  Başlık çubuğundaki ↶ düğmesi "Son değişiklikler" menüsünü açar (son 10 kayıt; birine
  tıklamak yalnızca o anahtarı eski değerine döndürür).
- Çoklu anahtarlı eylemler (ör. "renkleri duvar kağıdından al" → `colors.source` + palet)
  tek grup kaydıdır (`Change` dilimi, tek geri al).

### Önceki hâline dön (`src/services/snapshots.rs`, shell ADR-0001 §7)

Oturum içi geri al'ın tamamlayıcısıdır, kopyası değil: `History` bu oturumda Ayarlar'ın
yazdığı TOML anahtarlarını tutar (bellekte); snapshot'lar kalıcıdır ve shell'indir — shell dış
hedeflere (GTK/Qt/ikon/imleç, dconf, Hyprland) dokunurken eski hâli
`$XDG_STATE_HOME/nuxut/snapshots`'a yazar. Normal config düzenlemeleri snapshot almaz.
Ayarlar satırı olan her snapshot kimliği için `msg snapshot list --setting <id>` (shell `nuxut-apply`,
ADR-0001 §9.3/§9.5: `[{id, at, setting, label, source, state, restored_by?, restores?, pinned,
fragments, conflicts}]`; zaman alanı `at`) ile o ayarın en son `applied` kaydını bulur — `restored`
"zaten geri alınmış" demektir, geri yüklemenin kendisi yeni bir `applied` kayıttır
(`setting_for`: `colors.mode` → `look.mode`, `desktop.unified` → `look.unified`,
`desktop.icon_theme` → `look.icons`, `desktop.cursor_*` → `look.cursor`, `desktop.qt_backend` →
`look.qt_backend`, yazı tipleri → `look.fonts`, `compositor.ensure_source` → `hypr.source` — geri yüklenince shell bağlantı satırını kaldırır ve `removed_by_user` sayar; anahtar açıkken `msg hypr status` bunu söylerse anahtarın altında sakin bir not çıkar (`pages/source_note.rs`, `services/hypr_status.rs`; kapatıp açmak satırı yeniden ekletir); kimlikler donuk v1
kapalı listesinden — `hypr.<grup>` ve `displays` kimlik değildir) ve yalnızca kayıt varsa
satırda "varsayılana dön"ün yanında ⟲ (History) düğmesi gösterir (`rows::PreviousAction`;
standart satır ve özel bloklar aynı yardımcıyı kullanır). Tıklama `msg snapshot restore <id>
--setting <ayar>` gönderir (kayıt o ayarın değilse shell hiçbir şey yazmaz: `not_found`); sonuç
(`restored` — `id: null` "zaten önceki hâlinde" —, `partial`, `conflict`, `not_found`, `busy`,
`skipped` + `reason`, `error` + `kind`) şeritte; bilinmeyen sonuç ve `error: …` satırları shell
metni gösterilmeden sabit bir metinle. Shell `snapshot` gövdelerini ayrı thread'de yürütür ve
yanıt penceresi 15 sn'dir (`ipc::REPLY_TIMEOUT`): Ayarlar her `snapshot` isteğinde
`SNAPSHOT_TIMEOUT` (16 sn; framework `Client::with_timeout`, `IpcLink::with_timeout`) bekler —
arka plan thread'inde, UI beklemez. Yanıt düşerse (zaman aşımı ya da boş kapanan bağlantı)
geri yükleme yine olmuş ya da hâlâ sürüyor olabilir: `verify_restore` ~2 sn sonra, görünmezse
~5 sn sonra bir kez daha `snapshot list --setting` ister; kayıt `restored` görünürse "geri
yüklendi (ayrıntı doğrulanamadı)" (listeden `partial`/`restored` ayrılamaz), yoksa
"doğrulanamadı — durumu yenile". Restore isteği asla kendiliğinden yeniden gönderilmez.
`nuxut-apply` öncesi shell `snapshot`'ı tanımaz: liste boş, düğme görünmez. `msg hypr status`
`nuxut-apply`'da eski `backup` yerine `snapshot` (bu turun `hypr.source` kimliği) taşır; Ayarlar
`source`'u okur, `snapshot`'ı yalnızca tanı loguna yazar (UI'da kullanılmaz; Hoş geldin geri
alının kaynağı `snapshot list --setting hypr.source`), `backup`'ı yok sayar. Geri yükleme tek işlemdir (karar): shell dış dosyalarla
birlikte framework.toml'daki seçim anahtarını da geri koyar; Ayarlar istekten önce bekleyen
yazımını gönderir, istek sürerken framework.toml'a yazmaz ("geri yükleniyor…", düğme devre
dışı) ve yeni değeri dosya izleyicisinden gösterir. Geri yükleme de bir snapshot'tır: düğmeye yeniden basmak geri
yüklemeyi geri alır. Canlı testte sahte shell: `cargo run --example fake_shell`
(yalnızca `NUXUT_SHELL_INSTANCE` adıyla).

## 4. Ayar kaydı (`src/registry.rs`)

Basit satırlar bildirimseldir (noctalia `settings_registry` yaklaşımı): tek tabloda
tanımlanır, çizim/arama/sıfırlama/geri al aynı tanımdan çıkar.

```rust
pub struct SettingDesc {
    pub id: &'static str,                 // "motion.speed"
    pub page: PageId, pub section: &'static str,
    pub title: &'static str, pub subtitle: &'static str,
    pub keywords: &'static [&'static str],// arama için eş anlamlılar ("animasyon", "hız")
    pub file: FileId, pub path: &'static str,
    pub control: Control,
    pub visible_if: Option<fn(&Model) -> bool>,   // ör. eğri ayarları yalnızca stil = curve iken
}
pub enum Control {
    Toggle { default: bool },
    Slider { min: f64, max: f64, step: f64, default: f64, unit: &'static str, integer: bool },
    Choice { options: &'static [(&'static str, &'static str)], default: &'static str, style: ChoiceStyle },
    Color { default: Color },
    Text { default: &'static str, validate: fn(&str) -> Result<(), String> },
    Custom(CustomId),                     // sayfaya özgü bileşen (önizleme, easing editörü, kenar seçici…)
}
```

Aralıklar ve varsayılanlar framework `DESIGN.md` ve shell `ARCHITECTURE.md`'deki doğrulama
sınırlarıyla aynıdır (birim testi: her `Slider`'ın varsayılanı aralığında ve şemada).

**Shell şeması (`nuxut-shell schema`, `services/widgets.rs`):** varsayılanların yanında her
`[[setting]]` / `[[widget.setting]]` tanımı (tür, `options`, `min`/`max`, `since`) noktalı yolla
okunur (`widgets::shell_setting("wallpaper.backdrop")`, widget ayarı `bar.widget.<ad>.<key>`).
shell.toml satırlarında:
- **Seçenekler** (`SettingDesc::choices`): şema yüklüyse değerler ve sıra shell'inki; görünen
  ad şemanın `option_labels`'ından, yoksa kayıttaki tablodan (`EDGE_CLICK`, `WALLPAPER_BACKDROPS`…;
  aşağıda **Seçenek adı önceliği**), adı hiçbir yerde olmayan değer ham adıyla gösterilir; shell'in tanımadığı yerel değer listeden düşer. Şema yoksa
  (eski/bozuk/çalışmayan shell) yerel tablo. Satır, şema sonradan gelip liste/aralık değişirse
  denetimini yeniden kurar (indis → değer hep satırın kurulduğu listeden).
- **Aralık** (`SettingDesc::bounds`): kayıttaki ürün aralığı ile şemanın doğrulama sınırının
  kesişimi (shell daraltırsa reddedilecek değer yazılmaz; genişletirse ürün aralığı kalır;
  kesişim boşsa yerel). Adım ve birim kayıtta.
- **`since`** (`SettingDesc::since` / `WidgetSpec::since`, `services::seen::badge(kimlik, since)`):
  "Yeni" rozeti (shell ADR-0002 §3.6; görülme modeli karar, 2026-10-13;
  `services/seen.rs`, `new_badge.rs`). Görülme **ayar bazlıdır**, sürüm bazlı değil:
  - **Durum:** Ayarlar durum dosyası `$XDG_STATE_HOME/nuxut/settings.toml` (yoksa
    `~/.local/state/nuxut/…`), `[seen]` tablosu `"<kimlik>" = <since>`. Kimlik shell ayarında
    shell.toml noktalı yolu (`notifications.dnd`, `bar.widget.clock.format`), ekleme menüsündeki
    widget'ta `widget:<ad>`. Kullanıcı config'ine (shell.toml/framework.toml) hiç yazılmaz. Her
    yazımda disk yeniden okunur; başka anahtarlar ve başka süreçlerin `[seen]` girdileri korunur
    (kimlik başına büyük olan), atomik yazılır. Bozuk dosya (TOML değil ya da `seen` tablo değil):
    rozet yok, üzerine yazmadan önce `settings.toml.bak` kopyası alınır (kopya alınamazsa
    yazılmaz); okunamayan (izin) dosyaya hiç yazılmaz.
  - **Rozet:** `since > 1` ve `[seen]`'deki kayıt < `since` (kayıt yoksa 0). Satır başlığının
    sağında küçük "Yeni" hapı (`settings-badge-new`; `tonal_container` zemin + `accent_on`
    metin, sabit renk yok); başlık rozet genişliği kadar daralır. Widget ekleme menüsünde
    düğmenin sağ üst köşesinde; arama sonuçlarında satırın sağında metin (`ListTrailing::Text`;
    arama sonucu görülme sayılmaz).
  - **Görüldü:** öğe kesintisiz ≥ 1 sn ekranda kalınca (`seen::Tracker`, `DWELL`): Ayarlar
    penceresi etkin (`xdg_toplevel activated`), sayfa açık, başlık + rozet sayfanın görünür
    alanında tamamen. Bekleme kare istemez (`Ui::wake_at`); kareler arası 1,25 sn'den uzun
    boşluk (pencere gizli, kare gelmedi) sayacı sıfırlar. Kayıt toplu ve seyrek yazılır
    (`FLUSH_DELAY` = 3 sn sonra, config yazımında ve kapanışta).
  - **Yumuşak kalkış:** görülen satırın rozeti hemen kaybolmaz (satır gözün önünde kaymasın);
    o sayfadan ayrılınca (`App::navigate` → `seen::retire`) ya da sonraki açılışta kalkar.
    Rozetler açılıştaki kayda göre hesaplanır.
  - **İlk açılış:** `[seen]` hiç yoksa (taze kurulum, bozuk dosya) rozet yoktur; şema ilk kez
    başarıyla gelince (`Event::Widgets`) şemadaki tüm `since`'li ayarlar ve widget'lar görüldü
    yazılır. Sonra gelen yeni `since`'li öğeler rozetlidir. Şema yoksa (shell yok/eski)
    tohumlanmaz; ilk başarılı şemada yapılır.
  - **Bilinen sınır:** test döneminde taze kurulum kipi (install.sh, ön sürüm "her güncelleme
    ilk kurulum") durum dosyasını da sıfırlıyorsa rozetler hiç görünmez — her açılış ilk açılış
    sayılır; beklenen davranış.
  - **Framework talebi:** `nuxut-ui` `Badge` yalnızca nokta/sayaç çizer; metinli rozet
    (`Badge::text` ya da `Pill`) ve listede hap biçimli sağ öğe gelince `new_badge.rs` onunla
    değiştirilir (şimdilik en basit primitive'ler: dolu yuvarlak dikdörtgen + `Caption` etiketi).
- **Denetim:** şema yüklenince `registry::audit_schema` yerel yedekle farkları bir kez uyarı
  olarak loglar (seçenek listesi/sırası, etiketsiz değer, aralık dışı, tam sayı türü, şemadaki
  `keybinds.*` eylemiyle kısayol satırları). Aynı karşılaştırma (`schema_mismatches`) birim
  testinde gerçek şemayla çalışır: şema süreç içinde shell crate'inden üretilir
  (`nuxut_shell::schema::generate`, `[dev-dependencies] nuxut-shell = { path = "../shell" }`;
  `nuxut-shell schema` aynı işlevdir). Karar (istek 2026-10-15): eskiden
  derlenmiş `../shell/target/.../nuxut-shell` ikilisi çalıştırılıyordu; shell yeniden
  derlenmeden bayat ikiliyle yanlışlıkla geçiyor, ikili yoksa PATH'teki kurulu (eski) shell'e
  düşüyor ya da sessizce atlanıyordu (e413de1/86dc342 sınıfı). Dev-bağımlılıkla cargo shell
  kaynağı değişince testi yeniden derler, atlama yoktur. Bedeli: Ayarlar'ın test derlemesi shell
  lib'ini de derler (ürün ikilisine girmez). Şema üretimini ayrı bir crate'e taşımak
  düşünüldü, reddedildi: `ShellConfig` ağacının tamamına, `WidgetKind`'e ve kısayol eylemlerine
  bağlı; taşımak kabuğun config modülünü bölmek demek. Birim
  testlerinde çalışma zamanı şema yüklemesi kapalıdır (testler yerel yedekle, ortamdan bağımsız;
  şema gereken test `widgets::set_test_schema` ile iş parçacığına özel şema kurar).
- **Dil (shell a273b45+):** şema çıktısı etkin dildedir (widget/ayar başlıkları katalogdan, her
  seçimde `options` ile aynı sıralı `option_labels`). Şema `RwLock<Option<Arc<Schema>>>`'tadır:
  okuyucu `Arc`'ı kopyalayıp kilidi bırakır (`widgets::catalog()` → `Catalog` görüntüsü,
  `shell_setting` → `Arc<SettingSpec>`); ayrıştırma yalnızca arka plan yüklemesinde. Ayarlar'ın
  dili değişince (`App::locale_changed`) `widgets::reload()` şemayı arka planda yeniden okur;
  sıra numarası eski yüklemenin yenisinin üstüne yazmasını engeller, yeniden okuma başarısızsa
  önceki şema kalır; bitince `Event::Widgets` → bar önizlemesi, Widget'lar, Seçili widget ve
  satır denetimleri (`Shape` farkı) yeni adlarla kurulur. Varsayılanlar (`shell_default`,
  `&'static`) dile bağlı değildir: ilk başarılı yüklemeden alınır, oturum boyunca sabittir.
- **Seçenek adı önceliği** (`resolve_choices`): shell'in `option_labels`'ı (Ayarlar'ın
  `settings-choice-*` adlarıyla aynı) → kayıttaki yerel tablonun katalog adı (`choice_label`) →
  Ayarlar kataloğundaki genel ad
  (kenar değerleri: `settings-edge-*`) → ham değer. Uzunluğu `options`'a uymayan dizi bütünüyle,
  boş ad tek tek yok sayılır.
- **Birimler** (`rows::unit_label`; i18n kararı A, 2026-10-13): şemanın `unit`'i
  (`int` ve `float`) dile bağımsız kapalı sembol kümesidir — `px`, `%`, `ms`, `s`. Görünen ad
  Ayarlar kataloğundan: `px` → `settings-unit-pixel`, `ms` → `settings-unit-ms`, `s` (ve eski
  shell'in `sn`'si) → `settings-unit-second`; `%` yüzde biçimine gider; bilinmeyen sembol
  olduğu gibi gösterilir.
- **Kayıt metinleri** (registry → katalog; tasarım 2026-10-13):
  `s!` tablosundaki başlık, alt yazı, arama sözcükleri ve bölüm adı **kaynak Türkçe** olarak
  kalır (kartları gruplayan kimlik ve kataloğun yedeği); görünen metin erişimcilerden, etkin
  dilde: `SettingDesc::label/description/section_title`, `PageId::title` (`source_title`
  Türkçe), `NavGroup::title`, `MotionRole::title/subtitle`, `DetailGroup::label/description` (`settings-detail-<key>-*`), `choice_label`. Anahtar kimlikten
  türer: `settings-<kimlik; . _ → ->-label|-description|-keywords`; hareket rollerinin ortak
  satırları `motion.<rol>.<x>` → `settings-motion-role-<x>`; bölüm `SECTION_KEYS` tablosundan
  (rol bölümleri `settings-motion-role-<rol>`); sayfa `settings-page-<slug>`; seçenek
  `settings-choice-<CHOICE_TABLES öneki>-<değer; : _ → ->` (boş değer `default`). Erişimciler
  `&'static str` döner: `i18n::text_or` dil başına önbellek (katalog değişince boşalır; kaynaktan
  farklı değerler dil değişimi başına bir kez sızdırılır). Kart başlığı çizilirken çevrilir
  (`page.rs`). **Arama** etkin dildeki metinlerde, kaynak Türkçede ve ayar yolunda eşleşir.
  **Kapsam testi** (`registry::catalog_tests`): her metnin anahtarı var ve Türkçe değeri kaynakla
  aynı — kayıtta metin değişirse test kırılır; katalog Türkçesi koddan kopyalanır, İngilizcesi
  ve çevirmen notu yeniden üretilir. `tests/registry_english.rs`
  İngilizce katalogda Türkçe harf kalmadığını ve iki dilde aramayı denetler. Kaynak tablo
  kalıcıdır (karar A, 2026-10-15; tabloyu Türkçesiz yeniden yazmak görünür bir şey
  değiştirmez, paylaşılan dosyada çakışma riski yüksek). Gömülü metin bekçisinin izni dardır
  (framework 319f088 makro izni; 2026-10-16, görev): `src/registry.rs:s!()` (kayıt
  satırları), `source_text!()` (kaynak tablolar: sayfa/grup/rol adları, bölüm anahtarları,
  seçenek etiketleri — ifadeyi aynen dönen işaret makrosu), `text_or()` (kaynak yedek) ve
  `out.push()` (`schema_mismatches`, yalnızca günlük/test). Eski `registry.rs:*` toptan izni
  kalktı: tablo dışına yazılan Türkçe metin artık bekçiye takılır. Satır numaralı izin
  seçilmedi (yüzü aşkın satır, her düzenlemede kayardı).
- **Marka sloganı** (`settings-brand-slogan`; kullanıcı kararı, 2026-10-13): her dilde
  çevrilir — birebir değil, anlamı koruyan doğal çeviri; "The penguin" tekil ve belirli (Tux
  göndermesi) kalır. Türkçe değeri kullanıcı yazdı: "Penguen uçamaz. Bu uçuyor." Hakkında ve
  Hoş geldin aynı anahtardan okur; test yalnızca her dilde boş olmadığını denetler.
- **Dinamik widget satırları** (`bar_preview::dynamic_desc`): `SettingRow` `'static` tanım ister;
  tanım yalnızca dile bağlı olmayan alanlarla (yol, tür/sınır/birim, yer tutucu) anahtarlanıp bir
  kez kurulur. Başlık (`title` boş → `SettingDesc::display_title` şemadan) ve seçenek adları
  (yerel tablo boş → şemadan) satır kurulurken o anki şemadan okunur: dil değişimi tanım
  önbelleğini büyütmez (testli: tr↔en beş tur, aynı işaretçiler, sayı sabit).

Değer okunurken dosyada yoksa `default` gösterilir (hareket rolleri, `motion.style` ve
`blob.deform` için dosyadaki ön ayarınki: `defaults::get(store, …)`); seçim satırında
varsayılanı seçmek, kaydırıcı/anahtar/metin/renk satırında varsayılana eşit değer yazmak
(`SettingDesc::stored`; sayılar toleransla, renkler büyük/küçük harf duyarsız) ve satırın
"varsayılana dön" eylemi anahtarı siler. Arama: başlık + alt başlık + anahtar kelimeler üzerinde Türkçe büyük/küçük
harf ve aksan duyarsız (ı/i, ş/s…) alt dizi eşleşmesi, sayfa sırasıyla.

**Aşamalı açıklama ve sayfa durumu** (`docs/design/settings-ux.md` §3, t094 P0). Kayıt satırının
isteğe bağlı üç alanı daha vardır (makroda sondaki `detail "<ad>"`, `mirror [Sayfa]`,
`ui_if <fn>`; varsayılanlar `Tier::Basic`, boş, `None`):

- `tier: Tier::Detail(ad)` — ayar kartın sonundaki **açılır satırın** (`pages/disclosure.rs`)
  içindedir; aynı adlı ayarlar ilkinin yerinde tek blokta toplanır (`pages::build_from`).
  Başlık/açıklama/özet adı `registry::DETAILS`'te. Açılır satır kapalıyken özet ("10 renk ·
  2 tanesi elle ayarlı") ve varsayılandan farklı değer varsa nokta rozeti gösterir; aranabilir:
  arama sonucu açar, satıra kaydırır, satırı parlatır. Başlık, chevron, açılma yayı, rozet,
  başlık odağı/klavyesi (Enter/Space, Sağ/Sol) ve arama vurgusu framework `nuxut::ui::Disclosure`'dır;
  blok yalnızca kayıt bağlamasını tutar: satırların yerleşimi (`set_content_height` → `header_height`
  → `height(now)` → `set_rect`; satırlar `content_rect` altına), özet/rozet hesabı, içeride Tab
  odağı, satır çizimi (`draw_content` kırpma + solma; satır ayırıcısı yok, §5.1). Arama vurgusu
  `flash_content` (içerik koordinatı; `timing.flash_hold` tam, sonra söner); satırın yeri/genişliği
  bekleyiş içinde değişirse (sayfa geçişi) dikdörtgen bir sonraki `place`te yenilenir.
- `mirror_on` — aynı satır başka sayfada da çizilir (tek anahtar, tek geri al, tek arama sonucu).
  Ayna sayfada satır kayıt sırasıyla yerleşir ve kendinden önceki satırın kartına girer
  (`pages/mirror.rs` yalnızca bölümü değiştiren sarmalayıcı); bu yüzden ayna satır kayıtta ayna
  sayfanın satırları arasına yazılır (panel kısayolları Paneller bloğunda).
- `CustomId::Alias(<kimlik>)` — kendi satırı olmayan ayar: arama onu `<kimlik>` satırına götürür
  (`pages/links.rs` `contains_id`), ör. `windows.shadow`/`windows.motion` → "Köşe, gölge ve hareket".
- `ui_if: UiCond { shown, reveal }` (ikisi de `Store`'u da alır: durumun anlamı config'e bağlı olabilir — seçili widget bar'ın ucunda mı) — dosyaya yazılmayan sayfa durumuna (`page_ui.rs`: açık açılır
  satırlar, seçili widget) bağlı görünürlük. `PageUi` sayfa nesnesiyle oturum boyunca yaşar.
  Arama gizli bir satıra giderken `Page::reveal` önce `reveal` ile durumu kurar (ör. widget'ı
  seçer); sözleşme testi config'e göre görünür satırda varsayılan durumdan `reveal` sonrası `shown`
  olmasını şart koşar. Blok sayfa durumunu `Block::take_ui` ile ister (Bar önizlemesinde tıklanan
  widget); `Page` işaretçi/tuş olayından sonra toplar ve değiştiyse tüm bloklara akıtır.

Sözleşme testleri: `pages::tests::every_setting_has_a_home` (her ayar kenar çubuğundaki bir
sayfada tek blokta, aramada bulunur), `search_finds_moved_settings_on_their_new_pages` (t094
taşımaları: çerçeve → Bar, paneller → Paneller, dil ve biçimler → Bölge ve dil; kimlik ve
anahtarlar aynı), `registry::tests::sidebar_groups_follow_design`, `moved_settings_live_on_their_new_pages`.

Satır yüksekliği: tek satırlık (yan yana) satırlar `rows::row_height_for(theme, içerik)` =
içerik + dikey iç boşluk, en az framework `style::row_height` (`space(16)` = 64 px; açılır satır
başlığı da aynı). Arama vurgusu düz satırda (`Page::reveal`) ve açılır satırda aynı: `FLASH_TONE`,
`timing.flash_hold` tam (bekleyiş `wake_at`, kare yok), sonra `motion.effects` ile söner.

Satır kalıbı (`rows.rs`): `ChoiceStyle::Dropdown` framework `Select`'tir; menüsü kaplama
katmanında çizilir (satır kırpmasında kalmaz), açıkken işaretçi ve tuşlar yalnızca o satıra
gider. Tema değişimi `Select::set_theme(theme, now)` (metin sistemi istemez, ölçüm tembel; açık
menü de yeni temaya geçer). **Yığılan satır** (`rows::should_stack`, §4/§5.1; kayıt satırı, dil satırları, Ekranlar ve Hakkında aynı kuralı kullanır): içerik sütunu `space(140)`'tan darsa, kontrolün yanında metne `space(30)`'dan az yer kalıyorsa, başlık tek satıra ya da açıklama (≤ 1 satır kuralı) kontrolün yanına sığmıyorsa geniş kontroller (kaydırıcı, segmentli, metin,
açılır seçim) başlığın altına tam genişlikte iner; anahtar ve renk yerinde kalır. **Yüzde** tek yerden: `numfmt::percent_in` (framework `NumberOptions::percent`); `percent` kaydırıcılar (0..1) ve birimi `%` olanlar (tekerlek adımı, en yüksek ses, ekran ölçeği) bundan geçer.

## 5. Durum modeli ve akış

```
olay (pointer/key/IME) ─► sayfa ─► Edit { file, path, value } ─► Store.set ─► History
                                                                    │
                                    timer 150 ms ◄───────────────────┘
                                         │ flush (atomik)
                                         ▼
                 inotify ─► theme_changed / config_changed ─► on_disk_changed
                                         │ OwnWrite → yok say │ Reloaded → sayfalar yenilenir
```

- Sayfalar değerleri her zaman `Store`'dan okur (ayrı kopya yok); `Store` değişince
  `revision` artar, görünür sayfa kendi bileşenlerini `sync(&Store, now)` ile günceller
  (bileşenler değere yayla akar; sürüklenen bileşen kendi değerini korur).
- Ağır işler (palet çıkarma, D-Bus, portal) arka plan thread'lerindedir; sonuç
  `MessageSender<Msg>` ile döner. Thread'ler ilk ihtiyaçta açılır (sinyal maskesi —
  framework çalışma zamanı kuralı).
- Boşta sıfır iş: animasyon yoksa kare yok, timer yalnızca bekleyen yazım/şerit için.

**Diskteki dosyalar** (shell `install.sh` taze kurulum kipi için). Ayarlar'ın KENDİNE ait
kalıcı durum dosyası yok: pencere boyutu/sayfa, geri al geçmişi (`History`, ≤ 100, bellekte),
Hoş geldin ilerlemesi (`WelcomeView`, bellekte; sonuç shell'e `msg setup done|skip`),
tema önizleme önbelleği (`PreviewCache`, 8 MiB bellekte) süreçle biter. Yazdığı/dokunduğu:
- `$XDG_CONFIG_HOME/nuxut/framework.toml`, `shell.toml` — kullanıcı config'i (ortak; shell ve
  framework okur). Silinirse varsayılanlar.
- `$XDG_STATE_HOME/nuxut/palette.toml` — duvar kağıdından üretilen palet (Ayarlar renk
  kaynağı "duvar kağıdı" iken yazar; framework okur, Görünüm'de `meta.mode` yazılır).
  Silinirse tema sabit renklere döner, palet yeniden üretilene dek.
- `$XDG_STATE_HOME/nuxut/exports/` — palet şablon çıktıları (terminal vb. renkleri;
  `nuxut_palette::export`, palet her üretilişte yeniden yazılır). Silinirse sonraki üretimde
  geri gelir.
- `$XDG_RUNTIME_DIR/nuxut/settings[-<örnek>].sock` + `.lock` — tek örnek IPC (geçici, oturumla
  biter; yedeklenmez).
- Okur ama yazmaz: `$XDG_STATE_HOME/nuxut/setup.toml` (shell'in; `msg setup status` ile),
  `$XDG_STATE_HOME/nuxut/snapshots/` (shell'in; `msg snapshot …`), takvim hesapları (takvim
  daemon'unun).

## 6. Hesaplar — takvim akışı (`src/services/calendar.rs`)

Arka plan thread'i `zbus::blocking::Connection::session()` + `CalendarProxyBlocking`.
UI → thread: `std::sync::mpsc` komutları (`Refresh`, `Begin(provider, reconnect)`,
`Submit(flow, FlowInput)`, `Cancel`, `ReopenBrowser`, `SetCalendarEnabled`, `SyncNow`,
`Remove`). Thread → UI: `Msg::Calendar(CalendarEvent::{Accounts, Calendars, Providers,
Flow(id, FlowState), Unavailable(msg)})`. Sinyaller (`AccountsChanged`, `CalendarsChanged`,
`FlowChanged`) ayrı bir dinleyici thread'inde alınır ve listeyi tazeler.

UI akışı — her ekranda tek soru, büyük düğme:

| `step` | Ekran |
|---|---|
| (başlangıç) | sağlayıcı kartları: Google, Microsoft, Nextcloud, iCloud/Fastmail/diğer CalDAV, ICS aboneliği (yalnızca `AvailableProviders`) |
| `need-server` | "Nextcloud sunucunuzun adresi" + ipucu; Enter = devam |
| `need-email` | "E-posta adresiniz" |
| `need-password` | "{sağlayıcı} uygulama şifresi" + "Uygulama şifresi oluştur" bağlantısı; `server` boşsa sunucu alanı |
| `need-url` | "Takvim adresi (webcal:// ya da https://)" |
| `waiting-browser` | spinner + "Tarayıcıda devam edin…", "Bağlantıyı yeniden aç"; `user_code` varsa büyük kod + adres |
| `connecting` / `discovering` | spinner + "Takvimler bulunuyor… ({found})" |
| `done` | ✓ "{n} takvim eklendi" → takvim listesi (renk + göster/gizle) |
| `failed` | Türkçe mesaj + "Tekrar dene" (`retry_step`'e döner, girdi korunur) / "İptal" |

Akış daemon'da yaşar; Ayarlar yeniden açılırsa `ListFlows` ile süren akışa dönülür
(`nuxut-settings --flow <id>` ya da IPC `flow` komutu belirli bir akışı açar).
Daemon yoksa (D-Bus etkinleştirme başarısız) sayfa "Takvim servisi çalışmıyor" + "Tekrar
dene" gösterir; diğer sayfalar etkilenmez.

## 7. Duvar kağıdı ve palet (`src/services/{portal,palette}.rs`, `src/pages/wallpaper.rs`)

- **Dosya seçimi:** `org.freedesktop.portal.FileChooser.OpenFile` (zbus, arka plan
  thread'i). Yanıt `Request::Response` sinyaliyle gelir; yarış olmasın diye abonelik
  çağrıdan önce kurulur (tutamaç yolu `handle_token`'dan bilinir). Filtre: `image/*`
  (shell video çizmiyor); başlangıç klasörü `~/Resimler` ya da `~/Pictures`. `file://` URI'si
  çözülür; `..`/NUL içeren yol reddedilir (testli).
- **Küçük resim:** `nuxut_image::Wallpaper` (paylaşılan çözücü, doku, çapraz geçiş);
  çözüm bitince `services::wake` → yeniden çizim. Animasyonlu görselin ilk karesi gösterilir
  (`animated_source`; oynatım nuxut-video içe aktarımına bırakıldı).
- **Duvar kağıdı çözümü yalıtılmış işçide** (2026-10-13; framework tasarımı
  `docs/design/wallpaper-sandbox.md` + ADR-0007 takip, onay): kullanıcının
  seçtiği görsel güvenilmeyen girdidir, Ayarlar süreci onu kendisi çözmez. `services/wallpaper.rs`
  süreç başına tek `nuxut_image::WallpaperDecoder` kurar (`Sandbox::new(SandboxConfig::wallpaper())`,
  `cache_bytes` 16 MiB, `linger` varsayılan 10 sn); hero, Görünüm önizlemesi, monitör satırları ve
  renk örneklemesi hep onun üzerinden gider (birleştirme/türetme/önbellek ortak). Hedefler:
  hero ve Görünüm önizlemesi **kartın fiziksel boyutu** (`mantıksal × painter.scale()`, genişlik
  64 px katına yukarı yuvarlanır — pencere sürüklenirken her pikselde yeniden çözülmez — oran
  korunarak en çok 1280×800), `Cover`; monitör satırı `192×120 × ölçek`, `Cover`; renk ayrı istek
  `128×128 Contain` → `nuxut_palette::source_color_from_frame` (Cover kırpması kenar renklerini
  atardı; elde daha büyük Contain kare varsa framework türetir). `set_source` `draw`'da çağrılır
  (hedef yerleşim + ölçek orada belli; aynı kaynak + hedef + yerleşim yinelenmez).
  **JPEG XL çözücüsü (jxl-oxide) Ayarlar ağacında yok:** `nuxut-image` yalnızca `render`,
  `nuxut-palette` yalnızca `image` özelliğiyle (`cargo tree -i jxl-oxide` → paket yok).
  **İşçi yoksa** (framework S1): PNG/JPEG/WebP/GIF/QOI süreç içinde kapılarla, JXL/AVIF/SVG
  `Sandbox(Unavailable)` → kart ikonlu yer tutucuda kalır, Şema satırı katalog metni gösterir
  (`settings-wallpaper-error-worker`; çok büyük görsel `-error-too-large`, diğerleri
  `-error-source`); log yalnızca hata türü, yol yok. Boşta: çözülmüş kare, çözüm thread'i ve
  işçi süreci yok.
- **Palet `Store`'da sahte dosyadır** (`FileId::Palette`, `palette.toml`): yalnızca
  `[meta] image/scheme/mode/contrast` düzenlenir, böylece geri al/birleştirme/arama diğer
  ayarlarla aynı yoldan çalışır. `flush` bu dosyayı toml_edit ile **yazmaz**; bekleyen
  değişiklik bir `PaletteRequest` olur, palet thread'i `nuxut_palette::Palette::generate`
  ile üretip `GeneratedPalette::save` (atomik) ve şablon dışa aktarımlarını yazar. Runtime
  `palette.toml`'u izlediği için `theme_changed` gelir → belge diskten yenilenir (meta =
  istenen parametreler; döngü yok). Tohum renk görsel başına (yol + mtime) önbelleklenir;
  kuyruktaki eski istekler atlanır (kaydırıcı sürüklenirken yalnızca sonuncusu).
- **Şema seçici** (Görünüm › Renkler; karar 5): dokuz varyantın seçili kip/kontrastta 6 renklik şeridi (primary,
  secondary, tertiary, primary_container, surface_container, surface); tıklanan hemen uygulanır.
- **Shell anahtarları:** seçim tek geri al adımında `shell.toml [wallpaper] path` ve palet
  `meta.image`'a yazılır; gösterilen görsel önce `wallpaper.path` (`~` açılır), yoksa palet
  görseli. `wallpaper.fit` satırı.
- **Kenar boşluğu zemini** (shell 88885da, şemada `since = 2`): `wallpaper.backdrop`
  (`blur` bulanık kopya | `color` düz renk), `wallpaper.backdrop_blur` (tam sayı px, 4..=128;
  yalnızca `blur` seçiliyken görünür — `visible_if`, dosyadaki değere bağlı olduğu için
  `ui_if` değil), `wallpaper.backdrop_color` (tema rolü `surface | surface_container |
  surface_bright`; sabit renk yok, her kipte görünür: bulanık kopya hazırlanırken de yedek).
  Varsayılanlar, seçenekler ve yarıçap sınırı şemadan; yerel tablolar (`SHELL_DEFAULTS`,
  `WALLPAPER_BACKDROPS`, `WALLPAPER_BACKDROP_COLORS`, `WALLPAPER_BACKDROP_BLUR`) etiket ve
  şemasız yedek, gerçek şemayla birim testli. "Yeni" rozeti yalnızca model (`since`).
- **Monitör başına** (`src/pages/wallpaper_outputs.rs`): `[wallpaper.outputs."<bağlayıcı>"]
  path, fit` (ikisi de opsiyonel, en çok 32). Satırlar = bağlı Wayland çıkışları (`App`
  çıkış olaylarından `services::outputs`, `Event::Outputs`) ∪ config'te yazılı olanlar ∪ elle
  eklenen adlar (bağlı olmayan monitör; seçim yapılana dek dosyaya yazılmaz). Satır başlığı
  Ekranlar'daki insan adı (§8 t094): blok kurulunca bir kez `services::displays` listesi istenir
  (`Command::Refresh`; cevap tüm sayfalara `Event::Displays` olarak yayılır), bağlayıcı → ad ve
  mantıksal boy modül önbelleğinde (`known_from`: `Display::name()` = model, yoksa açıklama);
  shell yoksa Wayland açıklaması (sondaki " (DP-1)" atılır), o da yoksa bağlayıcı. Aynı ad iki
  kez çıkarsa bağlayıcı eklenir. Her satırda monitör oranlı küçük resim (o monitörde görünen
  görsel ve yerleşim, zemin dahil), yerleşim açılır seçimi (`Select`; "Genel · <genel
  yerleşim>" = `fit` anahtarı yok — segment değil: dar pencerede "Doldur v…" kırpılıyordu),
  görsel seç, × (iki anahtarı tek geri al adımında siler; yeri hep ayrılır, seçiciler hizalı).
  Elle ekleme kartın sonunda framework `Disclosure`'ında ("Bağlı olmayan monitör ekle").
- **Yerleşik Nuxut duvar kağıtları** (sözleşme `wallpaper-builtins-v1` + EK KARARLAR, ayrıntı
  `docs/design/wallpaper-builtins.md`): dizin, adres ayrıştırma (`parse_source`), tohumlar,
  adlar ve render shell deposundaki `nuxut-wallpapers` crate'indedir; Ayarlar kopya tutmaz.
  Duvar kağıdı sayfasında hero'nun altında "Nuxut duvar kağıtları" kart ızgarası
  (`wallpaper.builtin`, `CustomId::WallpaperBuiltins`, `src/pages/wallpaper_grid.rs`
  `WallpaperGrid`; Hoş geldin görünüm adımında aynı bileşen 2 sütunla). 9 kart (3×3; dar
  sayfada 2 sütun), 16:9, etkin renk kipinin varyantı (temanın çözdüğü kip; `live_theme`
  bellekteki belgeden kurduğu için kaydedilmemiş Koyu/Açık seçimi de yansır). Önizleme gerçek
  SVG'den, kartın fiziksel boyutunda tek arka plan thread'inde (crate `render_preview` bloklar;
  `services::wallpaper_source::BuiltinRender`, yalnızca son istek), hazır olana dek tema
  `surface_container` dolgusu (crate ortalama renk vermez). Adlar crate'in kataloğundan
  (`Builtin::name`; `nuxut_wallpapers::i18n_source()` Ayarlar'ın i18n kaynaklarına eklenir).
  Seçim tek geri al kaydıyla `shell.toml [wallpaper] path` ve `palette.toml [meta] image`'a
  varyantsız `nuxut:<id>`; zaten seçili kart yazmaz. Palet görsel çözülmeden tohumdan
  (`Builtin::seed`, `services::palette::PaletteSource::Seed`). Klavye Görünüm › Uygulamalar
  ızgarasıyla aynı.
  - **Kararlar:** 2026-10-13 — kimlikler İngilizce, gömülü SVG'ler (diskte index/SVG
    yok), Aurora v1'de; EK KARARLAR — crate shell deposunda, maskotlu 3 yerleşik (`cloud-rest`,
    `far-glide`, `chase`), adlar crate kataloğunda. 2026-10-13 — boş yol (ve palet
    görseli yok) = varsayılan kart (`pair-flight`) seçili görünür, dosyaya yazılmaz; palet
    tohumdan.
  - **Kaynak çözümü** (`services::wallpaper_source`): `WallSource::{File, Builtin}` crate
    `parse_source` üzerine; geçerli kaynak önce `wallpaper.path`, yoksa palet görseli, ikisi de
    yoksa varsayılan yerleşik. Geçersiz/bilinmeyen `nuxut:` adresi → crate'in yedeği
    (varsayılan) + uyarı. Hero, monitör başına satırlar ve Görünüm önizlemesi `WallThumb`
    kullanır: dosya framework `Wallpaper`'ına (sürecin paylaşılan yalıtılmış `WallpaperDecoder`'ı;
    hedef boy `draw`'da `card_target`/`row_target`), yerleşik süreç içi çizime gider — `nuxut:` adresi
    dosya çözücüye asla gitmez. Yerleşik her zaman cover (yerleşim/zemin yok sayılır).
  - **Yerleşim/zemin satırları** (2026-10-13): yerleşik her zaman cover, bu yüzden
    `wallpaper.fit` yalnızca genel görsel ya da bir monitörün kendi görseli dosyayken görünür
    (`registry::wallpaper_fit_matters`); kenar zemini satırları yalnızca dosya gösteren bir
    yüzeyin etkin yerleşimi "Tamamı" iken (`wallpaper_letterbox`). Monitör başına satırda o
    monitörde yerleşik görünüyorsa yerleşim seçicisi gizli (yeri ayrılır, hizalar bozulmaz).
  - **Tohum kipi** (2026-10-13): paletin `[meta] mode`'u — palet o kipte üretilir.
  - **Bilinen sınır:** palet şablon dışa aktarımlarına (`export_templates`) yerleşik adres
    gitmez, görsel boş verilir (`palette::template_image`). Crate'in PNG önbelleği
    (`cache::ensure`) değerlendirildi, kullanılmadı: önbellek LRU ile budanır (24 dosya / 96 MiB,
    shell de yazar), şablona yazılan yol sonradan silinebilir. Kalıcı bir dışa aktarım yolu
    (budanmayan) gelince bağlanır — takip işi.
- **Önizleme ölçeği:** hero, monitör küçük resimleri ve Görünüm önizlemesi aynı
  `wallpaper::Backdrop` çözümünü kullanır: zemin yalnızca `Contain`'de; bulanıklık yarıçapı
  önizleme genişliği / ekranın mantıksal genişliği oranında (ekranda göründüğü kadar bulanık).

## Ekran okuyucu (AT-SPI; framework ADR-0011, t127)

Karar 2026-10-15/16 (t127; framework'te uygulandı). `nuxut` `a11y` özelliği açık.

- **Ağaç (Ayarlar penceresi):** kök = uygulama başlığı; `Toolbar` (geri al, son değişiklikler,
  küçült, büyüt, kapat — adlar `settings-a11y-*`); arama alanı; kenar çubuğu sayfa listesi (açık
  sayfa `selected`, anahtar sayfa kimliği) ya da arama sonuçları / boş durum; açık sayfa `Pane`
  (ad = sayfa başlığı) → görünen bölüm başlıkları (`Heading`) ve bloklar ekran sırasıyla
  (`Page::access`, kaydırma dışındakiler yok; odaktaki hariç).
- **Bloklar:** `Block::access` (varsayılan boş). `SettingRow` uygular: denetim satır başlığıyla
  adlanır, alt metin açıklama, anahtar ayar kimliği; anahtar → Switch, kaydırıcı → Slider (şema
  aralığı, görünen değer metni), bölümlü → RadioGroup, çipler → RadioGroup + düğmeler, renk →
  Button (açıklamada hex), metin → TextInput (parola alanı maske), açılır seçim → ComboBox;
  varsayılana döndür düğmesi "<ayar>: varsayılana döndür". Satır odaktaysa denetim ağacın odağı.
  Özel bloklar (tema/duvar kağıdı ızgaraları, kısayollar, ekranlar…) henüz ağaçta yok.
- **Eylemler (AT yalnız gezinir; framework ADR-0011 madde 7, 2026-10-17):** a11y bus'ı oturumdaki
  her sürece (Flatpak dahil) açık; AT eylemi kullanıcı niyeti sayılmaz. AT `Click` yalnız kenar
  çubuğu sayfa satırlarında ve arama sonuçlarında sunulur ve uygulanır (düğüm ortasına
  uygulamanın kendi işaretçi işleyicisine tıklama, `src/app/a11y.rs`). Diğer her düğümden
  (anahtar, seçim, kaydırıcı, renk, varsayılana döndür, geri al, son değişiklikler, pencere
  düğmeleri, boş arama durumu) değiştiren eylemler (`Click`, `Increment`, `Decrement`,
  `SetValue`, `Expand`, `Collapse`) ağaçtan silinir (`gate_actions`); `Focus` kalır. Değer
  değişimi yalnız gerçek klavye ve işaretçiyle. Hoş geldin penceresi ağaç yayımlamaz ve AT
  eylemi almaz: Uygula/Bitir yalnız gerçek girdiyle. Bekçi `tests/at_policy.rs` (yeni ağaç ya da
  eylem işleyicisi `AT_POLICY:` belgesi ister), birim `assistive_actions_only_navigate`.
  Canlı (yalıtılmış a11y ortamı, düşmanca istemci `at_probe`): "Yüksek karşıtlık", "Nuxut
  teması", "Son değişiklikler", "Kapat" düğümlerinde Action arayüzü yok; sayfa satırı "Hareket"
  `DoAction` ile açıldı.
- **Güvenlik:** framework kapısı (askı/`input_allowed`) geçerli; Wi-Fi/VPN parola alanları
  `PasswordInput` + maske, AT'den metin yazılamaz (ADR-0011 tehdit modeli).

## 8. Kod düzeni

| Yol | İçerik |
|---|---|
| `src/main.rs` | CLI, tek örnek, log, `run_with` |
| `src/store/` | `ConfigFile`, `KeyPath`, `Store` (framework/shell/palet), atomik yazım, dış değişiklik algılama (testli) |
| `src/history.rs` | geri al / yinele, birleştirme, tek kaydı geri alma (testli) |
| `src/registry.rs` | `SettingDesc` tablosu, Türkçe arama, sınır/varsayılan testleri; shell.toml seçenek/aralık/`since` şemadan (`choices`, `bounds`, `resolve_*`), yerel yedekle fark denetimi (`schema_mismatches`, `audit_schema`; gerçek şemayla testli) |
| `src/live_theme.rs` | bellekteki belgeden tema (+ palet katmanı), renk geçişi |
| `src/block.rs` | `Block` trait'i, `Env`, `Action` |
| `src/page.rs` | sayfa: bölüm kartları, kaydırma, görünürlük yayı, odak, arama vurgusu |
| `src/rows.rs` | standart satır: anahtar, kaydırıcı, SegmentedButton, haplar, satır içi ColorPicker, metin |
| `src/app.rs` | `nuxut::App`: pencere/CSD, kenar çubuğu + arama, olaylar, imleç, timer'lar, IPC, test senaryosu |
| `src/pages/preview.rs` | Görünüm önizlemesi (gerçek duvar kağıdı + yerleşim/zemin; blob panel, gölge, esneme; shell'in gerçek dört kenarı ve bar kenarı) |
| `src/pages/motion.rs` | rol başına CurveEditor (yay/bezier sürükleme, oynayan önizleme) |
| `src/pages/motion_preset.rs` | hareket ön ayarı kartları, "Özelleştirilmiş" / "Ön ayara dön" |
| `src/pages/fonts.rs` | tipografi önizlemesi, ikon seti seçici, rol başına yazı tipi seçici |
| `src/welcome.rs` | Hoş geldin saf modeli: karşılama + 3 adım (renk/duvar kağıdı, uygulamalar, masaüstüyle ortak: `compositor.sync_look/windows/motion/keybinds`) + bitiş; ileri/geri/atla, ilerleme; sonuç shell'e bildirilir (ADR-0001 §10.5): bitince `msg setup done <n>`, bitmeden kapatılınca `msg setup skip <n>` (n = `WELCOME_FLOW_VERSION`, Hoş geldin akış sürümü 1..=65535, akış değişince artar; pencere kapanırken, yazımlar diske indikten sonra; SIGTERM bildirmez); durum `msg setup status` → `{first_run, setup_version, setup_state, hypr{session, source, launch}, migration{pending_notice}}` (bilinmeyen değerde kırılmaz). `setup.toml`'u yalnızca shell yazar; karşılamayı shell `nuxut-settings --welcome` ile ayrık ve `NUXUT_*` ortamı olmadan açar (normal örnek; çalışan Ayarlar varsa ona IPC `welcome`) |
| `src/welcome_ui.rs` | Hoş geldin görünümü: ortalanmış sütun, karşılama ve bitiş ortalanmış kahraman düzeni (yatayda ortalı, metinler `TextAlign::Center`; blok düğme çubuğuna kadarki alanda dikeyde ortanın biraz üstünde, logonun görünür üst/alt kenarıyla ölçülür): karşılamada 128 px iki tonlu marka (`BrandMark::duotone`, renkler temadan) → başlık → slogan (`primary`, Label) → açıklama; bitişte onay ikonlu daire → başlık → açıklama; adımlarda marka yok, metinler sola hizalı; ilerleme noktaları, adımın satırları kayıttaki bloklar (`pages::build_ids`, tek başlıksız kart — kontrol kopyası yok; Uygulamalar adımında birleşik görünüme bağlı satırlar kayıttaki `unified_on` koşuluyla, kapalıyken Görünüm'le ortak açıklama bloğu), Geri / Atla / İleri; kendi penceresinde (`nuxut-welcome`, sabit 720×640; Ayarlar penceresinden bağımsız, §Pencere) |
| `src/pages/look.rs` | Görünüm › Renkler koyu/açık (+ `palette.mode` eşanlamlısı; `links::is_alias_of`), › Uygulamalar: birleşik görünüm durumu (`LookStatus`: framework `Notice`, ton → düzey: uygulandı = `Success` (`CheckCircle`, `Palette::success`), eksik paket/hata = `Warning`, diğerleri `Info`), kapalıyken açıklama + "Birleşik görünümü aç" (`UnifiedOffNote`, Hoş geldin ile ortak) |
| `src/pages/theme_grid.rs` | ikon/imleç teması kart ızgarası (önerilen kart, shell kataloğundan aile başına tek kart + seçili ailede varyant seçici, kurulu olmayan aile = seçilmez kart + kurulum komutu (kurulu AUR yardımcısıyla), v2 düşüşü, klavye) |
| `src/pages/theme_card_preview.rs` | tema kartının önizleme şeridi (framework `CursorPreview`/`IconThemePreview`; önbellek anahtarı, ölçek değişiminde sıfırlama, doku yükleme) |
| `src/services/input.rs` | `msg input status` (girdi-v1 sürüm 1) istemcisi: saf ayrıştırma (ad/seçenek doğrulaması, boş metin = bilinmiyor, `available:false` → değer yok), `InputState`, `status_note`, `override_note`; arka planda tek sorgu (`refresh`, 2 sn zaman aşımı `IpcLink`, `Event::Input`), `ensure` (sayfa ilk kurulurken), `recheck` (odak dönünce), `effective` (satır değeri) (UI'dan bağımsız) |
| `src/services/xkb.rs` | XKB düzen kataloğu: `evdev.xml` (roxmltree) → `XkbCatalog` (düzenler, varyantlar, `grp:` seçenekleri); sınırlı ve doğrulanmış adlar, Türkçe katlamalı `search`, `display_name`; arka planda bir kez (`catalog()`, `Event::Xkb`) (UI'dan bağımsız) |
| `src/services/previews.rs` | tema önizlemesi çözme servisi: paylaşılan `PreviewCache` + düşük öncelikli `Loader`, ızgara başına teslim kutusu (UI'dan bağımsız) |
| `src/services/wallpaper.rs` | duvar kağıdı çözümü: süreç başına tek `WallpaperDecoder` (yalıtılmış işçi, duvar kağıdı profili), kart/satır/renk hedefleri, hata → katalog metni (§7) |
| `src/typography.rs` | tipografi rolleri (başlık, örnek), bilinen ailelerin Arch paketleri |
| `src/pages/edges.rs` | bar kenarı satırı (kompakt şema, standart satır yüksekliği), dört kenarın kalınlığı (bar = `bar.size`, genel/elle + genele bağla) |
| `src/pages/bar_preview.rs` | Bar hero'su (tıklanabilir önizleme), Seçili widget başlığı + dinamik widget ayarları, seçim yardımcıları (`selected`, `selected_end`, `end_widget`) |
| `src/pages/link.rs` | başka sayfadaki ayara yönlendiren satır (`CustomId::Link`) |
| `src/pages/keyboard.rs` | Klavye özel blokları: düzen listesi + seçici, düzenler arası geçiş, deneme alanı; saf liste işlemleri (`current_layouts_with`, `with_added`, `without`, `moved`, `options_with_switch`, `picker_results`) testli |
| `src/pages/input_note.rs` | Klavye/Fare sayfa notu (eşitleme kapalı + "Aç" / kabuk durumu) ve `[input]` değişince durum tazeleme |
| `src/services/default_apps.rs` | `msg defaults status/set` (defaults-v1 sürüm 1) istemcisi: saf ayrıştırma (desktop-id/ikon adı doğrulaması, metin temizliği, aday sınırı, bilinmeyen rol/alan yok sayılır), `set` hata kodları (`unknown_role`, `unknown_app`, `unavailable`, `writer_disabled`, `write_failed`) → katalog metni; arka planda tek sorgu (`refresh`, `Event::DefaultApps`), `ensure`, `recheck` (odak dönünce) (UI'dan bağımsız) |
| `src/services/app_icons.rs` | uygulama ikonu önbelleği: framework `nuxut_image::IconTheme` + önizleme servisinin yalıtılmış `Loader`'ı (ad güvenilmez → `Untrusted`, SVG işçide); UI thread'i dosya okumaz, sonuç `wake` ile |
| `src/pages/default_apps.rs` | Varsayılan uygulamalar sayfası: üst not (`DefaultAppsNote`), rol satırı (`DefaultAppRow`; saf `RowModel`: seçenek sırası, yer tutucu, notlar) |
| `src/pages/keybinds.rs` | kısayol satırı: tuş başlıklı alan, yakalama (Esc iptal, Geri silme temizler), temizle/varsayılana dön, çakışma uyarısı |
| `src/keybind.rs` | kısayol dizgesi ayrıştırma/yazım (shell `parse_setting_combo` ile aynı kurallar), tuş yakalama (`KeyEvent::xkb` → Hyprland seviye-0 keysym adı, ad tablosu yok), gösterim adları, çakışma (testli) |
| `src/pages/sections.rs` | bar bölümleri ve panel kartları (ReorderList, listeler arası taşıma, ekle/kaldır; kenarsız kuyu; dar kartta sütunlar alt alta; bar'da satır = widget seçimi) |
| `src/pages/wallpaper.rs` | Duvar kağıdı hero'su (monitör oranlı önizleme), Görünüm'e renk yönlendirmesi, şema varyantları, ortak yerleşim/zemin çözümü (`Backdrop`) |
| `src/pages/wallpaper_grid.rs` | yerleşik duvar kağıdı kart ızgarası (`WallpaperGrid`, sayfa ve Hoş geldin), `BuiltinThumb` (fiziksel boyutta arka plan çizimi + doku), `WallThumb` (dosya → framework `Wallpaper`, `nuxut:` → süreç içi çizim; hero, monitör satırları, Görünüm önizlemesi) |
| `src/services/wallpaper_source.rs` | duvar kağıdı kaynağı, `nuxut-wallpapers` crate'i üzerine (`WallSource`, `current`, `selected_builtin`, `choice_writes`, `seed_for`, `expand_home`) ve yerleşik çizim kuyruğu (`BuiltinRender`); UI'dan bağımsız, testli |
| `src/pages/wallpaper_outputs.rs` | monitör başına duvar kağıdı (Ekranlar adları, küçük resim, yerleşim seçimi, elle ekleme açılırı); Ekranlar ad/boy önbelleği |
| `src/pages/accounts.rs` | takvim hesapları ve ekleme akışı; durum satırı daemon `state_reason_message`/`last_error_message` metnini gösterir, `calendar-err-*` anahtarı asla gösterilmez (calendar ADR-0007 §8; metinsiz eski daemon → genel metin; testli). Servis hataları `WriteFailureInfo::from_zbus` ile anahtar/metin ayrılır |
| `src/pages/about.rs` | marka başlığı (logo + ad + slogan + sürüm, dar sütunda yığılır), dosya yolları |
| `dist/` | kurulum dosyaları (repoya; Ayarlar kendisi kurmaz): `nuxut-settings.desktop`, `nuxut-welcome.desktop` (gizli), `icons/hicolor/scalable/apps/nuxut-settings.svg` (§1 Kurulum; `tests/dist.rs`) |
| `src/pages/locale.rs` | Dil ve biçimler: `framework.toml [locale]` açılır seçimleri (Otomatik = anahtar yok; şema/doğrulama framework `LocaleSetting`). Dil: `nuxut::i18n::available_languages()` (yerel geri düşüş yok), adlar kendi dilinde (`language_label`). Biçimler: Otomatik + `nuxut::locale::available()` (menüde `Locale::display_name`, ada göre sıralı; dosyadaki ad kanonik ada çözülür, tanınmayan ad Otomatik + sıfırla). Önizleme: `Locale::format_system_time(now, DateTime(Short))` + `Locale::format_number` (testli) |
| `src/pages/datetime.rs` | Tarih ve saat blokları: üst not, otomatik saat, saat dilimi seçicisi, elle tarih/saat; saf yardımcılar (`zone_label`, `search_zones`, `offset_text`, `local_parts`/`local_to_usec`, durum metinleri) testli |
| `src/pages/user.rs` | Kullanıcı hesabı blokları: üst not, profil (avatar + kimlik + Değiştir…), görünen ad; saf yardımcılar (durum metinleri, kimlik satırı, görünürlük) testli |
| `src/pages/network.rs` | Wi-Fi ve VPN blokları: üst not (`NetworkNote`), Wi-Fi aç/kapa (`WifiStatusRow`), ağ listeleri (`NetListBlock`: bağlı/görünen/gizli ağ/kayıtlı/kablolu/VPN/içe aktarma; satır altında tek panel: şifre, ayrıntı, kayıtlı ağ ayarları, sil, gizli ağ formu); saf `content` → `RowData` (ikon, alt metin, basış, meşgul, hata, ayrıntı) ve hata/içe aktarma/güvenlik/bant metinleri testli |
| `src/pages/bluetooth.rs` | Bluetooth blokları: durum (`BtStatus`: maskotlu hata ya da eylem hatası notu), güç ve görünürlük anahtarları, adaptör adı, cihaz listeleri (`BtDeviceList`: eşleşmiş + satır içi ayrıntı, yakındaki + eşleştir/iptal, tarama ve adsız satırları, klavye hedefleri); saf `content` → `RowData` ve hata/güç/görünürlük/sinyal metinleri testli |
| `src/pages/night_light.rs` | Ekranlar › Gece ışığı özel satırları: salt okunur konum (`NightLocation`; koordinat/olay metni saf yardımcılar) ve başlangıç/bitiş saati (`NightTime`; `TimePicker`, `parse_clock`/`stored_clock`/`shown_time`: dosyada hep 24 saat "SS:DD", geçersiz değerde shell varsayılanı) |
| `src/services/night_light.rs` | `msg night-light status` istemcisi (UI'dan bağımsız): `location`/`today` ayrıştırma, sayfa açılınca / odak dönünce / `[night_light]` yazılınca gecikmeli yeniden sorma |
| `src/services/timedate.rs` | systemd-timedated istemcisi (UI'dan bağımsız): `GetAll` ayrıştırma, `Set*` (interactive, 300 sn), hata sınıflama (`classify_name`), `ListTimezones` süzme, `PropertiesChanged` dinleyicisi, `NUXUT_TIMEDATE_BUS` bus seçimi, yerel ofset (`local_offset`, `chrono::Local`); testli |
| `src/services/accounts.rs` | AccountsService istemcisi (UI'dan bağımsız): `FindUserById` + `GetAll`, `SetRealName`/`SetIconFile` (etkileşimli yetki bayrağı, 300 sn), hata sınıflama, `Changed`/`PropertiesChanged` dinleyicisi, `NUXUT_ACCOUNTS_BUS`; profil resmi hattı (portal → yalıtılmış çözüm → `square_avatar` → işaretli PNG → `~/.face` yedek/atomik); testli |
| `src/services/network.rs` | `nuxut-nm` bağlayıcısı (UI'dan bağımsız): ajansız istemci, `sink` → anlık görüntü + `Event::Network`; bus seçimi yalnız `test-support` özelliğiyle (oturum adresi yoksa açılmaz, düşmez); eylemler ve durum makinesi (`Op`, `Target`, `RowError`, `apply` saf; crate `RequestId` ile eşleme, kimlik eyleme anlık görüntü kilidi altında bağlanır), şifre/WireGuard dosyası tek `Zeroizing` kopya → `Secret`, cihaz ayrıntısı istekleri, görünürken tek tarama; saf görünümler testli |
| `src/services/bluetooth.rs` | `nuxut-bluez` bağlayıcısı (UI'dan bağımsız): ajansız istemci, `sink` → anlık görüntü + `Event::Bluetooth`, eşleştirme durum makinesi (`pairing` / `pair_failed`; sonuç `RequestId` ile; kullanıcı iptali hata değil; `Pair` öncesi kabuğa `msg bluetooth agent-status`), görünürlük sayacı (açılış görülünce; saniyede bir olay yayan kısa ömürlü thread), tarama yalnız sayfa görünürken, cihaz türü ikonu (Icon → Class → Appearance), bus seçimi (yalnız `test-support`; oturum adresi yoksa servis açılmaz) |
| `src/services/power.rs` | Güç görünüm modeli (`from_live`: `nuxut-power` durumu → model), `Source` (None / Fake / Live), canlı olaylar (`apply_live`: istek kimliğiyle sonuç), yazma durumu (`begin`/`finish`, iyimser değil), görünürlük (`listed`, `empty`, `charge_shown`, `battery_shown`); sahte kaynak `NUXUT_POWER_FAKE` (pil dahil); testli |
| `src/pages/power.rs` | Güç blokları: `PowerStatus` (maskotlu boş durum), `BatteryRow` (pil durumu), `ChargeLimitRow` (anahtar), `PowerProfileRow` (bölümlü düğme + not); saf `battery_view`/`profile_view`/`charge_view`/`charge_text`/`spoken_tail`/`duration_text` testli |
| `src/numfmt.rs` | Ayarlar'daki sayı gösterimlerinin (kaydırıcı değeri, yenileme hızı, yay özeti) locale'i: `[locale] formats` → framework sayı çözümü (`LanguagePrefs::resolve(..).numeric()`, auto → `LC_ALL`/`LC_NUMERIC`/`LANG`); açılışta ve `locale_changed`'de kurulur, biçimleyici `Locale::format_number_with` (gruplamasız, sondaki sıfırlar atılır) (testli) |
| `src/services/` | calendar (D-Bus), portal (FileChooser), palette (üretim), hyprland (`input:resolve_binds_by_sym`, `hyprctl getoption` ile salt okuma), `open_url`/`open_dir`; `shell_ipc` (shell `msg` istemcisi, `ShellLink` trait — testte sahte; Ayarlar test örneği `NUXUT_SHELL_INSTANCE` verilmeden shell'e hiç istek göndermez), `look` (kurulu ikon/imleç teması tarayıcısı, `msg look status` v3–v5: katalog `CatalogFamily`, hedefli eksik paketler, kurulum komutu, dış dosyalar `ExternalFile`) |

**Kendi çizimlerde renk karışımı (yüksek karşıtlık, framework t124 `accessibility.md` §1 2026-10-13):** Ayarlar'ın elle çizdiği katman/soluk ton/halka, framework `nuxut::ui::style` yardımcılarıyla çizilir — standart kipte bayt bayt eski ifade, yüksek karşıtlıkta eşikte: seçim/vurgu/parlama katmanı `state_layer(_under)`, soluk metin `faded_text`, soluk işaret `faded_mark`, ince ayraç `hairline`, odak halkası `focus_ring`, tonal kap `tinted` + üstündeki vurgu `accent_on`. Satır altı notları framework `Notice` (sıkı boyut). Bilinçli olarak ham bırakılanlar: görünürlük/geçiş animasyonu opaklıkları (`ui.with_alpha`, kart/başlık belirme, başlık sönmesi), iki palet yüzeyinin opak karışımı (kenar çubuğu zemini; metin iki uçta da eşikte, karışım arada kalır), şema "duvar kağıdı" tonları (`edges::wallpaper_tones`, önizleme ızgarası — içerik değil resim), palet şeridinin yükleniyor solukluğu (renklerin kendisi veri), kullanıcı renginin kenar çizgisi (`rows.rs` renk örneği; ölçülecek zemin örneğin kendisi değil çevresi, `hairline` karşılamıyor), kurulu olmayan tema kartının yer tutucu dolgusu, `displays.rs` seçim geçişi (iki uç zaten yardımcı) |

## 9. Açık konular / diğer ekiplerden beklenenler

- kabuk (`nuxut-nm` 2026-10-14): **kapandı** (2026-10-14 uyarlaması) — şifre güncelleme (`set_wifi_password`), `NmError::Denied`, istek kimlikli `Done`/`Failed`, `WireGuardImport::ignored`, sahte NM sahneleri ve gerçek etkinleşme geldi.
- kabuk (`nuxut-bluez`): **kapandı** (2026-10-14) — `Done { action: "pair" }`, istek kimliği, sahte BlueZ'de RSSI / `Pair` → `Paired` / `RemoveDevice` siler / sahneler; kabuğun ajan durumu `msg bluetooth agent-status`.
- kabuk: video duvar kağıdı ve ses — planlanmadı; anahtar yazılmaz.
- framework: Ayarlar'ın kullandığı ama `IconName`'de olmayan ikonlar (Undo, Maximize/Restore,
  Animation, Panel, Font, Widgets, Server, Work, AccountCircle, FolderOpen, Grid, Window) istendi;
  o güne kadar en yakın ad kullanılıyor (ör. geri al = ArrowBack, Yazı sayfası = Edit).
- takvim: Türkçe/sınıflandırılmış ağ hata metinleri.
- Video duvar kağıdı önizlemesi: framework `nuxut-video` hazır olunca.
- framework (F5): kenar çekmecesi ikonu (`dock_to_right`) yok; Paneller şimdilik `Widgets`.

## 10. Doğrulama

```sh
CARGO_BUILD_JOBS=3 cargo test
CARGO_BUILD_JOBS=3 cargo clippy --all-targets -- -D warnings
```

Canlı testler yalıtılmış headless sway'de (framework `tools/test-compositor.sh`; `grim -o
HEADLESS-1`), kullanıcının ekranında değil. Test örneği `NUXUT_SETTINGS_INSTANCE=<ad>` ile ayrı
IPC adı (`settings-<ad>`) alır — kullanıcının açık Ayarlar'ına hiçbir komut gitmez; config
`XDG_CONFIG_HOME`/`XDG_STATE_HOME` geçici dizinde. Gerçek fare/klavye kullanılmaz: **uygulama içi senaryo**
`NUXUT_SETTINGS_DEBUG="wait=600;reveal=motion.spatial;click=640,500;hover=640,500;drag=x1,y1,x2,y2;type=metin;key=enter;wallpaper=/yol.png;avatar=/yol.png;page=panels;search=gölge;expand=colors.primary"`
olayları uygulamanın kendi `pointer`/`keyboard` yolundan üretir; bitince stdout'a
`SETTINGS_SCRIPT_DONE` yazar. Takvim akışı `dbus-run-session` içinde yalıtılmış
`nuxut-calendard --memory-secrets --no-browser` ile denenir.
