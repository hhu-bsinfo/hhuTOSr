
use alloc::{boxed::Box};
use crate::devices::cga;         // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::corouts::coroutine;


struct Loop {
	cnt: u32,
}


impl coroutine::CoroutineEntry for Loop {
    fn run(&mut self, object: *mut coroutine::Coroutine) {

       /* Hier muss Code eingefuegt werden */
       
	}
}

pub fn run() {

   /* Hier muss Code eingefuegt werden
    * 
    * Die 3 Koroutinen einrichten, verketten und die 1. starten
    *
    */
}
