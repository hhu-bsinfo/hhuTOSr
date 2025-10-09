/*
 * Module: idt
 *
 * Description: Contains the Interrupt Descriptor Table (IDT) and its entries.
 *              The IDT stores 256 entries, each of which points to an interrupt handler.
 *              When an interrupt occurs, the CPU looks up the corresponding entry in the IDT
 *              and calls the handler function.
 *
 *              In hhuTOSr, all entries point to the `intdispatcher::int_disp()` function,
 *              which is responsible for dispatching the interrupt to a registered handler.
 *
 *              The IDT is loaded into the CPU using the `lidt` instruction, which is wrapped
 *              in the `load()` function of the `Idt` struct, for convenience.
 *
 * Author: Fabian Ruhland, Heinrich Heine University Duesseldorf, 31.07.2025
 */

use core::arch::asm;
use core::ptr;
use spin::once::Once;
use crate::kernel::interrupts::intdispatcher::int_disp;
use crate::kernel::interrupts::InterruptStackFrame;

/// Static instance of the Interrupt Descriptor Table (IDT).
/// Wrapped inside a Once, because Idt::new() is not const.
static IDT: Once<Idt> = Once::new();

/// Global access to the IDT via a static reference.
pub fn get_idt() -> &'static Idt {
    IDT.call_once(Idt::new)
}

/// The IDT has 256 entries.
pub const IDT_SIZE: usize = 256;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// Structure of an entry in the IDT.
/// See the OSDev-Wiki for more details:
/// https://wiki.osdev.org/Interrupt_Descriptor_Table#Structure_on_x86-64
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    options: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

#[repr(C, packed)]
/// The IDT itself is just a packed array of 256 IDT entries.
pub struct Idt {
    entries: [IdtEntry; IDT_SIZE]
}

#[repr(C, packed)]
/// The IDT descriptor is used to load the IDT into the CPU.
/// It contains the address of the IDT and its size.
struct IdtDescriptor {
    limit: u16,
    address: u64,
}

impl IdtDescriptor {
    /// Create a new IDT descriptor for a given IDT.
    fn new(idt: &Idt) -> IdtDescriptor {
        IdtDescriptor {
            limit: (size_of::<Idt>() - 1) as u16, // Limit is the size of the IDT - 1
            address: ptr::from_ref(idt) as u64 // Address just points to the beginning of the IDT
        }
    }
}

impl IdtEntry {
    /// Create a new IDT entry for an interrupt handler at the given offset.
    /// Each entry has the same selector and options:
    /// The selector is the second entry in the GDT (kernel code segment) -> 2 * 8 = 16.
    /// The options are always 'Present', 'DPL=0' and '64-bit interrupt gate'.
    const fn new(offset: u64) -> IdtEntry {
        IdtEntry {
            offset_low: (offset & 0xffff) as u16,
            selector: 2 * 8, // Second entry in the GDT (kernel code segment)
            options: 0x8e00, // Present, DPL=0, 64-bit interrupt gate
            offset_mid: ((offset >> 16) & 0xffff) as u16,
            offset_high: ((offset >> 32) & 0xffffffffff) as u32,
            reserved: 0,
        }
    }
    
    /// Create a new IDT entry for an interrupt handler function.
    /// The function must be marked as 'extern "x86-interrupt"'.
    pub fn without_error_code(handler: extern "x86-interrupt" fn(InterruptStackFrame)) -> IdtEntry {
        IdtEntry::new(handler as u64)
    }
    
    /// Create a new IDT entry for an interrupt handler function with an error code.
    /// The function must be marked as 'extern "x86-interrupt"'.
    /// This is only used for some CPU exceptions (e.g. Page Faults).
    /// See the OSDev wiki for a full list of exceptions: https://wiki.osdev.org/Exceptions
    pub fn with_error_code(handler: extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64)) -> IdtEntry {
        IdtEntry::new(handler as u64)
    }
}

#[macro_export]
/// Macro to create an IDT entry for a given interrupt number and handler function.
/// The macro automatically creates a wrapper function for the handler,
/// which is marked as 'extern "x86-interrupt"' an determines, whether an error code is needed
/// for the handler, or not.
/// This way normal Rust functions can be used as interrupt handlers.
/// The signature of an interrupt handler is:
/// fn handler(vector: u8, stack_frame: InterruptStackFrame, error_code: Option<u64>)
macro_rules! interrupt_handler {
    ($int_num:expr, $handler:expr) => {{
        match $int_num {
            0x08 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x11 | 0x15 | 0x1d | 0x1e => {
                extern "x86-interrupt" fn wrapper(stack_frame: InterruptStackFrame, error_code: u64) {
                    $handler($int_num, stack_frame, Some(error_code));
                }

                IdtEntry::with_error_code(wrapper)
            },
            _ => {
                extern "x86-interrupt" fn wrapper(stack_frame: InterruptStackFrame) {
                    $handler($int_num, stack_frame, None);
                }

                IdtEntry::without_error_code(wrapper)
            }
        }
    }};
}

