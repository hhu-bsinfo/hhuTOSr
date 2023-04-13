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


const NO_KEY: u8 = 0;    // used to init 'lastkey'

// Public functions for convenient access 


// Liefert 'lastkey' zuruecl
pub fn get_lastkey() -> u8 {
	// Interrupt greift auch auf gleiches Lock zu, daher sperren
	// Sonst kann ein Deadlock entstehen
	cpu::disable_int();
	let lk = KB.lock().get_lastkey();
	cpu::enable_int();

    // Verzoegerung, falls jemand staendig get_lastkey aufruft
    // Sonst kommt kein Interrupt durch
	let mut cnt = 0;
	loop {
		if cnt == 100000 { break; }
		cnt = cnt + 1;
	}
	
	lk
}

pub fn plugin() {
	KB.lock().plugin();
}


/*****************************************************************************
 * Implementierung: Keyboard                                                 *
 *****************************************************************************/
impl Keyboard {

   /*****************************************************************************
    * Funktion:        get_lastkey                                              *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Liefert ASCII-Code des letzten Tastendrucks und setzt    *
    *                  'lastkey' zurueck.                                       *
    *                                                                           *
    * Rueckgabewert:   Inhalt von 'lastkey'                                     *
    *****************************************************************************/
	fn get_lastkey(&mut self) -> u8 {
		let ret = self.lastkey;
		self.lastkey = NO_KEY;
		ret
	}


   fn key_hit(&mut self) -> key::Key {
      let invalid: key::Key = Default::default();  // nicht explizit initialisierte Tasten sind ungueltig
      let mut control: u8;

      self.lastkey = NO_KEY;
          
      // warten bis ein Byte abholbereit ist
      loop {
         control = cpu::inb(KBD_CTRL_PORT);
         if (control & KBD_OUTB) != 0 {
			 break;
         }
      }     

      // Byte einlesen
      self.code = cpu::inb(KBD_DATA_PORT);
    
      // Auch eine evtl. angeschlossene PS/2 Maus liefert ihre Daten ueber den
      // Tastaturcontroller. In diesem Fall ist zur Kennzeichnung das AUXB-Bit
      // gesetzt.
      if (control & KBD_AUXB)==0 && self.key_decoded()==true {
         return self.gather;
      }
      
      return invalid;
   }
   
   
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
