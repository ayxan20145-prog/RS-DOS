use crate::{keyboard, vga::Writer};
use core::fmt::Write;

pub fn run(writer: &mut Writer) {
    write!(
        writer,
        "+------------------+\n|                  |\n|      RS-DOS      |\n|                  |\n+------------------+\n"
    )
    .unwrap();
    write!(writer, "\nC:\\>").unwrap();

    let mut cmd_buffer = [0u8; 256];
    let mut cmd_len = 0;

    loop {
        if let Some(key) = keyboard::read_key() {
            match key {
                '\n' => {
                    if cmd_len == 0 {
                        write!(writer, "\n\nC:\\>").unwrap();
                    } else {
                        if cmd_len == 4 && &cmd_buffer[..4] == b"help" {
                            cmd_help(writer);
                            write!(writer, "\nC:\\>").unwrap();
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"cls" {
                            cmd_cls(writer);
                            write!(writer, "C:\\>").unwrap();
                        } else if cmd_len >= 4 && &cmd_buffer[..4] == b"echo" {
                            cmd_echo(writer, &cmd_buffer, cmd_len);
                            write!(writer, "\nC:\\>").unwrap();
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"ver" {
                            cmd_ver(writer);
                            write!(writer, "\nC:\\>").unwrap();
                        } else if cmd_len == 4 && &cmd_buffer[..4] == b"halt" {
                            cmd_halt(writer);
                            return;
                        } else if cmd_len == 5 && &cmd_buffer[..5] == b"panic" {
                            cmd_panic();
                        } else {
                            let command = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                            write!(writer, "\nunknown command: {}", command).unwrap();
                            write!(writer, "\nC:\\>").unwrap();
                        }
                    }
                    cmd_len = 0;
                }
                '\x08' => {
                    if cmd_len > 0 {
                        cmd_len -= 1;
                        writer.write_byte(b'\x08');
                    }
                }
                _ => {
                    if cmd_len < 256 {
                        cmd_buffer[cmd_len] = key as u8;
                        cmd_len += 1;
                        write!(writer, "{}", key).unwrap();
                    }
                }
            }
        }
    }
}
fn cmd_help(writer: &mut Writer) {
    write!(writer, "\nhelp\ncls\necho\nver\nhalt\npanic").unwrap();
}
fn cmd_cls(writer: &mut Writer) {
    writer.clear(0x0F);
    writer.reset_cursor();
}
fn cmd_echo(writer: &mut Writer, cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        write!(writer, "\n").unwrap();
    } else {
        let start = 5;
        let arg = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap_or("");
        write!(writer, "\n{}", arg).unwrap();
    }
}
fn cmd_ver(writer: &mut Writer) {
    write!(writer, "\n{}", env!("CARGO_PKG_VERSION")).unwrap();
}
fn cmd_halt(writer: &mut Writer) {
    write!(writer, "\nSystem halted").unwrap();
}
fn cmd_panic() {
    panic!();
}
