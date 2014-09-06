extern crate std;

use super::super::prelude::{
    HWND , UINT , WPARAM , LPARAM , DWORD , POINT , Point , wapi ,
    Window , MessageService , ToRustBoolConvertion
};

pub struct Message {
       hwnd : HWND      , 
    message : UINT      , 
     wParam : WPARAM    , 
     lParam : LPARAM    , 
       time : DWORD     , 
     pub pt : POINT     , 
}

pub type MSG = Message;

// WTypes.h:980 => typedef struct tagMSG *LPMSG;
pub type LPMSG = * MSG;

impl Message {
    pub fn new() -> Message {
        Message {
               hwnd : std::ptr::mut_null()  ,
            message : 0                     ,
             wParam : 0                     ,
             lParam : 0                     ,
               time : 0                     , 
                 pt : Point {
                     x : 0  , 
                     y : 0
                 }                          ,
        }
    }

    pub fn GetMessage(&mut self             ,
           window : Option<Window>      , 
        minFilter : Option<UINT>        ,
        maxFilter : Option<UINT>
    ) -> bool {
        MessageService::GetMessage(self , window , minFilter , maxFilter)
    }

    /**
        just quick call
    **/
    pub fn GetAnyOwnedMessage(&mut self , window : Window) -> bool {
        MessageService::GetMessage(
            self         , 
            Some(window) , 
            None         , 
            None
        )
    }

    pub fn GetPostedMessage(&mut self) -> bool {
        MessageService::GetMessage(
            self     , 
            //FIXME: &-1 as Window
            Some(-1 as Window) , 
            None     , 
            None
        )
    }

    pub fn TranslateMessage(&self) -> bool {
        unsafe {
            wapi::Message::TranslateMessage(self as *const MSG).bool()
        }
    }

    pub fn DispatchMessage(&self) -> bool {
        unsafe {
            wapi::Message::DispatchMessageW(self as *const MSG).bool()
        }
    }
}