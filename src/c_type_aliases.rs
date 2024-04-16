use std::ffi::{
    c_ulong,
    c_uint,
    c_int,
    c_long,
};

/// = std::ffi::c_long ≈ i32
pub type LONG = c_long;
/// = std::ffi::c_long ≈ u32
pub type DWORD = c_ulong;

/// = std::ffi::u_int ≈ u32
pub type UINT = c_uint;
/// = std::ffi::c_int ≈ i32
/// (Note the original wintab spec just uses `int` directly. I create this alias for consistency in this crate)
pub type INT = c_int;

/// = std::ffi::c_int ≈ i32
pub type BOOL = c_int;


/// This is not meant to be instantiated, we only ever use the pointer to this type
#[repr(C)]
pub struct HWND(isize);

/// This is not meant to be instantiated, we only ever use the pointer to this type
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct HCTX (std::ffi::c_int);


/// A 32-bit fixed-point arithmetic type, with the radix point between the two words.
/// Thus, the type contains 16 bits to the left of the radix point and 16 bits to the right of it.
/// 
/// This struct dereferences into an `f64` for actual usage,
/// no methods for actual fixed point arithmetic are actually provided.
/// 
/// > Note: this type makes the assumption that a [DWORD] translates to a [u32] on your system.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FIX32(DWORD);

impl FIX32 {
    /// Create a [FIX32] from a [DWORD]
    pub fn new(value: DWORD) -> Self {
        FIX32(value)
    }

    /// Extracts the integer part of the fixed-point number. (high word)
    pub fn int_part(self) -> c_uint {
        self.0 >> 16
    }

    /// Extracts the fractional part of the fixed-point number. (low word)
    pub fn frac_part(self) -> c_uint {
        self.0 & 0xFFFF
    }
}
impl std::fmt::Debug for FIX32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:}fix32", self)
    }
}
impl std::fmt::Display for FIX32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let as_float:f64 = (*self).into();
        write!(f, "{:}", as_float)
    }
}

/// TODO: untested
impl From<f64> for FIX32 {
    fn from(value: f64) -> Self {
        let fixed_val = (value * 65536.0).round() as c_uint;
        FIX32::new(fixed_val)
    }
}

/// TODO: untested
impl Into<f64> for FIX32 {
    fn into(self) -> f64 {
        (self.0 as f64) / 65536.0
    }
}
