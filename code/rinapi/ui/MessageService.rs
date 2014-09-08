extern crate std;

use super::super::prelude::{
    wapi , ToRustBoolConvertion , LPMSG , CCINT ,
    Message , Window , UINT , ToRustBoolConvertion
};

pub fn GetMessage(
      message : &mut Message        ,
       window : Option<Window>      , 
    minFilter : Option<UINT>        ,
    maxFilter : Option<UINT>

) -> bool {
    unsafe {
        wapi::Message::GetMessageW(
            message, 
            window.unwrap_or(std::ptr::mut_null()) , 
            minFilter.unwrap_or(0) , 
            maxFilter.unwrap_or(0)
        ).bool()
    }
}

/**
    [A]PostQuitMessageW
**/
pub fn postQuit(nExitCode : CCINT) {
    unsafe {
        wapi::Message::PostQuitMessage(nExitCode);
    }
}