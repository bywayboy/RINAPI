/**
    <summary>
        All defined in MSDN: [Window Constants (Windows)]/[Window Styles]

        The following are the window styles. After the window has been created, 
        these styles cannot be modified, except as noted.
    </summary>
    <applies-to>Desktop apps only</applies-to>
    <requirements>
    	<minimum-client>Windows 2000 Professional</minimum-client>
    	<minimum-server>Windows 2000 Server</minimum-server>
    	<header>Winuser.h (include Windows.h)</header>
    </requirements>
**/

use super::super::prelude::WindowStyle;

/**
	The window is an overlapped window. 
	An overlapped window has a title bar and a border. Same as the **WS_TILED** style.
**/
pub static       OverLapped : WindowStyle = 0x00000000;
/**
	The windows is a pop-up window. This style cannot be used with the **WS_CHILD** style.
**/
pub static            Popup : WindowStyle = 0x80000000;
/**
	The window is a child window. A window with this style cannot have a menu bar. 
	This style cannot be used with the **WS_POPUP** style.
**/
pub static            Child : WindowStyle = 0x40000000;
/**
	The window is initially minimized. Same as the **WS_ICONIC** style.
**/
pub static         Minimize : WindowStyle = 0x20000000;
/**
	The window is initially visible.

	This style can be turned on and off 
	by using the [!MSDN=ShowWindow] or [!MSDN=SetWindowPos] function.
**/
pub static          Visible : WindowStyle = 0x10000000;
/**
	The window is initially disabled. A disabled window cannot receive input from the user. 
	To change this after a window has been created, use the [!MSDN=EnableWindow] function.
**/
pub static         Disabled : WindowStyle = 0x08000000;
/**
	Clips child windows relative to each other; 
	that is, when a particular child window receives a [!MSDN=WM_PAINT] message, 
	the **WS_CLIPSIBLINGS** style clips all other overlapping child windows out of 
	the region of the child window to be updated. 
	If **WS_CLIPSIBLINGS** is not specified and child windows overlap, it is possible, 
	when drawing within the client area of a child window, 
	to draw within the client area of a neighboring child window.
**/
pub static     ClipSiblings : WindowStyle = 0x04000000;
/**
	Excludes the area occupied by child windows when drawing occurs within the parent window. 
	This style is used when creating the parent window.
**/
pub static     ClipChildren : WindowStyle = 0x02000000;
/**
	The window is initially maximized.
**/
pub static         Maximize : WindowStyle = 0x01000000;
/**
	The window has a title bar (includes the **WS_BORDER** style).
	Caption = Border | DlgFrame
**/
pub static          Caption : WindowStyle = 0x00C00000;
/**
	The window has a thin-line border.
**/
pub static           Border : WindowStyle = 0x00800000;
/**
	The window has a border of a style typically used with dialog boxes. 
	A window with this style cannot have a title bar.
**/
pub static         DlgFrame : WindowStyle = 0x00400000;
/**
	The window has a vertical scroll bar.
**/
pub static          VScroll : WindowStyle = 0x00200000;
/**
	The window has a horizontal scroll bar.
**/
pub static          HScroll : WindowStyle = 0x00100000;
/**
	The window has a window menu on its title bar. 
	The **WS_CAPTION** style must also be specified.
**/
pub static          SysMenu : WindowStyle = 0x00080000;
/**
	The window has a sizing border. Same as the **WS_SIZEBOX** style.
**/
pub static       ThickFrame : WindowStyle = 0x00040000;
/**
	The window is the first control of a group of controls. 
	The group consists of this first control and all controls defined after it, 
	up to the next control with the **WS_GROUP** style. 
	The first control in each group usually has the **WS_TABSTOP** style 
	so that the user can move from group to group. 
	The user can subsequently change the keyboard focus from one control in the group 
	to the next control in the group by using the direction keys.
	
	You can turn this style on and off to change dialog box navigation. 
	To change this style after a window has been created, use the [!MSDN=SetWindowLong] function.
**/
pub static            Group : WindowStyle = 0x00020000;
/**
	The window is a control that can receive the keyboard focus when the user presses the TAB key. 
	Pressing the TAB key changes the keyboard focus to the next control with the **WS_TABSTOP** style.

	You can turn this style on and off to change dialog box navigation. 
	To change this style after a window has been created, use the [!MSDN=SetWindowLong] function. 
	For user-created windows and modeless dialogs to work with tab stops, 
	alter the message loop to call the [!MSDN=IsDialogMessage] function.
**/
pub static          TabStop : WindowStyle = 0x00010000;
/**
	The window has a minimize button. Cannot be combined with the **WS_EX_CONTEXTHELP** style. 
	The **WS_SYSMENU** style must also be specified.
**/
pub static      MinimizeBox : WindowStyle = 0x00020000;
/**
	The window has a maximize button. Cannot be combined with the **WS_EX_CONTEXTHELP** style. 
	The **WS_SYSMENU** style must also be specified.
**/
pub static      MaximizeBox : WindowStyle = 0x00010000;
/**
	The window is an overlapped window. 
	An overlapped window has a title bar and a border. Same as the **WS_OVERLAPPED** style.
**/
pub static            Tiled : WindowStyle = 0x00000000;
/**
	The window is initially minimized. Same as the **WS_MINIMIZE** style.
**/
pub static           IconIC : WindowStyle = 0x20000000;
/**
	The window has a sizing border. Same as the **WS_THICKFRAME** style.
**/
pub static          SizeBox : WindowStyle = 0x00040000;
/**
	The window is an overlapped window. Same as the **WS_OVERLAPPEDWINDOW** style.
**/
pub static      TiledWindow : WindowStyle = (0x00000000u32 | 0x00C00000u32 | 0x00080000u32 | 
					 0x00040000u32 | 0x00020000u32 | 0x00010000u32 );
/**
	The window is an overlapped window. Same as the **WS_TILEDWINDOW** style.
**/
pub static OverLappedWindow : WindowStyle = (0x00000000u32 | 0x00C00000u32 | 0x00080000u32 | 
					 0x00040000u32 | 0x00020000u32 | 0x00010000u32 );
/**
	The window is a pop-up window. 
	The **WS_CAPTION** and **WS_POPUPWINDOW** styles must be combined to make the window menu visible.
**/
pub static      PopupWindow : WindowStyle = (0x80000000u32 | 0x00800000u32 | 0x00080000u32);
/**
	Same as the **WS_CHILD** style.
**/
pub static      ChildWindow : WindowStyle = 0x40000000;