#![no_std]

const MAX_ALLOCATIONS: usize = 128;
const ALIGNMENT: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Allocation {
    used: usize,
    start: usize,
    size: usize,
}

#[repr(C)]
pub struct RsAlloc {
    heap_start: usize,
    heap_end: usize,
    next_addr: usize,
    total_alloc: usize,
}


fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}


impl RsAlloc {
    const fn new() -> Self {
        Self {
	    heap_start: 0,
            heap_end: 0,
	    next_addr: 0,
	    total_alloc: 0,
        }
    }

    fn init(&mut self, heap_base: usize, heap_size: usize) {
        self.heap_start = heap_base;
        self.heap_end = heap_base + heap_size;
	self.next_addr = heap_base;
	self.total_alloc = 0;
    }

    fn malloc(&mut self, size: usize) -> Option<*mut u8> {
	if size == 0 {return None;}
	let start_addr = self.next_addr;
	let end_addr = start_addr + size;

	if end_addr >= self.heap_end {
	    None
	} else {
	    self.next_addr = end_addr;
	    self.total_alloc += 1;
	    Some(start_addr as *mut u8)
	}

    }

    fn free(&mut self, ptr: *mut u8) {
	self.total_alloc -= 1;
	if self.total_alloc == 0 {
	    self.next_addr = self.heap_start;
	}
    }

    fn addmem(&mut self, base: *mut u8, value: usize) -> i32 {
	self.heap_end += value;
	0
    }

    fn availmem(&mut self) -> usize {
	self.heap_end - self.next_addr
    }
}

#[no_mangle]
pub extern "C" fn rsalloc_init(allocator: *mut RsAlloc, heap_base: *mut u8, heap_size: usize) {
	let allocator = unsafe {&mut *allocator};
	allocator.init(heap_base as usize, heap_size);
}

#[no_mangle]
pub extern "C" fn rsalloc_malloc(allocator: *mut RsAlloc, size: usize) -> *mut u8 {
	let allocator = unsafe {&mut *allocator};
	allocator.malloc(size).unwrap_or(core::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn rsalloc_free(allocator: *mut RsAlloc, ptr: *mut u8) {
	let allocator = unsafe {&mut *allocator};
	allocator.free(ptr);
}

#[no_mangle]
pub extern "C" fn rsalloc_addmem(allocator: *mut RsAlloc, base: *mut u8, len: usize) -> i32 {
    let allocator = unsafe {&mut *allocator};
    return allocator.addmem(base, len);
}

// #[no_mangle]
// pub extern "C" fn rsalloc_availmem(allocator: *mut RsAlloc) -> usize {
//     let allocator = unsafe {& *allocator};
//     allocator.availmem()
// }
