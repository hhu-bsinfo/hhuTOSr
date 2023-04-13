/*****************************************************************************
 *                                                                           *
 *                         i n t d i s p a t c h e r                         *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Zentrale Unterbrechungsbehandlungsroutine des Systems.   *
 *                  Der Parameter gibt die Nummer des aufgetretenen          *
 *                  Interrupts an. Wenn eine Interrupt Service Routine       *
 *                  registriert ist, wird diese aufgerufen.                  *
 *                                                                           *
 * Autor:           Michael Schoettner, 07.03.2022                           *
 *****************************************************************************/
extern crate spin;

use spin::Mutex;
use alloc::{boxed::Box, vec::Vec};
use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::interrupts::isr;


pub const INT_VEC_TIMER:usize = 32;
pub const INT_VEC_KEYBOARD:usize = 33;


        
/*****************************************************************************
 * Funktion:        int_disp                                                 *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Low-Level Interrupt-Behandlung.                          *
 *                  Diese Funktion ist in der IDT fuer alle Eintraege einge- *
 *                  tragen. Dies geschieht bereits im Bootloader.            *
 *                  Sie wird also fuer alle Interrupts aufgerufen. Von hier  *
 *                  aus sollen die passenden ISR-Routinen der Treiber-Objekte*
 *                  aufgerufen werden.                                       *
 * Parameter:                                                                *
 *      vector:     Vektor-Nummer der Unterbrechung                          *
 *****************************************************************************/
#[no_mangle]
pub extern "C" fn int_disp(vector: u32) {
    if report(vector as usize) == false {
		cga::print_str("Panic: unexpected interrupt ", cga::CGA_STD_ATTR);
		cga::print_dec(vector);
	    cga::print_str(" - processor halted.", cga::CGA_STD_ATTR);
	    cpu::halt ();
	} 
}



const MAX_VEC_NUM: usize = 256;

static INT_VECTORS: Mutex<IntVectors> = Mutex::new( IntVectors{ map: Vec::new(), } );

// Interrupt vector map
struct IntVectors {
    map: Vec<Box<dyn isr::ISR>>,
}

// required by the compiler for gloabl 'INT_DISPATCHER'
unsafe impl Send for IntVectors {}
unsafe impl Sync for IntVectors {}


/*****************************************************************************
 * Funktion:        init                                                     *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Initialisierung der ISR map mit MAX_VEC_NUM Default-ISRs.*
 *                  Dies erlaubt später das Einfuegen einer ISR in 'assign'  *
 *                  an einer bestimmten Stelle.                              *
 *****************************************************************************/
pub fn init() {
	let mut vectors = INT_VECTORS.lock();
	
	for _ in 0..MAX_VEC_NUM {
          vectors.map.push(Box::new(isr::Default));
       }
}


/*****************************************************************************
 * Funktion:        assign                                                   *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Registrierung einer ISR.                                 *
 *                                                                           *
 * Parameter:                                                                *
 *      vector:     Vektor-Nummer der Unterbrechung                          *
 *      isr:        ISR die registriert werden soll                          *
 *                                                                           *
 * Rueckgabewert:   0 = Erfolg, -1 = Fehler                                  *
 *****************************************************************************/
pub fn assign(vector: usize, callback: Box<dyn isr::ISR>) -> bool {
	
	/* Hier muss Code eingefuegt werden */
	
}


/*****************************************************************************
 * Funktion:        report                                                   *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Eingetragene ISR ermitteln.                              *
 *                                                                           *
 * Parameter:                                                                *
 *      vector:     Gesuchtes ISR-Objekt fuer diese Vektor-Nummer.           *
 *                                                                           *
 * Rueckgabewert:   0 = Fehler, ansonsten Zeiger auf ISR                     *
 *****************************************************************************/
pub fn report(vnum: usize) -> bool {

	/* Hier muss Code eingefuegt werden */

}
