/*****************************************************************************
 *                                                                           *
 *                            A L L O C A T O R                              *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Schnittstelle und Funktionen für die Heap-Verwaltung.    *
 *                  Die Implementierung des freien Speichers erfolgt in      *
 *                  bump.rs bzw. list.rs
 *                                                                           *
 * Memory-Laylout                                                            *
 *                  System-Code                                              *
 *                    0x100000:	GRUB kann nur an Adressen >=1M laden         *
 *           Globale Variablen: Direkt nach dem Code liegen die globalen     *
 *                              Variablen.                                   *
 *                        Heap:                                              *
 *                    0x500000:	Start-Adresse der Heap-Verwaltung            *
 *                    0x600000: Letzte Adresse des Heaps                     *
 *                                                                           *
 * Achtung:         Benötigt einen PC mit mindestens 8 MB RAM!               *
 *                                                                           *
 * Autor:           Philipp Oppermann                                        *
 *                  https://os.phil-opp.com/allocator-designs/               *
 *****************************************************************************/

use crate::kernel::allocator::bump::BumpAllocator;
use crate::kernel::allocator::list::LinkedListAllocator;
use alloc::alloc::Layout;

pub mod bump;
pub mod list;

pub const HEAP_START: usize = 0x500000;
pub const HEAP_SIZE: usize = 1024 * 1024;        // 1 MB


#[global_allocator]
//static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

// init allocator (to be called very early)
pub fn init() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

// low-level alloc for system memory, e.g. stack
pub fn alloc(layout: Layout) -> *mut u8 {
    unsafe {
        ALLOCATOR.lock().alloc(layout)
    }
}

// low-level alloc for system memory, e.g. stack
pub fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe {
        ALLOCATOR.lock().dealloc(ptr, layout)
    }
}

// dump free list (for debugging)
#[allow(unused_unsafe)]
pub fn dump_free_list() {
    unsafe {
        ALLOCATOR.lock().dump_free_list();
    }
}

// A wrapper around spin::Mutex to permit trait implementations
// Required for implementing 'GlobalAlloc' in bump.rs and list.rs
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

// Helper function called by 'alloc' in the 'GlobalAlloc' in
// bump.rs and list.rs
// Align the given address `addr` upwards to alignment `align`.
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr // addr already aligned
    } else {
        addr - remainder + align
    }
}
