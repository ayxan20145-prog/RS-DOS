#![no_std]
#![no_main]

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
    let mut writer = vga::Writer::new(0, 0, vga::Color::White, vga::Color::Black);

    writer.clear();

    shell::run(&mut writer);

    cpu::halt();
}
