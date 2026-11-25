<div align="center">

<!-- Animated Night Red Header -->
<img src="https://capsule-render.vercel.app/api?type=waving&color=0:8A0303,50:E60000,100:050505&height=210&section=header&text=TPCC&fontSize=76&fontColor=FFFFFF&stroke=050505&strokeWidth=2&fontAlignY=38&desc=GitHub%20JSON%20Powered%20Terminal%20PNG%20Generator&descSize=20&descAlignY=62&descColor=FFFFFF&animation=twinkling" width="100%" alt="TPCC Banner" />

<!-- Animated Live Terminal Typing -->
<a href="https://github.com/tda45/TPCC">
  <img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=700&size=18&duration=2800&pause=900&color=E60000&background=05050500&center=true&vCenter=true&multiline=true&repeat=true&width=780&height=75&lines=%24+tpcc+--preset+github-banner+--text+%22TPCC+ENGINE%22+-o+banner.png;%24+tpcc+--preset+synthwave-80s+--text+%22NEON+MODE%22+--scanlines+4+-o+synth.png;%24+tpcc+--update+%26%26+tpcc+--palette-sheet+-o+full_palette.png" alt="Typing SVG" />
</a>

<!-- Night Red Badges -->
<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust_2021-E60000?style=for-the-badge&logo=rust&logoColor=white&labelColor=050505" alt="Rust" />
  <img src="https://img.shields.io/badge/Palette-110%2B_Remote_Colors-E60000?style=for-the-badge&logo=github&logoColor=white&labelColor=050505" alt="Colors" />
  <img src="https://img.shields.io/badge/Presets-31_Ready_Themes-E60000?style=for-the-badge&logo=palette&logoColor=white&labelColor=050505" alt="Presets" />
  <img src="https://img.shields.io/badge/Build-Zero_C_Dependencies-00FF41?style=for-the-badge&logo=windows-terminal&logoColor=white&labelColor=050505" alt="Zero Deps" />
</p>

</div>

---

## ⚡ Nedir?

**TPCC**, Linux tarafındaki klasik terminal tabanlı görsel oluşturucuların hızını **uzak GitHub JSON renk paletleri**, **hazır tema şablonları (presets)** ve **modern grafik efektleriyle** birleştiren saf Rust (`pure Rust`), tek `.exe`, az bağımlılıklı ve sadece **1005 satırlık tek bir `.rs`** dosyasıyla çalışan komut satırı aracıdır.

- 🌐 **Canlı GitHub JSON Entegrasyonu:** Renk kodlarını ezberlemek yerine doğrudan `colors.json` üzerindeki isimleri (`night-red`, `neon-cyan`, `matrix-green` vb.) kullanır. *(Şu anda bile buradan data alanlar vardır stttt…)*
- ⚡ **Yerel Önbellek (Offline Mod):** `--update` ile paleti diske kaydeder; internetsiz ortamda milisaniyeler içinde çalışır, internetsiz görsel oluşturabilirsiniz.
- 🎨 **31 Hazır Şablon (`--preset`):** GitHub banner, YouTube thumbnail, CRT terminal, Synthwave, Dracula, Nord gibi temaları tek parametreyle yükler. Hepsi `colors.json`'a bir bak!
- 🛠️ **Sıfır Harici Bağımlılık:** OpenSSL, FreeType veya ImageMagick kurulumu gerektirmez; Windows ve Linux'ta tek çalıştırılabilir dosya (`tpcc.exe` / `tpcc`) üretir.

---

## 🖼️ TPCC ile Üretilen Örnekler

<table>
  <tr>
    <td align="center" width="50%">
      <img src="assets/examples%20from%20TPCC/banner.png" alt="TPCC Banner Example" width="100%" />
      <br/>
      <b><code>--preset github-banner</code></b>
      <br/>
      <sub>Night Red gradyan, teknik ızgara (grid), çerçeve ve yuvarlatılmış köşeler</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/examples%20from%20TPCC/synth.png" alt="TPCC Synthwave Example" width="100%" />
      <br/>
      <b><code>--preset synthwave-80s</code></b>
      <br/>
      <sub>80'ler retro gradyan, çok satırlı başlık, alt başlık gölgesi ve CRT scanlines</sub>
    </td>
  </tr>
</table>

---

## 🚀 Kurulum ve Derleme

```bash
git clone https://github.com/tda45/TPCC.git
cd TPCC
cargo build --release
```

