use bitflags::bitflags;

/// The Information Category;
/// used as the first argument when querying wintab through the [WTInfo()](super::WTInfo) function.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WTI {
    /// Specifies a query for global interface identification and capability information. See [IFC]
    INTERFACE = 1,

    /// Specifies a query for current interface resource usage statistics. See [STA]
    STATUS = 2,

    /// Specifies a query for the current default digitizing logical context. See [CTX]
    /// 
    /// > NOTE: The digitizing context ([WTI::DDCTXS] or [WTI::DEFCONTEXT]) tells Wintab to deliver pen data packets
    /// > containing tablet count data to the app when polled for or through a Wintab WT_PACKET message. With digitizing 
    /// > context data, the application has high-resolution streaming pen data that can be used, for example, in 
    /// > fine-grained control of vectors in graphics apps or for biometric information in signature apps. For this
    /// > context, the user must interpolate the data into the app’s client rectangle.
    DEFCONTEXT = 3,

    /// Specifies a query for the current default system logical context. See [CTX]
    /// 
    /// I think "System Context" means that wintab will deliver packets with coordinate data already mapped to pixels on
    /// our behalf, ie we don't get raw device coordinates. If we wanted the raw device coordinates (maybe our tablet
    /// can deliver sub-pixel precision) then I think we can get that using the digitizing context;
    /// see [WTI::DEFCONTEXT].
    /// The documentation's explanation of the issue is pretty convoluted, so this is just my best guess.
    /// 
    /// The system context (WTI_DEFSYSCTX) tells Wintab to deliver pen data packets containing system pixel data (dpi
    /// adjusted) to the app when polled for or through a Wintab WT_PACKET message. With system data, it is very easy to
    /// make a system call to convert the data into the app’s client rectangle. Such data is ideal for drawing or 
    /// signature apps.
    DEFSYSCTX = 4,

    /// Specifies a query for the capability and status information for a device. See [DVC]
    /// 
    /// "Multiplexed"; use `WTI::DEVICES + 1` to refer to the second item etc
    DEVICES = 100,

    /// Specifies a query for the capability and status information for a cursor type. See [CSR]
    /// 
    /// "Multiplexed"; use `WTI::CURSORS + 1` to refer to the second item etc
    CURSORS = 200,

    /// Specifies a query for the descriptive information and defaults for an extension. See [EXT]
    /// 
    /// "Multiplexed"; use `WTI::EXTENSIONS + 1` to refer to the second item etc
    EXTENSIONS = 300,

    /// Specifies a query for the current default digitizing logical context for the corresponding device. See [CTX]
    /// 
    /// "Multiplexed"; use `WTI::DDCTXS + 1` to refer to the second item etc
    DDCTXS = 400,

    /// Specifies a query for the current default system logical context for the corresponding device. See [CTX]
    /// 
    /// "Multiplexed"; use `WTI::DSCTXS + 1` to refer to the second item etc
    DSCTXS = 500,
}

#[repr(u32)]
/// [WTI::INTERFACE] Index Definitions
pub enum IFC{
    /// `TCHAR[]` Returns a copy of the null-terminated tablet hardware identification string in the user buffer.
    /// This string should include make, model, and revision information in user-readable format.
    WINTABID    = 1,
    /// `WORD` Returns the specification version number.
    /// The high-order byte contains the major version number; the low-order byte contains the minor version number.
    SPECVERSION = 2,
    /// `WORD` Returns the implementation version number.
    /// The high-order byte contains the major version number; the low-order byte contains the minor version number.
    IMPLVERSION = 3,
    /// `UINT` Returns the number of devices supported.
    NDEVICES    = 4,
    /// `UINT` Returns the total number of cursor types supported.
    NCURSORS    = 5,
    /// `UINT` Returns the number of contexts supported.
    NCONTEXTS   = 6,
    /// `UINT` Returns flags indicating which context options are supported.
    CTXOPTIONS  = 7,
    /// `UINT` Returns the size of the save information returned from WTSave.
    CTXSAVESIZE = 8,
    /// `UINT` Returns the number of extension data items supported.
    NEXTENSIONS = 9,
    /// `UINT` Returns the number of manager handles supported.
    NMANAGERS   = 10,
}


