impl Scheduler {

    /// Check if the scheduler state is currently locked.
    pub fn is_locked(&self) -> bool {
        self.state.is_locked()
    }

    /// Prepare the current thread for blocking.
    /// This functions disables interrupts and return the current thread,
    /// as well as the return value from `cpu::disable_int_nested()`.
    /// To complete the blocking operation call `switch_from_blocked_thread()`,
    /// which will enable interrupts again and resume the scheduler.
    pub fn prepare_block(&self) -> (Box<Thread>, bool) {

        /* Hier muss Code eingefuegt werden */

    }

    /// Complete a blocking operation begun with `prepare_block()`.
    /// This resumes the scheduler and switches to the next thread in the ready queue.
    pub unsafe fn switch_from_blocked_thread(&self, blocked_thread: *mut Thread, interrupts_enabled: bool) {

        /* Hier muss Code eingefuegt werden */

    }

}
