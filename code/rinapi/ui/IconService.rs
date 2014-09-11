extern crate std;

use super::super::prelude::{
	wapi , 
	Application , Text , Icon
};

/**
	LoadIcon
**/
pub fn loadIcon(app : Option<Application> , name : Text) -> Icon {
	unsafe {
		wapi::Icon::LoadIconW(app.unwrap_or(std::ptr::mut_null()), name)
	}
}

/**
	LoadIcon
**/
pub fn load(name : Text) -> Icon {
	unsafe {
		wapi::Icon::LoadIconW(std::ptr::mut_null() , name)
	}
}