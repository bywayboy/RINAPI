
mod WindowsAPI;

pub fn LoadIcon(app : Application , iconName : &str) -> Icon {
	unsafe {
		WindowsAPI::LoadIcon(app, ) as Icon
	}
}

pub fn LoadStandardIcon(iconName : &str) -> Icon {
	unsafe {
		WindowsAPI::LoadIcon() as Icon
	}
}