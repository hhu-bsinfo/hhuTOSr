use crate::devices::keyboard;

/// Wait for a key press and return the character if it is a valid ASCII character.
pub fn getch() -> char {
   loop {
      let key = keyboard::get_key_buffer().wait_for_key();
      if key.valid() && key.get_ascii() != 0 {
         return char::from_u32(key.get_ascii() as u32).unwrap();
      }
   }
}

/// Wait for the Enter key to be pressed.
pub fn wait_for_return() {
   loop {
      if getch() == '\r' {
		  break;
      }
   }
}
