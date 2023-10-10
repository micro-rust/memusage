//! Implementation of `MemoryReport` for pointer and reference types.



use crate::MemoryReport;



impl<T: MemoryReport> MemoryReport for &T {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        T::direct()
    }

    fn children(&self) -> usize {
        if T::ALLOC || T::CHILD {
            let target: &T = self;

            target.indirect() + target.children()
        } else {
            0
        }
    }
}
