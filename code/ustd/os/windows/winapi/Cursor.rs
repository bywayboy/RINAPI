/**
    Cursor Functions
**/

use ustd::os::windows::common::types::win32::{
    HINSTANCE , LPCTSTR , HCURSOR
};
    
extern "stdcall" {
    pub fn LoadCursorW(
        /* _In_opt_ */   hInstance : HINSTANCE , 
        /*   _In_   */lpCursorName : LPCTSTR
    )-> HCURSOR /* WINAPI */;
}