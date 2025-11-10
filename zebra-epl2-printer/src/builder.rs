use crate::graphics::render_text_to_bitmap;
use crate::barcode::normalize_ean13;
use crate::epl::image_to_gw;

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
    // label canvas
    let label_w = 440u32;
    let label_h = 320u32;

    // render product images (rough layout)
    let prod_h = label_h / 2;
    let img1 = render_text_to_bitmap(&format!("{}\n{}", p1_name, p1_price), font_data, 28.0, label_w, prod_h);
    let img2 = render_text_to_bitmap(&format!("{}\n{}", p2_name, p2_price), font_data, 28.0, label_w, prod_h);

    // assemble EPL job
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(b"N\r\nq440\r\nQ320,24\r\nD10\r\n");

    // product 1 at y=0
    out.extend_from_slice(&image_to_gw(0, 0, &img1));
    // barcode line under product 1; use normalized code
    let b1 = normalize_ean13(p1_barcode.to_string()).unwrap_or_else(|_| p1_barcode.to_string());
    let barcode_line1 = format!("B20,90,0,1,2,4,60,B,\"{}\"\r\n", b1);
    out.extend_from_slice(barcode_line1.as_bytes());

    // product 2 at y = prod_h (leave some spacing)
    out.extend_from_slice(&image_to_gw(0, prod_h - 10, &img2));
    let b2 = normalize_ean13(p2_barcode.to_string()).unwrap_or_else(|_| p2_barcode.to_string());
    let barcode_line2 = format!("B20,250,0,1,2,4,60,B,\"{}\"\r\n", b2);
    out.extend_from_slice(barcode_line2.as_bytes());

    out.extend_from_slice(b"P1\r\n");
    out
}
