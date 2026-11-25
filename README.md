<div align="center">



<!-- Animated Night Red Header -->

<img src="https://capsule-render.vercel.app/api?type=waving\&color=0:050505,45:1A1A1A,100:E60000\&height=210\&section=header\&text=TPCC\&fontSize=76\&fontColor=FFFFFF\&fontAlignY=38\&desc=GitHub%20JSON%20Powered%20Terminal%20PNG%20Generator\&descSize=20\&descAlignY=62\&descColor=FF4D4D\&animation=twinkling" width="100%" alt="TPCC Banner" />



<!-- Animated Live Terminal Typing -->

<a href="https://github.com/tda45/TPCC">

&#x20; <img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono\&weight=700\&size=18\&duration=2800\&pause=900\&color=E60000\&background=05050500\&center=true\&vCenter=true\&multiline=true\&repeat=true\&width=780\&height=75\&lines=%24+tpcc+--preset+github-banner+--text+%22TPCC+ENGINE%22+-o+banner.png;%24+tpcc+--preset+synthwave-80s+--text+%22NEON+MODE%22+--scanlines+4+-o+synth.png;%24+tpcc+--update+%26%26+tpcc+--palette-sheet+-o+full\_palette.png" alt="Typing SVG" />

</a>



<br/>



<!-- Night Red Badges -->

<p>

&#x20; <img src="https://img.shields.io/badge/Language-Rust\_2021-E60000?style=for-the-badge\&logo=rust\&logoColor=white\&labelColor=050505" alt="Rust" />

&#x20; <img src="https://img.shields.io/badge/Palette-110%2B\_Remote\_Colors-E60000?style=for-the-badge\&logo=github\&logoColor=white\&labelColor=050505" alt="Colors" />

&#x20; <img src="https://img.shields.io/badge/Presets-31\_Ready\_Themes-E60000?style=for-the-badge\&logo=palette\&logoColor=white\&labelColor=050505" alt="Presets" />

&#x20; <img src="https://img.shields.io/badge/Build-Zero\_C\_Dependencies-00FF41?style=for-the-badge\&logo=windows-terminal\&logoColor=white\&labelColor=050505" alt="Zero Deps" />

</p>



</div>



\---



\## ⚡ Nedir?



\*\*TPCC\*\*, Linux tarafındaki klasik terminal tabanlı görsel oluşturucuların hızını \*\*uzak GitHub JSON renk paletleri\*\*, \*\*hazır tema şablonları (presets)\*\* ve \*\*modern grafik efektleriyle\*\* birleştiren saf Rust (`pure Rust`) ve tek exe biraz bağımlılık ve sadece 1005 satırlık tek bir .rs ile çalışan komut satırı aracıdır.



\* 🌐 \*\*Canlı GitHub JSON Entegrasyonu:\*\* Renk kodlarını ezberlemek yerine doğrudan `colors.json` üzerindeki isimleri (`night-red`, `neon-cyan`, `matrix-green` vb.) kullanır. (Şuanda bile burdan data alanlar vardır stttt…)

\* ⚡ \*\*Yerel Önbellek (Offline Mod):\*\* `--update` ile paleti diske kaydeder; internetsiz ortamda milisaniyeler içinde çalışır internetsiz görsel oluşturabilirsiniz

\* 🎨 \*\*31 Hazır Şablon (`--preset`):\*\* GitHub banner, YouTube thumbnail, CRT terminal, Synthwave, Dracula, Nord gibi temaları tek parametreyle yükler.Hepsi colors.json a bir bak

\* 🛠️ \*\*Sıfır Harici Bağımlılık:\*\* OpenSSL, FreeType veya ImageMagick kurulumu gerektirmez; Windows ve Linux'ta tek çalıştırılabilir dosya (`tpcc.exe` / `tpcc`) üretir.



\---



\## 🖼️ TPCC ile Üretilen Örnekler



<table>

&#x20; <tr>

&#x20;   <td align="center" width="50%">

&#x20;     <img src="assets/examples%20from%20TPCC/banner.png" alt="TPCC Banner Example" width="100%" />

&#x20;     <br/>

&#x20;     <b><code>--preset github-banner</code></b>

&#x20;     <br/>

&#x20;     <sub>Night Red gradyan, teknik ızgara (grid), çerçeve ve yuvarlatılmış köşeler</sub>

&#x20;   </td>

&#x20;   <td align="center" width="50%">

&#x20;     <img src="assets/examples%20from%20TPCC/synth.png" alt="TPCC Synthwave Example" width="100%" />

&#x20;     <br/>

&#x20;     <b><code>--preset synthwave-80s</code></b>

&#x20;     <br/>

&#x20;     <sub>80'ler retro gradyan, çok satırlı başlık, alt başlık gölgesi ve CRT scanlines</sub>

&#x20;   </td>

&#x20; </tr>

</table>



\---



\## 🚀 Kurulum ve Derleme



```bash

git clone \[https://github.com/tda45/TPCC.git](https://github.com/tda45/TPCC.git)

cd TPCC

cargo build --release

