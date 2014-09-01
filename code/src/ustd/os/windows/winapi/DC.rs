use ustd::os::windows::common::types::win32::{
    CCINT , HGDIOBJ
};

extern "stdcall" {
    fn GetStockObjectW(
        /* _In_opt_ */ fnObject : CCINT
    ) -> HGDIOBJ;
}