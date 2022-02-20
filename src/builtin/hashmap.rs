//! Implementation of `MemoryReport` for `HashMap`.



use crate::MemoryReport;
use std::collections::HashMap;



impl<K: MemoryReport, V: MemoryReport> MemoryReport for HashMap<K, V> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.capacity() * (K::direct() + V::direct())
    }

    fn children(&self) -> usize {
        let k = if K::ALLOC || K::CHILD {
            self.keys().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        };

        let v = if V::ALLOC || V::CHILD {
            self.values().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        };

        k + v
    }
}
