use ab_glyph::{FontRef, PxScale};
use clap::parser::ValueSource;
use clap::{CommandFactory, FromArgMatches, Parser};
use image::imageops::{overlay, resize, FilterType};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Varsayılan GitHub palet URL'si
const DEFAULT_PALETTE_URL: &str =
    "https://raw.githubusercontent.com/tda45/TPCC/main/colors.json";

#[derive(Parser, Debug)]
#[command(name = "tpcc", version = "0.2.0", about = "GitHub JSON destekli terminal PNG üreticisi")]
struct Cli {
    /// Çıktı dosya adı
    #[arg(short, long, default_value = "output.png")]
    output: String,

    /// Görsel boyutu (Örn: 1280x720)
    #[arg(short = 's', long, default_value = "800x600")]
    size: String,

    /// Renk isimlerinin çekileceği GitHub Raw JSON URL'si
    #[arg(short, long, default_value = DEFAULT_PALETTE_URL)]
    url: String,

    /// Uzak JSON'daki tüm renkleri listeler ve çıkar
    #[arg(short, long)]
    list: bool,

    /// Arka plan rengi (JSON'daki renk adı veya #HEX)
    #[arg(long, default_value = "pitch-black")]
    bg: String,

    /// İkinci arka plan rengi (Gradyan için, opsiyonel)
    #[arg(long)]
    bg2: Option<String>,

    /// Gradyan yönü: vertical veya horizontal
    #[arg(long, default_value = "vertical")]
    gradient: String,

    /// Görsel üzerine yazılacak metin (\n ile çok satırlı yazılabilir)
    #[arg(short, long)]
    text: Option<String>,

    /// Metin rengi (JSON'daki renk adı veya #HEX)
    #[arg(long, default_value = "night-red")]
    text_color: String,

    /// Yazı tipi boyutu (piksel)
    #[arg(long, default_value_t = 48.0)]
    font_size: f32,

    /// Özel .ttf font dosyası yolu (Belirtilmezse sistem fontu kullanılır)
    #[arg(long)]
    font: Option<PathBuf>,

    /// Metin konumu: center, top-left, top-right, bottom-left, bottom-right veya x,y
    #[arg(long, default_value = "center")]
    pos: String,

    /// Çerçeve (border) kalınlığı (piksel)
    #[arg(long, default_value_t = 0)]
    border: u32,

    /// Çerçeve rengi (JSON'daki renk adı veya #HEX)
    #[arg(long, default_value = "night-red")]
    border_color: String,

    /// Arka plana teknik ızgara (grid) çizgileri ekler (piksel aralığı, örn: 40)
    #[arg(long)]
    grid: Option<u32>,

    // --- YENİ EKLENEN ÖZELLİKLER ---

    /// Ana metnin altına eklenecek alt başlık metni (\n destekler)
    #[arg(long)]
    subtext: Option<String>,

    /// Alt başlık metin rengi (Belirtilmezse açık gri kullanılır)
    #[arg(long)]
    subtext_color: Option<String>,

    /// Alt başlık yazı tipi boyutu (Belirtilmezse font_size'ın %55'i alınır)
    #[arg(long)]
    subtext_size: Option<f32>,

    /// Metin gölgesi ekler (Tek başına --shadow veya --shadow "#000000" / renk adı)
    #[arg(long, num_args = 0..=1, default_missing_value = "#000000B4")]
    shadow: Option<String>,

    /// Metin gölge kaydırma mesafesi (piksel)
    #[arg(long, default_value_t = 3)]
    shadow_offset: i32,

    /// Üzerine yapıştırılacak dış PNG/görsel dosyası (logo, filigran vb.)
    #[arg(long)]
    overlay: Option<PathBuf>,

    /// Overlay konumu: center, top-left, top-right, bottom-left, bottom-right veya x,y
    #[arg(long, default_value = "bottom-right")]
    overlay_pos: String,

    /// Overlay ölçek katsayısı (Örn: 0.5 yarı boyut, 1.0 orijinal)
    #[arg(long, default_value_t = 1.0)]
    overlay_scale: f32,

    /// Köşe yuvarlama yarıçapı (piksel, köşeleri şeffaf yapar)
    #[arg(long, default_value_t = 0)]
    radius: u32,

    /// Retro CRT/terminal yatay çizgi efekti aralığı (piksel, örn: 4)
    #[arg(long)]
    scanlines: Option<u32>,

