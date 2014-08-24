

/**
    Window Class Structures (Windows) / WNDCLASS structure (Windows)
**/
struct RawWindowClass {
            style : UINT        , /* WindowClassStyle */
      lpfnWndProc : WNDPROC     , /*  */
       cbClsExtra : CCINT       , /* int */
       cbWndExtra : CCINT       , /* int */
        hInstance : HINSTANCE   , /* Application */
            hIcon : HICON       , /* Icon */
          hCursor : HCURSOR     , /* Cursor */
    hbrBackground : HBRUSH      , /* Brush */
     lpszMenuName : LPCTSTR     , /*  */
    lpszClassName : LPCTSTR     , /*  */
}


pub struct WindowClass {
    raw: RawWindowClass
}

impl WindowClass {
    pub fn new() -> WindowClass {
        WindowClass {
            raw : RawWindowClass {

            }
        }
    }

    pub fn setStyle(&self , style : WindowClassStyle) {
        self.raw.style = style;
    }
}