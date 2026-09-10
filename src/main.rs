#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod cpu;
mod io;
mod keyboard;
mod panic;
mod shell;
mod vga;

use core::arch::global_asm;

global_asm!(include_str!("boot.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    shell::run();

    cpu::halt();
}
