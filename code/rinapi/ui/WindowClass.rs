/**
    Window Class Structures (Windows) / WNDCLASS structure (Windows)
**/
extern crate std;

use super::super::prelude::{
    UINT , WNDPROC , CCINT , HINSTANCE , HICON , HCURSOR , 
    HBRUSH , LPCTSTR , wapi , ToWindowTextConvertion , 
    Application , Cursor , Icon , Text , WindowProcedure , 
    Atom , Brush , WindowService , WindowClassStyles , 
    WindowClassStyle
};

pub struct WindowClass {
    raw: RawWindowClass
}

struct RawWindowClass {
            style : UINT        , /* WindowClassStyle */
      lpfnWndProc : WNDPROC     , /* WindowProcedure */
       cbClsExtra : CCINT       , /* i32 */
       cbWndExtra : CCINT       , /* i32 */
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
                  lpfnWndProc : std::ptr::null()                                                        , 
                   cbClsExtra : 0                                                                       ,
                   cbWndExtra : 0                                                                       ,
                    hInstance : std::ptr::mut_null()                                                    ,
                        hIcon : std::ptr::mut_null()                                                    ,
                      hCursor : std::ptr::mut_null()                                                    ,
                hbrBackground : std::ptr::mut_null()                                                    ,
                 lpszMenuName : "Application Menu".asText().as_ptr()                                    ,
                lpszClassName : "Class".asText().as_ptr()                                               ,
            }
        }
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

    pub fn register(&self) -> Atom {
        unsafe {
            wapi::WindowClass::RegisterClassW(&self.raw)
        }
    }

    pub fn unregister(&self , app : Option<Application>) -> bool {
        WindowService::UnregisterClass(self.raw.lpszClassName , app)
    }
}