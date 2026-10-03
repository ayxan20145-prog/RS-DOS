use core::{
    alloc::Layout,
    ptr::{self, null_mut},
};

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
    pub fn push(&mut self, byte: u8) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            self.ptr.add(self.len).write(byte);
        }

        self.len += 1;
    }
    pub fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 8 } else { self.cap * 2 };
        let layout = Layout::array::<u8>(new_cap).unwrap();
        let new_ptr = unsafe { alloc::alloc::alloc(layout) };

        if new_ptr.is_null() {
            panic!("allocation failed");
        }

        if self.len > 0 {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
            }
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}
