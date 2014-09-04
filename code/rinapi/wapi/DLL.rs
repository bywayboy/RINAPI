use super::super::prelude::{
    LPCTSTR , HMODULE
};

extern "stdcall" {
    pub fn GetModuleHandleW(
        /* _In_opt_ */ lpModuleName : LPCTSTR
    ) -> HMODULE /* WINAPI */;
}