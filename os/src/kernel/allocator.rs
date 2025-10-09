/*
 * Module: allocator
 *
 * Description: Implements functions for the heap allocator used by the rust compiler.
 *
 * Memory-Layout:
 *    0x0        Real mode & BIOS stuff
 *    0x100000   Our OS image, including global variables
 *    0x800000   Start address of our heap
 *
 * Remarks:
 *    - Requires a PC with at least 8 MiB RAM
 *    - Lowest loading address for GRUB is 1 MiB
 *    - If your OS image grows larger than 7 MiB, you need to increase the heap start address.
 *
 * Author: Philipp Oppermann
 *         https://os.phil-opp.com/allocator-designs/
 */


use alloc::alloc::Layout;
use crate::consts;
use crate::kernel::allocator::list::LinkedListAllocator;
use spin::mutex::{Mutex, MutexGuard};

pub mod list;

// Define the allocator (which implements the 'GlobalAlloc' trait)
#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new(consts::HEAP_START, consts::HEAP_SIZE));

/// Initialize the heap allocator.
pub fn init() {
    unsafe {
        ALLOCATOR.lock().init();
    }
}

/// Check if the heap allocator is locked.
pub fn is_locked() -> bool {
    ALLOCATOR.inner.is_locked()
}

/// Allocates memory from the heap. Compiler generates code calling this function.
pub fn alloc(layout: Layout) -> *mut u8 {
    unsafe {
        ALLOCATOR.lock().alloc(layout)
    }
}

/// Deallocates memory from the heap. Compiler generates code calling this function.
pub fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe {
        ALLOCATOR.lock().dealloc(ptr, layout)
    }
}

/// Dump heap free list. Must be called by own program.
/// Can be used for debugging the heap allocator. 
pub fn dump_free_list() {
    ALLOCATOR.lock().dump_free_list();
}

/// A wrapper around `spin::Mutex` to allow for trait implementations.
/// Required for implementing `GlobalAlloc` in `bump.rs` and `list.rs`.
pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&'_ self) -> MutexGuard<'_, A> {
        self.inner.lock()
    }
}

/// Helper function used in `bump.rs` and `list.rs`. Rust requires pointers to be aligned.
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr // addr already aligned
    } else {
        addr - remainder + align
    }
}
