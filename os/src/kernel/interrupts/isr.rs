/*   ╔═════════════════════════════════════════════════════════════════════════╗
 *   ║ Module: isr                                                             ║
 *   ╟─────────────────────────────────────────────────────────────────────────╢
 *   ║ Descr.: Definition of the interface for an Interrupt Service Routine.   ║
 *   ║         Must be implemented by a device driver if it needs to handle    ║
 *   ║         interrupts. The ISR is registered using 'register' in           ║
 *   ║         'intdispatchter.rs'.                                            ║
 *   ╟─────────────────────────────────────────────────────────────────────────╢
 *   ║ Author: Michael Schoetter, Univ. Duesseldorf, 10.3.2022                 ║
 *   ╚═════════════════════════════════════════════════════════════════════════╝
 */

/// The Interrupt Service Routine trait
pub trait ISR {
    fn trigger(&self);
}
