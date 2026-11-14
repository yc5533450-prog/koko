# CHIMERA TR — Gerçek site olarak yayınlama paketi

Bu dosyalar **orijinal https://www.chimera.audio/ uygulamasını** MIT lisanslı GitHub deposundan derleyerek Türkçe metin girişini destekleyen bir sürüm hazırlamak içindir. Bu bir HTML taklidi değildir. Orijinal arayüz ve fiziksel Rust/WebAssembly ses motoru kullanılır.

## Siteyi `.vercel.app` adresinde açmak için

1. GitHub'da **yeni bir depo (repository)** oluştur (ör. `chimera-tr`).
2. Bu ZIP'i bilgisayarında aç. ZIP'in içindeki **tüm dosya ve klasörleri** (özellikle `scripts` klasörünü) oluşturduğun GitHub deposuna yükle. ZIP dosyasının kendisini değil, içindekileri yükle.
3. https://vercel.com/new adresinde GitHub ile giriş yap. `chimera-tr` deponu **Import** seçeneğiyle içeri al.
4. Framework `Vite`, Output Directory `chimera-vocal-synth/dist` ve Build Command `bash scripts/build.sh` olmalı; `vercel.json` bunları önceden belirliyor. Diğer ayarları değiştirmeden **Deploy** de.
5. Derleme tamamlanırsa Vercel sana `https://<proje-adı>.vercel.app` biçiminde **gerçek bir HTTPS site adresi** verir. Ses motoru HTTPS gerektirir.
6. `SPEAK` menüsünde `Merhaba İstanbul. Çığ ve şarkı.` dene; `SING` menüsünde Türkçe söz yaz. İngilizce için metnin başına `en:` ekleyebilirsin.

**Önemli:** Bu paket henüz internette yayınlanmış bir site değildir. Hesabında yayınlama yapılmadan aktif bir URL oluşmaz. Bu ortamda Rust ve Bun çalıştırılamadığı için gerçek derleme ve ses testi yapılmamıştır; Vercel derlemesi kaynak sürümüne göre hata verebilir. Kod yaması beklentisi karşılanmazsa sessizce bozuk site üretmek yerine hata verecek şekilde tasarlandı.

## Teknik bilgi

- `scripts/install.sh`: resmî [CHIMERA](https://github.com/jamesondh/chimera-vocal-synth) kaynak kodunu indirir, Rust WebAssembly hedefi ve Bun paketlerini kurar.
- `scripts/build.sh`: Türkçe fonetik yamayı uygular ve `dist/` statik web sitesini üretir.
- `turkce_yama.py`, `turkish.rs`: Türkçe `ç, ğ, ı, İ, ö, ş, ü`, ünlüler ve yaklaşık Türkçe heceleme.
- `vercel.json`: barındırma ve derleme ayarları.

Bu paket orijinal projenin **resmî Türkçe sürümü değildir**. Türkçe telaffuz ve vurgu bazı sözcüklerde yaklaşık olabilir. Menü etiketleri orijinalindeki gibi İngilizcedir. Orijinal yazarın MIT lisansına tabidir, `LICENSE` dosyasını sakla.

## Alternatif: Yerel derleme

Git, Rust, Python3 ve Bun kurulu bilgisayarda:

```bash
bash scripts/install.sh
bash scripts/build.sh
cd chimera-vocal-synth && bun run preview
```

WebAssembly tabanlı ses çalıştırma için localhost veya HTTPS gerekir.
