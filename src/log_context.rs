#![allow(non_snake_case)]
use bitflags::bitflags;
use crate::{Bitmask, CString40};
use super::c_type_aliases::*;
use super::wtpkt::WTPKT;
use super::coordinate::{
    XY,
    XYZ
};

/// The LOGCONTEXT data structure is used when opening and manipulating contexts. 
/// This structure contains everything applications and tablet managers need to know about a context.
/// To simplify context manipulations, applications may want to take advantage of the default context specification
/// available via the [WTInfoA](crate::WTInfo) function.
/// 
/// The move mask and button masks together determine what kinds of events will be processed by the context.
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOGCONTEXT {
    ///Contains a zero-terminated context name string.
    pub lcName: CString40,

    ///	Specifies options for the context. These options can be combined by using the bitwise OR operator. The
    /// `lcOptions` field can be any combination of the values defined. Specifying options that are unsupported in a
    /// particular implementation will cause `WTOpen` to fail.
    pub lcOptions: CXO,

    /// Specifies current status conditions for the context. These conditions can be combined by using the bitwise OR
    /// operator. The `lcStatus` field can be any combination of the values defined.
    pub lcStatus: CXS,

    /// Specifies which attributes of the context the application wishes to be locked. Lock conditions specify
    /// attributes of the context that cannot be changed once the context has been opened (calls to `WTConfig` will have
    /// no effect on the locked attributes). The lock conditions can be combined by using the bitwise OR operator. The
    /// `lcLocks` field can be any combination of the values defined. Locks can only be changed by the task or process
    /// that owns the context.
    pub lcLocks: CXL,

    /// The range of message numbers that will be used for reporting the activity of the context.
    pub lcMsgBase: UINT,

    /// The device whose input the context processes.
    pub lcDevice: UINT,

    /// The desired packet report rate in Hertz. Once the context is opened, this field will contain the
    /// actual report rate.
    pub lcPktRate: UINT,

    /// Specifies which optional data items will be in packets returned from the context. Requesting unsupported data
    /// items will cause `WTOpen` to fail.
    pub lcPktData: WTPKT,

    /// Specifies whether the packet data items will be returned in absolute or relative mode. If the item's bit is set
    /// in this field, the item will be returned in relative mode. Bits in this field for items not selected in the
    /// lcPktData field will be ignored. Bits for data items that only allow one mode (such as the serial number) will
    /// also be ignored.
    /// 
    /// Note that when all bits are clear (0) then all fields are in absolute mode.
    pub lcPktMode: WTPKT,

    /// Specifies which packet data items can generate move events in the context. Bits for items that are not part of
    /// the packet definition in the lcPktData field will be ignored. The bits for buttons, time stamp, and the serial
    /// number will also be ignored. In the case of overlapping contexts, movement events for data items not selected
    /// in this field may be processed by underlying contexts.
    pub lcMoveMask: WTPKT,

    /// The buttons for which button press events will be processed in the context. In the case of overlapping
    /// contexts, button press events for buttons that are not selected in this field may be processed by underlying
    /// contexts.
    pub lcBtnDnMask: Bitmask<DWORD>,

    /// The buttons for which button release events will be processed in the context. In the case of
    /// overlapping contexts, button release events for buttons that are not selected in this field may be processed by
    /// underlying contexts. If both press and release events are selected for a button
    /// (see the [LOGCONTEXT::lcBtnDnMask] field above), then the interface will cause the context to implicitly capture
    /// all tablet events while the button is down. In this case, events occurring outside the context will be clipped 
    /// to the context and processed as if they had occurred in the context. When the button is released, the context
    /// will receive the button release event, and then event processing will return to normal.
    pub lcBtnUpMask: Bitmask<DWORD>,

    /// The origin of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInOrgXYZ: XYZ<LONG>,

    /// The extent of the context's input area in the tablet's native coordinates.
    /// Will be clipped to the tablet native coordinate space when the context is opened or modified.
    pub lcInExtXYZ: XYZ<LONG>,

    /// The origin of the context's output area in context output coordinates.
    /// Used in coordinate scaling for absolute mode only.
    pub lcOutOrgXYZ: XYZ<LONG>,

    /// The extent of the context's output area in context output coordinates.
    /// Used in coordinate scaling for absolute mode only.
    pub lcOutExtXYZ: XYZ<LONG>,

    /// The relative-mode sensitivity factor.
    pub lcSensXYZ: XYZ<FIX32>,

    /// The system cursor tracking mode. Zero specifies absolute; non-zero means relative.
    pub lcSysMode: BOOL,

    /// The origin of the screen mapping area for system cursor tracking, in screen coordinates.
    pub lcSysOrgXY: XY<INT>,

    /// The extent of the screen mapping area for system cursor tracking, in screen coordinates.
    pub lcSysExtXY: XY<INT>,

    /// The system-cursor relative-mode sensitivity factors.
    pub lcSysSensXY: XY<FIX32>,
    
}
impl Default for LOGCONTEXT{
    fn default() -> Self {
        Self{
            lcName: CString40::default(),
            lcOptions: CXO::empty(),
            lcStatus: CXS::empty(),
            lcLocks: CXL::empty(),
            lcMsgBase: 0,
            lcDevice: 0,
            lcPktRate: 0,
            lcPktData:  WTPKT::empty(),
            lcPktMode:  WTPKT::empty(),
            lcMoveMask: WTPKT::empty(),
            lcBtnDnMask: Bitmask(0),
            lcBtnUpMask: Bitmask(0),
            lcInOrgXYZ: XYZ::default(),
            lcInExtXYZ: XYZ::default(),
            lcOutOrgXYZ: XYZ::default(),
            lcOutExtXYZ: XYZ::default(),
            lcSensXYZ: XYZ::default(),
            lcSysMode: 0,
            lcSysOrgXY: XY::default(),
            lcSysExtXY: XY::default(),
            lcSysSensXY: XY::default(),
        }
    }
}


