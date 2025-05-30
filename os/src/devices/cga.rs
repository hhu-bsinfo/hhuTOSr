impl Write for CGA {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.print_byte(byte),

                // not part of printable ASCII range
                _ => self.print_byte(0xfe),
            }
        }

        Ok(())
    }
}
