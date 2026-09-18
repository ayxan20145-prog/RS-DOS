use crate::{keyboard, print, vga::writer};
use core::fmt::Write;

struct Editor {
    buf: [u8; 256],
    buf_len: usize,
}
pub fn edit() {
    writer().clear();
    writer().reset_cursor();

    let mut editor = Editor {
        buf: [0u8; 256],
        buf_len: 0,
    };

    loop {
        if let Some(key) = keyboard::read_key() {
            match key {
                '\x08' => {
                    if editor.buf_len > 0 {
                        editor.buf_len -= 1;
                        writer().write_byte(b'\x08');
                    }
                }
                _ => {
                    if editor.buf_len < 256 {
                        editor.buf[editor.buf_len] = key as u8;
                        editor.buf_len += 1;
                        print!("{}", key);
                    }
                }
            }
        }
    }
}
