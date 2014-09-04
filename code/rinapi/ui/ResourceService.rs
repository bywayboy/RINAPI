extern crate std;

use super::super::prelude::{
	wapi , 
	Application , Text , Icon , Cursor
};

pub fn LoadIcon(app : Option<Application> , name : Text) -> Icon {
	unsafe {
		wapi::Icon::LoadIconW(app.unwrap_or(std::ptr::mut_null()), name)
	}
}

pub fn LoadCursor(app : Option<Application> , name : Text) -> Cursor {
	unsafe {
		wapi::Cursor::LoadCursorW(app.unwrap_or(std::ptr::mut_null()) , name)
	}
}