    /// Arka plana eklenecek kumlanma/doku şiddeti (Örn: 15)
    #[arg(long)]
    noise: Option<u8>,

    /// Dikdörtgen çizer: "x,y,genislik,yukseklik,renk" (Birden fazla kez verilebilir)
    #[arg(long)]
    rect: Vec<String>,

    /// Daire çizer: "merkez_x,merkez_y,yaricap,renk" (Birden fazla kez verilebilir)
    #[arg(long)]
    circle: Vec<String>,

    /// JSON'daki tüm renkleri görsel katalog (palette.png) olarak dışa aktarır
    #[arg(long)]
    palette_sheet: bool,

    /// Uzak GitHub JSON paletini yerel önbelleğe indirip günceller
    #[arg(long)]
    update: bool,

    /// JSON içindeki "presets" bloğundan hazır şablon yükler
    #[arg(long)]
    preset: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Cli::command().get_matches();
    let mut cli = Cli::from_arg_matches(&matches)?;

    // 1. JSON verisini önbellekten veya GitHub'dan yükle
    let json_data = load_palette_json(&cli.url, cli.update)?;
    let palette = extract_colors(&json_data);

    if cli.update {
        if let Some(cache_file) = get_cache_path() {
            println!("Yerel palet önbelleği güncellendi: {}", cache_file.display());
        }
        // Sadece --update çağrıldıysa başka işlem yapmadan çık
        if !cli.list
            && !cli.palette_sheet
            && cli.text.is_none()
            && cli.subtext.is_none()
            && cli.overlay.is_none()
            && cli.rect.is_empty()
            && cli.circle.is_empty()
            && cli.preset.is_none()
            && matches.value_source("output") != Some(ValueSource::CommandLine)
        {
            return Ok(());
        }
    }

    // 2. --preset belirtildiyse JSON'daki şablon ayarlarını uygula (CLI'dan elle verilenler korunur)
    if let Some(ref preset_name) = cli.preset.clone() {
        apply_preset(&mut cli, &matches, &json_data, preset_name)?;
    }

    // --list parametresi verildiyse renkleri yazdır ve çık
    if cli.list {
        println!("Palet URL: {}\nMevcut Renkler:", cli.url);
        let mut keys: Vec<_> = palette.keys().collect();
        keys.sort();
        for k in keys {
            println!("  {:<18} -> {}", k, palette[k]);
        }
        return Ok(());
    }

    // --palette-sheet parametresi verildiyse tüm paleti katalog PNG olarak üret ve çık
    if cli.palette_sheet {
        let out_path = if matches.value_source("output") == Some(ValueSource::CommandLine) {
            cli.output.clone()
        } else {
            "palette.png".to_string()
        };
        generate_palette_sheet(&palette, &out_path, cli.font.as_ref())?;
        return Ok(());
    }

    // 3. Boyutları ayrıştır (Örn: "1280x720")
    let (width, height) = parse_dimensions(&cli.size)?;

    // 4. Renkleri çözümle
    let bg_rgba = resolve_color(&cli.bg, &palette)?;
    let bg2_rgba = match &cli.bg2 {
        Some(c) => Some(resolve_color(c, &palette)?),
        None => None,
    };

    let mut img = RgbaImage::new(width, height);

    // 5. Arka planı (Düz renk veya Gradyan) çiz
    for y in 0..height {
        for x in 0..width {
            let pixel = if let Some(c2) = bg2_rgba {
                let t = if cli.gradient == "horizontal" {
                    x as f32 / (width.max(1) - 1).max(1) as f32
                } else {
                    y as f32 / (height.max(1) - 1).max(1) as f32
                };
                lerp_rgba(bg_rgba, c2, t)
            } else {
                bg_rgba
            };
            img.put_pixel(x, y, pixel);
        }
    }

    // 6. Opsiyonel Noise (Kumlanma/Doku) efekti
    if let Some(amount) = cli.noise {
        if amount > 0 {
            apply_noise(&mut img, amount);
        }
    }

    // 7. Opsiyonel Teknik Izgara (Grid) çizimi
    if let Some(step) = cli.grid {
        if step > 0 {
            let grid_color = Rgba([255, 255, 255, 22]);
            for x in (0..width).step_by(step as usize) {
                for y in 0..height {
                    blend_pixel(&mut img, x, y, grid_color);
                }
            }
            for y in (0..height).step_by(step as usize) {
                for x in 0..width {
                    blend_pixel(&mut img, x, y, grid_color);
                }
            }
        }
    }