bitflags! {
    /// See [LOGCONTEXT::lcOptions]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CXO:UINT {
        /// Specifies that the context is a system cursor context.
        const      SYSTEM = 0b0000000000000001;
        /// Specifies that the context is a Pen Windows context, if Pen Windows is installed.
        /// The context is also a system cursor context; specifying CXO_PEN implies CXO_SYSTEM.
        const         PEN = 0b0000000000000010;
        /// Specifies that the context returns WT_PACKET messages to its owner.
        const    MESSAGES = 0b0000000000000100;
        /// Specifies that the input context on the tablet will have a margin. The margin is an area outside the
        /// specified input area where events will be mapped to the edge of the input area.
        /// This feature makes it easier to input points at the edge of the context.
        const      MARGIN = 0b1000000000000000;
        /// If the CXO_MARGIN bit is on, specifies that the margin will be inside the specified context.
        /// Thus, scaling will occur from a context slightly smaller than the specified input context to the output
        /// coordinate space.
        const   MGNINSIDE = 0b0100000000000000;
        /// Specifies that the context returns WT_CSRCHANGE messages to its owner.
        const CSRMESSAGES = 0b0000000000001000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CXS:UINT {
        /// Specifies that the context has been disabled using the WTEnable function.
        const DISABLED = 0b001;
        /// Specifies that the context is at least partially obscured by an overlapping context that is higher in the context overlap order.
        const OBSCURED = 0b010;
        /// Specifies that the context is the topmost context in the context overlap order.
        const    ONTOP = 0b100;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CXL:UINT {
        /// Specifies that the context's input size cannot be changed. When this value is not specified,
        /// the context's input extents in x, y, and z can be changed.
        /// NOTE: The context's origins in x, y, and z can always be changed.
        const      INSIZE = 0b00001;
        /// Specifies that the context's input aspect ratio cannot be changed. When this value is specified, the
        /// context's size can be changed, but the ratios among x, y, and z extents will be kept as close to constant
        /// as possible.
        const    INASPECT = 0b00010;
        /// Specifies that the context's sensitivity settings for x, y, and z cannot be changed.
        const SENSITIVITY = 0b00100;
        /// Specifies that the context's margin options cannot be changed. This value controls the locking of the
        /// CXO_MARGIN and CXO_MGNINSIDE option values.
        const      MARGIN = 0b01000;
        /// If the context is a system cursor context, the value specifies that the system pointing control variables
        /// of the context cannot be changed.
        const      SYSOUT = 0b10000;
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cast_void, WTInfo, WTI};
    use libloading::{Library, Symbol};
    #[test]
    fn test_struct_sizes(){
        let size_required;
        unsafe{
            let wintab                  = Library::new("Wintab32.dll").unwrap();
            let wtinfoa:Symbol<WTInfo> = wintab.get(c"WTInfoA".to_bytes()).unwrap();
            size_required               = wtinfoa(WTI::DEFSYSCTX, 0, std::ptr::null_mut());
        }
        let size_of_type = std::mem::size_of::<LOGCONTEXT>();
        assert_eq!(size_required as usize, size_of_type);
    }

    /// Note: this is not a test, it just prints out the value obtained from the driver for review.
    #[test]
    fn test_struct_content(){
        let mut wintab_context = LOGCONTEXT::default();
        unsafe{
            let wintab                  = Library::new("Wintab32.dll").unwrap();
            let wtinfoa:Symbol<WTInfo> = wintab.get(c"WTInfoA".to_bytes()).unwrap();
            let _ = wtinfoa(WTI::DEFSYSCTX, 0, cast_void!(wintab_context));
        }
        println!("sys {:#?}", wintab_context);
    }
}