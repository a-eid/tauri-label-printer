/// EPL header and helpers
pub fn header() -> Vec<u8> {
    // N (clear), q width, Q height,24, D10, P1
    let mut v = Vec::new();
    v.extend_from_slice(b"N\n");
    v.extend_from_slice(b"q440\n");
    v.extend_from_slice(b"Q320,24\n");
    v.extend_from_slice(b"D10\n");
    // single copy
    v.extend_from_slice(b"P1\n");
    v
}

/// Append raw bytes to a Vec<u8>
pub fn append(dest: &mut Vec<u8>, data: &[u8]) {
    dest.extend_from_slice(data);
}
