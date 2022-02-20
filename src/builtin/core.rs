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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct() {
        assert!(core::mem::size_of::<u8>()   == u8::direct());
        assert!(core::mem::size_of::<u16>()  == u16::direct());
        assert!(core::mem::size_of::<u32>()  == u32::direct());
        assert!(core::mem::size_of::<u64>()  == u64::direct());
        assert!(core::mem::size_of::<u128>() == u128::direct());

        assert!(core::mem::size_of::<i8>()   == i8::direct());
        assert!(core::mem::size_of::<i16>()  == i16::direct());
        assert!(core::mem::size_of::<i32>()  == i32::direct());
        assert!(core::mem::size_of::<i64>()  == i64::direct());
        assert!(core::mem::size_of::<i128>() == i128::direct());

        assert!(core::mem::size_of::<isize>() == isize::direct());
        assert!(core::mem::size_of::<usize>() == usize::direct());

        assert!(core::mem::size_of::<f32>() == f32::direct());
        assert!(core::mem::size_of::<f64>() == f64::direct());

        assert!(core::mem::size_of::<char>() == char::direct());
        assert!(core::mem::size_of::<bool>() == bool::direct());
    }

    #[test]
    fn indirect() {
        assert!(0 == 0u8.indirect());
        assert!(0 == 0u16.indirect());
        assert!(0 == 0u32.indirect());
        assert!(0 == 0u64.indirect());
        assert!(0 == 0u128.indirect());

        assert!(0 == 0i8.indirect());
        assert!(0 == 0i16.indirect());
        assert!(0 == 0i32.indirect());
        assert!(0 == 0i64.indirect());
        assert!(0 == 0i128.indirect());

        assert!(0 == 0isize.indirect());
        assert!(0 == 0usize.indirect());

        assert!(0 == 0.0f32.indirect());
        assert!(0 == 0.0f64.indirect());

        assert!(0 == 'a'.indirect());
        assert!(0 == true.indirect());
    }

    #[test]
    fn children() {
        assert!(0 == 0u8.children());
        assert!(0 == 0u16.children());
        assert!(0 == 0u32.children());
        assert!(0 == 0u64.children());
        assert!(0 == 0u128.children());

        assert!(0 == 0i8.children());
        assert!(0 == 0i16.children());
        assert!(0 == 0i32.children());
        assert!(0 == 0i64.children());
        assert!(0 == 0i128.children());

        assert!(0 == 0isize.children());
        assert!(0 == 0usize.children());

        assert!(0 == 0.0f32.children());
        assert!(0 == 0.0f64.children());

        assert!(0 == 'a'.children());
        assert!(0 == true.children());
    }
}
