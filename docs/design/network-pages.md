# Bağlantılar › Wi-Fi ve VPN sayfaları (Ayarlar tarafı)

t114 v1: salt okuma iskeleti (2026-10-13) + **eylemler** (2026-10-14). Ortak
plan ve crate kararları: `../shell/docs/design/network-settings.md`. Bu not
Ayarlar'a özgü kararları tutar.

## Kapsam (v1)

- Gezinme: **Bağlantılar** grubu (`NavGroup::Connections`), Masaüstü ile Sistem arasında; sayfalar
  **Wi-Fi** (`wifi`; eş adlar `wi-fi`, `wlan`, `kablosuz`, `wireless`, `network`, `ag`) ve **VPN**
  (`vpn`). Karar: 2026-10-13. Bluetooth (t115) ayrı sayfa.
- **Wi-Fi:** aç/kapa (canlı); bağlı ağ (tıklayınca IP/DNS + "Bağlantıyı kes"); görünen ağlar
  (tıklayınca: açık ağa doğrudan bağlan, şifreli ağda satır içi şifre alanı, kurumsal ağda
  "desteklenmiyor" notu); "Gizli ağa bağlan…" (SSID + güvenlik + şifre); kayıtlı ağlar
  (tıklayınca otomatik bağlan, ölçülü bağlantı Otomatik/Evet/Hayır, Unut — iki adımlı onay);
  **Kablolu** kartı (salt okunur: IP, DNS, hız; yalnız kablolu aygıt varsa).
- **VPN:** her satırda aç/kapa anahtarı; tıklayınca Sil (iki adımlı onay); "WireGuard içe aktar…"
  (framework dosya seçicisi, `.conf` süzgeci).
- **v2'ye kalan:** 802.1X formu, elle IPv4/IPv6/DNS, MAC rastgeleleştirme, OpenVPN içe aktarma,
  içe aktarılan VPN'in adını düzenleme, şifre göster/gizle.

## Kararlar

1. **İstemci ortak, ajansız.** `nuxut-nm` `NetworkOptions::default()` ile açılır (crate'te
   ajansız güvenli varsayılan; `options_are_agentless` testi). Ayarlar asla gizli bilgi ajanı
   olmaz (2026-10-13); uçtan uca betik sahte NM logunda `Register` görürse başarısız.
2. **Canlı durum `sink` ile.** Crate tam anlık görüntüyü servis thread'inden `sink`'e verir;
   anlık görüntü saklanır, `Event::Network` yayılır.
3. **Kullanılamama nedeni ayrı metin** (2026-10-14): `NetState::Unavailable(Unavailable)` —
   `BusUnreachable` → "Sistem ileti yoluna (D-Bus) bağlanılamıyor…", `ServiceNotRunning` →
   "NetworkManager çalışmıyor…"; servis hiç açılamadıysa `Failed`.
4. **Bus seçimi yalnız `test-support` özelliğiyle** (crate kararına uyum,
   2026-10-14: üründe özelliksiz `bus_from_env` yok). Ayarlar'ın varsayılan olmayan `test-support`
   özelliği `nuxut-nm/test-support`'u açar; ürün derlemesinde ortam bus'ı değiştiremez
   (`product_build_always_uses_system_bus`). Özellik açıkken `NUXUT_NM_BUS=session` oturum bus'ına
   yönlendirir ve adres **açıkça** verilir; `DBUS_SESSION_BUS_ADDRESS` yoksa servis açılmaz
   (sistem bus'ına ya da zbus'ın varsayılan `$XDG_RUNTIME_DIR/bus` yoluna düşülmez —
   `bus_address` birim testi + uçtan uca `no_session_address`). Ayarlar'ın geçici
   `examples/fake_nm.rs` kopyası silindi; betik crate'in `examples/fake_nm`'ini derler.
