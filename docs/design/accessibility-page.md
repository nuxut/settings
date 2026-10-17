# Ayarlar › Erişilebilirlik

Durum: uygulandı (2026-10-16). İstek: 2026-10-15. Kabuk sözleşmeleri:
shell `docs/design/magnifier.md` ve `docs/design/osk.md` "Ayarlar sözleşmesi" (magnifier-v1, osk-v1).

## Yer ve düzen

- Sayfa **Kişisel** grubunun sonunda (`PageId::Accessibility`, slug `accessibility`; takma adlar
  `a11y`, `erisilebilirlik`, `buyutec`, `magnifier`, `osk`). Gerekçe: erişilebilirlik kişinin
  kendisine göre ayardır; Görünüm/Yazı/Hareket ile aynı küme.
- Kartlar (GNOME Erişilebilirlik'teki sırayla): **Görme**, **Büyüteç**, **Ekran klavyesi**.
- İkon: `IconName::Accessibility` (framework 3788870; Material accessibility_new). İlk sürümde
  framework ikonu gelene dek `Visibility` (göz) kullanıldı.

## Görme kartı: ayna satırlar, kopya yok

Yüksek karşıtlık (`colors.contrast`), imleç boyutu (`desktop.cursor_size`; birleşik görünüm
açıkken), yazı boyutu (`typography.size`), hareketi azalt (`motion.enabled`). Hepsi kendi
sayfalarındaki satırın **aynası** (`mirror [Accessibility]`, settings-ux §3.4): tek anahtar, tek
geri al kaydı; arama birincil yere (Görünüm/Yazı/Hareket) gider.

Ayna satırlar kayıtta Erişilebilirlik satırlarından önce geldiği için "önceki satırın kartı"
kuralı işlemez; yeni kural: sayfada önünde birincil satır yoksa ayna, sayfanın ayna kartına
girer (`PageId::mirror_home` → "Görme"); o da yoksa kendi bölümünde kalır (eski davranış).

## Büyüteç kartı

- **Büyüteç** (canlı): `msg magnifier status` ile anahtar; açmak/kapatmak `msg magnifier on|off`.
  Ayarlar Hyprland'e kendisi yazmaz (sahiplik kabukta). Alt metin: "Açık · 2× büyütme" /
  "Kapalı" / "Bu masaüstünde kullanılamıyor" (`available:false`) / "Masaüstü kabuğu çalışmıyor"
  (yanıt yok, eski kabuk). Eylem sürerken anahtar istenen konumda bekler ("Bekleniyor…");
  kabuk reddederse "Değiştirilemedi" (hata renginde), anahtar gerçek duruma döner.
- **Açılış büyütmesi** (`magnifier.factor`), **Adım** (`magnifier.step`), **En fazla büyütme**
  (`magnifier.max`): shell.toml kaydırıcıları; aralıklar şemadan (yerel yedek şemayla testli).
  Alt metin şema açıklaması varsa o (kabuğun dilinde).
- **Kısayollar**: Kısayollar sayfasına yönlendirme ("Ayarla"); alt metin tuşları gösterir
  ("Yakınlaştır Super+Alt+↑ · …", atanmamışsa "Atanmamış"). Kısayollar burada düzenlenmez.

## Ekran klavyesi kartı

- **Ekran klavyesi** (canlı): `msg osk status`; göster/gizle `msg osk show|hide` (göster/gizle
  ayar değil, çalışma anı durumu — osk.md). Açıkken alt metin düzenle: "Açık · Türkçe Q".
- **Düzen** (`osk.layout`: Otomatik / Türkçe Q / İngilizce (ABD); segmentli), **Tuş yüksekliği**
  (`osk.key_height` 32–72 px).
- **Kısayol**: `keybinds.osk_toggle` (SUPER+ALT+K; satırı ekler, Kısayollar sayfası) —
  buradan yönlendirme; satır kayıtta yokken yönlendirme gizli.

## Canlı durum (`services::assist`)

Sayfa açılınca bir kez, pencere odağı dönünce ve sayfa görünürken 2 sn arayla sorulur (kısayolla
yapılan büyütme sayfada görünsün); yalnızca değişince `Event::Assist`. Yanıtlar güvenilmez girdi
gibi doğrulanır (çarpan 1..=10, bilinmeyen düzen yok sayılır; alanlar yalnızca eklenir).

## Bilinen sınırlar

- Hızlı ayar kartlarını (`magnifier`, `osk` widget'ları) panele eklemek Paneller sayfasındaki
  içerik listesinden yapılır; Erişilebilirlik'te ayrı düğme yok.
- Kilit ekranında ekran klavyesi yok (osk v1 sınırı).

## Test

Birim: `services::assist` (ayrıştırma, hata eşleme, komutlar), `pages::assist` (satır görünümü:
açık/kapalı/kullanılamıyor/kabuk yok/beklemede/başarısız), `text_pages_tests::accessibility_page_cards_and_mirrors`
(kartlar, aynalar, arama birincil yere, kısayol alt metni). Canlı: izole sway + `fake_shell`
(`magnifier`/`osk` durum ve eylemleri; `NUXUT_FAKE_ASSIST=unsupported`).
