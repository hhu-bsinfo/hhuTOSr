
use crate::devices::pcspk;
use crate::devices::keyboard;


pub fn run() {
    println!("\nTest des PC-Lautsprechers:");

    println!("\nDruecke eine beliebige Taste!");
    keyboard::key_hit();
    println!("playing tetris ...");
    pcspk::tetris();

    println!("\nDruecke eine beliebige Taste!");
    keyboard::key_hit();
    println!("playing aerodynamic ...");
    pcspk::aerodynamic();

    println!("\n*** ENDE DER DEMO ***");
}
