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

/// Start address of the kernel heap (8 MiB -> Max image size = 7 MiB)
pub const HEAP_START: usize = 0x800000;
/// Size of the kernel heap (16 MiB)
pub const HEAP_SIZE: usize  = 16 * 1024 * 1024;
