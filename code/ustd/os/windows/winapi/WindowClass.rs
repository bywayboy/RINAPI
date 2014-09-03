use ustd::os::windows::common::types::win32::{
    LPCTSTR , HINSTANCE , ATOM , BOOL
};

use ustd::os::windows::dev::ui::WindowClass::WNDCLASS;
use ustd::os::windows::dev::ui::WindowClassExtra::WNDCLASSEX;

extern "stdcall" {
    pub fn RegisterClassW(
        /*   _In_   */ lpWndClass : *const WNDCLASS
    ) -> ATOM /* WINAPI */;

    pub fn RegisterClassExW(
        /*   _In_   */ lpwcx : *const WNDCLASSEX
    ) -> ATOM /* WINAPI */;

    pub fn UnregisterClassW(
        /*   _In_   */ lpClassName : LPCTSTR    , 
        /* _In_opt_ */   hInstance : HINSTANCE
    ) -> BOOL /* WINAPI */;
}