use core::alloc::GlobalAlloc;

use crate::lock::Locked;

pub enum AllocError {
    OutOfMemory,
    MemAddrOverflow,
}

pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

/// upward align memory address by the given alignment
fn align_up(ptr_addr: usize, align: usize) -> usize {
    let remainder = ptr_addr % align;
    if remainder == 0 {
        ptr_addr
    } else {
        ptr_addr - remainder + align
    }
}

unsafe impl GlobalAlloc for Locked<Allocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.lock();

        // align memory start address
        let start_addr = align_up(allocator.next, layout.align());
        let end_addr = start_addr
            .checked_add(layout.size())
            .ok_or(AllocError::MemAddrOverflow);

        // overflow check
        let end_addr = match end_addr {
            Ok(ptr) => ptr,
            Err(e) => return core::ptr::null_mut(),
        };

        // out of memory check
        if end_addr > allocator.heap_end {
            return core::ptr::null_mut();
        }

        let ptr = allocator.next;
        allocator.next += layout.size();
        allocator.allocations += 1;

        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = self.lock();

        allocator.allocations -= 1;

        if allocator.allocations == 0 {
            allocator.next = allocator.heap_start;
        }
    }
}