5. **Eylem durum makinesi** (`services::network::{Op, Target, Snapshot::{begin, attach}}`;
   uyarlaması 2026-10-14). Her eylem bir hedefe (satır) bağlanır: Wi-Fi anahtarı, görünen
   ağ (SSID baytı), kayıtlı bağlantı (yol), gizli ağ formu, içe aktarma; hedef başına en çok bir
   bekleyen. Crate eylem metotları `Option<RequestId>` döner; kimlik eyleme **anlık görüntü kilidi
   altında** bağlanır (`start`; gönderim `try_send`, bloklamaz; kilit sırası SNAPSHOT → SERVICE),
   böylece sonuç olayı kimlikten önce gelemez. `None` → istek kuyruğa girmedi → satırda
   "NetworkManager çalışmıyor". Sonuç `Done { request }` / `Failed { request }` kimlikle eşlenir
   (eski "eylem adından tek bekleyen" sezgisi kalktı; aynı türden iki eylem ayrı hedeflerde
   birlikte bekleyebilir). Crate `Done`'dan önce o ana dek gelen durumu yayınlar (buna güvenilir):
   Wi-Fi, kes, unut, otomatik, ölçülü `Done` ile biter. **Etkinleştirmeler** (bağlan, VPN aç, gizli
   ağ) `Done` ile bitmez — NM çağrıyı kabul edip sonradan vazgeçebilir: `Done` kabul işaretidir
   (`acked`; `Done.object` kayıtlı bağlantıyı verir), bitiş bağlantı `Activated` görülünce ya da
   `ActivationFailed` ile (bağlantı yoluyla, yoksa tek adaya). Kabulden önce "zaten etkin" görünen
   bağlantı eylemi bitirmez. Gizli ağda `object` yoksa kabulde biter. Zaman aşımı (bağlan 45 sn,
   diğerleri 15 sn) yalnız güvenlik ağı: sessiz, durum yine gerçeği gösterir. Kimliği eşleşmeyen
   `Failed` sayfa üstü nota (tarama/ayrıntı hataları sessiz); eşleşmeyen `Done` yok sayılır.
   Crate'in `#[non_exhaustive]` enum'ları: bilinmeyen olay yok sayılır, bilinmeyen hata genel
   metin (`settings-network-error-call-failed` + `Display`), bilinmeyen etkinleşme nedeni "bağlantı
   kurulamadı", bilinmeyen WireGuard hata türü sözdizimi metni, bilinmeyen `Unavailable` "NM
   çalışmıyor".
6. **İyimser değil.** Anahtarlar ve seçimler her zaman NM'deki gerçek değeri gösterir; basınca
   istek gider, bekleme süresince ilgili denetim kapalı ve satırda ilerleme (Spinner) görünür,
   değer durum gelince değişir. Hata → gerçek duruma dönüş + satırda hata metni.
7. **polkit reddi sessiz** (timedate/accounts kalıbı): crate türü `NmError::Denied`
   (`…PermissionDenied` adları) → eylem düşer, metin yok (hedefli ya da hedefsiz). Metinden tanıma
   sezgisi (`is_auth_refusal`) kaldırıldı (2026-10-14): `CallFailed` artık hep metin olarak
   gösterilir. `Denied`'ın yine de bir metni var (`settings-network-error-denied`; normalde
   görünmez). Diğer hatalar hedefin satırında (`RowError` → `row_error_text`); hedefsiz hata sayfa
   üstü notta.
8. **Şifre akışı.** Şifreli ve kayıtsız ağ → satır içi parola alanı (`TextInput::password`,
   framework `SecretBuffer`, kilitli bellek). Gönderimde alandan **tek kopya**
   (`with_secret` → `Zeroizing<Vec<u8>>`), istemci doğrulaması `valid_secret` (ödünç `&str`,
   kopya yok), sonra `Secret::from_bytes` (kopyasız sahiplenme); alan hemen temizlenir. Ara
   `String` yok, log yok. `ActivationFailed(NoSecrets|AuthFailed)` ya da geçersiz şifre → satırda
   hata ve **alan yeniden açılır** (kayıtlı ağ da artık şifre ister).
