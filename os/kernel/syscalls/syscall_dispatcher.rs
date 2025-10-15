/*
 * Module: syscall_dispatcher
 *
 * Description: All system calls are routed here via the IDT syscall handler (interrupt 0x80).
 *              The system call number is passed in the rax register and is used as index into
    *              the syscall table to call the corresponding function.
 *
 * Author: Stefan Lankes, RWTH Aachen University
 *         Licensed under the Apache License, Version 2.0 or MIT license, at your option.
 *
 *         Michael Schoettner, Heinrich Heine University Duesseldorf, 23.10.2024
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 15.10.2025
 */

use core::arch::{naked_asm};
use crate::kernel::syscalls::functions::hello::sys_hello_world;
use crate::kernel::syscalls::user_api::SyscallFunction;

/// Global syscall function table.
static SYSCALL_TABLE: SyscallFunctionTable = SyscallFunctionTable::new();

/// Struct to hold the syscall function pointers.
#[repr(align(64))]
#[repr(C)]
struct SyscallFunctionTable {
    table: [*const u64; SyscallFunction::NumSyscalls as usize],
}

impl SyscallFunctionTable {
    pub const fn new() -> SyscallFunctionTable {
        SyscallFunctionTable {
            table: [
                sys_hello_world as *const u64
            ],
        }
    }
}

unsafe impl Send for SyscallFunctionTable {}
unsafe impl Sync for SyscallFunctionTable {}

/// System call dispatcher.
/// This function is called from the IDT syscall handler (interrupt 0x80).
/// The syscall number is passed in the rax register and is used as index into
/// the syscall table to call the corresponding function.
/// All registers (except rax) are saved and restored.
/// The syscall function's return value is passed back in rax.
#[unsafe(naked)]
pub extern "C" fn syscall_disp() {
    naked_asm!(
        // Save all registers (except rax, which contains the syscall number)

        /*
         * Hier muss Code eingefuegt werden
         */

        // Call syscall handler (or syscall_abort for an invalid syscall number)
        "cmp rax, {NUM_SYSCALLS}",
        "jge syscall_abort",
        "call [{SYSCALL_TABLE} + rax * 8]",

        // Restore all registers (except rax)

        /*
         * Hier muss Code eingefuegt werden
         */

        // Return from interrupt
        "iretq",

        NUM_SYSCALLS = const SyscallFunction::NumSyscalls as usize,
        SYSCALL_TABLE = sym SYSCALL_TABLE
    )
}

/// This function is called if an invalid syscall number is passed.
/// It panics and halts the system.
#[unsafe(no_mangle)]
extern "C" fn syscall_abort() {
    panic!("Invalid syscall number");
}