/// [WTI::STATUS] Index Definitions
#[repr(u32)]
pub enum STA{
    /// `UINT` Returns the number of contexts currently open.
    CONTEXTS = 1,
    /// `UINT` Returns the number of system contexts currently open.
    SYSCTXS = 2,
    /// `UINT` Returns the maximum packet report rate currently being received by any context, in Hertz.
    PKTRATE = 3,
    /// `WTPKT` Returns a mask indicating which packet data items are requested by at least one context.
    PKTDATA = 4,
    /// `UINT` Returns the number of manager handles currently open.
    MANAGERS = 5,
    /// `BOOL` Returns a non-zero value if system pointing is available to the whole screen; zero otherwise.
    SYSTEM = 6,
    /// `DWORD` Returns a button mask indicating the logical buttons whose events are requested by at least one context.
    BUTTONUSE = 7,
    /// `DWORD` Returns a button mask indicating which logical buttons are assigned a system button function by the current cursor's system button map.
    SYSBTNUSE = 8,
}

/// [WTI::DEFCONTEXT], [WTI::DEFSYSCTX], [WTI::DDCTXS], and [WTI::DSCTXS] Index Definitions
#[repr(u32)]
pub enum CTX{
    /// `TCHAR[]` Returns a 40 character array containing the default name. The name may occupy zero to 39 characters;
    /// the remainder of the array is padded with zeroes.
    NAME      = 1,
    /// `UINT` Returns option flags. For the default digitizing context, CXO_MARGIN and CXO_MGNINSIDE are allowed.
    /// For the default system context, CXO_SYSTEM is required; CXO_PEN, CXO_MARGIN, and CXO_MGNINSIDE are allowed.
    OPTIONS   = 2,
    /// `UINT` Returns zero.
    STATUS    = 3,
    /// `UINT` Returns which attributes of the default context are locked.
    LOCKS     = 4,
    /// `UINT` Returns the value WT_DEFBASE.
    MSGBASE   = 5,
    /// `UINT` Returns the default device. If this value is -1, then it also known as a "virtual device".
    DEVICE    = 6,
    /// `UINT` Returns the default context packet report rate, in Hertz.
    PKTRATE   = 7,
    /// `WTPKT` Returns which optional data items will be in packets returned from the context. For the default
    /// digitizing context, this field must at least indicate buttons, x, and y data.
    PKTDATA   = 8,
    /// `WTPKT` Returns whether the packet data items will be returned in absolute or relative mode.
    PKTMODE   = 9,
    /// `WTPKT` Returns which packet data items can generate motion events in the context.
    MOVEMASK  = 10,
    /// `DWORD` Returns the buttons for which button press events will be processed in the context.
    /// The default context must at least select button press events for one button.
    BTNDNMASK = 11,
    /// `DWORD` Returns the buttons for which button release events will be processed in the context.
    BTNUPMASK = 12,
    /// `LONG` origin of the context's input area in the tablet's native coordinates.
    INORGX    = 13,
    /// `LONG` origin of the context's input area in the tablet's native coordinates.
    INORGY    = 14,
    /// `LONG` origin of the context's input area in the tablet's native coordinates.
    INORGZ    = 15,
    /// `LONG` extent of the context's input area in the tablet's native coordinates
    INEXTX    = 16,
    /// `LONG` extent of the context's input area in the tablet's native coordinates
    INEXTY    = 17,
    /// `LONG` extent of the context's input area in the tablet's native coordinates
    INEXTZ    = 18,	
    /// `LONG` origin of the context's output coordinate space in context output coordinates
    OUTORGX   = 19,
    /// `LONG` origin of the context's output coordinate space in context output coordinates
    OUTORGY   = 20,
    /// `LONG` origin of the context's output coordinate space in context output coordinates
    OUTORGZ   = 21	,
    /// `LONG` extent of the context's output coordinate space in context output coordinates
    OUTEXTX   = 22,
    /// `LONG` extent of the context's output coordinate space in context output coordinates
    OUTEXTY   = 23,
    /// `LONG` extent of the context's output coordinate space in context output coordinates
    OUTEXTZ   = 24,
    /// `FIX32` relative-mode sensitivity factor
    SENSX     = 25,
    /// `FIX32` relative-mode sensitivity factor
    SENSY     = 26,
    /// `FIX32` relative-mode sensitivity factor
    SENSZ     = 27,
    /// `BOOL` Returns the default system cursor tracking mode.
    SYSMODE   = 28,
    /// `int` returns 0.
    SYSORGX   = 29,
    /// `int` returns 0.
    SYSORGY   = 30,
    /// `int` current screen display size in pixels
    SYSEXTX   = 31,
    /// `int` current screen display size in pixels
    SYSEXTY   = 32,
    /// `FIX32` system cursor relative-mode sensitivity factor
    SYSSENSX  = 33,
    /// `FIX32` system cursor relative-mode sensitivity factor
    SYSSENSY  = 34,
}

