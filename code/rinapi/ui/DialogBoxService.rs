extern crate std;

use super::super::prelude::{
    wapi , 
    Window , Text , DialogBoxCommand , MessageBoxStyle
};

pub fn MessageBox(
      app : Option<Window> , 
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> DialogBoxCommand
{
    unsafe {
        wapi::DialogBox::MessageBoxW(
            app.unwrap_or(std::ptr::mut_null())     , 
            text.unwrap_or(std::ptr::null())    ,
            title.unwrap_or(std::ptr::null())   ,
            style
        )
    }
}