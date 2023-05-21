
use alloc::{boxed::Box};
use crate::devices::cga;           // shortcut for cga
use crate::devices::cga_print;     // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe5::preempt_thread_loop;
use crate::user::aufgabe5::sound_thread;


pub struct PreemptiveThreadDemo {
}

impl PreemptiveThreadDemo {

   pub fn new()-> Box<PreemptiveThreadDemo> {
      Box::new(PreemptiveThreadDemo { cnt: 0, } )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut PreemptiveThreadDemo {
	   self
   }
}

impl thread::ThreadEntry for PreemptiveThreadDemo {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
       // Den Sound-Thread im Scheduler anmelden


       // Den Loop-Thread im Scheduler anmelden

	}

}


pub fn init() {
   // Anwendung im Scheduler anmelden
   let pd = Box::new(PreemptiveThreadDemo { cnt:0, } );
   scheduler::Scheduler::ready( pd );  
}
