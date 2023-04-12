/*****************************************************************************
 *                                                                           *
 *                                  p i c                                    *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Mit Hilfe des PICs koennen Hardware-Interrupts (IRQs)    *
 *                  einzeln zugelassen oder unterdrueckt werden. Auf diese   *
 *                  Weise wird also bestimmt, ob die Unterbrechung eines     *
 *                  Geraetes ueberhaupt an den Prozessor weitergegeben wird. *
 *                  Selbst dann erfolgt eine Aktivierung der Unterbrechungs- *
 *                  routine nur, wenn der Prozessor bereit ist, auf Unter-   *
 *                  brechungen zu reagieren. Dies kann mit Hilfe der Klasse  *
 *                  CPU festgelegt werden.                                   *
 *                                                                           *
 * Autor:           Michael Schöttner, Universitaet Duesseldorf, 7.3.2022    *
 *****************************************************************************/

use crate::kernel::cpu;


// IRQ-Nummern von Geraeten
pub const IRQ_TIMER: u32    = 0;     // Programmable Interrupt Timer (PIT)
pub const IRQ_KEYBOARD: u32 = 1;     // Tastatur


const PIC_IMR1: u16   = 0x21;    // interrupt mask register von PIC 1
const PIC_IMR2: u16   = 0xa1;    // interrupt mask register von PIC 2


/*****************************************************************************
 * Funktion:        allow                                                    *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Sorgt dafuer, dass der uebergebene IRQ ab sofort durch   *
 *                  den PIC an den Prozessor weitergereicht wird. Um eine    * 
 *                  Unterbrechungsbehandlung zu ermoeglichen, muss           *
 *                  zusaetzlich CPU::enable_int() aufgerufen werden.         *
 *                                                                           *
 * Parameter:                                                                *
 *      irq:        IRQ der erlaubt werden soll                              *
 *****************************************************************************/
pub fn allow (irq: u32) {

 /* Hier muss Code eingefuegt werden */

}


/*****************************************************************************
 * Funktion:        forbid                                                   *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Unterdrueckt mit Hilfe des PICs einen bestimmten IRQ.    *
 *                                                                           *
 * Parameter:                                                                *
 *      irq:        IRQ der maskiert werden soll                             *
 *****************************************************************************/
pub fn forbid (irq: u32) {

 /* Hier muss Code eingefuegt werden */

}


/*****************************************************************************
 * Funktion:        status                                                   *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Liefert den aktuellen Zustand des Maskierbits eines      *
 *                  bestimmten IRQs.                                         *
 *                                                                           *
 * Parameter:                                                                *
 *      irq:        IRQ dessen Status erfragt werden soll                    *
 *                                                                           *
 * Rückgabewert:    true = bit is gesetzt, false = bit ist nicht gesetzt     *
 *****************************************************************************/
pub fn status (irq: u32) -> bool {

 /* Hier muss Code eingefuegt werden */

}
 
