# Ayarlar — UX yeniden düzeni (t094)

Durum: **onaylandı, uygulandı** (P0 + W1–W5, 2026-10-11/12; kararlar §13). Kullanıcı
öneriyi tasarım sayfasında onayladı; bu belge 2026-10-15'te main'e indi (önceden yalnızca
ayrı bir daldaydı). Uygulamadan sapmalar §13'te; güncel davranışın kaynağı
`docs/ARCHITECTURE.md` (Sayfalar tablosu). O daldaki PROTOTİP commit'i birleştirilmedi.
Kaynak: kullanıcı geri bildirimi (pano t094) — "çok çirkin, cognitive overload, kargaşa; ama
bazı şeyler çok güzel. Düzenleyebildiğimiz şeyleri AZALTMAYALIM."

Bu belge dört sorunun cevabıdır:

1. Ayarlar neden kalabalık görünüyor, oysa tek tek bileşenler güzel?
2. Nesneye göre gruplanmış bir bilgi mimarisi nasıl olur (Bar, Paneller, Pencereler…)?
3. Hiçbir ayar kaybolmadan ayrıntı nasıl geri çekilir?
4. İçerik genişliği, satır kalıbı, hero kullanımı için tek kural seti ne?

Ölçütler `nuxut::Theme` token'larıyla yazılmıştır (`space(n)` = n × 4 px mantıksal; renkler
yalnızca tema rolleri). Görseller: `<scratchpad>/test-ux/mockups/` (liste §12).

---

## 1. Denetim — bugün ne görüyoruz

İzole sway'de (1600×1000 ve 820×900; 1.0 ve 1.5 ölçek) tüm sayfalar, ayrıca Ekranlar
ve Görünüm (Uygulamalar, Hoş geldin) sayfalarının o günkü dalları açıldı.

### 1.1 Güzel olanlar (korunur)

- **Görünüm önizlemesi (hero):** canlı panel + bar + düğme; değişikliğin sonucunu anında
  gösteriyor. Kullanıcının "çok hoş" dediği şey bu: *görsel geri bildirim*.
- **Hareket ön ayar kartları**, **Yazı önizlemesi**, **ikon seti kartları**, **Ekranlar tuvali**:
  hepsi aynı ilke — ayarı sözle değil, sonuçla seçtiriyor.
- Tonal kartlar, yumuşak köşeler, anında uygulama + geri al şeridi, arama.

### 1.2 Kargaşanın kaynakları (önem sırasıyla)

