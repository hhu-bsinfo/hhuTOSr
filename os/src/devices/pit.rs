/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pit                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Programmable Interval Timer.                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 15.6.2023                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::boxed::Box;
use core::arch::asm;
use core::sync::atomic::AtomicUsize;
use spin::Once;
use crate::devices::cga;
use crate::devices::cga::{Color, CGA, CGA_COLUMNS, CGA_ROWS};
use crate::kernel::cpu;
use crate::kernel::cpu::IoPort;
use crate::kernel::interrupts::{intdispatcher, pic};
use crate::kernel::interrupts::intdispatcher::InterruptVector;
use crate::kernel::interrupts::isr::ISR;
use crate::kernel::interrupts::pic::Irq;
use crate::kernel::threads::scheduler::get_scheduler;

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;

const TIMER_FREQ: usize = 1193182; // Timer frequency in Hz
const NANOSECONDS_PER_TICK: usize = 1_000_000_000 / TIMER_FREQ; // Nanoseconds per timer tick

/// Global timer instance.
/// Not accessible from outside the module.
/// To get the current system time, use `get_system_time()`.
static TIMER: Once<Timer> = Once::new();

/// Global system time in milliseconds.
static SYSTEM_TIME: AtomicUsize = AtomicUsize::new(0);

/// Characters used for the spinner animation.
static SPINNER_CHARS: &[char] = &['|', '/', '-', '\\'];

/// Get the current system time in milliseconds.
pub fn get_system_time() -> usize {
    SYSTEM_TIME.load(core::sync::atomic::Ordering::Relaxed)
}

/// Wait for a specified number of milliseconds using the system time.
pub fn wait(ms: usize) {

    /* Hier muss Code eingefuegt werden */

}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Interrupt service routine implementation.                               ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

/// Register the timer interrupt handler.
pub fn plugin() {

    /* Hier muss Code eingefuegt werden */

}

/// The timer interrupt service routine.
struct TimerISR {
    /// The interval between timer interrupts in milliseconds.
    interval_ms: usize,
}

impl ISR for TimerISR {
    fn trigger(&self) {

        /* Hier muss Code eingefuegt werden */

    }
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Implementation of the PIT driver itself.                                ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

/// Represents the programmable interval timer.
struct Timer {
    control_port: IoPort,
    data_port0: IoPort
}

impl Timer {
    /// Create a new Timer instance.
    pub const fn new() -> Timer {
        Timer {
            control_port: IoPort::new(PORT_CTRL),
            data_port0: IoPort::new(PORT_DATA0)
        }
    }

    /// Set the timer interrupt interval in milliseconds.
    pub fn set_interrupt_interval(&mut self, interval_ms: usize) {

        /* Hier muss Code eingefuegt werden */

    }
}
