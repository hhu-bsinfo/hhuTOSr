/*****************************************************************************
 *                                                                           *
 *                              p i t                                        *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Programmable Interval Timer.                             *
 *                                                                           *
 * Autor:           Michael Schoettner, HHU, 18.5.2023                       *
 *****************************************************************************/
#![allow(dead_code)]

use spin::Mutex;
use alloc::{boxed::Box};

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::interrupts::isr;
use crate::kernel::interrupts::pic;
use crate::kernel::interrupts::intdispatcher;
use crate::kernel::threads::scheduler;


// Ports
const PORT_CTRL:u16  = 0x43;
const PORT_DATA0:u16 = 0x40;


// Global PIT; fuer den Zugriff auf 'get_systime'
static mut TIME: Option<PIT> = None;				     

// Registrieren des Interrupt-Handlers
pub fn plugin() {
   unsafe {
      if TIME.is_none() {
	     PIT::init();
      }
      TIME.as_mut().unwrap().plugin();
  }	
 }

pub fn get_systime() -> u64 {
   unsafe {
      if TIME.is_none() {
	     return 0
	  }
      TIME.as_ref().unwrap().systime	
   }	
}


struct PIT { 
   systime: u64, 
   direction: u32,
}


/*****************************************************************************
 * Implementierung: PIT                                                      *
 *****************************************************************************/
impl PIT {
	
   /*****************************************************************************
    * Funktion:        init                                                     *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Alloziert globales Objekt TIME für den PIT.              *
    *****************************************************************************/
   fn init() {
		 
       /* Hier muss Code eingefuegt werden */

   }

   /*****************************************************************************
    * Funktion:        interval                                                 *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Zeitinervall programmieren.                              *
    *                                                                           *
    * Parameter:                                                                *
    *      us:         Zeitintervall in Mikrosekunden, nachdem periodisch ein   * 
    *                  Interrupt erzeugt werden soll.                           *
    *****************************************************************************/
   pub fn interval (us: u32) {

       /* Hier muss Code eingefuegt werden */

   }

   /*****************************************************************************
    * Funktion:        plugin                                                   *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Unterbrechungen fuer den PIT erlauben. Ab sofort wird    *
    *                  bei einem Timer-Interrupt die Funktion 'trigger'         *
    *                  aufgerufen.                                              *
    *****************************************************************************/
   pub fn plugin (&mut self) {

       /* Hier muss Code eingefuegt werden */

   }
   
}


/*****************************************************************************
 * Implementierung: ISR                                                      *
 *****************************************************************************/
struct PitISR;
impl isr::ISR for PitISR {

   /*****************************************************************************
    * Funktion:        trigger                                                  *
    *---------------------------------------------------------------------------*
    * Beschreibung:    ISR fuer den Zeitgeber. Wird aufgerufen, wenn der        * 
    *                  Zeitgeber eine Unterbrechung ausloest. Anzeige der Uhr   *
    *                  aktualisieren und Thread wechseln durch Setzen der       *
    *                  Variable 'threadSwitch', wird in 'int_disp' behandelt.   *
    *****************************************************************************/
   fn trigger(&self)  {

       /* Hier muss Code eingefuegt werden */
      
   }
}
