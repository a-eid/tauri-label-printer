use image::GrayImage;

/// Convert a 1-bit gray image (white=255, black=0) into an EPL2 GW command payload.
/// Returns bytes: ASCII header + binary image data.
pub fn image_to_gw(x: u32, y: u32, img: &GrayImage) -> Vec<u8> {
    let width = img.width();
    let height = img.height();

    // bytes per row (8 pixels per byte)
    let bytes_per_row = ((width + 7) / 8) as usize;
    let mut data: Vec<u8> = Vec::new();

    // Header: GWx,y,bytes_per_row,height (EPL expects bytes-per-row then height)
    let header = format!("GW{},{},{},{}\r\n", x, y, bytes_per_row, height);
    data.extend_from_slice(header.as_bytes());

    // Build rows properly and append raw binary bytes (MSB-first)
    let bpr = bytes_per_row;
    let mut rows: Vec<u8> = vec![0u8; bpr * height as usize];
    for row in 0..height as usize {
        for col in 0..width as usize {
            let px = img.get_pixel(col as u32, row as u32)[0];
            let is_black = px < 128;
            if is_black {
                let idx = row * bpr + (col / 8);
                let bit = 7 - (col % 8);
                rows[idx] |= 1u8 << bit;
            }
        }
    }
    data.extend_from_slice(&rows);
    // End of GW payload must be terminated with CRLF
    data.extend_from_slice(b"\r\n");

    data
}
