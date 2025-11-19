use core::ptr;
use crate::consts::{PAGE_SIZE, STACK_SIZE, USER_STACK_VIRT_END, USER_STACK_VIRT_START};
use crate::kernel::paging::frames::{PhysAddr, FRAME_ALLOCATOR};

const PAGE_TABLE_ENTRIES: usize = 512;

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct PageFlags: u64 {
        const PRESENT = 1 << 0;
        const WRITEABLE = 1 << 1;
        const USER = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const CACHE_DISABLE = 1 << 4;
        const ACCESSED = 1 << 5;
        const DIRTY = 1 << 6;
        const HUGE_PAGE = 1 << 7;
        const GLOBAL = 1 << 8;
    }
}

impl PageFlags {
    fn kernel_flags() -> Self {
        /*
         * Hier muss Code eingefuegt werden
         */
    }

    fn user_flags() -> Self {
        /*
         * Hier muss Code eingefuegt werden
         */
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    fn new(addr: PhysAddr, flags: PageFlags) -> Self {
        let addr: u64 = addr.into();
        Self(addr | flags.bits())
    }

    pub fn set(&mut self, addr: PhysAddr, flags: PageFlags) {
        *self = PageTableEntry::new(addr, flags);
    }

    pub fn get_flags(&self) -> PageFlags {
        PageFlags::from_bits_truncate(self.0)
    }

    pub fn set_flags(&mut self, flags: PageFlags) {
        *self = PageTableEntry::new(self.get_addr(), flags);
    }

    pub fn get_addr(&self) -> PhysAddr {
        PhysAddr::new(self.0 & 0x000f_ffff_ffff_f000)
    }

    pub fn set_addr(&mut self, addr: PhysAddr) {
        *self = PageTableEntry::new(addr, self.get_flags());
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[addr={:?}, flags={:?}]",
            self.get_addr(),
            self.get_flags()
        )
    }
}

#[repr(transparent)]
pub struct PageTable {
    entries: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

impl PageTable {
    /// Set up a mapping from `virt_addr` to `num_pages` pages at the given `level`.
    /// If `kernel` is true, the pages will be mapped 1:1 to their physical addresses
    /// (virt_addr == phys_addr). Otherwise, new physical frames will be allocated
    /// for the mapping, using the frame allocator.
    fn map(&mut self, virt_addr: u64, num_pages: usize, kernel: bool) -> usize {
        /*
         * Hier muss Code eingefuegt werden
         */
    }
}

pub fn read_cr3() -> &'static mut PageTable {
    let value: u64;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) value);
    }

    unsafe {
        PhysAddr::new(value & 0xffff_ffff_ffff_f000)
            .as_mut_ptr::<PageTable>()
            .as_mut()
            .unwrap()
    }
}

pub unsafe fn write_cr3(pml4: &PageTable) {
    let addr: u64 = ptr::from_ref(pml4) as u64;
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) addr);
    }
}

pub fn init_kernel_tables() -> &'static mut PageTable {
    let max_phys_addr = FRAME_ALLOCATOR.lock().get_max_phys_addr();
    let num_pages = (max_phys_addr.raw() as usize + PAGE_SIZE - 1) / PAGE_SIZE;

    unsafe {
        let pml4 = FRAME_ALLOCATOR.lock()
                .alloc_block(1)
                .expect("Failed to allocate frame for PML4!")
                .as_mut_ptr::<PageTable>()
                .as_mut()
                .unwrap();

        pml4.map(0, num_pages, true);
        pml4
    }
}

pub unsafe fn map_user_stack(pml4_table: &mut PageTable) -> *mut u8 {
    /*
     * Hier muss Code eingefuegt werden
     */
}