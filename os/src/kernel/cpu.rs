/*
 * Module: cpu
 *
 * Description: Contains wrappers for different CPU functions,
 *              as well as the `IoPort` struct for reading and writing to I/O-ports.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 9.6.2024
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 30.6.2025
 */

use core::arch::asm;

/// Represents an I/O-port for reading and writing data.
pub struct IoPort {
    port: u16
}

impl IoPort {
    /// Create a new IoPort object
    pub const fn new(port: u16) -> IoPort {
        IoPort { port }
    }


    /// Write a single byte to a port
    #[inline]
    pub unsafe fn outb(&mut self, data: u8) {
        unsafe {
            asm!(
            "out dx, al",
            in("dx") self.port,
            in("al") data,
            );
        }
    }

    /// Read a single byte from a port
    #[inline]
    pub unsafe fn inb(&mut self) -> u8 {
        let ret: u8;
        unsafe {
            asm!(
            "in al, dx",
            in("dx") self.port,
            out("al") ret,
            );
        }
        ret
    }
}

/// Check if IE bit is set in RFLAGS
#[inline]
pub fn is_int_enabled() -> bool {
    let rflags: u64;

    unsafe {
        asm!(
        "pushf;",
        "pop {};",
        lateout(reg) rflags,
        options(nomem, nostack, preserves_flags)
        );
    };

    (rflags & (1u64 << 9)) != 0
}

/// Clear IE bit in RFLAGS and return the previous state
#[inline]
pub fn disable_int_nested() -> bool {
    let was_enabled = is_int_enabled();
    disable_int();
    was_enabled
}

/// Set IE bit in RFLAGS if it was set before
#[inline]
pub fn enable_int_nested(was_enabled: bool) {
    if was_enabled == true {
        enable_int();
    }
}

/// Set IE bit in RFLAGS
#[inline]
pub fn enable_int () {
    unsafe { asm!("sti"); }
}

/// Clear IE bit in RFLAGS
#[inline]
pub fn disable_int () {
    unsafe { asm!("cli"); }
}

/// Stop the CPU until an interrupt occurs
#[inline]
pub fn halt () {
    loop {
        unsafe { asm!("hlt"); }
    }
}

/// Get the current value of RFLAGS
#[inline]
pub fn get_flags() -> u64 {
    let rflags: u64;
    unsafe {
        asm!(
        "pushfq;",
        "pop {};",
        out(reg) rflags,
        options(nomem, preserves_flags)
        );
    }

    rflags
}

/// Execute a closure without interrupts
#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where F: FnOnce() -> R{
    let ie = disable_int_nested();
    let ret = f();

    enable_int_nested(ie);
    ret
}
