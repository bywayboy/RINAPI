/**
	Window Class Structures (Windows) / WNDCLASSEX structure (Windows)
**/
struct RawWindowClassExtend {
	       cbSize : UINT ,
	        style : UINT ,
	  lpfnWndProc : WNDPROC ,
	   cbClsExtra : CCINT ,
	   cbWndExtra : CCINT ,
	    hInstance : HINSTANCE ,
	        hIcon : HICON ,
	      hCursor : HCURSOR ,
	hbrBackground : HBRUSH ,
	 lpszMenuName : LPCTSTR ,
	lpszClassName : LPCTSTR ,
	      hIconSm : HICON ,
}

pub struct WindowClassExtend {
	raw : RawWindowClassExtend
}

impl WindowClassExtend {
	pub fn new() -> WindowClassExtend {
		WindowClassExtend {
			raw : RawWindowClassExtend {

			}
		}
	}
}