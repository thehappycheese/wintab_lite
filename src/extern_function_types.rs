//! This module requires the `libloading` feature
//! 
//! The types defined in this module are intended to be dynamically linked using
//! the `libloading` crate.
//! 
//! Using libloading with these types has the advantage of not requiring Wintab32.dll to be present
//! when at compile time. If you do have Wintab32.dll on your system at compile time, then consider
//! using the `raw-dylib` feature instead, and use the functions defined in
//! [crate::extern_functions]
//! 
use windows::Win32::Foundation::HWND;
use super::c_type_aliases::*;
use super::information_categories::WTI;
use super::LOGCONTEXT;
use libloading::Symbol;

/// Returns information about the interface in an application-supplied buffer. 
/// 
/// - `wCategory` Identifies the category from which information is being requested
///   (e.g. tablet coordinates, physical dimensions, capabilities, and cursor types)
/// - `nIndex` Identifies which information is being requested from within the category.
/// - `lpOutput` Points to a buffer to hold the requested information.
/// 
/// The return value is the size of the returned information in bytes. If the information is not
/// supported, the function returns zero. If a tablet is not physically present, this function
/// always returns zero.
pub type WTInfo<'a>  = Symbol<'a, unsafe extern fn (
    wCategory: WTI,
    nIndex: UINT,
    lpOutput: LPVOID
) -> UINT>;

/// Opens a connection to the tablet using the provided context.
/// If successful, the the specified window will receive tablet events via messages (if configured).
/// The handle that is returned may also be used to poll the context, or to perform other functions.
/// 
/// - `hWnd` The window handel that owns the tablet context, and receives messages from the context.
/// - `lpLogCtx` is a pointer to a [LOGCONTEXT] data structure describing the context to be opened.
/// - `fEnable` Specifies whether the new context will immediately begin processing input data.
/// 
/// The return value is the opened context handel. It will be a zero value if the context could not
/// be opened.
pub type WTOpen<'a>  = Symbol<'a, unsafe extern fn (
    hWnd: HWND,
    lpLogCtx: *mut LOGCONTEXT,
    fEnable: BOOL
) -> *mut HCTX>;

/// Closes and destroys the tablet context object.
/// After a calling the passed handle is invalid. The owning window (and all manager windows)
/// will receive a [WT::CTXCLOSE](crate::WT) message when the context has been closed.
/// 
/// - `hCtx` Identifies the context to be closed.
/// 
/// The function returns a non-zero value if the context was valid and was destroyed.
pub type WTClose<'a>  = Symbol<'a, unsafe extern fn (
    hCtx: *mut HCTX
) -> BOOL>;

/// Fills in the passed buffer with the event packet having the specified serial number.
/// The returned packet and any older packets are removed from the context's internal queue.
/// 
/// - `hCtx` Identifies the context whose packets are being returned.
/// - `wSerial` Serial number of the tablet event to return.
/// - `lpPkts` Points to a buffer to receive the event packets.
/// 
/// The return value is non-zero if the specified packet was found and returned.
/// It is zero if the specified packet was not found in the queue.
pub type WTPacket<'a> = Symbol<'a, unsafe extern fn (
    hCtx:*mut HCTX,
    wSerial:UINT,
    lpPkts:LPVOID
) -> BOOL>;


/// This function returns the serial numbers of the oldest and newest packets currently in the
/// queue.
/// 
/// - `hCtx` Identifies the context whose queue is being queried.
/// - `lpOld` Points to an unsigned integer to receive the oldest packet's serial number.
/// - `lpNew` Points to an unsigned integer to receive the newest packet's serial number.
/// 
/// The function returns non-zero if successful, zero otherwise.
pub type WTQueuePacketsEx<'a> = Symbol<'a, unsafe extern fn (
    hCtx: *mut HCTX,
    lpOld:*mut UINT,
    lpNew: *mut UINT
) -> BOOL>;

/// This function copies all packets with serial numbers between wBegin and wEnd inclusive from the
/// context's queue to the passed buffer and removes them from the queue.
/// The buffer pointed to by lpPkts must be at least cMaxPkts * sizeof(PACKET) bytes long to prevent
/// overflow.
/// 
/// - `hCtx`     Identifies the context whose packets are being returned.
/// - `wBegin`   Serial number of the oldest tablet event to return.
/// - `wEnd`     Serial number of the newest tablet event to return.
/// - `cMaxPkts` Specifies the maximum number of packets to return.
/// - `lpPkts`   Points to a buffer to receive the event packets.
/// - `lpNPkts`  Points to an integer to receive the number of packets actually copied.
/// 
/// The return value is the total number of packets found in the queue between wBegin and wEnd.
/// 
pub type WTDataGet<'a> = Symbol<'a, unsafe extern fn (
    hCtx: *mut HCTX,
    wBegin: UINT,
    wEnd: UINT,
    cMaxPkts: INT,
    lpPkts: LPVOID,
    lpNPkts: *mut INT
) -> BOOL>;


// int WTPacketsGet(hCtx, cMaxPkts, lpPkts)

/// This function copies the next cMaxPkts events from the packet queue of context
/// hCtx to the passed lpPkts buffer and removes them from the queue.
///
/// - `hCtx` Identifies the context whose packets are being returned.
/// - `cMaxPkts` Specifies the maximum number of packets to return.
/// - `lpPkts` Points to a buffer to receive the event packets.
///
/// The return value is the number of packets copied in the buffer.
///
/// - The exact structure of the returned packet is determined by the packet
///   information that was requested when the context was opened.
/// - The buffer pointed to by lpPkts must be at least cMaxPkts * sizeof(PACKET)
///   bytes long to prevent overflow.
/// - Applications may flush packets from the queue by calling this function with a
///   NULL lpPktargument.
/// 
pub type WTPacketsGet = Symbol<'static, unsafe extern fn (
    hCtx: *mut HCTX,
    cMaxPkts: INT,
    lpPkts: LPVOID
) -> INT>;
