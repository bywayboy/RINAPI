extern crate std;
extern crate libc;

use ustd::os::windows::common::types::win32::{
    LPCTSTR , DWORD , DWORD , CCINT , HWND , HMENU , HINSTANCE , LPVOID
};
use self::libc::types::common::c95::c_void;

use ustd::ui::os::windows::types::Window::Window;
use ustd::ui::os::windows::types::Application::Application;
use ustd::ui::os::windows::types::Menu::Menu;

use ustd::ui::os::windows::types::Constants::WindowStyle;

use ustd::os::windows::winapi;

pub mod CreateWindowOptions {
    // WinUser.h:3839 => #define CW_USEDEFAULT ((int)0x80000000)
    pub static UseDefault : int = 0x80000000;
}

pub type Atom = ATOM;

pub fn CreateWindow(
           className : Option<Text> ,
          windowName : Option<Text> ,
           drawStyle : WindowStyle ,
                   x : int ,
                   y : int ,
               width : int ,
              height : int ,
        parentWindow : Option<Window> ,
                menu : Option<Menu> ,
            instance : Option<Instance> ,
               param : Option<c_void>
) -> Window 
{
    std::ptr::mut_null()
    /*unsafe {
        WindowsAPI::CreateWindow()
    }*/
}

pub fn UnregisterClass(name : Text , app : Option<Application>) -> bool {
    unsafe {
        winapi::WindowClass::UnregisterClass(
            name , 
            app.unwrap_or(std::ptr::mut_null())
        ).bool()
    }
}