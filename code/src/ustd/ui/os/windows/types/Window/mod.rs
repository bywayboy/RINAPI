use ustd::types::os::windows::win32::{
    HWND , CCINT , BOOL , CBoolean2Rust
};
use ustd::ui::os::windows::types::Constants::WindowShowStyleCommand;

pub type Window = HWND;

mod WindowsAPI;


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