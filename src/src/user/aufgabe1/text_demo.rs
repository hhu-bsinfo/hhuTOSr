

use crate::devices::cga;        // shortcut for cga
use crate::devices::cga_print;  // used to import code needed by println! 

pub fn run () {
    

      let att = cga::attribute(cga::Color::White, cga::Color::Black, true); // create attribute
   
      // print text
      cga::print_str("Hello World!", att);
      schoettner_ausgabe(att);
      for i in 0..2 {
            cga::scrollup();
      }
        
        

}

fn schoettner_ausgabe(attrib: u8) {
      cga::print_str(
          "
   | dec | hex | bin   |
   ---------------------
   | ",
          attrib,
      );
      cga::print_dec(0);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x0);
      cga::print_str("   | ", attrib);
      cga::print_dec(0);
      cga::print_str(
          "     | 
   | ",
          attrib,
      );
      cga::print_dec(1);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x1);
      cga::print_str("   | ", attrib);
      cga::print_dec(1);
      cga::print_str(
          "     |
   | ",
          attrib,
      );
      cga::print_dec(2);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x2);
      cga::print_str("   | ", attrib);
      cga::print_dec(10);
      cga::print_str(
          "    |
   | ", attrib,
      );
      cga::print_dec(3);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x3);
      cga::print_str("   | ", attrib);
      cga::print_dec(11);
      cga::print_str(
          "    |
   | ", attrib,
      );
      cga::print_dec(4);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x4);
      cga::print_str("   | ", attrib);
      cga::print_dec(100);
      cga::print_str(
          "   |
   | ", attrib,
      );
      cga::print_dec(5);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x5);
      cga::print_str("   | ", attrib);
      cga::print_dec(101);
      cga::print_str(
          "   |
   | ", attrib,
      );
      cga::print_dec(6);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x6);
      cga::print_str("   | ", attrib);
      cga::print_dec(110);
      cga::print_str(
          "   |
   | ", attrib,
      );
      cga::print_dec(7);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x7);
      cga::print_str("   | ", attrib);
      cga::print_dec(111);
      cga::print_str(
          "   |
   | ", attrib,
      );
      cga::print_dec(8);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x8);
      cga::print_str("   | ", attrib);
      cga::print_dec(1000);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(9);
      cga::print_str("   | ", attrib);
      cga::print_hex(0x9);
      cga::print_str("   | ", attrib);
      cga::print_dec(1001);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(10);
      cga::print_str("  | ", attrib);
      cga::print_hex(0xa);
      cga::print_str("   | ", attrib);
      cga::print_dec(1010);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(11);
      cga::print_str("  | ", attrib);
      cga::print_hex(0xb);
      cga::print_str("   | ", attrib);
      cga::print_dec(1011);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(12);
      cga::print_str("  | ", attrib);
      cga::print_hex(0xc);
      cga::print_str("   | ", attrib);
      cga::print_dec(1100);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(13);
      cga::print_str("  | ", attrib);
      cga::print_hex(0xd);
      cga::print_str("   | ", attrib);
      cga::print_dec(1101);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(14);
      cga::print_str("  | ", attrib);
      cga::print_hex(0xe);
      cga::print_str("   | ", attrib);
      cga::print_dec(1110);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(15);
      cga::print_str("  | ", attrib);
      cga::print_hex(0xf);
      cga::print_str("   | ", attrib);
      cga::print_dec(1111);
      cga::print_str(
          "  |
   | ", attrib,
      );
      cga::print_dec(16);
      cga::print_str("  | ", attrib);
      cga::print_hex(0x10);
      cga::print_str("  | ", attrib);
      cga::print_dec(10000);
      cga::print_str(" |", attrib);
   }