/// [WTI::DEVICES] Index Definitions
#[repr(u32)]
pub enum DVC{
    /// `TCHAR[]` Returns a displayable null- terminated string describing the device, manufacturer, and revision level.
    NAME        = 1,
    /// [HWC] Returns flags indicating hardware and driver capabilities, as defined below:
	HARDWARE    = 2,
    /// `UINT` Returns the number of supported cursor types.
	NCSRTYPES   = 3,
    /// `UINT` Returns the first cursor type number for the device.
	FIRSTCSR    = 4,
    /// `UINT` Returns the maximum packet report rate in Hertz.
	PKTRATE     = 5,
    /// [WTPKT](super::WTPKT) Returns a bit mask indicating which packet data items are always available.
	PKTDATA     = 6,
    /// [WTPKT](super::WTPKT) Returns a bit mask indicating which packet data items are physically relative (i.e. items for which the hardware can only report change, not absolute measurement).
	PKTMODE     = 7,
    /// [WTPKT](super::WTPKT) Returns a bit mask indicating which packet data items are only available when certain cursors are connected. The individual cursor descriptions must be consulted to determine which cursors return which data.
	CSRDATA     = 8,
    /// [AXIS](super::AXIS) tablet context margins in tablet native coordinates
	XMARGIN     = 9,
    /// [AXIS](super::AXIS) tablet context margins in tablet native coordinates
	YMARGIN     = 10,
    /// [AXIS](super::AXIS) tablet context margins in tablet native coordinates
	ZMARGIN     = 11,
    /// [AXIS](super::AXIS) tablet's range and resolution capabilities
	X           = 12,
    /// [AXIS](super::AXIS) tablet's range and resolution capabilities
	Y           = 13,
    /// [AXIS](super::AXIS) tablet's range and resolution capabilities
	Z           = 14,
    /// [AXIS](super::AXIS) tablet's range and resolution capabilities for the normal pressure
	NPRESSURE   = 15,
    /// [AXIS](super::AXIS) tablet's range and resolution capabilities for the tangential pressure
	TPRESSURE   = 16,
    /// [(AXIS,AXIS,AXIS)](super::AXIS) the tablet's orientation range and resolution capabilities
	ORIENTATION = 17,
    /// [(AXIS,AXIS,AXIS)](super::AXIS) the tablet's rotation range and resolution capabilities
	ROTATION    = 18,
    /// `TCHAR[]` Returns a null-terminated string containing the device’s Plug and Play ID.
	PNPID       = 19,
}

