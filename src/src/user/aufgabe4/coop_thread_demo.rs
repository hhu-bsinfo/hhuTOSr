
use alloc::{boxed::Box};
use crate::devices::cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe4::coop_thread_loop;


pub struct CoopThreadDemo {
}

impl CoopThreadDemo {
  pub fn get_raw_pointer (&mut self) -> *mut CoopThreadDemo {
	   self
   }
}

impl thread::ThreadEntry for CoopThreadDemo {
    
    fn run(&mut self, thread_object: *mut thread::Thread) {

       /* Hier muss Code eingefuegt werden */

       // Die Loop-Threads im Scheduler anmelden

       // Eine Loop stoppen
       
	}
}

pub fn init() {

   /* Hier muss Code eingefuegt werden */
   // Anwendung im Scheduler anmelden

}
