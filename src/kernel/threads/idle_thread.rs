
use alloc::{boxed::Box};
use crate::devices::cga;
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;


pub struct IdleThread {
}

impl thread::ThreadEntry for IdleThread {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
       	scheduler::set_initialized();
       	loop {
		   scheduler::Scheduler::yield_cpu(thread_object);
		}
	}
	
}
