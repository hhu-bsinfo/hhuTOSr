/*****************************************************************************
 *                                                                           *
 *                                  k e y b o a r d                          *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Treiber für den Tastaturcontroller des PCs.              *
 *                                                                           *
 * Autor:           Michael Schoettner, HHU Duesseldorf, 18.1.2022           *
 *****************************************************************************/


use crate::kernel::interrupts::pic;
use crate::kernel::interrupts::isr;
use crate::kernel::interrupts::intdispatcher;



/*****************************************************************************
 * Implementierung: Keyboard                                                 *
 *****************************************************************************/
impl Keyboard {

   /*****************************************************************************
    * Funktion:        plugin                                                   *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Unterbrechungen fuer die Tastatur erlauben. Ab sofort    *
    *                  wird bei einem Tastendruck die Methode 'trigger'         *
    *                  aufgerufen.                                              *
    *****************************************************************************/
   pub fn plugin (&mut self) {

 	   /* Hier muss Code eingefuegt werden */

  }
   
}


/*****************************************************************************
 * Implementierung: ISR                                                      *
 *****************************************************************************/
struct KeyboardISR;
impl isr::ISR for KeyboardISR {

    /*****************************************************************************
     * Funktion:        trigger                                                  *
     *---------------------------------------------------------------------------*
     * Beschreibung:    ISR fuer die Tastatur. Wird aufgerufen, wenn die Tastatur*
     *                  eine Unterbrechung ausloest.                             *
     *****************************************************************************/
    fn trigger(&self)  {
        
 	   /* Hier muss Code eingefuegt werden */
 
    }
}
