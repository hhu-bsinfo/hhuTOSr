use core::fmt;

#[derive(Debug)]
pub enum VmaType {
    Code,
    Heap,
    Stack,
}

/// Virtual Memory Area (VMA)
pub struct VMA {
    start: u64,
    end: u64,
    typ: VmaType,
}

impl VMA {
    /// Create a new VMA with a start and end address and a given type.
    pub fn new(start: u64, end: u64, typ: VmaType) -> Self {
        VMA { start, end, typ }
    }

    /// Check if this VMA overlaps with another one.
    pub fn overlaps(&self, other: &VMA) -> bool {
        /*
         * Hier muss Code eingefuegt werden
         */
    }
}

impl fmt::Debug for VMA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VMA {{ start: 0x{:016x}, end: {:#016x}, type: {:?} }}", self.start, self.end, self.typ)
    }
}
