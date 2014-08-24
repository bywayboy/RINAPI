use ustd::ui::os::windows::types::Constants::WindowClassStyle;

pub static         VREDRAW : WindowClassStyle = 0x0001;
pub static         HREDRAW : WindowClassStyle = 0x0002;
pub static         DBLCLKS : WindowClassStyle = 0x0008;
pub static           OWNDC : WindowClassStyle = 0x0020;
pub static         CLASSDC : WindowClassStyle = 0x0040;
pub static        PARENTDC : WindowClassStyle = 0x0080;
pub static         NOCLOSE : WindowClassStyle = 0x0200;
pub static        SAVEBITS : WindowClassStyle = 0x0800;
pub static BYTEALIGNCLIENT : WindowClassStyle = 0x1000;
pub static BYTEALIGNWINDOW : WindowClassStyle = 0x2000;
pub static     GLOBALCLASS : WindowClassStyle = 0x4000;
pub static             IME : WindowClassStyle = 0x00010000;
pub static      DROPSHADOW : WindowClassStyle = 0x00020000;