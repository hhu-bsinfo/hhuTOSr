
pub mod pic;
pub mod isr;
pub mod intdispatcher;


extern "C" { fn _init_interrupts(); }


pub fn init() {

    // setup IDT and PIC (in 'interrupts.asm')
    unsafe {
       _init_interrupts();
    }
    
    // IntDispatcher initialisieren
    intdispatcher::init();

}
