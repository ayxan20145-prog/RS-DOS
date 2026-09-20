#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod arch;
mod drivers;
mod fs;
mod panic;
mod programs;
mod shell;

use core::arch::global_asm;

global_asm!(include_str!("arch/boot.asm"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    shell::run();

    arch::cpu::halt();
}
