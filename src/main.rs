#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod arch;
mod drivers;
mod fs;
mod panic;
mod programs;
mod shell;

use crate::{
    drivers::vga::{Color, Writer, writer},
    fs::FileSystem,
};
use core::{arch::global_asm, fmt::Write};

pub static mut WRITER: Writer = Writer::new(0, 0, Color::White, Color::Black);
pub static mut FS: FileSystem = FileSystem::new();

global_asm!(include_str!("arch/boot.asm"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    writer().clear();

    print!("init vga...   ");
    writer().set_color(Color::Green, Color::Black);
    print!("[ ok ]\n");
    writer().set_color(Color::White, Color::Black);
    delay(10000000);

    print!("init fs...    ");
    writer().set_color(Color::Green, Color::Black);
    print!("[ ok ]\n");
    writer().set_color(Color::White, Color::Black);
    delay(10000000);

    shell::run();

    arch::cpu::halt();
}

fn delay(mut amount: u32) {
    loop {
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}
