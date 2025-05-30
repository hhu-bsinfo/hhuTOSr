/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga_print                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implements the macros print! and println! using 'cga'.          ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann, see here:                                    ║
   ║         https://os.phil-opp.com/vga-text-mode/                          ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use core::fmt;
use core::fmt::Write;
use spin::Mutex;
use crate::devices::cga;
use crate::devices::cga::CGA;

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works 
// from outside the 'std' crate.
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::cga_print::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print_cga {
    ($cga:expr, $($arg:tt)*) => ({
        $crate::cga_print::print_cga($cga, format_args!($($arg)*));
    });
}

macro_rules! println_cga {
    ($cga:expr, $fmt:expr) => ($cga, print!(concat!($fmt, "\n")));
    ($cga:expr, $fmt:expr, $($arg:tt)*) => ($cga, print!(concat!($fmt, "\n"), $($arg)*));
}

/// Helper function of print macros (must be public)
pub fn print_cga(cga: &mut CGA, args: fmt::Arguments) {
    cga.write_fmt(args).unwrap();
}

/// Helper function of print_cga macros (must be public)
pub fn print(args: fmt::Arguments) {
    print_cga(&mut CGA.lock(), args);
}
