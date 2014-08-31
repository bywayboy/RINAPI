/**
	MSDN:
	Windows Development (Windows) / Windows Application UI Development |=>
	Windows and Messages / Window Classes / Window Class Reference |=>
	Window Class Styles
**/
use ustd::os::windows::dev::ui::etypes::WindowClassStyle;

pub static    VerticalRedraw : WindowClassStyle = 0x0001;
pub static  HorizontalRedraw : WindowClassStyle = 0x0002;
pub static      DoubleClicks : WindowClassStyle = 0x0008;
pub static          WindowDC : WindowClassStyle = 0x0020;
pub static           ClassDC : WindowClassStyle = 0x0040;
pub static          ParentDC : WindowClassStyle = 0x0080;
pub static           NoClose : WindowClassStyle = 0x0200;
pub static          SaveBits : WindowClassStyle = 0x0800;
pub static   ByteAlignClient : WindowClassStyle = 0x1000;
pub static   ByteAlignWindow : WindowClassStyle = 0x2000;
pub static       GlobalClass : WindowClassStyle = 0x4000;
pub static InputMethodEditor : WindowClassStyle = 0x00010000;
pub static        DropShadow : WindowClassStyle = 0x00020000;