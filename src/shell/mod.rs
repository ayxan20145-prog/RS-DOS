use crate::{
    arch::cpu,
    drivers::vga::{Color, writer},
    fs::fs,
    print,
    programs::{asm, bat::bat, calc::calc, fetch::fetch, vi::vi},
    std::{
        error::{Error, Result},
        io::read_line,
        str::string_to_hex,
        tui,
    },
};
use core::fmt::Write;

const PROMPT: &str = "C:\\>";
const PROGRAM_ADDR: usize = 0x100000;

pub fn run() {
    print!("\nWelcome to RS-DOS!\nType `help` to see available commands\n");
    print!("\n");

    loop {
        let input = read_line(PROMPT);
        let line = core::str::from_utf8(input.as_bytes()).unwrap_or("");
        execute(line);
        print!("\n");
    }
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
        "asm" => cmd_asm(args.as_bytes()),
        "run" => cmd_run(args.as_bytes()),
        "cpu_vendor" => cmd_cpu_vendor(),
        "tui" => cmd_tui(),
        other => {
            print!("\nunknown command: {}", other);
        }
    }
}
pub fn cmd_help() {
    print!(
        "\nhelp\ncls\necho\nver\nhalt\npanic\ncolor\nfetch\npeek\npoke\ndir\ntouch\ndel\nwrite\ntype\nvi\nmd\nrd\ncalc\nbat\nasm\nrun\ncpu_vendor"
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
    cpu::halt();
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
pub fn cmd_asm(name: &[u8]) {
    let source = match fs().read(name) {
        Ok(source) => source,
        Err(e) => {
            print!("\n{}", e);
            return;
        }
    };

    let source = match core::str::from_utf8(source) {
        Ok(source) => source,
        Err(_) => {
            print!("\n{}", Error::InvalidArgument);
            return;
        }
    };

    let mut lexer = asm::Lexer::new(source);

    let tokens = lexer.tokenize();

    let mut parser = asm::Parser::new(tokens);

    let program = parser.parse_program();

    let bytes = asm::assemble(&program);

    if let Err(e) = fs().create(b"a.out") {
        print!("\n{}", e);
    }
    if let Err(e) = fs().write(b"a.out", bytes.as_slice()) {
        print!("\n{}", e);
    }
}
pub fn cmd_run(name: &[u8]) {
    let code = match fs().read(name) {
        Ok(code) => code,
        Err(e) => {
            print!("\n{}", e);
            return;
        }
    };

    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), PROGRAM_ADDR as *mut u8, code.len());
    }

    let program: extern "C" fn() -> u32 = unsafe { core::mem::transmute(PROGRAM_ADDR) };

    let result = program();

    print!("\n{}", result);
}
pub fn cmd_cpu_vendor() {
    let bytes = cpu::vendor();

    print!("\n");

    for part in bytes {
        for byte in part {
            print!("{}", byte as char);
        }
    }
}
pub fn cmd_tui() {
    tui::draw_rect(0, 0, 5, 5, b'a', Color::White, Color::Black);
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
