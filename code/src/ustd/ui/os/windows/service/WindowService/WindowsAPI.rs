use ustd::types::os::windows::win32::{
    LPCTSTR , DWORD , CCINT , HWND , HMENU , HINSTANCE , LPVOID
};

extern "stdcall" {
    pub fn CreateWindow(
        /* _In_opt_ */ lpClassName  : LPCTSTR   ,
        /* _In_opt_ */ lpWindowName : LPCTSTR   ,
        /*   _In_   */ dwStyle      : DWORD     ,
        /*   _In_   */ x            : CCINT     ,
        /*   _In_   */ y            : CCINT     ,
        /*   _In_   */ nWidth       : CCINT     ,
        /*   _In_   */ nHeight      : CCINT     ,
        /* _In_opt_ */ hWndParent   : HWND      ,
        /* _In_opt_ */ hMenu        : HMENU     ,
        /* _In_opt_ */ hInstance    : HINSTANCE ,
        /* _In_opt_ */ lpParam      : LPVOID
    ) -> HWND /* WINAPI */;
}

extern "stdcall" {
    fn UnregisterClass(
        /*   _In_   */ lpClassName : LPCTSTR    , 
        /* _In_opt_ */   hInstance : HINSTANCE
    ) -> BOOL /* WINAPI */;
}