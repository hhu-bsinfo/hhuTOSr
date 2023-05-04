
use crate::devices::cga as cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::devices::key as key;      // shortcut for key
use crate::devices::keyboard as keyboard;  // shortcut for keyboard


pub fn run() {

   loop{
      let mut key: key::Key = keyboard::key_hit();
      if key.valid() {
         cga::print_byte(key.asc);
      }
   }

}
