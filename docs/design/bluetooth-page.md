# Bağlantılar › Bluetooth sayfası (Ayarlar tarafı)

t115 v1. Ortak plan ve crate kararları: `../shell/docs/design/network-settings.md` (§4 nuxut-bluez,
§6 Bluetooth kapsamı). Bu not Ayarlar'a özgü kararları tutar. Tarih 2026-10-14;
uygulayan.

## Kapsam

- Gezinme: **Bağlantılar** grubunda Wi-Fi ve VPN'den sonra **Bluetooth** (`bluetooth`). Sayfa
  her zaman listelenir (adaptör yoksa sayfada maskotlu hata; Güç gibi gizlenmez).
- **Bluetooth** kartı: aç/kapa, bu bilgisayarın adı, görünürlük.
- **Eşleşmiş cihazlar**: tür ikonu, ad, Bağlı / Bağlı değil (+ Engellendi), pil yüzdesi; satıra
  tıklayınca satır içi ayrıntı: Ad (yeniden adlandır), Güvenilir, Engelle, Bağlan / Bağlantıyı
  kes, Unut (iki basışlı `ConfirmButton`).
- **Yakındaki cihazlar**: tarama satırı (Aranıyor… + dönen gösterge / Arama durdu + "Yeniden
  ara"), eşleşmemiş cihazlar, Eşleştir / İptal, adsız cihaz satırı.

## Kararlar

1. **Ajansız** (2026-10-13: Ayarlar asla ajan olmaz). `BluetoothOptions::default()`
   (`agent_path: None`; `services::bluetooth::options`, testli). PIN/passkey/onay kabuğun
   ajanında sorulur; Ayarlar eşleştirilen satırda "Eşleştiriliyor; onayı kabuk penceresinde
   verin" gösterir ve yalnız `Pair` sonucunu yansıtır. Uçtan uca testlerin her senaryosu sahte
   BlueZ'de `RegisterAgent`/`RequestDefaultAgent` görürse başarısız olur.
2. **Eşleştirme durum makinesi** (`Snapshot::pairing`, `pair_failed`; saf `apply`, testli). Aynı
   anda tek eşleştirme (diğer satırların "Eşleştir"i devre dışı). `Pair` isteğinin kimliği
   (`RequestId` 2026-10-14) `Pairing::request`'te; anlık görüntü kilidi altında bağlanır
   (sonuç kimlikten önce gelemez). Sonuç kimlikle eşlenir: `Done { action: "pair" }` başarı,
   `Failed { action: "pair" }` hata; başka isteğin (eski eşleştirme) sonucu süren eşleştirmeye
   dokunmaz. Crate `Paired=true`'yu `Done`'dan önce yayınlar: başarı çoğunlukla durumdan görünür
   (cihaz `Paired` ya da bağ gerektirmeyen LE `Connected`; cihaz listeden düşerse de biter).
   Otomatik güven + bağlan adımlarının hataları `request: None` ile üst nota gelir. Hata türü
   satırın alt metninde (hata rengi), üst nota gitmez. Kullanıcının "İptal"i (`CancelPairing`)
   sonrası gelen `AuthenticationCanceled` gösterilmez; cihazdan gelen iptal gösterilir. Diğer
   eylemler (`act`) kimlik `None` dönerse (kuyruğa girmedi) üst notta "BlueZ çalışmıyor"; başarıları
   durumdan görünür. Crate'in `#[non_exhaustive]` enum'ları: bilinmeyen olay yok sayılır,
   bilinmeyen hata genel metin (`settings-bluetooth-error-call-failed` + `Display`).
3. **Kabuğun ajanı sorulur** (karar (a): 2026-10-14; ayrıntı
   `../shell/docs/design/network-settings.md` (7)). BlueZ "ajan yok"u ayırt ettirmez (varsayılan
   ajan yoksa just-works ya da `AuthenticationFailed`). `Pair`'dan önce kısa ömürlü thread
   kabuğa `nuxut-shell msg bluetooth agent-status` sorar (`shell_ipc::IpcLink::from_env`: test
   örneğinde yalnız `NUXUT_SHELL_INSTANCE`'a; ≤ 2 sn) → `{"registered":bool,"default":bool}`.
   `default` değilse satırda bekleme metni yerine **"Eşleştirme onayı için masaüstü kabuğu
   çalışmalı"** (`settings-bluetooth-pairing-no-agent`) ve `Pair` **yine gönderilir** (onay
   istemeyen cihazlar eşleşir). Kabuk yanıt vermez / eski kabuk / sözleşme dışı yanıt → eski
   davranış (bekleme metni). Sorgu sürerken "İptal" → `Pair` hiç gönderilmez. **Neden msg,
   `BluetoothState::agent` değil:** o alan *bu istemcinin* ajan durumudur; Ayarlar ajansız olduğu
   için hep `false` — kabuğun ajanını yalnız kabuk bilir (sistem bus'ında iyi bilinen ad yok).
   `AuthenticationFailed` metni yine "kod yanlış ya da onay istenemedi (kabuk çalışıyor mu?)".
4. **Tarama yalnız sayfa görünürken** (`set_page_visible`, `App::navigate` ve açılış): sayfa
   açılınca bir kez, adaptör açılınca, "Yeniden ara"da ve odak dönünce (≥ 10 sn arayla). Süre
   60 sn (`DISCOVERY_TIMEOUT`); sonra "Yeniden ara". Sayfadan çıkınca durdurulur — crate
   `StopDiscovery`'yi yalnız kendi taraması sürerken çağırır, başka istemcinin taramasına
   dokunulmaz (uçtan uca testli). Tarama hakkı `compare_exchange` ile alınır (tek istek).
5. **Güç ve rfkill.** `PowerState=off-blocked` → sysfs (`/sys/class/rfkill`, salt okunur;
   crate okur) sert ise "Donanım anahtarıyla kapatılmış" ve anahtar devre dışı; yumuşak ya da
   okunamadıysa "Uçak kipi açık", anahtar etkin (açma denenir, BlueZ reddederse üst not türüyle:
   "Uçak kipi açık; Bluetooth açılamadı" / donanım anahtarı metni). Geçişte ("Değiştiriliyor…")
   anahtar devre dışı. Durum iyimser değil: anahtar BlueZ bildirince döner.
6. **Ad.** Adaptör `Alias`; Enter ya da odak kaybında (değiştiyse) yazılır, kırpılır; Esc geri
   alır. Boş ad BlueZ'de sistem adına döner. Cihaz adı (ayrıntıdaki "Ad") aynı kurallarla
   `Device1.Alias`; boş → cihazın kendi adı.
7. **Görünürlük** 180 sn (`DISCOVERABLE_SECS`, BlueZ varsayılanı; süre önce yazılır, testli).
   BlueZ kalan süreyi bildirmez: açılış bu oturumda görüldüyse alt metin "Görünür; 2:05 sonra
   kapanır" saniyede bir güncellenir (sayfa görünürken olay yayan kısa ömürlü thread; ayrı
   bileşen yok). Başka araçla açılmış görünürlükte süre bilinmez: "Yakındaki cihazlar bu
   bilgisayarı bulabilir". Adaptör kapalıyken anahtar devre dışı.
8. **Listeler.** Eşleşmiş: bağlı önce, sonra ada göre (crate sırası). Yakındaki: RSSI'ye göre
   (güçlü önce, RSSI'siz — artık duyulmayan — sonda), eşitlikte ad. Alt metin sinyal düzeyi
   (Güçlü ≥ −60 dBm, Orta ≥ −80, Zayıf, "Şu an duyulmuyor"). **Adsız cihazlar** (BlueZ `Name`
   yok) varsayılan gizli; kartın altında "N adsız cihaz gizlendi · Göster" / "N adsız cihaz
   gösteriliyor · Gizle" (anahtar yerine sayı satırı: kaç cihaz gizlendiğini söyler, yer
   kaplamaz; seçim sayfada, kalıcı değil). Eşleştirilen adsız cihaz her zaman görünür.
9. **Cihaz ikonu.** BlueZ `Icon` (freedesktop adı) → `Class` (CoD ana/alt sınıf) → `Appearance`
   (LE) → genel `Bluetooth`. Framework'te karşılığı olmayan türler (telefon, saat, oyun kolu,
   tablet bilgisayar) genel ikona düşer (framework talebi aşağıda).
