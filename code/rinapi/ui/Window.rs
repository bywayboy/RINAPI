use super::super::prelude::{
	HWND , ToRustBoolConvertion , WindowShowStyleCommand , wapi
};

pub type Window = HWND;

pub trait WindowFunction {
    fn ShowWindow(&self , style : WindowShowStyleCommand) -> bool;
}

impl WindowFunction for Window {
    fn ShowWindow(&self , style : WindowShowStyleCommand) -> bool {
        unsafe {
            wapi::Window::ShowWindow(*self , style).bool()
        }
    }
}