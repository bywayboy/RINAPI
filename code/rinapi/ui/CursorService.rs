extern crate std;

use super::super::prelude::{
	wapi , 
	Application , Text , Cursor
};

/**
	LoadCursor
**/
pub fn loadCursor(app : Option<Application> , name : Text) -> Cursor {
	unsafe {
		wapi::Cursor::LoadCursorW(app.unwrap_or(std::ptr::mut_null()) , name)
	}
}

/**
	LoadCursor
**/
pub fn load(name :Text) -> Cursor {
	unsafe {
		wapi::Cursor::LoadCursorW(std::ptr::mut_null() , name)
	}
}