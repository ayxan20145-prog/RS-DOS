use crate::{
    drivers::{
        keyboard::{self, Key},
        vga::writer,
    },
    std::string::String,
};

pub fn read_line(prompt: &str) -> String {
    let mut buffer = [0u8; 256];
    let mut len = 0usize;
    let mut cursor = 0usize;

    writer().write_string(prompt);

    loop {
        let key = match keyboard::read_key() {
            Some(k) => k,
            None => continue,
        };

        match key {
            Key::Char('\n') => {
                let s = core::str::from_utf8(&buffer[..len]).unwrap_or("").into();
                return s;
            }
            Key::Char('\x08') => {
                let at_end = cursor == len;
                if cursor > 0 {
                    for i in cursor - 1..len - 1 {
                        buffer[i] = buffer[i + 1];
                    }
                    len -= 1;
                    cursor -= 1;

                    if at_end {
                        writer().write_byte(b'\x08');
                    } else {
                        repaint(prompt, &buffer[..len], cursor);
                    }
                }
            }
            Key::Char(c) => {
                if len < buffer.len() {
                    for i in (cursor..len).rev() {
                        buffer[i + 1] = buffer[i];
                    }
                    buffer[cursor] = c as u8;
                    len += 1;
                    cursor += 1;

                    if cursor == len {
                        writer().write_byte(c as u8);
                    } else {
                        repaint(prompt, &buffer[..len], cursor);
                    }
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
                if cursor < len {
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
