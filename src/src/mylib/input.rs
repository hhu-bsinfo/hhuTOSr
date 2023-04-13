

use crate::devices::keyboard as keyboard;  


pub fn getch() -> u8 {
   let mut k: u8;
   
   loop {
      k = keyboard::get_lastkey();
      if k != 0 {
		  break;
      }
   }
   k
}

pub fn wait_for_return() {
   loop {
      if keyboard::get_lastkey() == 10 {
		  break;
      }
   }
}
