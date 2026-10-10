pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}
impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_end;
        self.next = heap_start;
    }
    pub unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        let alloc_start = self.next;
        let alloc_end = match alloc_start.checked_add(size) {
            Some(end) => end,
            None => return core::ptr::null_mut(), // overflow
        };

        if alloc_end > self.heap_end {
            return core::ptr::null_mut(); // out of memory
        }

        self.next += size;
        alloc_start as *mut u8
    }
}
