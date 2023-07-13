
use crate::devices::cga as cga;  
use crate::devices::cga_print;       
use crate::devices::cga_print::print;
use crate::devices::key as key;     
use crate::devices::keyboard as keyboard;  
use crate::kernel::allocator as allocator;
use crate::kernel::allocator::dump_free_list;  
use alloc::{boxed::Box, vec::Vec};

struct DemoStruct {
    a: u64,
    b: u64,
}

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
    {
        cga::clear();
        println!("Demo 1/4: 2 Structs dynamisch allozieren");
        println!("========================================\n");
        
        allocator::dump_free_list();

        println!("\nStructs anlegen");
        let s1 = Box::new(DemoStruct{a: 1, b: 2});
        let s2 = Box::new(DemoStruct{a: 3, b: 4});
        println!("   s1.a={}, s1.b={}", s1.a, s1.b);
        println!("   s2.a={}, s2.b={}", s2.a, s2.b);

        allocator::dump_free_list();
        wait_for_return();
        // Structs werden freigegeben
    }
    {
        cga::clear();
        println!("Demo 2/4: 2 Structs wieder freigeben");
        println!("====================================\n");
        allocator::dump_free_list();
        wait_for_return();
    }
    {
        cga::clear();
        println!("Demo 3/4: Vec mit 3 Structs anlegen und Inhalt eines Structs ausgeben");
        println!("=====================================================================\n");
        
        println!("Vec anlegen");
        let mut vec = Vec::new();
        println!("3 Structs anlegen");
        let s1 = Box::new(DemoStruct{a: 1, b: 2});
        let s2 = Box::new(DemoStruct{a: 3, b: 4});
        let s3 = Box::new(DemoStruct{a: 5, b: 6});
        vec.push(s1);
        vec.push(s2);
        vec.push(s3);

        println!("   v[0].a={}, v[0].b={}", vec[0].a, vec[0].b);
        println!("   v[1].a={}, v[1].b={}", vec[1].a, vec[1].b);
        println!("   v[2].a={}, v[2].b={}", vec[2].a, vec[2].b);

        allocator::dump_free_list();
        wait_for_return();
    }
    {
        cga::clear();
        println!("Demo 4/4: Vec mit Structs wieder loeschen");
        println!("=========================================\n");
        allocator::dump_free_list();
        println!("\n*** ENDE DER DEMO ***");
    }
    // free heap allocated struct before return
}



pub fn run () {
    demo();
    wait_for_return();
}
