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

    draw_mode(&editor.mode);

    loop {
        if let Some(key) = keyboard::read_key() {
            match editor.mode {
                Mode::Normal => match key {
                    'q' => {
                        writer().clear();
                        writer().reset_cursor();
                        return;
                    }
                    'i' => {
                        editor.mode = Mode::Insert;
                        draw_mode(&editor.mode);
                    }
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
                    '\x1B' => {
                        editor.mode = Mode::Normal;
                        draw_mode(&editor.mode);
                    }
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
fn draw_mode(mode: &Mode) {
    let old_column = writer().column;
    let old_row = writer().row;

    match mode {
        Mode::Normal => {
            writer().column = 0;
            writer().row = 24;
            print!("-- NORMAL --");
        }
        Mode::Insert => {
            writer().column = 0;
            writer().row = 24;
            print!("-- INSERT --");
            writer().column = old_column;
            writer().row = old_row;
        }
    }

    writer().column = old_column;
    writer().row = old_row;
    writer().update_cursor(writer().row, writer().column);
}
