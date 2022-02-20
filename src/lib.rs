//! Memusage is a simple dependency for memory usage autoreporting.
//! It contains a single trait an default implementations for `core` and `std`.



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
