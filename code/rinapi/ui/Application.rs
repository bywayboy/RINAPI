use super::super::prelude::{
	HINSTANCE , CursorService , IconService , Text , Cursor , Icon
};

pub type Application = HINSTANCE;

pub trait ResourceFunctionInApplication {
	fn loadCursor(&self , iconName : Text) -> Cursor;

	fn loadIcon(&self , iconName : Text) -> Icon;
}

impl ResourceFunctionInApplication for Application {
	fn loadCursor(&self , cursorName : Text) -> Cursor {
		CursorService::loadCursor(Some(*self) , cursorName)
	}

	fn loadIcon(&self , iconName : Text) -> Icon {
		IconService::loadIcon(Some(*self) , iconName)
	}
}