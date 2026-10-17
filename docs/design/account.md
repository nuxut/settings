# Kullanıcı hesabı sayfası (t118)

Kararlar: 2026-10-13 (görev tanımı). Aşağıdaki
"geçici seçim" işaretli maddeler uygulayıcının makul varsayımıdır; "Açık sorular"da listelenir.

## Kararlar ve gerekçe

1. **AccountsService doğrudan.** Ayarlar `org.freedesktop.Accounts`'ı (sistem bus'ı) kendisi
   çağırır (`services/accounts.rs`, `services/timedate.rs` deseni): `FindUserById(uid)` (uid
   `/proc/self`'in sahibi; olmazsa `FindUserByName($USER)`) → kullanıcı nesnesi →
   `Properties.GetAll` (RealName, UserName, IconFile, AccountType). Sistem ayarıdır:
   `framework.toml`/`shell.toml`'a yazılmaz, geri al geçmişine girmez.
   - Yazma: `SetRealName(s)`, `SetIconFile(s)`. AccountsService yöntemlerinde `interactive`
     argümanı yok; çağrı iletisi **`ALLOW_INTERACTIVE_AUTHORIZATION`** bayrağıyla gider
     (`zbus::blocking::Proxy::call_with_flags`), polkit ajanı (nuxut-polkit) gerekirse sorar.
     Kendi hesabının adı/resmi için polkit varsayılanı (`change-own-user-data`) genellikle sormaz.
   - Zaman aşımı 300 sn (`CALL_TIMEOUT`, timedate ile aynı); bekleme sırasında satırın alt
     metni "Yetki bekleniyor…", öteki yazma kapalı.
   - Hata eşleme (`accounts::classify_name`): `org.freedesktop.Accounts.Error.PermissionDenied`
     ve timedate'in polkit/AccessDenied listesi → **sessiz** (önceki değer kalır; timedate ile
     aynı karar: iptal ile ret ayırt edilemez); `ServiceUnknown`… /
     `UserDoesNotExist` / bus'a bağlanılamaması → hizmet yok; diğerleri "Değiştirilemedi".
     Ayrıntı yalnız loga; ad, kullanıcı adı ve yollar loglanmaz.
   - Canlı: kullanıcı nesnesinin yolundaki `Changed` (AccountsService) ve `PropertiesChanged`
     sinyalleri yeniden okutur; sayfa açılınca ve pencere odağı dönünce de yeniden okunur
     (`accounts::recheck`, yalnızca servis bir kez başladıysa).
2. **Sayfa:** `PageId::User`, ad **"Kullanıcı hesabı"**, slug `user` (takma adlar `account`,
   `hesap`, `kullanici`, `users`), ikon `Person`, Sistem grubunda "Varsayılan uygulamalar" ile
   "Hesaplar" arasında. Gerekçe: "Hesap" tek başına, hemen altındaki takvim **"Hesaplar"**
   sayfasıyla (çevrimiçi hesaplar) karışır; GNOME da yerel hesabı "Users", çevrimiçileri
   "Online Accounts" diye ayırır. Çoğul `accounts` slug'ı takvim sayfasında kalır.
   - **Profil** kartı: büyük daire profil resmi (framework `Avatar`, `AvatarSize::Large`;
     resim yoksa addan baş harfler ve tema vurgusundan türeyen zemin), görünen ad (yoksa
     kullanıcı adı), altında "kullanıcı adı · hesap türü" (Yönetici / Standart hesap; salt
     okunur), sağda **"Değiştir…"**. Altında **Görünen ad** satırı (`TextInput`).
   - Hizmet yok / okunamadı: kartsız üst not (framework `Notice`, Warning) ve Profil kartı
     gizli (boş daire/alan gösterilmez). Okunurken kart yerinde durur.
   - Dar düzen (`rows::is_narrow`): "Değiştir…" avatarın altına, ad alanı başlığın altına iner.
