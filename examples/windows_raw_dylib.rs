//! This example demonstrates the use of wintab_lite using the windows crate directly
//! 
//! This one is pretty messy and only outputs to the terminal. I recommend you look at the winit_libloading example


use windows::{
    core::s,
    Win32::{
        Foundation::{
            HWND, LPARAM, LRESULT, POINT, RECT, WPARAM
        },
        Graphics::Gdi::{
            BeginPaint, ClientToScreen, Ellipse, EndPaint, FillRect, GetStockObject, InvalidateRect, RedrawWindow, ScreenToClient, SelectObject, ValidateRect, BLACK_PEN, HBRUSH, HRGN, PAINTSTRUCT, RDW_INTERNALPAINT, WHITE_BRUSH
        },
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::*
    },
};

use anyhow::Result;

use wintab_lite::{
    cast_void,
    Packet,
    AXIS,
    CXO,
    DVC,
    HCTX,
    LOGCONTEXT,
    WT,
    WTI,
    WTPKT, XYZ,
};

static mut CONTEXT_HANDEL:*mut HCTX = std::ptr::null_mut();
static mut X:i32 = 0;
static mut Y:i32 = 0;
static mut P:u32 = 0;
static mut REDRAW:bool = false;

fn main() -> Result<()> {
    unsafe {

        let window_module_handle = GetModuleHandleA(None)?;
        debug_assert!(window_module_handle.0 != 0);

        let window_class_name = s!("window");

        let window_class = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: window_module_handle.into(),
            lpszClassName: window_class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_procedure),
            ..Default::default()
        };

        let atom = RegisterClassA(&window_class);
        debug_assert!(atom != 0);

        let window_handel = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class_name,
            s!("wintab_lite example: windows_raw_raylib"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE|WS_CAPTION | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            window_module_handle,
            None,
        );
        
        // create the context object which configures our connection to wintab
        let mut log_context = LOGCONTEXT::default();
        log_context.lcOptions.insert(CXO::SYSTEM);

        let wtinfoa_return_value = wintab_lite::info(WTI::DEFCONTEXT, 0, cast_void!(log_context));
        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<LOGCONTEXT>());
        assert!(log_context.lcOptions.contains(CXO::SYSTEM));
        
        // not sure if there is a need to do this;
        log_context.lcName.write_str(format!("PrsTest Digitizing {:?}", window_module_handle.0).as_str());

        // Set the packet format These are the defaults anyway;
        log_context.lcPktData = WTPKT::all(); // If all() is not used, then you must define your own custom packet struct
        log_context.lcPktMode = WTPKT::empty();

        // Set which events cause a packet
        log_context.lcMoveMask = WTPKT::STATUS | WTPKT::X | WTPKT::Y | WTPKT::NORMAL_PRESSURE;

        // Which buttons events will be handled by this context.  lcBtnMask
        // is a bitfield with one bit per button.
        log_context.lcBtnUpMask = log_context.lcBtnDnMask;

        // Retrieve axis information
        let mut tablet_x = AXIS::default();
        let result = wintab_lite::info(WTI::DEVICES, DVC::X as u32, cast_void!(tablet_x));
        assert_eq!(result as usize, std::mem::size_of::<AXIS>());
        
        let mut tablet_y = AXIS::default();
        let result = wintab_lite::info(WTI::DEVICES, DVC::Y as u32, cast_void!(tablet_y));
        assert_eq!(result as usize, std::mem::size_of::<AXIS>());


        // ======================================
        // configure the context.

        // I found this was a redundant assignment when testing:
        log_context.lcInOrgXYZ = XYZ::default();
        // found this is a redundant assignment when testing:
        log_context.lcInExtXYZ = XYZ {
            x: tablet_x.axMax,
            y: tablet_y.axMax,
            z: 0,
        };

        // =======================================================
        // Tablet output coordinates are upside down by default 🙃
        let default_y_extent = log_context.lcOutExtXYZ.y;
        log_context.lcOutExtXYZ.y = -default_y_extent;

        log_context.lcInOrgXYZ.x = 0;
        log_context.lcInOrgXYZ.y = 0;
        log_context.lcInExtXYZ.x = tablet_x.axMax;
        log_context.lcInExtXYZ.y = tablet_y.axMax;

        // apparently this is supposed to be done in the manifest tho?
        

        // Guarantee the output coordinate space to be in screen coordinates.
        log_context.lcOutOrgXYZ.x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        log_context.lcOutOrgXYZ.y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        log_context.lcOutExtXYZ.x = GetSystemMetrics(SM_CXVIRTUALSCREEN);

        // In Wintab, the tablet origin is lower left.  Move origin to upper left
        // so that it coincides with screen origin.
        log_context.lcOutExtXYZ.y = -GetSystemMetrics(SM_CYVIRTUALSCREEN);

        // Leave the system origin and extents as received:
        // lcSysOrgX, lcSysOrgY, lcSysExtX, lcSysExtY
        println!("{log_context:#?}\n{tablet_x:?}\n{tablet_y:?}");
        // open the tablet context
        // The Wintab spec says we must open the context disabled if we are using cursor masks.
        CONTEXT_HANDEL = wintab_lite::open(window_handel.0, &mut log_context as *mut _, 0);
        println!("HCTX on open is {:?}", CONTEXT_HANDEL);

        let mut message = MSG::default();
        
        
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
    Ok(())
}

