extern crate std;

use ustd::os::windows::winapi;

use ustd::os::windows::dev::ui::Application::Application;
use ustd::os::windows::dev::ui::Text::Text;
use ustd::os::windows::dev::ui::Icon::Icon;
use ustd::os::windows::dev::ui::Cursor::Cursor;

pub fn LoadIcon(app : Option<Application> , name : Text) -> Icon {
	unsafe {
		winapi::Icon::LoadIconW(app.unwrap_or(std::ptr::mut_null()), name)
	}
}

pub fn LoadCursor(app : Option<Application> , name : Text) -> Cursor {
	unsafe {
		winapi::Cursor::LoadCursorW(app.unwrap_or(std::ptr::mut_null()) , name)
	}
}