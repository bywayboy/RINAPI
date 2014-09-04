/**
    Cursor Functions
**/

use super::super::prelude::{
    HINSTANCE , LPCTSTR , HCURSOR
};
    
extern "stdcall" {
    pub fn LoadCursorW(
        /* _In_opt_ */   hInstance : HINSTANCE , 
        /*   _In_   */lpCursorName : LPCTSTR
    )-> HCURSOR /* WINAPI */;
}