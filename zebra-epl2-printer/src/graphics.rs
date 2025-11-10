use rusttype::{Font, Scale, point};
use image::{GrayImage, Luma};

/// Render text into a grayscale image with simple thresholding.
/// This implementation uses rusttype to position glyphs left-to-right,
/// then aligns the whole line to the right side of the image so it works
/// for RTL languages without requiring external bidi/reshaper crates here.
pub fn render_text_to_bitmap(text: &str, font_data: &[u8], size: f32, width: u32, height: u32) -> GrayImage {
    // load font
    let font = Font::try_from_bytes(font_data).expect("failed to load font");
    let scale = Scale::uniform(size);

    // create image buffer (white background)
    let mut image = GrayImage::from_pixel(width, height, Luma([255u8]));

    // vertical metrics
    let v_metrics = font.v_metrics(scale);
    let baseline = v_metrics.ascent;
    let y = baseline.ceil() as i32 + 2;

    // build glyphs and compute approximate total advance width
    let mut glyphs = Vec::new();
    let mut total_advance: f32 = 0.0;
    for ch in text.chars() {
        let g = font.glyph(ch).scaled(scale);
        let adv = g.h_metrics().advance_width;
        glyphs.push((g, adv));
        total_advance += adv;
    }

    // starting x so the text is right-aligned with a small margin
    let mut x_cursor = (width as f32 - total_advance - 2.0).max(2.0);

    for (g, adv) in glyphs {
        let positioned = g.positioned(point(x_cursor, y as f32));
        if let Some(bb) = positioned.pixel_bounding_box() {
            let bb_min_x = bb.min.x;
            let bb_min_y = bb.min.y;
            positioned.draw(|gx, gy, v| {
                let px = bb_min_x + gx as i32;
                let py = bb_min_y + gy as i32;
                if px >= 0 && py >= 0 && (px as u32) < width && (py as u32) < height {
                    let alpha = (v * 255.0) as u8;
                    let curr = image.get_pixel(px as u32, py as u32)[0];
                    // simple alpha blend towards black
                    let new = ((curr as u16 * (255 - alpha) as u16) / 255) as u8;
                    image.put_pixel(px as u32, py as u32, Luma([new]));
                }
            });
        }
        x_cursor += adv;
    }

    // threshold to 1-bit style (keep as grayscale Luma)
    for px in image.pixels_mut() {
        *px = if px[0] < 200 { Luma([0u8]) } else { Luma([255u8]) };
    }

    image
}
