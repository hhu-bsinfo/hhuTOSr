

use crate::devices::cga;        // shortcut for cga
use crate::devices::cga_print;  // used to import code needed by println! 


pub fn run () {
    println!("Test der Zahlenausgabefunktion:\n");
    println!("  | dec | hex | square |");
    cga::print_str("  ----------------------\n",
        cga::attribute(cga::Color::Black, cga::Color::White, false));
}
