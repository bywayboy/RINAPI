extern crate std;

use ustd::os::windows::winapi;

use ustd::os::windows::dev::ui::{
    Application , Text , Icon , Cursor
};

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