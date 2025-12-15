use tar_no_std::TarArchiveRef;

#[derive(Debug, Copy, Clone)]
struct ModuleEntry {
    start: u32,
    end: u32,
    cmdline: u32,
    reserved: u32,
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
