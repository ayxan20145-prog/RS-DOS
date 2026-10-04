#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(static_mut_refs)]
#![cfg_attr(test, allow(dead_code))]

mod allocator;
mod arch;
mod drivers;
mod fs;
mod panic;
mod programs;
mod shell;
mod std;
mod tui;

use crate::{
    allocator::BumpAllocator,
    drivers::vga::{Color, Writer, writer},
    fs::FileSystem,
};
#[cfg(not(test))]
use core::arch::global_asm;

const HEAP_START: usize = 0x0040_0000;
const HEAP_END: usize = 0x0080_0000;

#[global_allocator]
pub static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();
pub static mut WRITER: Writer = Writer::new(0, 0, Color::White, Color::Black);
pub static mut FS: FileSystem = FileSystem::new();

#[cfg(not(test))]
global_asm!(include_str!("arch/boot.asm"));

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    writer().clear();

    unsafe {
        ALLOCATOR.init(HEAP_START, HEAP_END);
    }

    shell::run();

    arch::cpu::halt();
}
