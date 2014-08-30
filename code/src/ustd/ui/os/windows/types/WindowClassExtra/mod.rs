/**
    Window Class Structures (Windows) / WNDCLASSEX structure (Windows)
**/

mod WindowsAPI;

pub struct WindowClassExtra {
    raw : RawWindowClassExtra
}

struct RawWindowClassExtra {
           cbSize : UINT        , /* u32 */
            style : UINT        , /* WindowClassStyle */
      lpfnWndProc : WNDPROC     , /* */
       cbClsExtra : CCINT       , /* int */
       cbWndExtra : CCINT       , /* int */
        hInstance : HINSTANCE   , /* Application */
            hIcon : HICON       , /* Icon */
          hCursor : HCURSOR     , /* Cursor */
    hbrBackground : HBRUSH      , /* Brush */
     lpszMenuName : LPCTSTR     , /* Text */
    lpszClassName : LPCTSTR     , /* Text */
          hIconSm : HICON       , /* Icon */
}

pub type WNDCLASSEX = RawWindowClassExtra;

impl WindowClassExtra {
    pub fn new() -> WindowClassExtra {
        WindowClassExtra {
            raw : RawWindowClassExtra {
                       cbSize : UINT ,
                        style : WindowClassStyle.VerticalRedraw | WindowClassStyle.HorizontalRedraw , 
                  lpfnWndProc : std::ptr::mut_null()                                                , 
                   cbClsExtra : 0                                                                   ,
                   cbWndExtra : 0                                                                   ,
                    hInstance : std::ptr::mut_null()                                                ,
                        hIcon : std::ptr::mut_null()                                                ,
                      hCursor : std::ptr::mut_null()                                                ,
                hbrBackground : std::ptr::mut_null()                                                ,
                 lpszMenuName : "Application Menu".asText()                                         ,
                lpszClassName : "Class".asText()                                                    ,
                      hIconSm : std::ptr::mut_null()                                                ,
            }
        }
    }

    pub fn setSize(&mut self , size : u32) {
        self.raw.cbSize = size;
    }

    pub fn setStyle(&mut self , style : WindowClassStyle) {
        self.raw.style = style;
    }

    pub fn setWindowProcedure(&mut self , proc : &fn) {
        self.raw.lpfnWndProc = proc;
    }

    pub fn setClassExtraSize(&mut self , size : int) {
        self.raw.cbClsExtra = size;
    }

    pub fn setWindowExtraSize(&mut self , size : int) {
        self.raw.cbWndExtra = size;
    }

    pub fn setApplication(&mut self , app : Application) {
        self.raw.hInstance = app;
    }

    pub fn setIcon(&mut self , icon : Icon) {
        self.raw.hIcon = icon;
    }

    pub fn setCursor(&mut self , cursor : Cursor) {
        self.raw.hCursor = cursor;
    }

    pub fn setBackground(&mut self , bg : Brush) {
        self.raw.hbrBackground = bg;
    }

    pub fn setMenuName(&mut self , name : Text) {
        self.raw.lpszMenuName = name;
    }

    pub fn setClassName(&mut self , name : Text) {
        self.raw.lpszClassName = name;
    }

    pub fn setSmallIcon(&mut self , icon : Icon) {
        self.raw.hIconSm = icon;
    }

    pub fn register(&self) -> Atom {
        unsafe {
            WindowsAPI::RegisterClassEx(self)
        }
    }

    pub fn unregister(&self , app : Option<Application>) -> bool {
        WindowService::UnregisterClass(self.raw.lpszClassName , app)
    }
}