| # | Sorun | Nerede | Etki |
|---|---|---|---|
| D1 | **Uç eylemleri: 6 satır × 6–9 hap = 45 düğme**, hepsi aynı ağırlıkta, ikisi aynı soruyu iki uç için soruyor. Dar pencerede/1.5×'te 3 satıra sarılıyor. | Bar › Uç eylemleri | "Cehennem gibi" — tek bakışta ne seçili okunmuyor. |
| D2 | **Nesneye göre değil, dosyaya göre gruplama.** Panelin aç/kapa + içerik + derinliği "Shell" sayfasında; paneli açan kısayol "Kısayollar"da; bar'dan açan tıklama "Bar › Uç eylemleri"nde. Widget'ın ayarı "Widget ayarları" kartında, widget'ın kendisi "Widget'lar" kartında, tıklaması "Uç eylemleri"nde. | Shell, Bar, Kısayollar | Bir nesneyi ayarlamak için 3 sayfa. |
| D3 | **"Shell" sayfası** teknik bir ad; içinde iki ilgisiz nesne (çerçeve + paneller). | Kenar çubuğu | Kullanıcı dili değil. |
| D4 | **Aynı ayar iki yerde:** `palette.source` ile `colors.source` aynı anahtar (Görünüm › Renkler ve Duvar kağıdı › Renk şeması); `palette.mode` ile `colors.mode` aynı soruyu soruyor. | Görünüm, Duvar kağıdı | "Hangisi gerçek?" |
| D5 | **Her şey aynı yükseklikte bağırıyor:** 11 renk rolü, 3 hareket rolünün yay/eğri ayrıntıları, 5 yazı rolünün ölçekleri ilk bakışta açık. Uzman ayarı ile günlük ayar aynı ağırlıkta. | Görünüm, Hareket, Yazı | Aşamalı açıklama yok. |
| D6 | **Bölüm başlıkları `primary` renkte** — sayfada 5–8 mavi başlık + mavi hap metinleri + mavi kaydırıcılar; vurgu rengi "seçili/etkileşimli" anlamını yitiriyor. | Tüm sayfalar | Görsel gürültü. |
| D7 | **İçerik sütunu ortalanmış, başlık sola yaslı.** Sayfa başlığı x = kenar çubuğu + 24, kart ise 760 px'lik ortalanmış sütunda; 1600 px pencerede başlık ile kart arasında 260 px kayma. Sayfa değişince sol kenar da oynamıyor ama başlıkla hizasız. | Tüm sayfalar | Dağınık, "boşlukta yüzen" his. |
| D8 | **Satırlar arası 1 px çizgi + kart + başlık** üçlüsü; DESIGN.md "önce boşlukla çöz" diyor. Widget listesi kuyuları, kısayol tuş alanları `outline` kenarlıklı. | Tüm sayfalar | Izgara hissi, tonal dil dışı. |
| D9 | **Teknik dil:** "Workspace", "strftime", "Material You", "Mekânsal", "Sönüm", "Bağlayıcı adı (ör. HDMI-A-1)", "HEADLESS-1 · Headless output 1", "Hyprland bind örneği", "Masaüstü ayar dosyasına bağlan". | Bar, Hareket, Duvar kağıdı, Ses, Pencereler | Kullanıcı odaklı dil kuralına aykırı. |
| D10 | **Dar pencere:** 820 px'te kenar çubuğu 272 px'te kalıyor (ARCHITECTURE'daki ikon şeridi yapılmadı), içerik 500 px'e düşüyor; uç eylem hapları 3 satıra sarılıyor; widget sütunları "Ça…", "Sis…" diye kırpılıyor. | Bar, Ekranlar | Dar pencerede kullanılmaz. |
| D11 | **Bar kenarı seçicisi** kartın %70'ini boş bırakıyor (küçük şema sağda, "Üst" metni solda). | Bar › Konum | Gereksiz yükseklik. |
| D12 | Kenar çubuğu öğeleri 60 px yüksek, grupsuz 12–13 öğe; sıra dosya sırasını izliyor (Bar ile Shell, Duvar kağıdı ile Görünüm ayrık). | Kenar çubuğu | Tarama zor. |
| D13 | Ekranlar'da ölçek "hap", döndürme "segmentli" — aynı tür seçim iki farklı bileşenle. | Ekranlar | Tutarsızlık. |
| D14 | Panel içeriği ve bar widget'ları shell'in kataloğu yoksa ham adla (`active_window`, `notifications`) ve hepsi aynı ikonla görünür. | Bar, Paneller | Shell yokken bile insan adı gerekir. |

### 1.3 Genişlik bilinçli mi?

Kısmen. `max_content_width = space(190)` (760 px) okunabilir satır için doğru *bir* fikir, ama
(a) tüm sayfalara aynı uygulanıyor — Ekranlar tuvali, Bar önizlemesi, üç sütunlu widget
listesi 760 px'e sıkışıyor; (b) sütun ortalandığı için sayfa başlığıyla hizasız; (c) dar
pencerede kenar çubuğu daralmadığı için sınır hiç devreye girmiyor. Karar §4'te.

---

## 2. Bilgi mimarisi

### 2.1 İlke: nesneye göre

Kullanıcı "bar'daki saate tıklayınca ne olsun" diye düşünür, "`bar.edge_actions.end`" diye
değil. **Her sayfa ekrandaki bir nesnedir; o nesnenin her ayarı o sayfadadır.** Bir ayar iki
nesneyi ilgilendiriyorsa tek yerde yaşar, diğerinde **aynı bileşenin ikinci görünümü**
(ayna satır, §3.4) ya da yönlendirme satırı olur — kopya kontrol değil.

### 2.2 Kenar çubuğu (öneri)

Üç grup, grup başlıkları küçük `Label` (`on_surface_variant`), öğe yüksekliği `space(10)`
(40 px; bugün 60). 13 sayfa; "Shell" yok.

```
Kişisel          ◐ Görünüm        (renkler, koyu/açık, köşeler, gölge, uygulamalar)
                 ▣ Duvar kağıdı   (görsel, yerleşim, monitör başına)
                 Tт Yazı
                 ≈ Hareket
Masaüstü         ▭ Bar            (önizleme → seçili widget → widget'lar → konum → çerçeve)
                 ▤ Paneller       (her kenar paneli tek kart)
                 ◻ Pencereler
                 ▢ Ekranlar
                 ⌘ Kısayollar
Sistem           ◔ Bildirimler
                 ♪ Ses
                 ◎ Hesaplar
                 ⓘ Hakkında
```

Gerekçeler:
- **Görünüm ↔ Duvar kağıdı** yan yana: renkler çoğunlukla duvar kağıdından gelir.
- **Bar ve Paneller** ayrı nesneler; **çerçeve** bar'la aynı fiziksel kenarın parçası
  (bar = kalın çerçeve kenarı; `frame.edges` şeması zaten bar kenarını içeriyor) → Bar
  sayfasının son kartı. ("Bar ve çerçeve" başlığı açık karar, §13.)
- **Ekranlar** Masaüstü'nde, Pencereler'in yanında.
- `nuxut-settings shell` slug'ı Bar'a gider (geriye uyum).

### 2.3 Sayfa iskeleti

```
┌ sayfa başlığı (Title) ─────────────────────── ↶ ⟲  ─ □ ✕ ┐
│ [hero — yalnızca görsel geri bildirimli sayfalarda]        │
│ Bölüm başlığı (Label, on_surface_variant)                  │
│ ┌ kart (surface_container, radius_medium) ───────────────┐ │
│ │ Satır başlığı                              [kontrol]   │ │
│ │ kısa açıklama (≤ 1 satır)                              │ │
│ │ Satır başlığı                              [kontrol]   │ │
│ │ Gelişmiş ayrıntılar                              ⌄     │ │ ← açılır satır (§3)
│ └────────────────────────────────────────────────────────┘ │
```

---

## 3. Aşamalı açıklama — hiçbir ayar kaybolmaz

### 3.1 Üç kademe

| Kademe | Ne | Nasıl görünür |
|---|---|---|
| **Temel** | Kullanıcıların çoğunun değiştirdiği | Hep açık |
| **Koşullu** | Başka bir seçime bağlı (gölge kapalıyken bulanıklık) | `visible_if` — bugünkü gibi yayla açılır |
| **Ayrıntı** | Uzman/ince ayar | Kartın sonunda **açılır satır** ("Renkleri tek tek ayarla ⌄"); açık/kapalı durumu oturumda hatırlanır |

Kurallar:
- Açılır satırın içindeki ayar **aranabilir**: arama sonucu açılır satırı açar, satıra kaydırır,
  parlatır. (Gizli ≠ kayıp.)
- Açılır satırın kapalıyken **özet** göstermesi: "11 renk · 2'si elle ayarlı". Elle ayarlı
  (varsayılandan farklı) değer varsa satırda nokta rozeti → kullanıcı orada bir şey
  değiştirdiğini unutmaz.
- Bir sayfada en çok **bir** açılır satır kartı; iç içe açılır yok.

### 3.2 Kayıt değişikliği (uygulama)

`SettingDesc`'e tek alan: `tier: Tier { Basic, Detail(&'static str /* açılır satır adı */) }`.
Kimlikler (`id`), anahtar yolları ve kontroller **değişmez** — yalnızca `page`, `section`,
`title`, `subtitle`, sıra ve `tier`. Bu, Hoş geldin adımlarının, `snapshots` eşlemesinin
(Görünüm sayfası), aramanın ve testlerin bozulmamasını garanti eder.

### 3.3 Sayfa durumu

Seçili widget (Bar), açılır satırlar gibi *dosyaya yazılmayan* UI durumu sayfaya ait bir
`PageUi` yapısında tutulur; görünürlük `visible_if: fn(&Store) -> bool` yanında
`ui_if: Option<fn(&PageUi) -> bool>` ile sorulur. (Prototip bunu statik değişkenle taklit
ediyor; gerçek kod öyle olmaz.)

### 3.4 Ayna satır

Bir ayar iki nesnenin parçasıysa (panel kısayolu: hem Kısayollar'ın tablosunda hem panelin
kartında) **aynı `SettingDesc`** ikinci sayfada `mirror_on: &[PageId]` ile tekrar çizilir. Tek
anahtar, tek geri al kaydı, tek arama sonucu (birincil yer). Çakışma uyarısı yalnızca birincil
tabloda hesaplanır, ayna onu gösterir. Prototip bunu ayrı kimlikle yapıyor (`panels.right.keybind`);
gerçekte kimlik eklenmez.

### 3.5 Eski → yeni yer eşleme (tüm kayıt kimlikleri)

"Tier" sütunu: T = temel, K = koşullu (bugünkü `visible_if` korunur), A = açılır satır (adı
parantezde). Kimliği yazılmayan ayar yoktur; main ile Görünüm ve Ekranlar dallarının kayıtları dahil.

| Kimlik(ler) | Bugün | Yeni yer | Tier |
|---|---|---|---|
| `preview` | Görünüm (hero) | Görünüm (hero) | T |
| `colors.mode` | Görünüm › Uygulamalar | **Görünüm › Renkler, ilk satır** "Koyu ya da açık" | T |
| `colors.source` | Görünüm › Renkler | Görünüm › Renkler "Renkler nereden gelsin" | T |
| `palette.source` | Duvar kağıdı › Renk şeması (aynı anahtar `colors.source`) | **kaldırılır değil, aynalanır:** Duvar kağıdı'nda yönlendirme satırı "Renkler bu görselden alınıyor › Görünüm" (kimlik aramada Görünüm'e gider) | T |
| `palette` (şema), `palette.contrast` | Duvar kağıdı › Renk şeması | Görünüm › Renkler (yalnızca kaynak = duvar kağıdı iken) | K |
| `palette.mode` | Duvar kağıdı › Renk şeması | `colors.mode` ile tek satır (Görünüm sayfası zaten ikisini tek geri al adımında yazıyor); kimlik `colors.mode` satırına arama eşanlamlısı olur | K |
| `colors.primary` | Görünüm › Renkler | Görünüm › Renkler "Vurgu rengi" | T |
| `colors.on_primary`, `colors.surface`, `colors.surface_container`, `colors.surface_bright`, `colors.on_surface`, `colors.on_surface_variant`, `colors.error`, `colors.on_error`, `colors.outline`, `colors.shadow` | Görünüm › Renkler | Görünüm › Renkler | A ("Renkleri tek tek ayarla") |
| `shape.radius_small`, `shape.radius_medium`, `shape.radius_large` | Görünüm › Köşeler | Görünüm › Köşeler ve şekil | T |
| `shape.outlines` | Görünüm › Köşeler | Görünüm › Köşeler ve şekil | T |
| `blob.smoothing`, `blob.deform` | Görünüm › Şekiller | Görünüm › Köşeler ve şekil | T |
| `shadow.enabled` | Görünüm › Gölge | Görünüm › Gölge | T |
| `shadow.blur`, `shadow.offset_y`, `shadow.opacity` | Görünüm › Gölge | Görünüm › Gölge | K (gölge açıkken) |
| `desktop.unified`, `desktop.status` | Görünüm › Uygulamalar | Görünüm › Uygulamalar | T / K |
| `desktop.icon_theme`, `desktop.cursor_theme`, `desktop.cursor_size` | Görünüm › Uygulamalar | Görünüm › Uygulamalar | T |
| `wallpaper`, `wallpaper.fit` | Duvar kağıdı | Duvar kağıdı (hero = görsel) | T |
| `wallpaper.outputs` | Duvar kağıdı › Monitör başına | Duvar kağıdı › Monitör başına (monitör adları Ekranlar'dan; elle bağlayıcı ekleme A "Bağlı olmayan monitör ekle") | T |
| `typography.preview` | Yazı (hero) | Yazı (hero) | T |
| `typography.size`, `typography.icon_set` | Yazı | Yazı | T |
| `typography.ui.family`, `typography.display.family`, `typography.document.family`, `typography.mono.family` | Yazı › rol başına kart | Yazı › **Yazı tipleri** (tek kart, 4 satır) | T |
| `typography.ui.scale`, `typography.display.scale`, `typography.document.scale`, `typography.mono.scale`, `typography.icon.family`, `typography.icon.scale` | Yazı › rol kartları | Yazı › Yazı tipleri | A ("Boyut oranları ve ikon yazı tipi") |
| `motion.preset` | Hareket › Ön ayar | Hareket (hero: karakter kartları) | T |
| `motion.enabled`, `motion.speed` | Hareket › Genel | Hareket › Genel | T |
| `motion.style` | Hareket › Genel | Hareket › Genel | A ("Hareketi tek tek ayarla") |
| `motion.spatial`, `motion.spatial_fast`, `motion.effects` (eğri editörleri) ve her birinin `.style`, `.response`, `.damping`, `.easing`, `.duration_ms` | Hareket › rol kartları | Hareket › aynı açılır satırın içinde, rol başına alt bölüm | A + K |
| `bar.edge` | Bar › Konum | Bar › Konum ve boyut (kompakt şema, §6.1) | T |
| `bar.size`, `bar.padding`, `bar.spacing` | Bar › Boyut | Bar › Konum ve boyut | T |
| `bar.sections` | Bar › Widget'lar | Bar › hero (önizleme = seçim) + Widget'lar kartı (sırala/ekle) | T |
| `bar.edge_actions.start.left_click`, `.start.right_click`, `.start.scroll` | Bar › Uç eylemleri | Bar › **Seçili widget** kartı (sol uçtaki widget seçiliyken) | K (seçime bağlı) |
| `bar.edge_actions.end.left_click`, `.end.right_click`, `.end.scroll` | Bar › Uç eylemleri | Bar › Seçili widget (sağ uçtaki widget seçiliyken) | K |
| `bar.edge_actions.scroll_step` | Bar › Uç eylemleri | Bar › Seçili widget, yalnızca o ucun tekerleği ses/mikrofon/parlaklıksa | K |
| `bar.widget.workspaces.count`, `.style`, `.scroll` | Bar › Widget ayarları | Bar › Seçili widget (Çalışma alanları seçiliyken) | K |
| `bar.widget.clock.format` | Bar › Widget ayarları | Bar › Seçili widget (Saat) | K |
| `bar.widget.tray.icon_size` | Bar › Widget ayarları | Bar › Seçili widget (Sistem tepsisi) | K |
| *(yeni görünürlük, kimlik yok)* shell şemasındaki diğer widget ayarları: `clock.vertical_format`, `active_window.max_chars/max_width`, `session.panel`, `notifications.panel/show_zero`, `panel-toggle.panel/icon` | **Ayarlar'da yok** (yalnızca shell.toml) | Bar › Seçili widget, `widgets::WidgetSpec.settings`'ten dinamik satırlar | K |
| `frame.enabled` | Shell › Çerçeve | Bar › Çerçeve | T |
| `frame.thickness`, `frame.edges` | Shell › Çerçeve | Bar › Çerçeve | K |
| `frame.inner_corners` (bağlantı) | Shell › Çerçeve | Bar › Çerçeve | T |
| `panels.<kenar>.enabled` (right, left, top, bottom) | Shell › Paneller | **Paneller › <Kenar> panel** kartının ilk satırı | T |
| `panels.<kenar>.size` | Shell › Paneller | Paneller › <Kenar> panel "Genişlik"/"Yükseklik" | K |
| `panels.<kenar>.widgets` | Shell › Paneller | Paneller › <Kenar> panel "İçerik" (kartın sonunda) | K |
| `keybinds.panel_right`, `keybinds.panel_left` | Kısayollar › Bar ve paneller | Kısayollar (birincil) + Paneller › Sağ/Sol panel **ayna** | T |
| `keybinds.panel_clock`, `keybinds.bar_toggle` | Kısayollar › Bar ve paneller | Kısayollar (birincil) + Bar › Konum ve boyut ayna ("Bar'ı gizle/göster" kısayolu) | T |
| `keybinds.volume_*`, `keybinds.mic_mute`, `keybinds.brightness_*`, `keybinds.media_*` | Kısayollar | Kısayollar | T |
| `compositor.sync_keybinds` | Kısayollar › Genel | Kısayollar › en üst | T |
| `displays` | Ekranlar | Ekranlar (hero = tuval) | T |
| `windows.gap_inner`, `windows.gap_outer`, `windows.border` | Pencereler › Aralıklar, Kenarlık | Pencereler › Yerleşim | K (`sync_windows`) |
| `windows.blur`, `windows.blur_size` | Pencereler › Bulanıklık | Pencereler › Görünüm | K |
| `windows.corners`, `windows.shadow`, `windows.motion` (bağlantılar) | Pencereler › Temayla ortak (3 satır) | Pencereler › Görünüm, **tek satır** "Köşe, gölge ve hareket" + iki hedef (Görünüm, Hareket); üç kimlik aramada aynı satırı bulur | T |
| `compositor.sync_look`, `compositor.sync_windows`, `compositor.sync_motion`, `compositor.ensure_source` | Pencereler › Gelişmiş | Pencereler › Gelişmiş | A ("Masaüstüyle ortaklık") |
| `notifications.enabled`, `notifications.dnd` | Bildirimler › Genel | Bildirimler › Genel | T |
| `notifications.popup_timeout_ms`, `notifications.max_popups`, `notifications.history_limit` | Bildirimler › Açılır / Geçmiş | Bildirimler › Davranış (tek kart) | T |
| `notifications.apps` | Bildirimler › Uygulamalar | Bildirimler › Uygulamalar | T |
| `audio.max_volume`, `audio.max_volume.warning` | Ses › Ses | Ses › Ses | T / K |
| `osd.timeout_ms` | Ses › Ekran göstergesi | Ses › Ses ve parlaklık göstergesi | T |
| `media_keys.link` | Ses › Medya tuşları | Ses › Medya tuşları | T |
| `media_keys` (bind örneği) | Ses › Medya tuşları | Ses › Medya tuşları | K (kısayollar kapalıyken) |
| `accounts` | Hesaplar | Hesaplar | T |
| `about` | Hakkında | Hakkında | T |

Test (uygulamada): `every_setting_has_a_home` — her kimliğin sayfası `PageId::ALL`'da, A
kademesindekilerin açılır satırı aynı sayfada var; arama A kademesini de buluyor.

---

## 4. Genişlik kararı

| Sayfa türü | Azami genişlik | Sayfalar |
|---|---|---|
| **Metin/satır** | `space(180)` = 720 px | Pencereler, Kısayollar, Bildirimler, Ses, Hesaplar, Hakkında, Hareket, Yazı |
| **Tuval/önizleme/liste** | `space(260)` = 1040 px | Görünüm, Duvar kağıdı, Bar, Paneller, Ekranlar |

> **Geçersiz (kullanıcı kararı, 2026-10-12, f591bc3):** sütun yatayda **ortalı**, her sayfada
> tek azami genişlik `space(260)` = 1040 px; 720/1040 ayrımı ve sola hizalama kalktı. Aşağıdaki
> gerekçe tarihçe olarak duruyor.

- **Sola hizalı, ortalanmış değil.** Sütunun sol kenarı sayfa başlığının sol kenarıdır
  (kenar çubuğu + `space(6)`). Gerekçe: başlık ile içerik aynı dikeyde; sayfa değişince sol
  kenar oynamaz (720 ↔ 1040 geçişinde yalnızca sağ kenar değişir); geniş pencerede sağda kalan
  boşluk "nefes", solda kalan boşluk "kopukluk" gibi okunur. (Prototip bunu uyguluyor.)
- **720 neden:** satır = başlık sütunu + kontrol. Kontrol ~`space(70)` (280 px), iç boşluklar
  `space(4)` × 2 → metin sütunu ~400 px ≈ 55–60 karakter Caption: tek satır açıklama için
  ideal, ikinci satıra nadiren taşar. 760'ın ortalanmış hâli de buna yakındı; sorun değer
  değil hizalamaydı.
- **1040 neden:** üç widget sütunu her biri ≥ 300 px (adlar kırpılmaz), Ekranlar tuvali iki
  monitörü okunur boyda gösterir, bar önizlemesi gerçek bar oranına yaklaşır.
- **Dar pencere:** pencere < `space(225)` (900 px) → kenar çubuğu **ikon şeridine** iner
  (`space(16)` = 64 px; ARCHITECTURE'da yazılı ama yapılmamış). İçerik < `space(140)` (560 px)
  → satır **yığılır**: kontrol başlığın altına, tam genişlikte. Bar önizlemesi sığmayınca
  widget hapları yalnızca ikon olur; widget sütunları alt alta. Yatay kaydırma asla.
- **1.5 ölçek:** tüm ölçüler mantıksal; 2400×1500 fiziksel = 1600×1000 mantıksal, düzen 1.0
  ile birebir (doğrulandı: `14-sonra-bar-1.5x.png`). 1.5'te 820 mantıksal genişlik = 1230
  fiziksel pencere — dar kuralı mantıksal genişliğe bakar.

---

## 5. Görsel kurallar

### 5.1 Satır kalıbı (tek)

| Parça | Token |
|---|---|
| Başlık | `TextRole::Body`, `on_surface`; kısa isim öbeği ("Kalınlık", "Sol tık") |
| Açıklama | `TextRole::Caption`, `on_surface_variant`; **≤ 1 satır**, yoksa yazma. Ne yaptığını değil, *sonucunu* söyler |
| Kontrol | sağda, dikey ortada; genişlik ~`space(70)` |
| İç boşluk | yatay `space(4)`, dikey `space(3)`; en az yükseklik `space(14)` |
| Ayırıcı | **yok** — satırlar boşlukla ayrılır (DESIGN "önce boşlukla çöz") |
| Varsayılana dön | kontrolün solunda küçük ↺, yalnızca değer varsayılandan farklıyken (bugünkü gibi) |

**Kontrol seçimi kuralı** (bugün dağınık):

| Seçenek | Bileşen |
|---|---|
| açık/kapalı | `Toggle` |
| sayı | `Slider` + değer |
| 2–4 kısa seçenek | `SegmentedButton` |
| 5+ seçenek ya da uzun etiket | **açılır seçim** (`Select`) — hap bulutu yok |
| görsel seçim (ön ayar, ikon seti, şema, tema) | kart ızgarası (önizlemeli) |
| çoklu seçim / liste | `ReorderList` / hap (yalnızca çoklu seçimde) |

Bu kural tek başına D1'in yarısını çözer: uç eylemleri 9 hap yerine tek satır açılır seçim.
Ekranlar'daki ölçek (4 seçenek) `SegmentedButton` olur (D13). İkon/imleç temaları (Görünüm sayfası,
kurulu tema sayısı 3–40) açılır seçim + aramalı menü; satır içinde seçilinin adı.

### 5.2 Kart ve bölüm ritmi

- Bölüm başlığı: `TextRole::Label`, **`on_surface_variant`** (bugün `primary`). Vurgu rengi
  yalnızca seçili/etkin/etkileşimli öğeye kalır.
- Başlık → kart: `space(2)`; kart → sonraki başlık: `space(6)`; kart iç dikey boşluğu
  `space(1)` + satır boşlukları.
- Kart: `surface_container`, `radius_medium`, kenarsız. Kart içinde ikinci kart yok; kuyu
  (widget listesi) `surface` tonunda, **kenarsız** (bugün `outline` çizgili).
- Bir kart ≈ bir nesne ya da bir soru. Tek satırlık kartlar komşu kartla birleşir
  (Pencereler: 5 kart → 3).

### 5.3 Tipografi ölçeği

Sayfa başlığı `Title`; hero içi etiketler `Label`; bölüm başlığı `Label`; satır `Body`;
açıklama `Caption`; değer (kaydırıcı sağı) `Label` tabular. Yeni boyut yok — framework
rollerinin dışına çıkılmaz.

### 5.4 Hero — yalnızca görsel geri bildirimi olan sayfada

Hero = değiştirdiğin şeyin canlı resmi. Kural: **sayfanın ana nesnesi görülebiliyorsa hero
vardır, yoksa yoktur.**

| Sayfa | Hero | Not |
|---|---|---|
| Görünüm | canlı önizleme (var) | korunur |
| Duvar kağıdı | büyük görsel önizlemesi (bugün 160 px küçük resim) | monitör başına seçimde hangi monitörün gösterildiği |
| Ekranlar | monitör tuvali (var) | korunur |
| Bar | **bar önizlemesi — tıklanabilir** (yeni; seçim = ayar) | prototipte |
| Yazı | tipografi önizlemesi (var) | korunur |
| Hareket | ön ayar kartları (var, hero görevi görür) | korunur |
| Paneller | *isteğe bağlı, ikinci aşama:* ekran şeması, dört çekmece; tıklayınca o kartın üstüne kaydırır | ilk aşamada yok |
| Pencereler, Kısayollar, Bildirimler, Ses, Hesaplar, Hakkında | yok | metin sayfaları |

### 5.5 Değişmeyen kurallar

Hardcode renk yok (yalnızca tema rolleri, opaklıklar DESIGN state layer'ı); tonal, kenarsız;
hareket sakin (açılır satır `spatial_fast`, görünürlük yayı bugünkü gibi; giriş animasyonu
her kartta değil, sayfa geçişinde tek). İmleç kuralı DESIGN'daki gibi (bar önizlemesi hapları
`Pointer`).

---

## 6. Uç eylemleri

### 6.1 Öneri (A): bar önizlemesinden seç → "Seçili widget" kartı

```
┌ Bar önizlemesi (hero) ───────────────────────────────────────────────────────┐
│ ▦ Çalışma alanları  ▭ Etkin pencere      ◷ Saat     ⋯ Tepsi  ◔ Bildirimler [⏻ Oturum] │
│ ░░░░░░░░░░░░░░░░░░░░░ masaüstü ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
│ Ayarlamak için bir widget'a tıklayın                                         │
└──────────────────────────────────────────────────────────────────────────────┘
Seçili widget
┌──────────────────────────────────────────────────────────────────────────────┐
│ [⏻] Oturum                                                                    │
│     Bar'ın sağ ucunda: tıklama ve kaydırmayı siz belirlersiniz               │
│ Sol tık                                              ↺ [ Sağ panel      ⌄ ]  │
│ Sağ tık                                                [ Widget'ın kendi ⌄ ]  │
│ Tekerlek                                             ↺ [ Ses            ⌄ ]  │
│ Tekerlek adımı                                 ━━━●━━━━━━━━━━━━━━━━   %5    │  ← yalnızca ses/mik/parlaklık
└──────────────────────────────────────────────────────────────────────────────┘
```

- Varsayılan seçim: sağ uçtaki widget (en çok özelleştirilen; kullanıcının örneği).
- Uçtaki widget seçiliyken kart **önce widget'ın kendi ayarlarını, sonra tıklama/tekerleği**
  gösterir (`12-sonra-bar-sol-uc-calisma-alanlari.png`). Ortadaki widget'ta yalnızca kendi
  ayarları (`13-sonra-bar-orta-saat.png`). Ayarı olmayan, uçta olmayan widget: "Bu widget'ın
  ayarı yok" tek satırı.
- Açılır seçim seçenekleri yalnızca o anahtarın şemadaki geçerli değerleri (tıkta `brightness`
  yok, tekerlekte `panel:*` yok). Kayıtta eksik `panel:launcher` ("Uygulama başlatıcı") eklenir.
- Etiketler: "Widget'ın kendi" (varsayılan), "Hiçbir şey", "Sağ paneli aç", "Takvimi aç",
  "Sesi kapat/aç"…; tekerlekte "Ses", "Mikrofon", "Parlaklık", "Çalışma alanı".
- Hero'da uçtaki widget haplarının köşesinde küçük nokta: o uca özel eylem atanmış (varsayılan
  değilse) — kullanıcı nereye bakacağını bilir.
- Arama "sol tık" → Bar sayfası, ilgili uç widget'ı seçili, satır parlar.
- Dikey bar'da önizleme dikey çizilir; "sol uç / sağ uç" metinleri "üst uç / alt uç" olur.

Neden: kullanıcı nesneyi (saati) görüp ona tıklar; 45 düğme yerine 3 açılır seçim; uç kavramı
ayrıca öğretilmez (yalnızca uçtaki widget'ta seçenek belirir); widget ayarları ve tıklaması
aynı kartta (D2). Ölçü: ekrandaki etkileşimli öğe 45 → 3–4.

### 6.2 Değerlendirilen alternatifler

| | Fikir | Neden seçilmedi |
|---|---|---|
| B | 2 × 3 tablo (satır: sol/sağ tık/tekerlek; sütun: sol uç/sağ uç), hücrede açılır seçim | Kompakt ama "uç" soyutlamasını korur; hangi widget'ın uçta olduğunu kullanıcı kafasında eşler. Önizleme yoksa (A'nın dar pencere yedeği) iyi bir düşüş. |
| C | Bugünkü haplar, uç başına açılır bölüm | Hap sayısı aynı; yalnızca gizler. |
| D | Önizlemedeki hapa tıklayınca açılan popover | Güzel ama widget'ın diğer ayarları ve sıralama ile ilişkisi kopuk; popover içinde kaydırıcı/metin alanı dar. |

Dar pencere (< 560 px içerik): A aynen çalışır (hero ikonlara iner, kart satırları yığılır).

---

## 7. Paneller

Her kenar paneli **tek kart**: kullan (toggle) → kısayol (ayna) → genişlik/yükseklik → içerik
listesi (en uzun öğe en sonda). Kapalı panel tek satıra iner. Kart sırası: Sağ, Sol, Üst, Alt.
Kart altında "Bar'dan açma: sağ uçtaki widget'a sol tık · Ayarla ›" bilgisi (uç eylemi bu
paneli açıyorsa; tıklayınca Bar sayfasında o widget seçili açılır). Takvim popout'u (`panel:clock`)
panel değil: kısayolu Kısayollar'da, açılışı saatin tıklamasında kalır.

İkinci aşama (isteğe bağlı): ekran şeması hero'su, kart başlığında panel ikonu, içerik listesi
geniş sayfada iki sütun (8 kart = 4 satır).

---

## 8. Türkçe metin sadeleştirme

| Bugün | Öneri |
|---|---|
| Shell (sayfa) | — (Bar + Paneller) |
| Uç eylemleri | Tıklama ve kaydırma (Seçili widget kartında) |
| Baştaki widget · sol tık / Sondaki widget · kaydırma | Sol tık / Tekerlek |
| Yatay bar'da en soldaki, dikeyde en üstteki widget | (gerek kalmaz; önizleme gösterir) |
| Hiçbiri | Hiçbir şey |
| Sol panel / Takvim (eylem) | Sol paneli aç / Takvimi aç |
| Kaydırma adımı — Bir tekerlek çentiğinin değiştirdiği yüzde | Tekerlek adımı — Bir çentikte değişen yüzde |
| Workspace, Sayfa başına workspace | Çalışma alanı, Görünen çalışma alanı |
| Workspace görünümü / Workspace kaydırma | Görünüm / Tekerlekle geçiş |
| Saat biçimi — strftime: %H:%M, %a %d %b %H:%M… | Biçim — %H:%M → 14:32 · %a %d %b → Cmt 11 Eki |
| Tepsi ikon boyutu | İkon boyutu (Sistem tepsisi kartında) |
| Bar kalınlığı — Bar'ın yüksekliği (dikeyde genişliği); bar kenarındaki çerçeve de budur | Kalınlık — Bar'ın kenarındaki çerçeve de bu kalınlıktadır |
| Uç boşluğu — Bar'ın iki ucundaki iç boşluk | Uçlardaki boşluk |
| Ekran kenarı — Ekran şemasında bir kenara tıklayın + "Üst" | Ekran kenarı + kompakt dört düğmeli şema (metin tekrarı yok) |
| Sağ panel / Sağ panel derinliği — Panelin genişliği (üst/alt panelde yüksekliği) | Sağ paneli kullan / Genişlik (üst/altta Yükseklik) |
| Sağ panel içeriği — Kart ekle, çıkar ve sürükleyerek sırala (yukarıdan aşağı) | İçerik |
| Çerçeve kalınlığı — Bar dışındaki kenarlar (elle ayarlananlar hariç) | Kalınlık — Bar dışındaki kenarlar |
| İç köşeler — Kavsi temadan gelir: Görünüm › Şekiller › Birleşim yumuşaklığı | İç köşeler — Kavis temadan gelir · Ayarla › |
| Renk kaynağı — Renkler Nuxut temasından mı, duvar kağıdından mı gelsin | Renkler nereden gelsin — [Nuxut teması · Duvar kağıdından] (2026-10-16: "Sabit" yerine; Nuxut temasındayken altında Renk teması kartları) |
| Renkleri duvar kağıdından al — Material You: tüm arayüz duvar kağıdına uyar | (Görünüm'e taşınır) Tüm renkler duvar kağıdına uyar |
| Kip — Koyu ya da açık arayüz | (colors.mode ile birleşir) Koyu ya da açık |
| Kontrast — Düşük (-1) … standart (0) … yüksek (1) | Kontrast — kaydırıcı uçlarında "Düşük / Yüksek" |
| Vurgu üstü metin / Parlak yüzey — Üstüne gelme ve basılı katman tabanı | Vurgu üzerindeki yazı / Vurgulu yüzey — Fareyle üzerine gelince |
| Birleşim yumuşaklığı | Birleşim kavisi |
| Mekânsal / Hızlı mekânsal / Efektler | Paneller ve pencereler / Menüler ve küçük öğeler / Solma ve renk |
| Süre — Yayın periyodu; küçük = hızlı | Süre — Kısa = hızlı |
| Sönüm — 1'in altı hedefi aşıp geri oturur (puding); 1 taşmaz | Yaylanma — Düşük değer hedefi aşıp geri oturur |
| Stil — Yay kesilebilir ve hızını korur; eğri sabit sürelidir | Hareket türü — Yay (doğal, kesilebilir) · Eğri (sabit süreli) |
| Ön ayar — Tüm hareketlerin karakteri; aşağıdaki roller tek tek ezebilir | Karakter |
| Boyut ölçeği — Bu roldeki tüm yazıların çarpanı | Boyut oranı |
| İkon fontu — Boş: ikon setinin fontu | İkon yazı tipi — Boşsa setin kendi yazı tipi |
| Bağlayıcı adı (ör. HDMI-A-1) + Monitör ekle | Bağlı olmayan monitör ekle (açılır satır) |
| HEADLESS-1 · Headless output 1 · Genel duvar kağıdı | Dell U2720Q · Genel duvar kağıdı (Ekranlar'daki ad) |
| Hyprland bind örneği — … hyprland.conf'a kendiniz ekleyin | Kendi ayar dosyanız için örnek |
| Masaüstü ayar dosyasına bağlan — Açılışta Hyprland ayar dosyanıza… | Masaüstü ayarlarına Nuxut'u ekle — Ayar dosyanıza tek satır eklenir, önce yedek alınır |
| Pencereler temaya uysun — Köşe, gölge ve bulanıklık Nuxut temasından gelir; kapalıyken masaüstünüzün kendi ayarı geçerli | Pencereler temaya uysun — Köşe, gölge ve bulanıklık Nuxut'tan gelir |
| Aralık ve kenarlığı Nuxut belirlesin | Boşluk ve kenarlığı Nuxut belirlesin |
| Pencereler arası / Ekran kenarına / Kenarlık kalınlığı | Pencereler arası boşluk / Ekran kenarına boşluk / Kenarlık |
| Arka plan bulanıklığı — Saydam pencerelerin ardı buzlu cam gibi görünür | Saydam pencerelerin ardı bulanık — Buzlu cam görünümü |
| Bildirimler — Kapalıyken shell bildirim sunucusu olmaz (başka bir sunucu çalışabilir) | Bildirimleri Nuxut göstersin — Kapalıyken başka bir uygulama gösterebilir |
| Ekran göstergesi — Ses ya da parlaklık değişince beliren göstergenin ekranda kalma süresi | Ses ve parlaklık göstergesi — Ekranda kalma süresi |

Sözcük kuralları: "widget" kalır (yerleşik); "workspace" → "çalışma alanı" (shell kataloğu
da böyle diyor); hitap tutarlı **"siz"** (bugün Hoş geldin "sen", sayfalar "siz" karışık —
açık karar §13); "panel" = kenar çekmecesi, "kart" = panel içeriği; teknik ad (Hyprland,
strftime, Material You, matugen) yalnızca arama anahtar kelimesinde.

---

## 9. Sayfa sayfa ÖNCE / SONRA

| Sayfa | Önce | Sonra |
|---|---|---|
| **Görünüm** | hero; Renkler (kaynak + 11 rol açık); Köşeler (4); Gölge (4); Şekiller (2); Uygulamalar (6) | hero; **Renkler**: koyu/açık, renkler nereden gelsin, vurgu, (duvar kağıdından ise şema şeridi + kontrast), "Renkleri tek tek ayarla ⌄" (10 rol); **Köşeler ve şekil** (6); **Gölge** (1 + koşullu 3); **Uygulamalar** |
| **Duvar kağıdı** | küçük resim + seç; yerleşim; monitör başına (bağlayıcı adı alanı); Renk şeması (kaynak tekrarı, şema, kip, kontrast) | hero = büyük görsel + "Değiştir"; yerleşim; monitör başına (Ekranlar adlarıyla); "Renkler bu görselden alınıyor · Görünüm ›" yönlendirme |
| **Yazı** | önizleme; boyut; ikon kartları; 5 rol kartı × (aile + ölçek) | önizleme; boyut; ikon seti kartları; **Yazı tipleri** tek kart (4 aile); "Boyut oranları ve ikon yazı tipi ⌄" |
| **Hareket** | ön ayar kartları; genel (3); 3 rol kartı × (editör + stil + 2–3 kaydırıcı) | karakter kartları (hero); hareketi azalt, hız; "Hareketi tek tek ayarla ⌄" (genel stil + 3 rol, yeni adlarla) |
| **Bar** | Konum (büyük boş kart); Boyut (3); Widget'lar (3 sütun, ham ad); Uç eylemleri (6 × hap); Widget ayarları (5, tümü her zaman) | **hero bar önizlemesi** (tıkla = seç); **Seçili widget** (widget ayarları + uçtaysa tıklama/tekerlek); Widget'lar (sırala/ekle; kenarsız kuyular); Konum ve boyut (kompakt kenar şeması + 3); **Çerçeve** (Shell'den) |
| **Paneller** *(yeni)* | — (Shell › Paneller: 4 panel × 3 satır, düz liste; kısayol Kısayollar'da) | kenar başına kart: kullan → kısayol (ayna) → genişlik → içerik; kapalı panel tek satır |
| **Shell** | Çerçeve + Paneller | **kaldırılır** (slug Bar'a gider) |
| **Pencereler** | Aralıklar, Kenarlık, Bulanıklık, Temayla ortak (3 bağlantı), Gelişmiş (4, açık) — 5 kart | **Yerleşim** (3); **Görünüm** (bulanıklık 2 + tek bağlantı satırı); **Gelişmiş** "Masaüstüyle ortaklık ⌄" (4) — 3 kart |
| **Ekranlar** | tuval; monitör hapları; 9 satır; ölçek hap, döndürme segmentli | aynı; ölçek segmentli; geniş sayfa (1040); dar pencerede satırlar yığılır; Masaüstü grubunda kendi ikonu |
| **Kısayollar** | Genel; Ses; Parlaklık; Medya; Bar ve paneller — tuş alanları kenarlıklı | aynı gruplar, tonal tuş alanları; panel/bar satırlarında "Paneller'de de görünür" ipucu yok — ayna sessizdir |
| **Bildirimler** | Genel (2); Açılır (2); Geçmiş (1); Uygulamalar | Genel (2); **Davranış** (3); Uygulamalar |
| **Ses** | Ses (1); Ekran göstergesi (1); Medya tuşları (1) — 3 kart | **Ses** (en yüksek ses + uyarı + gösterge süresi); Medya tuşları — 2 kart |
| **Hesaplar** | (değişmez) | yalnızca §5 kuralları (başlık rengi, ayırıcı) |
| **Hakkında** | (değişmez) | aynı |
| **Hoş geldin** | sol yaslı başlık, altta ortada tek düğme; geniş pencerede boşlukta | adım içeriği 720 sütununda, düğmeler sütunun sağ altında; renk adımında Görünüm hero'sunun küçük hâli (görsel geri bildirim) — ikinci aşama |

---

## 10. Framework talepleri (framework'e)

| # | İstek | Neden | Geçici çözüm |
|---|---|---|---|
| F1 | `List`: **grup başlıkları** + **yoğun satır** (`space(10)`) + **ikon şeridi** kipi (yalnızca ikon, ipucunda ad) | Kenar çubuğu grupları (§2.2), dar pencere (§4) | Yok — grup başlıkları beklenir; yoğun satır olmadan da sıralama uygulanabilir |
| F2 | `Select` menüsü için **sayfa/yüzey üstü katman** (bloğun kırpmasının dışında çizim; popover gibi) | Standart satırdaki açılır seçimin menüsü satır kırpmasında kalıyor; Ekranlar sayfası bunu tüm bloğu sınır yaparak çözüyor | Satır bloğunun sınırını genişletmek (kırpılabilir) |
| F3 | **Disclosure / Expander** bileşeni: başlık + açıklama + özet + chevron, içerik yayla açılır, odak/klavye | Aşamalı açıklama tüm uygulamalarda (Takvim ayarları da) | Ayarlar'da yerel blok |
| F4 | `ReorderList` **yatay/hap** kipi | Widget sütunları dar alanda; bar'ın yatay sırasını aynalamak | Dikey liste |
| F5 | `IconName`: kenar çekmecesi/paneller (`dock_to_right`), ekran çerçevesi, monitörler (ayrı bir "Ekranlar" ikonu) | Bar, Paneller, Ekranlar aynı/benzer ikonu kullanıyor | En yakın ad |
| F6 | `Select` seçeneğinde **ikon + ikincil metin** | "Sağ paneli aç" yanında panel ikonu; tema listelerinde "kurulu değil" | Düz metin |

Ayarlar'a ait (framework değil): tonal kısayol alanı, satırın yığılan dar düzeni, bar önizlemesi
bloğu, ayna satır, `Tier`.

---

## 11. Uygulama planı

Kimlikler ve anahtarlar değişmediği için config/geri al/arama davranışı aynı kalır; iş
çoğunlukla `registry.rs` sırası/başlıkları + birkaç yeni blok.

### 11.1 İniş sırası

1. **Görünüm ve Ekranlar sayfası değişiklikleri önce iner** (tamamlanmışlar). Bu tasarım onları bozmaz.
2. **P0 — temel (tek işçi, diğerleri bunu bekler; ~1 gün):**
   `src/page.rs` (sola hizalı genişlik, sayfa türü, başlık rengi, ayırıcı yok, `PageUi`),
   `src/rows.rs` (`ChoiceStyle::Dropdown`, dar pencerede yığılan satır),
   `src/pages/disclosure.rs` (açılır satır bloğu; F3 gelince framework'e geçer),
   `src/registry.rs` **yalnızca yapı**: `PageId::Panels`, `PageId::ALL` yeni sıra, grup
   bilgisi, `Tier`, `mirror_on`, `ui_if`; `shell` slug'ı → Bar; test `every_setting_has_a_home`.
   `src/app.rs` kenar çubuğu sırası (grup başlıkları F1'i bekler).
3. **Paralel parçalar** (P0 indikten sonra; her biri `registry.rs`'te **yalnızca kendi
   bloğunu** düzenler — bloklar `// ---- <Sayfa> ----` sınırlarıyla ayrık, çakışma satır
   düzeyinde yok):

