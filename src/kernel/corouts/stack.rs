
use alloc::alloc::{Layout};

use crate::kernel::cpu as cpu;
use crate::kernel::allocator as allocator;

const STACK_ALIGNMENT:usize = 8;


#[repr(C)]
pub struct Stack {
    data: *mut u8,
    size: usize
}

impl Stack {
    pub fn new(size: usize) -> Stack {
		
		// 32 bit alignment for stack
		let layout = unsafe{ Layout::from_size_align_unchecked(size, STACK_ALIGNMENT) };
		
		// alloc memory for stack and set ptr. to end of block - 8
        let data = ((allocator::alloc(layout) as u64) + (size as u64) - 8) as *mut u8;
        if data.is_null() {
            println!("Panic: failed in 'Stack::new'");
       	    cpu::halt ();
        }
        Stack{ data, size }
    }
    
    pub fn get_data(&self) -> *mut u64 {
		self.data as *mut u64
	}
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe{
		   let layout = Layout::from_size_align_unchecked(self.size, STACK_ALIGNMENT);
           allocator::dealloc(self.data, layout);
        }
    }
}

impl Default for Stack {
   fn default() -> Self {
      Self { data: 0 as *mut u8, size: 0 }
   }
}