Derleme tamamlandığında çalıştırılabilir dosya `target/release/tpcc.exe` (Linux'ta `target/release/tpcc`) konumunda hazır olur.

---

## 💻 Hızlı Kullanım

### 1. Önbelleği Güncelle ve Renkleri Listele

```powershell
tpcc --update --list
```

### 2. Hazır Şablonla Tek Satırda Banner Üret

```powershell
tpcc --preset github-banner --text "TPCC ENGINE" --subtext "GitHub JSON Powered PNG Generator" -o banner.png
```

### 3. Özel Efektler, Geometrik Şekiller ve Logo Bindirme

```powershell
tpcc -s 1280x720 --bg pitch-black --bg2 dark-crimson --noise 10 --grid 40 --rect "240,220,800,280,glass-dark" --circle "1140,120,45,night-red" --text "TPCC v0.2.0\nPURE RUST" --text-color night-red --subtext "Fast • Offline Cache • 110+ Colors" --shadow --scanlines 4 --radius 24 --border 5 -o showcase.png
```

### 4. Tüm GitHub Paletini Tek Görsele Dök

```powershell
tpcc --palette-sheet -o full_palette.png
```

---

## 🎨 Canlı Renk Kataloğu (`colors.json`)

Varsayılan olarak [tda45/TPCC](https://github.com/tda45/TPCC) deposundaki `main/colors.json` dosyasından çekilen **110+ renk**, doğrudan `tpcc --palette-sheet` komutuyla görsel renk kataloğu (`palette.png`) olarak oluşturulabilir.

---

## 🧩 Komut Satırı Parametreleri

| Parametre | Kısa | Varsayılan | Açıklama |
|---|---|---|---|
| `--output <DOSYA>` | `-o` | `output.png` | Çıktı alınacak PNG dosya adı |
| `--size <GxY>` | `-s` | `800x600` | Görsel çözünürlüğü (Örn: `1280x720`, `1920x1080`) |
| `--url <URL>` | `-u` | GitHub Repo | Özel bir GitHub Raw `colors.json` adresi |
| `--update` | — | `false` | Uzak JSON paletini yerel önbelleğe indirip günceller |
| `--list` | `-l` | `false` | Palet içindeki tüm renk adlarını ve HEX kodlarını listeler |
| `--palette-sheet` | — | `false` | Tüm paleti görsel renk kataloğu (`palette.png`) olarak üretir |
| `--preset <AD>` | — | — | JSON içindeki `presets` bölümünden hazır tema yükler |
| `--bg <RENK>` | — | `pitch-black` | Ana arka plan rengi (Palet adı veya `#HEX`) |
| `--bg2 <RENK>` | — | — | İkinci arka plan rengi (Gradyan geçişi oluşturur) |
| `--gradient <YÖN>` | — | `vertical` | Gradyan yönü: `vertical` veya `horizontal` |
| `--text <METİN>` | `-t` | — | Ana başlık metni (`\n` ile çok satırlı yazım destekler) |
| `--text-color <RENK>` | — | `night-red` | Ana başlık rengi |
| `--subtext <METİN>` | — | — | Ana başlığın altına yerleştirilen ikincil alt başlık |
| `--subtext-color <RENK>` | — | `#D2D2D2` | Alt başlık rengi |
| `--font-size <PX>` | — | `48.0` | Ana metin boyutu (piksel) |
| `--font <YOL>` | — | Sistem Fontu | Özel `.ttf` yazı tipi dosyası |
| `--pos <KONUM>` | — | `center` | Konum: `center`, `top-left`, `top-right`, `bottom-left`, `bottom-right` veya `x,y` |
| `--shadow [RENK]` | — | `#000000B4` | Metinlere derinlik katan gölge efekti |
| `--border <PX>` | — | `0` | Dış çerçeve kalınlığı |
| `--border-color <RENK>` | — | `night-red` | Dış çerçeve rengi |
| `--grid <PX>` | — | — | Arka plana teknik ızgara çizgileri ekler (Örn: `40`) |
| `--scanlines <PX>` | — | — | Retro CRT monitör yatay çizgi efekti (Örn: `4`) |
| `--noise <0-255>` | — | — | Yüzeye hafif kumlanma / doku efekti (Örn: `12`) |
| `--radius <PX>` | — | `0` | Köşeleri şeffaf şekilde yuvarlar (Örn: `20`) |
| `--rect <SPEC>` | — | — | Dikdörtgen çizer: `x,y,genislik,yukseklik,renk` (Çoklu kullanılabilir) |
| `--circle <SPEC>` | — | — | Daire çizer: `cx,cy,yaricap,renk` (Çoklu kullanılabilir) |
| `--overlay <DOSYA>` | — | — | Görsel üzerine logo veya filigran PNG bindirir |
| `--overlay-pos <KONUM>` | — | `bottom-right` | Bindirilecek görselin konumu |
| `--overlay-scale <ORAN>` | — | `1.0` | Bindirilecek görselin ölçeği (Örn: `0.5`) |

---

## 🖼️ Hazır Şablonlar (Presets)

| Şablon Adı | Çözünürlük | Stil ve Efekt Özeti |
|---|---|---|
| `github-banner` | `1280x640` | Pitch Black → Dark Gray gradyan, Night Red çerçeve + grid |
| `night-red-pro` | `1280x720` | Void Black → Dark Crimson, doku efekti, gölge + 48px grid |
| `synthwave-80s` | `1280x720` | Deep Violet → Ruby gradyan, Neon Cyan metin + CRT scanlines |
| `matrix-terminal` | `1000x500` | Matrix Green fosfor yazı, scanlines + retro terminal çerçevesi |
| `cyber-card` | `800x450` | Yatay gradyan, Neon Cyan vurgu ve yumuşak gölge |
| `amber-crt` | `1024x768` | Klasik kehribar terminal görünümü + yoğun CRT tarama çizgileri |
| `dracula-theme` | `1200x630` | Dracula paleti (Pembe, Yeşil, Mor) + teknik grid |
| `nord-theme` | `1200x630` | Kuzey buz mavisi tonları ve sade çerçeve |
| `rust-engine` | `1280x640` | Karbon arka plan üzerinde Rust turuncusu vurgular |
| `wallpaper-4k` | `3840x2160` | 4K Night Red masaüstü duvar kağıdı düzeni |

---

## 👨‍💻 Geliştirici

**tda45** • Night Red Design System