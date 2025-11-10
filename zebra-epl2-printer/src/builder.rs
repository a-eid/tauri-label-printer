use crate::consts::*;
use crate::graphics::{render_arabic_line_bold_right, rotate90};
use crate::epl::{epl_line, image_to_row_bytes, gw_bytes};

fn center_x_for_ean13(label_w: u32, narrow: u32) -> u32 {
    let modules = 95i32; // EAN-13 modules
    let w = modules * narrow as i32;
    ((label_w as i32 - w) / 2).max(0) as u32
}

pub fn build_two_product_label_clean_centered(
    font_bytes: &[u8],
    p1_name: &str, p1_price: &str, p1_barcode: &str,
    p2_name: &str, p2_price: &str, p2_barcode: &str,
) -> Vec<u8> {
    // Arabic + currency
    let t1 = format!("{}    {} {}", p1_name, p1_price, "ج.م");
    let t2 = format!("{}    {} {}", p2_name, p2_price, "ج.م");

    // Render text (large, bold, right-aligned), tight height
    let mut im1 = render_arabic_line_bold_right(&t1, font_bytes, LABEL_W, PAD_R, FONT_PX);
    let mut im2 = render_arabic_line_bold_right(&t2, font_bytes, LABEL_W, PAD_R, FONT_PX);

    // If driver is Landscape, rotate our bitmaps and later swap coordinates
    if FORCE_LANDSCAPE {
        im1 = rotate90(&im1);
        im2 = rotate90(&im2);
    }

    let (w1,h1,r1) = image_to_row_bytes(&im1);
    let (w2,h2,r2) = image_to_row_bytes(&im2);

    // layout Y’s (ensures HRI fits)
    let text1_y = TOP_MARGIN;
    let bc1_y   = TOP_MARGIN + h1 as u32 + 18;      // leave gap below text
    let text2_y = bc1_y + HEIGHT + 28;     // below first barcode
    let bc2_y   = text2_y + h2 as u32 + 18;

    // center barcode
    let bx = center_x_for_ean13(LABEL_W, NARROW);

    let mut buf = Vec::new();
    epl_line(&mut buf, "N");
    epl_line(&mut buf, &format!("q{}", LABEL_W));
    epl_line(&mut buf, &format!("Q{},24", LABEL_H)); // adjust gap if needed
    epl_line(&mut buf, &format!("D{}", DARKNESS));
    epl_line(&mut buf, &format!("S{}", SPEED));

    if !FORCE_LANDSCAPE {
        gw_bytes(&mut buf, LEFT_MARGIN, text1_y, w1, h1, &r1);
        epl_line(&mut buf, &format!("B{},{},0,1,{},{},{},B,\"{}\"", bx, bc1_y, NARROW, 4, HEIGHT, p1_barcode));

        gw_bytes(&mut buf, LEFT_MARGIN, text2_y, w2, h2, &r2);
        epl_line(&mut buf, &format!("B{},{},0,1,{},{},{},B,\"{}\"", bx, bc2_y, NARROW, 4, HEIGHT, p2_barcode));
    } else {
        // landscape compensation: swap x/y when placing
        gw_bytes(&mut buf, text1_y, LEFT_MARGIN, w1, h1, &r1);
        epl_line(&mut buf, &format!("B{},{},1,1,{},{},{},B,\"{}\"", bc1_y, bx, NARROW, 4, HEIGHT, p1_barcode)); // rotation=1 (90°)

        gw_bytes(&mut buf, text2_y, LEFT_MARGIN, w2, h2, &r2);
        epl_line(&mut buf, &format!("B{},{},1,1,{},{},{},B,\"{}\"", bc2_y, bx, NARROW, 4, HEIGHT, p2_barcode));
    }

    epl_line(&mut buf, "P1");
    buf
}
