use crate::drivers::vga::{Color, write_byte_at};

pub fn draw_rect(x: u32, y: u32, width: u32, height: u32, byte: u8, fg: Color, bg: Color) {
    for dy in 0..height {
        for dx in 0..width {
            write_byte_at((x + dx) as usize, (y + dy) as usize, byte, fg, bg);
        }
    }
}
pub fn clear(byte: u8, fg: Color, bg: Color) {
    draw_rect(0, 0, 80, 25, byte, fg, bg);
}
pub fn draw_text(x: u32, y: u32, text: &str, fg: Color, bg: Color) {
    for (i, byte) in text.bytes().enumerate() {
        write_byte_at(x as usize + i, y as usize, byte, fg, bg);
    }
}
