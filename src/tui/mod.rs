use crate::drivers::vga::{Color, write_byte_at};

pub fn draw_rect(x: u32, y: u32, width: u32, height: u32, byte: u8, fg: Color, bg: Color) {
    for dy in 0..height {
        for dx in 0..width {
            write_byte_at((x + dx) as usize, (y + dy) as usize, byte, fg, bg);
        }
    }
}
