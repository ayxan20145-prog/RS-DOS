use crate::{
    drivers::{
        keyboard,
        vga::{Color, writer},
    },
    fs::fs,
    print,
    programs::{bat::bat, calc::calc, fetch::fetch, vi::vi},
};
use core::fmt::Write;

pub fn run() {
    writer().clear();
    writer().reset_cursor();

    print!("Welcome to RS-DOS!\nType `help` to see available commands\n");
    print!("\nC:\\>");

    let mut cmd_buffer = [0u8; 256];
    let mut cmd_len = 0;

    loop {
        if let Some(key) = keyboard::read_key() {
            match key {
                '\n' => {
                    if cmd_len == 0 {
                        write!(writer(), "\n\nC:\\>").unwrap();
                    } else if cmd_len == 3 && &cmd_buffer[..3] == b"cls" {
                        cmd_cls();
                        print!("C:\\>");
                    } else {
                        let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                        let mut parts = line.split_whitespace();

                        match parts.next() {
                            Some("help") => cmd_help(),
                            Some("echo") => {
                                if cmd_len == 4 {
                                    print!("\nusage: echo <text>");
                                } else {
                                    let start = 5;
                                    let text =
                                        core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap();
                                    cmd_echo(text);
                                }
                            }
                            Some("ver") => cmd_ver(),
                            Some("halt") => {
                                cmd_halt();
                                return;
                            }
                            Some("panic") => cmd_panic(),
                            Some("color") => {
                                let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                                cmd_color(line);
                            }
                            Some("fetch") => cmd_fetch(),
                            Some("peek") => {
                                if cmd_len == 4 {
                                    print!("\nusage: peek <addr>");
                                } else {
                                    let start = 5;
                                    let arg =
                                        core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap();
                                    cmd_peek(arg);
                                }
                            }
                            Some("poke") => {
                                if cmd_len == 4 {
                                    print!("\nusage: poke <addr> <val>");
                                } else {
                                    let line =
                                        core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                                    cmd_poke(line);
                                }
                            }
                            Some("dir") => cmd_dir(),
                            Some("touch") => {
                                if cmd_len == 5 {
                                    print!("\nusage: touch <name>");
                                } else {
                                    let start = 6;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_touch(name);
                                }
                            }
                            Some("del") => {
                                if cmd_len == 3 {
                                    print!("\nusage: del <name>");
                                } else {
                                    let start = 4;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_del(name);
                                }
                            }
                            Some("write") => {
                                if cmd_len == 5 {
                                    print!("\nusage: write <name> <data>");
                                } else {
                                    let line =
                                        core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                                    cmd_write(line);
                                }
                            }
                            Some("type") => {
                                if cmd_len == 4 {
                                    print!("\nusage: type <name>");
                                } else {
                                    let start = 5;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_type(name);
                                }
                            }
                            Some("vi") => {
                                if cmd_len == 2 {
                                    print!("\nusage: vi <name>");
                                } else {
                                    let start = 3;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_vi(name);
                                }
                            }
                            Some("md") => {
                                if cmd_len == 2 {
                                    print!("\nusage: md <name>");
                                } else {
                                    let start = 3;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_md(name);
                                }
                            }
                            Some("rd") => {
                                if cmd_len == 2 {
                                    print!("\nusage: rd <name>");
                                } else {
                                    let start = 3;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_rd(name);
                                }
                            }
                            Some("calc") => {
                                if cmd_len == 4 {
                                    print!("\nusage: calc <num1> <op> <num2>");
                                } else {
                                    let line =
                                        core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                                    cmd_calc(line);
                                }
                            }
                            Some("bat") => {
                                if cmd_len == 3 {
                                    print!("\nusage: bat <name>");
                                } else {
                                    let start = 4;
                                    let name = &cmd_buffer[start..cmd_len];
                                    cmd_bat(name);
                                }
                            }

                            Some(cmd) => {
                                print!("\nunknown command: {}", cmd);
                            }

                            None => {}
                        }
                        print!("\nC:\\>");
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
pub fn cmd_help() {
    print!(
        "\nhelp\ncls\necho\nver\nhalt\npanic\ncolor\nfetch\npeek\npoke\ndir\ntouch\ndel\nwrite\ntype\nvi\nmd\nrd\ncalc\nbat"
    );
}
pub fn cmd_cls() {
    writer().clear();
    writer().reset_cursor();
}
pub fn cmd_echo(text: &str) {
    print!("\n{}", text);
}
pub fn cmd_ver() {
    print!("\n{}", env!("CARGO_PKG_VERSION"));
}
pub fn cmd_halt() {
    print!("\nSystem halted");
}
pub fn cmd_panic() {
    panic!();
}
pub fn cmd_color(line: &str) {
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
pub fn cmd_fetch() {
    fetch();
}
pub fn cmd_peek(arg: &str) {
    let addr = string_to_hex(arg);
    let ptr = addr as *const u8;
    let value = unsafe { *ptr };
    print!("\n{}", value);
}
pub fn cmd_poke(line: &str) {
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
pub fn cmd_dir() {
    fs().list();
}
pub fn cmd_touch(name: &[u8]) {
    fs().create(name);
}
pub fn cmd_del(name: &[u8]) {
    fs().remove_file(name);
}
pub fn cmd_write(line: &str) {
    let mut parts = line.split_whitespace();

    parts.next();

    let file = parts.next();
    let data = parts.next();

    match (file, data) {
        (Some(file), Some(data)) => {
            fs().write(file.as_bytes(), data.as_bytes());
        }
        _ => {
            print!("\n");
        }
    }
}
pub fn cmd_type(name: &[u8]) {
    let content = core::str::from_utf8(fs().read(name).unwrap()).unwrap();
    print!("\n{}", content);
}
pub fn cmd_vi(name: &[u8]) {
    vi(name);
}
pub fn cmd_md(name: &[u8]) {
    fs().create_dir(name);
}
pub fn cmd_rd(name: &[u8]) {
    fs().remove_dir(name);
}
pub fn cmd_calc(line: &str) {
    let result = calc(line);

    match result {
        Ok(num) => {
            print!("\n{}", num);
        }
        Err(e) => {
            print!("\n{}", e);
        }
    }
}
pub fn cmd_bat(name: &[u8]) {
    bat(name);
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
