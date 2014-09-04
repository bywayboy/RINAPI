/**
    <summary>
        All defined in MSDN: [Window Functions (Windows)]
    </summary>
**/

use super::super::prelude::{
    LPCTSTR , DWORD , CCINT , HWND , HMENU , HINSTANCE , LPVOID , BOOL
};

#[link(name = "User32")]
extern "stdcall" {
    /**
        no CreateWindowW implemented in User32:
        WinUser.h:3892 =>
        #define CreateWindowW(lpClassName, lpWindowName, dwStyle, x, y,\
                        nWidth, nHeight, hWndParent, hMenu, hInstance, lpParam)\
                CreateWindowExW(0L, lpClassName, lpWindowName, dwStyle, x, y,\
                        nWidth, nHeight, hWndParent, hMenu, hInstance, lpParam)
    **/
    pub fn CreateWindowExW(
        /*   _In_   */    dwExStyle : DWORD     ,
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

    /**
        <summary>
            Sets the specified window's show state.
        </summary>
        <param name="hWnd">
            A handle to the window.
        </param>
        <param name="nCmdShow">
            Controls how the window is to be shown. 
            This parameter is ignored the first time an application calls **ShowWindow**, 
            if the program that launched the application provides a [!MSDN=STARTUPINFO] structure. 
            Otherwise, the first time **ShowWindow** is called, 
            the value should be the value obtained by the [!MSDN=WinMain] function in its nCmdShow parameter. 
            In subsequent calls, this parameter can be one of the following values.
        </param>
    **/
    pub fn ShowWindow(
        /*   _In_   */     hWnd : HWND  ,
        /*   _In_   */ nCmdShow : CCINT
    ) -> BOOL /* WINAPI */;
}