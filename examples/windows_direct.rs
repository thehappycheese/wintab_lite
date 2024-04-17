//! This example demonstrates the use of wintab_lite using the windows crate directly
//! 
//! This one is pretty messy and only outputs to the terminal. I recommend you look at the winit_libloading example

use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::ValidateRect,
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::*
    },
};
use anyhow::Result;

use wintab_lite::{
    cast_void, Packet, WTInfo, WTOpen, WTPacket, AXIS, CXO, DVC, HCTX, LOGCONTEXT, WT, WTI, WTPKT

};

fn main() -> Result<()> {
    unsafe {
        let lib = match libloading::Library::new("Wintab32.dll"){
            Ok(lib) => lib,
            Err(e) => panic!("Unable to load Wintab32.dll {}",e)
        };

        let wtopena:libloading::Symbol<WTOpen>  = lib.get(c"WTOpenA".to_bytes())?;
        let wtinfoa:libloading::Symbol<WTInfo>  = lib.get(c"WTInfoA".to_bytes())?;

        let window_module_handle = GetModuleHandleA(None)?;
        debug_assert!(window_module_handle.0 != 0);

        let window_class_name = s!("window");

        let window_class = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: window_module_handle.into(),
            lpszClassName: window_class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&window_class);
        debug_assert!(atom != 0);

        let window_handel = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class_name,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            window_module_handle,
            None,
        );
        
        let mut log_context = LOGCONTEXT::default();

        // let hctx:wt::HCTX = std::ptr::null_mut();
        // let wDevice       : wt::UINT = 0;
        // let wExtX         : wt::UINT = 0;
        // let wExtY         : wt::UINT = 0;
        // let wWTInfoRetVal : wt::UINT = 0;
        
        let mut TabletX = AXIS::default();
        let mut TabletY = AXIS::default();
        log_context.lcOptions.insert(CXO::SYSTEM);

        let wtinfoa_return_value = wtinfoa(WTI::DEFCONTEXT, 0, cast_void!(log_context));
        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<LOGCONTEXT>());
        assert!(log_context.lcOptions.contains(CXO::SYSTEM));
        
        // not sure if there is a need to do this;
        log_context.lcName.write_str(format!("PrsTest Digitizing {window_module_handle:?}").as_str());

        // these are the defaults anyway;
        log_context.lcPktData = WTPKT::all();
        log_context.lcPktMode = WTPKT::empty();

        // This bitfield determines whether or not this context will receive
        // a packet when a value for each packet field changes.  This is not
        // supported by the Intuos Wintab.  Your context will always receive
        // packets, even if there has been no change in the data.
        log_context.lcMoveMask = WTPKT::STATUS | WTPKT::X | WTPKT::Y | WTPKT::NORMAL_PRESSURE;

        // Which buttons events will be handled by this context.  lcBtnMask
        // is a bitfield with one bit per button.
        log_context.lcBtnUpMask = log_context.lcBtnDnMask;

        // Set the entire tablet as active
        // Note: only works with 0th tablet! clear your tablet prefs;
        //       otherwise, you may get some funky behavior
        let wtinfoa_return_value = wtinfoa(WTI::DEVICES, DVC::X as u32, cast_void!(TabletX));
        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<AXIS>());

        let wtinfoa_return_value = wtinfoa(WTI::DEVICES, DVC::Y as u32, cast_void!(TabletY));
        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<AXIS>());

        log_context.lcInOrgXYZ.x = 0;
        log_context.lcInOrgXYZ.y = 0;
        log_context.lcInExtXYZ.x = TabletX.axMax;
        log_context.lcInExtXYZ.y = TabletY.axMax;

        // Guarantee the output coordinate space to be in screen coordinates.
        log_context.lcOutOrgXYZ.x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        log_context.lcOutOrgXYZ.y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        log_context.lcOutExtXYZ.x = GetSystemMetrics(SM_CXVIRTUALSCREEN); //SM_CXSCREEN);

        // In Wintab, the tablet origin is lower left.  Move origin to upper left
        // so that it coincides with screen origin.
        log_context.lcOutExtXYZ.y = -GetSystemMetrics(SM_CYVIRTUALSCREEN);	//SM_CYSCREEN);

        // Leave the system origin and extents as received:
        // lcSysOrgX, lcSysOrgY, lcSysExtX, lcSysExtY

        // open the tablet context
        // The Wintab spec says we must open the context disabled if we are using cursor masks.
        let hctx = wtopena(window_handel.0, &mut log_context as *mut _, 0);
        println!("HCTX on open is {:?}", hctx);

        let mut message = MSG::default();
        
        
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // The original header used macro templates to generate these, because apparently it isnt allowed to be easy.
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                assert!(ValidateRect(window, None).as_bool());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                // TODO: close the context properly with WTClose
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SETCURSOR
            | WM_LBUTTONDOWN
            | WM_LBUTTONUP
            | WM_GETMINMAXINFO
            | WM_CREATE
            | WM_SHOWWINDOW
            | WM_NCCREATE
            | WM_NCHITTEST
            | WM_IME_SETCONTEXT
            | WM_IME_NOTIFY
            | WM_KILLFOCUS
            | WM_ACTIVATE
            | WM_CLOSE
            | WM_NCMOUSEMOVE
            | WM_NCLBUTTONDOWN
            | WM_MOUSEFIRST
            | WM_ACTIVATEAPP
            | WM_NCMOUSELEAVE
            | WM_GETICON
            | WM_GETOBJECT
            | WM_WINDOWPOSCHANGING
            | WM_SIZE
            | WM_MOVE
            | WM_DWMNCRENDERINGCHANGED
            | WM_WINDOWPOSCHANGED
            | WM_MOVING
            | WM_CAPTURECHANGED
            | WM_EXITSIZEMOVE
            | WM_NCPAINT
            | WM_ERASEBKGND => {
                DefWindowProcA(window, message, wparam, lparam)
            }
            // WINTAB EVENTS
            WT::PACKET => {
                println!("GOT A WT_PACKET! Yay!");
                let mut packet = Packet::default();

                // Have load the dynamic library again because I can't seem persude rust make this a `mut static`
                // variable. Sure, cool, great. Thanks for that rust. I sure hope this processes is cached or memoized
                // internally somewhere.
                let lib = libloading::Library::new("wintab32.dll").unwrap();
                let wtpacket: libloading::Symbol<WTPacket> = match lib.get(c"WTPacket".to_bytes()){
                    Ok(symbol) => symbol,
                    Err(err)   => panic!("Failed get wtpacket symbol :( {err}"),
                };

                let wtpacket_response =  wtpacket(lparam.0 as *mut HCTX, wparam.0 as u32, cast_void!(packet));
                assert!(wtpacket_response!=0);
                println!("HCTX recieved as lparam is {:?}", lparam.0);
                println!("Got a packet! Whooo! {:#?}", packet);

                DefWindowProcA(window, message, wparam, lparam)
            }
            x => {
                println!("Something else? {:?}", x);
                DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }
}
