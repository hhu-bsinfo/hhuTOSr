impl IdtEntry {
    /// Create a new IDT entry for an interrupt handler at the given offset.
    /// Each entry has the same selector and options:
    /// The selector is the second entry in the GDT (kernel code segment) -> 2 * 8 = 16.
    /// The options are always 'Present', 'DPL=0' and '64-bit interrupt gate'.
    const fn new_interrupt_gate(offset: u64) -> IdtEntry {
        IdtEntry {
            offset_low: (offset & 0xffff) as u16,
            selector: 2 * 8, // Second entry in the GDT (kernel code segment)
            options: 0x8e00, // Present, DPL=0, 64-bit interrupt gate
            offset_mid: ((offset >> 16) & 0xffff) as u16,
            offset_high: ((offset >> 32) & 0xffffffffff) as u32,
            reserved: 0,
        }
    }

    /// Create a new IDT entry for a trap gate at the given offset.
    /// This is used for system calls, which should be accessible from user mode (DPL=3).
    const fn new_trap_gate(offset: u64) -> IdtEntry {
        /*
         * Hier muss Code eingefuegt werden.
         */
    }

    /// Create a new IDT entry for an interrupt handler function.
    /// The function must be marked as 'extern "x86-interrupt"'.
    pub fn without_error_code(handler: extern "x86-interrupt" fn(InterruptStackFrame)) -> IdtEntry {
        IdtEntry::new_interrupt_gate(handler as u64)
    }

    /// Create a new IDT entry for an interrupt handler function with an error code.
    /// The function must be marked as 'extern "x86-interrupt"'.
    /// This is only used for some CPU exceptions (e.g. Page Faults).
    /// See the OSDev wiki for a full list of exceptions: https://wiki.osdev.org/Exceptions
    pub fn with_error_code(handler: extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64)) -> IdtEntry {
        IdtEntry::new_interrupt_gate(handler as u64)
    }

    /// Create a new IDT entry for a syscall handler function.
    /// The function must be marked as 'extern "x86-interrupt"'.
    /// This entry is a trap gate and has DPL=3, so it can be called from user mode.
    pub fn syscall_gate(handler: extern "C" fn()) -> IdtEntry {
        IdtEntry::new_trap_gate(handler as u64)
    }
}
