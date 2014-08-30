use ustd::types::os::windows::win32::{
    HINSTANCE , LPCTSTR , HICON
};

/**
    Icon Functions (Windows)
**/
extern "stdcall" {
    pub fn LoadIconW(
        /* _In_opt_ */  hInstance : HINSTANCE   ,
        /*   _In_   */ lpIconName : LPCTSTR
    ) -> HICON /* WINAPI */;
}