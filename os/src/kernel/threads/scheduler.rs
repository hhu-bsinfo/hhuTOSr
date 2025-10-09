/*
 * Module: scheduler
 * 
 * Description: A basic round-robin scheduler for cooperative threads.
 *              Priorities are not supported.
 *
 * Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 15.05.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 07.08.2025
 */

use alloc::boxed::Box;
use core::fmt::Display;
use core::{fmt, ptr};
use spin::Once;
use crate::kernel::{allocator, cpu};
use crate::kernel::threads::idle_thread::idle_thread;
use crate::kernel::threads::thread::Thread;
use crate::library::queue::LinkedQueue;
use spin::mutex::Mutex;

/// Global scheduler instance
static SCHEDULER: Once<Scheduler> = Once::new();

/// Global access to the scheduler.
pub fn get_scheduler() -> &'static Scheduler {
    SCHEDULER.call_once(|| { Scheduler::new() })
}

/// Unlock the scheduler state.
/// This function is called from assembly code.
/// Usually, the mutex would be unlocked automatically when going out of scope.
/// However, since we switch to a different thread in `yield_cpu()` and `exit()`,
/// the scope is not left and the mutex remains locked.
/// As a workaround, we provide this function to unlock the scheduler manually.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlock_scheduler() {
    unsafe {
        get_scheduler().state.force_unlock();
    }
}

/// The state of the scheduler.
/// It contains the active thread and the ready queue with all other threads.
/// The state is contained in its own struct so that it can be locked via a mutex.
struct SchedulerState {
    initialized: bool,
    active_thread: Option<Box<Thread>>,
    ready_queue: LinkedQueue<Box<Thread>>
}

/// Represents the scheduler.
/// It is round-robin-based and uses a queue to manage the threads.
pub struct Scheduler {
    state: Mutex<SchedulerState>,
}

impl Scheduler {
    /// Create a new scheduler instance with an empty ready queue
    /// and an idle thread as the active thread.
    pub fn new() -> Self {
        let state = SchedulerState {
            initialized: false,
            active_thread: Some(Thread::new_kernel_thread(idle_thread)),
            ready_queue: LinkedQueue::new(),
        };
        
        Scheduler { state: Mutex::new(state) }
    }

    /// Check if the scheduler has been initialized already.
    pub fn is_initialized(&self) -> bool {
        self.state.lock().initialized
    }
    
    /// Check if the scheduler state is currently locked.
    pub fn is_locked(&self) -> bool {
        self.state.is_locked()
    }
    
    /// Get the ID of the currently active thread.
    pub fn get_active_tid(&self) -> usize {
        let state = self.state.lock();
        
        state.active_thread.as_ref().unwrap().get_id()
    }

    /// Start the scheduler.
    /// This function must only be called once.
    pub fn schedule(&self) {
        let mut state = self.state.lock();

        // The active thread is never None, since we must at least have the idle thread.
        state.initialized = true;
        state.active_thread.as_mut().unwrap().start();
    }

    /// Register a new thread in the ready queue.
    pub fn ready(&self, thread: Box<Thread>) {
        let mut state = self.state.lock();
        
        state.ready_queue.enqueue(thread);
    }

    /// Terminate the current (calling) thread and switch to the next one.
    pub fn exit(&self) {
        let mut state = self.state.lock();

        // The active thread is never None, since we must at least have the idle thread.
        let mut current = state.active_thread.take().unwrap();
        // The idle thread never exits, so there must be at least one thread in the queue.
        let next = state.ready_queue.dequeue().unwrap();
            
        // Set the dequeued thread as the active thread,
        // overwriting the current one, which we want to exit.
        state.active_thread = Some(next);
        
        unsafe {
            // Switch to the next thread.
            // `current` still contains the old thread we want to exit,
            // while `state.active_thread` contains the next one.
            Thread::switch(current.as_mut(), state.active_thread.as_mut().unwrap().as_mut());
        }
    }

    /// Yield the CPU and switch to the next thread in the ready queue.
    pub fn yield_cpu(&self) {
        if allocator::is_locked() {
            // Allocator is locked, so queue operations would deadlock.
            return
        }

        if let Some(mut state) = self.state.try_lock() {
            if !state.initialized {
                // Scheduler is not initialized yet, so we just return.
                return;
            }

            // Check if there is a next thread in the queue.
            // Otherwise, we just return to the current thread.
            if let Some(next) = state.ready_queue.dequeue() {
                // The active thread is never None, since we must at least have the idle thread.
                let mut current = state.active_thread.take().unwrap();
                let current_ptr = ptr::from_mut(current.as_mut());

                // Set the dequeued thread as the active thread and enqueue the old active thread.
                state.active_thread = Some(next);
                state.ready_queue.enqueue(current);

                unsafe {
                    // Switch to the next thread.
                    // We created `current_ptr` earlier to still have access to the old thread after
                    // it has been enqueued. Since we know that the thread object still exists,
                    // and its memory address has not changed, because it is wrapped in a Box,
                    // we can safely use it here.
                    Thread::switch(current_ptr, state.active_thread.as_mut().unwrap().as_mut());
                }
            }
        }
    }
    
    /// Prepare the current thread for blocking.
    /// This functions disables interrupts and return the current thread, 
    /// as well as the return value from `cpu::disable_int_nested()`.
    /// To complete the blocking operation call `switch_from_blocked_thread()`,
    /// which will enable interrupts again and resume the scheduler.
    pub fn prepare_block(&self) -> (Box<Thread>, bool) {
        let mut state = self.state.lock();
        let interrupts = cpu::disable_int_nested();

        // The active thread is never None, since we must at least have the idle thread.
        let current = state.active_thread.take().unwrap();
        state.active_thread = state.ready_queue.dequeue();
        
        (current, interrupts)
    }
    
    /// Complete a blocking operation begun with `prepare_block()`.
    /// This resumes the scheduler and switches to the next thread in the ready queue.
    pub unsafe fn switch_from_blocked_thread(&self, blocked_thread: *mut Thread, interrupts_enabled: bool) {
        let mut state = self.state.lock();
        
        unsafe {
            // Switch to the active thread.
            // Interrupts should get enabled again by popping the flags from the stack.
            Thread::switch(blocked_thread, state.active_thread.as_mut().unwrap().as_mut());
            
            // We are now back in the blocked thread, which has been resumed.
            // We can now enable interrupts again, if they were enabled before blocking.
            cpu::enable_int_nested(interrupts_enabled);
        }
    }

    /// Kill the thread with the given ID by removing it from the ready queue.
    pub fn kill(&self, to_kill_id: usize) {
        // Cannot kill idle thread
        if to_kill_id == 0 {
            panic!("Cannot kill idle thread!");
        }
        
        // Cannot kill myself
        if to_kill_id == self.get_active_tid() {
            panic!("A thread cannot kill itself!");
        }

        let mut state = self.state.lock();
        state.ready_queue.remove(|thread| thread.get_id() == to_kill_id);
    }
}

impl Display for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = self.state.lock();
        let active = state.active_thread.as_ref().unwrap();
        
        write!(f, "active: {}, ready: {}", active, state.ready_queue)
    }
}