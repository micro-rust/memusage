//! Implementation of `MemoryReport` for `HashSet`.
//! The current `HashSet` implementation uses `HashMap<T, ()>` under the hood.
//! This makes it a bit difficult to estimate the actual size used due to the
//! possible optimizations that the compiler may have applied to the `()` type.
//! The current implementation tries it's best to estimate it's size but does
//! not guarantee to be exact.



use crate::MemoryReport;
use std::collections::HashSet;



impl<T: MemoryReport> MemoryReport for HashSet<T> {
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
