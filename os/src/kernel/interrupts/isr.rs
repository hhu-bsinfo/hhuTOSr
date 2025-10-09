/*
 * Module: isr
 *
 * Description: Definition of the interface for an Interrupt Service Routine (ISR).
 *              Must be implemented by a device driver if it needs to handle
 *              interrupts. The ISR is registered using the `register()` function
 *              in `intdispatcher.rs`.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 10.3.2022
 */

/// The Interrupt Service Routine trait
pub trait ISR {
    fn trigger(&self);
}
