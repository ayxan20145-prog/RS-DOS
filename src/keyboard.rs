use x86::io::inb;

pub const KEYMAP: [Option<char>; 128] = {
    let mut map = [None; 128];
    map[0x02] = Some('1');
    map[0x03] = Some('2');

    map
};

pub fn read_scancode() -> u8 {
    unsafe {
        while (inb(0x64) & 1) == 0 {
            continue;
        }
        inb(0x60)
    }
}

pub fn read_key() -> Option<char> {
    let scancode = read_scancode();

    if (scancode & 0x80) != 0 {
        return None;
    }

    KEYMAP[scancode as usize]
}
