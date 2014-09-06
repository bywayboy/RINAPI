/**
    Window Procedure Functions (Windows)
**/

use super::super::prelude::{
    HWND , UINT , WPARAM , LPARAM , LRESULT
};

#[link(name = "User32")]    
extern "stdcall" {
    pub fn DefWindowProcW(
        /*   _In_   */   hWnd : HWND    ,
        /*   _In_   */    Msg : UINT    ,
        /*   _In_   */ wParam : WPARAM  ,
        /*   _In_   */ lParam : LPARAM
    ) -> LRESULT /* WINAPI */;
}