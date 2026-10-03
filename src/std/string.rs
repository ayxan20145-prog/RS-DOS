use core::ptr::null_mut;

pub struct String {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl String {
    pub fn new() -> Self {
        Self {
            ptr: null_mut(),
            len: 0,
            cap: 0,
        }
    }
}
