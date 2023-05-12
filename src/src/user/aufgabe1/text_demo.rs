

use crate::devices::cga;        // shortcut for cga
use crate::devices::cga_print;  // used to import code needed by println! 


pub fn run () {
    println!("Test der Zahlenausgabefunktion:\n");
    println!("  | dec | hex | square |");
    cga::print_str("  ----------------------\n",
        cga::attribute(cga::Color::Black, cga::Color::White, false));
    for i in 0..=16 {
        print!("  | ");
        cga::print_dec(i);
        if i<10 {print!(" ")};
        print!("  | 0x");
        cga::print_hex(i);
        if i!=16 {print!(" ")};
        print!("| ");
        println!("{:<6} |", i*i);
    }
}
