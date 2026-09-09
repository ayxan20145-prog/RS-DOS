use crate::io::inb;

pub const KEYMAP: [Option<char>; 128] = {
    let mut map = [None; 128];

    // number row
    map[0x02] = Some('1');
    map[0x03] = Some('2');
    map[0x04] = Some('3');
    map[0x05] = Some('4');
    map[0x06] = Some('5');
    map[0x07] = Some('6');
    map[0x08] = Some('7');
    map[0x09] = Some('8');
    map[0x0A] = Some('9');
    map[0x0B] = Some('0');

    // main keyboard
    map[0x10] = Some('q');
    map[0x11] = Some('w');
    map[0x12] = Some('e');
    map[0x13] = Some('r');
    map[0x14] = Some('t');
    map[0x15] = Some('y');
    map[0x16] = Some('u');
    map[0x17] = Some('i');
    map[0x18] = Some('o');
    map[0x19] = Some('p');

    map[0x1E] = Some('a');
    map[0x1F] = Some('s');
    map[0x20] = Some('d');
    map[0x21] = Some('f');
    map[0x22] = Some('g');
    map[0x23] = Some('h');
    map[0x24] = Some('j');
    map[0x25] = Some('k');
    map[0x26] = Some('l');

    map[0x2C] = Some('z');
    map[0x2D] = Some('x');
    map[0x2E] = Some('c');
    map[0x2F] = Some('v');
    map[0x30] = Some('b');
    map[0x31] = Some('n');
    map[0x32] = Some('m');

    // symbols
    map[0x0C] = Some('-');
    map[0x0D] = Some('=');
    map[0x1A] = Some('[');
    map[0x1B] = Some(']');
    map[0x27] = Some(';');
    map[0x28] = Some('\'');
    map[0x29] = Some('`');
    map[0x2B] = Some('\\');
    map[0x33] = Some(',');
    map[0x34] = Some('.');
    map[0x35] = Some('/');

    // other keys
    map[0x1c] = Some('\n');
    map[0x39] = Some(' ');
    map[0x0E] = Some('\x08');

    map
};

pub fn read_scancode() -> u8 {
    while (inb(0x64) & 1) == 0 {
        continue;
    }
    inb(0x60)
}

pub fn read_key() -> Option<char> {
    let scancode = read_scancode();

    if (scancode & 0x80) != 0 {
        return None;
    }

    KEYMAP[scancode as usize]
}
