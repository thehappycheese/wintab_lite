use std::ffi::{c_int, c_void};

use super::c_type_aliases::*;
use super::information_categories::WTI;
use super::LOGCONTEXT;

/// This function returns global information about the interface in an application-supplied buffer. Different types of
/// information are specified by different index arguments. Applications use this function to receive information about
/// tablet coordinates, physical dimensions, capabilities, and cursor types.
/// 
/// - `wCategory` Identifies the category from which information is being requested.
/// - `nIndex` Identifies which information is being requested from within the category.
/// - `lpOutput` Points to a buffer to hold the requested information.
/// 
/// The return value specifies the size of the returned information in bytes. If the information is not supported, the
/// function returns zero. If a tablet is not physically present, this function always returns zero.
pub type WTInfo  = unsafe extern fn (wCategory: WTI, nIndex: UINT, lpOutput: *mut c_void) -> UINT;

/// This function establishes an active context on the tablet. On successful completion of this function, the
/// application may begin receiving tablet events via messages (if they were requested), and may use the handle returned
/// to poll the context, or to perform other context-related functions.
/// 
/// - `hWnd` Identifies the window that owns the tablet context, and receives messages from the context.
/// - `lpLogCtx` Points to an application-provided LOGCONTEXT data structure describing the context to be opened.
/// - `fEnable` Specifies whether the new context will immediately begin processing input data.
/// 
/// The return value identifies the new context. It is NULL if the context is not opened.
pub type WTOpen  = unsafe extern fn (hWnd: isize, lpLogCtx: *mut LOGCONTEXT, fEnable: BOOL  ) -> *mut HCTX;

/// This function closes and destroys the tablet context object.
/// After a call to this function, the passed handle is no longer valid. The owning window (and all manager windows)
/// will receive a WT_CTXCLOSE message when the context has been closed.
/// 
/// - `hCtx` Identifies the context to be closed.
/// 
/// The function returns a non-zero value if the context was valid and was destroyed. Otherwise, it returns zero.
pub type WTClose  = unsafe extern fn (hCtx: *mut HCTX) -> BOOL;

/// This function fills in the passed lpPkt buffer with the context event packet having the specified serial number.
/// The returned packet and any older packets are removed from the context's internal queue.
/// 
/// - `hCtx` Identifies the context whose packets are being returned.
/// - `wSerial` Serial number of the tablet event to return.
/// - `lpPkts` Points to a buffer to receive the event packets.
/// 
/// The return value is non-zero if the specified packet was found and returned. It is zero if the specified packet was
/// not found in the queue.
pub type WTPacket = unsafe extern fn (hCtx:*mut HCTX, wSerial:UINT, lpPkts:*mut c_void) -> BOOL;


/// This function returns the serial numbers of the oldest and newest packets currently in the queue.
/// 
/// - `hCtx` Identifies the context whose queue is being queried.
/// - `lpOld` Points to an unsigned integer to receive the oldest packet's serial number.
/// - `lpNew` Points to an unsigned integer to receive the newest packet's serial number.
/// 
/// The function returns non-zero if successful, zero otherwise.
pub type WTQueuePacketsEx = unsafe extern fn (hCtx: *mut HCTX, lpOld:*mut UINT, lpNew: *mut UINT) -> BOOL;

/// This function copies all packets with serial numbers between wBegin and wEnd inclusive from the context's queue to
/// the passed buffer and removes them from the queue.
/// The buffer pointed to by lpPkts must be at least cMaxPkts * sizeof(PACKET) bytes long to prevent overflow.
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
pub type WTDataGet = unsafe extern fn (
    hCtx: *mut HCTX,
    wBegin: UINT,
    wEnd: UINT,
    cMaxPkts: c_int,
    lpPkts: *mut c_void,
    lpNPkts: *mut c_int
) -> BOOL;


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
pub type WTPacketsGet = unsafe extern fn (
    hCtx: *mut HCTX,
    cMaxPkts: c_int,
    lpPkts: *mut c_void
) -> c_int;

