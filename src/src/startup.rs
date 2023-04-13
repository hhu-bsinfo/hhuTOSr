
#![feature(alloc_error_handler)] 


extern crate alloc;  // this is for heap allocations



use kernel::allocator;

use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;


fn aufgabe2() {
   heap_demo::run();
   sound_demo::run();
}


#[no_mangle]
pub extern fn startup() {

    // Speicherverwaltung initialisieren
	
    aufgabe2();
          
    loop{}
}

