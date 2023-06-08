
use alloc::{boxed::Box};
use crate::devices::cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;


pub struct CoopThreadLoop {
	cnt: u64,
}

impl CoopThreadLoop {

   pub fn new()-> Box<CoopThreadLoop> {
      Box::new(CoopThreadLoop { cnt: 0, } )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut CoopThreadLoop {
	   self
   }
}

impl thread::ThreadEntry for CoopThreadLoop {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {

      /* Hier muss Code eingefuegt werden */

	}
}