| Parça | Sahip dosyalar | İş |
|---|---|---|
| **W1 Bar** | `src/pages/bar_preview.rs` (yeni), `src/pages/sections.rs`, `src/pages/edges.rs`, registry Bar bloğu | hero + seçim, Seçili widget kartı (statik + `WidgetSpec.settings`'ten dinamik satırlar), kompakt kenar şeması, kenarsız kuyular, çerçeveyi Bar'a taşı, `panel:launcher` |
| **W2 Paneller** | registry Paneller bloğu, `src/pages/keybinds.rs` (ayna satır, tonal alan), `src/pages/panel_hint.rs` (yeni, "Bar'dan açma" satırı) | panel kartları, ayna kısayol |
| **W3 Görünüm + Duvar kağıdı** | `src/pages/preview.rs`, `src/pages/wallpaper.rs`, `src/pages/wallpaper_outputs.rs`, `src/pages/look.rs`, registry Görünüm/Duvar kağıdı blokları | Renkler kartı (mode + kaynak + vurgu + şema/kontrast), renk rolleri açılır satırı, duvar kağıdı hero'su, monitör adları (Ekranlar servisinden) |
| **W4 Metin sayfaları** | `src/pages/motion.rs`, `src/pages/motion_preset.rs`, `src/pages/fonts.rs`, registry Hareket/Yazı/Pencereler/Bildirimler/Ses blokları | açılır satırlar, yeni metinler (§8), kart birleştirmeleri |
| **W5 Ekranlar uyumu** | `src/pages/displays.rs` | ölçek → segmentli, geniş sayfa, dar pencere yığılması, ikon |

4. **Son: görsel doğrulama** — izole sway'de her sayfa 1.0/1.5 × geniş/dar; önce/sonra
   karşılaştırma; `every_setting_has_a_home`.

### 11.2 Ekranlar ve Görünüm sayfalarının uyumu

- **Ekranlar:** sayfa olduğu gibi kalır (hero = tuval, bu tasarımın örneği). Değişiklik:
  `PageId::ALL`'da Masaüstü grubuna (Pencereler'den sonra), ölçek `SegmentedButton`,
  `page_width` geniş türü, satır içi `Select` menüsü F2 gelince ortak katmana.
  `services/displays` monitör adları Duvar kağıdı › Monitör başına'da yeniden kullanılır.
