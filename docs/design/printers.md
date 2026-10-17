# Sistem › Yazıcılar sayfası (t125)

Tarih 2026-10-15; araştırma, tasarım ve uygulama. Kararlar
aşağıda tarihli; açıkça onaylı yazmayanlar öneri olarak
incelemesine açıktır.

## 1. CUPS'a nasıl konuşulur

CUPS'un tek kamu arayüzü IPP'dir (HTTP POST, `application/ipp` gövde). libcups da aynı şeyi
yapar: `/run/cups/cups.sock` yerel soketine (yoksa `localhost:631`) HTTP/1.1 ile IPP gönderir.

| Yol | Hız | Bakım | Bağımlılık | Sonuç |
|---|---|---|---|---|
| **A. `ipp` crate'i (yalnız protokol) + kendi küçük HTTP/1.1 istemcimiz** (yerel soket / localhost TCP) | istek başına < 1 ms yerelde; bağlantı başına bir thread yok, servis thread'inde bloklayan çağrı | `ipp` 7.0.0 (ancwrd1/ipp.rs, MIT/Apache-2.0, 2015'ten beri, düzenli sürüm; RFC 8010 değer türlerinin tamamı, CUPS işlemleri) | `ipp` + `bytes`, `http`, `enum-as-inner`, `enum-primitive-derive` (küçük; `num-traits`, `thiserror` zaten ağaçta). TLS/async yok | **Seçildi** |
| B. `ipp` crate'inin kendi istemcisi (`client` = ureq, `async-client` = reqwest + tokio) | aynı | aynı | ureq/reqwest + rustls; **Unix soketi desteklemez** (yalnız TCP; soket-yalnız CUPS kurulumunda çalışmaz) | reddedildi |
| C. libcups FFI (`cups-sys` ya da elle bindgen) | aynı | libcups'ın kendisi bakımlı; Rust binding'leri bakımsız | C kütüphanesi + `unsafe` — Ayarlar `#![forbid(unsafe_code)]` | reddedildi |
| D. `lpstat`/`lp`/`cancel` komutlarını çalıştırmak | süreç başına ~5–20 ms, çıktı yerelleştirilmiş ve kararsız | — | `cups` istemci araçları | reddedildi (ayrıştırma kırılgan, hata türü yok) |

HTTP katmanı bilerek küçük (`services/printers/cups.rs`): tek istek, `Connection: close`,
`Content-Length` ya da `chunked` yanıt, 5 sn okuma/yazma zaman aşımı, 4 MiB yanıt sınırı.
Kimlik doğrulama (401) yok: v1'in işlemleri CUPS'un varsayılan ilkesinde yetki istemez
(bkz. §2). Sunucu seçimi libcups'la aynı: `CUPS_SERVER` (`/` ile başlarsa soket yolu, değilse
`ana-makine[:port]`), yoksa `/run/cups/cups.sock` varsa o, yoksa `localhost:631`. Uzak
sunucuya (`CUPS_SERVER=print.example.com`) da düz HTTP ile gider; TLS gerekiyorsa sayfa
"Yazdırma servisine ulaşılamıyor" der (bilinen sınır).

## 2. Yetki ve v1 kapsamı

CUPS'un varsayılan ilkesi (`/etc/cups/cupsd.conf` `<Policy default>`):

- **Yetki istemeyen:** `CUPS-Get-Printers`, `CUPS-Get-Default`, `Get-Jobs`, `Print-Job`,
  kendi işini `Cancel-Job` (`Require user @OWNER @SYSTEM`; sahip `requesting-user-name`'den).
- **Yönetici (`@SYSTEM` = wheel/sys; parola ya da yerel sokette PeerCred):**
  `CUPS-Add-Modify-Printer`, `CUPS-Delete-Printer`, `CUPS-Set-Default`, `CUPS-Get-Devices`,
  yazıcıyı duraklat/sürdür, başkasının işini iptal. CUPS polkit kullanmaz.

**v1 (bu iş):**

1. Yazıcı listesi ve durumu (Hazır / Yazdırıyor / Durduruldu + CUPS'un durum iletisi; işi
   kabul etmiyorsa belirtilir). Satıra tıklayınca ayrıntı: model, konum, kuyruk adı,
   "Varsayılan yap", "Deneme sayfası".
2. **Varsayılan yazıcı kullanıcı düzeyinde**: `~/.cups/lpoptions` dosyasındaki `Default`
   satırı (libcups'ın `lpoptions -d` ile yazdığı yer; GTK/Qt yazdırma pencereleri ve `lp`
   okur). Yönetici yetkisi gerekmez. Etkin varsayılan = lpoptions `Default`'u (yazıcı hâlâ
   varsa), yoksa sunucunun `CUPS-Get-Default`'u. Yazım biçim korur: önceki `Default` satırı
   `Dest`'e döner (seçenekleri kalır), hedefin `Dest` satırı `Default` olur, yoksa sona
   `Default <ad>` eklenir; atomik (geçici dosya + rename). Sistem geneli varsayılan
   (`CUPS-Set-Default`) v2.
