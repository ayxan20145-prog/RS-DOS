use crate::{drivers::vga::writer, fs::fs, print, shell::*};
use core::fmt::Write;

pub fn bat(name: &[u8]) {
    let content = core::str::from_utf8(fs().read(name).unwrap()).unwrap();

    for line in content.lines() {
        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("cls") => cmd_cls(),
            Some("help") => cmd_help(),
            Some("echo") => cmd_echo(parts.next().unwrap()),
            Some("ver") => cmd_ver(),
            Some("panic") => cmd_panic(),
            Some("color") => cmd_color(line),
            Some("fetch") => cmd_fetch(),
            Some("peek") => cmd_peek(parts.next().unwrap()),
            Some("poke") => cmd_poke(line),
            Some("dir") => cmd_dir(),
            Some("touch") => cmd_touch(parts.next().unwrap().as_bytes()),
            Some("del") => cmd_del(parts.next().unwrap().as_bytes()),
            Some("write") => cmd_write(line),
            Some("type") => cmd_type(parts.next().unwrap().as_bytes()),
            Some("vi") => cmd_vi(parts.next().unwrap().as_bytes()),
            Some("md") => cmd_md(parts.next().unwrap().as_bytes()),
            Some("rd") => cmd_rd(parts.next().unwrap().as_bytes()),
            Some("calc") => cmd_calc(line),
            Some("bat") => cmd_bat(parts.next().unwrap().as_bytes()),

            Some(cmd) => {
                print!("unknown command: {}", cmd);
            }

            None => {}
        }
    }
}