- **Görünüm:** `colors.mode` satırı Uygulamalar'dan Renkler'in başına taşınır (kimlik aynı,
  Hoş geldin adımı etkilenmez — `build_ids` kimlikle çalışıyor). İkon/imleç teması hapları
  `Select`'e (çok sayıda kurulu tema) — W3. `rows::PreviousAction` (⟲) satır kalıbında
  ↺'nın yanında kalır. Hoş geldin'in hitabı (sen/siz) açık karar.

### 11.3 Prototip

Tasarım dalında "PROTOTİP" commit'i: `src/pages/proto.rs` (bar önizlemesi, seçili widget
başlığı, açılır satır — statik durumla), registry Bar/Paneller/Pencereler blokları, `rows.rs`
`Dropdown`, `page.rs` genişlik/hizalama/başlık rengi/ayırıcı. **Birleştirilmez**; uygulama
P0'dan temiz başlar, prototipten yalnızca fikir ve ölçüler alınır.

---

## 12. Görseller

Görseller geçici bir çalışma dizinindeydi (repoya girmedi)
(izole sway, yalnızca Ayarlar penceresi; sahte shell şeması ile insan adları):

| Dosya | İçerik |
|---|---|
| `01-once-bar-ust.png` | Bar bugün: konum, boyut, widget'lar |
| `02-once-bar-uc-eylemleri.png` | Uç eylemleri: 6 × hap |
| `03-once-bar-uc-eylemleri-dar-1.5x.png` | aynı, dar pencere 1.5× — 3 satıra sarılıyor |
| `04-once-shell-paneller.png` | Shell › Paneller |
| `05-once-pencereler.png` | Pencereler bugün |
| `06-once-gorunum-hero.png` | Görünüm hero (korunacak örnek) |
| `07-once-ekranlar.png` | Ekranlar |
| `08-once-gorunum-uygulamalar.png` | Görünüm › Uygulamalar |
| `11-sonra-bar-sag-uc-secili.png` | Bar: hero + seçili "Oturum" (sağ uç): 3 açılır seçim + tekerlek adımı |
| `12-sonra-bar-sol-uc-calisma-alanlari.png` | sol uçtaki Çalışma alanları: kendi ayarları + tıklama/tekerlek |
| `13-sonra-bar-orta-saat.png` | ortadaki Saat: yalnızca biçim |
| `14-sonra-bar-1.5x.png` | Bar sonrası 1.5 ölçek |
| `15-sonra-bar-dar-pencere-sorunlu.png` | dar pencere: prototipin çözmediği yer (hero hapları üst üste, sütunlar kırpılıyor) → §4 kuralları |
| `16-sonra-paneller.png` | Paneller: sağ panel kartı (kullan, kısayol, genişlik, içerik) |
| `17-sonra-pencereler.png` | Pencereler: 3 kart, Gelişmiş kapalı |
| `18-sonra-pencereler-gelismis-acik.png` | Gelişmiş açık |

