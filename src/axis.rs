use crate::{
    LONG,
    FIX32
};

/// Describes range and resolution for many of the packet data items.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct AXIS{
    /// Minimum value of the data item in the tablet's native coordinates.
    pub axMin        : LONG,
    /// Maximum value of the data item in the tablet's native coordinates.
    pub axMax        : LONG,
    /// Indicates the units used in calculating the resolution for the data item.
    pub axUnits      : TU, //UINT,
    /// Is a fixed-point number giving the number of data item increments per physical unit.
    pub axResolution : FIX32,
}

/// Physical Unit Specifiers
#[repr(u32)] // UINT ≈ std::ffi::u_int ≈ u32
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TU {
    /// Specifies that no resolution in terms of physical units is given.
    #[default]
    NONE        = 0,
    /// Specifies that resolution is given with respect to inches.
    INCHES      = 1,
    /// Specifies that resolution is given with respect to centimetres.
    CENTIMETERS = 2,
    /// Specifies that resolution is given with respect to one full revolution of arc. For example, if a data item returns degrees, the resolution would be 360 and the units would be TU_CIRCLE. If the item were in radians, the resolution would be 6.28318 (to FIX32’s precision) and the units would be TU_CIRCLE.
    CIRCLE      = 3,
}