    // 8. Opsiyonel Geometrik Şekiller (--rect ve --circle)
    for rect_spec in &cli.rect {
        draw_custom_rect(&mut img, rect_spec, &palette)?;
    }
    for circle_spec in &cli.circle {
        draw_custom_circle(&mut img, circle_spec, &palette)?;
    }

    // 9. Opsiyonel Görsel / Logo Bindirme (--overlay)
    if let Some(ref overlay_path) = cli.overlay {
        let loaded = image::open(overlay_path)
            .map_err(|e| format!("Overlay görseli açılamadı ({:?}): {}", overlay_path, e))?
            .to_rgba8();

        let final_overlay = if (cli.overlay_scale - 1.0).abs() > f32::EPSILON && cli.overlay_scale > 0.0 {
            let nw = ((loaded.width() as f32 * cli.overlay_scale).round() as u32).max(1);
            let nh = ((loaded.height() as f32 * cli.overlay_scale).round() as u32).max(1);
            resize(&loaded, nw, nh, FilterType::Lanczos3)
        } else {
            loaded
        };

        let (ox, oy) = calculate_text_position(
            &cli.overlay_pos,
            width,
            height,
            final_overlay.width(),
            final_overlay.height(),
            cli.border,
        );
        overlay(&mut img, &final_overlay, ox as i64, oy as i64);
    }

    // 10. Opsiyonel Çerçeve (Border) çizimi
    if cli.border > 0 {
        let b_color = resolve_color(&cli.border_color, &palette)?;
        for i in 0..cli.border {
            let w = width.saturating_sub(i * 2);
            let h = height.saturating_sub(i * 2);
            if w > 0 && h > 0 {
                draw_hollow_rect_mut(&mut img, Rect::at(i as i32, i as i32).of_size(w, h), b_color);
            }
        }
    }

    // 11. Çok Satırlı Metin, Alt Başlık ve Gölge çizimi
    if cli.text.is_some() || cli.subtext.is_some() {
        let font_bytes = load_font_bytes(cli.font.as_ref())?;
        let font = FontRef::try_from_slice(&font_bytes)
            .map_err(|_| "Font dosyası ayrıştırılamadı (geçerli bir .ttf olmalı)")?;

        let shadow_rgba = match &cli.shadow {
            Some(s) => Some(resolve_color(s, &palette)?),
            None => None,
        };

        let main_scale = PxScale::from(cli.font_size);
        let main_line_h = (cli.font_size * 1.25).round() as u32;
        let main_lines: Vec<String> = cli
            .text
            .as_deref()
            .unwrap_or("")
            .replace("\\n", "\n")
            .split('\n')
            .map(|s| s.to_string())
            .filter(|_| cli.text.is_some())
            .collect();

        let sub_font_size = cli.subtext_size.unwrap_or((cli.font_size * 0.55).max(12.0));
        let sub_scale = PxScale::from(sub_font_size);
        let sub_line_h = (sub_font_size * 1.25).round() as u32;
        let sub_lines: Vec<String> = cli
            .subtext
            .as_deref()
            .unwrap_or("")
            .replace("\\n", "\n")
            .split('\n')
            .map(|s| s.to_string())
            .filter(|_| cli.subtext.is_some())
            .collect();

        // Tüm metin bloğunun toplam genişlik ve yüksekliğini hesapla
        let mut max_w: u32 = 0;
        let mut total_h: u32 = 0;

        for line in &main_lines {
            let (lw, _) = text_size(main_scale, &font, line);
            max_w = max_w.max(lw);
            total_h += main_line_h;
        }

        let section_gap = if !main_lines.is_empty() && !sub_lines.is_empty() {
            (sub_font_size * 0.45).round() as u32
        } else {
            0
        };
        total_h += section_gap;

        for line in &sub_lines {
            let (lw, _) = text_size(sub_scale, &font, line);
            max_w = max_w.max(lw);
            total_h += sub_line_h;
        }

        let (block_x, mut cursor_y) =
            calculate_text_position(&cli.pos, width, height, max_w, total_h, cli.border);
        let center_align = cli.pos.eq_ignore_ascii_case("center");

        // Ana başlık satırlarını çiz
        if !main_lines.is_empty() {
            let t_color = resolve_color(&cli.text_color, &palette)?;
            for line in &main_lines {
                let (lw, _) = text_size(main_scale, &font, line);
                let lx = if center_align {
                    block_x + ((max_w as i32 - lw as i32) / 2)
                } else {
                    block_x
                };

                if let Some(sh_col) = shadow_rgba {
                    draw_text_mut(
                        &mut img,
                        sh_col,
                        lx + cli.shadow_offset,
                        cursor_y + cli.shadow_offset,
                        main_scale,
                        &font,
                        line,
                    );
                }
                draw_text_mut(&mut img, t_color, lx, cursor_y, main_scale, &font, line);
                cursor_y += main_line_h as i32;
            }
        }

        cursor_y += section_gap as i32;

        // Alt başlık satırlarını çiz
        if !sub_lines.is_empty() {
            let sub_color = match &cli.subtext_color {
                Some(c) => resolve_color(c, &palette)?,
                None => Rgba([210, 210, 210, 255]),
            };

            for line in &sub_lines {
                let (lw, _) = text_size(sub_scale, &font, line);
                let lx = if center_align {
                    block_x + ((max_w as i32 - lw as i32) / 2)
                } else {
                    block_x
                };

                if let Some(sh_col) = shadow_rgba {
                    let sub_off = (cli.shadow_offset - 1).max(1);
                    draw_text_mut(
                        &mut img,
                        sh_col,
                        lx + sub_off,
                        cursor_y + sub_off,
                        sub_scale,
                        &font,
                        line,
                    );
                }
                draw_text_mut(&mut img, sub_color, lx, cursor_y, sub_scale, &font, line);
                cursor_y += sub_line_h as i32;
            }
        }
    }

