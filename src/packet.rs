#![allow(non_snake_case)]
use crate::{
    c_type_aliases::{DWORD, HCTX, INT, LONG, UINT},
    Bitmask, WTPKT, XYZ,
};
use bitflags::bitflags;

/// See [ButtonChange]
#[repr(u16)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonChangeType {
    #[default]
    NONE = 0,
    UP = 1,
    DOWN = 2,
}

/// This struct is unused, in a previous version it was the type of [Packet::pkButtons]
/// I think it was working, but i opted to use the bitmask datatype instead.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonChange {
    /// Specifies which button changed.
    pub button_number: u16,
    pub change_type: ButtonChangeType,
}

/// The ROTATION data structure specifies the Pitch Roll and Yaw Rotation of the cursor with respect
/// to the tablet.
/// Each cursor type will have a major axis and "normal orientation" defined for it, based on its
/// physical characteristics.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rotation {
    /// Specifies the pitch of the cursor
    pub roPitch: INT,
    /// Specifies the roll of the cursor
    pub roRoll: INT,
    /// Specifies the yaw of the cursor
    pub roYaw: INT,
}

/// The ORIENTATION data structure specifies the Azimuth, Altitude and Twist Orientation of the
/// cursor with respect to the tablet.
/// Each cursor type will have rotation semantics defined for it, based on its physical
/// characteristics.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Orientation {
    /// Clockwise rotation of the cursor about the z axis through a full circular range.
    pub orAzimuth: INT,
    /// Angle with the x-y plane through a signed, semicircular range.
    /// Positive values specify an angle upward toward the positive z axis;
    /// Negative values specify an angle downward toward the negative z axis.
    pub orAltitude: INT,
    /// Clockwise rotation of the cursor about its own major axis.
    pub orTwist: INT,
}

bitflags! {
    /// See [Packet::pkStatus]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct TPS:UINT {
        /// Specifies that the cursor is out of the context.
        const PROXIMITY = 0b00001;
        /// Specifies that the event queue for the context has overflowed.
        const QUEUE_ERR = 0b00010;
        /// Specifies that the cursor is in the margin of the context.
        const    MARGIN = 0b00100;
        /// Specifies that the cursor is out of the context,
        /// but that the context has grabbed input while waiting for a
        /// button release event.
        const      GRAB = 0b01000;
        /// Specifies that the cursor is in its inverted state.
        const    INVERT = 0b10000;
    }
}

/// This is the full packet data structure,
/// for absolute mode packets except for buttons which is assumed to be in relative mode.
/// To receive packets with this struct the user MUST call [`WTOpen`] with a
/// [`LOGCONTEXT`] where
///
/// - [`.lcPktData`] field has been set to [`WTPKT::all()`] (include all fields in struct) and
/// - [`.lcPktMode`] has been set to [`WTPKT::empty()`] (everything absolute mode)
///
///
/// [`WTOpen`]:        crate::WTOpen
/// [`LOGCONTEXT`]:     crate::LOGCONTEXT
/// [`.lcPktData`]:     crate::LOGCONTEXT::lcPktData
/// [`.lcPktMode`]:     crate::LOGCONTEXT::lcPktMode
/// [`WTPKT::BUTTONS`]: crate::WTPKT::BUTTONS
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C, packed(4))]
pub struct Packet {
    /// Specifies the context that generated the event.
    pub pkContext: *mut HCTX,

    /// Specifies various status and error conditions.
    /// These conditions can be combined by using the bitwise OR operator.
    /// The pkStatus field can be any combination of the status values.
    pub pkStatus: TPS,

    /// In absolute mode, specifies the system time at which the event was posted.
    /// In relative mode, specifies the elapsed time in milliseconds since the last packet.
    pub pkTime: DWORD,

    /// Specifies which of the included packet data items have changed since the previously posted
    /// event.
    pub pkChanged: WTPKT,

    /// Contains a serial number assigned to the packet by the context.
    /// Consecutive packets will have consecutive serial numbers.
    pub pkSerialNumber: UINT,

    /// Specifies which cursor type generated the packet.
    pub pkCursor: UINT,

    /// a bitmask containing the current state of all buttons.
    /// This requires the context to be configured in absolute mode
    /// (i.e. `log_context.lcPktMode &= !WTPKT::BUTTONS;`)
    ///
    /// > When buttons are set to relative mode (i.e. `log_context.lcPktMode |= WTPKT::BUTTONS;`)
    /// > Then this field would be a [ButtonChange] Struct.
    /// > However this just didn't work when I tested it.
    /// > Hence I have hard coded the "absolute" i.e. bitmask option.
    //pub pkButtons: ButtonChange,
    pub pkButtons: Bitmask<u32>,

    /// In absolute mode, the scaled cursor location
    /// In relative mode, the scaled change in cursor position.
    ///
    /// Despite original documentation, this appears to always be a signed integer type.
    pub pkXYZ: XYZ<LONG>,

    /// The adjusted state of the normal pressure
    /// This is a UINT in absolute mode, and in relative mode it is an int containing the change 
    /// in pressure state.
    ///
    /// > Currently only absolute mode is supported with this struct,
    /// > however it might be safe to just interpret this as a signed integer if you configured
    /// > the context in relative mode.
    pub pkNormalPressure: UINT,

    /// The state of the tangent pressure
    /// This is a UINT in absolute mode, and in relative mode it is an int containing the change
    /// in pressure state.
    ///
    /// > Currently only absolute mode is supported with this struct,
    /// > however it might be safe to just interpret this as a signed integer if you configured
    /// > the context in relative mode.
    pub pkTangentPressure: UINT,

    /// Contains updated cursor orientation information.
    pub pkOrientation: Orientation,

    /// Contains updated cursor rotation information.
    pub pkRotation: Rotation,
}
impl Default for Packet {
    fn default() -> Self {
        Self {
            pkContext: std::ptr::null_mut(), // TODO: <-- this is why Default is manually implemented. How to avoid?
            pkStatus: Default::default(),
            pkTime: Default::default(),
            pkChanged: Default::default(),
            pkSerialNumber: Default::default(),
            pkCursor: Default::default(),
            pkButtons: Default::default(),
            pkXYZ: Default::default(),
            pkTangentPressure: Default::default(),
            pkNormalPressure: Default::default(),
            pkOrientation: Default::default(),
            pkRotation: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of, MaybeUninit};
    use std::ptr::addr_of;

    #[test]
    fn test_button_change() {
        const UNINITIALIZED: MaybeUninit<ButtonChange> = MaybeUninit::uninit();
        let ptr = UNINITIALIZED.as_ptr();
        assert_eq!(size_of::<ButtonChange>(), 4usize,);
        assert_eq!(align_of::<ButtonChange>(), 2usize,);
        assert_eq!(
            unsafe { addr_of!((*ptr).button_number) as usize - ptr as usize },
            0usize
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).change_type) as usize - ptr as usize },
            2usize
        );
    }
}
