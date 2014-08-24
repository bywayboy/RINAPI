extern crate libc;

use self::libc::types::os::arch::c95::{
	c_int , c_ulong , wchar_t
};
use self::libc::types::common::c95::c_void;

// Standard C types in Microsoft C
pub type CCINT = c_int;


pub type BOOL = c_int;



// WinDef.h:169 => typedef void far *LPVOID;
pub type LPVOID = *mut c_void;

// intsafe.h:45 => typedef unsigned long DWORD;
pub type DWORD = c_ulong;

// WinNT.h:344 => typedef wchar_t WCHAR;
pub type WCHAR = wchar_t;

// WinNT.h:357 => typedef __nullterminated CONST WCHAR *LPCWSTR, *PCWSTR;
pub type LPCWSTR = *const WCHAR;

// WinNT.h:447 => typedef LPCWSTR PCTSTR, LPCTSTR;
pub type LPCTSTR = LPCWSTR;

// WinNT.h:289 => typedef void *PVOID;
pub type PVOID = *mut c_void;

// WinNT.h:522 => typedef PVOID HANDLE;
pub type HANDLE = PVOID;

// WinDef.h:208 => DECLARE_HANDLE(HWND);
pub type HWND = HANDLE;

// WinDef.h:280 => DECLARE_HANDLE(HINSTANCE);
pub type HINSTANCE = HANDLE;

// WinDef.h:277 => DECLARE_HANDLE(HMENU);
pub type HMENU = HANDLE;

// WinDef.h:275 => DECLARE_HANDLE(HICON);
pub type HICON = HANDLE;

// WinDef.h:311 => DECLARE_HANDLE(HCURSOR);
pub type HCURSOR = HANDLE;

// WinDef.h:261 => DECLARE_HANDLE(HBRUSH);
pub type HBRUSH = HANDLE;




pub trait RustBool {
	fn bool(&self) -> bool;
}

impl RustBool for BOOL {
	fn bool(&self) -> bool {
		match *self {
			0 => false,
			_ => true
		}
	}
}