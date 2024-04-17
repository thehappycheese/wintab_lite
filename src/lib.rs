//! # Wintab Lite
//! 
//! This is an un-official port of the `wintab.h` headers to rust.
//! It is provided as is and without support or any sort of warranty express or implied etc etc.
//! 
//! The original wintab headers and documentation are marked with a copyright notice by Wacom.
//! 
//! See [here](https://github.com/Wacom-Developer/wacom-device-kit-windows/blob/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/Wintab/WINTAB.H#L1C1-L10C81) and [here](https://developer-docs.wacom.com/docs/icbt/windows/wintab/wintab-reference/)
//! However the example code provided by Wacom is MIT licensed [here](https://github.com/Wacom-Developer/wacom-device-kit-windows/blob/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/MIT-license.txt)
//! 
//! For example usage please read / run the example `cargo run --example wininit_libloading`
mod c_type_aliases;
mod log_context;
mod axis;
mod wtpkt;
mod packet;
mod extern_function_types;
mod coordinate;
mod information_categories;
mod c_string_types;
mod bitmask;
mod window_message;

pub use c_type_aliases::{
    BOOL,
    DWORD,
    FIX32,
    HCTX,
    INT,
    UINT,
    LONG,
};
pub use c_string_types::CString40;
pub use bitmask::Bitmask;
pub use coordinate::{XY, XYZ};
pub use axis::AXIS;
pub use extern_function_types::{
    WTInfo,
    WTOpen,
    WTClose,
    WTPacket,
    WTQueuePacketsEx,
    WTDataGet,
    WTPacketsGet
};
pub use log_context::{
    LOGCONTEXT,
    CXO,
    CXL,
    CXS,
};
pub use wtpkt::WTPKT;
pub use packet::{
    Packet,
    ButtonChange,
    ButtonChangeType
};
pub use information_categories::{
    WTI,
    DVC,
    CRC,
    CTX,
    CSR,
    EXT,
    HWC,
    IFC,
    STA,
};
pub mod WT{
    pub use crate::window_message::*;
}


/// Takes a mutable reference to the input then casts it to a void pointer:
/// The result can be interpreted as [LPVOID] or `*mut std::ffi::c_void`.
#[macro_export]
macro_rules! cast_void{
    ($e:expr) => {
        &mut $e as *mut _ as *mut std::ffi::c_void
    };
}
