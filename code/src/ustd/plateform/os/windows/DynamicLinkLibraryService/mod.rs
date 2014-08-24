mod WindowsAPI;

pub fn GetModuleHandle(moduleName : &str) -> Module {
	unsafe {
		WindowsAPI::GetModuleHandle(std::ptr::mut_null()) as Module
	}
}