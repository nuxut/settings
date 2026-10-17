# Sistem › Güç sayfası (Ayarlar tarafı, t117)

Ortak plan, UPower/ppd ayrıntıları ve kabuk servisi: `../shell/docs/design/power.md`.
Bu not Ayarlar'a özgü kararları tutar. Tarih 2026-10-14.

## Kapsam (iskelet aşaması, 2026-10-14; canlı kaynak aşağıda)

- **İskelet, sahte veriyle.** Gerçek veri kaynağı paylaşılan `nuxut-power` crate'idir; henüz
  yok (kabukta yazılıyor). Sayfa, model, görünürlük kuralları, yazma akışı ve metinler hazır;
  yalnız veri kaynağı eksik.
- Yer: **Sistem → Güç** (`PageId::Power`, Ses'ten sonra; slug `power`, eş adlar `guc`, `pil`,
  `battery`, `enerji`, `energy`).
- Kartlar: **Güç profili** (bölümlü düğme) ve **Pil** ("Pil ömrünü koru" anahtarı).

## Kararlar

1. **Veri yolu B: paylaşılan crate** (2026-10-13). Ayarlar kabuğun
   `msg power`'ına BAĞLANMAZ: kabuk kapalıyken de çalışmalı, canlı durum `PropertiesChanged` ile
   gelmeli. Kabuğun `msg power` sözleşmesi (power-v1) betikler/tuş bağları için kalır.
2. **v1 metinleri** (2026-10-13): anahtar "Pil ömrünü koru", alt metin
   "Şarj %N'de durur" (N = bitiş eşiği). Başlangıç eşiği de varsa aynı cümleye
   ", %S'in altına düşünce yeniden başlar" eklenir; eşik yoksa "Şarj sınırını aygıt yazılımı
   belirler"; karışık durumda "Pillerin bir kısmında açık" (her şeyden önce). Anahtar yalnızca
   destekleyen donanımda. **v1'de eşik düzenleme yok** (eşikler salt okunur, kök yetkili hwdb).
3. **Görünürlük** (2026-10-15 "Canlı kaynak" 4 ile değişti; `services::power::listed`, `charge_shown`): kaynak yoksa sayfa kenar
   çubuğunda ve aramada yok. Profil hizmeti ve şarj sınırının ikisi de yoksa sayfa gizli. Pil
   (şarj sınırı desteği) yoksa yalnız profil kartı. ppd yoksa profil kartı gizlenmez: seçim
   devre dışı + "Güç profilleri için power-profiles-daemon gerekli" (kullanıcı nedenini görsün;
   power.md planı).
4. **Yazma iyimser değil** (power.md). İstek → `OpState::Waiting` (seçim/anahtar istenen konumda,
   devre dışı); durum ancak kaynak bildirince değişir. Hata → durum aynen kalır, kontrol gerçek
   duruma döner, hata metni satırın alt metninde hata renginde (son işlemin satırında; sonraki
   istek temizler). Aynı anda tek yazma.
