/**

所有的嵌套註釋中 markdown 中的 
*  代表 預定義連接（代表連接到預定義文檔目錄）
** 代表 單純強調（表現爲加粗）

**/

use rinapi::prelude::{
	ToWindowTextConvertion , 
	QuickService , 
	MessageBoxStyles
};

mod rinapi;

fn cref(p : &int) {
	println!("by ref way: {}" , *p);
}

fn cptr(p : *int) {
	unsafe {
		println!("by ptr way: {}" , *p);
	}
}


fn main() {
	let txt1 = "Hello".asText();
	let txt2 = "World".asText();

	QuickService::MessageBox(Some(txt1.as_ptr()),Some(txt2.as_ptr()),MessageBoxStyles::Button::OkCancel);

	cref(&7);
	cptr(&7);

	//println!("like what you want!!! {} , {} " , "ABC".asText() , "DEF".asText());
}