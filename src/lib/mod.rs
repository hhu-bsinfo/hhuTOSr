/*

Muss auskommentiert werden, da wir jetzt 'std' nutzen

#[macro_use]


//
// required functions by compiler
//
use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] 
#[no_mangle] 
pub extern fn eh_personality() {
}
*/
