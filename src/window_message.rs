/// These are the event numbers of the wintab events which are sent to the window
/// that owns the a context or any 'manager' windows.
/// 
const WT_DEFBASE: u32 = 32752;


/// Sent to windows that have requested messaging for their context.
/// 
/// - wParam serial number of the packet that generated the message.
/// - lParam handle of the context that processed the packet.
/// 
/// See [crate::WTPacket] to read the packet content
pub const PACKET:u32     = WT_DEFBASE + 0;
/// a context is opened
/// 
/// - wParam  context handle of the opened context.
/// - lParam current context status flags [CXS](crate::CXS)
pub const CTXOPEN:u32    = WT_DEFBASE + 1;
/// a context is about to be closed
/// 
/// - wParam context handle of the context to be closed.
/// - lParam context status flags [CXS](crate::CXS)
/// 
/// 
pub const CTXCLOSE:u32   = WT_DEFBASE + 2;
/// A context is changed. To find out what happened call WTGet or WTExtGet
/// 
/// - wParam context handle of the changed context.
/// - lParam current status flags [CXS](crate::CXS)
pub const CTXUPDATE:u32  = WT_DEFBASE + 3;
/// moved in the overlap order
/// 
/// - wParam context handle of the re-overlapped context.
/// - lParam current status flags [CXS](crate::CXS)
pub const CTXOVERLAP:u32 = WT_DEFBASE + 4;
/// cursor enters or leaves context proximity
/// 
/// - wParam handle of the context that the cursor is entering or leaving.
/// - lParam The low-order word is non-zero when the cursor is entering the context and zero when it is leaving the context. The high-order word is non-zero when the cursor is leaving or entering hardware proximity.
pub const PROXIMITY:u32  = WT_DEFBASE + 5;
/// the number of connected tablets has changed
/// 
/// - wParam	Contains the manager handle of the tablet manager that changed the information, or zero if the change was reported through hardware.
/// - lParam	Contains category and index numbers for the changed information. The low-order word contains the category number; the high-order word contains the index number.
pub const INFOCHANGE:u32 = WT_DEFBASE + 6;
/// posted to the owning window when a new cursor enters the context
/// 
/// - wParam the serial number of the packet that generated the message.
/// - lParam the handle of the context that processed the packet.
/// 
/// Only contexts that have the [crate::CXO::CSRMESSAGES] option selected will generate this message.
pub const CSRCHANGE:u32  = WT_DEFBASE + 7;
/// ???
pub const PACKETEXT:u32  = WT_DEFBASE + 8;
