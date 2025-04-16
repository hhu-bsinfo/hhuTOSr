use kernel::allocator;
use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;


fn aufgabe2() {
   heap_demo::run();
   sound_demo::run();
}

#[no_mangle]
pub extern fn startup() {
    kprintln!("Welcome to hhuTOS!");

    // Speicherverwaltung initialisieren
    allocator::init();

    aufgabe2();
    
    loop {}
}

