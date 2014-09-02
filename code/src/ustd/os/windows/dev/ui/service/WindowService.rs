extern crate std;

use ustd::os::windows::common::types::win32::{
    LPCTSTR , DWORD , DWORD , CCINT , HWND , HMENU , HINSTANCE , LPVOID , ATOM
};

use ustd::os::windows::common::types::convertion::ToRustBoolConvertion;

use ustd::os::windows::dev::ui::Window::Window;
use ustd::os::windows::dev::ui::Application::Application;
use ustd::os::windows::dev::ui::Menu::Menu;
use ustd::os::windows::dev::ui::Text::Text;

use ustd::os::windows::dev::ui::etypes::WindowStyle;

use ustd::os::windows::winapi;

pub mod CreateWindowOptions {
    // WinUser.h:3839 => #define CW_USEDEFAULT ((int)0x80000000)
    pub static UseDefault : int = 0x80000000;
}

pub fn CreateWindow(
           className : Option<Text>         ,
          windowName : Option<Text>         ,
           drawStyle : WindowStyle          ,
                   x : int                  ,
                   y : int                  ,
               width : int                  ,
              height : int                  ,
        parentWindow : Option<Window>       ,
                menu : Option<Menu>         ,
            instance : Option<Application>  ,
               param : Option<LPVOID>
) -> Window 
{
    std::ptr::mut_null()
    /*unsafe {
        WindowsAPI::CreateWindow()
    }*/
}

pub fn UnregisterClass(name : Text , app : Option<Application>) -> bool {
    unsafe {
        winapi::WindowClass::UnregisterClassW(
            name , 
            app.unwrap_or(std::ptr::mut_null())
        ).bool()
    }
}