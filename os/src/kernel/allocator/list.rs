/*
 * Module: list
 *
 * Description: A heap allocator that uses a linked list to manage free memory blocks.
 *              It allows for dynamic memory allocation and deallocation.
 *
 * Author: Philipp Oppermann
 *         https://os.phil-opp.com/allocator-designs/
 */

use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};

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
        let size = self.heap_end - self.heap_start;
        
        unsafe {
            self.add_free_block(self.heap_start, size);
        }
    }
    
    /// Adds the given free memory block 'addr' to the front of the free list.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) {
        // Create a new ListNode with the given size put it at the front of the list
        let mut node = ListNode::new(size);
        node.next = self.head.next.take(); // Let `next` point to the old head
        
        unsafe {
            let node_ptr = addr as *mut ListNode; // Node lies at the beginning of the block
            node_ptr.write(node); // Write the new node to the memory block
            self.head.next = Some(&mut *node_ptr); // Set `head` to the new node
        }
    }
    
    /// Search a free block with the given size and alignment and remove it from the list.
    fn find_free_block(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head; // Reference to the current node in the list (updated with each iteration)
        
        while let Some(ref mut block) = current.next {
            if let Ok(alloc_start) = Self::check_block_for_alloc(&block, size, align) {
                let next = block.next.take(); // Save successor of `block`
                let ret = Some((current.next.take().unwrap(), alloc_start)); // Take `block`
                current.next = next; // Set `next` to successor of `block`
                
                return ret; // Return the block and the start address of the allocation inside the block
            } else {
                // Block too small -> Continue with next block
                current = current.next.as_mut().unwrap();
            }
        }
        
        None
    }
    
    /// Check if the given block is large enough for an allocation with `size` and `align`.
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize) -> Result<usize,()> {
        let start = align_up(block.start_addr(), align);
        let end = start + size;
        
        // Check if the block is too small
        if end > block.end_addr() {
            return Err(());
        }

        // Check if the rest of the block is large enough to hold a ListNode.
        // This is required, because the allocation splits the block in a used and a free part.
        let rest = block.end_addr() - end;
        if rest != 0 && rest < size_of::<ListNode>() {
            return Err(());
        }
        
        // Block is suitable for allocation
        Ok(start)
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
        println!("Linked list allocator:");
        println!("  Heap start: {:#x}, Heap end: {:#x}", self.heap_start, self.heap_end);
        println!("  Free blocks:");
        
        let mut current = &self.head;
        while let Some(ref block) = current.next {
            println!("    Block at {:#x} with size {}", block.start_addr(), block.size);
            current = current.next.as_ref().unwrap();
        }
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // Perform layout adjustments
        let (size, align) = LinkedListAllocator::size_align(layout);

        if let Some((block, alloc_start)) = self.find_free_block(size, align) {
            let alloc_end = alloc_start + size;

            // The remaining memory will be inserted as new block
            // if the size is large enough to store metadata.
            // This is checked in `check_block_for_alloc()` called by `find_free_block()`.
            let remaining_block_size = block.end_addr() - alloc_end;
            if remaining_block_size > 0 {
                unsafe {
                    self.add_free_block(alloc_end, remaining_block_size);
                }
            }
            
            return alloc_start as *mut u8;
        }

        panic!("List allocator: Out of memory!");
   }
    
   pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
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