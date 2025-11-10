use ar_reshaper::arabic::ArabicReshaper;
use rusttype::{Font, Scale, point};
use unicode_bidi::BidiInfo;
use image::{GrayImage, Luma};

/// Render RTL Arabic text into a grayscale image with simple thresholding.
pub fn render_text_to_bitmap(text: &str, font_data: &[u8], size: f32, width: u32, height: u32) -> GrayImage {
    // reshape
    let reshaper = ArabicReshaper::new();
    let reshaped = reshaper.reshape(text);

    // bidi
    let bidi = BidiInfo::new(&reshaped, None);
    let para = &bidi.paragraphs[0];
    let display = bidi.visual_runs(para, 0..reshaped.len())
        .map(|run| &reshaped[run.range.clone()])
        .collect::<Vec<_>>()
        .join("");

    // load font
    let font = Font::try_from_bytes(font_data).expect("failed to load font");
    let scale = Scale::uniform(size);

    // create image buffer
    let mut image = GrayImage::from_pixel(width, height, Luma([255u8]));

    // simple layout: draw from right edge (RTL)
    let v_metrics = font.v_metrics(scale);
    let mut y = (v_metrics.ascent).ceil() as i32 + 2;
    let mut x = width as i32 - 2; // start near right edge

    for ch in display.chars() {
        let glyph = font.glyph(ch).scaled(scale);
        if let Some(bb) = glyph.pixel_bounding_box() {
            let glyph_x = x + bb.min.x;
            let glyph_y = y + bb.min.y;
            glyph.draw(|gx, gy, v| {
                let px = glyph_x + gx as i32;
                let py = glyph_y + gy as i32;
                if px >= 0 && py >= 0 && (px as u32) < width && (py as u32) < height {
                    // blend (simple overwrite)
                    let alpha = (v * 255.0) as u8;
                    let curr = image.get_pixel(px as u32, py as u32)[0];
                    let new = ((curr as u16 * (255 - alpha) as u16 + (0u16) * alpha as u16) / 255) as u8;
                    image.put_pixel(px as u32, py as u32, Luma([new]));
                }
            });
            // move left by glyph width
            x += bb.min.x - bb.max.x;
        } else {
            // for missing bounding box, advance a small amount
            x -= (scale.x * 0.5) as i32;
        }
    }

    // threshold to 1-bit style (keep as grayscale Luma)
    for px in image.pixels_mut() {
        *px = if px[0] < 200 { Luma([0u8]) } else { Luma([255u8]) };
    }

    image
}
