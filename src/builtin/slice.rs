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
