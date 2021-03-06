use super::super::prelude::{
    HWND , LPCTSTR , UINT , CCINT
};

extern "stdcall" {
    pub fn MessageBoxW(
        /* _In_opt_ */      hWnd : HWND     , 
        /* _In_opt_ */    lpText : LPCTSTR  , 
        /* _In_opt_ */ lpCaption : LPCTSTR  , 
        /*   _In_   */     uType : UINT
    ) -> CCINT /* WINAPI */;
}