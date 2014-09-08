extern crate std;

use super::super::prelude::{
	Text , Module , SoundPlayOption , ToRustBoolConvertion ,
	wapi
};

/**
	PlaySound
**/
pub fn playSound(
	   text : Text 				, 
	 module : Option<Module> 	,
	options : SoundPlayOption
) -> bool {
	unsafe {
		wapi::Multimedia::PlaySoundW(
			text , 
			module.unwrap_or(std::ptr::mut_null()) , 
			options
		).bool()
	}
}

