mod WindowsAPI;

pub fn GetModuleHandle(name : Option<Text>) -> Module {
	unsafe {
		WindowsAPI::GetModuleHandleW(name.unwrap_or(std::ptr::mut_null()))
	}
}