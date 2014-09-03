/**
    <summary>
        All defined in MSDN: [Window Functions (Windows)]/[ShowWindow function]

        Controls how the window is to be shown. 
        This parameter is ignored the first time an application calls **ShowWindow**, 
        if the program that launched the application provides a [!MSDN=STARTUPINFO] structure. 
        Otherwise, the first time **ShowWindow** is called, 
        the value should be the value obtained by the [!MSDN=WinMain] function in its nCmdShow parameter. 
        In subsequent calls, this parameter can be one of the following values.
    </summary>
    <applies-to>Desktop apps only</applies-to>
    <requirements>
        <minimum-client>Windows 2000 Professional</minimum-client>
        <minimum-server>Windows 2000 Server</minimum-server>
        <header>Winuser.h (include Windows.h)</header>
        <library>User32.lib</library>
        <dll>User32.dll</dll>
    </requirements>
**/

use ustd::os::windows::dev::ui::etypes::WindowShowStyleCommand;

/**
    Hides the window and activates another window.
**/
pub static            Hide : WindowShowStyleCommand = 0;
/**
    Activates and displays a window. If the window is minimized or maximized, 
    the system restores it to its original size and position. 
    An application should specify this flag when displaying the window for the first time.
**/
pub static      ShowNormal : WindowShowStyleCommand = 1;
/**
    Undocumented in MSDN.
**/
pub static          Normal : WindowShowStyleCommand = 1;
/**
    Activates the window and displays it as a minimized window.
**/
pub static   ShowMinimized : WindowShowStyleCommand = 2;
/**
    Activates the window and displays it as a maximized window.
**/
pub static   ShowMaximized : WindowShowStyleCommand = 3;
/**
    Maximizes the specified window.
**/
pub static        Maximize : WindowShowStyleCommand = 3;
/**
    Displays a window in its most recent size and position. 
    This value is similar to **SW_SHOWNORMAL**, except that the window is not activated.
**/
pub static  ShowNoactivate : WindowShowStyleCommand = 4;
/**
    Activates the window and displays it in its current size and position.
**/
pub static            Show : WindowShowStyleCommand = 5;
/**
    Minimizes the specified window and activates the next top-level window in the Z order.
**/
pub static        Minimize : WindowShowStyleCommand = 6;
/**
    Displays the window as a minimized window. 
    This value is similar to **SW_SHOWMINIMIZED**, except the window is not activated.
**/
pub static ShowMinNoActive : WindowShowStyleCommand = 7;
/**
    Displays the window in its current size and position. 
    This value is similar to **SW_SHOW**, except that the window is not activated.
**/
pub static          ShowNA : WindowShowStyleCommand = 8;
/**
    Activates and displays the window. If the window is minimized or maximized, 
    the system restores it to its original size and position. 
    An application should specify this flag when restoring a minimized window.
**/
pub static         Restore : WindowShowStyleCommand = 9;
/**
    Sets the show state based on the SW_ value specified in the [!MSDN=STARTUPINFO] structure 
    passed to the [!MSDN=CreateProcess] function by the program that started the application.
**/
pub static     ShowDefault : WindowShowStyleCommand = 10;
/**
    Minimizes a window, even if the thread that owns the window is not responding. 
    This flag should only be used when minimizing windows from a different thread.
**/
pub static   ForceMinimize : WindowShowStyleCommand = 11;
/**
    Undocumented in MSDN.
**/
pub static             Max : WindowShowStyleCommand = 11;