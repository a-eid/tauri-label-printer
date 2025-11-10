use image::ImageBuffer;
use image::Luma;
use crate::zebra_epl2_printer::graphics::{render_arabic_line, image_to_gw};
use crate::zebra_epl2_printer::barcode::epl_ean13;
use crate::zebra_epl2_printer::epl;

/// Build an EPL2 buffer containing two product blocks on a 440x320 label.
pub fn build_two_product_label(
    p1: &str,
    p1_price: &str,
    p1_barcode: &str,
    p2: &str,
    p2_price: &str,
    p2_barcode: &str,
    font: &[u8],
) -> Vec<u8> {
    let mut out = epl::header();

    // Render product 1 text area (width 440, height ~60)
    let img1: ImageBuffer<Luma<u8>, Vec<u8>> = render_arabic_line(p1, font, 440, 60);
    let mut gw1 = image_to_gw(0, 10, &img1);
    out.append(&mut gw1);

    // Barcode for product 1
    let b1 = epl_ean13(0, 90, p1_barcode, 80);
    out.extend_from_slice(&b1);

    // Product 2
    let img2: ImageBuffer<Luma<u8>, Vec<u8>> = render_arabic_line(p2, font, 440, 60);
    let mut gw2 = image_to_gw(0, 170, &img2);
    out.append(&mut gw2);

    let b2 = epl_ean13(0, 250, p2_barcode, 80);
    out.extend_from_slice(&b2);

    // End with print command already in header (P1), or we keep N to clear.
    out
}
