/*****************************************************************************
 *                                                                           *
 *                                 L I S T                                   *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Eine einfache Heap-Verwaltung, welche freie Bloecke      *
 *                  verkettet und wieder benutzen kann.                      *
 *                                                                           *
 * Autor:           Philipp Oppermann                                        *
 *                  https://os.phil-opp.com/allocator-designs/               *
 *                  Modified by Michael Schoettner, 11.5.2022                *
 *****************************************************************************/

use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};


// metadata for each free memory block
struct ListNode {
	// size of the memory block
    size: usize,
    
    // &'static mut type semantically describes an owned object behind 
    // a pointer. Basically, it’s a Box without a destructor that frees 
    // the object at the end of the scope.
    next: Option<&'static mut ListNode>,
}


impl ListNode {
	
	// must be 'const', required later when constructing 
	// a static linked list allocator 
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    // return start address of memory block
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    // return end address of memory block
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}


pub struct LinkedListAllocator {
    head: ListNode,
    heap_start: usize,
    heap_end: usize,
}


impl LinkedListAllocator {
	
    // Creates an empty LinkedListAllocator.
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
            heap_start: 0,
            heap_end: 0,
        }
    }


    // Initialize the allocator with the given heap bounds.
    //
    // This function is unsafe because the caller must guarantee that 
    // the given heap bounds are valid. This method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        let mut start_node = ListNode::new(heap_size);
        start_node.next = self.head.next.take();
        let start_ptr = heap_start as *mut ListNode;
        start_ptr.write(start_node);
        self.head.next = Some(&mut *start_ptr);
    }


    // Adds the given free memory block 'addr' to the front of the free list.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) {
        let mut new_node = ListNode::new(size);
        new_node.next = self.head.next.take(); // Head wird in Node eingehangen
        let addr_ptr = addr as *mut ListNode;
        addr_ptr.write(new_node);
        self.head.next = Some(&mut *addr_ptr);
    }
    
    
    // Search a free block with the given size and alignment and remove
    // it from the free list.
    //
    // Return: 'ListNode' or 'None'
    fn find_free_block(&mut self, size: usize, align: usize)
        -> Option<&'static mut ListNode>
    {
        let mut curr_node = &mut self.head;
        while let Some(ref mut block) = curr_node.next {
            if let Ok(addr) = Self::check_block_for_alloc(&block, size, align) {
                let after_block = block.next.take();
                let ret_block = Some(curr_node.next.take().unwrap()); // Nimm Block aus Liste
                curr_node.next = after_block; // Hängt Nachfolger in Vorgänger
                return ret_block
            } else {
                curr_node = curr_node.next.as_mut().unwrap();
            }
        }
        return None;
    }
    
    
    // Check if the given 'block' is large enough for an allocation with  
    // 'size' and alignment 'align'
    //
    // Return: OK(allocation start address) or Err 
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize)
        -> Result<usize, ()>
    {
        let mem_start = align_up(block.start_addr(), align);
        let mem_end = mem_start.checked_add(size).ok_or(())?;
        if mem_end > block.end_addr() {
            return Err(()); // Block ist zu klein
        }
        let unused_size = block.end_addr() - mem_end;
        if unused_size > 0 && unused_size < mem::size_of::<ListNode>() {
            return Err(()); // Kein Platz für ListNode im Rest des Blocks
        }
        return Ok(mem_start);
    }


    // Dump free list
    pub fn dump_free_list(&mut self) {
        println!("\nFreispeicherliste:");
        println!("   Heap-Start:   {:#x}, Heap-End:  {:#x}", self.heap_start, self.heap_end);
        let mut current = &mut self.head;
        /*println!("   Block-Start:  {:#x}, Block-End: {:#x}, Block-Size: {:#x}",
                current.start_addr(), current.end_addr(), current.size);
        match &current.next {
            Some(X) => println!("next:start,end,size\t{:#x} {:#x} {:#x}",
                                            X.start_addr(),X.end_addr(),X.size),
            None => println!("Next is None")
        }*/
        while let Some(ref mut block) = current.next {
            println!("   Block-Start:  {:#x}, Block-End: {:#x}, Block-Size: {:#x}",
                block.start_addr(), block.end_addr(), block.size);
            current = current.next.as_mut().unwrap();
        }
    }

    
    // Adjust the given layout so that the resulting allocated memory
    // block is also capable of storing a `ListNode`.
    //
    // Returns the adjusted size and alignment as a (size, align) tuple.
    fn size_align(layout: Layout) -> (usize, usize) {
	    let layout = layout
             .align_to(mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }
    
    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
       // print!("   alloc: size={}, align={}", layout.size(), layout.align());

       // perform layout adjustments
       let (size, align) = LinkedListAllocator::size_align(layout);
       let ret_ptr: *mut u8; 

       if let Some(block) = self.find_free_block(size, align) {
           let alloc_end = block.start_addr().checked_add(size).expect("overflow");
            
           // the remaining memory will be inserted as new block
           // the size is large enough to store metadata; this is 
           // checked in 'check_block_for_alloc' called by 'find_free_block'
           let remaining_block_size = block.end_addr() - alloc_end;
           if remaining_block_size > 0 {
               self.add_free_block(alloc_end, remaining_block_size);
           }
           ret_ptr = block.start_addr() as *mut u8;
           // println!(", returning addr=0x{:x}", block.start_addr());
       } else {
           // println!(", *** out of memory ***");
           ret_ptr = ptr::null_mut(); // out of memory
       }
       ret_ptr
   }
    
   pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
   // println!("   dealloc: size={}, align={}; not supported", layout.size(), layout.align());

      let (size, _) = LinkedListAllocator::size_align(layout);
      self.add_free_block(ptr as usize, size)
   }

}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }
    
}
