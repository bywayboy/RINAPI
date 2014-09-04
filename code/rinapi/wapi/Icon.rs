/**
    Icon Functions (Windows)
**/

use super::super::prelude::{
    HINSTANCE , LPCTSTR , HICON
};
    
extern "stdcall" {
    pub fn LoadIconW(
        /* _In_opt_ */  hInstance : HINSTANCE   ,
        /*   _In_   */ lpIconName : LPCTSTR
    ) -> HICON /* WINAPI */;
}