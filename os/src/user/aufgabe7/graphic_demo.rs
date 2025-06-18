use crate::devices::lfb::{get_lfb, HHU_BLUE, HHU_GREEN, LFB};
use crate::devices::pcspk;
use crate::kernel::threads::scheduler::get_scheduler;
use crate::kernel::threads::thread::Thread;
use crate::user::aufgabe7::bmp_hhu;

const MESSAGE: &str = "Welcome to hhuTOS!";

pub fn run() {
    let draw_thread = Thread::new(draw_demo);
    let sound_thread = Thread::new(pcspk::tetris);
    
    let scheduler = get_scheduler();
    scheduler.ready(draw_thread);
    scheduler.ready(sound_thread);
    scheduler.schedule();
}

fn draw_demo() {
    let mut lfb = get_lfb().lock();
    let dimensions = lfb.get_dimensions();

    // Fill the framebuffer with a gradient
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let color = linear_interpolate_2d(x, y, dimensions.0, dimensions.1, 0x0000ff, 0x00ff00, 0xff0000, 0xffff00);

            // No need to check bounds here, as the loop ensures x and y are within the framebuffer dimensions
            unsafe {
                lfb.draw_pixel_unchecked(x, y, color);
            }
        }
    }

    // Draw a bitmap
    let bmp_pos = ((dimensions.0 - bmp_hhu::WIDTH) / 2, (dimensions.1 - bmp_hhu::HEIGHT) / 2);
    lfb.draw_bitmap(bmp_pos.0, bmp_pos.1, bmp_hhu::WIDTH, bmp_hhu::HEIGHT, bmp_hhu::DATA);

    // Draw a message
    let char_dimensions = lfb.get_char_dimensions();
    let text_pos = ((dimensions.0 - MESSAGE.len() as u32 * char_dimensions.0) / 2, bmp_pos.1 - char_dimensions.1 - char_dimensions.1 / 2);
    lfb.draw_str(text_pos.0, text_pos.1, HHU_BLUE, MESSAGE);
}

fn linear_interpolate_1d(x: u32, xr: u32, l: u32, r: u32) -> u32 {
    ((((l >> 16) * (xr - x) + (r >> 16) * x) / xr) << 16) // Red
        | (((((l >> 8) & 0xff) * (xr - x) + ((r >> 8) & 0xff) * x) / xr) << 8) // Green
        | (((l & 0xff) * (xr - x) + (r & 0xff) * x) / xr) // Blue
}

fn linear_interpolate_2d(x: u32, y: u32, xres: u32, yres: u32, lt: u32, rt: u32, lb:u32, rb: u32) -> u32 {
    linear_interpolate_1d(y, yres, linear_interpolate_1d(x, xres, lt, rt), linear_interpolate_1d(x, xres, lb, rb))
}