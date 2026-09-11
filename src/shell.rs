use crate::{
    keyboard, print,
    vga::{Color, writer},
};
use core::fmt::Write;

pub fn run() {
    writer().clear();

    print!(
        "+------------------+\n|                  |\n|      RS-DOS      |\n|                  |\n+------------------+\n"
    );
    print!("\nC:\\>");

    let mut cmd_buffer = [0u8; 256];
    let mut cmd_len = 0;

    loop {
        if let Some(key) = keyboard::read_key() {
            match key {
                '\n' => {
                    if cmd_len == 0 {
                        write!(writer(), "\n\nC:\\>").unwrap();
                    } else {
                        if cmd_len == 4 && &cmd_buffer[..4] == b"help" {
                            cmd_help();
                            print!("\nC:\\>");
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"cls" {
                            cmd_cls();
                            print!("C:\\>");
                        } else if cmd_len >= 4 && &cmd_buffer[..4] == b"echo" {
                            cmd_echo(&cmd_buffer, cmd_len);
                            print!("\nC:\\>");
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"ver" {
                            cmd_ver();
                            print!("\nC:\\>");
                        } else if cmd_len == 4 && &cmd_buffer[..4] == b"halt" {
                            cmd_halt();
                            return;
                        } else if cmd_len == 5 && &cmd_buffer[..5] == b"panic" {
                            cmd_panic();
                        } else if cmd_len >= 5 && &cmd_buffer[..5] == b"color" {
                            cmd_color(&cmd_buffer, cmd_len);
                            print!("\nC:\\>");
                        } else if cmd_len == 5 && &cmd_buffer[..5] == b"fetch" {
                            cmd_fetch();
                            print!("\nC:\\>");
                        } else if cmd_len >= 4 && &cmd_buffer[..4] == b"peek" {
                            cmd_peek(&cmd_buffer, cmd_len);
                            print!("\nC:\\>");
                        } else if cmd_len >= 4 && &cmd_buffer[..4] == b"poke" {
                            cmd_poke(&cmd_buffer, cmd_len);
                            print!("\nC:\\>");
                        } else {
                            let command = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                            print!("\nunknown command: {}", command);
                            print!("\nC:\\>");
                        }
                    }
                    cmd_len = 0;
                }
                '\x08' => {
                    if cmd_len > 0 {
                        cmd_len -= 1;
                        writer().write_byte(b'\x08');
                    }
                }
                _ => {
                    if cmd_len < 256 {
                        cmd_buffer[cmd_len] = key as u8;
                        cmd_len += 1;
                        print!("{}", key);
                    }
                }
            }
        }
    }
}
fn cmd_help() {
    print!("\nhelp\ncls\necho\nver\nhalt\npanic\ncolor\nfetch\npeek\npoke");
}
fn cmd_cls() {
    writer().clear();
    writer().reset_cursor();
}
fn cmd_echo(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        print!("\n");
    } else {
        let start = 5;
        let arg = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap_or("");
        print!("\n{}", arg);
    }
}
fn cmd_ver() {
    print!("\n{}", env!("CARGO_PKG_VERSION"));
}
fn cmd_halt() {
    print!("\nSystem halted");
}
fn cmd_panic() {
    panic!();
}
fn cmd_color(cmd_buffer: &[u8], cmd_len: usize) {
    let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap_or("");
    let mut parts = line.split_whitespace();
    parts.next();

    let fg = parts.next().and_then(parse_color);
    let bg = parts.next().and_then(parse_color);

    match (fg, bg) {
        (Some(fg), Some(bg)) => writer().set_color(fg, bg),
        _ => {
            print!("\nusage: color <fg> <bg>");
        }
    }
}
fn cmd_fetch() {
    print!(
        r#"
    ____  _____       ____  ____  _____ user@RS-DOS
   / __ \/ ___/      / __ \/ __ \/ ___/ ---
  / /_/ /\__ \______/ / / / / / /\__ \  OS: RS-DOS
 / _, _/___/ /_____/ /_/ / /_/ /___/ /  Arch: i686
/_/ |_|/____/     /_____/\____//___/    Resoloution: 80x25
                                        
"#
    );
}
fn cmd_peek(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        print!("\n");
    } else {
        let start = 5;
        let arg = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap();
        let addr = string_to_hex(arg);
        let ptr = addr as *const u8;
        let value = unsafe { *ptr };
        print!("\n{}", value);
    }
}
fn cmd_poke(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        print!("\n");
    } else {
        let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
        let mut parts = line.split_whitespace();

        parts.next();

        let addr = parts.next();
        let value = parts.next();

        match (addr, value) {
            (Some(addr), Some(value)) => {
                let addr = string_to_hex(addr);
                let value = value.parse::<u8>().unwrap();

                let ptr = addr as *mut u8;

                unsafe {
                    *ptr = value;
                }
            }
            _ => {
                print!("\n");
            }
        }
    }
}
fn parse_color(name: &str) -> Option<Color> {
    Some(match name {
        "black" => Color::Black,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "cyan" => Color::Cyan,
        "red" => Color::Red,
        "magenta" => Color::Magenta,
        "brown" => Color::Brown,
        "lightgray" => Color::LightGray,
        "darkgray" => Color::DarkGray,
        "lightblue" => Color::LightBlue,
        "lightgreen" => Color::LightGreen,
        "lightcyan" => Color::LightCyan,
        "lightred" => Color::LightRed,
        "pink" => Color::Pink,
        "yellow" => Color::Yellow,
        "white" => Color::White,
        _ => return None,
    })
}
fn string_to_hex(s: &str) -> u32 {
    let mut val: u32 = 0;

    for c in s.trim_start_matches("0x").chars() {
        let byte = c.to_digit(16).unwrap();
        val = (val << 4) | byte;
    }

    val
}
