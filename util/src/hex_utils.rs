pub fn hex_index_to_char(hex: u8) -> char {
    match hex {
        0x00..0x0a => (hex + 0x30) as char,
        0x0a..0x10 => (hex - 0x0a + 0x61) as char,
        _ => panic!("Invalid hex code: {}", hex),
    }
}
