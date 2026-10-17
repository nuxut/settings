# Varsayılan uygulamalar — Ayarlar tarafı (t116)

Sözleşme: defaults-v1 (taslak, onay 2026-10-13; kalıcı kopyası shell
`docs/design/defaults.md`). Bu belge
yalnızca Ayarlar'ın uygulama kararlarını tutar.

## Sınır
- Ayarlar `mimeapps.list`'e ve `xdg-terminals.list`'e **yazmaz** (sözleşme 1: yalnızca shell.toml +
  framework.toml). Aday listesi, mevcut seçim, yazım ve snapshot shell'dedir; Ayarlar
  `msg defaults status` okur, `msg defaults set <rol> <desktop-id>` gönderir.
- Geri alma shell snapshot'ıdır: `SettingId` `defaults.<rol>` (kayıttaki satır kimliğiyle aynı),
  "↺ önceki" mevcut `PreviousAction` deseniyle.

## Kararlar (2026-10-13)
1. **Yanıt güvenilmez girdi gibi doğrulanır**: desktop-id `[A-Za-z0-9._+-]`, `.desktop` sonu,
   `.`/`-` ile başlamaz, ≤ 255 bayt (shell'in `defaults set` doğrulamasıyla aynı küme); ad denetim
   karakterlerinden arındırılır, 128 karakter; rol başına en çok 256 aday, yinelenen kimlikte ilki;
   bilinmeyen rol/alan/kaynak yok sayılır; sürüm ≠ 1 → "bu sürüm yönetmiyor" notu.
2. **Durum okunamazsa satırlar gizli**, sayfanın üstünde kartsız not (yükleniyor / kabuk çalışmıyor /
   okunamadı / eski sürüm). Aramada rol satırı yoksa sonuç bu nota gider. Değer dosyada olmadığı için
   satırlar `Store`'a değil servis durumuna bakar.
3. **Seçici** (framework `Select::with_entries`, 44c10f6): görünür adaylar, ardından `NoDisplay`
   adaylar "Diğer" bölüm başlığının altında (başlık yalnızca üstünde ayıracağı bir seçenek varsa);
   hiçbir şey açmıyorsa ilk seçenek "Seçilmemiş" (seçilemez yer tutucu); mevcut seçim aday değilse
   "<ad> (kurulu değil)" seçili gösterilir; aynı adlı iki aday kimlikle ayrılır. `Select` indeksleri
   yalnızca seçenekleri sayar; desktop-id eşlemesi `RowModel` (`ids`) tutar. Seçim gönderilince
   durum gelene dek seçici yeni seçimi gösterir; hata olursa shell'in gerçek durumuna döner.
4. **Notlar**: `mixed` → bilgi notu; `shadowed_by` → hata renginde uyarı (yol shell'in `~` kısaltmasıyla,
   Nuxut o dosyaya dokunmaz); terminal `available:false` → not + seçici kapalı (odaklanamaz).
5. **Hatalar** şeritte, katalog metniyle; shell'in metni/`detail` alanı kullanıcıya geçmez. Shell JSON
   (`{"ok":false,"error":"<kod>"}`) gönderir; `error: <kod>` satırı da çözülür. Zaman aşımında yazım
   yapılmış olabilir: durum ve snapshot listesi yeniden okunur.
6. **Bekleme**: `status` 2 sn (çerçeve varsayılanı), `set` 5 sn (shell yazımdan önce snapshot alır).
   Sorgu ayrı thread'de, aynı anda tek; sürerken gelen istek bitince bir kez daha sorar. Dosya izleme
   yok: pencere odağı dönünce yeniden sorulur (yalnızca sayfa bu süreçte açıldıysa).
7. **İkon**: her seçenekte uygulamanın `.desktop` `Icon=` adı kullanıcının ikon temasından
   (`nuxut_image::IconTheme::system`, önizleme servisinin yalıtılmış `Loader`'ı); seçili seçeneğin
   ikonu kapalı alanda da görünür (satırda ayrı ikon yok). İkon yuvası baştan ayrılır
   (`with_icon_slot(true)`), ikonlar geldikçe `set_option_icon`; gelmeyen/temada olmayan ikon
   framework yer tutucusunda kalır. Ad shell'den gelir: yalnızca düz tema adı çözülür (mutlak yol
   reddedilir), güven `Untrusted` — tema dizinleri kullanıcı yazılabilir (`~/.local/share/icons`), SVG
   yalıtılmış işçide çözülür; işçi yoksa SVG ikon çözülmez, yer tutucu kalır. UI thread'i dosya
   okumaz; tema değişince (`Event::Look`) önbellek boşalır, kuşak numarası (`app_icons::generation`)
   satırlara yeniden yüklemeyi söyler.
