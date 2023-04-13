// insert other modules
mod mylib;


#[no_mangle]
pub extern fn startup() {
	
    // Interrupt-Strukturen initialisieren
    interrupts::init();
    	
    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    /* Hier muss Code eingefuegt werden */

    // Interrupts an der CPU erlauben 
    /* Hier muss Code eingefuegt werden */


    loop{}
}

