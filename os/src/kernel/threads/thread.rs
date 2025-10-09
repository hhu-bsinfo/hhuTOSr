/*
 * Module: thread
 *
 * Description: Contains functions to create, start, switch and end threads.
 *
 * Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 15.05.2023
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 07.08.2025
 */

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::{fmt, ptr};
use core::arch::naked_asm;
use core::fmt::Display;
use core::sync::atomic::AtomicUsize;
use crate::consts::{STACK_ENTRY_SIZE, STACK_SIZE};
use crate::kernel::cpu;
use crate::kernel::threads::scheduler::get_scheduler;

unsafe extern "C" {
    fn _tss_set_rsp0(rsp0: usize);
}

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_id() -> usize {
    THREAD_ID_COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst)
}

/// Low-level routine for starting a thread.
#[unsafe(naked)]
unsafe extern "C" fn thread_start(stack_ptr: usize) {
    naked_asm!(
        "mov rsp, rdi", // Switch stack

        "call unlock_scheduler", // Unlock scheduler

        "popf", // Pop rflags
        "pop rbp", // Pop all other registers
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "ret" // Return to 'kickoff'
    )
}

/// Low-level routine for switching to the next thread.
/// `current_stack_ptr` is a pointer to `stack_ptr` of the next coroutine (where the rsp is saved).
/// `next_stack` is the value of `stack_ptr` of the next thread (the new rsp value).
#[unsafe(naked)]
unsafe extern "C" fn thread_switch(current_stack_ptr: *mut usize, next_stack: usize, next_stack_end: usize) {
    naked_asm!(
        // Save all registers of the current thread on its stack
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",
        "pushf",

        // Save stackpointer in 'current_stack_ptr' (first parameter)
        "mov [rdi], rsp",

        // Update TSS rsp0 to 'next_stack_end' (third parameter)
        "mov rdi, rdx", // rdx = next_stack_end
        "call _tss_set_rsp0",

        // Switch stack to 'next_stack' (second parameter)
        "mov rsp, rsi",

        // Unlock scheduler
        "call unlock_scheduler",

        // Load all registers of the next thread
        "popf",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",

        "ret"
    )
}

#[unsafe(naked)]
unsafe extern "C" fn thread_user_start(stack_ptr: usize) {
    naked_asm!(
        "mov rsp, rdi", // Switch stack
        "pop rdi",
        "iretq" // Return to user mode
    )
}

/// Represents a coroutine in the system.
/// It contains the kernel and user stacks and the entry function.
/// Threads must be registered in the scheduler and are run automatically
/// once the scheduler is started.
#[repr(C)]
pub struct Thread {
    id: usize,
    is_kernel_thread: bool,
    kernel_stack: Vec<u64>,

    /*
     * Hier muss Code eingefuegt werden.
     */

    stack_ptr: usize, // Pointer on the stack to the saved context
    entry: fn(),
}

impl Thread {
    /// Create a new thread with the given entry function.
    pub fn new_kernel_thread(entry: fn()) -> Box<Thread> {
        // Allocate memory for the kernel stack and initialize it to zero
        let mut kernel_stack = Vec::<u64>::with_capacity(STACK_SIZE / 8);
        for _ in 0..kernel_stack.capacity() {
            kernel_stack.push(0);
        }

        // Allocate memory for the user stack and initialize it to zero
        /*
         * Hier muss Code eingefuegt werden.
         */

        // Set the stack pointer to the top of the stack
        let stack_ptr = ptr::from_ref(&kernel_stack[kernel_stack.capacity() - 1]) as usize;

        // Create a new thread object
        let mut thread = Box::new(
            Thread { id: next_id(), is_kernel_thread: true, kernel_stack, stack_ptr, entry }
        );

        // Prepare the stack for the thread so it can be started via `thread_start()`
        thread.prepare_kernel_stack();
        thread
    }

    pub fn new_user_thread(entry: fn()) -> Box<Thread> {
        let mut thread = Self::new_kernel_thread(entry);

        /*
         * Hier muss Code eingefuegt werden.
         */

        thread
    }

