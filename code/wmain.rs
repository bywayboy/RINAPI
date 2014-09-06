#![feature(globs)]
extern crate libc;
use libc::types::common::c95::c_void;

use rinapi::prelude::*;
mod rinapi;

extern fn window_procedure(
	 window : Window , 
	message : UINT , 
	 wParam : WPARAM , 
	 lParam : LPARAM
) -> LRESULT {
	QuickService::MessageBox(Some("In procedure.".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
	//println!("coming coming coming");
	window.DefWindowProc(message , wParam , lParam)
}

fn main(){
	QuickService::MessageBox(Some("Just Start".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
	let application = QuickService::Application();
	let appName = "RINAPI hello world".asText();
	let class_name = "New Class".asText().as_ptr();
	
	let mut window_class = WindowClass::new();
		window_class.setWindowProcedure(&window_procedure as WindowProcedure);
		window_class.setClassName(class_name);

	QuickService::MessageBox(Some("begin Class registed".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	let r = window_class.RegisterClass();
	//println!("register result::::{}" , r);

	QuickService::MessageBox(Some("Class registed".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	let window = WindowService::CreateWindow(
		Some(class_name) , 
		None , 
		WindowStyles::OverLappedWindow , 
		20 , 
		20 , 
		800 , 
		600 , 
		None , 
		None , 
		Some(application) , 
		None
	);

	QuickService::MessageBox(Some("Window Created".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	window.ShowWindow(WindowShowStyleCommands::Maximize);
	window.UpdateWindow();

	let mut message = Message::new();

	QuickService::MessageBox(Some("Message created.".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	while message.GetMessage(None , Some(0) , Some(0)) {
		QuickService::MessageBox(Some("got message".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
		message.TranslateMessage();
		QuickService::MessageBox(Some("message translated".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
		message.DispatchMessage();
	}

	//return message.wParam;
}