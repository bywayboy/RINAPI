use super::super::prelude::{
	HWND , ToRustBoolConvertion , WindowShowStyleCommand , wapi , 
	UINT , WPARAM , LPARAM , LRESULT
};

pub type Window = HWND;

pub trait WindowFunction {
    fn ShowWindow(&self , style : WindowShowStyleCommand) -> bool;

    fn UpdateWindow(&self) -> bool;

    fn DefWindowProc(&self ,
    	message : UINT , wParam : WPARAM , lParam : LPARAM) -> LRESULT;
}

impl WindowFunction for Window {
    fn ShowWindow(&self , style : WindowShowStyleCommand) -> bool {
        unsafe {
            wapi::Window::ShowWindow(*self , style).bool()
        }
    }

    fn UpdateWindow(&self) -> bool {
    	unsafe {
    		wapi::Window::UpdateWindow(*self).bool()
    	}
    }

    fn DefWindowProc(&self ,
    	message : UINT , 
    	 wParam : WPARAM ,
    	 lParam : LPARAM
    ) -> LRESULT {
    	unsafe {
    		wapi::WindowProcedure::DefWindowProcW(*self , message , wParam , lParam)
    	}
    }
}