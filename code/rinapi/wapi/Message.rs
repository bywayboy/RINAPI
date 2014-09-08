/**
    Message Functions (Windows)
**/

use super::super::prelude::{
    BOOL , LPMSG , HWND , UINT , MSG , LRESULT , CCINT ,
    ToRustBoolConvertion
};

#[link(name = "User32")]    
extern "stdcall" {
    pub fn GetMessageW(
        /*  __Out_  */         lpMsg : LPMSG  , 
        /* _In_opt_ */          hWnd : HWND   , 
        /*   _In_   */ wMsgFilterMin : UINT   , 
        /*   _In_   */ wMsgFilterMax : UINT
    ) -> BOOL /* WINAPI */;

    pub fn TranslateMessage(
        /*   _In_   */ lpMsg : *const MSG
    ) -> BOOL /* WINAPI */;

    pub fn DispatchMessageW(
        /*   _In_   */ lpMsg : *const MSG
    ) -> LRESULT /* WINAPI */;

    pub fn PostQuitMessage(
        /*   _In_   */ nExitCode : CCINT
    ) /* VOID WINAPI */;
}