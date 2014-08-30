
mod WindowsAPI;

pub fn LoadIcon(app : Application , name : Text) -> Icon {
	unsafe {
		WindowsAPI::LoadIconW(app, name)
	}
}

pub fn LoadStandardIcon(name : Text) -> Icon {
	unsafe {
		WindowsAPI::LoadIconW()
	}
}