extern crate std;

use super::super::prelude::{
	wapi , 
	Text , Module
};

pub fn GetModuleHandle(name : Option<Text>) -> Module {
	unsafe {
		wapi::DLL::GetModuleHandleW(name.unwrap_or(std::ptr::null()))
	}
}