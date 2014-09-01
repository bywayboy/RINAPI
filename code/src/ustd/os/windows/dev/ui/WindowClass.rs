/**
    Window Class Structures (Windows) / WNDCLASS structure (Windows)
**/
extern crate std;

use ustd::os::windows::winapi;

use ustd::os::windows::common::types::win32::{
    UINT , WNDPROC , CCINT , HINSTANCE , HICON , HCURSOR , HBRUSH , LPCTSTR
};

use ustd::os::windows::dev::ui::{
    Application , Cursor , Icon , Menu , Text , WindowProcedure
};

use ustd::os::windows::dev::ui::Atom;

use ustd::os::windows::gdi::Brush;

use ustd::os::windows::dev::ui::service::WindowService;

use ustd::os::windows::dev::ui::enums::WindowClassStyles;
use ustd::os::windows::dev::ui::etypes::WindowClassStyle;

pub struct WindowClass {
    raw: RawWindowClass
}

struct RawWindowClass {
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
}

pub type WNDCLASS = RawWindowClass;

impl WindowClass {
    pub fn new() -> WindowClass {
        WindowClass {
            raw : RawWindowClass {
                        style : WindowClassStyles::VerticalRedraw | WindowClassStyles::HorizontalRedraw , 
                  lpfnWndProc : std::ptr::mut_null()                                                  , 
                   cbClsExtra : 0                                                                     ,
                   cbWndExtra : 0                                                                     ,
                    hInstance : std::ptr::mut_null()                                                  ,
                        hIcon : std::ptr::mut_null()                                                  ,
                      hCursor : std::ptr::mut_null()                                                  ,
                hbrBackground : std::ptr::mut_null()                                                  ,
                 lpszMenuName : "Application Menu".asText()                                           ,
                lpszClassName : "Class".asText()                                                      ,
            }
        }
    }

    pub fn setStyle(&mut self , style : WindowClassStyle) {
        self.raw.style = style;
    }

    pub fn setWindowProcedure(&mut self , proce : WindowProcedure) {
        self.raw.lpfnWndProc = proce;
    }

    pub fn setClassExtraSize(&mut self , size : int) {
        self.raw.cbClsExtra = size;
    }

    pub fn setWindowExtraSize(&mut self , size : int) {
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

    pub fn register(&self) -> Atom {
        unsafe {
            winapi::WindowClass::RegisterClass(self)
        }
    }

    pub fn unregister(&self , app : Option<Application>) -> bool {
        WindowService::UnregisterClass(self.raw.lpszClassName , app)
    }
}