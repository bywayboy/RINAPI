extern crate std;

use super::super::prelude::{
    LPVOID , CCINT , WindowStyle , ToRustBoolConvertion , wapi , 
    Text , Window , Application , Menu , ExtendedWindowStyle , 
};

pub fn CreateWindow(
           className : Option<Text>         ,
          windowName : Option<Text>         ,
           drawStyle : WindowStyle          ,
                   x : CCINT                ,
                   y : CCINT                ,
               width : CCINT                ,
              height : CCINT                ,
        parentWindow : Option<Window>       ,
                menu : Option<Menu>         ,
            instance : Option<Application>  ,
               param : Option<LPVOID>
) -> Window 
{
    CreateWindowEx(
        0,
        className , 
        windowName , 
        drawStyle , 
        x , 
        y , 
        width , 
        height , 
        parentWindow , 
        menu , 
        instance , 
        param
    )
}

pub fn CreateWindowEx(
           dwExStyle : ExtendedWindowStyle ,
           className : Option<Text>         ,
          windowName : Option<Text>         ,
           drawStyle : WindowStyle          ,
                   x : CCINT                  ,
                   y : CCINT                  ,
               width : CCINT                  ,
              height : CCINT                  ,
        parentWindow : Option<Window>       ,
                menu : Option<Menu>         ,
            instance : Option<Application>  ,
               param : Option<LPVOID>
) -> Window 
{
    unsafe {
        wapi::Window::CreateWindowExW(
            dwExStyle ,
            className.unwrap_or(std::ptr::null()) ,
            windowName.unwrap_or(std::ptr::null()) ,
            drawStyle , 
            x , 
            y ,
            width , 
            height , 
            parentWindow.unwrap_or(std::ptr::mut_null()) ,
            menu.unwrap_or(std::ptr::mut_null()) ,
            instance.unwrap_or(std::ptr::mut_null()) ,
            param.unwrap_or(std::ptr::mut_null())
        )
    }
}

pub fn UnregisterClass(name : Text , app : Option<Application>) -> bool {
    unsafe {
        wapi::WindowClass::UnregisterClassW(
            name , 
            app.unwrap_or(std::ptr::mut_null())
        ).bool()
    }
}