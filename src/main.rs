#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    fmt::{self, Write},
    panic::PanicInfo,
};
use x86::io::outb;

global_asm!(include_str!("boot.asm"));

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

struct Writer {
    column: usize,
    row: usize,
    color: u8,
}

impl Writer {
    fn new(column: usize, row: usize, color: u8) -> Self {
        Self { column, row, color }
    }
    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.column = 0;
            self.row += 1;
            self.update_cursor(self.column, self.row);
            return;
        }

        if self.column >= 80 {
            self.column = 0;
            self.row += 1;
        }

        unsafe {
            let position = self.row * 80 + self.column;

            *VGA_BUFFER.add(position * 2) = byte;
            *VGA_BUFFER.add(position * 2 + 1) = self.color;
        }

        self.column += 1;
        self.update_cursor(self.column, self.row);
    }
    fn write_str(&mut self, text: &str) {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
    }
    fn clear(&mut self, background: u8) {
        for i in 0..2000 {
            unsafe {
                *VGA_BUFFER.add(i * 2) = b' ';
                *VGA_BUFFER.add(i * 2 + 1) = background;
            }
        }
    }
    fn update_cursor(&self, x: usize, y: usize) {
        let position = (y * 80) + x;

        unsafe {
            outb(0x3d4, 0x0f);
            outb(0x3d5, (position & 0xff) as u8);
            outb(0x3d4, 0x0e);
            outb(0x3d5, ((position >> 8) & 0xff) as u8);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_str(text);

        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer::new(0, 0, 0x0F);
    writer.clear(0x0F);
    write!(
        writer,
        "+------------------+\n|                  |\n|      RS-DOS      |\n|                  |\n+------------------+\n"
    )
    .unwrap();
    write!(writer, "\nC:\\>").unwrap();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(34, 12, 0x40);

    writer.clear(0x40);
    write!(writer, "KERNEL PANIC\n{}", info).unwrap();

    loop {}
}
