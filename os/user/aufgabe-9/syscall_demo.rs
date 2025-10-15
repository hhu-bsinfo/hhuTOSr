use crate::kernel::syscalls::user_api::usr_hello_world;
use crate::kernel::threads::scheduler::get_scheduler;
use crate::kernel::threads::thread::Thread;

pub fn syscall_test() {
    let thread = Thread::new_user_thread(syscall_test_thread);
    let scheduler = get_scheduler();
    scheduler.ready(thread);
    scheduler.schedule();
}

fn syscall_test_thread() {
    usr_hello_world();

    loop {

        /*
         * Hier muss Code eingefuegt werden
         */

    }
}
