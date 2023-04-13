/*****************************************************************************
 *                                                                           *
 *                          c o r o u t i n e                                *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Implementierung eines Koroutinen-Konzepts. Die Koroutinen*
 *                  sind miteinander verkettet.                              *
 *                                                                           *
 *                  In 'init' wird der initiale Kontext der Koroutine ein-   *
 *                  erichtet. Mit 'start' wird ein Koroutine erstmalig       *
 *                  aktiviert. Das Umschalten auf die naechste Koroutine     * 
 *                  erfolgt durch den Aufruf von 'switch_to_next'.           *
 *                                                                           *
 *                  Bei einem Koroutinenwechsel wird der aktuelle Kontext    *
 *                  auf dem Stack gesichert. Die Adresse des zuletzt ge-     *
 *                  nutzten Stackeintrags wird in 'context' gespeichert.     * 
 *                                                                           *
 * Autor:           Michael Schoettner, 14.03.2023                           *
 *****************************************************************************/


use alloc::{boxed::Box, rc::Rc};
use core::ptr;
use core::ffi::c_void;

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::corouts::stack;


extern "C" { 
    fn _coroutine_start  (context: *mut c_void); 
    fn _coroutine_switch (context_now: *mut c_void, context_then: *mut c_void);
}


// Definition of Coroutine interface
pub trait CoroutineEntry {
    // Einstiegsfunktion der Koroutine
    fn run(&mut self, object: *mut Coroutine);
}

// 'coroutine.asm' greift auf 'context' mit Offset 4 zu
// Falls Variablen vor 'context' eingefuegt werden, so 
// muss der Assembler-Code angepasst werden
#[repr(C)]
pub struct Coroutine {
   cid: u64,
   context: u64, // Stack-Zeiger auf gesicherten Kontext
   stack: stack::Stack,
   entry: Box<dyn CoroutineEntry>,
   next: *mut Coroutine,
}

impl Coroutine {
	
   // Koroutine anlegen (mit vorbereitetem Stack)
   pub fn new(mycid: u64, myentry: Box<dyn CoroutineEntry>)-> Box<Coroutine> {
      let mystack  = stack::Stack::new(4096);
      
      let mut corout = Box::new( Coroutine{ cid: mycid, 
		                                context: 0, 
		                                  stack: mystack, 
		                                  entry: myentry,
		                                   next: ptr::null_mut(),
		                                  } );
		                                
      corout.coroutine_state_init();
      
      corout
   }
	
   pub fn start (c: *mut Coroutine) { 

      /* Hier muss Code eingefuegt werden */

   }
  
   pub fn switch_to_next (now: *mut Coroutine) {
   
      /* Hier muss Code eingefuegt werden */

   }

   pub fn get_raw_pointer (&mut self) -> *mut Coroutine {
	   self
   }
   
   pub fn get_cid (object: *const Coroutine) -> u64 {
      unsafe{ (*object).cid }
   }

   pub fn set_next(&mut self, nxt: *mut Coroutine) {

      /* Hier muss Code eingefuegt werden */

   }
   
   fn coroutine_state_init (&mut self) {
	   let faddr = kickoff as *const ();
       let object: *const Coroutine = self;
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
	
#[no_mangle]
pub extern "C" fn kickoff(object: *mut Coroutine) {
   unsafe {
/*
  	   println!("kickoff, object={:x}", object as u64);
       println!("kickoff, tid={:x}", (*object).cid);
       loop {}
*/
      (*object).entry.run(object);
   }
}
