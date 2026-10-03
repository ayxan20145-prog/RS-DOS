use crate::std::error::{Error, Result};

pub fn string_to_hex(s: &str) -> Result<u32> {
    let mut val: u32 = 0;

    for c in s.trim_start_matches("0x").chars() {
        let byte = c.to_digit(16).ok_or(Error::InvalidAddress)?;
        val = (val << 4) | byte;
    }

    Ok(val)
}
