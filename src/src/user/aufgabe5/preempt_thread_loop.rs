
use alloc::{boxed::Box};
use crate::devices::cga;         // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;


pub struct PreemptiveThreadLoop {
	cnt: u32,
}

impl PreemptiveThreadLoop {

   pub fn new()-> Box<PreemptiveThreadLoop> {
      Box::new(PreemptiveThreadLoop { cnt: 0, } )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut PreemptiveThreadLoop {
	   self
   }
}

impl thread::ThreadEntry for PreemptiveThreadLoop {
	
   fn run(&mut self, thread_object: *mut thread::Thread) {

        /* Hier muss Code eingefuegt werden */
        
   }
}
