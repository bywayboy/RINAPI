use ustd::os::windows::winapi;

pub fn GetModuleHandle(name : Option<Text>) -> Module {
	unsafe {
		WindowsAPI::GetModuleHandleW(name.unwrap_or(std::ptr::mut_null()))
	}
}