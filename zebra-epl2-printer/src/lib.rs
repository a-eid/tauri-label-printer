//! Self-contained EPL2 label builder for Zebra LP-2824 (203 dpi).
//! - Fixes Arabic shaping + direction
//! - Renders tight 1-bit bitmaps (no gray strip)
//! - Optional bit inversion for GW polarity
//! - Compensates driver Landscape by rotating in code
//! - Centers EAN-13 barcodes and keeps HRI visible

use image::{ImageBuffer, Luma, DynamicImage};
use rusttype::{Font, Scale, point};
use ar_reshaper::{ArabicReshaper, ReshaperConfig};
use unicode_bidi::BidiInfo;

// ======== Config (edit if needed) ========

const LABEL_W: u32 = 440;          // dots (≈55 mm)
const LABEL_H: u32 = 320;          // dots (≈40 mm)
const PAD_RIGHT: u32 = 10;

const FONT_PX: f32 = 52.0;         // even larger for better readability
const BOLD_STROKE: bool = true;    // draw twice w/ 1px offset

const DARKNESS: u8 = 6;            // D0..D15 (tuned to reduce banding)
const SPEED: u8 = 3;               // S1..S6

const NARROW: u32 = 3;             // EAN13 module width (increased for better scanning)
const HEIGHT: u32 = 60;            // barcode bar height (increased for better scanning)

const FORCE_LANDSCAPE: bool = false; // Driver should be Portrait
const INVERT_BITS: bool = true;      // Invert GW bits for black-on-white

// ======== Public API ========

/// Build a single EPL2 print job for two products.
/// - `font_bytes`: embedded Arabic font bytes (e.g., Amiri-Regular.ttf or NotoNaskhArabic-Regular.ttf)
/// - `name1/price1/barcode1` + `name2/price2/barcode2`
/// Returns raw bytes ready to send to the printer (USB raw write).
pub fn build_two_product_label(
    font_bytes: &[u8],
    name1: &str, price1: &str, barcode1: &str,
    name2: &str, price2: &str, barcode2: &str,
) -> Vec<u8> {
    // Compose Arabic lines with currency "ج.م"
    let t1 = format!("{}    {} {}", name1, price1, "ج.م");
    let t2 = format!("{}    {} {}", name2, price2, "ج.م");

    // Ensure barcodes are valid EAN-13 format
    let bc1 = ensure_valid_ean13(barcode1);
    let bc2 = ensure_valid_ean13(barcode2);

    // Render tight, bold, 1-bit Arabic lines
    let mut im1 = render_arabic_line_tight_1bit(&t1, font_bytes, FONT_PX, 3, BOLD_STROKE);
    let mut im2 = render_arabic_line_tight_1bit(&t2, font_bytes, FONT_PX, 3, BOLD_STROKE);

    // No rotation needed - driver should be in Portrait mode
    // if FORCE_LANDSCAPE { im1 = rotate90(&im1); im2 = rotate90(&im2); }

    let (w1,h1,r1) = image_to_row_bytes(&im1);
    let (w2,h2,r2) = image_to_row_bytes(&im2);

    // Right-align text images (tight width → no gray band)
    let x1 = LABEL_W - PAD_RIGHT - w1;
    let x2 = LABEL_W - PAD_RIGHT - w2;

    // Y layout with space for HRI - both products on ONE label
    let text1_y = 10;
    let bc1_y   = text1_y + h1 + 8;
    let text2_y = bc1_y   + HEIGHT + 35;  // More spacing to avoid overlap
    let bc2_y   = text2_y + h2 + 8;

    // Center EAN-13 (95 modules)
    let bx = center_x_for_ean13(LABEL_W, NARROW);

    let mut buf = Vec::<u8>::new();
    epl_line(&mut buf, "N");
    epl_line(&mut buf, &format!("q{}", LABEL_W));
    epl_line(&mut buf, &format!("Q{},24", LABEL_H));
    epl_line(&mut buf, &format!("D{}", DARKNESS));
    epl_line(&mut buf, &format!("S{}", SPEED));

    // Always portrait mode - both products on same label
    gw_bytes(&mut buf, x1, text1_y, w1, h1, &r1);
    epl_line(&mut buf, &format!("B{},{},0,1,{},{},{},N,\"{}\"",
        bx, bc1_y, NARROW, NARROW, HEIGHT, bc1));

    // Dotted separator line between products (moved down ~2-3mm)
    let separator_y = bc1_y + HEIGHT + 32;  // ~1/4 cm more space
    draw_dotted_line(&mut buf, 20, separator_y, LABEL_W - 40);

    gw_bytes(&mut buf, x2, text2_y, w2, h2, &r2);
    epl_line(&mut buf, &format!("B{},{},0,1,{},{},{},N,\"{}\"",
        bx, bc2_y, NARROW, NARROW, HEIGHT, bc2));

    epl_line(&mut buf, "P1");  // Print exactly ONE label
    buf
}

// ======== Arabic rendering ========

