//! Implementation of `MemoryReport` for all basic types.



use crate::MemoryReport;



impl MemoryReport for i8   {}
impl MemoryReport for i16  {}
impl MemoryReport for i32  {}
impl MemoryReport for i64  {}
impl MemoryReport for i128 {}

impl MemoryReport for u8   {}
impl MemoryReport for u16  {}
impl MemoryReport for u32  {}
impl MemoryReport for u64  {}
impl MemoryReport for u128 {}

impl MemoryReport for isize {}
impl MemoryReport for usize {}

impl MemoryReport for f32 {}
impl MemoryReport for f64 {}

impl MemoryReport for bool {}
impl MemoryReport for char {}
