use ustd::os::windows::winapi;

pub fn LoadIcon(app : Option<Application> , name : Text) -> Icon {
	unsafe {
		winapi::Icon::LoadIconW(app.unwrap_or(std::ptr::mut_null()), name)
	}
}

pub fn LoadCursor(app : Option<Application> , name : Text) -> Cursor {
	unsafe {
		winapi::Icon::LoadCursorW(app.unwrap_or(std::ptr::mut_null()) , name)
	}
}