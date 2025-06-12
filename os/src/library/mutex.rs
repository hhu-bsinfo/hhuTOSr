use alloc::boxed::Box;
use core::arch::asm;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::AtomicBool;
use crate::kernel::cpu;
use crate::kernel::threads::scheduler::get_scheduler;
use crate::kernel::threads::thread::Thread;
use crate::library::queue::LinkedQueue;
use crate::library::spinlock::Spinlock;

/// A more sophisticated lock implementation than `Spinlock`, that blocks waiting threads
/// when the lock is already held. This improves performance, as no time is wasted by threads
/// spinning in a loop while waiting for the lock to be released.
pub struct Mutex<T> {
    /// The lock is represented by an atomic boolean that indicates whether the lock is held.
    lock: AtomicBool,
    /// The data protected by the mutex, stored in an `UnsafeCell` to allow mutable access.
    /// See `Spinlock` for more details on why we use `UnsafeCell`.
    data: UnsafeCell<T>,
    /// A queue of threads waiting for the lock to be released.
    wait_queue: Spinlock<LinkedQueue<Box<Thread>>>
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}
unsafe impl<T> Send for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            wait_queue: Spinlock::new(LinkedQueue::new())
        }
    }
    
    /// Try to acquire the lock once without blocking.
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {

        /* Hier muss Code eingefuegt werden */

        None
    }

    /// Acquire the lock, blocking if necessary until it is available.
    /// This method will dequeue the current thread from the scheduler if the lock is already held
    /// and store it in the `wait_queue`.
    /// Once the lock is available, the next thread in the `wait_queue` will be woken up
    /// so it can try to acquire the lock again.
    pub fn lock(&self) -> MutexGuard<T> {

        /* Hier muss Code eingefuegt werden */

        MutexGuard { lock: self }
    }
    
    /// Check if the lock is currently held.
    pub fn is_locked(&self) -> bool {

        /* Hier muss Code eingefuegt werden */
        
        false
    }

    /// Check if the wait queue is currently locked.
    pub fn is_queue_locked(&self) -> bool {
        self.wait_queue.is_locked()
    }

    /// Unlock the mutex, allowing other threads to acquire it.
    /// If there are threads waiting for the lock, the next thread in the wait queue is woken up.
    pub fn unlock(&self) {

        /* Hier muss Code eingefuegt werden */

    }
    
    /// Forcefully unlock the mutex without waking up any waiting threads.
    /// This should only be used in exceptional cases.
    pub unsafe fn force_unlock(&self) {

        /* Hier muss Code eingefuegt werden */

    }
}

/// A guard that provides access to the data protected by the mutex.
/// It implements `Deref` and `DerefMut` to allow transparent access to the data.
/// It also implements `Drop` to automatically unlock the mutex when it goes out of scope.
pub struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.lock.data.get().as_ref().unwrap()
        }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.lock.data.get().as_mut().unwrap()
        }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}
