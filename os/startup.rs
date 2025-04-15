
use kernel::allocator;

use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;


fn aufgabe2() {
   heap_demo::run();
   sound_demo::run();
}

#[no_mangle]
pub extern fn startup() {
    kprintln!("OS *** is running ...");

    // Speicherverwaltung initialisieren
    allocator::init();

    // Speicherverwaltung initialisieren
   /* Hier muss Code eingefuegt werden */

    aufgabe2();
    
    loop{}
}

