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
    allocations: [Allocation; MAX_ALLOCATIONS],
    free_allocations: [bool; MAX_ALLOCATIONS],
    next_free_slot: usize,
    heap_start: usize,
    heap_end: usize,
    mem_used: usize,
}

impl RsAlloc {
    const fn new() -> Self {
        Self {
            allocations: [Allocation{used: 0, start: 0, size: 0}; MAX_ALLOCATIONS],
            free_allocations: [true; MAX_ALLOCATIONS],
	    next_free_slot: 0,
	    heap_start: 0,
            heap_end: 0,
	    mem_used: 0,
        }
    }

    fn init(&mut self, heap_base: usize, heap_size: usize) {
        self.heap_start = heap_base;
        self.heap_end = heap_base + heap_size;
	self.mem_used = 0;
    }

    fn malloc(&mut self, size: usize) -> Option<*mut u8> {
	if size == 0 {return None;}
	let mut current_address = self.heap_start;
        for allocation in &mut self.allocations {
            if allocation.used == 0 {
                if current_address + size <= self.heap_end {
                    allocation.used = 1;
                    allocation.start = current_address;
                    allocation.size = size;
                    current_address += size;
		    self.mem_used += size;
                    return Some(allocation.start as *mut u8);
                }
            } else {
                current_address += allocation.size;
            }
        }
        None
    }

    fn free(&mut self, ptr: *mut u8) {
        let address = ptr as usize;
        for allocation in &mut self.allocations {
            if allocation.used == 1 && allocation.start == address {
                allocation.used = 0;
                self.mem_used -= allocation.size;
		break;
            }
        }
    }

    fn addmem(&mut self, value: usize) -> i32 {
	self.heap_end += value;
	return 0;
    }

    fn availmem(&mut self) -> usize {
	self.heap_end - self.heap_start - self.mem_used
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
pub extern "C" fn rsalloc_addmem(allocator: *mut RsAlloc, len: usize) -> i32 {
    let allocator = unsafe {&mut *allocator};
    return allocator.addmem(len);
}

// #[no_mangle]
// pub extern "C" fn rsalloc_availmem(allocator: *mut RsAlloc) -> usize {
//     let allocator = unsafe {& *allocator};
//     allocator.availmem()
// }
