/*
 * Module: consts
 *
 * Description: Defines global kernel constants, such as stack size or heap start and size.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 7.3.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 30.6.2025
 */

/// Stack size for each new thread (512 KiB)
pub const STACK_SIZE: usize = 0x80000;
/// Each stack should be aligned to 8 bytes
pub const STACK_ALIGNMENT: usize = 8;
/// Size of a stack entry (8 bytes, for 64-bit systems)
pub const STACK_ENTRY_SIZE: usize = 8;

/// Size of the kernel heap (16 MiB)
pub const HEAP_SIZE: usize  = 16 * 1024 * 1024;

/// Size of a physical page frame (4 KiB)
pub const PAGE_FRAME_SIZE: usize = 0x1000;
/// Size of a virtual page (4 KiB)
pub const PAGE_SIZE: usize = 0x1000;

/// Start address of the user code in virtual memory (1 TiB)
pub const USER_CODE_VIRT_START: usize = 0x100_0000_0000;

/// Start address of the user stack in virtual memory (64 TiB)
pub const USER_STACK_VIRT_START:usize = 0x4000_0000_0000;
/// End address of the user stack in virtual memory
pub const USER_STACK_VIRT_END: usize = USER_STACK_VIRT_START + STACK_SIZE;