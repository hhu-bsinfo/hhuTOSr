/*****************************************************************************
 *                                                                           *
 *                  S P I N L O C K                                          *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Implementierung eines einfachen Spinlocks, basierend auf *
 *                  busing-polling mithilfe von 'compare_exchange'. Der      *
 *                  SpinLock sperrt nicht die Interrupts.                    *
 *                                                                           *
 * Autor:           Stefan Lankes, RWTH Aachen University                    *
 *                  Angepasst von Michael Schoettner, HHU, 10.6.2023         *
 *****************************************************************************/
use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::kernel::cpu;

pub struct Spinlock<T: ?Sized> {
	lock: AtomicUsize,
	data: UnsafeCell<T>,
}

// A guard to which the protected data can be accessed
// When the guard falls out of scope it will release the lock.
pub struct SpinlockGuard<'a, T: ?Sized + 'a> {
	lock: &'a AtomicUsize,
	data: &'a mut T,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T: ?Sized + Send> Sync for Spinlock<T> {}
unsafe impl<T: ?Sized + Send> Send for Spinlock<T> {}

impl<T> Spinlock<T> {
	pub const fn new(user_data: T) -> Spinlock<T> {
		Spinlock {
			lock: AtomicUsize::new(0),
			data: UnsafeCell::new(user_data),
		}
	}

	// Consumes this mutex, returning the underlying data.
	pub fn into_inner(self) -> T {
		// We know statically that there are no outstanding references to
		// `self` so there's no need to lock.
		let Spinlock { data, .. } = self;
		data.into_inner()
	}
}

impl<T: ?Sized> Spinlock<T> {

	fn obtain_lock(&self) {
		// try contiguously to set lock
		loop {
		   let result = self.lock.compare_exchange(0, 1, Ordering::SeqCst,  Ordering::SeqCst);
		   if result.is_ok() == true {
		      break;
		   }
	       cpu::pause();
        }
	}

	pub fn lock(&self) -> SpinlockGuard<T> {
		self.obtain_lock();
		SpinlockGuard {
			lock: &self.lock,
			data: unsafe { &mut *self.data.get() },
		}
	}
}

impl<T: ?Sized + Default> Default for Spinlock<T> {
	fn default() -> Spinlock<T> {
		Spinlock::new(Default::default())
	}
}

impl<'a, T: ?Sized> Deref for SpinlockGuard<'a, T> {
	type Target = T;
	fn deref<'b>(&'b self) -> &'b T {
		&*self.data
	}
}

impl<'a, T: ?Sized> DerefMut for SpinlockGuard<'a, T> {
	fn deref_mut<'b>(&'b mut self) -> &'b mut T {
		&mut *self.data
	}
}

impl<'a, T: ?Sized> Drop for SpinlockGuard<'a, T> {

	// The dropping of the SpinlockGuard will release the lock it was created from.
	fn drop(&mut self) {
		self.lock.store(0, Ordering::SeqCst);
	}
}
