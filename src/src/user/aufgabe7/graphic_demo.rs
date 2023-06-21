
use alloc::{boxed::Box};
use crate::devices::cga;           // shortcut for cga
use crate::devices::cga_print;     // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::devices::vga;
use crate::devices::fonts::font_8x8 as font_8x8;


pub struct GraphicDemo {
}

impl GraphicDemo {

   pub fn new()-> Box<GraphicDemo> {
      Box::new(GraphicDemo {} )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut GraphicDemo {
	   self
   }
   
   /*****************************************************************************
    * Funktion:        lin_inter_pol_1d                                         *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Farbwert in einer Dimension interpoliert berechnen.      *
    *****************************************************************************/
   fn lin_inter_pol_1d(&self, x:u32, xr:u32, l:u32, r:u32) -> u32 {
      return ((((l>>16)*(xr-x)+(r>>16)*x)/xr)<<16)
              |(((((l>>8)&0xFF)*(xr-x)+((r>>8)&0xFF)*x)/xr)<<8)
              |(((l&0xFF)*(xr-x)+(r&0xFF)*x)/xr);
   }


   /*****************************************************************************
    * Funktion:        lin_inter_pol_2d                                         *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Farbwert in zwei Dimensionen interpoliert berechnen.     *
    *****************************************************************************/
   fn lin_inter_pol_2d(&self, x:u32,  y:u32, xres:u32, yres:u32, 
                       lt:u32, rt:u32,   lb:u32,   rb:u32) -> u32 {
      return self.lin_inter_pol_1d( y, yres,
                                    self.lin_inter_pol_1d(x, xres, lt, rt),
                                    self.lin_inter_pol_1d(x, xres, lb, rb) 
                                  );
   }


   /*****************************************************************************
    * Funktion:        draw_colors                                              *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Zeichnet einen Farbverlauf auf dem ganzen Bildschirm.    *
    *****************************************************************************/
   fn draw_colors (&self) {
      let (xres, yres) = vga::get_res();
      
      for y in 0 .. yres {
         for x in 0 .. xres {
		    let pix = self.lin_inter_pol_2d( x, y, xres, yres, 0x0000FF, 
		                                     0x00FF00, 0xFF0000, 0xFFFF00);
            vga::draw_pixel(x, y, pix);
         }
      }
   }

}

impl thread::ThreadEntry for GraphicDemo {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
	   let text_h = font_8x8::CHAR_HEIGHT;
	   
	   self.draw_colors();
		
       vga::draw_string(0, 0, vga::rgb_24(0,255,0), "hhuTOS 0.7");
       vga::draw_string(0, text_h, vga::rgb_24(0,255,0), "==========");
       vga::draw_string(0, 3*text_h, vga::rgb_24(0,255,0), "Wir sind jetzt im Grafikmodus!");
       
       loop {}
       // Beim exit gibt es noch ein Problem
       // scheduler::Scheduler::exit(thread_object);  
	}

}


pub fn init() {
    // Anwendung im Scheduler anmelden
    let pd = Box::new(GraphicDemo {} );
 	scheduler::Scheduler::ready( pd );  
}
