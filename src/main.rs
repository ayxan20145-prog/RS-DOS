#![no_std]
#![no_main]

mod cpu;
mod io;
mod keyboard;
mod panic;
mod vga;

use core::{arch::global_asm, fmt::Write};

global_asm!(include_str!("boot.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = vga::Writer::new(0, 0, 0x0F);
    writer.clear(0x0F);
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
                            write!(writer, "\nhelp\ncls\necho\nver\nhalt").unwrap();
                            write!(writer, "\nC:\\>").unwrap();
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"cls" {
                            writer.clear(0x0F);
                            writer.reset_cursor();
                            write!(writer, "C:\\>").unwrap();
                        } else if cmd_len >= 4 && &cmd_buffer[..4] == b"echo" {
                            if cmd_len == 4 {
                                write!(writer, "\n").unwrap();
                            } else {
                                let start = 5;
                                let arg =
                                    core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap_or("");
                                write!(writer, "\n{}", arg).unwrap();
                            }
                            write!(writer, "\nC:\\>").unwrap();
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"ver" {
                            write!(writer, "\n{}", env!("CARGO_PKG_VERSION")).unwrap();
                            write!(writer, "\nC:\\>").unwrap();
                        } else if cmd_len == 4 && &cmd_buffer[..4] == b"halt" {
                            write!(writer, "\nSystem halted").unwrap();
                            cpu::halt();
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
