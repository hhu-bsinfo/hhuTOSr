use core::ffi::{c_char, c_void, CStr};
use spin::Once;
use tar_no_std::TarArchiveRef;
use crate::consts::PAGE_FRAME_SIZE;
use crate::kernel::paging::frames::{PfListAllocator, PhysAddr, FRAME_ALLOCATOR};

unsafe extern "C" {
    static ___KERNEL_DATA_START__: c_void; // Start address of OS image
    static ___KERNEL_DATA_END__: c_void; // End address of OS image
}

pub static MULTIBOOT_INFO: Once<MultibootInfo> = Once::new();

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MultibootInfo {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: MultibootSymbols,
    mmap_length: u32,
    mmap_addr: u32,
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    boot_loader_name: u32,
    apm_table: u32,
    vbe_info: VbeInfo,
    framebuffer_info: FramebufferInfo,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum MultibootFlag {
    MemInfoAvailable = 1 << 0,
    BootDeviceInfoAvailable = 1 << 1,
    CommandLineAvailable = 1 << 2,
    ModulesAvailable = 1 << 3,
    SymbolsAvailable = 1 << 4,
    MemoryMapAvailable = 1 << 6,
    DriveInfoAvailable = 1 << 7,
    ConfigTableAvailable = 1 << 8,
    BootLoaderNameAvailable = 1 << 9,
    APMTableAvailable = 1 << 10,
    VBEInfoAvailable = 1 << 11,
    FramebufferInfoAvailable = 1 << 12,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct MultibootSymbols {
    tabsize: u32,
    strsize: u32,
    addr: u32,
    reserved: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct VbeInfo {
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub typ: FramebufferType,
    pub color_info: ColorInfo
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ColorInfo {
    rgb: ColorMask,
    palette: ColorPalette,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ColorPalette {
    palette_addr: u32,
    palette_length: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ColorMask {
    red_position: u8,
    red_mask_size: u8,
    green_position: u8,
    green_mask_size: u8,
    blue_position: u8,
    blue_mask_size: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct MemoryMapEntry {
    size: u32,
    addr: u64,
    len: u64,
    typ: MemoryType
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum MemoryType {
    Available = 1,
    Reserved = 2,
    Other
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FramebufferType {
    Indexed = 0,
    RGB = 1,
    Text = 2
}

#[derive(Debug, Copy, Clone)]
struct ModuleEntry {
    start: u32,
    end: u32,
    cmdline: u32,
    reserved: u32,
}

impl MultibootInfo {
    pub fn get_bootloader_name(&self) -> Option<&str> {
        if self.flags & (MultibootFlag::BootLoaderNameAvailable as u32) != 0 {
            unsafe {
                let name = CStr::from_ptr(self.boot_loader_name as *const c_char);
                Some (name.to_str().unwrap())
            }
        } else {
            None
        }
    }
    
    pub fn get_command_line(&self) -> Option<&str> {
        if self.flags & (MultibootFlag::CommandLineAvailable as u32) != 0 {
            unsafe {
                let cmdline = CStr::from_ptr(self.cmdline as *const c_char);
                Some(cmdline.to_str().unwrap())
            }
        } else {
            None
        }
    }
    
    pub fn get_framebuffer_info(&self) -> Option<&FramebufferInfo> {
        if self.flags & (MultibootFlag::FramebufferInfoAvailable as u32) != 0 {
            Some(&self.framebuffer_info)
        } else {
            None
        }
    }

    fn get_initrd(&self) -> Option<&'static [u8]> {
        if self.flags & (MultibootFlag::ModulesAvailable as u32) != 0 {
            if self.mods_count == 0 {
                return None;
            }
            if self.mods_count > 1 {
                panic!("Only one multiboot module (initrd) is supported!");
            }

            let module = unsafe { &*((self.mods_addr as *const u8) as *const ModuleEntry) };

            unsafe {
                return Some(core::slice::from_raw_parts(module.start as *const u8, (module.end - module.start) as usize));
            }
        }

        None
    }

    pub fn get_initrd_archive(&self) -> Option<TarArchiveRef<'_>> {
        match self.get_initrd() {
            Some(initrd) => {
                let start = initrd.as_ptr() as u32;
                let end = start + initrd.len() as u32;
                kprintln!("Found initrd module at 0x{:08x} - 0x{:08x}", start, end);

                Some(TarArchiveRef::new(initrd).expect("Failed to parse initrd archive"))
            },
            None => None,
        }
    }

    pub fn init_phys_memory_allocator(&self) {
        // End address of kernel image (must not be inserted into physical memory allocator!)
        let kernel_end = unsafe { &___KERNEL_DATA_END__ as *const c_void as u64 };
        // Initrd might also occupy physical memory that must not be used
        let initrd = self.get_initrd();

        // Access to the physical memory allocator
        let mut allocator = FRAME_ALLOCATOR.lock();

        if self.flags & (MultibootFlag::MemoryMapAvailable as u32) != 0 {
            // Start address of the memory map
            let mmap_addr = self.mmap_addr as *const u8;
            // Offset into the memory map, pointing to the current entry
            let mut offset = 0;

            // Loop over each entry in the memory map
            while offset < self.mmap_length {
                // mmap_addr + offset = pointer to current entry
                let entry_ptr = unsafe { mmap_addr.add(offset as usize) as *const MemoryMapEntry };
                let entry = unsafe { &*entry_ptr };
                let typ = entry.typ;

                // entry.size is the size of the entry struct itself -> Adding it to offset lets offset point to the next entry
                offset += size_of::<u32>() as u32 + entry.size;

                // Check if the current entry describes an available block of physical memory
                if typ != MemoryType::Available {
                    continue;
                }

                let mut start = entry.addr;
                let end = entry.addr + entry.len;

                // Leave kernel memory free
                if start < kernel_end {
                    start = kernel_end;
                    if start >= end {
                        continue;
                    }
                }

                // Leave initrd memory free
                if let Some(initrd) = initrd {
                    let initrd_start = initrd.as_ptr() as u64;
                    let initrd_end = initrd_start + initrd.len() as u64;

                    if start < initrd_end && end > initrd_start {
                        if start < initrd_start {
                            // Memory block starts before initrd -> Insert block before initrd
                            Self::insert_memory_block(&mut allocator, start, initrd_start);
                        }

                        // Adjust start to the end of initrd
                        start = initrd_end;
                        if start >= end {
                            continue;
                        }
                    }
                }

                Self::insert_memory_block(&mut allocator, start, end);
            }
        }
    }

    fn insert_memory_block(allocator: &mut PfListAllocator, start: u64, end: u64) {
        let mut start = start;
        let mut end = end;

        // Align start and end address to 4096 byte
        if start % (PAGE_FRAME_SIZE as u64) != 0 {
            start = (start / (PAGE_FRAME_SIZE as u64) + 1) * (PAGE_FRAME_SIZE as u64);
        }
        if end % (PAGE_FRAME_SIZE as u64) != 0 {
            end = (end / (PAGE_FRAME_SIZE as u64)) * (PAGE_FRAME_SIZE as u64);
        }

        // Insert block into physical memory allocator
        let num_frames = ((end - start) / (PAGE_FRAME_SIZE as u64)) as usize;
        if num_frames > 0 {
            kprintln!("Inserting physical memory block (Addr: 0x{:016x}, Size: {} frames)", start, num_frames);
            unsafe {
                allocator.free_block(PhysAddr::new(start), num_frames);
            }
        }
    }
}