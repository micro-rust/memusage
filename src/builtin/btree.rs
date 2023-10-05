//! Implementation of `MemoryReport` for `BTreeMap` and `BTreeSet`.



use crate::MemoryReport;
use std::collections::{
    BTreeMap, BTreeSet,
};



impl<K: MemoryReport, V: MemoryReport> MemoryReport for BTreeMap<K, V> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.len() * (K::direct() + V::direct())
    }

    fn children(&self) -> usize {
        let k = if K::ALLOC || K::CHILD {
            self.keys().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        };

        let v = if K::ALLOC || K::CHILD {
            self.values().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        };

        k + v
    }
}



impl<T: MemoryReport> MemoryReport for BTreeSet<T> {
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
