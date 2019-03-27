

pub fn u16_from_bytes(b: [u8; 2]) -> u16 {
    ((b[1] as u16) << 8) +
    ((b[0] as u16) << 0)
}

pub fn u16_from_u8_slice(b: &[u8]) -> u16 {
    u16_from_bytes([b[0], b[1]])
}

pub fn u32_from_bytes(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    ((b3 as u32) << 24) +
    ((b2 as u32) << 16) +
    ((b1 as u32) <<  8) +
    ((b0 as u32) <<  0)
}

pub fn u32_from_chars(c: [char; 4]) -> u32 {
    u32_from_bytes(c[0] as u8, c[1] as u8, c[2] as u8, c[3] as u8)
}

pub fn u32_from_u8_slice(b: &[u8]) -> u32 {
    u32_from_bytes(b[0], b[1], b[2], b[3])
}