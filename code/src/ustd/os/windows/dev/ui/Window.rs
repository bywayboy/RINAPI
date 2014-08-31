use ustd::os::windows::winapi;
use ustd::os::windows::common::types::win32::{
    HWND , CCINT , BOOL , CBoolean2Rust
};
use ustd::os::windows::dev::ui::etypes::WindowShowStyleCommand;

pub type Window = HWND;

trait WindowFunction {
    fn ShowWindow(&self , style : WindowShowStyleCommand) -> bool;
}

impl WindowFunction for Window {
    fn ShowWindow(&self , style : WindowShowStyleCommand) -> bool {
        unsafe {
            WindowsAPI::ShowWindow(*self , style).bool()
        }
    }
}