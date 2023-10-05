//! Memusage is a simple dependency for memory usage autoreporting.
//! It contains a single trait an default implementations for `core` and `std`.
//! To use this crate simply import the `MemoryReport` trait. This gives access
//! to 3 methods `T::direct()`, `T::indirect(&self)` and `T::children(&self)`.
//! The total memory usage of a `struct` will be the sum of the 3 measurements.
//! 
//! All measurements for allocating structs (e.g. `HashMap`) are estimations.
//! This is due to hidden fields in these structs, compiler optmimizations and
//! other miscellaneous effects.



#![no_std]



#[cfg(feature = "alloc")]
extern crate alloc;



mod builtin;



pub trait MemoryReport: Sized {
    /// A constant that indicates whether this object allocates memory other
    /// than the innate memory.
    const ALLOC: bool = false;

    /// A constant that indicates whether this object has children.
    const CHILD: bool = false;

    /// Reports the memory innate to the struct.
    /// This is equivalent to `core::mem::size_of::<Self>()`.
    #[inline]
    fn direct() -> usize {
        core::mem::size_of::<Self>()
    }

    /// Reports the memory directly allocated by this struct.
    /// For example, `Vec` returns its capacity times the size of the structs.
    #[inline]
    fn indirect(&self) -> usize {
        0
    }

    /// Reports the indirect and children memory of the children of the struct,
    /// if there are any. This returns non zero only if the children of the
    /// struct allocate memory.
    /// For example `Vec<Vec<T>>` would return a non zero value but a
    /// `Vec<usize>` would return 0.
    #[inline]
    fn children(&self) -> usize {
        0
    }
}
