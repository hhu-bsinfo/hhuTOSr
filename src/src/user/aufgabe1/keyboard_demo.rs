
use crate::devices::cga as cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::devices::key as key;      // shortcut for key
use crate::devices::keyboard as keyboard;  // shortcut for keyboard


pub fn run() {
    let mut input: key::Key;
    loop {
        input = keyboard::key_hit();
        cga::print_byte(input.get_ascii());
    }
}
