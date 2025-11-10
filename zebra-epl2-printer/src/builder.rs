use crate::graphics::render_arabic_line_1bit;
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
