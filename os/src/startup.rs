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
#![allow(dead_code)]

#[macro_use]
mod devices;
mod kernel;
mod user;
mod consts;

use core::panic::PanicInfo;
use devices::cga_print;
use devices::keyboard;
use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;
use crate::devices::cga::CGA;

const BANNER: &str =
"   __    __        ______  ____    ____
  / /   / /  __ __/_  __/ / __ \\  / __/
 / _ \\ / _ \\/ // / / /   / /_/ / _\\ \\
/_//_//_//_/\\_,_/ /_/    \\____/ /___/  ";

#[unsafe(no_mangle)]
pub extern "C" fn startup() {
    kprintln!("Welcome to hhuTOS!");

    CGA.lock().clear();
    
    text_demo::run();
    keyboard_demo::run();
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    loop {}
}