10. **Boş/hata durumları.** BlueZ yok (`Unavailable::ServiceNotRunning`) / sistem bus'ı yok
    (`BusUnreachable`) / adaptör yok / servis açılamadı → sayfanın tek içeriği kartsız
    `EmptyState` + Fiyu `Error` (`Spot::BluetoothUnavailable`), diğer kartlar gizli. Eşleşmiş
    cihaz yok → kart içinde `Empty` (`Spot::BluetoothPairedEmpty`). İkisi aynı anda olmaz
    (ekranda tek maskot). Okunurken kartlar "Okunuyor…" ile yerinde. Bluetooth kapalı →
    eşleşmişler görünür (bağlan devre dışı), Yakındakiler "Bluetooth kapalıyken yakındaki
    cihazlar görünmez."
11. **v1 yalnız ilk adaptör.** Birden çok adaptörde ilki (yol sırası) kullanılır; diğerlerinin
    cihazları listelenmez. Aç/kapa crate'te tüm adaptörlere gider.
12. **Bus.** Üründe yalnız sistem bus'ı (ortamdan seçim yok; 2026-10-14). Ayarlar'ın
    `test-support` özelliği (testler, canlı test ikilisi) `nuxut_bluez::bus_from_env()` kullanır;
    `NUXUT_BLUEZ_BUS=session` ise oturum adresi açıkça verilir, adres yoksa servis açılmaz
    (sistem bus'ına ya da `$XDG_RUNTIME_DIR/bus`'a düşülmez; testli). `NUXUT_BLUEZ_RFKILL_DIR`
    yalnız `test-support`'ta sysfs yerine geçici dizin.
