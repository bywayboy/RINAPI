extern crate std;

use ustd::os::windows::dev::ui::{
    Application , Text
};
use ustd::os::windows::dev::ss::service::DllService;
use ustd::os::windows::dev::ui::etypes::{
    DialogBoxCommand , MessageBoxStyle
};

use ustd::os::windows::winapi;

pub fn Application() -> Application {
    DllService::GetModuleHandle(None)
}

pub fn MessageBox(
      app : Option<Application> , 
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> DialogBoxCommand
{
    ApiMessageBox(app , text , title , style)
}

pub fn ApiMessageBox(
      app : Option<Application> , 
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> DialogBoxCommand
{
    unsafe {
        winapi::DialogBox::MessageBoxW(
            app.unwrap_or(std::ptr::mut_null())     , 
            text.unwrap_or(std::ptr::mut_null())    ,
            title.unwrap_or(std::ptr::mut_null())   ,
            style
        )
    }
}