
use alloc::{boxed::Box};
use crate::devices::pcspk;   // shortcut for cga
use crate::kernel::threads::thread;


pub struct SoundThread {
}

impl SoundThread {

   pub fn new()-> Box<SoundThread> {
      Box::new(SoundThread { } )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut SoundThread {
	   self
   }
}

impl thread::ThreadEntry for SoundThread {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {	

       /* Hier muss Code eingefuegt werden */

	}
}
