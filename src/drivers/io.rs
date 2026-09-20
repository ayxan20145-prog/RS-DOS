use core::arch::asm;

pub fn inb(port: u16) -> u8 {
    let mut value: u8;

    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value,);
    }

    value
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write!(writer(), $($arg)*).unwrap();
    };
}
