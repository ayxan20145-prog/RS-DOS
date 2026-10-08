use crate::ALLOCATOR;
use core::ptr::{self, NonNull};

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub const fn new() -> Self {
        Self {
            ptr: NonNull::dangling().as_ptr(),
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
        let new_size = new_cap * size_of::<T>();
        let new_ptr = unsafe { ALLOCATOR.alloc(new_size) };

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
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&*self.ptr.add(index)) }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe { Some(self.ptr.add(self.len).read()) }
    }
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        unsafe {
            let value = self.ptr.add(index).read();

            ptr::copy(
                self.ptr.add(index + 1),
                self.ptr.add(index),
                self.len - index - 1,
            );

            self.len -= 1;

            value
        }
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new = Self::new();

        for i in 0..self.len {
            unsafe {
                new.push((*self.ptr.add(i)).clone());
            }
        }

        new
    }
}