5. **Hata → metin/davranış:** `Denied` → "İzin verilmedi (yalnızca etkin yerel oturumda
   değiştirilebilir)"; `Unsupported` → şarj sınırında anahtar gizlenir ve durum yeniden okunur
   (profilde profil listeden düşer); `Unavailable` → "UPower çalışmıyor" (profilde "ppd gerekli"
   metni); `Failed(<D-Bus adı>)` → "Değiştirilemedi" (ad yalnız log'da).
6. **Kart başlığı "Güç profili", satır başlığı çizilmez** (aynı sözcük iki kez görünmesin; kayıt
   satırının başlığı aramada kullanılır). Düğmenin altında not: kısıt nedeni (`lap-detected` →
   "Performans kısıtlı: dizüstü kucakta algılandı", `high-operating-temperature` → "…sıcaklık
   yüksek", diğer → "Performans kısıtlı"), ppd yok ya da hata.
7. **Türkçe sayı ekleri katalogda.** "%80'de", "%60'ta", "%75'in" eki sayıya göre değişir.
   Kod dilbilgisi bilmez: yalnızca sayının okunuşundaki son sözcüğün değerini verir
   (`pages::power::spoken_tail`: 75 → 5, 80 → 80, 100 → 100) ve `tr.ftl` ek seçimini
   `$end_tail`/`$start_tail` üzerinden yapar. Bekçi (`i18n_catalog`) değişken eşliği istediği
   için en.ftl bunları tek seçenekli seçimle "kullanır". Gerekçe: koddaki Türkçe ek tablosu hem
   gömülü metin bekçisine takılır hem dilbilgisini koda taşır.

## Kaynak soyutlaması

`services/power.rs`:

- **Model** crate'in gelecek türlerinin birebir yakın kopyası (kabuktaki
  `src/services/power/model.rs` adlarıyla): `PowerProfile`, `Profiles { active, available,
  degraded: Option<String> }`, `ChargeLimit::{Unsupported, Supported { enabled, mixed, start,
  end }}`, `PowerState { upower, profiles, charge_limit }` (Ayarlar'ın okuduğu alt küme),
  `PowerError::{Denied, Unsupported, Unavailable, Failed(String)}`.
- **Sayfanın gördüğü** yalnızca `Snapshot { state: Option<PowerState>, op: OpState }` +
  `set_profile(p)` / `set_charge_limit(on)` + `Event::Power`. Saf `begin`/`finish` yazma
  durumunu yönetir (testli).
- **`Source`:** `None` (ürün yapısı, crate gelene kadar: `state = None`, sayfa gizli) ve
  `Fake` (`NUXUT_POWER_FAKE=<dosya>`; TOML ya da `.json`). Sahte dosya durum + yazma sonucu +
  gecikme taşır (`FakeSpec` belge yorumundaki örnek). Sahte yazma gerçek kaynak gibi eşzamansız
  (arka plan thread'i, `delay_ms` ≤ 10 sn); başarıda durum değişir, `unsupported`'da yeniden
  okuma (özellik yok) simüle edilir. Birim testleri ortamı okumaz (kaynak `None`).

Canlı deneme senaryoları (izole sway; `NUXUT_SETTINGS_DEBUG=wait=800;click=X,Y` ile tıklama):
dizüstü (profiller + sınır açık, yazma reddi), kısıt (lap-detected), ppd yok + karışık sınır,
pil yok (iki profil), veri yok (sayfa gizli, açılış ilk sayfaya düşer).

## Canlı kaynak (uygulandı 2026-10-15)

Kabukta `crates/nuxut-power` indi (shell a298014; `../shell/docs/design/power.md` "Karar: B").
Ayarlar tarafındaki değişiklikler:

1. **Bağlantı:** `Cargo.toml` `nuxut-power` (path), `test-support` özelliği crate'inkini açar.
   `Source::Live(PowerService)`: `ensure()` servisi bir kez açar (sistem bus'ı; `test-support` ile
   `NUXUT_POWER_BUS=session`, adres yoksa açılmaz → `None`). `NUXUT_POWER_FAKE` verildiyse sahte
   kaynak öncelikli.
2. **Görünüm modeli korunur:** sayfa crate türünü doğrudan okumaz; `services::power::PowerState`
   küçük görünüm modelidir, tek dönüşüm `from_live` (profiller sabit sırada, boş kısıt nedeni
   yok). Gerekçe: sahte kaynak ve saf testler aynı modeli kullanır, crate'in özel alanları
   (pil yolları, seri numaraları) sayfaya sızmaz. Hata türü `error_from_live` ile birebir.
3. **Yazma sonucu kimlikle:** crate `set_profile`/`set_charge_limit` kimlik döndürür;
   `PENDING` uçuştaki (kimlik, işlem)'i tutar, istek gönderilirken kilitli kalır (sonuç kimlik
   kaydedilmeden gelemez). `apply_live` saf ve testli: `Changed` → durum, eşleşen `*Done` →
   `finish` (durum zaten önce gelmiş olur), başka kimlik yok sayılır. Servis meşgulse
   (`None` kimlik) hemen `Unavailable`.
4. **Görünürlük (Karar 3'ün yerine, 2026-10-15):** kaynak varsa sayfa her zaman
   kenar çubuğunda ve aramada (UPower/ppd sonradan gelip gidebilir; kenar çubuğunu yeniden
   kurmak gerekmez — eski "kenar çubuğu canlı değil" sınırı kalktı). Pil, şarj sınırı ve ppd
   üçü de yoksa kartlar gizlenir, kartsız maskotlu boş durum (`PowerStatus`):
   UPower da yoksa `Spot::PowerUnavailable` (hata) "Güç bilgisi alınamıyor"; UPower var, pil
   yoksa `Spot::PowerEmpty` (boş) "Ayarlanacak güç seçeneği yok". Canlı kaynağın ilk anı
   (servis henüz bildirmedi) hizmet yok sayılır; ilk `Changed` ms içinde gelir.
5. **Pil durumu satırı** (`BatteryRow`, Pil kartının ilk satırı; pil yoksa gizli): düzey ikonu
   (şarjda `BatteryCharging`), başlık "%64", alt metin şarj durumu ve süre ("Pilden çalışıyor ·
   2 sa 10 dk kaldı", "Şarj oluyor · 35 dk içinde dolar", "Tam dolu", "Prizde, şarj olmuyor").
   Kart sırası Pil, Güç profili (kayıt sırası).
6. **Metinler:** 18 yeni anahtar (tr; en taslağı, tek düzeltme: anahtar
   sözcüklerde tekrar eden "battery" çıkarıldı).

Canlı deneme: izole sway + `dbus-run-session`; `NUXUT_POWER_BUS=session` (oturum bus'ında
UPower/ppd yok → hata boş durumu) ve `NUXUT_POWER_FAKE` dizüstü senaryosu.

## Seçenek başına devre dışı (uygulandı 2026-10-16)

Framework 86f0125 `SegmentedButton::set_segment_enabled` / `Segment::with_disabled` geldi
(framework kuyruğuna gerek kalmadı). Profil seçiminde üç seçenek hep
çizilir; ppd'nin sunmadığı tek başına devre dışı (`ProfileView::available`). Etkin profil
listede yoksa seçim Dengeli'ye, o da yoksa ilk sunulana düşer.

## Kilit ve boşta kartları (uygulandı 2026-10-16)

İstek: kilit tetikleyici düzeltmesi (shell 7b3c80e `idle.lock_before_sleep`, varsayılan açık — karar 2026-10-16,
shell `docs/design/lock-trigger.md`; `[lock] listen_logind`, varsayılan açık). Ayarlar'da o güne
dek hiç `idle.*` / `lock.*` satırı yoktu. Sıra kilit düzeltmesinin önerisi; diğer küçük
seçimler Ayarlar tasarımının (2026-10-16):

- **Yer: Güç sayfası**, Pil ve Güç profili kartlarının ardından iki kart. Gerekçe: GNOME
  (Güç › Ekranı karart/Otomatik askıya alma), KDE (Güç yönetimi) ve macOS (Pil › Ekran kapanınca)
  boşta eylemlerini güçte toplar; ayrı sayfa açmak 14 satır için fazla.
- **Kilit** (en üstte): "Uykudan önce kilitle" (`idle.lock_before_sleep`; en sık beklenen,
  varsayılan açık) ve "Nuxut kilit ekranını kullan" (`lock.listen_logind`; kendi kilit aracını —
  hyprlock, swaylock — kullanan kapatır; teknik adlar yalnız arama sözcüğünde).
- **Boşta kalınca**: ana anahtar "Boşta eylemlerini Nuxut yönetsin" (`idle.enabled`; kapalıyken
  aşama satırları gizli), boşta kilitle, karart, ekranı kapat, uyu (+ prizdeyken de), her biri
  açıkken süresi; "Medya oynarken bekle".
- **`idle.enabled = false` uykudan önce kilidi de kapatır** (shell `IdleConfig::settings`). Bu
  gizlenmez: "Uykudan önce kilitle" satırı görünür kalır ve anahtar açıkken altında uyarı
  renginde "Boşta eylemleri kapalı olduğu için uykudan önce kilitlenmiyor" yazar
  (`SettingDesc::note_in`, satır uyarı mekanizması — Hyprland geçersiz kılma uyarısıyla aynı
  görünüm). Ana anahtarın alt metni de söyler (shell şeması açıklama verirse onunki gösterilir).
  Anahtarı devre dışı çizmek seçilmedi: framework satırında devre dışı durum yok, nedeni de
  söylemezdi.
- Süre kaydırıcıları saniye yazar (şema birimi `s`), 1 dk adımla; 60 sn ve üstü dakikayla
  okunur ("10 dk"; `rows::format_number_in`, tüm `s` birimli kaydırıcılar). Yerel aralıklar
  shell `STAGE_SECS` (30..=14400) içinde: karart/kapat/kilit 1–60 dk, uyku 5–120 dk.
  `idle.lock_after = 0` ("boşta kilitleme") Ayarlar'dan yazılmaz: kapatmak için anahtar var.
- Şema `[idle]`'ı tanımıyorsa boşta kartı yok; "Uykudan önce kilitle" ve "Nuxut kilit ekranını
  kullan" yalnızca anahtarı tanıyan shell'de. Kartlar varken boş durum maskotu gösterilmez ve
  Güç sayfası güç hizmeti olmasa da kenar çubuğunda kalır.
- **Sınır:** kenar çubuğu açılışta bir kez kurulur; güç hizmeti açılamadığı ve shell şeması o
  ana dek yüklenmediği durumda Güç sayfası o oturumda listelenmez (arama da göstermez).

## Bilinen sınırlar / açık sorular

- Sahte kaynak `upower = false` iken şarj sınırını desteklenmiyor sayar (gerçek kaynakla aynı);
  "UPower çalışmıyor" metni yalnız yazma hatasında görünür. UPower düşerse sayfa üstü not
  gerekip gerekmediği crate davranışına göre karar.
- en.ftl metinleri geçici (gözden geçirilecek); Türkçe ek seçimi tablosu da
  onaya açık.
