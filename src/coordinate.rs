use std::fmt::{write, Debug};


#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct XYZ<T>{
    pub x:T,
    pub y:T,
    pub z:T
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct XY<T>{
    pub x:T,
    pub y:T
}

impl<T> Debug for XY<T> where T:Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xy{{ {:?}, {:?} }}", self.x, self.y)
    }
}

impl<T> Debug for XYZ<T> where T:Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xyz{{ {:?}, {:?}, {:?} }}", self.x, self.y, self.z)
    }
}