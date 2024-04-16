use std::{default, fmt::Binary, ops::Deref};



/// This is a wrapper for integer types that basically just changes
/// the debug display to show as a binary string of 1s and 0s
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitmask<T:std::fmt::Binary>(pub T);


impl<T:Default+Binary> Default for Bitmask<T>{
    fn default() -> Self {
        Bitmask(T::default())
    }
}

impl<T:std::fmt::Binary> std::fmt::Display for Bitmask<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bitsize = std::mem::size_of::<T>() * 8;
        let binary_string = format!("{:0b}", self.0);
        let binary_string_with_padding = format!("{:0>width$}", binary_string, width = bitsize);
        write!(f, "{}", binary_string_with_padding)
    }
}

impl <T:std::fmt::Binary> std::fmt::Debug for Bitmask<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bitsize = std::mem::size_of::<T>() * 8;
        let binary_string = format!("{:0b}", self.0);
        let binary_string_with_padding = format!("0b{:0>width$}", binary_string, width = bitsize);
        write!(f, "{}", binary_string_with_padding)
    }
}

impl<T:std::fmt::Binary> Deref for Bitmask<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}