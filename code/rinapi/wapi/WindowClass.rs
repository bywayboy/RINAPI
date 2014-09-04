use super::super::prelude::{
    LPCTSTR , HINSTANCE , ATOM , BOOL , 
    WNDCLASS , WNDCLASSEX
};

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