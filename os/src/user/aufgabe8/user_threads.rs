use crate::devices::cga::CGA;
use crate::devices::{cga, pit};
use crate::kernel::threads::scheduler::get_scheduler;
use crate::kernel::threads::thread::Thread;

pub fn thread_test() {
    let kernel_thread = Thread::new_kernel_thread(kernel_test_thread);
    let user_thread = Thread::new_user_thread(user_test_thread);
    let scheduler = get_scheduler();
    scheduler.ready(kernel_thread);
    scheduler.ready(user_thread);
    scheduler.schedule();
}

fn kernel_test_thread() {
    let id = get_scheduler().get_active_tid();

    for i in 0..cga::CGA_COLUMNS {
        {
            let mut cga = CGA.lock();
            cga.setpos(i, id);
            print_cga!(&mut cga, "K");
        }

        pit::wait(1000)
    }
}

fn user_test_thread() {
    let id = get_scheduler().get_active_tid();
    
    for i in 0..cga::CGA_COLUMNS {
        {
            let mut cga = CGA.lock();

            // User threads may not access I/O ports and thus cannot change the CGA cursor position.
            // Calling one of the print macros would thus lead to a protection fault.
            cga.show(i, id, 'U', cga::CGA_STD_ATTR);
        }

        // User threads may not yield the CPU manually and thus cannot call pit::wait(),
        // as it calls Scheduler::yield_cpu() internally.
        // Instead, we implement a simple busy-wait loop here.
        let start = pit::get_system_time();
        while pit::get_system_time() - start < 1000 {
            core::hint::spin_loop();
        }
    }
}
