/**
    Window Class Structures (Windows) / WNDCLASSEX structure (Windows)
**/
extern crate std;

use super::super::prelude::{
    UINT , WNDPROC , CCINT , HINSTANCE , HICON , 
    HCURSOR , HBRUSH , LPCTSTR , wapi , 
    ToWindowTextConvertion , Application , Cursor ,
    Icon , Text , WindowProcedure , Atom ,
    WindowService , Brush , WindowClassStyles , WindowClassStyle
};

pub struct WindowClassExtra {
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

pub type WNDCLASSEX = WindowClassExtra;

impl WindowClassExtra {
    pub fn new() -> WindowClassExtra {
        WindowClassExtra {
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

    pub fn setSize(&mut self , size : u32) {
        self.cbSize = size;
    }

    pub fn setStyle(&mut self , style : WindowClassStyle) {
        self.style = style;
    }

    pub fn setWindowProcedure(&mut self , proce : WindowProcedure) {
        self.lpfnWndProc = proce;
    }

    pub fn setClassExtraSize(&mut self , size : i32) {
        self.cbClsExtra = size;
    }

    pub fn setWindowExtraSize(&mut self , size : i32) {
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

    pub fn setSmallIcon(&mut self , icon : Icon) {
        self.hIconSm = icon;
    }

    pub fn register(&self) -> Atom {
        unsafe {
            wapi::WindowClass::RegisterClassExW(self as *const WNDCLASSEX)
        }
    }

    pub fn unregister(&self , app : Option<Application>) -> bool {
        WindowService::UnregisterClass(self.lpszClassName , app)
    }
}