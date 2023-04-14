/*****************************************************************************
 *                                                                           *
 *                             t h r e a d                                   *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Implementierung eines kooperativen Thread-Konzepts.      *
 *                  Thread-Structs werden vom Scheduler in einer verketteten *
 *                  Liste 'readylist' verwaltet.                             *
 *                                                                           *
 *                  In 'init' wird der initialie Kontext des Threads ein-    * 
 *                  gerichtet. Mit 'start' wird ein Thread aktiviert. Die    *
 *                  CPU sollte mit 'yield' (im Scheduler) freiwillig abgeg-  *
 *                  eben werden. Um bei einem Threadwechsel den Kontext      *
 * 	                sichern zu koennen, enthaelt jedes Threadobjekt eine     * 
 *                  Struktur 'ThreadState', in dem die Werte der nicht-      *
 *                  fluechtigen Register gesichert werden koennen.           *
 *                                                                           *
 * Autor:           Michael Schoettner, 26.04.2022                           *
 *****************************************************************************/

use std::rc::{Rc};
use std::cell::RefCell;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Display;
use alloc::{boxed::Box};

use crate::consts;
use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::threads::stack;
use crate::mylib::queue::Link;


extern "C" { 
	fn _thread_start  (tp:  *mut c_void); 
    fn _thread_switch (now: *mut c_void, then: *mut c_void);
}


// Definition of thread interface
pub trait ThreadEntry {
	
    // Einstiegsfunktion des Threads
    fn run(&mut self, thread_object: *mut Thread);

}

#[repr(C)]
pub struct Thread {
   tid: u64,
   context: u64,
   stack: stack::Stack,
   entry: Box<dyn ThreadEntry>,
}

pub type RBT = Rc<RefCell<Box<Thread>>>;


impl Thread {

   // Koroutine anlegen (mit vorbereitetem Stack)
   pub fn new(mytid: u64, myentry: Box<dyn ThreadEntry>)-> Box<Thread> {
      let mystack = stack::Stack::new(consts::STACK_SIZE);
      let mut threadobj =  Box::new( Thread{ tid: mytid, 
		                                 context: 0, 
		                                   stack: mystack, 
		                                   entry: myentry,
		                                   }
		                           );
		                          
      threadobj.thread_state_init();
      
      threadobj
   }

   pub fn start (that: *mut Thread) { 
      unsafe {
  	     _thread_start  (that as *mut c_void); 
      }
   }
   
   pub fn switch (that: *mut Thread, next: *mut Thread) { 
      unsafe {
  	     _thread_switch  (that as *mut c_void,
  	                      next as *mut c_void,
  	                     ); 
      }
   }
   
   pub fn get_tid (thread_object: *const Thread) -> u64 {
      unsafe{ (*thread_object).tid }
   }
   
   pub fn get_raw_pointer (&mut self) -> *mut Thread {
	   self
   }

   fn thread_state_init (&mut self) {
	   let faddr = kickoff_thread as *const ();
       let object: *const Thread = self;
       let sp: *mut u64 = self.stack.get_data();
       
 	
       // Stack initialisieren. Es soll so aussehen, als waere soeben die
       // eine Funktion aufgerufen worden, die als Parameter den Zeiger
       // "object" erhalten hat.
       // Da der Funktionsaufruf simuliert wird, kann fuer die Ruecksprung-
       // adresse nur ein unsinniger Wert eingetragen werden. Die aufgerufene
       // Funktion muss daher dafuer sorgen, dass diese Adresse nie benoetigt
       // wird, sie darf also nicht terminieren, sonst kracht's.
       unsafe {
          *sp = 0x131155 as u64; // Ruecksprungadresse
       }
	   
       // Nun legen wir noch die Adresse der Funktion "kickoff" ganz oben auf
       // den Stack. Wenn dann bei der ersten Aktivierung dieser Koroutine der
       // Stackpointer so initialisiert wird, dass er auf diesen Eintrag
       // verweist, genuegt ein ret, um die Funktion kickoff zu starten.
       // Genauso sollen auch alle spaeteren Threadwechsel ablaufen.
       unsafe {
          *sp.offset(-1) = faddr as u64;   // Adresse
      
          // Nun sichern wir noch alle relevanten Register auf dem Stack
          *sp.offset(-2)  = 0;   // r8
          *sp.offset(-3)  = 0;   // r9
          *sp.offset(-4)  = 0;   // r10
          *sp.offset(-5)  = 0;   // r11
          *sp.offset(-6)  = 0;   // r12
          *sp.offset(-7)  = 0;   // r13
          *sp.offset(-8)  = 0;   // r14
          *sp.offset(-9)  = 0;   // r15
       
          *sp.offset(-10) = 0;   // rax
          *sp.offset(-11) = 0;   // rbx
          *sp.offset(-12) = 0;   // rcx
          *sp.offset(-13) = 0;   // rdx
       
          *sp.offset(-14) = 0;   // rsi
          *sp.offset(-15) = object as u64;  // rdi -> 1. Param. fuer 'kickoff'
          *sp.offset(-16) = 0;   // rbp
          *sp.offset(-17) = cpu::getflags(); // flags

          // Zum Schluss speichern wir den Zeiger auf den zuletzt belegten
          // Eintrag auf dem Stack in 'context'. Daruber gelangen wir in 
          // Coroutine_start an die noetigen Register     
          self.context = (sp as u64) - (8*17); // aktuellen Stack-Zeiger speichern
       }
     
/*
      println!("Prepared Stack: top-address = {:x}", self.stack.get_data() as u64);
      unsafe {
         println!("  {:x}: {:x}  // dummy raddr", sp as u64, *(sp) as u64);
         println!("  {:x}: {:x}  // *object", sp.offset(-15) as u64, *(sp.offset(-15)) as u64);
         println!("  {:x}: {:x}  // kickoff", sp.offset(-1) as u64, *(sp.offset(-1)) as u64);
         println!("  {:x}: last used ", sp.offset(-17) as u64);
         println!("");
         println!("  self.context = {:x}  // context", self.context);
      }
      loop {}
*/
   }

}


// Notwendig, für die Queue-Implementierung im Scheduler
impl PartialEq for Thread {
   fn eq(&self, other: &Self) -> bool {
	   self.tid == other.tid 
   }
}

// Notwendig, falls wir die Ready-Queue ausgeben moechten
impl fmt::Display for Thread {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	   write!(f, "{}", self.tid)
   }
}


#[no_mangle]
pub extern "C" fn kickoff_thread(object: *mut Thread) {
   // cga::print_str("kickoff", cga::CGA_STD_ATTR);
	unsafe{
//	   println!("kickoff, object={:x}", object as u32);
//	   println!("kickoff, tid={:x}", (*object).tid);
       (*object).entry.run(object);
       
       // (*object).entry.run(object); kehrt hoffentlich nie hierher zurueck
       loop {}  
    }
}
