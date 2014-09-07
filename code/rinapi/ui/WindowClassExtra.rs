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

#[repr(C)]
pub struct WindowClassExtra {
           cbSize : UINT        , /* UINT */
            style : UINT        , /* WindowClassStyle */
      lpfnWndProc : WNDPROC     , /* WindowProcedure */
       cbClsExtra : CCINT       , /* UINT */
       cbWndExtra : CCINT       , /* UINT */
        hInstance : HINSTANCE   , /* Application */
            hIcon : HICON       , /* Icon */
          hCursor : HCURSOR     , /* Cursor */
    hbrBackground : HBRUSH      , /* Brush */
     lpszMenuName : LPCTSTR     , /* Text */
    lpszClassName : LPCTSTR     , /* Text */
          hIconSm : HICON       , /* Icon */
}

pub struct WindowClassExtraLayout {
    pub        class_size : Option<UINT>                , 
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
    pub        small_icon : Option<Icon>                ,
}

pub type WNDCLASSEX = WindowClassExtra;

impl WindowClassExtraLayout {
    pub fn asWindowClass(&self) -> WindowClassExtra {
        WindowClassExtra::new(self)
    }
}

impl WindowClassExtra {
    pub fn new(layout : &WindowClassExtraLayout) -> WindowClassExtra {
        let default_style = WindowClassStyles::VerticalRedraw | 
                            WindowClassStyles::HorizontalRedraw;
        let exclass_size = std::mem::size_of::<WindowClassExtra>() as UINT;

        WindowClassExtra {
                   cbSize : layout.class_size.unwrap_or(exclass_size)           ,
                    style : layout.class_style.unwrap_or(default_style)         , 
              lpfnWndProc : layout.window_procedure                             ,
               cbClsExtra : layout.class_extra_size                             , 
               cbWndExtra : layout.window_extra_size                            , 
                hInstance : layout.application                                  , 
                    hIcon : layout.icon.unwrap_or(std::ptr::mut_null())         , 
                  hCursor : layout.cursor.unwrap_or(std::ptr::mut_null())       ,
            hbrBackground : layout.background.unwrap_or(std::ptr::mut_null())   , 
             lpszMenuName : layout.menu_name.unwrap_or(std::ptr::null())        , 
            lpszClassName : layout.class_name                                   ,
                  hIconSm : layout.small_icon.unwrap_or(std::ptr::mut_null())   
        }
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