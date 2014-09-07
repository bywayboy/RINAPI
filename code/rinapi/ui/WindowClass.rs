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

#[repr(C)]
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

pub struct WindowClassLayout {
    pub       class_style : Option<WindowClassStyle>    ,
    pub  window_procedure : WindowProcedure             ,
    pub  class_extra_size : CCINT                       ,
    pub window_extra_size : CCINT                       ,
    pub       application : Application                 ,
    pub              icon : Option<Icon>                ,
    pub            cursor : Option<Cursor>              ,
    pub        background : Option<Brush>               ,
    pub         menu_name : Option<Text>                ,
    pub        class_name : Text                        ,
}

pub type WNDCLASS = WindowClass;

impl WindowClassLayout {
    pub fn asWindowClass(&self) -> WindowClass {
        WindowClass::new(self)
    }
}

impl WindowClass {
    pub fn new(layout : &WindowClassLayout) -> WindowClass {
        let default_style = WindowClassStyles::VerticalRedraw | 
                            WindowClassStyles::HorizontalRedraw;

        WindowClass {
                    style : layout.class_style.unwrap_or(default_style)         , 
              lpfnWndProc : layout.window_procedure                             ,
               cbClsExtra : layout.class_extra_size                             , 
               cbWndExtra : layout.window_extra_size                            , 
                hInstance : layout.application                                  , 
                    hIcon : layout.icon.unwrap_or(std::ptr::mut_null())         , 
                  hCursor : layout.cursor.unwrap_or(std::ptr::mut_null())       ,
            hbrBackground : layout.background.unwrap_or(std::ptr::mut_null())   , 
             lpszMenuName : layout.menu_name.unwrap_or(std::ptr::null())        , 
            lpszClassName : layout.class_name
        }
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