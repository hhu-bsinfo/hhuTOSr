use user::aufgabe5::preempt_demo;


fn aufgabe5() {
   // Idle-Thread anlegen

   // Anwendung im Scheduler anmelden

   // Scheduler starten
}


#[no_mangle]
pub extern fn startup() {

    // Speicherverwaltung initialisieren

    // Interrupt-Strukturen initialisieren
	
    // Tastatur-Unterbrechungsroutine 'einstoepseln'

    // Zeitgeber-Unterbrechungsroutine 'einstoepseln'

    // Interrupts erlauben (Tastatur)


    aufgabe5();

    loop{}
}
