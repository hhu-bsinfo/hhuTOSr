/*
 * Module: startup
 *
 * Description: Contains the main function `startup` which is called from the assembly boot code.
 *              Furthermore, the panic handler is defined here and all modules are defined.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 5.2.2024
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 30.6.2025
 */

#![no_std]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

extern crate alloc;

#[macro_use]
mod devices;
mod kernel;
mod library;
mod user;
mod consts;

use core::panic::PanicInfo;
use devices::keyboard;
use crate::devices::cga::CGA;
use crate::devices::pit;
use crate::kernel::cpu;
use crate::kernel::interrupts::{idt, intdispatcher, pic};
use crate::user::aufgabe8::user_threads::thread_test;

#[unsafe(no_mangle)]
pub extern "C" fn startup() {
    kprintln!("Welcome to hhuTOS!");
    
    kprintln!("Initializing heap allocator");
    kernel::allocator::init();
    
    kprintln!("Initializing PIC");
    pic::PIC.lock().init();

    kprintln!("Initializing interrupts");
    intdispatcher::INT_VECTORS.lock().init();
    idt::get_idt().load();

    kprintln!("Initializing keyboard");
    keyboard::plugin();
    
    kprintln!("Initializing Timer");
    pit::plugin();

    kprintln!("Enabling interrupts");
    cpu::enable_int();

    kprintln!("Boot sequence finished");

    CGA.lock().clear();
    thread_test();

    panic!("Returned to startup()!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    loop {}
}

