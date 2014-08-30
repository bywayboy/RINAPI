use ustd::types::os::windows::win32::{
    UINT , CCINT , DWORD
};

pub type MessageBoxResult = CCINT;
pub type MessageBoxStyle = UINT;
pub type WindowClassStyle = UINT;
pub type WindowShowStyleCommand = CCINT;
pub type WindowStyle = DWORD;