// Sadly this did not work, see docs for index_enum_and_struct
// index_enum_and_struct!{
//     "[WTI::DEVICES] Queries",
//     DVC_Struct,
//     DVC_Enum,
//     repr(u32),
//     [
//         (NAME        , 1,  CString40, "Returns a displayable null- terminated string describing the device, manufacturer, and revision level."),
//         (HARDWARE    , 2,  HWC, "Returns flags indicating hardware and driver capabilities, as defined below:"),
//         (NCSRTYPES   , 3,  UINT, "Returns the number of supported cursor types."),
//         (FIRSTCSR    , 4,  UINT, "Returns the first cursor type number for the device."),
//         (PKTRATE     , 5,  UINT, "Returns the maximum packet report rate in Hertz."),
//         (PKTDATA     , 6,  WTPKT, "Returns a bit mask indicating which packet data items are always available."),
//         (PKTMODE     , 7,  WTPKT, "Returns a bit mask indicating which packet data items are physically relative (i.e. items for which the hardware can only report change, not absolute measurement)."),
//         (CSRDATA     , 8,  WTPKT, "Returns a bit mask indicating which packet data items are only available when certain cursors are connected. The individual cursor descriptions must be consulted to determine which cursors return which data."),
//         (XMARGIN     , 9,  AXIS, "tablet context margins in tablet native coordinates"),
//         (YMARGIN     , 10, AXIS, "tablet context margins in tablet native coordinates"),
//         (ZMARGIN     , 11, AXIS, "tablet context margins in tablet native coordinates"),
//         (X           , 12, AXIS, "tablet's range and resolution capabilities"),
//         (Y           , 13, AXIS, "tablet's range and resolution capabilities"),
//         (Z           , 14, AXIS, "tablet's range and resolution capabilities"),
//         (NPRESSURE   , 15, AXIS, "tablet's range and resolution capabilities for the normal pressure"),
//         (TPRESSURE   , 16, AXIS, "tablet's range and resolution capabilities for the tangential pressure"),
//         (ORIENTATION , 17, (AXIS,AXIS,AXIS), "the tablet's orientation range and resolution capabilities"),
//         (ROTATION    , 18, (AXIS,AXIS,AXIS), "the tablet's rotation range and resolution capabilities"),
//         (PNPID       , 19, CString40, "Returns a null-terminated string containing the device’s Plug and Play ID.")
//     ]
// }

bitflags! {
    /// See [WTI::DEVICES] and [DVC::HARDWARE] hardware and driver capabilities
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HWC : u32 {
        /// Indicates that the display and digitizer share the same surface.
        const     INTEGRATED = 0b0001;
        /// Indicates that the cursor must be in physical contact with the device to report position.
        const          TOUCH = 0b0010;
        /// Indicates that device can generate events when the cursor is entering and leaving the physical detection range.
        const       HARDPROX = 0b0100;
        /// Indicates that device can uniquely identify the active cursor in hardware.
        const PHYSID_CURSORS = 0b1000;
    }
}

