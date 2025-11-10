// Label and printer tuning constants
pub const LABEL_W: u32 = 440;
pub const LABEL_H: u32 = 320;
pub const PAD_R: u32 = 10;
pub const FONT_PX: f32 = 40.0; // bigger & readable (38–42)
pub const DARKNESS: u8 = 6;    // D6 reduces banding
pub const SPEED: u8 = 3;       // S3 smoother
pub const NARROW: u32 = 2;     // barcode narrow module (2–3)
pub const HEIGHT: u32 = 50;    // barcode height (fit HRI)
pub const FORCE_LANDSCAPE: bool = false; // set true only if driver locks to Landscape

// helper: safe top/bottom margins can be tuned here
pub const TOP_MARGIN: u32 = 8;
pub const LEFT_MARGIN: u32 = 10;
