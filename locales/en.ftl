## Hesap listesi

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Hesabın hiç senkronlanmamış olduğu.
settings-accounts-sync-never = Not synced yet

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamanın az önce yapıldığı.
settings-accounts-sync-just-now = Synced just now

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamadan bu yana geçen dakika sayısı.
# Değişkenler: $count (dakika sayısı, ör. 5).
settings-accounts-sync-minutes-ago = Synced { $count } min ago

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamadan bu yana geçen saat sayısı.
# Değişkenler: $count (saat sayısı, ör. 3).
settings-accounts-sync-hours-ago = Synced { $count } h ago

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Son senkronlamadan bu yana geçen gün sayısı.
# Değişkenler: $count (gün sayısı, ör. 2).
settings-accounts-sync-days-ago = Synced { $count } d ago

# Nerede: Hesaplar sayfası, hesap listesi boşken maskotun altındaki başlık. Anlam: henüz hesap eklenmediği.
settings-accounts-empty-title = No accounts yet

# Nerede: Hesaplar sayfası, hesap listesi boşken başlığın altındaki açıklama. Anlam: hesap eklemenin ne sağladığı.
settings-accounts-empty-body = Add an account to see your calendars in the bar and in notifications.

# Nerede: Hesaplar sayfası, takvim servisine ulaşılamadığında maskotun altındaki başlık; altında hatanın kendisi yazar. Anlam: hesaplar şu an gösterilemiyor ve değiştirilemiyor.
settings-accounts-unavailable-title = Calendar accounts cannot be managed right now

# Nerede: Hesaplar sayfası, listenin altındaki dolu düğme. Anlam: Sağlayıcı seçim ekranını açar.
settings-accounts-add = Add account

# Nerede: Hesaplar sayfası, hesap kartının alt satırı. Anlam: Sağlayıcı adı ile giriş e-postasının birleşimi; parça parça kurulduğu için tek mesaj.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google"), $login (giriş e-postası, ör. "ad@gmail.com").
settings-accounts-card-sub = { $provider } · { $login }

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Hesabın bağlantısının koptuğu, yeniden bağlanması gerektiği.
settings-accounts-state-needs-reconnect = Reconnection required

# Nerede: Hesaplar sayfası, hesap kartının durum satırı. Anlam: Hesabın kapalı (devre dışı) olduğu.
settings-accounts-state-disabled = Off

# Nerede: Hesaplar sayfası, hesap kartının durum satırı ("Şimdi senkronla" sırasında da). Anlam: Senkronlamanın sürdüğü.
settings-accounts-state-syncing = Syncing…

# Nerede: Hesaplar sayfası, hesap kartının durum satırı (src/pages/accounts.rs). Anlam: Son senkronlamanın başarısız olduğu; daemon bir hata kodu verdi ama ileti yok.
settings-accounts-sync-failed = Last sync failed

# Nerede: Hesaplar sayfası, bağlantısı kopan hesap kartındaki dolu düğme. Anlam: Yeniden bağlanma akışını başlatır.
settings-accounts-button-reconnect = Reconnect

# Nerede: Hesaplar sayfası, adı olmayan takvimlerin satırındaki ad. Anlam: Takvimin adı boşsa kullanılan genel ad.
settings-accounts-calendar-default-name = Calendar

# Nerede: Hesaplar sayfası, takvim satırının renk seçicisi açılınca görünen metin düğme. Anlam: Takvimin rengini sağlayıcının belirlediği renge döndürür.
settings-accounts-calendar-color-reset = Provider color

# Nerede: Hesaplar sayfası, hesap kartındaki sil ikonuna ilk basışta durum satırında çıkan metin. Anlam: Kaldırmanın ikinci bir tık daha istediği.
settings-accounts-remove-confirm = Click again to remove

# Nerede: Hesaplar sayfası, hesap listesi okunurken görünen metin. Anlam: Hesapların yüklenmekte olduğu.
settings-accounts-loading = Loading accounts…

# Nerede: Hesaplar sayfası, hesap kartının çizimdeki birleşik alt satırı. Anlam: Alt bilgi ile durumun tek satırda birleşimi; parça parça kurulduğu için tek mesaj.
# Değişkenler: $sub (hesabın alt bilgisi, ör. "Google · ad@gmail.com"), $status (hesap durumu, ör. "Senkronlanıyor…").
settings-accounts-card-sub-status = { $sub } · { $status }

## Sağlayıcı seçimi

# Nerede: Hesaplar sayfası, "Hesap ekle" ekranındaki Google kartının alt satırı. Anlam: Google hesabının kapsadığı hizmetler.
settings-accounts-provider-google-description = Gmail and Google Calendar

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki Microsoft kartının alt satırı. Anlam: Microsoft hesabının kapsadığı hizmetler.
settings-accounts-provider-microsoft-description = Outlook, Hotmail, Microsoft 365

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki Nextcloud kartının alt satırı. Anlam: Kendi sunucusunu kullananların onayı tarayıcıda yapılır.
settings-accounts-provider-nextcloud-description = Your own server — confirm in the browser

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki CalDAV kartının başlığı; .description kartın altındaki açıklama. Anlam: CalDAV uyumlu diğer sağlayıcılar.
settings-accounts-provider-caldav = iCloud, Fastmail and others
    .description = CalDAV — email and app password

# Nerede: Hesaplar sayfası, sağlayıcı kartlarındaki abonelik kartının başlığı; .description kartın altındaki açıklama. Anlam: Salt okunur webcal/.ics takvim aboneliği.
settings-accounts-provider-subscription = Calendar subscription
    .description = A webcal:// or .ics address (read-only)

# Nerede: Hesaplar sayfası, bilinmeyen sağlayıcıya düşen kart ve hesapların başlığı. Anlam: Sağlayıcının tanınamadığı.
settings-accounts-provider-unknown = Unknown

## Hesap ekleme akışı

# Nerede: Hesaplar sayfası, sağlayıcı ve akış ekranlarının sol üstündeki metin düğme. Anlam: Önceki ekrana döner.
settings-accounts-back = Back

# Nerede: Hesaplar sayfası, yüklenemediğinde görünen ve akışın hata ekranındaki düğme. Anlam: İsteği yeniden dener.
settings-accounts-retry = Try again

# Nerede: Akış ekranının altındaki birincil düğme. Anlam: Girdiyi gönderip sonraki adıma geçer.
settings-accounts-continue = Continue

# Nerede: Akış ekranının altındaki metin düğme. Anlam: Akışı iptal eder.
settings-accounts-cancel = Cancel

# Nerede: Akış ekranının altındaki tonlu düğme (tarayıcı ve uygulama şifresi adımlarında). Anlam: Tarayıcı sayfasını ya da şifre oluşturma sayfasını yeniden açar.
settings-accounts-reopen-link = Reopen link

# Nerede: Akışın Nextcloud sunucu adımındaki soru. Anlam: Sunucu adresi isteniyor.
settings-accounts-flow-server-question = What is your Nextcloud server address?

# Nerede: Akışın Nextcloud sunucu adımındaki ipucu. Anlam: Adrese örnek verir.
settings-accounts-flow-server-hint = For example cloud.example.com — the address you sign in to in your browser.

# Nerede: Akışın Nextcloud sunucu adımındaki giriş alanının yer tutucusu. Anlam: Örnek sunucu adresi.
settings-accounts-flow-server-placeholder = cloud.example.com

# Nerede: Akışın e-posta adımındaki soru. Anlam: Hesabın e-posta adresi isteniyor.
settings-accounts-flow-email-question = What is your email address?

# Nerede: Akışın e-posta adımındaki ipucu. Anlam: Sunucunun kendiliğinden bulunacağı.
settings-accounts-flow-email-hint = We'll find the server for you.

# Nerede: Akışın e-posta adımındaki giriş alanının yer tutucusu. Anlam: Örnek e-posta adresi.
settings-accounts-flow-email-placeholder = you@example.com

# Nerede: Akışın uygulama şifresi adımındaki soru. Anlam: Sağlayıcıya özel uygulama şifresi isteniyor.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google").
settings-accounts-flow-password-question = { $provider } app password

# Nerede: Akışın uygulama şifresi adımındaki ipucu. Anlam: Uygulama şifresi ile hesap şifresinin farkını anlatır.
settings-accounts-flow-password-hint = Not your account password: create an app password for this device on the provider's security page and paste it here.

# Nerede: Akışın uygulama şifresi adımındaki parola alanının yer tutucusu. Anlam: Uygulama şifresinin beklenen biçimini gösterir.
settings-accounts-flow-password-placeholder = xxxx-xxxx-xxxx-xxxx

# Nerede: Akışın uygulama şifresi adımında, sunucu bilinmiyorsa çıkan ikinci alanın yer tutucusu. Anlam: CalDAV sunucu adresi isteniyor.
settings-accounts-flow-password-server-placeholder = Server address (caldav.example.com)

# Nerede: Akışın uygulama şifresi adımındaki tonlu düğme. Anlam: Sağlayıcının uygulama şifresi sayfasını açar.
settings-accounts-flow-password-link = Create an app password

# Nerede: Akışın takvim adresi (abonelik) adımındaki soru. Anlam: Abone olunacak takvimin adresi isteniyor.
settings-accounts-flow-url-question = What is the address of your calendar?

# Nerede: Akışın takvim adresi adımındaki ipucu. Anlam: Kabul edilen adres biçimini anlatır.
settings-accounts-flow-url-hint = A .ics link starting with webcal:// or https://.

# Nerede: Akışın takvim adresi adımındaki giriş alanının yer tutucusu. Anlam: Örnek adres biçimi.
settings-accounts-flow-url-placeholder = webcal://…

# Nerede: Akışın tarayıcı onayı adımındaki soru. Anlam: Kullanıcıdan tarayıcıya geçmesi isteniyor.
settings-accounts-flow-browser-question = Continue in your browser

# Nerede: Akışın tarayıcı onayı adımındaki ipucu. Anlam: Tarayıcıda açılan sayfada ne yapılacağı ve pencerenin kendiliğinden ilerleyeceği anlatılır.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google").
settings-accounts-flow-browser-hint = We opened a page in your browser for { $provider }. Sign in and grant access; this window will continue on its own.

# Nerede: Akışın sunucu onayı adımındaki soru. Anlam: Yönlendirilen sunucuyla devam edilip edilmeyeceği soruluyor.
settings-accounts-flow-confirm-server-question = Continue with this server?

# Nerede: Akışın sunucu onayı adımındaki ipucu. Anlam: Yönlendirme ve şifrenin henüz gönderilmediği anlatılır.
settings-accounts-flow-confirm-server-hint = The server redirected you to another address. Your password has not been sent yet; continue if you recognize the address.

# Nerede: Akışın sunucu onayı adımındaki birincil düğme. Anlam: Onayı gönderir.
settings-accounts-flow-confirm-yes = Yes, continue

# Nerede: Akışın abonelik onayı adımındaki soru. Anlam: Adresi verilen takvime abone olunup olunmayacağı soruluyor; $host sunucu adresi ya da "Bu" olur.
# Değişkenler: $host (sunucu adresi, ör. "bulut.ornek.com").
settings-accounts-flow-subscribe-question = Subscribe to the calendar on { $host }?

# Nerede: Akışın abonelik onayı adımındaki soru, sunucu adresi bilinmiyorken (src/pages/accounts.rs). Anlam: Adresi bilinmeyen takvime abone olunup olunmayacağı soruluyor.
settings-accounts-flow-subscribe-question-unnamed = Subscribe to this calendar?

# Nerede: Akışın abonelik onayı adımındaki ipucu. Anlam: Aboneliğin salt okunur olduğu anlatılır.
settings-accounts-flow-subscribe-hint = Read-only subscription: events appear in the bar calendar and cannot be changed here.

# Nerede: Akışın abonelik onayı adımındaki birincil düğme. Anlam: Aboneliği başlatır.
settings-accounts-flow-subscribe-confirm = Subscribe

# Nerede: Akışın bağlanma adımındaki soru. Anlam: Bağlantı kuruluyor.
settings-accounts-flow-connecting = Connecting…

# Nerede: Akışın bağlanma adımındaki ipucu. Anlam: Ne yapıldığını söyler.
settings-accounts-flow-connecting-hint = Verifying your account.

# Nerede: Akışın keşif adımındaki soru. Anlam: Takvimler aranıyor.
settings-accounts-flow-discovering = Finding calendars…

# Nerede: Akışın keşif adımındaki ipucu (takvim bulununca). Anlam: Bulunan takvim sayısı.
# Değişkenler: $count (bulunan takvim sayısı, ör. 3).
settings-accounts-flow-found =
    { $count ->
        [one] { $count } calendar found
       *[other] { $count } calendars found
    }

# Nerede: Akışın keşif adımındaki ipucu (henüz sonuç yokken). Anlam: İşlemin birkaç saniye sürebileceği.
settings-accounts-flow-discovering-hint = This may take a few seconds.

# Nerede: Akışın bitiş adımındaki soru. Anlam: Akışın tamamlandığı.
settings-accounts-flow-done = All set!

# Nerede: Akışın bitiş adımındaki ipucu. Anlam: Eklenen takvim sayısı ve nerede görüneceği.
# Değişkenler: $count (eklenen takvim sayısı, ör. 2).
settings-accounts-flow-done-hint =
    { $count ->
        [one] { $count } calendar added. It will appear in the bar calendar.
       *[other] { $count } calendars added. They will appear in the bar calendar.
    }

# Nerede: Akışın bitiş adımındaki birincil düğme. Anlam: Akışı kapatır.
settings-accounts-flow-done-button = Done

# Nerede: Akışın hata adımındaki soru (başarısızlık başlığı). Anlam: İşlemin başarısız olduğu.
settings-accounts-flow-failed = That didn't work

# Nerede: Akışın hata adımındaki hata satırı (akışın kendi mesajı yoksa). Anlam: Genel hata bildirimi.
settings-accounts-flow-error-generic = Something went wrong.

# Nerede: Hesap kartındaki yeniden bağlanma akışı başlarken soru satırında çıkan metin. Anlam: Akışın başlatılmakta olduğu.
settings-accounts-flow-starting = Starting…

# Nerede: Sağlayıcı seçilince akış ekranında soru satırında çıkan metin. Anlam: Seçilen sağlayıcının hesap ekleme akışı başlatılıyor.
# Değişkenler: $provider (sağlayıcının görünen adı, ör. "Google").
settings-accounts-flow-adding = Adding { $provider } account…

## Hakkında sayfası

# Nerede: Hakkında sayfası, düzenlenen dosya listesinde framework dosyasının satır başlığı. Anlam: Görünüm ve hareket ayarlarını içeren dosya.
settings-about-file-look = Appearance and motion

# Nerede: Hakkında sayfasındaki dosya satırının başlığı (src/pages/about.rs). Anlam: shell.toml dosyasının görünen adı.
settings-about-file-shell = Shell

# Nerede: Hakkında sayfası, düzenlenen dosya listesinde üretilmiş palet dosyasının satır başlığı. Anlam: Duvar kağıdından üretilen renk şemasını içeren dosya.
settings-about-file-palette = Generated color scheme

# Nerede: Hakkında sayfası, her dosya satırının sağındaki metin düğme. Anlam: Dosyanın klasörünü dosya yöneticisinde açar.
settings-about-open-folder = Open folder

# Nerede: Hakkında sayfası, başlığın altındaki sürüm satırı. Anlam: Uygulamanın sürümü.
# Değişkenler: $version (sürüm numarası, ör. "0.4.2").
settings-about-version = Version { $version }

# Nerede: Hakkında sayfası, başlığın altındaki tanıtım paragrafı. Anlam: Değişikliklerin anında uygulandığı, dosyalara yalnızca değişen satırın yazıldığı ve geçmişten geri alınabildiği anlatılır.
settings-about-blurb = Every change is applied instantly and only the changed lines are written to the files; your comments and layout are preserved. Recent changes can be undone one by one from the history button in the title bar.

# Nerede: Hakkında sayfasının başlığında "Nuxut" adının altında ve Hoş geldin penceresinin karşılama ekranında logonun altında. Anlam: Nuxut marka sloganı (Kara Sağan logosuyla birlikte). Marka sloganıdır; her dilde anlamı koruyan doğal bir çeviri yazılır, birebir çeviri şart değildir. "The penguin" tekil ve belirlidir; Linux maskotu Tux'a göndermedir, bu gönderme korunmalıdır.
settings-brand-slogan = The penguin can't fly. This one can.

## Yer tutucu satır

# Nerede: Kenar çubuğunda henüz yapılmamış sayfaların yer tutucu satırındaki not. Anlam: Sayfanın hazırlanmakta olduğu; $subtitle sayfanın alt başlığıdır.
# Değişkenler: $subtitle (sayfanın alt başlığı, ör. "Ses").
settings-placeholder-note = { $subtitle } — coming soon

## Önizleme

# Nerede: Görünüm sayfasındaki canlı önizlemede sahte bildirimin başlığı. Anlam: Panel içindeki örnek bildirim içeriği.
# [max: 12]
settings-preview-demo-title = Dinner

# Nerede: Görünüm sayfasındaki canlı önizlemede sahte bildirimin gövdesi. Anlam: Panel içindeki örnek bildirim içeriği.
settings-preview-demo-body = At eight, in Kadıköy

# Nerede: Görünüm sayfasındaki canlı önizlemedeki örnek düğme. Anlam: Panel içindeki düğme örneği.
# [max: 6]
settings-preview-demo-button = OK
## Tuval ve monitör seçici

# Nerede: Ekranlar sayfası, tuvaldeki monitör kutuları ve çoklu monitörde monitör seçme sekmeleri. Anlam: Monitörün kullanıcıya görünen adı; aynı adlı monitörlerde ayırt edici olarak bağlayıcı eklenir.
# Değişkenler: $name (monitör adı, ör. "LP27q"), $connector (bağlantı çıkışı, ör. "DP-1").
settings-displays-name-connector = { $name } ({ $connector })

# Nerede: Ekranlar sayfası, monitör sekmelerinin hemen altındaki ipucu satırı. Anlam: Monitörün bağlı olduğu çıkışa göre tanındığı; başka bir çıkışa takılırsa ayarlarının ayrı tutulacağı.
# Değişkenler: $connector (bağlantı çıkışı, ör. "DP-1").
settings-displays-hint-connector = This monitor is identified by the connector it is plugged into ({ $connector }); if it is moved to another connector, its settings are kept separate.

## Monitör ayar satırları

# Nerede: Ekranlar sayfası, seçili monitörün ilk ayar satırı; .description satırın altındaki açıklama. Anlam: Bu monitörü açıp kapatma.
settings-displays-enabled = Use this display
    .description = A display that is off shows no image

# Nerede: Ekranlar sayfası, "Yansıt" satırındaki seçim menüsünün ilk seçeneği. Anlam: Bu ekran hiçbir ekrana yansıtmıyor.
settings-choice-mirror-none = No mirroring

# Nerede: Ekranlar sayfası, yansıtma ayar satırı; .description satırın altındaki açıklama. Anlam: Seçilen başka ekrana yansıtma.
settings-displays-mirror = Mirror
    .description = This display shows the same image as the selected display

# Nerede: Ekranlar sayfası, birincil ekran anahtarı; .description satırın altındaki açıklama. Anlam: Birincil ekranın görevi.
settings-displays-primary = Primary display
    .description = The pointer and notifications appear here at login

# Nerede: Ekranlar sayfası, çözünürlük seçim menüsünde önerilen kipin etiketi. Anlam: Ekranın önerdiği çözünürlük.
# Değişkenler: $resolution (çözünürlük, ör. "1920 × 1080").
settings-choice-resolution-preferred = { $resolution } (recommended)

# Nerede: Ekranlar sayfası, çözünürlük ayar satırı. Anlam: Ekran çözünürlüğü seçimi.
settings-displays-resolution = Resolution

# Nerede: Ekranlar sayfası, yenileme hızı ayar satırı. Anlam: Ekranın saniyedeki kare yenileme sayısı.
settings-displays-rate = Refresh rate

# Nerede: Ekranlar sayfası, yenileme hızı seçim menüsündeki seçeneklerin biçimi. Anlam: Hertz cinsinden yenileme hızı.
# Değişkenler: $value (yenileme hızı değeri, ör. 60).
settings-choice-rate-hz = { $value } Hz

# Nerede: Ekranlar sayfası, ölçek ayar satırı; .description satırın altındaki açıklama. Anlam: Arayüz ölçekleme.
settings-displays-scale = Scale
    .description = Size of text and windows

# Nerede: Ekranlar sayfası, ölçek satırındaki çiplerin biçimi. Anlam: Yüzde olarak ölçek.
# Değişkenler: $value (ölçek yüzdesi, ör. 100).
settings-choice-scale-percent = %{ $value }

# Nerede: Ekranlar sayfası, döndürme ayar satırı. Anlam: Ekran görüntüsünün döndürülmesi.
settings-displays-rotation = Rotation

# Nerede: Ekranlar sayfası, döndürme seçim düğmelerinde döndürülmemiş durum. Anlam: Döndürme yok.
settings-choice-rotation-none = None

# Nerede: Ekranlar sayfası, yatay çevirme anahtarı; .description satırın altındaki açıklama. Anlam: Görüntünün aynalanması.
settings-displays-flip = Flip horizontally
    .description = Mirrors the image (for example, a rear-projection projector)

# Nerede: Ekranlar sayfası, VRR ayar satırı; .description satırın altındaki açıklama. Anlam: Değişken yenileme hızı (FreeSync, G-Sync).
settings-displays-vrr = Variable refresh rate
    .description = Reduces stuttering on displays that support it (FreeSync, G-Sync)

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR kapalı.
settings-choice-vrr-off = Off

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR her zaman açık.
settings-choice-vrr-on = Always

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR yalnızca tam ekran uygulamalarda açık.
settings-choice-vrr-fullscreen = In fullscreen

# Nerede: Ekranlar sayfası, VRR seçim menüsü. Anlam: VRR tam ekran oyun ve videolarda açık.
settings-choice-vrr-fullscreen-media = Fullscreen games and video

## Doğrulama uyarıları

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi (uygulamadan önce). Anlam: En az bir ekranın açık kalması gerektiği.
settings-displays-issue-no-active = At least one display must be on and showing its own image.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: İki monitörün yerleşimde üst üste biniyor olduğu.
# Değişkenler: $first (binen ilk monitörün adı, ör. "LP27q"), $second (binen ikinci monitörün adı, ör. "DP-1").
settings-displays-issue-overlap = { $first } and { $second } overlap.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: Yerleşimin tek parça olmadığı, monitörler arasında boşluk kaldığı.
settings-displays-issue-disconnected = Displays must touch; the pointer cannot cross the gap between them.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: Seçilen ölçeğin bu monitörün çözünürlüğünü tam bölmediği.
# Değişkenler: $display (monitörün adı, ör. "LP27q").
settings-displays-issue-invalid-scale = For { $display }, this scale does not divide the resolution evenly; pick a different scale from the list.

# Nerede: Ekranlar sayfası, tuvalin altındaki uyarı listesi. Anlam: Yansıtılan monitörün kapalı ya da kendisi yansıtma olduğu.
# Değişkenler: $display (monitörün adı, ör. "LP27q").
settings-displays-issue-bad-mirror = { $display } is off or mirroring another display.

## Durum çubuğu

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Değişiklik shell'e gönderildi, uygulanıyor.
settings-displays-status-applying = Applying…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (onay süresinin son saniyeleri). Anlam: Kullanıcıya yeni ayarların korunup korunmayacağı soruluyor.
settings-displays-status-awaiting = Keep the new settings?

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (onay geri sayımı). Anlam: Ayarların korunması için kalan süre; süre dolunca kendiliğinden geri alınacak.
# Değişkenler: $seconds (kalan saniye, ör. 10).
settings-displays-status-awaiting-countdown = Keep the new settings? Reverting in { $seconds }s.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Kullanıcı "Koru" dedi, kalıcı yazım sürüyor.
settings-displays-status-keeping = Keeping…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Kullanıcı "Geri al" dedi, önceki ayarlara dönülüyor.
settings-displays-status-reverting = Reverting…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Karar gönderildi, shell'den sonuç bekleniyor.
settings-displays-status-resolving = Waiting for the result…

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Monitörler değiştiği (takılıp çıktığı) için düzenleme yeni duruma göre sıfırlandı.
settings-displays-status-reset = The monitors changed; your edits were reset to the new state.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı. Anlam: Kullanıcının henüz uygulamadığı değişiklikler var.
settings-displays-status-unsaved = There are unapplied changes.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (boşta). Anlam: Görüntüyü değiştiren ayarların kısa sürede korunmazsa kendiliğinden geri alındığı.
settings-displays-status-hint = Settings that change the image are reverted automatically unless kept within a short time.

## Uygulama sonuçları

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Değişiklik uygulandı.
settings-displays-result-applied = Applied.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Kullanıcı korudu, yeni ayarlar kalıcı.
settings-displays-result-confirmed = The new display settings were kept.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Değişiklik geri alındı, önceki ayarlar geçerli.
settings-displays-result-reverted = Reverted; the previous settings are in effect.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Onay süresi doldu, önceki ayarlara dönüldü.
settings-displays-result-timeout = Time ran out; the change was not confirmed, so the previous settings were restored.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Monitör takılıp çıkarken değişiklik geri döndü.
settings-displays-result-hotplug = A display was connected or removed; the previous settings were restored.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu ve not_applied hatası. Anlam: Ekran bu ayarı kabul etmedi, önceki ayarlara dönüldü.
settings-displays-result-not-applied = The display did not accept this setting; the previous settings were restored.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Değişiklik gösterilemedi, önceki ayarlara dönüldü.
settings-displays-result-not-shown = The change could not be shown; the previous settings were restored.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in son işlem sonucu. Anlam: Hyprland ayarları kendisi yeniden yüklediği için değişiklik geri alındı.
settings-displays-result-reloaded = Hyprland reloaded its settings; the change was reverted.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; shell'in tanınmayan son işlem sonucu. Anlam: İşlem sonuçlandı, güncel durum sayfada görülüyor.
settings-displays-result-other = The change finished; the current state is shown below.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı; sonuç cümlesinin sonuna shell'in ek açıklaması eklenirken. Anlam: Sonuç cümlesi ve parantez içinde ek açıklama.
# Değişkenler: $text (sonuç cümlesi, ör. "Uygulandı."), $detail (shell'in ek açıklaması, ör. "ekran kilitli").
settings-displays-result-detail = { $text } ({ $detail })

## Hata mesajları

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Uygulama sırasında ekranlar değişti, liste yenilendi.
settings-displays-error-stale = The displays changed in the meantime; the list was refreshed, try again.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ekran sayısı çok fazla olduğu için istek gönderilemedi.
settings-displays-error-too-large = There are too many displays; the request could not be sent.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ayarlar ile shell arasındaki anlaşamazlık; sürümlerin güncel olması gerektiği.
settings-displays-error-protocol = Settings and the shell could not agree; make sure both are up to date.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Kastedilen monitörün artık bağlı olmadığı, liste yenilendi.
# Değişkenler: $display (monitörün adı; ad yoksa "Bir ekran", ör. "LP27q").
settings-displays-error-unknown-display = { $display } is no longer connected; the list was refreshed.

# Nerede: Ekranlar sayfası, hata cümlelerinde monitör adı bulunamadığında özne olarak kullanılan parça. Anlam: Belirsiz tek ekran.
settings-displays-error-subject-indefinite = A display

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Listeye yeni bir monitör eklendi, liste yenilendi.
settings-displays-error-missing-display = A new display was connected; the list was refreshed, try again.

# Nerede: Ekranlar sayfası, hata cümlelerinde monitör adı bulunamadığında özne olarak kullanılan parça. Anlam: Kastedilen ekran.
settings-displays-error-subject-definite = The display

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Monitörün istediği çözünürlük ya da yenileme hızının desteklenmediği.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q").
settings-displays-error-invalid-mode = { $display }: this resolution or refresh rate is not supported.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda); shell en yakın uygun ölçeği önerdiğinde. Anlam: Ölçek çözünürlüğü bölüyor, uygulanabilir ölçek öneriliyor.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q"), $scale (önerilen ölçek, önceden yüzde olarak biçimli, ör. "%125").
settings-displays-error-invalid-scale-hint = { $display }: this scale does not divide the resolution evenly; the nearest valid scale is { $scale }.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ölçeğin bu monitörün çözünürlüğünü tam bölmediği.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q").
settings-displays-error-invalid-scale = { $display }: this scale does not divide the resolution evenly.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Monitör için istenen konumun geçersiz olduğu.
# Değişkenler: $display (monitörün adı; ad yoksa "Ekran", ör. "LP27q").
settings-displays-error-invalid-position = { $display }: the position is invalid.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Yerleşimde monitörlerin üst üste biniyor olduğu.
settings-displays-error-overlap = The displays overlap.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: En az bir ekranın açık kalması gerektiği.
settings-displays-error-no-active-display = At least one display must stay on.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Yansıtılan bir ekranın kendisi başkasına yansıtamayacağı.
settings-displays-error-mirror-chain = A mirrored display cannot itself mirror another display.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Birincil ekranın açık ve yansıtma yapmayan bir ekran olması gerektiği.
settings-displays-error-invalid-primary = The primary display must be on and not mirroring.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Önceki değişikliğin onayı hâlâ sürdüğü için yenisinin gönderilemediği.
settings-displays-error-pending-exists = The previous change is still waiting for confirmation.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell'in şu an ekranları yönetemediği.
settings-displays-error-unavailable = The shell cannot manage the displays right now.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Ekran kilitliyken değişiklik uygulanamayacağı.
settings-displays-error-locked = Cannot apply while the screen is locked.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell'e ulaşılamadığı, onaylanmamış değişikliğin kendiliğinden geri alınacağı.
settings-displays-error-not-running = The shell cannot be reached; unconfirmed changes are reverted automatically.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell'in zamanında cevap vermediği, değişikliğin uygulanmadığı.
settings-displays-error-timeout = The shell did not respond in time; the change was not applied.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda). Anlam: Shell ile bağlantının kurulamadığı.
settings-displays-error-ipc = Could not connect to the shell.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda); kodu ve mesajı olmayan hata. Anlam: Uygulamanın başarısız olduğu.
settings-displays-error-unknown = Could not apply.

# Nerede: Ekranlar sayfası, alt çubuktaki durum satırı (hata tonunda); tanınmayan hata kodu. Anlam: Uygulamanın başarısız olduğu, kod parantez içinde.
# Değişkenler: $code (shell hata kodu, ör. "stale").
settings-displays-error-code = Could not apply ({ $code }).

## Düğmeler

# Nerede: Ekranlar sayfası, alt çubuğun sağdaki düğme çiftinden soluncası. Anlam: Bekleyen düzenlemeden vazgeçme.
settings-displays-button-cancel = Cancel

# Nerede: Ekranlar sayfası, alt çubuğun sağdaki düğme çiftinden sağuncası. Anlam: Değişiklikleri shell'e gönderme.
settings-displays-button-apply = Apply

# Nerede: Ekranlar sayfası, alt çubuk; onay beklerken Uygula düğmesinin yerini alan düğme. Anlam: Yeni ayarları koruma.
settings-displays-button-keep = Keep

# Nerede: Ekranlar sayfası, alt çubuk; onay beklerken Vazgeç düğmesinin yerini alan düğme. Anlam: Değişikliği geri alma.
settings-displays-button-revert = Revert

## Boş durum ve yükleme

# Nerede: Ekranlar sayfası, sayfa ilk açıldığında liste okunurken görünen satır. Anlam: Ekran listesinin alınıyor olduğu.
settings-displays-loading = Loading displays…

# Nerede: Ekranlar sayfası, boş durum başlığı (shell çalışmıyorken). Anlam: Nuxut shell'inin çalışmadığı.
settings-displays-empty-not-running-title = Nuxut shell is not running

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell çalışmıyorken). Anlam: Ekran ayarlarını shell'in uyguladığı, shell açılınca sayfanın dolacağı.
settings-displays-empty-not-running-body = Display settings are applied by the shell. This page fills in on its own once the shell starts.

# Nerede: Ekranlar sayfası, boş durum başlığı (shell sürümü desteklemiyorken). Anlam: Bu shell sürümünün ekran ayarlarını desteklemediği.
settings-displays-empty-unsupported-title = This shell version does not support display settings

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell sürümü desteklemiyorken). Anlam: Shell güncellenince monitörlerin burada görüneceği.
settings-displays-empty-unsupported-body = Monitors will appear here when the Nuxut shell is updated.

# Nerede: Ekranlar sayfası, boş durum başlığı (shell var ama ekranları yönetemiyorken); gövde metni shell'den gelir. Anlam: Ekranların şu an yönetilemediği.
settings-displays-empty-error-title = Displays cannot be managed right now

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell örneği Hyprland'e erişemiyorken). Anlam: Shell örneğinin ekran ayarlarını değiştiremediği.
settings-displays-empty-not-available = This shell instance cannot change display settings (it has no access to Hyprland).

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell zamanında cevap vermeyince). Anlam: Shell'in zamanında cevap vermediği.
settings-displays-empty-timeout = The shell did not respond in time.

# Nerede: Ekranlar sayfası, boş durum açıklaması (shell örneği adı geçersizken). Anlam: Örnek adı ortam değişkeninin geçersiz olduğu; kurallar parantez içinde.
# Değişkenler: $env (ortam değişkeninin adı, ör. "NUXUT_SHELL_INSTANCE").
settings-displays-error-instance-name = { $env } is invalid ([a-z0-9_-], at most 16)
## Açılır satır özeti

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Satırdaki toplam ayar sayısı.
# Değişkenler: $total (toplam ayar sayısı, ör. "4").
settings-disclosure-summary-settings =
    { $total ->
        [one] { $total } setting
       *[other] { $total } settings
    }

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Toplam ayar sayısı ve kaçı elle ayarlı.
# Değişkenler: $total (toplam ayar sayısı, ör. "4"), $changed (elle ayarlı sayısı, ör. "3").
settings-disclosure-summary-settings-changed =
    { $total ->
        [one] { $total } setting
       *[other] { $total } settings
    } · { $changed ->
        [one] { $changed } customized
       *[other] { $changed } customized
    }

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Satırdaki toplam renk sayısı.
# Değişkenler: $total (toplam renk sayısı, ör. "11").
settings-disclosure-summary-colors =
    { $total ->
        [one] { $total } color
       *[other] { $total } colors
    }

# Nerede: Kapalı açılır satırın özeti (src/pages/disclosure.rs). Anlam: Toplam renk sayısı ve kaçı elle ayarlı.
# Değişkenler: $total (toplam renk sayısı, ör. "11"), $changed (elle ayarlı renk sayısı, ör. "2").
settings-disclosure-summary-colors-changed =
    { $total ->
        [one] { $total } color
       *[other] { $total } colors
    } · { $changed ->
        [one] { $changed } customized
       *[other] { $changed } customized
    }

## Widget listeleri

# Nerede: Bar sayfası, bar bölüm düzenleyicide sıralama/ekleme/çıkarma yapılınca geçmiş (geri al) kaydına verilen etiket. Anlam: Barın üç bölümündeki widget listesinin düzenlendiği.
settings-sections-bar-widgets = Bar widgets

# Nerede: Paneller sayfası, kenar panelinin kart listesi değişince geçmiş kaydına verilen etiket. Anlam: Paneldeki kart listesinin düzenlendiği.
settings-sections-panel-cards = Panel cards

# Nerede: Bar sayfası ve Paneller sayfası, liste sütunlarının altındaki "+ Ekle" düğmesi; Bildirim uygulamaları listesindeki düğme de aynı metni kullanır. Anlam: Listeye yeni öğe ekleme.
settings-sections-add = Add

# Nerede: Boş widget kuyusundaki ipucu (src/pages/sections.rs). Anlam: Bar bölümünde gösterilecek widget olmadığı.
settings-sections-empty-bar = This section is empty

# Nerede: Boş widget kuyusundaki ipucu (src/pages/sections.rs). Anlam: Panelde kart yokken kartların Ekle düğmesiyle eklendiği.
settings-sections-empty-panel = This panel has no cards; add one with Add

# Nerede: Widget ekleme modundaki çıkış düğmesi (src/pages/sections.rs). Anlam: Düğmenin görevi: ekleme modundan vazgeçmek.
settings-sections-cancel = Cancel

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar dikeyken üst, yatayken sol sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Üst yönü.
settings-edge-top = Top

# Nerede: Bar bölüm düzenleyicinin orta sütun başlığı (bar yatayken ve dikeyken). Anlam: Barın ortasındaki bölüm.
settings-sections-center = Center

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar dikeyken alt, yatayken sağ sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Alt yönü.
settings-edge-bottom = Bottom

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar yatayken sol sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Sol yönü.
settings-edge-left = Left

# Nerede: Bar bölüm düzenleyicinin sütun başlıkları; bar yatayken sağ sütun (bar kenarı seçicisinde ve kalınlık satırlarında da kenarın adı). Anlam: Sağ yönü.
settings-edge-right = Right

# Nerede: Bar sayfası, "Ekle" akımında eklenecek widget kalmadığında görünen satır. Anlam: Tüm widget'ların zaten barın bölümlerinde olduğu.
settings-sections-all-in-bar = All widgets are already on the bar.

# Nerede: Paneller sayfası, "Ekle" akımında eklenecek kart kalmadığında görünen satır. Anlam: Tüm kartların bu panelde olduğu.
settings-sections-all-in-panel = All cards are already on this panel.

# Nerede: Bar sayfası ve Paneller sayfası, "Ekle" akımında widget kataloğu henüz gelmediğinde görünen satır. Anlam: Widget listesinin shell'den alınıyor olduğu.
settings-sections-loading = Getting the widget list from the shell…

## Widget listesi hataları

# Nerede: Bar sayfası ve Paneller sayfası, "Ekle" listesinde shell kataloğu yüklenemeyince görünen hata metni. Anlam: nuxut-shell'in bulunamadığı ve nerede arandığı.
settings-sections-error-not-found = Could not get the widget list: nuxut-shell was not found (NUXUT_SHELL_BIN, the Settings directory and PATH were searched).

# Nerede: Aynı hata satırı. Anlam: nuxut-shell'in bulunduğu ama başlatılamadığı.
# Değişkenler: $error (işletim sisteminin hata açıklaması, ör. "Permission denied").
settings-sections-error-spawn = Could not get the widget list: nuxut-shell could not be started ({ $error }).

# Nerede: Aynı hata satırı. Anlam: nuxut-shell'in süresi içinde yanıt vermediği.
settings-sections-error-timeout = Could not get the widget list: nuxut-shell did not respond in time.

# Nerede: Aynı hata satırı; shell çalışıp başarısız çıktığında. Anlam: Shell'in bir çıkış koduyla sonlandığı.
# Değişkenler: $code (çıkış kodu, ör. 1).
settings-sections-error-failed-code = Could not get the widget list: nuxut-shell exited with code { $code }.

# Nerede: Aynı hata satırı; shell sinyalle kırıldığında. Anlam: Shell'in çıkış kodu olmadan sonlandığı.
settings-sections-error-failed-signal = Could not get the widget list: nuxut-shell was terminated by a signal.

# Nerede: Aynı hata satırı; katalog boş geldiğinde de bu kullanılır. Anlam: Shell'in şema çıktısının okunamadığı; sürüm uyuşmazlığı olabileceği.
settings-sections-error-unparsable = Could not get the widget list: the shell's schema output could not be read (the versions may not match).

## Bar kenarı ve kenar kalınlıkları

# Nerede: Bar sayfası, bar kenarı seçicinin başlığının altındaki ipucu satırı. Anlam: Barın kenarının ekran şemasından tıklanarak seçileceği.
settings-edges-pick-hint = Click an edge on the screen diagram

# Nerede: Bar sayfası, bar kenarı değiştirilince geçmiş kaydına verilen etiket. Anlam: Barın konumundaki değişiklik.
settings-edges-bar-position = Bar position

# Nerede: Bar kenarının kalınlığı değiştirilince geçmiş kaydına verilen etiket. Anlam: Bar kalınlığındaki değişiklik.
settings-edges-bar-thickness = Bar thickness

# Nerede: Kenar kalınlığı değiştirilince geçmiş kaydına verilen etiket; kenar bar kenarı değilse. Anlam: İlgili kenarın kalınlığındaki değişiklik.
# Değişkenler: $edge (kenarın adı, ör. "Sol").
settings-edges-edge-thickness = { $edge } edge thickness

# Nerede: Bar sayfası, elle ayarlı kenar satırındaki metin düğme. Anlam: Kenarın genel çerçeve kalınlığına geri bağlanması.
settings-edges-link-general = Follow general

# Nerede: Kenar kalınlığı satırında kaydırıcının sağındaki değer etiketi; kenar bar kenarı. Anlam: Kalınlığın bar boyundan geldiği ve piksel değeri.
# Değişkenler: $px (kalınlık, piksel, ör. 48).
settings-edges-value-bar = Bar · { $px } px

# Nerede: Aynı değer etiketi; kenar elle ayarlanmışsa. Anlam: Kalınlığın bu kenara özel yazıldığı ve piksel değeri.
# Değişkenler: $px (kalınlık, piksel, ör. 12).
settings-edges-value-manual = Manual · { $px } px

# Nerede: Aynı değer etiketi; kenar genel kalınlığı izliyorsa. Anlam: Kalınlığın genel çerçeve kalınlığından geldiği ve piksel değeri.
# Değişkenler: $px (kalınlık, piksel, ör. 8).
settings-edges-value-general = General · { $px } px

## Bildirim uygulamaları

# Nerede: Bildirimler sayfası, uygulama listesinde açılır bildirim anahtarlarının üstündeki sütun başlığı. Anlam: Açılır (popup) bildirim anahtarı sütunu.
# [max: 10]
settings-notif-apps-col-popups = Popups

# Nerede: Bildirimler sayfası, uygulama listesi boşken görünen açıklama. Anlam: Henüz uygulama eklenmediği ve uygulamanın nasıl ekleneceği.
settings-notif-apps-empty = No apps yet. Type an app's name to turn off its popup notifications.

# Nerede: Bildirimler sayfası, uygulama listesi boşken maskotun altındaki başlık. Anlam: henüz uygulama eklenmediği.
settings-notif-apps-empty-title = No apps yet

# Nerede: Bildirimler sayfası, uygulama listesi boşken başlığın altındaki açıklama. Anlam: uygulamanın nasıl ekleneceği.
settings-notif-apps-empty-body = Type an app's name to turn off its popup notifications.

# Nerede: Bildirimler sayfası, uygulama eklemeye yarayan yazma alanının yer tutucusu. Anlam: Eklenecek uygulamanın adının beklendiği.
settings-notif-apps-name-placeholder = App name (e.g. Firefox)

# Nerede: Bildirimler sayfası, uygulamanın bildirim ayarı eklenince ya da kaldırılınca geçmiş kaydına verilen etiket. Anlam: O uygulamanın bildirim ayarındaki değişiklik.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox").
settings-notif-apps-notifications = { $name } notifications

# Nerede: Bildirimler sayfası, uygulamanın açılır bildirim anahtarı değişince geçmiş kaydına verilen etiket. Anlam: O uygulamanın açılır bildirim ayarındaki değişiklik.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox").
settings-notif-apps-popups-toggled = { $name } popup notifications

# Nerede: Bildirimler sayfası, uygulamanın geçmiş anahtarı değişince geçmiş kaydına verilen etiket. Anlam: O uygulamanın bildirim geçmişi ayarındaki değişiklik.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox").
settings-notif-apps-history-toggled = { $name } notification history

## Hareket rolü düzenleyici

# Nerede: Hareket sayfası, rol düzenleyicinin sağ üstündeki özet; rolin stili yayken. Anlam: Yayın cevap süresi ve yaylanması (sönüm oranı).
# Değişkenler: $response (yay cevap süresi, saniye, önceden biçimli, ör. "0,4"), $damping (sönüm oranı, önceden biçimli, ör. "0,6").
settings-motion-role-summary-spring = Spring · { $response } s · springiness { $damping }

# Nerede: Hareket sayfası, rol düzenleyicide eğrinin sağındaki ipucu; stil yayken. Anlam: Grafiği sürüklemenin hızı ve taşma üzerindeki etkisi.
settings-motion-role-hint-spring = Drag the graph: right is slower, up is more overshoot.

# Nerede: Hareket sayfası, rol düzenleyicinin özetindeki eğri adı; kayıttaki adlandırılmış eğrilerden biri değilse. Anlam: Kullanıcının kendi çizdiği özel eğri.
settings-motion-easing-custom = Custom curve

# Nerede: Hareket sayfası, rol düzenleyicinin sağ üstündeki özet; rolin stili eğriyken. Anlam: Eğrinin adı ve süresi.
# Değişkenler: $name (eğrinin adı, ör. "Hızlı"), $ms (süre, milisaniye, ör. 200).
settings-motion-role-summary-bezier = Curve · { $name } · { $ms } ms

# Nerede: Hareket sayfası, rol düzenleyicide eğrinin sağındaki ipucu; stil eğriyken. Anlam: Tutamaçları sürükleyerek eğriyi değiştirme yönergesi.
settings-motion-role-hint-bezier = Change the curve by dragging the handles.

# Nerede: Hareket sayfası, rolün yay değerleri sürüklenince geçmiş kaydına verilen etiket. Anlam: Rolün yayınındaki değişiklik.
# Değişkenler: $role (hareket rolünün adı, ör. "Efektler").
settings-motion-role-drag-spring = { $role } spring

# Nerede: Hareket sayfası, rolün eğrisi sürüklenince geçmiş kaydına verilen etiket. Anlam: Rolün eğrisindeki değişiklik.
# Değişkenler: $role (hareket rolünün adı, ör. "Efektler").
settings-motion-role-drag-bezier = { $role } curve

## Hareket ön ayarı kartları

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Sakin ön ayarı (varsayılan); yumuşak ve taşmasız hareket.
settings-motion-preset-calm = Calm
    .description = Smooth, no overshoot

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Canlı ön ayarı; esneyen, zıplayan hareket.
settings-motion-preset-lively = Lively
    .description = Pudding: stretches and bounces

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Çevik ön ayarı; kısa ve net hareket.
settings-motion-preset-snappy = Snappy
    .description = Short and crisp

# Nerede: Hareket sayfası, ön ayar kartının başlığı ve altındaki kısa açıklama. Anlam: Kapalı ön ayarı; animasyonların olmadığı.
settings-motion-preset-instant = Off
    .description = No animations

# Nerede: Hareket sayfası, ön ayarların elle özelleştirildiğinde görünen rozet. Anlam: Ön ayarın üstüne yazılmış değerler olduğu.
settings-motion-preset-customized = Customized

# Nerede: Hareket sayfası, özelleştirme rozetinin yanındaki açıklama. Anlam: Tek tek ayarlanan değerlerin karakterin (ön ayarın) önüne geçtiği.
settings-motion-preset-customized-note = Values set individually take precedence over the character.

# Nerede: Hareket sayfası, özelleştirilmişken görünen düğme; aynı metin geri al kaydının etiketi. Anlam: Elle yazılan değerleri silip karaktere (ön ayara) dönme.
settings-motion-preset-revert = Revert to character

# Nerede: Hareket sayfası, ön ayar kartı seçilince geçmiş kaydına verilen etiket. Anlam: Hareket ön ayarının değiştiği.
# Değişkenler: $preset (ön ayarın adı, ör. "Canlı").
settings-motion-preset-changed = Motion preset: { $preset }
## Yazı sayfası — eksik font bilgisi

# Nerede: Yazı sayfası, font seçicinin başlık satırındaki durum yazısı; seçili aile kurulu değilken yedekte kullanılan aile biliniyorsa. Anlam: Ailenin kurulu olmadığı, geçici olarak hangi ailenin kullanıldığı.
# Değişkenler: $used (yedek olarak kullanılan aile, ör. "DejaVu Sans").
settings-fonts-missing-used = Not installed — using { $used }

# Nerede: Aynı durum yazısı; ailenin bilinen bir Arch paketi varsa sona eklenen kısım. Anlam: Ailenin kurulu olmadığı, yedeğin kullanıldığı ve kurulabilecek Arch paketi.
# Değişkenler: $used (yedek olarak kullanılan aile, ör. "DejaVu Sans"), $package (Arch paketinin adı, ör. "inter-font").
settings-fonts-missing-used-package = Not installed — using { $used } · Arch package: { $package }

# Nerede: Yazı sayfası, eksik font durum yazısı; yedekte kullanılan aile bilinmiyorken (ikon seti kartının alt yazısında da). Anlam: Ailenin kurulu olmadığı, yedek bir ailenin kullanıldığı.
settings-fonts-missing-fallback = Not installed — using the fallback

# Nerede: Aynı durum yazısı; ailenin bilinen bir Arch paketi varsa. Anlam: Yedek kullanımına ek olarak kurulabilecek Arch paketi.
# Değişkenler: $package (Arch paketinin adı, ör. "noto-fonts").
settings-fonts-missing-fallback-package = Not installed — using the fallback · Arch package: { $package }

## Yazı sayfası — font seçici

# Nerede: Yazı sayfası, rolün font seçicisi açıldığındaki arama alanının yer tutucusu. Anlam: Yazı tipi ailelerinde arama yapılacağı.
settings-fonts-search-placeholder = Search fonts

# Nerede: Yazı sayfası, font seçicide aile listesini süzen çip (yalnızca metin rollerinde görünür). Anlam: Süzgenin kalktığı, tüm ailelerin listelendiği.
# [max: 6]
settings-fonts-filter-all = All

# Nerede: Yazı sayfası, font seçicinin kalınlık seçimindeki çipler; CSS ağırlık adları (100–900). Anlam: En ince kalınlık (CSS 100).
# [max: 12]
settings-fonts-weight-thin = Thin

# Nerede: Aynı kalınlık çipleri. Anlam: Çok hafif kalınlık (CSS 200).
# [max: 12]
settings-fonts-weight-extra-light = Extra light

# Nerede: Aynı kalınlık çipleri. Anlam: Hafif kalınlık (CSS 300).
# [max: 12]
settings-fonts-weight-light = Light

# Nerede: Aynı kalınlık çipleri. Anlam: Normal kalınlık (CSS 400).
# [max: 12]
settings-fonts-weight-regular = Regular

# Nerede: Aynı kalınlık çipleri. Anlam: Orta kalınlık (CSS 500).
# [max: 12]
settings-fonts-weight-medium = Medium

# Nerede: Aynı kalınlık çipleri. Anlam: Yarı kalın (CSS 600).
# [max: 12]
settings-fonts-weight-semi-bold = Semi-bold

# Nerede: Aynı kalınlık çipleri. Anlam: Kalın (CSS 700).
# [max: 12]
settings-fonts-weight-bold = Bold

# Nerede: Aynı kalınlık çipleri. Anlam: Çok kalın (CSS 800).
# [max: 12]
settings-fonts-weight-extra-bold = Extra bold

# Nerede: Aynı kalınlık çipleri. Anlam: En kalın (CSS 900).
# [max: 12]
settings-fonts-weight-black = Black

# Nerede: Yazı sayfası, rolün yazı ayarları değişmişken seçicinin altındaki metin düğme. Anlam: Rolün aile, kalınlık ve ölçek ayarlarını varsayılana döndürme.
settings-fonts-reset = Reset to default

# Nerede: Yazı sayfası, İkon rolü seçicisinin durum yazısı; aile boş bırakılıp setin fontu kullanılıyorsa. Anlam: Görünen ailenin seçili ikon setinin kendi fontu olduğu.
settings-fonts-icon-set-font = Icon set font

# Nerede: Yazı sayfası, font seçicinin durum yazısı; rol için kayıtta aile yazılı değilken. Anlam: Ailenin framework varsayılanı olduğu.
settings-fonts-default = Default

# Nerede: Yazı sayfası, rolün ailesi ve kalınlığı seçilince geçmiş (geri al) kaydına verilen etiket. Anlam: Rolün yazı tipindeki değişiklik.
# Değişkenler: $role (rolün adı, ör. "Arayüz").
settings-fonts-role-family = { $role } font

# Nerede: Yazı sayfası, rolün yazı ayarları varsayılana dönerken geçmiş kaydına verilen etiket. Anlam: Rolün yazı ayarlarındaki sıfırlama.
# Değişkenler: $role (rolün adı, ör. "Arayüz").
settings-fonts-role-reset = { $role } font

## Yazı rolleri

# Nerede: Yazı sayfası, İkon rolünün önizleme satırındaki başlık; geçmiş kayıtlarındaki rol adı olarak da. Anlam: İkon fontu rolünün adı.
settings-typography-role-icon = Icon

## Duvar kağıdı seçici

# Nerede: Duvar kağıdı sayfası, hero önizlemenin yanındaki dolu düğme; görsel seçilmemişken (src/pages/wallpaper.rs). Anlam: Dosya seçiciyle bir görsel seçme.
settings-wallpaper-button-choose = Choose an image…

# Nerede: Duvar kağıdı sayfası, hero önizlemenin yanındaki tonlu düğme; görsel seçiliyken (src/pages/wallpaper.rs). Anlam: Mevcut görselin değiştirilmesi.
settings-wallpaper-button-change = Change…

# Nerede: Duvar kağıdı sayfası, hero önizlemenin altındaki kapsam satırı; tüm monitörler aynı görseli kullanırken (src/pages/wallpaper.rs). Anlam: Duvar kağıdının tüm monitörlerde ortak olduğu.
settings-wallpaper-scope-all = On all monitors

# Nerede: Duvar kağıdı sayfası, hero önizlemenin altındaki kapsam satırı; bazı monitörler ayrı görsel kullanırken (src/pages/wallpaper.rs). Anlam: Ayrı görseli olan monitör sayısı.
# Değişkenler: $count (ayrı görseli olan monitör sayısı, ör. 2).
settings-wallpaper-scope-per-monitor =
    { $count ->
        [one] { $count } monitor has its own image
       *[other] { $count } monitors have their own image
    }

## Yerleşik Nuxut duvar kağıtları

# Nerede: Duvar kağıdı sayfasında, hero önizlemenin yanında görsel adının altındaki açıklama satırı; seçili duvar kağıdı Nuxut ile gelen bir yerleşik duvar kağıdıyken görünür (src/pages/wallpaper.rs). Anlam: Bu duvar kağıdının bir dosya değil, Nuxut'un hazır duvar kağıdı olduğunu anlatır.
settings-wallpaper-builtin-caption = Nuxut wallpaper

# Nerede: Duvar kağıdı sayfasında, hero önizlemenin yanında görsel adının altındaki açıklama satırı; hiç duvar kağıdı seçilmemişken görünür (src/pages/wallpaper.rs). Anlam: Seçim yapılmadığı için Nuxut'un varsayılan duvar kağıdının gösterildiğini anlatır.
settings-wallpaper-builtin-default = Default Nuxut wallpaper

# Nerede: Duvar kağıdı sayfası, yerleşik duvar kağıtlarının kart ızgarası satırının başlığı (src/registry.rs, wallpaper.builtin). Anlam: Nuxut ile gelen hazır duvar kağıtları.
settings-wallpaper-builtin-label = Nuxut wallpapers

# Nerede: Duvar kağıdı sayfası, yerleşik duvar kağıtları satırının alt yazısı. Anlam: Bu duvar kağıtları koyu/açık kipe göre kendiliğinden değişir ve arayüz renkleri onlardan türetilir.
settings-wallpaper-builtin-description = Adapts automatically to dark and light mode; colors come from here too

# Nerede: Duvar kağıdı sayfası, yerleşik duvar kağıtları satırının arama sözcükleri (virgüllü liste). Anlam: Genel terimler ve yerleşik duvar kağıtlarının görünen adları; adlar nuxut-wallpapers kataloğundaki çevirilerle aynı olmalı.
settings-wallpaper-builtin-keywords = nuxut, built-in, preset, default, wallpaper, background, pair flight, wingtip, climb, migration, aurora, big silhouette, cloud break, far glide, chase

# Nerede: Duvar kağıdı sayfasında bir bölüm (kart) başlığı. Anlam: Nuxut ile gelen hazır duvar kağıtlarını toplayan bölüm.
settings-section-builtin-wallpapers = Nuxut wallpapers

## Renkler bağlantısı

# Nerede: Duvar kağıdı sayfası, Renkler bağlantı satırının başlığı; renkler duvar kağıdından alınırken (src/pages/wallpaper.rs). Anlam: Arayüz renklerinin bu görselden türetildiği.
settings-wallpaper-colors-link-title-wallpaper = Colors come from this image

# Nerede: Aynı satırın alt yazısı. Anlam: Renk şeması ve kontrastın Görünüm › Renkler'de ayarlandığı.
settings-wallpaper-colors-link-subtitle-wallpaper = Scheme and contrast in Appearance › Colors

# Nerede: Aynı satırın başlığı; renkler sabitken (src/pages/wallpaper.rs). Anlam: Renklerin duvar kağıdından türetilmediği.
settings-wallpaper-colors-link-title-static = Colors come from the Nuxut theme

# Nerede: Aynı satırın alt yazısı. Anlam: Tüm renklerin bu görsele uyabileceği.
settings-wallpaper-colors-link-subtitle-static = All colors can match this image

# Nerede: Duvar kağıdı sayfası, Renkler bağlantı satırının sağındaki metin düğme (src/pages/wallpaper.rs). Anlam: Görünüm › Renkler sayfasına götürür.
settings-wallpaper-colors-link-go = Appearance

## Renk şeması

# Nerede: Duvar kağıdı sayfası, renk şeması seçicideki dokuz varyant kartının başlıkları. Anlam: Material You Tonal Spot varyantı; kağıdın tonuna yakın renkler.
settings-wallpaper-variant-tonal = Tonal

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Vibrant varyantı; canlı, doygun renkler.
settings-wallpaper-variant-vibrant = Vibrant

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Expressive varyantı; özgün ve ifadeli renkler.
settings-wallpaper-variant-expressive = Expressive

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Neutral varyantı; yumuşak, nötr renkler.
settings-wallpaper-variant-neutral = Neutral

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Fidelity varyantı; görseldeki renklere sadık.
settings-wallpaper-variant-fidelity = Fidelity

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Content varyantı; görselin içeriğine göre ayarlı renkler.
settings-wallpaper-variant-content = Content

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Monochrome varyantı; tek renkli, gri tonlar.
settings-wallpaper-variant-monochrome = Monochrome

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Rainbow varyantı; gökkuşağı tonları.
settings-wallpaper-variant-rainbow = Rainbow

# Nerede: Aynı varyant kartı başlıkları. Anlam: Material You Fruit Salad varyantı; canlı, karışık renkler.
settings-wallpaper-variant-fruit-salad = Fruit salad

# Nerede: Duvar kağıdı sayfası, renk şeması seçicinin durum satırı; görsel seçilmemişken. Anlam: Renk üretilebilmesi için önce duvar kağıdı seçilmesi gerektiği.
settings-wallpaper-scheme-need-image = Choose a wallpaper first.

# Nerede: Duvar kağıdı sayfası, renk şeması seçicinin durum satırı; yeni görselin renk şeritleri üretilirken. Anlam: Renklerin hazırlanıyor olduğu.
settings-wallpaper-scheme-preparing = Preparing colors…

## Duvar kağıdı hataları

# Nerede: Duvar kağıdı sayfası ve monitör başına duvar kağıdı listesi; dosya seçici açılamayınca satırın altındaki hata yazısı. Anlam: xdg-desktop-portal dosya seçicisinin açılamadığı.
settings-wallpaper-error-portal = Could not open the file chooser (is xdg-desktop-portal running?).

# Nerede: Duvar kağıdı sayfası, renk şeması seçicinin durum satırı (hata); görselden tohum renk çıkarılamayınca. Anlam: Bu görselden renk çıkarılamadığı.
settings-wallpaper-error-source = Could not extract colors from this image.

# Nerede: Aynı durum satırı (hata); görsel boyut/piksel sınırını aşınca renk çıkarılamadığında. Anlam: Görselin çok büyük olduğu.
settings-wallpaper-error-too-large = This image is too large to extract colors from.

# Nerede: Aynı durum satırı (hata); görsel işçisi (nuxut-image-worker) kurulu değilken JPEG XL gibi yalnızca işçide açılan bir biçim seçilince. Anlam: Bu biçimin görsel işçisi gerektirdiği.
settings-wallpaper-error-worker = This format needs the image worker (nuxut-image-worker); it was not found.

# Nerede: Aynı durum satırı (hata); palette.toml'un yazılacağı durum dizini bulunamayınca. Anlam: Durum dizininin bulunamadığı.
settings-wallpaper-error-state-dir = Could not find the state directory ($XDG_STATE_HOME).

# Nerede: Aynı durum satırı (hata); üretilen palet dosyaya yazılamayınca. Anlam: Renk şemasının kaydedilemediği.
settings-wallpaper-error-save = Could not save the color scheme.

## Monitör başına duvar kağıdı

# Nerede: Duvar kağıdı sayfası, monitör başına satırdaki yerleşim seçim düğmelerinin ilk segmenti. Anlam: Monitörün sayfanın genel duvar kağıdını ve yerleşimini izlediği.
# [max: 8]
settings-outputs-fit-general = General

# Nerede: Duvar kağıdı sayfası, monitör başına listedeki yerleşim seçim menüsünün ilk seçeneği (src/pages/wallpaper_outputs.rs). Anlam: Monitörün genel yerleşimi izlediği ve genel yerleşimin adı.
# Değişkenler: $fit (genel yerleşimin adı, ör. "Tamamı").
settings-outputs-fit-general-option = General · { $fit }

# Nerede: Duvar kağıdı sayfası, monitör başına listede hiç monitör yokken görünen satır (src/pages/wallpaper_outputs.rs). Anlam: Monitör bulunamadığı.
settings-outputs-no-monitors = No monitors found.

# Nerede: Duvar kağıdı sayfası, monitör başına listede elle monitör ekleme alanının yer tutucusu (src/pages/wallpaper_outputs.rs). Anlam: Eklenecek monitörün bağlantı adının beklendiği.
settings-outputs-connector-placeholder = Connection name, e.g. HDMI-A-1

# Nerede: Duvar kağıdı sayfası, monitör başına listeyi açan açılır satırın başlığı (src/pages/wallpaper_outputs.rs). Anlam: Bağlı olmayan bir monitörün elle eklenebildiği.
settings-outputs-add-unconnected = Add a monitor that isn't connected

# Nerede: Aynı açılır satırın başlığındaki açıklama (src/pages/wallpaper_outputs.rs). Anlam: Henüz takılı olmayan bir monitöre görselin önceden atanabildiği.
settings-outputs-add-unconnected-description = Assign an image to a monitor that isn't plugged in yet

# Nerede: Aynı açılır satırın içindeki, yazılan adı onaylayan düğme (src/pages/wallpaper_outputs.rs). Anlam: Yazılan bağlantı adıyla monitör satırı ekleme.
settings-outputs-add-button = Add

# Nerede: Duvar kağıdı sayfası, monitör satırının alt yazısı; monitör için görsel seçilmemişse dosya adının yerine. Anlam: Monitörün genel duvar kağıdını kullandığı.
settings-outputs-general-wallpaper = General wallpaper

# Nerede: Duvar kağıdı sayfası, monitör satırının alt yazısı; monitör kayıtlı ama bağlı değilken (src/pages/wallpaper_outputs.rs). Anlam: Monitörün bağlı olmadığı ve görselin dosya adının gösterildiği.
# Değişkenler: $image (görselin dosya adı, ör. "wallpaper.png"; görsel seçilmemişse "Genel duvar kağıdı").
settings-outputs-detail-disconnected = Disconnected · { $image }

# Nerede: Duvar kağıdı sayfası, monitör satırının alt yazısı; monitör bağlıyken donanım açıklaması yoksa. Anlam: Monitörün bağlı olduğu.
settings-outputs-connected = Connected

# Nerede: Duvar kağıdı sayfası, monitör başına listede elle monitör ekleme alanının altındaki hata yazısı (src/pages/wallpaper_outputs.rs). Anlam: Girilen bağlantı adının kurallara uymadığı.
settings-outputs-error-name-rules = Invalid name: at most 64 characters, no spaces or quotes (e.g. DP-1).

# Nerede: Aynı hata yazısı; monitör sınırı dolduğunda. Anlam: En fazla 32 monitör eklenebileceği.
settings-outputs-error-too-many = You can add at most 32 monitors.

# Nerede: Duvar kağıdı sayfası, monitörün yerleşimi değişince geçmiş (geri al) kaydına verilen etiket. Anlam: Monitörün yerleşimindeki değişiklik.
# Değişkenler: $name (monitörün bağlayıcı adı, ör. "DP-1").
settings-outputs-fit = { $name } fit

# Nerede: Duvar kağıdı sayfası, monitör için görsel seçilince geçmiş kaydına verilen etiket. Anlam: O monitörün duvar kağıdındaki değişiklik.
# Değişkenler: $name (monitörün bağlayıcı adı, ör. "DP-1").
settings-outputs-wallpaper = { $name } wallpaper

## Kısayol satırı

# Nerede: Kısayollar sayfası, kısayol alanı yakalama modundayken satırın altındaki ipucu. Anlam: Tuş kombinasyonunun beklendiği; Esc'in iptal, Geri silmenin temizleme yaptığı.
settings-keybinds-hint = Press a key combination · Esc cancels · Backspace clears

# Nerede: Kısayollar sayfası, yakalama sırasında alanın içindeki metin; henüz değiştirici basılı değilken. Anlam: Tuşların bekleniyor olduğu.
settings-keybinds-capturing = Press keys…

# Nerede: Kısayollar sayfası, atanmamış kısayol alanının içindeki metin. Anlam: Eyleme kısayol atanmadığı.
settings-keybinds-unassigned = Unassigned

# Nerede: Kısayollar sayfası, satırın altındaki uyarı; dosyadaki kısayol değeri geçersizken (her hata için ayrı mesaj). Anlam: Değerin boş olduğu ve kısayolun atanmamış sayıldığı.
settings-keybinds-error-empty = Invalid shortcut (empty); treated as unassigned

# Nerede: Aynı uyarı. Anlam: Kısayolun çok uzun olduğu ve atanmamış sayıldığı.
settings-keybinds-error-too-long = Invalid shortcut (too long); treated as unassigned

# Nerede: Aynı uyarı. Anlam: Tanınmayan değiştirici içerdiği ve atanmamış sayıldığı.
settings-keybinds-error-unknown-modifier = Invalid shortcut (unknown modifier); treated as unassigned

# Nerede: Aynı uyarı. Anlam: Geçersiz tuş içerdiği ve atanmamış sayıldığı.
settings-keybinds-error-invalid-key = Invalid shortcut (invalid key); treated as unassigned

# Nerede: Kısayollar sayfası, yakalama sırasında satırın altındaki uyarı; yazı tuşu değiştiricisiz basıldığında. Anlam: Yazı tuşlarının Super, Ctrl ya da Alt ile kullanılması gerektiği.
settings-keybinds-need-modifier = Letter keys must be combined with Super, Ctrl or Alt

# Nerede: Kısayollar sayfası, yakalama sırasında satırın altındaki uyarı; tuşun adı çözülemeyince. Anlam: Tuşun tanınmadığı, başka bir birleşim denenmesi gerektiği.
settings-keybinds-unknown-key = This key could not be recognized; try another combination

# Nerede: Kısayollar sayfası, satırın altındaki uyarı; aynı birleşim başka eylemde de var ve bu satır shell sırasında önde. Anlam: Kısayolun iki eylemde olduğu, diğerinde çalışmayacağı.
# Değişkenler: $other (diğer eylemin adı, ör. "Sesi kapat").
settings-keybinds-clash-wins = The same shortcut is also used by “{ $other }”; it does not work there

# Nerede: Aynı uyarı; bu satır shell sırasında geride. Anlam: Kısayolun iki eylemde olduğu, bu eylemde çalışmayacağı.
# Değişkenler: $other (diğer eylemin adı, ör. "Sesi kapat").
settings-keybinds-clash-loses = The same shortcut is also used by “{ $other }”; it does not work here

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
settings-keybind-cap-volume-up = Volume +

# Nerede: Aynı tuş başlıkları. Anlam: Sesi azaltma tuşu.
# [max: 12]
settings-keybind-cap-volume-down = Volume −

# Nerede: Aynı tuş başlıkları. Anlam: Sesi kapama tuşu.
# [max: 12]
settings-keybind-cap-mute = Mute

# Nerede: Aynı tuş başlıkları. Anlam: Mikrofonu kapama tuşu.
# [max: 12]
settings-keybind-cap-mic-mute = Mic

# Nerede: Aynı tuş başlıkları. Anlam: Ekran parlaklığını artırma tuşu.
# [max: 12]
settings-keybind-cap-brightness-up = Brightness +

# Nerede: Aynı tuş başlıkları. Anlam: Ekran parlaklığını azaltma tuşu.
# [max: 12]
settings-keybind-cap-brightness-down = Brightness −

# Nerede: Aynı tuş başlıkları. Anlam: Oynatma tuşu.
# [max: 12]
settings-keybind-cap-play = Play

# Nerede: Aynı tuş başlıkları. Anlam: Duraklatma tuşu.
# [max: 12]
settings-keybind-cap-pause = Pause

# Nerede: Aynı tuş başlıkları. Anlam: Durdurma tuşu.
# [max: 12]
settings-keybind-cap-stop = Stop

# Nerede: Aynı tuş başlıkları. Anlam: Sonraki parça tuşu.
# [max: 12]
settings-keybind-cap-next = Next

# Nerede: Aynı tuş başlıkları. Anlam: Önceki parça tuşu.
# [max: 12]
settings-keybind-cap-previous = Previous

# Nerede: Aynı tuş başlıkları. Anlam: Hesap makinesi açma tuşu.
# [max: 14]
settings-keybind-cap-calculator = Calculator

# Nerede: Aynı tuş başlıkları. Anlam: Arama tuşu.
# [max: 12]
settings-keybind-cap-search = Search

# Nerede: Aynı tuş başlıkları. Anlam: Posta tuşu.
# [max: 12]
settings-keybind-cap-mail = Mail

# Nerede: Aynı tuş başlıkları. Anlam: Ana sayfa tuşu.
# [max: 12]
settings-keybind-cap-home-page = Home page

# Nerede: Aynı tuş başlıkları. Anlam: Ortamı çıkarma tuşu.
# [max: 12]
settings-keybind-cap-eject = Eject

# Nerede: Aynı tuş başlıkları. Anlam: Uyku tuşu.
# [max: 12]
settings-keybind-cap-sleep = Sleep

# Nerede: Aynı tuş başlıkları. Anlam: Ekran değiştirme tuşu.
# [max: 12]
settings-keybind-cap-display = Display

# Nerede: Aynı tuş başlıkları. Anlam: Kablosuz ağ tuşu.
# [max: 12]
settings-keybind-cap-wlan = Wi-Fi

# Nerede: Aynı tuş başlıkları. Anlam: Dokunmatik yüzeyi açıp kapama tuşu.
settings-keybind-cap-touchpad = Touchpad

# Nerede: Kısayollar sayfası, kısayol alanındaki tuş başlıkları. Anlam: Boşluk tuşu.
# [max: 8]
settings-keybind-cap-space = Space

# Nerede: Aynı tuş başlıkları. Anlam: Geri silme tuşu.
# [max: 10]
settings-keybind-cap-backspace = Backspace

# Nerede: Kısayollar sayfası, fare düğmesine atanmış kısayolun tuş başlığı. Anlam: Fare düğmesi ve numarası.
# Değişkenler: $button (fare düğme numarası, ör. 1).
settings-keybind-cap-mouse = Mouse { $button }

# Nerede: Kısayollar sayfası, ham tuş koduyla atanmış kısayolun tuş başlığı. Anlam: Tuş kodu ve numarası.
# Değişkenler: $code (tuş kodu, ör. 10).
settings-keybind-cap-code = Key code { $code }
## Sayfa başlıkları (kenar çubuğu)

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Görünüm (tema ve renk) sayfasının etiketi.
# [max: 14]
settings-page-appearance = Appearance

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Yazı tipleri sayfasının etiketi.
# [max: 14]
settings-page-fonts = Fonts

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Hareket ve animasyon sayfasının etiketi.
# [max: 14]
settings-page-motion = Motion

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Üst bar ayarları sayfasının etiketi; "Bar" arayüzde olduğu gibi kalır.
# [max: 14]
settings-page-bar = Bar

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Shell ayarları sayfasının etiketi; "Shell" arayüzde olduğu gibi kalır.
# [max: 14]
settings-page-shell = Shell

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Monitör ve ekran ayarları sayfasının etiketi.
# [max: 14]
settings-page-displays = Displays

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Pencere davranışları sayfasının etiketi.
# [max: 14]
settings-page-windows = Windows

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Duvar kağıdı sayfasının etiketi.
# [max: 14]
settings-page-wallpaper = Wallpaper

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Kullanıcı hesapları sayfasının etiketi.
# [max: 14]
settings-page-accounts = Accounts

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Bildirim ayarları sayfasının etiketi.
# [max: 14]
settings-page-notifications = Notifications

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Ses ayarları sayfasının etiketi.
# [max: 14]
settings-page-sound = Sound

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Klavye ayarları sayfasının etiketi.
# [max: 14]
settings-page-keyboard = Keyboard

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Fare ve dokunmatik yüzey ayarları sayfasının etiketi.
# kenar çubuğunda uzun etiket
settings-page-pointer = Mouse & touchpad

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Klavye kısayolları sayfasının etiketi.
# [max: 14]
settings-page-keybinds = Shortcuts

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Uygulama bilgisi sayfasının etiketi.
# [max: 14]
settings-page-about = About

## Kenar çubuğu grup başlıkları

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Görünüm, Duvar kağıdı, Yazı, Hareket sayfalarını toplayan kişisel grup.
# [max: 10]
settings-nav-group-personal = Personal

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Bar, Shell, Pencereler, Ekranlar, Kısayollar sayfalarını toplayan masaüstü grubu.
# [max: 10]
settings-nav-group-desktop = Desktop

# Nerede: Ayarlar penceresi, kenar çubuğundaki grup başlığı. Anlam: Bildirimler, Ses, Hesaplar, Hakkında sayfalarını toplayan sistem grubu.
# [max: 10]
settings-nav-group-system = System

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
settings-unit-second = s

# Nerede: Klavye sayfası, Tekrar hızı kaydırıcısında değerin yanında. Anlam: saniyedeki tuş tekrarı sayısı birimi.
settings-unit-repeat-rate = keys/s

# Nerede: Dakika birimli kaydırıcılarda (gece ışığı geçiş süresi) değerin yanında. Anlam: dakika birimi kısaltması.
settings-unit-minute = min

## Seçenekler: Görünüm

# Nerede: Görünüm sayfası, Renk kaynağı ayarının segmentleri. Anlam: renklerin seçili Nuxut renk temasından geldiği seçenek.
settings-choice-color-source-static = Nuxut theme

# Nerede: Görünüm sayfası, Renk kaynağı ayarının segmentleri. Anlam: renklerin duvar kağıdından türetildiği seçenek.
settings-choice-color-source-wallpaper = From wallpaper

# Nerede: Palet kipi seçenekleri. Anlam: koyu palet seçeneği.
settings-choice-palette-mode-dark = Dark

# Nerede: Palet kipi seçenekleri. Anlam: açık palet seçeneği.
settings-choice-palette-mode-light = Light

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının segmentleri. Anlam: kenarlık çizilmemesi seçeneği.
settings-choice-outline-none = None

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının segmentleri. Anlam: hafif görünür ince kenarlık seçeneği.
settings-choice-outline-subtle = Subtle

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının segmentleri. Anlam: tam kalınlıkta kenarlık seçeneği.
settings-choice-outline-full = Full

## Seçenekler: Duvar kağıdı

# Nerede: Duvar kağıdı sayfasındaki yerleşim seçenekleri. Anlam: kağıdı ekrana doldurup taşan kısmı kırpan seçenek.
settings-choice-wallpaper-fit-cover = Fill and crop

# Nerede: Duvar kağıdı sayfasındaki yerleşim seçenekleri. Anlam: kağıdın tamamını sığdıran seçenek.
settings-choice-wallpaper-fit-contain = Fit

# Nerede: Duvar kağıdı sayfasındaki yerleşim seçenekleri. Anlam: kağıdı oranı bozarak ekrana geren seçenek.
settings-choice-wallpaper-fit-fill = Stretch

## Seçenekler: Hareket

# Nerede: Hareket sayfası, Stil ayarının segmentleri. Anlam: yay (spring) animasyon stili.
settings-choice-motion-style-spring = Spring

# Nerede: Hareket sayfası, Stil ayarının segmentleri. Anlam: eğri (easing) animasyon stili.
settings-choice-motion-style-curve = Curve

# Nerede: Hareket rollerindeki Stil ayarının segmentleri. Anlam: rolün genel stili izlediği seçenek.
settings-choice-role-style-default = General

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: Material vurgulu eğri ailesi.
settings-choice-easing-emphasized = Emphasized

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: vurgulu ve yavaşlayarak biten eğri.
settings-choice-easing-emphasized-decelerate = Emphasized decelerate

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: vurgulu ve hızlanarak başlayan eğri.
settings-choice-easing-emphasized-accelerate = Emphasized accelerate

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: yumuşak giriş çıkışlı standart eğri.
settings-choice-easing-ease = Smooth

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: yavaşlayarak biten eğri.
settings-choice-easing-ease-out = Decelerate

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: hızlanarak başlayan eğri.
settings-choice-easing-ease-in = Accelerate

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: yavaşlayıp hızlanıp yeniden yavaşlayan eğri.
settings-choice-easing-ease-in-out = Slow-fast-slow

# Nerede: Hareket sayfası, Eğri ayarının çipleri. Anlam: sabit hızlı doğrusal eğri.
settings-choice-easing-linear = Linear

## Seçenekler: Bar uç eylemleri

# Nerede: Bar sayfası, uç eylemi seçim çipleri. Anlam: eylemin widget'ın kendi görevi olduğu seçenek.
settings-choice-edge-click-widget = Widget's own

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: eylemi takvim panelini açmak olan seçenek.
settings-choice-edge-click-clock = Calendar

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: eylemi uygulama başlatıcıyı açmak olan seçenek.
settings-choice-edge-click-launcher = App launcher

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: sesi açıp kapatan seçenek.
settings-choice-edge-click-volume = Mute/unmute sound

# Nerede: Bar sayfası, sol tık eylemi çipleri. Anlam: mikrofonu açıp kapatan seçenek.
settings-choice-edge-click-mic = Mute/unmute microphone

# Nerede: Bar sayfası, kaydırma eylemi çipleri. Anlam: kaydırmayla ses düzeyini değiştiren seçenek.
settings-choice-edge-scroll-volume = Volume

# Nerede: Bar sayfası, kaydırma eylemi çipleri. Anlam: kaydırmayla mikrofonu değiştiren seçenek.
settings-choice-edge-scroll-mic = Microphone

# Nerede: Bar sayfası, kaydırma eylemi çipleri. Anlam: kaydırmayla parlaklığı değiştiren seçenek.
settings-choice-edge-scroll-brightness = Brightness

## Seçenekler: Workspace widget'ı

# Nerede: Bar sayfası, Workspace görünümü ayarının segmentleri. Anlam: workspace'leri uygulama ikonuyla gösteren seçenek.
settings-choice-workspace-style-icons = Icons

# Nerede: Bar sayfası, Workspace görünümü ayarının segmentleri. Anlam: workspace'leri numarayla gösteren seçenek.
settings-choice-workspace-style-numbers = Numbers

# Nerede: Bar sayfası, Workspace görünümü ayarının segmentleri. Anlam: workspace'leri noktayla gösteren seçenek.
settings-choice-workspace-style-dots = Dots

# Nerede: Bar sayfası, Workspace kaydırma ayarının segmentleri. Anlam: boş workspace'ler de dahil tümünde kaydırmayı etkinleştiren seçenek.
settings-choice-workspace-scroll-all = All

# Nerede: Bar sayfası, Workspace kaydırma ayarının segmentleri. Anlam: yalnızca dolu workspace'lerde kaydırmayı etkinleştiren seçenek.
settings-choice-workspace-scroll-occupied = Occupied only

# Nerede: Bar sayfası, Workspace kaydırma ayarının segmentleri. Anlam: widget üstünde kaydırmayı kapatır.
settings-choice-workspace-scroll-off = Off

## Seçenekler: Girdi

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının segmentleri. Anlam: ivmenin uyarlanır profilde çalıştığı, hızlı hareketle işaretçinin daha uzağa gittiği seçenek.
settings-choice-accel-adaptive = Adaptive

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının segmentleri. Anlam: ivmenin kapatıldığı, sabit hızda hareket eden seçenek.
settings-choice-accel-flat = Flat

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: iki parmakla kaydırmayı etkinleştiren seçenek.
settings-choice-scroll-method-2fg = Two-finger

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: yüzeyin kenarında kaydırmayı etkinleştiren seçenek.
settings-choice-scroll-method-edge = Edge

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: düğme basılı tutulurken kaydıran seçenek.
settings-choice-scroll-method-on-button-down = On button press

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının segmentleri. Anlam: kaydırmayı tamamen kapatan seçenek.
settings-choice-scroll-method-no-scroll = Off

## Görünüm sayfası — Önizleme ve Renkler

# Nerede: Görünüm ve Yazı sayfalarının en üstündeki canlı önizleme kartının başlığı. Anlam: seçili ayarların önizlemesini gösteren kart.
settings-preview-label = Preview

# Nerede: Görünüm sayfası, Önizleme kartının arama sözcükleri. Anlam: "canlı" ya da "örnek" yazılınca kart bulunur.
settings-preview-keywords = live, sample

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: renk ayarlarını toplayan bölüm.
settings-section-colors = Colors

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: çerçeve, bar ve panel zemininin rengini seçen ayarın başlığı.
settings-colors-surface-label = Surface

# Nerede: Görünüm sayfası, Yüzey ayarının altındaki açıklama. Anlam: rengin nerede kullanıldığını söyler.
settings-colors-surface-description = Background of the frame, bar and panels

# Nerede: Görünüm sayfası, Yüzey ayarının arama sözcükleri. Anlam: "arka plan" gibi terimlerle ayar bulunur.
settings-colors-surface-keywords = background, base

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: kart zemini rengini seçen ayarın başlığı.
settings-colors-surface-container-label = Card

# Nerede: Görünüm sayfası ve Köşeler bölümündeki kart ayarlarının altındaki açıklama. Anlam: rengin ve köşe yarıçapının kartlara uygulandığını söyler.
settings-panel-cards-description = Cards inside panels

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: hata rengini seçen ayarın başlığı.
settings-colors-error-label = Error

# Nerede: Görünüm sayfası, Hata ayarının altındaki açıklama. Anlam: rengin nerede kullanıldığını söyler.
settings-colors-error-description = Failed actions, warnings

# Nerede: Görünüm sayfası, Hata ayarının arama sözcükleri. Anlam: "kırmızı" gibi terimlerle ayar bulunur.
settings-colors-error-keywords = red, warning

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: ayırıcı ve kenarlık rengini seçen ayarın başlığı.
settings-colors-outline-label = Line

# Nerede: Görünüm sayfası, Çizgi ayarının altındaki açıklama. Anlam: rengin nerede kullanıldığını söyler.
settings-colors-outline-description = Separators and borders

# Nerede: Görünüm sayfası, Çizgi ayarının arama sözcükleri. Anlam: "kenarlık" terimiyle ayar bulunur.
settings-colors-outline-keywords = border

# Nerede: Görünüm sayfası, Renkler bölümü. Anlam: gölgelerin rengini seçen ayarın başlığı.
settings-colors-shadow-label = Shadow color

# Nerede: Görünüm sayfası, Gölge rengi ayarının altındaki açıklama. Anlam: rengin neyi boyadığını söyler.
settings-colors-shadow-description = The color of the shadows

## Görünüm sayfası — Köşeler, Gölge ve Şekiller

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: köşe yarıçapı ayarlarını toplayan bölüm.
settings-section-corners = Corners

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: küçük öğelerin köşe yarıçapı ayarının başlığı.
settings-shape-radius-small-label = Small elements

# Nerede: Görünüm sayfası, Köşeler bölümündeki üç yarıçap ayarının arama sözcükleri. Anlam: "yarıçap", "yuvarlak" gibi terimlerle ayarlar bulunur.
settings-shape-radius-keywords = radius, round

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: kartların köşe yarıçapı ayarının başlığı.
settings-shape-radius-medium-label = Cards

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: panel ve pencere köşe yarıçapı ayarının başlığı.
settings-shape-radius-large-label = Panels

# Nerede: Görünüm sayfası, Paneller ayarının altındaki açıklama. Anlam: yarıçapın hangi öğelere uygulandığını söyler.
settings-shape-radius-large-description = Panels and windows

# Nerede: Görünüm sayfası, Köşeler bölümü. Anlam: yüzey kenarlıklarının kalınlığını seçen ayarın başlığı.
settings-shape-outlines-label = Outlines

# Nerede: Görünüm sayfası, Kenar çizgileri ayarının arama sözcükleri. Anlam: "kontrast", "outline" gibi terimlerle ayar bulunur.
settings-shape-outlines-keywords = contrast, accessibility, border, outline

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: gölge ayarlarını toplayan bölüm.
settings-section-shadow = Shadow

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölgeleri açıp kapatan ayarın başlığı.
settings-shadow-enabled-label = Shadows

# Nerede: Görünüm sayfası, Gölgeler ayarının altındaki açıklama. Anlam: gölgenin nerede göründüğünü söyler.
settings-shadow-enabled-description = Soft shadow under the bar, panels and menus

# Nerede: Görünüm sayfası, Gölgeler ayarının arama sözcükleri. Anlam: "shadow", "derinlik" terimleriyle ayar bulunur.
settings-shadow-enabled-keywords = shadow, depth

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölge bulanıklığını ayarlayan denetimin başlığı.
settings-shadow-blur-label = Softness

# Nerede: Görünüm sayfası, Yumuşaklık ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-shadow-blur-description = The shadow's blur radius

# Nerede: Görünüm sayfası, Yumuşaklık ayarının arama sözcükleri. Anlam: "blur", "bulanık" terimleriyle ayar bulunur.
settings-shadow-blur-keywords = blur

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölgenin dikey kaymasını ayarlayan denetimin başlığı.
settings-shadow-offset-y-label = Vertical offset

# Nerede: Görünüm sayfası, Dikey kayma ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-shadow-offset-y-description = How far the shadow drops down

# Nerede: Görünüm sayfası, Dikey kayma ayarının arama sözcükleri. Anlam: "offset" terimiyle ayar bulunur.
settings-shadow-offset-y-keywords = offset

# Nerede: Görünüm sayfası, Gölge bölümü. Anlam: gölge opaklığını ayarlayan denetimin başlığı.
settings-shadow-opacity-label = Intensity

# Nerede: Görünüm sayfası, Yoğunluk ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-shadow-opacity-description = The shadow's opacity

# Nerede: Görünüm sayfası, Yoğunluk ayarının arama sözcükleri. Anlam: "opacity", "koyuluk" terimleriyle ayar bulunur.
settings-shadow-opacity-keywords = opacity, darkness

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: maskot Fiyu'yla ilgili ayarı toplayan bölüm.
settings-section-mascot = Mascot

# Nerede: Görünüm sayfası, Maskot bölümündeki anahtarın başlığı. Anlam: açılınca maskot Fiyu hiçbir yerde gösterilmez. "Fiyu" maskotun özel adıdır, çevrilmez.
settings-appearance-mascot-label = Hide Fiyu

# Nerede: Görünüm sayfası, "Fiyu'yu gizle" anahtarının altındaki açıklama. Anlam: maskotun nerelerde göründüğünü söyler.
settings-appearance-mascot-description = The mascot on empty screens and error messages

# Nerede: Görünüm sayfası, "Fiyu'yu gizle" anahtarının arama sözcükleri. Anlam: bu sözcüklerle maskot ayarı bulunur.
settings-appearance-mascot-keywords = mascot, fiyu, bird, swift

# Nerede: Görünüm sayfasındaki bölüm başlığı. Anlam: çerçeve birleşimi ve esneme ayarlarını toplayan bölüm.
settings-section-shapes = Shapes

# Nerede: Görünüm sayfası, Şekiller bölümü. Anlam: panel esnemesini ayarlayan denetimin başlığı.
settings-blob-deform-label = Stretch

# Nerede: Görünüm sayfası, Esneme ayarının arama sözcükleri. Anlam: "deform", "jöle" gibi terimlerle ayar bulunur.
settings-blob-deform-keywords = blob, deform, jelly

## Yazı sayfası

# Nerede: Yazı sayfası, Önizleme kartının arama sözcükleri. Anlam: "örnek", "font" terimleriyle kart bulunur.
settings-typography-preview-keywords = sample, font

# Nerede: Yazı ve Hareket sayfalarındaki bölüm başlığı. Anlam: bir sayfanın genel ayarlarını toplayan bölüm.
settings-section-general = General

# Nerede: Yazı sayfası, Genel bölümü. Anlam: taban yazı boyutu ayarının başlığı.
settings-typography-size-label = Font size

# Nerede: Yazı sayfası, Yazı boyutu ayarının arama sözcükleri. Anlam: "font", "punto" gibi terimlerle ayar bulunur.
settings-typography-size-keywords = font, point, size

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: ikon seti ve ikon fontu ayarlarını toplayan bölüm.
settings-section-icons = Icons

# Nerede: Yazı sayfası, İkonlar bölümü. Anlam: ikon çizim setini seçen ayarın başlığı.
settings-typography-icon-set-label = Icon set

# Nerede: Yazı sayfası, İkon seti ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-typography-icon-set-description = The drawing set for all interface icons

# Nerede: Yazı sayfası, İkon seti ayarının arama sözcükleri. Anlam: "ikon", "nerd font" gibi terimlerle ayar bulunur.
settings-typography-icon-set-keywords = icon, symbol, material, nerd font

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: arayüz yazı tipi rolünü toplayan bölüm.
settings-section-interface = Interface

# Nerede: Yazı sayfası, Arayüz, Başlık, Belge, Eş aralıklı ve İkonlar bölümlerindeki font seçicilerin başlığı. Anlam: rolün yazı tipini seçen denetim.
settings-typography-family-label = Font

# Nerede: Yazı sayfası, Arayüz rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-ui-description = Buttons, menus, setting rows

# Nerede: Yazı sayfası, Arayüz font seçicinin arama sözcükleri. Anlam: "font", "weight" gibi terimlerle ayar bulunur.
settings-typography-ui-keywords = font, family, weight, inter, sans

# Nerede: Yazı sayfası, rol ölçek ayarlarının başlığı. Anlam: roldeki yazıların boyut çarpanını ayarlayan denetim.
settings-typography-scale-label = Size scale

# Nerede: Yazı sayfası, rol ölçek ayarlarının altındaki açıklama. Anlam: değerin neyi çarptığını söyler.
settings-typography-scale-description = Multiplier for all text in this role

# Nerede: Yazı sayfası, rol ölçek ayarlarının arama sözcükleri. Anlam: "ölçek", "büyüklük" gibi terimlerle ayar bulunur.
settings-typography-scale-keywords = font, size, scale

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: saat ve büyük başlık rolünü toplayan bölüm.
settings-section-display = Display

# Nerede: Yazı sayfası, Başlık rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-display-description = Clock, large headings

# Nerede: Yazı sayfası, Başlık font seçicinin arama sözcükleri. Anlam: "saat", "display" gibi terimlerle ayar bulunur.
settings-typography-display-keywords = font, family, weight, clock, display

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: uzun metin rolünü toplayan bölüm.
settings-section-document = Document

# Nerede: Yazı sayfası, Belge rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-document-description = Long text: notification bodies, calendar notes

# Nerede: Yazı sayfası, Belge font seçicinin arama sözcükleri. Anlam: "serif", "okuma" gibi terimlerle ayar bulunur.
settings-typography-document-keywords = font, family, weight, serif, reading

# Nerede: Yazı sayfasındaki bölüm başlığı. Anlam: eş aralıklı yazı rolünü toplayan bölüm.
settings-section-mono = Monospace

# Nerede: Yazı sayfası, Eş aralıklı rolünün altındaki açıklama. Anlam: rolün nerede kullanıldığını söyler.
settings-typography-mono-description = Code and numeric tables; also the font of the Nerd Font icon set

# Nerede: Yazı sayfası, Eş aralıklı font seçicinin arama sözcükleri. Anlam: "monospace", "kod" gibi terimlerle ayar bulunur.
settings-typography-mono-keywords = font, family, weight, monospace, code, nerd font

# Nerede: Yazı sayfası, İkonlar bölümündeki font seçicinin altındaki açıklama. Anlam: boş bırakılırsa ne olacağını söyler.
settings-typography-icon-description = Empty: the icon set's font is used

# Nerede: Yazı sayfası, İkon font seçicinin arama sözcükleri. Anlam: "material symbols" gibi terimlerle ayar bulunur.
settings-typography-icon-keywords = font, symbol, weight, material symbols

# Nerede: Yazı sayfası, İkon ölçeği ayarının altındaki açıklama. Anlam: değerin neyi çarptığını söyler.
settings-typography-icon-scale-description = Size multiplier for icons

## Hareket sayfası

# Nerede: Hareket sayfasındaki bölüm başlığı. Anlam: ön ayar seçimini toplayan bölüm.
settings-section-preset = Preset

# Nerede: Hareket sayfası, Genel bölümü. Anlam: animasyonları azaltan erişilebilirlik ayarının başlığı.
settings-motion-enabled-label = Reduce motion

# Nerede: Hareket sayfası, Hareketi azalt ayarının arama sözcükleri. Anlam: "reduce motion" terimiyle ayar bulunur.
settings-motion-enabled-keywords = animation, accessibility, reduce motion

# Nerede: Hareket sayfası, Genel bölümü. Anlam: animasyon hız çarpanını ayarlayan denetimin başlığı.
settings-motion-speed-label = Speed

# Nerede: Hareket sayfası, Hız ayarının arama sözcükleri. Anlam: "animasyon hızı", "speed" terimleriyle ayar bulunur.
settings-motion-speed-keywords = animation speed, speed, duration

# Nerede: Hareket sayfasındaki üç rol kartının ve bölüm başlığının arama sözcükleri. Anlam: "animasyon", "önizleme" terimleriyle roller bulunur.
settings-motion-role-keywords = animation, preview

# Nerede: Hareket sayfası, rol süre ayarlarının başlığı. Anlam: yayın süresini ya da eğrinin toplam süresini ayarlayan denetim.
settings-motion-role-duration-ms-label = Duration

# Nerede: Hareket sayfası, rol Süre (yay) ayarlarının arama sözcükleri. Anlam: "response", "hız" terimleriyle ayar bulunur.
settings-motion-role-response-keywords = response, spring, speed

# Nerede: Hareket sayfası, rol Eğri ayarlarının başlığı. Anlam: hız profili eğrisini seçen denetim.
settings-motion-role-easing-label = Curve

# Nerede: Hareket sayfası, rol Eğri ayarlarının arama sözcükleri. Anlam: "easing", "bezier" terimleriyle ayar bulunur.
settings-motion-role-easing-keywords = easing, bezier, cubic-bezier

# Nerede: Hareket sayfası, rol Süre (eğri) ayarlarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-motion-role-duration-ms-description = The curve's total duration

# Nerede: Hareket sayfası, rol Süre (eğri) ayarlarının arama sözcükleri. Anlam: "duration", "ms" terimleriyle ayar bulunur.
settings-motion-role-duration-ms-keywords = duration, ms

## Shell sayfası — Çerçeve

# Nerede: Shell sayfasındaki bölüm başlığı. Anlam: ekran çerçevesi ayarlarını toplayan bölüm.
settings-section-frame = Frame

# Nerede: Shell sayfası, Çerçeve bölümü. Anlam: ekran çerçevesini açıp kapatan ayarın başlığı.
settings-frame-enabled-label = Screen frame

# Nerede: Shell sayfası, Ekran çerçevesi ayarının arama sözcükleri. Anlam: "kenar", "border" terimleriyle ayar bulunur.
settings-frame-enabled-keywords = edge, border, frame

# Nerede: Shell sayfası, Çerçeve bölümü. Anlam: kenar bazında kalınlık ayarını açan satırın başlığı.
settings-frame-edges-label = Edges

# Nerede: Shell sayfası, Kenarlar ayarının arama sözcükleri. Anlam: "üst", "sol" gibi terimlerle ayar bulunur.
settings-frame-edges-keywords = top, bottom, left, right, thickness, bar

# Nerede: Shell sayfası, Çerçeve bölümü. Anlam: iç köşe kavisini gösteren bağlantı satırının başlığı.
settings-frame-inner-corners-label = Inner corners

# Nerede: Shell sayfası, İç köşeler satırının arama sözcükleri. Anlam: "iç kavis", "radius" terimleriyle satır bulunur.
settings-frame-inner-corners-keywords = inner curve, corner, radius, round, inner_radius

## Bar sayfası

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: barın konum ayarlarını toplayan bölüm.
settings-section-position = Position

# Nerede: Bar sayfası, Konum bölümü. Anlam: barın hangi ekran kenarında olduğunu seçen ayarın başlığı.
settings-bar-edge-label = Screen edge

# Nerede: Bar sayfası, Ekran kenarı ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-edge-description = The screen edge the bar sits on

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: barın boyut ayarlarını toplayan bölüm.
settings-section-size = Size

# Nerede: Bar sayfası, Uç boşluğu ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-padding-description = Inner padding at both ends of the bar

# Nerede: Bar sayfası, Boyut bölümü. Anlam: widget'lar arası boşluğu ayarlayan denetimin başlığı.
settings-bar-spacing-label = Widget spacing

# Nerede: Bar sayfası, Widget aralığı ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-spacing-description = The gap between widgets

# Nerede: Bar sayfası, Widget aralığı ayarının arama sözcükleri. Anlam: "spacing", "boşluk" terimleriyle ayar bulunur.
settings-bar-spacing-keywords = spacing, gap

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: bar bölümlerini düzenleyen satırı toplayan bölüm.
settings-section-widgets = Widgets

# Nerede: Bar sayfası, Widget'lar bölümü. Anlam: bar bölümlerini düzenleyen düzenleyicinin başlığı.
settings-bar-sections-label = Bar sections

# Nerede: Bar sayfası, Bar bölümleri ayarının altındaki açıklama. Anlam: düzenleyicinin nasıl kullanıldığını anlatır.
settings-bar-sections-description = Add, remove and drag widgets to reorder.

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: bar uçlarına tık ve kaydırma eylemi atan ayarları toplayan bölüm.
settings-section-edge-actions = Edge actions

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: baştaki widget'a sol tık eylemi seçen ayarın başlığı.
settings-bar-start-left-click-label = Start widget · left click

# Nerede: Bar sayfası, baştaki widget sol tık ayarının altındaki açıklama. Anlam: hangi widget'ın "baş" olduğunu anlatır.
settings-bar-start-left-click-description = On a horizontal bar the leftmost, on a vertical bar the topmost widget

# Nerede: Bar sayfası, baştaki ve sondaki widget sol tık ayarlarının arama sözcükleri. Anlam: "panel aç" gibi terimlerle ayar bulunur.
settings-bar-edge-click-open-keywords = edge, click, action, open panel

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: baştaki widget'a sağ tık eylemi seçen ayarın başlığı.
settings-bar-start-right-click-label = Start widget · right click

# Nerede: Bar sayfası, baştaki ve sondaki widget sağ tık ayarlarının arama sözcükleri. Anlam: "tık", "eylem" terimleriyle ayar bulunur.
settings-bar-edge-click-keywords = edge, click, action

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: baştaki widget'a kaydırma eylemi seçen ayarın başlığı.
settings-bar-start-scroll-label = Start widget · scroll

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: sondaki widget'a sol tık eylemi seçen ayarın başlığı.
settings-bar-end-left-click-label = End widget · left click

# Nerede: Bar sayfası, sondaki widget sol tık ayarının altındaki açıklama. Anlam: hangi widget'ın "son" olduğunu anlatır.
settings-bar-end-left-click-description = On a horizontal bar the rightmost, on a vertical bar the bottommost widget

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: sondaki widget'a sağ tık eylemi seçen ayarın başlığı.
settings-bar-end-right-click-label = End widget · right click

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: sondaki widget'a kaydırma eylemi seçen ayarın başlığı.
settings-bar-end-scroll-label = End widget · scroll

# Nerede: Bar sayfası, baştaki ve sondaki widget kaydırma ayarlarının altındaki açıklama. Anlam: kaydırma eyleminin geri bildirimini anlatır.
settings-bar-edge-scroll-description = For volume, microphone or brightness an icon appears beside the widget's outer side.

# Nerede: Bar sayfası, baştaki ve sondaki widget kaydırma ayarlarının arama sözcükleri. Anlam: "tekerlek", "ses" gibi terimlerle ayar bulunur.
settings-bar-edge-scroll-keywords = edge, wheel, scroll, volume, brightness, workspace

# Nerede: Bar sayfası, Uç eylemleri bölümü. Anlam: bir kaydırma adımının değiştirdiği yüzeyi ayarlayan denetimin başlığı.
settings-bar-scroll-step-label = Scroll step

# Nerede: Bar sayfası, Kaydırma adımı ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-bar-scroll-step-description = The percentage a single wheel notch changes

# Nerede: Bar sayfası, Kaydırma adımı ayarının arama sözcükleri. Anlam: "adım", "tekerlek" terimleriyle ayar bulunur.
settings-bar-scroll-step-keywords = step, wheel, volume, brightness

# Nerede: Bar sayfasındaki bölüm başlığı. Anlam: widget'a özel ayarları toplayan bölüm.
settings-section-widget-settings = Widget settings

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: sayfa başına workspace sayısını ayarlayan denetimin başlığı.
settings-bar-workspaces-count-label = Workspaces per page

# Nerede: Bar sayfası, Sayfa başına workspace ayarının altındaki açıklama. Anlam: değerin nasıl çalıştığını anlatır.
settings-bar-workspaces-count-description = Slots always shown; extras go to the next page with the arrows.

# Nerede: Bar sayfası, Sayfa başına workspace ayarının arama sözcükleri. Anlam: "çalışma alanı" terimiyle ayar bulunur.
settings-bar-workspaces-count-keywords = workspace, count
## Bar sayfası — Widget ayarları

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: workspace'lerin (çalışma alanlarının) nasıl gösterileceğini seçen ayarın başlığı.
settings-bar-workspaces-style-label = Workspace display

# Nerede: Bar sayfası, Workspace görünümü ayarının altındaki açıklama. Anlam: seçilebilecek gösterim biçimlerini sıralar.
settings-bar-workspaces-style-description = App icon, number or dot

# Nerede: Bar sayfası, Workspace görünümü ayarının arama sözcükleri. Anlam: "çalışma alanı", "ikon" gibi terimlerle ayar bulunur.
settings-bar-workspaces-style-keywords = workspace, icon, number, dot

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: widget üstünde kaydırınca workspace değiştirmeyi ayarlayan denetimin başlığı.
settings-bar-workspaces-scroll-label = Workspace scrolling

# Nerede: Bar sayfası, Workspace kaydırma ayarının altındaki açıklama. Anlam: kaydırma eyleminin nerede çalıştığını söyler.
settings-bar-workspaces-scroll-description = Switch workspaces with the wheel on the widget

# Nerede: Bar sayfası, Workspace kaydırma ayarının arama sözcükleri. Anlam: "tekerlek", "scroll" terimleriyle ayar bulunur.
settings-bar-workspaces-scroll-keywords = workspace, wheel, scroll

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: bar saat widget'ının biçimini ayarlayan metin alanının başlığı.
settings-bar-clock-format-label = Clock format

# Nerede: Bar sayfası, Saat biçimi ayarının altındaki açıklama. Anlam: kabul edilen strftime kalıplarına örnek verir.
settings-bar-clock-format-description = strftime: %H:%M, %a %d %b %H:%M…

# Nerede: Bar sayfası, Saat biçimi ayarının arama sözcükleri. Anlam: "saat", "format" terimleriyle ayar bulunur.
settings-bar-clock-format-keywords = clock, date, format

# Nerede: Bar sayfası, Widget ayarları bölümü. Anlam: sistem tepsisi ikonlarının boyutunu ayarlayan denetimin başlığı.
settings-bar-tray-icon-size-label = Tray icon size

# Nerede: Bar sayfası, Tepsi ikon boyutu ayarının altındaki açıklama. Anlam: değerin hangi ikonları etkilediğini söyler.
settings-bar-tray-icon-size-description = Icons in the system tray

# Nerede: Bar sayfası, Tepsi ikon boyutu ayarının arama sözcükleri. Anlam: "tray" terimiyle ayar bulunur.
settings-bar-tray-icon-size-keywords = tray, icon

## Shell sayfası — Paneller

# Nerede: Shell sayfasındaki bölüm başlığı. Anlam: dört kenardaki panel ayarlarını toplayan bölüm.
settings-section-panels = Panels

# Nerede: Shell sayfası, Paneller bölümü. Anlam: sağ paneli açıp kapatan ayarın başlığı.
settings-panels-right-label = Right panel

# Nerede: Shell sayfası, Sağ panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-right-description = A drawer that opens from the right edge of the screen

# Nerede: Shell sayfası, dört kenar panelini açıp kapatan ayarlarının arama sözcükleri. Anlam: "çekmece", "kenar paneli" terimleriyle ayarlar bulunur.
settings-panels-enabled-keywords = drawer, edge panel

# Nerede: Shell sayfası, dört kenar panelinin derinlik ayarlarının altındaki açıklama. Anlam: değerin yönelime göre genişlik ya da yükseklik olduğunu anlatır.
settings-panels-size-description = Panel width (height for top and bottom panels)

# Nerede: Shell sayfası, dört kenar panelinin derinlik ayarlarının arama sözcükleri. Anlam: "genişlik", "boyut" terimleriyle ayarlar bulunur.
settings-panels-size-keywords = width, size

# Nerede: Shell sayfası, dört kenar panelinin içerik ayarlarının altındaki açıklama. Anlam: düzenleyicinin nasıl kullanıldığını anlatır.
settings-panels-widgets-description = Add, remove and drag cards to reorder (top to bottom).

# Nerede: Shell sayfası, dört kenar panelinin içerik ayarlarının arama sözcükleri. Anlam: "kart", "hızlı ayarlar" terimleriyle ayarlar bulunur.
settings-panels-widgets-keywords = card, widget, quick settings, media, notification, order

# Nerede: Shell sayfası, Paneller bölümü. Anlam: sol paneli açıp kapatan ayarın başlığı.
settings-panels-left-label = Left panel

# Nerede: Shell sayfası, Sol panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-left-description = A drawer that opens from the left edge of the screen

# Nerede: Shell sayfası, Paneller bölümü. Anlam: üst paneli açıp kapatan ayarın başlığı.
settings-panels-top-label = Top panel

# Nerede: Shell sayfası, Üst panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-top-description = A drawer that opens from the top edge of the screen

# Nerede: Shell sayfası, Paneller bölümü. Anlam: alt paneli açıp kapatan ayarın başlığı.
settings-panels-bottom-label = Bottom panel

# Nerede: Shell sayfası, Alt panel ayarının altındaki açıklama. Anlam: panelin ekranın hangi kenarından açıldığını anlatır.
settings-panels-bottom-description = A drawer that opens from the bottom edge of the screen

## Ekranlar sayfası

# Nerede: Ekranlar sayfasındaki bölüm başlığı. Anlam: monitör düzeni ve görüntü ayarlarını toplayan bölüm.
settings-section-displays = Monitors

# Nerede: Ekranlar sayfasındaki ayar satırının başlığı; satır ekran düzenleyiciyi açar. Anlam: monitör yerleşimi ve görüntü ayarları.
settings-displays-label = Monitors

# Nerede: Ekranlar sayfası, Monitörler satırının altındaki açıklama. Anlam: düzenleyicideki ayarları özetler.
settings-displays-description = Layout, resolution, refresh rate, scale, rotation, mirroring

# Nerede: Ekranlar sayfası, Monitörler satırının arama sözcükleri. Anlam: "çözünürlük", "yansıt" gibi terimlerle sayfa bulunur.
settings-displays-keywords = screen, monitor, resolution, refresh, hz, scale, rotate, mirror, vrr, primary, hdmi, dp

## Pencereler sayfası

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: pencere aralığı ayarlarını toplayan bölüm.
settings-section-gaps = Gaps

# Nerede: Pencereler sayfası, Pencereler arası ayarının arama sözcükleri. Anlam: "boşluk", "döşeme" terimleriyle ayar bulunur.
settings-windows-gap-inner-keywords = gap, spacing, tiling

# Nerede: Pencereler sayfası, Ekran kenarına ayarının arama sözcükleri. Anlam: "kenar", "gap" terimleriyle ayar bulunur.
settings-windows-gap-outer-keywords = gap, spacing, edge

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: pencere kenarlığı ayarını toplayan bölüm.
settings-section-border = Border

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: pencere bulanıklığı ayarlarını toplayan bölüm.
settings-section-blur = Blur

# Nerede: Pencereler sayfası, bulanıklık yoğunluğu ayarının arama sözcükleri. Anlam: "yarıçap" terimiyle ayar bulunur.
settings-windows-blur-size-keywords = blur, blurry, radius

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: temayla paylaşılan görünüm ayarlarını toplayan bölüm.
settings-section-theme-shared = Shared with theme

# Nerede: Pencereler sayfası, Gölge satırının arama sözcükleri. Anlam: "gölge", "derinlik" terimleriyle satır bulunur.
settings-windows-shadow-keywords = shadow, depth

# Nerede: Pencereler sayfasındaki bölüm başlığı. Anlam: masaüstü eşitleme ayarlarını toplayan bölüm.
settings-section-advanced = Advanced

# Nerede: Pencereler sayfası, Gelişmiş bölümü. Anlam: köşe, gölge ve bulanıklığın temadan alınmasını açıp kapatan ayarın başlığı.
settings-compositor-sync-look-label = Let windows follow the theme

# Nerede: Pencereler sayfası, Pencereler temaya uysun ayarının arama sözcükleri. Anlam: "tema", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-look-keywords = theme, corner, shadow, blur, sync

# Nerede: Pencereler sayfası, Aralık ve kenarlığı Nuxut belirlesin ayarının arama sözcükleri. Anlam: "aralık", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-windows-keywords = gap, spacing, border, sync

# Nerede: Pencereler sayfası, Gelişmiş bölümü. Anlam: pencere animasyonlarının temadan alınmasını açıp kapatan ayarın başlığı.
settings-compositor-sync-motion-label = Let the theme drive window motion

# Nerede: Pencereler sayfası, Pencere hareketleri temaya uysun ayarının arama sözcükleri. Anlam: "animasyon", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-motion-keywords = animation, motion, sync

## Duvar kağıdı sayfası

# Nerede: Duvar kağıdı sayfasındaki bölüm başlığı. Anlam: duvar kağıdı ayarlarını toplayan bölüm.
settings-section-wallpaper = Wallpaper

# Nerede: Duvar kağıdı sayfasındaki ayar satırının başlığı; satır kağıt seçimini açar. Anlam: duvar kağıdı görseli.
settings-wallpaper-label = Wallpaper

# Nerede: Duvar kağıdı sayfası, Duvar kağıdı bölümü. Anlam: görselin ekrana nasıl sığdırılacağını seçen ayarın başlığı.
settings-wallpaper-fit-label = Fit

# Nerede: Duvar kağıdı sayfası, Yerleşim ayarının arama sözcükleri. Anlam: "sığdır", "kırp" terimleriyle ayar bulunur.
settings-wallpaper-fit-keywords = fit, fill, crop, scale

# Nerede: Duvar kağıdı sayfası, Duvar kağıdı bölümü. Anlam: monitör başına kağıt ayarını açan satırın başlığı.
settings-wallpaper-outputs-label = Per monitor

# Nerede: Duvar kağıdı sayfası, Monitör başına satırının altındaki açıklama. Anlam: ayarın kapsamını anlatır.
settings-wallpaper-outputs-description = A separate image or fit per monitor

# Nerede: Duvar kağıdı sayfası, Monitör başına satırının arama sözcükleri. Anlam: "monitör", "çoklu" terimleriyle satır bulunur.
settings-wallpaper-outputs-keywords = monitor, screen, multiple, connector, hdmi, dp

# Nerede: Duvar kağıdı sayfasındaki bölüm başlığı. Anlam: renk şeması ayarlarını toplayan bölüm.
settings-section-color-scheme = Color scheme

# Nerede: Duvar kağıdı sayfası, Renk şeması bölümü. Anlam: arayüz renklerinin duvar kağıdından türetilmesini seçen ayarın başlığı.
settings-palette-source-label = Take colors from the wallpaper

# Nerede: Duvar kağıdı sayfası, Renk şeması bölümü. Anlam: duvar kağıdından türeyen palet varyantını seçen ayarın başlığı.
settings-palette-label = Scheme

# Nerede: Duvar kağıdı sayfası, Şema ayarının altındaki açıklama. Anlam: seçilebilecek palet çeşitliliğini anlatır.
settings-palette-scheme-description = Nine different palettes from the same wallpaper

# Nerede: Duvar kağıdı sayfası, Şema ayarının arama sözcükleri. Anlam: "palet", "tonal" terimleriyle ayar bulunur.
settings-palette-scheme-keywords = matugen, palette, variant, tonal, vivid

# Nerede: Duvar kağıdı sayfası, Kip ayarının altındaki açıklama. Anlam: değerin neyi değiştirdiğini söyler.
settings-palette-mode-description = A dark or light interface

## Hesaplar sayfası

# Nerede: Hesaplar sayfasındaki bölüm başlığı. Anlam: takvim hesabı ayarlarını toplayan bölüm.
settings-section-accounts = Calendar accounts

# Nerede: Hesaplar sayfasındaki ayar satırının başlığı; satır hesap yöneticisini açar. Anlam: takvim hesaplarını yöneten düzenleyici.
settings-accounts-label = Calendar accounts

# Nerede: Hesaplar sayfası, Takvim hesapları satırının altındaki açıklama. Anlam: desteklenen hesap türlerini sıralar.
settings-accounts-description = Google, Microsoft, Nextcloud, CalDAV, ICS

# Nerede: Hesaplar sayfası, Takvim hesapları satırının arama sözcükleri. Anlam: "takvim", "outlook" terimleriyle sayfa bulunur.
settings-accounts-keywords = calendar, google, outlook, nextcloud, icloud, caldav

## Bildirimler sayfası

# Nerede: Bildirimler sayfası, Genel bölümü. Anlam: shell bildirim sunucusunu açıp kapatan ayarın başlığı.
settings-notifications-label = Notifications

# Nerede: Bildirimler sayfası, Bildirimler ayarının altındaki açıklama. Anlam: kapattığında ne olacağını anlatır.
settings-notifications-description = When off, the shell runs no notification server (another one may be running).

# Nerede: Bildirimler sayfası, Bildirimler ayarının arama sözcükleri. Anlam: "bildirim sunucusu" terimiyle ayar bulunur.
settings-notifications-keywords = notification server

# Nerede: Bildirimler sayfası, Genel bölümü. Anlam: açılır bildirimleri susturan ayarın başlığı.
settings-notifications-dnd-label = Do Not Disturb

# Nerede: Bildirimler sayfası, Rahatsız etme ayarının arama sözcükleri. Anlam: "dnd", "sessiz" terimleriyle ayar bulunur.
settings-notifications-dnd-keywords = dnd, silent, mute

# Nerede: Bildirimler sayfasındaki bölüm başlığı. Anlam: açılır bildirim ayarlarını toplayan bölüm.
settings-section-popup-notifications = Pop-up notifications

# Nerede: Bildirimler sayfası açılır bildirim ayarında ve Ses sayfası ekran göstergesi ayarında kaydırıcı başlığı. Anlam: öğenin ekranda kalma süresi.
settings-display-duration-label = Display duration

# Nerede: Bildirimler sayfası, Görünme süresi ayarının altındaki açıklama. Anlam: değerin hangi bildirimler için geçerli olduğunu söyler.
settings-notifications-popup-timeout-description = How long normal notifications stay on screen

# Nerede: Bildirimler sayfası, Görünme süresi ayarının arama sözcükleri. Anlam: "popup", "zaman aşımı" terimleriyle ayar bulunur.
settings-notifications-popup-timeout-keywords = popup, duration, timeout

# Nerede: Bildirimler sayfası, Açılır bildirimler bölümü. Anlam: aynı anda görünecek açılır bildirim sayısını sınırlayan ayarın başlığı.
settings-notifications-max-popups-label = Maximum at once

# Nerede: Bildirimler sayfası, Aynı anda en çok ayarının arama sözcükleri. Anlam: "popup", "yığın" terimleriyle ayar bulunur.
settings-notifications-max-popups-keywords = popup, count, stack

# Nerede: Bildirimler sayfasındaki bölüm başlığı. Anlam: bildirim geçmişi ayarlarını toplayan bölüm.
settings-section-history = History

# Nerede: Bildirimler sayfası, Geçmiş bölümü. Anlam: merkezde saklanan bildirim sayısını sınırlayan ayarın başlığı.
settings-notifications-history-limit-label = Stored notifications

# Nerede: Bildirimler sayfası, Saklanan bildirim ayarının altındaki açıklama. Anlam: değerin nerede geçerli olduğunu söyler.
settings-notifications-history-limit-description = The maximum number of notifications kept in the notification center

# Nerede: Bildirimler sayfası, Saklanan bildirim ayarının arama sözcükleri. Anlam: "geçmiş", "history" terimleriyle ayar bulunur.
settings-notifications-history-limit-keywords = history, center

# Nerede: Bildirimler sayfasındaki bölüm başlığı. Anlam: uygulama bazlı bildirim ayarlarını toplayan bölüm.
settings-section-apps = Apps

# Nerede: Bildirimler sayfası, Uygulamalar bölümü. Anlam: uygulama bazlı bildirim ayarını açan satırın başlığı.
settings-notifications-apps-label = Per app

# Nerede: Bildirimler sayfası, Uygulama başına satırının altındaki açıklama. Anlam: satırın ne yaptığını anlatır.
settings-notifications-apps-description = Turn off an app's pop-up notifications or history.

# Nerede: Bildirimler sayfası, Uygulama başına satırının arama sözcükleri. Anlam: "popup", "uygulama" terimleriyle satır bulunur.
settings-notifications-apps-keywords = popup, app

## Ses sayfası

# Nerede: Kısayollar sayfasında ses kısayollarının bölüm başlığı. Anlam: ses ayarlarını toplayan bölüm.
settings-section-sound = Sound

# Nerede: Ses sayfasının ilk kartının bölüm başlığı (en yüksek ses, uyarı, ses ve parlaklık göstergesi). Anlam: sayfa adı "Ses"i tekrarlamadan ses düzeyiyle ilgili ayarlar.
settings-section-sound-level = Sound level

# Nerede: Ses sayfası, Ses bölümü. Anlam: sistem genelinde en yüksek ses düzeyini sınırlayan ayarın başlığı.
settings-audio-max-volume-label = Maximum volume

# Nerede: Ses sayfası, En yüksek ses ayarının arama sözcükleri. Anlam: "volume", "boost" terimleriyle ayar bulunur.
settings-audio-max-volume-keywords = volume, boost, amplify

# Nerede: Ses sayfası, Ses bölümü. Anlam: ses yükseltmenin (boost) riskini anlatan uyarı satırının başlığı.
settings-audio-max-volume-warning-label = Warning

# Nerede: Ses sayfası, Uyarı satırının altındaki açıklama. Anlam: 100 üstü yükseltmenin olası zararını söyler.
settings-audio-max-volume-warning-description = Boosting above 100 can distort sound and damage speakers.

# Nerede: Ses sayfası, Uyarı satırının arama sözcükleri. Anlam: "volume" terimiyle satır bulunur.
settings-audio-max-volume-warning-keywords = volume

# Nerede: Ses sayfasındaki bölüm başlığı. Anlam: ekran göstergesi (OSD) ayarlarını toplayan bölüm.
settings-section-osd = On-screen display

# Nerede: Ses sayfası, Görünme süresi ayarının altındaki açıklama. Anlam: değerin hangi göstergeler için geçerli olduğunu söyler.
settings-osd-timeout-description = How long the indicator that appears when volume or brightness changes stays on screen

# Nerede: Ses sayfası, ekran göstergesi Görünme süresi ayarının arama sözcükleri. Anlam: "osd", "gösterge" terimleriyle ayar bulunur.
settings-osd-timeout-keywords = osd, indicator, duration

# Nerede: Ses sayfasındaki bölüm başlığı. Anlam: medya tuşu ayarlarını toplayan bölüm.
settings-section-media-keys = Media keys

# Nerede: Ses sayfası, Medya tuşları bölümü. Anlam: donanım tuşlarını Kısayollar sayfasına bağlayan satırın başlığı.
settings-media-keys-link-label = Sound and media keys

# Nerede: Ses sayfası, Ses ve medya tuşları satırının arama sözcükleri. Anlam: "xf86", "bind" terimleriyle satır bulunur.
settings-media-keys-link-keywords = xf86, shortcut, key, bind, brightness, microphone

# Nerede: Ses sayfası, Medya tuşları bölümü. Anlam: Hyprland'e elle bind ekleme örneğini gösteren satırın başlığı.
settings-media-keys-example-label = Hyprland bind example

# Nerede: Ses sayfası, Hyprland bind örneği satırının altındaki açıklama. Anlam: Nuxut kısayolları kapalıyken ne yapılması gerektiğini söyler.
settings-media-keys-example-description = Nuxut shortcuts are off; add binds to hyprland.conf yourself if you like.

# Nerede: Ses sayfası, Hyprland bind örneği satırının arama sözcükleri. Anlam: "playerctl", "bind" terimleriyle satır bulunur.
settings-media-keys-example-keywords = xf86, shortcut, bind, playerctl, brightness, microphone

## Klavye sayfası ve girdi durumu

# Nerede: Klavye sayfası, düzen listesinin altındaki metin düğme; seçici kapalıyken (src/pages/keyboard.rs). Anlam: Yeni düzen eklemek için arama alanlı seçiciyi açar.
settings-keyboard-add = Add layout

# Nerede: Klavye sayfası, düzen listesi boşken görünen açıklama; etkin liste okunamadığında (src/pages/keyboard.rs). Anlam: Etkin düzenlerin okunamadığı, kullanıcının eklediği düzenlerin yine de yazılacağı.
settings-keyboard-empty-effective = The active layouts could not be read; the layouts you add are written anyway.

# Nerede: Klavye sayfası, düzen listesi doluyken "Düzen ekle" düğmesinin yerine geçen açıklama (src/pages/keyboard.rs). Anlam: Listeye en çok bu kadar düzen eklenebileceği.
# Değişkenler: $count (en çok düzen sayısı, ör. 4).
settings-keyboard-max =
    { $count ->
        [one] Up to { $count } layout
       *[other] Up to { $count } layouts
    }

# Nerede: Klavye sayfası, "Düzen ekle" seçicisinin arama alanının yer tutucusu (src/pages/keyboard.rs). Anlam: Düzenlerin dil, ülke ya da kodla aranabileceği.
settings-keyboard-search-placeholder = Search layouts: language, country or code (tr, us, dvorak)

# Nerede: Klavye sayfası, seçici açıkken XKB kataloğu henüz yüklenmemişse görünen durum yazısı (src/pages/keyboard.rs). Anlam: Düzen listesinin yüklenmekte olduğu.
settings-keyboard-loading = Loading the layout list…

# Nerede: Klavye sayfası, seçici açıkken sorguya uyan düzen bulunamadığında görünen durum yazısı (src/pages/keyboard.rs). Anlam: Sorguya eşleşen düzen olmadığı.
settings-keyboard-no-match = No matching layouts

# Nerede: Klavye sayfası, Düzenler arası geçiş satırındaki seçim menüsünün ilk seçeneği (src/pages/keyboard.rs). Anlam: Düzen değiştirme seçeneğinin olmadığı.
settings-keyboard-switch-none = None

# Nerede: Klavye sayfası, tuş tekrarını denemek için yerel metin alanının yer tutucusu (src/pages/keyboard.rs). Anlam: Alanın deneme amaçlı olduğu, bir tuşun basılı tutularak tekrarın hissedilebileceği.
settings-keyboard-test-placeholder = Try here: hold down a key

# Nerede: Klavye ve Fare sayfalarının üstündeki sakin not; girdi eşitlemesi kapalıyken (src/pages/input_note.rs). Anlam: Girdi ayarlarının masaüstüne uygulanmadığı, masaüstünün kendi ayarının geçerli olduğu.
settings-input-sync-off = Keyboard and mouse settings are not applied to the desktop; the desktop's own settings are in effect.

# Nerede: Aynı notun sağındaki tonlu düğme (src/pages/input_note.rs). Anlam: Girdi eşitleme anahtarını varsayılanına (açık) döndürür.
settings-input-turn-on = Turn on

# Nerede: Klavye ve Fare sayfalarının üstündeki sakin not; shell çalışıyor ama masaüstüne (Hyprland'e) ulaşamıyor (src/services/input.rs). Anlam: Değerlerin varsayılanla gösterildiği, değişikliklerin masaüstü açılınca uygulanacağı.
settings-input-unavailable = The desktop could not be reached; values are shown with defaults, and changes are applied once the desktop is up.

# Nerede: Aynı not; masaüstü kabuğu çalışmıyorken (src/services/input.rs). Anlam: Değerlerin kabuk açılınca uygulanacağı.
settings-input-not-running = The desktop shell is not running; values are applied once the shell starts.

# Nerede: Aynı not; shell sürümü girdi durumunu bildirmiyorken (src/services/input.rs). Anlam: Kabuğun bu sürümünün girdi ayarlarını bildirmediği, değerlerin varsayılanla gösterildiği.
settings-input-unsupported = This version of the shell does not report input settings; values are shown with defaults.

# Nerede: Klavye ve Fare sayfalarında satırın altındaki geçersiz kılma uyarısı; dosya ve satır biliniyorken (src/services/input.rs). Anlam: Hyprland ayar dosyasındaki satırın bu değeri geçersiz kıldığı.
# Değişkenler: $file (dosya yolu, ör. "~/.config/hypr/hyprland.conf"), $line (satır numarası, ör. 12).
settings-input-override-file-line = Your Hyprland settings file overrides this value: { $file }:{ $line }

# Nerede: Aynı uyarı; dosya biliniyor ama satır bilinmiyorken (src/services/input.rs). Anlam: Hyprland ayar dosyasının bu değeri geçersiz kıldığı.
# Değişkenler: $file (dosya yolu, ör. "~/.config/hypr/hyprland.lua").
settings-input-override-file = Your Hyprland settings file overrides this value: { $file }

# Nerede: Aynı uyarı; dosya bilinmiyorken (Lua config), aranacak Hyprland anahtarı söylenirken (src/services/input.rs). Anlam: Hyprland ayarının bu değeri geçersiz kıldığı ve hangi anahtarın olduğu.
# Değişkenler: $key (Hyprland anahtarı, ör. "input:sensitivity").
settings-input-override-hypr = Your Hyprland settings override this value ({ $key }).

# Nerede: Aynı uyarı; ne dosya ne Hyprland anahtarı biliniyorken (src/services/input.rs). Anlam: Hyprland ayarının bu değeri geçersiz kıldığı.
settings-input-override = Your Hyprland settings override this value.

## Klavye sayfası

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: klavye ayarlarının uygulanma durumunu gösteren bölüm.
settings-section-status = Status

# Nerede: Klavye sayfası, Durum bölümü. Anlam: klavye ayarlarının uygulanıp uygulanmadığını bildiren satırın başlığı.
settings-keyboard-note-label = Keyboard settings status

# Nerede: Klavye sayfası, Klavye ayarlarının durumu satırının arama sözcükleri. Anlam: "masaüstü", "uygulanmıyor" terimleriyle satır bulunur.
settings-keyboard-note-keywords = desktop, shell, not applied

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: klavye düzeni ayarlarını toplayan bölüm.
settings-section-layouts = Layouts

# Nerede: Klavye sayfası, Düzenler bölümü. Anlam: etkin klavye düzenlerini seçen ayarın başlığı.
settings-input-keyboard-layouts-label = Keyboard layouts

# Nerede: Klavye sayfası, Klavye düzenleri ayarının arama sözcükleri. Anlam: "layout", "xkb", "q klavye" terimleriyle ayar bulunur.
settings-input-keyboard-layouts-keywords = keyboard, layout, xkb, variant, kb_layout, language, q keyboard, f keyboard

# Nerede: Klavye sayfası, Düzenler bölümü. Anlam: düzenler arasında geçiş yapan tuş birleşiminin başlığı.
settings-input-keyboard-switch-label = Switch between layouts

# Nerede: Klavye sayfası, Düzenler arası geçiş ayarının altındaki açıklama. Anlam: düzeni hangi tuş birleşimiyle değiştireceğini söyler.
settings-input-keyboard-switch-description = Key combination that changes the layout

# Nerede: Klavye sayfası, Düzenler arası geçiş ayarının arama sözcükleri. Anlam: "grp", "alt shift" terimleriyle ayar bulunur.
settings-input-keyboard-switch-keywords = keyboard, layout, switch, toggle, grp, kb_options, alt shift

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: tuş tekrarı ayarlarını toplayan bölüm.
settings-section-key-repeat = Key repeat

# Nerede: Klavye sayfası, Tuş tekrarı bölümü. Anlam: basılı tutulan tuşun bekleyip tekrarlamaya başladığı süreyi belirleyen ayarın başlığı.
settings-input-keyboard-repeat-delay-label = Repeat delay

# Nerede: Klavye sayfası, Tekrar gecikmesi ayarının altındaki açıklama. Anlam: gecikmenin ne anlama geldiğini söyler.
settings-input-keyboard-repeat-delay-description = A held key repeats after this delay

# Nerede: Klavye sayfası, Tekrar gecikmesi ayarının arama sözcükleri. Anlam: "repeat", "delay", "basılı tut" terimleriyle ayar bulunur.
settings-input-keyboard-repeat-delay-keywords = keyboard, repeat, delay, hold

# Nerede: Klavye sayfası, Tuş tekrarı bölümü. Anlam: saniyedeki tekrar sayısını belirleyen ayarın başlığı.
settings-input-keyboard-repeat-rate-label = Repeat rate

# Nerede: Klavye sayfası, Tekrar hızı ayarının altındaki açıklama. Anlam: hızın ölçüsünü açıklar.
settings-input-keyboard-repeat-rate-description = Repeats per second

# Nerede: Klavye sayfası, Tekrar hızı ayarının arama sözcükleri. Anlam: "repeat", "rate", "hız" terimleriyle ayar bulunur.
settings-input-keyboard-repeat-rate-keywords = keyboard, repeat, speed, rate

# Nerede: Klavye sayfası, Tuş tekrarı bölümü. Anlam: tuş tekrarını denemek için yazı alanının başlığı.
settings-input-keyboard-try-label = Try it here

# Nerede: Klavye sayfası, Burada deneyin satırının arama sözcükleri. Anlam: "deneme", "test" terimleriyle satır bulunur.
settings-input-keyboard-try-keywords = test, try, typing, repeat

# Nerede: Klavye sayfasındaki bölüm başlığı. Anlam: diğer klavye ayarlarını toplayan bölüm.
settings-section-other = Other

# Nerede: Klavye sayfası, Diğer bölümü. Anlam: oturum açılışında Num Lock davranışını seçen ayarın başlığı.
settings-input-keyboard-numlock-by-default-label = Num Lock on at startup

# Nerede: Klavye sayfası, Açılışta Num Lock açık ayarının altındaki açıklama. Anlam: açıkken oturum açılınca ne olacağını söyler.
settings-input-keyboard-numlock-by-default-description = Number keys are active when you log in

# Nerede: Klavye sayfası, Açılışta Num Lock açık ayarının arama sözcükleri. Anlam: "numlock", "sayı tuşları" terimleriyle ayar bulunur.
settings-input-keyboard-numlock-by-default-keywords = numlock, num lock, number keys, keyboard

## Fare sayfası

# Nerede: Fare ve dokunmatik yüzey sayfası, Durum bölümü (Klavye sayfasıyla ortak settings-section-status). Anlam: fare ve dokunmatik yüzey ayarlarının uygulanıp uygulanmadığını bildiren satırın başlığı.
settings-pointer-note-label = Mouse and touchpad status

# Nerede: Fare ve dokunmatik yüzey sayfası, Fare ve dokunmatik yüzey durumu satırının arama sözcükleri. Anlam: "masaüstü", "uygulanmıyor" terimleriyle satır bulunur.
settings-pointer-note-keywords = desktop, shell, not applied

# Nerede: Fare ve dokunmatik yüzey sayfası, Genel bölümü (Görünüm ve Yazı sayfalarıyla ortak settings-section-general). Anlam: işaretçi hızını belirleyen ayarın başlığı.
settings-input-mouse-sensitivity-label = Pointer speed

# Nerede: Fare ve dokunmatik yüzey sayfası, İşaretçi hızı ayarının altındaki açıklama. Anlam: ayarın hangi aygıtlar için geçerli olduğunu söyler.
settings-input-mouse-sensitivity-description = For mouse and touchpad

# Nerede: Fare ve dokunmatik yüzey sayfası, İşaretçi hızı ayarının arama sözcükleri. Anlam: "fare", "hassasiyet", "sensitivity" terimleriyle ayar bulunur.
settings-input-mouse-sensitivity-keywords = mouse, pointer, cursor, speed, sensitivity, touchpad

# Nerede: Fare ve dokunmatik yüzey sayfası, Genel bölümü. Anlam: işaretçi ivme profilini seçen ayarın başlığı.
settings-input-mouse-accel-profile-label = Acceleration

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının altındaki açıklama. Anlam: uyarlanır ivmenin ne yaptığını açıklar.
settings-input-mouse-accel-profile-description = Adaptive: faster movement moves the pointer farther

# Nerede: Fare ve dokunmatik yüzey sayfası, İvme ayarının arama sözcükleri. Anlam: "ivme", "accel", "düz" terimleriyle ayar bulunur.
settings-input-mouse-accel-profile-keywords = mouse, acceleration, accel, flat, adaptive

# Nerede: Fare ve dokunmatik yüzey sayfası, Genel bölümü. Anlam: farenin doğal kaydırma davranışını seçen ayarın başlığı.
settings-input-mouse-natural-scroll-label = Natural scrolling

# Nerede: Fare ve dokunmatik yüzey sayfası, Fare için Doğal kaydırma ayarının altındaki açıklama. Anlam: doğal kaydırmanın fare tekerleğinde ne yaptığını söyler.
settings-input-mouse-natural-scroll-description = The wheel moves content in the scroll direction

# Nerede: Fare ve dokunmatik yüzey sayfası, Fare için Doğal kaydırma ayarının arama sözcükleri. Anlam: "tekerlek", "ters kaydırma" terimleriyle ayar bulunur.
settings-input-mouse-natural-scroll-keywords = mouse, wheel, natural scroll, reverse scrolling

# Nerede: Fare ve dokunmatik yüzey sayfasındaki bölüm başlığı. Anlam: yalnızca fareye ilişkin ayarları toplayan bölüm.
settings-section-mouse = Mouse

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: dokunmatik yüzeyin doğal kaydırma davranışını seçen ayarın başlığı.
settings-input-touchpad-natural-scroll-label = Natural scrolling

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey için Doğal kaydırma ayarının altındaki açıklama. Anlam: doğal kaydırmanın dokunmatik yüzeyde ne yaptığını söyler.
settings-input-touchpad-natural-scroll-description = Content scrolls in the direction of your fingers

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey için Doğal kaydırma ayarının arama sözcükleri. Anlam: "touchpad", "ters kaydırma" terimleriyle ayar bulunur.
settings-input-touchpad-natural-scroll-keywords = touchpad, natural scroll, reverse scrolling

# Nerede: Fare ve dokunmatik yüzey sayfasındaki bölüm başlığı. Anlam: dokunmatik yüzey ayarlarını toplayan bölüm.
settings-section-touchpad = Touchpad

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: hafif dokunuşun tıklama sayılmasını seçen ayarın başlığı.
settings-input-touchpad-tap-to-click-label = Tap to click

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunarak tıkla ayarının altındaki açıklama. Anlam: özelliğin nasıl çalıştığını söyler.
settings-input-touchpad-tap-to-click-description = A light tap counts as a click

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunarak tıkla ayarının arama sözcükleri. Anlam: "tap", "tıkla" terimleriyle ayar bulunur.
settings-input-touchpad-tap-to-click-keywords = touchpad, tap, tap to click, click

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: yazarken dokunmatik yüzeyi kapatmayı seçen ayarın başlığı.
settings-input-touchpad-disable-while-typing-label = Disable while typing

# Nerede: Fare ve dokunmatik yüzey sayfası, Yazarken devre dışı ayarının altındaki açıklama. Anlam: özelliğin ne sağladığını açıklar.
settings-input-touchpad-disable-while-typing-description = Palm touches are ignored while typing

# Nerede: Fare ve dokunmatik yüzey sayfası, Yazarken devre dışı ayarının arama sözcükleri. Anlam: "yazarken", "palm", "dwt" terimleriyle ayar bulunur.
settings-input-touchpad-disable-while-typing-keywords = touchpad, typing, palm, dwt

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: kaydırma çarpanını belirleyen ayarın başlığı.
settings-input-touchpad-scroll-factor-label = Scroll speed

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma hızı ayarının arama sözcükleri. Anlam: "kaydırma", "scroll", "factor" terimleriyle ayar bulunur.
settings-input-touchpad-scroll-factor-keywords = touchpad, scroll, speed, factor

# Nerede: Fare ve dokunmatik yüzey sayfası, Dokunmatik yüzey bölümü. Anlam: kaydırma yöntemini seçen ayarın başlığı.
settings-input-touchpad-scroll-method-label = Scrolling method

# Nerede: Fare ve dokunmatik yüzey sayfası, Kaydırma yöntemi ayarının arama sözcükleri. Anlam: "iki parmak", "kenar", "scroll method" terimleriyle ayar bulunur.
settings-input-touchpad-scroll-method-keywords = touchpad, scroll, two-finger, edge, scroll method

## Kısayollar sayfası

# Nerede: Kısayollar sayfası, Genel bölümü. Anlam: Nuxut kısayollarının masaüstü kısayollarını ezmesini açıp kapatan ayarın başlığı.
settings-compositor-sync-keybinds-label = Nuxut shortcuts

# Nerede: Kısayollar sayfası, Nuxut kısayolları ayarının arama sözcükleri. Anlam: "kısayol", "eşitle" terimleriyle ayar bulunur.
settings-compositor-sync-keybinds-keywords = shortcut, key, bind, sync

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: sesi yükseltme kısayolunun adı.
settings-keybinds-volume-up-label = Raise volume

# Nerede: Kısayollar sayfası, ses açma ve kısma kısayollarının arama sözcükleri. Anlam: "volume", "xf86" terimleriyle kısayollar bulunur.
settings-keybinds-volume-keywords = shortcut, key, volume, xf86

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: sesi kısaltma kısayolunun adı.
settings-keybinds-volume-down-label = Lower volume

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: sesi açıp kapatan kısayolun adı.
settings-keybinds-volume-mute-label = Mute / unmute sound

# Nerede: Kısayollar sayfası, ses kapatma kısayolunun arama sözcükleri. Anlam: "sessiz", "mute" terimleriyle kısayol bulunur.
settings-keybinds-volume-mute-keywords = shortcut, key, silent, mute, xf86

# Nerede: Kısayollar sayfası, Ses bölümü. Anlam: mikrofonu açıp kapatan kısayolun adı.
settings-keybinds-mic-mute-label = Mute / unmute microphone

# Nerede: Kısayollar sayfası, mikrofon kısayolunun arama sözcükleri. Anlam: "mikrofon", "mute" terimleriyle kısayol bulunur.
settings-keybinds-mic-mute-keywords = shortcut, key, microphone, mute, xf86

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: parlaklık kısayollarını toplayan bölüm.
settings-section-brightness = Brightness

# Nerede: Kısayollar sayfası, Parlaklık bölümü. Anlam: parlaklığı artırma kısayolunun adı.
settings-keybinds-brightness-up-label = Increase brightness

# Nerede: Kısayollar sayfası, parlaklık artırma ve azaltma kısayollarının arama sözcükleri. Anlam: "brightness", "ekran" terimleriyle kısayollar bulunur.
settings-keybinds-brightness-keywords = shortcut, key, screen, brightness, xf86

# Nerede: Kısayollar sayfası, Parlaklık bölümü. Anlam: parlaklığı azaltma kısayolunun adı.
settings-keybinds-brightness-down-label = Decrease brightness

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: medya kısayollarını toplayan bölüm.
settings-section-media = Media

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: oynatma/duraklatma kısayolunun adı.
settings-keybinds-media-play-label = Play / pause

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: klavyedeki ayrı duraklat tuşunun kısayolunun adı (shell keybinds.media_pause); oynatmayı duraklatır ya da sürdürür.
settings-keybinds-media-pause-label = Pause key

# Nerede: Kısayollar sayfası, keybinds.media_pause satırının arama sözcükleri (virgüllü liste).
settings-keybinds-media-pause-keywords = shortcut, key, music, pause, playerctl, xf86

# Nerede: Kısayollar sayfası, medya kısayollarının arama sözcükleri. Anlam: "müzik", "playerctl" terimleriyle kısayollar bulunur.
settings-keybinds-media-keywords = shortcut, key, music, playerctl, xf86

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: sonraki parçaya geçme kısayolunun adı.
settings-keybinds-media-next-label = Next track

# Nerede: Kısayollar sayfası, Medya bölümü. Anlam: önceki parçaya geçme kısayolunun adı.
settings-keybinds-media-prev-label = Previous track

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: bar ve panel kısayollarını toplayan bölüm.
settings-section-bar-panels = Bar and panels

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: sağ paneli açıp kapatan kısayolun adı.
settings-keybinds-panel-right-label = Open / close right panel

# Nerede: Kısayollar sayfası, sağ panel kısayolunun arama sözcükleri. Anlam: "çekmece", "hızlı ayarlar" terimleriyle kısayol bulunur.
settings-keybinds-panel-right-keywords = shortcut, key, drawer, quick settings

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: sol paneli açıp kapatan kısayolun adı.
settings-keybinds-panel-left-label = Open / close left panel

# Nerede: Kısayollar sayfası, sol panel kısayolunun arama sözcükleri. Anlam: "çekmece" terimiyle kısayol bulunur.
settings-keybinds-panel-left-keywords = shortcut, key, drawer

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: takvim panelini açıp kapatan kısayolun adı.
settings-keybinds-panel-clock-label = Open / close calendar

# Nerede: Kısayollar sayfası, takvim kısayolunun arama sözcükleri. Anlam: "saat", "takvim" terimleriyle kısayol bulunur.
settings-keybinds-panel-clock-keywords = shortcut, key, clock, calendar

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü. Anlam: barı gösterip gizleyen kısayolun adı.
settings-keybinds-bar-toggle-label = Show / hide bar

# Nerede: Kısayollar sayfası, bar kısayolunun arama sözcükleri. Anlam: "bar", "gizle" terimleriyle kısayol bulunur.
settings-keybinds-bar-toggle-keywords = shortcut, key, bar, hide

# Nerede: Kısayollar sayfası, Bar ve paneller bölümü (ayrıca Erişilebilirlik). Anlam: bar'da klavyeyle (ekran okuyucuyla) gezinmeye başlatan kısayolun adı.
settings-keybinds-bar-focus-label = Move keyboard focus to the bar

# Nerede: Aynı satırın alt yazısı. Anlam: ekran okuyucu ve yalnız klavye kullananlar için; Escape gezinmeden çıkar.
settings-keybinds-bar-focus-description = For screen readers and keyboard use; Escape ends it

# Nerede: Aynı kısayolun arama sözcükleri. Anlam: odak, ekran okuyucu, Orca terimleriyle bulunur.
settings-keybinds-bar-focus-keywords = shortcut, key, bar, focus, screen reader, orca, keyboard

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekran görüntüsü kısayollarını toplayan bölüm.
settings-section-screenshot = Screenshot

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: pano (kopyalanan öğeler) kısayolunu toplayan bölüm.
settings-section-clipboard = Clipboard

# Nerede: Kısayollar sayfası, Ekran görüntüsü bölümü. Anlam: fareyle seçilen bölgenin görüntüsünü alan kısayolun adı.
settings-keybinds-shot-region-label = Capture a region

# Nerede: Kısayollar sayfası, bölge görüntüsü kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-shot-region-keywords = shortcut, key, screenshot, print, region

# Nerede: Kısayollar sayfası, Ekran görüntüsü bölümü. Anlam: imlecin bulunduğu monitörün tamamının görüntüsünü alan kısayolun adı.
settings-keybinds-shot-output-label = Capture the whole screen

# Nerede: Kısayollar sayfası, tüm ekran görüntüsü kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-shot-output-keywords = shortcut, key, screenshot, print, monitor

# Nerede: Kısayollar sayfası, Ekran görüntüsü bölümü. Anlam: etkin pencerenin görüntüsünü alan kısayolun adı.
settings-keybinds-shot-window-label = Capture a window

# Nerede: Kısayollar sayfası, pencere görüntüsü kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-shot-window-keywords = shortcut, key, screenshot, print, window

# Nerede: Kısayollar sayfası, Pano bölümü. Anlam: kopyalanan öğelerin geçmişini (başlatıcının pano kipi) açan kısayolun adı.
settings-keybinds-clipboard-label = Open clipboard history

# Nerede: Kısayollar sayfası, pano kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-clipboard-keywords = shortcut, key, clipboard, copy, paste, history

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekranı imlecin çevresinde büyüten büyüteç kısayollarını toplayan bölüm.
settings-section-magnifier = Magnifier

# Nerede: Kısayollar sayfası, Büyüteç bölümü. Anlam: büyütmeyi bir adım artıran kısayolun adı (basılı tutunca tekrarlar).
settings-keybinds-magnifier-in-label = Zoom in

# Nerede: Kısayollar sayfası, yakınlaştırma kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-magnifier-in-keywords = shortcut, key, magnifier, zoom in, zoom, accessibility

# Nerede: Kısayollar sayfası, Büyüteç bölümü. Anlam: büyütmeyi bir adım azaltan kısayolun adı (basılı tutunca tekrarlar).
settings-keybinds-magnifier-out-label = Zoom out

# Nerede: Kısayollar sayfası, uzaklaştırma kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-magnifier-out-keywords = shortcut, key, magnifier, zoom out, zoom, accessibility

# Nerede: Kısayollar sayfası, Büyüteç bölümü. Anlam: büyütmeyi tamamen kapatıp ekranı normal boyuta döndüren kısayolun adı.
settings-keybinds-magnifier-reset-label = Turn off magnification

# Nerede: Kısayollar sayfası, büyütmeyi kapatma kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-magnifier-reset-keywords = shortcut, key, magnifier, reset, turn off, zoom, accessibility

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: başlatıcının emoji seçici kipini açan kısayolu toplayan bölüm.
settings-section-emoji = Emoji

# Nerede: Kısayollar sayfası, Emoji bölümü. Anlam: başlatıcıyı emoji seçici kipinde açan kısayolun adı.
settings-keybinds-emoji-label = Emoji picker

# Nerede: Kısayollar sayfası, emoji seçici kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-emoji-keywords = shortcut, key, emoji, emoticon, symbol, launcher

## Kısayollar: temel katman (shell ADR-0006)

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Uygulama başlatıcısı" kısayolunun adı (shell keybinds.launcher).
settings-keybinds-launcher-label = App launcher

# Nerede: Kısayollar sayfası, keybinds.launcher satırının arama sözcükleri (virgüllü liste).
settings-keybinds-launcher-keywords = shortcut, key, launcher, super, app, search

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Ayarlar" kısayolunun adı (shell keybinds.settings).
settings-keybinds-settings-label = Settings

# Nerede: Kısayollar sayfası, keybinds.settings satırının arama sözcükleri (virgüllü liste).
settings-keybinds-settings-keywords = shortcut, key, settings, preferences

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Terminal" kısayolunun adı (shell keybinds.terminal).
settings-keybinds-terminal-label = Terminal

# Nerede: Kısayollar sayfası, keybinds.terminal satırının arama sözcükleri (virgüllü liste).
settings-keybinds-terminal-keywords = shortcut, key, terminal, console, command line

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Dosya yöneticisi" kısayolunun adı (shell keybinds.file_manager).
settings-keybinds-file-manager-label = File manager

# Nerede: Kısayollar sayfası, keybinds.file_manager satırının arama sözcükleri (virgüllü liste).
settings-keybinds-file-manager-keywords = shortcut, key, file, folder, file manager, files

# Nerede: Kısayollar sayfası, Uygulamalar bölümü. Anlam: "Web tarayıcısı" kısayolunun adı (shell keybinds.browser).
settings-keybinds-browser-label = Web browser

# Nerede: Kısayollar sayfası, keybinds.browser satırının arama sözcükleri (virgüllü liste).
settings-keybinds-browser-keywords = shortcut, key, browser, internet, web

# Nerede: Kısayollar sayfası, Oturum bölümü. Anlam: "Ekranı kilitle" kısayolunun adı (shell keybinds.lock).
settings-keybinds-lock-label = Lock screen

# Nerede: Kısayollar sayfası, keybinds.lock satırının arama sözcükleri (virgüllü liste).
settings-keybinds-lock-keywords = shortcut, key, lock, screen

# Nerede: Kısayollar sayfası, Oturum bölümü. Anlam: "Oturum menüsü" kısayolunun adı (shell keybinds.session_menu).
settings-keybinds-session-menu-label = Session menu

# Nerede: Kısayollar sayfası, keybinds.session_menu satırının arama sözcükleri (virgüllü liste).
settings-keybinds-session-menu-keywords = shortcut, key, session, shut down, restart, log out, ctrl alt delete

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereyi kapat" kısayolunun adı (shell keybinds.close_window).
settings-keybinds-close-window-label = Close window

# Nerede: Kısayollar sayfası, keybinds.close_window satırının arama sözcükleri (virgüllü liste).
settings-keybinds-close-window-keywords = shortcut, key, window, close

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereyi kapat (ikinci kısayol)" kısayolunun adı (shell keybinds.close_window_alt).
settings-keybinds-close-window-alt-label = Close window (secondary shortcut)

# Nerede: Kısayollar sayfası, keybinds.close_window_alt satırının arama sözcükleri (virgüllü liste).
settings-keybinds-close-window-alt-keywords = shortcut, key, window, close, alt f4

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: yanıt vermeyen bir pencereyi zorla kapatma kısayolunun adı (shell keybinds.force_kill); kısayoldan sonra kapatılacak pencereye tıklanır.
settings-keybinds-force-kill-label = Force close an unresponsive window

# Nerede: Kısayollar sayfası, keybinds.force_kill satırının arama sözcükleri (virgüllü liste).
settings-keybinds-force-kill-keywords = shortcut, key, window, force, close, kill

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Tam ekran" kısayolunun adı (shell keybinds.fullscreen).
settings-keybinds-fullscreen-label = Fullscreen

# Nerede: Kısayollar sayfası, keybinds.fullscreen satırının arama sözcükleri (virgüllü liste).
settings-keybinds-fullscreen-keywords = shortcut, key, window, fullscreen

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Ekranı kapla" kısayolunun adı (shell keybinds.maximize).
settings-keybinds-maximize-label = Maximize

# Nerede: Kısayollar sayfası, keybinds.maximize satırının arama sözcükleri (virgüllü liste).
settings-keybinds-maximize-keywords = shortcut, key, window, maximize, fill screen

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Yüzen pencere" kısayolunun adı (shell keybinds.toggle_floating).
settings-keybinds-toggle-floating-label = Floating window

# Nerede: Kısayollar sayfası, keybinds.toggle_floating satırının arama sözcükleri (virgüllü liste).
settings-keybinds-toggle-floating-keywords = shortcut, key, window, floating, tiling

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereyi sabitle" kısayolunun adı (shell keybinds.pin).
settings-keybinds-pin-label = Pin window

# Nerede: Kısayollar sayfası, keybinds.pin satırının arama sözcükleri (virgüllü liste).
settings-keybinds-pin-keywords = shortcut, key, window, pin, always on top

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereler arasında geçiş" kısayolunun adı (shell keybinds.cycle_windows).
settings-keybinds-cycle-windows-label = Cycle windows

# Nerede: Kısayollar sayfası, keybinds.cycle_windows satırının arama sözcükleri (virgüllü liste).
settings-keybinds-cycle-windows-keywords = shortcut, key, window, switch, alt tab

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Pencereler arasında geri geçiş" kısayolunun adı (shell keybinds.cycle_windows_back).
settings-keybinds-cycle-windows-back-label = Cycle windows backwards

# Nerede: Kısayollar sayfası, keybinds.cycle_windows_back satırının arama sözcükleri (virgüllü liste).
settings-keybinds-cycle-windows-back-keywords = shortcut, key, window, switch, alt shift tab

# Nerede: Kısayollar sayfası, Pencereler bölümü. Anlam: "Bölme yönünü çevir" kısayolunun adı (shell keybinds.toggle_split).
settings-keybinds-toggle-split-label = Toggle split direction

# Nerede: Kısayollar sayfası, keybinds.toggle_split satırının arama sözcükleri (virgüllü liste).
settings-keybinds-toggle-split-keywords = shortcut, key, window, split, side by side, stacked

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: sol" kısayolunun adı (shell keybinds.focus_left).
settings-keybinds-focus-left-label = Focus: left

# Nerede: Kısayollar sayfası, keybinds.focus_left satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-left-keywords = shortcut, key, focus, left, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: sağ" kısayolunun adı (shell keybinds.focus_right).
settings-keybinds-focus-right-label = Focus: right

# Nerede: Kısayollar sayfası, keybinds.focus_right satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-right-keywords = shortcut, key, focus, right, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: yukarı" kısayolunun adı (shell keybinds.focus_up).
settings-keybinds-focus-up-label = Focus: up

# Nerede: Kısayollar sayfası, keybinds.focus_up satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-up-keywords = shortcut, key, focus, up, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Odak: aşağı" kısayolunun adı (shell keybinds.focus_down).
settings-keybinds-focus-down-label = Focus: down

# Nerede: Kısayollar sayfası, keybinds.focus_down satırının arama sözcükleri (virgüllü liste).
settings-keybinds-focus-down-keywords = shortcut, key, focus, down, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi sola taşı" kısayolunun adı (shell keybinds.move_left).
settings-keybinds-move-left-label = Move window left

# Nerede: Kısayollar sayfası, keybinds.move_left satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-left-keywords = shortcut, key, move, left, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi sağa taşı" kısayolunun adı (shell keybinds.move_right).
settings-keybinds-move-right-label = Move window right

# Nerede: Kısayollar sayfası, keybinds.move_right satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-right-keywords = shortcut, key, move, right, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi yukarı taşı" kısayolunun adı (shell keybinds.move_up).
settings-keybinds-move-up-label = Move window up

# Nerede: Kısayollar sayfası, keybinds.move_up satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-up-keywords = shortcut, key, move, up, window

# Nerede: Kısayollar sayfası, Odak ve taşıma bölümü. Anlam: "Pencereyi aşağı taşı" kısayolunun adı (shell keybinds.move_down).
settings-keybinds-move-down-label = Move window down

# Nerede: Kısayollar sayfası, keybinds.move_down satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-down-keywords = shortcut, key, move, down, window

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi daralt" kısayolunun adı (shell keybinds.shrink_width).
settings-keybinds-shrink-width-label = Make window narrower

# Nerede: Kısayollar sayfası, keybinds.shrink_width satırının arama sözcükleri (virgüllü liste).
settings-keybinds-shrink-width-keywords = shortcut, key, size, shrink, width, resize

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi genişlet" kısayolunun adı (shell keybinds.grow_width).
settings-keybinds-grow-width-label = Make window wider

# Nerede: Kısayollar sayfası, keybinds.grow_width satırının arama sözcükleri (virgüllü liste).
settings-keybinds-grow-width-keywords = shortcut, key, size, grow, width, resize

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi kısalt" kısayolunun adı (shell keybinds.shrink_height).
settings-keybinds-shrink-height-label = Make window shorter

# Nerede: Kısayollar sayfası, keybinds.shrink_height satırının arama sözcükleri (virgüllü liste).
settings-keybinds-shrink-height-keywords = shortcut, key, size, shrink, height, resize

# Nerede: Kısayollar sayfası, Boyutlandırma bölümü. Anlam: "Pencereyi uzat" kısayolunun adı (shell keybinds.grow_height).
settings-keybinds-grow-height-label = Make window taller

# Nerede: Kısayollar sayfası, keybinds.grow_height satırının arama sözcükleri (virgüllü liste).
settings-keybinds-grow-height-keywords = shortcut, key, size, grow, height, resize

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Çalışma alanına geç (1…0)" kısayolunun adı (shell keybinds.workspace).
settings-keybinds-workspace-label = Switch to workspace (1…0)

# Nerede: Kısayollar sayfası, keybinds.workspace satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-keywords = shortcut, key, workspace, desktop, number

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi çalışma alanına taşı (1…0)" kısayolunun adı (shell keybinds.move_to_workspace).
settings-keybinds-move-to-workspace-label = Move window to workspace (1…0)

# Nerede: Kısayollar sayfası, keybinds.move_to_workspace satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-workspace-keywords = shortcut, key, workspace, move, number

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Sonraki çalışma alanı" kısayolunun adı (shell keybinds.workspace_next).
settings-keybinds-workspace-next-label = Next workspace

# Nerede: Kısayollar sayfası, keybinds.workspace_next satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-next-keywords = shortcut, key, workspace, next, page down

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Önceki çalışma alanı" kısayolunun adı (shell keybinds.workspace_prev).
settings-keybinds-workspace-prev-label = Previous workspace

# Nerede: Kısayollar sayfası, keybinds.workspace_prev satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-prev-keywords = shortcut, key, workspace, previous, page up

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi sonraki alana taşı" kısayolunun adı (shell keybinds.move_to_workspace_next).
settings-keybinds-move-to-workspace-next-label = Move window to next workspace

# Nerede: Kısayollar sayfası, keybinds.move_to_workspace_next satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-workspace-next-keywords = shortcut, key, workspace, move, next

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi önceki alana taşı" kısayolunun adı (shell keybinds.move_to_workspace_prev).
settings-keybinds-move-to-workspace-prev-label = Move window to previous workspace

# Nerede: Kısayollar sayfası, keybinds.move_to_workspace_prev satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-workspace-prev-keywords = shortcut, key, workspace, move, previous

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Son çalışma alanına dön" kısayolunun adı (shell keybinds.workspace_last).
settings-keybinds-workspace-last-label = Return to last workspace

# Nerede: Kısayollar sayfası, keybinds.workspace_last satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-last-keywords = shortcut, key, workspace, last, previous, super tab

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Karalama alanını göster/gizle" kısayolunun adı (shell keybinds.scratchpad).
settings-keybinds-scratchpad-label = Show or hide scratchpad

# Nerede: Kısayollar sayfası, keybinds.scratchpad satırının arama sözcükleri (virgüllü liste).
settings-keybinds-scratchpad-keywords = shortcut, key, scratchpad, hidden, special area

# Nerede: Kısayollar sayfası, Çalışma alanları bölümü. Anlam: "Pencereyi karalama alanına gönder" kısayolunun adı (shell keybinds.move_to_scratchpad).
settings-keybinds-move-to-scratchpad-label = Send window to scratchpad

# Nerede: Kısayollar sayfası, keybinds.move_to_scratchpad satırının arama sözcükleri (virgüllü liste).
settings-keybinds-move-to-scratchpad-keywords = shortcut, key, scratchpad, hidden, move

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Fareyle pencere taşı" kısayolunun adı (shell keybinds.window_drag).
settings-keybinds-window-drag-label = Move window with mouse

# Nerede: Kısayollar sayfası, keybinds.window_drag satırının arama sözcükleri (virgüllü liste).
settings-keybinds-window-drag-keywords = shortcut, key, mouse, drag, move, window

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Fareyle pencere boyutlandır" kısayolunun adı (shell keybinds.window_resize_drag).
settings-keybinds-window-resize-drag-label = Resize window with mouse

# Nerede: Kısayollar sayfası, keybinds.window_resize_drag satırının arama sözcükleri (virgüllü liste).
settings-keybinds-window-resize-drag-keywords = shortcut, key, mouse, drag, resize, window

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Tekerlekle sonraki çalışma alanı" kısayolunun adı (shell keybinds.workspace_scroll_next).
settings-keybinds-workspace-scroll-next-label = Next workspace (mouse wheel)

# Nerede: Kısayollar sayfası, keybinds.workspace_scroll_next satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-scroll-next-keywords = shortcut, key, mouse, wheel, workspace, scroll

# Nerede: Kısayollar sayfası, Fare bölümü. Anlam: "Tekerlekle önceki çalışma alanı" kısayolunun adı (shell keybinds.workspace_scroll_prev).
settings-keybinds-workspace-scroll-prev-label = Previous workspace (mouse wheel)

# Nerede: Kısayollar sayfası, keybinds.workspace_scroll_prev satırının arama sözcükleri (virgüllü liste).
settings-keybinds-workspace-scroll-prev-keywords = shortcut, key, mouse, wheel, workspace, scroll

# Nerede: Kısayollar sayfası, Genel bölümü. Anlam: Nuxut'un hazır pencere yönetimi kısayollarını, pencere kurallarını ve davranış varsayılanlarını (temel katman) açıp kapatan ayarın başlığı.
settings-compositor-base-layer-label = Built-in window management

# Nerede: Kısayollar sayfası, compositor.base_layer ayarının alt yazısı. Anlam: temel katmanın neleri içerdiği ve kullanıcının kendi Hyprland ayarındaki satırların bunları ezdiği.
settings-compositor-base-layer-description = Window shortcuts and rules; your Hyprland config overrides them

# Nerede: Kısayollar sayfası, compositor.base_layer ayarının arama sözcükleri (virgüllü liste).
settings-compositor-base-layer-keywords = shortcut, window, workspace, base layer, hyprland, rule

# Nerede: Kısayollar sayfası, Genel bölümü. Anlam: kullanıcının kendi Hyprland ayarında ses, parlaklık, medya, ekran görüntüsü tuşları başka bir şeye bağlıysa Nuxut'un bu tuşları kendi işlevleri için devralmasını açıp kapatan ayarın başlığı.
settings-compositor-take-over-keys-label = Use Nuxut for media and volume keys

# Nerede: Kısayollar sayfası, compositor.take_over_keys ayarının alt yazısı. Anlam: tuşların Nuxut'a geçtiği, kullanıcının Hyprland dosyasının değiştirilmediği.
settings-compositor-take-over-keys-description = Matching Hyprland keys go to Nuxut; the file is unchanged

# Nerede: Kısayollar sayfası, compositor.take_over_keys ayarının arama sözcükleri (virgüllü liste).
settings-compositor-take-over-keys-keywords = shortcut, key, volume, media, brightness, screenshot, take over

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: uygulama başlatıcısı dışında oturumu kilitleme ve oturum menüsü kısayollarını toplayan bölüm.
settings-section-session = Session

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: pencere kapatma, tam ekran, yüzen pencere gibi pencere kısayollarını toplayan bölüm.
settings-section-windows = Windows

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: odağı ve pencereyi ok tuşlarıyla yönlere taşıyan kısayolları toplayan bölüm.
settings-section-focus-move = Focus and movement

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: pencereyi klavyeyle büyütüp küçülten kısayolları toplayan bölüm.
settings-section-resize = Resize

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: çalışma alanları (sanal masaüstleri) arasında geçiş ve pencere taşıma kısayollarını toplayan bölüm.
settings-section-workspaces = Workspaces

# Nerede: Kısayollar sayfası, yakalama modunda başlatıcı satırının altındaki ipucu. Anlam: tek bir değiştirici tuşa (ör. Super) basıp bırakılması gerektiği; Esc iptal eder.
settings-keybinds-hint-tap = Press and release a single modifier (e.g. Super) · Esc to cancel

# Nerede: Kısayollar sayfası, yakalama modunda çalışma alanı satırlarının altındaki ipucu. Anlam: 1…0 tuşlarıyla birlikte kullanılacak değiştiricilere basılıp bırakılması gerektiği.
settings-keybinds-hint-digits = Press and release modifiers to use with 1…0 · Esc to cancel

# Nerede: Kısayollar sayfası, başlatıcı satırında yakalama sırasında uyarı. Anlam: bu kısayolun yalnızca tek bir değiştirici tuşa dokunup bırakarak atanabildiği.
settings-keybinds-tap-single = Press and release only a single modifier

# Nerede: Kısayollar sayfası, fare kısayolu satırına tıklanınca altındaki bilgi. Anlam: fare kısayollarının klavyeden kaydedilemediği; yalnızca kaldırılabildiği ya da varsayılana döndürülebildiği.
settings-keybinds-pointer-hint = Mouse shortcut: can only be removed or reset to default

# Nerede: Kısayollar sayfası, fare tekerleği aşağı yönüne atanmış kısayolun tuş başlığı. Anlam: fare tekerleğini aşağı çevirme.
settings-keybind-cap-wheel-down = Wheel down

# Nerede: Kısayollar sayfası, fare tekerleği yukarı yönüne atanmış kısayolun tuş başlığı. Anlam: fare tekerleğini yukarı çevirme.
settings-keybind-cap-wheel-up = Wheel up

# Nerede: Kısayollar sayfası, farenin sol düğmesine (sürükleyerek) atanmış kısayolun tuş başlığı. Anlam: farenin sol düğmesi.
settings-keybind-cap-mouse-left = Left click

# Nerede: Kısayollar sayfası, farenin sağ düğmesine (sürükleyerek) atanmış kısayolun tuş başlığı. Anlam: farenin sağ düğmesi.
settings-keybind-cap-mouse-right = Right click

# Nerede: Kısayollar sayfasındaki bölüm başlığı. Anlam: ekrana dokunarak yazılan sanal klavyenin kısayolunu toplayan bölüm.
settings-section-osk = On-screen keyboard

# Nerede: Kısayollar sayfası, Ekran klavyesi bölümü. Anlam: ekran klavyesini gösteren ya da gizleyen kısayolun adı.
settings-keybinds-osk-toggle-label = Show or hide the on-screen keyboard

# Nerede: Kısayollar sayfası, ekran klavyesi kısayolunun arama sözcükleri (virgüllü liste).
settings-keybinds-osk-toggle-keywords = shortcut, key, on-screen keyboard, virtual keyboard, touch, accessibility

## Hakkında sayfası

# Nerede: Hakkında sayfasındaki ayar satırının başlığı; satır bilgi penceresini açar. Anlam: sürüm ve yapılandırma bilgisi.
settings-about-label = About

# Nerede: Hakkında sayfası, Hakkında satırının altındaki açıklama. Anlam: pencerede gösterilen bilgileri özetler.
settings-about-description = Version and config files

# Nerede: Hakkında sayfası, Hakkında satırının arama sözcükleri. Anlam: "sürüm", "lisans" terimleriyle satır bulunur.
settings-about-keywords = version, license
## Uygulama penceresi ve arama

# Nerede: Ana pencere, sol üstteki uygulama başlığı (src/app.rs). Anlam: Uygulamanın adı.
settings-app-title = Settings

# Nerede: Kenar çubuğundaki arama alanının yer tutucusu (src/app.rs). Anlam: Aramanın ayarlar içinde yapıldığı.
settings-app-search-placeholder = Search settings

# Nerede: Arama sonuç listesi boşken listenin ilk (etkisiz) satırı (src/app.rs). Anlam: Eşleşme bulunamadığı.
settings-app-search-empty = No results

# Nerede: "Sonuç yok" satırının alt yazısı (src/app.rs). Anlam: Başka bir kelime denenmesi önerisi.
settings-app-search-empty-hint = Try another word

# Nerede: Arama sonuçlarında sayfa eşleşmelerinin alt yazısı (src/app.rs). Anlam: Sonucun bir ayar değil, bir sayfa olduğu.
settings-app-result-page = Page

# Nerede: Ayar satırı başlığının yanındaki küçük hap, widget ekleme menüsündeki düğmenin köşesi ve arama sonucunun sağı (src/new_badge.rs, src/app.rs). Anlam: Ayarın/widget'ın son güncellemeyle geldiği ve kullanıcının henüz görmediği. Çok kısa olmalı (tek sözcük).
settings-badge-new = New

# Nerede: Başlık çubuğundaki "Son değişiklikler" menüsü boşken görünen (etkisiz) öğe (src/app.rs). Anlam: Henüz değişiklik yapılmadığı.
settings-app-no-changes = No changes yet

# Nerede: Portal duvar kağıdı seçicinin pencere başlığı (src/services/mod.rs). Anlam: Dosya seçicide duvar kağıdı seçildiği.
settings-services-wallpaper-dialog-title = Choose a wallpaper

# Nerede: Portal dosya seçicideki tür süzgecinin adı (src/services/portal.rs). Anlam: Seçicide gösterilen dosya türleri.
settings-portal-filter-images = Images and GIFs

## Bildirim şeridi ve geçmiş

# Nerede: Panoya kopyalama başarılı olunca çıkan bildirim şeridi (src/app.rs). Anlam: Kopyalamanın yapıldığı.
settings-app-copied = Copied to clipboard

# Nerede: Panoya kopyalama başarısız olunca çıkan bildirim şeridi (src/app.rs). Anlam: Kopyalamanın neden başarısız olduğu.
# Değişkenler: $error (hata ayrıntısı, ör. "izin verilmedi").
settings-app-error-copy = Could not copy: { $error }

# Nerede: Bir ayar değişince çıkan bildirim şeridi ("Geri al" düğmeli) (src/app.rs). Anlam: Hangi ayarın değiştiği.
# Değişkenler: $label (değişen ayarın başlığı, ör. "Panel düzeni").
settings-app-changed = { $label } changed

# Nerede: Geri yükleme başlatılınca çıkan bildirim şeridi (src/app.rs). Anlam: Ayarın önceki hâline döndürülmekte olduğu.
# Değişkenler: $label (geri yüklenen ayarın başlığı, ör. "Panel düzeni").
settings-app-restoring = Restoring { $label }…

# Nerede: "Geri al" uygulandıktan sonraki bildirim şeridi (src/app.rs). Anlam: Değişikliğin geri alındığı.
# Değişkenler: $label (değişikliğin etiketi, ör. "Panel düzeni değişti").
settings-app-history-undone = { $label } undone

# Nerede: "Yinele" uygulandıktan sonraki bildirim şeridi (src/app.rs). Anlam: Değişikliğin yeniden uygulandığı.
# Değişkenler: $label (değişikliğin etiketi, ör. "Panel düzeni değişti").
settings-app-history-redone = { $label } redone

# Nerede: "Son değişiklikler" menüsünden tek kayıt geri alındığında çıkan bildirim şeridi (src/history.rs). Anlam: Kaydın geri alındığı; kaydın etiketine eklenir.
# Değişkenler: $label (geri alınan kaydın etiketi, ör. "Panel düzeni değişti").
settings-history-reverted = { $label } (undone)

# Nerede: Bir ayar dosyası okunamadığında pencerenin üstündeki uyarı bandı (src/app.rs). Anlam: Dosya düzeltilene kadar değişikliklerin kaydedilmediği.
# Değişkenler: $file (dosyanın adı, uzantısız, ör. "look"), $error (hata ayrıntısı, ör. "geçersiz TOML").
settings-app-error-broken-file = { $file }.toml could not be read ({ $error }) — changes are not saved until it is fixed.

# Nerede: Ayar dosyası kaydedilemediğinde çıkan bildirim şeridi (src/app.rs). Anlam: Kaydetmenin neden başarısız olduğu.
# Değişkenler: $file (uzantısız dosya adı, ör. "shell"), $error (hata ayrıntısı, ör. "disk dolu").
settings-app-error-save = Could not save { $file }.toml: { $error }

# Nerede: Ayar araması sonuçlarındaki satırın alt yazısı (src/app.rs). Anlam: Sonucun hangi sayfanın hangi bölümünde olduğu.
# Değişkenler: $page (sayfa başlığı, ör. "Görünüm ve hareket"), $section (bölüm adı, ör. "Panel düzeni").
settings-app-result-subtitle = { $page } › { $section }

# Nerede: Pencere başlığı (src/app.rs). Anlam: Pencerede açık olan ayar sayfası.
# Değişkenler: $page (sayfa başlığı, ör. "Görünüm ve hareket").
settings-app-window-title = Settings — { $page }

# Nerede: Bildirim şeridindeki geri alma düğmesi (src/app.rs). Anlam: Düğmenin görevi: son değişikliği geri almak.
settings-app-snackbar-undo = Undo

## Yönlendirme satırı

# Nerede: Ayar başka bir sayfadayken görünen yönlendirme satırının sağındaki metin düğme (src/pages/link.rs). Anlam: Düğme, hedef ayarı açar.
settings-link-configure = Configure

## Dil ve biçim

# Nerede: Dil ve Biçim sayfasında Biçimler satırının alt yazısı (src/pages/locale.rs). Anlam: Seçilen biçimin tanınmadığı, sistem biçiminin kullanıldığı.
# Değişkenler: $sample (biçim önizlemesi, ör. "11.10.2026 · 1.234,56").
settings-locale-subtitle-unrecognized = Unrecognized, using system formats: { $sample }

# Nerede: Dil ve Biçimler açılır menülerinin ilk seçeneği (src/pages/locale.rs). Anlam: Sistem ayarının izlendiği.
settings-locale-auto = Automatic

# Nerede: Dil satırının alt yazısı (src/pages/locale.rs). Anlam: Otomatik seçimde kullanılan sistem dili.
# Değişkenler: $language (sistem dilinin adı, ör. "Türkçe (Türkiye)").
settings-locale-subtitle-system-language = System language: { $language }

# Nerede: Biçimler satırının alt yazısı (src/pages/locale.rs). Anlam: Otomatik seçimde kullanılan sistem biçimi.
# Değişkenler: $formats (biçim yerelinin adı, ör. "Türkçe (Türkiye)").
settings-locale-subtitle-system-formats = System formats: { $formats }

# Nerede: Biçim önizleme kartının başlığı (src/pages/locale.rs). Anlam: Biçim açıkça seçili değilken sistem biçiminin izlendiği.
# Değişkenler: $formats (biçim yerelinin adı, ör. "Türkçe (Türkiye)").
settings-locale-preview-caption-system = { $formats } · system

# Nerede: Biçim önizleme kartındaki kısa tarih örneğinin etiketi (src/pages/locale.rs). Anlam: Örneğin kısa tarih olduğu.
settings-locale-preview-short-date = Short date

# Nerede: Biçim önizleme kartındaki sayı örneğinin etiketi (src/pages/locale.rs). Anlam: Örneğin sayı biçimi olduğu.
settings-locale-preview-number = Number

# Nerede: Biçim önizleme kartındaki yüzde örneğinin etiketi (src/pages/locale.rs). Anlam: Örneğin yüzde biçimi olduğu.
settings-locale-preview-percent = Percent

## Görünüm sayfası

# Nerede: Görünüm sayfasındaki Renk kipi seçiminin ilk seçeneği; seçilince anahtar dosyadan silinir (src/pages/look.rs). Anlam: Sistem takibi (varsayılan).
settings-look-mode-auto = Auto

# Nerede: Görünüm sayfası, Renk kipi seçiminin ikinci seçeneği (src/pages/look.rs). Anlam: Koyu temanın açıkça seçilmesi.
settings-look-mode-dark = Dark

# Nerede: Görünüm sayfası, Renk kipi seçiminin üçüncü seçeneği (src/pages/look.rs). Anlam: Açık temanın açıkça seçilmesi.
settings-look-mode-light = Light

# Nerede: Görünüm sayfasında ikon ve imleç teması seçiminin not satırı (src/pages/look.rs). Anlam: Kurulu temaların taranmakta olduğu.
settings-look-themes-loading = Looking for installed themes…

# Nerede: Görünüm sayfasında kurulu tema hiç bulunamayınca not satırı (src/pages/look.rs). Anlam: Önerilen tema kurulunca temaların burada görüneceği.
settings-look-themes-empty = No installed themes found; the recommended theme will appear here once it is installed.

# Nerede: Görünüm sayfasındaki durum kartında, kurulum komutunun yanındaki metin düğme (src/pages/look.rs). Anlam: Komutu panoya kopyalar.
settings-look-copy-command = Copy command

# Nerede: Görünüm sayfası, birleşik görünüm kapalıyken anahtarın hemen altındaki açıklama satırı (src/pages/look.rs). Anlam: Kapalıyken uygulamaların kendi temalarını kullanmaya devam edeceği.
settings-look-unified-off-note = When off, apps use their own themes.

# Nerede: Görünüm sayfası, aynı açıklama satırının sağındaki tonlu düğme (src/pages/look.rs). Anlam: Birleşik görünümü açar.
settings-look-unified-turn-on = Turn on unified look

# Nerede: Görünüm sayfasındaki tema kartı ızgarasının ilk kartı; shell'den öneri okunamadığında kartın adı ve önerilen kurulu kartın alt yazısı (src/pages/theme_grid.rs). Anlam: Kartın önerilen temayı temsil ettiği.
settings-theme-grid-recommended = Recommended

# Nerede: Görünüm sayfasında, seçili önerilen aile kartının alt yazısı; etkin varyant önerinin kendisiyken (src/pages/theme_grid.rs). Anlam: Önerilen varyantın seçili olduğu.
# Değişkenler: $name (varyantın seçicideki kısa adı, ör. "White").
settings-theme-grid-caption-recommended-variant = Recommended · { $name }

# Nerede: Görünüm sayfasında, seçili önerilen aile kartının alt yazısı; etkin varyant öneri değilken (src/pages/theme_grid.rs). Anlam: Ailenin kaç kurulu varyantı olduğu.
# Değişkenler: $count (ailenin kurulu varyant sayısı, ör. 3).
settings-theme-grid-caption-recommended-variants =
    Recommended · { $count ->
        [one] { $count } variant
       *[other] { $count } variants
    }

# Nerede: Görünüm sayfasında, seçili aile kartının alt yazısı; kart önerilen değilken (src/pages/theme_grid.rs). Anlam: Ailenin kaç kurulu varyantı olduğu.
# Değişkenler: $count (ailenin kurulu varyant sayısı, ör. 3).
settings-theme-grid-caption-variants =
    { $count ->
        [one] { $count } variant
       *[other] { $count } variants
    }

# Nerede: Görünüm sayfasında, önerilen kurulu kartın alt yazısı; önerinin adı okunamadığında (src/pages/theme_grid.rs). Anlam: Hangi temanın seçileceğine kabuğun karar vereceği.
settings-theme-grid-caption-shell-picks = Shell picks

# Nerede: Görünüm sayfasında, önerilen kartın alt yazısı; önerilen aile kurulu değilken (src/pages/theme_grid.rs). Anlam: Önerilen temanın kurulu olmadığı.
settings-theme-grid-caption-recommended-missing = Recommended · not installed

# Nerede: Aynı alt yazı; önerilen ailenin paketi AUR'dayken (src/pages/theme_grid.rs). Anlam: Önerilen temanın kurulu olmadığı ve paketinin AUR'da olduğu.
settings-theme-grid-caption-recommended-missing-aur = Recommended · not installed · AUR

# Nerede: Görünüm sayfasında, kurulu olmayan aile kartının alt yazısı; paket AUR'dayken (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı ve paketinin AUR'da olduğu.
settings-theme-grid-caption-missing-aur = Not installed · AUR

# Nerede: Görünüm sayfasında, kurulu olmayan aile kartının alt yazısı (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı.
settings-theme-grid-caption-missing = Not installed

# Nerede: Görünüm sayfasında, ızgaranın altındaki varyant seçicinin etiketi (src/pages/theme_grid.rs). Anlam: Seçicinin kartın ailesinin varyantlarını seçtiği.
# Değişkenler: $name (tema ailesinin adı, ör. "Bibata Modern").
settings-theme-grid-variant-picker-label = { $name } variants

# Nerede: Görünüm sayfasında, kurulum komutunun yanındaki metin düğme (src/pages/theme_grid.rs). Anlam: Komutu panoya kopyalar.
settings-theme-grid-button-copy-command = Copy install command

# Nerede: Görünüm sayfasında, varyant adlarının ortak kısmı atılınca ortak adı kalmayan varyantın seçicideki etiketi (src/pages/theme_grid.rs). Anlam: Ailenin temel (adsız) varyantı.
settings-theme-grid-variant-base = Standard

# Nerede: Görünüm sayfasında ızgaranın altındaki not; kurulu olmayan aile kartı gösterilirken ve kurulum komutu biliniyorken (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı ve terminalden kurulabileceği.
# Değişkenler: $name (tema ailesinin adı, ör. "Flat-Remix"), $command (AUR kurulum komutu, ör. "paru -S flat-remix-gtk-theme").
settings-theme-grid-footer-missing-command = { $name } is not installed. To install, run in a terminal: { $command }

# Nerede: Aynı not; kurulum komutu bilinmiyorken (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı.
# Değişkenler: $name (tema ailesinin adı, ör. "Flat-Remix").
settings-theme-grid-footer-missing = { $name } is not installed.

# Nerede: Görünüm sayfasında ızgaranın altındaki not; seçili tema kurulu değilken ve kurulum komutu biliniyorken (src/pages/theme_grid.rs). Anlam: Tema kurulana dek yedek temanın kullanılacağı ve terminalden kurulabileceği.
# Değişkenler: $name (temanın adı, ör. "Flat-Remix"), $command (AUR kurulum komutu, ör. "paru -S flat-remix-gtk-theme").
settings-theme-grid-footer-fallback-command = { $name } is not installed; a fallback theme is used until it is installed. To install, run in a terminal: { $command }

# Nerede: Görünüm sayfasında ızgaranın altındaki not; kurulu olmayan aile kartı gösterilirken ve paket resmî depodayken; yanında "Kur" düğmesi (src/pages/theme_grid.rs). Anlam: Ailenin kurulu olmadığı ve düğmeyle, yönetici onayıyla kurulabileceği.
# Değişkenler: $name (tema ailesinin adı, ör. "Papirus").
settings-theme-grid-footer-missing-install = { $name } isn't installed. Install sets it up with administrator approval.

# Nerede: Görünüm sayfasında ızgaranın altındaki not; seçili tema kurulu değilken ve paket resmî depodayken; yanında "Kur" düğmesi (src/pages/theme_grid.rs). Anlam: Tema kurulana dek yedek temanın kullanılacağı ve düğmeyle kurulabileceği.
# Değişkenler: $name (temanın adı, ör. "Papirus").
settings-theme-grid-footer-fallback-install = { $name } isn't installed; a fallback theme is used until it is. Install sets it up with administrator approval.

# Nerede: Görünüm sayfasında eksik paket notunun ve tema ızgarasının yanındaki metin düğme (src/pages/fix_button.rs). Anlam: Önerilen resmî depo paketlerini kurar; yönetici onayı istenir.
settings-packages-install = Install

# Nerede: Aynı düğme, kurulum sürerken (basılamaz) (src/pages/fix_button.rs). Anlam: Paketlerin kurulmakta olduğu.
settings-packages-installing = Installing…

# Nerede: Eksik paket notunun sonuna eklenen cümle (src/pages/look.rs, src/pages/theme_grid.rs). Anlam: Son kurulum denemesinin başarısız olduğu.
settings-packages-failed = The last installation didn't finish.

# Nerede: Aynı not; kurulum komutu bilinmiyorken (src/pages/theme_grid.rs). Anlam: Tema kurulunca uygulanacağı, o zamana dek yedek temanın kullanılacağı.
# Değişkenler: $name (temanın adı, ör. "Flat-Remix").
settings-theme-grid-footer-fallback = { $name } is not installed; it is applied once installed, and a fallback theme is used until then.

# Nerede: Ortam tuşları satırındaki örnek bind bloğunun ilk satırındaki yorum (src/pages/media_keys.rs). Anlam: Bloğun ne olduğu; blok kopyalanıp hyprland.conf'a yapıştırılır. Bloğun kalanı config sözdizimi olduğundan çevrilmez.
settings-media-keys-example-comment = Nuxut — media keys (e: repeat while held, l: also on the lock screen)

# Nerede: Ortam tuşları satırındaki tonlu düğme (src/pages/media_keys.rs). Anlam: Örnek bind bloğunu panoya kopyalar.
settings-media-keys-copy = Copy

## Görünüm durumu

# Nerede: Görünüm sayfasındaki durum kartı; shell'e soru sorulurken (src/services/look.rs). Anlam: Uygulamaların durumunun sorulmakta olduğu.
settings-look-status-pending = Asking apps for their status…

# Nerede: Görünüm sayfasındaki durum kartı; shell çalışmıyorken (src/services/look.rs). Anlam: Seçimlerin kabuk açılınca uygulanacağı.
settings-look-status-not-running = The desktop shell is not running; your choices will be applied once it starts.

# Nerede: Görünüm sayfasındaki durum kartı; shell sürümü görünüm aktarımını desteklemiyorken (src/services/look.rs). Anlam: Kabuğun bu sürümünün görünüm aktarımını henüz desteklemediği.
settings-look-status-unsupported = This version of the shell does not yet support applying the look to apps.

# Nerede: Görünüm sayfasındaki durum kartı; Ayarlar test örneğindeyken (src/services/look.rs). Anlam: Test örneğinde kabuğa sorulmadığı.
settings-look-status-test-instance = Test instance: the shell was not asked.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); durum sorgusu başarısızken (src/services/look.rs). Anlam: Uygulamaların durumunun alınamadığı.
settings-look-status-failed = Could not get the apps' status.

# Nerede: Görünüm sayfasındaki durum kartı; shell'in dış yazımı bu sürümde kapalıyken (src/services/look.rs). Anlam: Seçimlerin kaydedildiği ama uygulamalara aktarımın sonraki sürümde geleceği.
settings-look-status-writer-disabled = Choices saved; applying them to other apps will arrive in a later version of the shell.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); son uygulama hatalıyken (src/services/look.rs). Anlam: Görünümün bazı uygulamalara aktarılamadığı.
settings-look-status-apply-failed = The look could not be applied to some apps; details are in the shell log.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); kabuk aynı ayarı kısa sürede çok kez yeniden yazmak zorunda kalıp geçici olarak durdurduğunda (src/services/look.rs). Anlam: Başka bir araçla çekişme yüzünden bazı ayarların birkaç dakika bekletildiği.
settings-look-status-paused = Some look settings are paused for a few minutes because another tool keeps reverting them.

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); bir dosyayı başka bir araç yönettiği için Nuxut ona yazmadığında, dosya başına bir cümle; ardından settings-look-external-effect gelir (src/services/look.rs). Anlam: Dosyanın salt okunur, bağlantı ya da başka bir aracın denetiminde olduğu.
# Değişkenler: $path (dosya yolu, ör. "~/.config/gtk-3.0/gtk.css").
settings-look-external-managed = { $path } is managed by another tool.

# Nerede: settings-look-external-managed ile aynı yer; dosya başka bir kullanıcıya (çoğunlukla root) aitken (src/services/look.rs). Anlam: Nuxut'un dosyayı şimdilik değiştiremediği; sahiplik komutu önerilmez.
# Değişkenler: $path (dosya yolu, ör. "~/.config/kdeglobals").
settings-look-external-not-owned = { $path } belongs to another user, so Nuxut can't change it yet.

# Nerede: settings-look-external-managed ile aynı yer; dosya Nix ya da Guix deposundayken (src/services/look.rs). Anlam: Değişikliğin o paket yöneticisinin yapılandırmasından yapılması gerektiği.
# Değişkenler: $path (dosya yolu).
settings-look-external-store = { $path } is managed by Nix or Guix; changes belong there.

# Nerede: settings-look-external-managed ile aynı yer; dosyanın bulunduğu klasöre yazılamıyorken (src/services/look.rs). Anlam: Dosyanın konumuna yazma izni olmadığı.
# Değişkenler: $path (dosya yolu).
settings-look-external-dir = { $path } can't be written to.

# Nerede: Görünüm durum kartında her dış dosya cümlesinin ardından (src/services/look.rs). Anlam: O dosyayı okuyan uygulamaların Nuxut'un ilgili ayarını almadığı.
# Değişkenler: $what (settings-look-external-what-* metinlerinden biri, ör. "Nuxut renklerini").
settings-look-external-effect = Apps that read this file don't get { $what }.

# Nerede: settings-look-external-effect içindeki $what; dosya renk kipi ayarına aitken. Anlam: Nuxut'un renkleri (cümlede nesne, belirtme durumu).
settings-look-external-what-colors = the Nuxut colors

# Nerede: settings-look-external-effect içindeki $what; ikon teması ayarı. Anlam: Nuxut'un ikonları (nesne).
settings-look-external-what-icons = the Nuxut icons

# Nerede: settings-look-external-effect içindeki $what; imleç ayarı. Anlam: Nuxut'un imleci (nesne).
settings-look-external-what-cursor = the Nuxut cursor

# Nerede: settings-look-external-effect içindeki $what; yazı tipi ayarı. Anlam: Nuxut'un yazı tipleri (nesne).
settings-look-external-what-fonts = the Nuxut fonts

# Nerede: settings-look-external-effect içindeki $what; diğer görünüm ayarları. Anlam: Nuxut'un görünümü (nesne).
settings-look-external-what-look = the Nuxut look

# Nerede: Görünüm durum kartında, ilk iki dosyadan sonra kalan dış dosyaların sayısı (src/services/look.rs). Anlam: Aynı durumda başka dosyalar da olduğu.
# Değişkenler: $count (kalan dosya sayısı, ör. 2).
settings-look-external-more = { $count ->
    [one] { $count } more file is in the same state.
   *[other] { $count } more files are in the same state.
}

# Nerede: Görünüm sayfasındaki durum kartı (olumlu ton), "uygulandı" metninin sonuna eklenir; Nuxut başka bir aracın yönettiği bir dosyayı (ör. bir temaya bağlantı) kendi dosyasıyla değiştirdiğinde (src/services/look.rs). Anlam: Dosyanın artık Nuxut'ta olduğu ve birleşik görünümü kapatmanın eskisini geri getirdiği.
# Değişkenler: $path (dosya yolu, ör. "~/.config/gtk-4.0/gtk.css").
settings-look-taken-over-one = Nuxut took over { $path } from another tool; turning off unified look puts the original back.

# Nerede: settings-look-taken-over-one ile aynı yer; birden çok dosya devralındığında (src/services/look.rs). Anlam: İlk dosya ve kalan devralınan dosya sayısı.
# Değişkenler: $path (ilk dosyanın yolu), $count (kalan dosya sayısı, ör. 2).
settings-look-taken-over-many = { $count ->
    [one] Nuxut took over { $path } and { $count } more file from other tools; turning off unified look puts the originals back.
   *[other] Nuxut took over { $path } and { $count } more files from other tools; turning off unified look puts the originals back.
}

# Nerede: Görünüm sayfasındaki durum kartı (uyarı tonu); paket eksikken; AUR paketi varsa ve yardımcı yoksa sonuna AUR yardımcısı notu eklenir (src/services/look.rs). Anlam: Bazı uygulamalar için gereken ek bileşen sayısı ve adları.
# Değişkenler: $count (eksik bileşen sayısı, ör. 2), $names (paket adlarının virgüllü listesi, ör. "xdg-desktop-portal-gtk, xdg-desktop-portal-hyprland").
settings-look-status-missing-packages =
    { $count ->
        [one] Some apps need { $count } extra component: { $names }.
       *[other] Some apps need { $count } extra components: { $names }.
    }

# Nerede: Görünüm sayfasındaki durum kartı; başka bir araç görünüm ayarlarken (src/services/look.rs). Anlam: Çakışma var; Nuxut'un üstüne yazdığı.
settings-look-status-conflicts = Another tool is also setting the look; Nuxut overrides it.

## Takvim ve bağlantı

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata) (src/services/calendar.rs). Anlam: Takvim servisinin çalışmadığı.
settings-calendar-error-not-running = The calendar service is not running. Is nuxut-calendard installed?

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata) (src/services/calendar.rs). Anlam: Servisin yanıt vermediği.
settings-calendar-error-timeout = The calendar service did not respond.

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata) (src/services/calendar.rs). Anlam: Servise ulaşılamadığı.
settings-calendar-error-unreachable = Could not reach the calendar service.

# Nerede: Hesaplar sayfası, takvim hesabının durum satırı (hata); daemon iletisi yokken (src/services/calendar.rs). Anlam: Takvim işleminin başarısız olduğu.
settings-calendar-error-write = The calendar operation failed.

# Nerede: Görünüm sayfası, kabuk bağlantısı ayar dosyasından kaldırılmışsa çıkan not (src/services/hypr_status.rs). Anlam: Bağlantının nasıl geri ekleneceği.
settings-hypr-status-source-removed = The include line was removed from the settings file. Turn this setting off and on to add it again.

## Anlık görüntü: zaman etiketi

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın az önce alındığı.
settings-snapshots-ago-now = just now

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç dakika önce alındığı.
# Değişkenler: $count (dakika sayısı, ör. 5).
settings-snapshots-ago-minutes = { $count } min ago

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç saat önce alındığı.
# Değişkenler: $count (saat sayısı, ör. 3).
settings-snapshots-ago-hours = { $count } h ago

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın dün alındığı.
settings-snapshots-ago-yesterday = yesterday

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç gün önce alındığı.
# Değişkenler: $count (gün sayısı, ör. 4).
settings-snapshots-ago-days = { $count } d ago

# Nerede: Ayar kartlarındaki "Önceki hâl" listesinde kaydın yaşı (src/services/snapshots.rs). Anlam: Kaydın kaç hafta önce alındığı.
# Değişkenler: $count (hafta sayısı, ör. 2).
settings-snapshots-ago-weeks = { $count } w ago

## Anlık görüntü: geri yükleme

# Nerede: Ayar kartındaki geri yükleme sonucu (src/services/snapshots.rs). Anlam: Geri yüklemenin başarılı olduğu.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-restored = { $label } is back to its previous state

# Nerede: Ayar kartındaki geri yükleme sonucu (src/services/snapshots.rs). Anlam: Ayarın zaten önceki hâlinde olduğu.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-already = { $label } was already in its previous state

# Nerede: Ayar kartındaki geri yükleme sonucu (kısmi) (src/services/snapshots.rs). Anlam: Ayarın kısmen döndüğü, sonradan başka yerden değiştirilen ayarlara dokunulmadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni"), $count (dokunulmayan ayar sayısı, ör. 2).
settings-snapshots-outcome-partial =
    { $count ->
        [one] { $label } was partially restored: { $count } setting changed elsewhere afterwards was left untouched
       *[other] { $label } was partially restored: { $count } settings changed elsewhere afterwards were left untouched
    }

# Nerede: Ayar kartındaki geri yükleme sonucu (çakışma) (src/services/snapshots.rs). Anlam: Hiçbir şeye dokunulmadığı, çünkü ayarlar sonradan başka bir araçla değişmiş.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-conflict = { $label } was not restored: the settings were changed afterwards by another tool and none were touched

# Nerede: Ayar kartındaki geri yükleme sonucu (kayıt yok) (src/services/snapshots.rs). Anlam: Önceki hâlin artık kayıtlı olmadığı.
settings-snapshots-outcome-not-found = The previous state is no longer recorded

# Nerede: Ayar kartındaki geri yükleme sonucu (meşgul) (src/services/snapshots.rs). Anlam: Başka bir değişiklik sürdüğü için beklenmesi gerektiği.
settings-snapshots-outcome-busy = Another change is being applied; try again shortly

# Nerede: Ayar kartındaki geri yükleme sonucu (test örneği) (src/services/snapshots.rs). Anlam: Kabuğun test örneğinin dış ayarlara dokunmadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-skipped-test = { $label } was not restored: the shell's test instance does not touch external settings

# Nerede: Ayar kartındaki geri yükleme sonucu (oturum yok) (src/services/snapshots.rs). Anlam: Masaüstü oturumu bulunamadığı için geri alınamadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-skipped-no-session = { $label } cannot be restored right now: the desktop session was not found

# Nerede: Ayar kartındaki geri yükleme sonucu (bu oturumda kapalı) (src/services/snapshots.rs). Anlam: Kabuğun bu oturumda dış ayarlara dokunmadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-skipped-disabled = { $label } cannot be restored right now; the shell does not touch external settings in this session

# Nerede: Ayar kartındaki geri yükleme sonucu (hata) (src/services/snapshots.rs). Anlam: Geri yüklemenin başarısız olduğu; kabuğun hata türü parantez içinde.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni"), $kind (kabuğun hata türü kısa adı, ör. "io"; bilinmiyorsa "bilinmeyen").
settings-snapshots-outcome-error = { $label } could not be restored ({ $kind })

# Nerede: Ayar kartındaki geri yükleme sonucu (doğrulanamadı) (src/services/snapshots.rs). Anlam: Geri yüklemenin yapıldığı ama ayrıntının doğrulanamadığı.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-restored-unverified = { $label } was restored (details could not be verified)

# Nerede: Ayar kartındaki geri yükleme sonucu (onaysız) (src/services/snapshots.rs). Anlam: Geri yüklemenin doğrulanamadığı; durumun yenilenmesi gerektiği.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni").
settings-snapshots-outcome-unconfirmed = { $label } could not be confirmed as restored — refresh the status

# Nerede: Ayar kartındaki geri yükleme sonucu (başarısız) (src/services/snapshots.rs). Anlam: Geri yüklemenin başarısız olduğu; hata ayrıntısı cümlenin sonuna eklenir.
# Değişkenler: $label (ayarın başlığı, ör. "Panel düzeni"), $error (hata ayrıntısı, ör. "masaüstü kabuğu çalışmıyor").
settings-snapshots-outcome-failed = { $label } could not be restored: { $error }

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; shell çalışmıyorken (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-not-running = the desktop shell is not running

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; Ayarlar test örneğindeyken (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-test-instance = test instance

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; shell isteği reddedince (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-rejected = the shell did not accept the request

# Nerede: Ayar kartındaki geri yükleme başarısızlık ayrıntısı; shell'in yanıtı okunamadığında (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-failed içinde $error olarak görünür.
settings-snapshots-error-unparsable = the shell's reply could not be understood

## Widget listesi hataları

# Nerede: Paneller sayfası, widget listesi yüklenemediğinde hata satırı; shell şema komutu hatalı çıkış koduyla bittiğinde (src/services/widgets.rs). Anlam: Hatalı çıkış kodu.
# Değişkenler: $code (çıkış kodu, ör. 127).
settings-widgets-error-exit-code = Could not get the widget list: nuxut-shell schema exited with code { $code }.

# Nerede: Paneller sayfası, widget listesi yüklenemediğinde hata satırı; shell sinyalle sonlandığında (src/services/widgets.rs). Anlam: Sürecin sinyalle sonlandığı.
settings-widgets-error-signal = Could not get the widget list: nuxut-shell schema was terminated by a signal.

## Hoş geldin

# Nerede: Hoş geldin penceresinin giriş ekranı başlığı; aynı metin pencere başlığı olarak da kullanılır (src/welcome.rs, src/app.rs). Anlam: Karşılama.
settings-welcome-intro-title = Welcome to Nuxut

# Nerede: Hoş geldin penceresinin giriş ekranı açıklaması (src/welcome.rs). Anlam: Birkaç tercih yapılacağı, sonra Ayarlar'dan değiştirilebileceği.
settings-welcome-intro-body = Let's set a few preferences; you can change all of it later in Settings.

# Nerede: Hoş geldin 1. adım kartının başlığı (src/welcome.rs). Anlam: Renkler ve duvar kağıdı adımı.
settings-welcome-step1-title = Colors and wallpaper

# Nerede: Hoş geldin 1. adım kartının açıklaması (src/welcome.rs). Anlam: Koyu/açık seçimi ve renklerin duvar kağıdından gelme seçeneği.
settings-welcome-step1-body = Dark or light; the colors can also come from your wallpaper.

# Nerede: Hoş geldin 2. adım kartının başlığı (src/welcome.rs). Anlam: Uygulamaların Nuxut görünümünü alıp almayacağı sorusu.
settings-welcome-step2-title = Should apps look like Nuxut?

# Nerede: Hoş geldin 2. adım kartının açıklaması (src/welcome.rs). Anlam: Önerilenin ne olduğu ve geri alınabildiği.
settings-welcome-step2-body = Recommended: apps use Nuxut's colors, icons, and cursor. Your current look can be undone at any time.

# Nerede: Hoş geldin 3. adım kartının başlığı (src/welcome.rs). Anlam: Masaüstüyle ortak ayarlar adımı.
settings-welcome-step3-title = Shared with the desktop

# Nerede: Hoş geldin 3. adım kartının açıklaması (src/welcome.rs). Anlam: Köşe, gölge, hareket ve kısayol birliğinin önerildiği.
settings-welcome-step3-body = Corners, shadows, motion, and keybinds shared with windows (recommended).

# Nerede: Hoş geldin penceresi, "Masaüstüyle ortak" adımında en üstteki anahtarın başlığı (yalnızca ilk kurulumda, kullanıcının kendi Hyprland ayarı varken). Anlam: kullanıcının Hyprland ana ayar dosyasını Nuxut'un yönetmesi.
settings-welcome-hyprland-config-title = Let Nuxut manage my Hyprland config

# Nerede: Aynı anahtarın alt yazısı. Anlam: eski ayar dosyasının yedekleneceği; klavye, fare, ekran kartı ve ortam ayarlarının yeni ayara taşınacağı; kısayolların ve başlangıç komutlarının taşınmayacağı; çıkan bildirimdeki "Geri al" ile tek tıkla eski hâline dönüleceği.
settings-welcome-hyprland-config-body = Your old config is backed up; keyboard, mouse, graphics card and environment settings move over. Shortcuts and startup commands do not. Use "Undo" in the notification to go back.
# Nerede: Hoş geldin penceresi, 4. adımın başlığı (src/welcome.rs). Anlam: Bilgisayar açılırken ve kapanırken görünen ekranların görünümü.
settings-welcome-step4-title = Startup look

# Nerede: Hoş geldin, 4. adımın açıklaması (src/welcome.rs). Anlam: Açılış ekranının ve önyükleme menüsünün Nuxut renklerini alması; yönetici parolası, yedek ve geri alma.
settings-welcome-step4-body = Let the screens you see while the computer starts and shuts down use Nuxut's colors too. An administrator password is required; every file is backed up first and can be undone with one click.

# Nerede: Hoş geldin 4. adım, anahtar satırının başlığı (src/pages/system_theme.rs). Anlam: Açılış ekranını ve önyükleme menüsünü Nuxut temasına geçirme seçeneği.
settings-system-theme-switch = Switch the boot menu and startup screen to Nuxut

# Nerede: Anahtar satırının alt metninde hedef adı (src/pages/system_theme.rs). Anlam: Açılışta görünen, disk parolasını da soran ekran.
settings-system-theme-target-plymouth = Startup screen

# Nerede: Anahtar satırının alt metninde hedef adı (src/pages/system_theme.rs). Anlam: GRUB önyükleme menüsü.
settings-system-theme-target-grub = Boot menu (GRUB)

# Nerede: Anahtar satırının alt metninde hedef adı (src/pages/system_theme.rs). Anlam: Limine önyükleme menüsü.
settings-system-theme-target-limine = Boot menu (Limine)

# Nerede: Anahtar satırının alt metni, durum okunurken (src/pages/system_theme.rs). Anlam: Sistemin incelendiği.
settings-system-theme-reading = Checking the system…

# Nerede: Anahtar satırının alt metni (src/pages/system_theme.rs). Anlam: Bu kurulumda özelliğin çalışmadığı.
settings-system-theme-unavailable = Not available in this version.

# Nerede: Anahtar satırının alt metni (src/pages/system_theme.rs). Anlam: Değiştirilecek bir şey olmadığı.
settings-system-theme-nothing = Nothing to change on this system.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Gereken sistem yardımcısının bu kurulumda bulunmadığı (komut önerilmez).
settings-system-theme-helper-missing = The system helper isn't part of this installation, so the boot look can't be changed.

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: Sistem yardımcısı dosyasının güvenli olmadığı için kullanılmadığı.
settings-system-theme-helper-unsafe = The system helper isn't in a safe location, so it isn't used.

# Nerede: Kartın notu, bir hedef engelliyken (src/pages/system_theme.rs). Anlam: Bu hedefe dokunulmayacağı.
settings-system-theme-blocked = left unchanged.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: systemd-boot menüsünün temalanamadığı.
settings-system-theme-systemd-boot = The systemd-boot menu can't be themed; the startup screen still uses Nuxut.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Tema zaten uygulanmış.
settings-system-theme-applied = The startup look is Nuxut. To go back, use "Undo".

# Nerede: "Neler değişecek" satırı (src/pages/system_theme.rs). Anlam: Bir dosyanın ya da klasörün ekleneceği.
# Değişkenler: $what (yol).
settings-system-theme-change-create = { $what } is added

# Nerede: "Neler değişecek" satırı (src/pages/system_theme.rs). Anlam: Bir dosyanın değişeceği ve önce yedekleneceği.
# Değişkenler: $what (yol).
settings-system-theme-change-modify = { $what } changes (backed up first)

# Nerede: "Neler değişecek" satırı (src/pages/system_theme.rs). Anlam: Bir komutun çalıştırılacağı.
# Değişkenler: $what (komut).
settings-system-theme-change-run = { $what } runs

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Açılış ekranı yazılımının kurulu ama etkin olmadığı.
settings-system-theme-note-plymouth-not-in-initramfs = The startup screen isn't enabled; the theme is installed, enabling it is a separate step.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Çekirdek satırında açılış ekranı seçeneğinin olmadığı.
settings-system-theme-note-plymouth-no-splash = The kernel command line has no "splash"; the startup screen may not appear.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Açılış imgesi yeniden kurulmadıkça açılış ekranının görünmeyeceği; Nuxut bu sistemde bunu yapamıyor.
# Değişkenler: $command (açılış imgesi üreticisinin adı: mkinitcpio ya da dracut; komut değil).
settings-system-theme-note-plymouth-manual-rebuild = The splash screen appears once the boot image ({ $command }) is rebuilt; Nuxut can't do that itself on this system.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: GRUB'un metin kipinde çalıştığı.
settings-system-theme-note-grub-text-terminal = GRUB runs in text mode; the theme won't show.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: GRUB ayar dosyasının elle yazılmış olduğu.
settings-system-theme-note-grub-hand-written = The GRUB configuration is hand-written; it's left unchanged.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: GRUB ayar dosyasının okunamadığı.
settings-system-theme-note-grub-cfg-unreadable = The GRUB configuration can't be read.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Eski GRUB sürümünün ek ayar dosyalarını desteklemediği.
settings-system-theme-note-grub-no-dropins = This GRUB version doesn't read extra configuration files; it's left unchanged.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Başka bir ayar dosyasının temayı ezdiği.
# Değişkenler: $path (o dosyanın yolu).
settings-system-theme-note-grub-overridden = Left unchanged because { $path } overrides the theme.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Birden çok Limine ayar dosyası bulunduğu.
# Değişkenler: $paths (yollar).
settings-system-theme-note-limine-ambiguous = There are several Limine configurations ({ $paths }); left unchanged.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Limine ayarının okunamadığı.
settings-system-theme-note-limine-unreadable = The Limine configuration can't be read.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Limine ayarının tanınmayan biçimde olduğu.
settings-system-theme-note-limine-unsupported = The Limine configuration has an unrecognized format; left unchanged.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Limine'nin ayar dosyasının imzalı olduğu, değişirse açılmayacağı.
settings-system-theme-note-limine-enrolled = The Limine configuration is signed (checksum enrolled); left unchanged.

# Nerede: Kartın notu, uygulama başlarken (src/pages/system_theme.rs). Anlam: Yönetici parolasının sorulduğu ya da işin başladığı.
settings-system-theme-waiting = Waiting for administrator approval…

# Nerede: Kartın notu, uygulama sürerken (src/pages/system_theme.rs). Anlam: Hangi hedefte hangi adımın yapıldığı.
# Değişkenler: $target (hedef adı), $step (teknik adım adı).
settings-system-theme-running-step = { $target }: { $step }…

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Yönetici parolasının verilmediği, hiçbir şey değişmediği.
settings-system-theme-cancelled = Administrator approval wasn't given; nothing changed.

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: Bir hedefin uygulanamadığı, değişikliklerinin geri alındığı.
# Değişkenler: $target (hedef adı), $message (ayrıntı).
settings-system-theme-rolled-back = { $target } couldn't be applied; its changes were undone. { $message }

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: Geri almanın da başarısız olduğu; yedeklerin yeri mesajda.
# Değişkenler: $target (hedef adı), $message (yedek yolları).
settings-system-theme-inconsistent = { $target } was left half-done; backups: { $message }

# Nerede: Kartın uyarı notu (src/pages/system_theme.rs). Anlam: İşin tamamlanamadığı.
settings-system-theme-failed = Couldn't finish; nothing was left half-done.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Temanın uygulandığı, yeniden başlatınca görüneceği.
settings-system-theme-done = Applied. You'll see it after restarting.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Önceki görünümün geri geldiği.
settings-system-theme-reverted = The previous startup look is back.

# Nerede: Kartın düğmesi (src/pages/system_theme.rs). Anlam: Nuxut açılış temasını geri almak.
settings-system-theme-revert = Undo

# Nerede: Kartın düğmesi (src/pages/system_theme.rs). Anlam: Kurulu sistem dosyalarını (açılış görünümü yardımcısı, giriş ekranı) yeni sürüme güncellemek; yönetici onayı istenir.
settings-system-theme-update = Update

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Sistem dosyalarının yeni sürümünün hazır olduğu; düğmeyle güncellenir.
settings-system-theme-helper-update = A newer version of the system files is ready; Update installs it with administrator approval.

# Nerede: Kartın notu (src/pages/system_theme.rs). Anlam: Sistem yardımcısı henüz kurulu değil; uygularken yönetici onayıyla kendiliğinden kurulacak.
settings-system-theme-helper-install = Applying also installs the system files, with administrator approval.

# Nerede: Kartın başarı notu (src/pages/system_theme.rs). Anlam: Sistem dosyalarının güncellendiği.
settings-system-theme-updated = The system files were updated.

# Nerede: Hoş geldin birincil düğmesi, açılış görünümü adımında (src/welcome_ui.rs). Anlam: Seçimi uygulamak.
settings-system-theme-button-apply = Apply

# Nerede: Hoş geldin birincil düğmesi, uygulama sürerken (src/welcome_ui.rs). Anlam: İşin sürdüğü.
settings-system-theme-button-running = Applying…

# Nerede: Hoş geldin bitiş ekranının başlığı (src/welcome.rs). Anlam: Akışın tamamlandığı.
settings-welcome-done-title = All set

# Nerede: Hoş geldin bitiş ekranının açıklaması; aynı metin Görünüm durum kartında "Uygulandı." sonuna da eklenir (src/welcome.rs, src/services/look.rs). Anlam: Bazı uygulamaların yeniden açılınca yeni görünüme geçeceği.
settings-welcome-done-body = Some open apps will pick up the new look when reopened.

# Nerede: Hoş geldin giriş ekranındaki birincil düğme; sonraki ekranlarda ilk değer olarak da kullanılır (src/welcome.rs, src/welcome_ui.rs). Anlam: Akışı başlatır.
settings-welcome-button-start = Get started

# Nerede: Hoş geldin adım ekranlarındaki birincil düğme (src/welcome.rs). Anlam: Sonraki adıma geçer.
settings-welcome-button-next = Next

# Nerede: Hoş geldin adım ekranlarındaki metin düğme (src/welcome_ui.rs). Anlam: Adımı atlayıp ilerler.
settings-welcome-button-skip = Skip

# Nerede: Hoş geldin son adımındaki birincil düğme (src/welcome.rs). Anlam: Akışı bitirip bitiş ekranına geçer.
settings-welcome-button-finish = Finish

# Nerede: Hoş geldin bitiş ekranındaki birincil düğme (src/welcome.rs). Anlam: Pencereyi kapatır.
settings-welcome-button-close = Close

# Nerede: Hoş geldin adım ekranlarındaki geri düğmesi (src/welcome_ui.rs). Anlam: Önceki adıma döner.
settings-welcome-button-back = Back

# Nerede: Hoş geldin adım ekranlarının üstündeki ilerleme satırı (src/welcome.rs). Anlam: Bulunulan adım ve toplam adım sayısı.
# Değişkenler: $current (bulunulan adım, ör. 2), $total (toplam adım sayısı, ör. 3).
settings-welcome-progress = Step { $current } / { $total }

## Tarih ve saat sayfası

# Nerede: Saat dilimi satırının alt metni (src/pages/datetime.rs). Anlam: UTC ofseti.
# Değişkenler: $offset (ör. +03:00).
settings-datetime-utc = UTC{ $offset }

# Nerede: Saat dilimi satırının alt metni (src/pages/datetime.rs). Anlam: Geçerli saat dilimi ve UTC ofseti.
# Değişkenler: $zone (ör. Istanbul, Europe), $offset (ör. UTC+03:00).
settings-datetime-zone-with-offset = { $zone } — { $offset }

## Varsayılan uygulamalar

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Kabuktan uygulama listesi bekleniyor.
settings-defaults-note-loading = Loading apps…

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Masaüstü kabuğu çalışmadığı için uygulama listesi gösterilemiyor.
settings-defaults-note-not-running = The desktop shell isn't running; default apps will appear when the shell starts.

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Kabuğa bağlanıldı ama yanıt zamanında gelmedi ya da bağlantı koptu.
settings-defaults-note-failed = Default apps couldn't be read from the shell; it will retry when you return to the window.

# Nerede: Varsayılan uygulamalar sayfası, kabuk çalışmıyorken maskotun altındaki başlık. Anlam: masaüstü kabuğu kapalı.
settings-defaults-empty-not-running-title = The desktop shell is not running

# Nerede: Varsayılan uygulamalar sayfası, kabuk çalışmıyorken başlığın altındaki açıklama. Anlam: liste kabuk açılınca kendiliğinden gelir.
settings-defaults-empty-not-running-body = Default apps will appear here when the shell starts.

# Nerede: Varsayılan uygulamalar sayfası, kabuktan okuma başarısızken maskotun altındaki başlık. Anlam: liste okunamadı.
settings-defaults-empty-failed-title = Default apps could not be read

# Nerede: Varsayılan uygulamalar sayfası, okuma başarısızken başlığın altındaki açıklama. Anlam: pencereye dönünce yeniden denenir.
settings-defaults-empty-failed-body = It will retry when you return to the window.

# Nerede: Varsayılan uygulamalar sayfasının üstündeki not (src/services/default_apps.rs). Anlam: Çalışan kabuk sürümü bu özelliği bilmiyor.
settings-defaults-note-unsupported = This version of the shell doesn't manage default apps; update the shell.

# Nerede: Varsayılan uygulamalar, rol satırının açılır seçicisi (src/pages/default_apps.rs). Anlam: Bu dosya türünü şu an hiçbir uygulama açmıyor; seçilemeyen yer tutucu.
settings-defaults-none = None

# Nerede: Varsayılan uygulamalar, rol satırının açılır seçicisi (src/pages/default_apps.rs). Anlam: Mevcut varsayılan uygulama artık kurulu değil.
# Değişkenler: $name (uygulamanın adı ya da kimliği, ör. "firefox.desktop").
settings-defaults-not-installed = { $name } (not installed)

# Nerede: Varsayılan uygulamalar, rol satırının açılır seçicisi (src/pages/default_apps.rs). Anlam: Aynı adlı iki uygulamayı kimlikleriyle ayırır.
# Değişkenler: $name (uygulamanın adı, ör. "Firefox"), $id (masaüstü kimliği, ör. "firefox-esr.desktop").
settings-defaults-app-with-id = { $name } ({ $id })

# Nerede: Varsayılan uygulamalar sayfasında, rol satırının açılır seçicisindeki bölüm başlığı (src/pages/default_apps.rs). Anlam: Menülerde gösterilmeyen (NoDisplay) ama bu dosya türünü açabilen yardımcı uygulamalar bu başlığın altında listelenir.
settings-defaults-other = Other

# Nerede: Varsayılan uygulamalar, Terminal satırının altındaki not (src/pages/default_apps.rs). Anlam: Terminal seçimi için gereken araç kurulu değil; seçici kapalı.
settings-defaults-note-terminal-unavailable = xdg-terminal-exec is needed for terminal selection; it isn't installed.

# Nerede: Varsayılan uygulamalar, rol satırının altındaki not (src/pages/default_apps.rs). Anlam: Bu rolün dosya türlerinden bazıları farklı uygulamalarda açılıyor; yeni seçim hepsini tek uygulamaya bağlar.
settings-defaults-note-mixed = Some file types open in a different app; picking one will open all of them with the same app.

# Nerede: Varsayılan uygulamalar, rol satırının altındaki uyarı (src/pages/default_apps.rs). Anlam: Masaüstüne özgü bir ayar dosyası bu seçimi geçersiz kılıyor; Nuxut o dosyaya dokunmuyor.
# Değişkenler: $file (dosya yolu, ör. "~/.config/hyprland-mimeapps.list").
settings-defaults-note-shadowed = { $file } overrides this choice; Nuxut won't change that file.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk bu rolü tanımıyor; Ayarlar ile kabuğun sürümleri uyuşmuyor.
settings-defaults-error-unknown-role = The shell doesn't recognize this role; update Settings and the shell.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Seçilen uygulama bu rol için uygun değil ya da kaldırılmış.
settings-defaults-error-unknown-app = This app is no longer installed or cannot open this file type; the list has been refreshed.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Terminal seçimi için gereken araç yok.
settings-defaults-error-unavailable = xdg-terminal-exec is required for terminal selection; the selection was not saved.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk bu oturumda ayar dosyalarına yazmıyor.
settings-defaults-error-writer-disabled = The shell is not writing to settings files in this session; the selection was not saved.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: mimeapps.list dosyası yazılamadı.
settings-defaults-error-write-failed = mimeapps.list could not be written; the selection was not saved.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk isteği reddetti ya da yanıtı anlaşılamadı.
settings-defaults-error-rejected = The shell didn't accept the selection; make sure Settings and the shell are up to date.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuk çalışmıyor.
settings-defaults-error-not-running = The desktop shell is not running; the selection was not saved.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Yanıt zamanında gelmedi; seçim yapılmış olabilir, durum yeniden okunuyor.
settings-defaults-error-timeout = The shell did not respond in time; the selection's state is being read again.

# Nerede: Varsayılan uygulama seçimi başarısız olunca alttaki şerit (src/services/default_apps.rs). Anlam: Kabuğa bağlanırken hata oluştu.
# Değişkenler: $error (bağlantı hatasının metni).
settings-defaults-error-io = Could not reach the shell: { $error }

## Ekranlar: Gece ışığı

# Nerede: Ekranlar › Gece ışığı, "Konum" satırı; zamanlama "Gün batımından gün doğumuna" iken (src/pages/night_light.rs). Anlam: Gün batımı/doğumu için kullanılan konum, saat diliminin şehrinden yaklaşık olarak bulundu.
# Değişkenler: $lat (enlem, yön harfiyle, ör. "41,0° K"), $lon (boylam, yön harfiyle, ör. "29,0° D").
settings-night-light-location-timezone = Location: from time zone (≈ { $lat }, { $lon })

# Nerede: Aynı satır; konum kullanıcının ayar dosyasında elle yazılıysa (src/pages/night_light.rs). Anlam: Elle girilmiş konum kullanılıyor.
# Değişkenler: $lat (enlem, yön harfiyle, ör. "41,0° K"), $lon (boylam, yön harfiyle, ör. "29,0° D").
settings-night-light-location-manual = Location: manual ({ $lat }, { $lon })

# Nerede: Aynı satır; konum bulunamadığında (src/pages/night_light.rs). Anlam: Gün batımı hesaplanamıyor, sabit 20:00–07:00 aralığı kullanılıyor (saatler sabit, çevrilirken aynen kalır).
settings-night-light-location-none = No location found — using 20:00–07:00

# Nerede: Konum bulunamadığında satırın altındaki küçük not (src/pages/night_light.rs). Anlam: Konum elle ayar dosyasına yazılabilir; dosya ve anahtar adları çevrilmez.
settings-night-light-location-hint = Manual location: shell.toml [night_light] latitude, longitude

# Nerede: Konum satırının altındaki küçük not (src/pages/night_light.rs). Anlam: Bugün güneşin batış ve doğuş saati (yerel saat).
# Değişkenler: $sunset (gün batımı saati, ör. "18:32"), $sunrise (gün doğumu saati, ör. "07:14").
settings-night-light-today = Today: sunset { $sunset } · sunrise { $sunrise }

# Nerede: Aynı not; kutup gününde (src/pages/night_light.rs). Anlam: Bugün güneş hiç batmıyor, gece ışığı açılmaz.
settings-night-light-polar-day = The sun doesn't set today

# Nerede: Aynı not; kutup gecesinde (src/pages/night_light.rs). Anlam: Bugün güneş hiç doğmuyor, gece ışığı tüm gün açık.
settings-night-light-polar-night = The sun doesn't rise today

# Nerede: Konum satırındaki enlem (src/pages/night_light.rs). Anlam: Kuzey enlemi; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "41,0").
settings-night-light-north = { $value }° N

# Nerede: Konum satırındaki enlem (src/pages/night_light.rs). Anlam: Güney enlemi; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "33,9").
settings-night-light-south = { $value }° S

# Nerede: Konum satırındaki boylam (src/pages/night_light.rs). Anlam: Doğu boylamı; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "29,0").
settings-night-light-east = { $value }° E

# Nerede: Konum satırındaki boylam (src/pages/night_light.rs). Anlam: Batı boylamı; harf yönün kısaltması.
# Değişkenler: $value (derece, bir ondalıkla, ör. "74,0").
settings-night-light-west = { $value }° W


# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.mode ayarının satır başlığı. Anlam: Kullanıcıya koyu ve açık renk kipi seçeneğini tanıtır.
settings-colors-mode-label = Dark or light

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.mode ayarının alt yazısı. Anlam: Nuxut ile uygulamaların aynı renk kipini paylaştığını belirtir.
settings-colors-mode-description = Nuxut and apps follow the same mode

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.mode ayarının arama sözcükleri. Anlam: Bu sözcüklerle renk kipi ayarı aramada bulunur.
settings-colors-mode-keywords = dark, light, mode, night, theme

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette.mode ayarının satır başlığı. Anlam: Renk şeması kipi seçiminin başlığıdır.
settings-palette-mode-label = Color scheme mode

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.source ayarının satır başlığı. Anlam: Renklerin kaynağının seçildiği satırın başlığıdır.
settings-colors-source-label = Where colors come from

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.source ayarının alt yazısı. Anlam: Duvar kağıdı seçildiğinde tüm renklerin görsele uyacağını anlatır.
settings-colors-source-description = Nuxut theme: ready-made color themes. From wallpaper: all colors match the image

# Nerede: Görünüm › Renkler, colors.theme ayarının satır başlığı (src/registry.rs). Anlam: altı hazır Nuxut renk temasından birinin seçildiği satır.
settings-colors-theme-label = Color theme

# Nerede: Aynı satırın alt yazısı. Anlam: seçilen temanın arayüze, uygulamalara ve yerleşik Nuxut duvar kağıtlarına uygulandığı.
settings-colors-theme-description = The interface, apps and Nuxut wallpapers take on these colors

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette ayarının alt yazısı. Anlam: Aynı görselden dokuz farklı palet seçilebildiğini anlatır.
settings-palette-description = Nine different palettes from the same image

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette ayarının arama sözcükleri. Anlam: Bu sözcüklerle palet ayarı aramada bulunur.
settings-palette-keywords = color scheme, matugen, palette, variant, tonal, vivid

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.primary ayarının satır başlığı. Anlam: Vurgu rengi seçiminin başlığıdır.
settings-colors-primary-label = Accent color

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.primary ayarının alt yazısı. Anlam: Vurgu renginin etkin çalışma alanında, seçimlerde ve düğmelerde kullanıldığını sıralar.
settings-colors-primary-description = Active workspace, selection and buttons

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_primary ayarının satır başlığı. Anlam: Vurgu rengi üstündeki yazı renginin başlığıdır.
settings-colors-on-primary-label = Text on accent

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_primary ayarının alt yazısı. Anlam: Vurgu renginin üstündeki yazı ve ikonları etkilediğini anlatır.
settings-colors-on-primary-description = Text and icons on top of the accent color

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_primary ayarının arama sözcükleri. Anlam: Bu sözcükle on_primary rengi aramada bulunur.
settings-colors-on-primary-keywords = text on accent

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_container ayarının alt yazısı. Anlam: Panel içindeki kartların bu rengi kullandığını anlatır.
settings-colors-surface-container-description = Cards inside the panel

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_bright ayarının satır başlığı. Anlam: Vurgulu yüzey renginin başlığıdır.
settings-colors-surface-bright-label = Highlighted surface

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_bright ayarının alt yazısı. Anlam: Bu rengin fareyle üzerine gelindiğinde kullanıldığını anlatır.
settings-colors-surface-bright-description = When hovered with the mouse

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface ayarının satır başlığı. Anlam: Ana yazı renginin başlığıdır.
settings-colors-on-surface-label = Text

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface ayarının alt yazısı. Anlam: Bunun arayüzdeki ana yazı rengi olduğunu anlatır.
settings-colors-on-surface-description = Main text color

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface_variant ayarının satır başlığı. Anlam: İkincil yazı renginin başlığıdır.
settings-colors-on-surface-variant-label = Secondary text

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface_variant ayarının alt yazısı. Anlam: Açıklama ve soluk yazılarda kullanıldığını anlatır.
settings-colors-on-surface-variant-description = Descriptions, dimmed text

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface_variant ayarının arama sözcükleri. Anlam: Bu sözcüklerle ikincil yazı rengi aramada bulunur.
settings-colors-on-surface-variant-keywords = secondary text

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_error ayarının satır başlığı. Anlam: Hata rengi üstündeki yazının başlığıdır.
settings-colors-on-error-label = Text on error

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_error ayarının alt yazısı. Anlam: Hata renginin üstündeki yazıyı etkilediğini anlatır.
settings-colors-on-error-description = Text on top of the error color

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_error ayarının arama sözcükleri. Anlam: Bu sözcüklerle hata üstü yazı rengi aramada bulunur.
settings-colors-on-error-keywords = text on error

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_small ayarının alt yazısı. Anlam: Küçük köşe yarıçapının çiplerde ve çalışma alanı göstergelerinde kullanıldığını sıralar.
settings-shape-radius-small-description = Chips, workspace indicators

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_small ayarının arama sözcükleri. Anlam: Bu sözcüklerle küçük yarıçap ayarı aramada bulunur.
settings-shape-radius-small-keywords = corner, radius, round

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_medium ayarının alt yazısı. Anlam: Orta köşe yarıçapının panel içindeki kartlarda kullanıldığını anlatır.
settings-shape-radius-medium-description = Cards inside the panel

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_medium ayarının arama sözcükleri. Anlam: Bu sözcüklerle orta yarıçap ayarı aramada bulunur.
settings-shape-radius-medium-keywords = corner, radius, round

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, shape.radius_large ayarının arama sözcükleri. Anlam: Bu sözcüklerle büyük yarıçap ayarı aramada bulunur.
settings-shape-radius-large-keywords = corner, radius, round

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.smoothing ayarının satır başlığı. Anlam: Birleşim kavisi ayarının başlığıdır.
settings-blob-smoothing-label = Merge curve

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.smoothing ayarının alt yazısı. Anlam: Çerçevenin iç köşelerinin ve panel birleşimlerinin yumuşatıldığını anlatır.
settings-blob-smoothing-description = Inner corners of the frame and panel joins

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.deform ayarının alt yazısı. Anlam: Hızlı açılan panellerin esneyip basıldığını anlatır.
settings-blob-deform-description = Fast panels stretching and squishing

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.unified ayarının satır başlığı. Anlam: Uygulamaların da Nuxut görünümünü almasını sağlayan ayarın başlığıdır.
settings-desktop-unified-label = Make apps look like Nuxut too

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.unified ayarının alt yazısı. Anlam: Renk, ikon, imleç ve yazı tiplerinin Nuxut'tan geldiğini ve her an geri alınabildiğini anlatır.
settings-desktop-unified-description = Color, icon, cursor and fonts come from Nuxut; revert any time

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.unified ayarının arama sözcükleri. Anlam: Bu sözcüklerle birleşik görünüm ayarı aramada bulunur.
settings-desktop-unified-keywords = unified look, gtk, qt, kde, libadwaita, theme, compatible

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.status ayarının satır başlığı. Anlam: Uygulama teması durumunun gösterildiği satırın başlığıdır.
settings-desktop-status-label = Status

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.status ayarının arama sözcükleri. Anlam: Bu sözcüklerle durum satırı aramada bulunur.
settings-desktop-status-keywords = missing package, installation

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.icon_theme ayarının satır başlığı. Anlam: İkon teması seçiminin başlığıdır.
settings-desktop-icon-theme-label = Icon theme

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.icon_theme ayarının alt yazısı. Anlam: İkon temasının uygulama, dosya ve klasör ikonlarını etkilediğini anlatır.
settings-desktop-icon-theme-description = App, file and folder icons

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.icon_theme ayarının arama sözcükleri. Anlam: Bu sözcüklerle ikon teması ayarı aramada bulunur.
settings-desktop-icon-theme-keywords = icon, papirus, adwaita, breeze

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_theme ayarının satır başlığı. Anlam: İmleç teması seçiminin başlığıdır.
settings-desktop-cursor-theme-label = Cursor

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_theme ayarının alt yazısı. Anlam: Fare imlecinin görünümünü belirlediğini anlatır.
settings-desktop-cursor-theme-description = How the mouse cursor looks

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_theme ayarının arama sözcükleri. Anlam: Bu sözcüklerle imleç teması ayarı aramada bulunur.
settings-desktop-cursor-theme-keywords = mouse, cursor, pointer, bibata, adwaita, vimix, capitaine, oreo, volantes, qogir, sweet

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_size ayarının satır başlığı. Anlam: İmleç boyutu ayarının başlığıdır.
settings-desktop-cursor-size-label = Cursor size

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_size ayarının alt yazısı. Anlam: Büyük imlecin daha kolay görüldüğünü anlatır.
settings-desktop-cursor-size-description = A larger cursor is easier to see

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.cursor_size ayarının arama sözcükleri. Anlam: Bu sözcüklerle imleç boyutu ayarı aramada bulunur.
settings-desktop-cursor-size-keywords = mouse, cursor, size, accessibility

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.qt_backend ayarının satır başlığı. Anlam: Qt uygulama teması seçiminin başlığıdır.
settings-desktop-qt-backend-label = Qt apps

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.qt_backend ayarının alt yazısı. Anlam: Qt arka ucunun otomatik seçildiğini, kuruluysa KDE eklentisinin yoksa qt6ct'nin kullanıldığını anlatır.
settings-desktop-qt-backend-description = Automatic: KDE theme plugin if installed, otherwise qt6ct

# Nerede: Appearance sayfasındaki Uygulamalar bölümünde, desktop.qt_backend ayarının arama sözcükleri. Anlam: Bu sözcüklerle Qt arka ucu ayarı aramada bulunur.
settings-desktop-qt-backend-keywords = qt, kde, plasma, qt6ct, breeze, kvantum

# Nerede: Bar sayfasında, bölüm başlığı olmayan ana alandaki bar.preview ayarının satır başlığıdır. Anlam: Kullanıcıya barın canlı önizlemesini gösteren ayarı tanıtır.
settings-bar-preview-label = Bar preview

# Nerede: Bar sayfasında, bölüm başlığı olmayan ana alandaki bar.preview ayarının arama sözcükleridir. Anlam: Kullanıcı "önizleme" ya da "widget" diye aradığında bu ayarı bulmayı sağlar.
settings-bar-preview-keywords = preview, widget, select

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.selected ayarının satır başlığıdır. Anlam: Kullanıcıya, ayarlarını düzenleyeceği bar widget'ını seçme alanını gösterir.
settings-bar-selected-label = Selected widget

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.selected ayarının arama sözcükleridir. Anlam: "Widget" ya da "ayar" diye arayan kullanıcının bu ayara ulaşmasını sağlar.
settings-bar-selected-keywords = widget, setting

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.count ayarının satır başlığıdır. Anlam: Barda aynı anda kaç çalışma alanının görüneceğini belirtir.
settings-bar-widget-workspaces-count-label = Visible workspaces

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.count ayarının alt yazısıdır. Anlam: Sığmayan çalışma alanlarının oklarla sonraki sayfada göründüğünü anlatır.
settings-bar-widget-workspaces-count-description = The rest are on the next page with the arrows

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.count ayarının arama sözcükleridir. Anlam: "Çalışma alanı", "sayı" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-widget-workspaces-count-keywords = workspace, count, per page

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.style ayarının satır başlığıdır. Anlam: Çalışma alanı widget'ının görünümünü seçme ayarını adlandırır.
settings-bar-widget-workspaces-style-label = Appearance

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.style ayarının alt yazısıdır. Anlam: Görünümün uygulama ikonu, numara ya da nokta olabileceğini anlatır.
settings-bar-widget-workspaces-style-description = App icon, number, or dots

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.style ayarının arama sözcükleridir. Anlam: "İkon", "numara", "nokta" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-bar-widget-workspaces-style-keywords = workspace, icon, number, dots

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.scroll ayarının satır başlığıdır. Anlam: Fare tekerleğiyle çalışma alanı değiştirmeyi açıp kapatma ayarını tanıtır.
settings-bar-widget-workspaces-scroll-label = Switch with the wheel

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.scroll ayarının alt yazısıdır. Anlam: Widget'ın üstünde tekerlek çevrildiğinde çalışma alanının değiştiğini anlatır.
settings-bar-widget-workspaces-scroll-description = Scrolling over the widget changes the workspace

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.workspaces.scroll ayarının arama sözcükleridir. Anlam: "Tekerlek", "kaydırma" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-widget-workspaces-scroll-keywords = workspace, wheel, scroll

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.clock.format ayarının satır başlığıdır. Anlam: Saat widget'ının biçimini ayarlama alanını adlandırır.
settings-bar-widget-clock-format-label = Format

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.tray.icon_size ayarının satır başlığıdır. Anlam: Sistem tepsisi ikonlarının boyutunu ayarlama alanını adlandırır.
settings-bar-widget-tray-icon-size-label = Icon size

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.widget.tray.icon_size ayarının arama sözcükleridir. Anlam: "Tepsi" ya da "ikon" diye arayan kullanıcının bu ayara ulaşmasını sağlar.
settings-bar-widget-tray-icon-size-keywords = tray, system tray, icon

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.left_click ayarının satır başlığıdır. Anlam: Barın başındaki uçta sol tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-start-left-click-label = Left click

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.left_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "panel aç" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-bar-edge-actions-start-left-click-keywords = end, start, left end, edge, click, action, open panel

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.right_click ayarının satır başlığıdır. Anlam: Barın başındaki uçta sağ tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-start-right-click-label = Right click

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.right_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "eylem" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-start-right-click-keywords = end, start, left end, edge, click, action

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.scroll ayarının satır başlığıdır. Anlam: Barın başındaki uçta tekerlek çevrince çalışacak eylemi seçtirir.
settings-bar-edge-actions-start-scroll-label = Wheel

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.scroll ayarının alt yazısıdır. Anlam: Tekerlek çevrildiğinde widget'ın yanında bir ikon belirdiğini anlatır.
settings-bar-edge-actions-start-scroll-description = An icon appears next to the widget for the system action

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.start.scroll ayarının arama sözcükleridir. Anlam: "Kaydırma", "ses", "parlaklık" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-start-scroll-keywords = end, start, left end, edge, scroll, volume, brightness, workspace

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.left_click ayarının satır başlığıdır. Anlam: Barın sonundaki uçta sol tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-end-left-click-label = Left click

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.left_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "panel aç" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-bar-edge-actions-end-left-click-keywords = end, right end, edge, click, action, open panel

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.right_click ayarının satır başlığıdır. Anlam: Barın sonundaki uçta sağ tıkta çalışacak eylemi seçtirir.
settings-bar-edge-actions-end-right-click-label = Right click

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.right_click ayarının arama sözcükleridir. Anlam: "Uç", "tık", "eylem" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-end-right-click-keywords = end, right end, edge, click, action

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.scroll ayarının satır başlığıdır. Anlam: Barın sonundaki uçta tekerlek çevrince çalışacak eylemi seçtirir.
settings-bar-edge-actions-end-scroll-label = Wheel

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.scroll ayarının alt yazısıdır. Anlam: Tekerlek çevrildiğinde widget'ın yanında bir ikon belirdiğini anlatır.
settings-bar-edge-actions-end-scroll-description = An icon appears next to the widget for the system action

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.end.scroll ayarının arama sözcükleridir. Anlam: "Kaydırma", "ses", "parlaklık" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-end-scroll-keywords = end, right end, edge, scroll, volume, brightness, workspace

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.scroll_step ayarının satır başlığıdır. Anlam: Tekerleğin bir çentiğinde ne kadar değişeceğini ayarlar.
settings-bar-edge-actions-scroll-step-label = Wheel step

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.scroll_step ayarının alt yazısıdır. Anlam: Her çentikte yüzdenin ne kadar değiştiğini anlatır.
settings-bar-edge-actions-scroll-step-description = Percentage changed per notch

# Nerede: Bar sayfasındaki Seçili widget bölümündeki bar.edge_actions.scroll_step ayarının arama sözcükleridir. Anlam: "Adım", "tekerlek", "ses" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-bar-edge-actions-scroll-step-keywords = step, scroll step, wheel, volume, brightness

# Nerede: Bar sayfasındaki Konum ve boyut bölümündeki bar.size ayarının satır başlığıdır. Anlam: Barın kalınlığını ayarlama alanını adlandırır.
settings-bar-size-label = Thickness

# Nerede: Bar sayfasındaki Konum ve boyut bölümündeki bar.size ayarının alt yazısıdır. Anlam: Bar kenarındaki çerçevenin de aynı kalınlığı kullandığını anlatır.
settings-bar-size-description = The frame along the bar's edge also has this thickness

# Nerede: Bar sayfasındaki Konum ve boyut bölümündeki bar.padding ayarının satır başlığıdır. Anlam: Barın uçlarındaki boşluğu ayarlama alanını adlandırır.
settings-bar-padding-label = Gap at the ends

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.enabled ayarının alt yazısıdır. Anlam: Ekranı saran ince kenarın kapatıldığında yalnızca barın kaldığını anlatır.
settings-frame-enabled-description = A thin border around the screen; when off, only the bar remains

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.thickness ayarının satır başlığıdır. Anlam: Çerçevenin kalınlığını ayarlama alanını adlandırır.
settings-frame-thickness-label = Thickness

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.thickness ayarının alt yazısıdır. Anlam: Bu kalınlığın barın dışındaki kenarlara uygulandığını anlatır.
settings-frame-thickness-description = The edges outside the bar

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.edges ayarının alt yazısıdır. Anlam: Bar kenarının bar kalınlığını izlediğini, tek bir kenarın ayrı ayarlanıp sonra genele bağlanabildiğini anlatır.
settings-frame-edges-description = The bar edge follows the bar thickness; a single edge can be set separately and then linked back to the general setting

# Nerede: Bar sayfasındaki Çerçeve bölümündeki frame.inner_corners ayarının alt yazısıdır. Anlam: İç köşe kavisinin temadan geldiğini, buradan değiştirilemediğini anlatır.
settings-frame-inner-corners-description = The curvature comes from the theme

# Nerede: Wallpaper sayfasında, bölüm başlığı olmayan ana alandaki wallpaper ayarının alt yazısıdır. Anlam: Düz görsel ya da hareketli GIF/WebP/APNG seçilebildiğini anlatır.
settings-wallpaper-description = An image or animated GIF/WebP/APNG

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.fit ayarının alt yazısıdır. Anlam: Görselin ekrana nasıl sığacağını seçme alanını açıklar.
settings-wallpaper-fit-description = How the image fits the screen

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop ayarının satır başlığıdır. Anlam: Görselin kenarlarında kalan boşluğu ayarlama alanını adlandırır.
settings-wallpaper-backdrop-label = Gap at the edges

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop ayarının alt yazısıdır. Anlam: Bu alanın görsel ekranı tam kaplamadığında göründüğünü anlatır.
settings-wallpaper-backdrop-description = When the image does not cover the screen

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop ayarının arama sözcükleridir. Anlam: "Kenar", "siyah", "bulanık" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-wallpaper-backdrop-keywords = margin backdrop, letterbox, edge, stripe, black, blur

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_blur ayarının satır başlığıdır. Anlam: Kenarlardaki zemin kopyasının bulanıklığını ayarlama alanını adlandırır.
settings-wallpaper-backdrop-blur-label = Blur

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_blur ayarının alt yazısıdır. Anlam: Kenarlardaki kopyanın ne kadar yumuşak görüneceğini anlatır.
settings-wallpaper-backdrop-blur-description = Softness of the copy at the edges

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_blur ayarının arama sözcükleridir. Anlam: "Bulanık", "yarıçap", "kenar" gibi sözcüklerle aramada bu ayarı bulmayı sağlar.
settings-wallpaper-backdrop-blur-keywords = backdrop blur, blur, radius, edge

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_color ayarının satır başlığıdır. Anlam: Zemin rengini seçme alanını adlandırır.
settings-wallpaper-backdrop-color-label = Backdrop color

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_color ayarının alt yazısıdır. Anlam: Bu rengin temayla birlikte değiştiğini anlatır.
settings-wallpaper-backdrop-color-description = Changes with the theme

# Nerede: Wallpaper sayfasındaki Yerleşim bölümündeki wallpaper.backdrop_color ayarının arama sözcükleridir. Anlam: "Renk", "zemin", "yüzey" gibi sözcüklerle aramada bu ayara ulaşmayı sağlar.
settings-wallpaper-backdrop-color-keywords = color, edge, backdrop, surface

# Nerede: Wallpaper sayfasındaki Renkler bölümündeki palette.source ayarının alt yazısıdır. Anlam: Şema ve kontrast ayarlarının Görünüm sayfasında bulunduğunu anlatır.
settings-palette-source-description = Scheme and contrast are in Appearance

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.enabled ayarının satır başlığıdır. Anlam: Ekranın sağ kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-right-enabled-label = Use right panel

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.enabled ayarının alt yazısıdır. Anlam: Sağ panelin ekranın sağ kenarından çekmece gibi açıldığını anlatır.
settings-panels-right-enabled-description = Drawer that opens from the right edge of the screen

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-enabled-keywords = right panel, drawer, edge panel

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.size ayarının satır başlığıdır. Anlam: Sağ panelin genişliğini belirler.
settings-panels-right-size-label = Width

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-size-keywords = right panel, width, size, depth

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.widgets ayarının satır başlığıdır. Anlam: Sağ panelde hangi içeriklerin görüneceğini seçer.
settings-panels-right-widgets-label = Contents

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-widgets-keywords = right panel, card, widget, quick settings, media, notification, order

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.from_bar ayarının satır başlığıdır. Anlam: Sağ panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-right-from-bar-label = Open from bar

# Nerede: Panels sayfasındaki Sağ panel bölümünde, panels.right.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-right-from-bar-keywords = right panel, edge action, click, bar

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.enabled ayarının satır başlığıdır. Anlam: Ekranın sol kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-left-enabled-label = Use left panel

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.enabled ayarının alt yazısıdır. Anlam: Sol panelin ekranın sol kenarından çekmece gibi açıldığını anlatır.
settings-panels-left-enabled-description = Drawer that opens from the left edge of the screen

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-enabled-keywords = left panel, drawer, edge panel

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.size ayarının satır başlığıdır. Anlam: Sol panelin genişliğini belirler.
settings-panels-left-size-label = Width

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-size-keywords = left panel, width, size, depth

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.widgets ayarının satır başlığıdır. Anlam: Sol panelde hangi içeriklerin görüneceğini seçer.
settings-panels-left-widgets-label = Contents

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-widgets-keywords = left panel, card, widget, quick settings, media, notification, order

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.from_bar ayarının satır başlığıdır. Anlam: Sol panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-left-from-bar-label = Open from bar

# Nerede: Panels sayfasındaki Sol panel bölümünde, panels.left.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-left-from-bar-keywords = left panel, edge action, click, bar

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.enabled ayarının satır başlığıdır. Anlam: Ekranın üst kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-top-enabled-label = Use top panel

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.enabled ayarının alt yazısıdır. Anlam: Üst panelin ekranın üst kenarından çekmece gibi açıldığını anlatır.
settings-panels-top-enabled-description = Drawer that opens from the top edge of the screen

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-enabled-keywords = top panel, drawer, edge panel

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.size ayarının satır başlığıdır. Anlam: Üst panelin yüksekliğini belirler.
settings-panels-top-size-label = Height

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-size-keywords = top panel, height, size, depth

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.widgets ayarının satır başlığıdır. Anlam: Üst panelde hangi içeriklerin görüneceğini seçer.
settings-panels-top-widgets-label = Contents

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-widgets-keywords = top panel, card, widget, quick settings, media, notification, order

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.from_bar ayarının satır başlığıdır. Anlam: Üst panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-top-from-bar-label = Open from bar

# Nerede: Panels sayfasındaki Üst panel bölümünde, panels.top.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-top-from-bar-keywords = top panel, edge action, click, bar

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.enabled ayarının satır başlığıdır. Anlam: Ekranın alt kenarından açılan panelin kullanılıp kullanılmayacağını belirler.
settings-panels-bottom-enabled-label = Use bottom panel

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.enabled ayarının alt yazısıdır. Anlam: Alt panelin ekranın alt kenarından çekmece gibi açıldığını anlatır.
settings-panels-bottom-enabled-description = Drawer that opens from the bottom edge of the screen

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.enabled ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-enabled-keywords = bottom panel, drawer, edge panel

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.size ayarının satır başlığıdır. Anlam: Alt panelin yüksekliğini belirler.
settings-panels-bottom-size-label = Height

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.size ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-size-keywords = bottom panel, height, size, depth

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.widgets ayarının satır başlığıdır. Anlam: Alt panelde hangi içeriklerin görüneceğini seçer.
settings-panels-bottom-widgets-label = Contents

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.widgets ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-widgets-keywords = bottom panel, card, widget, quick settings, media, notification, order

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.from_bar ayarının satır başlığıdır. Anlam: Alt panelin bar üzerinden açılıp açılmayacağını belirler.
settings-panels-bottom-from-bar-label = Open from bar

# Nerede: Panels sayfasındaki Alt panel bölümünde, panels.bottom.from_bar ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-panels-bottom-from-bar-keywords = bottom panel, edge action, click, bar

# Nerede: Fonts sayfasında, hiçbir bölüme bağlı olmayan typography.preview ayarının satır başlığıdır. Anlam: Yazı tipi ayarlarının etkisini gösteren önizlemeyi açar.
settings-typography-preview-label = Preview

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.size ayarının alt yazısıdır. Anlam: Tüm yazıların varsayılan taban boyutunu belirler.
settings-typography-size-description = Base size for all text

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.family ayarının satır başlığıdır. Anlam: Düğme ve menü gibi arayüz öğelerinin yazı tipini seçer.
settings-typography-ui-family-label = Interface

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin düğmelerde, menülerde ve ayar satırlarında kullanıldığını anlatır.
settings-typography-ui-family-description = Buttons, menus, settings rows

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-ui-family-keywords = font, family, weight, inter, sans

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.family ayarının satır başlığıdır. Anlam: Saat ve büyük başlıkların yazı tipini seçer.
settings-typography-display-family-label = Heading

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin saatte ve büyük başlıklarda kullanıldığını anlatır.
settings-typography-display-family-description = Clock, large headings

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-display-family-keywords = font, family, weight, clock, display

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.family ayarının satır başlığıdır. Anlam: Uzun metinlerin yazı tipini seçer.
settings-typography-document-family-label = Document

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin uzun metinlerde, bildirim gövdesinde ve takvim notlarında kullanıldığını anlatır.
settings-typography-document-family-description = Long text: notification body, calendar notes

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-document-family-keywords = font, family, weight, serif, reading

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.family ayarının satır başlığıdır. Anlam: Kod ve sayı tablolarının yazı tipini seçer.
settings-typography-mono-family-label = Monospace

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.family ayarının alt yazısıdır. Anlam: Bu yazı tipinin kodda, sayı tablolarında ve Nerd Font ikonlarında kullanıldığını anlatır.
settings-typography-mono-family-description = Code and number tables; also the font for Nerd Font icons

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-mono-family-keywords = font, family, weight, monospace, code, nerd font

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.scale ayarının satır başlığıdır. Anlam: Arayüz yazılarının boyut oranını belirler.
settings-typography-ui-scale-label = Interface scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.ui.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-ui-scale-keywords = font, size, scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.scale ayarının satır başlığıdır. Anlam: Başlık yazılarının boyut oranını belirler.
settings-typography-display-scale-label = Heading scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.display.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-display-scale-keywords = font, size, scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.scale ayarının satır başlığıdır. Anlam: Belge metinlerinin boyut oranını belirler.
settings-typography-document-scale-label = Document scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.document.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-document-scale-keywords = font, size, scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.scale ayarının satır başlığıdır. Anlam: Eş aralıklı yazıların boyut oranını belirler.
settings-typography-mono-scale-label = Monospace scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.mono.scale ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-mono-scale-keywords = font, size, scale

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.family ayarının satır başlığıdır. Anlam: İkonların çizildiği yazı tipini seçer.
settings-typography-icon-family-label = Icon font

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.family ayarının alt yazısıdır. Anlam: Boş bırakılırsa ikon setinin kendi yazı tipinin kullanılacağını anlatır.
settings-typography-icon-family-description = If empty, the icon set's own font

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.family ayarının arama sözcükleridir. Anlam: Bu ayarı ayarlarda ararken yazılabilecek sözcükleri listeler.
settings-typography-icon-family-keywords = font, icon, symbol, weight, material symbols

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.scale ayarının satır başlığıdır. Anlam: İkonların boyut oranını belirler.
settings-typography-icon-scale-label = Icon scale

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_inner ayarının satır başlığı. Anlam: Yan yana iki pencere arasında bırakılacak boşluğu belirtir.
settings-windows-gap-inner-label = Gap between windows

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_inner ayarının alt yazısı. Anlam: Boşluğun yan yana iki pencere arasına uygulandığını açıklar.
settings-windows-gap-inner-description = Between two side-by-side windows

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_outer ayarının satır başlığı. Anlam: Pencereler ile ekran kenarı arasındaki boşluğu belirtir.
settings-windows-gap-outer-label = Gap to the screen edge

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.gap_outer ayarının alt yazısı. Anlam: Boşluğun pencerelerle ekran kenarı ve bar arasında uygulandığını açıklar.
settings-windows-gap-outer-description = Between windows and the screen edge and the bar

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.border ayarının satır başlığı. Anlam: Pencere kenarlığı ayarının başlığıdır.
settings-windows-border-label = Border

# Nerede: Windows sayfasında, Yerleşim bölümündeki windows.border ayarının alt yazısı. Anlam: Kenarlığın etkin pencerede vurgu rengiyle çizildiğini, 0 değerinin kenarlığı kapattığını anlatır.
settings-windows-border-description = In accent color on the active window; 0 disables it

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur ayarının satır başlığı. Anlam: Saydam pencerelerin arkasının bulanık gösterilmesini sağlayan ayarın başlığıdır.
settings-windows-blur-label = Blur behind transparent windows

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur ayarının alt yazısı. Anlam: Bu özelliğin buzlu cam görünümü verdiğini açıklar.
settings-windows-blur-description = Frosted glass look

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur_size ayarının satır başlığı. Anlam: Bulanıklığın yoğunluğunu belirleyen ayarın başlığıdır.
settings-windows-blur-size-label = Intensity

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.blur_size ayarının alt yazısı. Anlam: Değer büyüdükçe bulanıklığın daha yumuşak olacağını anlatır.
settings-windows-blur-size-description = A larger value is softer

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.corners ayarının satır başlığı. Anlam: Pencere köşelerini, gölgesini ve hareketlerini içeren ayarın başlığıdır.
settings-windows-corners-label = Corners, shadow and motion

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.corners ayarının alt yazısı. Anlam: Bu ayarın bar ve panellerle ortak olduğunu açıklar.
settings-windows-corners-description = Shared with the bar and panels

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.shadow ayarının satır başlığı. Anlam: Pencere gölgesini yöneten ayarın başlığıdır.
settings-windows-shadow-label = Window shadow

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.shadow ayarının alt yazısı. Anlam: Pencere gölgesinin bar ve panellerin gölgesiyle aynı olduğunu anlatır.
settings-windows-shadow-description = Same as for the bar and panels

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.motion ayarının satır başlığı. Anlam: Pencere hareket animasyonlarını yöneten ayarın başlığıdır.
settings-windows-motion-label = Window motion

# Nerede: Windows sayfasında, Görünüm bölümündeki windows.motion ayarının alt yazısı. Anlam: Açılıp kapanmanın ve çalışma alanı geçişlerinin Hareket sayfasındaki ayarı izlediğini anlatır.
settings-windows-motion-description = Opening, closing and workspace changes follow Motion

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_look ayarının alt yazısı. Anlam: Köşe, gölge ve bulanıklık ayarlarının Nuxut'tan alındığını açıklar.
settings-compositor-sync-look-description = Corners, shadow and blur come from Nuxut

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_windows ayarının satır başlığı. Anlam: Pencere boşluklarının ve kenarlığının Nuxut tarafından belirlenmesini isteyen seçeneğin başlığıdır.
settings-compositor-sync-windows-label = Let Nuxut set gaps and border

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_windows ayarının alt yazısı. Anlam: Seçenek kapalıyken masaüstünün kendi boşluk ve kenarlık ayarının geçerli olduğunu anlatır.
settings-compositor-sync-windows-description = When off, the desktop's own setting applies

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_motion ayarının alt yazısı. Anlam: Seçenek kapalıyken masaüstünün kendi animasyonlarının geçerli olduğunu anlatır.
settings-compositor-sync-motion-description = When off, the desktop's own animations apply

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_input ayarının satır başlığı. Anlam: Klavye ve fare ayarlarının Nuxut tarafından belirlenmesini isteyen seçeneğin başlığıdır.
settings-compositor-sync-input-label = Let Nuxut set keyboard and mouse

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_input ayarının alt yazısı. Anlam: Seçenek kapalıyken masaüstünün kendi klavye ve fare ayarının geçerli olduğunu anlatır.
settings-compositor-sync-input-description = When off, the desktop's own setting applies

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.sync_input ayarının arama sözcükleri. Anlam: Bu seçeneği ararken yazılabilecek sözcükleri toplar.
settings-compositor-sync-input-keywords = keyboard, mouse, touchpad, input, sync

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.ensure_source ayarının satır başlığı. Anlam: Nuxut bağlantı satırını masaüstünün ayarlarına ekleyen seçeneğin başlığıdır.
settings-compositor-ensure-source-label = Add Nuxut to the desktop settings

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.ensure_source ayarının alt yazısı. Anlam: Ayar dosyasına tek satır eklendiğini ve öncesinde yedek alındığını anlatır.
settings-compositor-ensure-source-description = One line is added to the settings file, a backup is taken first

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.source_note ayarının satır başlığı. Anlam: Masaüstü ayar dosyasına eklenen Nuxut bağlantı satırını gösteren başlıktır.
settings-compositor-source-note-label = Source line

# Nerede: Windows sayfasında, Gelişmiş bölümündeki compositor.source_note ayarının arama sözcükleri. Anlam: Bağlantı satırı ayarını ararken yazılabilecek sözcükleri toplar.
settings-compositor-source-note-keywords = source, connection, removed

# Nerede: Motion sayfasında, bölüm olmadan, motion.preset ayarının satır başlığı. Anlam: Hareketin genel karakterini seçen hazır ayarın başlığıdır.
settings-motion-preset-label = Character

# Nerede: Motion sayfasında, bölüm olmadan, motion.preset ayarının alt yazısı. Anlam: Bu ayarın tüm hareketlerin karakterini belirlediğini anlatır.
settings-motion-preset-description = Character of all motion

# Nerede: Motion sayfasında, Genel bölümündeki motion.enabled ayarının alt yazısı. Anlam: Kapalıyken animasyonların kapandığını ve her şeyin anında değiştiğini anlatır.
settings-motion-enabled-description = Animations turn off; everything changes instantly

# Nerede: Motion sayfasında, Genel bölümündeki motion.speed ayarının alt yazısı. Anlam: Değerin tüm animasyonların çarpanı olduğunu, 2× değerinin iki kat hızlı olacağını anlatır.
settings-motion-speed-description = A multiplier for all animations; 2× is twice as fast

# Nerede: Motion sayfasında, Genel bölümündeki motion.style ayarının satır başlığı. Anlam: Hareket türünü seçen satırın başlığıdır.
settings-motion-style-label = Motion type

# Nerede: Motion sayfasında, Genel bölümündeki motion.style ayarının alt yazısı. Anlam: Yay hareketinin doğal ve kesilebilir olduğunu, eğri hareketin sabit süreli olduğunu açıklar.
settings-motion-style-description = Spring is natural and interruptible; the curve has a fixed duration

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial ayarının satır başlığı. Anlam: Panellerin ve pencerelerin hareket ayarlarını içeren grubun başlığıdır.
settings-motion-spatial-label = Panels and windows

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial ayarının alt yazısı. Anlam: Bu grubun açılma, boyut ve konum değişimi animasyonlarını kapsadığını açıklar.
settings-motion-spatial-description = Opening, resizing and moving

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial ayarının arama sözcükleri. Anlam: Bu grubu ararken yazılabilecek sözcükleri toplar.
settings-motion-spatial-keywords = spatial, animation, preview

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.style ayarının satır başlığı. Anlam: Paneller ve pencereler için hareket türünü seçen satırın başlığıdır.
settings-motion-role-style-label = Motion type

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.style ayarının alt yazısı. Anlam: Genel seçildiğinde üstteki hareket türünü izlediğini anlatır.
settings-motion-role-style-description = General: the type above

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.response ayarının satır başlığı. Anlam: Hareketin ne kadar sürede tamamlandığını belirleyen ayarın başlığıdır.
settings-motion-role-response-label = Duration

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.response ayarının alt yazısı. Anlam: Kısa sürenin daha hızlı hareket anlamına geldiğini anlatır.
settings-motion-role-response-description = Short = fast

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.damping ayarının satır başlığı. Anlam: Yay hareketindeki yaylanmayı ayarlayan satırın başlığıdır.
settings-motion-role-damping-label = Springiness

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.damping ayarının alt yazısı. Anlam: Değer düşükken hareketin hedefi aşıp geri oturduğunu anlatır.
settings-motion-role-damping-description = A low value overshoots the target and settles back

# Nerede: Motion sayfasında, Paneller ve pencereler bölümündeki motion.spatial.easing ayarının alt yazısı. Anlam: Hareketin hızlanıp yavaşlama biçimini seçen ayarın alt yazısıdır.
settings-motion-role-easing-description = How it speeds up and slows down

# Nerede: Motion sayfasında, Menüler ve küçük öğeler bölümündeki motion.spatial_fast ayarının satır başlığı. Anlam: Menülerin ve küçük öğelerin hareket ayarlarını içeren grubun başlığıdır.
settings-motion-spatial-fast-label = Menus and small items

# Nerede: Motion sayfasında, Menüler ve küçük öğeler bölümündeki motion.spatial_fast ayarının alt yazısı. Anlam: Bu grubun menüleri, göstergeleri ve küçük düğmeleri kapsadığını açıklar.
settings-motion-spatial-fast-description = Menus, indicators, small buttons

# Nerede: Motion sayfasında, Menüler ve küçük öğeler bölümündeki motion.spatial_fast ayarının arama sözcükleri. Anlam: Bu grubu ararken yazılabilecek sözcükleri toplar.
settings-motion-spatial-fast-keywords = spatial_fast, animation, preview

# Nerede: Motion sayfasında, Solma ve renk bölümündeki motion.effects ayarının alt yazısı. Anlam: Saydamlık ve renk geçişlerinin animasyonlu olduğunu ve taşma yapmadığını anlatır.
settings-motion-effects-description = Opacity and color transitions; no overshoot

# Nerede: Motion sayfasında, Solma ve renk bölümündeki motion.effects ayarının arama sözcükleri. Anlam: Bu grubu ararken yazılabilecek sözcükleri toplar.
settings-motion-effects-keywords = effects, opacity, animation, preview

# Nerede: Keybinds sayfasında Genel bölümünde, compositor.sync_keybinds ayarının alt yazısı. Anlam: Aynı tuşa bağlı başka bir masaüstü kısayolu varsa onun geçerli kaldığını söyler.
settings-compositor-sync-keybinds-description = If a desktop shortcut is already bound to the same key, that one stays active

# Nerede: Keybinds sayfasında Ses bölümünde, keybinds.volume_up ayarının arama sözcükleri. Anlam: Ses açma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-volume-up-keywords = shortcut, key, volume, xf86

# Nerede: Keybinds sayfasında Ses bölümünde, keybinds.volume_down ayarının arama sözcükleri. Anlam: Ses kısma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-volume-down-keywords = shortcut, key, volume, xf86

# Nerede: Keybinds sayfasında Parlaklık bölümünde, keybinds.brightness_up ayarının arama sözcükleri. Anlam: Ekran parlaklığını artırma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-brightness-up-keywords = shortcut, key, screen, brightness, xf86

# Nerede: Keybinds sayfasında Parlaklık bölümünde, keybinds.brightness_down ayarının arama sözcükleri. Anlam: Ekran parlaklığını azaltma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-brightness-down-keywords = shortcut, key, screen, brightness, xf86

# Nerede: Keybinds sayfasında Medya bölümünde, keybinds.media_play ayarının arama sözcükleri. Anlam: Müziği başlatma ya da duraklatma kısayolunu ararken kullanılan kelimeler.
settings-keybinds-media-play-keywords = shortcut, key, music, playerctl, xf86

# Nerede: Keybinds sayfasında Medya bölümünde, keybinds.media_next ayarının arama sözcükleri. Anlam: Sıradaki parçaya geçme kısayolunu ararken kullanılan kelimeler.
settings-keybinds-media-next-keywords = shortcut, key, music, playerctl, xf86

# Nerede: Keybinds sayfasında Medya bölümünde, keybinds.media_prev ayarının arama sözcükleri. Anlam: Önceki parçaya dönme kısayolunu ararken kullanılan kelimeler.
settings-keybinds-media-prev-keywords = shortcut, key, music, playerctl, xf86

# Nerede: Notifications sayfasında Genel bölümünde, notifications.enabled ayarının satır başlığı. Anlam: Bildirimleri Nuxut'un kendisinin göstereceğini söyler.
settings-notifications-enabled-label = Let Nuxut show notifications

# Nerede: Notifications sayfasında Genel bölümünde, notifications.enabled ayarının alt yazısı. Anlam: Ayar kapalıyken bildirimleri başka bir uygulamanın gösterebileceğini söyler.
settings-notifications-enabled-description = Another app can show them when this is off

# Nerede: Notifications sayfasında Genel bölümünde, notifications.enabled ayarının arama sözcükleri. Anlam: Bildirim ayarını ararken kullanılan kelimeler.
settings-notifications-enabled-keywords = notifications, notification server

# Nerede: Notifications sayfasında Genel bölümünde, notifications.dnd ayarının alt yazısı. Anlam: Rahatsız etmeyin açıkken açılır bildirim çıkmadığını ve bildirimlerin merkezde birikeceğini söyler.
settings-notifications-dnd-description = No popups; they all pile up in the center

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.popup_timeout_ms ayarının satır başlığı. Anlam: Açılır bildirimin ekranda kalacağını belirleyen sürenin ayarı olduğunu söyler.
settings-notifications-popup-timeout-ms-label = Display duration

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.popup_timeout_ms ayarının alt yazısı. Anlam: Açılır bildirimin ekranda kalma süresini açıklar.
settings-notifications-popup-timeout-ms-description = How long a popup stays on screen

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.popup_timeout_ms ayarının arama sözcükleri. Anlam: Açılır bildirim süresini ararken kullanılan kelimeler.
settings-notifications-popup-timeout-ms-keywords = popup, duration, timeout, popup notifications

# Nerede: Notifications sayfasında Davranış bölümünde, notifications.max_popups ayarının alt yazısı. Anlam: Aynı anda en kaç açılır bildirimin üst üste görünebileceğini açıklar.
settings-notifications-max-popups-description = Number of popups shown at once

# Nerede: Region sayfasında bölümsüz duran locale.preview ayarının satır başlığı. Anlam: Dil ayarlarının örnek biçim görünümünü gösterdiğini söyler.
settings-locale-preview-label = Format preview

# Nerede: Region sayfasında bölümsüz duran locale.preview ayarının arama sözcükleri. Anlam: Biçim önizlemesini ararken kullanılan kelimeler.
settings-locale-preview-keywords = preview, example, date, time, number, percent

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.language ayarının satır başlığı. Anlam: Arayüz dilinin seçildiği ayar olduğunu söyler.
settings-locale-language-label = Language

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.language ayarının alt yazısı. Anlam: Menü ve metinlerin seçilen dilde gösterileceğini söyler.
settings-locale-language-description = Menus and text are shown in this language

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.language ayarının arama sözcükleri. Anlam: Dil ayarını ararken kullanılan kelimeler.
settings-locale-language-keywords = language, translation, turkish, english, locale

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.formats ayarının satır başlığı. Anlam: Tarih, saat ve sayı biçimlerinin seçildiği bölüm olduğunu söyler.
settings-locale-formats-label = Formats

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.formats ayarının alt yazısı. Anlam: Tarih, saat ve sayıların nasıl yazıldığını açıklar.
settings-locale-formats-description = How dates, times and numbers are written

# Nerede: Region sayfasında Dil ve biçimler bölümünde, locale.formats ayarının arama sözcükleri. Anlam: Biçim ayarlarını ararken kullanılan kelimeler.
settings-locale-formats-keywords = date, time, number, decimal, region, format, locale

# Nerede: Sound sayfasında Ses bölümünde, audio.max_volume ayarının alt yazısı. Anlam: Ses kısayollarının ve kaydırıcıların çıkabileceği en yüksek seviyeyi açıklar.
settings-audio-max-volume-description = Upper limit for shortcuts and sliders

# Nerede: Sound sayfasında Ses bölümünde, osd.timeout_ms ayarının satır başlığı. Anlam: Ses ve parlaklık değişimini ekranda gösteren göstergenin süresinin ayarı olduğunu söyler.
settings-osd-timeout-ms-label = Volume and brightness OSD

# Nerede: Sound sayfasında Ses bölümünde, osd.timeout_ms ayarının alt yazısı. Anlam: Göstergenin ekranda kalma süresini açıklar.
settings-osd-timeout-ms-description = How long it stays on screen

# Nerede: Sound sayfasında Ses bölümünde, osd.timeout_ms ayarının arama sözcükleri. Anlam: Ekran göstergesini ararken kullanılan kelimeler.
settings-osd-timeout-ms-keywords = osd, screen indicator, duration, display time

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys.link ayarının alt yazısı. Anlam: Klavyedeki ses, parlaklık ve oynatma tuşlarının bu ayarla bağlandığını açıklar.
settings-media-keys-link-description = Volume, brightness and playback keys on your keyboard

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys ayarının satır başlığı. Anlam: Masaüstü ayar dosyasına eklenebilecek bir örnek gösterdiğini söyler.
settings-media-keys-label = Example for the desktop config file

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys ayarının alt yazısı. Anlam: Nuxut'un kendi kısayollarının kapalı olduğunu, tuşların elle bağlanabileceğini söyler.
settings-media-keys-description = Nuxut shortcuts are off; keys can be bound manually

# Nerede: Sound sayfasında Medya tuşları bölümünde, media_keys ayarının arama sözcükleri. Anlam: Medya tuşlarını ve Hyprland bağlama örneğini ararken kullanılan kelimeler.
settings-media-keys-keywords = hyprland bind example, xf86, shortcut, bind, playerctl, brightness, microphone

# Nerede: Duvar kağıdı sayfasında, görselin kaplamadığı alanın zeminini seçen listede blur seçeneği. Anlam: Boşluğun bulanıklaştırılmış görselle doldurulduğunu söyler.
settings-choice-wallpaper-backdrop-blur = Blurred image

# Nerede: Duvar kağıdı sayfasında, görselin kaplamadığı alanın zeminini seçen listede color seçeneği. Anlam: Boşluğun düz bir renkle doldurulduğunu söyler.
settings-choice-wallpaper-backdrop-color = Solid color

# Nerede: Duvar kağıdı sayfasında, düz renk zeminin hangi tema rengi olacağını seçen listede surface seçeneği. Anlam: Zeminin normal yüzey rengi olduğunu söyler.
settings-choice-wallpaper-backdrop-color-surface = Surface

# Nerede: Duvar kağıdı sayfasında, düz renk zeminin hangi tema rengi olacağını seçen listede surface_container seçeneği. Anlam: Zeminin kart rengi olduğunu söyler.
settings-choice-wallpaper-backdrop-color-surface-container = Card

# Nerede: Duvar kağıdı sayfasında, düz renk zeminin hangi tema rengi olacağını seçen listede surface_bright seçeneği. Anlam: Zeminin parlak yüzey rengi olduğunu söyler.
settings-choice-wallpaper-backdrop-color-surface-bright = Bright surface

# Nerede: Görünüm sayfasında Uygulamalar bölümünde, Qt uygulamalarının tema altyapısını seçen listede auto seçeneği. Anlam: Altyapının otomatik seçildiğini söyler.
settings-choice-qt-backend-auto = Automatic

# Nerede: Görünüm sayfasında Uygulamalar bölümünde, Qt uygulamalarının tema altyapısını seçen listede kde seçeneği. Anlam: Qt uygulamalarının KDE temasını kullanacağını söyler.
settings-choice-qt-backend-kde = KDE

# Nerede: Görünüm sayfasında Uygulamalar bölümünde, Qt uygulamalarının tema altyapısını seçen listede qt6ct seçeneği. Anlam: Qt uygulamalarının qt6ct ayarlarını kullanacağını söyler.
settings-choice-qt-backend-qt6ct = qt6ct

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede none seçeneği. Anlam: Tıklamada hiçbir şey yapılmadığını söyler.
settings-choice-edge-click-none = Nothing

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:left seçeneği. Anlam: Tıklamada sol panelin açıldığını söyler.
settings-choice-edge-click-panel-left = Open left panel

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:right seçeneği. Anlam: Tıklamada sağ panelin açıldığını söyler.
settings-choice-edge-click-panel-right = Open right panel

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:top seçeneği. Anlam: Tıklamada üst panelin açıldığını söyler.
settings-choice-edge-click-panel-top = Open top panel

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:bottom seçeneği. Anlam: Tıklamada alt panelin açıldığını söyler.
settings-choice-edge-click-panel-bottom = Open bottom panel

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:clock seçeneği. Anlam: Tıklamada takvim panelinin açıldığını söyler.
settings-choice-edge-click-panel-clock = Open calendar

# Nerede: Bar sayfasında, bar ucundaki widget'a tıklanınca yapılacak eylemi seçen listede panel:launcher seçeneği. Anlam: Tıklamada uygulama başlatıcının açıldığını söyler.
settings-choice-edge-click-panel-launcher = Open app launcher

# Nerede: Bar sayfasında, bar ucundaki widget üzerinde tekerlek çevirince yapılacak eylemi seçen listede widget seçeneği. Anlam: Kaydırmanın doğrudan widget'ın kendi işlevini kullandığını söyler.
settings-choice-edge-scroll-widget = Widget's own action

# Nerede: Bar sayfasında, bar ucundaki widget üzerinde tekerlek çevirince yapılacak eylemi seçen listede none seçeneği. Anlam: Tekerlek çevirmede hiçbir şey yapılmadığını söyler.
settings-choice-edge-scroll-none = Nothing

# Nerede: Bar sayfasında, bar ucundaki widget üzerinde tekerlek çevirince yapılacak eylemi seçen listede workspace seçeneği. Anlam: Tekerleğin çalışma alanları arasında geçiş yaptığını söyler.
settings-choice-edge-scroll-workspace = Workspace

# Nerede: Hareket sayfasında, hareket rolünün türünü seçen listede spring seçeneği (Genel, üstteki genel türü seçer). Anlam: Hareketin yay gibi esneyerek bittiğini söyler.
settings-choice-role-style-spring = Spring

# Nerede: Hareket sayfasında, hareket rolünün türünü seçen listede curve seçeneği (Genel, üstteki genel türü seçer). Anlam: Hareketin bir eğriye göre yapıldığını söyler.
settings-choice-role-style-curve = Curve

# Nerede: Kenar çubuğundaki sayfa adı. Anlam: Paneller ayarlarının bulunduğu sayfayı açar.
settings-page-panels = Panels

# Nerede: Kenar çubuğundaki sayfa adı. Anlam: Bölge ve dil ayarlarının bulunduğu sayfayı açar.
settings-page-region = Region and language

# Nerede: Hareket sayfasında rol kartının başlığı ve bölüm adı. Anlam: Panellerin ve pencerelerin hareketini ayarlayan bölüm olduğunu söyler.
settings-motion-role-spatial = Panels and windows

# Nerede: Hareket sayfasında rol kartının başlığı ve bölüm adı. Anlam: Menülerin ve küçük öğelerin hareketini ayarlayan bölüm olduğunu söyler.
settings-motion-role-spatial-fast = Menus and small items

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Alt panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-bottom = Bottom panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Sağ panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-right = Right panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Sol panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-left = Left panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Üst panelin ayarlarını içeren bölüm olduğunu söyler.
settings-section-panel-top = Top panel

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Davranış ayarlarını içeren bölüm olduğunu söyler.
settings-section-behavior = Behavior

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Dil ve biçim ayarlarını içeren bölüm olduğunu söyler.
settings-section-language-formats = Language and formats

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Görünüm ayarlarını içeren bölüm olduğunu söyler.
settings-section-appearance = Appearance

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Konum ve boyut ayarlarını içeren bölüm olduğunu söyler.
settings-section-position-size = Position and size

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Köşe ve şekil ayarlarını içeren bölüm olduğunu söyler.
settings-section-corners-shape = Corners and shape

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Monitör başına yapılan ayarları içeren bölüm olduğunu söyler.
settings-section-per-monitor = Per monitor

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Seçili widget'ın ayarlarını içeren bölüm olduğunu söyler.
settings-section-selected-widget = Selected widget

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Yazı tipi seçimini içeren bölüm olduğunu söyler.
settings-section-font-families = Fonts

# Nerede: Ayarlar sayfasında bir bölüm (kart) başlığı. Anlam: Yerleşim ayarlarını içeren bölüm olduğunu söyler.
settings-section-layout = Layout

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Sistem saati ağ saatiyle eşitlendi.
settings-datetime-ntp-synced = Time synced

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Otomatik saat açık ama henüz eşitlenmedi.
settings-datetime-ntp-unsynced = Time not synced yet

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Otomatik saat kapalı, saat elle ayarlanıyor.
settings-datetime-ntp-off = Time is set manually

# Nerede: Tarih ve saat sayfasında, Otomatik saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Sistemde ağ saati hizmeti (ör. systemd-timesyncd) yok; anahtar kapalı.
settings-datetime-ntp-unavailable = No network time service on this system

# Nerede: Tarih ve saat sayfasında, tarih ve saat satırlarının alt yazısı (src/pages/datetime.rs). Anlam: Değişiklik için yönetici yetkisi isteniyor, kullanıcının yanıtı bekleniyor.
settings-datetime-waiting = Waiting for authorization…

# Nerede: Tarih ve saat sayfasında, tarih ve saat satırlarının alt yazısı (src/pages/datetime.rs). Anlam: Sistem değişikliği kabul etmedi (genel hata).
settings-datetime-failed = Could not be changed

# Nerede: Tarih ve saat sayfasında, tarih ve saat satırlarının alt yazısı (src/pages/datetime.rs). Anlam: Tarih ve saat hizmeti bulunamadığı için değişiklik yapılamadı.
settings-datetime-missing-short = Time service not found

# Nerede: Tarih ve saat sayfasının üstündeki not (src/pages/datetime.rs). Anlam: systemd-timedated hizmeti yok; ayarlar buradan yapılamıyor.
settings-datetime-missing = systemd-timedated not found; date and time cannot be set here.

# Nerede: Tarih ve saat sayfasının üstündeki not (src/pages/datetime.rs). Anlam: Sistemden tarih ve saat bilgisi okunamadı.
settings-datetime-read-failed = Could not read date and time information.

# Nerede: Tarih ve saat sayfasının üstündeki not (src/pages/datetime.rs). Anlam: Donanım saati UTC yerine yerel saatte tutuluyor; yaz saati geçişlerinde kayma uyarısı.
settings-datetime-local-rtc = Hardware clock is kept in local time (LocalRTC). Common on systems installed alongside Windows; the clock may shift at daylight saving time changes.

# Nerede: Tarih ve saat sayfasında, saat dilimi satırındaki düğme (src/pages/datetime.rs). Anlam: Saat dilimi seçicisini açar.
settings-datetime-zone-change = Change

# Nerede: Tarih ve saat sayfasında, seçici açıkken aynı düğme (src/pages/datetime.rs). Anlam: Seçimi yapmadan seçiciyi kapatır.
settings-datetime-zone-cancel = Cancel

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisindeki arama alanının yer tutucusu (src/pages/datetime.rs). Anlam: Şehir ya da bölge adıyla arama yapılır.
settings-datetime-zone-search = Search city or region

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisinin durum metni (src/pages/datetime.rs). Anlam: Saat dilimi listesi sistemden okunuyor.
settings-datetime-zone-loading = Loading time zones…

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisinin durum metni (src/pages/datetime.rs). Anlam: Aramaya uyan saat dilimi yok.
settings-datetime-zone-none = No matching time zone

# Nerede: Tarih ve saat sayfasında, saat dilimi seçicisinin durum metni (src/pages/datetime.rs). Anlam: Saat dilimi listesi sistemden alınamadı.
settings-datetime-zone-list-failed = Could not get time zone list

# Nerede: Tarih ve saat sayfasında, saat dilimi satırının alt yazısı (src/pages/datetime.rs). Anlam: Geçerli saat dilimi bilinmiyor.
settings-datetime-zone-unknown = Unknown

# Nerede: Tarih ve saat sayfasında, Elle tarih ve saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Otomatik saat açıkken elle ayar kapalıdır.
settings-datetime-manual-auto-on = Off while automatic time is on

# Nerede: Tarih ve saat sayfasında, Elle tarih ve saat satırının alt yazısı (src/pages/datetime.rs). Anlam: Tarih ve saati seçip Uygula düğmesine basın.
settings-datetime-manual-hint = Choose the date and time and apply

# Nerede: Tarih ve saat sayfasında, Elle tarih ve saat satırındaki düğme (src/pages/datetime.rs). Anlam: Seçilen tarih ve saati sisteme yazar.
settings-datetime-manual-apply = Apply

# Nerede: Tarih ve saat sayfası, Biçim kartında Saat biçimi seçiminin altındaki not (src/pages/datetime.rs). Anlam: Bar saati için elle strftime kalıbı yazılmış; 12/24 saat seçimi bar saatini etkilemez.
settings-datetime-clock-custom = Bar clock uses a custom format

# Nerede: Tarih ve saat sayfası, aynı notun sağındaki düğme (src/pages/datetime.rs). Anlam: Bar saatinin özel biçimini siler; saat yeniden 12/24 saat seçimine uyar.
settings-datetime-clock-clear = Clear

# Nerede: Bar sayfası, Seçili widget kartında saat widget'ının Biçim ve Dikey bar biçimi alanları boşken görünen yer tutucu (src/registry.rs). Anlam: Alan boş bırakılırsa saat, Tarih ve saat sayfasındaki 12/24 saat seçimine göre gösterilir.
settings-bar-clock-format-auto = Automatic (follows 12/24 hour preference)

# Nerede: Hareket (Motion) sayfasında, Solma ve renk bölümündeki motion.effects ayarının satır başlığı. Anlam: Solma ve renk animasyonlarını içeren grubun başlığıdır.
settings-motion-effects-label = Fade and color

# Nerede: Hareket (Motion) sayfasında, rol kartının başlığı ve bölüm adı. Anlam: Solma ve renk geçişlerini ayarlayan bölüm olduğunu söyler.
settings-motion-role-effects = Fade and color

# Nerede: Appearance sayfasındaki Renkler bölümünde, palette.mode ayarının (Renk şemasının kipi) arama sözcükleri. Anlam: Bu sözcükler, renk şemasının karanlık ya da aydınlık kipini aramada bulmayı sağlar.
settings-palette-mode-keywords = dark, light, palette, mode

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.source ayarının (Renkler nereden gelsin) arama sözcükleri. Anlam: Bu sözcükler, renklerin nereden alınacağını (tema, palet ya da duvar kağıdı) aramada bulmayı sağlar.
settings-colors-source-keywords = color source, theme, palette, matugen, material you, wallpaper, automatic color

# Nerede: Görünüm › Renkler, colors.theme ayarının (Renk teması) arama sözcükleri. Anlam: tema adlarıyla aramada bulmayı sağlar.
settings-colors-theme-keywords = color theme, theme, blue, neutral, sage, rose, amber, graphite, color

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.primary ayarının (Vurgu rengi) arama sözcükleri. Anlam: Bu sözcükler, arayüzün vurgu rengini aramada bulmayı sağlar.
settings-colors-primary-keywords = accent, primary color, theme color

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.surface_bright ayarının (Vurgulu yüzey) arama sözcükleri. Anlam: Bu sözcükler, üzerine gelindiğinde parlayan yüzeyin rengini aramada bulmayı sağlar.
settings-colors-surface-bright-keywords = hover, bright surface

# Nerede: Appearance sayfasındaki Renkler bölümünde, colors.on_surface ayarının (Yazı) arama sözcükleri. Anlam: Bu sözcükler, yüzeylerin üzerindeki metin ve yazı rengini aramada bulmayı sağlar.
settings-colors-on-surface-keywords = text, text color

# Nerede: Appearance sayfasındaki Köşeler ve şekil bölümünde, blob.smoothing ayarının (Birleşim kavisi) arama sözcükleri. Anlam: Bu sözcükler, birleşen köşelerin yumuşaklığını, yani iç kavis ve fileto yumuşaklığını aramada bulmayı sağlar.
settings-blob-smoothing-keywords = smoothing, blob, fillet, pudding, inner curve, inner corner, frame

# Nerede: Fonts sayfasındaki Yazı tipleri bölümünde, typography.icon.scale ayarının (İkon boyut oranı) arama sözcükleri. Anlam: Bu sözcükler, simgelerin büyüklük ve ölçek oranını aramada bulmayı sağlar.
settings-typography-icon-scale-keywords = icon, size, scale, ratio

# Nerede: Motion sayfasında, bölümsüz duran motion.preset ayarının (Karakter) arama sözcükleri. Anlam: Bu sözcükler, hareketin genel karakterini (sakin, canlı, çevik ya da kapalı) aramada bulmayı sağlar.
settings-motion-preset-keywords = preset, calm, lively, snappy, off, pudding, animation

# Nerede: Motion sayfasındaki Genel bölümünde, motion.style ayarının (Hareket türü) arama sözcükleri. Anlam: Bu sözcükler, hareketin yay mı eğri mi olacağını, yani hareket türünü aramada bulmayı sağlar.
settings-motion-style-keywords = style, spring, curve, bezier, easing

# Nerede: Motion sayfasındaki Paneller ve pencereler bölümünde, motion.spatial.style ayarının (Hareket türü) arama sözcükleri. Anlam: Bu sözcükler, panellerin ve pencerelerin hareket türünü aramada bulmayı sağlar.
settings-motion-role-style-keywords = style, spring, curve

# Nerede: Motion sayfasındaki Paneller ve pencereler bölümünde, motion.spatial.damping ayarının (Yaylanma) arama sözcükleri. Anlam: Bu sözcükler, hareketin yaylanma ve sönüm miktarını aramada bulmayı sağlar.
settings-motion-role-damping-keywords = damping, bounce, pudding

# Nerede: Bar sayfasındaki Seçili widget bölümünde, bar.widget.clock.format ayarının (Biçim) arama sözcükleri. Anlam: Bu sözcükler, saat widget'ının saat biçimini (12 ya da 24 saat, otomatik) aramada bulmayı sağlar.
settings-bar-widget-clock-format-keywords = clock, clock format, date, format, strftime, 12 hour, 24 hour, automatic

# Nerede: Bar sayfasındaki Widget'lar bölümünde, bar.sections ayarının (Bar bölümleri) arama sözcükleri. Anlam: Bu sözcükler, barda hangi widget'ların (saat, tepsi, çalışma alanı) sıralanacağını aramada bulmayı sağlar.
settings-bar-sections-keywords = widget, clock, tray, workspace, order

# Nerede: Bar sayfasındaki Konum ve boyut bölümünde, bar.edge ayarının (Ekran kenarı) arama sözcükleri. Anlam: Bu sözcükler, barın ekranın üst, alt, sol ya da sağ kenarında duracağını aramada bulmayı sağlar.
settings-bar-edge-keywords = position, top, bottom, left, right, vertical, panel

# Nerede: Bar sayfasındaki Konum ve boyut bölümünde, bar.size ayarının (Kalınlık) arama sözcükleri. Anlam: Bu sözcükler, barın kalınlığını, yani yükseklik ve boyutunu aramada bulmayı sağlar.
settings-bar-size-keywords = bar thickness, height, size, edge

# Nerede: Bar sayfasındaki Konum ve boyut bölümünde, bar.padding ayarının (Uçlardaki boşluk) arama sözcükleri. Anlam: Bu sözcükler, barın uçlarındaki iç boşluğu aramada bulmayı sağlar.
settings-bar-padding-keywords = edge padding, padding, inner space

# Nerede: Bar sayfasındaki Çerçeve bölümünde, frame.thickness ayarının (Kalınlık) arama sözcükleri. Anlam: Bu sözcükler, çerçevenin kenar kalınlığını aramada bulmayı sağlar.
settings-frame-thickness-keywords = frame thickness, edge, border

# Nerede: Windows sayfasındaki Yerleşim bölümünde, windows.border ayarının (Kenarlık) arama sözcükleri. Anlam: Bu sözcükler, etkin pencerenin kenarlık ve çerçeve çizgisi kalınlığını aramada bulmayı sağlar.
settings-windows-border-keywords = border, frame, line, active window, border thickness

# Nerede: Windows sayfasındaki Görünüm bölümünde, windows.blur ayarının (Saydam pencerelerin ardı bulanık) arama sözcükleri. Anlam: Bu sözcükler, saydam pencerelerin arkasındaki bulanıklığı, yani cam etkisini aramada bulmayı sağlar.
settings-windows-blur-keywords = blur, transparency, glass, background

# Nerede: Windows sayfasındaki Görünüm bölümünde, windows.corners ayarının (Köşe, gölge ve hareket) arama sözcükleri. Anlam: Bu sözcükler, pencere köşelerinin yuvarlaklığını, gölgesini ve animasyonunu aramada bulmayı sağlar.
settings-windows-corners-keywords = corner, round, radius, rounding, shadow, animation, shared with theme

# Nerede: Windows sayfasındaki Görünüm bölümünde, windows.motion ayarının (Pencere hareketleri) arama sözcükleri. Anlam: Bu sözcükler, pencerelerin animasyon ve geçiş hareketlerini aramada bulmayı sağlar.
settings-windows-motion-keywords = animation, motion, transition, workspace

# Nerede: Windows sayfasındaki Gelişmiş bölümünde, compositor.ensure_source ayarının (Masaüstü ayarlarına Nuxut'u ekle) arama sözcükleri. Anlam: Bu sözcükler, Nuxut'un Hyprland yapılandırma dosyasına dahil edilmesini aramada bulmayı sağlar.
settings-compositor-ensure-source-keywords = hyprland, source, hyprland.conf, hyprland.lua, include, connect

# Nerede: Wallpaper sayfasında, bölümsüz duran wallpaper ayarının (Duvar kağıdı) arama sözcükleri. Anlam: Bu sözcükler, duvar kağıdı olarak kullanılacak arka plan resmini aramada bulmayı sağlar.
settings-wallpaper-keywords = background, image, visual, gif, webp

# Nerede: Wallpaper sayfasındaki Renkler bölümünde, palette.source ayarının (Renkleri duvar kağıdından al) arama sözcükleri. Anlam: Bu sözcükler, renk şemasının duvar kağıdından otomatik alınmasını aramada bulmayı sağlar.
settings-palette-source-keywords = matugen, material you, automatic color, theme, color scheme

# Nerede: Bar sayfasındaki Seçili widget bölümünde, bar.widget.clock.format ayarının alt yazısı burada. Anlam: Saat biçimi boş bırakılırsa 12/24 saat tercihine uyulacağını, strftime kalıbı girilirse onun kullanılacağını anlatır.
settings-bar-widget-clock-format-description = Follows the 12/24 hour preference if left empty; a format pattern is used if entered.

# Nerede: Varsayılan uygulamalar sayfasındaki Durum bölümünde, defaults.note ayarının satır başlığı burada. Anlam: Varsayılan uygulamaların durumunu gösteren satırın başlığıdır.
settings-defaults-note-label = Default apps status

# Nerede: Varsayılan uygulamalar sayfasındaki Durum bölümünde, defaults.note ayarının arama sözcükleri burada. Anlam: Bu ayarı ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-note-keywords = default app, shell, desktop

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.browser ayarının satır başlığı burada. Anlam: Varsayılan tarayıcıyı seçen satırın başlığıdır.
settings-defaults-browser-label = Browser

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.browser ayarının alt yazısı burada. Anlam: Bu ayarın web sayfalarını ve bağlantıları açan uygulamayı seçtiğini anlatır.
settings-defaults-browser-description = Web pages and links

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.browser ayarının arama sözcükleri burada. Anlam: Tarayıcı ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-browser-keywords = browser, web, internet, link, http, html, firefox, chrome, chromium, default app

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.email ayarının satır başlığı burada. Anlam: Varsayılan e-posta istemcisini seçen satırın başlığıdır.
settings-defaults-email-label = Email

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.email ayarının alt yazısı burada. Anlam: Bu ayarın e-posta bağlantılarını (mailto) açan uygulamayı seçtiğini anlatır.
settings-defaults-email-description = Email links (mailto)

# Nerede: Varsayılan uygulamalar sayfasındaki İnternet bölümünde, defaults.email ayarının arama sözcükleri burada. Anlam: E-posta ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-email-keywords = email, mail, mailto, thunderbird, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.file_manager ayarının satır başlığı burada. Anlam: Varsayılan dosya yöneticisini seçen satırın başlığıdır.
settings-defaults-file-manager-label = File manager

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.file_manager ayarının alt yazısı burada. Anlam: Bu ayarın klasörleri açan uygulamayı seçtiğini anlatır.
settings-defaults-file-manager-description = Opens folders

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.file_manager ayarının arama sözcükleri burada. Anlam: Dosya yöneticisi ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-file-manager-keywords = file manager, files, folder, directory, nautilus, thunar, dolphin, nemo, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.terminal ayarının satır başlığı burada. Anlam: Varsayılan terminali seçen satırın başlığıdır.
settings-defaults-terminal-label = Terminal

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.terminal ayarının alt yazısı burada. Anlam: Bu ayarın terminalde açılan uygulamalar için kullanılacağını anlatır.
settings-defaults-terminal-description = For apps opened in a terminal

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.terminal ayarının arama sözcükleri burada. Anlam: Terminal ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-terminal-keywords = terminal, console, command line, xdg-terminal-exec, kitty, foot, alacritty, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.text_editor ayarının satır başlığı burada. Anlam: Varsayılan metin düzenleyiciyi seçen satırın başlığıdır.
settings-defaults-text-editor-label = Text editor

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.text_editor ayarının alt yazısı burada. Anlam: Bu ayarın düz metin dosyalarını açan uygulamayı seçtiğini anlatır.
settings-defaults-text-editor-description = Plain text files

# Nerede: Varsayılan uygulamalar sayfasındaki Sistem bölümünde, defaults.text_editor ayarının arama sözcükleri burada. Anlam: Metin düzenleyici ayarını ararken kullanılabilecek anahtar kelimeleri listeler.
settings-defaults-text-editor-keywords = text, editor, txt, notepad, gedit, kate, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.image ayarının satır başlığı. Anlam: Bu metin, varsayılan resim görüntüleyiciyi seçen satırın başlığıdır.
settings-defaults-image-label = Image viewer

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.image ayarının alt yazısı. Anlam: Bu metin, hangi dosya türlerinin resim görüntüleyicide açıldığını kısaca açıklar.
settings-defaults-image-description = PNG, JPEG and other images

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.image ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken resim görüntüleyici satırını bulmayı sağlar.
settings-defaults-image-keywords = image, photo, png, jpg, jpeg, viewer, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.video ayarının satır başlığı. Anlam: Bu metin, varsayılan video oynatıcıyı seçen satırın başlığıdır.
settings-defaults-video-label = Video player

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.video ayarının alt yazısı. Anlam: Bu metin, hangi dosya türlerinin video oynatıcıda açıldığını kısaca açıklar.
settings-defaults-video-description = MP4, MKV, WebM and other videos

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.video ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken video oynatıcı satırını bulmayı sağlar.
settings-defaults-video-keywords = video, film, player, mp4, mkv, webm, mpv, vlc, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.music ayarının satır başlığı. Anlam: Bu metin, varsayılan müzik çaları seçen satırın başlığıdır.
settings-defaults-music-label = Music player

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.music ayarının alt yazısı. Anlam: Bu metin, hangi dosya türlerinin müzik çalarda açıldığını kısaca açıklar.
settings-defaults-music-description = MP3, FLAC, OGG and other audio files

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.music ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken müzik çalar satırını bulmayı sağlar.
settings-defaults-music-keywords = music, audio file, song, audio, mp3, flac, player, default app

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.pdf ayarının satır başlığı. Anlam: Bu metin, varsayılan PDF görüntüleyiciyi seçen satırın başlığıdır.
settings-defaults-pdf-label = PDF viewer

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.pdf ayarının alt yazısı. Anlam: Bu metin, PDF belgelerinin varsayılan uygulamayla açıldığını kısaca açıklar.
settings-defaults-pdf-description = PDF documents

# Nerede: Varsayılan uygulamalar sayfasındaki Medya ve belgeler bölümünde, defaults.pdf ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken PDF görüntüleyici satırını bulmayı sağlar.
settings-defaults-pdf-keywords = pdf, document, reader, evince, okular, zathura, default app

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.note ayarının satır başlığı. Anlam: Bu metin, sistemdeki tarih ve saat durumunu gösteren satırın başlığıdır.
settings-datetime-note-label = Date and time status

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.note ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken tarih ve saat durumu satırını bulmayı sağlar.
settings-datetime-note-keywords = timedated, rtc, hardware clock

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.ntp ayarının satır başlığı. Anlam: Bu metin, saatin internet üzerinden otomatik eşitlenmesini açan satırın başlığıdır.
settings-datetime-ntp-label = Automatic time

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.ntp ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken otomatik saat satırını bulmayı sağlar.
settings-datetime-ntp-keywords = ntp, network time, sync, internet time, automatic time

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.timezone ayarının satır başlığı. Anlam: Bu metin, sistem saat dilimini seçen satırın başlığıdır.
settings-datetime-timezone-label = Time zone

# Nerede: Tarih ve saat sayfasındaki Saat bölümünde, datetime.timezone ayarı için arama sözcükleri. Anlam: Bu sözcükler, ayarlar içinde arama yaparken saat dilimi satırını bulmayı sağlar.
settings-datetime-timezone-keywords = timezone, utc, gmt, city

# Nerede: Tarih ve saat sayfasında, Saat bölümündeki datetime.manual ayarının satır başlığı. Anlam: Tarih ve saatin elle ayarlanabileceğini belirtir.
settings-datetime-manual-label = Manual date and time

# Nerede: Tarih ve saat sayfasında, Saat bölümündeki datetime.manual ayarının arama sözcükleri. Anlam: Kullanıcı bu sözcüklerle elle tarih ve saat ayarını bulur.
settings-datetime-manual-keywords = date, time, manual, set

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki locale.hour_cycle ayarının satır başlığı. Anlam: Saatin hangi biçimde gösterileceğini seçtirir.
settings-locale-hour-cycle-label = Clock format

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki locale.hour_cycle ayarının alt yazısı. Anlam: Saatin 24 ya da 12 saatlik biçimde gösterilebileceğini anlatır.
settings-locale-hour-cycle-description = Whether the time is shown in 24-hour or 12-hour format

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki locale.hour_cycle ayarının arama sözcükleri. Anlam: Kullanıcı bu sözcüklerle saat biçimi ayarını bulur.
settings-locale-hour-cycle-keywords = 24 hour, 12 hour, am, pm, clock format

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki datetime.clock_format_note ayarının satır başlığı. Anlam: Bar saatine özel biçim ayarı verilebileceğini belirtir.
settings-datetime-clock-format-note-label = Bar clock format

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki datetime.clock_format_note ayarının alt yazısı. Anlam: Bar saatinin şu anda özel bir biçim kullandığını bildirir.
settings-datetime-clock-format-note-description = The bar clock uses a custom format

# Nerede: Tarih ve saat sayfasında, Biçim bölümündeki datetime.clock_format_note ayarının arama sözcükleri. Anlam: Kullanıcı bu sözcüklerle bar saati biçimi ayarını bulur.
settings-datetime-clock-format-note-keywords = clock format, bar clock, custom format, clear

# Nerede: Kenar çubuğunda görünen sayfa adı. Anlam: Tarih ve saat ayarlarının bulunduğu sayfayı açar.
settings-page-datetime = Date and time

# Nerede: Kenar çubuğunda görünen sayfa adı. Anlam: Varsayılan uygulamaların seçildiği sayfayı açar.
settings-page-default-apps = Default apps

# Nerede: Tarih ve saat sayfasındaki bölüm adı. Anlam: Otomatik saat, saat dilimi ve elle tarih ve saat ayarlarını içeren bölümü başlıklandırır.
settings-section-clock = Clock

# Nerede: Tarih ve saat sayfasındaki bölüm adı. Anlam: Saat biçimi ve bar saati biçimi ayarlarını içeren bölümü başlıklandırır.
settings-section-format = Format

# Nerede: Varsayılan uygulamalar sayfasındaki bölüm adı. Anlam: Tarayıcı ve e-posta uygulamalarının seçildiği bölümü başlıklandırır.
settings-section-internet = Internet

# Nerede: Varsayılan uygulamalar sayfasındaki bölüm adı. Anlam: Dosya yöneticisi, terminal ve metin düzenleyici seçimlerinin yer aldığı bölümü başlıklandırır.
settings-section-system = System

# Nerede: Varsayılan uygulamalar sayfasındaki bölüm adı. Anlam: Resim, video, müzik ve PDF uygulamalarının seçildiği bölümü başlıklandırır.
settings-section-media-documents = Media and documents

# Nerede: Tarih ve saat sayfasında, Saat biçimi seçeneğinin Otomatik (auto) değerinin etiketi. Anlam: Saat biçiminin dile göre otomatik belirleneceğini belirtir.
settings-choice-hour-cycle-auto = Automatic

# Nerede: Tarih ve saat sayfasında, Saat biçimi seçeneğinin 24 saat (h23) değerinin etiketi. Anlam: Saatin 24 saatlik biçimde gösterileceğini seçer.
settings-choice-hour-cycle-h23 = 24-hour

# Nerede: Tarih ve saat sayfasında, Saat biçimi seçeneğinin 12 saat (h12) değerinin etiketi. Anlam: Saatin ÖÖ/ÖS eklenerek 12 saatlik biçimde gösterileceğini seçer.
settings-choice-hour-cycle-h12 = 12-hour

# Nerede: Yazı sayfasındaki kartın sonundaki açılır satırın başlığıdır; içinde rol başına boyut oranları, ikon yazı tipi ve ikon boyutu ayarları bulunur. Anlam: Bu metin, rol başına boyut oranlarını ve ikon yazı tipini ayarlayan bölümün başlığıdır.
settings-detail-typography-label = Size ratios and icon font

# Nerede: Yazı sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde rol başına boyutun, ikonların yazı tipini ve boyutunun ayarlandığını özetler.
settings-detail-typography-description = Size per role, icon font and size

# Nerede: Hareket sayfasındaki kartın sonundaki açılır satırın başlığıdır; içinde hareket türü ve öğe gruplarının yay ya da eğri ayarları bulunur. Anlam: Bu metin, hareket ayarlarını tek tek yapmayı sağlayan bölümün başlığıdır.
settings-detail-motion-label = Adjust motion individually

# Nerede: Hareket sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde her hareket türü ve öğe grubu için yayın ya da eğrinin ayarlandığını özetler.
settings-detail-motion-description = Motion type and each item group's spring or curve

# Nerede: Pencereler sayfasındaki kartın sonundaki açılır satırın başlığıdır; içinde Nuxut'un pencerelere ve masaüstü ayar dosyasına uyguladığı ayarlar bulunur. Anlam: Bu metin, Nuxut'un masaüstüyle birlikte çalışma biçimini ayarlayan bölümün başlığıdır.
settings-detail-compositor-label = Desktop integration

# Nerede: Pencereler sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde Nuxut'un pencerelere ve masaüstü ayar dosyasına uyguladığı ayarların görüldüğünü özetler.
settings-detail-compositor-description = What Nuxut applies to windows and the desktop config file

# Nerede: Görünüm sayfasındaki Renkler kartının sonundaki açılır satırın başlığıdır; vurgu dışındaki on renk rolü burada ayarlanır. Anlam: Bu metin, renkleri tek tek ayarlamayı sağlayan bölümün başlığıdır.
settings-detail-colors-label = Adjust colors individually

# Nerede: Görünüm sayfasında aynı açılır satırın alt yazısıdır. Anlam: Bu metin, bölümde yüzey, yazı, hata ve çizgi renklerinin ayarlandığını özetler.
settings-detail-colors-description = Surface, text, error and line colors

# Nerede: Görünüm sayfasındaki Köşeler ve şekil bölümünde, Kenar çizgileri satırının alt yazısı yerine görünen durum notu. Anlam: Yüksek karşıtlık açıkken kenar çizgisinin "Yok" seçili olsa bile ince olarak çizildiğini bildirir.
settings-shape-outlines-high-contrast-note = High contrast is on: drawn thin

# Nerede: Görünüm sayfasındaki Renkler bölümünde, colors.contrast ayarının satır başlığıdır. Anlam: Bu metin, yüksek karşıtlık seçeneğinin adını gösterir.
settings-colors-contrast-label = High contrast

# Nerede: Görünüm sayfasındaki Renkler bölümünde, colors.contrast ayarının alt yazısıdır. Anlam: Bu metin, yüksek karşıtlık açıldığında metin ve kenarların daha belirgin olacağını açıklar.
settings-colors-contrast-description = Text and borders become more distinct

# Nerede: Görünüm sayfasındaki Renkler bölümünde, colors.contrast ayarının arama sözcükleridir. Anlam: Bu sözcükler, yüksek karşıtlık ayarının aramada bulunmasını sağlar.
settings-colors-contrast-keywords = contrast, accessibility, vision, readability, high contrast

# Nerede: Görünüm sayfasındaki Renkler bölümünde, palette.contrast ayarının satır başlığıdır. Anlam: Bu metin, palet kontrastı ayarının adını gösterir.
settings-palette-contrast-label = Palette contrast

# Nerede: Görünüm sayfasındaki Renkler bölümünde, palette.contrast ayarının alt yazısıdır. Anlam: Bu metin, duvar kağıdından gelen renklerin birbirinden ne kadar farklı olacağını açıklar; solda az, sağda çok fark olur.
settings-palette-contrast-description = Difference between colors from the wallpaper; less on the left, more on the right

# Nerede: Görünüm sayfasındaki Köşeler ve şekil bölümünde, shape.outlines ayarının alt yazısıdır. Anlam: Bu metin, bileşen sınırlarını gösteren çizgileri açıklar; yüksek karşıtlık açıkken bu çizgiler en az ince biçimde çizilir.
settings-shape-outlines-description = Component borders; drawn thin at minimum when high contrast is on

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.enabled ayarının satır başlığıdır. Anlam: Bu metin, gece ışığı özelliğini açıp kapatan satırın başlığını gösterir.
settings-night-light-enabled-label = Use night light

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.enabled ayarının alt yazısıdır. Anlam: Bu metin, gece ışığının akşamları ekran renklerini sıcaklaştırdığını açıklar.
settings-night-light-enabled-description = Makes screen colors warmer in the evenings

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.enabled ayarının arama sözcükleridir. Anlam: Bu sözcükler, gece ışığı ayarının aramada bulunmasını sağlar.
settings-night-light-enabled-keywords = night, night light, blue light, temperature, eyes, hyprsunset, color temperature

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.temperature ayarının satır başlığıdır. Anlam: Bu metin, gece sıcaklığı ayarının adını gösterir.
settings-night-light-temperature-label = Night temperature

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.temperature ayarının alt yazısıdır. Anlam: Bu metin, düşük sıcaklık değerinin daha turuncu bir renk verdiğini açıklar.
settings-night-light-temperature-description = Lower value is more orange

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.temperature ayarının arama sözcükleridir. Anlam: Bu sözcükler, gece sıcaklığı ayarının aramada bulunmasını sağlar.
settings-night-light-temperature-keywords = night, temperature, kelvin, color temperature, orange

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.schedule ayarının satır başlığıdır. Anlam: Bu metin, gece ışığı zamanlaması satırının başlığını gösterir.
settings-night-light-schedule-label = Schedule

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.schedule ayarının arama sözcükleridir. Anlam: Bu sözcükler, zamanlama ayarının aramada bulunmasını sağlar.
settings-night-light-schedule-keywords = night, schedule, sunset, sunrise, time

# Nerede: Ekranlar sayfasındaki Gece ışığı bölümünde, night_light.from ayarının satır başlığıdır. Anlam: Bu metin, gece ışığının başlangıç saati satırının başlığını gösterir.
settings-night-light-from-label = Start time

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.from ayarının alt yazısı. Anlam: Bitiş saati daha erken bir değere ayarlandığında aralığın gece yarısını aştığını anlatır.
settings-night-light-from-description = Crosses midnight when the end is earlier

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.from ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-from-keywords = night, start, time, evening

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.to ayarının satır başlığı. Anlam: Gece ışığının kapanacağı bitiş saatinin seçildiği satır.
settings-night-light-to-label = End time

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.to ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-to-keywords = night, end, time, morning

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.location ayarının satır başlığı. Anlam: Gün batımı ve gün doğumu saatlerinin hesaplanacağı konumun girildiği satır.
settings-night-light-location-label = Location

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.location ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-location-keywords = night, location, latitude, longitude, sunset, sunrise

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.transition ayarının satır başlığı. Anlam: Renk değişiminin kaç dakika süreceğinin seçildiği satır.
settings-night-light-transition-label = Transition time

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.transition ayarının alt yazısı. Anlam: Renklerin bu süre içinde yavaşça değiştiğini anlatır.
settings-night-light-transition-description = Colors change gradually over this time

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki night_light.transition ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-night-light-transition-keywords = night, transition, smooth, duration, minutes

# Nerede: Görünüm sayfasında, Renkler bölümündeki palette.contrast (Palet kontrastı) ayarının arama sözcükleri. Anlam: Bu ayarı ararken kullanılacak sözcüklerin virgülle ayrılmış listesi.
settings-palette-contrast-keywords = contrast, accessibility, readability

# Nerede: Ekranlar sayfasındaki bölüm (kart) adı. Anlam: Gece ışığı ayarlarını içeren bölümün başlığı.
settings-section-night-light = Night light

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki Zamanlama seçeneğinin bir değeri (always). Anlam: Gece ışığının sürekli açık kalacağını belirten seçim.
settings-choice-night-schedule-always = Always

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki Zamanlama seçeneğinin bir değeri (manual). Anlam: Gece ışığının başlangıç ve bitiş saatleri arasında açık kalacağını belirten seçim.
settings-choice-night-schedule-manual = At set times

# Nerede: Ekranlar sayfasında, Gece ışığı bölümündeki Zamanlama seçeneğinin bir değeri (sun). Anlam: Gece ışığının gün batımından gün doğumuna kadar açık kalacağını belirten seçim.
settings-choice-night-schedule-sun = Sunset to sunrise

# ---- Kullanıcı hesabı (t118) ----

# Nerede: Kenar çubuğunda görünen sayfa adı (Sistem grubu). Anlam: Bu bilgisayardaki kendi kullanıcı hesabının (görünen ad, profil resmi) ayarlandığı sayfa; çevrimiçi takvim hesaplarını içeren "Hesaplar" sayfasından farklıdır.
settings-page-user = User account

# Nerede: Kullanıcı hesabı sayfasındaki bölüm başlığı. Anlam: Profil resmi ve görünen ad satırlarını toplayan bölüm.
settings-section-profile = Profile

# Nerede: Kullanıcı hesabı sayfasında, user.note satırının başlığı (aramada görünür). Anlam: Hesap hizmetinin (AccountsService) çalışıp çalışmadığını bildiren üst not.
settings-user-note-label = Account status

# Nerede: Kullanıcı hesabı sayfasında, user.note satırı için arama sözcükleri. Anlam: Hesap hizmeti notunu aramada bulmayı sağlar.
settings-user-note-keywords = accountsservice, account service

# Nerede: Kullanıcı hesabı sayfasının Profil bölümünde, user.avatar satırının başlığı (aramada görünür). Anlam: Kullanıcının profil resmi (kilit ekranında ve giriş ekranında görünen resim).
settings-user-avatar-label = Profile picture

# Nerede: Kullanıcı hesabı sayfasında, user.avatar satırı için arama sözcükleri. Anlam: Profil resmi satırını aramada bulmayı sağlar; satır kullanıcı adını ve hesap türünü de gösterir.
settings-user-avatar-keywords = avatar, profile picture, photo, picture, face, username, account type, administrator

# Nerede: Kullanıcı hesabı sayfasının Profil bölümünde, user.real_name satırının başlığı. Anlam: Kullanıcının tam adı (ad soyad); kullanıcı adı (giriş adı) değil.
settings-user-real-name-label = Display name

# Nerede: Kullanıcı hesabı sayfasında, user.real_name satırı için arama sözcükleri. Anlam: Görünen ad satırını aramada bulmayı sağlar.
settings-user-real-name-keywords = name, full name, real name, display name

# Nerede: Kullanıcı hesabı sayfasının üstündeki uyarı notu. Anlam: Sistemde AccountsService hizmeti yok ya da çalışmıyor; ad ve profil resmi değiştirilemez.
settings-user-missing = Account service (AccountsService) not found; the name and profile picture cannot be changed.

# Nerede: Kullanıcı hesabı sayfasının üstündeki uyarı notu. Anlam: Hesap hizmeti var ama kullanıcı bilgileri okunamadı.
settings-user-read-failed = Could not read the account details.

# Nerede: Kullanıcı hesabı sayfasında bir satırın alt metni, değişiklik sırasında hizmet kaybolduysa. Anlam: Hesap hizmeti yok, değişiklik yapılamadı.
settings-user-missing-short = Account service unavailable

# Nerede: Kullanıcı hesabı sayfasında bir satırın alt metni (kırmızı). Anlam: Görünen ad ya da profil resmi değiştirilemedi.
settings-user-failed = Could not be changed

# Nerede: Kullanıcı hesabı sayfasında bir satırın alt metni, değişiklik sürerken. Anlam: Sistem yetki istedi; kullanıcının parola diyaloğunu onaylaması bekleniyor.
settings-user-waiting = Waiting for authorization…

# Nerede: Profil resmi satırındaki düğme. Anlam: Dosya seçiciyi açıp yeni profil resmi seçtirir.
# [max: 14]
settings-user-avatar-change = Change…

# Nerede: Profil resmi için açılan dosya seçici penceresinin başlığı. Anlam: Kullanıcıdan profil resmi olacak bir görsel seçmesini ister.
settings-user-avatar-dialog-title = Choose a profile picture

# Nerede: Profil resmi dosya seçicisindeki dosya türü filtresinin adı. Anlam: Yalnızca görsel dosyaları (PNG, JPEG, WebP, GIF) gösterilir.
settings-user-avatar-filter = Images

# Nerede: Profil resmi satırının alt metni (kırmızı). Anlam: Seçilen dosya açılamadı; bozuk, çok büyük (8 MB üstü) ya da desteklenmeyen biçimde.
settings-user-avatar-bad-image = Could not open the image (damaged, too large or unsupported format)

# Nerede: Profil resmi satırının alt metni (kırmızı). Anlam: Sistemin dosya seçici hizmeti (xdg-desktop-portal) açılamadı.
settings-user-avatar-portal = Could not open the file chooser

# Nerede: Profil resmi satırının alt metni (kırmızı). Anlam: Profil resmi hesapta değişti, ama kilit ekranının okuduğu ev dizinindeki ~/.face dosyası yazılamadı.
settings-user-face-failed = The profile picture was changed, but the lock screen picture (~/.face) could not be written

# Nerede: Görünen ad satırının alt metni (kırmızı). Anlam: Girilen ad kabul edilmedi: iki nokta üst üste ya da virgül içeriyor veya 64 karakterden uzun.
settings-user-name-invalid = The name cannot contain colons or commas and can be at most 64 characters

# Nerede: Görünen ad alanı boşken içinde görünen soluk ipucu. Anlam: Buraya kullanıcının adı soyadı yazılır.
settings-user-name-placeholder = Full name

# Nerede: Profil resmi satırında, kullanıcı adının yanında gösterilen hesap türü. Anlam: Bu hesap yönetici (sudo/wheel) yetkili.
settings-user-type-admin = Administrator

# Nerede: Profil resmi satırında, kullanıcı adının yanında gösterilen hesap türü. Anlam: Bu hesap yönetici yetkisi olmayan sıradan bir hesap.
settings-user-type-standard = Standard account

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
settings-nav-group-connections = Connections

# Nerede: Wi-Fi sayfasında kart başlığı. Anlam: Taramada görünen kablosuz ağların listesi.
settings-section-wifi-visible = Visible networks

# Nerede: Wi-Fi sayfasında kart başlığı. Anlam: Daha önce bağlanılmış, ayarları saklanan kablosuz ağlar.
settings-section-wifi-saved = Saved networks

# Nerede: VPN sayfasında kart başlığı. Anlam: Kayıtlı VPN ve WireGuard bağlantılarının listesi.
settings-section-vpn-connections = VPN connections

# Nerede: Wi-Fi sayfasında, wifi.note satırının başlığı (aramada görünür). Anlam: NetworkManager hizmetinin çalışıp çalışmadığını bildiren üst not.
settings-wifi-note-label = Network status

# Nerede: Wi-Fi sayfasında, wifi.note satırı için arama sözcükleri. Anlam: Ağ hizmeti notunu aramada bulmayı sağlar.
settings-wifi-note-keywords = networkmanager, network service

# Nerede: Wi-Fi sayfasının Wi-Fi kartında, wifi.enabled satırının başlığı. Anlam: Kablosuz ağın açık mı kapalı mı olduğu.
settings-wifi-enabled-label = Wi-Fi

# Nerede: Wi-Fi sayfasında, wifi.enabled satırı için arama sözcükleri. Anlam: Wi-Fi aç/kapa satırını aramada bulmayı sağlar.
settings-wifi-enabled-keywords = wireless, wlan, turn on, turn off, airplane mode

# Nerede: Wi-Fi sayfasının Wi-Fi kartında, wifi.current satırının başlığı (aramada görünür). Anlam: Şu an bağlı olunan kablosuz ağ.
settings-wifi-current-label = Connected network

# Nerede: Wi-Fi sayfasında, wifi.current satırı için arama sözcükleri. Anlam: Bağlı ağ satırını aramada bulmayı sağlar.
settings-wifi-current-keywords = connected, ssid, signal, internet, disconnect, ip address, dns

# Nerede: Wi-Fi sayfasında, wifi.networks satırının başlığı (aramada görünür). Anlam: Taramada görünen kablosuz ağların listesi.
settings-wifi-networks-label = Visible networks

# Nerede: Wi-Fi sayfasında, wifi.networks satırı için arama sözcükleri. Anlam: Ağ listesini aramada bulmayı sağlar.
settings-wifi-networks-keywords = networks, scan, access point, ssid, signal, connect, password

# Nerede: Wi-Fi sayfasında, wifi.saved satırının başlığı (aramada görünür). Anlam: Daha önce bağlanılmış, ayarları saklanan kablosuz ağlar.
settings-wifi-saved-label = Saved networks

# Nerede: Wi-Fi sayfasında, wifi.saved satırı için arama sözcükleri. Anlam: Kayıtlı ağlar listesini aramada bulmayı sağlar.
settings-wifi-saved-keywords = saved, known networks, forget, connect automatically, metered

# Nerede: VPN sayfasında, vpn.note satırının başlığı (aramada görünür). Anlam: NetworkManager hizmetinin çalışıp çalışmadığını bildiren üst not.
settings-vpn-note-label = VPN status

# Nerede: VPN sayfasında, vpn.note satırı için arama sözcükleri. Anlam: Ağ hizmeti notunu aramada bulmayı sağlar.
settings-vpn-note-keywords = networkmanager, network service

# Nerede: VPN sayfasında, vpn.connections satırının başlığı (aramada görünür). Anlam: Kayıtlı VPN ve WireGuard bağlantılarının listesi.
settings-vpn-connections-label = VPN connections

# Nerede: VPN sayfasında, vpn.connections satırı için arama sözcükleri. Anlam: VPN listesini aramada bulmayı sağlar.
settings-vpn-connections-keywords = vpn, wireguard, openvpn, tunnel, delete

# Nerede: Wi-Fi ve VPN sayfalarının üstündeki uyarı notu. Anlam: Sistemde ağ hizmeti (NetworkManager) yok ya da çalışmıyor.
settings-network-missing = NetworkManager is not running; network details cannot be shown.

# Nerede: Wi-Fi ve VPN sayfalarının üstündeki uyarı notu. Anlam: Ağ hizmeti istemcisi başlatılamadı.
settings-network-failed = Could not connect to the network service.

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: $detail NetworkManager'ın (çevrilmemiş) hata metni.
settings-network-error-call-failed = Action failed: { $detail }

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Ağ hizmeti yok.
settings-network-error-unavailable = NetworkManager is not running

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: İstenen kayıtlı bağlantı artık yok.
settings-network-error-connection-unknown = Connection not found

# Nerede: Wi-Fi/VPN sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Kapatılmak istenen bağlantı zaten kapalı.
settings-network-error-connection-inactive = Connection is not active

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Bağlanılmak istenen kablosuz ağ taramada artık yok.
settings-network-error-ap-unknown = The network is no longer visible

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Kullanıcı adı/sertifika isteyen kurumsal ağlara buradan bağlanılamıyor.
settings-network-error-enterprise = Enterprise (802.1X) networks are not supported yet

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Girilen şifre bu güvenlik türünün kurallarına uymuyor.
settings-network-error-invalid-password = The password is not valid for this network

# Nerede: Wi-Fi sayfasının üst notu, bir ağ işlemi başarısız olunca. Anlam: Bilgisayarda kablosuz ağ aygıtı bulunamadı.
settings-network-error-no-device = No Wi-Fi device

# Nerede: Wi-Fi sayfasının üst notu, gizli ağ adı geçersizken. Anlam: Ağ adı (SSID) 1–32 bayt olmalı.
settings-network-error-invalid-ssid = Invalid network name

# Nerede: Wi-Fi sayfasının üst notu, ayrıntısı istenen ağ aygıtı artık yokken. Anlam: Aygıt bulunamadı (çıkarılmış olabilir).
settings-network-error-device-unknown = Network device not found

# Nerede: Wi-Fi/VPN sayfası, bir ağ işlemi izin verilmediği için reddedilince (normalde gösterilmez). Anlam: Sistem bu işleme izin vermedi (polkit).
settings-network-error-denied = You don't have permission for this action

# Nerede: Wi-Fi sayfasında ağ satırının hata metni, kayıtlı ağın şifresi değiştirilmek istenince. Anlam: Bu ağın güvenlik türünde (WEP, kurumsal, açık) şifre buradan değiştirilemiyor.
settings-network-error-unsupported = The password of this network type can't be changed here

# Nerede: Wi-Fi sayfasında ağ satırının hata metni, kayıtlı ağın şifresi değiştirilmek istenince. Anlam: Ağ "şifreyi her seferinde sor" olarak ayarlı; şifre kaydedilmiyor, bağlanırken sorulur.
settings-network-error-secret-not-saved = This network's password isn't saved; you'll be asked for it when connecting

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Şifresiz ağ.
settings-network-security-open = Open

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Şifre istemeyen ama trafiği şifreleyen ağ (OWE).
settings-network-security-owe = Open (encrypted)

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Eski WEP güvenliği.
settings-network-security-wep = WEP

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: WPA/WPA2 kişisel (şifreli) güvenlik.
settings-network-security-wpa = WPA/WPA2

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: WPA3 kişisel güvenlik.
settings-network-security-sae = WPA3

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Kullanıcı adı ya da sertifika isteyen kurumsal ağ.
settings-network-security-enterprise = Enterprise (802.1X)

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Ağın frekans bandı (ondalık ayırıcı dile göre).
settings-network-band-2g = 2.4 GHz

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Ağın frekans bandı.
settings-network-band-5g = 5 GHz

# Nerede: Wi-Fi sayfasında ağ satırının alt metni. Anlam: Ağın frekans bandı.
settings-network-band-6g = 6 GHz

# Nerede: Wi-Fi ve VPN listelerinde satırın alt metni. Anlam: Bağlantı şu an kuruluyor.
settings-network-connecting = Connecting…

# Nerede: Wi-Fi satırının alt metni. Anlam: Bilgisayarda kablosuz ağ aygıtı yok.
settings-wifi-status-no-device = Wi-Fi device not found

# Nerede: Wi-Fi satırının alt metni. Anlam: Wi-Fi fiziksel anahtar ya da uçak modu tuşuyla kapatılmış.
settings-wifi-status-hardware-off = Turned off with the hardware switch

# Nerede: Wi-Fi satırının alt metni. Anlam: Wi-Fi kapalı.
settings-wifi-status-off = Off

# Nerede: Wi-Fi satırının alt metni. Anlam: Wi-Fi açık.
settings-wifi-status-on = On

# Nerede: Wi-Fi sayfasında bağlı ağın ya da kayıtlı ağın alt metni. Anlam: Bu ağa şu an bağlı.
settings-wifi-connected = Connected

# Nerede: Wi-Fi sayfasında bağlı ağın alt metni. Anlam: Ağ tarayıcıda giriş sayfası (esir portal) istiyor.
settings-wifi-connected-portal = Connected, sign-in required

# Nerede: Wi-Fi sayfasında bağlı ağın alt metni. Anlam: Ağa bağlı ama internete erişim yok.
settings-wifi-connected-limited = Connected, no internet

# Nerede: Wi-Fi sayfasında Görünen ağlar kartının boş durumu. Anlam: Wi-Fi kapalı olduğu için liste boş.
settings-wifi-visible-off = Networks aren't shown while Wi-Fi is off.

# Nerede: Wi-Fi sayfasında Görünen ağlar kartının boş durumu. Anlam: Taramada hiç ağ bulunamadı.
settings-wifi-visible-empty = No visible networks.

# Nerede: Wi-Fi sayfasında görünen ağ satırının sağındaki kısa etiket. Anlam: Bu ağın ayarları kayıtlı.
# [max: 10]
settings-wifi-tag-saved = Saved

# Nerede: Wi-Fi sayfasında kayıtlı ağ satırının alt metni. Anlam: Ağ şu an taramada görünüyor.
settings-wifi-saved-in-range = In range

# Nerede: Wi-Fi sayfasında kayıtlı ağ satırının alt metni. Anlam: Ağ şu an taramada görünmüyor.
settings-wifi-saved-out-of-range = Out of range

# Nerede: Wi-Fi sayfasında Kayıtlı ağlar kartının boş durumu. Anlam: Hiç kayıtlı kablosuz ağ yok.
settings-wifi-saved-empty = No saved networks.

# Nerede: VPN sayfasında bağlantı satırının alt metni. Anlam: Bağlantı türü WireGuard.
settings-vpn-kind-wireguard = WireGuard

# Nerede: VPN sayfasında bağlantı satırının alt metni. Anlam: Bağlantı türü eklentili VPN (OpenVPN vb.).
settings-vpn-kind-vpn = VPN

# Nerede: VPN sayfasında bağlantı satırının sağındaki kısa etiket. Anlam: Bu VPN şu an açık.
# [max: 10]
settings-vpn-tag-active = Active

# Nerede: VPN sayfasında listenin boş durumu. Anlam: Hiç VPN ya da WireGuard bağlantısı yok.
settings-vpn-empty = No saved VPN connections.

# Nerede: Wi-Fi ve VPN sayfalarında, ağ durumu ilk kez okunurken kartın ya da satırın metni. Anlam: Bilgi NetworkManager'dan okunuyor.
settings-network-reading = Loading…

# ---- Bağlantılar: eylemler (t114 v1 ikinci yarı; docs/design/network-pages.md) ----

# Nerede: Wi-Fi sayfasında kart başlığı. Anlam: Kablolu (Ethernet) bağlantının salt okunur bilgisi.
settings-section-wifi-wired = Wired

# Nerede: Wi-Fi sayfasında, wifi.wired satırının başlığı (aramada görünür). Anlam: Kablolu ağ bağlantısı.
settings-wifi-wired-label = Wired connection

# Nerede: Wi-Fi sayfasında, wifi.wired satırı için arama sözcükleri. Anlam: Kablolu bağlantı kartını aramada bulmayı sağlar.
settings-wifi-wired-keywords = ethernet, wired, lan, ip address, dns, speed

# Nerede: Wi-Fi sayfasında Görünen ağlar kartının son satırı. Anlam: Adını yayınlamayan bir ağa adını yazarak bağlanmayı açar.
settings-wifi-hidden-label = Connect to hidden network…

# Nerede: Wi-Fi sayfasında, wifi.hidden satırı için arama sözcükleri. Anlam: Gizli ağ satırını aramada bulmayı sağlar.
settings-wifi-hidden-keywords = hidden network, ssid

# Nerede: VPN sayfasında VPN kartının son satırı. Anlam: Bir WireGuard yapılandırma dosyasını (.conf) seçip yeni VPN bağlantısı olarak ekler.
settings-vpn-import-label = Import WireGuard…

# Nerede: VPN sayfasında, vpn.import satırı için arama sözcükleri. Anlam: İçe aktarma satırını aramada bulmayı sağlar.
settings-vpn-import-keywords = import, wireguard, conf, file

# Nerede: Wi-Fi ve VPN sayfalarının üstündeki uyarı notu. Anlam: Sistem ileti yoluna (D-Bus) bağlanılamadığı için ağ bilgisi alınamıyor.
settings-network-no-bus = Cannot reach the system message bus (D-Bus); network information is unavailable.

# Nerede: Wi-Fi sayfasında ağ satırının hata metni. Anlam: Ağ şifre istedi ama kayıtlı şifre yok ya da yanlış; şifre alanı açılır.
settings-wifi-error-no-secrets = Password required or incorrect. Enter the password.

# Nerede: Wi-Fi sayfasında ağ satırının hata metni. Anlam: Ağ kimlik doğrulamayı reddetti, büyük olasılıkla şifre yanlış.
settings-wifi-error-auth-failed = Authentication failed; the password may be wrong.

# Nerede: VPN sayfasında bağlantı satırının hata metni. Anlam: VPN bağlanmak için gizli bilgi (anahtar, şifre) istedi ama alamadı.
settings-vpn-error-no-secrets = The VPN's required secrets were not provided.

# Nerede: VPN sayfasında bağlantı satırının hata metni. Anlam: VPN sunucusu girişi reddetti.
settings-vpn-error-auth-failed = VPN login was rejected.

# Nerede: Wi-Fi ve VPN satırlarının hata metni. Anlam: Bağlantı başka bir nedenle kurulamadı.
settings-network-error-activation = Could not connect.

# Nerede: Wi-Fi sayfasında, şifreli bir ağa tıklayınca açılan şifre alanının ipucu metni. Anlam: Ağın şifresini yazın.
settings-wifi-password-placeholder = Password

# Nerede: Wi-Fi sayfasında şifre alanının ve gizli ağ formunun düğmesi. Anlam: Ağa bağlan.
# [max: 16]
settings-wifi-connect = Connect

# Nerede: Wi-Fi sayfasında şifre alanının ve gizli ağ formunun düğmesi. Anlam: Formu kapat, bağlanma.
# [max: 16]
settings-wifi-cancel = Cancel

# Nerede: Wi-Fi sayfasında bağlı ağa tıklayınca açılan bölümdeki düğme. Anlam: Bu ağla bağlantıyı kopar.
# [max: 24]
settings-wifi-disconnect = Disconnect

# Nerede: Wi-Fi sayfasında kurumsal ağa tıklayınca açılan not. Anlam: Kullanıcı adı/sertifika isteyen ağlara bağlanma sonraki sürümde gelecek.
settings-wifi-enterprise-note = Enterprise (802.1X) networks are not supported yet; coming in a later version.

# Nerede: Wi-Fi sayfasında gizli ağ formundaki ad alanının ipucu metni. Anlam: Ağın adını (SSID) yazın.
settings-wifi-hidden-ssid-placeholder = Network name (SSID)

# Nerede: Wi-Fi sayfasında kayıtlı ağa tıklayınca açılan ayar satırı. Anlam: Ağ kapsamadayken kendiliğinden bağlan.
settings-wifi-autoconnect = Connect automatically

# Nerede: Wi-Fi sayfasında kayıtlı ağa tıklayınca açılan ayar satırı. Anlam: Bu bağlantı kotalı/ücretli sayılsın mı (büyük indirmeler ertelenir).
settings-wifi-metered = Metered connection

# Nerede: Wi-Fi sayfasında Ölçülü bağlantı seçeneği. Anlam: Sistem kendisi karar versin.
# [max: 12]
settings-wifi-metered-auto = Automatic

# Nerede: Wi-Fi sayfasında Ölçülü bağlantı seçeneği. Anlam: Bu bağlantı ölçülü.
# [max: 12]
settings-wifi-metered-yes = Yes

# Nerede: Wi-Fi sayfasında Ölçülü bağlantı seçeneği. Anlam: Bu bağlantı ölçülü değil.
# [max: 12]
settings-wifi-metered-no = No

# Nerede: Wi-Fi sayfasında kayıtlı ağın ayar bölümündeki düğme. Anlam: Ağın kayıtlı ayarlarını ve şifresini sil.
# [max: 16]
settings-wifi-forget = Forget

# Nerede: Wi-Fi sayfasında Unut düğmesine bir kez basınca görünen onay metni. Anlam: Silmek için tekrar basın.
# [max: 16]
settings-wifi-forget-confirm = Forget it?

# Nerede: VPN sayfasında bağlantıya tıklayınca açılan bölümdeki düğme. Anlam: VPN bağlantısını sil.
# [max: 16]
settings-vpn-delete = Delete

# Nerede: VPN sayfasında Sil düğmesine bir kez basınca görünen onay metni. Anlam: Silmek için tekrar basın.
# [max: 16]
settings-vpn-delete-confirm = Delete it?

# Nerede: Wi-Fi sayfasında bağlı ağın ve kablolu bağlantının ayrıntı satırı etiketi. Anlam: IPv4 adresi.
settings-network-detail-ipv4 = IPv4

# Nerede: Wi-Fi sayfasında bağlı ağın ve kablolu bağlantının ayrıntı satırı etiketi. Anlam: IPv6 adresi.
settings-network-detail-ipv6 = IPv6

# Nerede: Wi-Fi sayfasında bağlı ağın ve kablolu bağlantının ayrıntı satırı etiketi. Anlam: Ad çözümleme (DNS) sunucuları.
settings-network-detail-dns = DNS

# Nerede: Wi-Fi sayfasında kablolu bağlantının ayrıntı satırı etiketi. Anlam: Bağlantı hızı.
settings-network-detail-speed = Speed

# Nerede: Wi-Fi sayfasında kablolu bağlantının hız değeri. Anlam: $mbps saniyede megabit.
settings-network-speed-mbps = { $mbps } Mb/s

# Nerede: Wi-Fi sayfasında kablolu bağlantının alt metni. Anlam: Kablo takılı değil ya da bağlantı yok.
settings-wired-disconnected = Not connected

# Nerede: VPN sayfasında içe aktarma satırının alt metni, dosya seçici açıkken ya da içe aktarma sürerken. Anlam: İşlem sürüyor.
settings-vpn-import-busy = Waiting for file…

# Nerede: WireGuard içe aktarma dosya seçicisinde dosya türü süzgecinin adı. Anlam: WireGuard .conf dosyaları.
settings-vpn-import-filter = WireGuard configuration (.conf)

# Nerede: WireGuard içe aktarma dosya seçicisinin başlığı. Anlam: İçe aktarılacak dosyayı seçin.
settings-vpn-import-dialog-title = Choose a WireGuard configuration

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosya seçici açılamadı.
settings-vpn-import-error-portal = Could not open the file chooser (is xdg-desktop-portal running?).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Seçilen dosya okunamadı.
settings-vpn-import-error-read = Could not read the file.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosya bir WireGuard yapılandırması için fazla büyük.
settings-vpn-import-error-too-large = The file is too large (64 KB at most).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosya düz metin değil.
settings-vpn-import-error-not-text = The file is not text.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosyanın $line numaralı satırı WireGuard biçimine uymuyor.
settings-vpn-import-error-syntax = Invalid WireGuard file (line { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosyada [Interface] özel anahtarı yok.
settings-vpn-import-error-private-key = The file has no private key (PrivateKey).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: $line satırındaki eşin ortak anahtarı yok.
settings-vpn-import-error-public-key = A peer has no public key (PublicKey) (line { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: $line satırındaki anahtar geçerli bir WireGuard anahtarı değil.
settings-vpn-import-error-key = Invalid key (line { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: $line satırındaki adres, sayı ya da uç nokta geçersiz.
settings-vpn-import-error-value = Invalid value (line { $line }).

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Dosyada çok fazla eş ya da adres var.
settings-vpn-import-error-too-many = The file has too many peers or addresses.

# Nerede: VPN sayfasında içe aktarma satırının hata metni. Anlam: Bağlantı adı dosya adından alınır; ad 1–15 harf/rakam ve - _ . = + olabilir.
settings-vpn-import-error-name = The file name cannot be a connection name: 1–15 characters, letters, digits and - _ . = + only.

# Nerede: VPN sayfasında içe aktarma satırının alt metni, içe aktarmadan sonra. Anlam: Dosyadaki bazı wg-quick ayarları (PostUp, Table…) içe aktarılmadı ve çalıştırılmayacak; $keys virgülle ayrılmış ayar adlarıdır (çevrilmez).
settings-vpn-import-ignored = These settings weren't imported: { $keys }

# ---- Sistem › Güç (t117 iskeleti; docs/design/power-page.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Güç profilleri ve pil şarj sınırı sayfasının etiketi.
# [max: 14]
settings-page-power = Power

# Nerede: Güç sayfasında kart başlığı. Anlam: Sistemin güç/performans dengesi seçimi.
settings-section-power-profile = Power profile

# Nerede: Güç sayfasında kart başlığı. Anlam: Dizüstü pili ile ilgili ayarlar.
settings-section-power-battery = Battery

# Nerede: Güç sayfasının Güç profili kartında, satır başlığı. Anlam: Güç tasarrufu, dengeli ya da performans profilinden birini seçme.
settings-power-profile-label = Power profile

# Nerede: Güç sayfasında, power.profile satırı için arama sözcükleri. Anlam: Güç profili seçimini aramada bulmayı sağlar.
settings-power-profile-keywords = power, profile, performance, power saver, balanced, power-profiles-daemon, battery

# Nerede: Güç sayfasının Pil kartında, anahtar satırının başlığı. Anlam: Açıkken pil tam dolmadan şarj durur; pilin ömrü uzar.
settings-power-charge-limit-label = Protect battery health

# Nerede: Güç sayfasında, power.charge_limit satırı için arama sözcükleri. Anlam: Şarj sınırı anahtarını aramada bulmayı sağlar.
settings-power-charge-limit-keywords = charge, charge limit, battery health, battery, charging threshold

# Nerede: Güç profili seçiminde, ilk seçeneğin adı. Anlam: Pili ve enerjiyi koruyan, performansı düşüren profil.
# [max: 16]
settings-power-profile-power-saver = Power saver

# Nerede: Güç profili seçiminde, ortadaki seçeneğin adı. Anlam: Varsayılan, dengeli profil.
# [max: 16]
settings-power-profile-balanced = Balanced

# Nerede: Güç profili seçiminde, son seçeneğin adı. Anlam: Daha çok enerji harcayan, en hızlı profil.
# [max: 16]
settings-power-profile-performance = Performance

# Nerede: Güç profili satırının alt metni. Anlam: Performans profili kısıtlandı, çünkü dizüstü bilgisayar kucakta (algılayıcı ısınmayı önlemek için).
settings-power-degraded-lap = Performance limited: laptop detected on lap

# Nerede: Güç profili satırının alt metni. Anlam: Performans profili kısıtlandı, çünkü aygıtın sıcaklığı yüksek.
settings-power-degraded-temperature = Performance limited: high temperature

# Nerede: Güç profili satırının alt metni. Anlam: Performans profili bilinmeyen bir nedenle kısıtlandı.
settings-power-degraded-other = Performance limited

# Nerede: Güç profili satırının alt metni, seçim devre dışıyken. Anlam: Profilleri değiştirmek için bu sistem hizmeti kurulu ve çalışıyor olmalı. "power-profiles-daemon" paket adıdır, çevrilmez.
settings-power-profiles-missing = Power profiles require power-profiles-daemon

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Birden çok pil var; şarj sınırı bazılarında açık, bazılarında kapalı.
settings-power-charge-mixed = On for some batteries

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Şarj $end yüzdesinde durur. $end_tail sayının okunuşundaki son sözcüğün değeridir (75 → 5, 80 → 80, 100 → 100); Türkçe bulunma eki ('de/'da/'te/'ta) ona göre seçilir. Eki sayıya bağlı olmayan diller $end_tail'i kullanmaz.
# en: $end_tail yalnız Türkçe ek seçimi için; değişken eşliği (i18n_catalog) gereği tek seçenekli seçimle kullanılır.
settings-power-charge-end = { $end_tail ->
       *[other] Charging stops at { $end }%
    }

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Şarj $end yüzdesinde durur; pil $start yüzdesinin altına düşünce yeniden şarj olmaya başlar. $end_tail / $start_tail sayıların okunuşundaki son sözcüğün değeridir; Türkçe bulunma ('de) ve ilgi ('in) ekleri ona göre seçilir. Eki sayıya bağlı olmayan diller bunları kullanmaz.
settings-power-charge-range = { $end_tail ->
       *[other] { $start_tail ->
           *[other] Charging stops at { $end }% and resumes below { $start }%
        }
    }

# Nerede: "Pil ömrünü koru" satırının alt metni. Anlam: Şarjın hangi yüzdede duracağını sistem değil pilin/aygıtın kendi yazılımı belirliyor.
settings-power-charge-firmware = Charge limit is set by the device firmware

# Nerede: Güç sayfasında satırın alt metni (hata rengi), değişiklik reddedilince. Anlam: Sistem izin vermedi; bu ayar yalnızca bilgisayarın başında oturan etkin kullanıcı tarafından değiştirilebilir.
settings-power-error-denied = Not allowed (can only be changed from the active local session)

# Nerede: "Pil ömrünü koru" satırının alt metni (hata rengi). Anlam: Pil bilgisini sağlayan UPower sistem hizmeti çalışmıyor. "UPower" ad, çevrilmez.
settings-power-error-upower-missing = UPower is not running

# Nerede: Güç sayfasında satırın alt metni (hata rengi). Anlam: Değişiklik bilinmeyen bir nedenle uygulanamadı.
settings-power-error-failed = Could not change the setting

# Nerede: Güç sayfasında, power.status bloğunun başlığı (aramada görünür). Anlam: Güç bilgisinin (pil, güç profilleri) alınamadığı durumu bildiren üst blok.
settings-power-status-label = Power info

# Nerede: Güç sayfasında, power.status bloğu için arama sözcükleri. Anlam: Boş durum bloğunu aramada bulmayı sağlar. "UPower" ve "power-profiles-daemon" ad, çevrilmez.
settings-power-status-keywords = power, upower, power-profiles-daemon

# Nerede: Güç sayfasının Pil kartında, pil durumu satırının başlığı (yalnız aramada; satırda yüzde görünür). Anlam: Pilin doluluk ve şarj durumu.
settings-power-battery-label = Battery status

# Nerede: Güç sayfasında, power.battery satırı için arama sözcükleri. Anlam: Pil durumu satırını aramada bulmayı sağlar.
settings-power-battery-keywords = battery, charge, percent, remaining time

# Nerede: Pil kartındaki durum satırının başlığı. Anlam: Pilin doluluk yüzdesi. Not: yüzde işareti Türkçede sayının önünde.
# Değişkenler: $percent (0–100 arası tam sayı, örnek: 64).
settings-power-battery-percent = { $percent }%

# Nerede: Pil durum satırının alt metni; pil şarj oluyor, dolma süresi bilinmiyor.
settings-power-battery-charging = Charging

# Nerede: Pil durum satırının alt metni; pil şarj oluyor ve dolmasına kalan süre biliniyor.
# Değişkenler: $time (biçimlenmiş süre, örnek: "35 dk", "1 sa 20 dk").
settings-power-battery-charging-time = Charging · full in { $time }

# Nerede: Pil durum satırının alt metni; bilgisayar pilden çalışıyor, kalan süre bilinmiyor.
settings-power-battery-discharging = On battery

# Nerede: Pil durum satırının alt metni; bilgisayar pilden çalışıyor ve pilin bitmesine kalan süre biliniyor.
# Değişkenler: $time (biçimlenmiş süre, örnek: "2 sa 10 dk").
settings-power-battery-discharging-time = On battery · { $time } left

# Nerede: Pil durum satırının alt metni; pil tamamen dolu.
settings-power-battery-full = Fully charged

# Nerede: Pil durum satırının alt metni; bilgisayar prize takılı ama pil şarj olmuyor (ör. şarj sınırına ulaştı).
settings-power-battery-not-charging = Plugged in, not charging

# Nerede: Pil durum satırındaki süre (kalan ya da dolma süresi). Anlam: bir saatten kısa süre, yalnız dakika.
# Değişkenler: $minutes (1–59, örnek: 35).
settings-power-duration-minutes = { $minutes } min

# Nerede: Pil durum satırındaki süre. Anlam: tam saat uzunluğunda süre.
# Değişkenler: $hours (örnek: 2).
settings-power-duration-hours = { $hours } hr

# Nerede: Pil durum satırındaki süre. Anlam: saat ve dakikalı süre.
# Değişkenler: $hours (örnek: 2), $minutes (1–59, örnek: 10).
settings-power-duration-hours-minutes = { $hours } hr { $minutes } min

# Nerede: Güç sayfasının tek içeriği (maskotlu hata durumu başlığı); pil bilgisi ve güç profilleri hizmetlerinin ikisine de ulaşılamıyor.
settings-power-empty-unavailable = Power info unavailable

# Nerede: Güç sayfasındaki hata durumunun açıklaması. Anlam: Pil için UPower, güç profilleri için power-profiles-daemon sistem hizmeti gerekir; ikisi de çalışmıyor. Hizmet adları çevrilmez.
settings-power-empty-unavailable-body = UPower is required for battery info and power-profiles-daemon for power profiles; neither is running.

# Nerede: Güç sayfasının tek içeriği (maskotlu boş durum başlığı); bilgisayarda pil yok ve güç profilleri hizmeti yok (masaüstü, sanal makine).
settings-power-empty-no-battery = No power options to configure

# Nerede: Güç sayfasındaki boş durumun açıklaması. Anlam: Pil bulunamadı; güç profilleri için power-profiles-daemon kurulup çalıştırılabilir. Hizmet adı çevrilmez.
settings-power-empty-no-battery-body = No battery found on this computer. power-profiles-daemon is required for power profiles.

# ---- Sistem › Yazıcılar (t125; docs/design/printers.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Yazıcılar ve yazdırma kuyruğu sayfasının etiketi.
# [max: 14]
settings-page-printers = Printers

# Nerede: Yazıcılar sayfasında kart başlığı. Anlam: Bu bilgisayara tanımlı yazıcıların listesi.
settings-section-printers = Printers

# Nerede: Yazıcılar sayfasında kart başlığı. Anlam: Kullanıcının henüz bitmemiş yazdırma işleri.
settings-section-printers-jobs = Print queue

# Nerede: Yazıcılar sayfasında, printers.status satırının başlığı (aramada görünür). Anlam: Yazdırma servisinin (CUPS) durumunu bildiren üst blok.
settings-printers-status-label = Printing service

# Nerede: Yazıcılar sayfasında, printers.status satırı için arama sözcükleri. Anlam: Durum bloğunu aramada bulmayı sağlar.
settings-printers-status-keywords = cups, printing service, print service

# Nerede: Yazıcılar sayfasında, printers.list satırının başlığı (aramada görünür). Anlam: Yazıcı listesi.
settings-printers-list-label = Printers

# Nerede: Yazıcılar sayfasında, printers.list satırı için arama sözcükleri. Anlam: Yazıcı listesini aramada bulmayı sağlar.
settings-printers-list-keywords = printer, default printer, test page

# Nerede: Yazıcılar kartının son satırının başlığı. Anlam: Yeni yazıcı tanımlama.
settings-printers-add-label = Add printer

# Nerede: "Yazıcı ekle" satırının alt metni. Anlam: Yazıcı ekleme ve gelişmiş yazıcı ayarlarının CUPS'un tarayıcıda açılan yönetim sayfasında yapıldığı. "CUPS" ad, çevrilmez.
settings-printers-add-description = Add printers and change advanced settings in the CUPS web interface

# Nerede: Yazıcılar sayfasında, printers.add satırı için arama sözcükleri. Anlam: "Yazıcı ekle" satırını aramada bulmayı sağlar.
settings-printers-add-keywords = add printer, new printer, cups web, administration

# Nerede: Yazıcılar sayfasında, printers.jobs satırının başlığı (aramada görünür). Anlam: Yazdırma kuyruğu listesi.
settings-printers-jobs-label = Print queue

# Nerede: Yazıcılar sayfasında, printers.jobs satırı için arama sözcükleri. Anlam: Kuyruğu aramada bulmayı sağlar.
settings-printers-jobs-keywords = queue, job, print job, cancel, pending

# Nerede: Yazıcılar sayfası, maskotlu durumun başlığı. Anlam: Yazdırma servisi (CUPS) kurulu değil ya da çalışmıyor.
settings-printers-unavailable = Printing service unavailable

# Nerede: Yazıcılar sayfası, maskotlu durumun gövdesi. Anlam: CUPS'un kurulması ve başlatılması gerektiği; paket adı "cups" ve birim adı "cups.socket" çevrilmez.
settings-printers-unavailable-body = CUPS is not installed or is not running. To use a printer, install the cups package and start the cups.socket service.

# Nerede: Yazıcılar sayfası, maskotlu durumun başlığı. Anlam: Yazdırma servisine bağlanıldı ama bilgi okunamadı.
settings-printers-failed = Could not read printer information

# Nerede: Yazıcılar sayfası, maskotlu durumun gövdesi. Anlam: Sayfanın kendiliğinden yeniden deneyeceği.
settings-printers-failed-body = The printing service returned an unexpected response. The page retries every few seconds.

# Nerede: Yazıcılar kartı, ilk okuma sürerken. Anlam: Yazıcı listesinin okunmakta olduğu.
settings-printers-reading = Reading printers…

# Nerede: Yazıcılar kartı, maskotlu boş durumun başlığı. Anlam: Tanımlı yazıcı olmadığı.
settings-printers-empty = No printers

# Nerede: Yazıcılar kartı, maskotlu boş durumun gövdesi. Anlam: Altındaki "Yazıcı ekle" satırından yazıcı eklenebileceği.
settings-printers-empty-body = You can set up a printer with "Add printer" below.

# Nerede: Yazıcı satırının alt metni. Anlam: Yazıcının boşta, yazdırmaya hazır olduğu.
settings-printers-state-idle = Ready

# Nerede: Yazıcı satırının alt metni. Anlam: Yazıcının şu an bir iş yazdırdığı.
settings-printers-state-processing = Printing

# Nerede: Yazıcı satırının alt metni (hata rengi). Anlam: Yazıcının durdurulduğu (duraklatılmış ya da hata nedeniyle).
settings-printers-state-stopped = Stopped

# Nerede: Yazıcı satırının alt metni, durumdan sonra. Anlam: Yazıcının yeni yazdırma işi kabul etmediği.
settings-printers-not-accepting = Not accepting jobs

# Nerede: Yazıcı satırının alt metninin başı. Anlam: Bu yazıcının varsayılan yazıcı olduğu.
settings-printers-default = Default

# Nerede: Yazıcı satırının alt metninin sonu, deneme sayfası gönderilince. Anlam: Deneme sayfasının yazıcıya gönderildiği.
settings-printers-test-page-sent = Test page sent

# Nerede: Yazıcı satırı açılınca ayrıntı satırının etiketi. Anlam: Yazıcının markası ve modeli.
settings-printers-detail-model = Model

# Nerede: Yazıcı satırı açılınca ayrıntı satırının etiketi. Anlam: Yazıcının bulunduğu yer (CUPS'ta girilen konum).
settings-printers-detail-location = Location

# Nerede: Yazıcı satırı açılınca ayrıntı satırının etiketi. Anlam: CUPS'taki kuyruk adı (komut satırında kullanılan ad).
settings-printers-detail-queue = Queue name

# Nerede: Yazıcı ayrıntısında düğme. Anlam: Bu yazıcıyı kullanıcının varsayılan yazıcısı yapar.
settings-printers-make-default = Set as default

# Nerede: Yazıcı ayrıntısında, yazıcı zaten varsayılanken devre dışı düğmenin etiketi. Anlam: Bu yazıcının zaten varsayılan olduğu.
settings-printers-is-default = Default printer

# Nerede: Yazıcı ayrıntısında düğme. Anlam: Yazıcıya deneme sayfası gönderir.
settings-printers-test-page = Test page

# Nerede: Yazdırma kuyruğunda deneme sayfası işinin adı. Anlam: Ayarlar'dan gönderilen deneme sayfası.
settings-printers-test-page-title = Test page

# Nerede: "Yazıcı ekle" satırındaki düğme. Anlam: CUPS web arayüzünü tarayıcıda açar.
settings-printers-open-web = Open

# Nerede: Yazdırma kuyruğu kartı, iş yokken. Anlam: Bekleyen yazdırma işi olmadığı.
settings-printers-jobs-empty = No pending print jobs

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin sırasını beklediği.
settings-printers-job-pending = Pending

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin bekletildiği (elle ya da yazıcı nedeniyle).
settings-printers-job-held = Held

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin şu an yazdırıldığı.
settings-printers-job-processing = Printing

# Nerede: Yazdırma kuyruğunda iş satırının alt metni. Anlam: İşin yazdırılırken durduğu.
settings-printers-job-stopped = Stopped

# Nerede: Yazdırma kuyruğunda iş satırının alt metni (kısa an). Anlam: İşin bittiği ya da iptal edildiği.
settings-printers-job-done = Done

# Nerede: Yazdırma kuyruğunda adı olmayan işin başlığı. Anlam: Numarasıyla anılan yazdırma işi.
# Değişkenler: $id (iş numarası, ör. 42).
settings-printers-job-untitled = Job { $id }

# Nerede: Yazdırma kuyruğunda iş satırının alt metninin sonu. Anlam: İşin boyutu (kilobayt).
# Değişkenler: $size (etkin biçimde yazılmış kilobayt sayısı, ör. "1.250").
settings-printers-job-size = { $size } KB

# Nerede: Yazdırma kuyruğunda iş satırının düğmesi. Anlam: Yazdırma işini iptal eder.
settings-printers-job-cancel = Cancel

# Nerede: Yazdırma kuyruğunda, iptal isteği sürerken düğmenin etiketi. Anlam: İşin iptal edilmekte olduğu.
settings-printers-job-cancelling = Cancelling…

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Varsayılan yazıcı ayarı kullanıcının dosyasına yazılamadı.
settings-printers-error-write = Could not save the default printer

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: İşlem yapılan yazıcı bu arada kaldırılmış.
settings-printers-error-gone = The printer no longer exists

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Yazdırma servisi bu işleme izin vermedi.
settings-printers-error-denied = The printing service denied the request

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: İşlem sırasında yazdırma servisine ulaşılamadı.
settings-printers-error-unreachable = Could not reach the printing service

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: CUPS'un deneme sayfası dosyası bulunamadı.
settings-printers-error-test-page-missing = Could not find the test page file

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Yazıcı varsayılan yapılamadı.
settings-printers-error-default = Could not set the default printer

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Deneme sayfası yazıcıya gönderilemedi.
settings-printers-error-test-page = Could not send the test page

# Nerede: Yazıcılar sayfasının üstündeki uyarı notu. Anlam: Yazdırma işi iptal edilemedi.
settings-printers-error-cancel = Could not cancel the job
# ---- Bağlantılar: Bluetooth (t115; docs/design/bluetooth-page.md) ----

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi. Anlam: Bluetooth sayfasının etiketi.
# [max: 14]
settings-page-bluetooth = Bluetooth

# Nerede: Bluetooth sayfasında kart başlığı. Anlam: Bluetooth aç/kapa, bu bilgisayarın adı ve görünürlüğü.
settings-section-bluetooth = Bluetooth

# Nerede: Bluetooth sayfasında kart başlığı. Anlam: Daha önce eşleştirilmiş cihazların listesi.
settings-section-bluetooth-paired = Paired devices

# Nerede: Bluetooth sayfasında kart başlığı. Anlam: Taramada bulunan, henüz eşleştirilmemiş cihazlar.
settings-section-bluetooth-nearby = Nearby devices

# Nerede: Bluetooth sayfasında, bluetooth.status satırının başlığı (aramada görünür). Anlam: Bluetooth hizmetinin ve adaptörün durumunu bildiren üst blok.
settings-bluetooth-status-label = Bluetooth status

# Nerede: Bluetooth sayfasında, bluetooth.status satırı için arama sözcükleri. Anlam: Durum bloğunu aramada bulmayı sağlar.
settings-bluetooth-status-keywords = bluez, adapter, bluetooth service

# Nerede: Bluetooth sayfasının Bluetooth kartında, bluetooth.enabled satırının başlığı. Anlam: Bluetooth'un açık mı kapalı mı olduğu.
settings-bluetooth-enabled-label = Bluetooth

# Nerede: Bluetooth sayfasında, bluetooth.enabled satırı için arama sözcükleri. Anlam: Aç/kapa satırını aramada bulmayı sağlar.
settings-bluetooth-enabled-keywords = bluetooth, on, off, airplane mode, rfkill

# Nerede: Bluetooth kartında, bluetooth.name satırının başlığı. Anlam: Diğer cihazların bu bilgisayarı gördüğü Bluetooth adı.
settings-bluetooth-name-label = This computer's name

# Nerede: Bluetooth sayfasında, bluetooth.name satırı için arama sözcükleri. Anlam: Ad satırını aramada bulmayı sağlar.
settings-bluetooth-name-keywords = name, adapter name, device name, alias

# Nerede: Bluetooth kartında, bluetooth.discoverable satırının başlığı. Anlam: Bu bilgisayarın yakındaki cihazlarca bulunabilir olması.
settings-bluetooth-discoverable-label = Visible

# Nerede: Bluetooth sayfasında, bluetooth.discoverable satırı için arama sözcükleri. Anlam: Görünürlük satırını aramada bulmayı sağlar.
settings-bluetooth-discoverable-keywords = visibility, discoverable, findable

# Nerede: Bluetooth sayfasında, bluetooth.paired satırının başlığı (aramada görünür). Anlam: Eşleşmiş cihazlar listesi.
settings-bluetooth-paired-label = Paired devices

# Nerede: Bluetooth sayfasında, bluetooth.paired satırı için arama sözcükleri. Anlam: Eşleşmiş cihazlar listesini aramada bulmayı sağlar.
settings-bluetooth-paired-keywords = device, headphones, keyboard, mouse, connect, forget, trusted, block, battery

# Nerede: Bluetooth sayfasında, bluetooth.nearby satırının başlığı (aramada görünür). Anlam: Yakındaki (eşleşmemiş) cihazlar listesi.
settings-bluetooth-nearby-label = Nearby devices

# Nerede: Bluetooth sayfasında, bluetooth.nearby satırı için arama sözcükleri. Anlam: Yakındaki cihazlar listesini aramada bulmayı sağlar.
settings-bluetooth-nearby-keywords = scan, pair, new device, search

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Ayarlar'ın Bluetooth hizmeti istemcisi başlatılamadı.
settings-bluetooth-failed = Could not connect to the Bluetooth service

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: Ayarlar'ı yeniden açmayı öneren açıklama.
settings-bluetooth-failed-body = Try closing and reopening Settings.

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Sistem ileti yoluna (D-Bus sistem bus'ı) ulaşılamıyor.
settings-bluetooth-unreachable = System services cannot be reached

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: Bağlantı geri gelince sayfanın kendiliğinden yenileneceği.
settings-bluetooth-unreachable-body = This page updates automatically when the connection is restored.

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Bluetooth sistem hizmeti (BlueZ) çalışmıyor. "BlueZ" ad, çevrilmez.
settings-bluetooth-missing = BlueZ is not running

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: Bluetooth hizmetinin kurulup başlatılması gerektiği; "bluetooth.service" sistem hizmetinin adı, çevrilmez.
settings-bluetooth-missing-body = Bluetooth needs the bluez package installed and bluetooth.service running.

# Nerede: Bluetooth sayfası, maskotlu hata durumunun başlığı. Anlam: Bilgisayarda Bluetooth donanımı (adaptör) bulunamadı.
settings-bluetooth-no-adapter = No Bluetooth adapter

# Nerede: Bluetooth sayfası, maskotlu hata durumunun gövdesi. Anlam: USB Bluetooth adaptörü takılınca sayfanın kendiliğinden güncelleneceği.
settings-bluetooth-no-adapter-body = If you plug in a Bluetooth adapter, it will show up here.

# Nerede: Bluetooth sayfasının üst notu, bir işlem başarısız olunca. Anlam: $detail BlueZ'in (çevrilmemiş) hata metni.
# Değişkenler: $detail (BlueZ hata metni).
settings-bluetooth-error-call-failed = Action failed: { $detail }

# Nerede: Bluetooth sayfasının üst notu. Anlam: İşlem zaman aşımına uğradı.
settings-bluetooth-error-timeout = The action timed out

# Nerede: Bluetooth sayfasının üst notu. Anlam: Bluetooth adaptörü bulunamadı.
settings-bluetooth-error-no-adapter = No Bluetooth adapter found

# Nerede: Bluetooth sayfasının üst notu. Anlam: Bluetooth hizmeti (BlueZ) çalışmıyor.
settings-bluetooth-error-unavailable = BlueZ is not running

# Nerede: Bluetooth sayfasının üst notu. Anlam: İşlem yapılan cihaz artık listede yok.
settings-bluetooth-error-unknown-device = The device is no longer visible

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi), eşleştirme bitince. Anlam: Eşleştirme kabukta ya da cihazda reddedildi.
settings-bluetooth-error-auth-rejected = Pairing was rejected

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Eşleştirme başarısız (yanlış kod ya da onay istenemedi — onay penceresini kabuk açar).
settings-bluetooth-error-auth-failed = Pairing failed; wrong code or no confirmation possible (is the shell running?)

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Eşleştirme cihaz tarafında iptal edildi.
settings-bluetooth-error-auth-canceled = Pairing was canceled

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Onay zamanında verilmedi.
settings-bluetooth-error-auth-timeout = Confirmation was not given in time

# Nerede: Bluetooth sayfası (not ya da satır alt metni). Anlam: Cihaza ulaşılamadı; kapalı ya da uzakta olabilir.
settings-bluetooth-error-connection-failed = Could not reach the device; is it on and nearby?

# Nerede: Yakındaki cihaz satırının alt metni (hata rengi). Anlam: Cihaz zaten eşleşmiş.
settings-bluetooth-error-already-exists = This device is already paired

# Nerede: Bluetooth sayfasının üst notu. Anlam: Aynı işlem zaten sürüyor.
settings-bluetooth-error-in-progress = This action is already in progress

# Nerede: Bluetooth sayfasının üst notu. Anlam: Adaptör hazır değil (çoğunlukla kapalı).
settings-bluetooth-error-not-ready = Bluetooth is not ready; turn it on first

# Nerede: Bluetooth sayfasının üst notu. Anlam: Girilen değer (ör. ad) geçersiz ya da çok uzun.
settings-bluetooth-error-invalid-value = The value is invalid or too long

# Nerede: Bluetooth sayfasının üst notu, açma başarısız olunca. Anlam: Bluetooth fiziksel bir anahtarla kapatılmış; yazılımla açılamaz.
settings-bluetooth-error-rfkill-hard = Bluetooth is turned off by a hardware switch; use the switch on the device to turn it on

# Nerede: Bluetooth sayfasının üst notu, açma başarısız olunca. Anlam: Bluetooth uçak kipi (yazılım engeli) nedeniyle açılamadı.
settings-bluetooth-error-rfkill = Airplane mode is on; Bluetooth could not be turned on

# Nerede: Bluetooth satırının alt metni. Anlam: Bluetooth açık.
settings-bluetooth-power-on = On

# Nerede: Bluetooth satırının alt metni. Anlam: Bluetooth kapalı.
settings-bluetooth-power-off = Off

# Nerede: Bluetooth satırının alt metni. Anlam: Bluetooth açılıyor ya da kapanıyor.
settings-bluetooth-power-turning = Switching…

# Nerede: Bluetooth satırının alt metni. Anlam: Uçak kipi (yazılım engeli) Bluetooth'u kapatmış.
settings-bluetooth-power-soft-blocked = Airplane mode is on

# Nerede: Bluetooth satırının alt metni. Anlam: Bilgisayardaki fiziksel anahtar Bluetooth'u kapatmış.
settings-bluetooth-power-hard-blocked = Off by hardware switch

# Nerede: "Bu bilgisayarın adı" satırının alt metni. Anlam: Bu adın yakındaki cihazlarda göründüğü.
settings-bluetooth-name-hint = Other devices see this computer by this name

# Nerede: "Görünür" satırının alt metni. Anlam: Bu bilgisayar şu an yakındaki cihazlarca bulunamaz.
settings-bluetooth-discoverable-off = Nearby devices can't find this computer

# Nerede: "Görünür" satırının alt metni, görünürken. Anlam: Görünürlüğün kendiliğinden kapanmasına kalan süre.
# Değişkenler: $time (dakika:saniye, ör. 2:05).
settings-bluetooth-discoverable-remaining = Visible; turns off in { $time }

# Nerede: "Görünür" satırının alt metni, süresiz görünürken. Anlam: Yakındaki cihazlar bu bilgisayarı bulabilir.
settings-bluetooth-discoverable-on = Nearby devices can find this computer

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihazın sinyali güçlü (yakın).
settings-bluetooth-signal-strong = Strong signal

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihazın sinyali orta.
settings-bluetooth-signal-medium = Medium signal

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihazın sinyali zayıf (uzak).
settings-bluetooth-signal-weak = Weak signal

# Nerede: Yakındaki cihaz satırının alt metni. Anlam: Cihaz son taramada duyulmadı.
settings-bluetooth-signal-none = Not heard right now

# Nerede: Eşleşmiş cihaz satırının alt metni. Anlam: Cihaz bağlı.
settings-bluetooth-connected = Connected

# Nerede: Eşleşmiş cihaz satırının alt metni. Anlam: Cihaz eşleşmiş ama bağlı değil.
settings-bluetooth-not-connected = Not connected

# Nerede: Eşleşmiş cihaz satırının alt metni, durumdan sonra. Anlam: Cihaz engellendi (bağlanamaz).
settings-bluetooth-blocked = Blocked

# Nerede: Yakındaki cihaz satırının alt metni, eşleştirme sürerken. Anlam: PIN/onayın kabuğun açtığı pencerede verilmesi gerektiği.
settings-bluetooth-pairing-wait = Pairing; confirm in the shell window

# Nerede: Yakındaki cihaz satırının alt metni, eşleştirme sürerken masaüstü kabuğunun eşleştirme ajanı yokken. Anlam: PIN/onay penceresini kabuk açar; kabuk çalışmıyorsa onay istenen cihazlar eşleşemez (onaysız cihazlar yine eşleşebilir).
settings-bluetooth-pairing-no-agent = The desktop shell must be running to confirm pairing

# Nerede: Bluetooth sayfası kartları, ilk durum okunurken. Anlam: Bilgiler okunuyor.
settings-bluetooth-reading = Reading…

# Nerede: Eşleşmiş cihazlar kartı, maskotlu boş durumun başlığı. Anlam: Hiç eşleşmiş cihaz yok.
settings-bluetooth-paired-empty = No paired devices

# Nerede: Eşleşmiş cihazlar kartı, maskotlu boş durumun gövdesi. Anlam: Yakındaki cihazlardan birini eşleştirmeyi önerir.
settings-bluetooth-paired-empty-body = Pair one of the nearby devices below.

# Nerede: Yakındaki cihazlar kartı, Bluetooth kapalıyken. Anlam: Kapalıyken tarama yapılamadığı.
settings-bluetooth-nearby-off = Nearby devices aren't shown while Bluetooth is off.

# Nerede: Yakındaki cihazlar kartı, tarama bitti ve cihaz yokken. Anlam: Çevrede eşleştirilecek cihaz bulunamadı.
settings-bluetooth-nearby-empty = No devices found nearby.

# Nerede: Yakındaki cihazlar kartının üst satırı, tarama sürerken. Anlam: Cihaz aranıyor.
settings-bluetooth-scanning = Searching…

# Nerede: Yakındaki cihazlar kartının üst satırı, tarama durunca. Anlam: Tarama bitti.
settings-bluetooth-scan-stopped = Search stopped

# Nerede: Yakındaki cihazlar kartının üst satırındaki düğme. Anlam: Cihaz aramayı yeniden başlatır.
settings-bluetooth-rescan = Search again

# Nerede: Yakındaki cihaz satırındaki düğme. Anlam: Bu cihazla eşleştirmeyi başlatır.
settings-bluetooth-pair = Pair

# Nerede: Yakındaki cihaz satırındaki düğme, eşleştirme sürerken. Anlam: Süren eşleştirmeyi iptal eder.
settings-bluetooth-cancel = Cancel

# Nerede: Yakındaki cihazlar kartının alt satırı. Anlam: Adı bilinmeyen kaç cihazın gizlendiği.
# Değişkenler: $count (gizlenen cihaz sayısı, ör. 3).
settings-bluetooth-unnamed-hidden =
    { $count ->
        [one] { $count } unnamed device hidden
       *[other] { $count } unnamed devices hidden
    }

# Nerede: Yakındaki cihazlar kartının alt satırındaki düğme. Anlam: Adsız cihazları da listeler.
settings-bluetooth-unnamed-show = Show

# Nerede: Yakındaki cihazlar kartının alt satırı. Anlam: Adı bilinmeyen kaç cihazın listede gösterildiği.
# Değişkenler: $count (gösterilen adsız cihaz sayısı, ör. 3).
settings-bluetooth-unnamed-shown =
    { $count ->
        [one] { $count } unnamed device shown
       *[other] { $count } unnamed devices shown
    }

# Nerede: Yakındaki cihazlar kartının alt satırındaki düğme. Anlam: Adsız cihazları yeniden gizler.
settings-bluetooth-unnamed-hide = Hide

# Nerede: Eşleşmiş cihazın açılan ayrıntısında, ad alanının etiketi. Anlam: Cihazın bu bilgisayarda görünen adı (yeniden adlandırma).
settings-bluetooth-rename = Name

# Nerede: Eşleşmiş cihazın ayrıntısında anahtar başlığı. Anlam: Cihaz onay sormadan bağlanabilir.
settings-bluetooth-trusted = Trusted

# Nerede: "Güvenilir" anahtarının alt metni. Anlam: Güvenilir cihazın bağlanırken onay istemediği.
settings-bluetooth-trusted-hint = Can connect without asking

# Nerede: Eşleşmiş cihazın ayrıntısında anahtar başlığı. Anlam: Cihazı engeller (bağlanamaz).
settings-bluetooth-block = Block

# Nerede: "Engelle" anahtarının alt metni. Anlam: Engelli cihazın bağlanamayacağı.
settings-bluetooth-block-hint = This device can't connect

# Nerede: Eşleşmiş cihazın ayrıntısındaki düğme. Anlam: Cihaza bağlanır.
settings-bluetooth-connect = Connect

# Nerede: Eşleşmiş cihazın ayrıntısındaki düğme, bağlıyken. Anlam: Cihazla bağlantıyı keser.
settings-bluetooth-disconnect = Disconnect

# Nerede: Eşleşmiş cihazın ayrıntısındaki düğme. Anlam: Eşleşmeyi siler (cihaz yeniden eşleştirilmeli).
settings-bluetooth-forget = Forget

# Nerede: "Unut" düğmesine bir kez basılınca düğmenin etiketi. Anlam: Onay için yeniden basılması gerektiği.
settings-bluetooth-forget-confirm = Forget · press again
## Yazı tipleri ve tipografi

# Nerede: Yazı sayfası, Eş aralıklı rolünün alt yazısı (src/typography.rs). Anlam: Rolün nerede kullanıldığını söyler.
settings-typography-mono-subtitle = Code, terminal output, numbers

# Nerede: Yazı sayfası, İkon rolünün alt yazısı (src/typography.rs). Anlam: Rolün ligatürlü sembol fontu olduğunu söyler.
settings-typography-icon-subtitle = Symbol font (with ligatures)

# Nerede: Yazı sayfası, önizleme kartında Arayüz rolünün kendi fontuyla çizilen örnek satırı (src/typography.rs). Anlam: Arayüz fontuyla çizilen örnek metin.
settings-typography-sample-ui = Network connection · Volume 60%

# Nerede: Yazı sayfası, önizleme kartında Belge rolünün kendi fontuyla çizilen örnek satırı (src/typography.rs). Anlam: Belge fontuyla çizilen örnek cümle (Türkçe pangram).
settings-typography-sample-document = The quick brown fox jumps over the lazy dog.

# Nerede: Yazı sayfası, font seçicinin başlık satırındaki örnek; metin rollerinde ailenin kendi fontuyla çizilir (src/pages/fonts.rs). Anlam: Türkçe harfleri (Ğ, ğ, İ, ı) sınamaya yarayan örnek metin.
settings-fonts-sample-text = On Air Éé Ññ Üü 0123

## Bildirim uygulamaları, bar önizlemesi ve küçük sayfalar

# Nerede: Paneller sayfası, panel kartındaki "Bar'dan açma" satırı; yatay barda paneli sol uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-left-left-click = Widget at the left end{ $widget } · left click

# Nerede: Aynı satır; yatay barda paneli sol uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-left-right-click = Widget at the left end{ $widget } · right click

# Nerede: Aynı satır; yatay barda paneli sağ uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-right-left-click = Widget at the right end{ $widget } · left click

# Nerede: Aynı satır; yatay barda paneli sağ uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-right-right-click = Widget at the right end{ $widget } · right click

# Nerede: Aynı satır; dikey barda paneli üst uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-top-left-click = Widget at the top end{ $widget } · left click

# Nerede: Aynı satır; dikey barda paneli üst uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-top-right-click = Widget at the top end{ $widget } · right click

# Nerede: Aynı satır; dikey barda paneli alt uçtaki widget'ın sol tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-bottom-left-click = Widget at the bottom end{ $widget } · left click

# Nerede: Aynı satır; dikey barda paneli alt uçtaki widget'ın sağ tıkı açıyorsa (src/pages/panel_hint.rs). Anlam: Panelin hangi uçtaki widget'a hangi tıkla açıldığı.
# Değişkenler: $widget (parantezli widget adı, ör. " (Oturum)"; uçta widget yoksa boş).
settings-panel-hint-bottom-right-click = Widget at the bottom end{ $widget } · right click

# Nerede: Bar sayfası, bar önizlemesinin altındaki ipucu satırı; barda widget varken (src/pages/bar_preview.rs). Anlam: Önizlemede bir widget'ın seçilince ayarlarının aşağıda açıldığı.
settings-bar-preview-hint = Click a widget to open its settings below

# Nerede: Bar sayfası, Seçili widget kartındaki yer satırı; seçili widget barın hiçbir bölümünde değilken (src/pages/bar_preview.rs). Anlam: Widget'ın barda olmadığı, Widget'lar kartından eklenebileceği.
settings-bar-preview-location-not-in-bar = Not in the bar · Add it from the Widgets card

# Nerede: Bar sayfası, Seçili widget kartındaki yer satırı; widget yatay barın baş ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın sol ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-left-end = Left end of the bar · Click and wheel can also be set

# Nerede: Aynı yer satırı; widget yatay barın son ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın sağ ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-right-end = Right end of the bar · Click and wheel can also be set

# Nerede: Aynı yer satırı; widget dikey barın baş ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın üst ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-top-end = Top end of the bar · Click and wheel can also be set

# Nerede: Aynı yer satırı; widget dikey barın son ucundaysa (src/pages/bar_preview.rs). Anlam: Widget'ın barın alt ucunda olduğu; tıklama ve tekerlek eylemlerinin de ayarlanabildiği.
settings-bar-preview-location-bottom-end = Bottom end of the bar · Click and wheel can also be set

# Nerede: Aynı yer satırı; widget baş bölümünde ama uçta değilken, bar yatayken (src/pages/bar_preview.rs). Anlam: Widget'ın barın sol tarafında olduğu.
settings-bar-preview-location-left = Left side of the bar

# Nerede: Aynı yer satırı; widget baş bölümünde ama uçta değilken, bar dikeyken (src/pages/bar_preview.rs). Anlam: Widget'ın barın üst tarafında olduğu.
settings-bar-preview-location-top = Top side of the bar

# Nerede: Aynı yer satırı; widget orta bölümdeyken (src/pages/bar_preview.rs). Anlam: Widget'ın barın ortasında olduğu.
settings-bar-preview-location-center = Middle of the bar

# Nerede: Aynı yer satırı; widget son bölümde ama uçta değilken, bar yatayken (src/pages/bar_preview.rs). Anlam: Widget'ın barın sağ tarafında olduğu.
settings-bar-preview-location-right = Right side of the bar

# Nerede: Aynı yer satırı; widget son bölümde ama uçta değilken, bar dikeyken (src/pages/bar_preview.rs). Anlam: Widget'ın barın alt tarafında olduğu.
settings-bar-preview-location-bottom = Bottom side of the bar

# Nerede: Bar sayfası, Seçili widget kartının başlığı; barda hiç widget yokken (src/pages/bar_preview.rs). Anlam: Barın boş olduğu.
settings-bar-preview-empty-title = No widgets in the bar

# Nerede: Bar sayfası, Seçili widget kartının alt yazısı; barda hiç widget yokken (src/pages/bar_preview.rs). Anlam: Widget'ların Widget'lar kartından eklenebileceği.
settings-bar-preview-empty-place = Can be added from the Widgets card

# Nerede: Bar sayfası, Seçili widget kartındaki yer satırının sonu; widget uçta değil ve hiçbir ayarı yokken (src/pages/bar_preview.rs). Anlam: Yer bilgisine widget'ın ayarı olmadığı bilgisinin eklendiği.
# Değişkenler: $place (widget'ın bar'daki yeri, ör. "Bar'ın ortasında").
settings-bar-preview-place-no-settings = { $place } · This widget has no settings
## Servis durum ve hata metinleri

# Nerede: Görünüm sayfasındaki durum kartı; seçimler shell tarafından uygulamalara aktarılırken (src/services/look.rs). Anlam: Görünümün uygulamalara aktarılmakta olduğu.
settings-look-status-applying = Applying…

# Nerede: Görünüm sayfasındaki durum kartı; seçimler tüm uygulamalara aktarıldığında (src/services/look.rs). Anlam: Görünümün uygulandığı.
settings-look-status-applied = Applied.

# Nerede: Görünüm sayfasındaki durum kartında eksik paket cümlenin sonunda; AUR paketi var ama paru ya da yay kurulu değilken (src/services/look.rs). Anlam: AUR paketini kurmak için bir yardımcının gerektiği.
settings-look-status-aur-helper-note = An AUR helper is required (paru or yay).

# Nerede: Ayar kartındaki geri yükleme sonucunun hata parantezi içinde; kabuğun bildirdiği hata türü okunamadığında (src/services/snapshots.rs). Anlam: settings-snapshots-outcome-error içinde $kind olarak görünür.
settings-snapshots-error-unknown-kind = unknown
## Erişilebilirlik (Kişisel grubu; büyüteç magnifier-v1, ekran klavyesi osk-v1)

# Nerede: Ayarlar penceresi, kenar çubuğundaki sayfa listesi (Kişisel grubu). Anlam: Görme, büyüteç ve ekran klavyesi ayarlarını toplayan sayfanın adı.
settings-page-accessibility = Accessibility

# Nerede: Erişilebilirlik sayfasının ilk bölüm başlığı (src/registry.rs). Anlam: Görmeyi kolaylaştıran ayarlar (yüksek karşıtlık, imleç ve yazı boyutu, hareketi azalt).
settings-section-seeing = Seeing

# Nerede: Erişilebilirlik › Büyüteç kartındaki aç/kapa satırının başlığı (src/pages/assist.rs). Anlam: Ekran büyüteci.
settings-magnifier-live-label = Magnifier

# Nerede: Erişilebilirlik › Büyüteç satırının alt metni; durum henüz okunmadan görünür. Anlam: Büyütecin ne yaptığı.
settings-magnifier-live-description = Magnifies the screen around the cursor

# Nerede: Ayarlar araması; Büyüteç satırının arama sözcükleri (virgülle ayrılır). Anlam: Kullanıcının bu ayarı bulmak için yazabileceği sözcükler.
settings-magnifier-live-keywords = magnifier, zoom, magnify, vision, enlarge

# Nerede: Erişilebilirlik › Büyüteç kartında kaydırıcı başlığı. Anlam: Büyüteç açıldığında ekranın kaç kat büyütüleceği.
settings-magnifier-factor-label = Starting zoom

# Nerede: Açılış büyütmesi satırının alt metni (kabuk açıklama vermezse). Anlam: Büyüteç açılınca kullanılan büyütme oranı.
settings-magnifier-factor-description = Zoom used when the magnifier opens

# Nerede: Ayarlar araması; Açılış büyütmesi satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-magnifier-factor-keywords = magnifier, factor, zoom, multiplier, scale

# Nerede: Erişilebilirlik › Büyüteç kartında kaydırıcı başlığı. Anlam: Her yakınlaştırma ya da uzaklaştırma adımının büyüklüğü.
settings-magnifier-step-label = Step

# Nerede: Adım satırının alt metni (kabuk açıklama vermezse). Anlam: Her yakınlaştırmada büyütmenin bu sayıyla çarpıldığı.
settings-magnifier-step-description = Each zoom multiplies the magnification by this number

# Nerede: Ayarlar araması; Adım satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-magnifier-step-keywords = magnifier, step, zoom, increment

# Nerede: Erişilebilirlik › Büyüteç kartında kaydırıcı başlığı. Anlam: Yakınlaştırmanın çıkabileceği en büyük oran.
settings-magnifier-max-label = Maximum zoom

# Nerede: En fazla büyütme satırının alt metni. Anlam: Yakınlaştırmanın bu sınırda durduğu.
settings-magnifier-max-description = Zooming stops at this limit

# Nerede: Ayarlar araması; En fazla büyütme satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-magnifier-max-keywords = magnifier, limit, zoom, maximum, max

# Nerede: Erişilebilirlik › Büyüteç kartında, Kısayollar sayfasına yönlendiren satırın başlığı; alt metinde tuşlar görünür. Anlam: Büyütecin klavye kısayolları.
settings-magnifier-keys-label = Shortcuts

# Nerede: Ayarlar araması; büyüteç kısayolları yönlendirme satırının arama sözcükleri. Anlam: Bu satırı bulduran sözcükler.
settings-magnifier-keys-keywords = magnifier, shortcut, key, zoom, keys

# Nerede: Erişilebilirlik › Ekran klavyesi kartındaki göster/gizle satırının başlığı (src/pages/assist.rs). Anlam: Ekranda çizilen sanal klavye.
settings-osk-live-label = On-screen keyboard

# Nerede: Ekran klavyesi satırının alt metni; durum henüz okunmadan görünür. Anlam: Ekran klavyesinin ne için olduğu.
settings-osk-live-description = For typing with mouse or touch

# Nerede: Ayarlar araması; Ekran klavyesi satırının arama sözcükleri. Anlam: Kullanıcının bu ayarı bulmak için yazabileceği sözcükler.
settings-osk-live-keywords = on-screen keyboard, virtual keyboard, touchscreen, osk

# Nerede: Erişilebilirlik › Ekran klavyesi kartında düzen seçimi satırının başlığı. Anlam: Ekran klavyesinin tuş düzeni.
settings-osk-layout-label = Layout

# Nerede: Düzen satırının alt metni (kabuk açıklama vermezse). Anlam: Otomatik seçeneğin masaüstündeki klavye düzenini izlediği.
settings-osk-layout-description = Automatic: follows the desktop keyboard layout

# Nerede: Ayarlar araması; ekran klavyesi Düzen satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-osk-layout-keywords = on-screen keyboard, layout, turkish q, language

# Nerede: Erişilebilirlik › Ekran klavyesi kartında kaydırıcı başlığı. Anlam: Ekran klavyesindeki tuşların yüksekliği.
settings-osk-key-height-label = Key height

# Nerede: Tuş yüksekliği satırının alt metni. Anlam: Klavyenin boyunun ve genişliğinin tuş yüksekliğinden hesaplandığı.
settings-osk-key-height-description = The keyboard's height and width are calculated from this

# Nerede: Ayarlar araması; Tuş yüksekliği satırının arama sözcükleri. Anlam: Bu ayarı bulduran sözcükler.
settings-osk-key-height-keywords = on-screen keyboard, size, key

# Nerede: Erişilebilirlik › Ekran klavyesi kartında, Kısayollar sayfasına yönlendiren satırın başlığı; alt metinde tuşlar görünür. Anlam: Ekran klavyesini açıp kapatan kısayol.
settings-osk-keys-label = Shortcut

# Nerede: Ayarlar araması; ekran klavyesi kısayolu yönlendirme satırının arama sözcükleri. Anlam: Bu satırı bulduran sözcükler.
settings-osk-keys-keywords = on-screen keyboard, shortcut, key

# Nerede: Ekran klavyesi › Düzen seçeneği (auto). Anlam: Masaüstünde etkin klavye düzenini izler.
settings-choice-osk-layout-auto = Automatic

# Nerede: Ekran klavyesi › Düzen seçeneği (tr); ayrıca açıkken satırın alt metninde. Anlam: Türkçe Q klavye düzeni.
settings-choice-osk-layout-tr = Turkish Q

# Nerede: Ekran klavyesi › Düzen seçeneği (us); ayrıca açıkken satırın alt metninde. Anlam: ABD İngilizce klavye düzeni.
settings-choice-osk-layout-us = English (US)

# Nerede: Erişilebilirlik › Büyüteç satırının alt metni, büyüteç açıkken (src/pages/assist.rs). Anlam: Büyütecin açık olduğu ve şu anki büyütme oranı.
# Değişkenler: $factor (biçimlenmiş oran, ör. 2,5).
settings-assist-magnifier-on = On · { $factor }× zoom

# Nerede: Erişilebilirlik › Ekran klavyesi satırının alt metni, klavye açıkken (src/pages/assist.rs). Anlam: Klavyenin açık olduğu ve kullandığı düzen.
# Değişkenler: $layout (düzen adı, ör. Türkçe Q).
settings-assist-osk-on = On · { $layout }

# Nerede: Erişilebilirlik › Ekran klavyesi satırının alt metni, düzen bilinmiyorken açık durumda. Anlam: Açık.
settings-assist-on = On

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni, kapalıyken. Anlam: Kapalı.
settings-assist-off = Off

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni; kabuk aracın bu masaüstünde çalışmadığını bildirince (anahtar devre dışı). Anlam: Bu masaüstü bu aracı desteklemiyor.
settings-assist-unsupported = Not available on this desktop

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni; masaüstü kabuğu yanıt vermeyince (anahtar devre dışı). Anlam: Nuxut masaüstü kabuğu çalışmıyor.
settings-assist-no-shell = Desktop shell is not running

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni; açma/kapama isteği gönderilmiş, yanıt bekleniyor. Anlam: İşlem sürüyor.
settings-assist-waiting = Waiting…

# Nerede: Erişilebilirlik › Büyüteç ve Ekran klavyesi satırlarının alt metni (hata renginde); kabuk açma/kapama isteğini yerine getiremedi. Anlam: Değişiklik yapılamadı.
settings-assist-failed = Could not be changed

# Nerede: Ekran okuyucu (Orca) — başlık çubuğundaki geri al düğmesinin adı; görünmez, sesli okunur. Anlam: son ayar değişikliğini geri alır.
settings-a11y-undo = Undo

# Nerede: Ekran okuyucu — başlık çubuğundaki geçmiş düğmesinin adı; görünmez, sesli okunur. Anlam: son değişiklikler listesini açar.
settings-a11y-history = Recent changes

# Nerede: Ekran okuyucu — başlık çubuğundaki pencereyi küçült düğmesi; görünmez, sesli okunur.
settings-a11y-minimize = Minimize

# Nerede: Ekran okuyucu — başlık çubuğundaki pencereyi büyüt/eski boyutuna getir düğmesi; görünmez, sesli okunur.
settings-a11y-maximize = Maximize

# Nerede: Ekran okuyucu — başlık çubuğundaki pencereyi kapat düğmesi; görünmez, sesli okunur.
settings-a11y-close = Close

# Nerede: Ekran okuyucu — kenar çubuğundaki ayar arama alanının adı; görünmez, sesli okunur.
settings-a11y-search = Search settings

# Nerede: Ekran okuyucu — kenar çubuğundaki sayfa listesinin adı; görünmez, sesli okunur.
settings-a11y-nav = Pages

# Nerede: Ekran okuyucu — kenar çubuğundaki arama sonuçları listesinin adı; görünmez, sesli okunur.
settings-a11y-results = Search results

# Nerede: Ekran okuyucu — ayar satırındaki varsayılana döndür düğmesinin adı; görünmez, sesli okunur. Anlam: bu ayarı varsayılan değerine sıfırlar.
# Değişkenler: $setting (ayarın görünen adı, örnek: "Bar yüksekliği").
settings-a11y-reset = { $setting }: Reset to default

## Güç: Kilit ve boşta

# Nerede: Güç sayfasında kart başlığı. Anlam: Ekran kilidiyle ilgili ayarlar (uykudan önce kilit, hangi kilit ekranı).
settings-section-lock = Lock

# Nerede: Güç sayfasında kart başlığı. Anlam: Bilgisayar bir süre kullanılmayınca yapılacaklar (karart, ekranı kapat, uyu).
settings-section-idle = When idle

# Nerede: Güç › Boşta kalınca kartının ana anahtarı (idle.enabled). Anlam: Kapalıyken Nuxut hiçbir boşta eylemini yapmaz, uykudan önce de kilitlemez.
settings-idle-enabled-label = Let Nuxut handle idle actions

# Nerede: idle.enabled satırının alt yazısı. Anlam: Kapalıyken uykudan önce kilit dahil hiçbir eylemi Nuxut yapmaz; başka bir araç (ör. hypridle) yönetebilir.
settings-idle-enabled-description = When off, lock before sleep doesn't work either; another tool can handle them

# Nerede: idle.enabled satırının arama sözcükleri. Anlam: Ana anahtarı aramada bulmayı sağlar.
settings-idle-enabled-keywords = idle, lock, hypridle, sleep, automatic

# Nerede: Güç › Kilit kartı, idle.lock_before_sleep anahtarı. Anlam: Bilgisayar uyumadan önce oturum kilitlenir.
settings-idle-lock-before-sleep-label = Lock before sleep

# Nerede: idle.lock_before_sleep satırının alt yazısı. Anlam: Uyandığında masaüstü kilitli gelir.
settings-idle-lock-before-sleep-description = The desktop is locked when the computer wakes up

# Nerede: idle.lock_before_sleep satırının arama sözcükleri.
settings-idle-lock-before-sleep-keywords = lock, sleep, suspend, lid, security

# Nerede: idle.lock_before_sleep satırının altında, uyarı renginde; anahtar açık ama "Boşta eylemlerini Nuxut yönetsin" kapalı. Anlam: Ana anahtar kapalı olduğu için uykudan önce kilit çalışmıyor.
settings-idle-lock-before-sleep-off-note = Not locking before sleep because idle actions are off

# Nerede: Güç › Kilit kartı, lock.listen_logind anahtarı. Anlam: Kilit istendiğinde Nuxut'un kendi kilit ekranı açılır.
settings-lock-listen-logind-label = Use the Nuxut lock screen

# Nerede: lock.listen_logind satırının alt yazısı (kabuk açıklama vermezse). Anlam: Kilitle düğmesi, kısayol ve otomatik kilit Nuxut kilit ekranını açar; başka bir kilit programı kullanılıyorsa kapatılmalı.
settings-lock-listen-logind-description = The Lock button, its shortcut and automatic lock open the Nuxut lock screen; turn off when using another locker

# Nerede: lock.listen_logind satırının arama sözcükleri.
settings-lock-listen-logind-keywords = lock screen, locker, hyprlock, swaylock, loginctl, logind

# Nerede: Güç › Boşta kalınca kartı, idle.lock anahtarı. Anlam: Bilgisayar bir süre kullanılmayınca oturum kilitlenir.
settings-idle-lock-label = Lock when idle

# Nerede: idle.lock satırının alt yazısı. Anlam: Belirlenen süre boşta kalınca oturum kilitlenir.
settings-idle-lock-description = The session locks after the set idle time

# Nerede: idle.lock satırının arama sözcükleri.
settings-idle-lock-keywords = lock, idle, automatic lock, screen lock

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.lock_after). Anlam: Kaç dakika boşta kalınca kilitleneceği.
settings-idle-lock-after-label = Lock delay

# Nerede: idle.lock_after satırının arama sözcükleri.
settings-idle-lock-after-keywords = lock, delay, idle

# Nerede: Güç › Boşta kalınca kartı, idle.dim anahtarı. Anlam: Boşta kalınca ekran karartılır.
settings-idle-dim-label = Dim screen

# Nerede: idle.dim satırının arama sözcükleri.
settings-idle-dim-keywords = dim, idle, screen, brightness

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.dim_after). Anlam: Kaç dakika sonra karartılacağı.
settings-idle-dim-after-label = Dim delay

# Nerede: idle.dim_after satırının arama sözcükleri.
settings-idle-dim-after-keywords = dim, delay, idle

# Nerede: Güç › Boşta kalınca kartı, idle.screen_off anahtarı. Anlam: Boşta kalınca monitörler kapatılır.
settings-idle-screen-off-label = Turn off screen

# Nerede: idle.screen_off satırının arama sözcükleri.
settings-idle-screen-off-keywords = screen off, idle, monitor, dpms, blank screen

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.screen_off_after). Anlam: Kaç dakika sonra ekranın kapatılacağı.
settings-idle-screen-off-after-label = Screen off delay

# Nerede: idle.screen_off_after satırının arama sözcükleri.
settings-idle-screen-off-after-keywords = screen off, delay, idle

# Nerede: Güç › Boşta kalınca kartı, idle.suspend anahtarı. Anlam: Boşta kalınca bilgisayar uyku kipine geçer.
settings-idle-suspend-label = Suspend

# Nerede: idle.suspend satırının alt yazısı. Anlam: Varsayılan olarak yalnızca pille çalışırken uyunur.
settings-idle-suspend-description = On battery only by default

# Nerede: idle.suspend satırının arama sözcükleri.
settings-idle-suspend-keywords = sleep, suspend, idle, automatic suspend

# Nerede: Güç › Boşta kalınca kartı, kaydırıcı (idle.suspend_after). Anlam: Kaç dakika sonra uyunacağı.
settings-idle-suspend-after-label = Suspend delay

# Nerede: idle.suspend_after satırının arama sözcükleri.
settings-idle-suspend-after-keywords = sleep, suspend, delay, idle

# Nerede: Güç › Boşta kalınca kartı, idle.suspend_on_ac anahtarı. Anlam: Bilgisayar prize takılıyken de boşta uyusun.
settings-idle-suspend-on-ac-label = Suspend on AC too

# Nerede: idle.suspend_on_ac satırının arama sözcükleri.
settings-idle-suspend-on-ac-keywords = sleep, suspend, plugged in, charger, ac

# Nerede: Güç › Boşta kalınca kartı, idle.media_inhibits anahtarı. Anlam: Müzik ya da video çalarken boşta eylemleri beklesin.
settings-idle-media-inhibits-label = Wait while media plays

# Nerede: idle.media_inhibits satırının alt yazısı. Anlam: Müzik ya da video çalarken ekran kararmaz ve kilitlenmez.
settings-idle-media-inhibits-description = The screen won't dim or lock while music or video is playing

# Nerede: idle.media_inhibits satırının arama sözcükleri.
settings-idle-media-inhibits-keywords = media, music, video, movie, idle, inhibit

## Renk teması kartları (src/pages/color_theme.rs; framework `[colors] theme` adları)

# Nerede: Görünüm › Renkler, Renk teması kartı. Anlam: varsayılan serin mavi tema.
settings-color-theme-blue = Nuxut blue
# Nerede: Aynı kartlar. Anlam: sıcak taş/kum grisi, zeytin dokunuşlu tema.
settings-color-theme-neutral = Neutral
# Nerede: Aynı kartlar. Anlam: griye çalan yumuşak yeşil tema.
settings-color-theme-sage = Sage
# Nerede: Aynı kartlar. Anlam: tozlu pembe tema.
settings-color-theme-rose = Dusty rose
# Nerede: Aynı kartlar. Anlam: bal sarısı vurgulu tema.
settings-color-theme-amber = Amber
# Nerede: Aynı kartlar. Anlam: neredeyse renksiz serin gri tema.
settings-color-theme-graphite = Graphite
# Nerede: Renk teması seçilince geri al geçmişindeki adım. Anlam: hangi temaya geçildiği; $theme temanın görünen adı (ör. Adaçayı).
settings-color-theme-changed = Color theme: { $theme }
