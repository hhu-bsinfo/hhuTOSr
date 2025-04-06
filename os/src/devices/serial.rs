use core::fmt;
use spin::Mutex;
use crate::kernel::cpu;
use crate::kernel::cpu::IoPort;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum ComBaseAddress {
    Com1 = 0x3f8,
    Com2 = 0x2f8,
    Com3 = 0x3e8,
    Com4 = 0x2e8,
}

/// Struct representing a COM port
pub struct ComPort {
    /// IO-port where output is written to
    data_port: IoPort
}

impl ComPort {
    /// Create a new COM port
    pub const fn new(base_addr: ComBaseAddress) -> ComPort {
        ComPort {
            data_port: IoPort::new(base_addr as u16)
        }
    }
}

/// Implement the `Write` trait for `ComPort`.
/// This allows us to use `kprint!` and `kprintln!` macros
impl fmt::Write for ComPort {

    /// Write a string to the COM port
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Iterate over each byte in the string
        for &b in s.as_bytes() {
            // Write the current byte to the COM port
            //
            // Unsafe because we are writing to hardware.
            // By only allowing enum-values in the constructor,
            // we ensure that the port address is valid.
            //
            // Since we are using a mutable reference to the port,
            // we can be sure that the port is not used by another thread.
            unsafe { self.data_port.outb(b); }
        }
        Ok(())
    }
}

// Standard com-port for kernel output via kprint! and kprintln!
pub static COM1: Mutex<ComPort> = Mutex::new(ComPort::new(ComBaseAddress::Com1));
