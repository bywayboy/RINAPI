#[allow(non_camel_case_types)]

extern crate libc;

use self::libc::types::os::arch::c95::{
	c_int , c_long , c_ulong , wchar_t , c_uint , c_ushort
};
use self::libc::types::common::c95::c_void;

// Standard C types in Microsoft C
pub type CCINT = c_int;

// WinDef.h:173 => typedef unsigned int UINT;
pub type UINT = c_uint;

// WinNT.h:333 => typedef long LONG;
pub type LONG = c_long;

pub type BOOL = c_int;

// WinUser.h:62 => typedef LRESULT (CALLBACK* WNDPROC)(HWND, UINT, WPARAM, LPARAM);
//TODO
//pub type WNDPROC = *const c_void;
//pub type WNDPROC = *extern fn(HWND,UINT,WPARAM,LPARAM)->LRESULT;
pub type WNDPROC = extern "system" fn(HWND , UINT , WPARAM , LPARAM) -> LRESULT;

// WinDef.h:155 => typedef unsigned short WORD;
pub type WORD = c_ushort;

// intsafe.h:45 => typedef unsigned long DWORD;
pub type DWORD = c_ulong;

// WinDef.h:215 => typedef WORD ATOM;
pub type ATOM = WORD;

// WinNT.h:344 => typedef wchar_t WCHAR;
pub type WCHAR = wchar_t;

// WinNT.h:357 => typedef __nullterminated CONST WCHAR *LPCWSTR, *PCWSTR;
pub type LPCWSTR = *const WCHAR;

// WinNT.h:447 => typedef LPCWSTR PCTSTR, LPCTSTR;
pub type LPCTSTR = LPCWSTR;

// WinNT.h:289 => typedef void *PVOID;
pub type PVOID = *mut c_void;

// WinDef.h:169 => typedef void far *LPVOID;
pub type LPVOID = *mut c_void;

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

// WinDef.h:281 => typedef HINSTANCE HMODULE;      
// /*HMODULEs can be used in place of HINSTANCEs*/
pub type HMODULE = HINSTANCE;

// intsafe.h:57 => typedef [public] unsigned __int3264 UINT_PTR;
pub type UINT_PTR = c_uint;

// intsafe.h:58 => typedef [public] __int3264 LONG_PTR;
pub type LONG_PTR = c_long;

// WinDef.h:183 => typedef UINT_PTR WPARAM;
pub type WPARAM = UINT_PTR;

// WinDef.h:184 => typedef LONG_PTR LPARAM;
pub type LPARAM = LONG_PTR;

// WinDef.h:249 => DECLARE_HANDLE(HGDIOBJ);
pub type HGDIOBJ = HANDLE;

// WinDef.h:185 => typedef LONG_PTR LRESULT;
pub type LRESULT = LONG_PTR;