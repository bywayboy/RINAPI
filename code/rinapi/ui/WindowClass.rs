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
            style : UINT        , /* WindowClassStyle */
      lpfnWndProc : WNDPROC     , /* WindowProcedure */
       cbClsExtra : CCINT       , /* CCINT */
       cbWndExtra : CCINT       , /* CCINT */
        hInstance : HINSTANCE   , /* Application */
            hIcon : HICON       , /* Icon */
          hCursor : HCURSOR     , /* Cursor */
    hbrBackground : HBRUSH      , /* Brush */
     lpszMenuName : LPCTSTR     , /* Text */
    lpszClassName : LPCTSTR     , /* Text */
}

pub type WNDCLASS = WindowClass;

impl WindowClass {
    pub fn new() -> WindowClass {
        WindowClass {
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

    pub fn setStyle(&mut self , style : WindowClassStyle) {
        self.style = style;
    }

    pub fn setWindowProcedure(&mut self , proce : WindowProcedure) {
        self.lpfnWndProc = proce;
    }

    pub fn setClassExtraSize(&mut self , size : CCINT) {
        self.cbClsExtra = size;
    }

    pub fn setWindowExtraSize(&mut self , size : CCINT) {
        self.cbWndExtra = size;
    }

    pub fn setApplication(&mut self , app : Application) {
        self.hInstance = app;
    }

    pub fn setIcon(&mut self , icon : Icon) {
        self.hIcon = icon;
    }

    pub fn setCursor(&mut self , cursor : Cursor) {
        self.hCursor = cursor;
    }

    pub fn setBackground(&mut self , bg : Brush) {
        self.hbrBackground = bg;
    }

    pub fn setMenuName(&mut self , name : Text) {
        self.lpszMenuName = name;
    }

    pub fn setClassName(&mut self , name : Text) {
        self.lpszClassName = name;
    }

    pub fn RegisterClass(&self) -> Atom {
        unsafe {
            wapi::WindowClass::RegisterClassW(self as *const WNDCLASS)
        }
    }

    pub fn unregister(&self , app : Option<Application>) -> bool {
        WindowService::UnregisterClass(self.lpszClassName , app)
    }
}