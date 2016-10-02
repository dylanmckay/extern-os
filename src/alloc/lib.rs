#![feature(allocator)]

#![allocator]
#![no_std]

extern crate linked_list_allocator;

use linked_list_allocator::Heap;

/// How many bytes in the heap.
const HEAP_SIZE: usize = 100000;
/// The global heap.
static mut HEAP: Option<Heap> = None;

#[allow(improper_ctypes)]
extern {
    /// A symbol placed at the bottom of the stack.
    static stack_bottom: ();
}

fn heap() -> &'static mut Heap {
    unsafe {
        if !HEAP.is_some() {
            let heap_top = &stack_bottom as *const _ as usize;
            let heap_bottom = heap_top - HEAP_SIZE;

            let heap = Heap::new(heap_bottom, HEAP_SIZE);

            HEAP = Some(heap);
        }

        HEAP.as_mut().unwrap()
    }
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    heap().allocate_first_fit(size, align).unwrap()
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    unsafe {
        heap().deallocate(ptr, size, align)
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                                align: usize) -> *mut u8 {
    unsafe {
        heap().deallocate(ptr, old_size, align);
        heap().allocate_first_fit(size, align).unwrap()
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