/// Visual-order string: BiDi runs; reshape only RTL runs.
fn bidi_then_shape(text: &str, reshaper: &ArabicReshaper) -> String {
    let info = BidiInfo::new(text, None);
    let para = &info.paragraphs[0];
    let (levels, ranges) = info.visual_runs(para, para.range.clone());

    let mut out = String::new();
    // Visual order runs; reshape RTL runs only, preserve LTR (digits) order
    for (level, range) in levels.into_iter().zip(ranges.into_iter()) {
        let slice = &text[range];
        if level.is_rtl() {
            // Only reverse if it's actually Arabic text (not digits/punctuation)
            let shaped = reshaper.reshape(slice);
            // Check if the slice contains Arabic letters vs just digits/symbols
            if slice.chars().any(|c| c >= '\u{0600}' && c <= '\u{06FF}') {
                // Contains Arabic - reverse after shaping
                let reversed: String = shaped.chars().rev().collect();
                out.push_str(&reversed);
            } else {
                // Just numbers/punctuation - don't reverse
                out.push_str(&shaped);
            }
        } else {
            out.push_str(slice);
        }
    }
    out
}

/// Render tight 1-bit Arabic line; optional 1-px stroke for bold.
fn render_arabic_line_tight_1bit(
    text: &str,
    font_bytes: &[u8],
    font_px: f32,
    pad_lr: u32,
    bold: bool,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let font = Font::try_from_bytes(font_bytes).expect("bad font");
    let reshaper = ArabicReshaper::new(ReshaperConfig::default());
    let visual = bidi_then_shape(text, &reshaper);

    let scale = Scale { x: font_px, y: font_px };
    let vm = font.v_metrics(scale);
    let ascent = vm.ascent.ceil();
    let descent = vm.descent.floor();
    let line_h = (ascent - descent).ceil().max(30.0) as u32;

    // Measure tight width
    let glyphs: Vec<_> = font.layout(&visual, scale, point(0.0, ascent)).collect();
    let text_w = glyphs.iter().rev()
        .find_map(|g| g.pixel_bounding_box().map(|bb| bb.max.x as f32))
        .unwrap_or(0.0).ceil() as u32;

    let w = (text_w + pad_lr * 2).max(2);
    let mut img = ImageBuffer::from_pixel(w, line_h, Luma([255]));

    let passes: &[(i32,i32)] = if bold { &[(0,0),(1,0)] } else { &[(0,0)] };
    for (dx, dy) in passes {
        for g in font.layout(&visual, scale, point(pad_lr as f32 + *dx as f32, ascent + *dy as f32)) {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    if v > 0.65 {
                        let px = x + bb.min.x as u32;
                        let py = y + bb.min.y as u32;
                        if px < w && py < line_h { img.put_pixel(px, py, Luma([0])); }
                    }
                });
            }
        }
    }
    img
}

fn rotate90(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    DynamicImage::ImageLuma8(img.clone()).rotate90().to_luma8()
}

// ======== EPL2 helpers (binary GW + CRLF, optional invert) ========

fn epl_line(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(s.as_bytes());
    buf.extend_from_slice(b"\r\n");
}

fn image_to_row_bytes(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> (u32,u32,Vec<u8>) {
    let (w,h) = (img.width(), img.height());
    let bpr = ((w + 7)/8) as usize;
    let mut out = vec![0u8; bpr*h as usize];

    for y in 0..h {
        for x in 0..w {
            if img.get_pixel(x,y).0[0] < 128 {
                let i = y as usize * bpr + (x as usize / 8);
                out[i] |= 1 << (7 - (x as usize % 8));
            }
        }
    }
    if INVERT_BITS { for b in &mut out { *b = !*b; } }
    (w,h,out)
}

fn gw_bytes(buf:&mut Vec<u8>, x:u32, y:u32, w:u32, h:u32, rows:&[u8]) {
    let bpr = ((w+7)/8) as usize;
    epl_line(buf, &format!("GW{},{},{},{}", x,y,bpr,h));
    buf.extend_from_slice(rows);  // RAW binary
    buf.extend_from_slice(b"\r\n");
}

// ======== Utility ========

fn center_x_for_ean13(label_w: u32, narrow: u32) -> u32 {
    let w = 95 * narrow; // EAN-13 total width (95 modules)
    (label_w - w) / 2
}

fn draw_dotted_line(buf: &mut Vec<u8>, start_x: u32, y: u32, width: u32) {
    // Draw dotted line using EPL LO command (Line Oblique)
    let dot_length = 8;  // Length of each dot segment
    let gap_length = 6;  // Gap between dots
    let total_pattern = dot_length + gap_length;
    
    let mut x = start_x;
    while x + dot_length < start_x + width {
        // LO command: LO x,y,thickness,width
        epl_line(buf, &format!("LO{},{},1,{}", x, y, dot_length));
        x += total_pattern;
    }
}

// Ensure barcode is valid 12-digit EAN-13 (without check digit)
fn ensure_valid_ean13(barcode: &str) -> String {
    let digits: String = barcode.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() >= 12 {
        // Take first 12 digits (EPL2 will calculate check digit)
        digits[..12].to_string()
    } else if digits.len() == 13 {
        // If 13 digits provided, use first 12 (remove check digit)
        digits[..12].to_string()
    } else {
        // Pad with zeros to make 12 digits
        format!("{:0<12}", digits)
    }
}

// ======== Windows printer (optional, keep if you need send_raw_to_printer) ========

#[cfg(target_os = "windows")]
pub mod printer;

#[cfg(target_os = "windows")]
pub use printer::send_raw_to_printer;
