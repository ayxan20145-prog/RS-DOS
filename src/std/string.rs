use crate::ALLOCATOR;
use core::{
    fmt,
    ops::Deref,
    ptr::{self, null_mut},
};

#[derive(PartialEq)]
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
    pub fn push_str(&mut self, text: &str) {
        for byte in text.as_bytes() {
            self.push(*byte);
        }
    }
    pub fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 8 } else { self.cap * 2 };
        let new_ptr = unsafe { ALLOCATOR.alloc(new_cap) };

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
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.ptr, self.len)) }
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        let mut string = Self::new();
        string.push_str(value);
        string
    }
}
impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
impl fmt::Write for String {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}
impl Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
impl Clone for String {
    fn clone(&self) -> Self {
        let mut string = Self::new();

        for i in 0..self.len {
            unsafe {
                string.push(*self.ptr.add(i));
            }
        }

        string
    }
}
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut s = String::new();
        use core::fmt::Write;
        write!(&mut s, $($arg)*).unwrap();
        s
    }};
}
