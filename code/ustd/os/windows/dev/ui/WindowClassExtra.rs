/**
    Window Class Structures (Windows) / WNDCLASSEX structure (Windows)
**/
extern crate std;

use ustd::os::windows::winapi;

use ustd::os::windows::common::types::win32::{
    UINT , WNDPROC , CCINT , HINSTANCE , HICON , HCURSOR , HBRUSH , LPCTSTR
};

use ustd::os::windows::common::types::convertion::ToWindowTextConvertion;

use ustd::os::windows::dev::ui::Application::Application;
use ustd::os::windows::dev::ui::Cursor::Cursor;
use ustd::os::windows::dev::ui::Icon::Icon;
use ustd::os::windows::dev::ui::Text::Text;

use ustd::os::windows::dev::ui::utypes::WindowProcedure;
use ustd::os::windows::dev::ui::utypes::Atom;

use ustd::os::windows::dev::ui::service::WindowService;

use ustd::os::windows::gdi::Brush::Brush;

use ustd::os::windows::dev::ui::enums::WindowClassStyles;
use ustd::os::windows::dev::ui::etypes::WindowClassStyle;

pub struct WindowClassExtra {
    raw : RawWindowClassExtra
}

struct RawWindowClassExtra {
           cbSize : UINT        , /* u32 */
            style : UINT        , /* WindowClassStyle */
      lpfnWndProc : WNDPROC     , /* WindowProcedure */
       cbClsExtra : CCINT       , /* int */
       cbWndExtra : CCINT       , /* int */
        hInstance : HINSTANCE   , /* Application */
            hIcon : HICON       , /* Icon */
          hCursor : HCURSOR     , /* Cursor */
    hbrBackground : HBRUSH      , /* Brush */
     lpszMenuName : LPCTSTR     , /* Text */
    lpszClassName : LPCTSTR     , /* Text */
          hIconSm : HICON       , /* Icon */
}

pub type WNDCLASSEX = RawWindowClassExtra;

impl WindowClassExtra {
    pub fn new() -> WindowClassExtra {
        WindowClassExtra {
            raw : RawWindowClassExtra {
                       cbSize : std::mem::size_of::<WNDCLASSEX>() as UINT   ,
                        style : WindowClassStyles::VerticalRedraw   | 
                                WindowClassStyles::HorizontalRedraw         , 
                  lpfnWndProc : std::ptr::null()                            , 
                   cbClsExtra : 0                                           ,
                   cbWndExtra : 0                                           ,
                    hInstance : std::ptr::mut_null()                        ,
                        hIcon : std::ptr::mut_null()                        ,
                      hCursor : std::ptr::mut_null()                        ,
                hbrBackground : std::ptr::mut_null()                        ,
                 lpszMenuName : "Application Menu".asText().as_ptr()        ,
                lpszClassName : "Class".asText().as_ptr()                   ,
                      hIconSm : std::ptr::mut_null()                        ,
            }
        }
    }

    pub fn setSize(&mut self , size : u32) {
        self.raw.cbSize = size;
    }

    pub fn setStyle(&mut self , style : WindowClassStyle) {
        self.raw.style = style;
    }

    pub fn setWindowProcedure(&mut self , proce : WindowProcedure) {
        self.raw.lpfnWndProc = proce;
    }

    pub fn setClassExtraSize(&mut self , size : i32) {
        self.raw.cbClsExtra = size;
    }

    pub fn setWindowExtraSize(&mut self , size : i32) {
        self.raw.cbWndExtra = size;
    }

    pub fn setApplication(&mut self , app : Application) {
        self.raw.hInstance = app;
    }

    pub fn setIcon(&mut self , icon : Icon) {
        self.raw.hIcon = icon;
    }

    pub fn setCursor(&mut self , cursor : Cursor) {
        self.raw.hCursor = cursor;
    }

    pub fn setBackground(&mut self , bg : Brush) {
        self.raw.hbrBackground = bg;
    }

    pub fn setMenuName(&mut self , name : Text) {
        self.raw.lpszMenuName = name;
    }

    pub fn setClassName(&mut self , name : Text) {
        self.raw.lpszClassName = name;
    }

    pub fn setSmallIcon(&mut self , icon : Icon) {
        self.raw.hIconSm = icon;
    }

    pub fn register(&self) -> Atom {
        unsafe {
            winapi::WindowClass::RegisterClassExW(&self.raw)
        }
    }

    pub fn unregister(&self , app : Option<Application>) -> bool {
        WindowService::UnregisterClass(self.raw.lpszClassName , app)
    }
}