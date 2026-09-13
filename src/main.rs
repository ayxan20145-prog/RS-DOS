#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod cpu;
mod fs;
mod io;
mod keyboard;
mod panic;
mod shell;
mod vga;

use core::arch::global_asm;

use crate::fs::FileSystem;

global_asm!(include_str!("boot.asm"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    let mut fs = FileSystem::new();

    shell::run(&mut fs);

    cpu::halt();
}
