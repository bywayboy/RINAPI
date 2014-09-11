#![feature(globs)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate libc;
use libc::types::common::c95::c_void;

use rinapi::prelude::*;
mod rinapi;

extern "stdcall" fn window_procedure(
	 window : Window , 
	message : UINT , 
	 wParam : WPARAM , 
	 lParam : LPARAM
) -> LRESULT {
	//rinapi::prelude::QuickService::MessageBox(Some("In procedure.".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	match message {
		WindowMessages::Create => {
			let ok = MultimediaService::playSound("helloworld.wav".asText().as_ptr() , None , 
				SoundPlayOptions::FileName | SoundPlayOptions::Async
			);
			if ok {
				println!("sound played!!");
			}
			else {
				println!("sound play with failure!!");
			}
			return PostWindowMessages::Normal;
		}
		/*
		WindowMessages::Paint => {
			return PostWindowMessages::Normal;
		} ,
		*/
		WindowMessages::Destroy => {
			MessageService::postQuit(PostWindowMessages::Normal);
			return PostWindowMessages::Normal;
		}

		_ => {
			;
		}
	}

	return window.pass(message , wParam , lParam);

	//println!("inner coming coming coming");
	
}

fn main(){
	//unsafe { ::std::rt::stack::record_sp_limit(0); }

	//QuickService::MessageBox(Some("Just Start".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
	let application = QuickService::Application();
	let appName = "RINAPI hello world".asText();
	let new_class_name = "New Class".asText().as_ptr();
	
	let mut window_class = WindowClass::new(&WindowClassLayout{
		      class_style : None    			,
	     window_procedure : window_procedure 	,
	     class_extra_size : 0                 	,
	    window_extra_size : 0              		,
	          application : application     	,
	                 icon : Some( IconService::load(StandardIcons::Warning) )                ,
	               cursor : Some(CursorService::load(StandardCursors::Wait))              	,
	           background : Some(DCService::GetStockObject(StockLogicalObjects::DarkGrayBrush)) ,
	            menu_name : None                ,
	           class_name : new_class_name
	});

	//QuickService::MessageBox(Some("begin Class registed".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	let r = window_class.RegisterClass();
	//println!("register result::::{}" , r);

	//QuickService::MessageBox(Some("Class registed".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	let window = WindowService::CreateWindow(
		Some(new_class_name) , 
		None , 
		WindowStyles::OverLappedWindow , 
		CreateWindowOptions::UseDefault , 
		CreateWindowOptions::UseDefault , 
		CreateWindowOptions::UseDefault , 
		CreateWindowOptions::UseDefault , 
		None , 
		None , 
		Some(application) , 
		None
	);

	println!("Window create started.");

	if window == std::ptr::mut_null() {
		println!("maybe window creation failure.");
	}

	println!("window creation ended.");
	//QuickService::MessageBox(Some("Window Created".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	let s = window.show(WindowShowStyleCommands::Maximize);
	let u = window.update();

	println!("show:{} , update:{}" ,s ,u);

	let mut message = Message::new();

	//QuickService::MessageBox(Some("Message created.".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	println!("get or not? {}" , message.GetMessage(None , Some(0) , Some(0)));

	//QuickService::MessageBox(Some("Message tested.".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	while message.GetMessage(None , Some(0) , Some(0)) {
		//println!("go message");
		//QuickService::MessageBox(Some("got message".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
		message.translate();
		//println!("message translated.");
		//QuickService::MessageBox(Some("message translated".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);
		message.dispatch();
		//println!("message Dispatched.");
	}

	//QuickService::MessageBox(Some("Messaging queue ended.".asText().as_ptr()) , None , MessageBoxStyles::Button::OnlyOk);

	println!("Messaging queue ended")

	//return message.wParam;
	return;
}