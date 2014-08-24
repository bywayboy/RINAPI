use ustd::types::os::windows::win32::{
    LPCTSTR , HMODULE
};

extern "stdcall" {
	pub fn GetModuleHandle(
		/* _In_opt_ */ lpModuleName : LPCTSTR
	) -> HMODULE /* WINAPI */;
}