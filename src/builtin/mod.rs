//! Implementation for `core` and `std` structures.


#[cfg(feature = "alloc")]
mod btree;
#[cfg(feature = "alloc")]
mod hashmap;
#[cfg(feature = "alloc")]
mod hashset;
#[cfg(feature = "alloc")]
mod heap;
#[cfg(feature = "alloc")]
mod linked;
#[cfg(feature = "alloc")]
mod string;
#[cfg(feature = "alloc")]
mod vec;

mod core;
mod slice;
mod pointer;