3. **Görünen ad:** Enter'da ya da odak alandan çıkınca yazılır; Esc geri alır. Kırpılır, boşsa
   ya da değişmemişse yazılmaz. Sınır 64 karakter (geçici seçim; GECOS'ta pratik), `:` ve `,`
   ve denetim karakterleri reddedilir (`:` GECOS'u bozar ve AccountsService reddeder; `,`
   GECOS alt alanlarını böler) → "Ad iki nokta üst üste ya da virgül içeremez…". Yazılan ad
   yanıt gelene dek alanda kalır (iyimser); ret/iptalde servisin değerine döner.
4. **Profil resmi hattı:**
   1. "Değiştir…" → framework dosya seçicisi (`nuxut::portal`, `portal` özelliği; filtre
      "Görseller": PNG/JPEG/WebP/GIF MIME), başlangıç klasörü `~/Resimler`/`~/Pictures`. Seçici
      açıkken düğme kapalı (`OpState::Picking`); iptal sessiz; portal yoksa "Dosya seçici
      açılamadı".
   2. Seçilen dosya en çok 8 MiB okunur (normal dosya değilse okunmaz) ve **yalıtılmış işçide**
      çözülür: `nuxut_image_sandbox::Sandbox` (ADR-0007; önizleme servisiyle aynı yol, aynı
      `locate()` sırası; zaman aşımı 6 sn, ön ısıtma yok). İşçi yoksa yalnızca PNG/JPEG/WebP/GIF
      süreç içine düşer (K2-b). Hedef `256×256 Cover` → **kare merkez kırpma** →
      uzun kenar en çok 256 (büyütülmez; `accounts::square_avatar`).
   3. PNG kodlanır (`png` crate, `tEXt Software=nuxut-settings` işaretli) ve Ayarlar'ın önbellek
      dizinine atomik yazılır: `$XDG_CACHE_HOME/nuxut-settings/avatar.png` (yoksa
      `~/.cache/…`). Sonra `SetIconFile(o yol)`; AccountsService dosyayı kendi dizinine
      (`/var/lib/AccountsService/icons/<kullanıcı>`) kopyalar.
   4. Yalnızca `SetIconFile` başarılıysa **`~/.face`** güncellenir (kilit ekranı onu okur):
      aynı PNG, aynı dizinde geçici dosya + `fsync` + `rename` (atomik), izin 0644. Var olan
      `~/.face` Nuxut'un değilse (işaret yok; bağlantı ya da başka tür dosya dahil — bağlantının
      hedefi değişmez, bağlantının kendisi taşınır) önce **bir kez** `~/.face.nuxut-bak`'a
      taşınır; yedek varsa üstüne yazılmaz. `~/.face` dizinse yazılmaz. `~/.face` yazılamazsa
      satır "Profil resmi değişti, ancak kilit ekranı resmi (~/.face) yazılamadı".
   5. Gösterilen resim `IconFile`'dan, aynı yalıtımla çözülür (kullanıcı içeriği); yol ve
      değiştirilme zamanı değişmedikçe yeniden çözülmez.
5. **Parola değiştirme: v2** — bu sürümde yok (AccountsService `SetPassword` önceden şifrelenmiş
   ister; doğru akış `passwd`/PAM ile mevcut parolayı doğrulamaktır, ayrı tasarım).
6. **i18n:** sayfa metinleri `settings-user-*`, `settings-page-user`, `settings-section-profile`.
   Kaynak Türkçe; `en.ftl` değerleri GEÇİCİ (uygulayıcının İngilizcesi) — çeviri gözden geçirmesinde yeniden
   yazacak.

## Test

- Birim (servise hiç gitmez; `cfg!(test)` → thread yok): hata eşleme, ad doğrulama, `GetAll`
  ayrıştırma, kare kırpma/küçültme boyutları, PNG işareti, `~/.face` yedek-bir-kez/atomik/izin,
  bağlantı ve dizin durumları (geçici HOME), bozuk/özel dosya, sayfa metinleri ve görünürlük.
