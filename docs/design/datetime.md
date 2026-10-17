# Tarih ve saat sayfası (t119)

Kararlar: 2026-10-13.

## Kararlar ve gerekçe

1. **timedated doğrudan.** Ayarlar `systemd-timedated`'i (`org.freedesktop.timedate1`, sistem
   bus'ı) kendisi çağırır; shell üzerinden gitmez. Gerekçe: bu bir sistem ayarıdır —
   `shell.toml`/`framework.toml`'a yazılmaz, geri al geçmişine girmez. Tüm yazma çağrıları
   `interactive = true`: polkit ajanı (nuxut-polkit) yetki sorar.
   - Çağrılar tek arka plan thread'inde (`services/timedate.rs`, zbus blocking; `portal.rs`
     deseni); UI beklemez. Yöntem zaman aşımı 300 sn (`CALL_TIMEOUT`, karar ≥ 120 sn): polkit
     diyaloğu kullanıcıyı bekler. Bekleme sırasında ilgili satırın alt metni "Yetki bekleniyor…",
     diğer yazmalar kapalı.
   - Hata eşleme (`timedate::classify_name`): `org.freedesktop.PolicyKit1.Error.NotAuthorized`,
     `…Cancelled`, `org.freedesktop.DBus.Error.AccessDenied`, `AuthFailed`,
     `InteractiveAuthorizationRequired` → **sessiz**: hata kutusu/metni yok, satır önceki değere
     döner ("değişmedi"). İptal ile ret ayırt edilemediğinden ikisi de sessiz (karar,
     2026-10-13); `ServiceUnknown`, `NameHasNoOwner`,
     `UnknownObject`, `UnknownInterface` ya da bus'a bağlanılamaması → servis yok (sayfa notu
     "systemd-timedated bulunamadı", seçenekler kapalı); diğerleri → "Değiştirilemedi". Servisin
     hata ayrıntısı (`detail`) kullanıcıya gösterilmez, yalnızca loga gider.
