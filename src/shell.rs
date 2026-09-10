use crate::{
    keyboard,
    vga::{Color, writer},
};
use core::fmt::Write;

pub fn run() {
    writer().clear();

    write!(
        writer(),
        "+------------------+\n|                  |\n|      RS-DOS      |\n|                  |\n+------------------+\n"
    )
    .unwrap();
    write!(writer(), "\nC:\\>").unwrap();

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
                            write!(writer(), "\nC:\\>").unwrap();
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"cls" {
                            cmd_cls();
                            write!(writer(), "C:\\>").unwrap();
                        } else if cmd_len >= 4 && &cmd_buffer[..4] == b"echo" {
                            cmd_echo(&cmd_buffer, cmd_len);
                            write!(writer(), "\nC:\\>").unwrap();
                        } else if cmd_len == 3 && &cmd_buffer[..3] == b"ver" {
                            cmd_ver();
                            write!(writer(), "\nC:\\>").unwrap();
                        } else if cmd_len == 4 && &cmd_buffer[..4] == b"halt" {
                            cmd_halt();
                            return;
                        } else if cmd_len == 5 && &cmd_buffer[..5] == b"panic" {
                            cmd_panic();
                        } else if cmd_len >= 5 && &cmd_buffer[..5] == b"color" {
                            cmd_color(&cmd_buffer, cmd_len);
                            write!(writer(), "\nC:\\>").unwrap();
                        } else if cmd_len == 5 && &cmd_buffer[..5] == b"fetch" {
                            cmd_fetch();
                            write!(writer(), "\nC:\\>").unwrap();
                        } else {
                            let command = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                            write!(writer(), "\nunknown command: {}", command).unwrap();
                            write!(writer(), "\nC:\\>").unwrap();
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
                        write!(writer(), "{}", key).unwrap();
                    }
                }
            }
        }
    }
}
fn cmd_help() {
    write!(
        writer(),
        "\nhelp\ncls\necho\nver\nhalt\npanic\ncolor\nfetch"
    )
    .unwrap();
}
fn cmd_cls() {
    writer().clear();
    writer().reset_cursor();
}
fn cmd_echo(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        write!(writer(), "\n").unwrap();
    } else {
        let start = 5;
        let arg = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap_or("");
        write!(writer(), "\n{}", arg).unwrap();
    }
}
fn cmd_ver() {
    write!(writer(), "\n{}", env!("CARGO_PKG_VERSION")).unwrap();
}
fn cmd_halt() {
    write!(writer(), "\nSystem halted").unwrap();
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
        _ => write!(writer(), "\nusage: color <fg> <bg>").unwrap(),
    }
}
fn cmd_fetch() {
    write!(
        writer(),
        r#"
    ____  _____       ____  ____  _____ user@RS-DOS
   / __ \/ ___/      / __ \/ __ \/ ___/ ---
  / /_/ /\__ \______/ / / / / / /\__ \  OS: RS-DOS
 / _, _/___/ /_____/ /_/ / /_/ /___/ /  Arch: i686
/_/ |_|/____/     /_____/\____//___/    Resoloution: 80x25
                                        
"#
    )
    .unwrap();
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
