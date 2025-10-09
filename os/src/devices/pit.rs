/*
 * Module: pit
 *
 * Description: Driver for the Programmable Interval Timer (PIT).
 *              The PIT provides a timer interrupt that can be used for scheduling and timing.
 *              This module offers functions to read the time since boot in milliseconds,
 *              and wait for a specified number of milliseconds.
 *
 * Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 15.6.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 07.08.2025
 */

use alloc::boxed::Box;
use core::sync::atomic::AtomicUsize;
use spin::Once;
use crate::devices::cga::{Color, CGA, CGA_COLUMNS};
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
const NANOSECONDS_PER_TICK: usize = 1_000_000_000 / TIMER_FREQ; // Nanoseconds per tick

static TIMER: Once<Timer> = Once::new();
static SYSTEM_TIME: AtomicUsize = AtomicUsize::new(0);
static SPINNER_CHARS: &[char] = &['|', '/', '-', '\\'];

/// Get the current system time in milliseconds.
pub fn get_system_time() -> usize {
    SYSTEM_TIME.load(core::sync::atomic::Ordering::Relaxed)
}

/// Wait for a specified number of milliseconds using the system time.
pub fn wait(ms: usize) {
    let start_time = get_system_time();
    while get_system_time() - start_time < ms {
        get_scheduler().yield_cpu();
    }
}

/*
 * Interrupt service routine implementation.
 */

/// Register the timer interrupt handler.
pub fn plugin() {
    let mut vectors = intdispatcher::INT_VECTORS.lock();
    let mut pic = pic::PIC.lock();

    // Initialize the timer with a 1 ms interval
    let interval_ms = 1;
    TIMER.call_once(|| {
        let mut timer = Timer::new();
        timer.set_interrupt_interval(interval_ms);

        timer
    });

    vectors.register(InterruptVector::Pit, Box::new(TimerISR { interval_ms }));
    pic.allow(Irq::Timer);
}

/// The timer interrupt service routine.
struct TimerISR {
    interval_ms: usize,
}

impl ISR for TimerISR {
    fn trigger(&self) {
        let time = SYSTEM_TIME.fetch_add(self.interval_ms, core::sync::atomic::Ordering::Relaxed);
        
        if !get_scheduler().is_locked() {
            if let Some(mut cga) = CGA.try_lock() {
                cga.show(CGA_COLUMNS - 1, 0, SPINNER_CHARS[(time / 250) % SPINNER_CHARS.len()], CGA::attribute(Color::Black, Color::Red, false));
            }
        }

        if time % 10 == 0 {
            unsafe { intdispatcher::INT_VECTORS.force_unlock(); }
            get_scheduler().yield_cpu();
        }
    }
}

/*
 * Implementation of the PIT driver itself.
 */

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
        let ticks = (interval_ms * 1000000) / NANOSECONDS_PER_TICK;
        if ticks > u16::MAX as usize {
            panic!("Timer interval too large");
        }

        unsafe {
            // Mode 3 (square wave generator), Low byte/High byte, Binary format
            self.control_port.outb(0x36);
            self.data_port0.outb((ticks & 0xff) as u8);
            self.data_port0.outb(((ticks & 0xff00) >> 8) as u8);
        }
    }
}
