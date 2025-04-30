/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pic                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: The PIC allows to enable or disable IRQs. This determines       ║
   ║         whether an interruption from a device is forwarded to the cpu   ║
   ║         at all. Even then, activation of the interrupt routine which is ║
   ║         registered in the IDT only occurs if the processor is ready to  ║
   ║         respond to interrupts. This depends on the Interrupt Enable IE  ║
   ║         bit in the RFLAGS register. This can be controlled using        ║
   ║         function in the 'cpu.rs' module.                                ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 7.3.2022                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
 */
use spin::Mutex;
use crate::kernel::cpu as cpu;
use crate::kernel::cpu::IoPort;

/// Global PIC instance, used for interrupt handling in the whole kernel.
pub static PIC: Mutex<Pic> = Mutex::new(Pic::new());

const PIC_COMMAND_1: u16 = 0x20; // Command register of PIC 1 (Master)
const PIC_COMMAND_2: u16 = 0xa0; // Command register of PIC 2 (Slave)
const PIC_DATA_1: u16 = 0x21; // Data register of PIC 1 (Master)
const PIC_DATA_2: u16 = 0xa1; // Data register of PIC 2 (Slave)

const PIC_COMMAND_INITIALIZE: u8 = 0x11; // Initialization command for PIC

#[repr(u8)]
/// Enumeration of all IRQs (Interrupt Request Lines).
pub enum Irq {
    Timer = 0x00,
    Keyboard = 0x01,
    Cascade = 0x02,
    Com2 = 0x03,
    Com1 = 0x04,
    Lpt2 = 0x05,
    Floppy = 0x06,
    Lpt1 = 0x07,
    Rtc = 0x08,
    Free1 = 0x09,
    Free2 = 0x0a,
    Free3 = 0x0b,
    Mouse = 0x0c,
    Fpu = 0x0d,
    PrimaryAta = 0x0e,
    SecondaryAta = 0x0f,
}

/// Representation of the Programmable Interrupt Controller (PIC).
/// The PIC is responsible for handling hardware interrupts and forwarding them to the CPU.
/// It actually consists of two chips (PIC 1 and PIC 2),
/// with PIC 2 being connected to interrupt line 2 of PIC 1.
pub struct Pic {
    command1: IoPort,
    command2: IoPort,
    data1: IoPort,
    data2: IoPort
}

impl Pic {
    /// Create a new PIC instance (needs to be initialized before use)
    pub const fn new() -> Self {
        Pic {
            command1: IoPort::new(PIC_COMMAND_1),
            command2: IoPort::new(PIC_COMMAND_2),
            data1: IoPort::new(PIC_DATA_1),
            data2: IoPort::new(PIC_DATA_2)
        }
    }

    /// Initialize the PIC.
    /// See the OSDev wiki for details: https://wiki.osdev.org/8259_PIC
    pub fn init(&mut self) {
        unsafe {
            // Start initialization sequence on both PICs (ICW1)
            self.command1.outb(PIC_COMMAND_INITIALIZE);
            cpu::io_wait();
            self.command2.outb(PIC_COMMAND_INITIALIZE);
            cpu::io_wait();

            // Set interrupt offsets (ICW2); Interrupts 0-31 are reserved for CPU exceptions
            self.data1.outb(32); // PIC 1 handles interrupts 32-39
            cpu::io_wait();
            self.data2.outb(40); // PIC 2 handles interrupts 40-47
            cpu::io_wait();

            // Setup cascading PICs (ICW3)
            self.data1.outb(0x04); // PIC 2 is connected to IRQ 2 of PIC 1
            cpu::io_wait();
            self.data2.outb(0x02); // Tell PIC 2 its cascade identity
            cpu::io_wait();

            // Enable 8086-mode and automatic EOI (ICW4)
            self.data1.outb(0x03); // Configure PIC 1 for 8086 mode with automatic EOI
            cpu::io_wait();
            self.data2.outb(0x03); // Configure PIC 2 for 8086 mode with automatic EOI
            cpu::io_wait();

            // Disable all interrupt lines
            self.data1.outb(0xfb); // Allow cascading interrupts
            cpu::io_wait();
            self.data2.outb(0xff); // Disable all interrupts
            cpu::io_wait();
        }
    }

    /// Enable an IRQ to be forwarded to the processor by the PIC.
    pub fn allow (&mut self, irq: Irq) {

        /* Hier muss Code eingefuegt werden */

    }

    /// Disable an IRQ to be forwarded to the processor by the PIC.
    pub fn forbid (&mut self, irq: Irq) {

        /* Hier muss Code eingefuegt werden */

    }

    /// Get the state (enabled/disabled) of an IRQ in the PIC.
    pub fn status (&mut self, irq: Irq) -> bool {

        /* Hier muss Code eingefuegt werden */

}
