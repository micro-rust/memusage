//! Implementation of `MemoryReport` for `&[T]`.



use crate::MemoryReport;



impl<T: MemoryReport> MemoryReport for &[T] {
    const ALLOC: bool = false;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.len() * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}

impl<T: MemoryReport, const N: usize> MemoryReport for &[T; N] {
    const ALLOC: bool = false;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        N * T::direct()
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
        assert!((core::mem::size_of::<usize>() * 2) == <&[u8]>::direct());
        assert!((core::mem::size_of::<usize>()    ) == <&[u8; 2]>::direct());
    }

    #[test]
    fn indirect() {
        let array = [0u8, 1, 2, 3];
        assert!((core::mem::size_of::<u8>() * 4) == (&array).indirect());
        assert!((core::mem::size_of::<u8>() * 4) == (&array[..]).indirect());
    }

    #[test]
    fn children() {
        let array = [0u8, 1, 2, 3];

        assert!(0 == (&array).children());
        assert!(0 == (&array[..]).children());
    }
}
