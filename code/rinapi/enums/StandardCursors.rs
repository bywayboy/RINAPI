/**
	WinUser:157
	#define MAKEINTRESOURCEA(i) ((LPSTR)((ULONG_PTR)((WORD)(i))))
	#define MAKEINTRESOURCEW(i) ((LPWSTR)((ULONG_PTR)((WORD)(i))))

	WinUser.h:8824->8846
**/

use super::super::prelude::{
	Text
};
 
pub static       Arrow : Text = 32512 as Text;
pub static       IBEAM : Text = 32513 as Text;
pub static        Wait : Text = 32514 as Text;
pub static       Cross : Text = 32515 as Text;
pub static     UpArrow : Text = 32516 as Text;
pub static        Size : Text = 32640 as Text; 
pub static        Icon : Text = 32641 as Text;
pub static    SizeNWSE : Text = 32642 as Text;
pub static    SizeNESW : Text = 32643 as Text;
pub static      SizeWE : Text = 32644 as Text;
pub static      SizeNS : Text = 32645 as Text;
pub static     SizeAll : Text = 32646 as Text;
pub static          No : Text = 32648 as Text;
pub static        Hand : Text = 32649 as Text;
pub static AppStarting : Text = 32650 as Text;
pub static        Help : Text = 32651 as Text;