// Stack size for each new thread
pub const STACK_SIZE: usize = 0x80000;             // 512 KB for each stack
pub const STACK_ALIGNMENT: usize = 8; 
pub const STACK_ENTRY_SIZE: usize = 8;

pub const HEAP_START: usize = 0x800000;            // 8 MB -> max image size = 7 MB 
pub const HEAP_SIZE: usize  = 16 * 1024 * 1024;    // 16 MB heap size