- Uçtan uca (`tests/accounts_dbus.rs`, `#[ignore]`): `examples/fake_accounts.rs` (oturum
  bus'ında sahte AccountsService; yazma `ALLOW_INTERACTIVE_AUTHORIZATION` bayraksız gelirse
  `PermissionDenied`; `NUXUT_FAKE_ACCOUNTS_DENY`, `…_DELAY_MS`, `…_ICON`, `…_ADMIN`, `…_NAME`).
  `tools/accounts-dbus-test.sh <test ikilisi> <fake_accounts> <dizin>` ortamı yalıtır ve
  kendini `dbus-run-session` içinde çalıştırır; senaryolar ayrı süreç + ayrı geçici HOME:
  hizmet yok, okuma/ad/geçersiz ad/resim hattı/`~/.face` yedeği/bozuk resim, ret sessiz.
  Gerçek sistem bus'ına ve AccountsService'e asla gidilmez (`NUXUT_ACCOUNTS_BUS=session`).
- Canlı görsel: izole sway + `dbus-run-session` + sahte servis; `NUXUT_SETTINGS_DEBUG` adımı
  `avatar=<yol>` dosya seçici seçimini taklit eder. İşçi `NUXUT_IMAGE_WORKER` ile verildiğinde
  yalıtım doğrulandı (seccomp + landlock).

## Framework talepleri

- **Daha büyük avatar boyu:** `AvatarSize::Large` 64 px (`space(16)`); profil sayfası için
  ~96 px (`XLarge`) istenebilir. Şimdilik `Large` kullanılıyor (settings'te kopya yok).
- **Dosya seçicide ebeveyn pencere:** sayfa bloklarının `cx.export_toplevel`'e erişimi yok;
  seçici `ParentWindow::none()` ile açılıyor (kipli değil). Ayarlar uygulama düzeyinde tanıtıcıyı
  tutup servislere verebilir — framework tarafında eksik yok, settings işi (sonraki tur).

## Bekleme sınırları (inceleme bulgusu, gerekçe)

- Yazma çağrıları `CALL_TIMEOUT` (300 sn) ile sınırlı; yalıtılmış çözüm 6 sn; dosya okuma
  8 MiB ve yalnız normal dosya (FIFO/aygıt okunmaz).
- `accounts` iş thread'i kanalda bekler: kanal süreç boyunca açık (boşta CPU yok), gönderen
  düşerse döngü biter. Sinyal dinleyicisi süreç ömrü boyunca yaşar (timedate ile aynı desen);
  bağlantı koparsa yineleyici biter, thread çıkar.
- Dosya seçici yanıtı kullanıcıyı bekler (diyalog kapanınca ya da portal düşünce sonuç gelir,
  framework `nuxut-portal` sözleşmesi: her istek için tam bir kez).
- Sahte servis (`examples/fake_accounts.rs`) en çok `NUXUT_FAKE_ACCOUNTS_LIFETIME_S` (600 sn)
  yaşar; betik ayrıca `trap` ile öldürür.

## Bilinen sınırlar

- 8 MiB üstü görsel (yalıtılmış işçinin girdi sınırı) açılmaz; hata metni bunu söyler.
- AccountsService yokken resim/ad hiç yazılmaz; `~/.face` de yazılmaz (yalnız başarılı
  `SetIconFile` sonrası).
- dbus-daemon'un yanıt zaman aşımı polkit diyaloğundan kısaysa çağrı genel hatayla düşer
  (dbus-broker'da sınır yok; timedate ile aynı).
- Hazır avatar galerisi (`/usr/share/pixmaps/faces`) ve kamera yok (v2 adayı).

## Uygulama soruları — kararlar (2026-10-13)

1. AccountsService yokken `~/.face` **yazılmaz**: tek doğruluk kaynağı AccountsService; yalnız
   kilit ekranının resmi değişirse giriş ekranı/diğer araçlarla tutarsızlık doğar. Sayfa not gösterir.
2. Ad sınırı 64 karakter ve `:` / `,` / denetim karakteri reddi **onaylı** (GECOS bütünlüğü).
3. Avatar boyu: framework'ten `AvatarSize::XLarge` (~96 px) **istendi**; gelene dek `Large` (64 px).
