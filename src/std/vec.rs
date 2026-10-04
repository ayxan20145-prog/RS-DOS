use crate::ALLOCATOR;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::{self, null_mut},
};

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
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            self.ptr.add(self.len).write(value);
        }

        self.len += 1;
    }
    pub fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 8 } else { self.cap * 2 };
        let layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr = unsafe { ALLOCATOR.alloc(layout) };

        if new_ptr.is_null() {
            panic!("allocation failed");
        }

        if self.len > 0 {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr.cast::<T>(), self.len);
            }
        }

        self.ptr = new_ptr.cast::<T>();
        self.cap = new_cap;
    }
}