    /// Start the thread.
    /// This function is only once by the scheduler.
    /// The scheduler does further thread switching via `switch()`.
    pub fn start(&mut self) {
        unsafe {
            thread_start(self.stack_ptr);
        }
    }

    /// Switch from the `current` thread to the `next` thread.
    /// This function is called by the scheduler to switch between threads.
    pub unsafe fn switch(current: *mut Thread, next: *mut Thread) {
        unsafe {
            let current = &mut *current;
            let next = &*next;
            let next_stack_end = Thread::get_top_of_stack(&next.kernel_stack);

            thread_switch(&mut current.stack_ptr, next.stack_ptr, next_stack_end as usize);
        }
    }

    /// Get the ID of the thread.
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// Prepare the stack of a newly created thread in a way that it can be used
    /// to return to the 'kickoff' function with the thread itself as parameter.
    /// The prepared stack is used in 'thread_start' to start the first thread.
    /// Other threads are started by 'thread_switch' with the prepared stack.
    fn prepare_kernel_stack(&mut self) {
        let kickoff = Thread::kickoff_kernel_thread as u64;
        let thread = ptr::from_mut(self) as u64;
        let length = self.kernel_stack.len();
        let kernel_stack_top = Self::get_top_of_stack(&self.kernel_stack);

        self.kernel_stack[length - 1] = 0x131155; // Dummy return address
        self.kernel_stack[length - 2] = kickoff; // Address of 'kickoff'
        self.kernel_stack[length - 3] = 0; // r8
        self.kernel_stack[length - 4] = 0; // r9
        self.kernel_stack[length - 5] = 0; // r10
        self.kernel_stack[length - 6] = 0; // r11
        self.kernel_stack[length - 7] = 0; // r12
        self.kernel_stack[length - 8] = 0; // r13
        self.kernel_stack[length - 9] = 0; // r14
        self.kernel_stack[length - 10] = 0; // r15
        self.kernel_stack[length - 11] = 0; // rax
        self.kernel_stack[length - 12] = 0; // rbx
        self.kernel_stack[length - 13] = 0; // rcx
        self.kernel_stack[length - 14] = 0; // rdx
        self.kernel_stack[length - 15] = 0; // rsi
        self.kernel_stack[length - 16] = thread; // rdi -> First parameter for 'kickoff'
        self.kernel_stack[length - 17] = 0; // rbp
        self.kernel_stack[length - 18] = 0x2; // rflags (IE = 0); interrupts disabled

        self.stack_ptr = kernel_stack_top as usize - (STACK_ENTRY_SIZE * 18);
    }

    /// Switch this thread from Ring 0 to Ring 3.
    /// For this, the kernel stack is prepared in a way that an 'iretq' instruction
    /// switches to user mode (Ring 3) and the user stack is used. If this function works correctly,
    /// the thread continues in user mode in the function 'kickoff_user_thread'.
    fn switch_to_usermode(&mut self) {

        /*
         * Hier muss Code eingefuegt werden.
         */

    }

    /// Called indirectly by using the prepared stack in 'thread_start' and 'thread_switch'.
    fn kickoff_kernel_thread(&mut self) {
        // Set TSS rsp0 to the top of the kernel stack of this thread
        unsafe {
            let rsp0 = Self::get_top_of_stack(&self.kernel_stack);
            _tss_set_rsp0(rsp0 as usize);
        }

        if self.is_kernel_thread {
            cpu::enable_int(); // interrupts are disabled during thread start
            ((*self).entry)();
        } else {
            self.switch_to_usermode();
        }

        get_scheduler().exit();
    }

    /// Called indirectly by using the prepared stack in 'switch_to_usermode'.
    /// At this point, the thread is in user mode (Ring 3) and its entry function is called.
    fn kickoff_user_thread(&self) {

        /*
         * Hier muss Code eingefuegt werden.
         */

        loop {} // User threads may currently not exit
    }

    /// Get a pointer to the top of the given stack.
    fn get_top_of_stack(stack: &Vec<u64>) -> *const u64 {
        unsafe {
            ptr::from_ref(&stack[stack.len() - 1]).offset(1)
        }
    }
}

impl Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "T{}", self.id)
    }
}