3. **İş kuyruğu**: kullanıcının bitmemiş işleri (`Get-Jobs`, `which-jobs=not-completed`,
   `my-jobs=true`); satırda yazıcı, durum, boyut ve "İptal" (`Cancel-Job`).
4. **Deneme sayfası**: CUPS'un kendi deneme sayfası (`/usr/share/cups/data/testprint`,
   `application/vnd.cups-banner`; GNOME ile aynı). Dosya yoksa düğme gizli.
5. **Yazıcı ekleme ve gelişmiş yönetim**: v1'de CUPS web arayüzüne bağlantı
   ("Yazıcı ekle…" → `http://localhost:631/admin`, `services::open_url`). Yalnız sunucu
   yerelse.

**v2'ye kalan:** ağ/USB keşfi ve sürücüsüz ekleme (Avahi D-Bus `_ipp._tcp`/`_ipps._tcp` +
`CUPS-Get-Devices`, IPP Everywhere `CUPS-Add-Modify-Printer` `ppd-name=everywhere`) — hepsi
yönetici yetkisi ister; PeerCred'li 401 akışı ve parola sorma tasarımı ayrı iş. Ayrıca:
yazıcıyı duraklat/sürdür, sil, sistem varsayılanı, mürekkep düzeyi (`marker-levels`),
yazıcı başına seçenekler (kâğıt, çift yüz), CUPS bildirim aboneliği (yoklama yerine).

Karar gerekçesi: v1'deki her işlem yetkisiz çalışır; yetki isteyen akışı yarım (parolasız
wheel'de çalışan, diğerlerinde sessizce reddedilen) göndermek yerine web arayüzüne bağlanılır.

## 3. Canlılık

CUPS durum değişikliğini yalnız abonelikle (IPP notifications) iter; v1 **yoklar**, ama
yalnız sayfa görünürken: sayfa açılınca hemen, sonra bekleyen iş varken 2 sn, yokken 5 sn
arayla; pencere odağı dönünce hemen (`printers::recheck`). Sayfadan çıkınca thread uyur
(istek yok). Yoklama arka plan thread'inde (`nuxut-printers`); UI'a `Event::Printers` yalnız
anlık görüntü değişince gider.

## 4. CUPS yoksa

Bu VM'de (2026-10-15, salt okuma): `systemctl status cups` → "Unit cups.service could not be
found", `lpstat` yok, `/run/cups` yok; yalnız `libcups` kurulu (GTK bağımlılığı). Sayfa:

- Sunucuya bağlanılamadı (soket yok / bağlantı reddedildi) → sayfanın tek içeriği maskotlu
  boş durum (`Spot::PrintersUnavailable`, `Error`): "Yazdırma servisi yok" + "CUPS kurulu
  değil ya da çalışmıyor…" gövdesi. Sayfa kenar çubuğunda **görünür kalır** (Güç'ün aksine):
  kullanıcı yazıcıyı nerede arayacağını bilsin, nedenini görsün.
- Bağlandı ama yazıcı yok → Yazıcılar kartında maskotlu boş durum (`Spot::PrintersEmpty`,
  `Empty`) + "Yazıcı ekle…" satırı.
- Diğer hata (HTTP/IPP hatası, bozuk yanıt) → maskotlu "Yazıcı bilgisi okunamadı".

## 5. Kenar çubuğunda yer

**Sistem grubu, Güç'ten sonra** (Bildirimler · Ses · Güç · **Yazıcılar** · Bölge ve dil …).
Gerekçe: settings-ux §2.2 grupları nesneye göredir; Bağlantılar ağ/kablosuz bağlantılardır
(Wi-Fi, VPN, Bluetooth) — yazıcı bir bağlantı türü değil, sisteme bağlı bir aygıttır. Sistem
grubunun başı aygıt/donanım kümesi (Ses, Güç); Yazıcılar onların yanında, bölge/saat/hesap
kümesinden önce. GNOME (Printers, Sound/Power yakınında) ve KDE (Donanım › Yazıcılar) ile
uyumlu. `PageId::Printers`, slug `printers`, eş adlar `yazici`, `printer`, `cups`, `yazdir`.
İkon `IconName::Print`.

## 6. Kaynak düzeni

- `services/printers/cups.rs` — UI'sız istemci: `Server`, HTTP/IPP gidiş-dönüş, işlemler
  (`printers`, `server_default`, `jobs`, `cancel_job`, `print_document`, `print_test_page`), saf ayrıştırıcılar
  (`Printer`, `Job`, `PrinterState`, `JobState`). Hata türü `CupsError` (bağlanamadı / HTTP /
  IPP durum kodu / bozuk yanıt).
