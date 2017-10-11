
pub fn make_u16(lo: u8, hi: u8) -> u16 {
    (hi as u16) << 8 | lo as u16
}