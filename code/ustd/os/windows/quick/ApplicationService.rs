extern crate std;

use ustd::os::windows::dev::ui::Application::Application;
use ustd::os::windows::dev::ui::Text::Text;

use ustd::os::windows::dev::ss::service::DllService;
use ustd::os::windows::dev::ui::service::DialogBoxService;
use ustd::os::windows::dev::ui::etypes::{
    DialogBoxCommand , MessageBoxStyle
};

pub fn Application() -> Application {
    DllService::GetModuleHandle(None)
}

pub fn MessageBox(
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> DialogBoxCommand
{
    DialogBoxService::MessageBox(None , text , title , style)
}