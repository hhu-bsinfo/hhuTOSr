#![no_std]

use core::panic::PanicInfo;
use usrlib::user_api::usr_hello_world;

#[unsafe(link_section = ".main")]
#[unsafe(no_mangle)]
fn main() {
    usr_hello_world();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