impl Idt {
    /// Create a new IDT with all entries set to the default handler `int_disp()`.
    pub fn new() -> Idt {
        Idt {
            entries: [
                interrupt_handler!(0x00, int_disp),
                interrupt_handler!(0x01, int_disp),
                interrupt_handler!(0x02, int_disp),
                interrupt_handler!(0x03, int_disp),
                interrupt_handler!(0x04, int_disp),
                interrupt_handler!(0x05, int_disp),
                interrupt_handler!(0x06, int_disp),
                interrupt_handler!(0x07, int_disp),
                interrupt_handler!(0x08, int_disp),
                interrupt_handler!(0x09, int_disp),
                interrupt_handler!(0x0a, int_disp),
                interrupt_handler!(0x0b, int_disp),
                interrupt_handler!(0x0c, int_disp),
                interrupt_handler!(0x0d, int_disp),
                interrupt_handler!(0x0e, int_disp),
                interrupt_handler!(0x0f, int_disp),
                interrupt_handler!(0x10, int_disp),
                interrupt_handler!(0x11, int_disp),
                interrupt_handler!(0x12, int_disp),
                interrupt_handler!(0x13, int_disp),
                interrupt_handler!(0x14, int_disp),
                interrupt_handler!(0x15, int_disp),
                interrupt_handler!(0x16, int_disp),
                interrupt_handler!(0x17, int_disp),
                interrupt_handler!(0x18, int_disp),
                interrupt_handler!(0x19, int_disp),
                interrupt_handler!(0x1a, int_disp),
                interrupt_handler!(0x1b, int_disp),
                interrupt_handler!(0x1c, int_disp),
                interrupt_handler!(0x1d, int_disp),
                interrupt_handler!(0x1e, int_disp),
                interrupt_handler!(0x1f, int_disp),
                interrupt_handler!(0x20, int_disp),
                interrupt_handler!(0x21, int_disp),
                interrupt_handler!(0x22, int_disp),
                interrupt_handler!(0x23, int_disp),
                interrupt_handler!(0x24, int_disp),
                interrupt_handler!(0x25, int_disp),
                interrupt_handler!(0x26, int_disp),
                interrupt_handler!(0x27, int_disp),
                interrupt_handler!(0x28, int_disp),
                interrupt_handler!(0x29, int_disp),
                interrupt_handler!(0x2a, int_disp),
                interrupt_handler!(0x2b, int_disp),
                interrupt_handler!(0x2c, int_disp),
                interrupt_handler!(0x2d, int_disp),
                interrupt_handler!(0x2e, int_disp),
                interrupt_handler!(0x2f, int_disp),
                interrupt_handler!(0x30, int_disp),
                interrupt_handler!(0x31, int_disp),
                interrupt_handler!(0x32, int_disp),
                interrupt_handler!(0x33, int_disp),
                interrupt_handler!(0x34, int_disp),
                interrupt_handler!(0x35, int_disp),
                interrupt_handler!(0x36, int_disp),
                interrupt_handler!(0x37, int_disp),
                interrupt_handler!(0x38, int_disp),
                interrupt_handler!(0x39, int_disp),
                interrupt_handler!(0x3a, int_disp),
                interrupt_handler!(0x3b, int_disp),
                interrupt_handler!(0x3c, int_disp),
                interrupt_handler!(0x3d, int_disp),
                interrupt_handler!(0x3e, int_disp),
                interrupt_handler!(0x3f, int_disp),
                interrupt_handler!(0x40, int_disp),
                interrupt_handler!(0x41, int_disp),
                interrupt_handler!(0x42, int_disp),
                interrupt_handler!(0x43, int_disp),
                interrupt_handler!(0x44, int_disp),
                interrupt_handler!(0x45, int_disp),
                interrupt_handler!(0x46, int_disp),
                interrupt_handler!(0x47, int_disp),
                interrupt_handler!(0x48, int_disp),
                interrupt_handler!(0x49, int_disp),
                interrupt_handler!(0x4a, int_disp),
                interrupt_handler!(0x4b, int_disp),
                interrupt_handler!(0x4c, int_disp),
                interrupt_handler!(0x4d, int_disp),
                interrupt_handler!(0x4e, int_disp),
                interrupt_handler!(0x4f, int_disp),
                interrupt_handler!(0x50, int_disp),
                interrupt_handler!(0x51, int_disp),
                interrupt_handler!(0x52, int_disp),
                interrupt_handler!(0x53, int_disp),
                interrupt_handler!(0x54, int_disp),
                interrupt_handler!(0x55, int_disp),
                interrupt_handler!(0x56, int_disp),
                interrupt_handler!(0x57, int_disp),
                interrupt_handler!(0x58, int_disp),
                interrupt_handler!(0x59, int_disp),
                interrupt_handler!(0x5a, int_disp),
                interrupt_handler!(0x5b, int_disp),
                interrupt_handler!(0x5c, int_disp),
                interrupt_handler!(0x5d, int_disp),
                interrupt_handler!(0x5e, int_disp),
                interrupt_handler!(0x5f, int_disp),
                interrupt_handler!(0x60, int_disp),
                interrupt_handler!(0x61, int_disp),
                interrupt_handler!(0x62, int_disp),
                interrupt_handler!(0x63, int_disp),
                interrupt_handler!(0x64, int_disp),
                interrupt_handler!(0x65, int_disp),
                interrupt_handler!(0x66, int_disp),
                interrupt_handler!(0x67, int_disp),
                interrupt_handler!(0x68, int_disp),
                interrupt_handler!(0x69, int_disp),
                interrupt_handler!(0x6a, int_disp),
                interrupt_handler!(0x6b, int_disp),
                interrupt_handler!(0x6c, int_disp),
                interrupt_handler!(0x6d, int_disp),
                interrupt_handler!(0x6e, int_disp),
                interrupt_handler!(0x6f, int_disp),
                interrupt_handler!(0x70, int_disp),
                interrupt_handler!(0x71, int_disp),
                interrupt_handler!(0x72, int_disp),
                interrupt_handler!(0x73, int_disp),
                interrupt_handler!(0x74, int_disp),
                interrupt_handler!(0x75, int_disp),
                interrupt_handler!(0x76, int_disp),
                interrupt_handler!(0x77, int_disp),
                interrupt_handler!(0x78, int_disp),
                interrupt_handler!(0x79, int_disp),
                interrupt_handler!(0x7a, int_disp),
                interrupt_handler!(0x7b, int_disp),
                interrupt_handler!(0x7c, int_disp),
                interrupt_handler!(0x7d, int_disp),
                interrupt_handler!(0x7e, int_disp),
                interrupt_handler!(0x7f, int_disp),
                interrupt_handler!(0x80, int_disp),
                interrupt_handler!(0x81, int_disp),
                interrupt_handler!(0x82, int_disp),
                interrupt_handler!(0x83, int_disp),
                interrupt_handler!(0x84, int_disp),
                interrupt_handler!(0x85, int_disp),
                interrupt_handler!(0x86, int_disp),
                interrupt_handler!(0x87, int_disp),
                interrupt_handler!(0x88, int_disp),
                interrupt_handler!(0x89, int_disp),
                interrupt_handler!(0x8a, int_disp),
                interrupt_handler!(0x8b, int_disp),
                interrupt_handler!(0x8c, int_disp),
                interrupt_handler!(0x8d, int_disp),
                interrupt_handler!(0x8e, int_disp),
                interrupt_handler!(0x8f, int_disp),
                interrupt_handler!(0x90, int_disp),
                interrupt_handler!(0x91, int_disp),
                interrupt_handler!(0x92, int_disp),
                interrupt_handler!(0x93, int_disp),
                interrupt_handler!(0x94, int_disp),
                interrupt_handler!(0x95, int_disp),
                interrupt_handler!(0x96, int_disp),
                interrupt_handler!(0x97, int_disp),
                interrupt_handler!(0x98, int_disp),
                interrupt_handler!(0x99, int_disp),
                interrupt_handler!(0x9a, int_disp),
                interrupt_handler!(0x9b, int_disp),
                interrupt_handler!(0x9c, int_disp),
                interrupt_handler!(0x9d, int_disp),
                interrupt_handler!(0x9e, int_disp),
                interrupt_handler!(0x9f, int_disp),
                interrupt_handler!(0xa0, int_disp),
                interrupt_handler!(0xa1, int_disp),
                interrupt_handler!(0xa2, int_disp),
                interrupt_handler!(0xa3, int_disp),
                interrupt_handler!(0xa4, int_disp),
                interrupt_handler!(0xa5, int_disp),
                interrupt_handler!(0xa6, int_disp),
                interrupt_handler!(0xa7, int_disp),
                interrupt_handler!(0xa8, int_disp),
                interrupt_handler!(0xa9, int_disp),
                interrupt_handler!(0xaa, int_disp),
                interrupt_handler!(0xab, int_disp),
                interrupt_handler!(0xac, int_disp),
                interrupt_handler!(0xad, int_disp),
                interrupt_handler!(0xae, int_disp),
                interrupt_handler!(0xaf, int_disp),
                interrupt_handler!(0xb0, int_disp),
                interrupt_handler!(0xb1, int_disp),
                interrupt_handler!(0xb2, int_disp),
                interrupt_handler!(0xb3, int_disp),
                interrupt_handler!(0xb4, int_disp),
                interrupt_handler!(0xb5, int_disp),
                interrupt_handler!(0xb6, int_disp),
                interrupt_handler!(0xb7, int_disp),
                interrupt_handler!(0xb8, int_disp),
                interrupt_handler!(0xb9, int_disp),
                interrupt_handler!(0xba, int_disp),
                interrupt_handler!(0xbb, int_disp),
                interrupt_handler!(0xbc, int_disp),
                interrupt_handler!(0xbd, int_disp),
                interrupt_handler!(0xbe, int_disp),
                interrupt_handler!(0xbf, int_disp),
                interrupt_handler!(0xc0, int_disp),
                interrupt_handler!(0xc1, int_disp),
                interrupt_handler!(0xc2, int_disp),
                interrupt_handler!(0xc3, int_disp),
                interrupt_handler!(0xc4, int_disp),
                interrupt_handler!(0xc5, int_disp),
                interrupt_handler!(0xc6, int_disp),
                interrupt_handler!(0xc7, int_disp),
                interrupt_handler!(0xc8, int_disp),
                interrupt_handler!(0xc9, int_disp),
                interrupt_handler!(0xca, int_disp),
                interrupt_handler!(0xcb, int_disp),
                interrupt_handler!(0xcc, int_disp),
                interrupt_handler!(0xcd, int_disp),
                interrupt_handler!(0xce, int_disp),
                interrupt_handler!(0xcf, int_disp),
                interrupt_handler!(0xd0, int_disp),
                interrupt_handler!(0xd1, int_disp),
                interrupt_handler!(0xd2, int_disp),
                interrupt_handler!(0xd3, int_disp),
                interrupt_handler!(0xd4, int_disp),
                interrupt_handler!(0xd5, int_disp),
                interrupt_handler!(0xd6, int_disp),
                interrupt_handler!(0xd7, int_disp),
                interrupt_handler!(0xd8, int_disp),
                interrupt_handler!(0xd9, int_disp),
                interrupt_handler!(0xda, int_disp),
                interrupt_handler!(0xdb, int_disp),
                interrupt_handler!(0xdc, int_disp),
                interrupt_handler!(0xdd, int_disp),
                interrupt_handler!(0xde, int_disp),
                interrupt_handler!(0xdf, int_disp),
                interrupt_handler!(0xe0, int_disp),
                interrupt_handler!(0xe1, int_disp),
                interrupt_handler!(0xe2, int_disp),
                interrupt_handler!(0xe3, int_disp),
                interrupt_handler!(0xe4, int_disp),
                interrupt_handler!(0xe5, int_disp),
                interrupt_handler!(0xe6, int_disp),
                interrupt_handler!(0xe7, int_disp),
                interrupt_handler!(0xe8, int_disp),
                interrupt_handler!(0xe9, int_disp),
                interrupt_handler!(0xea, int_disp),
                interrupt_handler!(0xeb, int_disp),
                interrupt_handler!(0xec, int_disp),
                interrupt_handler!(0xed, int_disp),
                interrupt_handler!(0xee, int_disp),
                interrupt_handler!(0xef, int_disp),
                interrupt_handler!(0xf0, int_disp),
                interrupt_handler!(0xf1, int_disp),
                interrupt_handler!(0xf2, int_disp),
                interrupt_handler!(0xf3, int_disp),
                interrupt_handler!(0xf4, int_disp),
                interrupt_handler!(0xf5, int_disp),
                interrupt_handler!(0xf6, int_disp),
                interrupt_handler!(0xf7, int_disp),
                interrupt_handler!(0xf8, int_disp),
                interrupt_handler!(0xf9, int_disp),
                interrupt_handler!(0xfa, int_disp),
                interrupt_handler!(0xfb, int_disp),
                interrupt_handler!(0xfc, int_disp),
                interrupt_handler!(0xfd, int_disp),
                interrupt_handler!(0xfe, int_disp),
                interrupt_handler!(0xff, int_disp),
            ]
        }
    }

    /// Overwrite an entry in the IDT with a new IDT entry.
    pub fn set_entry(&mut self, index: usize, entry: IdtEntry) {
        self.entries[index] = entry;
    }

    /// Load the IDT into the CPU.
    pub fn load(&self) {
        let idt_descriptor = IdtDescriptor::new(self);
        unsafe {
            asm!(
            "lidt [{}]",
            in(reg) &idt_descriptor,
            options(nostack)
            );
        }
    }
}