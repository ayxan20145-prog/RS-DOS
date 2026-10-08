use crate::ALLOCATOR;
use core::ops::{Deref, DerefMut};

pub struct Box<T> {
    ptr: *mut T,
}

impl<T> Box<T> {
    pub fn new(value: T) -> Self {
        let ptr = unsafe { ALLOCATOR.alloc(size_of::<T>()) as *mut T };

        if ptr.is_null() {
            panic!("allocation failed");
        }

        unsafe {
            ptr.write(value);
        }

        Self { ptr }
    }
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
