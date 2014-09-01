use ustd::os::windows::winapi;

use ustd::os::windows::gdi::GdiObject;

pub fn GetStockObject() -> GdiObject {
    unsafe {
        winapi::DC::GetStockObjectW()
    }
}