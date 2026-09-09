#![no_std]
#![no_main]

mod keyboard;
mod panic;
mod vga;

use core::{arch::global_asm, fmt::Write};

global_asm!(include_str!("boot.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = vga::Writer::new(0, 0, 0x0F);
    writer.clear(0x0F);
    write!(
        writer,
        "+------------------+\n|                  |\n|      RS-DOS      |\n|                  |\n+------------------+\n"
    )
    .unwrap();
    write!(writer, "\nC:\\>").unwrap();

    loop {
        if let Some(key) = keyboard::read_key() {
            match key {
                '\n' => {
                    write!(writer, "\nC:\\>").unwrap();
                }
                _ => {
                    write!(writer, "{}", key).unwrap();
                }
            }
        }
    }
}
