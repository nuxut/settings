# Yerleşik Nuxut duvar kağıtları — Ayarlar tarafı

Sözleşme: wallpaper-builtins-v1 (taslak, kararlar
2026-10-13; shell `docs/design/wallpapers.md`).

## Kararlar

| Karar | Kim, ne zaman |
|---|---|
| Kimlikler İngilizce: `pair-flight` (Eş uçuş, varsayılan), `wingtip` (Kanat ucu), `climb` (Tırmanış), `migration` (Göç), `aurora` (Aurora), `silhouette` (Büyük siluet) | 2026-10-13 |
| SVG'ler gömülü crate'te (`nuxut-wallpapers`); diskte `index.toml`/SVG yok, IPC yok | 2026-10-13 |
| Crate shell deposunda (`shell/crates/nuxut-wallpapers`); adlar crate'in kataloğunda (`i18n_source`); maskotlu 3 yerleşik eklendi (`cloud-rest`, `far-glide`, `chase`; maskot Fiyu); dither crate'te | EK KARARLAR (kullanıcı), 2026-10-13 |
| Aurora v1'de (banding için framework'te dither takip işi) | 2026-10-13 |
| Boş `[wallpaper] path` ve palet görseli yok → varsayılan kart seçili görünür, dosyaya yazılmaz | 2026-10-13 |
| Palet görsel çözülmeden tohumdan (`seed`) üretilir | 2026-10-13 |
| Sayfa sırası: hero (geçerli duvar kağıdı + Değiştir…) en üstte, ızgara altında; bölüm başlığı "Nuxut duvar kağıtları", kartta yalnızca açıklama (Hoş geldin'de bölüm başlığı yok, kart başlığı çizilir) | 2026-10-13 |
| Yerleşik her zaman cover: geçerli duvar kağıdı yerleşikken Yerleşim ve kenar zemini satırları gizli; monitör başına satırda o monitör yerleşik gösteriyorsa yerleşim seçicisi gizli | 2026-10-13 |
| Tohum paletin `[meta] mode`'undaki varyanttan (palet o kipte üretilir) | 2026-10-13 |

## Davranış

Kaynak: shell deposundaki `nuxut-wallpapers` crate'i (EK KARARLAR 2026-10-13; dizin, adres
ayrıştırma, tohumlar, adlar, render). Ayarlar yalnızca kendi kurallarını ekler.

- **Adres:** crate `parse_source`'un kabul ettiği biçim (`nuxut:<id>`, sabit varyantlı biçim
  crate'te). Ayarlar yalnızca varyantsız `nuxut:<id>` yazar; sabit varyantlı adresi okur ve
  gösterir (kart yine seçili). Geçersiz/bilinmeyen adres crate'in yedeği (varsayılan) + uyarı.
- **Etkin kip:** temanın çözdüğü kip (`Theme::mode`). Ayarlar teması bellekteki belgeden kurulur
  (`live_theme.rs`), bu yüzden kaydedilmemiş Koyu/Açık seçimi kartlara anında yansır.
- **Izgara** (`src/pages/wallpaper_grid.rs`): 9 kart (6 maskotsuz + 3 maskotlu: `cloud-rest`,
  `far-glide`, `chase`), sayfada 3×3 (dar sayfada 2 sütun; kart en az `space(44)`), Hoş geldin'de
  2 sütun. Kart 16:9 önizleme + altında ad (crate kataloğu, `Builtin::name`). Seçili kartta
  önizlemenin içinde 3 px vurgu çerçevesi ve sağ üstte onay rozeti; ad vurgu renginde. Klavye:
  oklar, Home/End, Enter/Boşluk. Yalnızca ekranda görünen kart çizim ister. Sayfada hero üstte,
  ızgara altında; bölüm başlığı "Nuxut duvar kağıtları", kartta yalnızca açıklama.
- **Önizleme:** gerçek SVG, kartın fiziksel boyutunda (`rect × ölçek`) tek arka plan thread'inde
  (`builtin-wallpapers`, crate `render_preview` bloklar); tüketici başına yalnızca son istek
  çizilir, sonuç `services::wake` ile. Doku hazır olana dek tema `surface_container` dolgusu
  (crate ortalama renk vermez). Boşta iş yok.
- **Seçim:** tek geri al kaydı (`Action::Group`): `shell.toml [wallpaper] path` ve
  `palette.toml [meta] image` = `nuxut:<id>`. Zaten seçili kart yazmaz. "Görsel seç" akışı
  hero'da aynen sürer.
- **Palet:** `services::palette::palette_source` → `PaletteSource::Seed(Builtin::seed(varyant))`;
  varyant paletin `[meta] mode`'undan (sabit varyantlı adreste adresinki). Görsel önbelleğine
  girmez.
- **Hero, monitör satırları, Görünüm önizlemesi:** `WallThumb`; `nuxut:` adresi dosya çözücüye
  (`Wallpaper`/`WallpaperDecoder`) gitmez, gömülü SVG süreç içinde çizilir. Yerleşik her zaman
  cover; yerleşik gösterilirken Yerleşim ve kenar zemini satırları ile monitör satırının yerleşim
  seçicisi gizli. Boş yolda hero alt satırı "Varsayılan Nuxut duvar kağıdı".

## Main'e taşıma (2026-10-14)

Eski dalının net sonucu (paylaşılan yalıtılmış `WallpaperDecoder`) ve
Fiyu sonrası main üzerine yeniden kuruldu (görev):

- Bağımlılık `nuxut-wallpapers = { path = "../shell/crates/nuxut-wallpapers" }`; `frame`
  özelliği kapalı, crate'in dosya önbelleği kullanılmaz. Yerleşik bellek içi çizilir
  (`render_preview`), dosya çözücüsüne hiç gitmez.
- `WallThumb` dosya kaynağını sürecin paylaşılan çözücüsüyle açar
  (`services::wallpaper::shared()`); dosya isteği `draw`'da hedef boyutla yapılır
  (`card_target` hero/önizleme, `row_target` monitör satırı) — paylaşılan çözücünün "fiziksel boy
  `draw`'da belli" kuralı. `set_source` yalnız kaynağı (dosya ya da yerleşik) tutar.
- en değerleri (`settings-wallpaper-builtin-label/-description/-keywords`,
  `settings-section-builtin-wallpapers`, `-builtin-caption/-default`) çeviri gözden geçirmesinden
  birebir; artık geçici değil.
- Crate adları cümle düzeninde ("Eş uçuş"); test buna göre.

## Bilinen sınırlar

- Palet şablon dışa aktarımlarına yerleşik adres gitmez; görsel boş verilir
  (`palette::template_image`; 2026-10-13). Crate'in PNG önbelleği değerlendirildi,
  kullanılmadı: LRU ile budanır (shell de yazar), şablondaki yol sonradan silinebilir. Kalıcı
  (budanmayan) bir dışa aktarım yolu gelince bağlanır — takip işi.
