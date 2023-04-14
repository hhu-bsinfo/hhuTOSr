/*****************************************************************************
 *                                                                           *
 *                              p i t                                        *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Programmable Interval Timer.                             *
 *                                                                           *
 * Autor:           Michael Schoettner, HHU, 15.6.2022                       *
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


// Global thread-safe access to PIT systime
static TIME: Mutex<PIT> = Mutex::new( PIT{ systime:0, direction:0, } );				     

pub fn plugin() {
   TIME.lock().plugin();
}

pub fn get_systime() -> u64 {
   cpu::disable_int();
   let ret = TIME.lock().systime;
   cpu::enable_int();

   // Verzoegerung, falls staendig get_systime aufgerufen wird.
   // Sonst kommt kein Interrupt durch, warum? unklar
   let mut cnt = 0;
   loop {
      if cnt == 100000 { break; }
      cnt = cnt + 1;
   }
   ret
}

// nachstehende drei Funktionen werden nur in der ISR benoetigt
// 'get_systime_interal' ist notwendig, da 'get_systime' die 
// Interrupts sperrt und freigibt
fn get_systime_internal() -> u64 {
   TIME.lock().systime
}

fn set_systime_internal(val:u64) {
   TIME.lock().systime = val;
}

fn get_direction_internal() -> u32 {
   TIME.lock().direction
}

// wird nur in der ISR benoetigt
fn set_direction_internal(val: u32) {
   TIME.lock().direction = val;
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
        
      // Alle 100 Ticks den Uhrzeiger rechts oben in der Ecke etwas
      // weiter drehen. Bei einer Unterbrechungsfrequenz von 100 Herz
      // bewegt er sich dann etwa im Sekunden Rhythmus.

      /* Hier muss Code eingefuegt werden */
         	   
      // Bei jedem Tick einen Threadwechsel ausloesen.
      intdispatcher::force_unlock();    
      unsafe {
         // den Scheduler sicherheitshalber entsperren
	     scheduler::SCHEDULER.force_unlock();
      }
      scheduler::SCHEDULER.lock().preempt();
      
   }
}
