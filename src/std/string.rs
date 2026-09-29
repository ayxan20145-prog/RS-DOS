pub fn string_to_hex(s: &str) -> u32 {
    let mut val: u32 = 0;

    for c in s.trim_start_matches("0x").chars() {
        let byte = c.to_digit(16).unwrap();
        val = (val << 4) | byte;
    }

    val
}
