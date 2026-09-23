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
    drivers::vga::{Color, Writer},
    fs::FileSystem,
};
use core::arch::global_asm;

pub static mut WRITER: Writer = Writer::new(0, 0, Color::White, Color::Black);
pub static mut FS: FileSystem = FileSystem::new();

global_asm!(include_str!("arch/boot.asm"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    shell::run();

    arch::cpu::halt();
}
