use crate::{
    drivers::{
        keyboard::{self, Key},
        vga::{Color, writer},
    },
    fs::fs,
    print,
    programs::{bat::bat, calc::calc, fetch::fetch, vi::vi},
    std::{
        error::{Error, Result},
        str::string_to_hex,
    },
    tui,
};
use core::fmt::Write;

const PROMPT: &str = "C:\\>";

pub fn run() {
    writer().clear();
    writer().reset_cursor();

    print!("Welcome to RS-DOS!\nType `help` to see available commands\n");
    print!("\n{}", PROMPT);

    let mut cmd_buffer = [0u8; 256];
    let mut cmd_len = 0;
    let mut cursor = 0;

    loop {
        let key = match keyboard::read_key() {
            Some(k) => k,
            None => continue,
        };

        match key {
            Key::Char('\n') => {
                if cmd_len == 0 {
                    print!("\n\n{}", PROMPT);
                } else {
                    let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap_or("");

                    if line == "cls" {
                        cmd_cls();
                        print!("{}", PROMPT);
                    } else {
                        execute(line);
                        print!("\n{}", PROMPT);
                    }
                }
                cmd_len = 0;
                cursor = 0;
            }
            Key::Char('\x08') => {
                let at_end = cursor == cmd_len;
                if cursor > 0 {
                    for i in cursor - 1..cmd_len - 1 {
                        cmd_buffer[i] = cmd_buffer[i + 1];
                    }
                    cmd_len -= 1;
                    cursor -= 1;

                    if at_end {
                        writer().write_byte(b'\x08');
                    } else {
                        repaint(&cmd_buffer[..cmd_len], cursor);
                    }
                }
            }
            Key::Char(c) => {
                if cmd_len < cmd_buffer.len() {
                    for i in (cursor..cmd_len).rev() {
                        cmd_buffer[i + 1] = cmd_buffer[i];
                    }
                    cmd_buffer[cursor] = c as u8;
                    cmd_len += 1;
                    cursor += 1;

                    if cursor == cmd_len {
                        print!("{}", c);
                    } else {
                        repaint(&cmd_buffer[..cmd_len], cursor);
                    }
                }
            }
            Key::Left => {
                if cursor > 0 {
                    cursor -= 1;
                    writer().column = PROMPT.len() + cursor;
                    writer().update_cursor(writer().column, writer().row);
                }
            }
            Key::Right => {
                if cursor < cmd_len {
                    cursor += 1;
                    writer().column = PROMPT.len() + cursor;
                    writer().update_cursor(writer().column, writer().row);
                }
            }

            Key::Up | Key::Down => {}
        }
    }
}
fn repaint(buffer: &[u8], cursor: usize) {
    let row = writer().row;

    writer().column = 0;
    writer().write_string(PROMPT);
    for &b in buffer {
        writer().write_byte(b);
    }

    while writer().column < 79 {
        writer().write_byte(b' ');
    }

    writer().column = PROMPT.len() + cursor;
    writer().row = row;
    writer().update_cursor(writer().column, writer().row);
}
pub fn execute(line: &str) {
    let mut parts = line.split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };

    let args = line[cmd.len()..].trim_start();

    match cmd {
        "help" => cmd_help(),
        "cls" => cmd_cls(),
        "echo" => cmd_echo(args),
        "ver" => cmd_ver(),
        "halt" => cmd_halt(),
        "panic" => cmd_panic(),
        "color" => cmd_color(args),
        "fetch" => cmd_fetch(),
        "peek" => cmd_peek(args),
        "poke" => cmd_poke(args),
        "dir" => cmd_dir(),
        "touch" => cmd_touch(args.as_bytes()),
        "del" => cmd_del(args.as_bytes()),
        "write" => cmd_write(args),
        "type" => cmd_type(args.as_bytes()),
        "vi" => cmd_vi(args.as_bytes()),
        "md" => cmd_md(args.as_bytes()),
        "rd" => cmd_rd(args.as_bytes()),
        "calc" => cmd_calc(args),
        "bat" => cmd_bat(args.as_bytes()),
        "tui" => cmd_tui(),
        other => {
            print!("\nunknown command: {}", other);
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
pub fn cmd_color(args: &str) {
    let mut parts = args.split_whitespace();

    let fg = parts
        .next()
        .ok_or(Error::MissingArgument)
        .and_then(parse_color);
    let bg = parts
        .next()
        .ok_or(Error::MissingArgument)
        .and_then(parse_color);

    match (fg, bg) {
        (Ok(fg), Ok(bg)) => writer().set_color(fg, bg),
        _ => {
            print!("\nusage: color <fg> <bg>");
        }
    }
}
pub fn cmd_fetch() {
    fetch();
}
pub fn cmd_peek(arg: &str) {
    let addr = match string_to_hex(arg) {
        Ok(addr) => addr,
        Err(e) => {
            print!("\n{}", e);
            return;
        }
    };
    let ptr = addr as *const u8;
    let value = unsafe { *ptr };
    print!("\n{}", value);
}
pub fn cmd_poke(args: &str) {
    let mut parts = args.split_whitespace();

    let addr = parts.next();
    let value = parts.next();

    match (addr, value) {
        (Some(addr), Some(value)) => {
            let addr = match string_to_hex(addr) {
                Ok(addr) => addr,
                Err(e) => {
                    print!("\n{}", e);
                    return;
                }
            };

            let value = match value.parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    print!("\n{}", Error::InvalidNumber);
                    return;
                }
            };

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
    if let Err(e) = fs().create(name) {
        print!("\n{}", e);
    }
}
pub fn cmd_del(name: &[u8]) {
    if let Err(e) = fs().remove_file(name) {
        print!("\n{}", e);
    }
}
pub fn cmd_write(args: &str) {
    let mut parts = args.split_whitespace();

    let file = parts.next();
    let data = parts.next();

    match (file, data) {
        (Some(file), Some(data)) => {
            if let Err(e) = fs().write(file.as_bytes(), data.as_bytes()) {
                print!("\n{}", e);
            }
        }
        _ => {
            print!("\n");
        }
    }
}
pub fn cmd_type(name: &[u8]) {
    let content = match fs().read(name) {
        Ok(content) => content,
        Err(e) => {
            print!("\n{}", e);
            return;
        }
    };

    match core::str::from_utf8(content) {
        Ok(text) => {
            print!("\n{}", text);
        }
        Err(_) => {
            print!("\n{}", Error::InvalidArgument);
        }
    }
}
pub fn cmd_vi(name: &[u8]) {
    vi(name);
}
pub fn cmd_md(name: &[u8]) {
    if let Err(e) = fs().create_dir(name) {
        print!("\n{}", e);
    }
}
pub fn cmd_rd(name: &[u8]) {
    if let Err(e) = fs().remove_dir(name) {
        print!("\n{}", e);
    }
}
pub fn cmd_calc(args: &str) {
    let result = calc(args);

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
pub fn cmd_tui() {
    tui::draw_text(0, 0, "hi", Color::Red, Color::Black);
}
fn parse_color(name: &str) -> Result<Color> {
    Ok(match name {
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
        _ => return Err(Error::InvalidColor),
    })
}
