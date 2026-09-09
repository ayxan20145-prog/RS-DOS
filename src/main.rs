#![no_std]
#![no_main]

mod panic;
mod vga;

use core::{arch::global_asm, fmt::Write};
use x86::io::inb;

global_asm!(include_str!("boot.asm"));

const KEYMAP: [Option<char>; 128] = {
    let mut map = [None; 128];
    map[0x02] = Some('1');
    map[0x03] = Some('2');

    map
};

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
        if let Some(key) = read_key() {
            write!(writer, "{}", key).unwrap();
        }
    }
}

fn read_scancode() -> u8 {
    unsafe {
        while (inb(0x64) & 1) == 0 {
            continue;
        }
        inb(0x60)
    }
}

fn read_key() -> Option<char> {
    let scancode = read_scancode();

    if (scancode & 0x80) != 0 {
        return None;
    }

    KEYMAP[scancode as usize]
}
