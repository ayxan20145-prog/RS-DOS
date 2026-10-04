use core::alloc::{GlobalAlloc, Layout};

static mut NEXT: usize = 0;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
}
impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_end;
        *next() = heap_start;
    }
}
unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = *next();
        *next() += layout.size();
        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!();
    }
}
pub fn next() -> &'static mut usize {
    unsafe { &mut NEXT }
}
