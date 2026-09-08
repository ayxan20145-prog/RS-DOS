use crate::vga::Writer;
use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(34, 12, 0x40);

    writer.clear(0x40);
    write!(writer, "KERNEL PANIC\n{}", info).unwrap();

    loop {}
}
