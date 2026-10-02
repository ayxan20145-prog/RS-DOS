#[cfg(not(test))]
use crate::drivers::vga::{Color, Writer};
#[cfg(not(test))]
use core::{fmt::Write, panic::PanicInfo};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(34, 12, Color::Black, Color::Red);

    writer.clear();
    write!(writer, "KERNEL PANIC\n{}", info).unwrap();

    loop {}
}
