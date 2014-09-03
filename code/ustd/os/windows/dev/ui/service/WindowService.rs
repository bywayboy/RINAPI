extern crate std;

use ustd::os::windows::common::types::win32::{
    LPVOID , CCINT
};

use ustd::os::windows::common::types::convertion::ToRustBoolConvertion;

use ustd::os::windows::dev::ui::Window::Window;
use ustd::os::windows::dev::ui::Application::Application;
use ustd::os::windows::dev::ui::Menu::Menu;
use ustd::os::windows::dev::ui::Text::Text;

use ustd::os::windows::dev::ui::etypes::ExtendedWindowStyle;
use ustd::os::windows::dev::ui::etypes::WindowStyle;

use ustd::os::windows::winapi;

pub mod CreateWindowOptions {
    use ustd::os::windows::common::types::win32::{
        CCINT
    };
    // WinUser.h:3839 => #define CW_USEDEFAULT ((int)0x80000000)
    pub static UseDefault : CCINT = 0x80000000;
}

pub fn CreateWindow(
           className : Option<Text>         ,
          windowName : Option<Text>         ,
           drawStyle : WindowStyle          ,
                   x : CCINT                  ,
                   y : CCINT                  ,
               width : CCINT                  ,
              height : CCINT                  ,
        parentWindow : Option<Window>       ,
                menu : Option<Menu>         ,
            instance : Option<Application>  ,
               param : Option<LPVOID>
) -> Window 
{
    CreateWindowEx(
        0,
        className , 
        windowName , 
        drawStyle , 
        x , 
        y , 
        width , 
        height , 
        parentWindow , 
        menu , 
        instance , 
        param
    )
}

pub fn CreateWindowEx(
           dwExStyle : ExtendedWindowStyle ,
           className : Option<Text>         ,
          windowName : Option<Text>         ,
           drawStyle : WindowStyle          ,
                   x : CCINT                  ,
                   y : CCINT                  ,
               width : CCINT                  ,
              height : CCINT                  ,
        parentWindow : Option<Window>       ,
                menu : Option<Menu>         ,
            instance : Option<Application>  ,
               param : Option<LPVOID>
) -> Window 
{
    unsafe {
        winapi::Window::CreateWindowExW(
            dwExStyle ,
            className.unwrap_or(std::ptr::null()) ,
            windowName.unwrap_or(std::ptr::null()) ,
            drawStyle , 
            x , 
            y ,
            width , 
            height , 
            parentWindow.unwrap_or(std::ptr::mut_null()) ,
            menu.unwrap_or(std::ptr::mut_null()) ,
            instance.unwrap_or(std::ptr::mut_null()) ,
            param.unwrap_or(std::ptr::mut_null())
        )
    }
}

pub fn UnregisterClass(name : Text , app : Option<Application>) -> bool {
    unsafe {
        winapi::WindowClass::UnregisterClassW(
            name , 
            app.unwrap_or(std::ptr::mut_null())
        ).bool()
    }
}