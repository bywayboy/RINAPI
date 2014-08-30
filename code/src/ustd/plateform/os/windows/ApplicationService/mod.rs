use ustd::ui::os::windows::types::Application::Application;
use utsd::plateform::os::windows::DynamicLinkLibraryService;

mod WindowsAPI;

pub fn Application() -> Application {
    DynamicLinkLibraryService::GetModuleHandle(None);
}

pub fn MessageBox(
      app : Option<Application> , 
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> MessageBoxResult
{
    WindowsAPI::MessageBox(
        app.unwrap_or(std::ptr::mut_null())     , 
        text.unwrap_or(std::ptr::mut_null())    ,
        title.unwrap_or(std::ptr::mut_null())   ,
        style
    )
}