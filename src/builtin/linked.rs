//! Implementation of `MemoryReport` for `LinkedList`.



use crate::MemoryReport;
use alloc::collections::LinkedList;



impl<T: MemoryReport> MemoryReport for LinkedList<T> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.len() * T::direct()
    }

    fn children(&self) -> usize {
        if T::ALLOC || T::CHILD {
            self.iter().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        }
    }
}
