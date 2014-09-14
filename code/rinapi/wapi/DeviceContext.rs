use super::super::prelude::{
    CCINT , HGDIOBJ
};

#[link(name = "Gdi32")]
extern "stdcall" {
    pub fn GetStockObject(
        /* _In_opt_ */ fnObject : CCINT
    ) -> HGDIOBJ;
}