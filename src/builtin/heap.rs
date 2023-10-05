//! Implementation of `MemoryReport` for `BinaryHeap`.



use crate::MemoryReport;
use std::collections::BinaryHeap;



impl<T: MemoryReport> MemoryReport for BinaryHeap<T> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.capacity() * T::direct()
    }

    fn children(&self) -> usize {
        if T::ALLOC || T::CHILD {
            self.iter().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        }
    }
}
