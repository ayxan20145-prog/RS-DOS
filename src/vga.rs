use core::fmt::{self, Write};
use x86::io::outb;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub struct Writer {
    column: usize,
    row: usize,
    color: u8,
}

impl Writer {
    pub fn new(column: usize, row: usize, color: u8) -> Self {
        Self { column, row, color }
    }
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.column = 0;
            self.row += 1;
            self.update_cursor(self.column, self.row);
            return;
        }

        if byte == b'\x08' {
            if self.column > 0 {
                self.column -= 1;
                self.write_byte(b' ');
                self.column -= 1;
                self.update_cursor(self.column, self.row);
                return;
            }
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
    pub fn write_string(&mut self, text: &str) {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
    }
    pub fn clear(&mut self, background: u8) {
        for i in 0..2000 {
            unsafe {
                *VGA_BUFFER.add(i * 2) = b' ';
                *VGA_BUFFER.add(i * 2 + 1) = background;
            }
            self.column = 0;
            self.row = 0;
            self.update_cursor(0, 0);
        }
    }
    fn update_cursor(&self, column: usize, row: usize) {
        let position = row * 80 + column;

        unsafe {
            outb(0x3d4, 0x0f);
            outb(0x3d5, (position & 0xff) as u8);
            outb(0x3d4, 0x0e);
            outb(0x3d5, ((position >> 8) & 0xff) as u8);
        }
    }
    pub fn reset_cursor(&mut self) {
        self.column = 0;
        self.row = 0;
        self.update_cursor(0, 0);
    }
}

impl Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_string(text);

        Ok(())
    }
}
