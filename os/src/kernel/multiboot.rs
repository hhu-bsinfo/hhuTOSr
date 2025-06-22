use core::ffi::{c_char, CStr};

#[repr(C, packed)]
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
struct MultibootSymbols {
    tabsize: u32,
    strsize: u32,
    addr: u32,
    reserved: u32,
}

#[repr(C, packed)]
struct VbeInfo {
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
}

#[repr(C, packed)]
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

#[repr(u8)]
pub enum FramebufferType {
    Indexed = 0,
    RGB = 1,
    Text = 2
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
}