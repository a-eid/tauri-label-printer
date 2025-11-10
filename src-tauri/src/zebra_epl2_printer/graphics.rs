use image::{ImageBuffer, Luma};
use rusttype::{Font, Scale, point};
use ar_reshaper::arabic_reshaper::reshape;
use unicode_bidi::BidiInfo;

/// Render an Arabic line into a grayscale image buffer (Luma8).
pub fn render_arabic_line(text: &str, font_data: &[u8], width: u32, height: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    // Reshape Arabic and apply bidi.
    let reshaped = reshape(text);
    let bidi = BidiInfo::new(&reshaped, None);
    let para = &bidi.paragraphs[0];
    let display = bidi.reorder_line(para, 0, reshaped.len());

    let font = Font::try_from_bytes(font_data).expect("invalid font data");
    let scale = Scale::uniform((height as f32) * 0.18); // heuristic

    let mut img = ImageBuffer::from_pixel(width, height, Luma([255u8]));

    // Simple left-to-right drawing of the reordered string; Arabic shaping already applied.
    let v_metrics = font.v_metrics(scale);
    let mut x = 10.0f32;
    // place baseline at y start
    let y = 10.0 + v_metrics.ascent;

    for ch in display.chars() {
        if ch.is_control() { continue }
        let glyph = font.glyph(ch).scaled(scale).positioned(point(x, y));
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let px = gx as i32 + bb.min.x;
                let py = gy as i32 + bb.min.y;
                if px >= 0 && py >= 0 && (px as u32) < width && (py as u32) < height {
                    let cur = img.get_pixel_mut(px as u32, py as u32);
                    let existing = cur.0[0] as f32;
                    // blend: darker for glyph coverage
                    let alpha = (1.0 - v) * existing + v * 0.0;
                    cur.0[0] = alpha as u8;
                }
            });
        }
        x += glyph.unpositioned().h_metrics().advance_width;
    }

    img
}

/// Convert a grayscale image to EPL2 GW command bytes. Returns Vec<u8> containing commands + binary.
pub fn image_to_gw(x: u32, y: u32, img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
    let width = img.width();
    let height = img.height();
    let bytes_per_row = ((width + 7) / 8) as usize;
    let mut data: Vec<u8> = Vec::with_capacity((bytes_per_row * height as usize) + 128);

    // GW x,y,w,h,<binary>
    // We'll build the header as ASCII, then append binary.
    let header = format!("GW {},{}, {},{},\n", x, y, width, height);
    data.extend_from_slice(header.as_bytes());

    for row in 0..height {
        let mut byte = 0u8;
        let mut bit = 0u8;
        for col in 0..width {
            let pix = img.get_pixel(col, row).0[0];
            let black = pix < 128;
            byte <<= 1;
            if black { byte |= 1; }
            bit += 1;
            if bit == 8 {
                data.push(byte);
                byte = 0;
                bit = 0;
            }
        }
        if bit != 0 {
            // pad remaining bits
            byte <<= 8 - bit;
            data.push(byte);
        }
    }

    // End of graphic command
    data.extend_from_slice(b"\n");
    data
}
