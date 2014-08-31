use ustd::os::windows::common::types::win32::{
    LPCTSTR , HMODULE
};

extern "stdcall" {
    pub fn GetModuleHandleW(
        /* _In_opt_ */ lpModuleName : LPCTSTR
    ) -> HMODULE /* WINAPI */;
}