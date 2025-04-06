/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: kprint                                                          ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implements the macros kprint! and kprintln! using 'serial'.     ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 7.3.2023                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use crate::devices::serial;
use core::fmt;
use core::fmt::Write;
use spin::Mutex;
use crate::devices::serial::{ComBaseAddress, ComPort};

/// The global writer that can used as an interface from other modules.
/// It is threadsafe by using 'Mutex'.
pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

/// Writer for writing formatted strings to the CGA screen.
pub struct Writer {
    com_port: ComPort
}

impl Writer {
    /// Create a new Writer object.
    /// The 'com_port' is initialized with the default COM port.
    pub const fn new() -> Writer {
        Writer {
            com_port: ComPort::new(ComBaseAddress::Com1)
        }
    }
}

/// Implementation of the 'core::fmt::Write' trait for our Writer.
/// Required to output formatted strings.
/// Requires only one function 'write_str'.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial::COM1.lock().write_str(s)
    }
}

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works
// from outside the 'std' crate.
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::devices::kprint::kprint(format_args!($($arg)*));
    });
}

macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

/// Helper function of print macros (must be public)
pub fn kprint(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
