/**

所有的嵌套註釋中 markdown 中的 
*  代表 預定義連接（代表連接到預定義文檔目錄）
** 代表 單純強調（表現爲加粗）

**/


use ustd::os::windows::common::types::convertion::ToWindowTextConvertion;
use ustd::os::windows::quick::ApplicationService;
use ustd::os::windows::dev::ui::enums::MessageBoxStyles;


mod ustd;


fn main() {
	ApplicationService::MessageBox(None,Some("Hello".asText()),None,MessageBoxStyles::Button::OkCancel);

	//println!("like what you want!!! {} , {} " , "ABC".asText() , "DEF".asText());
}