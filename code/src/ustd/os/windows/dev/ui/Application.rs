use ustd::os::windows::common::types::win32::HINSTANCE;
use ustd::plateform::os::windows::ResourceService;

use ustd::os::windows::winapi;

pub type Application = HINSTANCE;

trait IconFunctionInApplication {
	fn LoadIcon(&self , iconName : &str) -> Icon;
}

impl IconFunctionInApplication for Application {
	fn LoadIcon(&self , iconName : &str) -> Icon {
		ResourceService.LoadIcon(*self , iconName)
	}
}