/// [WTI::CURSORS] Index Definitions
pub enum CSR {
    /// `TCHAR[]` Returns a displayable zero-terminated string containing the name of the cursor.
    NAME         = 1,
    /// `BOOL` Returns whether the cursor is currently connected.
    ACTIVE       = 2,
    /// `WTPKT` Returns a bit mask indicating the packet data items supported when this cursor is connected.
    PKTDATA      = 3,
    /// `BYTE` Returns the number of buttons on this cursor.
    BUTTONS      = 4,
    /// `BYTE` Returns the number of bits of raw button data returned by the hardware.
    BUTTONBITS   = 5,
    /// `TCHAR[]` Returns a list of zero-terminated strings containing the names of the cursor's buttons.
    /// The number of names in the list is the same as the number of buttons on the cursor.
    /// The names are separated by a single zero character; the list is terminated by two zero characters.
    BTNNAMES     = 6,
    /// `BYTE[]` Returns a 32 byte array of logical button numbers, one for each physical button.
    BUTTONMAP    = 7,
    /// `BYTE[]` Returns a 32 byte array of button action codes, one for each logical button.
    SYSBTNMAP    = 8,
    /// `BYTE` Returns the physical button number of the button that is controlled by normal pressure.
    NPBUTTON     = 9,
    /// `UINT[]` Returns an array of two UINTs, specifying the button marks for the normal pressure button.
    /// The first UINT contains the release mark; the second contains the press mark.
    NPBTNMARKS   = 10,
    /// `UINT[]` Returns an array of UINTs describing the pressure response curve for normal pressure.
    NPRESPONSE   = 11,
    /// `BYTE` Returns the physical button number of the button that is controlled by tangential pressure.
    TPBUTTON     = 12,
    /// `UINT[]` Returns an array of two UINTs, specifying the button marks for the tangential pressure button.
    /// The first UINT contains the release mark; the second contains the press mark.
    TPBTNMARKS   = 13,
    /// `UINT[]` Returns an array of UINTs describing the pressure response curve for tangential pressure.
    TPRESPONSE   = 14,
    /// `DWORD` Returns a manufacturer-specific physical identifier for the cursor.
    /// This value will distinguish the physical cursor from others on the same device.
    /// This physical identifier allows applications to bind functions to specific physical cursors,
    /// even if category numbers change and multiple, otherwise identical, physical cursors are present.
    PHYSID       = 15,
    /// `UINT` Returns the cursor mode number of this cursor type, if this cursor type has the CRC_MULTIMODE capability.
    MODE         = 16,
    /// `UINT` Returns the minimum set of data available from a physical cursor in this cursor type,
    /// if this cursor type has the CRC_AGGREGATE capability.
    MINPKTDATA   = 17,
    /// `UINT` Returns the minimum number of buttons of physical cursors in the cursor type,
    /// if this cursor type has the CRC_AGGREGATE capability.
    MINBUTTONS   = 18,
    /// `[CRC]` Returns flags indicating cursor capabilities, as defined by the values and their meanings, below:
    CAPABILITIES = 19,
    /// TODO: UNDOCUMENTED?
    TYPE     = 20,
}

bitflags! {
    /// See [CSR::CAPABILITIES] cursor capabilities
    pub struct CRC:u32 {
        ///  Indicates this cursor type describes one of several modes of a single physical cursor.
        /// Consecutive cursor type categories describe the modes;
        /// the CSR_MODE data item gives the mode number of each cursor type.
        const CRC_MULTIMODE = 0b001;
        /// Indicates this cursor type describes several physical cursors that cannot be distinguished by software.
        const CRC_AGGREGATE = 0b010;
        /// Indicates this cursor type describes the physical cursor in its inverted orientation;
        /// the previous consecutive cursor type category describes the normal orientation.
        const    CRC_INVERT = 0b100;
    }
}

/// [WTI::EXTENSIONS] Index Definitions
pub enum EXT {
    /// `TCHAR[]` Returns a unique, null-terminated string describing the extension.
    NAME       = 1,
    /// `UINT` Returns a unique identifier for the extension.
    TAG        = 2,
    /// `WTPKT` Returns a mask that can be bitwise OR'ed with WTPKT-type variables to select the extension.
    MASK       = 3,
    /// `UINT[]` Returns an array of two UINTs specifying the extension's size within a packet (in bytes).
    /// The first is for absolute mode; the second is for relative mode.
    SIZE       = 4,
    /// `AXIS[]` Returns an array of axis descriptions, as needed for the extension.
    AXES       = 5,
    /// `BYTE[]` Returns the current global default data, as needed for the extension.
    /// This data is modified via the WTMgrExt function.
    DEFAULT    = 6,
    /// `BYTE[]` current default context-specific data, as needed for the extension for the system-context
    DEFSYSCTX  = 7,
    /// `BYTE[]` current default context-specific data, as needed for the extension for the digitizing-context
    DEFCONTEXT = 8,
    /// `BYTE[]` Is the first of one or more consecutive indices, one per cursor type.
    /// Each returns the current default cursor-specific data, as needed for the extension.
    /// This data is modified via the WTMgrCsrExt function.
    CURSORS    = 9,
    /// TODO: UNDOCUMENTED?
    DEVICES    = 110,
}


