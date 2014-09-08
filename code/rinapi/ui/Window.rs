extern crate std;

use super::super::prelude::{
	HWND , ToRustBoolConvertion , WindowShowStyleCommand , wapi , 
	UINT , WPARAM , LPARAM , LRESULT
};

pub type Window = HWND;

pub trait WindowFunction {
    /**
        [C]isNone
    **/
    fn isNone(&self) -> bool;

    /**
        [A]ShowWindow
    **/
    fn show(&self , style : WindowShowStyleCommand) -> bool;

    /**
        [A]UpdateWindow
    **/
    fn update(&self) -> bool;

    /**
        [A]DefWindowProc
    **/
    fn pass(&self ,
    	message : UINT , 
        wParam : WPARAM , 
        lParam : LPARAM
    ) -> LRESULT;
}

impl WindowFunction for Window {
    fn isNone(&self) -> bool {
        *self == std::ptr::mut_null()
    }

    fn show(&self , style : WindowShowStyleCommand) -> bool {
        unsafe {
            wapi::Window::ShowWindow(*self , style).bool()
        }
    }

    fn update(&self) -> bool {
    	unsafe {
    		wapi::Window::UpdateWindow(*self).bool()
    	}
    }

    fn pass(&self ,
    	message : UINT , 
    	 wParam : WPARAM ,
    	 lParam : LPARAM
    ) -> LRESULT {
    	unsafe {
    		wapi::WindowProcedure::DefWindowProcW(*self , message , wParam , lParam)
    	}
    }
}