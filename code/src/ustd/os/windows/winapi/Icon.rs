/**
    Icon Functions (Windows)
**/

use ustd::os::windows::common::types::win32::{
    HINSTANCE , LPCTSTR , HICON
};
    
extern "stdcall" {
    pub fn LoadIconW(
        /* _In_opt_ */  hInstance : HINSTANCE   ,
        /*   _In_   */ lpIconName : LPCTSTR
    ) -> HICON /* WINAPI */;
}