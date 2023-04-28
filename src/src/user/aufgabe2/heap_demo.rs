
use crate::devices::cga as cga;  
use crate::devices::cga_print;       
use crate::devices::key as key;     
use crate::devices::keyboard as keyboard;  
use crate::kernel::allocator as allocator;  
use alloc::{boxed::Box, vec::Vec};



// Hilfsfunktion: Auf Return-Taste warten
fn wait_for_return() {
	
	println!("");
	println!("");
    println!("Weiter mit <ENTER>");

   loop {
      let mut key: key::Key = keyboard::key_hit();
        
      if key.valid() == true {
		  if key.get_ascii() == 13 { break; }
      }
   }
}


fn demo() {

    /* Hier muss Code eingefuegt werden */

    // free heap allocated struct before return
}



pub fn run () {

    demo();

    /* Hier muss Code eingefuegt werden */

}
