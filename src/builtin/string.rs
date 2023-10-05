//! Implementation of `MemoryReport` for `String`.



use crate::MemoryReport;
use alloc::string::String;



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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct() {
        assert!((core::mem::size_of::<usize>() * 3) == String::direct());
    }

    #[test]
    fn indirect() {
        assert!((core::mem::size_of::<u8>() * 4) == String::with_capacity(4).indirect());
    }

    #[test]
    fn children() {
        assert!(0 == String::from("TEST").children());
    }
}