    // 12. Opsiyonel Scanlines (CRT Yatay Çizgi) efekti
    if let Some(step) = cli.scanlines {
        if step > 0 {
            let scan_color = Rgba([0, 0, 0, 95]);
            for y in (0..height).step_by(step as usize) {
                for x in 0..width {
                    blend_pixel(&mut img, x, y, scan_color);
                }
            }
        }
    }

    // 13. Opsiyonel Köşe Yuvarlama (--radius)
    if cli.radius > 0 {
        apply_rounded_corners(&mut img, cli.radius);
    }

    // 14. PNG olarak kaydet
    img.save(&cli.output)?;
    println!("Oluşturuldu: {} ({}x{})", cli.output, width, height);
    Ok(())
}

/// Yerel önbellek dosya yolunu döndürür (%LOCALAPPDATA%\tpcc\palette.json veya ~/.cache/tpcc/palette.json)
fn get_cache_path() -> Option<PathBuf> {
    if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
        return Some(PathBuf::from(local_app).join("tpcc").join("palette.json"));
    }
    if let Some(xdg) = std::env::var_os("XDG_CACHE_HOME") {
        return Some(PathBuf::from(xdg).join("tpcc").join("palette.json"));
    }
    if let Some(home) = std::env::var_os("HOME") {
        return Some(
            PathBuf::from(home)
                .join(".cache")
                .join("tpcc")
                .join("palette.json"),
        );
    }
    Some(std::env::temp_dir().join("tpcc").join("palette.json"))
}

/// Önce yerel önbelleğe bakar; yoksa veya --update verildiyse GitHub'dan indirip kaydeder
fn load_palette_json(url: &str, force_update: bool) -> Result<Value, Box<dyn std::error::Error>> {
    let cache_path = get_cache_path();
    let is_default_url = url == DEFAULT_PALETTE_URL;

    if !force_update && is_default_url {
        if let Some(ref path) = cache_path {
            if let Ok(cached_str) = fs::read_to_string(path) {
                if let Ok(parsed) = serde_json::from_str::<Value>(&cached_str) {
                    return Ok(parsed);
                }
            }
        }
    }

    match ureq::get(url).call() {
        Ok(resp) => {
            let body = resp.into_string()?;
            let json: Value = serde_json::from_str(&body)
                .map_err(|e| format!("Geçersiz JSON formatı: {}", e))?;

            if is_default_url {
                if let Some(ref path) = cache_path {
                    if let Some(parent) = path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::write(path, &body);
                }
            }
            Ok(json)
        }
        Err(e) => {
            // Ağ hatası durumunda varsa önbelleğe düş
            if let Some(ref path) = cache_path {
                if let Ok(cached_str) = fs::read_to_string(path) {
                    if let Ok(parsed) = serde_json::from_str::<Value>(&cached_str) {
                        eprintln!("Uyarı: Uzak sunucuya ulaşılamadı, yerel önbellek kullanılıyor.");
                        return Ok(parsed);
                    }
                }
            }
            Err(format!("URL'den JSON alınamadı ({}): {}", url, e).into())
        }
    }
}

