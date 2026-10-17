## Hesap listesi

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Hesabın hiç senkronlanmamış olduğu.
settings-accounts-sync-never = henüz senkronlanmadı

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamanın az önce yapıldığı.
settings-accounts-sync-just-now = az önce senkronlandı

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamadan bu yana geçen dakika sayısı.
# Değişkenler: $count (dakika sayısı, ör. 5).
settings-accounts-sync-minutes-ago = { $count } dk önce senkronlandı

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamadan bu yana geçen saat sayısı.
# Değişkenler: $count (saat sayısı, ör. 3).
settings-accounts-sync-hours-ago = { $count } sa önce senkronlandı

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamadan bu yana geçen gün sayısı.
# Değişkenler: $count (gün sayısı, ör. 2).
settings-accounts-sync-days-ago = { $count } gün önce senkronlandı

# Nerede: Hesaplar sayfası, hesap listesi boşken maskotun altındaki başlık. Anlam: henüz hesap eklenmediği.
settings-accounts-empty-title = Henüz hesap yok

# Nerede: Hesaplar sayfası, hesap listesi boşken başlığın altındaki açıklama. Anlam: hesap eklemenin ne sağladığı.
settings-accounts-empty-body = Takvimlerinizi bar'da ve bildirimlerde görmek için bir hesap ekleyin.

# Nerede: Hesaplar sayfası, takvim servisine ulaşılamadığında maskotun altındaki başlık; altında hatanın kendisi yazar. Anlam: hesaplar şu an gösterilemiyor ve değiştirilemiyor.
settings-accounts-unavailable-title = Takvim hesapları şu an yönetilemiyor

# Nerede: Hesaplar sayfası, listenin altındaki dolu düğme. Anlam: Sağlayıcı seçim ekranını açar.
settings-accounts-add = Hesap ekle

# Nerede: Hesaplar sayfası, hesap kartının alt satırı. Anlam: Sağlayıcı adı ile giriş e-postasının birleşimi; parça parça kurulduğu için tek mesaj.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google"), $login (giriş e-postası, ör. "ad@gmail.com").
settings-accounts-card-sub = { $provider } · { $login }

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Hesabın bağlantısının koptuğu, yeniden bağlanması gerektiği.
settings-accounts-state-needs-reconnect = Yeniden bağlanması gerekiyor

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Hesabın kapalı (devre dışı) olduğu.
settings-accounts-state-disabled = Kapalı

# Nerede: Hesaplar sayfası, hesap kartının durum satırı ("Şimdi senkronla" sırasında da). Anlam: Senkronlamanın sürdüğü.
settings-accounts-state-syncing = Senkronlanıyor…

# Nerede: Hesaplar sayfası, hesap kartının durum satırı (src/pages/accounts.rs). Anlam: Son senkronlamanın başarısız olduğu; daemon bir hata kodu verdi ama ileti yok.
settings-accounts-sync-failed = Son senkron başarısız oldu

# Nerede: Hesaplar sayfası, bağlantısı kopan hesap kartındaki dolu düğme. Anlam: Yeniden bağlanma akışını başlatır.
settings-accounts-button-reconnect = Yeniden bağlan

# Nerede: Hesaplar sayfası, adı olmayan takvimlerin satırındaki ad. Anlam: Takvimin adı boşsa kullanılan genel ad.
settings-accounts-calendar-default-name = Takvim

# Nerede: Hesaplar sayfası, takvim satırının renk seçicisi açılınca görünen metin düğme. Anlam: Takvimin rengini sağlayıcının belirlediği renge döndürür.
settings-accounts-calendar-color-reset = Sağlayıcının rengi

# Nerede: Hesaplar sayfası, hesap kartındaki sil ikonuna ilk basışta durum satırında çıkan metin. Anlam: Kaldırmanın ikinci bir tık daha istediği.
settings-accounts-remove-confirm = Kaldırmak için yeniden tıklayın

# Nerede: Hesaplar sayfası, hesap listesi okunurken görünen metin. Anlam: Hesapların yüklenmekte olduğu.
settings-accounts-loading = Hesaplar yükleniyor…

# Nerede: Hesaplar sayfası, hesap kartının çizimdeki birleşik alt satırı. Anlam: Alt bilgi ile durumun tek satırda birleşimi; parça parça kurulduğu için tek mesaj.
# Değişkenler: $sub (hesabın alt bilgisi, ör. "Google · ad@gmail.com"), $status (hesap durumu, ör. "Senkronlanıyor…").
settings-accounts-card-sub-status = { $sub } · { $status }

## Sağlayıcı seçimi

# Nerede: Hesaplar sayfası, "Hesap ekle" ekranındaki Google kartının alt satırı. Anlam: Google hesabının kapsadığı hizmetler.
settings-accounts-provider-google-description = Gmail ve Google Takvim

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki Microsoft kartının alt satırı. Anlam: Microsoft hesabının kapsadığı hizmetler.
settings-accounts-provider-microsoft-description = Outlook, Hotmail, Microsoft 365

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki Nextcloud kartının alt satırı. Anlam: Kendi sunucusunu kullananların onayı tarayıcıda yapılır.
settings-accounts-provider-nextcloud-description = Kendi sunucunuz — tarayıcıda onaylayın

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki CalDAV kartının başlığı; .description kartın altındaki açıklama. Anlam: CalDAV uyumlu diğer sağlayıcılar.
settings-accounts-provider-caldav = iCloud, Fastmail ve diğerleri
    .description = CalDAV — e-posta ve uygulama şifresi

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki abonelik kartının başlığı; .description kartın altındaki açıklama. Anlam: Salt okunur webcal/.ics takvim aboneliği.
settings-accounts-provider-subscription = Takvim aboneliği
    .description = webcal:// ya da .ics adresi (salt okunur)

# Nerede: Hesaplar sayfası, bilinmeyen sağlayıcıya düşen kart ve hesapların başlığı. Anlam: Sağlayıcının tanınamadığı.
settings-accounts-provider-unknown = Bilinmeyen

## Hesap ekleme akışı

# Nerede: Hesaplar sayfası, sağlayıcı ve akış ekranlarının sol üstündeki metin düğme. Anlam: Önceki ekrana döner.
settings-accounts-back = Geri

# Nerede: Hesaplar sayfası, yüklenemediğinde görünen ve akışın hata ekranındaki düğme. Anlam: İsteği yeniden dener.
settings-accounts-retry = Tekrar dene

# Nerede: Akış ekranının altındaki birincil düğme. Anlam: Girdiyi gönderip sonraki adıma geçer.
settings-accounts-continue = Devam

# Nerede: Akış ekranının altındaki metin düğme. Anlam: Akışı iptal eder.
settings-accounts-cancel = İptal

# Nerede: Akış ekranının altındaki tonlu düğme (tarayıcı ve uygulama şifresi adımlarında). Anlam: Tarayıcı sayfasını ya da şifre oluşturma sayfasını yeniden açar.
settings-accounts-reopen-link = Bağlantıyı yeniden aç

# Nerede: Akışın Nextcloud sunucu adımındaki soru. Anlam: Sunucu adresi isteniyor.
settings-accounts-flow-server-question = Nextcloud sunucunuzun adresi nedir?

# Nerede: Akışın Nextcloud sunucu adımındaki ipucu. Anlam: Adrese örnek verir.
settings-accounts-flow-server-hint = Örneğin bulut.ornek.com — tarayıcınızda giriş yaptığınız adres.

# Nerede: Akışın Nextcloud sunucu adımındaki giriş alanının yer tutucusu. Anlam: Örnek sunucu adresi.
settings-accounts-flow-server-placeholder = bulut.ornek.com

# Nerede: Akışın e-posta adımındaki soru. Anlam: Hesabın e-posta adresi isteniyor.
settings-accounts-flow-email-question = E-posta adresiniz nedir?

# Nerede: Akışın e-posta adımındaki ipucu. Anlam: Sunucunun kendiliğinden bulunacağı.
settings-accounts-flow-email-hint = Sunucuyu kendimiz bulacağız.

# Nerede: Akışın e-posta adımındaki giriş alanının yer tutucusu. Anlam: Örnek e-posta adresi.
settings-accounts-flow-email-placeholder = ad@ornek.com

# Nerede: Akışın uygulama şifresi adımındaki soru. Anlam: Sağlayıcıya özel uygulama şifresi isteniyor.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google").
settings-accounts-flow-password-question = { $provider } uygulama şifresi

# Nerede: Akışın uygulama şifresi adımındaki ipucu. Anlam: Uygulama şifresi ile hesap şifresinin farkını anlatır.
settings-accounts-flow-password-hint = Hesap şifreniz değil: sağlayıcının güvenlik sayfasında bu cihaz için bir uygulama şifresi oluşturun ve buraya yapıştırın.

# Nerede: Akışın uygulama şifresi adımındaki parola alanının yer tutucusu. Anlam: Uygulama şifresinin beklenen biçimini gösterir.
settings-accounts-flow-password-placeholder = xxxx-xxxx-xxxx-xxxx

# Nerede: Akışın uygulama şifresi adımında, sunucu bilinmiyorsa çıkan ikinci alanın yer tutucusu. Anlam: CalDAV sunucu adresi isteniyor.
settings-accounts-flow-password-server-placeholder = Sunucu adresi (caldav.ornek.com)

# Nerede: Akışın uygulama şifresi adımındaki tonlu düğme. Anlam: Sağlayıcının uygulama şifresi sayfasını açar.
settings-accounts-flow-password-link = Uygulama şifresi oluştur

# Nerede: Akışın takvim adresi (abonelik) adımındaki soru. Anlam: Abone olunacak takvimin adresi isteniyor.
settings-accounts-flow-url-question = Takvimin adresi nedir?

# Nerede: Akışın takvim adresi adımındaki ipucu. Anlam: Kabul edilen adres biçimini anlatır.
settings-accounts-flow-url-hint = webcal:// ya da https:// ile başlayan .ics bağlantısı.

# Nerede: Akışın takvim adresi adımındaki giriş alanının yer tutucusu. Anlam: Örnek adres biçimi.
settings-accounts-flow-url-placeholder = webcal://…

# Nerede: Akışın tarayıcı onayı adımındaki soru. Anlam: Kullanıcıdan tarayıcıya geçmesi isteniyor.
settings-accounts-flow-browser-question = Tarayıcıda devam edin

# Nerede: Akışın tarayıcı onayı adımındaki ipucu. Anlam: Tarayıcıda açılan sayfada ne yapılacağı ve pencerenin kendiliğinden ilerleyeceği anlatılır.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google").
settings-accounts-flow-browser-hint = { $provider } için tarayıcınızda bir sayfa açtık. Giriş yapıp izin verin; bu pencere kendiliğinden ilerleyecek.

# Nerede: Akışın sunucu onayı adımındaki soru. Anlam: Yönlendirilen sunucuyla devam edilip edilmeyeceği soruluyor.
settings-accounts-flow-confirm-server-question = Bu sunucuya devam edilsin mi?

# Nerede: Akışın sunucu onayı adımındaki ipucu. Anlam: Yönlendirme ve şifrenin henüz gönderilmediği anlatılır.
settings-accounts-flow-confirm-server-hint = Sunucu sizi başka bir adrese yönlendirdi. Şifreniz henüz gönderilmedi; adres tanıdıksa devam edin.

# Nerede: Akışın sunucu onayı adımındaki birincil düğme. Anlam: Onayı gönderir.
settings-accounts-flow-confirm-yes = Evet, devam et

# Nerede: Akışın abonelik onayı adımındaki soru. Anlam: Adresi verilen takvime abone olunup olunmayacağı soruluyor; $host sunucu adresi ya da "Bu" olur.
# Değişkenler: $host (sunucu adresi, ör. "bulut.ornek.com").
settings-accounts-flow-subscribe-question = { $host } takvimine abone olunsun mu?

# Nerede: Akışın abonelik onayı adımındaki soru, sunucu adresi bilinmiyorken (src/pages/accounts.rs). Anlam: Adresi bilinmeyen takvime abone olunup olunmayacağı soruluyor.
settings-accounts-flow-subscribe-question-unnamed = Bu takvime abone olunsun mu?

# Nerede: Akışın abonelik onayı adımındaki ipucu. Anlam: Aboneliğin salt okunur olduğu anlatılır.
settings-accounts-flow-subscribe-hint = Salt okunur abonelik: etkinlikler bar'daki takvimde görünür, buradan değiştirilemez.

# Nerede: Akışın abonelik onayı adımındaki birincil düğme. Anlam: Aboneliği başlatır.
settings-accounts-flow-subscribe-confirm = Abone ol

# Nerede: Akışın bağlanma adımındaki soru. Anlam: Bağlantı kuruluyor.
settings-accounts-flow-connecting = Bağlanılıyor…

# Nerede: Akışın bağlanma adımındaki ipucu. Anlam: Ne yapıldığını söyler.
settings-accounts-flow-connecting-hint = Hesabınız doğrulanıyor.

# Nerede: Akışın keşif adımındaki soru. Anlam: Takvimler aranıyor.
settings-accounts-flow-discovering = Takvimler bulunuyor…

# Nerede: Akışın keşif adımındaki ipucu (takvim bulununca). Anlam: Bulunan takvim sayısı.
# Değişkenler: $count (bulunan takvim sayısı, ör. 3).
settings-accounts-flow-found = { $count } takvim bulundu.

# Nerede: Akışın keşif adımındaki ipucu (henüz sonuç yokken). Anlam: İşlemin birkaç saniye sürebileceği.
settings-accounts-flow-discovering-hint = Bu birkaç saniye sürebilir.

# Nerede: Akışın bitiş adımındaki soru. Anlam: Akışın tamamlandığı.
settings-accounts-flow-done = Hazır!

# Nerede: Akışın bitiş adımındaki ipucu. Anlam: Eklenen takvim sayısı ve nerede görüneceği.
# Değişkenler: $count (eklenen takvim sayısı, ör. 2).
settings-accounts-flow-done-hint = { $count } takvim eklendi. Bar'daki takvimde görünecek.

# Nerede: Akışın bitiş adımındaki birincil düğme. Anlam: Akışı kapatır.
settings-accounts-flow-done-button = Bitti

# Nerede: Akışın hata adımındaki soru (başarısızlık başlığı). Anlam: İşlemin başarısız olduğu.
settings-accounts-flow-failed = Olmadı

# Nerede: Akışın hata adımındaki hata satırı (akışın kendi mesajı yoksa). Anlam: Genel hata bildirimi.
settings-accounts-flow-error-generic = Bir sorun oluştu.

# Nerede: Hesap kartındaki yeniden bağlanma akışı başlarken soru satırında çıkan metin. Anlam: Akışın başlatılmakta olduğu.
settings-accounts-flow-starting = Başlatılıyor…

# Nerede: Sağlayıcı seçilince akış ekranında soru satırında çıkan metin. Anlam: Seçilen sağlayıcının hesap ekleme akışı başlatılıyor.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google").
settings-accounts-flow-adding = { $provider } hesabı ekleniyor…

## Hakkında sayfası

# Nerede: Hakkında sayfası, düzenlenen dosya listesinde framework dosyasının satır başlığı. Anlam: Görünüm ve hareket ayarlarını içeren dosya.
settings-about-file-look = Görünüm ve hareket

# Nerede: Hakkında sayfasındaki dosya satırının başlığı (src/pages/about.rs). Anlam: shell.toml dosyasının görünen adı.
settings-about-file-shell = Shell

# Nerede: Hakkında sayfası, düzenlenen dosya listesinde üretilmiş palet dosyasının satır başlığı. Anlam: Duvar kağıdından üretilen renk şemasını içeren dosya.
settings-about-file-palette = Üretilmiş renk şeması

# Nerede: Hakkında sayfası, her dosya satırının sağındaki metin düğme. Anlam: Dosyanın klasörünü dosya yöneticisinde açar.
settings-about-open-folder = Klasörü aç

# Nerede: Hakkında sayfası, başlığın altındaki sürüm satırı. Anlam: Uygulamanın sürümü.
# Değişkenler: $version (sürüm numarası, ör. "0.4.2").
settings-about-version = Sürüm { $version }

# Nerede: Hakkında sayfası, başlığın altındaki tanıtım paragrafı. Anlam: Değişikliklerin anında uygulandığı, dosyalara yalnızca değişen satırın yazıldığı ve geçmişten geri alınabildiği anlatılır.
settings-about-blurb = Her değişiklik anında uygulanır ve dosyalara yalnızca değişen satır olarak yazılır; yorumlarınız ve düzeniniz korunur. Son değişiklikler başlık çubuğundaki geçmiş düğmesinden tek tek geri alınabilir.

# Nerede: Hakkında sayfasının başlığında "Nuxut" adının altında ve Hoş geldin penceresinin karşılama ekranında logonun altında. Anlam: Nuxut marka sloganı (Kara Sağan logosuyla birlikte). Marka sloganıdır; her dilde anlamı koruyan doğal bir çeviri yazılır, birebir çeviri şart değildir. "The penguin" tekil ve belirlidir; Linux maskotu Tux'a göndermedir, bu gönderme korunmalıdır.
settings-brand-slogan = Penguen uçamaz. Bu uçuyor.

## Yer tutucu satır

# Nerede: Kenar çubuğunda henüz yapılmamış sayfaların yer tutucu satırındaki not. Anlam: Sayfanın hazırlanmakta olduğu; $subtitle sayfanın alt başlığıdır.
# Değişkenler: $subtitle (sayfanın alt başlığı, ör. "Ses").
settings-placeholder-note = { $subtitle } — hazırlanıyor

## Önizleme

# Nerede: Görünüm sayfasındaki canlı önizlemede sahte bildirimin başlığı. Anlam: Panel içindeki örnek bildirim içeriği.
# [max: 12]
settings-preview-demo-title = Akşam yemeği

# Nerede: Görünüm sayfasındaki canlı önizlemede sahte bildirimin gövdesi. Anlam: Panel içindeki örnek bildirim içeriği.
settings-preview-demo-body = Saat sekizde Kadıköy'de

# Nerede: Görünüm sayfasındaki canlı önizlemedeki örnek düğme. Anlam: Panel içindeki düğme örneği.
# [max: 6]
settings-preview-demo-button = Tamam
## Tuval ve monitör seçici

# Nerede: Ekranlar sayfası, tuvaldeki monitör kutuları ve çoklu monitörde monitör seçme sekmeleri. Anlam: Monitörün kullanıcıya görünen adı; aynı adlı monitörlerde ayırt edici olarak bağlayıcı eklenir.
# Değişkenler: $name (monitör adı, ör. "LP27q"), $connector (bağlantı çıkışı, ör. "DP-1").
settings-displays-name-connector = { $name } ({ $connector })

# Nerede: Ekranlar sayfası, monitör sekmelerinin hemen altındaki ipucu satırı. Anlam: Monitörün bağlı olduğu çıkışa göre tanındığı; başka bir çıkışa takılırsa ayarlarının ayrı tutulacağı.
# Değişkenler: $connector (bağlantı çıkışı, ör. "DP-1").
settings-displays-hint-connector = Bu monitör bağlı olduğu çıkışa ({ $connector }) göre tanınıyor; başka çıkışa takılırsa ayarları ayrı tutulur.

## Monitör ayar satırları

# Nerede: Ekranlar sayfası, seçili monitörün ilk ayar satırı; .description satırın altındaki açıklama. Anlam: Bu monitörü açıp kapatma.
settings-displays-enabled = Bu ekranı kullan
    .description = Kapalı ekran görüntü almaz

# Nerede: Ekranlar sayfası, "Yansıt" satırındaki seçim menüsünün ilk seçeneği. Anlam: Bu ekran hiçbir ekrana yansıtmıyor.
settings-choice-mirror-none = Yansıtma yok

# Nerede: Ekranlar sayfası, yansıtma ayar satırı; .description satırın altındaki açıklama. Anlam: Seçilen başka ekrana yansıtma.
settings-displays-mirror = Yansıt
    .description = Bu ekran seçilen ekranın aynısını gösterir

# Nerede: Ekranlar sayfası, birincil ekran anahtarı; .description satırın altındaki açıklama. Anlam: Birincil ekranın görevi.
settings-displays-primary = Birincil ekran
    .description = Oturum açılışında imleç ve bildirimler burada

# Nerede: Ekranlar sayfası, çözünürlük seçim menüsünde önerilen kipin etiketi. Anlam: Ekranın önerdiği çözünürlük.
# Değişkenler: $resolution (çözünürlük, ör. "1920 × 1080").
settings-choice-resolution-preferred = { $resolution } (önerilen)

# Nerede: Ekranlar sayfası, çözünürlük ayar satırı. Anlam: Ekran çözünürlüğü seçimi.
settings-displays-resolution = Çözünürlük

# Nerede: Ekranlar sayfası, yenileme hızı ayar satırı. Anlam: Ekranın saniyedeki kare yenileme sayısı.
settings-displays-rate = Yenileme hızı

# Nerede: Ekranlar sayfası, yenileme hızı seçim menüsündeki seçeneklerin biçimi. Anlam: Hertz cinsinden yenileme hızı.
# Değişkenler: $value (yenileme hızı değeri, ör. 60).
settings-choice-rate-hz = { $value } Hz

# Nerede: Ekranlar sayfası, ölçek ayar satırı; .description satırın altındaki açıklama. Anlam: Arayüz ölçekleme.
settings-displays-scale = Ölçek
    .description = Yazı ve pencerelerin büyüklüğü

# Nerede: Ekranlar sayfası, ölçek satırındaki çiplerin biçimi. Anlam: Yüzde olarak ölçek.
# Değişkenler: $value (ölçek yüzdesi, ör. 100).
settings-choice-scale-percent = %{ $value }

# Nerede: Ekranlar sayfası, döndürme ayar satırı. Anlam: Ekran görüntüsünün döndürülmesi.
settings-displays-rotation = Döndürme

# Nerede: Ekranlar sayfası, döndürme seçim düğmelerinde döndürülmemiş durum. Anlam: Döndürme yok.
settings-choice-rotation-none = Yok

# Nerede: Ekranlar sayfası, yatay çevirme anahtarı; .description satırın altındaki açıklama. Anlam: Görüntünün aynalanması.
settings-displays-flip = Yatay çevir
    .description = Görüntüyü aynalar (ör. arkadan yansıtmalı projektör)

# Nerede: Ekranlar sayfası, VRR ayar satırı; .description satırın altındaki açıklama. Anlam: Değişken yenileme hızı (FreeSync, G-Sync).
settings-displays-vrr = Değişken yenileme hızı
    .description = Destekleyen ekranlarda takılmayı azaltır (FreeSync, G-Sync)

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR kapalı.
settings-choice-vrr-off = Kapalı

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR her zaman açık.
settings-choice-vrr-on = Her zaman

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR yalnızca tam ekran uygulamalarda açık.
settings-choice-vrr-fullscreen = Tam ekranda

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR tam ekran oyun ve videolarda açık.
settings-choice-vrr-fullscreen-media = Tam ekran oyun ve video

## Doğrulama uyarıları

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi (uygulamadan önce). Anlam: En az bir ekranın açık kalması gerektiği.
settings-displays-issue-no-active = En az bir ekran açık ve kendi görüntüsünü gösteriyor olmalı.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: İki monitörün yerleşimde üst üste biniyor olduğu.
# Değişkenler: $first (binen ilk monitörün adı, ör. "LP27q"), $second (binen ikinci monitörün adı, ör. "DP-1").
settings-displays-issue-overlap = { $first } ile { $second } üst üste biniyor.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: Yerleşimin tek parça olmadığı, monitörler arasında boşluk kaldığı.
settings-displays-issue-disconnected = Ekranlar birbirine değmeli; fare aradaki boşluktan geçemez.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: Seçilen ölçeğin bu monitörün çözünürlüğünü tam bölmediği.
# Değişkenler: $display (monitörün adı, ör. "LP27q").
settings-displays-issue-invalid-scale = { $display } için bu ölçek çözünürlüğü tam bölmüyor; listeden başka bir ölçek seçin.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: Yansıtılan monitörün kapalı ya da kendisi yansıtma olduğu.
# Değişkenler: $display (monitörün adı, ör. "LP27q").
settings-displays-issue-bad-mirror = { $display } kapalı ya da yansıtan bir ekranı yansıtıyor.

## Durum çubuğu

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Değişiklik shell'e gönderildi, uygulanıyor.
settings-displays-status-applying = Uygulanıyor…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (onay süresinin son saniyeleri). Anlam: Kullanıcıya yeni ayarların korunup korunmayacağı soruluyor.
settings-displays-status-awaiting = Yeni ayarlar korunsun mu?

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (onay geri sayımı). Anlam: Ayarların korunması için kalan süre; süre dolunca kendiliğinden geri alınacak.
# Değişkenler: $seconds (kalan saniye, ör. 10).
settings-displays-status-awaiting-countdown = Yeni ayarlar korunsun mu? { $seconds } sn içinde geri alınacak.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Kullanıcı "Koru" dedi, kalıcı yazım sürüyor.
settings-displays-status-keeping = Korunuyor…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Kullanıcı "Geri al" dedi, önceki ayarlara dönülüyor.
settings-displays-status-reverting = Geri alınıyor…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Karar gönderildi, shell'den sonuç bekleniyor.
settings-displays-status-resolving = Sonuç bekleniyor…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Monitörler değiştiği (takılıp çıktığı) için düzenleme yeni duruma göre sıfırlandı.
settings-displays-status-reset = Monitörler değişti; düzenleme yeni duruma göre sıfırlandı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Kullanıcının henüz uygulamadığı değişiklikler var.
settings-displays-status-unsaved = Uygulanmamış değişiklikler var.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (boşta). Anlam: Görüntüyü değiştiren ayarların kısa sürede korunmazsa kendiliğinden geri alındığı.
settings-displays-status-hint = Görüntüyü değiştiren ayarlar, kısa süre içinde korunmazsa kendiliğinden geri alınır.

## Uygulama sonuçları

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Değişiklik uygulandı.
settings-displays-result-applied = Uygulandı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Kullanıcı korudu, yeni ayarlar kalıcı.
settings-displays-result-confirmed = Yeni ekran ayarları korundu.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Değişiklik geri alındı, önceki ayarlar geçerli.
settings-displays-result-reverted = Geri alındı; önceki ayarlar geçerli.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Onay süresi doldu, önceki ayarlara dönüldü.
settings-displays-result-timeout = Süre doldu, onaylanmadığı için önceki ayarlara dönüldü.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Monitör takılıp çıkarken değişiklik geri döndü.
settings-displays-result-hotplug = Bir ekran takıldı ya da çıkarıldı; önceki ayarlara dönüldü.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu ve not_applied hatası. Anlam: Ekran bu ayarı kabul etmedi, önceki ayarlara dönüldü.
settings-displays-result-not-applied = Ekran bu ayarı uygulamadı; önceki ayarlara dönüldü.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Değişiklik gösterilemedi, önceki ayarlara dönüldü.
settings-displays-result-not-shown = Değişiklik gösterilemedi; önceki ayarlara dönüldü.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Hyprland ayarları kendisi yeniden yüklediği için değişiklik geri alındı.
settings-displays-result-reloaded = Hyprland ayarları yeniden yüklendi; değişiklik geri alındı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in tanınmayan son işlem sonucu. Anlam: İşlem sonuçlandı, güncel durum sayfada görülüyor.
settings-displays-result-other = Değişiklik sonuçlandı; güncel durum aşağıda.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; sonuç cümlesinin sonuna shell'in ek açıklaması eklenirken. Anlam: Sonuç cümlesi ve parantez içinde ek açıklama.
# Değişkenler: $text (sonuç cümlesi, ör. "Uygulandı."), $detail (shell'in ek açıklaması, ör. "ekran kilitli").
settings-displays-result-detail = { $text } ({ $detail })

## Hata mesajları

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Uygulama sırasında ekranlar değişti, liste yenilendi.
settings-displays-error-stale = Ekranlar bu arada değişti; liste yenilendi, yeniden deneyin.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ekran sayısı çok fazla olduğu için istek gönderilemedi.
settings-displays-error-too-large = Çok fazla ekran var; istek gönderilemedi.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ayarlar ile shell arasındaki anlaşamazlık; sürümlerin güncel olması gerektiği.
settings-displays-error-protocol = Ayarlar ile shell anlaşamadı; ikisinin de güncel olduğundan emin olun.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Kastedilen monitörün artık bağlı olmadığı, liste yenilendi.
# Değişkenler: $display (monitörün adı; ad yoksa "Bir ekran", ör. "LP27q").
settings-displays-error-unknown-display = { $display } artık bağlı değil; liste yenilendi.

# Nerede: Ekranlar sayfası, hata cümlelerinde monitör adı bulunamadığında özne olarak kullanılan parça. Anlam: Belirsiz tek ekran.
settings-displays-error-subject-indefinite = Bir ekran

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Listeye yeni bir monitör eklendi, liste yenilendi.
settings-displays-error-missing-display = Yeni bir ekran bağlandı; liste yenilendi, yeniden deneyin.

# Nerede: Ekranlar sayfası, hata cümlelerinde monitör adı bulunamadığında özne olarak kullanılan parça. Anlam: Kastedilen ekran.
settings-displays-error-subject-definite = Ekran

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Monitörün istediği çözünürlük ya da yenileme hızının desteklenmediği.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q").
settings-displays-error-invalid-mode = { $display }: bu çözünürlük ya da yenileme hızı desteklenmiyor.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda); shell en yakın uygun ölçeği önerdiğinde. Anlam: Ölçek çözünürlüğü bölüyor, uygulanabilir ölçek öneriliyor.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q"), $scale (önerilen ölçek, önceden yüzde olarak biçimli, ör. "%125").
settings-displays-error-invalid-scale-hint = { $display }: bu ölçek çözünürlüğü tam bölmüyor; en yakın uygun ölçek { $scale }.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ölçeğin bu monitörün çözünürlüğünü tam bölmediği.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q").
settings-displays-error-invalid-scale = { $display }: bu ölçek çözünürlüğü tam bölmüyor.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Monitör için istenen konumun geçersiz olduğu.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q").
settings-displays-error-invalid-position = { $display }: konum geçersiz.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Yerleşimde monitörlerin üst üste biniyor olduğu.
settings-displays-error-overlap = Ekranlar üst üste biniyor.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: En az bir ekranın açık kalması gerektiği.
settings-displays-error-no-active-display = En az bir ekran açık kalmalı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Yansıtılan bir ekranın kendisi başkasına yansıtamayacağı.
settings-displays-error-mirror-chain = Yansıtılan ekran kendisi başka bir ekranı yansıtamaz.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Birincil ekranın açık ve yansıtma yapmayan bir ekran olması gerektiği.
settings-displays-error-invalid-primary = Birincil ekran açık olmalı ve yansıtma yapmamalı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Önceki değişikliğin onayı hâlâ sürdüğü için yenisinin gönderilemediği.
settings-displays-error-pending-exists = Önceki değişiklik hâlâ onay bekliyor.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell'in şu an ekranları yönetemediği.
settings-displays-error-unavailable = Shell şu an ekranları yönetemiyor.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ekran kilitliyken değişiklik uygulanamayacağı.
settings-displays-error-locked = Ekran kilitliyken uygulanamaz.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell'e ulaşılamadığı, onaylanmamış değişikliğin kendiliğinden geri alınacağı.
settings-displays-error-not-running = Shell'e ulaşılamıyor; onaylanmamış değişiklik kendiliğinden geri alınır.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell'in zamanında cevap vermediği, değişikliğin uygulanmadığı.
settings-displays-error-timeout = Shell zamanında cevap vermedi; değişiklik uygulanmadı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell ile bağlantının kurulamadığı.
settings-displays-error-ipc = Shell ile bağlantı kurulamadı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda); kodu ve mesajı olmayan hata. Anlam: Uygulamanın başarısız olduğu.
settings-displays-error-unknown = Uygulanamadı.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda); tanınmayan hata kodu. Anlam: Uygulamanın başarısız olduğu, kod parantez içinde.
# Değişkenler: $code (shell hata kodu, ör. "stale").
settings-displays-error-code = Uygulanamadı ({ $code }).

## Düğmeler

# Nerede: Ekranlar sayfası, alt çubuğun sağdaki düğme çiftinden soluncası. Anlam: Bekleyen düzenlemeden vazgeçme.
settings-displays-button-cancel = Vazgeç

# Nerede: Ekranlar sayfası, alt çubuğun sağdaki düğme çiftinden sağuncası. Anlam: Değişiklikleri shell'e gönderme.
settings-displays-button-apply = Uygula

# Nerede: Ekranlar sayfası, alt çubuk; onay beklerken Uygula düğmesinin yerini alan düğme. Anlam: Yeni ayarları koruma.
settings-displays-button-keep = Koru

# Nerede: Ekranlar sayfası, alt çubuk; onay beklerken Vazgeç düğmesinin yerini alan düğme. Anlam: Değişikliği geri alma.
settings-displays-button-revert = Geri al

## Boş durum ve yükleme

# Nerede: Ekranlar sayfası, sayfa ilk açıldığında liste okunurken görünen satır. Anlam: Ekran listesinin alınıyor olduğu.
settings-displays-loading = Ekranlar okunuyor…

# Nerede: Ekranlar sayfası, boş durum başlığı (shell çalışmıyorken). Anlam: Nuxut shell'inin çalışmadığı.
settings-displays-empty-not-running-title = Nuxut shell çalışmıyor

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell çalışmıyorken). Anlam: Ekran ayarlarını shell'in uyguladığı, shell açılınca sayfanın dolacağı.
settings-displays-empty-not-running-body = Ekran ayarlarını shell uygular. Shell açıldığında bu sayfa kendiliğinden dolar.

# Nerede: Ekranlar sayfası, boş durum başlığı (shell sürümü desteklemiyorken). Anlam: Bu shell sürümünün ekran ayarlarını desteklemediği.
settings-displays-empty-unsupported-title = Bu shell sürümü ekran ayarlarını desteklemiyor

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell sürümü desteklemiyorken). Anlam: Shell güncellenince monitörlerin burada görüneceği.
settings-displays-empty-unsupported-body = Nuxut shell güncellendiğinde monitörler burada görünecek.

# Nerede: Ekranlar sayfası, boş durum başlığı (shell var ama ekranları yönetemiyorken); gövde metni shell'den gelir. Anlam: Ekranların şu an yönetilemediği.
settings-displays-empty-error-title = Ekranlar şu an yönetilemiyor

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell örneği Hyprland'e erişemiyorken). Anlam: Shell örneğinin ekran ayarlarını değiştiremediği.
settings-displays-empty-not-available = Bu shell örneği ekran ayarlarını değiştiremiyor (Hyprland'e erişimi yok).

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell zamanında cevap vermeyince). Anlam: Shell'in zamanında cevap vermediği.
settings-displays-empty-timeout = Shell zamanında cevap vermedi.

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell örneği adı geçersizken). Anlam: Örnek adı ortam değişkeninin geçersiz olduğu; kurallar parantez içinde.
# Değişkenler: $env (ortam değişkeninin adı, ör. "NUXUT_SHELL_INSTANCE").
settings-displays-error-instance-name = { $env } geçersiz ([a-z0-9_-], en çok 16)
## Açılır satır özeti

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Satırdaki toplam ayar sayısı.
# Değişkenler: $total (toplam ayar sayısı, ör. "4").
settings-disclosure-summary-settings = { $total } ayar

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Toplam ayar sayısı ve kaçı elle ayarlı.
# Değişkenler: $total (toplam ayar sayısı, ör. "4"), $changed (elle ayarlı sayısı, ör. "3").
settings-disclosure-summary-settings-changed = { $total } ayar · { $changed } tanesi elle ayarlı

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Satırdaki toplam renk sayısı.
# Değişkenler: $total (toplam renk sayısı, ör. "11").
settings-disclosure-summary-colors = { $total } renk

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Toplam renk sayısı ve kaçı elle ayarlı.
# Değişkenler: $total (toplam renk sayısı, ör. "11"), $changed (elle ayarlı renk sayısı, ör. "2").
settings-disclosure-summary-colors-changed = { $total } renk · { $changed } tanesi elle ayarlı

## Widget listeleri

# Nerede: Bar sayfası, bar bölüm düzenleyicide sıralama/ekleme/çıkarma yapılınca geçmiş (geri al) kaydına verilen etiket. Anlam: Barın üç bölümündeki widget listesinin düzenlendiği.
settings-sections-bar-widgets = Bar widget'ları

# Nerede: Paneller sayfası, kenar panelinin kart listesi değişince geçmiş kaydına verilen etiket. Anlam: Paneldeki kart listesinin düzenlendiği.
settings-sections-panel-cards = Panel kartları

# Nerede: Bar sayfası ve Paneller sayfası, liste sütunlarının altındaki "+ Ekle" düğmesi; Bildirim uygulamaları listesindeki düğme de aynı metni kullanır. Anlam: Listeye yeni öğe ekleme.
settings-sections-add = Ekle

# Nerede: Boş widget kuyusundaki ipucu (src/pages/sections.rs). Anlam: Bar bölümünde gösterilecek widget olmadığı.
settings-sections-empty-bar = Bu bölüm boş

# Nerede: Boş widget kuyusundaki ipucu (src/pages/sections.rs). Anlam: Panelde kart yokken kartların Ekle düğmesiyle eklendiği.
settings-sections-empty-panel = Bu panelde kart yok; Ekle ile kart eklenir

# Nerede: Widget ekleme modundaki çıkış düğmesi (src/pages/sections.rs). Anlam: Düğmenin görevi: ekleme modundan vazgeçmek.
settings-sections-cancel = Vazgeç

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar dikeyken üst, yatayken sol sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Üst yönü.
settings-edge-top = Üst

# Nerede: Bar bölüm düzenleyicinin orta sütun başlığı (bar yatayken ve dikeyken). Anlam: Barın ortasındaki bölüm.
settings-sections-center = Orta

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar dikeyken alt, yatayken sağ sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Alt yönü.
settings-edge-bottom = Alt

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar yatayken sol sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Sol yönü.
settings-edge-left = Sol

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar yatayken sağ sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Sağ yönü.
settings-edge-right = Sağ

# Nerede: Bar sayfası, "Ekle" akımında eklenecek widget kalmadığında görünen satır. Anlam: Tüm widget'ların zaten barın bölümlerinde olduğu.
settings-sections-all-in-bar = Tüm widget'lar bar'da.

# Nerede: Paneller sayfası, "Ekle" akımında eklenecek kart kalmadığında görünen satır. Anlam: Tüm kartların bu panelde olduğu.
settings-sections-all-in-panel = Tüm kartlar bu panelde.

# Nerede: Bar sayfası ve Paneller sayfası, "Ekle" akımında widget kataloğu henüz gelmediğinde görünen satır. Anlam: Widget listesinin shell'den alınıyor olduğu.
settings-sections-loading = Widget listesi shell'den alınıyor…

## Widget listesi hataları

# Nerede: Bar sayfası ve Paneller sayfası, "Ekle" listesinde shell kataloğu yüklenemeyince görünen hata metni. Anlam: nuxut-shell'in bulunamadığı ve nerede arandığı.
settings-sections-error-not-found = Widget listesi alınamadı: nuxut-shell bulunamadı (NUXUT_SHELL_BIN, Ayarlar'ın dizini ve PATH arandı).

# Nerede: Aynı hata satırı. Anlam: nuxut-shell'in bulunduğu ama başlatılamadığı.
# Değişkenler: $error (işletim sisteminin hata açıklaması, ör. "Permission denied").
settings-sections-error-spawn = Widget listesi alınamadı: nuxut-shell başlatılamadı ({ $error }).

# Nerede: Aynı hata satırı. Anlam: nuxut-shell'in süresi içinde yanıt vermediği.
settings-sections-error-timeout = Widget listesi alınamadı: nuxut-shell zamanında yanıt vermedi.

# Nerede: Aynı hata satırı; shell çalışıp başarısız çıktığında. Anlam: Shell'in bir çıkış koduyla sonlandığı.
# Değişkenler: $code (çıkış kodu, ör. 1).
settings-sections-error-failed-code = Widget listesi alınamadı: nuxut-shell { $code } koduyla çıktı.

# Nerede: Aynı hata satırı; shell sinyalle kırıldığında. Anlam: Shell'in çıkış kodu olmadan sonlandığı.
settings-sections-error-failed-signal = Widget listesi alınamadı: nuxut-shell sinyalle sonlandı.

# Nerede: Aynı hata satırı; katalog boş geldiğinde de bu kullanılır. Anlam: Shell'in şema çıktısının okunamadığı; sürüm uyuşmazlığı olabileceği.
settings-sections-error-unparsable = Widget listesi alınamadı: shell'in şema çıktısı okunamadı (sürüm uyumsuz olabilir).

## Bar kenarı ve kenar kalınlıkları

# Nerede: Bar sayfası, bar kenarı seçicinin başlığının altındaki ipucu satırı. Anlam: Barın kenarının ekran şemasından tıklanarak seçileceği.
settings-edges-pick-hint = Ekran şemasında bir kenara tıklayın

# Nerede: Bar sayfası, bar kenarı değiştirilince geçmiş kaydına verilen etiket. Anlam: Barın konumundaki değişiklik.
settings-edges-bar-position = Bar konumu

# Nerede: Bar kenarının kalınlığı değiştirilince geçmiş kaydına verilen etiket. Anlam: Bar kalınlığındaki değişiklik.
settings-edges-bar-thickness = Bar kalınlığı

# Nerede: Kenar kalınlığı değiştirilince geçmiş kaydına verilen etiket; kenar bar kenarı değilse. Anlam: İlgili kenarın kalınlığındaki değişiklik.
# Değişkenler: $edge (kenarın adı, ör. "Sol").
settings-edges-edge-thickness = { $edge } kenar kalınlığı

# Nerede: Bar sayfası, elle ayarlı kenar satırındaki metin düğme. Anlam: Kenarın genel çerçeve kalınlığına geri bağlanması.
settings-edges-link-general = Genele bağla

# Nerede: Kenar kalınlığı satırında kaydırıcının sağındaki değer etiketi; kenar bar kenarı. Anlam: Kalınlığın bar boyundan geldiği ve piksel değeri.
# Değişkenler: $px (kalınlık, piksel, ör. 48).
settings-edges-value-bar = Bar · { $px } px

# Nerede: Aynı değer etiketi; kenar elle ayarlanmışsa. Anlam: Kalınlığın bu kenara özel yazıldığı ve piksel değeri.
# Değişkenler: $px (kalınlık, piksel, ör. 12).
settings-edges-value-manual = Elle · { $px } px

# Nerede: Aynı değer etiketi; kenar genel kalınlığı izliyorsa. Anlam: Kalınlığın genel çerçeve kalınlığından geldiği ve piksel değeri.
# Değişkenler: $px (kalınlık, piksel, ör. 8).
settings-edges-value-general = Genel · { $px } px

## Bildirim uygulamaları

# Nerede: Bildirimler sayfası, uygulama listesinde açılır bildirim anahtarlarının üstündeki sütun başlığı. Anlam: Açılır (popup) bildirim anahtarı sütunu.
# [max: 10]
settings-notif-apps-col-popups = Açılır

# Nerede: Bildirimler sayfası, uygulama listesi boşken görünen açıklama. Anlam: Henüz uygulama eklenmediği ve uygulamanın nasıl ekleneceği.
settings-notif-apps-empty = Henüz uygulama yok. Bir uygulamanın açılır bildirimlerini kapatmak için adını yaz.

# Nerede: Bildirimler sayfası, uygulama listesi boşken maskotun altındaki başlık. Anlam: henüz uygulama eklenmediği.
settings-notif-apps-empty-title = Henüz uygulama yok

# Nerede: Bildirimler sayfası, uygulama listesi boşken başlığın altındaki açıklama. Anlam: uygulamanın nasıl ekleneceği.
settings-notif-apps-empty-body = Bir uygulamanın açılır bildirimlerini kapatmak için adını yazın.

# Nerede: Bildirimler sayfası, uygulama eklemeye yarayan yazma alanının yer tutucusu. Anlam: Eklenecek uygulamanın adının beklendiği.
settings-notif-apps-name-placeholder = Uygulama adı (ör. Firefox)

# Nerede: Bildirimler sayfası, uygulamanın bildirim ayarı eklenince ya da kaldırılınca geçmiş kaydına verilen etiket. Anlam: O uygulamanın bildirim ayarındaki değişiklik.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox").
settings-notif-apps-notifications = { $name } bildirimleri

# Nerede: Bildirimler sayfası, uygulamanın açılır bildirim anahtarı değişince geçmiş kaydına verilen etiket. Anlam: O uygulamanın açılır bildirim ayarındaki değişiklik.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox").
settings-notif-apps-popups-toggled = { $name } açılır bildirimleri

# Nerede: Bildirimler sayfası, uygulamanın geçmiş anahtarı değişince geçmiş kaydına verilen etiket. Anlam: O uygulamanın bildirim geçmişi ayarındaki değişiklik.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox").
settings-notif-apps-history-toggled = { $name } bildirim geçmişi

## Hareket rolü düzenleyici

# Nerede: Hareket sayfası, rol düzenleyicinin sağ üstündeki özet; rolin stili yayken. Anlam: Yayın cevap süresi ve yaylanması (sönüm oranı).
# Değişkenler: $response (yay cevap süresi, saniye, önceden biçimli, ör. "0,4"), $damping (sönüm oranı, önceden biçimli, ör. "0,6").
settings-motion-role-summary-spring = Yay · { $response } sn · yaylanma { $damping }

# Nerede: Hareket sayfası, rol düzenleyicide eğrinin sağındaki ipucu; stil yayken. Anlam: Grafiği sürüklemenin hızı ve taşma üzerindeki etkisi.
settings-motion-role-hint-spring = Grafiği sürükle: sağa daha yavaş, yukarı daha çok taşma.

# Nerede: Hareket sayfası, rol düzenleyicinin özetindeki eğri adı; kayıttaki adlandırılmış eğrilerden biri değilse. Anlam: Kullanıcının kendi çizdiği özel eğri.
settings-motion-easing-custom = Özel eğri

# Nerede: Hareket sayfası, rol düzenleyicinin sağ üstündeki özet; rolin stili eğriyken. Anlam: Eğrinin adı ve süresi.
# Değişkenler: $name (eğrinin adı, ör. "Hızlı"), $ms (süre, milisaniye, ör. 200).
settings-motion-role-summary-bezier = Eğri · { $name } · { $ms } ms

# Nerede: Hareket sayfası, rol düzenleyicide eğrinin sağındaki ipucu; stil eğriyken. Anlam: Tutamaçları sürükleyerek eğriyi değiştirme yönergesi.
settings-motion-role-hint-bezier = Tutamaçları sürükleyerek eğriyi değiştir.

# Nerede: Hareket sayfası, rolün yay değerleri sürüklenince geçmiş kaydına verilen etiket. Anlam: Rolün yayınındaki değişiklik.
# Değişkenler: $role (hareket rolünün adı, ör. "Efektler").
settings-motion-role-drag-spring = { $role } yayı

# Nerede: Hareket sayfası, rolün eğrisi sürüklenince geçmiş kaydına verilen etiket. Anlam: Rolün eğrisindeki değişiklik.
# Değişkenler: $role (hareket rolünün adı, ör. "Efektler").
settings-motion-role-drag-bezier = { $role } eğrisi

## Hareket ön ayarı kartları

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Sakin ön ayarı (varsayılan); yumuşak ve taşmasız hareket.
settings-motion-preset-calm = Sakin
    .description = Yumuşak, taşmasız

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Canlı ön ayarı; esneyen, zıplayan hareket.
settings-motion-preset-lively = Canlı
    .description = Puding: esner, zıplar

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Çevik ön ayarı; kısa ve net hareket.
settings-motion-preset-snappy = Çevik
    .description = Kısa ve net

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Kapalı ön ayarı; animasyonların olmadığı.
settings-motion-preset-instant = Kapalı
    .description = Animasyon yok

# Nerede: Hareket sayfası, ön ayarların elle özelleştirildiğinde görünen rozet. Anlam: Ön ayarın üstüne yazılmış değerler olduğu.
settings-motion-preset-customized = Özelleştirilmiş

# Nerede: Hareket sayfası, özelleştirme rozetinin yanındaki açıklama. Anlam: Tek tek ayarlanan değerlerin karakterin (ön ayarın) önüne geçtiği.
settings-motion-preset-customized-note = Tek tek ayarlanan değerler karakterin önüne geçiyor.

# Nerede: Hareket sayfası, özelleştirilmişken görünen düğme; aynı metin geri al kaydının etiketi. Anlam: Elle yazılan değerleri silip karaktere (ön ayara) dönme.
settings-motion-preset-revert = Karaktere dön

# Nerede: Hareket sayfası, ön ayar kartı seçilince geçmiş kaydına verilen etiket. Anlam: Hareket ön ayarının değiştiği.
# Değişkenler: $preset (ön ayarın adı, ör. "Canlı").
settings-motion-preset-changed = Hareket ön ayarı: { $preset }
## Yazı sayfası — eksik font bilgisi

# Nerede: Yazı sayfası, font seçicinin başlık satırındaki durum yazısı; seçili aile kurulu değilken yedekte kullanılan aile biliniyorsa. Anlam: Ailenin kurulu olmadığı, geçici olarak hangi ailenin kullanıldığı.
# Değişkenler: $used (yedek olarak kullanılan aile, ör. "DejaVu Sans").
settings-fonts-missing-used = Kurulu değil — { $used } kullanılıyor

# Nerede: Aynı durum yazısı; ailenin bilinen bir Arch paketi varsa sona eklenen kısım. Anlam: Ailenin kurulu olmadığı, yedeğin kullanıldığı ve kurulabilecek Arch paketi.
# Değişkenler: $used (yedek olarak kullanılan aile, ör. "DejaVu Sans"), $package (Arch paketinin adı, ör. "inter-font").
settings-fonts-missing-used-package = Kurulu değil — { $used } kullanılıyor · Arch paketi: { $package }

# Nerede: Yazı sayfası, eksik font durum yazısı; yedekte kullanılan aile bilinmiyorken (ikon seti kartının alt yazısında da). Anlam: Ailenin kurulu olmadığı, yedek bir ailenin kullanıldığı.
settings-fonts-missing-fallback = Kurulu değil — yedek kullanılıyor

# Nerede: Aynı durum yazısı; ailenin bilinen bir Arch paketi varsa. Anlam: Yedek kullanımına ek olarak kurulabilecek Arch paketi.
# Değişkenler: $package (Arch paketinin adı, ör. "noto-fonts").
settings-fonts-missing-fallback-package = Kurulu değil — yedek kullanılıyor · Arch paketi: { $package }

## Yazı sayfası — font seçici

# Nerede: Yazı sayfası, rolün font seçicisi açıldığındaki arama alanının yer tutucusu. Anlam: Yazı tipi ailelerinde arama yapılacağı.
settings-fonts-search-placeholder = Yazı tipi ara

# Nerede: Yazı sayfası, font seçicide aile listesini süzen çip (yalnızca metin rollerinde görünür). Anlam: Süzgenin kalktığı, tüm ailelerin listelendiği.
# [max: 6]
settings-fonts-filter-all = Tümü

# Nerede: Yazı sayfası, font seçicinin kalınlık seçimindeki çipler; CSS ağırlık adları (100–900). Anlam: En ince kalınlık (CSS 100).
# [max: 12]
settings-fonts-weight-thin = İnce

# Nerede: Aynı kalınlık çipleri. Anlam: Çok hafif kalınlık (CSS 200).
# [max: 12]
settings-fonts-weight-extra-light = Çok hafif

# Nerede: Aynı kalınlık çipleri. Anlam: Hafif kalınlık (CSS 300).
# [max: 12]
settings-fonts-weight-light = Hafif

# Nerede: Aynı kalınlık çipleri. Anlam: Normal kalınlık (CSS 400).
# [max: 12]
settings-fonts-weight-regular = Normal

# Nerede: Aynı kalınlık çipleri. Anlam: Orta kalınlık (CSS 500).
# [max: 12]
settings-fonts-weight-medium = Orta

# Nerede: Aynı kalınlık çipleri. Anlam: Yarı kalın (CSS 600).
# [max: 12]
settings-fonts-weight-semi-bold = Yarı kalın

# Nerede: Aynı kalınlık çipleri. Anlam: Kalın (CSS 700).
# [max: 12]
settings-fonts-weight-bold = Kalın

# Nerede: Aynı kalınlık çipleri. Anlam: Çok kalın (CSS 800).
# [max: 12]
settings-fonts-weight-extra-bold = Çok kalın

# Nerede: Aynı kalınlık çipleri. Anlam: En kalın (CSS 900).
# [max: 12]
settings-fonts-weight-black = Siyah

# Nerede: Yazı sayfası, rolün yazı ayarları değişmişken seçicinin altındaki metin düğme. Anlam: Rolün aile, kalınlık ve ölçek ayarlarını varsayılana döndürme.
settings-fonts-reset = Varsayılana dön

# Nerede: Yazı sayfası, İkon rolü seçicisinin durum yazısı; aile boş bırakılıp setin fontu kullanılıyorsa. Anlam: Görünen ailenin seçili ikon setinin kendi fontu olduğu.
settings-fonts-icon-set-font = İkon setinin fontu

# Nerede: Yazı sayfası, font seçicinin durum yazısı; rol için kayıtta aile yazılı değilken. Anlam: Ailenin framework varsayılanı olduğu.
settings-fonts-default = Varsayılan

# Nerede: Yazı sayfası, rolün ailesi ve kalınlığı seçilince geçmiş (geri al) kaydına verilen etiket. Anlam: Rolün yazı tipindeki değişiklik.
# Değişkenler: $role (rolün adı, ör. "Arayüz").
settings-fonts-role-family = { $role } yazı tipi

# Nerede: Yazı sayfası, rolün yazı ayarları varsayılana dönerken geçmiş kaydına verilen etiket. Anlam: Rolün yazı ayarlarındaki sıfırlama.
# Değişkenler: $role (rolün adı, ör. "Arayüz").
settings-fonts-role-reset = { $role } yazısı

## Yazı rolleri

# Nerede: Yazı sayfası, İkon rolünün önizleme satırındaki başlık; geçmiş kayıtlarındaki rol adı olarak da. Anlam: İkon fontu rolünün adı.
settings-typography-role-icon = İkon

## Duvar kağıdı seçici

# Nerede: Duvar kağıdı sayfası, hero önizlemenin yanındaki dolu düğme; görsel seçilmemişken (src/pages/wallpaper.rs). Anlam: Dosya seçiciyle bir görsel seçme.
settings-wallpaper-button-choose = Görsel seç…

# Nerede: Duvar kağıdı sayfası, hero önizlemenin yanındaki tonlu düğme; görsel seçiliyken (src/pages/wallpaper.rs). Anlam: Mevcut görselin değiştirilmesi.
settings-wallpaper-button-change = Değiştir…

# Nerede: Duvar kağıdı sayfası, hero önizlemenin altındaki kapsam satırı; tüm monitörler aynı görseli kullanırken (src/pages/wallpaper.rs). Anlam: Duvar kağıdının tüm monitörlerde ortak olduğu.
settings-wallpaper-scope-all = Tüm monitörlerde

# Nerede: Duvar kağıdı sayfası, hero önizlemenin altındaki kapsam satırı; bazı monitörler ayrı görsel kullanırken (src/pages/wallpaper.rs). Anlam: Ayrı görseli olan monitör sayısı.
# Değişkenler: $count (ayrı görseli olan monitör sayısı, ör. 2).
settings-wallpaper-scope-per-monitor = { $count } monitörde ayrı görsel var

## Yerleşik Nuxut duvar kağıtları

# Nerede: Duvar kağıdı sayfasında, hero önizlemenin yanında görsel adının altındaki açıklama satırı; seçili duvar kağıdı Nuxut ile gelen bir yerleşik duvar kağıdıyken görünür (src/pages/wallpaper.rs). Anlam: Bu duvar kağıdının bir dosya değil, Nuxut'un hazır duvar kağıdı olduğunu anlatır.
settings-wallpaper-builtin-caption = Nuxut duvar kağıdı

# Nerede: Duvar kağıdı sayfasında, hero önizlemenin yanında görsel adının altındaki açıklama satırı; hiç duvar kağıdı seçilmemişken görünür (src/pages/wallpaper.rs). Anlam: Seçim yapılmadığı için Nuxut'un varsayılan duvar kağıdının gösterildiğini anlatır.
settings-wallpaper-builtin-default = Varsayılan Nuxut duvar kağıdı

# Nerede: Duvar kağıdı sayfası, yerleşik duvar kağıtlarının kart ızgarası satırının başlığı (src/registry.rs, wallpaper.builtin). Anlam: Nuxut ile gelen hazır duvar kağıtları.
settings-wallpaper-builtin-label = Nuxut duvar kağıtları

# Nerede: Duvar kağıdı sayfası, yerleşik duvar kağıtları satırının alt yazısı. Anlam: Bu duvar kağıtları koyu/açık kipe göre kendiliğinden değişir ve arayüz renkleri onlardan türetilir.
settings-wallpaper-builtin-description = Koyu ve açık kipe kendiliğinden uyar; renkler de buradan gelir

# Nerede: Duvar kağıdı sayfası, yerleşik duvar kağıtları satırının arama sözcükleri (virgüllü liste). Anlam: Genel terimler ve yerleşik duvar kağıtlarının görünen adları; adlar nuxut-wallpapers kataloğundaki çevirilerle aynı olmalı.
settings-wallpaper-builtin-keywords = nuxut, yerleşik, hazır, varsayılan, duvar kağıdı, eş uçuş, kanat ucu, tırmanış, göç, aurora, büyük silüet, bulut molası, uzak süzülüş, kovalamaca

# Nerede: Duvar kağıdı sayfasında bir bölüm (kart) başlığı. Anlam: Nuxut ile gelen hazır duvar kağıtlarını toplayan bölüm.
settings-section-builtin-wallpapers = Nuxut duvar kağıtları

## Renkler bağlantısı

# Nerede: Duvar kağıdı sayfası, Renkler bağlantı satırının başlığı; renkler duvar kağıdından alınırken (src/pages/wallpaper.rs). Anlam: Arayüz renklerinin bu görselden türetildiği.
settings-wallpaper-colors-link-title-wallpaper = Renkler bu görselden alınıyor

# Nerede: Aynı satırın alt yazısı. Anlam: Renk şeması ve kontrastın Görünüm › Renkler'de ayarlandığı.
settings-wallpaper-colors-link-subtitle-wallpaper = Şema ve kontrast Görünüm › Renkler'de

# Nerede: Aynı satırın başlığı; renkler sabitken (src/pages/wallpaper.rs). Anlam: Renklerin duvar kağıdından türetilmediği.
settings-wallpaper-colors-link-title-static = Renkler Nuxut temasından

# Nerede: Aynı satırın alt yazısı. Anlam: Tüm renklerin bu görsele uyabileceği.
settings-wallpaper-colors-link-subtitle-static = Tüm renkler bu görsele uyabilir

# Nerede: Duvar kağıdı sayfası, Renkler bağlantı satırının sağındaki metin düğme (src/pages/wallpaper.rs). Anlam: Görünüm › Renkler sayfasına götürür.
settings-wallpaper-colors-link-go = Görünüm

## Renk şeması

# Nerede: Duvar kağıdı sayfası, renk şeması seçicideki dokuz varyant kartının başlıkları. Anlam: Material You Tonal Spot varyantı; kağıdın tonuna yakın renkler.
settings-wallpaper-variant-tonal = Tonal

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Vibrant varyantı; canlı, doygun renkler.
settings-wallpaper-variant-vibrant = Canlı

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Expressive varyantı; özgün ve ifadeli renkler.
settings-wallpaper-variant-expressive = Etkileyici

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Neutral varyantı; yumuşak, nötr renkler.
settings-wallpaper-variant-neutral = Nötr

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Fidelity varyantı; görseldeki renklere sadık.
settings-wallpaper-variant-fidelity = Sadık

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Content varyantı; görselin içeriğine göre ayarlı renkler.
settings-wallpaper-variant-content = İçerik

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Monochrome varyantı; tek renkli, gri tonlar.
settings-wallpaper-variant-monochrome = Tek renk

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Rainbow varyantı; gökkuşağı tonları.
settings-wallpaper-variant-rainbow = Gökkuşağı

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Fruit Salad varyantı; canlı, karışık renkler.
settings-wallpaper-variant-fruit-salad = Meyve salatası

# Nerede: Duvar kağıdı sayfası, renk şeması seçicinin durum satırı; görsel seçilmemişken. Anlam: Renk üretilebilmesi için önce duvar kağıdı seçilmesi gerektiği.
settings-wallpaper-scheme-need-image = Önce bir duvar kağıdı seçin.

# Nerede: Duvar kağıdı sayfası, renk şeması seçicinin durum satırı; yeni görselin renk şeritleri üretilirken. Anlam: Renklerin hazırlanıyor olduğu.
settings-wallpaper-scheme-preparing = Renkler hazırlanıyor…

## Duvar kağıdı hataları

# Nerede: Duvar kağıdı sayfası ve monitör başına duvar kağıdı listesi; dosya seçici açılamayınca satırın altındaki hata yazısı. Anlam: xdg-desktop-portal dosya seçicisinin açılamadığı.
settings-wallpaper-error-portal = Dosya seçici açılamadı (xdg-desktop-portal çalışıyor mu?).

# Nerede: Duvar kağıdı sayfası, renk şeması seçicinin durum satırı (hata); görselden tohum renk çıkarılamayınca. Anlam: Bu görselden renk çıkarılamadığı.
settings-wallpaper-error-source = Bu görselden renk çıkarılamadı.

# Nerede: Aynı durum satırı (hata); görsel boyut/piksel sınırını aşınca renk çıkarılamadığında. Anlam: Görselin çok büyük olduğu.
settings-wallpaper-error-too-large = Bu görsel çok büyük; renk çıkarılamadı.

# Nerede: Aynı durum satırı (hata); görsel işçisi (nuxut-image-worker) kurulu değilken JPEG XL gibi yalnızca işçide açılan bir biçim seçilince. Anlam: Bu biçimin görsel işçisi gerektirdiği.
settings-wallpaper-error-worker = Bu biçim için görsel işçisi (nuxut-image-worker) gerekli; bulunamadı.

# Nerede: Aynı durum satırı (hata); palette.toml'un yazılacağı durum dizini bulunamayınca. Anlam: Durum dizininin bulunamadığı.
settings-wallpaper-error-state-dir = Durum dizini bulunamadı ($XDG_STATE_HOME).

# Nerede: Aynı durum satırı (hata); üretilen palet dosyaya yazılamayınca. Anlam: Renk şemasının kaydedilemediği.
settings-wallpaper-error-save = Renk şeması kaydedilemedi.

## Monitör başına duvar kağıdı

# Nerede: Duvar kağıdı sayfası, monitör başına satırdaki yerleşim seçim düğmelerinin ilk segmenti. Anlam: Monitörün sayfanın genel duvar kağıdını ve yerleşimini izlediği.
# [max: 8]
settings-outputs-fit-general = Genel

# Nerede: Duvar kağıdı sayfası, monitör başına listedeki yerleşim seçim menüsünün ilk seçeneği (src/pages/wallpaper_outputs.rs). Anlam: Monitörün genel yerleşimi izlediği ve genel yerleşimin adı.
# Değişkenler: $fit (genel yerleşimin adı, ör. "Tamamı").
settings-outputs-fit-general-option = Genel · { $fit }

# Nerede: Duvar kağıdı sayfası, monitör başına listede hiç monitör yokken görünen satır (src/pages/wallpaper_outputs.rs). Anlam: Monitör bulunamadığı.
settings-outputs-no-monitors = Monitör bulunamadı.

# Nerede: Duvar kağıdı sayfası, monitör başına listede elle monitör ekleme alanının yer tutucusu (src/pages/wallpaper_outputs.rs). Anlam: Eklenecek monitörün bağlantı adının beklendiği.
settings-outputs-connector-placeholder = Bağlantı adı, ör. HDMI-A-1

# Nerede: Duvar kağıdı sayfası, monitör başına listeyi açan açılır satırın başlığı (src/pages/wallpaper_outputs.rs). Anlam: Bağlı olmayan bir monitörün elle eklenebildiği.
settings-outputs-add-unconnected = Bağlı olmayan monitör ekle

# Nerede: Aynı açılır satırın başlığındaki açıklama (src/pages/wallpaper_outputs.rs). Anlam: Henüz takılı olmayan bir monitöre görselin önceden atanabildiği.
settings-outputs-add-unconnected-description = Henüz takılı olmayan bir monitöre önceden görsel atamak için

# Nerede: Aynı açılır satırın içindeki, yazılan adı onaylayan düğme (src/pages/wallpaper_outputs.rs). Anlam: Yazılan bağlantı adıyla monitör satırı ekleme.
settings-outputs-add-button = Ekle

# Nerede: Duvar kağıdı sayfası, monitör satırının alt yazısı; monitör için görsel seçilmemişse dosya adının yerine. Anlam: Monitörün genel duvar kağıdını kullandığı.
settings-outputs-general-wallpaper = Genel duvar kağıdı

# Nerede: Duvar kağıdı sayfası, monitör satırının alt yazısı; monitör kayıtlı ama bağlı değilken (src/pages/wallpaper_outputs.rs). Anlam: Monitörün bağlı olmadığı ve görselin dosya adının gösterildiği.
# Değişkenler: $image (görselin dosya adı, ör. "wallpaper.png"; görsel seçilmemişse "Genel duvar kağıdı").
settings-outputs-detail-disconnected = Bağlı değil · { $image }

# Nerede: Duvar kağıdı sayfası, monitör satırının alt yazısı; monitör bağlıyken donanım açıklaması yoksa. Anlam: Monitörün bağlı olduğu.
settings-outputs-connected = Bağlı

# Nerede: Duvar kağıdı sayfası, monitör başına listede elle monitör ekleme alanının altındaki hata yazısı (src/pages/wallpaper_outputs.rs). Anlam: Girilen bağlantı adının kurallara uymadığı.
settings-outputs-error-name-rules = Geçersiz ad: boşluk ya da tırnak içermeyen en çok 64 karakter (ör. DP-1).

# Nerede: Aynı hata yazısı; monitör sınırı dolduğunda. Anlam: En fazla 32 monitör eklenebileceği.
settings-outputs-error-too-many = En fazla 32 monitör.

# Nerede: Duvar kağıdı sayfası, monitörün yerleşimi değişince geçmiş (geri al) kaydına verilen etiket. Anlam: Monitörün yerleşimindeki değişiklik.
# Değişkenler: $name (monitörün bağlayıcı adı, ör. "DP-1").
settings-outputs-fit = { $name } yerleşimi

# Nerede: Duvar kağıdı sayfası, monitör için görsel seçilince geçmiş kaydına verilen etiket. Anlam: O monitörün duvar kağıdındaki değişiklik.
# Değişkenler: $name (monitörün bağlayıcı adı, ör. "DP-1").
settings-outputs-wallpaper = { $name } duvar kağıdı

## Kısayol satırı

# Nerede: Kısayollar sayfası, kısayol alanı yakalama modundayken satırın altındaki ipucu. Anlam: Tuş kombinasyonunun beklendiği; Esc'in iptal, Geri silmenin temizleme yaptığı.
settings-keybinds-hint = Tuş birleşimine bas · Esc iptal · Geri silme temizler

# Nerede: Kısayollar sayfası, yakalama sırasında alanın içindeki metin; henüz değiştirici basılı değilken. Anlam: Tuşların bekleniyor olduğu.
settings-keybinds-capturing = Tuşlara bas…

# Nerede: Kısayollar sayfası, atanmamış kısayol alanının içindeki metin. Anlam: Eyleme kısayol atanmadığı.
settings-keybinds-unassigned = Atanmamış

# Nerede: Kısayollar sayfası, satırın altındaki uyarı; dosyadaki kısayol değeri geçersizken (her hata için ayrı mesaj). Anlam: Değerin boş olduğu ve kısayolun atanmamış sayıldığı.
settings-keybinds-error-empty = Geçersiz kısayol (boş); atanmamış sayılıyor

# Nerede: Aynı uyarı. Anlam: Kısayolun çok uzun olduğu ve atanmamış sayıldığı.
settings-keybinds-error-too-long = Geçersiz kısayol (çok uzun); atanmamış sayılıyor

# Nerede: Aynı uyarı. Anlam: Tanınmayan değiştirici içerdiği ve atanmamış sayıldığı.
settings-keybinds-error-unknown-modifier = Geçersiz kısayol (bilinmeyen değiştirici); atanmamış sayılıyor

# Nerede: Aynı uyarı. Anlam: Geçersiz tuş içerdiği ve atanmamış sayıldığı.
settings-keybinds-error-invalid-key = Geçersiz kısayol (geçersiz tuş); atanmamış sayılıyor

# Nerede: Kısayollar sayfası, yakalama sırasında satırın altındaki uyarı; yazı tuşu değiştiricisiz basıldığında. Anlam: Yazı tuşlarının Super, Ctrl ya da Alt ile kullanılması gerektiği.
settings-keybinds-need-modifier = Yazı tuşları Super, Ctrl ya da Alt ile birlikte kullanılmalı

# Nerede: Kısayollar sayfası, yakalama sırasında satırın altındaki uyarı; tuşun adı çözülemeyince. Anlam: Tuşun tanınmadığı, başka bir birleşim denenmesi gerektiği.
settings-keybinds-unknown-key = Bu tuş tanınamadı; başka bir birleşim dene

# Nerede: Kısayollar sayfası, satırın altındaki uyarı; aynı birleşim başka eylemde de var ve bu satır shell sırasında önde. Anlam: Kısayolun iki eylemde olduğu, diğerinde çalışmayacağı.
# Değişkenler: $other (diğer eylemin adı, ör. "Sesi kapat").
settings-keybinds-clash-wins = Aynı kısayol «{ $other }» eyleminde de var; orada çalışmaz

# Nerede: Aynı uyarı; bu satır shell sırasında geride. Anlam: Kısayolun iki eylemde olduğu, bu eylemde çalışmayacağı.
# Değişkenler: $other (diğer eylemin adı, ör. "Sesi kapat").
settings-keybinds-clash-loses = Aynı kısayol «{ $other }» eyleminde de var; burada çalışmaz

## Tuş başlıkları

# Nerede: Kısayollar sayfası, kısayol alanındaki tuş başlıkları ve yakalarken basılı değiştiriciler. Anlam: Super değiştiricisinin başlığı.
settings-keybind-cap-super = Super

# Nerede: Aynı tuş başlıkları. Anlam: Ctrl değiştiricisinin başlığı.
settings-keybind-cap-ctrl = Ctrl

# Nerede: Aynı tuş başlıkları. Anlam: Alt değiştiricisinin başlığı.
settings-keybind-cap-alt = Alt

# Nerede: Aynı tuş başlıkları. Anlam: Shift değiştiricisinin başlığı.
settings-keybind-cap-shift = Shift

# Nerede: Kısayollar sayfası, kısayol alanındaki tuş başlıkları; medya tuşlarının görünen adı. Anlam: Sesi yükseltme tuşu.
# [max: 12]
settings-keybind-cap-volume-up = Ses +

# Nerede: Aynı tuş başlıkları. Anlam: Sesi azaltma tuşu.
# [max: 12]
settings-keybind-cap-volume-down = Ses −

# Nerede: Aynı tuş başlıkları. Anlam: Sesi kapama tuşu.
# [max: 12]
settings-keybind-cap-mute = Sessiz

# Nerede: Aynı tuş başlıkları. Anlam: Mikrofonu kapama tuşu.
# [max: 12]
settings-keybind-cap-mic-mute = Mikrofon

# Nerede: Aynı tuş başlıkları. Anlam: Ekran parlaklığını artırma tuşu.
# [max: 12]
settings-keybind-cap-brightness-up = Parlaklık +

# Nerede: Aynı tuş başlıkları. Anlam: Ekran parlaklığını azaltma tuşu.
# [max: 12]
settings-keybind-cap-brightness-down = Parlaklık −

# Nerede: Aynı tuş başlıkları. Anlam: Oynatma tuşu.
# [max: 12]
settings-keybind-cap-play = Oynat

# Nerede: Aynı tuş başlıkları. Anlam: Duraklatma tuşu.
# [max: 12]
settings-keybind-cap-pause = Duraklat

# Nerede: Aynı tuş başlıkları. Anlam: Durdurma tuşu.
# [max: 12]
settings-keybind-cap-stop = Durdur

# Nerede: Aynı tuş başlıkları. Anlam: Sonraki parça tuşu.
# [max: 12]
settings-keybind-cap-next = Sonraki

# Nerede: Aynı tuş başlıkları. Anlam: Önceki parça tuşu.
# [max: 12]
settings-keybind-cap-previous = Önceki

# Nerede: Aynı tuş başlıkları. Anlam: Hesap makinesi açma tuşu.
# [max: 14]
settings-keybind-cap-calculator = Hesap makinesi

# Nerede: Aynı tuş başlıkları. Anlam: Arama tuşu.
# [max: 12]
settings-keybind-cap-search = Ara

# Nerede: Aynı tuş başlıkları. Anlam: Posta tuşu.
# [max: 12]
settings-keybind-cap-mail = Posta

# Nerede: Aynı tuş başlıkları. Anlam: Ana sayfa tuşu.
# [max: 12]
settings-keybind-cap-home-page = Ana sayfa

# Nerede: Aynı tuş başlıkları. Anlam: Ortamı çıkarma tuşu.
# [max: 12]
settings-keybind-cap-eject = Çıkar

# Nerede: Aynı tuş başlıkları. Anlam: Uyku tuşu.
# [max: 12]
settings-keybind-cap-sleep = Uyku

# Nerede: Aynı tuş başlıkları. Anlam: Ekran değiştirme tuşu.
# [max: 12]
settings-keybind-cap-display = Ekran

# Nerede: Aynı tuş başlıkları. Anlam: Kablosuz ağ tuşu.
# [max: 12]
settings-keybind-cap-wlan = Kablosuz

# Nerede: Aynı tuş başlıkları. Anlam: Dokunmatik yüzeyi açıp kapama tuşu.
settings-keybind-cap-touchpad = Dokunmatik yüzey

# Nerede: Kısayollar sayfası, kısayol alanındaki tuş başlıkları. Anlam: Boşluk tuşu.
# [max: 8]
settings-keybind-cap-space = Boşluk

# Nerede: Aynı tuş başlıkları. Anlam: Geri silme tuşu.
# [max: 10]
settings-keybind-cap-backspace = Geri sil

# Nerede: Kısayollar sayfası, fare düğmesine atanmış kısayolun tuş başlığı. Anlam: Fare düğmesi ve numarası.
# Değişkenler: $button (fare düğme numarası, ör. 1).
settings-keybind-cap-mouse = Fare { $button }

# Nerede: Kısayollar sayfası, ham tuş koduyla atanmış kısayolun tuş başlığı. Anlam: Tuş kodu ve numarası.
# Değişkenler: $code (tuş kodu, ör. 10).
settings-keybind-cap-code = Tuş kodu { $code }
## Sayfa başlıkları (kenar çubuğu)

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Görünüm (tema ve renk) sayfasının etiketi.
# [max: 14]
settings-page-appearance = Görünüm

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Yazı tipleri sayfasının etiketi.
# [max: 14]
settings-page-fonts = Yazı

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Hareket ve animasyon sayfasının etiketi.
# [max: 14]
settings-page-motion = Hareket

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Üst bar ayarları sayfasının etiketi; "Bar" arayüzde olduğu gibi kalır.
# [max: 14]
settings-page-bar = Bar

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Shell ayarları sayfasının etiketi; "Shell" arayüzde olduğu gibi kalır.
# [max: 14]
settings-page-shell = Shell

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Monitör ve ekran ayarları sayfasının etiketi.
# [max: 14]
settings-page-displays = Ekranlar

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Pencere davranışları sayfasının etiketi.
# [max: 14]
settings-page-windows = Pencereler

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Duvar kağıdı sayfasının etiketi.
# [max: 14]
settings-page-wallpaper = Duvar kağıdı

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Kullanıcı hesapları sayfasının etiketi.
# [max: 14]
settings-page-accounts = Hesaplar

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Bildirim ayarları sayfasının etiketi.
# [max: 14]
settings-page-notifications = Bildirimler

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Ses ayarları sayfasının etiketi.
# [max: 14]
settings-page-sound = Ses

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Klavye ayarları sayfasının etiketi.
# [max: 14]
settings-page-keyboard = Klavye

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Fare ve dokunmatik yüzey ayarları sayfasının etiketi.
# kenar çubuğunda uzun etiket
settings-page-pointer = Fare ve dokunmatik yüzey

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Klavye kısayolları sayfasının etiketi.
# [max: 14]
settings-page-keybinds = Kısayollar

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Uygulama bilgisi sayfasının etiketi.
# [max: 14]
settings-page-about = Hakkında

## Kenar çubuğu grup başlıkları

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Görünüm, Duvar kağıdı, Yazı, Hareket sayfalarını toplayan kişisel grup.
# [max: 10]
settings-nav-group-personal = Kişisel

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Bar, Shell, Pencereler, Ekranlar, Kısayollar sayfalarını toplayan masaüstü grubu.
# [max: 10]
settings-nav-group-desktop = Masaüstü

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Bildirimler, Ses, Hesaplar, Hakkında sayfalarını toplayan sistem grubu.
# [max: 10]
settings-nav-group-system = Sistem

## Hareket rolleri (Hareket sayfası)

## Kaydırıcı birimleri

# Nerede: Ayarlar'daki piksel birimli kaydırıcılarda değerin yanında. Anlam: piksel birimi kısaltması.
settings-unit-pixel = px

# Nerede: Yüzde birimli kaydırıcılarda (gölge yoğunluğu, kaydırma adımı) değerin yanında. Anlam: yüzde işareti.
settings-unit-percent = %

# Nerede: Çarpan birimli kaydırıcılarda (hız, ölçek) değerin yanında. Anlam: kat işareti.
settings-unit-times = ×

# Nerede: Milisaniye birimli kaydırıcılarda (eğri süresi) değerin yanında. Anlam: milisaniye kısaltması.
settings-unit-ms = ms

# Nerede: Saniye birimli kaydırıcılarda (yay süresi) değerin yanında. Anlam: saniye birimi kısaltması.
settings-unit-second = sn

# Nerede: Klavye sayfası, Tekrar hızı kaydırıcısında değerin yanında. Anlam: saniyedeki tuş tekrarı sayısı birimi.
settings-unit-repeat-rate = tuş/sn

# Nerede: Dakika birimli kaydırıcılarda (gece ışığı geçiş süresi) değerin yanında. Anlam: dakika birimi kısaltması.
settings-unit-minute = dk

## Seçenekler: Görünüm

# Nerede: Görünüm sayfası, Renk kaynağı ayarının segmentleri. Anlam: renklerin seçili Nuxut renk temasından geldiği seçenek.
settings-choice-color-source-static = Nuxut teması

# Nerede: Görünüm sayfası, Renk kaynağı ayarının segmentleri. Anlam: renklerin duvar kağıdından türetildiği seçenek.
settings-choice-color-source-wallpaper = Duvar kağıdından

# Nerede: Palet kipi seçenekleri. Anlam: koyu palet seçeneği.
settings-choice-palette-mode-dark = Koyu

# Nerede: Palet kipi seçenekleri. Anlam: açık palet seçeneği.
settings-choice-palette-mode-light = Açık

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının segmentleri. Anlam: kenarlık çizilmemesi seçeneği.
settings-choice-outline-none = Yok

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının segmentleri. Anlam: hafif görünür ince kenarlık seçeneği.
settings-choice-outline-subtle = İnce

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının segmentleri. Anlam: tam kalınlıkta kenarlık seçeneği.
settings-choice-outline-full = Tam

## Seçenekler: Duvar kağıdı

# Nerede: Duvar kağıdı sayfasındaki yerleşim seçenekleri. Anlam: kağıdı ekrana doldurup taşan kısmı kırpan seçenek.
settings-choice-wallpaper-fit-cover = Doldur ve kırp

# Nerede: Duvar kağıdı sayfasındaki yerleşim seçenekleri. Anlam: kağıdın tamamını sığdıran seçenek.
settings-choice-wallpaper-fit-contain = Tamamı

# Nerede: Duvar kağıdı sayfasındaki yerleşim seçenekleri. Anlam: kağıdı oranı bozarak ekrana geren seçenek.
settings-choice-wallpaper-fit-fill = Uzat

## Seçenekler: Hareket

# Nerede: Hareket sayfası, Stil ayarının segmentleri. Anlam: yay (spring) animasyon stili.
settings-choice-motion-style-spring = Yay

# Nerede: Hareket sayfası, Stil ayarının segmentleri. Anlam: eğri (easing) animasyon stili.
settings-choice-motion-style-curve = Eğri

# Nerede: Hareket rollerindeki Stil ayarının segmentleri. Anlam: rolün genel stili izlediği seçenek.
settings-choice-role-style-default = Genel

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: Material vurgulu eğri ailesi.
settings-choice-easing-emphasized = Vurgulu

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: vurgulu ve yavaşlayarak biten eğri.
settings-choice-easing-emphasized-decelerate = Vurgulu yavaşlayan

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: vurgulu ve hızlanarak başlayan eğri.
settings-choice-easing-emphasized-accelerate = Vurgulu hızlanan

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: yumuşak giriş çıkışlı standart eğri.
settings-choice-easing-ease = Yumuşak

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: yavaşlayarak biten eğri.
settings-choice-easing-ease-out = Yavaşlayan

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: hızlanarak başlayan eğri.
settings-choice-easing-ease-in = Hızlanan

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: yavaşlayıp hızlanıp yeniden yavaşlayan eğri.
settings-choice-easing-ease-in-out = Yavaş-hızlı-yavaş

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: sabit hızlı doğrusal eğri.
settings-choice-easing-linear = Doğrusal

## Seçenekler: Bar uç eylemleri

# Nerede: Bar sayfası, uç eylemi seçim çipleri. Anlam: eylemin widget'ın kendi görevi olduğu seçenek.
settings-choice-edge-click-widget = Widget'ın kendi

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: eylemi takvim panelini açmak olan seçenek.
settings-choice-edge-click-clock = Takvim

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: eylemi uygulama başlatıcıyı açmak olan seçenek.
settings-choice-edge-click-launcher = Uygulama başlatıcı

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: sesi açıp kapatan seçenek.
settings-choice-edge-click-volume = Sesi kapat/aç

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: mikrofonu açıp kapatan seçenek.
settings-choice-edge-click-mic = Mikrofonu kapat/aç

# Nerede: Bar sayfası, kaydırma eylemi çipleri. Anlam: kaydırmayla ses düzeyini değiştiren seçenek.
settings-choice-edge-scroll-volume = Ses

# Nerede: Bar sayfası, kaydırma eylemi çipleri. Anlam: kaydırmayla mikrofonu değiştiren seçenek.
settings-choice-edge-scroll-mic = Mikrofon

# Nerede: Bar sayfası, kaydırma eylemi çipleri. Anlam: kaydırmayla parlaklığı değiştiren seçenek.
settings-choice-edge-scroll-brightness = Parlaklık

## Seçenekler: Workspace widget'ı

# Nerede: Bar sayfası, Workspace görünümü ayarının segmentleri. Anlam: workspace'leri uygulama ikonuyla gösteren seçenek.
settings-choice-workspace-style-icons = İkon

# Nerede: Bar sayfası, Workspace görünümü ayarının segmentleri. Anlam: workspace'leri numarayla gösteren seçenek.
settings-choice-workspace-style-numbers = Numara

# Nerede: Bar sayfası, Workspace görünümü ayarının segmentleri. Anlam: workspace'leri noktayla gösteren seçenek.
settings-choice-workspace-style-dots = Nokta

# Nerede: Bar sayfası, Workspace kaydırma ayarının segmentleri. Anlam: boş workspace'ler de dahil tümünde kaydırmayı etkinleştiren seçenek.
settings-choice-workspace-scroll-all = Tümü

# Nerede: Bar sayfası, Workspace kaydırma ayarının segmentleri. Anlam: yalnızca dolu workspace'lerde kaydırmayı etkinleştiren seçenek.
settings-choice-workspace-scroll-occupied = Yalnızca dolu

# Nerede: Bar sayfası, Workspace kaydırma ayarının segmentleri. Anlam: widget üstünde kaydırmayı kapatır.
settings-choice-workspace-scroll-off = Kapalı

## Seçenekler: Girdi

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının segmentleri. Anlam: ivmenin uyarlanır profilde çalıştığı, hızlı hareketle işaretçinin daha uzağa gittiği seçenek.
settings-choice-accel-adaptive = Uyarlanır

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının segmentleri. Anlam: ivmenin kapatıldığı, sabit hızda hareket eden seçenek.
settings-choice-accel-flat = Düz

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: iki parmakla kaydırmayı etkinleştiren seçenek.
settings-choice-scroll-method-2fg = İki parmak

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: yüzeyin kenarında kaydırmayı etkinleştiren seçenek.
settings-choice-scroll-method-edge = Kenar

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: düğme basılı tutulurken kaydıran seçenek.
settings-choice-scroll-method-on-button-down = Düğmeyle

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: kaydırmayı tamamen kapatan seçenek.
settings-choice-scroll-method-no-scroll = Kapalı

## Görünüm sayfası — Önizleme ve Renkler

# Nerede: Görünüm ve Yazı sayfalarının en üstündeki canlı önizleme kartının başlığı. Anlam: seçili ayarların önizlemesini gösteren kart.
settings-preview-label = Önizleme

# Nerede: Görünüm sayfası, Önizleme kartının arama sözcükleri. Anlam: "canlı" ya da "örnek" yazılınca kart bulunur.
settings-preview-keywords = canlı, örnek

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: renk ayarlarını toplayan bölüm.
settings-section-colors = Renkler

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: çerçeve, bar ve panel zemininin rengini seçen ayarın başlığı.
settings-colors-surface-label = Yüzey

# Nerede: Görünüm sayfası, Yüzey ayarının altındaki açıklama. Anlam: rengin nerede kullanıldığını söyler.
settings-colors-surface-description = Çerçeve, bar ve panel zemini

# Nerede: Görünüm sayfası, Yüzey ayarının arama sözcükleri. Anlam: "arka plan" gibi terimlerle ayar bulunur.
settings-colors-surface-keywords = arka plan, zemin

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: kart zemini rengini seçen ayarın başlığı.
settings-colors-surface-container-label = Kart

# Nerede: Görünüm sayfası ve Köşeler bölümündeki kart ayarlarının altındaki açıklama. Anlam: rengin ve köşe yarıçapının kartlara uygulandığını söyler.
settings-panel-cards-description = Panel içindeki kartlar

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: hata rengini seçen ayarın başlığı.
settings-colors-error-label = Hata

# Nerede: Görünüm sayfası, Hata ayarının altındaki açıklama. Anlam: rengin nerede kullanıldığını söyler.
settings-colors-error-description = Başarısız eylemler, uyarılar

# Nerede: Görünüm sayfası, Hata ayarının arama sözcükleri. Anlam: "kırmızı" gibi terimlerle ayar bulunur.
settings-colors-error-keywords = kırmızı, uyarı

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: ayırıcı ve kenarlık rengini seçen ayarın başlığı.
settings-colors-outline-label = Çizgi

# Nerede: Görünüm sayfası, Çizgi ayarının altındaki açıklama. Anlam: rengin nerede kullanıldığını söyler.
settings-colors-outline-description = Ayırıcılar ve kenarlıklar

# Nerede: Görünüm sayfası, Çizgi ayarının arama sözcükleri. Anlam: "kenarlık" terimiyle ayar bulunur.
settings-colors-outline-keywords = kenarlık

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: gölgelerin rengini seçen ayarın başlığı.
settings-colors-shadow-label = Gölge rengi

# Nerede: Görünüm sayfası, Gölge rengi ayarının altındaki açıklama. Anlam: rengin neyi boyadığını söyler.
settings-colors-shadow-description = Gölgelerin rengi

## Görünüm sayfası — Köşeler, Gölge ve Şekiller

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: köşe yarıçapı ayarlarını toplayan bölüm.
settings-section-corners = Köşeler

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: küçük öğelerin köşe yarıçapı ayarının başlığı.
settings-shape-radius-small-label = Küçük öğeler

# Nerede: Görünüm sayfası, Köşeler bölümündeki üç yarıçap ayarının arama sözcükleri. Anlam: "yarıçap", "yuvarlak" gibi terimlerle ayarlar bulunur.
settings-shape-radius-keywords = yarıçap, radius, yuvarlak

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: kartların köşe yarıçapı ayarının başlığı.
settings-shape-radius-medium-label = Kartlar

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: panel ve pencere köşe yarıçapı ayarının başlığı.
settings-shape-radius-large-label = Paneller

# Nerede: Görünüm sayfası, Paneller ayarının altındaki açıklama. Anlam: yarıçapın hangi öğelere uygulandığını söyler.
settings-shape-radius-large-description = Paneller ve pencereler

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: yüzey kenarlıklarının kalınlığını seçen ayarın başlığı.
settings-shape-outlines-label = Kenar çizgileri

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının arama sözcükleri. Anlam: "kontrast", "outline" gibi terimlerle ayar bulunur.
settings-shape-outlines-keywords = kontrast, erişilebilirlik, kenarlık, outline

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: gölge ayarlarını toplayan bölüm.
settings-section-shadow = Gölge

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölgeleri açıp kapatan ayarın başlığı.
settings-shadow-enabled-label = Gölgeler

# Nerede: Görünüm sayfası, Gölgeler ayarının altındaki açıklama. Anlam: gölgenin nerede göründüğünü söyler.
settings-shadow-enabled-description = Bar, panel ve menülerin altında yumuşak gölge

# Nerede: Görünüm sayfası, Gölgeler ayarının arama sözcükleri. Anlam: "shadow", "derinlik" terimleriyle ayar bulunur.
settings-shadow-enabled-keywords = shadow, derinlik

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölge bulanıklığını ayarlayan denetimin başlığı.
settings-shadow-blur-label = Yumuşaklık

# Nerede: Görünüm sayfası, Yumuşaklık ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-shadow-blur-description = Gölgenin bulanıklık yarıçapı

# Nerede: Görünüm sayfası, Yumuşaklık ayarının arama sözcükleri. Anlam: "blur", "bulanık" terimleriyle ayar bulunur.
settings-shadow-blur-keywords = blur, bulanık

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölgenin dikey kaymasını ayarlayan denetimin başlığı.
settings-shadow-offset-y-label = Dikey kayma

# Nerede: Görünüm sayfası, Dikey kayma ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-shadow-offset-y-description = Gölgenin aşağı kayması

# Nerede: Görünüm sayfası, Dikey kayma ayarının arama sözcükleri. Anlam: "offset" terimiyle ayar bulunur.
settings-shadow-offset-y-keywords = offset

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölge opaklığını ayarlayan denetimin başlığı.
settings-shadow-opacity-label = Yoğunluk

# Nerede: Görünüm sayfası, Yoğunluk ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-shadow-opacity-description = Gölgenin opaklığı

# Nerede: Görünüm sayfası, Yoğunluk ayarının arama sözcükleri. Anlam: "opacity", "koyuluk" terimleriyle ayar bulunur.
settings-shadow-opacity-keywords = opacity, koyuluk

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: maskot Fiyu'yla ilgili ayarı toplayan bölüm.
settings-section-mascot = Maskot

# Nerede: Görünüm sayfası, Maskot bölümündeki anahtarın başlığı. Anlam: açılınca maskot Fiyu hiçbir yerde gösterilmez. "Fiyu" maskotun özel adıdır, çevrilmez.
settings-appearance-mascot-label = Fiyu'yu gizle

# Nerede: Görünüm sayfası, "Fiyu'yu gizle" anahtarının altındaki açıklama. Anlam: maskotun nerelerde göründüğünü söyler.
settings-appearance-mascot-description = Boş ekranlarda ve hata mesajlarında görünen maskot

# Nerede: Görünüm sayfası, "Fiyu'yu gizle" anahtarının arama sözcükleri. Anlam: bu sözcüklerle maskot ayarı bulunur.
settings-appearance-mascot-keywords = maskot, fiyu, kuş, kırlangıç

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: çerçeve birleşimi ve esneme ayarlarını toplayan bölüm.
settings-section-shapes = Şekiller

# Nerede: Görünüm sayfası, Şekiller bölümü. Anlam: panel esnemesini ayarlayan denetimin başlığı.
settings-blob-deform-label = Esneme

# Nerede: Görünüm sayfası, Esneme ayarının arama sözcükleri. Anlam: "deform", "jöle" gibi terimlerle ayar bulunur.
settings-blob-deform-keywords = blob, deform, jöle, puding

## Yazı sayfası

# Nerede: Yazı sayfası, Önizleme kartının arama sözcükleri. Anlam: "örnek", "font" terimleriyle kart bulunur.
settings-typography-preview-keywords = örnek, font

# Nerede: Yazı ve Hareket sayfalarındaki bölüm başlığı. Anlam: bir sayfanın genel ayarlarını toplayan bölüm.
settings-section-general = Genel

# Nerede: Yazı sayfası, Genel bölümü. Anlam: taban yazı boyutu ayarının başlığı.
settings-typography-size-label = Yazı boyutu

# Nerede: Yazı sayfası, Yazı boyutu ayarının arama sözcükleri. Anlam: "font", "punto" gibi terimlerle ayar bulunur.
settings-typography-size-keywords = font, punto, büyüklük

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: ikon seti ve ikon fontu ayarlarını toplayan bölüm.
settings-section-icons = İkonlar

# Nerede: Yazı sayfası, İkonlar bölümü. Anlam: ikon çizim setini seçen ayarın başlığı.
settings-typography-icon-set-label = İkon seti

# Nerede: Yazı sayfası, İkon seti ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-typography-icon-set-description = Arayüzdeki tüm ikonların çizim seti

# Nerede: Yazı sayfası, İkon seti ayarının arama sözcükleri. Anlam: "ikon", "nerd font" gibi terimlerle ayar bulunur.
settings-typography-icon-set-keywords = ikon, simge, material, nerd font, sembol

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: arayüz yazı tipi rolünü toplayan bölüm.
settings-section-interface = Arayüz

# Nerede: Yazı sayfası, Arayüz, Başlık, Belge, Eş aralıklı ve İkonlar bölümlerindeki font seçicilerin başlığı. Anlam: rolün yazı tipini seçen denetim.
settings-typography-family-label = Yazı tipi

# Nerede: Yazı sayfası, Arayüz rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-ui-description = Düğmeler, menüler, ayar satırları

# Nerede: Yazı sayfası, Arayüz font seçicinin arama sözcükleri. Anlam: "font", "weight" gibi terimlerle ayar bulunur.
settings-typography-ui-keywords = font, aile, kalınlık, weight, inter, sans

# Nerede: Yazı sayfası, rol ölçek ayarlarının başlığı. Anlam: roldeki yazıların boyut çarpanını ayarlayan denetim.
settings-typography-scale-label = Boyut ölçeği

# Nerede: Yazı sayfası, rol ölçek ayarlarının altındaki açıklama. Anlam: değerin neyi çarptığını söyler.
settings-typography-scale-description = Bu roldeki tüm yazıların çarpanı

# Nerede: Yazı sayfası, rol ölçek ayarlarının arama sözcükleri. Anlam: "ölçek", "büyüklük" gibi terimlerle ayar bulunur.
settings-typography-scale-keywords = font, büyüklük, ölçek

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: saat ve büyük başlık rolünü toplayan bölüm.
settings-section-display = Başlık

# Nerede: Yazı sayfası, Başlık rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-display-description = Saat, büyük başlıklar

# Nerede: Yazı sayfası, Başlık font seçicinin arama sözcükleri. Anlam: "saat", "display" gibi terimlerle ayar bulunur.
settings-typography-display-keywords = font, aile, kalınlık, weight, saat, display

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: uzun metin rolünü toplayan bölüm.
settings-section-document = Belge

# Nerede: Yazı sayfası, Belge rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-document-description = Uzun metin: bildirim gövdesi, takvim notları

# Nerede: Yazı sayfası, Belge font seçicinin arama sözcükleri. Anlam: "serif", "okuma" gibi terimlerle ayar bulunur.
settings-typography-document-keywords = font, aile, kalınlık, weight, serif, okuma

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: eş aralıklı yazı rolünü toplayan bölüm.
settings-section-mono = Eş aralıklı

# Nerede: Yazı sayfası, Eş aralıklı rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-mono-description = Kod, sayısal tablolar; Nerd Font ikon setinin de fontu

# Nerede: Yazı sayfası, Eş aralıklı font seçicinin arama sözcükleri. Anlam: "monospace", "kod" gibi terimlerle ayar bulunur.
settings-typography-mono-keywords = font, aile, kalınlık, weight, monospace, kod, nerd font

# Nerede: Yazı sayfası, İkonlar bölümündeki font seçicinin altındaki açıklama. Anlam: boş bırakılırsa ne olacağını söyler.
settings-typography-icon-description = Boş: ikon setinin fontu

# Nerede: Yazı sayfası, İkon font seçicinin arama sözcükleri. Anlam: "material symbols" gibi terimlerle ayar bulunur.
settings-typography-icon-keywords = font, sembol, kalınlık, weight, material symbols

# Nerede: Yazı sayfası, İkon ölçeği ayarının altındaki açıklama. Anlam: değerin neyi çarptığını söyler.
settings-typography-icon-scale-description = İkonların boyut çarpanı

## Hareket sayfası

# Nerede: Hareket sayfasındaki bölüm başlığı. Anlam: ön ayar seçimini toplayan bölüm.
settings-section-preset = Ön ayar

# Nerede: Hareket sayfası, Genel bölümü. Anlam: animasyonları azaltan erişilebilirlik ayarının başlığı.
settings-motion-enabled-label = Hareketi azalt

# Nerede: Hareket sayfası, Hareketi azalt ayarının arama sözcükleri. Anlam: "reduce motion" terimiyle ayar bulunur.
settings-motion-enabled-keywords = animasyon, erişilebilirlik, reduce motion

# Nerede: Hareket sayfası, Genel bölümü. Anlam: animasyon hız çarpanını ayarlayan denetimin başlığı.
settings-motion-speed-label = Hız

# Nerede: Hareket sayfası, Hız ayarının arama sözcükleri. Anlam: "animasyon hızı", "speed" terimleriyle ayar bulunur.
settings-motion-speed-keywords = animasyon hızı, speed, süre

# Nerede: Hareket sayfasındaki üç rol kartının ve bölüm başlığının arama sözcükleri. Anlam: "animasyon", "önizleme" terimleriyle roller bulunur.
settings-motion-role-keywords = animasyon, önizleme

# Nerede: Hareket sayfası, rol süre ayarlarının başlığı. Anlam: yayın süresini ya da eğrinin toplam süresini ayarlayan denetim.
settings-motion-role-duration-ms-label = Süre

# Nerede: Hareket sayfası, rol Süre (yay) ayarlarının arama sözcükleri. Anlam: "response", "hız" terimleriyle ayar bulunur.
settings-motion-role-response-keywords = response, yay, hız

# Nerede: Hareket sayfası, rol Eğri ayarlarının başlığı. Anlam: hız profili eğrisini seçen denetim.
settings-motion-role-easing-label = Eğri

# Nerede: Hareket sayfası, rol Eğri ayarlarının arama sözcükleri. Anlam: "easing", "bezier" terimleriyle ayar bulunur.
settings-motion-role-easing-keywords = easing, bezier, cubic-bezier

# Nerede: Hareket sayfası, rol Süre (eğri) ayarlarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-motion-role-duration-ms-description = Eğrinin toplam süresi

# Nerede: Hareket sayfası, rol Süre (eğri) ayarlarının arama sözcükleri. Anlam: "duration", "ms" terimleriyle ayar bulunur.
settings-motion-role-duration-ms-keywords = duration, ms

## Shell sayfası — Çerçeve

# Nerede: Shell sayfasındaki bölüm başlığı. Anlam: ekran çerçevesi ayarlarını toplayan bölüm.
settings-section-frame = Çerçeve

# Nerede: Shell sayfası, Çerçeve bölümü. Anlam: ekran çerçevesini açıp kapatan ayarın başlığı.
settings-frame-enabled-label = Ekran çerçevesi

# Nerede: Shell sayfası, Ekran çerçevesi ayarının arama sözcükleri. Anlam: "kenar", "border" terimleriyle ayar bulunur.
settings-frame-enabled-keywords = kenar, border, frame

# Nerede: Shell sayfası, Çerçeve bölümü. Anlam: kenar bazında kalınlık ayarını açan satırın başlığı.
settings-frame-edges-label = Kenarlar

# Nerede: Shell sayfası, Kenarlar ayarının arama sözcükleri. Anlam: "üst", "sol" gibi terimlerle ayar bulunur.
settings-frame-edges-keywords = üst, alt, sol, sağ, kalınlık, bar

# Nerede: Shell sayfası, Çerçeve bölümü. Anlam: iç köşe kavisini gösteren bağlantı satırının başlığı.
settings-frame-inner-corners-label = İç köşeler

# Nerede: Shell sayfası, İç köşeler satırının arama sözcükleri. Anlam: "iç kavis", "radius" terimleriyle satır bulunur.
settings-frame-inner-corners-keywords = iç kavis, köşe, radius, yuvarlak, inner_radius

## Bar sayfası

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: barın konum ayarlarını toplayan bölüm.
settings-section-position = Konum

# Nerede: Bar sayfası, Konum bölümü. Anlam: barın hangi ekran kenarında olduğunu seçen ayarın başlığı.
settings-bar-edge-label = Ekran kenarı

# Nerede: Bar sayfası, Ekran kenarı ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-edge-description = Bar'ın bulunduğu ekran kenarı

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: barın boyut ayarlarını toplayan bölüm.
settings-section-size = Boyut

# Nerede: Bar sayfası, Uç boşluğu ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-padding-description = Bar'ın iki ucundaki iç boşluk

# Nerede: Bar sayfası, Boyut bölümü. Anlam: widget'lar arası boşluğu ayarlayan denetimin başlığı.
settings-bar-spacing-label = Widget aralığı

# Nerede: Bar sayfası, Widget aralığı ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-spacing-description = Widget'lar arasındaki boşluk

# Nerede: Bar sayfası, Widget aralığı ayarının arama sözcükleri. Anlam: "spacing", "boşluk" terimleriyle ayar bulunur.
settings-bar-spacing-keywords = spacing, boşluk

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: bar bölümlerini düzenleyen satırı toplayan bölüm.
settings-section-widgets = Widget'lar

# Nerede: Bar sayfası, Widget'lar bölümü. Anlam: bar bölümlerini düzenleyen düzenleyicinin başlığı.
settings-bar-sections-label = Bar bölümleri

# Nerede: Bar sayfası, Bar bölümleri ayarının altındaki açıklama. Anlam: düzenleyicinin nasıl kullanıldığını anlatır.
settings-bar-sections-description = Widget ekle, çıkar ve sürükleyerek sırala

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: bar uçlarına tık ve kaydırma eylemi atan ayarları toplayan bölüm.
settings-section-edge-actions = Uç eylemleri

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: baştaki widget'a sol tık eylemi seçen ayarın başlığı.
settings-bar-start-left-click-label = Baştaki widget · sol tık

# Nerede: Bar sayfası, baştaki widget sol tık ayarının altındaki açıklama. Anlam: hangi widget'ın "baş" olduğunu anlatır.
settings-bar-start-left-click-description = Yatay bar'da en soldaki, dikeyde en üstteki widget

# Nerede: Bar sayfası, baştaki ve sondaki widget sol tık ayarlarının arama sözcükleri. Anlam: "panel aç" gibi terimlerle ayar bulunur.
settings-bar-edge-click-open-keywords = uç, kenar, tık, eylem, panel aç

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: baştaki widget'a sağ tık eylemi seçen ayarın başlığı.
settings-bar-start-right-click-label = Baştaki widget · sağ tık

# Nerede: Bar sayfası, baştaki ve sondaki widget sağ tık ayarlarının arama sözcükleri. Anlam: "tık", "eylem" terimleriyle ayar bulunur.
settings-bar-edge-click-keywords = uç, kenar, tık, eylem

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: baştaki widget'a kaydırma eylemi seçen ayarın başlığı.
settings-bar-start-scroll-label = Baştaki widget · kaydırma

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: sondaki widget'a sol tık eylemi seçen ayarın başlığı.
settings-bar-end-left-click-label = Sondaki widget · sol tık

# Nerede: Bar sayfası, sondaki widget sol tık ayarının altındaki açıklama. Anlam: hangi widget'ın "son" olduğunu anlatır.
settings-bar-end-left-click-description = Yatay bar'da en sağdaki, dikeyde en alttaki widget

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: sondaki widget'a sağ tık eylemi seçen ayarın başlığı.
settings-bar-end-right-click-label = Sondaki widget · sağ tık

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: sondaki widget'a kaydırma eylemi seçen ayarın başlığı.
settings-bar-end-scroll-label = Sondaki widget · kaydırma

# Nerede: Bar sayfası, baştaki ve sondaki widget kaydırma ayarlarının altındaki açıklama. Anlam: kaydırma eyleminin geri bildirimini anlatır.
settings-bar-edge-scroll-description = Ses, mikrofon ya da parlaklıkta widget'ın dış yanında ikon belirir

# Nerede: Bar sayfası, baştaki ve sondaki widget kaydırma ayarlarının arama sözcükleri. Anlam: "tekerlek", "ses" gibi terimlerle ayar bulunur.
settings-bar-edge-scroll-keywords = uç, kenar, tekerlek, scroll, ses, parlaklık, workspace

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: bir kaydırma adımının değiştirdiği yüzeyi ayarlayan denetimin başlığı.
settings-bar-scroll-step-label = Kaydırma adımı

# Nerede: Bar sayfası, Kaydırma adımı ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-scroll-step-description = Bir tekerlek çentiğinin değiştirdiği yüzde

# Nerede: Bar sayfası, Kaydırma adımı ayarının arama sözcükleri. Anlam: "adım", "tekerlek" terimleriyle ayar bulunur.
settings-bar-scroll-step-keywords = adım, tekerlek, ses, parlaklık

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: widget'a özel ayarları toplayan bölüm.
settings-section-widget-settings = Widget ayarları

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: sayfa başına workspace sayısını ayarlayan denetimin başlığı.
settings-bar-workspaces-count-label = Sayfa başına workspace

# Nerede: Bar sayfası, Sayfa başına workspace ayarının altındaki açıklama. Anlam: değerin nasıl çalıştığını anlatır.
settings-bar-workspaces-count-description = Her zaman gösterilen yuva sayısı; fazlası oklarla sonraki sayfada

# Nerede: Bar sayfası, Sayfa başına workspace ayarının arama sözcükleri. Anlam: "çalışma alanı" terimiyle ayar bulunur.
settings-bar-workspaces-count-keywords = çalışma alanı, sayı
## Bar sayfası — Widget ayarları

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: workspace'lerin (çalışma alanlarının) nasıl gösterileceğini seçen ayarın başlığı.
settings-bar-workspaces-style-label = Workspace görünümü

# Nerede: Bar sayfası, Workspace görünümü ayarının altındaki açıklama. Anlam: seçilebilecek gösterim biçimlerini sıralar.
settings-bar-workspaces-style-description = Uygulama ikonu, numara ya da nokta

# Nerede: Bar sayfası, Workspace görünümü ayarının arama sözcükleri. Anlam: "çalışma alanı", "ikon" gibi terimlerle ayar bulunur.
settings-bar-workspaces-style-keywords = çalışma alanı, ikon, numara, nokta

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: widget üstünde kaydırınca workspace değiştirmeyi ayarlayan denetimin başlığı.
settings-bar-workspaces-scroll-label = Workspace kaydırma

# Nerede: Bar sayfası, Workspace kaydırma ayarının altındaki açıklama. Anlam: kaydırma eyleminin nerede çalıştığını söyler.
settings-bar-workspaces-scroll-description = Widget üstünde tekerlekle workspace değiştirme

# Nerede: Bar sayfası, Workspace kaydırma ayarının arama sözcükleri. Anlam: "tekerlek", "scroll" terimleriyle ayar bulunur.
settings-bar-workspaces-scroll-keywords = çalışma alanı, tekerlek, scroll

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: bar saat widget'ının biçimini ayarlayan metin alanının başlığı.
settings-bar-clock-format-label = Saat biçimi

# Nerede: Bar sayfası, Saat biçimi ayarının altındaki açıklama. Anlam: kabul edilen strftime kalıplarına örnek verir.
settings-bar-clock-format-description = strftime: %H:%M, %a %d %b %H:%M…

# Nerede: Bar sayfası, Saat biçimi ayarının arama sözcükleri. Anlam: "saat", "format" terimleriyle ayar bulunur.
settings-bar-clock-format-keywords = saat, tarih, format

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: sistem tepsisi ikonlarının boyutunu ayarlayan denetimin başlığı.
settings-bar-tray-icon-size-label = Tepsi ikon boyutu

# Nerede: Bar sayfası, Tepsi ikon boyutu ayarının altındaki açıklama. Anlam: değerin hangi ikonları etkilediğini söyler.
settings-bar-tray-icon-size-description = Sistem tepsisindeki ikonlar

# Nerede: Bar sayfası, Tepsi ikon boyutu ayarının arama sözcükleri. Anlam: "tray" terimiyle ayar bulunur.
settings-bar-tray-icon-size-keywords = tray, ikon

## Shell sayfası — Paneller

# Nerede: Shell sayfasındaki bölüm başlığı. Anlam: dört kenardaki panel ayarlarını toplayan bölüm.
settings-section-panels = Paneller

# Nerede: Shell sayfası, Paneller bölümü. Anlam: sağ paneli açıp kapatan ayarın başlığı.
settings-panels-right-label = Sağ panel

# Nerede: Shell sayfası, Sağ panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-right-description = Ekranın sağ kenarından açılan çekmece

# Nerede: Shell sayfası, dört kenar panelini açıp kapatan ayarlarının arama sözcükleri. Anlam: "çekmece", "kenar paneli" terimleriyle ayarlar bulunur.
settings-panels-enabled-keywords = çekmece, drawer, kenar paneli

# Nerede: Shell sayfası, dört kenar panelinin derinlik ayarlarının altındaki açıklama. Anlam: değerin yönelime göre genişlik ya da yükseklik olduğunu anlatır.
settings-panels-size-description = Panelin genişliği (üst/alt panelde yüksekliği)

# Nerede: Shell sayfası, dört kenar panelinin derinlik ayarlarının arama sözcükleri. Anlam: "genişlik", "boyut" terimleriyle ayarlar bulunur.
settings-panels-size-keywords = genişlik, boyut

# Nerede: Shell sayfası, dört kenar panelinin içerik ayarlarının altındaki açıklama. Anlam: düzenleyicinin nasıl kullanıldığını anlatır.
settings-panels-widgets-description = Kart ekle, çıkar ve sürükleyerek sırala (yukarıdan aşağı)

# Nerede: Shell sayfası, dört kenar panelinin içerik ayarlarının arama sözcükleri. Anlam: "kart", "hızlı ayarlar" terimleriyle ayarlar bulunur.
settings-panels-widgets-keywords = kart, widget, hızlı ayarlar, medya, bildirim, sıra

# Nerede: Shell sayfası, Paneller bölümü. Anlam: sol paneli açıp kapatan ayarın başlığı.
settings-panels-left-label = Sol panel

# Nerede: Shell sayfası, Sol panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-left-description = Ekranın sol kenarından açılan çekmece

# Nerede: Shell sayfası, Paneller bölümü. Anlam: üst paneli açıp kapatan ayarın başlığı.
settings-panels-top-label = Üst panel

# Nerede: Shell sayfası, Üst panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-top-description = Ekranın üst kenarından açılan çekmece

# Nerede: Shell sayfası, Paneller bölümü. Anlam: alt paneli açıp kapatan ayarın başlığı.
settings-panels-bottom-label = Alt panel

# Nerede: Shell sayfası, Alt panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-bottom-description = Ekranın alt kenarından açılan çekmece

## Ekranlar sayfası

# Nerede: Ekranlar sayfasındaki bölüm başlığı. Anlam: monitör düzeni ve görüntü ayarlarını toplayan bölüm.
settings-section-displays = Monitörler

# Nerede: Ekranlar sayfasındaki ayar satırının başlığı; satır ekran düzenleyiciyi açar. Anlam: monitör yerleşimi ve görüntü ayarları.
settings-displays-label = Monitörler

# Nerede: Ekranlar sayfası, Monitörler satırının altındaki açıklama. Anlam: düzenleyicideki ayarları özetler.
settings-displays-description = Yerleşim, çözünürlük, yenileme hızı, ölçek, döndürme, yansıtma

# Nerede: Ekranlar sayfası, Monitörler satırının arama sözcükleri. Anlam: "çözünürlük", "yansıt" gibi terimlerle sayfa bulunur.
settings-displays-keywords = ekran, monitör, çözünürlük, yenileme, hz, ölçek, döndür, yansıt, vrr, birincil, hdmi, dp

## Pencereler sayfası

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: pencere aralığı ayarlarını toplayan bölüm.
settings-section-gaps = Aralıklar

# Nerede: Pencereler sayfası, Pencereler arası ayarının arama sözcükleri. Anlam: "boşluk", "döşeme" terimleriyle ayar bulunur.
settings-windows-gap-inner-keywords = boşluk, gap, aralık, döşeme

# Nerede: Pencereler sayfası, Ekran kenarına ayarının arama sözcükleri. Anlam: "kenar", "gap" terimleriyle ayar bulunur.
settings-windows-gap-outer-keywords = boşluk, gap, aralık, kenar

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: pencere kenarlığı ayarını toplayan bölüm.
settings-section-border = Kenarlık

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: pencere bulanıklığı ayarlarını toplayan bölüm.
settings-section-blur = Bulanıklık

# Nerede: Pencereler sayfası, bulanıklık yoğunluğu ayarının arama sözcükleri. Anlam: "yarıçap" terimiyle ayar bulunur.
settings-windows-blur-size-keywords = blur, bulanık, yarıçap

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: temayla paylaşılan görünüm ayarlarını toplayan bölüm.
settings-section-theme-shared = Temayla ortak

# Nerede: Pencereler sayfası, Gölge satırının arama sözcükleri. Anlam: "gölge", "derinlik" terimleriyle satır bulunur.
settings-windows-shadow-keywords = gölge, shadow, derinlik

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: masaüstü eşitleme ayarlarını toplayan bölüm.
settings-section-advanced = Gelişmiş

# Nerede: Pencereler sayfası, Gelişmiş bölümü. Anlam: köşe, gölge ve bulanıklığın temadan alınmasını açıp kapatan ayarın başlığı.
settings-compositor-sync-look-label = Pencereler temaya uysun

# Nerede: Pencereler sayfası, Pencereler temaya uysun ayarının arama sözcükleri. Anlam: "tema", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-look-keywords = tema, köşe, gölge, bulanıklık, eşitle

# Nerede: Pencereler sayfası, Aralık ve kenarlığı Nuxut belirlesin ayarının arama sözcükleri. Anlam: "aralık", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-windows-keywords = aralık, kenarlık, gap, eşitle

# Nerede: Pencereler sayfası, Gelişmiş bölümü. Anlam: pencere animasyonlarının temadan alınmasını açıp kapatan ayarın başlığı.
settings-compositor-sync-motion-label = Pencere hareketleri temaya uysun

# Nerede: Pencereler sayfası, Pencere hareketleri temaya uysun ayarının arama sözcükleri. Anlam: "animasyon", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-motion-keywords = animasyon, hareket, eşitle

## Duvar kağıdı sayfası

# Nerede: Duvar kağıdı sayfasındaki bölüm başlığı. Anlam: duvar kağıdı ayarlarını toplayan bölüm.
settings-section-wallpaper = Duvar kağıdı

# Nerede: Duvar kağıdı sayfasındaki ayar satırının başlığı; satır kağıt seçimini açar. Anlam: duvar kağıdı görseli.
settings-wallpaper-label = Duvar kağıdı

# Nerede: Duvar kağıdı sayfası, Duvar kağıdı bölümü. Anlam: görselin ekrana nasıl sığdırılacağını seçen ayarın başlığı.
settings-wallpaper-fit-label = Yerleşim

# Nerede: Duvar kağıdı sayfası, Yerleşim ayarının arama sözcükleri. Anlam: "sığdır", "kırp" terimleriyle ayar bulunur.
settings-wallpaper-fit-keywords = sığdır, doldur, kırp, ölçek

# Nerede: Duvar kağıdı sayfası, Duvar kağıdı bölümü. Anlam: monitör başına kağıt ayarını açan satırın başlığı.
settings-wallpaper-outputs-label = Monitör başına

# Nerede: Duvar kağıdı sayfası, Monitör başına satırının altındaki açıklama. Anlam: ayarın kapsamını anlatır.
settings-wallpaper-outputs-description = Bir monitöre ayrı görsel ya da yerleşim

# Nerede: Duvar kağıdı sayfası, Monitör başına satırının arama sözcükleri. Anlam: "monitör", "çoklu" terimleriyle satır bulunur.
settings-wallpaper-outputs-keywords = monitör, ekran, çoklu, bağlayıcı, hdmi, dp

# Nerede: Duvar kağıdı sayfasındaki bölüm başlığı. Anlam: renk şeması ayarlarını toplayan bölüm.
settings-section-color-scheme = Renk şeması

# Nerede: Duvar kağıdı sayfası, Renk şeması bölümü. Anlam: arayüz renklerinin duvar kağıdından türetilmesini seçen ayarın başlığı.
settings-palette-source-label = Renkleri duvar kağıdından al

# Nerede: Duvar kağıdı sayfası, Renk şeması bölümü. Anlam: duvar kağıdından türeyen palet varyantını seçen ayarın başlığı.
settings-palette-label = Şema

# Nerede: Duvar kağıdı sayfası, Şema ayarının altındaki açıklama. Anlam: seçilebilecek palet çeşitliliğini anlatır.
settings-palette-scheme-description = Aynı duvar kağıdından dokuz farklı palet

# Nerede: Duvar kağıdı sayfası, Şema ayarının arama sözcükleri. Anlam: "palet", "tonal" terimleriyle ayar bulunur.
settings-palette-scheme-keywords = matugen, palet, varyant, tonal, canlı

# Nerede: Duvar kağıdı sayfası, Kip ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-palette-mode-description = Koyu ya da açık arayüz

## Hesaplar sayfası

# Nerede: Hesaplar sayfasındaki bölüm başlığı. Anlam: takvim hesabı ayarlarını toplayan bölüm.
settings-section-accounts = Takvim hesapları

# Nerede: Hesaplar sayfasındaki ayar satırının başlığı; satır hesap yöneticisini açar. Anlam: takvim hesaplarını yöneten düzenleyici.
settings-accounts-label = Takvim hesapları

# Nerede: Hesaplar sayfası, Takvim hesapları satırının altındaki açıklama. Anlam: desteklenen hesap türlerini sıralar.
settings-accounts-description = Google, Microsoft, Nextcloud, CalDAV, ICS

# Nerede: Hesaplar sayfası, Takvim hesapları satırının arama sözcükleri. Anlam: "takvim", "outlook" terimleriyle sayfa bulunur.
settings-accounts-keywords = takvim, google, outlook, nextcloud, icloud, caldav

## Bildirimler sayfası

# Nerede: Bildirimler sayfası, Genel bölümü. Anlam: shell bildirim sunucusunu açıp kapatan ayarın başlığı.
settings-notifications-label = Bildirimler

# Nerede: Bildirimler sayfası, Bildirimler ayarının altındaki açıklama. Anlam: kapattığında ne olacağını anlatır.
settings-notifications-description = Kapalıyken shell bildirim sunucusu olmaz (başka bir sunucu çalışabilir)

# Nerede: Bildirimler sayfası, Bildirimler ayarının arama sözcükleri. Anlam: "bildirim sunucusu" terimiyle ayar bulunur.
settings-notifications-keywords = bildirim sunucusu

# Nerede: Bildirimler sayfası, Genel bölümü. Anlam: açılır bildirimleri susturan ayarın başlığı.
settings-notifications-dnd-label = Rahatsız etme

# Nerede: Bildirimler sayfası, Rahatsız etme ayarının arama sözcükleri. Anlam: "dnd", "sessiz" terimleriyle ayar bulunur.
settings-notifications-dnd-keywords = dnd, sessiz, sustur

# Nerede: Bildirimler sayfasındaki bölüm başlığı. Anlam: açılır bildirim ayarlarını toplayan bölüm.
settings-section-popup-notifications = Açılır bildirimler

# Nerede: Bildirimler sayfası açılır bildirim ayarında ve Ses sayfası ekran göstergesi ayarında kaydırıcı başlığı. Anlam: öğenin ekranda kalma süresi.
settings-display-duration-label = Görünme süresi

# Nerede: Bildirimler sayfası, Görünme süresi ayarının altındaki açıklama. Anlam: değerin hangi bildirimler için geçerli olduğunu söyler.
settings-notifications-popup-timeout-description = Normal bildirimlerin ekranda kalma süresi

# Nerede: Bildirimler sayfası, Görünme süresi ayarının arama sözcükleri. Anlam: "popup", "zaman aşımı" terimleriyle ayar bulunur.
settings-notifications-popup-timeout-keywords = popup, süre, zaman aşımı

# Nerede: Bildirimler sayfası, Açılır bildirimler bölümü. Anlam: aynı anda görünecek açılır bildirim sayısını sınırlayan ayarın başlığı.
settings-notifications-max-popups-label = Aynı anda en çok

# Nerede: Bildirimler sayfası, Aynı anda en çok ayarının arama sözcükleri. Anlam: "popup", "yığın" terimleriyle ayar bulunur.
settings-notifications-max-popups-keywords = popup, sayı, yığın

# Nerede: Bildirimler sayfasındaki bölüm başlığı. Anlam: bildirim geçmişi ayarlarını toplayan bölüm.
settings-section-history = Geçmiş

# Nerede: Bildirimler sayfası, Geçmiş bölümü. Anlam: merkezde saklanan bildirim sayısını sınırlayan ayarın başlığı.
settings-notifications-history-limit-label = Saklanan bildirim

# Nerede: Bildirimler sayfası, Saklanan bildirim ayarının altındaki açıklama. Anlam: değerin nerede geçerli olduğunu söyler.
settings-notifications-history-limit-description = Bildirim merkezinde tutulan en çok bildirim

# Nerede: Bildirimler sayfası, Saklanan bildirim ayarının arama sözcükleri. Anlam: "geçmiş", "history" terimleriyle ayar bulunur.
settings-notifications-history-limit-keywords = geçmiş, merkez, history

# Nerede: Bildirimler sayfasındaki bölüm başlığı. Anlam: uygulama bazlı bildirim ayarlarını toplayan bölüm.
settings-section-apps = Uygulamalar

# Nerede: Bildirimler sayfası, Uygulamalar bölümü. Anlam: uygulama bazlı bildirim ayarını açan satırın başlığı.
settings-notifications-apps-label = Uygulama başına

# Nerede: Bildirimler sayfası, Uygulama başına satırının altındaki açıklama. Anlam: satırın ne yaptığını anlatır.
settings-notifications-apps-description = Bir uygulamanın açılır bildirimlerini ya da geçmişini kapat

# Nerede: Bildirimler sayfası, Uygulama başına satırının arama sözcükleri. Anlam: "popup", "uygulama" terimleriyle satır bulunur.
settings-notifications-apps-keywords = popup, uygulama

## Ses sayfası

# Nerede: Kısayollar sayfasında ses kısayollarının bölüm başlığı. Anlam: ses ayarlarını toplayan bölüm.
settings-section-sound = Ses

# Nerede: Ses sayfasının ilk kartının bölüm başlığı (en yüksek ses, uyarı, ses ve parlaklık göstergesi). Anlam: sayfa adı "Ses"i tekrarlamadan ses düzeyiyle ilgili ayarlar.
settings-section-sound-level = Ses düzeyi

# Nerede: Ses sayfası, Ses bölümü. Anlam: sistem genelinde en yüksek ses düzeyini sınırlayan ayarın başlığı.
settings-audio-max-volume-label = En yüksek ses

# Nerede: Ses sayfası, En yüksek ses ayarının arama sözcükleri. Anlam: "volume", "boost" terimleriyle ayar bulunur.
settings-audio-max-volume-keywords = volume, yükseltme, boost, amplify

# Nerede: Ses sayfası, Ses bölümü. Anlam: ses yükseltmenin (boost) riskini anlatan uyarı satırının başlığı.
settings-audio-max-volume-warning-label = Uyarı

# Nerede: Ses sayfası, Uyarı satırının altındaki açıklama. Anlam: 100 üstü yükseltmenin olası zararını söyler.
settings-audio-max-volume-warning-description = 100 üstü yükseltme sesi bozabilir ve hoparlöre zarar verebilir

# Nerede: Ses sayfası, Uyarı satırının arama sözcükleri. Anlam: "volume" terimiyle satır bulunur.
settings-audio-max-volume-warning-keywords = volume

# Nerede: Ses sayfasındaki bölüm başlığı. Anlam: ekran göstergesi (OSD) ayarlarını toplayan bölüm.
settings-section-osd = Ekran göstergesi

# Nerede: Ses sayfası, Görünme süresi ayarının altındaki açıklama. Anlam: değerin hangi göstergeler için geçerli olduğunu söyler.
settings-osd-timeout-description = Ses ya da parlaklık değişince beliren göstergenin ekranda kalma süresi

# Nerede: Ses sayfası, ekran göstergesi Görünme süresi ayarının arama sözcükleri. Anlam: "osd", "gösterge" terimleriyle ayar bulunur.
settings-osd-timeout-keywords = osd, gösterge, süre

# Nerede: Ses sayfasındaki bölüm başlığı. Anlam: medya tuşu ayarlarını toplayan bölüm.
settings-section-media-keys = Medya tuşları

# Nerede: Ses sayfası, Medya tuşları bölümü. Anlam: donanım tuşlarını Kısayollar sayfasına bağlayan satırın başlığı.
settings-media-keys-link-label = Ses ve medya tuşları

# Nerede: Ses sayfası, Ses ve medya tuşları satırının arama sözcükleri. Anlam: "xf86", "bind" terimleriyle satır bulunur.
settings-media-keys-link-keywords = xf86, kısayol, tuş, bind, parlaklık, mikrofon

# Nerede: Ses sayfası, Medya tuşları bölümü. Anlam: Hyprland'e elle bind ekleme örneğini gösteren satırın başlığı.
settings-media-keys-example-label = Hyprland bind örneği

# Nerede: Ses sayfası, Hyprland bind örneği satırının altındaki açıklama. Anlam: Nuxut kısayolları kapalıyken ne yapılması gerektiğini söyler.
settings-media-keys-example-description = Nuxut kısayolları kapalı; isterseniz hyprland.conf'a kendiniz ekleyin

# Nerede: Ses sayfası, Hyprland bind örneği satırının arama sözcükleri. Anlam: "playerctl", "bind" terimleriyle satır bulunur.
settings-media-keys-example-keywords = xf86, kısayol, bind, playerctl, parlaklık, brightness, mikrofon

## Klavye sayfası ve girdi durumu

# Nerede: Klavye sayfası, düzen listesinin altındaki metin düğme; seçici kapalıyken (src/pages/keyboard.rs). Anlam: Yeni düzen eklemek için arama alanlı seçiciyi açar.
settings-keyboard-add = Düzen ekle

# Nerede: Klavye sayfası, düzen listesi boşken görünen açıklama; etkin liste okunamadığında (src/pages/keyboard.rs). Anlam: Etkin düzenlerin okunamadığı, kullanıcının eklediği düzenlerin yine de yazılacağı.
settings-keyboard-empty-effective = Etkin düzen okunamadı; eklediğin düzenler yazılır.

# Nerede: Klavye sayfası, düzen listesi doluyken "Düzen ekle" düğmesinin yerine geçen açıklama (src/pages/keyboard.rs). Anlam: Listeye en çok bu kadar düzen eklenebileceği.
# Değişkenler: $count (en çok düzen sayısı, ör. 4).
settings-keyboard-max = En fazla { $count } düzen

# Nerede: Klavye sayfası, "Düzen ekle" seçicisinin arama alanının yer tutucusu (src/pages/keyboard.rs). Anlam: Düzenlerin dil, ülke ya da kodla aranabileceği.
settings-keyboard-search-placeholder = Düzen ara: dil, ülke ya da kod (tr, us, dvorak)

# Nerede: Klavye sayfası, seçici açıkken XKB kataloğu henüz yüklenmemişse görünen durum yazısı (src/pages/keyboard.rs). Anlam: Düzen listesinin yüklenmekte olduğu.
settings-keyboard-loading = Düzen listesi yükleniyor…

# Nerede: Klavye sayfası, seçici açıkken sorguya uyan düzen bulunamadığında görünen durum yazısı (src/pages/keyboard.rs). Anlam: Sorguya eşleşen düzen olmadığı.
settings-keyboard-no-match = Eşleşen düzen yok

# Nerede: Klavye sayfası, Düzenler arası geçiş satırındaki seçim menüsünün ilk seçeneği (src/pages/keyboard.rs). Anlam: Düzen değiştirme seçeneğinin olmadığı.
settings-keyboard-switch-none = Yok

# Nerede: Klavye sayfası, tuş tekrarını denemek için yerel metin alanının yer tutucusu (src/pages/keyboard.rs). Anlam: Alanın deneme amaçlı olduğu, bir tuşun basılı tutularak tekrarın hissedilebileceği.
settings-keyboard-test-placeholder = Burada deneyin: bir tuşu basılı tutun

# Nerede: Klavye ve Fare sayfalarının üstündeki sakin not; girdi eşitlemesi kapalıyken (src/pages/input_note.rs). Anlam: Girdi ayarlarının masaüstüne uygulanmadığı, masaüstünün kendi ayarının geçerli olduğu.
settings-input-sync-off = Klavye ve fare ayarları masaüstüne uygulanmıyor; masaüstünün kendi ayarı geçerli.

# Nerede: Aynı notun sağındaki tonlu düğme (src/pages/input_note.rs). Anlam: Girdi eşitleme anahtarını varsayılanına (açık) döndürür.
settings-input-turn-on = Aç

# Nerede: Klavye ve Fare sayfalarının üstündeki sakin not; shell çalışıyor ama masaüstüne (Hyprland'e) ulaşamıyor (src/services/input.rs). Anlam: Değerlerin varsayılanla gösterildiği, değişikliklerin masaüstü açılınca uygulanacağı.
settings-input-unavailable = Masaüstüne ulaşılamadı; değerler varsayılanla gösteriliyor, değişiklikler masaüstü açılınca uygulanır.

# Nerede: Aynı not; masaüstü kabuğu çalışmıyorken (src/services/input.rs). Anlam: Değerlerin kabuk açılınca uygulanacağı.
settings-input-not-running = Masaüstü kabuğu çalışmıyor; değerler kabuk açılınca uygulanır.

# Nerede: Aynı not; shell sürümü girdi durumunu bildirmiyorken (src/services/input.rs). Anlam: Kabuğun bu sürümünün girdi ayarlarını bildirmediği, değerlerin varsayılanla gösterildiği.
settings-input-unsupported = Kabuğun bu sürümü girdi ayarlarını bildirmiyor; değerler varsayılanla gösteriliyor.

# Nerede: Klavye ve Fare sayfalarında satırın altındaki geçersiz kılma uyarısı; dosya ve satır biliniyorken (src/services/input.rs). Anlam: Hyprland ayar dosyasındaki satırın bu değeri geçersiz kıldığı.
# Değişkenler: $file (dosya yolu, ör. "~/.config/hypr/hyprland.conf"), $line (satır numarası, ör. 12).
settings-input-override-file-line = Hyprland ayar dosyan bu değeri geçersiz kılıyor: { $file }:{ $line }

# Nerede: Aynı uyarı; dosya biliniyor ama satır bilinmiyorken (src/services/input.rs). Anlam: Hyprland ayar dosyasının bu değeri geçersiz kıldığı.
# Değişkenler: $file (dosya yolu, ör. "~/.config/hypr/hyprland.lua").
settings-input-override-file = Hyprland ayar dosyan bu değeri geçersiz kılıyor: { $file }

# Nerede: Aynı uyarı; dosya bilinmiyorken (Lua config), aranacak Hyprland anahtarı söylenirken (src/services/input.rs). Anlam: Hyprland ayarının bu değeri geçersiz kıldığı ve hangi anahtarın olduğu.
# Değişkenler: $key (Hyprland anahtarı, ör. "input:sensitivity").
settings-input-override-hypr = Hyprland ayarın bu değeri geçersiz kılıyor ({ $key }).

# Nerede: Aynı uyarı; ne dosya ne Hyprland anahtarı biliniyorken (src/services/input.rs). Anlam: Hyprland ayarının bu değeri geçersiz kıldığı.
settings-input-override = Hyprland ayarın bu değeri geçersiz kılıyor.

## Klavye sayfası

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: klavye ayarlarının uygulanma durumunu gösteren bölüm.
settings-section-status = Durum

# Nerede: Klavye sayfası, Durum bölümü. Anlam: klavye ayarlarının uygulanıp uygulanmadığını bildiren satırın başlığı.
settings-keyboard-note-label = Klavye ayarlarının durumu

# Nerede: Klavye sayfası, Klavye ayarlarının durumu satırının arama sözcükleri. Anlam: "masaüstü", "uygulanmıyor" terimleriyle satır bulunur.
settings-keyboard-note-keywords = masaüstü, kabuk, uygulanmıyor

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: klavye düzeni ayarlarını toplayan bölüm.
settings-section-layouts = Düzenler

# Nerede: Klavye sayfası, Düzenler bölümü. Anlam: etkin klavye düzenlerini seçen ayarın başlığı.
settings-input-keyboard-layouts-label = Klavye düzenleri

# Nerede: Klavye sayfası, Klavye düzenleri ayarının arama sözcükleri. Anlam: "layout", "xkb", "q klavye" terimleriyle ayar bulunur.
settings-input-keyboard-layouts-keywords = klavye, düzen, layout, xkb, varyant, variant, kb_layout, dil, q klavye, f klavye

# Nerede: Klavye sayfası, Düzenler bölümü. Anlam: düzenler arasında geçiş yapan tuş birleşiminin başlığı.
settings-input-keyboard-switch-label = Düzenler arası geçiş

# Nerede: Klavye sayfası, Düzenler arası geçiş ayarının altındaki açıklama. Anlam: düzeni hangi tuş birleşimiyle değiştireceğini söyler.
settings-input-keyboard-switch-description = Düzeni değiştiren tuş birleşimi

# Nerede: Klavye sayfası, Düzenler arası geçiş ayarının arama sözcükleri. Anlam: "grp", "alt shift" terimleriyle ayar bulunur.
settings-input-keyboard-switch-keywords = klavye, düzen, geçiş, değiştir, grp, kb_options, alt shift, layout

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: tuş tekrarı ayarlarını toplayan bölüm.
settings-section-key-repeat = Tuş tekrarı

# Nerede: Klavye sayfası, Tuş tekrarı bölümü. Anlam: basılı tutulan tuşun bekleyip tekrarlamaya başladığı süreyi belirleyen ayarın başlığı.
settings-input-keyboard-repeat-delay-label = Tekrar gecikmesi

# Nerede: Klavye sayfası, Tekrar gecikmesi ayarının altındaki açıklama. Anlam: gecikmenin ne anlama geldiğini söyler.
settings-input-keyboard-repeat-delay-description = Basılı tutulan tuş bu süreden sonra tekrarlar

# Nerede: Klavye sayfası, Tekrar gecikmesi ayarının arama sözcükleri. Anlam: "repeat", "delay", "basılı tut" terimleriyle ayar bulunur.
settings-input-keyboard-repeat-delay-keywords = klavye, tekrar, gecikme, repeat, delay, basılı tut

# Nerede: Klavye sayfası, Tuş tekrarı bölümü. Anlam: saniyedeki tekrar sayısını belirleyen ayarın başlığı.
settings-input-keyboard-repeat-rate-label = Tekrar hızı

# Nerede: Klavye sayfası, Tekrar hızı ayarının altındaki açıklama. Anlam: hızın ölçüsünü açıklar.
settings-input-keyboard-repeat-rate-description = Saniyedeki tekrar sayısı

# Nerede: Klavye sayfası, Tekrar hızı ayarının arama sözcükleri. Anlam: "repeat", "rate", "hız" terimleriyle ayar bulunur.
settings-input-keyboard-repeat-rate-keywords = klavye, tekrar, hız, repeat, rate

# Nerede: Klavye sayfası, Tuş tekrarı bölümü. Anlam: tuş tekrarını denemek için yazı alanının başlığı.
settings-input-keyboard-try-label = Burada deneyin

# Nerede: Klavye sayfası, Burada deneyin satırının arama sözcükleri. Anlam: "deneme", "test" terimleriyle satır bulunur.
settings-input-keyboard-try-keywords = deneme, dene, test, tekrar

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: diğer klavye ayarlarını toplayan bölüm.
settings-section-other = Diğer

# Nerede: Klavye sayfası, Diğer bölümü. Anlam: oturum açılışında Num Lock davranışını seçen ayarın başlığı.
settings-input-keyboard-numlock-by-default-label = Açılışta Num Lock açık

# Nerede: Klavye sayfası, Açılışta Num Lock açık ayarının altındaki açıklama. Anlam: açıkken oturum açılınca ne olacağını söyler.
settings-input-keyboard-numlock-by-default-description = Oturum açılınca sayı tuşları etkin olur

# Nerede: Klavye sayfası, Açılışta Num Lock açık ayarının arama sözcükleri. Anlam: "numlock", "sayı tuşları" terimleriyle ayar bulunur.
settings-input-keyboard-numlock-by-default-keywords = numlock, num lock, sayı tuşları, klavye

## Fare sayfası

# Nerede: Fare ve dokunmatik yüzey sayfası, Durum bölümü (Klavye sayfasıyla ortak settings-section-status). Anlam: fare ve dokunmatik yüzey ayarlarının uygulanıp uygulanmadığını bildiren satırın başlığı.
settings-pointer-note-label = Fare ve dokunmatik yüzey durumu

# Nerede: Fare ve dokunmatik yüzey sayfası, Fare ve dokunmatik yüzey durumu satırının arama sözcükleri. Anlam: "masaüstü", "uygulanmıyor" terimleriyle satır bulunur.
settings-pointer-note-keywords = masaüstü, kabuk, uygulanmıyor

# Nerede: Fare ve dokunmatik yüzey sayfası, Genel bölümü (Görünüm ve Yazı sayfalarıyla ortak settings-section-general). Anlam: işaretçi hızını belirleyen ayarın başlığı.
settings-input-mouse-sensitivity-label = İşaretçi hızı

# Nerede: Fare ve dokunmatik yüzey sayfası, İşaretçi hızı ayarının altındaki açıklama. Anlam: ayarın hangi aygıtlar için geçerli olduğunu söyler.
settings-input-mouse-sensitivity-description = Fare ve dokunmatik yüzey için

# Nerede: Fare ve dokunmatik yüzey sayfası, İşaretçi hızı ayarının arama sözcükleri. Anlam: "fare", "hassasiyet", "sensitivity" terimleriyle ayar bulunur.
settings-input-mouse-sensitivity-keywords = fare, işaretçi, imleç, hız, hassasiyet, sensitivity, mouse, touchpad, dokunmatik

# Nerede: Fare ve dokunmatik yüzey sayfası, Genel bölümü. Anlam: işaretçi ivme profilini seçen ayarın başlığı.
settings-input-mouse-accel-profile-label = İvme

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının altındaki açıklama. Anlam: uyarlanır ivmenin ne yaptığını açıklar.
settings-input-mouse-accel-profile-description = Uyarlanır: hızlı hareket işaretçiyi daha uzağa götürür

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının arama sözcükleri. Anlam: "ivme", "accel", "düz" terimleriyle ayar bulunur.
settings-input-mouse-accel-profile-keywords = fare, ivme, hızlanma, acceleration, accel, düz, flat

# Nerede: Fare ve dokunmatik yüzey sayfası, Genel bölümü. Anlam: farenin doğal kaydırma davranışını seçen ayarın başlığı.
settings-input-mouse-natural-scroll-label = Doğal kaydırma

# Nerede: Fare ve dokunmatik yüzey sayfası, Fare için Doğal kaydırma ayarının altındaki açıklama. Anlam: doğal kaydırmanın fare tekerleğinde ne yaptığını söyler.
settings-input-mouse-natural-scroll-description = Tekerlek içeriği kaydırma yönünde taşır

# Nerede: Fare ve dokunmatik yüzey sayfası, Fare için Doğal kaydırma ayarının arama sözcükleri. Anlam: "tekerlek", "ters kaydırma" terimleriyle ayar bulunur.
settings-input-mouse-natural-scroll-keywords = fare, tekerlek, doğal kaydırma, natural scroll, ters kaydırma, mouse

# Nerede: Fare ve dokunmatik yüzey sayfasındaki bölüm başlığı. Anlam: yalnızca fareye ilişkin ayarları toplayan bölüm.
settings-section-mouse = Fare

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: dokunmatik yüzeyin doğal kaydırma davranışını seçen ayarın başlığı.
settings-input-touchpad-natural-scroll-label = Doğal kaydırma

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey için Doğal kaydırma ayarının altındaki açıklama. Anlam: doğal kaydırmanın dokunmatik yüzeyde ne yaptığını söyler.
settings-input-touchpad-natural-scroll-description = İçerik parmakların yönünde kayar

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey için Doğal kaydırma ayarının arama sözcükleri. Anlam: "touchpad", "ters kaydırma" terimleriyle ayar bulunur.
settings-input-touchpad-natural-scroll-keywords = dokunmatik, touchpad, doğal kaydırma, natural scroll, ters kaydırma

# Nerede: Fare ve dokunmatik yüzey sayfasındaki bölüm başlığı. Anlam: dokunmatik yüzey ayarlarını toplayan bölüm.
settings-section-touchpad = Dokunmatik yüzey

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: hafif dokunuşun tıklama sayılmasını seçen ayarın başlığı.
settings-input-touchpad-tap-to-click-label = Dokunarak tıkla

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunarak tıkla ayarının altındaki açıklama. Anlam: özelliğin nasıl çalıştığını söyler.
settings-input-touchpad-tap-to-click-description = Hafif dokunuş tıklama sayılır

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunarak tıkla ayarının arama sözcükleri. Anlam: "tap", "tıkla" terimleriyle ayar bulunur.
settings-input-touchpad-tap-to-click-keywords = dokunmatik, touchpad, tap, dokun, tıkla

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: yazarken dokunmatik yüzeyi kapatmayı seçen ayarın başlığı.
settings-input-touchpad-disable-while-typing-label = Yazarken devre dışı

# Nerede: Fare ve dokunmatik yüzey sayfası, Yazarken devre dışı ayarının altındaki açıklama. Anlam: özelliğin ne sağladığını açıklar.
settings-input-touchpad-disable-while-typing-description = Yazarken avuç içi dokunuşları yok sayılır

# Nerede: Fare ve dokunmatik yüzey sayfası, Yazarken devre dışı ayarının arama sözcükleri. Anlam: "yazarken", "palm", "dwt" terimleriyle ayar bulunur.
settings-input-touchpad-disable-while-typing-keywords = dokunmatik, touchpad, yazarken, avuç, palm, dwt

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: kaydırma çarpanını belirleyen ayarın başlığı.
settings-input-touchpad-scroll-factor-label = Kaydırma hızı

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma hızı ayarının arama sözcükleri. Anlam: "kaydırma", "scroll", "factor" terimleriyle ayar bulunur.
settings-input-touchpad-scroll-factor-keywords = dokunmatik, touchpad, kaydırma, hız, scroll, factor

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: kaydırma yöntemini seçen ayarın başlığı.
settings-input-touchpad-scroll-method-label = Kaydırma yöntemi

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının arama sözcükleri. Anlam: "iki parmak", "kenar", "scroll method" terimleriyle ayar bulunur.
settings-input-touchpad-scroll-method-keywords = dokunmatik, touchpad, kaydırma, iki parmak, kenar, scroll method

## Kısayollar sayfası

# Nerede: Kısayollar sayfası, Genel bölümü. Anlam: Nuxut kısayollarının masaüstü kısayollarını ezmesini açıp kapatan ayarın başlığı.
settings-compositor-sync-keybinds-label = Nuxut kısayolları

# Nerede: Kısayollar sayfası, Nuxut kısayolları ayarının arama sözcükleri. Anlam: "kısayol", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-keybinds-keywords = kısayol, tuş, bind, eşitle

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: sesi yükseltme kısayolunun adı.
settings-keybinds-volume-up-label = Sesi aç

# Nerede: Kısayollar sayfası, ses açma ve kısma kısayollarının arama sözcükleri. Anlam: "volume", "xf86" terimleriyle kısayollar bulunur.
settings-keybinds-volume-keywords = kısayol, tuş, volume, xf86

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: sesi kısaltma kısayolunun adı.
settings-keybinds-volume-down-label = Sesi kıs

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: sesi açıp kapatan kısayolun adı.
settings-keybinds-volume-mute-label = Sesi kapat / aç

# Nerede: Kısayollar sayfası, ses kapatma kısayolunun arama sözcükleri. Anlam: "sessiz", "mute" terimleriyle kısayol bulunur.
settings-keybinds-volume-mute-keywords = kısayol, tuş, sessiz, mute, xf86

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: mikrofonu açıp kapatan kısayolun adı.
settings-keybinds-mic-mute-label = Mikrofonu kapat / aç

# Nerede: Kısayollar sayfası, mikrofon kısayolunun arama sözcükleri. Anlam: "mikrofon", "mute" terimleriyle kısayol bulunur.
settings-keybinds-mic-mute-keywords = kısayol, tuş, mikrofon, mute, xf86

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: parlaklık kısayollarını toplayan bölüm.
settings-section-brightness = Parlaklık

# Nerede: Kısayollar sayfası, Parlaklık bölümü. Anlam: parlaklığı artırma kısayolunun adı.
settings-keybinds-brightness-up-label = Parlaklığı artır

# Nerede: Kısayollar sayfası, parlaklık artırma ve azaltma kısayollarının arama sözcükleri. Anlam: "brightness", "ekran" terimleriyle kısayollar bulunur.
settings-keybinds-brightness-keywords = kısayol, tuş, ekran, brightness, xf86

# Nerede: Kısayollar sayfası, Parlaklık bölümü. Anlam: parlaklığı azaltma kısayolunun adı.
settings-keybinds-brightness-down-label = Parlaklığı azalt

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: medya kısayollarını toplayan bölüm.
settings-section-media = Medya

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: oynatma/duraklatma kısayolunun adı.
settings-keybinds-media-play-label = Oynat / duraklat

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: klavyedeki ayrı duraklat tuşunun kısayolunun adı (shell keybinds.media_pause); oynatmayı duraklatır ya da sürdürür.
settings-keybinds-media-pause-label = Duraklat tuşu

# Nerede: Kısayollar sayfası, keybinds.media_pause satırının arama sözcükleri (virgüllü liste).
settings-keybinds-media-pause-keywords = kısayol, tuş, müzik, duraklat, playerctl, xf86

# Nerede: Kısayollar sayfası, medya kısayollarının arama sözcükleri. Anlam: "müzik", "playerctl" terimleriyle kısayollar bulunur.
settings-keybinds-media-keywords = kısayol, tuş, müzik, playerctl, xf86

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: sonraki parçaya geçme kısayolunun adı.
settings-keybinds-media-next-label = Sonraki parça

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: önceki parçaya geçme kısayolunun adı.
settings-keybinds-media-prev-label = Önceki parça

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: bar ve panel kısayollarını toplayan bölüm.
settings-section-bar-panels = Bar ve paneller

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: sağ paneli açıp kapatan kısayolun adı.
settings-keybinds-panel-right-label = Sağ paneli aç / kapat

# Nerede: Kısayollar sayfası, sağ panel kısayolunun arama sözcükleri. Anlam: "çekmece", "hızlı ayarlar" terimleriyle kısayol bulunur.
settings-keybinds-panel-right-keywords = kısayol, tuş, çekmece, hızlı ayarlar

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: sol paneli açıp kapatan kısayolun adı.
settings-keybinds-panel-left-label = Sol paneli aç / kapat

# Nerede: Kısayollar sayfası, sol panel kısayolunun arama sözcükleri. Anlam: "çekmece" terimiyle kısayol bulunur.
settings-keybinds-panel-left-keywords = kısayol, tuş, çekmece

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: takvim panelini açıp kapatan kısayolun adı.
settings-keybinds-panel-clock-label = Takvimi aç / kapat

# Nerede: Kısayollar sayfası, takvim kısayolunun arama sözcükleri. Anlam: "saat", "takvim" terimleriyle kısayol bulunur.
settings-keybinds-panel-clock-keywords = kısayol, tuş, saat, takvim

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: barı gösterip gizleyen kısayolun adı.
settings-keybinds-bar-toggle-label = Barı göster / gizle

# Nerede: Kısayollar sayfası, bar kısayolunun arama sözcükleri. Anlam: "bar", "gizle" terimleriyle kısayol bulunur.
settings-keybinds-bar-toggle-keywords = kısayol, tuş, bar, gizle

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü (ayrıca Erişilebilirlik). Anlam: bar'da klavyeyle (ekran okuyucuyla) gezinmeye başlatan kısayolun adı.
settings-keybinds-bar-focus-label = Bar'da klavyeyle gezin

# Nerede: Aynı satırın alt yazısı. Anlam: ekran okuyucu ve yalnız klavye kullananlar için; Escape gezinmeden çıkar.
settings-keybinds-bar-focus-description = Ekran okuyucu ve klavye için; Escape gezinmeyi bitirir

# Nerede: Aynı kısayolun arama sözcükleri. Anlam: odak, ekran okuyucu, Orca terimleriyle bulunur.
settings-keybinds-bar-focus-keywords = kısayol, tuş, bar, odak, ekran okuyucu, orca, klavye

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekran görüntüsü kısayollarını toplayan bölüm.
settings-section-screenshot = Ekran görüntüsü

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: pano (kopyalanan öğeler) kısayolunu toplayan bölüm.
settings-section-clipboard = Pano

# Nerede: Kısayollar sayfası, Ekran görüntüsü bölümü. Anlam: fareyle seçilen bölgenin görüntüsünü alan kısayolun adı.
settings-keybinds-shot-region-label = Bölgenin görüntüsünü al

# Nerede: Kısayollar sayfası, bölge görüntüsü kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-shot-region-keywords = kısayol, tuş, ekran görüntüsü, print, bölge

# Nerede: Kısayollar sayfası, Ekran görüntüsü bölümü. Anlam: imlecin bulunduğu monitörün tamamının görüntüsünü alan kısayolun adı.
settings-keybinds-shot-output-label = Tüm ekranın görüntüsünü al

# Nerede: Kısayollar sayfası, tüm ekran görüntüsü kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-shot-output-keywords = kısayol, tuş, ekran görüntüsü, print, monitör

# Nerede: Kısayollar sayfası, Ekran görüntüsü bölümü. Anlam: etkin pencerenin görüntüsünü alan kısayolun adı.
settings-keybinds-shot-window-label = Pencerenin görüntüsünü al

# Nerede: Kısayollar sayfası, pencere görüntüsü kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-shot-window-keywords = kısayol, tuş, ekran görüntüsü, print, pencere

# Nerede: Kısayollar sayfası, Pano bölümü. Anlam: kopyalanan öğelerin geçmişini (başlatıcının pano kipi) açan kısayolun adı.
settings-keybinds-clipboard-label = Pano geçmişini aç

# Nerede: Kısayollar sayfası, pano kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-clipboard-keywords = kısayol, tuş, pano, kopyala, yapıştır, geçmiş

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekranı imlecin çevresinde büyüten büyüteç kısayollarını toplayan bölüm.
settings-section-magnifier = Büyüteç

# Nerede: Kısayollar sayfası, Büyüteç bölümü. Anlam: büyütmeyi bir adım artıran kısayolun adı (basılı tutunca tekrarlar).
settings-keybinds-magnifier-in-label = Yakınlaştır

# Nerede: Kısayollar sayfası, yakınlaştırma kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-magnifier-in-keywords = kısayol, tuş, büyüteç, yakınlaştır, zoom, erişilebilirlik

# Nerede: Kısayollar sayfası, Büyüteç bölümü. Anlam: büyütmeyi bir adım azaltan kısayolun adı (basılı tutunca tekrarlar).
settings-keybinds-magnifier-out-label = Uzaklaştır

# Nerede: Kısayollar sayfası, uzaklaştırma kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-magnifier-out-keywords = kısayol, tuş, büyüteç, uzaklaştır, zoom, erişilebilirlik

# Nerede: Kısayollar sayfası, Büyüteç bölümü. Anlam: büyütmeyi tamamen kapatıp ekranı normal boyuta döndüren kısayolun adı.
settings-keybinds-magnifier-reset-label = Büyütmeyi kapat

# Nerede: Kısayollar sayfası, büyütmeyi kapatma kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-magnifier-reset-keywords = kısayol, tuş, büyüteç, sıfırla, kapat, zoom, erişilebilirlik

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: başlatıcının emoji seçici kipini açan kısayolu toplayan bölüm.
settings-section-emoji = Emoji

# Nerede: Kısayollar sayfası, Emoji bölümü. Anlam: başlatıcıyı emoji seçici kipinde açan kısayolun adı.
settings-keybinds-emoji-label = Emoji seçici

# Nerede: Kısayollar sayfası, emoji seçici kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-emoji-keywords = kısayol, tuş, emoji, ifade, simge, başlatıcı

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekrana dokunarak yazılan sanal klavyenin kısayolunu toplayan bölüm.
settings-section-osk = Ekran klavyesi

# Nerede: Kısayollar sayfası, Ekran klavyesi bölümü. Anlam: ekran klavyesini gösteren ya da gizleyen kısayolun adı.
settings-keybinds-osk-toggle-label = Ekran klavyesini aç / kapat

# Nerede: Kısayollar sayfası, ekran klavyesi kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-osk-toggle-keywords = kısayol, tuş, ekran klavyesi, sanal klavye, dokunmatik, erişilebilirlik

## Kısayollar: temel katman (shell ADR-0006)

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Uygulama başlatıcısı" kısayolunun adı (shell keybinds.launcher).
settings-keybinds-launcher-label = Uygulama başlatıcısı

# Nerede: Kısayollar sayfası, keybinds.launcher satırının arama sözcükleri (virgüllü liste).
settings-keybinds-launcher-keywords = kısayol, tuş, başlatıcı, super, uygulama, arama, launcher

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Ayarlar" kısayolunun adı (shell keybinds.settings).
settings-keybinds-settings-label = Ayarlar

# Nerede: Kısayollar sayfası, keybinds.settings satırının arama sözcükleri (virgüllü liste).
settings-keybinds-settings-keywords = kısayol, tuş, ayarlar, settings, tercihler

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Terminal" kısayolunun adı (shell keybinds.terminal).
settings-keybinds-terminal-label = Terminal

# Nerede: Kısayollar sayfası, keybinds.terminal satırının arama sözcükleri (virgüllü liste).
settings-keybinds-terminal-keywords = kısayol, tuş, terminal, konsol, komut satırı

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Dosya yöneticisi" kısayolunun adı (shell keybinds.file_manager).
settings-keybinds-file-manager-label = Dosya yöneticisi

# Nerede: Kısayollar sayfası, keybinds.file_manager satırının arama sözcükleri (virgüllü liste).
settings-keybinds-file-manager-keywords = kısayol, tuş, dosya, klasör, dosya yöneticisi, files

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Web tarayıcısı" kısayolunun adı (shell keybinds.browser).
settings-keybinds-browser-label = Web tarayıcısı

# Nerede: Kısayollar sayfası, keybinds.browser satırının arama sözcükleri (virgüllü liste).
settings-keybinds-browser-keywords = kısayol, tuş, tarayıcı, internet, web, browser

# Nerede: Kısayollar sayfası, Oturum bölümü. Anlam: "Ekranı kilitle" kısayolunun adı (shell keybinds.lock).
settings-keybinds-lock-label = Ekranı kilitle

# Nerede: Kısayollar sayfası, keybinds.lock satırının arama sözcükleri (virgüllü liste).
settings-keybinds-lock-keywords = kısayol, tuş, kilit, kilitle, lock, ekran

# Nerede: Kısayollar sayfası, Oturum bölümü. Anlam: "Oturum menüsü" kısayolunun adı (shell keybinds.session_menu).
settings-keybinds-session-menu-label = Oturum menüsü

# Nerede: Kısayollar sayfası, keybinds.session_menu satırının arama sözcükleri (virgüllü liste).
settings-keybinds-session-menu-keywords = kısayol, tuş, oturum, kapat, yeniden başlat, çıkış, ctrl alt delete

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereyi kapat" kısayolunun adı (shell keybinds.close_window).
settings-keybinds-close-window-label = Pencereyi kapat

# Nerede: Kısayollar sayfası, keybinds.close_window satırının arama sözcükleri (virgüllü liste).
settings-keybinds-close-window-keywords = kısayol, tuş, pencere, kapat, close

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereyi kapat (ikinci kısayol)" kısayolunun adı (shell keybinds.close_window_alt).
settings-keybinds-close-window-alt-label = Pencereyi kapat (ikinci kısayol)

# Nerede: Kısayollar sayfası, keybinds.close_window_alt satırının arama sözcükleri (virgüllü liste).
settings-keybinds-close-window-alt-keywords = kısayol, tuş, pencere, kapat, alt f4, close

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: yanıt vermeyen bir pencereyi zorla kapatma kısayolunun adı (shell keybinds.force_kill); kısayoldan sonra kapatılacak pencereye tıklanır.
settings-keybinds-force-kill-label = Yanıt vermeyen pencereyi zorla kapat

# Nerede: Kısayollar sayfası, keybinds.force_kill satırının arama sözcükleri (virgüllü liste).
settings-keybinds-force-kill-keywords = kısayol, tuş, pencere, zorla, kapat, öldür, kill

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Tam ekran" kısayolunun adı (shell keybinds.fullscreen).
settings-keybinds-fullscreen-label = Tam ekran

# Nerede: Kısayollar sayfası, keybinds.fullscreen satırının arama sözcükleri (virgüllü liste).
settings-keybinds-fullscreen-keywords = kısayol, tuş, pencere, tam ekran, fullscreen

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Ekranı kapla" kısayolunun adı (shell keybinds.maximize).
settings-keybinds-maximize-label = Ekranı kapla

# Nerede: Kısayollar sayfası, keybinds.maximize satırının arama sözcükleri (virgüllü liste).
settings-keybinds-maximize-keywords = kısayol, tuş, pencere, büyüt, ekranı kapla, maximize

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Yüzen pencere" kısayolunun adı (shell keybinds.toggle_floating).
settings-keybinds-toggle-floating-label = Yüzen pencere

# Nerede: Kısayollar sayfası, keybinds.toggle_floating satırının arama sözcükleri (virgüllü liste).
settings-keybinds-toggle-floating-keywords = kısayol, tuş, pencere, yüzen, floating, döşeme

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereyi sabitle" kısayolunun adı (shell keybinds.pin).
settings-keybinds-pin-label = Pencereyi sabitle

# Nerede: Kısayollar sayfası, keybinds.pin satırının arama sözcükleri (virgüllü liste).
settings-keybinds-pin-keywords = kısayol, tuş, pencere, sabitle, her zaman üstte, pin

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereler arasında geçiş" kısayolunun adı (shell keybinds.cycle_windows).
settings-keybinds-cycle-windows-label = Pencereler arasında geçiş

# Nerede: Kısayollar sayfası, keybinds.cycle_windows satırının arama sözcükleri (virgüllü liste).
settings-keybinds-cycle-windows-keywords = kısayol, tuş, pencere, geçiş, alt tab, değiştir

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereler arasında geri geçiş" kısayolunun adı (shell keybinds.cycle_windows_back).
settings-keybinds-cycle-windows-back-label = Pencereler arasında geri geçiş

# Nerede: Kısayollar sayfası, keybinds.cycle_windows_back satırının arama sözcükleri (virgüllü liste).
settings-keybinds-cycle-windows-back-keywords = kısayol, tuş, pencere, geçiş, alt shift tab

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Bölme yönünü çevir" kısayolunun adı (shell keybinds.toggle_split).
settings-keybinds-toggle-split-label = Bölme yönünü çevir

# Nerede: Kısayollar sayfası, keybinds.toggle_split satırının arama sözcükleri (virgüllü liste).
settings-keybinds-toggle-split-keywords = kısayol, tuş, pencere, bölme, yan yana, alt alta, split

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: sol" kısayolunun adı (shell keybinds.focus_left).
settings-keybinds-focus-left-label = Odak: sol

# Nerede: Kısayollar sayfası, keybinds.focus_left satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-left-keywords = kısayol, tuş, odak, sol, pencere, focus

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: sağ" kısayolunun adı (shell keybinds.focus_right).
settings-keybinds-focus-right-label = Odak: sağ

# Nerede: Kısayollar sayfası, keybinds.focus_right satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-right-keywords = kısayol, tuş, odak, sağ, pencere, focus

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: yukarı" kısayolunun adı (shell keybinds.focus_up).
settings-keybinds-focus-up-label = Odak: yukarı

# Nerede: Kısayollar sayfası, keybinds.focus_up satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-up-keywords = kısayol, tuş, odak, yukarı, pencere, focus

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: aşağı" kısayolunun adı (shell keybinds.focus_down).
settings-keybinds-focus-down-label = Odak: aşağı

# Nerede: Kısayollar sayfası, keybinds.focus_down satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-down-keywords = kısayol, tuş, odak, aşağı, pencere, focus

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi sola taşı" kısayolunun adı (shell keybinds.move_left).
settings-keybinds-move-left-label = Pencereyi sola taşı

# Nerede: Kısayollar sayfası, keybinds.move_left satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-left-keywords = kısayol, tuş, taşı, sol, pencere, move

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi sağa taşı" kısayolunun adı (shell keybinds.move_right).
settings-keybinds-move-right-label = Pencereyi sağa taşı

# Nerede: Kısayollar sayfası, keybinds.move_right satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-right-keywords = kısayol, tuş, taşı, sağ, pencere, move

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi yukarı taşı" kısayolunun adı (shell keybinds.move_up).
settings-keybinds-move-up-label = Pencereyi yukarı taşı

# Nerede: Kısayollar sayfası, keybinds.move_up satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-up-keywords = kısayol, tuş, taşı, yukarı, pencere, move

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi aşağı taşı" kısayolunun adı (shell keybinds.move_down).
settings-keybinds-move-down-label = Pencereyi aşağı taşı

# Nerede: Kısayollar sayfası, keybinds.move_down satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-down-keywords = kısayol, tuş, taşı, aşağı, pencere, move

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi daralt" kısayolunun adı (shell keybinds.shrink_width).
settings-keybinds-shrink-width-label = Pencereyi daralt

# Nerede: Kısayollar sayfası, keybinds.shrink_width satırının arama sözcükleri (virgüllü liste).
settings-keybinds-shrink-width-keywords = kısayol, tuş, boyut, daralt, genişlik, resize

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi genişlet" kısayolunun adı (shell keybinds.grow_width).
settings-keybinds-grow-width-label = Pencereyi genişlet

# Nerede: Kısayollar sayfası, keybinds.grow_width satırının arama sözcükleri (virgüllü liste).
settings-keybinds-grow-width-keywords = kısayol, tuş, boyut, genişlet, genişlik, resize

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi kısalt" kısayolunun adı (shell keybinds.shrink_height).
settings-keybinds-shrink-height-label = Pencereyi kısalt

# Nerede: Kısayollar sayfası, keybinds.shrink_height satırının arama sözcükleri (virgüllü liste).
settings-keybinds-shrink-height-keywords = kısayol, tuş, boyut, kısalt, yükseklik, resize

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi uzat" kısayolunun adı (shell keybinds.grow_height).
settings-keybinds-grow-height-label = Pencereyi uzat

# Nerede: Kısayollar sayfası, keybinds.grow_height satırının arama sözcükleri (virgüllü liste).
settings-keybinds-grow-height-keywords = kısayol, tuş, boyut, uzat, yükseklik, resize

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Çalışma alanına geç (1…0)" kısayolunun adı (shell keybinds.workspace).
settings-keybinds-workspace-label = Çalışma alanına geç (1…0)

# Nerede: Kısayollar sayfası, keybinds.workspace satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-keywords = kısayol, tuş, çalışma alanı, masaüstü, workspace, rakam

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi çalışma alanına taşı (1…0)" kısayolunun adı (shell keybinds.move_to_workspace).
settings-keybinds-move-to-workspace-label = Pencereyi çalışma alanına taşı (1…0)

# Nerede: Kısayollar sayfası, keybinds.move_to_workspace satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-workspace-keywords = kısayol, tuş, çalışma alanı, taşı, workspace, rakam

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Sonraki çalışma alanı" kısayolunun adı (shell keybinds.workspace_next).
settings-keybinds-workspace-next-label = Sonraki çalışma alanı

# Nerede: Kısayollar sayfası, keybinds.workspace_next satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-next-keywords = kısayol, tuş, çalışma alanı, sonraki, workspace, page down

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Önceki çalışma alanı" kısayolunun adı (shell keybinds.workspace_prev).
settings-keybinds-workspace-prev-label = Önceki çalışma alanı

# Nerede: Kısayollar sayfası, keybinds.workspace_prev satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-prev-keywords = kısayol, tuş, çalışma alanı, önceki, workspace, page up

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi sonraki alana taşı" kısayolunun adı (shell keybinds.move_to_workspace_next).
settings-keybinds-move-to-workspace-next-label = Pencereyi sonraki alana taşı

# Nerede: Kısayollar sayfası, keybinds.move_to_workspace_next satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-workspace-next-keywords = kısayol, tuş, çalışma alanı, taşı, sonraki, workspace

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi önceki alana taşı" kısayolunun adı (shell keybinds.move_to_workspace_prev).
settings-keybinds-move-to-workspace-prev-label = Pencereyi önceki alana taşı

# Nerede: Kısayollar sayfası, keybinds.move_to_workspace_prev satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-workspace-prev-keywords = kısayol, tuş, çalışma alanı, taşı, önceki, workspace

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Son çalışma alanına dön" kısayolunun adı (shell keybinds.workspace_last).
settings-keybinds-workspace-last-label = Son çalışma alanına dön

# Nerede: Kısayollar sayfası, keybinds.workspace_last satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-last-keywords = kısayol, tuş, çalışma alanı, son, önceki, super tab

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Karalama alanını göster/gizle" kısayolunun adı (shell keybinds.scratchpad).
settings-keybinds-scratchpad-label = Karalama alanını göster/gizle

# Nerede: Kısayollar sayfası, keybinds.scratchpad satırının arama sözcükleri (virgüllü liste).
settings-keybinds-scratchpad-keywords = kısayol, tuş, karalama, gizli, scratchpad, özel alan

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi karalama alanına gönder" kısayolunun adı (shell keybinds.move_to_scratchpad).
settings-keybinds-move-to-scratchpad-label = Pencereyi karalama alanına gönder

# Nerede: Kısayollar sayfası, keybinds.move_to_scratchpad satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-scratchpad-keywords = kısayol, tuş, karalama, gizli, scratchpad, taşı

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Fareyle pencere taşı" kısayolunun adı (shell keybinds.window_drag).
settings-keybinds-window-drag-label = Fareyle pencere taşı

# Nerede: Kısayollar sayfası, keybinds.window_drag satırının arama sözcükleri (virgüllü liste).
settings-keybinds-window-drag-keywords = kısayol, tuş, fare, sürükle, taşı, pencere

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Fareyle pencere boyutlandır" kısayolunun adı (shell keybinds.window_resize_drag).
settings-keybinds-window-resize-drag-label = Fareyle pencere boyutlandır

# Nerede: Kısayollar sayfası, keybinds.window_resize_drag satırının arama sözcükleri (virgüllü liste).
settings-keybinds-window-resize-drag-keywords = kısayol, tuş, fare, sürükle, boyut, pencere

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Tekerlekle sonraki çalışma alanı" kısayolunun adı (shell keybinds.workspace_scroll_next).
settings-keybinds-workspace-scroll-next-label = Tekerlekle sonraki çalışma alanı

# Nerede: Kısayollar sayfası, keybinds.workspace_scroll_next satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-scroll-next-keywords = kısayol, tuş, fare, tekerlek, çalışma alanı, kaydır

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Tekerlekle önceki çalışma alanı" kısayolunun adı (shell keybinds.workspace_scroll_prev).
settings-keybinds-workspace-scroll-prev-label = Tekerlekle önceki çalışma alanı

# Nerede: Kısayollar sayfası, keybinds.workspace_scroll_prev satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-scroll-prev-keywords = kısayol, tuş, fare, tekerlek, çalışma alanı, kaydır

# Nerede: Kısayollar sayfası, Genel bölümü. Anlam: Nuxut'un hazır pencere yönetimi kısayollarını, pencere kurallarını ve davranış varsayılanlarını (temel katman) açıp kapatan ayarın başlığı.
settings-compositor-base-layer-label = Hazır pencere yönetimi

# Nerede: Kısayollar sayfası, compositor.base_layer ayarının alt yazısı. Anlam: temel katmanın neleri içerdiği ve kullanıcının kendi Hyprland ayarındaki satırların bunları ezdiği.
settings-compositor-base-layer-description = Pencere kısayolları ve kuralları; Hyprland ayarı üstün gelir

# Nerede: Kısayollar sayfası, compositor.base_layer ayarının arama sözcükleri (virgüllü liste).
settings-compositor-base-layer-keywords = kısayol, pencere, çalışma alanı, temel katman, hyprland, kural

# Nerede: Kısayollar sayfası, Genel bölümü. Anlam: kullanıcının kendi Hyprland ayarında ses, parlaklık, medya, ekran görüntüsü tuşları başka bir şeye bağlıysa Nuxut'un bu tuşları kendi işlevleri için devralmasını açıp kapatan ayarın başlığı.
settings-compositor-take-over-keys-label = Medya ve ses tuşlarını Nuxut kullansın

# Nerede: Kısayollar sayfası, compositor.take_over_keys ayarının alt yazısı. Anlam: tuşların Nuxut'a geçtiği, kullanıcının Hyprland dosyasının değiştirilmediği.
settings-compositor-take-over-keys-description = Hyprland'deki aynı tuşlar Nuxut'a geçer; dosya değişmez

# Nerede: Kısayollar sayfası, compositor.take_over_keys ayarının arama sözcükleri (virgüllü liste).
settings-compositor-take-over-keys-keywords = kısayol, tuş, ses, medya, parlaklık, ekran görüntüsü, devral

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekranı kilitleme ve oturum menüsü kısayollarını toplayan bölüm.
settings-section-session = Oturum

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: pencere kapatma, tam ekran, yüzen pencere gibi pencere kısayollarını toplayan bölüm.
settings-section-windows = Pencereler

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: odağı ve pencereyi ok tuşlarıyla yönlere taşıyan kısayolları toplayan bölüm.
settings-section-focus-move = Odak ve taşıma

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: pencereyi klavyeyle büyütüp küçülten kısayolları toplayan bölüm.
settings-section-resize = Boyutlandırma

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: çalışma alanları (sanal masaüstleri) arasında geçiş ve pencere taşıma kısayollarını toplayan bölüm.
settings-section-workspaces = Çalışma alanları

# Nerede: Kısayollar sayfası, yakalama modunda başlatıcı satırının altındaki ipucu. Anlam: tek bir değiştirici tuşa (ör. Super) basıp bırakılması gerektiği; Esc iptal eder.
settings-keybinds-hint-tap = Tek bir değiştiriciye (ör. Super) basıp bırak · Esc iptal

# Nerede: Kısayollar sayfası, yakalama modunda çalışma alanı satırlarının altındaki ipucu. Anlam: 1…0 tuşlarıyla birlikte kullanılacak değiştiricilere basılıp bırakılması gerektiği.
settings-keybinds-hint-digits = 1…0 ile kullanılacak değiştiricilere basıp bırak · Esc iptal

# Nerede: Kısayollar sayfası, başlatıcı satırında yakalama sırasında uyarı. Anlam: bu kısayolun yalnızca tek bir değiştirici tuşa dokunup bırakarak atanabildiği.
settings-keybinds-tap-single = Yalnızca tek bir değiştiriciye basıp bırakın

# Nerede: Kısayollar sayfası, fare kısayolu satırına tıklanınca altındaki bilgi. Anlam: fare kısayollarının klavyeden kaydedilemediği; yalnızca kaldırılabildiği ya da varsayılana döndürülebildiği.
settings-keybinds-pointer-hint = Fare kısayolu: yalnızca kaldırılabilir ya da varsayılana döndürülebilir

# Nerede: Kısayollar sayfası, fare tekerleği aşağı yönüne atanmış kısayolun tuş başlığı. Anlam: fare tekerleğini aşağı çevirme.
settings-keybind-cap-wheel-down = Tekerlek aşağı

# Nerede: Kısayollar sayfası, fare tekerleği yukarı yönüne atanmış kısayolun tuş başlığı. Anlam: fare tekerleğini yukarı çevirme.
settings-keybind-cap-wheel-up = Tekerlek yukarı

# Nerede: Kısayollar sayfası, farenin sol düğmesine (sürükleyerek) atanmış kısayolun tuş başlığı. Anlam: farenin sol düğmesi.
settings-keybind-cap-mouse-left = Sol tık

# Nerede: Kısayollar sayfası, farenin sağ düğmesine (sürükleyerek) atanmış kısayolun tuş başlığı. Anlam: farenin sağ düğmesi.
settings-keybind-cap-mouse-right = Sağ tık

## Hakkında sayfası

# Nerede: Hakkında sayfasındaki ayar satırının başlığı; satır bilgi penceresini açar. Anlam: sürüm ve yapılandırma bilgisi.
settings-about-label = Hakkında

# Nerede: Hakkında sayfası, Hakkında satırının altındaki açıklama. Anlam: pencerede gösterilen bilgileri özetler.
settings-about-description = Sürüm, config dosyaları

# Nerede: Hakkında sayfası, Hakkında satırının arama sözcükleri. Anlam: "sürüm", "lisans" terimleriyle satır bulunur.
settings-about-keywords = sürüm, versiyon, lisans
## Uygulama penceresi ve arama

# Nerede: Ana pencere, sol üstteki uygulama başlığı (src/app.rs). Anlam: Uygulamanın adı.
settings-app-title = Ayarlar

# Nerede: Kenar çubuğundaki arama alanının yer tutucusu (src/app.rs). Anlam: Aramanın ayarlar içinde yapıldığı.
settings-app-search-placeholder = Ayarlarda ara

# Nerede: Arama sonuç listesi boşken listenin ilk (etkisiz) satırı (src/app.rs). Anlam: Eşleşme bulunamadığı.
settings-app-search-empty = Sonuç yok

# Nerede: "Sonuç yok" satırının alt yazısı (src/app.rs). Anlam: Başka bir kelime denenmesi önerisi.
settings-app-search-empty-hint = Başka bir kelime deneyin

# Nerede: Arama sonuçlarında sayfa eşleşmelerinin alt yazısı (src/app.rs). Anlam: Sonucun bir ayar değil, bir sayfa olduğu.
settings-app-result-page = Sayfa

# Nerede: Ayar satırı başlığının yanındaki küçük hap, widget ekleme menüsündeki düğmenin köşesi ve arama sonucunun sağı (src/new_badge.rs, src/app.rs). Anlam: Ayarın/widget'ın son güncellemeyle geldiği ve kullanıcının henüz görmediği. Çok kısa olmalı (tek sözcük).
settings-badge-new = Yeni

# Nerede: Başlık çubuğundaki "Son değişiklikler" menüsü boşken görünen (etkisiz) öğe (src/app.rs). Anlam: Henüz değişiklik yapılmadığı.
settings-app-no-changes = Henüz değişiklik yok

# Nerede: Portal duvar kağıdı seçicinin pencere başlığı (src/services/mod.rs). Anlam: Dosya seçicide duvar kağıdı seçildiği.
settings-services-wallpaper-dialog-title = Duvar kağıdı seçin

# Nerede: Portal dosya seçicideki tür süzgecinin adı (src/services/portal.rs). Anlam: Seçicide gösterilen dosya türleri.
settings-portal-filter-images = Görseller ve GIF

## Bildirim şeridi ve geçmiş

# Nerede: Panoya kopyalama başarılı olunca çıkan bildirim şeridi (src/app.rs). Anlam: Kopyalamanın yapıldığı.
settings-app-copied = Panoya kopyalandı

# Nerede: Panoya kopyalama başarısız olunca çıkan bildirim şeridi (src/app.rs). Anlam: Kopyalamanın neden başarısız olduğu.
# Değişkenler: $error (hata ayrıntısı, ör. "izin verilmedi").
settings-app-error-copy = Kopyalanamadı: { $error }

# Nerede: Bir ayar değişince çıkan bildirim şeridi ("Geri al" düğmeli) (src/app.rs). Anlam: Hangi ayarın değiştiği.
# Değişkenler: $label (değişen ayarın başlığı, ör. "Panel düzeni").
settings-app-changed = { $label } değişti

# Nerede: Geri yükleme başlatılınca çıkan bildirim şeridi (src/app.rs). Anlam: Ayarın önceki hâline döndürülmekte olduğu.
# Değişkenler: $label (geri yüklenen ayarın başlığı, ör. "Panel düzeni").
settings-app-restoring = { $label } geri yükleniyor…

# Nerede: "Geri al" uygulandıktan sonraki bildirim şeridi (src/app.rs). Anlam: Değişikliğin geri alındığı.
# Değişkenler: $label (değişikliğin etiketi, ör. "Panel düzeni değişti").
settings-app-history-undone = { $label } geri alındı

# Nerede: "Yinele" uygulandıktan sonraki bildirim şeridi (src/app.rs). Anlam: Değişikliğin yeniden uygulandığı.
# Değişkenler: $label (değişikliğin etiketi, ör. "Panel düzeni değişti").
settings-app-history-redone = { $label } yinelendi

# Nerede: "Son değişiklikler" menüsünden tek kayıt geri alındığında çıkan bildirim şeridi (src/history.rs). Anlam: Kaydın geri alındığı; kaydın etiketine eklenir.
# Değişkenler: $label (geri alınan kaydın etiketi, ör. "Panel düzeni değişti").
settings-history-reverted = { $label } (geri alındı)

# Nerede: Bir ayar dosyası okunamadığında pencerenin üstündeki uyarı bandı (src/app.rs). Anlam: Dosya düzeltilene kadar değişikliklerin kaydedilmediği.
# Değişkenler: $file (dosyanın adı, uzantısız, ör. "look"), $error (hata ayrıntısı, ör. "geçersiz TOML").
settings-app-error-broken-file = { $file }.toml okunamıyor ({ $error }) — düzeltilene kadar değişiklikler kaydedilmiyor.

# Nerede: Ayar dosyası kaydedilemediğinde çıkan bildirim şeridi (src/app.rs). Anlam: Kaydetmenin neden başarısız olduğu.
# Değişkenler: $file (uzantısız dosya adı, ör. "shell"), $error (hata ayrıntısı, ör. "disk dolu").
settings-app-error-save = { $file }.toml kaydedilemedi: { $error }

# Nerede: Ayar araması sonuçlarındaki satırın alt yazısı (src/app.rs). Anlam: Sonucun hangi sayfanın hangi bölümünde olduğu.
# Değişkenler: $page (sayfa başlığı, ör. "Görünüm ve hareket"), $section (bölüm adı, ör. "Panel düzeni").
settings-app-result-subtitle = { $page } › { $section }

# Nerede: Pencere başlığı (src/app.rs). Anlam: Pencerede açık olan ayar sayfası.
# Değişkenler: $page (sayfa başlığı, ör. "Görünüm ve hareket").
settings-app-window-title = Ayarlar — { $page }

# Nerede: Bildirim şeridindeki geri alma düğmesi (src/app.rs). Anlam: Düğmenin görevi: son değişikliği geri almak.
settings-app-snackbar-undo = Geri al

## Yönlendirme satırı

# Nerede: Ayar başka bir sayfadayken görünen yönlendirme satırının sağındaki metin düğme (src/pages/link.rs). Anlam: Düğme, hedef ayarı açar.
settings-link-configure = Ayarla

## Dil ve biçim

# Nerede: Dil ve Biçim sayfasında Biçimler satırının alt yazısı (src/pages/locale.rs). Anlam: Seçilen biçimin tanınmadığı, sistem biçiminin kullanıldığı.
# Değişkenler: $sample (biçim önizlemesi, ör. "11.10.2026 · 1.234,56").
settings-locale-subtitle-unrecognized = Tanınmıyor, sistem biçimi: { $sample }

# Nerede: Dil ve Biçimler açılır menülerinin ilk seçeneği (src/pages/locale.rs). Anlam: Sistem ayarının izlendiği.
settings-locale-auto = Otomatik

# Nerede: Dil satırının alt yazısı (src/pages/locale.rs). Anlam: Otomatik seçimde kullanılan sistem dili.
# Değişkenler: $language (sistem dilinin adı, ör. "Türkçe (Türkiye)").
settings-locale-subtitle-system-language = Sistem dili: { $language }

# Nerede: Biçimler satırının alt yazısı (src/pages/locale.rs). Anlam: Otomatik seçimde kullanılan sistem biçimi.
# Değişkenler: $formats (biçim yerelinin adı, ör. "Türkçe (Türkiye)").
settings-locale-subtitle-system-formats = Sistem biçimi: { $formats }

# Nerede: Biçim önizleme kartının başlığı (src/pages/locale.rs). Anlam: Biçim açıkça seçili değilken sistem biçiminin izlendiği.
# Değişkenler: $formats (biçim yerelinin adı, ör. "Türkçe (Türkiye)").
settings-locale-preview-caption-system = { $formats } · sistem

# Nerede: Biçim önizleme kartındaki kısa tarih örneğinin etiketi (src/pages/locale.rs). Anlam: Örneğin kısa tarih olduğu.
settings-locale-preview-short-date = Kısa tarih

# Nerede: Biçim önizleme kartındaki sayı örneğinin etiketi (src/pages/locale.rs). Anlam: Örneğin sayı biçimi olduğu.
settings-locale-preview-number = Sayı

# Nerede: Biçim önizleme kartındaki yüzde örneğinin etiketi (src/pages/locale.rs). Anlam: Örneğin yüzde biçimi olduğu.
settings-locale-preview-percent = Yüzde

## Görünüm sayfası

# Nerede: Görünüm sayfasındaki Renk kipi seçiminin ilk seçeneği; seçilince anahtar dosyadan silinir (src/pages/look.rs). Anlam: Sistem takibi (varsayılan).
settings-look-mode-auto = Otomatik

# Nerede: Görünüm sayfası, Renk kipi seçiminin ikinci seçeneği (src/pages/look.rs). Anlam: Koyu temanın açıkça seçilmesi.
settings-look-mode-dark = Koyu

# Nerede: Görünüm sayfası, Renk kipi seçiminin üçüncü seçeneği (src/pages/look.rs). Anlam: Açık temanın açıkça seçilmesi.
settings-look-mode-light = Açık

# Nerede: Görünüm sayfasında ikon ve imleç teması seçiminin not satırı (src/pages/look.rs). Anlam: Kurulu temaların taranmakta olduğu.
settings-look-themes-loading = Kurulu temalar aranıyor…

# Nerede: Görünüm sayfasında kurulu tema hiç bulunamayınca not satırı (src/pages/look.rs). Anlam: Önerilen tema kurulunca temaların burada görüneceği.
settings-look-themes-empty = Kurulu tema bulunamadı; önerilen tema kurulunca burada görünür.

# Nerede: Görünüm sayfasındaki durum kartında, kurulum komutunun yanındaki metin düğme (src/pages/look.rs). Anlam: Komutu panoya kopyalar.
settings-look-copy-command = Komutu kopyala

# Nerede: Görünüm sayfası, birleşik görünüm kapalıyken anahtarın hemen altındaki açıklama satırı (src/pages/look.rs). Anlam: Kapalıyken uygulamaların kendi temalarını kullanmaya devam edeceği.
settings-look-unified-off-note = Kapalıyken uygulamalar kendi temalarını kullanır.

# Nerede: Görünüm sayfası, aynı açıklama satırının sağındaki tonlu düğme (src/pages/look.rs). Anlam: Birleşik görünümü açar.
settings-look-unified-turn-on = Birleşik görünümü aç

# Nerede: Görünüm sayfasındaki tema kartı ızgarasının ilk kartı; shell'den öneri okunamadığında kartın adı ve önerilen kurulu kartın alt yazısı (src/pages/theme_grid.rs). Anlam: Kartın önerilen temayı temsil ettiği.
settings-theme-grid-recommended = Önerilen

# Nerede: Görünüm sayfasında, seçili önerilen aile kartının alt yazısı; etkin varyant önerinin kendisiyken (src/pages/theme_grid.rs). Anlam: Önerilen varyantın seçili olduğu.
# Değişkenler: $name (varyantın seçicideki kısa adı, ör. "White").
settings-theme-grid-caption-recommended-variant = Önerilen · { $name }

# Nerede: Görünüm sayfasında, seçili önerilen aile kartının alt yazısı; etkin varyant öneri değilken (src/pages/theme_grid.rs). Anlam: Ailenin kaç kurulu varyantı olduğu.
# Değişkenler: $count (ailenin kurulu varyant sayısı, ör. 3).
settings-theme-grid-caption-recommended-variants = Önerilen · { $count } varyant

# Nerede: Görünüm sayfasında, seçili aile kartının alt yazısı; kart önerilen değilken (src/pages/theme_grid.rs). Anlam: Ailenin kaç kurulu varyantı olduğu.
# Değişkenler: $count (ailenin kurulu varyant sayısı, ör. 3).
settings-theme-grid-caption-variants = { $count } varyant

# Nerede: Görünüm sayfasında, önerilen kurulu kartın alt yazısı; önerinin adı okunamadığında (src/pages/theme_grid.rs). Anlam: Hangi temanın seçileceğine kabuğun karar vereceği.
settings-theme-grid-caption-shell-picks = Kabuk seçer

# Nerede: Görünüm sayfasında, önerilen kartın alt yazısı; önerilen aile kurulu değilken (src/pages/theme_grid.rs). Anlam: Önerilen temanın kurulu olmadığı.
settings-theme-grid-caption-recommended-missing = Önerilen · kurulu değil

# Nerede: Aynı alt yazı; önerilen ailenin paketi AUR'dayken (src/pages/theme_grid.rs). Anlam: Önerilen temanın kurulu olmadığı ve paketinin AUR'da olduğu.
settings-theme-grid-caption-recommended-missing-aur = Önerilen · kurulu değil · AUR

# Nerede: Görünüm sayfasında, kurulu olmayan aile kartının alt yazısı; paket AUR'dayken (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı ve paketinin AUR'da olduğu.
settings-theme-grid-caption-missing-aur = Kurulu değil · AUR

# Nerede: Görünüm sayfasında, kurulu olmayan aile kartının alt yazısı (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı.
settings-theme-grid-caption-missing = Kurulu değil

# Nerede: Görünüm sayfasında, ızgaranın altındaki varyant seçicinin etiketi (src/pages/theme_grid.rs). Anlam: Seçicinin kartın ailesinin varyantlarını seçtiği.
# Değişkenler: $name (tema ailesinin adı, ör. "Bibata Modern").
settings-theme-grid-variant-picker-label = { $name } varyantı

# Nerede: Görünüm sayfasında, kurulum komutunun yanındaki metin düğme (src/pages/theme_grid.rs). Anlam: Komutu panoya kopyalar.
settings-theme-grid-button-copy-command = Kurulum komutunu kopyala

# Nerede: Görünüm sayfasında, varyant adlarının ortak kısmı atılınca ortak adı kalmayan varyantın seçicideki etiketi (src/pages/theme_grid.rs). Anlam: Ailenin temel (adsız) varyantı.
settings-theme-grid-variant-base = Standart

# Nerede: Görünüm sayfasında ızgaranın altındaki not; kurulu olmayan aile kartı gösterilirken ve kurulum komutu biliniyorken (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı ve terminalden kurulabileceği.
# Değişkenler: $name (tema ailesinin adı, ör. "Flat-Remix"), $command (AUR kurulum komutu, ör. "paru -S flat-remix-gtk-theme").
settings-theme-grid-footer-missing-command = { $name } kurulu değil. Kurmak için terminalde: { $command }

# Nerede: Aynı not; kurulum komutu bilinmiyorken (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı.
# Değişkenler: $name (tema ailesinin adı, ör. "Flat-Remix").
settings-theme-grid-footer-missing = { $name } kurulu değil.

# Nerede: Görünüm sayfasında ızgaranın altındaki not; seçili tema kurulu değilken ve kurulum komutu biliniyorken (src/pages/theme_grid.rs). Anlam: Tema kurulana dek yedek temanın kullanılacağı ve terminalden kurulabileceği.
# Değişkenler: $name (temanın adı, ör. "Flat-Remix"), $command (AUR kurulum komutu, ör. "paru -S flat-remix-gtk-theme").
settings-theme-grid-footer-fallback-command = { $name } kurulu değil; kurulana dek yedek tema kullanılır. Kurmak için terminalde: { $command }

# Nerede: Görünüm sayfasında ızgaranın altındaki not; kurulu olmayan aile kartı gösterilirken ve paket resmî depodayken; yanında "Kur" düğmesi (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı ve düğmeyle, yönetici onayıyla kurulabileceği.
# Değişkenler: $name (tema ailesinin adı, ör. "Papirus").
settings-theme-grid-footer-missing-install = { $name } kurulu değil. Kur düğmesi yönetici onayıyla kurar.

# Nerede: Görünüm sayfasında ızgaranın altındaki not; seçili tema kurulu değilken ve paket resmî depodayken; yanında "Kur" düğmesi (src/pages/theme_grid.rs). Anlam: Tema kurulana dek yedek temanın kullanılacağı ve düğmeyle kurulabileceği.
# Değişkenler: $name (temanın adı, ör. "Papirus").
settings-theme-grid-footer-fallback-install = { $name } kurulu değil; kurulana dek yedek tema kullanılır. Kur düğmesi yönetici onayıyla kurar.

# Nerede: Görünüm sayfasında eksik paket notunun ve tema ızgarasının yanındaki metin düğme (src/pages/fix_button.rs). Anlam: Önerilen resmî depo paketlerini kurar; yönetici onayı istenir.
settings-packages-install = Kur

# Nerede: Aynı düğme, kurulum sürerken (basılamaz) (src/pages/fix_button.rs). Anlam: Paketlerin kurulmakta olduğu.
settings-packages-installing = Kuruluyor…

# Nerede: Eksik paket notunun sonuna eklenen cümle (src/pages/look.rs, src/pages/theme_grid.rs). Anlam: Son kurulum denemesinin başarısız olduğu.
settings-packages-failed = Son kurulum tamamlanamadı.

# Nerede: Aynı not; kurulum komutu bilinmiyorken (src/pages/theme_grid.rs). Anlam: Tema kurulunca uygulanacağı, o zamana dek yedek temanın kullanılacağı.
# Değişkenler: $name (temanın adı, ör. "Flat-Remix").
settings-theme-grid-footer-fallback = { $name } kurulu değil; kurulunca uygulanır, o zamana dek yedek tema kullanılır.

# Nerede: Ortam tuşları satırındaki örnek bind bloğunun ilk satırındaki yorum (src/pages/media_keys.rs). Anlam: Bloğun ne olduğu; blok kopyalanıp hyprland.conf'a yapıştırılır. Bloğun kalanı config sözdizimi olduğundan çevrilmez.
settings-media-keys-example-comment = Nuxut — medya tuşları (e: basılı tutunca tekrar, l: kilit ekranında da)

# Nerede: Ortam tuşları satırındaki tonlu düğme (src/pages/media_keys.rs). Anlam: Örnek bind bloğunu panoya kopyalar.
settings-media-keys-copy = Kopyala

## Görünüm durumu

# Nerede: Görünüm sayfasındaki durum kartı; shell'e soru sorulurken (src/services/look.rs). Anlam: Uygulamaların durumunun sorulmakta olduğu.
settings-look-status-pending = Uygulamaların durumu soruluyor…

# Nerede: Görünüm sayfasındaki durum kartı; shell çalışmıyorken (src/services/look.rs). Anlam: Seçimlerin kabuk açılınca uygulanacağı.
settings-look-status-not-running = Masaüstü kabuğu çalışmıyor; seçimler kabuk açılınca uygulanır.

# Nerede: Görünüm sayfasındaki durum kartı; shell sürümü görünüm aktarımını desteklemiyorken (src/services/look.rs). Anlam: Kabuğun bu sürümünün görünüm aktarımını henüz desteklemediği.
settings-look-status-unsupported = Kabuğun bu sürümü uygulamalara görünüm aktarmayı henüz desteklemiyor.

# Nerede: Görünüm sayfasındaki durum kartı; Ayarlar test örneğindeyken (src/services/look.rs). Anlam: Test örneğinde kabuğa sorulmadığı.
settings-look-status-test-instance = Test örneği: kabuğa sorulmadı.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); durum sorgusu başarısızken (src/services/look.rs). Anlam: Uygulamaların durumunun alınamadığı.
settings-look-status-failed = Uygulamaların durumu alınamadı.

# Nerede: Görünüm sayfasındaki durum kartı; shell'in dış yazımı bu sürümde kapalıyken (src/services/look.rs). Anlam: Seçimlerin kaydedildiği ama uygulamalara aktarımın sonraki sürümde geleceği.
settings-look-status-writer-disabled = Seçimlerin kaydedildi; diğer uygulamalara aktarım kabuğun sonraki bir sürümüyle gelecek.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); son uygulama hatalıyken (src/services/look.rs). Anlam: Görünümün bazı uygulamalara aktarılamadığı.
settings-look-status-apply-failed = Görünüm bazı uygulamalara aktarılamadı; ayrıntı kabuk günlüğünde.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); kabuk aynı ayarı kısa sürede çok kez yeniden yazmak zorunda kalıp geçici olarak durdurduğunda (src/services/look.rs). Anlam: Başka bir araçla çekişme yüzünden bazı ayarların birkaç dakika bekletildiği.
settings-look-status-paused = Başka bir araç bazı görünüm ayarlarını sürekli geri aldığı için bunlar birkaç dakikalığına durduruldu.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); bir dosyayı başka bir araç yönettiği için Nuxut ona yazmadığında, dosya başına bir cümle; ardından settings-look-external-effect gelir (src/services/look.rs). Anlam: Dosyanın salt okunur, bağlantı ya da başka bir aracın denetiminde olduğu.
# Değişkenler: $path (dosya yolu, ör. "~/.config/gtk-3.0/gtk.css").
settings-look-external-managed = { $path } başka bir araç tarafından yönetiliyor.

# Nerede: settings-look-external-managed ile aynı yer; dosya başka bir kullanıcıya (çoğunlukla root) aitken (src/services/look.rs). Anlam: Nuxut'un dosyayı şimdilik değiştiremediği; sahiplik komutu önerilmez.
# Değişkenler: $path (dosya yolu, ör. "~/.config/kdeglobals").
settings-look-external-not-owned = { $path } başka bir kullanıcıya ait; Nuxut şimdilik değiştiremiyor.

# Nerede: settings-look-external-managed ile aynı yer; dosya Nix ya da Guix deposundayken (src/services/look.rs). Anlam: Değişikliğin o paket yöneticisinin yapılandırmasından yapılması gerektiği.
# Değişkenler: $path (dosya yolu).
settings-look-external-store = { $path } Nix ya da Guix ile yönetiliyor; değişiklik oradan yapılır.

# Nerede: settings-look-external-managed ile aynı yer; dosyanın bulunduğu klasöre yazılamıyorken (src/services/look.rs). Anlam: Dosyanın konumuna yazma izni olmadığı.
# Değişkenler: $path (dosya yolu).
settings-look-external-dir = { $path } konumuna yazılamıyor.

# Nerede: Görünüm durum kartında her dış dosya cümlesinin ardından (src/services/look.rs). Anlam: O dosyayı okuyan uygulamaların Nuxut'un ilgili ayarını almadığı.
# Değişkenler: $what (settings-look-external-what-* metinlerinden biri, ör. "Nuxut renklerini").
settings-look-external-effect = Bu dosyayı okuyan uygulamalar { $what } almıyor.

# Nerede: settings-look-external-effect içindeki $what; dosya renk kipi ayarına aitken. Anlam: Nuxut'un renkleri (cümlede nesne, belirtme durumu).
settings-look-external-what-colors = Nuxut renklerini

# Nerede: settings-look-external-effect içindeki $what; ikon teması ayarı. Anlam: Nuxut'un ikonları (nesne).
settings-look-external-what-icons = Nuxut ikonlarını

# Nerede: settings-look-external-effect içindeki $what; imleç ayarı. Anlam: Nuxut'un imleci (nesne).
settings-look-external-what-cursor = Nuxut imlecini

# Nerede: settings-look-external-effect içindeki $what; yazı tipi ayarı. Anlam: Nuxut'un yazı tipleri (nesne).
settings-look-external-what-fonts = Nuxut yazı tiplerini

# Nerede: settings-look-external-effect içindeki $what; diğer görünüm ayarları. Anlam: Nuxut'un görünümü (nesne).
settings-look-external-what-look = Nuxut görünümünü

# Nerede: Görünüm durum kartında, ilk iki dosyadan sonra kalan dış dosyaların sayısı (src/services/look.rs). Anlam: Aynı durumda başka dosyalar da olduğu.
# Değişkenler: $count (kalan dosya sayısı, ör. 2).
settings-look-external-more = { $count } dosya daha aynı durumda.

# Nerede: Görünüm sayfasındaki durum kartı (olumlu ton), "uygulandı" metninin sonuna eklenir; Nuxut başka bir aracın yönettiği bir dosyayı (ör. bir temaya bağlantı) kendi dosyasıyla değiştirdiğinde (src/services/look.rs). Anlam: Dosyanın artık Nuxut'ta olduğu ve birleşik görünümü kapatmanın eskisini geri getirdiği.
# Değişkenler: $path (dosya yolu, ör. "~/.config/gtk-4.0/gtk.css").
settings-look-taken-over-one = { $path } önceden başka bir araçtaydı, Nuxut devraldı; birleşik görünümü kapatmak eskisini geri koyar.

# Nerede: settings-look-taken-over-one ile aynı yer; birden çok dosya devralındığında (src/services/look.rs). Anlam: İlk dosya ve kalan devralınan dosya sayısı.
# Değişkenler: $path (ilk dosyanın yolu), $count (kalan dosya sayısı, ör. 2).
settings-look-taken-over-many = { $path } ve { $count } dosya daha önceden başka araçlardaydı, Nuxut devraldı; birleşik görünümü kapatmak eskilerini geri koyar.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); paket eksikken; AUR paketi varsa ve yardımcı yoksa sonuna AUR yardımcısı notu eklenir (src/services/look.rs). Anlam: Bazı uygulamalar için gereken ek bileşen sayısı ve adları.
# Değişkenler: $count (eksik bileşen sayısı, ör. 2), $names (paket adlarının virgüllü listesi, ör. "xdg-desktop-portal-gtk, xdg-desktop-portal-hyprland").
settings-look-status-missing-packages = Bazı uygulamalar için { $count } ek bileşen gerekli: { $names }.

# Nerede: Görünüm sayfasındaki durum kartı; başka bir araç görünüm ayarlarken (src/services/look.rs). Anlam: Çakışma var; Nuxut'un üstüne yazdığı.
settings-look-status-conflicts = Başka bir araç da görünüm ayarlıyor; Nuxut onları geçersiz kılar.

## Takvim ve bağlantı

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata) (src/services/calendar.rs). Anlam: Takvim servisinin çalışmadığı.
settings-calendar-error-not-running = Takvim servisi çalışmıyor. nuxut-calendard kurulu mu?

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata) (src/services/calendar.rs). Anlam: Servisin yanıt vermediği.
settings-calendar-error-timeout = Takvim servisi yanıt vermedi.

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata) (src/services/calendar.rs). Anlam: Servise ulaşılamadığı.
settings-calendar-error-unreachable = Takvim servisine ulaşılamadı.

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata); daemon iletisi yokken (src/services/calendar.rs). Anlam: Takvim işleminin başarısız olduğu.
settings-calendar-error-write = Takvim işlemi başarısız oldu.

# Nerede: Görünüm sayfası, kabuk bağlantısı ayar dosyasından kaldırılmışsa çıkan not (src/services/hypr_status.rs). Anlam: Bağlantının nasıl geri ekleneceği.
settings-hypr-status-source-removed = Bağlantı satırı ayar dosyasından kaldırıldı. Yeniden eklemek için bu ayarı kapatıp açın.

## Anlık görüntü: zaman etiketi

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın az önce alındığı.
settings-snapshots-ago-now = az önce

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç dakika önce alındığı.
# Değişkenler: $count (dakika sayısı, ör. 5).
settings-snapshots-ago-minutes = { $count } dk önce

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç saat önce alındığı.
# Değişkenler: $count (saat sayısı, ör. 3).
settings-snapshots-ago-hours = { $count } sa önce

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın dün alındığı.
settings-snapshots-ago-yesterday = dün

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç gün önce alındığı.
# Değişkenler: $count (gün sayısı, ör. 4).
settings-snapshots-ago-days = { $count } gün önce

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç hafta önce alındığı.
# Değişkenler: $count (hafta sayısı, ör. 2).
settings-snapshots-ago-weeks = { $count } hafta önce

## Anlık görüntü: geri yükleme

# Nerede: Ayar kartındaki geri yükleme sonucu (src/services/snapshots.rs). Anlam: Geri yüklemenin başarılı olduğu.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-restored = { $label } önceki hâline döndü

# Nerede: Ayar kartındaki geri yükleme sonucu (src/services/snapshots.rs). Anlam: Ayarın zaten önceki hâlinde olduğu.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-already = { $label } zaten önceki hâlinde

# Nerede: Ayar kartındaki geri yükleme sonucu (kısmi) (src/services/snapshots.rs). Anlam: Ayarın kısmen döndüğü, sonradan başka yerden değiştirilen ayarlara dokunulmadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni"), $count (dokunulmayan ayar sayısı, ör. 2).
settings-snapshots-outcome-partial = { $label } kısmen döndü: sonradan başka yerden değiştirilen { $count } ayara dokunulmadı

# Nerede: Ayar kartındaki geri yükleme sonucu (çakışma) (src/services/snapshots.rs). Anlam: Hiçbir şeye dokunulmadığı, çünkü ayarlar sonradan başka bir araçla değişmiş.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-conflict = { $label } döndürülmedi: ayarlar sonradan başka bir araçla değiştirilmiş, hiçbirine dokunulmadı

# Nerede: Ayar kartındaki geri yükleme sonucu (kayıt yok) (src/services/snapshots.rs). Anlam: Önceki hâlin artık kayıtlı olmadığı.
settings-snapshots-outcome-not-found = Önceki hâl artık kayıtlı değil

# Nerede: Ayar kartındaki geri yükleme sonucu (meşgul) (src/services/snapshots.rs). Anlam: Başka bir değişiklik sürdüğü için beklenmesi gerektiği.
settings-snapshots-outcome-busy = Başka bir değişiklik uygulanıyor; birazdan tekrar dene

# Nerede: Ayar kartındaki geri yükleme sonucu (test örneği) (src/services/snapshots.rs). Anlam: Kabuğun test örneğinin dış ayarlara dokunmadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-skipped-test = { $label } geri alınmadı: kabuğun test örneği dış ayarlara dokunmuyor

# Nerede: Ayar kartındaki geri yükleme sonucu (oturum yok) (src/services/snapshots.rs). Anlam: Masaüstü oturumu bulunamadığı için geri alınamadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-skipped-no-session = { $label } şu an geri alınamıyor: masaüstü oturumu bulunamadı

# Nerede: Ayar kartındaki geri yükleme sonucu (bu oturumda kapalı) (src/services/snapshots.rs). Anlam: Kabuğun bu oturumda dış ayarlara dokunmadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-skipped-disabled = { $label } şu an geri alınamıyor; kabuk bu oturumda dış ayarlara dokunmuyor

# Nerede: Ayar kartındaki geri yükleme sonucu (hata) (src/services/snapshots.rs). Anlam: Geri yüklemenin başarısız olduğu; kabuğun hata türü parantez içinde.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni"), $kind (kabuğun hata türü kısa adı, ör. "io"; bilinmiyorsa "bilinmeyen").
settings-snapshots-outcome-error = { $label } döndürülemedi ({ $kind })

# Nerede: Ayar kartındaki geri yükleme sonucu (doğrulanamadı) (src/services/snapshots.rs). Anlam: Geri yüklemenin yapıldığı ama ayrıntının doğrulanamadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-restored-unverified = { $label } geri yüklendi (ayrıntı doğrulanamadı)

# Nerede: Ayar kartındaki geri yükleme sonucu (onaysız) (src/services/snapshots.rs). Anlam: Geri yüklemenin doğrulanamadığı; durumun yenilenmesi gerektiği.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-unconfirmed = { $label } geri yüklemesi doğrulanamadı — durumu yenile

# Nerede: Ayar kartındaki geri yükleme sonucu (başarısız) (src/services/snapshots.rs). Anlam: Geri yüklemenin başarısız olduğu; hata ayrıntısı cümlenin sonuna eklenir.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni"), $error (hata ayrıntısı, ör. "masaüstü kabuğu çalışmıyor").
settings-snapshots-outcome-failed = { $label } döndürülemedi: { $error }

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; shell çalışmıyorken (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-not-running = masaüstü kabuğu çalışmıyor

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; Ayarlar test örneğindeyken (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-test-instance = test örneği

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; shell isteği reddedince (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-rejected = kabuk isteği kabul etmedi

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; shell'in yanıtı okunamadığında (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-unparsable = kabuğun yanıtı anlaşılamadı

## Widget listesi hataları

# Nerede: Paneller sayfası, widget listesi yüklenemediğinde hata satırı; shell şema komutu hatalı çıkış koduyla bittiğinde (src/services/widgets.rs). Anlam: Hatalı çıkış kodu.
# Değişkenler: $code (çıkış kodu, ör. 127).
settings-widgets-error-exit-code = Widget listesi alınamadı: nuxut-shell schema { $code } koduyla çıktı.

# Nerede: Paneller sayfası, widget listesi yüklenemediğinde hata satırı; shell sinyalle sonlandığında (src/services/widgets.rs). Anlam: Sürecin sinyalle sonlandığı.
settings-widgets-error-signal = Widget listesi alınamadı: nuxut-shell schema sinyalle sonlandı.

## Hoş geldin

# Nerede: Hoş geldin penceresinin giriş ekranı başlığı; aynı metin pencere başlığı olarak da kullanılır (src/welcome.rs, src/app.rs). Anlam: Karşılama.
settings-welcome-intro-title = Nuxut'a hoş geldin

# Nerede: Hoş geldin penceresinin giriş ekranı açıklaması (src/welcome.rs). Anlam: Birkaç tercih yapılacağı, sonra Ayarlar'dan değiştirilebileceği.
settings-welcome-intro-body = Birkaç tercih yapalım; hepsini sonra Ayarlar'dan değiştirebilirsin.

# Nerede: Hoş geldin 1. adım kartının başlığı (src/welcome.rs). Anlam: Renkler ve duvar kağıdı adımı.
settings-welcome-step1-title = Renkler ve duvar kağıdı

# Nerede: Hoş geldin 1. adım kartının açıklaması (src/welcome.rs). Anlam: Koyu/açık seçimi ve renklerin duvar kağıdından gelme seçeneği.
settings-welcome-step1-body = Koyu ya da açık; istersen renkler duvar kağıdından gelsin.

# Nerede: Hoş geldin 2. adım kartının başlığı (src/welcome.rs). Anlam: Uygulamaların Nuxut görünümünü alıp almayacağı sorusu.
settings-welcome-step2-title = Uygulamalar Nuxut gibi görünsün mü?

# Nerede: Hoş geldin 2. adım kartının açıklaması (src/welcome.rs). Anlam: Önerilenin ne olduğu ve geri alınabildiği.
settings-welcome-step2-body = Önerilen: uygulamalar Nuxut'un renklerini, ikonlarını ve imlecini kullanır. Mevcut görünümün istediğin an geri alınabilir.

# Nerede: Hoş geldin 3. adım kartının başlığı (src/welcome.rs). Anlam: Masaüstüyle ortak ayarlar adımı.
settings-welcome-step3-title = Masaüstüyle ortak

# Nerede: Hoş geldin 3. adım kartının açıklaması (src/welcome.rs). Anlam: Köşe, gölge, hareket ve kısayol birliğinin önerildiği.
settings-welcome-step3-body = Köşeler, gölgeler, hareketler ve kısayollar pencerelerle ortak olsun (önerilen).

# Nerede: Hoş geldin penceresi, "Masaüstüyle ortak" adımında en üstteki anahtarın başlığı (yalnızca ilk kurulumda, kullanıcının kendi Hyprland ayarı varken). Anlam: kullanıcının Hyprland ana ayar dosyasını Nuxut'un yönetmesi.
settings-welcome-hyprland-config-title = Hyprland ayarımı Nuxut yönetsin

# Nerede: Aynı anahtarın alt yazısı. Anlam: eski ayar dosyasının yedekleneceği; klavye, fare, ekran kartı ve ortam ayarlarının yeni ayara taşınacağı; kısayolların ve başlangıç komutlarının taşınmayacağı; çıkan bildirimdeki "Geri al" ile tek tıkla eski hâline dönüleceği.
settings-welcome-hyprland-config-body = Eski ayarın yedeklenir; klavye, fare, ekran kartı ve ortam ayarları taşınır. Kısayollar ve başlangıç komutları taşınmaz. Bildirimdeki "Geri al" ile dönebilirsin.
# Nerede: Hoş geldin penceresi, 4. adımın başlığı (src/welcome.rs). Anlam: Bilgisayar açılırken ve kapanırken görünen ekranların görünümü.
settings-welcome-step4-title = Açılış görünümü

# Nerede: Hoş geldin, 4. adımın açıklaması (src/welcome.rs). Anlam: Açılış ekranının ve önyükleme menüsünün Nuxut renklerini alması; yönetici parolası, yedek ve geri alma.
settings-welcome-step4-body = Bilgisayar açılırken ve kapanırken görünen ekranlar da Nuxut'un renklerini alsın. Yönetici parolası sorulur; her dosya önce yedeklenir, tek tıkla geri alınır.

# Nerede: Hoş geldin 4. adım, anahtar satırının başlığı (src/pages/system_theme.rs). Anlam: Açılış ekranını ve önyükleme menüsünü Nuxut temasına geçirme seçeneği.
settings-system-theme-switch = Önyükleme ve açılış ekranını Nuxut'a geçir

# Nerede: Anahtar satırının alt metninde hedef adı (src/pages/system_theme.rs). Anlam: Açılışta görünen, disk parolasını da soran ekran.
settings-system-theme-target-plymouth = Açılış ekranı

# Nerede: Anahtar satırının alt metninde hedef adı (src/pages/system_theme.rs). Anlam: GRUB önyükleme menüsü.
settings-system-theme-target-grub = Önyükleme menüsü (GRUB)

# Nerede: Anahtar satırının alt metninde hedef adı (src/pages/system_theme.rs). Anlam: Limine önyükleme menüsü.
settings-system-theme-target-limine = Önyükleme menüsü (Limine)

# Nerede: Anahtar satırının alt metni, durum okunurken (src/pages/system_theme.rs). Anlam: Sistemin incelendiği.
settings-system-theme-reading = Sistem inceleniyor…

# Nerede: Anahtar satırının alt metni (src/pages/system_theme.rs). Anlam: Bu kurulumda özelliğin çalışmadığı.
settings-system-theme-unavailable = Bu sürümde kullanılamıyor.

# Nerede: Anahtar satırının alt metni (src/pages/system_theme.rs). Anlam: Değiştirilecek bir şey olmadığı.
settings-system-theme-nothing = Bu sistemde değiştirilecek bir şey yok.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Gereken sistem yardımcısının bu kurulumda bulunmadığı (komut önerilmez).
settings-system-theme-helper-missing = Sistem yardımcısı bu kurulumda yok; açılış görünümü değiştirilemiyor.

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: Sistem yardımcısı dosyasının güvenli olmadığı için kullanılmadığı.
settings-system-theme-helper-unsafe = Sistem yardımcısı güvenli bir konumda değil; kullanılmıyor.

# Nerede: Kartın notu, bir hedef engelliyken (src/pages/system_theme.rs). Anlam: Bu hedefe dokunulmayacağı.
settings-system-theme-blocked = dokunulmayacak.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: systemd-boot menüsünün temalanamadığı.
settings-system-theme-systemd-boot = systemd-boot menüsü temalanamaz; açılış ekranı yine Nuxut olur.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Tema zaten uygulanmış.
settings-system-theme-applied = Açılış görünümü Nuxut. Geri almak için "Geri al".

# Nerede: "Neler değişecek" satırı (src/pages/system_theme.rs). Anlam: Bir dosyanın ya da klasörün ekleneceği.
# Değişkenler: $what (yol).
settings-system-theme-change-create = { $what } eklenir

# Nerede: "Neler değişecek" satırı (src/pages/system_theme.rs). Anlam: Bir dosyanın değişeceği ve önce yedekleneceği.
# Değişkenler: $what (yol).
settings-system-theme-change-modify = { $what } değişir (önce yedeklenir)

# Nerede: "Neler değişecek" satırı (src/pages/system_theme.rs). Anlam: Bir komutun çalıştırılacağı.
# Değişkenler: $what (komut).
settings-system-theme-change-run = { $what } çalıştırılır

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Açılış ekranı yazılımının kurulu ama etkin olmadığı.
settings-system-theme-note-plymouth-not-in-initramfs = Açılış ekranı etkin değil; tema kurulur, etkinleştirmek ayrı bir adım.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Çekirdek satırında açılış ekranı seçeneğinin olmadığı.
settings-system-theme-note-plymouth-no-splash = Çekirdek satırında "splash" yok; açılış ekranı görünmeyebilir.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Açılış imgesi yeniden kurulmadıkça açılış ekranının görünmeyeceği; Nuxut bu sistemde bunu yapamıyor.
# Değişkenler: $command (açılış imgesi üreticisinin adı: mkinitcpio ya da dracut; komut değil).
settings-system-theme-note-plymouth-manual-rebuild = Açılış ekranı, açılış imgesi ({ $command }) yeniden kurulunca görünür; Nuxut bunu bu sistemde kendisi yapamıyor.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: GRUB'un metin kipinde çalıştığı.
settings-system-theme-note-grub-text-terminal = GRUB metin kipinde; tema görünmez.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: GRUB ayar dosyasının elle yazılmış olduğu.
settings-system-theme-note-grub-hand-written = GRUB ayarı elle yazılmış; dokunulmaz.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: GRUB ayar dosyasının okunamadığı.
settings-system-theme-note-grub-cfg-unreadable = GRUB ayarı okunamıyor.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Eski GRUB sürümünün ek ayar dosyalarını desteklemediği.
settings-system-theme-note-grub-no-dropins = Bu GRUB sürümü ek ayar dosyalarını okumuyor; dokunulmaz.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Başka bir ayar dosyasının temayı ezdiği.
# Değişkenler: $path (o dosyanın yolu).
settings-system-theme-note-grub-overridden = { $path } temayı ezdiği için dokunulmaz.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Birden çok Limine ayar dosyası bulunduğu.
# Değişkenler: $paths (yollar).
settings-system-theme-note-limine-ambiguous = Birden çok Limine ayarı var ({ $paths }); dokunulmaz.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Limine ayarının okunamadığı.
settings-system-theme-note-limine-unreadable = Limine ayarı okunamıyor.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Limine ayarının tanınmayan biçimde olduğu.
settings-system-theme-note-limine-unsupported = Limine ayarı tanınmayan biçimde; dokunulmaz.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Limine'nin ayar dosyasının imzalı olduğu, değişirse açılmayacağı.
settings-system-theme-note-limine-enrolled = Limine ayarı imzalı (sağlama toplamı kayıtlı); dokunulmaz.

# Nerede: Kartın notu, uygulama başlarken (src/pages/system_theme.rs). Anlam: Yönetici parolasının sorulduğu ya da işin başladığı.
settings-system-theme-waiting = Yönetici onayı bekleniyor…

# Nerede: Kartın notu, uygulama sürerken (src/pages/system_theme.rs). Anlam: Hangi hedefte hangi adımın yapıldığı.
# Değişkenler: $target (hedef adı), $step (teknik adım adı).
settings-system-theme-running-step = { $target }: { $step }…

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Yönetici parolasının verilmediği, hiçbir şey değişmediği.
settings-system-theme-cancelled = Yönetici onayı verilmedi; hiçbir şey değişmedi.

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: Bir hedefin uygulanamadığı, değişikliklerinin geri alındığı.
# Değişkenler: $target (hedef adı), $message (ayrıntı).
settings-system-theme-rolled-back = { $target } uygulanamadı, değişiklikler geri alındı. { $message }

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: Geri almanın da başarısız olduğu; yedeklerin yeri mesajda.
# Değişkenler: $target (hedef adı), $message (yedek yolları).
settings-system-theme-inconsistent = { $target } yarım kaldı; yedekler: { $message }

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: İşin tamamlanamadığı.
settings-system-theme-failed = Tamamlanamadı; hiçbir hedef yarım bırakılmadı.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Temanın uygulandığı, yeniden başlatınca görüneceği.
settings-system-theme-done = Uygulandı. Yeniden başlattığınızda görünecek.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Önceki görünümün geri geldiği.
settings-system-theme-reverted = Önceki açılış görünümü geri geldi.

# Nerede: Kartın düğmesi (src/pages/system_theme.rs). Anlam: Nuxut açılış temasını geri almak.
settings-system-theme-revert = Geri al

# Nerede: Kartın düğmesi (src/pages/system_theme.rs). Anlam: Kurulu sistem dosyalarını (açılış görünümü yardımcısı, giriş ekranı) yeni sürüme güncellemek; yönetici onayı istenir.
settings-system-theme-update = Güncelle

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Sistem dosyalarının yeni sürümünün hazır olduğu; düğmeyle güncellenir.
settings-system-theme-helper-update = Sistem dosyalarının yeni sürümü hazır; Güncelle yönetici onayıyla kurar.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Sistem yardımcısı henüz kurulu değil; uygularken yönetici onayıyla kendiliğinden kurulacak.
settings-system-theme-helper-install = Uygularken sistem dosyaları da yönetici onayıyla kurulur.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Sistem dosyalarının güncellendiği.
settings-system-theme-updated = Sistem dosyaları güncellendi.

# Nerede: Hoş geldin birincil düğmesi, açılış görünümü adımında (src/welcome_ui.rs). Anlam: Seçimi uygulamak.
settings-system-theme-button-apply = Uygula

# Nerede: Hoş geldin birincil düğmesi, uygulama sürerken (src/welcome_ui.rs). Anlam: İşin sürdüğü.
settings-system-theme-button-running = Uygulanıyor…

# Nerede: Hoş geldin bitiş ekranının başlığı (src/welcome.rs). Anlam: Akışın tamamlandığı.
settings-welcome-done-title = Hazır

# Nerede: Hoş geldin bitiş ekranının açıklaması; aynı metin Görünüm durum kartında "Uygulandı." sonuna da eklenir (src/welcome.rs, src/services/look.rs). Anlam: Bazı uygulamaların yeniden açılınca yeni görünüme geçeceği.
settings-welcome-done-body = Bazı açık uygulamalar yeni görünümü yeniden açılınca alır.

# Nerede: Hoş geldin giriş ekranındaki birincil düğme; sonraki ekranlarda ilk değer olarak da kullanılır (src/welcome.rs, src/welcome_ui.rs). Anlam: Akışı başlatır.
settings-welcome-button-start = Başla

# Nerede: Hoş geldin adım ekranlarındaki birincil düğme (src/welcome.rs). Anlam: Sonraki adıma geçer.
settings-welcome-button-next = İleri

# Nerede: Hoş geldin adım ekranlarındaki metin düğme (src/welcome_ui.rs). Anlam: Adımı atlayıp ilerler.
settings-welcome-button-skip = Atla

# Nerede: Hoş geldin son adımındaki birincil düğme (src/welcome.rs). Anlam: Akışı bitirip bitiş ekranına geçer.
settings-welcome-button-finish = Bitir

# Nerede: Hoş geldin bitiş ekranındaki birincil düğme (src/welcome.rs). Anlam: Pencereyi kapatır.
settings-welcome-button-close = Kapat

# Nerede: Hoş geldin adım ekranlarındaki geri düğmesi (src/welcome_ui.rs). Anlam: Önceki adıma döner.
settings-welcome-button-back = Geri

# Nerede: Hoş geldin adım ekranlarının üstündeki ilerleme satırı (src/welcome.rs). Anlam: Bulunulan adım ve toplam adım sayısı.
# Değişkenler: $current (bulunulan adım, ör. 2), $total (toplam adım sayısı, ör. 3).
settings-welcome-progress = Adım { $current } / { $total }

## Tarih ve saat sayfası

# Nerede: Saat dilimi satırının alt metni (src/pages/datetime.rs). Anlam: UTC ofseti.
# Değişkenler: $offset (ör. +03:00).
settings-datetime-utc = UTC{ $offset }

# Nerede: Saat dilimi satırının alt metni (src/pages/datetime.rs). Anlam: Geçerli saat dilimi ve UTC ofseti.
# Değişkenler: $zone (ör. Istanbul, Europe), $offset (ör. UTC+03:00).
settings-datetime-zone-with-offset = { $zone } — { $offset }

## Varsayılan uygulamalar

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Kabuktan uygulama listesi bekleniyor.
settings-defaults-note-loading = Uygulamalar okunuyor…

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Masaüstü kabuğu çalışmadığı için uygulama listesi gösterilemiyor.
settings-defaults-note-not-running = Masaüstü kabuğu çalışmıyor; varsayılan uygulamalar kabuk açılınca gösterilir.

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Kabuğa bağlanıldı ama yanıt zamanında gelmedi ya da bağlantı koptu.
settings-defaults-note-failed = Varsayılan uygulamalar kabuktan okunamadı; pencereye dönünce yeniden denenir.

# Nerede: Varsayılan uygulamalar sayfası, kabuk çalışmıyorken maskotun altındaki başlık. Anlam: masaüstü kabuğu kapalı.
settings-defaults-empty-not-running-title = Masaüstü kabuğu çalışmıyor

# Nerede: Varsayılan uygulamalar sayfası, kabuk çalışmıyorken başlığın altındaki açıklama. Anlam: liste kabuk açılınca kendiliğinden gelir.
settings-defaults-empty-not-running-body = Varsayılan uygulamalar kabuk açılınca burada görünür.

# Nerede: Varsayılan uygulamalar sayfası, kabuktan okuma başarısızken maskotun altındaki başlık. Anlam: liste okunamadı.
settings-defaults-empty-failed-title = Varsayılan uygulamalar okunamadı

# Nerede: Varsayılan uygulamalar sayfası, okuma başarısızken başlığın altındaki açıklama. Anlam: pencereye dönünce yeniden denenir.
settings-defaults-empty-failed-body = Pencereye dönünce yeniden denenir.

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Çalışan kabuk sürümü bu özelliği bilmiyor.
settings-defaults-note-unsupported = Kabuğun bu sürümü varsayılan uygulamaları yönetmiyor; kabuğu güncelleyin.

# Nerede: Varsayılan uygulamalar, rol satırının açılır seçicisi (src/pages/default_apps.rs). Anlam: Bu dosya türünü şu an hiçbir uygulama açmıyor; seçilemeyen yer tutucu.
settings-defaults-none = Seçilmemiş

# Nerede: Varsayılan uygulamalar, rol satırının açılır seçicisi (src/pages/default_apps.rs). Anlam: Mevcut varsayılan uygulama artık kurulu değil.
# Değişkenler: $name (uygulamanın adı ya da kimliği, ör. "firefox.desktop").
settings-defaults-not-installed = { $name } (kurulu değil)

# Nerede: Varsayılan uygulamalar, rol satırının açılır seçicisi (src/pages/default_apps.rs). Anlam: Aynı adlı iki uygulamayı kimlikleriyle ayırır.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox"), $id (masaüstü kimliği, ör. "firefox-esr.desktop").
settings-defaults-app-with-id = { $name } ({ $id })

# Nerede: Varsayılan uygulamalar sayfasında, rol satırının açılır seçicisindeki bölüm başlığı (src/pages/default_apps.rs). Anlam: Menülerde gösterilmeyen (NoDisplay) ama bu dosya türünü açabilen yardımcı uygulamalar bu başlığın altında listelenir.
settings-defaults-other = Diğer

# Nerede: Varsayılan uygulamalar, Terminal satırının altındaki not (src/pages/default_apps.rs). Anlam: Terminal seçimi için gereken araç kurulu değil; seçici kapalı.
settings-defaults-note-terminal-unavailable = Terminal seçimi için xdg-terminal-exec gerekli; kurulu değil.

# Nerede: Varsayılan uygulamalar, rol satırının altındaki not (src/pages/default_apps.rs). Anlam: Bu rolün dosya türlerinden bazıları farklı uygulamalarda açılıyor; yeni seçim hepsini tek uygulamaya bağlar.
settings-defaults-note-mixed = Bazı dosya türleri başka uygulamada açılıyor; bir seçim yapınca hepsi aynı uygulamada açılır.

# Nerede: Varsayılan uygulamalar, rol satırının altındaki uyarı (src/pages/default_apps.rs). Anlam: Masaüstüne özgü bir ayar dosyası bu seçimi geçersiz kılıyor; Nuxut o dosyaya dokunmuyor.
# Değişkenler: $file (dosya yolu, ör. "~/.config/hyprland-mimeapps.list").
settings-defaults-note-shadowed = { $file } bu seçimi geçersiz kılıyor; Nuxut o dosyayı değiştirmez.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk bu rolü tanımıyor; Ayarlar ile kabuğun sürümleri uyuşmuyor.
settings-defaults-error-unknown-role = Kabuk bu rolü tanımıyor; Ayarlar ile kabuğu güncelleyin.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Seçilen uygulama bu rol için uygun değil ya da kaldırılmış.
settings-defaults-error-unknown-app = Bu uygulama artık kurulu değil ya da bu tür dosyaları açamıyor; liste yenilendi.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Terminal seçimi için gereken araç yok.
settings-defaults-error-unavailable = Terminal seçimi için xdg-terminal-exec gerekli; seçim kaydedilmedi.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk bu oturumda ayar dosyalarına yazmıyor.
settings-defaults-error-writer-disabled = Kabuk bu oturumda ayar dosyalarına yazmıyor; seçim kaydedilmedi.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: mimeapps.list dosyası yazılamadı.
settings-defaults-error-write-failed = mimeapps.list yazılamadı; seçim kaydedilmedi.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk isteği reddetti ya da yanıtı anlaşılamadı.
settings-defaults-error-rejected = Kabuk seçimi kabul etmedi; Ayarlar ile kabuğun güncel olduğundan emin olun.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk çalışmıyor.
settings-defaults-error-not-running = Masaüstü kabuğu çalışmıyor; seçim kaydedilmedi.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Yanıt zamanında gelmedi; seçim yapılmış olabilir, durum yeniden okunuyor.
settings-defaults-error-timeout = Kabuk zamanında yanıt vermedi; seçimin durumu yeniden okunuyor.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuğa bağlanırken hata oluştu.
# Değişkenler: $error (bağlantı hatasının metni).
settings-defaults-error-io = Kabuğa ulaşılamadı: { $error }

## Ekranlar: Gece ışığı

# Nerede: Ekranlar › Gece ışığı, "Konum" satırı; zamanlama "Gün batımından gün doğumuna" iken (src/pages/night_light.rs). Anlam: Gün batımı/doğumu için kullanılan konum, saat diliminin şehrinden yaklaşık olarak bulundu.
# Değişkenler: $lat (enlem, yön harfiyle, ör. "41,0° K"), $lon (boylam, yön harfiyle, ör. "29,0° D").
settings-night-light-location-timezone = Konum: saat diliminden (≈ { $lat }, { $lon })

# Nerede: Aynı satır; konum kullanıcının ayar dosyasında elle yazılıysa (src/pages/night_light.rs). Anlam: Elle girilmiş konum kullanılıyor.
# Değişkenler: $lat (enlem, yön harfiyle, ör. "41,0° K"), $lon (boylam, yön harfiyle, ör. "29,0° D").
settings-night-light-location-manual = Konum: elle ({ $lat }, { $lon })

# Nerede: Aynı satır; konum bulunamadığında (src/pages/night_light.rs). Anlam: Gün batımı hesaplanamıyor, sabit 20:00–07:00 aralığı kullanılıyor (saatler sabit, çevrilirken aynen kalır).
settings-night-light-location-none = Konum bulunamadı — 20:00–07:00 kullanılıyor

# Nerede: Konum bulunamadığında satırın altındaki küçük not (src/pages/night_light.rs). Anlam: Konum elle ayar dosyasına yazılabilir; dosya ve anahtar adları çevrilmez.
settings-night-light-location-hint = Elle konum: shell.toml [night_light] latitude, longitude

# Nerede: Konum satırının altındaki küçük not (src/pages/night_light.rs). Anlam: Bugün güneşin batış ve doğuş saati (yerel saat).
# Değişkenler: $sunset (gün batımı saati, ör. "18:32"), $sunrise (gün doğumu saati, ör. "07:14").
settings-night-light-today = Bugün gün batımı { $sunset } · gün doğumu { $sunrise }

# Nerede: Aynı not; kutup gününde (src/pages/night_light.rs). Anlam: Bugün güneş hiç batmıyor, gece ışığı açılmaz.
settings-night-light-polar-day = Bugün güneş batmıyor

# Nerede: Aynı not; kutup gecesinde (src/pages/night_light.rs). Anlam: Bugün güneş hiç doğmuyor, gece ışığı tüm gün açık.
settings-night-light-polar-night = Bugün güneş doğmuyor

# Nerede: Konum satırındaki enlem (src/pages/night_light.rs). Anlam: Kuzey enlemi; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "41,0").
settings-night-light-north = { $value }° K

# Nerede: Konum satırındaki enlem (src/pages/night_light.rs). Anlam: Güney enlemi; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "33,9").
settings-night-light-south = { $value }° G

# Nerede: Konum satırındaki boylam (src/pages/night_light.rs). Anlam: Doğu boylamı; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "29,0").
settings-night-light-east = { $value }° D

# Nerede: Konum satırındaki boylam (src/pages/night_light.rs). Anlam: Batı boylamı; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "74,0").
settings-night-light-west = { $value }° B


# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.mode ayarının satır başlığı. Anlam: Kullanıcıya koyu ve açık renk kipi seçeneğini tanıtır.
settings-colors-mode-label = Koyu ya da açık

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.mode ayarının alt yazısı. Anlam: Nuxut ile uygulamaların aynı renk kipini paylaştığını belirtir.
settings-colors-mode-description = Nuxut ve uygulamalar aynı kipte

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.mode ayarının arama sözcükleri. Anlam: Bu sözcüklerle renk kipi ayarı aramada bulunur.
settings-colors-mode-keywords = karanlık, aydınlık, dark, light, kip, gece, tema

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette.mode ayarının satır başlığı. Anlam: Renk şeması kipi seçiminin başlığıdır.
settings-palette-mode-label = Renk şemasının kipi

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.source ayarının satır başlığı. Anlam: Renklerin kaynağının seçildiği satırın başlığıdır.
settings-colors-source-label = Renkler nereden gelsin

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.source ayarının alt yazısı. Anlam: Duvar kağıdı seçildiğinde tüm renklerin görsele uyacağını anlatır.
settings-colors-source-description = Nuxut teması: hazır renk temaları. Duvar kağıdından: tüm renkler görsele uyar

# Nerede: Görünüm › Renkler, colors.theme ayarının satır başlığı (src/registry.rs). Anlam: altı hazır Nuxut renk temasından birinin seçildiği satır.
settings-colors-theme-label = Renk teması

# Nerede: Aynı satırın alt yazısı. Anlam: seçilen temanın arayüze, uygulamalara ve yerleşik Nuxut duvar kağıtlarına uygulandığı.
settings-colors-theme-description = Arayüz, uygulamalar ve Nuxut duvar kağıtları bu renklere boyanır

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette ayarının alt yazısı. Anlam: Aynı görselden dokuz farklı palet seçilebildiğini anlatır.
settings-palette-description = Aynı görselden dokuz farklı palet

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette ayarının arama sözcükleri. Anlam: Bu sözcüklerle palet ayarı aramada bulunur.
settings-palette-keywords = renk şeması, matugen, palet, varyant, tonal, canlı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.primary ayarının satır başlığı. Anlam: Vurgu rengi seçiminin başlığıdır.
settings-colors-primary-label = Vurgu rengi

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.primary ayarının alt yazısı. Anlam: Vurgu renginin etkin çalışma alanında, seçimlerde ve düğmelerde kullanıldığını sıralar.
settings-colors-primary-description = Etkin çalışma alanı, seçim ve düğmeler

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_primary ayarının satır başlığı. Anlam: Vurgu rengi üstündeki yazı renginin başlığıdır.
settings-colors-on-primary-label = Vurgu üzerindeki yazı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_primary ayarının alt yazısı. Anlam: Vurgu renginin üstündeki yazı ve ikonları etkilediğini anlatır.
settings-colors-on-primary-description = Vurgu renginin üstündeki yazı ve ikonlar

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_primary ayarının arama sözcükleri. Anlam: Bu sözcükle on_primary rengi aramada bulunur.
settings-colors-on-primary-keywords = vurgu üstü metin

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_container ayarının alt yazısı. Anlam: Panel içindeki kartların bu rengi kullandığını anlatır.
settings-colors-surface-container-description = Panel içindeki kartlar

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_bright ayarının satır başlığı. Anlam: Vurgulu yüzey renginin başlığıdır.
settings-colors-surface-bright-label = Vurgulu yüzey

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_bright ayarının alt yazısı. Anlam: Bu rengin fareyle üzerine gelindiğinde kullanıldığını anlatır.
settings-colors-surface-bright-description = Fareyle üzerine gelince

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface ayarının satır başlığı. Anlam: Ana yazı renginin başlığıdır.
settings-colors-on-surface-label = Yazı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface ayarının alt yazısı. Anlam: Bunun arayüzdeki ana yazı rengi olduğunu anlatır.
settings-colors-on-surface-description = Ana yazı rengi

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface_variant ayarının satır başlığı. Anlam: İkincil yazı renginin başlığıdır.
settings-colors-on-surface-variant-label = İkincil yazı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface_variant ayarının alt yazısı. Anlam: Açıklama ve soluk yazılarda kullanıldığını anlatır.
settings-colors-on-surface-variant-description = Açıklamalar, soluk yazı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface_variant ayarının arama sözcükleri. Anlam: Bu sözcüklerle ikincil yazı rengi aramada bulunur.
settings-colors-on-surface-variant-keywords = ikincil metin

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_error ayarının satır başlığı. Anlam: Hata rengi üstündeki yazının başlığıdır.
settings-colors-on-error-label = Hata üzerindeki yazı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_error ayarının alt yazısı. Anlam: Hata renginin üstündeki yazıyı etkilediğini anlatır.
settings-colors-on-error-description = Hata renginin üstündeki yazı

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_error ayarının arama sözcükleri. Anlam: Bu sözcüklerle hata üstü yazı rengi aramada bulunur.
settings-colors-on-error-keywords = hata üstü metin

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_small ayarının alt yazısı. Anlam: Küçük köşe yarıçapının çiplerde ve çalışma alanı göstergelerinde kullanıldığını sıralar.
settings-shape-radius-small-description = Çipler, çalışma alanı göstergeleri

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_small ayarının arama sözcükleri. Anlam: Bu sözcüklerle küçük yarıçap ayarı aramada bulunur.
settings-shape-radius-small-keywords = köşe, yarıçap, radius, yuvarlak

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_medium ayarının alt yazısı. Anlam: Orta köşe yarıçapının panel içindeki kartlarda kullanıldığını anlatır.
settings-shape-radius-medium-description = Panel içindeki kartlar

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_medium ayarının arama sözcükleri. Anlam: Bu sözcüklerle orta yarıçap ayarı aramada bulunur.
settings-shape-radius-medium-keywords = köşe, yarıçap, radius, yuvarlak

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_large ayarının arama sözcükleri. Anlam: Bu sözcüklerle büyük yarıçap ayarı aramada bulunur.
settings-shape-radius-large-keywords = köşe, yarıçap, radius, yuvarlak

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.smoothing ayarının satır başlığı. Anlam: Birleşim kavisi ayarının başlığıdır.
settings-blob-smoothing-label = Birleşim kavisi

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.smoothing ayarının alt yazısı. Anlam: Çerçevenin iç köşelerinin ve panel birleşimlerinin yumuşatıldığını anlatır.
settings-blob-smoothing-description = Çerçevenin iç köşeleri ve panel birleşimleri

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.deform ayarının alt yazısı. Anlam: Hızlı açılan panellerin esneyip basıldığını anlatır.
settings-blob-deform-description = Hızlı panellerin esneyip basılması

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.unified ayarının satır başlığı. Anlam: Uygulamaların da Nuxut görünümünü almasını sağlayan ayarın başlığıdır.
settings-desktop-unified-label = Uygulamalar da Nuxut gibi görünsün

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.unified ayarının alt yazısı. Anlam: Renk, ikon, imleç ve yazı tiplerinin Nuxut'tan geldiğini ve her an geri alınabildiğini anlatır.
settings-desktop-unified-description = Renk, ikon, imleç ve yazı tipleri Nuxut'tan; her an geri alınır

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.unified ayarının arama sözcükleri. Anlam: Bu sözcüklerle birleşik görünüm ayarı aramada bulunur.
settings-desktop-unified-keywords = birleşik görünüm, gtk, qt, kde, libadwaita, tema, uyumlu

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.status ayarının satır başlığı. Anlam: Uygulama teması durumunun gösterildiği satırın başlığıdır.
settings-desktop-status-label = Durum

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.status ayarının arama sözcükleri. Anlam: Bu sözcüklerle durum satırı aramada bulunur.
settings-desktop-status-keywords = eksik paket, kurulum

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.icon_theme ayarının satır başlığı. Anlam: İkon teması seçiminin başlığıdır.
settings-desktop-icon-theme-label = İkon teması

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.icon_theme ayarının alt yazısı. Anlam: İkon temasının uygulama, dosya ve klasör ikonlarını etkilediğini anlatır.
settings-desktop-icon-theme-description = Uygulama, dosya ve klasör ikonları

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.icon_theme ayarının arama sözcükleri. Anlam: Bu sözcüklerle ikon teması ayarı aramada bulunur.
settings-desktop-icon-theme-keywords = simge, icon, papirus, adwaita, breeze

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_theme ayarının satır başlığı. Anlam: İmleç teması seçiminin başlığıdır.
settings-desktop-cursor-theme-label = İmleç

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_theme ayarının alt yazısı. Anlam: Fare imlecinin görünümünü belirlediğini anlatır.
settings-desktop-cursor-theme-description = Fare imlecinin görünümü

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_theme ayarının arama sözcükleri. Anlam: Bu sözcüklerle imleç teması ayarı aramada bulunur.
settings-desktop-cursor-theme-keywords = fare, cursor, işaretçi, bibata, adwaita, vimix, capitaine, oreo, volantes, qogir, sweet

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_size ayarının satır başlığı. Anlam: İmleç boyutu ayarının başlığıdır.
settings-desktop-cursor-size-label = İmleç boyutu

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_size ayarının alt yazısı. Anlam: Büyük imlecin daha kolay görüldüğünü anlatır.
settings-desktop-cursor-size-description = Büyük imleç daha kolay görünür

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_size ayarının arama sözcükleri. Anlam: Bu sözcüklerle imleç boyutu ayarı aramada bulunur.
settings-desktop-cursor-size-keywords = fare, cursor, büyüklük, erişilebilirlik

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.qt_backend ayarının satır başlığı. Anlam: Qt uygulama teması seçiminin başlığıdır.
settings-desktop-qt-backend-label = Qt uygulamaları

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.qt_backend ayarının alt yazısı. Anlam: Qt arka ucunun otomatik seçildiğini, kuruluysa KDE eklentisinin yoksa qt6ct'nin kullanıldığını anlatır.
settings-desktop-qt-backend-description = Otomatik: kuruluysa KDE'nin tema eklentisi, yoksa qt6ct

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.qt_backend ayarının arama sözcükleri. Anlam: Bu sözcüklerle Qt arka ucu ayarı aramada bulunur.
settings-desktop-qt-backend-keywords = qt, kde, plasma, qt6ct, breeze, kvantum

# Nerede: Bar sayfasında, bölüm başlığı olmayan ana alandaki bar.preview ayarının satır başlığıdır. Anlam: Kullanıcıya barın canlı önizlemesini gösteren ayarı tanıtır.
settings-bar-preview-label = Bar önizlemesi

# Nerede: Bar sayfasında, bölüm başlığı olmayan ana alandaki bar.preview ayarının arama sözcükleridir. Anlam: Kullanıcı "önizleme" ya da "widget" diye aradığında bu ayarı bulmayı sağlar.
settings-bar-preview-keywords = önizleme, widget, seç

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.selected ayarının satır başlığıdır. Anlam: Kullanıcıya, ayarlarını düzenleyeceği bar widget'ını seçme alanını gösterir.
settings-bar-selected-label = Seçili widget

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.selected ayarının arama sözcükleridir. Anlam: "Widget" ya da "ayar" diye arayan kullanıcının bu ayara ulaşmasını sağlar.
settings-bar-selected-keywords = widget, ayar

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.count ayarının satır başlığıdır. Anlam: Barda aynı anda kaç çalışma alanının görüneceğini belirtir.
settings-bar-widget-workspaces-count-label = Görünen çalışma alanı

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.count ayarının alt yazısıdır. Anlam: Sığmayan çalışma alanlarının oklarla sonraki sayfada göründüğünü anlatır.
settings-bar-widget-workspaces-count-description = Fazlası oklarla sonraki sayfada

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.count ayarının arama sözcükleridir. Anlam: "Çalışma alanı", "sayı" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-widget-workspaces-count-keywords = çalışma alanı, workspace, sayı, sayfa başına

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.style ayarının satır başlığıdır. Anlam: Çalışma alanı widget'ının görünümünü seçme ayarını adlandırır.
settings-bar-widget-workspaces-style-label = Görünüm

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.style ayarının alt yazısıdır. Anlam: Görünümün uygulama ikonu, numara ya da nokta olabileceğini anlatır.
settings-bar-widget-workspaces-style-description = Uygulama ikonu, numara ya da nokta

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.style ayarının arama sözcükleridir. Anlam: "İkon", "numara", "nokta" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-bar-widget-workspaces-style-keywords = çalışma alanı, workspace, ikon, numara, nokta

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.scroll ayarının satır başlığıdır. Anlam: Fare tekerleğiyle çalışma alanı değiştirmeyi açıp kapatma ayarını tanıtır.
settings-bar-widget-workspaces-scroll-label = Tekerlekle geçiş

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.scroll ayarının alt yazısıdır. Anlam: Widget'ın üstünde tekerlek çevrildiğinde çalışma alanının değiştiğini anlatır.
settings-bar-widget-workspaces-scroll-description = Widget'ın üstünde tekerlek çalışma alanını değiştirir

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.scroll ayarının arama sözcükleridir. Anlam: "Tekerlek", "kaydırma" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-widget-workspaces-scroll-keywords = çalışma alanı, workspace, tekerlek, kaydırma, scroll

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.clock.format ayarının satır başlığıdır. Anlam: Saat widget'ının biçimini ayarlama alanını adlandırır.
settings-bar-widget-clock-format-label = Biçim

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.tray.icon_size ayarının satır başlığıdır. Anlam: Sistem tepsisi ikonlarının boyutunu ayarlama alanını adlandırır.
settings-bar-widget-tray-icon-size-label = İkon boyutu

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.tray.icon_size ayarının arama sözcükleridir. Anlam: "Tepsi" ya da "ikon" diye arayan kullanıcının bu ayara ulaşmasını sağlar.
settings-bar-widget-tray-icon-size-keywords = tepsi, sistem tepsisi, tray, ikon

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.left_click ayarının satır başlığıdır. Anlam: Barın başındaki uçta sol tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-start-left-click-label = Sol tık

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.left_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "panel aç" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-bar-edge-actions-start-left-click-keywords = uç, baştaki, sol uç, kenar, tık, eylem, panel aç

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.right_click ayarının satır başlığıdır. Anlam: Barın başındaki uçta sağ tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-start-right-click-label = Sağ tık

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.right_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "eylem" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-start-right-click-keywords = uç, baştaki, sol uç, kenar, tık, eylem

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.scroll ayarının satır başlığıdır. Anlam: Barın başındaki uçta tekerlek çevrince çalışacak eylemi seçtirir.
settings-bar-edge-actions-start-scroll-label = Tekerlek

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.scroll ayarının alt yazısıdır. Anlam: Tekerlek çevrildiğinde widget'ın yanında bir ikon belirdiğini anlatır.
settings-bar-edge-actions-start-scroll-description = Sistem eyleminde widget'ın yanında ikon belirir

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.scroll ayarının arama sözcükleridir. Anlam: "Kaydırma", "ses", "parlaklık" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-start-scroll-keywords = uç, baştaki, sol uç, kenar, kaydırma, scroll, ses, parlaklık, çalışma alanı

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.left_click ayarının satır başlığıdır. Anlam: Barın sonundaki uçta sol tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-end-left-click-label = Sol tık

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.left_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "panel aç" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-bar-edge-actions-end-left-click-keywords = uç, sondaki, sağ uç, kenar, tık, eylem, panel aç

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.right_click ayarının satır başlığıdır. Anlam: Barın sonundaki uçta sağ tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-end-right-click-label = Sağ tık

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.right_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "eylem" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-end-right-click-keywords = uç, sondaki, sağ uç, kenar, tık, eylem

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.scroll ayarının satır başlığıdır. Anlam: Barın sonundaki uçta tekerlek çevrince çalışacak eylemi seçtirir.
settings-bar-edge-actions-end-scroll-label = Tekerlek

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.scroll ayarının alt yazısıdır. Anlam: Tekerlek çevrildiğinde widget'ın yanında bir ikon belirdiğini anlatır.
settings-bar-edge-actions-end-scroll-description = Sistem eyleminde widget'ın yanında ikon belirir

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.scroll ayarının arama sözcükleridir. Anlam: "Kaydırma", "ses", "parlaklık" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-end-scroll-keywords = uç, sondaki, sağ uç, kenar, kaydırma, scroll, ses, parlaklık, çalışma alanı

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.scroll_step ayarının satır başlığıdır. Anlam: Tekerleğin bir çentiğinde ne kadar değişeceğini ayarlar.
settings-bar-edge-actions-scroll-step-label = Tekerlek adımı

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.scroll_step ayarının alt yazısıdır. Anlam: Her çentikte yüzdenin ne kadar değiştiğini anlatır.
settings-bar-edge-actions-scroll-step-description = Bir çentikte değişen yüzde

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.scroll_step ayarının arama sözcükleridir. Anlam: "Adım", "tekerlek", "ses" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-scroll-step-keywords = adım, kaydırma adımı, tekerlek, ses, parlaklık

# Nerede: Bar sayfasındaki Konum ve boyut bölümündeki bar.size ayarının satır başlığıdır. Anlam: Barın kalınlığını ayarlama alanını adlandırır.
settings-bar-size-label = Kalınlık

# Nerede: Bar sayfasındaki Konum ve boyut bölümündeki bar.size ayarının alt yazısıdır. Anlam: Bar kenarındaki çerçevenin de aynı kalınlığı kullandığını anlatır.
settings-bar-size-description = Bar'ın kenarındaki çerçeve de bu kalınlıktadır

# Nerede: Bar sayfasındaki Konum ve boyut bölümündeki bar.padding ayarının satır başlığıdır. Anlam: Barın uçlarındaki boşluğu ayarlama alanını adlandırır.
settings-bar-padding-label = Uçlardaki boşluk

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.enabled ayarının alt yazısıdır. Anlam: Ekranı saran ince kenarın kapatıldığında yalnızca barın kaldığını anlatır.
settings-frame-enabled-description = Ekranı saran ince kenar; kapalıyken yalnızca bar kalır

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.thickness ayarının satır başlığıdır. Anlam: Çerçevenin kalınlığını ayarlama alanını adlandırır.
settings-frame-thickness-label = Kalınlık

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.thickness ayarının alt yazısıdır. Anlam: Bu kalınlığın barın dışındaki kenarlara uygulandığını anlatır.
settings-frame-thickness-description = Bar dışındaki kenarlar

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.edges ayarının alt yazısıdır. Anlam: Bar kenarının bar kalınlığını izlediğini, tek bir kenarın ayrı ayarlanıp sonra genele bağlanabildiğini anlatır.
settings-frame-edges-description = Bar kenarı bar kalınlığıdır; bir kenar ayrı ayarlanıp sonra genele bağlanabilir

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.inner_corners ayarının alt yazısıdır. Anlam: İç köşe kavisinin temadan geldiğini, buradan değiştirilemediğini anlatır.
settings-frame-inner-corners-description = Kavis temadan gelir

# Nerede: Wallpaper sayfasında, bölüm başlığı olmayan ana alandaki wallpaper ayarının alt yazısıdır. Anlam: Düz görsel ya da hareketli GIF/WebP/APNG seçilebildiğini anlatır.
settings-wallpaper-description = Görsel ya da hareketli GIF/WebP/APNG

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.fit ayarının alt yazısıdır. Anlam: Görselin ekrana nasıl sığacağını seçme alanını açıklar.
settings-wallpaper-fit-description = Görsel ekrana nasıl sığsın

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop ayarının satır başlığıdır. Anlam: Görselin kenarlarında kalan boşluğu ayarlama alanını adlandırır.
settings-wallpaper-backdrop-label = Kenarlardaki boşluk

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop ayarının alt yazısıdır. Anlam: Bu alanın görsel ekranı tam kaplamadığında göründüğünü anlatır.
settings-wallpaper-backdrop-description = Görsel ekranı kaplamadığında

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop ayarının arama sözcükleridir. Anlam: "Kenar", "siyah", "bulanık" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-wallpaper-backdrop-keywords = kenar boşluğu zemini, letterbox, kenar, bant, siyah, zemin, bulanık

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_blur ayarının satır başlığıdır. Anlam: Kenarlardaki zemin kopyasının bulanıklığını ayarlama alanını adlandırır.
settings-wallpaper-backdrop-blur-label = Bulanıklık

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_blur ayarının alt yazısıdır. Anlam: Kenarlardaki kopyanın ne kadar yumuşak görüneceğini anlatır.
settings-wallpaper-backdrop-blur-description = Kenarlardaki kopyanın yumuşaklığı

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_blur ayarının arama sözcükleridir. Anlam: "Bulanık", "yarıçap", "kenar" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-wallpaper-backdrop-blur-keywords = zemin bulanıklığı, bulanık, blur, yarıçap, kenar

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_color ayarının satır başlığıdır. Anlam: Zemin rengini seçme alanını adlandırır.
settings-wallpaper-backdrop-color-label = Zemin rengi

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_color ayarının alt yazısıdır. Anlam: Bu rengin temayla birlikte değiştiğini anlatır.
settings-wallpaper-backdrop-color-description = Temayla birlikte değişir

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_color ayarının arama sözcükleridir. Anlam: "Renk", "zemin", "yüzey" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-wallpaper-backdrop-color-keywords = renk, kenar, zemin, yüzey

# Nerede: Wallpaper sayfasındaki Renkler bölümündeki palette.source ayarının alt yazısıdır. Anlam: Şema ve kontrast ayarlarının Görünüm sayfasında bulunduğunu anlatır.
settings-palette-source-description = Şema ve kontrast Görünüm'de

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.enabled ayarının satır başlığıdır. Anlam: Ekranın sağ kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-right-enabled-label = Sağ paneli kullan

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.enabled ayarının alt yazısıdır. Anlam: Sağ panelin ekranın sağ kenarından çekmece gibi açıldığını anlatır.
settings-panels-right-enabled-description = Ekranın sağ kenarından açılan çekmece

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-enabled-keywords = sağ panel, çekmece, drawer, kenar paneli

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.size ayarının satır başlığıdır. Anlam: Sağ panelin genişliğini belirler.
settings-panels-right-size-label = Genişlik

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-size-keywords = sağ panel, genişlik, boyut, derinlik

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.widgets ayarının satır başlığıdır. Anlam: Sağ panelde hangi içeriklerin görüneceğini seçer.
settings-panels-right-widgets-label = İçerik

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-widgets-keywords = sağ panel, kart, widget, hızlı ayarlar, medya, bildirim, sıra

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.from_bar ayarının satır başlığıdır. Anlam: Sağ panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-right-from-bar-label = Bar'dan açma

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-from-bar-keywords = sağ panel, uç eylemi, tık, bar

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.enabled ayarının satır başlığıdır. Anlam: Ekranın sol kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-left-enabled-label = Sol paneli kullan

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.enabled ayarının alt yazısıdır. Anlam: Sol panelin ekranın sol kenarından çekmece gibi açıldığını anlatır.
settings-panels-left-enabled-description = Ekranın sol kenarından açılan çekmece

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-enabled-keywords = sol panel, çekmece, drawer, kenar paneli

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.size ayarının satır başlığıdır. Anlam: Sol panelin genişliğini belirler.
settings-panels-left-size-label = Genişlik

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-size-keywords = sol panel, genişlik, boyut, derinlik

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.widgets ayarının satır başlığıdır. Anlam: Sol panelde hangi içeriklerin görüneceğini seçer.
settings-panels-left-widgets-label = İçerik

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-widgets-keywords = sol panel, kart, widget, hızlı ayarlar, medya, bildirim, sıra

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.from_bar ayarının satır başlığıdır. Anlam: Sol panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-left-from-bar-label = Bar'dan açma

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-from-bar-keywords = sol panel, uç eylemi, tık, bar

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.enabled ayarının satır başlığıdır. Anlam: Ekranın üst kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-top-enabled-label = Üst paneli kullan

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.enabled ayarının alt yazısıdır. Anlam: Üst panelin ekranın üst kenarından çekmece gibi açıldığını anlatır.
settings-panels-top-enabled-description = Ekranın üst kenarından açılan çekmece

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-enabled-keywords = üst panel, çekmece, drawer, kenar paneli

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.size ayarının satır başlığıdır. Anlam: Üst panelin yüksekliğini belirler.
settings-panels-top-size-label = Yükseklik

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-size-keywords = üst panel, yükseklik, boyut, derinlik

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.widgets ayarının satır başlığıdır. Anlam: Üst panelde hangi içeriklerin görüneceğini seçer.
settings-panels-top-widgets-label = İçerik

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-widgets-keywords = üst panel, kart, widget, hızlı ayarlar, medya, bildirim, sıra

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.from_bar ayarının satır başlığıdır. Anlam: Üst panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-top-from-bar-label = Bar'dan açma

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-from-bar-keywords = üst panel, uç eylemi, tık, bar

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.enabled ayarının satır başlığıdır. Anlam: Ekranın alt kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-bottom-enabled-label = Alt paneli kullan

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.enabled ayarının alt yazısıdır. Anlam: Alt panelin ekranın alt kenarından çekmece gibi açıldığını anlatır.
settings-panels-bottom-enabled-description = Ekranın alt kenarından açılan çekmece

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-enabled-keywords = alt panel, çekmece, drawer, kenar paneli

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.size ayarının satır başlığıdır. Anlam: Alt panelin yüksekliğini belirler.
settings-panels-bottom-size-label = Yükseklik

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-size-keywords = alt panel, yükseklik, boyut, derinlik

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.widgets ayarının satır başlığıdır. Anlam: Alt panelde hangi içeriklerin görüneceğini seçer.
settings-panels-bottom-widgets-label = İçerik

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-widgets-keywords = alt panel, kart, widget, hızlı ayarlar, medya, bildirim, sıra

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.from_bar ayarının satır başlığıdır. Anlam: Alt panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-bottom-from-bar-label = Bar'dan açma

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-from-bar-keywords = alt panel, uç eylemi, tık, bar

# Nerede: Fonts sayfasında, hiçbir bölüme bağlı olmayan typography.preview ayarının satır başlığıdır. Anlam: Yazı tipi ayarlarının etkisini gösteren önizlemeyi açar.
settings-typography-preview-label = Önizleme

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.size ayarının alt yazısıdır. Anlam: Tüm yazıların varsayılan taban boyutunu belirler.
settings-typography-size-description = Tüm yazıların taban boyutu

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.family ayarının satır başlığıdır. Anlam: Düğme ve menü gibi arayüz öğelerinin yazı tipini seçer.
settings-typography-ui-family-label = Arayüz

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin düğmelerde, menülerde ve ayar satırlarında kullanıldığını anlatır.
settings-typography-ui-family-description = Düğmeler, menüler, ayar satırları

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-ui-family-keywords = font, aile, yazı tipi, kalınlık, weight, inter, sans

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.family ayarının satır başlığıdır. Anlam: Saat ve büyük başlıkların yazı tipini seçer.
settings-typography-display-family-label = Başlık

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin saatte ve büyük başlıklarda kullanıldığını anlatır.
settings-typography-display-family-description = Saat, büyük başlıklar

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-display-family-keywords = font, aile, yazı tipi, kalınlık, weight, saat, display

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.family ayarının satır başlığıdır. Anlam: Uzun metinlerin yazı tipini seçer.
settings-typography-document-family-label = Belge

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin uzun metinlerde, bildirim gövdesinde ve takvim notlarında kullanıldığını anlatır.
settings-typography-document-family-description = Uzun metin: bildirim gövdesi, takvim notları

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-document-family-keywords = font, aile, yazı tipi, kalınlık, weight, serif, okuma

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.family ayarının satır başlığıdır. Anlam: Kod ve sayı tablolarının yazı tipini seçer.
settings-typography-mono-family-label = Eş aralıklı

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin kodda, sayı tablolarında ve Nerd Font ikonlarında kullanıldığını anlatır.
settings-typography-mono-family-description = Kod ve sayı tabloları; Nerd Font ikonlarının da yazı tipi

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-mono-family-keywords = font, aile, yazı tipi, kalınlık, weight, monospace, kod, nerd font

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.scale ayarının satır başlığıdır. Anlam: Arayüz yazılarının boyut oranını belirler.
settings-typography-ui-scale-label = Arayüz boyut oranı

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-ui-scale-keywords = font, büyüklük, ölçek, oran

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.scale ayarının satır başlığıdır. Anlam: Başlık yazılarının boyut oranını belirler.
settings-typography-display-scale-label = Başlık boyut oranı

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-display-scale-keywords = font, büyüklük, ölçek, oran

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.scale ayarının satır başlığıdır. Anlam: Belge metinlerinin boyut oranını belirler.
settings-typography-document-scale-label = Belge boyut oranı

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-document-scale-keywords = font, büyüklük, ölçek, oran

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.scale ayarının satır başlığıdır. Anlam: Eş aralıklı yazıların boyut oranını belirler.
settings-typography-mono-scale-label = Eş aralıklı boyut oranı

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-mono-scale-keywords = font, büyüklük, ölçek, oran

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.family ayarının satır başlığıdır. Anlam: İkonların çizildiği yazı tipini seçer.
settings-typography-icon-family-label = İkon yazı tipi

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.family ayarının alt yazısıdır. Anlam: Boş bırakılırsa ikon setinin kendi yazı tipinin kullanılacağını anlatır.
settings-typography-icon-family-description = Boşsa ikon setinin kendi yazı tipi

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-icon-family-keywords = font, ikon fontu, sembol, kalınlık, weight, material symbols

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.scale ayarının satır başlığıdır. Anlam: İkonların boyut oranını belirler.
settings-typography-icon-scale-label = İkon boyut oranı

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_inner ayarının satır başlığı. Anlam: Yan yana iki pencere arasında bırakılacak boşluğu belirtir.
settings-windows-gap-inner-label = Pencereler arası boşluk

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_inner ayarının alt yazısı. Anlam: Boşluğun yan yana iki pencere arasına uygulandığını açıklar.
settings-windows-gap-inner-description = Yan yana iki pencere arasında

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_outer ayarının satır başlığı. Anlam: Pencereler ile ekran kenarı arasındaki boşluğu belirtir.
settings-windows-gap-outer-label = Ekran kenarına boşluk

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_outer ayarının alt yazısı. Anlam: Boşluğun pencerelerle ekran kenarı ve bar arasında uygulandığını açıklar.
settings-windows-gap-outer-description = Pencerelerle ekran kenarı ve bar arasında

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.border ayarının satır başlığı. Anlam: Pencere kenarlığı ayarının başlığıdır.
settings-windows-border-label = Kenarlık

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.border ayarının alt yazısı. Anlam: Kenarlığın etkin pencerede vurgu rengiyle çizildiğini, 0 değerinin kenarlığı kapattığını anlatır.
settings-windows-border-description = Etkin pencerede vurgu renginde; 0 kapatır

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur ayarının satır başlığı. Anlam: Saydam pencerelerin arkasının bulanık gösterilmesini sağlayan ayarın başlığıdır.
settings-windows-blur-label = Saydam pencerelerin ardı bulanık

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur ayarının alt yazısı. Anlam: Bu özelliğin buzlu cam görünümü verdiğini açıklar.
settings-windows-blur-description = Buzlu cam görünümü

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur_size ayarının satır başlığı. Anlam: Bulanıklığın yoğunluğunu belirleyen ayarın başlığıdır.
settings-windows-blur-size-label = Yoğunluk

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur_size ayarının alt yazısı. Anlam: Değer büyüdükçe bulanıklığın daha yumuşak olacağını anlatır.
settings-windows-blur-size-description = Büyük değer daha yumuşak

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.corners ayarının satır başlığı. Anlam: Pencere köşelerini, gölgesini ve hareketlerini içeren ayarın başlığıdır.
settings-windows-corners-label = Köşe, gölge ve hareket

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.corners ayarının alt yazısı. Anlam: Bu ayarın bar ve panellerle ortak olduğunu açıklar.
settings-windows-corners-description = Bar ve panellerle ortak

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.shadow ayarının satır başlığı. Anlam: Pencere gölgesini yöneten ayarın başlığıdır.
settings-windows-shadow-label = Pencere gölgesi

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.shadow ayarının alt yazısı. Anlam: Pencere gölgesinin bar ve panellerin gölgesiyle aynı olduğunu anlatır.
settings-windows-shadow-description = Bar ve panellerinkiyle aynı

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.motion ayarının satır başlığı. Anlam: Pencere hareket animasyonlarını yöneten ayarın başlığıdır.
settings-windows-motion-label = Pencere hareketleri

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.motion ayarının alt yazısı. Anlam: Açılıp kapanmanın ve çalışma alanı geçişlerinin Hareket sayfasındaki ayarı izlediğini anlatır.
settings-windows-motion-description = Açılıp kapanma ve çalışma alanı geçişleri Hareket'i izler

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_look ayarının alt yazısı. Anlam: Köşe, gölge ve bulanıklık ayarlarının Nuxut'tan alındığını açıklar.
settings-compositor-sync-look-description = Köşe, gölge ve bulanıklık Nuxut'tan gelir

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_windows ayarının satır başlığı. Anlam: Pencere boşluklarının ve kenarlığının Nuxut tarafından belirlenmesini isteyen seçeneğin başlığıdır.
settings-compositor-sync-windows-label = Boşluk ve kenarlığı Nuxut belirlesin

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_windows ayarının alt yazısı. Anlam: Seçenek kapalıyken masaüstünün kendi boşluk ve kenarlık ayarının geçerli olduğunu anlatır.
settings-compositor-sync-windows-description = Kapalıyken masaüstünün kendi ayarı geçerli

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_motion ayarının alt yazısı. Anlam: Seçenek kapalıyken masaüstünün kendi animasyonlarının geçerli olduğunu anlatır.
settings-compositor-sync-motion-description = Kapalıyken masaüstünün kendi animasyonları geçerli

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_input ayarının satır başlığı. Anlam: Klavye ve fare ayarlarının Nuxut tarafından belirlenmesini isteyen seçeneğin başlığıdır.
settings-compositor-sync-input-label = Klavye ve fareyi Nuxut belirlesin

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_input ayarının alt yazısı. Anlam: Seçenek kapalıyken masaüstünün kendi klavye ve fare ayarının geçerli olduğunu anlatır.
settings-compositor-sync-input-description = Kapalıyken masaüstünün kendi ayarı geçerli

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_input ayarının arama sözcükleri. Anlam: Bu seçeneği ararken yazılabilecek sözcükleri toplar.
settings-compositor-sync-input-keywords = klavye, fare, dokunmatik, girdi, input, eşitle

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.ensure_source ayarının satır başlığı. Anlam: Nuxut bağlantı satırını masaüstünün ayarlarına ekleyen seçeneğin başlığıdır.
settings-compositor-ensure-source-label = Masaüstü ayarlarına Nuxut'u ekle

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.ensure_source ayarının alt yazısı. Anlam: Ayar dosyasına tek satır eklendiğini ve öncesinde yedek alındığını anlatır.
settings-compositor-ensure-source-description = Ayar dosyasına tek satır eklenir, önce yedek alınır

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.source_note ayarının satır başlığı. Anlam: Masaüstü ayar dosyasına eklenen Nuxut bağlantı satırını gösteren başlıktır.
settings-compositor-source-note-label = Bağlantı satırı

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.source_note ayarının arama sözcükleri. Anlam: Bağlantı satırı ayarını ararken yazılabilecek sözcükleri toplar.
settings-compositor-source-note-keywords = source, bağlantı, kaldırıldı

# Nerede: Motion sayfasında, bölüm olmadan, motion.preset ayarının satır başlığı. Anlam: Hareketin genel karakterini seçen hazır ayarın başlığıdır.
settings-motion-preset-label = Karakter

# Nerede: Motion sayfasında, bölüm olmadan, motion.preset ayarının alt yazısı. Anlam: Bu ayarın tüm hareketlerin karakterini belirlediğini anlatır.
settings-motion-preset-description = Tüm hareketlerin karakteri

# Nerede: Motion sayfasında, Genel bölümündeki motion.enabled ayarının alt yazısı. Anlam: Kapalıyken animasyonların kapandığını ve her şeyin anında değiştiğini anlatır.
settings-motion-enabled-description = Animasyonlar kapanır; her şey anında değişir

# Nerede: Motion sayfasında, Genel bölümündeki motion.speed ayarının alt yazısı. Anlam: Değerin tüm animasyonların çarpanı olduğunu, 2× değerinin iki kat hızlı olacağını anlatır.
settings-motion-speed-description = Tüm animasyonların çarpanı; 2× iki kat hızlı

# Nerede: Motion sayfasında, Genel bölümündeki motion.style ayarının satır başlığı. Anlam: Hareket türünü seçen satırın başlığıdır.
settings-motion-style-label = Hareket türü

# Nerede: Motion sayfasında, Genel bölümündeki motion.style ayarının alt yazısı. Anlam: Yay hareketinin doğal ve kesilebilir olduğunu, eğri hareketin sabit süreli olduğunu açıklar.
settings-motion-style-description = Yay doğal ve kesilebilir; eğri sabit sürelidir

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial ayarının satır başlığı. Anlam: Panellerin ve pencerelerin hareket ayarlarını içeren grubun başlığıdır.
settings-motion-spatial-label = Paneller ve pencereler

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial ayarının alt yazısı. Anlam: Bu grubun açılma, boyut ve konum değişimi animasyonlarını kapsadığını açıklar.
settings-motion-spatial-description = Açılma, boyut ve konum değişimi

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial ayarının arama sözcükleri. Anlam: Bu grubu ararken yazılabilecek sözcükleri toplar.
settings-motion-spatial-keywords = mekânsal, spatial, animasyon, önizleme

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.style ayarının satır başlığı. Anlam: Paneller ve pencereler için hareket türünü seçen satırın başlığıdır.
settings-motion-role-style-label = Hareket türü

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.style ayarının alt yazısı. Anlam: Genel seçildiğinde üstteki hareket türünü izlediğini anlatır.
settings-motion-role-style-description = Genel: üstteki tür

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.response ayarının satır başlığı. Anlam: Hareketin ne kadar sürede tamamlandığını belirleyen ayarın başlığıdır.
settings-motion-role-response-label = Süre

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.response ayarının alt yazısı. Anlam: Kısa sürenin daha hızlı hareket anlamına geldiğini anlatır.
settings-motion-role-response-description = Kısa = hızlı

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.damping ayarının satır başlığı. Anlam: Yay hareketindeki yaylanmayı ayarlayan satırın başlığıdır.
settings-motion-role-damping-label = Yaylanma

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.damping ayarının alt yazısı. Anlam: Değer düşükken hareketin hedefi aşıp geri oturduğunu anlatır.
settings-motion-role-damping-description = Düşük değer hedefi aşıp geri oturur

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.easing ayarının alt yazısı. Anlam: Hareketin hızlanıp yavaşlama biçimini seçen ayarın alt yazısıdır.
settings-motion-role-easing-description = Hızlanıp yavaşlama biçimi

# Nerede: Motion sayfasında, Menüler ve küçük öğeler bölümündeki motion.spatial_fast ayarının satır başlığı. Anlam: Menülerin ve küçük öğelerin hareket ayarlarını içeren grubun başlığıdır.
settings-motion-spatial-fast-label = Menüler ve küçük öğeler

# Nerede: Motion sayfasında, Menüler ve küçük öğeler bölümündeki motion.spatial_fast ayarının alt yazısı. Anlam: Bu grubun menüleri, göstergeleri ve küçük düğmeleri kapsadığını açıklar.
settings-motion-spatial-fast-description = Menüler, göstergeler, küçük düğmeler

# Nerede: Motion sayfasında, Menüler ve küçük öğeler bölümündeki motion.spatial_fast ayarının arama sözcükleri. Anlam: Bu grubu ararken yazılabilecek sözcükleri toplar.
settings-motion-spatial-fast-keywords = hızlı mekânsal, spatial_fast, animasyon, önizleme

# Nerede: Motion sayfasında, Solma ve renk bölümündeki motion.effects ayarının alt yazısı. Anlam: Saydamlık ve renk geçişlerinin animasyonlu olduğunu ve taşma yapmadığını anlatır.
settings-motion-effects-description = Saydamlık ve renk geçişleri; taşmaz

# Nerede: Motion sayfasında, Solma ve renk bölümündeki motion.effects ayarının arama sözcükleri. Anlam: Bu grubu ararken yazılabilecek sözcükleri toplar.
settings-motion-effects-keywords = efektler, effects, opaklık, animasyon, önizleme

# Nerede: Keybinds sayfasında Genel bölümünde, compositor.sync_keybinds ayarının alt yazısı. Anlam: Aynı tuşa bağlı başka bir masaüstü kısayolu varsa onun geçerli kaldığını söyler.
settings-compositor-sync-keybinds-description = Aynı tuşa bağlı masaüstü kısayolu varsa o geçerli kalır

# Nerede: Keybinds sayfasında Ses bölümünde, keybinds.volume_up ayarının arama sözcükleri. Anlam: Ses açma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-volume-up-keywords = kısayol, tuş, volume, xf86

# Nerede: Keybinds sayfasında Ses bölümünde, keybinds.volume_down ayarının arama sözcükleri. Anlam: Ses kısma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-volume-down-keywords = kısayol, tuş, volume, xf86

# Nerede: Keybinds sayfasında Parlaklık bölümünde, keybinds.brightness_up ayarının arama sözcükleri. Anlam: Ekran parlaklığını artırma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-brightness-up-keywords = kısayol, tuş, ekran, brightness, xf86

# Nerede: Keybinds sayfasında Parlaklık bölümünde, keybinds.brightness_down ayarının arama sözcükleri. Anlam: Ekran parlaklığını azaltma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-brightness-down-keywords = kısayol, tuş, ekran, brightness, xf86

# Nerede: Keybinds sayfasında Medya bölümünde, keybinds.media_play ayarının arama sözcükleri. Anlam: Müziği başlatma ya da duraklatma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-media-play-keywords = kısayol, tuş, müzik, playerctl, xf86

# Nerede: Keybinds sayfasında Medya bölümünde, keybinds.media_next ayarının arama sözcükleri. Anlam: Sıradaki parçaya geçme kısayolunu ararken kullanılan kelimeler.
settings-keybinds-media-next-keywords = kısayol, tuş, müzik, playerctl, xf86

# Nerede: Keybinds sayfasında Medya bölümünde, keybinds.media_prev ayarının arama sözcükleri. Anlam: Önceki parçaya dönme kısayolunu ararken kullanılan kelimeler.
settings-keybinds-media-prev-keywords = kısayol, tuş, müzik, playerctl, xf86

# Nerede: Notifications sayfasında Genel bölümünde, notifications.enabled ayarının satır başlığı. Anlam: Bildirimleri Nuxut'un kendisinin göstereceğini söyler.
settings-notifications-enabled-label = Bildirimleri Nuxut göstersin

# Nerede: Notifications sayfasında Genel bölümünde, notifications.enabled ayarının alt yazısı. Anlam: Ayar kapalıyken bildirimleri başka bir uygulamanın gösterebileceğini söyler.
settings-notifications-enabled-description = Kapalıyken başka bir uygulama gösterebilir

# Nerede: Notifications sayfasında Genel bölümünde, notifications.enabled ayarının arama sözcükleri. Anlam: Bildirim ayarını ararken kullanılan kelimeler.
settings-notifications-enabled-keywords = bildirimler, bildirim sunucusu

# Nerede: Notifications sayfasında Genel bölümünde, notifications.dnd ayarının alt yazısı. Anlam: Rahatsız etmeyin açıkken açılır bildirim çıkmadığını ve bildirimlerin merkezde birikeceğini söyler.
settings-notifications-dnd-description = Açılır bildirim yok; hepsi merkezde birikir

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.popup_timeout_ms ayarının satır başlığı. Anlam: Açılır bildirimin ekranda kalacağını belirleyen sürenin ayarı olduğunu söyler.
settings-notifications-popup-timeout-ms-label = Görünme süresi

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.popup_timeout_ms ayarının alt yazısı. Anlam: Açılır bildirimin ekranda kalma süresini açıklar.
settings-notifications-popup-timeout-ms-description = Açılır bildirimin ekranda kalma süresi

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.popup_timeout_ms ayarının arama sözcükleri. Anlam: Açılır bildirim süresini ararken kullanılan kelimeler.
settings-notifications-popup-timeout-ms-keywords = popup, süre, zaman aşımı, açılır bildirimler

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.max_popups ayarının alt yazısı. Anlam: Aynı anda en kaç açılır bildirimin üst üste görünebileceğini açıklar.
settings-notifications-max-popups-description = Üst üste görünen açılır bildirim sayısı

# Nerede: Region sayfasında bölümsüz duran locale.preview ayarının satır başlığı. Anlam: Dil ayarlarının örnek biçim görünümünü gösterdiğini söyler.
settings-locale-preview-label = Biçim önizlemesi

# Nerede: Region sayfasında bölümsüz duran locale.preview ayarının arama sözcükleri. Anlam: Biçim önizlemesini ararken kullanılan kelimeler.
settings-locale-preview-keywords = önizleme, örnek, tarih, saat, sayı, yüzde

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.language ayarının satır başlığı. Anlam: Arayüz dilinin seçildiği ayar olduğunu söyler.
settings-locale-language-label = Dil

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.language ayarının alt yazısı. Anlam: Menü ve metinlerin seçilen dilde gösterileceğini söyler.
settings-locale-language-description = Menüler ve metinler bu dilde gösterilir

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.language ayarının arama sözcükleri. Anlam: Dil ayarını ararken kullanılan kelimeler.
settings-locale-language-keywords = dil, language, lisan, çeviri, türkçe, english, locale

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.formats ayarının satır başlığı. Anlam: Tarih, saat ve sayı biçimlerinin seçildiği bölüm olduğunu söyler.
settings-locale-formats-label = Biçimler

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.formats ayarının alt yazısı. Anlam: Tarih, saat ve sayıların nasıl yazıldığını açıklar.
settings-locale-formats-description = Tarih, saat ve sayıların yazılışı

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.formats ayarının arama sözcükleri. Anlam: Biçim ayarlarını ararken kullanılan kelimeler.
settings-locale-formats-keywords = tarih, saat, sayı, ondalık, bölge, region, format, locale

# Nerede: Sound sayfasında Ses bölümünde, audio.max_volume ayarının alt yazısı. Anlam: Ses kısayollarının ve kaydırıcıların çıkabileceği en yüksek seviyeyi açıklar.
settings-audio-max-volume-description = Kısayol ve kaydırıcıların üst sınırı

# Nerede: Sound sayfasında Ses bölümünde, osd.timeout_ms ayarının satır başlığı. Anlam: Ses ve parlaklık değişimini ekranda gösteren göstergenin süresinin ayarı olduğunu söyler.
settings-osd-timeout-ms-label = Ses ve parlaklık göstergesi

# Nerede: Sound sayfasında Ses bölümünde, osd.timeout_ms ayarının alt yazısı. Anlam: Göstergenin ekranda kalma süresini açıklar.
settings-osd-timeout-ms-description = Ekranda kalma süresi

# Nerede: Sound sayfasında Ses bölümünde, osd.timeout_ms ayarının arama sözcükleri. Anlam: Ekran göstergesini ararken kullanılan kelimeler.
settings-osd-timeout-ms-keywords = osd, ekran göstergesi, gösterge, süre, görünme süresi

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys.link ayarının alt yazısı. Anlam: Klavyedeki ses, parlaklık ve oynatma tuşlarının bu ayarla bağlandığını açıklar.
settings-media-keys-link-description = Klavyedeki ses, parlaklık ve oynatma tuşları

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys ayarının satır başlığı. Anlam: Masaüstü ayar dosyasına eklenebilecek bir örnek gösterdiğini söyler.
settings-media-keys-label = Masaüstü ayar dosyası için örnek

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys ayarının alt yazısı. Anlam: Nuxut'un kendi kısayollarının kapalı olduğunu, tuşların elle bağlanabileceğini söyler.
settings-media-keys-description = Nuxut kısayolları kapalı; tuşlar elle bağlanabilir

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys ayarının arama sözcükleri. Anlam: Medya tuşlarını ve Hyprland bağlama örneğini ararken kullanılan kelimeler.
settings-media-keys-keywords = hyprland bind örneği, xf86, kısayol, bind, playerctl, parlaklık, brightness, mikrofon

# Nerede: Duvar kağıdı sayfasında, görselin kaplamadığı alanın zeminini seçen listede blur seçeneği. Anlam: Boşluğun bulanıklaştırılmış görselle doldurulduğunu söyler.
settings-choice-wallpaper-backdrop-blur = Bulanık görsel

# Nerede: Duvar kağıdı sayfasında, görselin kaplamadığı alanın zeminini seçen listede color seçeneği. Anlam: Boşluğun düz bir renkle doldurulduğunu söyler.
settings-choice-wallpaper-backdrop-color = Düz renk

# Nerede: Duvar kağıdı sayfasında, düz renk zeminin hangi tema rengi olacağını seçen listede surface seçeneği. Anlam: Zeminin normal yüzey rengi olduğunu söyler.
settings-choice-wallpaper-backdrop-color-surface = Yüzey

# Nerede: Duvar kağıdı sayfasında, düz renk zeminin hangi tema rengi olacağını seçen listede surface_container seçeneği. Anlam: Zeminin kart rengi olduğunu söyler.
settings-choice-wallpaper-backdrop-color-surface-container = Kart

# Nerede: Duvar kağıdı sayfasında, düz renk zeminin hangi tema rengi olacağını seçen listede surface_bright seçeneği. Anlam: Zeminin parlak yüzey rengi olduğunu söyler.
settings-choice-wallpaper-backdrop-color-surface-bright = Vurgulu yüzey

# Nerede: Görünüm sayfasında Uygulamalar bölümünde, Qt uygulamalarının tema altyapısını seçen listede auto seçeneği. Anlam: Altyapının otomatik seçildiğini söyler.
settings-choice-qt-backend-auto = Otomatik

# Nerede: Görünüm sayfasında Uygulamalar bölümünde, Qt uygulamalarının tema altyapısını seçen listede kde seçeneği. Anlam: Qt uygulamalarının KDE temasını kullanacağını söyler.
settings-choice-qt-backend-kde = KDE

# Nerede: Görünüm sayfasında Uygulamalar bölümünde, Qt uygulamalarının tema altyapısını seçen listede qt6ct seçeneği. Anlam: Qt uygulamalarının qt6ct ayarlarını kullanacağını söyler.
settings-choice-qt-backend-qt6ct = qt6ct

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede none seçeneği. Anlam: Tıklamada hiçbir şey yapılmadığını söyler.
settings-choice-edge-click-none = Hiçbir şey

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:left seçeneği. Anlam: Tıklamada sol panelin açıldığını söyler.
settings-choice-edge-click-panel-left = Sol paneli aç

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:right seçeneği. Anlam: Tıklamada sağ panelin açıldığını söyler.
settings-choice-edge-click-panel-right = Sağ paneli aç

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:top seçeneği. Anlam: Tıklamada üst panelin açıldığını söyler.
settings-choice-edge-click-panel-top = Üst paneli aç

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:bottom seçeneği. Anlam: Tıklamada alt panelin açıldığını söyler.
settings-choice-edge-click-panel-bottom = Alt paneli aç

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:clock seçeneği. Anlam: Tıklamada takvim panelinin açıldığını söyler.
settings-choice-edge-click-panel-clock = Takvimi aç

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:launcher seçeneği. Anlam: Tıklamada uygulama başlatıcının açıldığını söyler.
settings-choice-edge-click-panel-launcher = Uygulama başlatıcıyı aç

# Nerede: Bar sayfasında, bar ucundaki widget üzerinde tekerlek çevirince yapılacak eylemi seçen listede widget seçeneği. Anlam: Kaydırmanın doğrudan widget'ın kendi işlevini kullandığını söyler.
settings-choice-edge-scroll-widget = Widget'ın kendi

# Nerede: Bar sayfasında, bar ucundaki widget üzerinde tekerlek çevirince yapılacak eylemi seçen listede none seçeneği. Anlam: Tekerlek çevirmede hiçbir şey yapılmadığını söyler.
settings-choice-edge-scroll-none = Hiçbir şey

# Nerede: Bar sayfasında, bar ucundaki widget üzerinde tekerlek çevirince yapılacak eylemi seçen listede workspace seçeneği. Anlam: Tekerleğin çalışma alanları arasında geçiş yaptığını söyler.
settings-choice-edge-scroll-workspace = Çalışma alanı

# Nerede: Hareket sayfasında, hareket rolünün türünü seçen listede spring seçeneği (Genel, üstteki genel türü seçer). Anlam: Hareketin yay gibi esneyerek bittiğini söyler.
settings-choice-role-style-spring = Yay

# Nerede: Hareket sayfasında, hareket rolünün türünü seçen listede curve seçeneği (Genel, üstteki genel türü seçer). Anlam: Hareketin bir eğriye göre yapıldığını söyler.
settings-choice-role-style-curve = Eğri

# Nerede: Kenar çubuğundaki sayfa adı. Anlam: Paneller ayarlarının bulunduğu sayfayı açar.
settings-page-panels = Paneller

# Nerede: Kenar çubuğundaki sayfa adı. Anlam: Bölge ve dil ayarlarının bulunduğu sayfayı açar.
settings-page-region = Bölge ve dil

# Nerede: Hareket sayfasında rol kartının başlığı ve bölüm adı. Anlam: Panellerin ve pencerelerin hareketini ayarlayan bölüm olduğunu söyler.
settings-motion-role-spatial = Paneller ve pencereler

# Nerede: Hareket sayfasında rol kartının başlığı ve bölüm adı. Anlam: Menülerin ve küçük öğelerin hareketini ayarlayan bölüm olduğunu söyler.
settings-motion-role-spatial-fast = Menüler ve küçük öğeler

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Alt panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-bottom = Alt panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Sağ panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-right = Sağ panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Sol panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-left = Sol panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Üst panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-top = Üst panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Davranış ayarlarını içeren bölüm olduğunu söyler.
settings-section-behavior = Davranış

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Dil ve biçim ayarlarını içeren bölüm olduğunu söyler.
settings-section-language-formats = Dil ve biçimler

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Görünüm ayarlarını içeren bölüm olduğunu söyler.
settings-section-appearance = Görünüm

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Konum ve boyut ayarlarını içeren bölüm olduğunu söyler.
settings-section-position-size = Konum ve boyut

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Köşe ve şekil ayarlarını içeren bölüm olduğunu söyler.
settings-section-corners-shape = Köşeler ve şekil

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Monitör başına yapılan ayarları içeren bölüm olduğunu söyler.
settings-section-per-monitor = Monitör başına

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Seçili widget'ın ayarlarını içeren bölüm olduğunu söyler.
settings-section-selected-widget = Seçili widget

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Yazı tipi seçimini içeren bölüm olduğunu söyler.
settings-section-font-families = Yazı tipleri

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Yerleşim ayarlarını içeren bölüm olduğunu söyler.
settings-section-layout = Yerleşim

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Sistem saati ağ saatiyle eşitlendi.
settings-datetime-ntp-synced = Saat eşitlendi

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Otomatik saat açık ama henüz eşitlenmedi.
settings-datetime-ntp-unsynced = Saat henüz eşitlenmedi

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Otomatik saat kapalı, saat elle ayarlanıyor.
settings-datetime-ntp-off = Saat elle ayarlanıyor

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Sistemde ağ saati hizmeti (ör. systemd-timesyncd) yok; anahtar kapalı.
settings-datetime-ntp-unavailable = Bu sistemde ağ saati hizmeti yok

# Nerede: Tarih ve saat sayfasında, tarih ve saat satırlarının alt yazısı (src/pages/datetime.rs). Anlam: Değişiklik için yönetici yetkisi isteniyor, kullanıcının yanıtı bekleniyor.
settings-datetime-waiting = Yetki bekleniyor…

# Nerede: Tarih ve saat sayfasında, tarih ve saat satırlarının alt yazısı (src/pages/datetime.rs). Anlam: Sistem değişikliği kabul etmedi (genel hata).
settings-datetime-failed = Değiştirilemedi

# Nerede: Tarih ve saat sayfasında, tarih ve saat satırlarının alt yazısı (src/pages/datetime.rs). Anlam: Tarih ve saat hizmeti bulunamadığı için değişiklik yapılamadı.
settings-datetime-missing-short = Saat hizmeti bulunamadı

# Nerede: Tarih ve saat sayfasının üstündeki not (src/pages/datetime.rs). Anlam: systemd-timedated hizmeti yok; ayarlar buradan yapılamıyor.
settings-datetime-missing = systemd-timedated bulunamadı; tarih ve saat buradan ayarlanamıyor.

# Nerede: Tarih ve saat sayfasının üstündeki not (src/pages/datetime.rs). Anlam: Sistemden tarih ve saat bilgisi okunamadı.
settings-datetime-read-failed = Tarih ve saat bilgisi okunamadı.

# Nerede: Tarih ve saat sayfasının üstündeki not (src/pages/datetime.rs). Anlam: Donanım saati UTC yerine yerel saatte tutuluyor; yaz saati geçişlerinde kayma uyarısı.
settings-datetime-local-rtc = Donanım saati yerel saatte tutuluyor (LocalRTC). Windows ile birlikte kurulu sistemlerde yaygındır; yaz saati geçişlerinde saat kayabilir.

# Nerede: Tarih ve saat sayfasında, saat dilimi satırındaki düğme (src/pages/datetime.rs). Anlam: Saat dilimi seçicisini açar.
settings-datetime-zone-change = Değiştir

# Nerede: Tarih ve saat sayfasında, seçici açıkken aynı düğme (src/pages/datetime.rs). Anlam: Seçimi yapmadan seçiciyi kapatır.
settings-datetime-zone-cancel = Vazgeç

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisindeki arama alanının yer tutucusu (src/pages/datetime.rs). Anlam: Şehir ya da bölge adıyla arama yapılır.
settings-datetime-zone-search = Şehir ya da bölge ara

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisinin durum metni (src/pages/datetime.rs). Anlam: Saat dilimi listesi sistemden okunuyor.
settings-datetime-zone-loading = Saat dilimleri yükleniyor…

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisinin durum metni (src/pages/datetime.rs). Anlam: Aramaya uyan saat dilimi yok.
settings-datetime-zone-none = Eşleşen saat dilimi yok

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisinin durum metni (src/pages/datetime.rs). Anlam: Saat dilimi listesi sistemden alınamadı.
settings-datetime-zone-list-failed = Saat dilimi listesi alınamadı

# Nerede: Tarih ve saat sayfasında, saat dilimi satırının alt yazısı (src/pages/datetime.rs). Anlam: Geçerli saat dilimi bilinmiyor.
settings-datetime-zone-unknown = Bilinmiyor

# Nerede: Tarih ve saat sayfasında, Elle tarih ve saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Otomatik saat açıkken elle ayar kapalıdır.
settings-datetime-manual-auto-on = Otomatik saat açıkken kapalı

# Nerede: Tarih ve saat sayfasında, Elle tarih ve saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Tarih ve saati seçip Uygula düğmesine basın.
settings-datetime-manual-hint = Tarih ve saati seçip uygulayın

# Nerede: Tarih ve saat sayfasında, Elle tarih ve saat satırındaki düğme (src/pages/datetime.rs). Anlam: Seçilen tarih ve saati sisteme yazar.
settings-datetime-manual-apply = Uygula

# Nerede: Tarih ve saat sayfası, Biçim kartında Saat biçimi seçiminin altındaki not (src/pages/datetime.rs). Anlam: Bar saati için elle strftime kalıbı yazılmış; 12/24 saat seçimi bar saatini etkilemez.
settings-datetime-clock-custom = Bar saati özel biçim kullanıyor

# Nerede: Tarih ve saat sayfası, aynı notun sağındaki düğme (src/pages/datetime.rs). Anlam: Bar saatinin özel biçimini siler; saat yeniden 12/24 saat seçimine uyar.
settings-datetime-clock-clear = Temizle

# Nerede: Bar sayfası, Seçili widget kartında saat widget'ının Biçim ve Dikey bar biçimi alanları boşken görünen yer tutucu (src/registry.rs). Anlam: Alan boş bırakılırsa saat, Tarih ve saat sayfasındaki 12/24 saat seçimine göre gösterilir.
settings-bar-clock-format-auto = Otomatik (12/24 saat tercihine uyar)

# Nerede: Hareket (Motion) sayfasında, Solma ve renk bölümündeki motion.effects ayarının satır başlığı. Anlam: Solma ve renk animasyonlarını içeren grubun başlığıdır.
settings-motion-effects-label = Solma ve renk

# Nerede: Hareket (Motion) sayfasında, rol kartının başlığı ve bölüm adı. Anlam: Solma ve renk geçişlerini ayarlayan bölüm olduğunu söyler.
settings-motion-role-effects = Solma ve renk

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette.mode ayarının (Renk şemasının kipi) arama sözcükleri. Anlam: Bu sözcükler, renk şemasının karanlık ya da aydınlık kipini aramada bulmayı sağlar.
settings-palette-mode-keywords = karanlık, aydınlık, dark, light, palet, kip

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.source ayarının (Renkler nereden gelsin) arama sözcükleri. Anlam: Bu sözcükler, renklerin nereden alınacağını (tema, palet ya da duvar kağıdı) aramada bulmayı sağlar.
settings-colors-source-keywords = renk kaynağı, tema, palet, matugen, material you, duvar kağıdı, otomatik renk

# Nerede: Görünüm › Renkler, colors.theme ayarının (Renk teması) arama sözcükleri. Anlam: tema adlarıyla aramada bulmayı sağlar.
settings-colors-theme-keywords = renk teması, tema, mavi, nötr, adaçayı, gül kurusu, kehribar, grafit, renk

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.primary ayarının (Vurgu rengi) arama sözcükleri. Anlam: Bu sözcükler, arayüzün vurgu rengini aramada bulmayı sağlar.
settings-colors-primary-keywords = vurgu, accent, ana renk, tema rengi

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_bright ayarının (Vurgulu yüzey) arama sözcükleri. Anlam: Bu sözcükler, üzerine gelindiğinde parlayan yüzeyin rengini aramada bulmayı sağlar.
settings-colors-surface-bright-keywords = hover, parlak yüzey

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface ayarının (Yazı) arama sözcükleri. Anlam: Bu sözcükler, yüzeylerin üzerindeki metin ve yazı rengini aramada bulmayı sağlar.
settings-colors-on-surface-keywords = metin, yazı rengi

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.smoothing ayarının (Birleşim kavisi) arama sözcükleri. Anlam: Bu sözcükler, birleşen köşelerin yumuşaklığını, yani iç kavis ve fileto yumuşaklığını aramada bulmayı sağlar.
settings-blob-smoothing-keywords = birleşim yumuşaklığı, blob, fileto, puding, smoothing, iç kavis, iç köşe, çerçeve

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.scale ayarının (İkon boyut oranı) arama sözcükleri. Anlam: Bu sözcükler, simgelerin büyüklük ve ölçek oranını aramada bulmayı sağlar.
settings-typography-icon-scale-keywords = ikon, büyüklük, ölçek, oran

# Nerede: Motion sayfasında, bölümsüz duran motion.preset ayarının (Karakter) arama sözcükleri. Anlam: Bu sözcükler, hareketin genel karakterini (sakin, canlı, çevik ya da kapalı) aramada bulmayı sağlar.
settings-motion-preset-keywords = ön ayar, sakin, canlı, çevik, kapalı, puding, preset, animasyon

# Nerede: Motion sayfasındaki Genel bölümünde, motion.style ayarının (Hareket türü) arama sözcükleri. Anlam: Bu sözcükler, hareketin yay mı eğri mi olacağını, yani hareket türünü aramada bulmayı sağlar.
settings-motion-style-keywords = stil, yay, eğri, spring, curve, bezier, easing

# Nerede: Motion sayfasındaki Paneller ve pencereler bölümünde, motion.spatial.style ayarının (Hareket türü) arama sözcükleri. Anlam: Bu sözcükler, panellerin ve pencerelerin hareket türünü aramada bulmayı sağlar.
settings-motion-role-style-keywords = stil, yay, eğri, spring, curve

# Nerede: Motion sayfasındaki Paneller ve pencereler bölümünde, motion.spatial.damping ayarının (Yaylanma) arama sözcükleri. Anlam: Bu sözcükler, hareketin yaylanma ve sönüm miktarını aramada bulmayı sağlar.
settings-motion-role-damping-keywords = sönüm, damping, zıplama, puding

# Nerede: Bar sayfasındaki Seçili widget bölümünde, bar.widget.clock.format ayarının (Biçim) arama sözcükleri. Anlam: Bu sözcükler, saat widget'ının saat biçimini (12 ya da 24 saat, otomatik) aramada bulmayı sağlar.
settings-bar-widget-clock-format-keywords = saat, saat biçimi, tarih, format, strftime, 12 saat, 24 saat, otomatik

# Nerede: Bar sayfasındaki Widget'lar bölümünde, bar.sections ayarının (Bar bölümleri) arama sözcükleri. Anlam: Bu sözcükler, barda hangi widget'ların (saat, tepsi, çalışma alanı) sıralanacağını aramada bulmayı sağlar.
settings-bar-sections-keywords = widget, saat, tepsi, çalışma alanı, workspace, sıra

# Nerede: Bar sayfasındaki Konum ve boyut bölümünde, bar.edge ayarının (Ekran kenarı) arama sözcükleri. Anlam: Bu sözcükler, barın ekranın üst, alt, sol ya da sağ kenarında duracağını aramada bulmayı sağlar.
settings-bar-edge-keywords = konum, üst, alt, sol, sağ, dikey, panel

# Nerede: Bar sayfasındaki Konum ve boyut bölümünde, bar.size ayarının (Kalınlık) arama sözcükleri. Anlam: Bu sözcükler, barın kalınlığını, yani yükseklik ve boyutunu aramada bulmayı sağlar.
settings-bar-size-keywords = bar kalınlığı, yükseklik, boyut, kenar

# Nerede: Bar sayfasındaki Konum ve boyut bölümünde, bar.padding ayarının (Uçlardaki boşluk) arama sözcükleri. Anlam: Bu sözcükler, barın uçlarındaki iç boşluğu aramada bulmayı sağlar.
settings-bar-padding-keywords = uç boşluğu, padding, iç boşluk

# Nerede: Bar sayfasındaki Çerçeve bölümünde, frame.thickness ayarının (Kalınlık) arama sözcükleri. Anlam: Bu sözcükler, çerçevenin kenar kalınlığını aramada bulmayı sağlar.
settings-frame-thickness-keywords = çerçeve kalınlığı, kenar, border

# Nerede: Windows sayfasındaki Yerleşim bölümünde, windows.border ayarının (Kenarlık) arama sözcükleri. Anlam: Bu sözcükler, etkin pencerenin kenarlık ve çerçeve çizgisi kalınlığını aramada bulmayı sağlar.
settings-windows-border-keywords = border, çerçeve, çizgi, etkin pencere, kenarlık kalınlığı

# Nerede: Windows sayfasındaki Görünüm bölümünde, windows.blur ayarının (Saydam pencerelerin ardı bulanık) arama sözcükleri. Anlam: Bu sözcükler, saydam pencerelerin arkasındaki bulanıklığı, yani cam etkisini aramada bulmayı sağlar.
settings-windows-blur-keywords = blur, bulanıklık, saydam, cam, arka plan

# Nerede: Windows sayfasındaki Görünüm bölümünde, windows.corners ayarının (Köşe, gölge ve hareket) arama sözcükleri. Anlam: Bu sözcükler, pencere köşelerinin yuvarlaklığını, gölgesini ve animasyonunu aramada bulmayı sağlar.
settings-windows-corners-keywords = köşe, yuvarlak, radius, rounding, gölge, animasyon, temayla ortak

# Nerede: Windows sayfasındaki Görünüm bölümünde, windows.motion ayarının (Pencere hareketleri) arama sözcükleri. Anlam: Bu sözcükler, pencerelerin animasyon ve geçiş hareketlerini aramada bulmayı sağlar.
settings-windows-motion-keywords = animasyon, hareket, geçiş, workspace

# Nerede: Windows sayfasındaki Gelişmiş bölümünde, compositor.ensure_source ayarının (Masaüstü ayarlarına Nuxut'u ekle) arama sözcükleri. Anlam: Bu sözcükler, Nuxut'un Hyprland yapılandırma dosyasına dahil edilmesini aramada bulmayı sağlar.
settings-compositor-ensure-source-keywords = hyprland, source, hyprland.conf, hyprland.lua, dahil et, bağlan

# Nerede: Wallpaper sayfasında, bölümsüz duran wallpaper ayarının (Duvar kağıdı) arama sözcükleri. Anlam: Bu sözcükler, duvar kağıdı olarak kullanılacak arka plan resmini aramada bulmayı sağlar.
settings-wallpaper-keywords = arka plan, resim, görsel, gif, webp

# Nerede: Wallpaper sayfasındaki Renkler bölümünde, palette.source ayarının (Renkleri duvar kağıdından al) arama sözcükleri. Anlam: Bu sözcükler, renk şemasının duvar kağıdından otomatik alınmasını aramada bulmayı sağlar.
settings-palette-source-keywords = matugen, material you, otomatik renk, tema, renk şeması

# Nerede: Bar sayfasındaki Seçili widget bölümünde, bar.widget.clock.format ayarının alt yazısı burada. Anlam: Saat biçimi boş bırakılırsa 12/24 saat tercihine uyulacağını, strftime kalıbı girilirse onun kullanılacağını anlatır.
settings-bar-widget-clock-format-description = Boş bırakılırsa 12/24 saat tercihine uyar; bir biçim kalıbı yazılırsa o kullanılır.

# Nerede: Varsayılan uygulamalar sayfasındaki Durum bölümünde, defaults.note ayarının satır başlığı burada. Anlam: Varsayılan uygulamaların durumunu gösteren satırın başlığıdır.
settings-defaults-note-label = Varsayılan uygulamaların durumu

# Nerede: Varsayılan uygulamalar sayfasındaki Durum bölümünde, defaults.note ayarının arama sözcükleri burada. Anlam: Bu ayarı ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-note-keywords = varsayılan uygulama, default app, kabuk, masaüstü

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.browser ayarının satır başlığı burada. Anlam: Varsayılan tarayıcıyı seçen satırın başlığıdır.
settings-defaults-browser-label = Tarayıcı

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.browser ayarının alt yazısı burada. Anlam: Bu ayarın web sayfalarını ve bağlantıları açan uygulamayı seçtiğini anlatır.
settings-defaults-browser-description = Web sayfaları ve bağlantılar

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.browser ayarının arama sözcükleri burada. Anlam: Tarayıcı ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-browser-keywords = tarayıcı, browser, web, internet, bağlantı, link, http, html, firefox, chrome, chromium, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.email ayarının satır başlığı burada. Anlam: Varsayılan e-posta istemcisini seçen satırın başlığıdır.
settings-defaults-email-label = E-posta

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.email ayarının alt yazısı burada. Anlam: Bu ayarın e-posta bağlantılarını (mailto) açan uygulamayı seçtiğini anlatır.
settings-defaults-email-description = E-posta bağlantıları (mailto)

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.email ayarının arama sözcükleri burada. Anlam: E-posta ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-email-keywords = e-posta, eposta, posta, mail, email, mailto, thunderbird, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.file_manager ayarının satır başlığı burada. Anlam: Varsayılan dosya yöneticisini seçen satırın başlığıdır.
settings-defaults-file-manager-label = Dosya yöneticisi

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.file_manager ayarının alt yazısı burada. Anlam: Bu ayarın klasörleri açan uygulamayı seçtiğini anlatır.
settings-defaults-file-manager-description = Klasörleri açar

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.file_manager ayarının arama sözcükleri burada. Anlam: Dosya yöneticisi ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-file-manager-keywords = dosya yöneticisi, dosyalar, klasör, dizin, file manager, folder, nautilus, thunar, dolphin, nemo, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.terminal ayarının satır başlığı burada. Anlam: Varsayılan terminali seçen satırın başlığıdır.
settings-defaults-terminal-label = Terminal

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.terminal ayarının alt yazısı burada. Anlam: Bu ayarın terminalde açılan uygulamalar için kullanılacağını anlatır.
settings-defaults-terminal-description = Terminalde açılan uygulamalar için

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.terminal ayarının arama sözcükleri burada. Anlam: Terminal ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-terminal-keywords = terminal, konsol, console, komut satırı, xdg-terminal-exec, kitty, foot, alacritty, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.text_editor ayarının satır başlığı burada. Anlam: Varsayılan metin düzenleyiciyi seçen satırın başlığıdır.
settings-defaults-text-editor-label = Metin düzenleyici

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.text_editor ayarının alt yazısı burada. Anlam: Bu ayarın düz metin dosyalarını açan uygulamayı seçtiğini anlatır.
settings-defaults-text-editor-description = Düz metin dosyaları

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.text_editor ayarının arama sözcükleri burada. Anlam: Metin düzenleyici ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-text-editor-keywords = metin, düzenleyici, editör, editor, text editor, txt, not defteri, gedit, kate, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.image ayarının satır başlığı. Anlam: Bu metin, varsayılan resim görüntüleyiciyi seçen satırın başlığıdır.
settings-defaults-image-label = Resim görüntüleyici

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.image ayarının alt yazısı. Anlam: Bu metin, hangi dosya türlerinin resim görüntüleyicide açıldığını kısaca açıklar.
settings-defaults-image-description = PNG, JPEG ve diğer resimler

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.image ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken resim görüntüleyici satırını bulmayı sağlar.
settings-defaults-image-keywords = resim, görüntü, fotoğraf, image, photo, png, jpg, jpeg, görüntüleyici, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.video ayarının satır başlığı. Anlam: Bu metin, varsayılan video oynatıcıyı seçen satırın başlığıdır.
settings-defaults-video-label = Video oynatıcı

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.video ayarının alt yazısı. Anlam: Bu metin, hangi dosya türlerinin video oynatıcıda açıldığını kısaca açıklar.
settings-defaults-video-description = MP4, MKV, WebM ve diğer videolar

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.video ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken video oynatıcı satırını bulmayı sağlar.
settings-defaults-video-keywords = video, film, oynatıcı, player, mp4, mkv, webm, mpv, vlc, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.music ayarının satır başlığı. Anlam: Bu metin, varsayılan müzik çaları seçen satırın başlığıdır.
settings-defaults-music-label = Müzik çalar

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.music ayarının alt yazısı. Anlam: Bu metin, hangi dosya türlerinin müzik çalarda açıldığını kısaca açıklar.
settings-defaults-music-description = MP3, FLAC, OGG ve diğer ses dosyaları

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.music ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken müzik çalar satırını bulmayı sağlar.
settings-defaults-music-keywords = müzik, ses dosyası, şarkı, music, audio, mp3, flac, çalar, varsayılan, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.pdf ayarının satır başlığı. Anlam: Bu metin, varsayılan PDF görüntüleyiciyi seçen satırın başlığıdır.
settings-defaults-pdf-label = PDF görüntüleyici

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.pdf ayarının alt yazısı. Anlam: Bu metin, PDF belgelerinin varsayılan uygulamayla açıldığını kısaca açıklar.
settings-defaults-pdf-description = PDF belgeleri

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.pdf ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken PDF görüntüleyici satırını bulmayı sağlar.
settings-defaults-pdf-keywords = pdf, belge, doküman, document, okuyucu, reader, evince, okular, zathura, varsayılan, default app

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.note ayarının satır başlığı. Anlam: Bu metin, sistemdeki tarih ve saat durumunu gösteren satırın başlığıdır.
settings-datetime-note-label = Tarih ve saat durumu

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.note ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken tarih ve saat durumu satırını bulmayı sağlar.
settings-datetime-note-keywords = timedated, rtc, donanım saati

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.ntp ayarının satır başlığı. Anlam: Bu metin, saatin internet üzerinden otomatik eşitlenmesini açan satırın başlığıdır.
settings-datetime-ntp-label = Otomatik saat

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.ntp ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken otomatik saat satırını bulmayı sağlar.
settings-datetime-ntp-keywords = ntp, ağ saati, eşitle, senkron, internet saati, automatic time

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.timezone ayarının satır başlığı. Anlam: Bu metin, sistem saat dilimini seçen satırın başlığıdır.
settings-datetime-timezone-label = Saat dilimi

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.timezone ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken saat dilimi satırını bulmayı sağlar.
settings-datetime-timezone-keywords = saat dilimi, zaman dilimi, timezone, utc, gmt, şehir

# Nerede: Tarih ve saat sayfasında, Saat bölümündeki datetime.manual ayarının satır başlığı. Anlam: Tarih ve saatin elle ayarlanabileceğini belirtir.
settings-datetime-manual-label = Elle tarih ve saat

# Nerede: Tarih ve saat sayfasında, Saat bölümündeki datetime.manual ayarının arama sözcükleri. Anlam: Kullanıcı bu sözcüklerle elle tarih ve saat ayarını bulur.
settings-datetime-manual-keywords = tarih, saat, elle, manual, ayarla

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki locale.hour_cycle ayarının satır başlığı. Anlam: Saatin hangi biçimde gösterileceğini seçtirir.
settings-locale-hour-cycle-label = Saat biçimi

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki locale.hour_cycle ayarının alt yazısı. Anlam: Saatin 24 ya da 12 saatlik biçimde gösterilebileceğini anlatır.
settings-locale-hour-cycle-description = Saatin 24 ya da 12 saatlik gösterimi

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki locale.hour_cycle ayarının arama sözcükleri. Anlam: Kullanıcı bu sözcüklerle saat biçimi ayarını bulur.
settings-locale-hour-cycle-keywords = 24 saat, 12 saat, öö, ös, am, pm, saat biçimi, hour

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki datetime.clock_format_note ayarının satır başlığı. Anlam: Bar saatine özel biçim ayarı verilebileceğini belirtir.
settings-datetime-clock-format-note-label = Bar saati biçimi

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki datetime.clock_format_note ayarının alt yazısı. Anlam: Bar saatinin şu anda özel bir biçim kullandığını bildirir.
settings-datetime-clock-format-note-description = Bar saati özel biçim kullanıyor

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki datetime.clock_format_note ayarının arama sözcükleri. Anlam: Kullanıcı bu sözcüklerle bar saati biçimi ayarını bulur.
settings-datetime-clock-format-note-keywords = saat biçimi, bar saati, özel biçim, temizle

# Nerede: Kenar çubuğunda görünen sayfa adı. Anlam: Tarih ve saat ayarlarının bulunduğu sayfayı açar.
settings-page-datetime = Tarih ve saat

# Nerede: Kenar çubuğunda görünen sayfa adı. Anlam: Varsayılan uygulamaların seçildiği sayfayı açar.
settings-page-default-apps = Varsayılan uygulamalar

# Nerede: Tarih ve saat sayfasındaki bölüm adı. Anlam: Otomatik saat, saat dilimi ve elle tarih ve saat ayarlarını içeren bölümü başlıklandırır.
settings-section-clock = Saat

# Nerede: Tarih ve saat sayfasındaki bölüm adı. Anlam: Saat biçimi ve bar saati biçimi ayarlarını içeren bölümü başlıklandırır.
settings-section-format = Biçim

# Nerede: Varsayılan uygulamalar sayfasındaki bölüm adı. Anlam: Tarayıcı ve e-posta uygulamalarının seçildiği bölümü başlıklandırır.
settings-section-internet = İnternet

# Nerede: Varsayılan uygulamalar sayfasındaki bölüm adı. Anlam: Dosya yöneticisi, terminal ve metin düzenleyici seçimlerinin yer aldığı bölümü başlıklandırır.
settings-section-system = Sistem

# Nerede: Varsayılan uygulamalar sayfasındaki bölüm adı. Anlam: Resim, video, müzik ve PDF uygulamalarının seçildiği bölümü başlıklandırır.
settings-section-media-documents = Medya ve belgeler

# Nerede: Tarih ve saat sayfasında, Saat biçimi seçeneğinin Otomatik (auto) değerinin etiketi. Anlam: Saat biçiminin dile göre otomatik belirleneceğini belirtir.
settings-choice-hour-cycle-auto = Otomatik

# Nerede: Tarih ve saat sayfasında, Saat biçimi seçeneğinin 24 saat (h23) değerinin etiketi. Anlam: Saatin 24 saatlik biçimde gösterileceğini seçer.
settings-choice-hour-cycle-h23 = 24 saat

# Nerede: Tarih ve saat sayfasında, Saat biçimi seçeneğinin 12 saat (h12) değerinin etiketi. Anlam: Saatin ÖÖ/ÖS eklenerek 12 saatlik biçimde gösterileceğini seçer.
settings-choice-hour-cycle-h12 = 12 saat

# Nerede: Yazı sayfasındaki kartın sonundaki açılır satırın başlığıdır; içinde rol başına boyut oranları, ikon yazı tipi ve ikon boyutu ayarları bulunur. Anlam: Bu metin, rol başına boyut oranlarını ve ikon yazı tipini ayarlayan bölümün başlığıdır.
settings-detail-typography-label = Boyut oranları ve ikon yazı tipi

# Nerede: Yazı sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde rol başına boyutun, ikonların yazı tipini ve boyutunun ayarlandığını özetler.
settings-detail-typography-description = Rol başına boyut, ikonların yazı tipi ve boyutu

# Nerede: Hareket sayfasındaki kartın sonundaki açılır satırın başlığıdır; içinde hareket türü ve öğe gruplarının yay ya da eğri ayarları bulunur. Anlam: Bu metin, hareket ayarlarını tek tek yapmayı sağlayan bölümün başlığıdır.
settings-detail-motion-label = Hareketi tek tek ayarla

# Nerede: Hareket sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde her hareket türü ve öğe grubu için yayın ya da eğrinin ayarlandığını özetler.
settings-detail-motion-description = Hareket türü ve her öğe grubunun yayı ya da eğrisi

# Nerede: Pencereler sayfasındaki kartın sonundaki açılır satırın başlığıdır; içinde Nuxut'un pencerelere ve masaüstü ayar dosyasına uyguladığı ayarlar bulunur. Anlam: Bu metin, Nuxut'un masaüstüyle birlikte çalışma biçimini ayarlayan bölümün başlığıdır.
settings-detail-compositor-label = Masaüstüyle ortaklık

# Nerede: Pencereler sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde Nuxut'un pencerelere ve masaüstü ayar dosyasına uyguladığı ayarların görüldüğünü özetler.
settings-detail-compositor-description = Nuxut'un pencerelere ve masaüstü ayar dosyasına uyguladıkları

# Nerede: Görünüm sayfasındaki Renkler kartının sonundaki açılır satırın başlığıdır; vurgu dışındaki on renk rolü burada ayarlanır. Anlam: Bu metin, renkleri tek tek ayarlamayı sağlayan bölümün başlığıdır.
settings-detail-colors-label = Renkleri tek tek ayarla

# Nerede: Görünüm sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde yüzey, yazı, hata ve çizgi renklerinin ayarlandığını özetler.
settings-detail-colors-description = Yüzey, yazı, hata ve çizgi renkleri

# Nerede: Görünüm sayfasındaki Köşeler ve şekil bölümünde, Kenar çizgileri satırının alt yazısı yerine görünen durum notu. Anlam: Yüksek karşıtlık açıkken kenar çizgisinin "Yok" seçili olsa bile ince olarak çizildiğini bildirir.
settings-shape-outlines-high-contrast-note = Yüksek karşıtlık açık: ince çiziliyor

# Nerede: Görünüm sayfasındaki Renkler bölümünde, colors.contrast ayarının satır başlığıdır. Anlam: Bu metin, yüksek karşıtlık seçeneğinin adını gösterir.
settings-colors-contrast-label = Yüksek karşıtlık

# Nerede: Görünüm sayfasındaki Renkler bölümünde, colors.contrast ayarının alt yazısıdır. Anlam: Bu metin, yüksek karşıtlık açıldığında metin ve kenarların daha belirgin olacağını açıklar.
settings-colors-contrast-description = Metin ve kenarlar daha belirgin olur

# Nerede: Görünüm sayfasındaki Renkler bölümünde, colors.contrast ayarının arama sözcükleridir. Anlam: Bu sözcükler, yüksek karşıtlık ayarının aramada bulunmasını sağlar.
settings-colors-contrast-keywords = karşıtlık, kontrast, erişilebilirlik, görme, okunabilirlik, high contrast

# Nerede: Görünüm sayfasındaki Renkler bölümünde, palette.contrast ayarının satır başlığıdır. Anlam: Bu metin, palet kontrastı ayarının adını gösterir.
settings-palette-contrast-label = Palet kontrastı

# Nerede: Görünüm sayfasındaki Renkler bölümünde, palette.contrast ayarının alt yazısıdır. Anlam: Bu metin, duvar kağıdından gelen renklerin birbirinden ne kadar farklı olacağını açıklar; solda az, sağda çok fark olur.
settings-palette-contrast-description = Duvar kağıdından gelen renklerin birbirinden farkı; solda az, sağda çok

# Nerede: Görünüm sayfasındaki Köşeler ve şekil bölümünde, shape.outlines ayarının alt yazısıdır. Anlam: Bu metin, bileşen sınırlarını gösteren çizgileri açıklar; yüksek karşıtlık açıkken bu çizgiler en az ince biçimde çizilir.
settings-shape-outlines-description = Bileşen sınırları; yüksek karşıtlık açıkken en az ince çizilir

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.enabled ayarının satır başlığıdır. Anlam: Bu metin, gece ışığı özelliğini açıp kapatan satırın başlığını gösterir.
settings-night-light-enabled-label = Gece ışığını kullan

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.enabled ayarının alt yazısıdır. Anlam: Bu metin, gece ışığının akşamları ekran renklerini sıcaklaştırdığını açıklar.
settings-night-light-enabled-description = Akşamları ekran renklerini sıcaklaştırır

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.enabled ayarının arama sözcükleridir. Anlam: Bu sözcükler, gece ışığı ayarının aramada bulunmasını sağlar.
settings-night-light-enabled-keywords = gece, gece ışığı, mavi ışık, sıcaklık, göz, night light, hyprsunset, renk sıcaklığı

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.temperature ayarının satır başlığıdır. Anlam: Bu metin, gece sıcaklığı ayarının adını gösterir.
settings-night-light-temperature-label = Gece sıcaklığı

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.temperature ayarının alt yazısıdır. Anlam: Bu metin, düşük sıcaklık değerinin daha turuncu bir renk verdiğini açıklar.
settings-night-light-temperature-description = Düşük değer daha turuncu

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.temperature ayarının arama sözcükleridir. Anlam: Bu sözcükler, gece sıcaklığı ayarının aramada bulunmasını sağlar.
settings-night-light-temperature-keywords = gece, sıcaklık, kelvin, renk sıcaklığı, turuncu

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.schedule ayarının satır başlığıdır. Anlam: Bu metin, gece ışığı zamanlaması satırının başlığını gösterir.
settings-night-light-schedule-label = Zamanlama

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.schedule ayarının arama sözcükleridir. Anlam: Bu sözcükler, zamanlama ayarının aramada bulunmasını sağlar.
settings-night-light-schedule-keywords = gece, zamanlama, gün batımı, gün doğumu, saat

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.from ayarının satır başlığıdır. Anlam: Bu metin, gece ışığının başlangıç saati satırının başlığını gösterir.
settings-night-light-from-label = Başlangıç saati

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.from ayarının alt yazısı. Anlam: Bitiş saati daha erken bir değere ayarlandığında aralığın gece yarısını aştığını anlatır.
settings-night-light-from-description = Bitiş daha erkense gece yarısını aşar

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.from ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-from-keywords = gece, başlangıç, saat, akşam

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.to ayarının satır başlığı. Anlam: Gece ışığının kapanacağı bitiş saatinin seçildiği satır.
settings-night-light-to-label = Bitiş saati

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.to ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-to-keywords = gece, bitiş, saat, sabah

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.location ayarının satır başlığı. Anlam: Gün batımı ve gün doğumu saatlerinin hesaplanacağı konumun girildiği satır.
settings-night-light-location-label = Konum

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.location ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-location-keywords = gece, konum, enlem, boylam, gün batımı, gün doğumu, latitude, longitude

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.transition ayarının satır başlığı. Anlam: Renk değişiminin kaç dakika süreceğinin seçildiği satır.
settings-night-light-transition-label = Geçiş süresi

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.transition ayarının alt yazısı. Anlam: Renklerin bu süre içinde yavaşça değiştiğini anlatır.
settings-night-light-transition-description = Renk bu sürede yavaşça değişir

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.transition ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-transition-keywords = gece, geçiş, yumuşak, süre, dakika

# Nerede: Görünüm sayfasında, Renkler bölümündeki palette.contrast (Palet kontrastı) ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-palette-contrast-keywords = kontrast, erişilebilirlik, okunabilirlik

# Nerede: Ekranlar sayfasındaki bölüm (kart) adı. Anlam: Gece ışığı ayarlarını içeren bölümün başlığı.
settings-section-night-light = Gece ışığı

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki Zamanlama seçeneğinin bir değeri (always). Anlam: Gece ışığının sürekli açık kalacağını belirten seçim.
settings-choice-night-schedule-always = Her zaman

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki Zamanlama seçeneğinin bir değeri (manual). Anlam: Gece ışığının başlangıç ve bitiş saatleri arasında açık kalacağını belirten seçim.
settings-choice-night-schedule-manual = Belirli saatlerde

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki Zamanlama seçeneğinin bir değeri (sun). Anlam: Gece ışığının gün batımından gün doğumuna kadar açık kalacağını belirten seçim.
settings-choice-night-schedule-sun = Gün batımından gün doğumuna

# ---- Kullanıcı hesabı (t118) ----

# Nerede: Kenar çubuğunda görünen sayfa adı (Sistem grubu). Anlam: Bu bilgisayardaki kendi kullanıcı hesabının (görünen ad, profil resmi) ayarlandığı sayfa; çevrimiçi takvim hesaplarını içeren "Hesaplar" sayfasından farklıdır.
settings-page-user = Kullanıcı hesabı

# Nerede: Kullanıcı hesabı sayfasındaki bölüm başlığı. Anlam: Profil resmi ve görünen ad satırlarını toplayan bölüm.
settings-section-profile = Profil

# Nerede: Kullanıcı hesabı sayfasında, user.note satırının başlığı (aramada görünür). Anlam: Hesap hizmetinin (AccountsService) çalışıp çalışmadığını bildiren üst not.
settings-user-note-label = Hesap durumu

# Nerede: Kullanıcı hesabı sayfasında, user.note satırı için arama sözcükleri. Anlam: Hesap hizmeti notunu aramada bulmayı sağlar.
settings-user-note-keywords = accountsservice, hesap hizmeti

# Nerede: Kullanıcı hesabı sayfasının Profil bölümünde, user.avatar satırının başlığı (aramada görünür). Anlam: Kullanıcının profil resmi (kilit ekranında ve giriş ekranında görünen resim).
settings-user-avatar-label = Profil resmi

# Nerede: Kullanıcı hesabı sayfasında, user.avatar satırı için arama sözcükleri. Anlam: Profil resmi satırını aramada bulmayı sağlar; satır kullanıcı adını ve hesap türünü de gösterir.
settings-user-avatar-keywords = avatar, profil resmi, fotoğraf, resim, face, kullanıcı adı, hesap türü, yönetici

# Nerede: Kullanıcı hesabı sayfasının Profil bölümünde, user.real_name satırının başlığı. Anlam: Kullanıcının tam adı (ad soyad); kullanıcı adı (giriş adı) değil.
settings-user-real-name-label = Görünen ad

# Nerede: Kullanıcı hesabı sayfasında, user.real_name satırı için arama sözcükleri. Anlam: Görünen ad satırını aramada bulmayı sağlar.
settings-user-real-name-keywords = ad, isim, ad soyad, gerçek ad, real name, full name

# Nerede: Kullanıcı hesabı sayfasının üstündeki uyarı notu. Anlam: Sistemde AccountsService hizmeti yok ya da çalışmıyor; ad ve profil resmi değiştirilemez.
settings-user-missing = Hesap hizmeti (AccountsService) bulunamadı; ad ve profil resmi değiştirilemez.

# Nerede: Kullanıcı hesabı sayfasının üstündeki uyarı notu. Anlam: Hesap hizmeti var ama kullanıcı bilgileri okunamadı.
settings-user-read-failed = Hesap bilgileri okunamadı.

# Nerede: Kullanıcı hesabı sayfasında bir satırın alt metni, değişiklik sırasında hizmet kaybolduysa. Anlam: Hesap hizmeti yok, değişiklik yapılamadı.
settings-user-missing-short = Hesap hizmeti yok

# Nerede: Kullanıcı hesabı sayfasında bir satırın alt metni (kırmızı). Anlam: Görünen ad ya da profil resmi değiştirilemedi.
settings-user-failed = Değiştirilemedi

# Nerede: Kullanıcı hesabı sayfasında bir satırın alt metni, değişiklik sürerken. Anlam: Sistem yetki istedi; kullanıcının parola diyaloğunu onaylaması bekleniyor.
settings-user-waiting = Yetki bekleniyor…

# Nerede: Profil resmi satırındaki düğme. Anlam: Dosya seçiciyi açıp yeni profil resmi seçtirir.
# [max: 14]
settings-user-avatar-change = Değiştir…

# Nerede: Profil resmi için açılan dosya seçici penceresinin başlığı. Anlam: Kullanıcıdan profil resmi olacak bir görsel seçmesini ister.
settings-user-avatar-dialog-title = Profil resmi seçin

# Nerede: Profil resmi dosya seçicisindeki dosya türü filtresinin adı. Anlam: Yalnızca görsel dosyaları (PNG, JPEG, WebP, GIF) gösterilir.
settings-user-avatar-filter = Görseller

# Nerede: Profil resmi satırının alt metni (kırmızı). Anlam: Seçilen dosya açılamadı; bozuk, çok büyük (8 MB üstü) ya da desteklenmeyen biçimde.
settings-user-avatar-bad-image = Görsel açılamadı (bozuk, çok büyük ya da desteklenmeyen biçim)

# Nerede: Profil resmi satırının alt metni (kırmızı). Anlam: Sistemin dosya seçici hizmeti (xdg-desktop-portal) açılamadı.
settings-user-avatar-portal = Dosya seçici açılamadı

# Nerede: Profil resmi satırının alt metni (kırmızı). Anlam: Profil resmi hesapta değişti, ama kilit ekranının okuduğu ev dizinindeki ~/.face dosyası yazılamadı.
settings-user-face-failed = Profil resmi değişti, ancak kilit ekranı resmi (~/.face) yazılamadı

# Nerede: Görünen ad satırının alt metni (kırmızı). Anlam: Girilen ad kabul edilmedi: iki nokta üst üste ya da virgül içeriyor veya 64 karakterden uzun.
settings-user-name-invalid = Ad iki nokta üst üste ya da virgül içeremez, en çok 64 karakter olabilir

# Nerede: Görünen ad alanı boşken içinde görünen soluk ipucu. Anlam: Buraya kullanıcının adı soyadı yazılır.
settings-user-name-placeholder = Ad soyad

# Nerede: Profil resmi satırında, kullanıcı adının yanında gösterilen hesap türü. Anlam: Bu hesap yönetici (sudo/wheel) yetkili.
settings-user-type-admin = Yönetici

# Nerede: Profil resmi satırında, kullanıcı adının yanında gösterilen hesap türü. Anlam: Bu hesap yönetici yetkisi olmayan sıradan bir hesap.
settings-user-type-standard = Standart hesap

# Nerede: Profil resmi satırında, adın altındaki satır. Anlam: $user giriş için kullanılan kullanıcı adı (ör. "ayse"), $kind hesap türü ("Yönetici" ya da "Standart hesap"); aralarında orta nokta.
settings-user-identity = { $user } · { $kind }

# ---- Bağlantılar: Wi-Fi ve VPN (t114, salt okuma; docs/design/network-pages.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Kablosuz ağ sayfasının etiketi.
# [max: 14]
settings-page-wifi = Wi-Fi

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: VPN bağlantıları sayfasının etiketi.
# [max: 14]
settings-page-vpn = VPN

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Wi-Fi ve VPN (ileride Bluetooth) sayfalarını toplayan grup.
# [max: 12]
settings-nav-group-connections = Bağlantılar

# Nerede: Wi-Fi sayfasında kart başlığı. Anlam: Taramada görünen kablosuz ağların listesi.
settings-section-wifi-visible = Görünen ağlar

# Nerede: Wi-Fi sayfasında kart başlığı. Anlam: Daha önce bağlanılmış, ayarları saklanan kablosuz ağlar.
settings-section-wifi-saved = Kayıtlı ağlar

# Nerede: VPN sayfasında kart başlığı. Anlam: Kayıtlı VPN ve WireGuard bağlantılarının listesi.
settings-section-vpn-connections = VPN bağlantıları

# Nerede: Wi-Fi sayfasında, wifi.note satırının başlığı (aramada görünür). Anlam: NetworkManager hizmetinin çalışıp çalışmadığını bildiren üst not.
settings-wifi-note-label = Ağ durumu

# Nerede: Wi-Fi sayfasında, wifi.note satırı için arama sözcükleri. Anlam: Ağ hizmeti notunu aramada bulmayı sağlar.
settings-wifi-note-keywords = networkmanager, ağ hizmeti

# Nerede: Wi-Fi sayfasının Wi-Fi kartında, wifi.enabled satırının başlığı. Anlam: Kablosuz ağın açık mı kapalı mı olduğu.
settings-wifi-enabled-label = Wi-Fi

# Nerede: Wi-Fi sayfasında, wifi.enabled satırı için arama sözcükleri. Anlam: Wi-Fi aç/kapa satırını aramada bulmayı sağlar.
settings-wifi-enabled-keywords = kablosuz, wireless, wlan, aç, kapat, uçak modu

# Nerede: Wi-Fi sayfasının Wi-Fi kartında, wifi.current satırının başlığı (aramada görünür). Anlam: Şu an bağlı olunan kablosuz ağ.
settings-wifi-current-label = Bağlı ağ

# Nerede: Wi-Fi sayfasında, wifi.current satırı için arama sözcükleri. Anlam: Bağlı ağ satırını aramada bulmayı sağlar.
settings-wifi-current-keywords = bağlı, ssid, sinyal, internet, bağlantıyı kes, ip adresi, dns

# Nerede: Wi-Fi sayfasında, wifi.networks satırının başlığı (aramada görünür). Anlam: Taramada görünen kablosuz ağların listesi.
settings-wifi-networks-label = Görünen ağlar

# Nerede: Wi-Fi sayfasında, wifi.networks satırı için arama sözcükleri. Anlam: Ağ listesini aramada bulmayı sağlar.
settings-wifi-networks-keywords = ağlar, tarama, erişim noktası, ssid, sinyal, bağlan, şifre

# Nerede: Wi-Fi sayfasında, wifi.saved satırının başlığı (aramada görünür). Anlam: Daha önce bağlanılmış, ayarları saklanan kablosuz ağlar.
settings-wifi-saved-label = Kayıtlı ağlar

# Nerede: Wi-Fi sayfasında, wifi.saved satırı için arama sözcükleri. Anlam: Kayıtlı ağlar listesini aramada bulmayı sağlar.
settings-wifi-saved-keywords = kayıtlı, bilinen ağlar, unut, otomatik bağlan, ölçülü, metered

# Nerede: VPN sayfasında, vpn.note satırının başlığı (aramada görünür). Anlam: NetworkManager hizmetinin çalışıp çalışmadığını bildiren üst not.
settings-vpn-note-label = VPN durumu

# Nerede: VPN sayfasında, vpn.note satırı için arama sözcükleri. Anlam: Ağ hizmeti notunu aramada bulmayı sağlar.
settings-vpn-note-keywords = networkmanager, ağ hizmeti

# Nerede: VPN sayfasında, vpn.connections satırının başlığı (aramada görünür). Anlam: Kayıtlı VPN ve WireGuard bağlantılarının listesi.
settings-vpn-connections-label = VPN bağlantıları

# Nerede: VPN sayfasında, vpn.connections satırı için arama sözcükleri. Anlam: VPN listesini aramada bulmayı sağlar.
settings-vpn-connections-keywords = vpn, wireguard, openvpn, tünel, sil

# Nerede: Wi-Fi ve VPN sayfalarının üstündeki uyarı notu. Anlam: Sistemde ağ hizmeti (NetworkManager) yok ya da çalışmıyor.
settings-network-missing = NetworkManager çalışmıyor; ağ bilgileri gösterilemiyor.

# Nerede: Wi-Fi ve VPN sayfalarının üstündeki uyarı notu. Anlam: Ağ hizmeti istemcisi başlatılamadı.
settings-network-failed = Ağ hizmetine bağlanılamadı.

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: $detail NetworkManager'ın (çevrilmemiş) hata metni.
settings-network-error-call-failed = İşlem başarısız: { $detail }

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Ağ hizmeti yok.
settings-network-error-unavailable = NetworkManager çalışmıyor

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: İstenen kayıtlı bağlantı artık yok.
settings-network-error-connection-unknown = Bağlantı bulunamadı

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Kapatılmak istenen bağlantı zaten kapalı.
settings-network-error-connection-inactive = Bağlantı etkin değil

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Bağlanılmak istenen kablosuz ağ taramada artık yok.
settings-network-error-ap-unknown = Ağ artık görünmüyor

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Kullanıcı adı/sertifika isteyen kurumsal ağlara buradan bağlanılamıyor.
settings-network-error-enterprise = Kurumsal (802.1X) ağlar henüz desteklenmiyor

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Girilen şifre bu güvenlik türünün kurallarına uymuyor.
settings-network-error-invalid-password = Şifre bu ağ için geçersiz

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Bilgisayarda kablosuz ağ aygıtı bulunamadı.
settings-network-error-no-device = Wi-Fi aygıtı yok

# Nerede: Wi-Fi sayfasının üst notu, gizli ağ adı geçersizken. Anlam: Ağ adı (SSID) 1–32 bayt olmalı.
settings-network-error-invalid-ssid = Ağ adı geçersiz

# Nerede: Wi-Fi sayfasının üst notu, ayrıntısı istenen ağ aygıtı artık yokken. Anlam: Aygıt bulunamadı (çıkarılmış olabilir).
settings-network-error-device-unknown = Ağ aygıtı bulunamadı

# Nerede: Wi-Fi/VPN sayfası, bir ağ işlemi izin verilmediği için reddedilince (normalde gösterilmez). Anlam: Sistem bu işleme izin vermedi (polkit).
settings-network-error-denied = Bu işlem için izniniz yok

# Nerede: Wi-Fi sayfasında ağ satırının hata metni, kayıtlı ağın şifresi değiştirilmek istenince. Anlam: Bu ağın güvenlik türünde (WEP, kurumsal, açık) şifre buradan değiştirilemiyor.
settings-network-error-unsupported = Bu ağ türünde şifre buradan değiştirilemiyor

# Nerede: Wi-Fi sayfasında ağ satırının hata metni, kayıtlı ağın şifresi değiştirilmek istenince. Anlam: Ağ "şifreyi her seferinde sor" olarak ayarlı; şifre kaydedilmiyor, bağlanırken sorulur.
settings-network-error-secret-not-saved = Bu ağın şifresi kaydedilmiyor; bağlanırken her seferinde sorulur

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Şifresiz ağ.
settings-network-security-open = Açık

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Şifre istemeyen ama trafiği şifreleyen ağ (OWE).
settings-network-security-owe = Açık (şifreli)

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Eski WEP güvenliği.
settings-network-security-wep = WEP

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: WPA/WPA2 kişisel (şifreli) güvenlik.
settings-network-security-wpa = WPA/WPA2

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: WPA3 kişisel güvenlik.
settings-network-security-sae = WPA3

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Kullanıcı adı ya da sertifika isteyen kurumsal ağ.
settings-network-security-enterprise = Kurumsal (802.1X)

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Ağın frekans bandı (ondalık ayırıcı dile göre).
settings-network-band-2g = 2,4 GHz

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Ağın frekans bandı.
settings-network-band-5g = 5 GHz

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Ağın frekans bandı.
settings-network-band-6g = 6 GHz

# Nerede: Wi-Fi ve VPN listelerinde satırın alt metni. Anlam: Bağlantı şu an kuruluyor.
settings-network-connecting = Bağlanıyor…

# Nerede: Wi-Fi satırının alt metni. Anlam: Bilgisayarda kablosuz ağ aygıtı yok.
settings-wifi-status-no-device = Wi-Fi aygıtı bulunamadı

# Nerede: Wi-Fi satırının alt metni. Anlam: Wi-Fi fiziksel anahtar ya da uçak modu tuşuyla kapatılmış.
settings-wifi-status-hardware-off = Donanım anahtarıyla kapatılmış

# Nerede: Wi-Fi satırının alt metni. Anlam: Wi-Fi kapalı.
settings-wifi-status-off = Kapalı

# Nerede: Wi-Fi satırının alt metni. Anlam: Wi-Fi açık.
settings-wifi-status-on = Açık

# Nerede: Wi-Fi sayfasında bağlı ağın ya da kayıtlı ağın alt metni. Anlam: Bu ağa şu an bağlı.
settings-wifi-connected = Bağlı

# Nerede: Wi-Fi sayfasında bağlı ağın alt metni. Anlam: Ağ tarayıcıda giriş sayfası (esir portal) istiyor.
settings-wifi-connected-portal = Bağlı, ağa giriş gerekiyor

# Nerede: Wi-Fi sayfasında bağlı ağın alt metni. Anlam: Ağa bağlı ama internete erişim yok.
settings-wifi-connected-limited = Bağlı, internet yok

# Nerede: Wi-Fi sayfasında Görünen ağlar kartının boş durumu. Anlam: Wi-Fi kapalı olduğu için liste boş.
settings-wifi-visible-off = Wi-Fi kapalıyken ağlar görünmez.

# Nerede: Wi-Fi sayfasında Görünen ağlar kartının boş durumu. Anlam: Taramada hiç ağ bulunamadı.
settings-wifi-visible-empty = Görünen ağ yok.

# Nerede: Wi-Fi sayfasında görünen ağ satırının sağındaki kısa etiket. Anlam: Bu ağın ayarları kayıtlı.
# [max: 10]
settings-wifi-tag-saved = Kayıtlı

# Nerede: Wi-Fi sayfasında kayıtlı ağ satırının alt metni. Anlam: Ağ şu an taramada görünüyor.
settings-wifi-saved-in-range = Kapsama alanında

# Nerede: Wi-Fi sayfasında kayıtlı ağ satırının alt metni. Anlam: Ağ şu an taramada görünmüyor.
settings-wifi-saved-out-of-range = Kapsama dışında

# Nerede: Wi-Fi sayfasında Kayıtlı ağlar kartının boş durumu. Anlam: Hiç kayıtlı kablosuz ağ yok.
settings-wifi-saved-empty = Kayıtlı ağ yok.

# Nerede: VPN sayfasında bağlantı satırının alt metni. Anlam: Bağlantı türü WireGuard.
settings-vpn-kind-wireguard = WireGuard

# Nerede: VPN sayfasında bağlantı satırının alt metni. Anlam: Bağlantı türü eklentili VPN (OpenVPN vb.).
settings-vpn-kind-vpn = VPN

# Nerede: VPN sayfasında bağlantı satırının sağındaki kısa etiket. Anlam: Bu VPN şu an açık.
# [max: 10]
settings-vpn-tag-active = Etkin

# Nerede: VPN sayfasında listenin boş durumu. Anlam: Hiç VPN ya da WireGuard bağlantısı yok.
settings-vpn-empty = Kayıtlı VPN bağlantısı yok.

# Nerede: Wi-Fi ve VPN sayfalarında, ağ durumu ilk kez okunurken kartın ya da satırın metni. Anlam: Bilgi NetworkManager'dan okunuyor.
settings-network-reading = Okunuyor…

# ---- Bağlantılar: eylemler (t114 v1 ikinci yarı; docs/design/network-pages.md) ----

# Nerede: Wi-Fi sayfasında kart başlığı. Anlam: Kablolu (Ethernet) bağlantının salt okunur bilgisi.
settings-section-wifi-wired = Kablolu

# Nerede: Wi-Fi sayfasında, wifi.wired satırının başlığı (aramada görünür). Anlam: Kablolu ağ bağlantısı.
settings-wifi-wired-label = Kablolu bağlantı

# Nerede: Wi-Fi sayfasında, wifi.wired satırı için arama sözcükleri. Anlam: Kablolu bağlantı kartını aramada bulmayı sağlar.
settings-wifi-wired-keywords = ethernet, kablolu, lan, ip adresi, dns, hız

# Nerede: Wi-Fi sayfasında Görünen ağlar kartının son satırı. Anlam: Adını yayınlamayan bir ağa adını yazarak bağlanmayı açar.
settings-wifi-hidden-label = Gizli ağa bağlan…

# Nerede: Wi-Fi sayfasında, wifi.hidden satırı için arama sözcükleri. Anlam: Gizli ağ satırını aramada bulmayı sağlar.
settings-wifi-hidden-keywords = gizli ağ, hidden network, ssid

# Nerede: VPN sayfasında VPN kartının son satırı. Anlam: Bir WireGuard yapılandırma dosyasını (.conf) seçip yeni VPN bağlantısı olarak ekler.
settings-vpn-import-label = WireGuard içe aktar…

# Nerede: VPN sayfasında, vpn.import satırı için arama sözcükleri. Anlam: İçe aktarma satırını aramada bulmayı sağlar.
settings-vpn-import-keywords = içe aktar, import, wireguard, conf, dosya

# Nerede: Wi-Fi ve VPN sayfalarının üstündeki uyarı notu. Anlam: Sistem ileti yoluna (D-Bus) bağlanılamadığı için ağ bilgisi alınamıyor.
settings-network-no-bus = Sistem ileti yoluna (D-Bus) bağlanılamıyor; ağ bilgileri gösterilemiyor.

# Nerede: Wi-Fi sayfasında ağ satırının hata metni. Anlam: Ağ şifre istedi ama kayıtlı şifre yok ya da yanlış; şifre alanı açılır.
settings-wifi-error-no-secrets = Şifre gerekli ya da yanlış. Şifreyi girin.

# Nerede: Wi-Fi sayfasında ağ satırının hata metni. Anlam: Ağ kimlik doğrulamayı reddetti, büyük olasılıkla şifre yanlış.
settings-wifi-error-auth-failed = Kimlik doğrulama başarısız; şifre yanlış olabilir.

# Nerede: VPN sayfasında bağlantı satırının hata metni. Anlam: VPN bağlanmak için gizli bilgi (anahtar, şifre) istedi ama alamadı.
settings-vpn-error-no-secrets = VPN için gerekli gizli bilgi verilmedi.

# Nerede: VPN sayfasında bağlantı satırının hata metni. Anlam: VPN sunucusu girişi reddetti.
settings-vpn-error-auth-failed = VPN girişi reddedildi.

# Nerede: Wi-Fi ve VPN satırlarının hata metni. Anlam: Bağlantı başka bir nedenle kurulamadı.
settings-network-error-activation = Bağlantı kurulamadı.

# Nerede: Wi-Fi sayfasında, şifreli bir ağa tıklayınca açılan şifre alanının ipucu metni. Anlam: Ağın şifresini yazın.
settings-wifi-password-placeholder = Şifre

# Nerede: Wi-Fi sayfasında şifre alanının ve gizli ağ formunun düğmesi. Anlam: Ağa bağlan.
# [max: 16]
settings-wifi-connect = Bağlan

# Nerede: Wi-Fi sayfasında şifre alanının ve gizli ağ formunun düğmesi. Anlam: Formu kapat, bağlanma.
# [max: 16]
settings-wifi-cancel = Vazgeç

# Nerede: Wi-Fi sayfasında bağlı ağa tıklayınca açılan bölümdeki düğme. Anlam: Bu ağla bağlantıyı kopar.
# [max: 24]
settings-wifi-disconnect = Bağlantıyı kes

# Nerede: Wi-Fi sayfasında kurumsal ağa tıklayınca açılan not. Anlam: Kullanıcı adı/sertifika isteyen ağlara bağlanma sonraki sürümde gelecek.
settings-wifi-enterprise-note = Kurumsal (802.1X) ağlara bağlanma henüz desteklenmiyor; sonraki sürümde gelecek.

# Nerede: Wi-Fi sayfasında gizli ağ formundaki ad alanının ipucu metni. Anlam: Ağın adını (SSID) yazın.
settings-wifi-hidden-ssid-placeholder = Ağ adı (SSID)

# Nerede: Wi-Fi sayfasında kayıtlı ağa tıklayınca açılan ayar satırı. Anlam: Ağ kapsamadayken kendiliğinden bağlan.
settings-wifi-autoconnect = Otomatik bağlan

# Nerede: Wi-Fi sayfasında kayıtlı ağa tıklayınca açılan ayar satırı. Anlam: Bu bağlantı kotalı/ücretli sayılsın mı (büyük indirmeler ertelenir).
settings-wifi-metered = Ölçülü bağlantı

# Nerede: Wi-Fi sayfasında Ölçülü bağlantı seçeneği. Anlam: Sistem kendisi karar versin.
# [max: 12]
settings-wifi-metered-auto = Otomatik

# Nerede: Wi-Fi sayfasında Ölçülü bağlantı seçeneği. Anlam: Bu bağlantı ölçülü.
# [max: 12]
settings-wifi-metered-yes = Evet

# Nerede: Wi-Fi sayfasında Ölçülü bağlantı seçeneği. Anlam: Bu bağlantı ölçülü değil.
# [max: 12]
settings-wifi-metered-no = Hayır

# Nerede: Wi-Fi sayfasında kayıtlı ağın ayar bölümündeki düğme. Anlam: Ağın kayıtlı ayarlarını ve şifresini sil.
# [max: 16]
settings-wifi-forget = Unut

# Nerede: Wi-Fi sayfasında Unut düğmesine bir kez basınca görünen onay metni. Anlam: Silmek için tekrar basın.
# [max: 16]
settings-wifi-forget-confirm = Unutulsun mu?

# Nerede: VPN sayfasında bağlantıya tıklayınca açılan bölümdeki düğme. Anlam: VPN bağlantısını sil.
# [max: 16]
settings-vpn-delete = Sil

# Nerede: VPN sayfasında Sil düğmesine bir kez basınca görünen onay metni. Anlam: Silmek için tekrar basın.
# [max: 16]
settings-vpn-delete-confirm = Silinsin mi?

# Nerede: Wi-Fi sayfasında bağlı ağın ve kablolu bağlantının ayrıntı satırı etiketi. Anlam: IPv4 adresi.
settings-network-detail-ipv4 = IPv4

# Nerede: Wi-Fi sayfasında bağlı ağın ve kablolu bağlantının ayrıntı satırı etiketi. Anlam: IPv6 adresi.
settings-network-detail-ipv6 = IPv6

# Nerede: Wi-Fi sayfasında bağlı ağın ve kablolu bağlantının ayrıntı satırı etiketi. Anlam: Ad çözümleme (DNS) sunucuları.
settings-network-detail-dns = DNS

# Nerede: Wi-Fi sayfasında kablolu bağlantının ayrıntı satırı etiketi. Anlam: Bağlantı hızı.
settings-network-detail-speed = Hız

# Nerede: Wi-Fi sayfasında kablolu bağlantının hız değeri. Anlam: $mbps saniyede megabit.
settings-network-speed-mbps = { $mbps } Mb/sn

# Nerede: Wi-Fi sayfasında kablolu bağlantının alt metni. Anlam: Kablo takılı değil ya da bağlantı yok.
settings-wired-disconnected = Bağlı değil

# Nerede: VPN sayfasında içe aktarma satırının alt metni, dosya seçici açıkken ya da içe aktarma sürerken. Anlam: İşlem sürüyor.
settings-vpn-import-busy = Dosya bekleniyor…

# Nerede: WireGuard içe aktarma dosya seçicisinde dosya türü süzgecinin adı. Anlam: WireGuard .conf dosyaları.
settings-vpn-import-filter = WireGuard yapılandırması (.conf)

# Nerede: WireGuard içe aktarma dosya seçicisinin başlığı. Anlam: İçe aktarılacak dosyayı seçin.
settings-vpn-import-dialog-title = WireGuard yapılandırması seçin

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosya seçici açılamadı.
settings-vpn-import-error-portal = Dosya seçici açılamadı (xdg-desktop-portal çalışıyor mu?).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Seçilen dosya okunamadı.
settings-vpn-import-error-read = Dosya okunamadı.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosya bir WireGuard yapılandırması için fazla büyük.
settings-vpn-import-error-too-large = Dosya çok büyük (en çok 64 KB).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosya düz metin değil.
settings-vpn-import-error-not-text = Dosya metin değil.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosyanın $line numaralı satırı WireGuard biçimine uymuyor.
settings-vpn-import-error-syntax = Geçersiz WireGuard dosyası (satır { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosyada [Interface] özel anahtarı yok.
settings-vpn-import-error-private-key = Dosyada özel anahtar (PrivateKey) yok.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: $line satırındaki eşin ortak anahtarı yok.
settings-vpn-import-error-public-key = Eşin ortak anahtarı (PublicKey) yok (satır { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: $line satırındaki anahtar geçerli bir WireGuard anahtarı değil.
settings-vpn-import-error-key = Geçersiz anahtar (satır { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: $line satırındaki adres, sayı ya da uç nokta geçersiz.
settings-vpn-import-error-value = Geçersiz değer (satır { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosyada çok fazla eş ya da adres var.
settings-vpn-import-error-too-many = Dosyada çok fazla eş ya da adres var.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Bağlantı adı dosya adından alınır; ad 1–15 harf/rakam ve - _ . = + olabilir.
settings-vpn-import-error-name = Dosya adı bağlantı adı olamaz: 1–15 karakter, yalnız harf, rakam ve - _ . = +.

# Nerede: VPN sayfasında içe aktarma satırının alt metni, içe aktarmadan sonra. Anlam: Dosyadaki bazı wg-quick ayarları (PostUp, Table…) içe aktarılmadı ve çalıştırılmayacak; $keys virgülle ayrılmış ayar adlarıdır (çevrilmez).
settings-vpn-import-ignored = Şu ayarlar içe aktarılmadı: { $keys }


# ---- Sistem › Güç (t117 iskeleti; docs/design/power-page.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Güç profilleri ve pil şarj sınırı sayfasının etiketi.
# [max: 14]
settings-page-power = Güç

# Nerede: Güç sayfasında kart başlığı. Anlam: Sistemin güç/performans dengesi seçimi.
settings-section-power-profile = Güç profili

# Nerede: Güç sayfasında kart başlığı. Anlam: Dizüstü pili ile ilgili ayarlar.
settings-section-power-battery = Pil

# Nerede: Güç sayfasının Güç profili kartında, satır başlığı. Anlam: Güç tasarrufu, dengeli ya da performans profilinden birini seçme.
settings-power-profile-label = Güç profili

# Nerede: Güç sayfasında, power.profile satırı için arama sözcükleri. Anlam: Güç profili seçimini aramada bulmayı sağlar.
settings-power-profile-keywords = güç, profil, performans, güç tasarrufu, dengeli, power-profiles-daemon, pil

# Nerede: Güç sayfasının Pil kartında, anahtar satırının başlığı. Anlam: Açıkken pil tam dolmadan şarj durur; pilin ömrü uzar.
settings-power-charge-limit-label = Pil ömrünü koru

# Nerede: Güç sayfasında, power.charge_limit satırı için arama sözcükleri. Anlam: Şarj sınırı anahtarını aramada bulmayı sağlar.
settings-power-charge-limit-keywords = şarj, şarj sınırı, pil sağlığı, pil, battery, charge limit

# Nerede: Güç profili seçiminde, ilk seçeneğin adı. Anlam: Pili ve enerjiyi koruyan, performansı düşüren profil.
# [max: 16]
settings-power-profile-power-saver = Güç tasarrufu

# Nerede: Güç profili seçiminde, ortadaki seçeneğin adı. Anlam: Varsayılan, dengeli profil.
# [max: 16]
settings-power-profile-balanced = Dengeli

# Nerede: Güç profili seçiminde, son seçeneğin adı. Anlam: Daha çok enerji harcayan, en hızlı profil.
# [max: 16]
settings-power-profile-performance = Performans

# Nerede: Güç profili satırının alt metni. Anlam: Performans profili kısıtlandı, çünkü dizüstü bilgisayar kucakta (algılayıcı ısınmayı önlemek için).
settings-power-degraded-lap = Performans kısıtlı: dizüstü kucakta algılandı

# Nerede: Güç profili satırının alt metni. Anlam: Performans profili kısıtlandı, çünkü aygıtın sıcaklığı yüksek.
settings-power-degraded-temperature = Performans kısıtlı: sıcaklık yüksek

# Nerede: Güç profili satırının alt metni. Anlam: Performans profili bilinmeyen bir nedenle kısıtlandı.
settings-power-degraded-other = Performans kısıtlı

# Nerede: Güç profili satırının alt metni, seçim devre dışıyken. Anlam: Profilleri değiştirmek için bu sistem hizmeti kurulu ve çalışıyor olmalı. "power-profiles-daemon" paket adıdır, çevrilmez.
settings-power-profiles-missing = Güç profilleri için power-profiles-daemon gerekli

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Birden çok pil var; şarj sınırı bazılarında açık, bazılarında kapalı.
settings-power-charge-mixed = Pillerin bir kısmında açık

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Şarj $end yüzdesinde durur. $end_tail sayının okunuşundaki son sözcüğün değeridir — kural: n%10 ≠ 0 ise n%10; değilse n%100 ≠ 0 ise n%100; değilse 100 (n = 0 → 0). Örnek: 75 → 5, 4 → 4, 40 → 40, 80 → 80, 100 → 100, 0 → 0; Türkçe bulunma eki ('de/'da/'te/'ta) ona göre seçilir. Eki sayıya bağlı olmayan diller $end_tail'i kullanmaz.
settings-power-charge-end = Şarj %{ $end }'{ $end_tail ->
        [0] da
        [1] de
        [2] de
        [3] te
        [4] te
        [5] te
        [6] da
        [7] de
        [8] de
        [9] da
        [10] da
        [20] de
        [30] da
        [40] ta
        [50] de
        [60] ta
        [70] te
        [80] de
        [90] da
       *[100] de
    } durur

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Şarj $end yüzdesinde durur; pil $start yüzdesinin altına düşünce yeniden şarj olmaya başlar. $end_tail / $start_tail sayıların okunuşundaki son sözcüğün değeridir (kural: n%10 ≠ 0 ise n%10; değilse n%100 ≠ 0 ise n%100; değilse 100; n = 0 → 0); Türkçe bulunma ('de) ve ilgi ('in) ekleri ona göre seçilir. Eki sayıya bağlı olmayan diller bunları kullanmaz.
settings-power-charge-range = Şarj %{ $end }'{ $end_tail ->
        [0] da
        [1] de
        [2] de
        [3] te
        [4] te
        [5] te
        [6] da
        [7] de
        [8] de
        [9] da
        [10] da
        [20] de
        [30] da
        [40] ta
        [50] de
        [60] ta
        [70] te
        [80] de
        [90] da
       *[100] de
    } durur, %{ $start }'{ $start_tail ->
        [0] ın
        [1] in
        [2] nin
        [3] ün
        [4] ün
        [5] in
        [6] nın
        [7] nin
        [8] in
        [9] un
        [10] un
        [20] nin
        [30] un
        [40] ın
        [50] nin
        [60] ın
        [70] in
        [80] in
        [90] ın
       *[100] ün
    } altına düşünce yeniden başlar

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Şarjın hangi yüzdede duracağını sistem değil pilin/aygıtın kendi yazılımı belirliyor.
settings-power-charge-firmware = Şarj sınırını aygıt yazılımı belirler

# Nerede: Güç sayfasında satırın alt metni (hata rengi), değişiklik reddedilince. Anlam: Sistem izin vermedi; bu ayar yalnızca bilgisayarın başında oturan etkin kullanıcı tarafından değiştirilebilir.
settings-power-error-denied = İzin verilmedi (yalnızca etkin yerel oturumda değiştirilebilir)

# Nerede: "Pil ömrünü koru" satırının alt metni (hata rengi). Anlam: Pil bilgisini sağlayan UPower sistem hizmeti çalışmıyor. "UPower" ad, çevrilmez.
settings-power-error-upower-missing = UPower çalışmıyor

# Nerede: Güç sayfasında satırın alt metni (hata rengi). Anlam: Değişiklik bilinmeyen bir nedenle uygulanamadı.
settings-power-error-failed = Değiştirilemedi

# Nerede: Güç sayfasında, power.status bloğunun başlığı (aramada görünür). Anlam: Güç bilgisinin (pil, güç profilleri) alınamadığı durumu bildiren üst blok.
settings-power-status-label = Güç bilgisi

# Nerede: Güç sayfasında, power.status bloğu için arama sözcükleri. Anlam: Boş durum bloğunu aramada bulmayı sağlar. "UPower" ve "power-profiles-daemon" ad, çevrilmez.
settings-power-status-keywords = güç, upower, power-profiles-daemon

# Nerede: Güç sayfasının Pil kartında, pil durumu satırının başlığı (yalnız aramada; satırda yüzde görünür). Anlam: Pilin doluluk ve şarj durumu.
settings-power-battery-label = Pil durumu

# Nerede: Güç sayfasında, power.battery satırı için arama sözcükleri. Anlam: Pil durumu satırını aramada bulmayı sağlar.
settings-power-battery-keywords = pil, şarj, yüzde, kalan süre, battery

# Nerede: Pil kartındaki durum satırının başlığı. Anlam: Pilin doluluk yüzdesi. Not: yüzde işareti Türkçede sayının önünde.
# Değişkenler: $percent (0–100 arası tam sayı, örnek: 64).
settings-power-battery-percent = %{ $percent }

# Nerede: Pil durum satırının alt metni; pil şarj oluyor, dolma süresi bilinmiyor.
settings-power-battery-charging = Şarj oluyor

# Nerede: Pil durum satırının alt metni; pil şarj oluyor ve dolmasına kalan süre biliniyor.
# Değişkenler: $time (biçimlenmiş süre, örnek: "35 dk", "1 sa 20 dk").
settings-power-battery-charging-time = Şarj oluyor · { $time } içinde dolar

# Nerede: Pil durum satırının alt metni; bilgisayar pilden çalışıyor, kalan süre bilinmiyor.
settings-power-battery-discharging = Pilden çalışıyor

# Nerede: Pil durum satırının alt metni; bilgisayar pilden çalışıyor ve pilin bitmesine kalan süre biliniyor.
# Değişkenler: $time (biçimlenmiş süre, örnek: "2 sa 10 dk").
settings-power-battery-discharging-time = Pilden çalışıyor · { $time } kaldı

# Nerede: Pil durum satırının alt metni; pil tamamen dolu.
settings-power-battery-full = Tam dolu

# Nerede: Pil durum satırının alt metni; bilgisayar prize takılı ama pil şarj olmuyor (ör. şarj sınırına ulaştı).
settings-power-battery-not-charging = Prizde, şarj olmuyor

# Nerede: Pil durum satırındaki süre (kalan ya da dolma süresi). Anlam: bir saatten kısa süre, yalnız dakika.
# Değişkenler: $minutes (1–59, örnek: 35).
settings-power-duration-minutes = { $minutes } dk

# Nerede: Pil durum satırındaki süre. Anlam: tam saat uzunluğunda süre.
# Değişkenler: $hours (örnek: 2).
settings-power-duration-hours = { $hours } sa

# Nerede: Pil durum satırındaki süre. Anlam: saat ve dakikalı süre.
# Değişkenler: $hours (örnek: 2), $minutes (1–59, örnek: 10).
settings-power-duration-hours-minutes = { $hours } sa { $minutes } dk

# Nerede: Güç sayfasının tek içeriği (maskotlu hata durumu başlığı); pil bilgisi ve güç profilleri hizmetlerinin ikisine de ulaşılamıyor.
settings-power-empty-unavailable = Güç bilgisi alınamıyor

# Nerede: Güç sayfasındaki hata durumunun açıklaması. Anlam: Pil için UPower, güç profilleri için power-profiles-daemon sistem hizmeti gerekir; ikisi de çalışmıyor. Hizmet adları çevrilmez.
settings-power-empty-unavailable-body = Pil bilgisi için UPower, güç profilleri için power-profiles-daemon gerekli; ikisi de çalışmıyor.

# Nerede: Güç sayfasının tek içeriği (maskotlu boş durum başlığı); bilgisayarda pil yok ve güç profilleri hizmeti yok (masaüstü, sanal makine).
settings-power-empty-no-battery = Ayarlanacak güç seçeneği yok

# Nerede: Güç sayfasındaki boş durumun açıklaması. Anlam: Pil bulunamadı; güç profilleri için power-profiles-daemon kurulup çalıştırılabilir. Hizmet adı çevrilmez.
settings-power-empty-no-battery-body = Bu bilgisayarda pil bulunamadı. Güç profilleri için power-profiles-daemon gerekli.

# ---- Sistem › Yazıcılar (t125; docs/design/printers.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Yazıcılar ve yazdırma kuyruğu sayfasının etiketi.
# [max: 14]
settings-page-printers = Yazıcılar

# Nerede: Yazıcılar sayfasında kart başlığı. Anlam: Bu bilgisayara tanımlı yazıcıların listesi.
settings-section-printers = Yazıcılar

# Nerede: Yazıcılar sayfasında kart başlığı. Anlam: Kullanıcının henüz bitmemiş yazdırma işleri.
settings-section-printers-jobs = Yazdırma kuyruğu

# Nerede: Yazıcılar sayfasında, printers.status satırının başlığı (aramada görünür). Anlam: Yazdırma servisinin (CUPS) durumunu bildiren üst blok.
settings-printers-status-label = Yazdırma servisi

# Nerede: Yazıcılar sayfasında, printers.status satırı için arama sözcükleri. Anlam: Durum bloğunu aramada bulmayı sağlar.
settings-printers-status-keywords = cups, yazdırma servisi, yazdırma hizmeti

# Nerede: Yazıcılar sayfasında, printers.list satırının başlığı (aramada görünür). Anlam: Yazıcı listesi.
settings-printers-list-label = Yazıcılar

# Nerede: Yazıcılar sayfasında, printers.list satırı için arama sözcükleri. Anlam: Yazıcı listesini aramada bulmayı sağlar.
settings-printers-list-keywords = yazıcı, varsayılan yazıcı, deneme sayfası, test sayfası, printer

# Nerede: Yazıcılar kartının son satırının başlığı. Anlam: Yeni yazıcı tanımlama.
settings-printers-add-label = Yazıcı ekle

# Nerede: "Yazıcı ekle" satırının alt metni. Anlam: Yazıcı ekleme ve gelişmiş yazıcı ayarlarının CUPS'un tarayıcıda açılan yönetim sayfasında yapıldığı. "CUPS" ad, çevrilmez.
settings-printers-add-description = Yazıcı ekleme ve gelişmiş ayarlar CUPS web arayüzünde

# Nerede: Yazıcılar sayfasında, printers.add satırı için arama sözcükleri. Anlam: "Yazıcı ekle" satırını aramada bulmayı sağlar.
settings-printers-add-keywords = yazıcı ekle, yeni yazıcı, cups web, yönetim

# Nerede: Yazıcılar sayfasında, printers.jobs satırının başlığı (aramada görünür). Anlam: Yazdırma kuyruğu listesi.
settings-printers-jobs-label = Yazdırma kuyruğu

# Nerede: Yazıcılar sayfasında, printers.jobs satırı için arama sözcükleri. Anlam: Kuyruğu aramada bulmayı sağlar.
settings-printers-jobs-keywords = kuyruk, iş, yazdırma işi, iptal, bekleyen

# Nerede: Yazıcılar sayfası, maskotlu durumun başlığı. Anlam: Yazdırma servisi (CUPS) kurulu değil ya da çalışmıyor.
settings-printers-unavailable = Yazdırma servisi yok

# Nerede: Yazıcılar sayfası, maskotlu durumun gövdesi. Anlam: CUPS'un kurulması ve başlatılması gerektiği; paket adı "cups" ve birim adı "cups.socket" çevrilmez.
settings-printers-unavailable-body = CUPS kurulu değil ya da çalışmıyor. Yazıcı kullanmak için cups paketini kurup cups.socket hizmetini başlatın.

# Nerede: Yazıcılar sayfası, maskotlu durumun başlığı. Anlam: Yazdırma servisine bağlanıldı ama bilgi okunamadı.
settings-printers-failed = Yazıcı bilgisi okunamadı

# Nerede: Yazıcılar sayfası, maskotlu durumun gövdesi. Anlam: Sayfanın kendiliğinden yeniden deneyeceği.
settings-printers-failed-body = Yazdırma servisi beklenmeyen bir yanıt verdi. Sayfa birkaç saniyede bir yeniden dener.

# Nerede: Yazıcılar kartı, ilk okuma sürerken. Anlam: Yazıcı listesinin okunmakta olduğu.
settings-printers-reading = Yazıcılar okunuyor…

# Nerede: Yazıcılar kartı, maskotlu boş durumun başlığı. Anlam: Tanımlı yazıcı olmadığı.
settings-printers-empty = Yazıcı yok

# Nerede: Yazıcılar kartı, maskotlu boş durumun gövdesi. Anlam: Altındaki "Yazıcı ekle" satırından yazıcı eklenebileceği.
settings-printers-empty-body = Aşağıdaki "Yazıcı ekle" ile bir yazıcı tanımlayabilirsiniz.

# Nerede: Yazıcı satırının alt metni. Anlam: Yazıcının boşta, yazdırmaya hazır olduğu.
settings-printers-state-idle = Hazır

# Nerede: Yazıcı satırının alt metni. Anlam: Yazıcının şu an bir iş yazdırdığı.
settings-printers-state-processing = Yazdırıyor

# Nerede: Yazıcı satırının alt metni (hata rengi). Anlam: Yazıcının durdurulduğu (duraklatılmış ya da hata nedeniyle).
settings-printers-state-stopped = Durduruldu

# Nerede: Yazıcı satırının alt metni, durumdan sonra. Anlam: Yazıcının yeni yazdırma işi kabul etmediği.
settings-printers-not-accepting = İş kabul etmiyor

# Nerede: Yazıcı satırının alt metninin başı. Anlam: Bu yazıcının varsayılan yazıcı olduğu.
settings-printers-default = Varsayılan

# Nerede: Yazıcı satırının alt metninin sonu, deneme sayfası gönderilince. Anlam: Deneme sayfasının yazıcıya gönderildiği.
settings-printers-test-page-sent = Deneme sayfası gönderildi

# Nerede: Yazıcı satırı açılınca ayrıntı satırının etiketi. Anlam: Yazıcının markası ve modeli.
settings-printers-detail-model = Model

# Nerede: Yazıcı satırı açılınca ayrıntı satırının etiketi. Anlam: Yazıcının bulunduğu yer (CUPS'ta girilen konum).
settings-printers-detail-location = Konum

# Nerede: Yazıcı satırı açılınca ayrıntı satırının etiketi. Anlam: CUPS'taki kuyruk adı (komut satırında kullanılan ad).
settings-printers-detail-queue = Kuyruk adı

# Nerede: Yazıcı ayrıntısında düğme. Anlam: Bu yazıcıyı kullanıcının varsayılan yazıcısı yapar.
settings-printers-make-default = Varsayılan yap

# Nerede: Yazıcı ayrıntısında, yazıcı zaten varsayılanken devre dışı düğmenin etiketi. Anlam: Bu yazıcının zaten varsayılan olduğu.
settings-printers-is-default = Varsayılan yazıcı

# Nerede: Yazıcı ayrıntısında düğme. Anlam: Yazıcıya deneme sayfası gönderir.
settings-printers-test-page = Deneme sayfası

# Nerede: Yazdırma kuyruğunda deneme sayfası işinin adı. Anlam: Ayarlar'dan gönderilen deneme sayfası.
settings-printers-test-page-title = Deneme sayfası

# Nerede: "Yazıcı ekle" satırındaki düğme. Anlam: CUPS web arayüzünü tarayıcıda açar.
settings-printers-open-web = Aç

# Nerede: Yazdırma kuyruğu kartı, iş yokken. Anlam: Bekleyen yazdırma işi olmadığı.
settings-printers-jobs-empty = Bekleyen yazdırma işi yok

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin sırasını beklediği.
settings-printers-job-pending = Sırada

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin bekletildiği (elle ya da yazıcı nedeniyle).
settings-printers-job-held = Bekletiliyor

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin şu an yazdırıldığı.
settings-printers-job-processing = Yazdırılıyor

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin yazdırılırken durduğu.
settings-printers-job-stopped = Durdu

# Nerede: Yazdırma kuyruğunda iş satırının alt metni (kısa an). Anlam: İşin bittiği ya da iptal edildiği.
settings-printers-job-done = Bitti

# Nerede: Yazdırma kuyruğunda adı olmayan işin başlığı. Anlam: Numarasıyla anılan yazdırma işi.
# Değişkenler: $id (iş numarası, ör. 42).
settings-printers-job-untitled = İş { $id }

# Nerede: Yazdırma kuyruğunda iş satırının alt metninin sonu. Anlam: İşin boyutu (kilobayt).
# Değişkenler: $size (etkin biçimde yazılmış kilobayt sayısı, ör. "1.250").
settings-printers-job-size = { $size } KB

# Nerede: Yazdırma kuyruğunda iş satırının düğmesi. Anlam: Yazdırma işini iptal eder.
settings-printers-job-cancel = İptal

# Nerede: Yazdırma kuyruğunda, iptal isteği sürerken düğmenin etiketi. Anlam: İşin iptal edilmekte olduğu.
settings-printers-job-cancelling = İptal ediliyor…

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Varsayılan yazıcı ayarı kullanıcının dosyasına yazılamadı.
settings-printers-error-write = Varsayılan yazıcı kaydedilemedi

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: İşlem yapılan yazıcı bu arada kaldırılmış.
settings-printers-error-gone = Yazıcı artık yok

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Yazdırma servisi bu işleme izin vermedi.
settings-printers-error-denied = Yazdırma servisi izin vermedi

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: İşlem sırasında yazdırma servisine ulaşılamadı.
settings-printers-error-unreachable = Yazdırma servisine ulaşılamadı

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: CUPS'un deneme sayfası dosyası bulunamadı.
settings-printers-error-test-page-missing = Deneme sayfası dosyası bulunamadı

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Yazıcı varsayılan yapılamadı.
settings-printers-error-default = Varsayılan yapılamadı

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Deneme sayfası yazıcıya gönderilemedi.
settings-printers-error-test-page = Deneme sayfası gönderilemedi

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Yazdırma işi iptal edilemedi.
settings-printers-error-cancel = İş iptal edilemedi

# ---- Bağlantılar: Bluetooth (t115; docs/design/bluetooth-page.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Bluetooth sayfasının etiketi.
# [max: 14]
settings-page-bluetooth = Bluetooth

# Nerede: Bluetooth sayfasında kart başlığı. Anlam: Bluetooth aç/kapa, bu bilgisayarın adı ve görünürlüğü.
settings-section-bluetooth = Bluetooth

# Nerede: Bluetooth sayfasında kart başlığı. Anlam: Daha önce eşleştirilmiş cihazların listesi.
settings-section-bluetooth-paired = Eşleşmiş cihazlar

# Nerede: Bluetooth sayfasında kart başlığı. Anlam: Taramada bulunan, henüz eşleştirilmemiş cihazlar.
settings-section-bluetooth-nearby = Yakındaki cihazlar

# Nerede: Bluetooth sayfasında, bluetooth.status satırının başlığı (aramada görünür). Anlam: Bluetooth hizmetinin ve adaptörün durumunu bildiren üst blok.
settings-bluetooth-status-label = Bluetooth durumu

# Nerede: Bluetooth sayfasında, bluetooth.status satırı için arama sözcükleri. Anlam: Durum bloğunu aramada bulmayı sağlar.
settings-bluetooth-status-keywords = bluez, adaptör, bluetooth hizmeti

# Nerede: Bluetooth sayfasının Bluetooth kartında, bluetooth.enabled satırının başlığı. Anlam: Bluetooth'un açık mı kapalı mı olduğu.
settings-bluetooth-enabled-label = Bluetooth

# Nerede: Bluetooth sayfasında, bluetooth.enabled satırı için arama sözcükleri. Anlam: Aç/kapa satırını aramada bulmayı sağlar.
settings-bluetooth-enabled-keywords = bluetooth, aç, kapat, uçak modu, rfkill

# Nerede: Bluetooth kartında, bluetooth.name satırının başlığı. Anlam: Diğer cihazların bu bilgisayarı gördüğü Bluetooth adı.
settings-bluetooth-name-label = Bu bilgisayarın adı

# Nerede: Bluetooth sayfasında, bluetooth.name satırı için arama sözcükleri. Anlam: Ad satırını aramada bulmayı sağlar.
settings-bluetooth-name-keywords = ad, isim, adaptör adı, cihaz adı

# Nerede: Bluetooth kartında, bluetooth.discoverable satırının başlığı. Anlam: Bu bilgisayarın yakındaki cihazlarca bulunabilir olması.
settings-bluetooth-discoverable-label = Görünür

# Nerede: Bluetooth sayfasında, bluetooth.discoverable satırı için arama sözcükleri. Anlam: Görünürlük satırını aramada bulmayı sağlar.
settings-bluetooth-discoverable-keywords = görünürlük, keşfedilebilir, discoverable, bulunabilir

# Nerede: Bluetooth sayfasında, bluetooth.paired satırının başlığı (aramada görünür). Anlam: Eşleşmiş cihazlar listesi.
settings-bluetooth-paired-label = Eşleşmiş cihazlar

# Nerede: Bluetooth sayfasında, bluetooth.paired satırı için arama sözcükleri. Anlam: Eşleşmiş cihazlar listesini aramada bulmayı sağlar.
settings-bluetooth-paired-keywords = cihaz, kulaklık, klavye, fare, bağlan, unut, güvenilir, engelle, pil

# Nerede: Bluetooth sayfasında, bluetooth.nearby satırının başlığı (aramada görünür). Anlam: Yakındaki (eşleşmemiş) cihazlar listesi.
settings-bluetooth-nearby-label = Yakındaki cihazlar

# Nerede: Bluetooth sayfasında, bluetooth.nearby satırı için arama sözcükleri. Anlam: Yakındaki cihazlar listesini aramada bulmayı sağlar.
settings-bluetooth-nearby-keywords = tara, eşleştir, yeni cihaz, pair, ara

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Ayarlar'ın Bluetooth hizmeti istemcisi başlatılamadı.
settings-bluetooth-failed = Bluetooth hizmetine bağlanılamadı

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: Ayarlar'ı yeniden açmayı öneren açıklama.
settings-bluetooth-failed-body = Ayarlar'ı kapatıp yeniden açmayı deneyin.

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Sistem ileti yoluna (D-Bus sistem bus'ı) ulaşılamıyor.
settings-bluetooth-unreachable = Sistem hizmetlerine ulaşılamıyor

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: Bağlantı geri gelince sayfanın kendiliğinden yenileneceği.
settings-bluetooth-unreachable-body = Bağlantı geri gelince bu sayfa kendiliğinden güncellenir.

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Bluetooth sistem hizmeti (BlueZ) çalışmıyor. "BlueZ" ad, çevrilmez.
settings-bluetooth-missing = BlueZ çalışmıyor

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: Bluetooth hizmetinin kurulup başlatılması gerektiği; "bluetooth.service" sistem hizmetinin adı, çevrilmez.
settings-bluetooth-missing-body = Bluetooth için bluez paketinin kurulu ve bluetooth.service hizmetinin çalışıyor olması gerekir.

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Bilgisayarda Bluetooth donanımı (adaptör) bulunamadı.
settings-bluetooth-no-adapter = Bluetooth adaptörü yok

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: USB Bluetooth adaptörü takılınca sayfanın kendiliğinden güncelleneceği.
settings-bluetooth-no-adapter-body = Bir Bluetooth adaptörü takarsanız burada görünür.

# Nerede: Bluetooth sayfasının üst notu, bir işlem başarısız olunca. Anlam: $detail BlueZ'in (çevrilmemiş) hata metni.
# Değişkenler: $detail (BlueZ hata metni).
settings-bluetooth-error-call-failed = İşlem başarısız: { $detail }

# Nerede: Bluetooth sayfasının üst notu. Anlam: İşlem zaman aşımına uğradı.
settings-bluetooth-error-timeout = İşlem zaman aşımına uğradı

# Nerede: Bluetooth sayfasının üst notu. Anlam: Bluetooth adaptörü bulunamadı.
settings-bluetooth-error-no-adapter = Bluetooth adaptörü bulunamadı

# Nerede: Bluetooth sayfasının üst notu. Anlam: Bluetooth hizmeti (BlueZ) çalışmıyor.
settings-bluetooth-error-unavailable = BlueZ çalışmıyor

# Nerede: Bluetooth sayfasının üst notu. Anlam: İşlem yapılan cihaz artık listede yok.
settings-bluetooth-error-unknown-device = Cihaz artık görünmüyor

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi), eşleştirme bitince. Anlam: Eşleştirme kabukta ya da cihazda reddedildi.
settings-bluetooth-error-auth-rejected = Eşleştirme reddedildi

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Eşleştirme başarısız (yanlış kod ya da onay istenemedi — onay penceresini kabuk açar).
settings-bluetooth-error-auth-failed = Eşleştirme başarısız; kod yanlış ya da onay istenemedi (kabuk çalışıyor mu?)

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Eşleştirme cihaz tarafında iptal edildi.
settings-bluetooth-error-auth-canceled = Eşleştirme iptal edildi

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Onay zamanında verilmedi.
settings-bluetooth-error-auth-timeout = Onay zamanında verilmedi

# Nerede: Bluetooth sayfası (not ya da satır alt metni). Anlam: Cihaza ulaşılamadı; kapalı ya da uzakta olabilir.
settings-bluetooth-error-connection-failed = Cihaza ulaşılamadı; açık ve yakında mı?

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Cihaz zaten eşleşmiş.
settings-bluetooth-error-already-exists = Bu cihaz zaten eşleşmiş

# Nerede: Bluetooth sayfasının üst notu. Anlam: Aynı işlem zaten sürüyor.
settings-bluetooth-error-in-progress = Bu işlem zaten sürüyor

# Nerede: Bluetooth sayfasının üst notu. Anlam: Adaptör hazır değil (çoğunlukla kapalı).
settings-bluetooth-error-not-ready = Bluetooth hazır değil; önce açın

# Nerede: Bluetooth sayfasının üst notu. Anlam: Girilen değer (ör. ad) geçersiz ya da çok uzun.
settings-bluetooth-error-invalid-value = Değer geçersiz ya da çok uzun

# Nerede: Bluetooth sayfasının üst notu, açma başarısız olunca. Anlam: Bluetooth fiziksel bir anahtarla kapatılmış; yazılımla açılamaz.
settings-bluetooth-error-rfkill-hard = Bluetooth donanım anahtarıyla kapatılmış; açmak için cihazdaki anahtarı kullanın

# Nerede: Bluetooth sayfasının üst notu, açma başarısız olunca. Anlam: Bluetooth uçak kipi (yazılım engeli) nedeniyle açılamadı.
settings-bluetooth-error-rfkill = Uçak kipi açık; Bluetooth açılamadı

# Nerede: Bluetooth satırının alt metni. Anlam: Bluetooth açık.
settings-bluetooth-power-on = Açık

# Nerede: Bluetooth satırının alt metni. Anlam: Bluetooth kapalı.
settings-bluetooth-power-off = Kapalı

# Nerede: Bluetooth satırının alt metni. Anlam: Bluetooth açılıyor ya da kapanıyor.
settings-bluetooth-power-turning = Değiştiriliyor…

# Nerede: Bluetooth satırının alt metni. Anlam: Uçak kipi (yazılım engeli) Bluetooth'u kapatmış.
settings-bluetooth-power-soft-blocked = Uçak kipi açık

# Nerede: Bluetooth satırının alt metni. Anlam: Bilgisayardaki fiziksel anahtar Bluetooth'u kapatmış.
settings-bluetooth-power-hard-blocked = Donanım anahtarıyla kapatılmış

# Nerede: "Bu bilgisayarın adı" satırının alt metni. Anlam: Bu adın yakındaki cihazlarda göründüğü.
settings-bluetooth-name-hint = Diğer cihazlar bu bilgisayarı bu adla görür

# Nerede: "Görünür" satırının alt metni. Anlam: Bu bilgisayar şu an yakındaki cihazlarca bulunamaz.
settings-bluetooth-discoverable-off = Yakındaki cihazlar bu bilgisayarı bulamaz

# Nerede: "Görünür" satırının alt metni, görünürken. Anlam: Görünürlüğün kendiliğinden kapanmasına kalan süre.
# Değişkenler: $time (dakika:saniye, ör. 2:05).
settings-bluetooth-discoverable-remaining = Görünür; { $time } sonra kapanır

# Nerede: "Görünür" satırının alt metni, süresiz görünürken. Anlam: Yakındaki cihazlar bu bilgisayarı bulabilir.
settings-bluetooth-discoverable-on = Yakındaki cihazlar bu bilgisayarı bulabilir

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihazın sinyali güçlü (yakın).
settings-bluetooth-signal-strong = Güçlü sinyal

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihazın sinyali orta.
settings-bluetooth-signal-medium = Orta sinyal

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihazın sinyali zayıf (uzak).
settings-bluetooth-signal-weak = Zayıf sinyal

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihaz son taramada duyulmadı.
settings-bluetooth-signal-none = Şu an duyulmuyor

# Nerede: Eşleşmiş cihaz satırının alt metni. Anlam: Cihaz bağlı.
settings-bluetooth-connected = Bağlı

# Nerede: Eşleşmiş cihaz satırının alt metni. Anlam: Cihaz eşleşmiş ama bağlı değil.
settings-bluetooth-not-connected = Bağlı değil

# Nerede: Eşleşmiş cihaz satırının alt metni, durumdan sonra. Anlam: Cihaz engellendi (bağlanamaz).
settings-bluetooth-blocked = Engellendi

# Nerede: Yakındaki cihaz satırının alt metni, eşleştirme sürerken. Anlam: PIN/onayın kabuğun açtığı pencerede verilmesi gerektiği.
settings-bluetooth-pairing-wait = Eşleştiriliyor; onayı kabuk penceresinde verin

# Nerede: Yakındaki cihaz satırının alt metni, eşleştirme sürerken masaüstü kabuğunun eşleştirme ajanı yokken. Anlam: PIN/onay penceresini kabuk açar; kabuk çalışmıyorsa onay istenen cihazlar eşleşemez (onaysız cihazlar yine eşleşebilir).
settings-bluetooth-pairing-no-agent = Eşleştirme onayı için masaüstü kabuğu çalışmalı

# Nerede: Bluetooth sayfası kartları, ilk durum okunurken. Anlam: Bilgiler okunuyor.
settings-bluetooth-reading = Okunuyor…

# Nerede: Eşleşmiş cihazlar kartı, maskotlu boş durumun başlığı. Anlam: Hiç eşleşmiş cihaz yok.
settings-bluetooth-paired-empty = Eşleşmiş cihaz yok

# Nerede: Eşleşmiş cihazlar kartı, maskotlu boş durumun gövdesi. Anlam: Yakındaki cihazlardan birini eşleştirmeyi önerir.
settings-bluetooth-paired-empty-body = Aşağıdaki yakındaki cihazlardan birini eşleştirin.

# Nerede: Yakındaki cihazlar kartı, Bluetooth kapalıyken. Anlam: Kapalıyken tarama yapılamadığı.
settings-bluetooth-nearby-off = Bluetooth kapalıyken yakındaki cihazlar görünmez.

# Nerede: Yakındaki cihazlar kartı, tarama bitti ve cihaz yokken. Anlam: Çevrede eşleştirilecek cihaz bulunamadı.
settings-bluetooth-nearby-empty = Yakında cihaz bulunamadı.

# Nerede: Yakındaki cihazlar kartının üst satırı, tarama sürerken. Anlam: Cihaz aranıyor.
settings-bluetooth-scanning = Aranıyor…

# Nerede: Yakındaki cihazlar kartının üst satırı, tarama durunca. Anlam: Tarama bitti.
settings-bluetooth-scan-stopped = Arama durdu

# Nerede: Yakındaki cihazlar kartının üst satırındaki düğme. Anlam: Cihaz aramayı yeniden başlatır.
settings-bluetooth-rescan = Yeniden ara

# Nerede: Yakındaki cihaz satırındaki düğme. Anlam: Bu cihazla eşleştirmeyi başlatır.
settings-bluetooth-pair = Eşleştir

# Nerede: Yakındaki cihaz satırındaki düğme, eşleştirme sürerken. Anlam: Süren eşleştirmeyi iptal eder.
settings-bluetooth-cancel = İptal

# Nerede: Yakındaki cihazlar kartının alt satırı. Anlam: Adı bilinmeyen kaç cihazın gizlendiği.
# Değişkenler: $count (gizlenen cihaz sayısı, ör. 3).
settings-bluetooth-unnamed-hidden = { $count } adsız cihaz gizlendi

# Nerede: Yakındaki cihazlar kartının alt satırındaki düğme. Anlam: Adsız cihazları da listeler.
settings-bluetooth-unnamed-show = Göster

# Nerede: Yakındaki cihazlar kartının alt satırı. Anlam: Adı bilinmeyen kaç cihazın listede gösterildiği.
# Değişkenler: $count (gösterilen adsız cihaz sayısı, ör. 3).
settings-bluetooth-unnamed-shown = { $count } adsız cihaz gösteriliyor

# Nerede: Yakındaki cihazlar kartının alt satırındaki düğme. Anlam: Adsız cihazları yeniden gizler.
settings-bluetooth-unnamed-hide = Gizle

# Nerede: Eşleşmiş cihazın açılan ayrıntısında, ad alanının etiketi. Anlam: Cihazın bu bilgisayarda görünen adı (yeniden adlandırma).
settings-bluetooth-rename = Ad

# Nerede: Eşleşmiş cihazın ayrıntısında anahtar başlığı. Anlam: Cihaz onay sormadan bağlanabilir.
settings-bluetooth-trusted = Güvenilir

# Nerede: "Güvenilir" anahtarının alt metni. Anlam: Güvenilir cihazın bağlanırken onay istemediği.
settings-bluetooth-trusted-hint = Onay sormadan bağlanabilir

# Nerede: Eşleşmiş cihazın ayrıntısında anahtar başlığı. Anlam: Cihazı engeller (bağlanamaz).
settings-bluetooth-block = Engelle

# Nerede: "Engelle" anahtarının alt metni. Anlam: Engelli cihazın bağlanamayacağı.
settings-bluetooth-block-hint = Bu cihaz bağlanamaz

# Nerede: Eşleşmiş cihazın ayrıntısındaki düğme. Anlam: Cihaza bağlanır.
settings-bluetooth-connect = Bağlan

# Nerede: Eşleşmiş cihazın ayrıntısındaki düğme, bağlıyken. Anlam: Cihazla bağlantıyı keser.
settings-bluetooth-disconnect = Bağlantıyı kes

# Nerede: Eşleşmiş cihazın ayrıntısındaki düğme. Anlam: Eşleşmeyi siler (cihaz yeniden eşleştirilmeli).
settings-bluetooth-forget = Unut

# Nerede: "Unut" düğmesine bir kez basılınca düğmenin etiketi. Anlam: Onay için yeniden basılması gerektiği.
settings-bluetooth-forget-confirm = Unut · tekrar bas
## Yazı tipleri ve tipografi

# Nerede: Yazı sayfası, Eş aralıklı rolünün alt yazısı (src/typography.rs). Anlam: Rolün nerede kullanıldığını söyler.
settings-typography-mono-subtitle = Kod, terminal çıktısı, sayılar

# Nerede: Yazı sayfası, İkon rolünün alt yazısı (src/typography.rs). Anlam: Rolün ligatürlü sembol fontu olduğunu söyler.
settings-typography-icon-subtitle = Sembol fontu (ligatürlü)

# Nerede: Yazı sayfası, önizleme kartında Arayüz rolünün kendi fontuyla çizilen örnek satırı (src/typography.rs). Anlam: Arayüz fontuyla çizilen örnek metin.
settings-typography-sample-ui = Ağ bağlantısı · Ses düzeyi %60

# Nerede: Yazı sayfası, önizleme kartında Belge rolünün kendi fontuyla çizilen örnek satırı (src/typography.rs). Anlam: Belge fontuyla çizilen örnek cümle (Türkçe pangram).
settings-typography-sample-document = Pijamalı hasta yağız şoföre çabucak güvendi.

# Nerede: Yazı sayfası, font seçicinin başlık satırındaki örnek; metin rollerinde ailenin kendi fontuyla çizilir (src/pages/fonts.rs). Anlam: Türkçe harfleri (Ğ, ğ, İ, ı) sınamaya yarayan örnek metin.
settings-fonts-sample-text = Ağaç Şiir Ğğ İı 0123

## Bildirim uygulamaları, bar önizlemesi ve küçük sayfalar

# Nerede: Paneller sayfası, panel kartındaki "Bar'dan açma" satırı; yatay barda paneli sol uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-left-left-click = Sol uçtaki widget{ $widget } · sol tık

# Nerede: Aynı satır; yatay barda paneli sol uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-left-right-click = Sol uçtaki widget{ $widget } · sağ tık

# Nerede: Aynı satır; yatay barda paneli sağ uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-right-left-click = Sağ uçtaki widget{ $widget } · sol tık

# Nerede: Aynı satır; yatay barda paneli sağ uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-right-right-click = Sağ uçtaki widget{ $widget } · sağ tık

# Nerede: Aynı satır; dikey barda paneli üst uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-top-left-click = Üst uçtaki widget{ $widget } · sol tık

# Nerede: Aynı satır; dikey barda paneli üst uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-top-right-click = Üst uçtaki widget{ $widget } · sağ tık

# Nerede: Aynı satır; dikey barda paneli alt uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-bottom-left-click = Alt uçtaki widget{ $widget } · sol tık

# Nerede: Aynı satır; dikey barda paneli alt uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-bottom-right-click = Alt uçtaki widget{ $widget } · sağ tık

# Nerede: Bar sayfası, bar önizlemesinin altındaki ipucu satırı; barda widget varken (src/pages/bar_preview.rs). Anlam: Önizlemede bir widget'ın seçilince ayarlarının aşağıda açıldığı.
settings-bar-preview-hint = Bir widget'a tıklayınca ayarları aşağıda açılır

# Nerede: Bar sayfası, Seçili widget kartındaki yer satırı; seçili widget barın hiçbir bölümünde değilken (src/pages/bar_preview.rs). Anlam: Widget'ın barda olmadığı, Widget'lar kartından eklenebileceği.
settings-bar-preview-location-not-in-bar = Bar'da değil · Widget'lar kartından eklenebilir

# Nerede: Bar sayfası, Seçili widget kartındaki yer satırı; widget yatay barın baş ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın sol ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-left-end = Bar'ın sol ucunda · tıklama ve tekerlek de ayarlanabilir

# Nerede: Aynı yer satırı; widget yatay barın son ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın sağ ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-right-end = Bar'ın sağ ucunda · tıklama ve tekerlek de ayarlanabilir

# Nerede: Aynı yer satırı; widget dikey barın baş ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın üst ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-top-end = Bar'ın üst ucunda · tıklama ve tekerlek de ayarlanabilir

# Nerede: Aynı yer satırı; widget dikey barın son ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın alt ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-bottom-end = Bar'ın alt ucunda · tıklama ve tekerlek de ayarlanabilir

# Nerede: Aynı yer satırı; widget baş bölümünde ama uçta değilken, bar yatayken (src/pages/bar_preview.rs). Anlam: Widget'ın barın sol tarafında olduğu.
settings-bar-preview-location-left = Bar'ın solunda

# Nerede: Aynı yer satırı; widget baş bölümünde ama uçta değilken, bar dikeyken (src/pages/bar_preview.rs). Anlam: Widget'ın barın üst tarafında olduğu.
settings-bar-preview-location-top = Bar'ın üstünde

# Nerede: Aynı yer satırı; widget orta bölümdeyken (src/pages/bar_preview.rs). Anlam: Widget'ın barın ortasında olduğu.
settings-bar-preview-location-center = Bar'ın ortasında

# Nerede: Aynı yer satırı; widget son bölümde ama uçta değilken, bar yatayken (src/pages/bar_preview.rs). Anlam: Widget'ın barın sağ tarafında olduğu.
settings-bar-preview-location-right = Bar'ın sağında

# Nerede: Aynı yer satırı; widget son bölümde ama uçta değilken, bar dikeyken (src/pages/bar_preview.rs). Anlam: Widget'ın barın alt tarafında olduğu.
settings-bar-preview-location-bottom = Bar'ın altında

# Nerede: Bar sayfası, Seçili widget kartının başlığı; barda hiç widget yokken (src/pages/bar_preview.rs). Anlam: Barın boş olduğu.
settings-bar-preview-empty-title = Bar'da widget yok

# Nerede: Bar sayfası, Seçili widget kartının alt yazısı; barda hiç widget yokken (src/pages/bar_preview.rs). Anlam: Widget'ların Widget'lar kartından eklenebileceği.
settings-bar-preview-empty-place = Widget'lar kartından eklenebilir

# Nerede: Bar sayfası, Seçili widget kartındaki yer satırının sonu; widget uçta değil ve hiçbir ayarı yokken (src/pages/bar_preview.rs). Anlam: Yer bilgisine widget'ın ayarı olmadığı bilgisinin eklendiği.
# Değişkenler: $place (widget'ın bar'daki yeri, ör. "Bar'ın ortasında").
settings-bar-preview-place-no-settings = { $place } · Bu widget'ın ayarı yok
## Servis durum ve hata metinleri

# Nerede: Görünüm sayfasındaki durum kartı; seçimler shell tarafından uygulamalara aktarılırken (src/services/look.rs). Anlam: Görünümün uygulamalara aktarılmakta olduğu.
settings-look-status-applying = Uygulanıyor…

# Nerede: Görünüm sayfasındaki durum kartı; seçimler tüm uygulamalara aktarıldığında (src/services/look.rs). Anlam: Görünümün uygulandığı.
settings-look-status-applied = Uygulandı.

# Nerede: Görünüm sayfasındaki durum kartında eksik paket cümlenin sonunda; AUR paketi var ama paru ya da yay kurulu değilken (src/services/look.rs). Anlam: AUR paketini kurmak için bir yardımcının gerektiği.
settings-look-status-aur-helper-note = AUR yardımcısı gerekir (paru ya da yay).

# Nerede: Ayar kartındaki geri yükleme sonucunun hata parantezi içinde; kabuğun bildirdiği hata türü okunamadığında (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-error içinde $kind olarak görünür.
settings-snapshots-error-unknown-kind = bilinmeyen

## Erişilebilirlik (Kişisel grubu; büyüteç magnifier-v1, ekran klavyesi osk-v1)

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi (Kişisel grubu). Anlam: Görme, büyüteç ve ekran klavyesi ayarlarını toplayan sayfanın adı.
settings-page-accessibility = Erişilebilirlik

# Nerede: Erişilebilirlik sayfasının ilk bölüm başlığı (src/registry.rs). Anlam: Görmeyi kolaylaştıran ayarlar (yüksek karşıtlık, imleç ve yazı boyutu, hareketi azalt).
settings-section-seeing = Görme

# Nerede: Erişilebilirlik › Büyüteç kartındaki aç/kapa satırının başlığı (src/pages/assist.rs). Anlam: Ekran büyüteci.
settings-magnifier-live-label = Büyüteç

# Nerede: Erişilebilirlik › Büyüteç satırının alt metni; durum henüz okunmadan görünür. Anlam: Büyütecin ne yaptığı.
settings-magnifier-live-description = Ekranın imleç çevresini büyütür

# Nerede: Ayarlar araması; Büyüteç satırının arama sözcükleri (virgülle ayrılır). Anlam: Kullanıcının bu ayarı bulmak için yazabileceği sözcükler.
settings-magnifier-live-keywords = büyüteç, yakınlaştır, zoom, magnifier, görme, büyüt

# Nerede: Erişilebilirlik › Büyüteç kartında kaydırıcı başlığı. Anlam: Büyüteç açıldığında ekranın kaç kat büyütüleceği.
settings-magnifier-factor-label = Açılış büyütmesi

# Nerede: Açılış büyütmesi satırının alt metni (kabuk açıklama vermezse). Anlam: Büyüteç açılınca kullanılan büyütme oranı.
settings-magnifier-factor-description = Büyüteç açılınca kullanılan büyütme

# Nerede: Ayarlar araması; Açılış büyütmesi satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-magnifier-factor-keywords = büyüteç, çarpan, zoom, factor

# Nerede: Erişilebilirlik › Büyüteç kartında kaydırıcı başlığı. Anlam: Her yakınlaştırma ya da uzaklaştırma adımının büyüklüğü.
settings-magnifier-step-label = Adım

# Nerede: Adım satırının alt metni (kabuk açıklama vermezse). Anlam: Her yakınlaştırmada büyütmenin bu sayıyla çarpıldığı.
settings-magnifier-step-description = Her yakınlaştırma büyütmeyi bu sayıyla çarpar

# Nerede: Ayarlar araması; Adım satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-magnifier-step-keywords = büyüteç, adım, zoom, step

# Nerede: Erişilebilirlik › Büyüteç kartında kaydırıcı başlığı. Anlam: Yakınlaştırmanın çıkabileceği en büyük oran.
settings-magnifier-max-label = En fazla büyütme

# Nerede: En fazla büyütme satırının alt metni. Anlam: Yakınlaştırmanın bu sınırda durduğu.
settings-magnifier-max-description = Yakınlaştırma bu sınırda durur

# Nerede: Ayarlar araması; En fazla büyütme satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-magnifier-max-keywords = büyüteç, sınır, zoom, max

# Nerede: Erişilebilirlik › Büyüteç kartında, Kısayollar sayfasına yönlendiren satırın başlığı; alt metinde tuşlar görünür. Anlam: Büyütecin klavye kısayolları.
settings-magnifier-keys-label = Kısayollar

# Nerede: Ayarlar araması; büyüteç kısayolları yönlendirme satırının arama sözcükleri. Anlam: Bu satırı bulduran sözcükler.
settings-magnifier-keys-keywords = büyüteç, kısayol, tuş, zoom

# Nerede: Erişilebilirlik › Ekran klavyesi kartındaki göster/gizle satırının başlığı (src/pages/assist.rs). Anlam: Ekranda çizilen sanal klavye.
settings-osk-live-label = Ekran klavyesi

# Nerede: Ekran klavyesi satırının alt metni; durum henüz okunmadan görünür. Anlam: Ekran klavyesinin ne için olduğu.
settings-osk-live-description = Fareyle ya da dokunmatik ekranla yazmak için

# Nerede: Ayarlar araması; Ekran klavyesi satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-osk-live-keywords = ekran klavyesi, sanal klavye, dokunmatik, osk, on-screen keyboard

# Nerede: Erişilebilirlik › Ekran klavyesi kartında düzen seçimi satırının başlığı. Anlam: Ekran klavyesinin tuş düzeni.
settings-osk-layout-label = Düzen

# Nerede: Düzen satırının alt metni (kabuk açıklama vermezse). Anlam: Otomatik seçeneğin masaüstündeki klavye düzenini izlediği.
settings-osk-layout-description = Otomatik: masaüstündeki klavye düzenini izler

# Nerede: Ayarlar araması; ekran klavyesi Düzen satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-osk-layout-keywords = ekran klavyesi, düzen, türkçe q, layout

# Nerede: Erişilebilirlik › Ekran klavyesi kartında kaydırıcı başlığı. Anlam: Ekran klavyesindeki tuşların yüksekliği.
settings-osk-key-height-label = Tuş yüksekliği

# Nerede: Tuş yüksekliği satırının alt metni. Anlam: Klavyenin boyunun ve genişliğinin tuş yüksekliğinden hesaplandığı.
settings-osk-key-height-description = Klavyenin boyu ve genişliği bundan

# Nerede: Ayarlar araması; Tuş yüksekliği satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-osk-key-height-keywords = ekran klavyesi, boyut, tuş

# Nerede: Erişilebilirlik › Ekran klavyesi kartında, Kısayollar sayfasına yönlendiren satırın başlığı; alt metinde tuşlar görünür. Anlam: Ekran klavyesini açıp kapatan kısayol.
settings-osk-keys-label = Kısayol

# Nerede: Ayarlar araması; ekran klavyesi kısayolu yönlendirme satırının arama sözcükleri. Anlam: Bu satırı bulduran sözcükler.
settings-osk-keys-keywords = ekran klavyesi, kısayol, tuş

# Nerede: Ekran klavyesi › Düzen seçeneği (auto). Anlam: Masaüstünde etkin klavye düzenini izler.
settings-choice-osk-layout-auto = Otomatik

# Nerede: Ekran klavyesi › Düzen seçeneği (tr); ayrıca açıkken satırın alt metninde. Anlam: Türkçe Q klavye düzeni.
settings-choice-osk-layout-tr = Türkçe Q

# Nerede: Ekran klavyesi › Düzen seçeneği (us); ayrıca açıkken satırın alt metninde. Anlam: ABD İngilizce klavye düzeni.
settings-choice-osk-layout-us = İngilizce (ABD)

# Nerede: Erişilebilirlik › Büyüteç satırının alt metni, büyüteç açıkken (src/pages/assist.rs). Anlam: Büyütecin açık olduğu ve şu anki büyütme oranı.
# Değişkenler: $factor (biçimlenmiş oran, ör. 2,5).
settings-assist-magnifier-on = Açık · { $factor }× büyütme

# Nerede: Erişilebilirlik › Ekran klavyesi satırının alt metni, klavye açıkken (src/pages/assist.rs). Anlam: Klavyenin açık olduğu ve kullandığı düzen.
# Değişkenler: $layout (düzen adı, ör. Türkçe Q).
settings-assist-osk-on = Açık · { $layout }

# Nerede: Erişilebilirlik › Ekran klavyesi satırının alt metni, düzen bilinmiyorken açık durumda. Anlam: Açık.
settings-assist-on = Açık

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni, kapalıyken. Anlam: Kapalı.
settings-assist-off = Kapalı

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni; kabuk aracın bu masaüstünde çalışmadığını bildirince (anahtar devre dışı). Anlam: Bu masaüstü bu aracı desteklemiyor.
settings-assist-unsupported = Bu masaüstünde kullanılamıyor

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni; masaüstü kabuğu yanıt vermeyince (anahtar devre dışı). Anlam: Nuxut masaüstü kabuğu çalışmıyor.
settings-assist-no-shell = Masaüstü kabuğu çalışmıyor

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni; açma/kapama isteği gönderilmiş, yanıt bekleniyor. Anlam: İşlem sürüyor.
settings-assist-waiting = Bekleniyor…

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni (hata renginde); kabuk açma/kapama isteğini yerine getiremedi. Anlam: Değişiklik yapılamadı.
settings-assist-failed = Değiştirilemedi

# Nerede: Ekran okuyucu (Orca) — başlık çubuğundaki geri al düğmesinin adı; görünmez, sesli okunur. Anlam: son ayar değişikliğini geri alır.
settings-a11y-undo = Geri al

# Nerede: Ekran okuyucu — başlık çubuğundaki geçmiş düğmesinin adı; görünmez, sesli okunur. Anlam: son değişiklikler listesini açar.
settings-a11y-history = Son değişiklikler

# Nerede: Ekran okuyucu — başlık çubuğundaki pencereyi küçült düğmesi; görünmez, sesli okunur.
settings-a11y-minimize = Küçült

# Nerede: Ekran okuyucu — başlık çubuğundaki pencereyi büyüt/eski boyutuna getir düğmesi; görünmez, sesli okunur.
settings-a11y-maximize = Büyüt

# Nerede: Ekran okuyucu — başlık çubuğundaki pencereyi kapat düğmesi; görünmez, sesli okunur.
settings-a11y-close = Kapat

# Nerede: Ekran okuyucu — kenar çubuğundaki ayar arama alanının adı; görünmez, sesli okunur.
settings-a11y-search = Ayarlarda ara

# Nerede: Ekran okuyucu — kenar çubuğundaki sayfa listesinin adı; görünmez, sesli okunur.
settings-a11y-nav = Sayfalar

# Nerede: Ekran okuyucu — kenar çubuğundaki arama sonuçları listesinin adı; görünmez, sesli okunur.
settings-a11y-results = Arama sonuçları

# Nerede: Ekran okuyucu — ayar satırındaki varsayılana döndür düğmesinin adı; görünmez, sesli okunur. Anlam: bu ayarı varsayılan değerine sıfırlar.
# Değişkenler: $setting (ayarın görünen adı, örnek: "Bar yüksekliği").
settings-a11y-reset = { $setting }: varsayılana döndür

## Güç: Kilit ve boşta

# Nerede: Güç sayfasında kart başlığı. Anlam: Ekran kilidiyle ilgili ayarlar (uykudan önce kilit, hangi kilit ekranı).
settings-section-lock = Kilit

# Nerede: Güç sayfasında kart başlığı. Anlam: Bilgisayar bir süre kullanılmayınca yapılacaklar (karart, ekranı kapat, uyu).
settings-section-idle = Boşta kalınca

# Nerede: Güç › Boşta kalınca kartının ana anahtarı (idle.enabled). Anlam: Kapalıyken Nuxut hiçbir boşta eylemini yapmaz, uykudan önce de kilitlemez.
settings-idle-enabled-label = Boşta eylemlerini Nuxut yönetsin

# Nerede: idle.enabled satırının alt yazısı. Anlam: Kapalıyken uykudan önce kilit dahil hiçbir eylemi Nuxut yapmaz; başka bir araç (ör. hypridle) yönetebilir.
settings-idle-enabled-description = Kapalıyken uykudan önce kilit de çalışmaz; başka bir araç yönetebilir

# Nerede: idle.enabled satırının arama sözcükleri. Anlam: Ana anahtarı aramada bulmayı sağlar.
settings-idle-enabled-keywords = boşta, kilit, hypridle, idle, uyku, otomatik

# Nerede: Güç › Kilit kartı, idle.lock_before_sleep anahtarı. Anlam: Bilgisayar uyumadan önce oturum kilitlenir.
settings-idle-lock-before-sleep-label = Uykudan önce kilitle

# Nerede: idle.lock_before_sleep satırının alt yazısı. Anlam: Uyandığında masaüstü kilitli gelir.
settings-idle-lock-before-sleep-description = Bilgisayar uyandığında masaüstü kilitli gelir

# Nerede: idle.lock_before_sleep satırının arama sözcükleri.
settings-idle-lock-before-sleep-keywords = kilit, uyku, askıya alma, kapak, suspend, lock, güvenlik

# Nerede: idle.lock_before_sleep satırının altında, uyarı renginde; anahtar açık ama "Boşta eylemlerini Nuxut yönetsin" kapalı. Anlam: Ana anahtar kapalı olduğu için uykudan önce kilit çalışmıyor.
settings-idle-lock-before-sleep-off-note = Boşta eylemleri kapalı olduğu için uykudan önce kilitlenmiyor

# Nerede: Güç › Kilit kartı, lock.listen_logind anahtarı. Anlam: Kilit istendiğinde Nuxut'un kendi kilit ekranı açılır.
settings-lock-listen-logind-label = Nuxut kilit ekranını kullan

# Nerede: lock.listen_logind satırının alt yazısı (kabuk açıklama vermezse). Anlam: Kilitle düğmesi, kısayol ve otomatik kilit Nuxut kilit ekranını açar; başka bir kilit programı kullanılıyorsa kapatılmalı.
settings-lock-listen-logind-description = Kilitle düğmesi, kısayolu ve otomatik kilit Nuxut kilit ekranını açar; başka bir kilit aracıyla birlikte kapatılır

# Nerede: lock.listen_logind satırının arama sözcükleri.
settings-lock-listen-logind-keywords = kilit ekranı, kilitleyici, hyprlock, swaylock, loginctl, logind, lock

# Nerede: Güç › Boşta kalınca kartı, idle.lock anahtarı. Anlam: Bilgisayar bir süre kullanılmayınca oturum kilitlenir.
settings-idle-lock-label = Boşta kilitle

# Nerede: idle.lock satırının alt yazısı. Anlam: Belirlenen süre boşta kalınca oturum kilitlenir.
settings-idle-lock-description = Belirlenen süre boşta kalınca oturum kilitlenir

# Nerede: idle.lock satırının arama sözcükleri.
settings-idle-lock-keywords = kilit, boşta, otomatik kilit, ekran kilidi, lock

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.lock_after). Anlam: Kaç dakika boşta kalınca kilitleneceği.
settings-idle-lock-after-label = Kilitleme süresi

# Nerede: idle.lock_after satırının arama sözcükleri.
settings-idle-lock-after-keywords = kilit, süre, boşta

# Nerede: Güç › Boşta kalınca kartı, idle.dim anahtarı. Anlam: Boşta kalınca ekran karartılır.
settings-idle-dim-label = Ekranı karart

# Nerede: idle.dim satırının arama sözcükleri.
settings-idle-dim-keywords = karart, boşta, ekran, parlaklık, dim

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.dim_after). Anlam: Kaç dakika sonra karartılacağı.
settings-idle-dim-after-label = Karartma süresi

# Nerede: idle.dim_after satırının arama sözcükleri.
settings-idle-dim-after-keywords = karart, süre, boşta

# Nerede: Güç › Boşta kalınca kartı, idle.screen_off anahtarı. Anlam: Boşta kalınca monitörler kapatılır.
settings-idle-screen-off-label = Ekranı kapat

# Nerede: idle.screen_off satırının arama sözcükleri.
settings-idle-screen-off-keywords = ekranı kapat, boşta, monitör, dpms, ekran

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.screen_off_after). Anlam: Kaç dakika sonra ekranın kapatılacağı.
settings-idle-screen-off-after-label = Ekranı kapatma süresi

# Nerede: idle.screen_off_after satırının arama sözcükleri.
settings-idle-screen-off-after-keywords = ekranı kapat, süre, boşta

# Nerede: Güç › Boşta kalınca kartı, idle.suspend anahtarı. Anlam: Boşta kalınca bilgisayar uyku kipine geçer.
settings-idle-suspend-label = Uyu

# Nerede: idle.suspend satırının alt yazısı. Anlam: Varsayılan olarak yalnızca pille çalışırken uyunur.
settings-idle-suspend-description = Varsayılan olarak yalnızca pildeyken

# Nerede: idle.suspend satırının arama sözcükleri.
settings-idle-suspend-keywords = uyku, askıya alma, boşta, suspend, otomatik uyku

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.suspend_after). Anlam: Kaç dakika sonra uyunacağı.
settings-idle-suspend-after-label = Uyku süresi

# Nerede: idle.suspend_after satırının arama sözcükleri.
settings-idle-suspend-after-keywords = uyku, süre, boşta

# Nerede: Güç › Boşta kalınca kartı, idle.suspend_on_ac anahtarı. Anlam: Bilgisayar prize takılıyken de boşta uyusun.
settings-idle-suspend-on-ac-label = Prizdeyken de uyu

# Nerede: idle.suspend_on_ac satırının arama sözcükleri.
settings-idle-suspend-on-ac-keywords = uyku, priz, şarj, ac

# Nerede: Güç › Boşta kalınca kartı, idle.media_inhibits anahtarı. Anlam: Müzik ya da video çalarken boşta eylemleri beklesin.
settings-idle-media-inhibits-label = Medya oynarken bekle

# Nerede: idle.media_inhibits satırının alt yazısı. Anlam: Müzik ya da video çalarken ekran kararmaz ve kilitlenmez.
settings-idle-media-inhibits-description = Müzik ya da video çalarken ekran kararmaz, kilitlenmez

# Nerede: idle.media_inhibits satırının arama sözcükleri.
settings-idle-media-inhibits-keywords = medya, müzik, video, film, boşta, engelle

## Renk teması kartları (src/pages/color_theme.rs; framework `[colors] theme` adları)

# Nerede: Görünüm › Renkler, Renk teması kartı. Anlam: varsayılan serin mavi tema.
settings-color-theme-blue = Nuxut mavisi
# Nerede: Aynı kartlar. Anlam: sıcak taş/kum grisi, zeytin dokunuşlu tema.
settings-color-theme-neutral = Nötr
# Nerede: Aynı kartlar. Anlam: griye çalan yumuşak yeşil tema.
settings-color-theme-sage = Adaçayı
# Nerede: Aynı kartlar. Anlam: tozlu pembe tema.
settings-color-theme-rose = Gül kurusu
# Nerede: Aynı kartlar. Anlam: bal sarısı vurgulu tema.
settings-color-theme-amber = Kehribar
# Nerede: Aynı kartlar. Anlam: neredeyse renksiz serin gri tema.
settings-color-theme-graphite = Grafit
# Nerede: Renk teması seçilince geri al geçmişindeki adım. Anlam: hangi temaya geçildiği; $theme temanın görünen adı (ör. Adaçayı).
settings-color-theme-changed = Renk teması: { $theme }
