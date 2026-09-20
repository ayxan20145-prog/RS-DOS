use crate::{
    drivers::{keyboard, vga::writer},
    fs::FileSystem,
    print,
};
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

impl Mode {
    fn display(&self) {
        let old_column = writer().column;
        let old_row = writer().row;

        match self {
            Mode::Normal => {
                writer().column = 0;
                writer().row = 24;
                print!("-- NORMAL --");
            }
            Mode::Insert => {
                writer().column = 0;
                writer().row = 24;
                print!("-- INSERT --");
            }
        }

        writer().column = old_column;
        writer().row = old_row;
        writer().update_cursor(writer().column, writer().row);
    }
}
pub fn vi(fs: &mut FileSystem, name: &[u8]) {
    writer().clear();
    writer().reset_cursor();

    let mut editor = Editor {
        mode: Mode::Normal,
        buf: [0u8; 256],
        buf_len: 0,
    };

    editor.mode.display();

    let content = fs.read(name).unwrap();

    editor.buf[..content.len()].copy_from_slice(content);
    editor.buf_len = content.len();

    print!("{}", core::str::from_utf8(content).unwrap());

    loop {
        if let Some(key) = keyboard::read_key() {
            match editor.mode {
                Mode::Normal => match key {
                    'q' => {
                        writer().clear();
                        writer().reset_cursor();
                        return;
                    }
                    'w' => {
                        let data = core::str::from_utf8(&editor.buf[..editor.buf_len]).unwrap();
                        fs.write(name, data.as_bytes());
                    }
                    'i' => {
                        editor.mode = Mode::Insert;
                        editor.mode.display();
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
                        editor.mode.display();
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
