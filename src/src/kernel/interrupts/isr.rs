/*****************************************************************************
 *                                                                           *
 *                                    i s r                                  *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Definition der Schnittstelle für eine Interrupt-Service- *
 *                  Routine. Muss von einem Geraetetreiber implementiert,    * 
 *                  falls dieser Interrupts registrieren und verarbeiten     * 
 *                                                                           *
 * Autor:           Michael Schoettner, 10.03.2022                           *
 *****************************************************************************/


// Definition of Interrupt Service Routine
pub trait ISR {
	fn is_default_isr(&self) -> bool { return false; }
    fn trigger(&self);
}

// Default ISR needed by intdispatcher
#[derive(Copy, Clone)]
pub struct Default;

impl ISR for Default {
	fn is_default_isr(&self) -> bool { 
		return true; 
	}
	
    fn trigger(&self)  { 
	}
}


