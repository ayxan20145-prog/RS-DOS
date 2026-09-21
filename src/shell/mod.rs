use crate::{
    drivers::{
        keyboard,
        vga::{Color, writer},
    },
    fs::fs,
    print,
    programs::vi::vi,
};
use core::fmt::Write;

pub fn run() {
    writer().clear();

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
                    } else {
                        let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
                        let mut parts = line.split_whitespace();

                        match parts.next() {
                            Some("help") => cmd_help(),
                            Some("cls") => cmd_cls(),
                            Some("echo") => cmd_echo(&cmd_buffer, cmd_len),
                            Some("ver") => cmd_ver(),
                            Some("halt") => {
                                cmd_halt();
                                return;
                            }
                            Some("panic") => cmd_panic(),
                            Some("color") => cmd_color(&cmd_buffer, cmd_len),
                            Some("fetch") => cmd_fetch(),
                            Some("peek") => cmd_peek(&cmd_buffer, cmd_len),
                            Some("poke") => cmd_poke(&cmd_buffer, cmd_len),
                            Some("dir") => cmd_dir(),
                            Some("touch") => cmd_touch(&cmd_buffer, cmd_len),
                            Some("del") => cmd_del(&cmd_buffer, cmd_len),
                            Some("write") => cmd_write(&cmd_buffer, cmd_len),
                            Some("type") => cmd_type(&cmd_buffer, cmd_len),
                            Some("vi") => cmd_vi(&cmd_buffer, cmd_len),
                            Some("md") => cmd_md(&cmd_buffer, cmd_len),
                            Some("rd") => cmd_rd(&cmd_buffer, cmd_len),

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
fn cmd_help() {
    print!(
        "\nhelp\ncls\necho\nver\nhalt\npanic\ncolor\nfetch\npeek\npoke\ndir\ntouch\ndel\nwrite\ntype\nvi\nmd\nrd"
    );
}
fn cmd_cls() {
    writer().clear();
    writer().reset_cursor();
}
fn cmd_echo(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        print!("\nusage: echo <text>");
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
   / __ \/ ___/      / __ \/ __ \/ ___/ -----------
  / /_/ /\__ \______/ / / / / / /\__ \  OS: RS-DOS
 / _, _/___/ /_____/ /_/ / /_/ /___/ /  Arch: i686
/_/ |_|/____/     /_____/\____//___/    Resoloution: 80x25
                                        
"#
    );
}
fn cmd_peek(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        print!("\nusage: peek <addr>");
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
        print!("\nusage: poke <addr> <val>");
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
fn cmd_dir() {
    fs().list();
}
fn cmd_touch(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 5 {
        print!("\nusage: touch <name>");
    } else {
        let start = 6;
        fs().create(&cmd_buffer[start..cmd_len]);
    }
}
fn cmd_del(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 3 {
        print!("\nusage: del <name>");
    } else {
        let start = 4;
        fs().remove_file(&cmd_buffer[start..cmd_len]);
    }
}
fn cmd_write(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 5 {
        print!("\nusage: write <file> <data>");
    } else {
        let line = core::str::from_utf8(&cmd_buffer[..cmd_len]).unwrap();
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
}
fn cmd_type(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 4 {
        print!("\nusage: type <file>");
        return;
    } else {
        let start = 5;
        let content =
            core::str::from_utf8(fs().read(&cmd_buffer[start..cmd_len]).unwrap()).unwrap();
        print!("\n{}", content);
    }
}
fn cmd_vi(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 2 {
        print!("\nusage: vi <file>");
        return;
    } else {
        let start = 3;
        let name = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap();
        vi(name.as_bytes());
    }
}
fn cmd_md(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 2 {
        print!("\nusage: md <name>");
        return;
    } else {
        let start = 3;
        let name = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap();
        fs().create_dir(name.as_bytes());
    }
}
fn cmd_rd(cmd_buffer: &[u8], cmd_len: usize) {
    if cmd_len == 2 {
        print!("\nusage: rd <name>");
        return;
    } else {
        let start = 3;
        let name = core::str::from_utf8(&cmd_buffer[start..cmd_len]).unwrap();
        fs().remove_dir(name.as_bytes());
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
