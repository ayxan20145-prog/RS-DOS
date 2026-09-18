use crate::{keyboard, print, vga::writer};
use core::fmt::Write;

pub fn edit() {
    writer().clear();
    writer().reset_cursor();

    loop {
        if let Some(key) = keyboard::read_key() {
            print!("{}", key);
        }
    }
}
