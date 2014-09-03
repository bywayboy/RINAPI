use ustd::os::windows::common::types::win32::HINSTANCE;
use ustd::os::windows::dev::ui::service::ResourceService;

use ustd::os::windows::dev::ui::Text::Text;
use ustd::os::windows::dev::ui::Icon::Icon;

pub type Application = HINSTANCE;

trait IconFunctionInApplication {
	fn LoadIcon(&self , iconName : Text) -> Icon;
}

impl IconFunctionInApplication for Application {
	fn LoadIcon(&self , iconName : Text) -> Icon {
		ResourceService::LoadIcon(Some(*self) , iconName)
	}
}