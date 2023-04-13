use std::rc::{Rc};
use std::cell::RefCell;
use alloc::{boxed::Box};
use spin::Mutex;
use std::ptr;
use std::mem;

use crate::mylib::queue as queue;
use crate::kernel::threads::thread as thread;
use crate::devices::cga as cga;
use crate::kernel::cpu as cpu;


pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());


struct Dispatcher {
   	
}

pub struct Scheduler {
   active: *mut thread::Thread,
   ready_queue: queue::Queue<Box<thread::Thread>>,   // auf die CPU wartende Threads
   next_thread_id: u64,
}

// Notwendig, da sonst der Compiler 'SCHEDULER' als nicht akzeptiert
unsafe impl Send for Scheduler {}


impl Scheduler {
	
   // Scheduler mit Ready-Queue anlegen
   pub const fn new() -> Self {
   
      /* Hier muss Code eingefuegt werden */
      
   }


   // ID fuer neuen Thread zurueckgeben
   pub fn get_next_tid (&mut self) -> u64 {

      /* Hier muss Code eingefuegt werden */

   }
   
   
   // Scheduler starten -> 1. Thread wird angestossen
   pub fn schedule () {
      let to_start = SCHEDULER.lock().ready_queue.dequeue();
      if let Some(that) = to_start {
		 // Mit dem naechsten Aufruf uebernehmen wir das Memory-Mgmt.
		 // fuer 'that', ansonsten wuerde dies spaeter beim Umschalten
		 // zu frueh geloescht. Warum ist unklar. Durch das Speichern
		 // der Raw-Pointers muessen wir spaeter manuell 'drop' aufrufen
		 // Wir machen das in 'exitÄ'
		 let raw = Box::into_raw(that);
	
		 SCHEDULER.lock().active = raw;
	     thread::Thread::start( raw);
	  }
	  else {
 		cga::print_str("Panic: no thread, cannot start scheduler", cga::CGA_STD_ATTR);
		cpu::halt ();
	  }
   }
       
       
   // Einen neuen Thread beim Scheduler registrieren
   // Zurueckgegeben wird die TID des neuen Threads
   pub fn ready (that: Box<dyn thread::ThreadEntry> ) -> u64 {
      let tid = SCHEDULER.lock().get_next_tid();
      let thread_wrapper = thread::Thread::new(tid, that);
      SCHEDULER.lock().ready_queue.enqueue( thread_wrapper );
      tid
   }
    
    
   // Thread terminiert sich selbst
   pub fn exit (that: *mut thread::Thread) {
	  // Naechsten Thread aus der Ready-Queue holen
      let next = SCHEDULER.lock().ready_queue.dequeue();
      
      // Falls kein weiterer Thread wartet, abbrechen
      if next.is_none() {
         cga::print_str("Panic: cannot exit thread", cga::CGA_STD_ATTR);
         cpu::halt ();
	  }

      // Speicher des Aufrufers freigeben, siehe Beschreibung in 
      // 'schedule'	   
      unsafe {
         drop(Box::from_raw(that));
	  }
	  
      // Umschalten
      if let Some(nx) = next {
          // Aufruf verhindert, dass 'nx' geloescht wird, siehe auch
          // 'schedule'
		  let raw = Box::into_raw(nx);
          thread::Thread::switch( that, raw );
      }
   }


   // Thread mit 'Gewalt' terminieren
   pub fn kill (tokill_tid: u64) {

      /* Hier muss Code eingefuegt werden */

   }

   // CPU freiwillig abgeben und Auswahl des naechsten Threads
   pub fn yield_cpu (that: *mut thread::Thread) {

      /* Hier muss Code eingefuegt werden */

   }

}


// Dummy, wird zum Loeschen eines Threads benoetigt
// Siehe Queue::remove
struct Dummy { }

impl thread::ThreadEntry for Dummy {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
	}
}

