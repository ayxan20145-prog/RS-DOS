use crate::{WRITER, arch::io::outb};
use core::fmt::{self, Write};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

pub struct Writer {
    pub column: usize,
    pub row: usize,
    pub fg: Color,
    pub bg: Color,
}

impl Writer {
    pub const fn new(column: usize, row: usize, fg: Color, bg: Color) -> Self {
        Self {
            column,
            row,
            fg,
            bg,
        }
    }
    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.fg = fg;
        self.bg = bg;
    }
    pub fn color_byte(&self) -> u8 {
        Color::vga_color(self.fg, self.bg)
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

        if self.row >= 25 {
            self.clear();
            self.reset_cursor();
        }

        write_byte_at(self.column, self.row, byte, self.fg, self.bg);

        self.column += 1;
        self.update_cursor(self.column, self.row);
    }
    pub fn write_string(&mut self, text: &str) {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
    }
    pub fn clear(&mut self) {
        for y in 0..25 {
            for x in 0..80 {
                write_byte_at(x, y, b' ', self.fg, self.bg);
            }
        }
    }
    pub fn update_cursor(&self, column: usize, row: usize) {
        let position = row * 80 + column;

        outb(0x3d4, 0x0f);
        outb(0x3d5, (position & 0xff) as u8);
        outb(0x3d4, 0x0e);
        outb(0x3d5, ((position >> 8) & 0xff) as u8);
    }
    pub fn reset_cursor(&mut self) {
        self.column = 0;
        self.row = 0;
        self.update_cursor(0, 0);
    }
}

pub fn write_byte_at(x: usize, y: usize, byte: u8, fg: Color, bg: Color) {
    if x >= 80 || y >= 25 {
        return;
    }

    let index = y * 80 + x;

    unsafe {
        *VGA_BUFFER.add(index * 2) = byte;
        *VGA_BUFFER.add(index * 2 + 1) = Color::vga_color(fg, bg);
    }
}

impl Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_string(text);

        Ok(())
    }
}

impl Color {
    pub fn vga_color(fg: Color, bg: Color) -> u8 {
        ((bg as u8) << 4) | (fg as u8)
    }
}

pub fn writer() -> &'static mut Writer {
    unsafe { &mut WRITER }
}
