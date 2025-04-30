pub mod pic;
pub mod idt;
pub mod intdispatcher;
pub mod isr;

#[derive(Debug)]
#[repr(C, packed)]
/// Context that is pushed onto the stack automatically
/// by the CPU when an interrupt occurs.
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}