Prototipte olmayanlar (yalnızca belgede): kenar çubuğu grupları/ikon şeridi (F1), Görünüm
renk açılır satırı, Duvar kağıdı hero'su, dar pencere yığılması, hero'daki eylem noktası.

---

## 13. Kararlar (önceden açık kararlar)

1. **Çerçeve nerede?** Öneri: Bar sayfasının son kartı. Alternatif: sayfa adı "Bar ve çerçeve".
2. **Hitap:** "siz" (sayfalar) mi "sen" (Hoş geldin) mi — tek seçilmeli. Öneri: "siz" yok,
   **eylem kipi** ("Sağ paneli kullan", "Masaüstü ayarlarına Nuxut'u ekle") — hitapsız.
3. **Sola hizalı sütun** (öneri) mı, ortalanmış (bugün) mı.
4. **Varsayılan seçili widget** Bar'da: sağ uç (öneri) mı, hiçbiri (kart "bir widget seçin") mı.
5. **Duvar kağıdı renk şeması** Görünüm'e taşınsın mı (öneri: evet, tek yer) yoksa Duvar
   kağıdı'nda kalıp Görünüm'de yönlendirme mi.
6. **Hareket rol adları** (§8): "Paneller ve pencereler / Menüler ve küçük öğeler / Solma ve
   renk" uygun mu.
