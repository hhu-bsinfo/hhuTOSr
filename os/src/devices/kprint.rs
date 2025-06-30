/*
 * Module: kprint
 *
 * Description: Implements format printing macros for the kernel, using the serial port.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 7.3.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 30.6.2025
 */

use core::fmt;
use core::fmt::Write;
use crate::devices::serial::COM1;

/// Print a formatted string to the COM port.
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::devices::kprint::kprint(format_args!($($arg)*));
    });
}

/// Print a formatted string to the COM port with a newline.
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

/// Helper function of print macros (must be public)
pub fn kprint(args: fmt::Arguments) {
    COM1.lock().write_fmt(args).unwrap();
}
