extern "stdcall" {
    fn MessageBox(
        /* _In_opt_ */      hWnd : HWND     , 
        /* _In_opt_ */    lpText : LPCTSTR  , 
        /* _In_opt_ */ lpCaption : LPCTSTR  , 
        /*   _In_   */     uType : UINT
    ) -> CCINT /* WINAPI */;
}