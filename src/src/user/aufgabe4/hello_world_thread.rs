
use alloc::{boxed::Box};
use crate::devices::cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe4::coop_thread_loop;


pub struct HelloWorldThread {
}

impl HelloWorldThread {
  pub fn get_raw_pointer (&mut self) -> *mut HelloWorldThread {
	   self
   }
}

impl thread::ThreadEntry for HelloWorldThread {
    
    fn run(&mut self, thread_object: *mut thread::Thread) {
        println!("Hallo Welt von einem Thread!");
        scheduler::Scheduler::exit(thread_object);  
	}
}

pub fn init() {
    // Anwendung im Scheduler anmelden
    let hw_thread = Box::new(HelloWorldThread { } );
 	scheduler::Scheduler::ready(hw_thread);  
}
