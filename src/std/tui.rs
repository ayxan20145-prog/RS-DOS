use crate::drivers::vga::{Color, write_byte_at};

pub fn draw_rect(x: usize, y: usize, width: usize, height: usize, byte: u8, fg: Color, bg: Color) {
    for dy in 0..height {
        for dx in 0..width {
            write_byte_at(x + dx, y + dy, byte, fg, bg);
        }
    }
}
