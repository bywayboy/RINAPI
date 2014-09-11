/**
	WinUser:157
	#define MAKEINTRESOURCEA(i) ((LPSTR)((ULONG_PTR)((WORD)(i))))
	#define MAKEINTRESOURCEW(i) ((LPWSTR)((ULONG_PTR)((WORD)(i))))

	WinUser.h:9227->9260
**/

use super::super::prelude::{
	Text
};

pub static Application : Text = 32512 as Text;
pub static        Hand : Text = 32513 as Text;
pub static    Question : Text = 32514 as Text;
pub static Exclamation : Text = 32515 as Text;
pub static    ASTERISK : Text = 32516 as Text;
pub static     WinLogo : Text = 32517 as Text;
pub static      Shield : Text = 32518 as Text;
pub static     Warning : Text = Exclamation;
pub static       Error : Text = Hand;
pub static Information : Text = ASTERISK;