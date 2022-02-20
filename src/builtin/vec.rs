//! Implementation of `MemoryReport` for `Vec`.



use crate::MemoryReport;



impl<T: MemoryReport> MemoryReport for Vec<T> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.capacity() * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct() {
        assert!((core::mem::size_of::<usize>() * 3) == Vec::<u64>::direct());
    }

    #[test]
    fn indirect() {
        assert!((core::mem::size_of::<usize>() * 4) == Vec::<u64>::with_capacity(4).indirect());
    }

    #[test]
    fn children() {
        assert!(0 == vec![0usize, 1, 2, 3].children());
    }
}
