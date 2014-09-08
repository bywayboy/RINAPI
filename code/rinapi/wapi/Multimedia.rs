use super::super::prelude::{
	LPCTSTR , HMODULE , DWORD , BOOL
};

#[link(name = "WinMM")]
extern "stdcall" {
	pub fn PlaySoundW(
		pszSound : LPCTSTR  , 
			hmod : HMODULE  , 
		fdwSound : DWORD
	) -> BOOL;
}