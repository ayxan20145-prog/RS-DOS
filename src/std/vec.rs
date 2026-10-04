use core::ptr::null_mut;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            ptr: null_mut(),
            len: 0,
            cap: 0,
        }
    }
}
