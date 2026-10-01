use crate::{drivers::vga::writer, fs::fs, print, shell::*, std::error::Error};
use core::fmt::Write;

pub fn bat(name: &[u8]) {
    let content = match fs().read(name) {
        Ok(smth) => smth,
        Err(e) => {
            print!("\n{}", e);
            return;
        }
    };
    let content = match core::str::from_utf8(content) {
        Ok(s) => s,
        Err(_) => {
            print!("\n{}", Error::InvalidArgument);
            return;
        }
    };

    for line in content.lines() {
        execute(line);
    }
}
