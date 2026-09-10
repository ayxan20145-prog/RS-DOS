use crate::vga::{Color, Writer};
use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(34, 12, Color::Black, Color::Red);

    writer.clear();
    write!(writer, "KERNEL PANIC\n{}", info).unwrap();

    loop {}
}
