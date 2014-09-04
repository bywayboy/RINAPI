use super::super::prelude::{
	HINSTANCE , ResourceService , Text , Icon
};

pub type Application = HINSTANCE;

pub trait IconFunctionInApplication {
	fn LoadIcon(&self , iconName : Text) -> Icon;
}

impl IconFunctionInApplication for Application {
	fn LoadIcon(&self , iconName : Text) -> Icon {
		ResourceService::LoadIcon(Some(*self) , iconName)
	}
}