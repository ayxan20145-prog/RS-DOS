use core::arch::{asm, x86::__cpuid};

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
pub fn vendor() -> [[u8; 4]; 3] {
    let result = unsafe { __cpuid(0) };

    [
        result.ebx.to_le_bytes(),
        result.edx.to_le_bytes(),
        result.ecx.to_le_bytes(),
    ]
}
