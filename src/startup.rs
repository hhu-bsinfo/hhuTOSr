#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(const_mut_refs)]
#![no_std]
#![allow(dead_code)]          // avoid warnings 
#![allow(unused_variables)]   // avoid warnings 
#![feature(alloc_error_handler)] 
#![allow(unused_imports)]
#![feature(restricted_std)]
#![feature(const_fn_trait_bound)]


extern crate spin; // we need a mutex in devices::cga_print
extern crate alloc;  // this is for heap allocations
extern crate std;    // needed for threads


#[no_mangle]
pub extern fn startup() {


	//cga::clear();
	
    // Interrupt-Strukturen initialisieren
    /* Hier muss Code eingefuegt werden */
    	
    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    /* Hier muss Code eingefuegt werden */

    // Interrupts erlauben (Tastatur)
    /* Hier muss Code eingefuegt werden */


    loop{}
}

