/// Wait a very short amount of time (1-4 us) by writing a byte to an unused port (0x80).
/// This is not a reliable way to wait, but works for some cases (e.g. PIC initialization).
/// See OSDev for more details: https://wiki.osdev.org/Inline_Assembly/Examples#I/O_access.
#[inline]
pub fn io_wait() {
    unsafe {
        asm!(
            "out 0x80, al",
             in("al") 0u8,
        );
    }
}
