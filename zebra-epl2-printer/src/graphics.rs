use image::{ImageBuffer, Luma};
use rusttype::{Font, Scale, point};
use ar_reshaper::ArabicReshaper;
use unicode_bidi::BidiInfo;

/// Render a single Arabic line (tight 1-bit), right-aligned into a minimal-height bitmap.
pub fn render_arabic_line_1bit(
    text: &str,
    font_bytes: &[u8],
    width_px: u32,
    pad_right: u32,
    px_size: f32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let font = Font::try_from_bytes(font_bytes).expect("invalid font");

    // 1) Shape & reorder for Arabic (logical -> visual)
    let reshaper = ArabicReshaper::new();
    let reshaped = reshaper.reshape(text);
    let bidi = BidiInfo::new(&reshaped, None);
    let para = &bidi.paragraphs[0];
    // collect visual runs as a string (RTL)
    let (_levels, runs) = bidi.visual_runs(para, 0..reshaped.len());
    let mut visual = String::new();
    for range in runs {
        visual.push_str(&reshaped[range.clone()]);
    }

    // 2) Layout to measure width & height
    let scale = Scale { x: px_size, y: px_size };
    let v_metrics = font.v_metrics(scale);
    let ascent = v_metrics.ascent;
    let descent = v_metrics.descent; // negative value
    let line_h = ((ascent - descent).ceil()).max(24.0) as u32;

    // Measure text width
    let glyphs: Vec<_> = font.layout(&visual, scale, point(0.0, ascent)).collect();
    let text_w = glyphs
        .iter()
        .rev()
        .find_map(|g| g.pixel_bounding_box().map(|bb| bb.max.x as f32))
        .unwrap_or(0.0)
        .ceil() as u32;

    // 3) Create tight white canvas
    let mut img = ImageBuffer::from_pixel(width_px, line_h, Luma([255u8]));

    // Right-align so text ends at (width - pad_right)
    let start_x = (width_px as i32 - pad_right as i32 - text_w as i32).max(0) as f32;
    let baseline_y = ascent;

    // 4) Draw glyphs as solid black with hard threshold
    for g in font.layout(&visual, scale, point(start_x, baseline_y)) {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                if v > 0.6 {
                    let px = x as i32 + bb.min.x;
                    let py = y as i32 + bb.min.y;
                    if px >= 0 && py >= 0 && (px as u32) < width_px && (py as u32) < line_h {
                        img.put_pixel(px as u32, py as u32, Luma([0u8]));
                    }
                }
            });
        }
    }

    img
}
