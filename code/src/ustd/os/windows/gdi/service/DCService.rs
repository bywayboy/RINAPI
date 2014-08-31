use ustd::os::windows::winapi;

pub fn GetStockObject() -> GdiObject {
    unsafe {
        winapi::DC::GetStockObjectW()
    }
}