2. **Sayfa:** `PageId::DateTime` ("Tarih ve saat", slug `datetime`), Sistem grubunda Bölge ve
   dil'in hemen ardından.
   - **Otomatik saat** (`SetNTP(b, true)`): alt metin "Saat eşitlendi" / "Saat henüz
     eşitlenmedi" (`NTPSynchronized`) / "Saat elle ayarlanıyor". `CanNTP = false` → anahtar
     kapalı + "Bu sistemde ağ saati hizmeti yok". Yetki verilmezse anahtar sessizce timedated'in
     değerine döner.
   - **Saat dilimi** (`SetTimezone(tz, true)`): satırda geçerli dilim "Şehir, Bölge — UTC±hh:mm"
     (alt çizgi boşluk; `America/Argentina/Buenos_Aires` → "Buenos Aires, America / Argentina").
     "Değiştir" satır içi arama alanlı seçiciyi açar (`ListTimezones`, bir kez okunur, ad
     doğrulanır). Arama `registry::fold` ile büyük/küçük harf ve aksan duyarsız, şehir ve bölgede;
     şehir adıyla başlayan önce; boş sorguda geçerli dilimin bölgesi; en çok 8 sonuç; oklar/Enter/Esc.
   - **Elle tarih ve saat** (`SetTime(usec, false, true)`): yalnızca otomatik saat kapalıyken
     framework `DatePicker` + `TimePicker` + "Uygula"; açıkken satır kapalı ve "Otomatik saat
     açıkken kapalı". Düzenlenmemiş alanlar dakika dönünce "şimdi"yi izler.
     Saat alanı (framework 663042e, 2026-10-13): `TimePicker::with_locale` + 1 dk adım
     (`with_minute_step(1)`, talep: elle ayarda tam dakika). Locale
     `locale::time_locale(store)`: framework `time_locale_for` ile aynı kural (`formats` +
     `hour_cycle`), ama değerler store'dan — süreç geneli `nuxut::i18n::time_locale()` yalnızca
     dosya diske yazılıp runtime yeniden yükleyince (`locale_changed`) güncellenir; store
     kullanıcının seçimini ve geri almayı hemen gösterir (Bölge ve dil önizlemesiyle aynı an;
     eşitlik birim testli). Biçim ya da 12/24 değişince alan yerinde `set_locale` ile güncellenir
     (yeniden kurulmaz; tarih alanı biçim adı değişince yeniden kurulur). Serbest giriş (rakamla
     saat) satır odağı ya da satır içi odak saat alanından çıkınca `cancel_entry` ile bırakılır.
   - **Notlar** framework `Notice` ile: üst not (timedated yok / okunamadı / LocalRTC)
     `Warning` (kırmızı uyarı ikonu); bar saati özel biçim notu `Info` (gri "i", kırmızısız),
     "Temizle" notun sağında.
   - **Saat biçimi** (`framework.toml [locale] hour_cycle = auto | h23 | h12`, framework 636b1b8):
     standart kayıt satırı (segmentli). Görünürlüğü framework şemasından: varsayılanı yoksa
     (eski framework) gizli (`registry::hour_cycle_supported`). "Otomatik" anahtarı siler. Bölge ve
     dil'in biçim önizlemesindeki saat bu ayara uyar (`locale::hour_cycle_override`).
   - **Bar saati özel biçim notu** (karar kaynağı: shell `docs/design/clock-locale.md`
     sözleşmesi, 2026-10-13). `[bar.widget.clock] format` /
     `vertical_format` yazılmamış ya da `""` (yalnız boşluk dahil) ise bar saati `hour_cycle`'a
     uyar; dolu strftime kalıbı kazanır. Not ("Bar saati özel biçim kullanıyor" + **Temizle**)
     yalnızca *etkili* kalıp doluyken görünür: shell yatay bar'da yalnız `format`'ı, dikey bar'da
     (`bar.edge` = left/right) yalnız `vertical_format`'ı okur (`ClockSettings::pattern`), öteki o
     yönde hiç kullanılmaz — yatay bar'da yalnız dikey kalıp yazılıysa saat yine tercihe uyar
     (`registry::clock_format_overrides`). Dize olmayan değer de "yazılmış" sayılır (shell
     uyarıyla reddeder ama dosyada duruyor; Temizle onu da kaldırır). **Temizle** duran iki
     anahtarı da tek geri al kaydıyla siler (silmek = `""`; bar sonradan yön değiştirince de saat
     tercihe uysun) — `datetime::clock_clear_writes`.
   - **Bar › Seçili widget › Biçim / Dikey bar biçimi:** yedek varsayılanlar (`defaults.rs`)
     shell şemasıyla eş: `""` (↺/varsayılan karşılaştırması boş değeri varsayılan sayar). Boş
     alanın yer tutucusu "Otomatik (12/24 saat tercihine uyar)" (`SettingDesc::placeholder`;
     shell'in varsayılanı dolu ise — eski shell — o kalıp). Alt metin şemanın `description`'ı
     (`shell-bar-clock-format-describe`, etkin dilde; `SettingDesc::display_subtitle` şema
     açıklamasını yerel metnin önüne koyar), şema yoksa kayıttaki aynı sözleşme metni. Alanı
     boşaltıp Enter anahtarı siler.
3. **Canlı güncelleme:** ayrı dinleyici thread'i `/org/freedesktop/timedate1` yolundaki
   `PropertiesChanged`'i (gövdedeki arayüz `org.freedesktop.timedate1`) dinler ve yeniden okur;
   timedated isteğe bağlı etkinleşip boşta çıktığı için eşleşme gönderen adına bağlı değil.
   `NTPSynchronized`/`TimeUSec` sinyal vermez: sayfa açılınca (`App::navigate`) ve pencere
   odağı dönünce (`App::keyboard_focus`) yeniden okunur (`timedate::recheck`, yalnızca servis
   bir kez başlatıldıysa). Her durum değişimi `Event::Timedate`; Bölge ve dil önizlemesi de
   yeniden biçimlenir (saat dilimi değişince saat yeni dilimde).
4. **Güvenlik:** testlerde ve canlı denemelerde gerçek sistem bus'ındaki timedated'e HİÇBİR
   yazma çağrısı yapılmaz. Birim testleri servise hiç gitmez (`cfg!(test)` → thread yok). Bus
   seçimi enjekte edilebilir: `NUXUT_TIMEDATE_BUS=session` (yalnız test/örnek) →
   `examples/fake_timedated.rs` (oturum bus'ında sahte timedate1: özellikleri tutar, `Set*`'ta
   günceller + `PropertiesChanged`; `NUXUT_FAKE_TIMEDATE_DENY=1` NotAuthorized,
   `…_DELAY_MS` bekleme, `…_NTP=0`, `…_NO_NTP=1`, `…_LOCALRTC=1`). Canlı test
   `dbus-run-session` içinde, izole sway'de.
5. **i18n:** sayfa metinleri `settings-datetime-*` (en + tr). Kayıt satırlarının başlıkları
   (registry) diğer satırlar gibi aşama B taşımasını bekler.

## Uygulama notları

- Yerel UTC ofseti (`timedate::local_offset`): `chrono::Local` (`TZ` / `/etc/localtime`). chrono
  doğrudan bağımlılık (onay, 2026-10-13; zaten nuxut-locale üzerinden ağaçta, aynı
  sürüm, yeni paket yok). chrono-tz eklenmedi. Elle saatte yerel tarih/saat → UNIX µs bu ofsetle.
- Yazma isteği yeni durum gelene dek iyimser: anahtar istenen konumda kalır; sonuç gelince
  timedated'in değerine eşitlenir.

- **polkit diyaloğunda kimlik:** isteyen çalıştırılabilir dosya yalnızca kök sahipli bir sistem
  yolundaysa (`/usr/bin/…`) doğrulanmış görünür; `~/.local` altına kurulumda diyalog
  "doğrulanmamış konum" etiketi gösterir — beklenen davranış (2026-10-13).

## Bilinen sınırlar

- **Kapsam dışı:** otomatik saat dilimi (geoclue); `LocalRTC` değiştirme — yalnızca
  `LocalRTC = true` ise sayfa üstünde uyarı notu.
- Seçicide geçerli dilim dışındaki dilimlerin ofseti gösterilmez (framework'te saat dilimi
  veritabanı yok; TZif ayrıştırıcı yazılmadı). Bölge adları IANA'nın İngilizcesiyle.
- Gösterilen ofset sistemin yerel saatinden (`/etc/localtime`); sahte timedated ile canlı testte
  dilim adı değişir, ofset değişmez.
- Elle saatte ofset hedef anın kuralıyla hesaplanır, ama yaz saati geçişindeki belirsiz/olmayan
  yerel saatler için seçim yapılmaz (ilk eşleşme).
- dbus-daemon'un kendi yanıt zaman aşımı polkit diyaloğundan kısaysa çağrı genel hatayla düşer
  (dbus-broker'da sınır yok).