7. **Paneller hero'su** ilk aşamada mı, ikinci mi (öneri: ikinci).
8. **İkon/imleç teması**: açılır seçim (öneri) mi, bugünkü hap bulutu mu.
9. **Dil ve biçimler** bugün Hakkında sayfasında bir kart (ADR-0010 adım 3, 5adadda). Ayrı bir
   "Bölge ve dil" sayfası (Sistem grubunda) mı açılsın, Hakkında'da mı kalsın? Öneri: ayrı sayfa,
   ileride klavye düzeni ve saat dilimi de oraya gelir.

### 13.1 Verilen kararlar

Öneriler tasarım sayfasında ("Nuxut Ayarlar Yeniden Tasarımı") kullanıcıya sunuldu ve
onaylandı (2026-10-11); uygulama commit'leri karar numarasıyla anar.

| # | Karar | Kim / ne zaman | Uygulama |
|---|---|---|---|
| 1 | Çerçeve Bar sayfasının son kartı ("Bar ve çerçeve" adı yok) | kullanıcı, 2026-10-11 | 987bbe9, ad4ba46 |
| 2 | Hitap yok: eylem kipi etiketler ("Sağ paneli kullan") | kullanıcı, 2026-10-11 | 03f043a, 725b93c (test: metin uzunluğu/hitap) |
| 3 | ~~Sola hizalı~~ → **ortalı, tek azami genişlik 1040** | kullanıcı, 2026-10-12 (öneri geri alındı) | f591bc3 |
| 4 | Bar açılınca sağ uçtaki widget seçili | kullanıcı, 2026-10-11 | ad4ba46 |
| 5 | Duvar kağıdı renk şeması Görünüm › Renkler'de; Duvar kağıdı'nda yönlendirme satırı | kullanıcı, 2026-10-11 | 3cc576b |
| 6 | Hareket rol adları: Paneller ve pencereler / Menüler ve küçük öğeler / Solma ve renk | kullanıcı, 2026-10-11 | 03f043a |
| 7 | Paneller hero'su ikinci aşamada (şimdilik yok) | kullanıcı, 2026-10-11 | 03f043a |
| 8 | İkon/imleç teması: ne açılır seçim ne hap — önizlemeli **kart ızgarası** (t103) | t103 (2026-10-11/12) | 3cc576b, 2870d28 |
| 9 | Ayrı "Bölge ve dil" sayfası (Sistem grubu) | kullanıcı, 2026-10-11 | 987bbe9, 725b93c |

