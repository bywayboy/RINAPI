extern crate std;

use super::super::prelude::{
	Application , Text , 
	DialogBoxCommand , MessageBoxStyle , 
	DllService , DialogBoxService
};

pub fn Application() -> Application {
    DllService::GetModuleHandle(None)
}

pub fn MessageBox(
     text : Option<Text>        ,
    title : Option<Text>        ,
    style : MessageBoxStyle
) -> DialogBoxCommand
{
    DialogBoxService::MessageBox(None , text , title , style)
}