/// JSON içerisinden renk anahtar-değer haritasını çıkarır
fn extract_colors(json: &Value) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let target_obj = json
        .get("colors")
        .and_then(|v| v.as_object())
        .or_else(|| json.as_object());

    if let Some(obj) = target_obj {
        for (k, v) in obj {
            if let Some(hex) = v.as_str() {
                map.insert(k.to_lowercase(), hex.to_string());
            }
        }
    }
    map
}

/// JSON'daki "presets" bloğundan şablon ayarlarını Cli yapısına uygular
fn apply_preset(
    cli: &mut Cli,
    matches: &clap::ArgMatches,
    json: &Value,
    preset_name: &str,
) -> Result<(), String> {
    let presets = json
        .get("presets")
        .and_then(|v| v.as_object())
        .ok_or("JSON dosyasında 'presets' bölümü bulunamadı.")?;

    let preset = presets
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(preset_name))
        .map(|(_, v)| v)
        .ok_or_else(|| format!("'{}' şablonu JSON presets içinde bulunamadı!", preset_name))?;

    let is_cli = |id: &str| matches.value_source(id) == Some(ValueSource::CommandLine);

    if !is_cli("size") {
        if let Some(v) = preset.get("size").and_then(|v| v.as_str()) {
            cli.size = v.to_string();
        }
    }
    if !is_cli("bg") {
        if let Some(v) = preset.get("bg").and_then(|v| v.as_str()) {
            cli.bg = v.to_string();
        }
    }
    if !is_cli("bg2") {
        if let Some(v) = preset.get("bg2").and_then(|v| v.as_str()) {
            cli.bg2 = Some(v.to_string());
        }
    }
    if !is_cli("gradient") {
        if let Some(v) = preset.get("gradient").and_then(|v| v.as_str()) {
            cli.gradient = v.to_string();
        }
    }
    if !is_cli("text_color") {
        if let Some(v) = preset
            .get("text_color")
            .or_else(|| preset.get("text-color"))
            .and_then(|v| v.as_str())
        {
            cli.text_color = v.to_string();
        }
    }
    if !is_cli("font_size") {
        if let Some(v) = preset
            .get("font_size")
            .or_else(|| preset.get("font-size"))
            .and_then(|v| v.as_f64())
        {
            cli.font_size = v as f32;
        }
    }
    if !is_cli("subtext_color") {
        if let Some(v) = preset
            .get("subtext_color")
            .or_else(|| preset.get("subtext-color"))
            .and_then(|v| v.as_str())
        {
            cli.subtext_color = Some(v.to_string());
        }
    }
    if !is_cli("pos") {
        if let Some(v) = preset.get("pos").and_then(|v| v.as_str()) {
            cli.pos = v.to_string();
        }
    }
    if !is_cli("border") {
        if let Some(v) = preset.get("border").and_then(|v| v.as_u64()) {
            cli.border = v as u32;
        }
    }
    if !is_cli("border_color") {
        if let Some(v) = preset
            .get("border_color")
            .or_else(|| preset.get("border-color"))
            .and_then(|v| v.as_str())
        {
            cli.border_color = v.to_string();
        }
    }
    if !is_cli("grid") {
        if let Some(v) = preset.get("grid").and_then(|v| v.as_u64()) {
            cli.grid = Some(v as u32);
        }
    }
    if !is_cli("radius") {
        if let Some(v) = preset.get("radius").and_then(|v| v.as_u64()) {
            cli.radius = v as u32;
        }
    }
    if !is_cli("scanlines") {
        if let Some(v) = preset.get("scanlines").and_then(|v| v.as_u64()) {
            cli.scanlines = Some(v as u32);
        }
    }
    if !is_cli("noise") {
        if let Some(v) = preset.get("noise").and_then(|v| v.as_u64()) {
            cli.noise = Some(v.min(255) as u8);
        }
    }
    if !is_cli("shadow") {
        if let Some(v) = preset.get("shadow").and_then(|v| v.as_str()) {
            cli.shadow = Some(v.to_string());
        }
    }

    Ok(())
}

