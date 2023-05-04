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

   /* Hier muss Code eingefuegt werden */
   
}

impl BumpAllocator {
    // Creates a new empty bump allocator.
    pub const fn new() -> Self {

        /* Hier muss Code eingefuegt werden */
        return BumpAllocator {  };
    }

    /*
     * Initializes the bump allocator with the given heap bounds.
     * 
     * This method is unsafe because the caller must ensure that the given
     *  memory range is unused. Also, this method must be called only once.
     */
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {

       /* Hier muss Code eingefuegt werden */

    }

    // Dump free list
    pub fn dump_free_list(&mut self) {

       /* Hier muss Code eingefuegt werden */
 		
	}

   pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {

        /* Hier muss Code eingefuegt werden */
        return ptr::null_mut();
   }
   
   pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
      println!("   dealloc: size={}, align={}; not supported", layout.size(), layout.align());
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
