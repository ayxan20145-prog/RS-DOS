use crate::{
    drivers::{
        keyboard::{self, Key},
        vga::writer,
    },
    std::{string::String, vec::Vec},
};

pub fn read_line(prompt: &str) -> String {
    let mut buffer = Vec::<u8>::new();
    let mut cursor = 0usize;

    writer().write_string(prompt);

    loop {
        let key = match keyboard::read_key() {
            Some(k) => k,
            None => continue,
        };

        match key {
            Key::Char('\n') => {
                let s = core::str::from_utf8(buffer.as_slice()).unwrap_or("").into();
                return s;
            }
            Key::Char('\x08') => {
                if cursor > 0 {
                    let at_end = cursor == buffer.len();
                    buffer.remove(cursor - 1);
                    cursor -= 1;

                    if at_end {
                        writer().write_byte(b'\x08');
                    } else {
                        repaint(prompt, buffer.as_slice(), cursor);
                    }
                }
            }
            Key::Char(c) => {
                buffer.insert(cursor, c as u8);
                cursor += 1;

                if cursor == buffer.len() {
                    writer().write_byte(c as u8);
                } else {
                    repaint(prompt, buffer.as_slice(), cursor);
                }
            }
            Key::Left => {
                if cursor > 0 {
                    cursor -= 1;
                    writer().column = prompt.len() + cursor;
                    writer().update_cursor(writer().column, writer().row);
                }
            }
            Key::Right => {
                if cursor < buffer.len() {
                    cursor += 1;
                    writer().column = prompt.len() + cursor;
                    writer().update_cursor(writer().column, writer().row);
                }
            }
            Key::Up | Key::Down => {}
        }
    }
}

fn repaint(prompt: &str, buffer: &[u8], cursor: usize) {
    let row = writer().row;

    writer().column = 0;
    writer().write_string(prompt);
    for &b in buffer {
        writer().write_byte(b);
    }

    while writer().column < 79 {
        writer().write_byte(b' ');
    }

    writer().column = prompt.len() + cursor;
    writer().row = row;
    writer().update_cursor(writer().column, writer().row);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write!(writer(), $($arg)*).unwrap();
    };
}
