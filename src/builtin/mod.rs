//! Implementation for `core` and `std` structures.



#[cfg(feature = "std")]
mod hashmap;
#[cfg(feature = "std")]
mod hashset;



#[cfg(feature = "alloc")]
mod btree;
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
