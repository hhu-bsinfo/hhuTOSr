// Serielle Schnittstelle
use core::fmt;
use crate::kernel::cpu;

pub struct ComPort {
   base_addr: u16,   // Port-Adresse des Ports
}

impl ComPort {
	// COM-Port erzeugen für gegebene Port-Adresse
	const fn new(base_addr: u16) -> ComPort {
		ComPort {
			base_addr: base_addr,
		}
	}
}

impl fmt::Write for ComPort {
   // String auf dem COM-Port ausgeben.
   fn write_str(&mut self, s: &str) -> fmt::Result {
      // Output each byte of our string.
      for &b in s.as_bytes() {
	     // Write our byte.
	     cpu::outb(self.base_addr, b);
      }
      Ok(())
   }
}

// Unser Port fuer Ausgaben mit kprint
pub static mut COM1: ComPort = ComPort::new(0x3F8);