Sonradan eklenenler (bu belgenin §2.2 listesinden farklı):
- **Bağlantılar** grubu (Wi-Fi, VPN, Bluetooth; 2026-10-13) — kenar çubuğu dört
  grup: Kişisel · Masaüstü · Bağlantılar · Sistem.
- Masaüstü'ne Klavye, Fare ve dokunmatik yüzey; Sistem'e Güç, Tarih ve saat, Varsayılan
  uygulamalar, Kullanıcı hesabı eklendi. Yeni sayfalar da §4–§5 kurallarına uyar.
- Kişisel grubunun sonuna **Erişilebilirlik** (2026-10-16; `docs/design/accessibility-page.md`):
  Görme kartı Görünüm/Yazı/Hareket satırlarının aynası, Büyüteç, Ekran klavyesi.
- Fiyu (maskot) yalnızca boş durumlarda (`EmptyState`), "Fiyu'yu gizle" seçeneğine uyar.

### 13.2 Uygulama sonrası denetim (2026-10-15)

İzole sway'de (1600×1000 ve 820×900) Paneller, metin sayfaları, Görünüm, Duvar kağıdı ve
Ekranlar tasarıma karşı yeniden açıldı. Düzeltilenler:
- Duvar kağıdı › Monitör başına: sıfırlama düğmesinin yeri yalnızca bir monitörde elle ayar
  varken ayrılır; seç düğmesi kart kenarında, ↺ solunda (kayıt satırlarıyla aynı). Önce, hiç
  ayar yokken düğme kenardan ~40 px içerde duruyordu.
