#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(const_mut_refs)]
#![no_std]
#![allow(dead_code)]          // avoid warnings 
#![allow(unused_variables)]   // avoid warnings 
#![feature(alloc_error_handler)] 
#![allow(unused_imports)]
#![feature(restricted_std)]

extern crate spin; // we need a mutex in devices::cga_print
extern crate alloc;  // this is for heap allocations
extern crate std;    // needed for threads


// insert other modules
#[macro_use]   // import macros, too
mod devices;
mod kernel;
mod lib;
mod user;
mod consts;

use alloc::{boxed::Box};


use devices::cga as cga;  // shortcut for cga
use devices::cga_print;   // used to import code needed by println! 
use devices::pcspk as pcspk;   // shortcut for cga
use devices::keyboard as keyboard;  // shortcut for keyboard
use devices::pit;        // shortcut for pit

use kernel::allocator;
use kernel::cpu;
use kernel::interrupts;
use kernel::threads::thread;
use kernel::threads::scheduler;

use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;
use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;
use user::aufgabe3::keyboard_irq_demo;
use user::aufgabe4::corouts_demo;
use user::aufgabe4::coop_thread_demo;
use user::aufgabe4::hello_world_thread;
use user::aufgabe5::preempt_demo;



fn aufgabe1() {
   cga::clear();
   text_demo::run();
   keyboard_demo::run();
}


fn aufgabe2() {
   cga::clear();
   heap_demo::run();
   sound_demo::run();
}


fn aufgabe3() {
   cga::clear();
   keyboard_irq_demo::run();
}


fn aufgabe4() {
//   corouts_demo::run();

    // Anwendung im Scheduler anmelden
    coop_thread_demo::init();
	
    // Scheduler starten
	scheduler::Scheduler::schedule();
}


fn aufgabe5() {

    // Anwendung im Scheduler anmelden
    preempt_demo::init();

    // Scheduler starten
    scheduler::Scheduler::schedule ();
}


#[no_mangle]
pub extern fn startup() {

    // Speicherverwaltung initialisieren
    allocator::init();

    // Interrupt-Strukturen initialisieren
    interrupts::init();

    // Scheduler initialisieren (benoetigt einen Allokator)
    scheduler::init();
	
    // Zeitgeber starten
    pit::plugin ();

    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    keyboard::plugin ();

    // Interrupts erlauben 
    cpu::enable_int ();

	//cga::clear();
    println!("HHUos 0.5"); 
	


//    aufgabe1();
//    aufgabe2();
//    aufgabe3();
//    aufgabe4();
    aufgabe5();
    
    loop{}
}