13. **Gizlilik.** Cihaz adı, adresi ve D-Bus yolu (MAC içerir) loglanmaz; yalnız eylem adı ve
    crate'in redakte `Debug`'ı. `CallFailed` ayrıntısı notta gösterilir, loglanmaz.
14. **Klavye.** Güç/görünürlük satırları Enter/Space; ad alanı metin alanı; cihaz listesi kendi
    içinde Tab ile gezilir (satır → ayrıntı alanları → düğmeler; yakındakilerde Yeniden ara →
    Eşleştir'ler → Göster), Enter/Space etkinleştirir, Esc ayrıntıyı kapatır.

## Bilinen sınırlar

- **Ses profili seçimi** (A2DP/HFP) v2.
- **Çoklu adaptör** v1'de ilki (karar 11).
- Kabuk ajan sorgusu kabuk yanıt verene dek (en çok IPC zaman aşımı, 2 sn) `Pair`'ı geciktirir;
  satır bu sırada zaten "Eşleştiriliyor" gösterir.

## Test

- Birim: `services::bluetooth` (sayfa durumu sırası, güç/rfkill, yakındaki sıralama ve adsız
  gizleme, yalnız ilk adaptör, ikon eşleme, görünürlük kalan süresi, olay katlama, eşleştirme
  durum makinesi, sonucun istek kimliğiyle eşlenmesi, kabuk ajan sorgusu ve uyarı kararı, tarama
  kararı, bus seçimi, ajansız seçenek) ve `pages::bluetooth` (satır verisi, ajan uyarısı metni,
  boş/gizli durumlar, hata/güç/görünürlük/sinyal metinleri, maskot yerleri, ayrıntının
  yenilemede korunması).
- Uçtan uca (gerçek BlueZ'e çağrı yok):
  `cargo test --features test-support --test bluetooth_dbus --no-run`, sonra
  `sh tools/bluetooth-dbus-test.sh <test ikilisi> <boş, kısa dizin>` (Unix soket yolu ≤ 107
  bayt; betik denetler). Sahte BlueZ crate'in paylaşılan `tests/fake_bluez`'i, test sürecinde
  (sahneler `Scene::*`). Betik her senaryoya geçici `XDG_RUNTIME_DIR` ve
  `NUXUT_SHELL_INSTANCE=stbttest` verir: kabuk sorgusu kullanıcının kabuğuna asla gitmez; sahte
  kabuk `msg` sunucusu test sürecinde. Senaryolar (12): BlueZ yok, sahne `no-adapter`, sahne
  `off` (kapalı başlar, tarama yok → aç → tarama), sahne `empty`, sahne `rfkill` (açma reddi),
  yakındakiler RSSI sırası (+ adsız gösterilince sonda), aç/kapa + rfkill yumuşak/sert, ad +
  görünürlük, tarama sayfa görünürlüğüne bağlı (yalnız kendi taramamız durur), eşleştirme
  başarı (Pair → `Paired`, eşleşmiş listeye geçer)/başarısız/ret/iptal, kabuk ajan uyarısı
  (varsayılan değil → uyarı + `Pair` gönderildi; varsayılan → bekleme metni; eski kabuk → uyarı
  yok, just-works eşleşir), bağlan/kopar, güven/engelle, yeniden adlandır, unut (cihaz listeden
  silinir), pil yüzdesi; her senaryoda ajan kaydı yok.
- Canlı görsel: izole headless sway + `dbus-run-session` + crate'in `examples/fake_bluez`'i
  (`cargo build --locked --manifest-path ../shell/crates/nuxut-bluez/Cargo.toml --features
  test-support --example fake_bluez --target-dir target/fake-bluez`;
  `NUXUT_FAKE_BLUEZ_SCENE=default|no-adapter|off|rfkill|empty|basic`), açık ve koyu kip.
  Ayarlar'ın geçici `examples/fake_bluez.rs` sahne kopyası silindi (2026-10-14: crate
  sahneleri aynısını verir).

## Açık sorular

- Eşleşmiş cihaz yokken "Yakındaki cihazlar"ı kartın üstüne almak (ilk kullanım) gerekir mi?
  Geçici seçim: sıra sabit, boş durum gövdesi "Aşağıdaki…" diye yönlendirir.
- Adsız cihaz görünürlüğü kalıcı olmalı mı (dosyaya yazılmayan sayfa durumu)? Geçici: kalıcı
  değil.
