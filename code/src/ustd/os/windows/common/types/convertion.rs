use ustd::os::windows::common::types::win32::{
    BOOL
};

pub trait ToRustBoolConvertion {
    fn bool(&self) -> bool;
}

impl ToRustBoolConvertion for BOOL {
    fn bool(&self) -> bool {
        match *self {
            0 => false,
            _ => true
        }
    }
}

/**
    <example>
        let app_name = "Application Name".asWindowText();
    </example>
**/
pub trait ToWindowTextConvertion {
    // Vec<u16>
    fn asText(&self) -> *const u16;
}

impl<'a> ToWindowTextConvertion for &'a str {
    fn asText(&self) -> *const u16 {
        let mut newtext = Vec::from_slice(self.to_utf16().as_slice());
        newtext.push(0u16);println!("<inner>:{};",newtext.as_ptr());
        newtext.as_ptr()
    }
}