/// JSON içindeki tüm renkleri katalog PNG görseli olarak çizer
fn generate_palette_sheet(
    palette: &HashMap<String, String>,
    output_path: &str,
    custom_font: Option<&PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if palette.is_empty() {
        return Err("Palet boş, katalog görseli oluşturulamadı.".into());
    }

    let font_bytes = load_font_bytes(custom_font)?;
    let font = FontRef::try_from_slice(&font_bytes)
        .map_err(|_| "Font dosyası ayrıştırılamadı (geçerli bir .ttf olmalı)")?;

    let mut entries: Vec<(&String, &String)> = palette.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let cols: u32 = (entries.len() as u32).min(4).max(1);
    let rows: u32 = (entries.len() as u32).div_ceil(cols);

    let margin: u32 = 32;
    let header_h: u32 = 64;
    let card_w: u32 = 260;
    let swatch_h: u32 = 84;
    let footer_h: u32 = 54;
    let card_h: u32 = swatch_h + footer_h;
    let gap: u32 = 20;

    let total_w = margin * 2 + cols * card_w + (cols - 1) * gap;
    let total_h = margin * 2 + header_h + rows * card_h + (rows - 1) * gap;

    let mut img = RgbaImage::from_pixel(total_w, total_h, Rgba([12, 12, 14, 255]));

    // Üst başlık
    let title = format!("TPCC PALETTE CATALOG ({} COLORS)", entries.len());
    draw_text_mut(
        &mut img,
        Rgba([230, 0, 0, 255]),
        margin as i32,
        margin as i32,
        PxScale::from(28.0),
        &font,
        &title,
    );

    let name_scale = PxScale::from(20.0);
    let hex_scale = PxScale::from(16.0);

    for (idx, (name, hex)) in entries.iter().enumerate() {
        let col = (idx as u32) % cols;
        let row = (idx as u32) / cols;

        let cx = margin + col * (card_w + gap);
        let cy = margin + header_h + row * (card_h + gap);

        let swatch_rgba = parse_hex_color(hex).unwrap_or(Rgba([128, 128, 128, 255]));

        // Renk kutusu
        draw_filled_rect_mut(
            &mut img,
            Rect::at(cx as i32, cy as i32).of_size(card_w, swatch_h),
            swatch_rgba,
        );
        // Alt bilgi kutusu
        draw_filled_rect_mut(
            &mut img,
            Rect::at(cx as i32, (cy + swatch_h) as i32).of_size(card_w, footer_h),
            Rgba([24, 24, 28, 255]),
        );
        // Kart dış çerçevesi
        draw_hollow_rect_mut(
            &mut img,
            Rect::at(cx as i32, cy as i32).of_size(card_w, card_h),
            Rgba([55, 55, 62, 255]),
        );

        draw_text_mut(
            &mut img,
            Rgba([245, 245, 245, 255]),
            (cx + 12) as i32,
            (cy + swatch_h + 7) as i32,
            name_scale,
            &font,
            name,
        );
        draw_text_mut(
            &mut img,
            Rgba([165, 165, 175, 255]),
            (cx + 12) as i32,
            (cy + swatch_h + 30) as i32,
            hex_scale,
            &font,
            &hex.to_uppercase(),
        );
    }

    img.save(output_path)?;
    println!(
        "Palet kataloğu oluşturuldu: {} ({} renk, {}x{})",
        output_path,
        entries.len(),
        total_w,
        total_h
    );
    Ok(())
}

/// "x,y,w,h,renk" formatında dikdörtgen çizer
fn draw_custom_rect(
    img: &mut RgbaImage,
    spec: &str,
    palette: &HashMap<String, String>,
) -> Result<(), String> {
    let parts: Vec<&str> = spec.split(',').map(|s| s.trim()).collect();
    if parts.len() != 5 {
        return Err(format!(
            "--rect formatı 'x,y,genislik,yukseklik,renk' olmalı (verilen: '{}')",
            spec
        ));
    }

    let rx: i32 = parts[0].parse().map_err(|_| "Geçersiz rect X")?;
    let ry: i32 = parts[1].parse().map_err(|_| "Geçersiz rect Y")?;
    let rw: u32 = parts[2].parse().map_err(|_| "Geçersiz rect genişlik")?;
    let rh: u32 = parts[3].parse().map_err(|_| "Geçersiz rect yükseklik")?;
    let color = resolve_color(parts[4], palette)?;

    if rw == 0 || rh == 0 {
        return Ok(());
    }

    if color[3] == 255 {
        draw_filled_rect_mut(img, Rect::at(rx, ry).of_size(rw, rh), color);
    } else {
        let img_w = img.width() as i32;
        let img_h = img.height() as i32;
        let x0 = rx.max(0);
        let y0 = ry.max(0);
        let x1 = (rx + rw as i32).min(img_w);
        let y1 = (ry + rh as i32).min(img_h);
        for y in y0..y1 {
            for x in x0..x1 {
                blend_pixel(img, x as u32, y as u32, color);
            }
        }
    }
    Ok(())
}