- `services/printers/lpoptions.rs` — kullanıcı varsayılanını okuma/yazma (saf metin dönüşümü,
  testli).
- `services/printers.rs` — süreç geneli anlık görüntü, yoklama thread'i, eylem durumu
  (`Op`: bekleyen eylem ve son hata), `Event::Printers`.
- `pages/printers.rs` — `PrintersStatus` (kartsız: maskotlu hata ya da son eylem hatası
  notu), `PrinterList` (satırlar + satır içi ayrıntı), `PrintJobs`, `PrintersAdmin`
  ("Yazıcı ekle…").

Gizlilik: yazıcı adı, iş adı ve kullanıcı adı loglanmaz; yalnız işlem adı ve IPP durum kodu.

## 7. Test

- Birim: ayrıştırıcılar (IPP yanıtı → model), lpoptions dönüşümü, görünüm türetme
  (durum metinleri, varsayılan seçimi, iş sırası), HTTP yanıt ayrıştırma (chunked/length).
- Entegrasyon (`tests/printers_ipp.rs`): süreç içi **sahte IPP sunucusu**
  (`tests/support/fake_cups.rs`; geçici dizinde Unix soketi, `ipp` crate'iyle istek
  ayrıştırır, yazıcı/iş tablosu tutar): liste, varsayılan, iş listesi, iptal, deneme sayfası,
  TCP yolu, sunucu yok. **Gerçek CUPS'a yazma işlemi gönderilmez**: testler sunucuyu her
  zaman açıkça verir (ortamdan okumaz); deneme sayfası ve iptal yalnız sahte sunucuda.
- Canlı ekran görüntüsü: izole sway'de `CUPS_SERVER=<sahte soket>` ile
  (`examples/fake_cups.rs` aynı sahte sunucuyu ayrı süreçte çalıştırır), geçici `HOME` ve
  `XDG_CONFIG_HOME`.

## 7a. Güvenlik: düşmanca sunucu yanıtları

Otomatik commit incelemesinin "resource-cap-defeat / integer overflow" bulgusu
(2026-10-15). İlk sürümde chunked gövdede `body.len() + n` (`n` = hex parça boyu, `usize::MAX`
olabilir) taşabiliyordu; trailer, 1xx ve uzunluksuz akışlar süre sınırsızdı; `ipp`
ayrıştırıcısı koleksiyon derinliği ve öznitelik sayısı sınırlamıyor (derin iç içe koleksiyon
özyinelemeli `Drop`'ta yığını taşırır). Düzeltme (`cups.rs`):

- Gövde ≤ 1 MiB (`MAX_RESPONSE`); sınır her ayırmadan **önce**, `checked_add` ile; parça boyu en
  çok 16 hex hane, `u64` → `usize` dönüşümü denetimli; `Content-Length` `u64` olarak okunup
  sınırla karşılaştırılır, çelişen ikinci değer reddedilir; parça sonrası CRLF zorunlu.
- Satır ≤ 8 KiB, başlık/trailer ≤ 128 satır, ara (1xx) yanıt ≤ 4; yanıt okuması toplam 15 sn
  (`Deadline`; okuma başına 5 sn zaman aşımı damla damla gönderen sunucuya yetmez).
- IPP gövdesi ayrıştırıcıdan önce `check_ipp` ile taranır: her ad/değer uzunluğu gövdede, değer
  sayısı ≤ 50 000, koleksiyon derinliği ≤ 8, koleksiyonlar dengeli. Liste en çok 1000 kuyruk ve
  1000 iş.
- Düşmanca testler: birim (`hostile_http`, `hostile_ipp`, `deadline_cuts_slow_server`; sonsuz
  parça/trailer/başlık/1xx/gövde akışları) ve sahte sunucuyla uçtan uca
  (`hostile_server_replies_are_rejected`: `Content-Length: u64::MAX`, `ffffffffffffffff` parça,
  2 MiB parça akışı, `u16::MAX` değer uzunluğu, sınır+1 öznitelik, 50 000 düzey koleksiyon).

## 8. Bilinen sınırlar

- `LPDEST`/`PRINTER` ortam değişkenleri ve `/etc/cups/lpoptions` (sistem geneli kullanıcı
  varsayılanı) okunmaz; etkin varsayılan yalnız `~/.cups/lpoptions` + sunucu.
- `lpoptions` örnekleri (`Dest ad/örnek`) listede ayrı yazıcı olarak gösterilmez; örneğe
  verilmiş `Default` korunur ama "Varsayılan" rozeti ana yazıcıda gösterilir.
- Geçici (dnssd ile otomatik bulunan, henüz kuyruk açılmamış) IPP Everywhere yazıcılar
  CUPS-Get-Printers'ta yoktur; GTK yazdırma penceresi onları ayrıca gösterir. v2 keşfiyle.
- TLS'li uzak CUPS sunucusu desteklenmez.