- Paneller › İçerik: tek listede kuyunun altındaki boş bırakma payı (48 px) kalktı; bar'ın çok
  sütunlu listelerinde kalır (sütunlar arası sürükleme).
- Görünüm: tek satırlık Maskot kartı sayfa sonuna (§5.2: Gölge → Uygulamalar akışı bölünmez).
- Ekranlar › Gece ışığı: satır başlığı bölüm başlığını tekrarlamaz → "Gece ışığını kullan"
- Ses: ilk kartın başlığı "Ses" → "Ses düzeyi" (sayfa adını tekrarlamaz; 2026-10-15).
  (Paneller'deki "Sağ paneli kullan" kalıbı).

Shell'e iletilen: şema açıklamalarında hitap ve teknik ad ("hyprsunset
kullanıyorsan kapalı bırak", "hypridle kullanıyorsan kapat", "strftime kalıbı") — Ayarlar
şemanın açıklamasını aynen gösterir.

**Karar — hitap, Hoş geldin istisnası** (2026-10-15): Hoş geldin metinlerinde samimi
"sen" kalır ("değiştirebilirsin", "istersen"). Gerekçe: Hoş geldin bir karşılama konuşmasıdır
(macOS kurulum yardımcısı ve GNOME Initial Setup da doğrudan hitap eder) ve Fiyu'nun tonuyla
uyumludur. Ayarlar sayfaları başvuru metnidir; karar 2'deki hitapsız dil orada geçerli.

**Karar — Görünüm, dış dosya mesajı** (küçük UX seçimi,
2026-10-16): shell `msg look status` v5 `last_apply.external` geldiğinde genel "aktarılamadı;
ayrıntı kabuk günlüğünde" yerine dosya başına iki cümle: neden ("`~/.config/gtk-3.0/gtk.css`
başka bir araç tarafından yönetiliyor.") + etki ("Bu dosyayı okuyan uygulamalar Nuxut
renklerini almıyor."). Hitapsız (karar 2); teknik ad yalnızca yol — kullanıcı dosyayı bulabilsin
diye gerekli; GTK/Qt/neden kodları (`read_only`, `hard_link`) gösterilmez, dört kümeye indirilir.
Durum kartı tek not olduğu için en çok iki benzersiz dosya adıyla anılır, kalanı "N dosya daha".
Başka kullanıcıya ait dosyada sahiplik komutu mevcut "Komutu kopyala" düğmesiyle verilir (Nuxut
çalıştırmaz). "Nuxut yönetsin" düğmesi framework `take_over` gelince (shell look-external.md §4).

**Karar — Güç › Kilit ve boşta** (sıra kilit düzeltmesinin önerisi,
2026-10-16): "Uykudan önce kilitle" ve "Nuxut kilit ekranını kullan" ayrı **Kilit** kartında en
üstte; boşta ana anahtarı ve aşamaları **Boşta kalınca** kartında. Shell'de `idle.enabled = false`
uykudan önce kilidi de kapatır; bu gizlenmez ve anahtar devre dışı da çizilmez — satır görünür
kalır, altında uyarı renginde nedeni yazar (çalışmayan bir anahtarın "açık" görünmesi güvenlik
sürprizi olurdu; devre dışı anahtar nedeni söylemezdi). Ayrıntı: `power-page.md`.
