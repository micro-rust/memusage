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
