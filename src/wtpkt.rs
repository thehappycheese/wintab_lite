use crate::DWORD;
use bitflags::bitflags;

bitflags! {
    /// Data items available in event packets.
    /// Can also contain extension data items not documented below.
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct WTPKT: DWORD {
        /// Specifies the handle of the reporting context
        const          CONTEXT = 0b00000000000001;
        /// Status information (Mainly, if the context been disabled i think)
        const           STATUS = 0b00000000000010;
        /// Specifies the time at which the packet was generated
        const             TIME = 0b00000000000100;
        /// Specifies the time at which the packet was generated
        const          CHANGED = 0b00000000001000;
        /// Specifies the packet serial number
        const    SERIAL_NUMBER = 0b00000000010000;
        /// Specifies the cursor that generated the packet
        const           CURSOR = 0b00000000100000;
        /// button information (buttons being pressed)
        const          BUTTONS = 0b00000001000000;
        /// x axis pen position data
        const                X = 0b00000010000000;
        /// y axis pen position data
        const                Y = 0b00000100000000;
        /// z axis pen position data
        const                Z = 0b00001000000000;
        /// tip-button or normal-to-surface pressure data
        const  NORMAL_PRESSURE = 0b00010000000000;
        /// barrel-button or tangent-to-surface pressure data
        const TANGENT_PRESSURE = 0b00100000000000;
        /// cursor orientation information
        const      ORIENTATION = 0b01000000000000;
        /// cursor rotation information
        const         ROTATION = 0b10000000000000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wtpkt() {
        let basic_combo = WTPKT::X | WTPKT::Y | WTPKT::BUTTONS | WTPKT::NORMAL_PRESSURE;
        println!("{:?}", basic_combo);
        assert_eq!(basic_combo.bits(), 1472);
    }
}
