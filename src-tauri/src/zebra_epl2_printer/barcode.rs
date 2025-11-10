/// Compute EAN-13 checksum digit for 12-digit input (returns 13-digit string)
pub fn ean13_with_checksum(mut code: String) -> String {
    let digits: Vec<u32> = code.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
    if digits.len() >= 13 { return code; }
    let mut sum = 0u32;
    for (i, &d) in digits.iter().enumerate() {
        if i % 2 == 0 { sum += d; } else { sum += d * 3; }
    }
    let check = (10 - (sum % 10)) % 10;
    code.push_str(&check.to_string());
    code
}

/// Create an EPL2 barcode command for EAN-13 at position x,y with given height.
pub fn epl_ean13(x: u32, y: u32, code: &str, height: u32) -> Vec<u8> {
    // Ensure checksum
    let mut full = code.to_string();
    if full.len() == 12 { full = ean13_with_checksum(full); }

    // B x,y,rotation,narrow,wide,height,readable,? ,data\n
    let cmd = format!("B {},{},0,2,3,{},1,0,{}\n", x, y, height, full);
    cmd.into_bytes()
}
