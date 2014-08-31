use ustd::ui::os::windows::types::Application::Application;
use utsd::os::windows::dev::ss::service::DllService;

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
    unsafe {
        api::DialogBox::MessageBoxW(
            app.unwrap_or(std::ptr::mut_null())     , 
            text.unwrap_or(std::ptr::mut_null())    ,
            title.unwrap_or(std::ptr::mut_null())   ,
            style
        )
    }
}