/// "cx,cy,yaricap,renk" formatında yumuşak kenarlı daire çizer
fn draw_custom_circle(
    img: &mut RgbaImage,
    spec: &str,
    palette: &HashMap<String, String>,
) -> Result<(), String> {
    let parts: Vec<&str> = spec.split(',').map(|s| s.trim()).collect();
    if parts.len() != 4 {
        return Err(format!(
            "--circle formatı 'cx,cy,yaricap,renk' olmalı (verilen: '{}')",
            spec
        ));
    }

    let cx: f32 = parts[0].parse().map_err(|_| "Geçersiz circle X")?;
    let cy: f32 = parts[1].parse().map_err(|_| "Geçersiz circle Y")?;
    let r: f32 = parts[2].parse().map_err(|_| "Geçersiz circle yarıçap")?;
    let color = resolve_color(parts[3], palette)?;

    if r <= 0.0 {
        return Ok(());
    }

    let min_x = ((cx - r - 1.0).floor() as i32).max(0);
    let max_x = ((cx + r + 1.0).ceil() as i32).min(img.width() as i32 - 1);
    let min_y = ((cy - r - 1.0).floor() as i32).max(0);
    let max_y = ((cy + r + 1.0).ceil() as i32).min(img.height() as i32 - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = (x as f32 + 0.5) - cx;
            let dy = (y as f32 + 0.5) - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= r + 0.5 {
                let aa = (r + 0.5 - dist).clamp(0.0, 1.0);
                let effective_alpha = ((color[3] as f32) * aa).round() as u8;
                blend_pixel(
                    img,
                    x as u32,
                    y as u32,
                    Rgba([color[0], color[1], color[2], effective_alpha]),
                );
            }
        }
    }
    Ok(())
}

/// Görsele hafif deterministik kumlanma/doku uygular
fn apply_noise(img: &mut RgbaImage, amount: u8) {
    let mut state: u64 = 0x5450_4343_4E4F_4953; // "TPCCNOIS"
    let amp = amount as i32;
    let span = (amp * 2 + 1) as u64;

    for pixel in img.pixels_mut() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let delta = (state % span) as i32 - amp;

        pixel[0] = (pixel[0] as i32 + delta).clamp(0, 255) as u8;
        pixel[1] = (pixel[1] as i32 + delta).clamp(0, 255) as u8;
        pixel[2] = (pixel[2] as i32 + delta).clamp(0, 255) as u8;
    }
}

/// Görselin 4 köşesini yumuşak kenarlı olarak şeffaf yuvarlar
fn apply_rounded_corners(img: &mut RgbaImage, radius: u32) {
    let w = img.width() as f32;
    let h = img.height() as f32;
    let r = (radius as f32).min(w / 2.0).min(h / 2.0);
    if r <= 0.0 {
        return;
    }

    for y in 0..img.height() {
        for x in 0..img.width() {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let corner = if px < r && py < r {
                Some((r, r))
            } else if px > w - r && py < r {
                Some((w - r, r))
            } else if px < r && py > h - r {
                Some((r, h - r))
            } else if px > w - r && py > h - r {
                Some((w - r, h - r))
            } else {
                None
            };

            if let Some((cx, cy)) = corner {
                let dist = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
                if dist > r + 0.5 {
                    img.get_pixel_mut(x, y)[3] = 0;
                } else if dist > r - 0.5 {
                    let factor = (r + 0.5 - dist).clamp(0.0, 1.0);
                    let p = img.get_pixel_mut(x, y);
                    p[3] = ((p[3] as f32) * factor).round() as u8;
                }
            }
        }
    }
}

/// Renk adını JSON haritasından bulur veya doğrudan #HEX kodunu Rgba'ya çevirir
fn resolve_color(input: &str, palette: &HashMap<String, String>) -> Result<Rgba<u8>, String> {
    let raw_hex = if input.starts_with('#') {
        input.to_string()
    } else {
        palette
            .get(&input.to_lowercase())
            .cloned()
            .ok_or_else(|| format!("'{}' rengi JSON paletinde bulunamadı!", input))?
    };

    parse_hex_color(&raw_hex)
}

