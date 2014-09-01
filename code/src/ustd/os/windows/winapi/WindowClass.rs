use ustd::os::windows::common::types::win32::{
    LPCTSTR , HINSTANCE , ATOM , BOOL
};

use ustd::os::windows::dev::ui::WindowClass::WNDCLASS;
use ustd::os::windows::dev::ui::WindowClassExtra::WNDCLASSEX;

extern "stdcall" {
    fn RegisterClass(
        /*   _In_   */ lpWndClass : *const WNDCLASS
    ) -> ATOM /* WINAPI */;

    fn RegisterClassEx(
        /*   _In_   */ lpwcx : *const WNDCLASSEX
    ) -> ATOM /* WINAPI */;

    fn UnregisterClass(
        /*   _In_   */ lpClassName : LPCTSTR    , 
        /* _In_opt_ */   hInstance : HINSTANCE
    ) -> BOOL /* WINAPI */;
}