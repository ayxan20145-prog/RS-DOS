use crate::{drivers::vga::writer, fs::fs, print};
use core::fmt::Write;

pub fn bat(name: &[u8]) {
    print!("\n");

    let content = core::str::from_utf8(fs().read(name).unwrap()).unwrap();

    for line in content.lines() {
        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("echo") => {
                print!("{}", parts.next().unwrap());
            }
            _ => {
                print!("unknown command");
            }
        }
    }
}