extern "system" fn window_procedure(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                //assert!(ValidateRect(window, None).as_bool());
                assert!(InvalidateRect(window, None, false).as_bool());

                let mut client_rect: RECT = RECT::default();
                GetClientRect(window, &mut client_rect).unwrap();
                // Transform the tablet input into client window coordinates
                let mut inner_position = POINT::default();
                assert!(ClientToScreen(window, &mut inner_position).as_bool());
                
                
                let tx = X - inner_position.x;
                let ty = Y - inner_position.y;
                
                println!("{client_rect:?} {inner_position:?} {X} {Y} {tx} {ty}");

                // create a paint context objet thingo
                let mut paint_struct: PAINTSTRUCT = PAINTSTRUCT::default();

                // start painting
                let hdc = BeginPaint(window, &mut paint_struct);

                if REDRAW {
                    REDRAW = false;
                    
                    let brush = HBRUSH(GetStockObject(WHITE_BRUSH).0);
                    FillRect(hdc, &client_rect, brush);
                }

                //assert!(PatBlt(hdc, tx,   ty-15,1 ,30, BLACKNESS).as_bool());
                //assert!(PatBlt(hdc, tx-15,ty   ,30, 1, BLACKNESS).as_bool());
                let size: i32 = P as i32 / 150;
                SelectObject(hdc, GetStockObject(BLACK_PEN));
                SelectObject(hdc, GetStockObject(WHITE_BRUSH));
                assert!(Ellipse(hdc, tx - size, ty - size, tx + size, ty + size).as_bool());
                assert!(EndPaint(window, &mut paint_struct).as_bool());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                // let lib = libloading::Library::new("wintab32.dll").unwrap();
                // let wtclose: libloading::Symbol<WTClose> = lib.get(c"WTClose".to_bytes()).unwrap();
                match wintab_lite::close(CONTEXT_HANDEL){
                    0 => println!("WTClose FAILED!"),
                    _ => println!("WTClose SUCCESS!"),
                };
                PostQuitMessage(0);
                LRESULT(0)
            }
            WT::PACKET => {
                // Confirm that we have received the expected context handel via lparam
                // this check is not required
                assert_eq!(CONTEXT_HANDEL, lparam.0 as *mut HCTX);
                
                let mut packet = Packet::default();
                let packet_id= wparam.0 as u32;
                assert_ne!(
                    wintab_lite::packet(CONTEXT_HANDEL, packet_id, cast_void!(packet)),
                    0
                );
                X = packet.pkXYZ.x;
                Y = packet.pkXYZ.y;
                P = packet.pkNormalPressure;
                if P>0 {
                    //println!("{X} {Y} {P}");
                    assert!(RedrawWindow(window, None, HRGN(0), RDW_INTERNALPAINT).as_bool());
                }
                // println!("Got a packet! {:#?}", packet);
                DefWindowProcA(window, message, wparam, lparam)
            }
            _ => {
                DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }
}
