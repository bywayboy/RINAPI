extern crate std;

use super::super::prelude::{
    wapi , ToRustBoolConvertion , LPMSG ,
    Message , Window , UINT , ToRustBoolConvertion
};

pub fn GetMessage(
      message : &Message        ,
       window : Option<Window>      , 
    minFilter : Option<UINT>        ,
    maxFilter : Option<UINT>

) -> bool {
    unsafe {
        wapi::Message::GetMessageW(
            message as LPMSG, 
            window.unwrap_or(std::ptr::mut_null()) , 
            minFilter.unwrap_or(0) , 
            maxFilter.unwrap_or(0)
        ).bool()
    }
}