use ustd::types::os::windows::win32::{
    HWND , CCINT , BOOL
};

/**
    <summary>
        All defined in MSDN: [Window Functions (Windows)]
    </summary>
**/
extern "stdcall" {
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