/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: allocator                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implementing functions for the heap allocator used by the rust  ║
   ║         compiler.                                                       ║
   ║                                                                         ║ 
   ║         Memory-Layout                                                   ║
   ║            0x0        real mode & bios stuff       	                 ║
   ║            0x100000   our OS image, including global variables          ║ 
   ║            0x500000   Start address of our heap                         ║ 
   ║                                                                         ║ 
   ║         Remarks                                                         ║
   ║            - Requires a PC with at least 8 MB RAM                       ║
   ║            - Lowest loading address for grub is 1 MB                    ║ 
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann                                               ║
   ║         https://os.phil-opp.com/allocator-designs/                      ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::alloc::Layout;
use crate::kernel::allocator::bump::BumpAllocator;
use crate::kernel::allocator::list::LinkedListAllocator;

pub mod bump;
pub mod list;

const HEAP_START: usize = 0x500000;
const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB heap size

// Define the allocator (which implements the 'GlobalAlloc' trait)
#[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new(HEAP_START, HEAP_SIZE));
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new(HEAP_START, HEAP_SIZE));

/// Initialize the heap allocator.
pub fn init() {
    unsafe {
        ALLOCATOR.lock().init();
    }
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
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
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
