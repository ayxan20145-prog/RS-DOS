use crate::{keyboard, print, vga::writer};
use core::fmt::Write;

enum Mode {
    Normal,
    Insert,
}

struct Editor {
    mode: Mode,
    buf: [u8; 256],
    buf_len: usize,
}
pub fn edit() {
    writer().clear();
    writer().reset_cursor();

    let mut editor = Editor {
        mode: Mode::Normal,
        buf: [0u8; 256],
        buf_len: 0,
    };

    loop {
        if let Some(key) = keyboard::read_key() {
            match editor.mode {
                Mode::Normal => match key {
                    'q' => {
                        writer().clear();
                        writer().reset_cursor();
                        return;
                    }
                    'i' => editor.mode = Mode::Insert,
                    _ => {}
                },
                Mode::Insert => match key {
                    '\n' => {
                        if editor.buf_len < editor.buf.len() {
                            editor.buf[editor.buf_len] = b'\n';
                            editor.buf_len += 1;
                            writer().write_byte(b'\n');
                        }
                    }
                    '\x08' => {
                        if editor.buf_len > 0 {
                            editor.buf_len -= 1;
                            writer().write_byte(b'\x08');
                        }
                    }
                    '\x1B' => editor.mode = Mode::Normal,
                    _ => {
                        if editor.buf_len < 256 {
                            editor.buf[editor.buf_len] = key as u8;
                            editor.buf_len += 1;
                            print!("{}", key);
                        }
                    }
                },
            }
        }
    }
}
