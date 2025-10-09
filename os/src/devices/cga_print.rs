/*
 * Module: cga_print
 *
 * Description: Implements format printing macros, similar to the `io` crate, using `cga`.
 *
 * Author: Philipp Oppermann, see here: https://os.phil-opp.com/vga-text-mode/
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 30.6.2025
 */

use core::fmt;
use core::fmt::Write;
use crate::devices::cga::CGA;

/// Print a formatted string to the CGA text buffer.
/// This macro locks the CGA instance and writes the formatted string to it.
macro_rules! print {
    ($($arg:tt)*) => ({
        let mut cga = $crate::devices::cga::CGA.lock();
        $crate::devices::cga_print::print(&mut cga, format_args!($($arg)*));
    });
}

/// Print a formatted string to the CGA text buffer with a newline.
/// This is a convenience macro, wrapping the `print!` macro to add a newline at the end.
/// This macro locks the CGA instance and writes the formatted string to it.
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print a formatted string to the CGA text buffer.
/// This macro is similar to `print!`, but it takes a mutable reference to a CGA instance,
/// instead of locking the global CGA instance. This is useful for performing multiple writes
/// without locking and unlocking the CGA instance each time.
macro_rules! print_cga {
    ($cga:expr, $($arg:tt)*) => ({
        $crate::devices::cga_print::print($cga, format_args!($($arg)*));
    });
}

/// Print a formatted string to the CGA text buffer with a newline.
/// This macro is similar to println!, but it takes a mutable reference to a CGA instance,
/// instead of locking the global CGA instance. This is useful for performing multiple writes
/// without locking and unlocking the CGA instance each time.
macro_rules! println_cga {
    ($cga:expr, $fmt:expr) => (print_cga!($cga, concat!($fmt, "\n")));
    ($cga:expr, $fmt:expr, $($arg:tt)*) => (print_cga!($cga, concat!($fmt, "\n"), $($arg)*));
}

/// Helper function of print macros (must be public)
pub fn print(cga: &mut CGA, args: fmt::Arguments) {
    cga.write_fmt(args).unwrap();
}