extern crate std;

use ustd::os::windows::dev::ui::Window::Window;
use ustd::os::windows::dev::ui::Text::Text;

use ustd::os::windows::dev::ui::etypes::{
    DialogBoxCommand , MessageBoxStyle
};

use ustd::os::windows::winapi;

pub fn MessageBox(
      app : Option<Window> , 
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> DialogBoxCommand
{
    unsafe {
        winapi::DialogBox::MessageBoxW(
            app.unwrap_or(std::ptr::mut_null())     , 
            text.unwrap_or(std::ptr::null())    ,
            title.unwrap_or(std::ptr::null())   ,
            style
        )
    }
}