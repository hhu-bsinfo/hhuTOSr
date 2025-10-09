/*
 * Module: intdispatcher
 *
 * Description: Handle dispatching of interrupts to their respective
 *              Interrupt Service Routines (ISRs).
 *              The main function is `int_disp()` which is called for any interrupt
 *              and calls a registered ISR of a device driver (e.g., the keyboard).
 *
 *              `int_disp()` is called from `idt.asm` where the x86 interrupt table
 *              is set up and the low-level interrupt handling is done.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 7.3.2022
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 31.7.2025
 */

use crate::kernel::interrupts::InterruptStackFrame;
use alloc::{boxed::Box, vec::Vec};
use crate::kernel::cpu;
use crate::kernel::interrupts::idt::IDT_SIZE;
use crate::kernel::interrupts::isr::ISR;
use spin::mutex::Mutex;

/// Enumeration of all standardized interrupt vectors.
pub enum InterruptVector {
    // CPU exceptions
    DivisionByZero = 0,
    Debug = 1,
    NonMaskableInterrupt = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRangeExceeded = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverrun = 9,
    InvalidTaskStateSegment = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtectionFault = 13,
    PageFault = 14,
    X87FloatingPointException = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SimdFloatingPointException = 19,
    VirtualizationException = 20,
    ControlProtectionException = 21,
    HypervisorInjectionException = 28,
    VmmCommunicationException = 29,
    SecurityException = 30,

    // Hardware interrupts
    Pit = 0x20,
    Keyboard = 0x21,
    Cascade = 0x22,
    Com2 = 0x23,
    Com1 = 0x24,
    Lpt2 = 0x25,
    Floppy = 0x26,
    Lpt1 = 0x27,
    Rtc = 0x28,
    Free1 = 0x29,
    Free2 = 0x2a,
    Free3 = 0x2b,
    Mouse = 0x2c,
    Fpu = 0x2d,
    PrimaryAta = 0x2e,
    SecondaryAta = 0x2f,
}

/// Global instance of the interrupt vector map.
pub static INT_VECTORS: Mutex<IntVectors> = Mutex::new(IntVectors::new());

/// The main interrupt dispatcher.
/// Every interrupt is routed here, if not specified otherwise in the IDT.
pub fn int_disp(vector: u8, stack_frame: InterruptStackFrame, error_code: Option<u64>) {
    if INT_VECTORS.is_locked() {
        panic!("Interrupt dispatcher is locked -> Cannot handle interrupt!");
    }

    let mut vectors = INT_VECTORS.lock();
    if !vectors.report(vector) {
        panic!("Unhandled interrupt {}!\nError Code: {:x?}\nStack Frame: {:#x?}", vector, error_code, stack_frame);
    }
}

/// The Interrupt vector map. Each ISR is registered in this map.
pub struct IntVectors {
    // Each ISR is wrapped in a Box, because the size of the ISRs is not known at compile time.
    map: Vec<Option<Box<dyn ISR>>>,
}

// Tell the compiler that IntVectors is safe to be shared between threads.
// This is ok, since we use a Mutex to protect the map.
unsafe impl Send for IntVectors {}
unsafe impl Sync for IntVectors {}

impl IntVectors {
    /// Create a new empty ISR map. init() must be called before using the map.
    pub const fn new() -> Self {
        IntVectors { map: Vec::new() }
    }

    /// Fill the ISR map with IDT_SIZE empty Options.
    /// Specific ISRs can be overwritten by calling `register()`.
    pub fn init(&mut self) {
        if !self.map.is_empty() {
            panic!("ISR map is already initialized!");
        }

        for _ in 0..IDT_SIZE {
            self.map.push(None);
        }
    }

    /// Register an ISR.
    /// Interrupts get disabled while registering the ISR to avoid race conditions with int_disp().
    pub fn register(&mut self, vector: InterruptVector, isr: Box<dyn ISR>) {
        let vector = vector as usize;
        let interrupts_enabled = cpu::disable_int_nested();

        // Check if the ISR is already registered
        if self.map[vector].is_some() {
            panic!("ISR already registered for vector {}", vector);
        }

        // Register the ISR
        self.map[vector] = Some(isr);
        cpu::enable_int_nested(interrupts_enabled);
    }

    /// Check if an ISR is registered for `vector`. If so, call it.
    pub fn report(&mut self, vector: u8) -> bool {
        let vector = vector as usize;

        match self.map[vector] {
            Some(ref isr) => {
                isr.trigger(); // Call the ISR
                true
            }
            None => {
                false // No ISR registered for this vector
            }
        }
    }
}