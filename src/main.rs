#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(static_mut_refs)]
#![cfg_attr(test, allow(dead_code))]

mod arch;
mod drivers;
mod fs;
mod panic;
mod programs;
mod shell;
mod std;

use crate::{
    drivers::vga::{Color, Writer, writer},
    fs::FileSystem,
};
#[cfg(not(test))]
use core::arch::global_asm;

pub static mut WRITER: Writer = Writer::new(0, 0, Color::White, Color::Black);
pub static mut FS: FileSystem = FileSystem::new();

#[cfg(not(test))]
global_asm!(include_str!("arch/boot.asm"));

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    writer().clear();

    shell::run();

    arch::cpu::halt();
}