fn parse_hex_color(hex: &str) -> Result<Rgba<u8>, String> {
    let cleaned = hex.trim().trim_start_matches('#');
    match cleaned.len() {
        6 => {
            let r = u8::from_str_radix(&cleaned[0..2], 16).map_err(|_| "Geçersiz HEX")?;
            let g = u8::from_str_radix(&cleaned[2..4], 16).map_err(|_| "Geçersiz HEX")?;
            let b = u8::from_str_radix(&cleaned[4..6], 16).map_err(|_| "Geçersiz HEX")?;
            Ok(Rgba([r, g, b, 255]))
        }
        8 => {
            let r = u8::from_str_radix(&cleaned[0..2], 16).map_err(|_| "Geçersiz HEX")?;
            let g = u8::from_str_radix(&cleaned[2..4], 16).map_err(|_| "Geçersiz HEX")?;
            let b = u8::from_str_radix(&cleaned[4..6], 16).map_err(|_| "Geçersiz HEX")?;
            let a = u8::from_str_radix(&cleaned[6..8], 16).map_err(|_| "Geçersiz HEX")?;
            Ok(Rgba([r, g, b, a]))
        }
        _ => Err(format!("Desteklenmeyen HEX uzunluğu: {}", hex)),
    }
}

fn parse_dimensions(dim: &str) -> Result<(u32, u32), String> {
    let (w_str, h_str) = dim
        .trim()
        .split_once(['x', 'X'])
        .ok_or("Boyut formatı GENISLIKxYUKSEKLIK olmalı (örn: 1280x720)")?;
    let w: u32 = w_str.trim().parse().map_err(|_| "Geçersiz genişlik")?;
    let h: u32 = h_str.trim().parse().map_err(|_| "Geçersiz yükseklik")?;
    Ok((w, h))
}

fn calculate_text_position(pos: &str, w: u32, h: u32, tw: u32, th: u32, pad: u32) -> (i32, i32) {
    let margin = (pad + 20) as i32;
    match pos.to_lowercase().as_str() {
        "center" => (((w as i32 - tw as i32) / 2), ((h as i32 - th as i32) / 2)),
        "top-left" => (margin, margin),
        "top-right" => (w as i32 - tw as i32 - margin, margin),
        "bottom-left" => (margin, h as i32 - th as i32 - margin),
        "bottom-right" => (w as i32 - tw as i32 - margin, h as i32 - th as i32 - margin),
        custom => {
            if let Some((x_str, y_str)) = custom.split_once(',') {
                let x = x_str.trim().parse().unwrap_or(margin);
                let y = y_str.trim().parse().unwrap_or(margin);
                (x, y)
            } else {
                (((w as i32 - tw as i32) / 2), ((h as i32 - th as i32) / 2))
            }
        }
    }
}

fn lerp_rgba(c1: Rgba<u8>, c2: Rgba<u8>, t: f32) -> Rgba<u8> {
    let mix = |a: u8, b: u8| -> u8 { (a as f32 + (b as f32 - a as f32) * t).round() as u8 };
    Rgba([
        mix(c1[0], c2[0]),
        mix(c1[1], c2[1]),
        mix(c1[2], c2[2]),
        mix(c1[3], c2[3]),
    ])
}

fn blend_pixel(img: &mut RgbaImage, x: u32, y: u32, overlay: Rgba<u8>) {
    let base = img.get_pixel(x, y);
    let alpha = overlay[3] as f32 / 255.0;
    let inv = 1.0 - alpha;
    let r = (overlay[0] as f32 * alpha + base[0] as f32 * inv) as u8;
    let g = (overlay[1] as f32 * alpha + base[1] as f32 * inv) as u8;
    let b = (overlay[2] as f32 * alpha + base[2] as f32 * inv) as u8;
    img.put_pixel(x, y, Rgba([r, g, b, 255]));
}

fn load_font_bytes(custom_path: Option<&PathBuf>) -> Result<Vec<u8>, String> {
    if let Some(p) = custom_path {
        return fs::read(p).map_err(|e| format!("Font okunamadı ({:?}): {}", p, e));
    }

    let candidates = [
        r"C:\Windows\Fonts\consola.ttf",
        r"C:\Windows\Fonts\segoeui.ttf",
        r"C:\Windows\Fonts\arial.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
    ];

    for path in candidates {
        if let Ok(bytes) = fs::read(path) {
            return Ok(bytes);
        }
    }

    Err("Sistemde varsayılan TTF fontu bulunamadı. Lütfen --font <dosya.ttf> belirtin.".into())
}