// Backwards-compat: expose the new right-aligned renderer under the old name
use crate::graphics::render_arabic_line_1bit_right as render_arabic_line_1bit;
use crate::barcode::normalize_ean13;
use crate::epl::{image_to_row_bytes, gw_bytes, epl_line};

/// Build a two-product EPL2 label as raw bytes ready to send to printer.
/// Sizes are in dots (203 dpi): the example requested ~440x320 for full label.
pub fn build_two_product_label(
    p1_name: &str,
    p1_price: &str,
    p1_barcode: &str,
    p2_name: &str,
    p2_price: &str,
    p2_barcode: &str,
    font_data: &[u8],
) -> Vec<u8> {
    // label canvas and render tight Arabic lines
    let label_w = 440u32;

    // Build two tight images with larger font size for readability
    let img1 = render_arabic_line_1bit(&format!("{}    {}", p1_name, p1_price), font_data, label_w, 12, 30.0);
    let img2 = render_arabic_line_1bit(&format!("{}    {}", p2_name, p2_price), font_data, label_w, 12, 30.0);

    let (w1, h1, rows1) = image_to_row_bytes(&img1);
    let (w2, h2, rows2) = image_to_row_bytes(&img2);

    // assemble EPL job using CRLF and P1 terminator
    let mut out: Vec<u8> = Vec::new();
    epl_line(&mut out, "N");
    epl_line(&mut out, "q440");
    epl_line(&mut out, "Q320,24");
    epl_line(&mut out, "D7"); // reduce darkness to avoid fill

    // product 1 text at (10,10)
    gw_bytes(&mut out, 10, 10, w1, h1, &rows1);
    let b1 = normalize_ean13(p1_barcode.to_string()).unwrap_or_else(|_| p1_barcode.to_string());
    epl_line(&mut out, &format!("B20,90,0,1,2,4,60,B,\"{}\"", b1));

    // product 2 text at (10, 175)
    gw_bytes(&mut out, 10, 175, w2, h2, &rows2);
    let b2 = normalize_ean13(p2_barcode.to_string()).unwrap_or_else(|_| p2_barcode.to_string());
    epl_line(&mut out, &format!("B20,255,0,1,2,4,60,B,\"{}\"", b2));

    epl_line(&mut out, "P1");
    out
}

// Center EAN13: width = modules * narrow, modules = 95
fn center_x_for_ean13(label_w: u32, narrow: u32) -> u32 {
    let barcode_w = 95u32.saturating_mul(narrow);
    if barcode_w >= label_w { 0 } else { (label_w - barcode_w) / 2 }
}

/// Build a centered, clean two-product label with larger Arabic text, centered EAN-13, and tuned barcodes.
pub fn build_two_product_label_clean_centered(
    font_bytes: &[u8],
    p1_name: &str,
    p1_price: &str,
    p1_barcode: &str,
    p2_name: &str,
    p2_price: &str,
    p2_barcode: &str,
) -> Vec<u8> {
    // currency fix: use Arabic currency symbol
    let t1 = format!("{}    {} {}", p1_name, p1_price, "ج.م");
    let t2 = format!("{}    {} {}", p2_name, p2_price, "ج.م");

    // Render tight images with larger font
    let label_w = 440u32;
    let img1 = render_arabic_line_1bit_right(&t1, font_bytes, label_w, 10, 36.0);
    let img2 = render_arabic_line_1bit_right(&t2, font_bytes, label_w, 10, 36.0);

    let (_w1, h1, r1) = image_to_row_bytes(&img1);
    let (_w2, h2, r2) = image_to_row_bytes(&img2);

    // barcode tuning (avoid clipping & center)
    let narrow: u32 = 2; // 2-3 recommended
    let wide: u32 = 4;
    let bc_height: u32 = 52; // reduce to allow HRI
    let x_center = center_x_for_ean13(label_w, narrow);

    let mut buf = Vec::new();
    epl_line(&mut buf, "N");
    epl_line(&mut buf, "q440");
    epl_line(&mut buf, "Q320,24");
    epl_line(&mut buf, "D6"); // lighter to reduce banding
    epl_line(&mut buf, "S3"); // slower = cleaner

    // Block 1
    gw_bytes(&mut buf, 10, 8, 440, h1, &r1);
    epl_line(&mut buf, &format!("B{},{},0,1,{},{},{},B,\"{}\"", x_center, 88, narrow, wide, bc_height, p1_barcode));

    // Block 2
    gw_bytes(&mut buf, 10, 172, 440, h2, &r2);
    epl_line(&mut buf, &format!("B{},{},0,1,{},{},{},B,\"{}\"", x_center, 232, narrow, wide, bc_height, p2_barcode));

    epl_line(&mut buf, "P1");
    buf
}

/// New clean builder that takes combined text strings (Arabic + price) directly
pub fn build_two_product_label_clean(
    font_data: &[u8],
    p1_text_ar_price: &str,
    p1_barcode: &str,
    p2_text_ar_price: &str,
    p2_barcode: &str,
) -> Vec<u8> {
    // Render tight images
    let label_w = 440u32;
    let img1 = render_arabic_line_1bit(p1_text_ar_price, font_data, label_w, 12, 30.0);
    let img2 = render_arabic_line_1bit(p2_text_ar_price, font_data, label_w, 12, 30.0);

    let (w1, h1, rows1) = image_to_row_bytes(&img1);
    let (w2, h2, rows2) = image_to_row_bytes(&img2);

    let mut out = Vec::new();
    epl_line(&mut out, "N");
    epl_line(&mut out, "q440");
    epl_line(&mut out, "Q320,24");
    epl_line(&mut out, "D7");

    gw_bytes(&mut out, 10, 10, w1, h1, &rows1);
    epl_line(&mut out, &format!("B20,90,0,1,2,4,60,B,\"{}\"", p1_barcode));

    gw_bytes(&mut out, 10, 175, w2, h2, &rows2);
    epl_line(&mut out, &format!("B20,255,0,1,2,4,60,B,\"{}\"", p2_barcode));

    epl_line(&mut out, "P1");
    out
}
