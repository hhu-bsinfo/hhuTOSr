/* ╔═════════════════════════════════════════════════════════════════════════╗
 *  ║ Module: list                                                            ║
 *  ╟─────────────────────────────────────────────────────────────────────────╢
 *  ║ Descr.: Implementing a list heap allocator.                             ║
 *  ╟─────────────────────────────────────────────────────────────────────────╢
 *  ║ Author: Philipp Oppermann                                               ║
 *  ║         https://os.phil-opp.com/allocator-designs/                      ║
 *  ╚═════════════════════════════════════════════════════════════════════════╝
 */
use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};
use crate::kernel::allocator::bump::BumpAllocator;
use crate::kernel::cpu as cpu;

/// Header of a free block in the list allocator.
struct ListNode {
    /// Size of the memory block
    size: usize,

    /// &'static mut type semantically describes an owned object behind
    /// a pointer. Basically, it’s a Box without a destructor that frees
    /// the object at the end of the scope. Its lifetime is static,
    /// meaning it will live for the entire duration of the program.
    /// Of course, this is not true in reality, as we might delete the
    /// list node at some point. But the compiler does not know this.
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    /// Creates a new ListNode with the given size and no next node.
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    /// Get the start address of the memory block.
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    /// Get the end address of the memory block.
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

/// A linked list allocator that uses a free list to manage memory.
pub struct LinkedListAllocator {
    head: ListNode,
    heap_start: usize,
    heap_end: usize,
}

impl LinkedListAllocator {
    /// Create a new empty linked list allocator.
    pub const fn new(heap_start: usize, heap_size: usize) -> LinkedListAllocator {
        LinkedListAllocator {
            head: ListNode::new(heap_size),
            heap_start,
            heap_end: heap_start + heap_size,
        }
    }

    /// Initialize the allocator with the heap bounds given in the constructor.
    pub unsafe fn init(&mut self) {

        /* Hier muss Code eingefuegt werden */

    }

    /// Adds the given free memory block 'addr' to the front of the free list.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) {

        /* Hier muss Code eingefuegt werden */

    }

    /// Search a free block with the given size and alignment and remove it from the list.
    fn find_free_block(&mut self, size: usize, align: usize) -> Option<&'static mut ListNode> {

        /* Hier muss Code eingefuegt werden */

    }

    /// Check if the given block is large enough for an allocation with `size` and `align`.
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize) -> Result<(),()> {

        /* Hier muss Code eingefuegt werden */

    }

    /// Adjust the given layout so that the resulting allocated memory
    /// block is also capable of storing a `ListNode`.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
        .align_to(align_of::<ListNode>())
        .expect("adjusting alignment failed")
        .pad_to_align();
        let size = layout.size().max(size_of::<ListNode>());

        (size, layout.align())
    }

    /// Dump the free list for debugging purposes.
    pub fn dump_free_list(&mut self) {

        /* Hier muss Code eingefuegt werden */

    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        kprint!("list-alloc: size={}, align={}", layout.size(), layout.align());

        /* Hier muss Code eingefuegt werden */

    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        kprintln!("list-dealloc: size={}, align={}; not supported", layout.size(), layout.align());

        let (size, _) = LinkedListAllocator::size_align(layout);

        unsafe {
            self.add_free_block(ptr as usize, size)
        }
    }

}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            self.lock().alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.lock().dealloc(ptr, layout);
        }
    }
}