9. **Kayıtlı ağın şifresini yenileme = yerinde güncelle + etkinleştir** (karar K3,
   2026-10-14: API gelince sessizce `Update`'e geç ile geldi). Kayıtlı ağa şifreyle
   bağlanmak `set_wifi_password(bağlantı, Secret)` gönderir (crate: `GetSettings` + yalnız şifre
   anahtarı + `Update`; otomatik bağlanma, ölçülü vb. **korunur**, bağlantı yolu aynı kalır). `Done`
   gelince aynı eylemin ikinci adımı (`FollowUp::Activate`): görünen ağsa `connect_access_point`
   (cihaz + erişim noktası; crate kayıtlı bağlantıyı etkinleştirir), değilse `activate`. Şifre
   adımı sürerken bağlantının eski hâliyle etkin görünmesi eylemi bitirmez. Eski "unut + yeni
   bağlantı" yolu ve `HELD` şifre tutucusu kaldırıldı (şifre Ayarlar'da bekletilmez; doğrudan
   crate'e devredilir). Crate yalnız `wpa-psk`/`sae` günceller: WEP / kurumsal / açık →
   `NmError::Unsupported` ("Bu ağ türünde şifre buradan değiştirilemiyor"); `psk-flags` 2 ("her
   seferinde sor") → `NmError::SecretNotSaved` ("Bu ağın şifresi kaydedilmiyor; bağlanırken her
   seferinde sorulur"). İkisi de satırda metin, şifre alanını yeniden açmaz, bağlantı değişmez.
   Yanlış yeni şifre: güncellenir, etkinleştirme `NoSecrets`/`AuthFailed` ile düşer → alan yeniden
   açılır.
10. **Kurumsal ağ:** bağlan yok, satır içi "desteklenmiyor; sonraki sürümde" notu.
11. **Gizli ağ:** SSID (1–32 bayt, istemcide `InvalidSsid`), güvenlik Açık / WPA/WPA2 / WPA3
    (WEP ve 802.1X v1'de yok), şifre (açıkta gizli). Form, yeni kayıtlı bağlantı (`Done.object`)
    etkinleşince kapanır (etkinleşme hatası satırda).
12. **Kayıtlı ağ ayrıntısı:** Otomatik bağlan (`set_autoconnect`), Ölçülü bağlantı
    (`set_metered`: Otomatik = `Metered::Unknown`, Evet, Hayır; NM tahminleri `GuessYes/GuessNo`
    Otomatik gösterilir), Unut (framework `ConfirmButton`: iki adımlı, süreli onay; geri al yok
    çünkü sistem ayarı). Bağlı ağın IP/DNS'i `device_details` ile (salt okunur; bağlı Wi-Fi ve
    kablolu cihazlar için bir kez, etkin bağlantı değişince ve sayfa açılınca yeniden).
13. **WireGuard içe aktarma:** `nuxut::portal` OpenFile (`*.conf` süzgeci) → dosya
    `MAX_CONF_BYTES` (64 KB) sınırıyla tek sıfırlanan tampona okunur (yeniden ayırma yok; dosya
    okurken büyürse fark edilir), UTF-8 denetimi `Secret::from_bytes` ile kopyasız, ad = dosya adı
    (uzantısız; geçersizse crate `InvalidName` → "Dosya adı bağlantı adı olamaz…"). Ayrıştırma
    hataları satır numarasıyla, değer içermeden (`import_error_text`). Yol, ad ve içerik loglanmaz.
    Crate `import_wireguard` → `WireGuardImport { request, ignored }`: `ignored` boş
    değilse (wg-quick'e özgü `PostUp`, `Table`… — çalıştırılmaz, içe aktarılmaz) içe aktarma
    satırının alt metninde bilgi notu: "Şu ayarlar içe aktarılmadı: PostUp, Table"
    (`settings-vpn-import-ignored`; **yalnız anahtar adları**, değer asla; logda yalnız sayı). Not
    `Snapshot::import_ignored`'da, yeni içe aktarma başlayınca silinir.
14. **Geri al geçmişi:** sistem eylemleri `Action::Set/Group` üretmez; geçmişe ve geri al şeridine
    girmez.
15. **Görsel dil (review notları, 2026-10-14):** "Wi-Fi" üç kez yinelenmesin diye ilk kart
    başlıksız (sayfa başlığı + "Wi-Fi" satırı yeter; `settings-section-wifi` kaldırıldı).
    Kapsama dışındaki kayıtlı ağda sinyal ikonu yerine nötr `Bookmark`; kapsamdakinde gerçek
    sinyal. Satır tıklanınca altında panel açılır (aynı kartta en çok bir), satır başında hover
    katmanı (`Interaction`). Framework bileşenleri: `TextInput`, `Button`, `ConfirmButton`,
    `Toggle`, `SegmentedButton`, `Spinner`, `Interaction`; sabit renk yok, hata metni `error`.
16. **Arama:** yeni kayıt satırları `wifi.hidden`, `wifi.wired`, `vpn.import`; anahtar kelimeler
    eylemleri de bulur (unut, ölçülü, bağlantıyı kes, sil, içe aktar).

## Bilinen sınırlar

- NM etkinleştirmeyi uzun sürdürürse (DHCP) satır 45 sn sonra beklemeyi bırakır (güvenlik ağı),
  durum yine gerçek.
- Şifre güncellemenin `Unsupported` / `SecretNotSaved` yolları yalnız birim testinde: sahte NM'nin
  WEP ve `psk-flags` örnek bağlantılarının (`EskiWep`, `Sormali`) erişim noktası görünmüyor;
  Ayarlar şifre alanını yalnız görünen ağda açar.
- polkit reddi (`Denied`) uçtan uca denenmiyor: sahte NM'nin `set_deny`'ı süreç içi API, ayrı
  süreç örneğinde (`examples/fake_nm`) açılamıyor; birim testinde.
- Şifre alanında göster/gizle yok (framework `TextInput` parola kipinde yok).

## Açık sorular

- Kablolu kartı Wi-Fi sayfasında mı kalsın, ayrı "Kablolu" sayfası mı? Geçici: Wi-Fi sayfasının
  altında küçük salt okunur kart (önceki açık soru).
- İçe aktarılan WireGuard bağlantısının adı dosya adından (1–15 bayt); uzun dosya adında kullanıcı
  dosyayı yeniden adlandırmalı. v2'de ad alanı.

## Test

- Birim: `services::network` (durum sırası, sıralama, kayıtlı/VPN durumu, olay katlama, bus
  kararı, ürün yapısında sistem bus'ı, ajansız seçenek, tarama kararı; durum makinesi: bağlan →
  kabul → şifre hatası → yeniden dene → bağlandı (kabulden önce etkin görünmek bitirmez), kayıtlı
  şifre yerinde güncelleme → etkinleştir (`Unsupported`/`SecretNotSaved` satırda, alan açılmaz),
  `Denied` sessiz (metin sezgisi yok), sonuçlar istek kimliğiyle eşlenir (aynı türden iki eylem,
  bilinmeyen kimlik, kuyruğa girmeyen istek), Wi-Fi gerçek durumla ya da `Done` ile biter,
  unut/otomatik/ölçülü bitişi, VPN etkinleştirme hatası hedefi, gizli ağ etkinleşmeyi bekler, NM
  gidince eylemler düşer, yeni içe aktarma eski notu siler, ayrıntı bağlı cihazları izler,
  şifre/SSID doğrulama, `Target` Debug
  redaksiyonu, `.conf` okuma sınırları) ve `pages::network` (basış eşlemesi güvenliğe göre, hata
  şifre alanını açar ve başarıda kapanır, bağlı ağ ayrıntısı, kablolu kart, nötr ikon, ölçülü
  eşleme, VPN anahtar/hata metni, gizli ağ/içe aktarma satırları, boş/gizli durumlar, notlar,
  içe aktarma hata metinleri, maskot yerleşimi).
- Uçtan uca (gerçek NM'e çağrı yok):
  ```sh
  cargo test --features test-support --test network_dbus --no-run
  cargo build --locked --manifest-path ../shell/crates/nuxut-nm/Cargo.toml --features test-support \
      --example fake_nm --target-dir target/fake-nm
  sh tools/network-dbus-test.sh <test ikilisi> target/fake-nm/debug/examples/fake_nm <boş dizin>
  ```
  Betik ortamı yalıtıp kendini `dbus-run-session` içinde çalıştırır; sahte NM sahnesi
  `NUXUT_FAKE_NM_SCENE` (crate'in sahneleri). Senaryolar (20): NM yok, bus ulaşılamaz, oturum
  adresi yok (düşme yok), bağlı (listeler + kablolu ayrıntı + tek tarama), doğrudan Wi-Fi sayfası,
  Wi-Fi kapat/aç, açık ağa bağlan, şifreli doğru, şifreli geçersiz (NM'ye çağrı yok), kayıtlı ağ
  şifre ister → yanlış yeni şifre (`Update` + `NoSecrets`) → doğru şifre (`Update secrets=1` +
  etkinleştir; `Delete`/`AddAndActivate` yok, aynı kayıt, otomatik bağlanma korunur), gizli ağ
  (geçersiz SSID + geçerli, etkinleşince biter), bağlantıyı kes, otomatik/ölçülü/unut (gizli
  bilgisiz `Update`, `secrets=0`), VPN aç (etkin değilken kapatma çağrı yapmaz), VPN aç → kapat
  (`Deactivate`), VPN sil, WireGuard içe aktar (bozuk dosya, geçersiz ad, geçerli + `PostUp`/`Table`
  bilgi notu, değer yok), sahneler `no-device` (aygıt yok), `empty` (boş listeler), `wifi-off`
  (kapalı başlar → aç → tarama). Her senaryoda ajan kaydı ve tarama sayısı sahte NM logundan
  denetlenir. Betik düzeltmesi (2026-10-14): senaryo arasındaki temizlik `set +e` bırakıyordu; ilk
  sahte NM senaryosundan sonra **test hataları betiği durdurmuyordu** (yalnız log denetimleri
  çalışıyordu). Temizlik `stop_fake` (set -e'ye dokunmaz) + trap'te `cleanup` olarak ayrıldı.
- Canlı görsel: izole headless sway + `dbus-run-session` içinde crate'in sahte NM'si
  (`cargo build --features test-support`), `NUXUT_SETTINGS_DEBUG` adımları (`expand=`, `click=`,
  `type=`, `key=enter`, `wgimport=<yol>`): şifre alanı açık, şifre hatası, gizli ağ formu, kayıtlı
  ayrıntı, bağlı ağ, kablolu kart, VPN sil + içe aktarma hatası/başarısı; açık ve koyu kip.
