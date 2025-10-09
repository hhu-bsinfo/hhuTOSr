/*
 * Module: idle_thread
 * 
 * Description: Contains the function that is run in the idle thread.
 *              It just switches to the next thread in the scheduler.
 *
 * Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 15.05.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 07.08.2025
 */

use crate::kernel::threads::scheduler::get_scheduler;

pub fn idle_thread() {
    let scheduler = get_scheduler();
    loop {
        scheduler.yield_cpu();
    }
}
