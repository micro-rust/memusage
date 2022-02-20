//! Implementation of `MemoryReport` for `String`.



use crate::MemoryReport;



impl MemoryReport for String {
    const ALLOC: bool = true;

    fn indirect(&self) -> usize {
        self.capacity()
    }

    #[inline]
    fn children(&self) -> usize {
        0
    }
}

impl MemoryReport for &str {
    const ALLOC: bool = true;

    fn indirect(&self) -> usize {
        self.len()
    }

    #[inline]
    fn children(&self) -> usize {
        0
    }
}
