extern crate std;

use ustd::os::windows::winapi;

use ustd::os::windows::dev::ui::Text::Text;

use ustd::os::windows::dev::ss::Module::Module;

pub fn GetModuleHandle(name : Option<Text>) -> Module {
	unsafe {
		winapi::DLL::GetModuleHandleW(name.unwrap_or(std::ptr::null()))
	}
}