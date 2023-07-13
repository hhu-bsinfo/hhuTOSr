/*****************************************************************************
 *                                                                           *
 *                                 B U M P                                   *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Eine sehr einfache Heap-Verwaltung, welche freigegebenen *
 *                  Speicher nicht mehr nutzen kann.                         *
 *                                                                           *
 * Autor:           Philipp Oppermann                                        *
 *                  https://os.phil-opp.com/allocator-designs/               *
 *                  Modified by Michael Schoettner, 15.3.2022                *
 *****************************************************************************/

use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;
use std::u8;


pub struct BumpAllocator {
    start: usize,
    end: usize,
    next: usize,
}

impl BumpAllocator {
    // Creates a new empty bump allocator.
    pub const fn new() -> Self {
        return BumpAllocator {
            start: 0,
            end: 0,
            next: 0,
        };
    }

    /*
     * Initializes the bump allocator with the given heap bounds.
     * 
     * This method is unsafe because the caller must ensure that the given
     *  memory range is unused. Also, this method must be called only once.
     */
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.start = heap_start;
        self.end = heap_start + heap_size;
        self.next = heap_start;
    }

    // Dump free list
    pub fn dump_free_list(&mut self) {
        println!("\nHeap-Start:   {:#x}, Heap-End:  {:#x}", self.start, self.end);
        println!("Next-Address: {:#x}, free:      {:#x}", self.next, self.end - self.next);
	}

   pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if self.next + layout.size() > self.end {
            return ptr::null_mut()
        }
        let p = (self.next + layout.size()) as *mut u8;
        self.next += layout.size();
        return p;
   }
   
   pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
      //println!("   dealloc: size={}, align={}; not supported", layout.size(), layout.align());
   }

}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
	
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }

}
