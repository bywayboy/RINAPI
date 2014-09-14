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
	let normal = PostWindowMessages::Normal;

	match message {
		WindowMessages::Create => {
			MultimediaService::playSound("helloworld.wav".asText().as_ptr() , None , 
				SoundPlayOptions::FileName | SoundPlayOptions::Async
			);

			normal
		}
		/*
		WindowMessages::Paint => {
			return PostWindowMessages::Normal;
		} ,
		*/
		WindowMessages::Destroy => {
			MessageService::postQuit(PostWindowMessages::Normal);
			
			normal
		}

		_ => {
			window.pass(message , wParam , lParam)
		}
	}
}

fn main(){
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

	window_class.RegisterClass();

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

	window.show(WindowShowStyleCommands::ShowNormal);
	window.update();

	let mut message = Message::new();

	while message.GetMessage(None , Some(0) , Some(0)) {
		message.translate();
		message.dispatch();
	}

	//return message.wParam;
	return;
}