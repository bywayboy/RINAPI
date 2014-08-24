use ustd::types::os::windows::win32::HINSTANCE;
use ustd::plateform::os::windows::ResourceService;

mod WindowsAPI;

pub type Application = HINSTANCE;

trait IconFunctionInApplication {
	fn LoadIcon(&self , iconName : &str) -> Icon;
}

impl IconFunctionInApplication for Application {
	fn LoadIcon(&self , iconName : &str) -> Icon {
		ResourceService.LoadIcon(*self , iconName)
	}
}