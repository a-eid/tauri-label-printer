use image::GrayImage;

/// Convert a 1-bit gray image (white=255, black=0) into an EPL2 GW command payload.
/// Returns bytes: ASCII header + binary image data.
pub fn image_to_gw(x: u32, y: u32, img: &GrayImage) -> Vec<u8> {
    let width = img.width();
    let height = img.height();

    // bytes per row (8 pixels per byte)
    let bytes_per_row = ((width + 7) / 8) as usize;
    let mut data: Vec<u8> = Vec::new();

    // Header: GWx,y,w,h,bytesperrow, (using ASCII header and CRLF)
    // Many EPL implementations expect binary following the comma, so we send header then binary.
    // EPL expects the GW header in ASCII followed by the raw bitmap bytes.
    // Use CRLF after the header and do not append extra newlines after the binary data.
    let header = format!("GW{},{},{},{},{}\r\n", x, y, width, height, bytes_per_row);
    data.extend_from_slice(header.as_bytes());

    // Pack bits MSB first per byte
    for row in 0..height {
        let mut byte: u8 = 0;
        let mut bit_index = 0;
        for col in 0..width {
            let px = img.get_pixel(col, row)[0];
            let bit = if px == 0 { 1 } else { 0 }; // black=1
            byte <<= 1;
            byte |= bit;
            bit_index += 1;
            if bit_index == 8 {
                data.push(byte);
                byte = 0;
                bit_index = 0;
            }
        }
        if bit_index != 0 {
            // pad remaining bits
            byte <<= 8 - bit_index;
            data.push(byte);
        }
    }

    // Do not add extra newline/terminator bytes after the binary payload.
    data
}
