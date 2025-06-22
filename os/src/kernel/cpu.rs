pub struct IoPort {
    port: u16
}

impl IoPort {
    /// Create a new IoPort object
    pub const fn new(port: u16) -> IoPort {
        IoPort { port }
    }


    /// Write a single byte to a port
    #[inline]
    pub unsafe fn outb(&mut self, data: u8) {
        unsafe {
            asm!(
            "out dx, al",
            in("dx") self.port,
            in("al") data,
            );
        }
    }

    /// Write a word (two bytes) to a port
    #[inline]
    pub unsafe fn outw(&mut self, data: u16) {
        unsafe {
            asm!(
            "out dx, ax",
            in("dx") self.port,
            in("ax") data,
            );
        }
    }

    /// Write a double word (four bytes) to a port
    #[inline]
    pub unsafe fn outdw(&mut self, data: u32) {
        unsafe {
            asm!(
            "out dx, eax",
            in("dx") self.port,
            in("eax") data,
            );
        }
    }

    /// Read a single byte from a port
    #[inline]
    pub unsafe fn inb(&mut self) -> u8 {
        let ret: u8;
        unsafe {
            asm!(
            "in al, dx",
            in("dx") self.port,
            out("al") ret,
            );
        }
        ret
    }

    /// Read a word (two bytes) from a port
    #[inline]
    pub unsafe fn inw(&mut self) -> u16 {
        let ret: u16;
        unsafe {
            asm!(
            "in ax, dx",
            in("dx") self.port,
            out("ax") ret,
            );
        }
        ret
    }

    /// Read a double word (four bytes) from a port
    #[inline]
    pub unsafe fn indw(&mut self) -> u32 {
        let ret: u32;
        unsafe {
            asm!(
            "in eax, dx",
            in("dx") self.port,
            out("eax") ret,
            );
        }
        ret
    }
}
