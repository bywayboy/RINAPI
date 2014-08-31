use ustd::os::windows::dev::ui::etypes::MessageBoxStyle;

/**
    WinUser.h:7444
**/
pub mod Button {
    pub static            OnlyOk : MessageBoxStyle = 0x00000000 ;
    pub static          OkCancel : MessageBoxStyle = 0x00000001 ;
    pub static  AbortRetryIgnore : MessageBoxStyle = 0x00000002 ;
    pub static       YesNoCancel : MessageBoxStyle = 0x00000003 ;
    pub static             YesNo : MessageBoxStyle = 0x00000004 ;
    pub static       RetryCancel : MessageBoxStyle = 0x00000005 ;
    pub static CancelTryContinue : MessageBoxStyle = 0x00000006 ;
    pub static              Help : MessageBoxStyle = 0x00004000 ;
}

/**
    WinUser.h:7455
**/
pub mod Icon {
    pub static        Hand : MessageBoxStyle = 0x00000010   ;
    pub static    Question : MessageBoxStyle = 0x00000020   ;
    pub static Exclamation : MessageBoxStyle = 0x00000030   ;
    pub static    Asterisk : MessageBoxStyle = 0x00000040   ;
    pub static        User : MessageBoxStyle = 0x00000080   ;
    pub static     Warning : MessageBoxStyle = Exclamation  ;
    pub static       Error : MessageBoxStyle = Hand         ;
    pub static Information : MessageBoxStyle = Asterisk     ;
    pub static        Stop : MessageBoxStyle = Hand         ;
}

/**
    WinUser.h:7469
**/
pub mod DefaultButton {
    pub static  First : MessageBoxStyle = 0x00000000 ;
    pub static Second : MessageBoxStyle = 0x00000100 ;
    pub static  Third : MessageBoxStyle = 0x00000200 ;
    pub static Fourth : MessageBoxStyle = 0x00000300 ;
}

/**
    WinUser.h:7476
**/
pub mod Modal {
    pub static Application : MessageBoxStyle = 0x00000000 ;
    pub static      System : MessageBoxStyle = 0x00001000 ;
    pub static        Task : MessageBoxStyle = 0x00002000 ;
}

/**
    WinUser.h:7483
**/
pub mod OtherOption {
    pub static             NoFocus : MessageBoxStyle = 0x00008000 ;
    pub static       SetForeground : MessageBoxStyle = 0x00010000 ;
    pub static  DefaultDesktopOnly : MessageBoxStyle = 0x00020000 ;
    pub static             TopMost : MessageBoxStyle = 0x00040000 ;
    pub static      RightJustified : MessageBoxStyle = 0x00080000 ;
    pub static          RTLReading : MessageBoxStyle = 0x00100000 ;
    pub static ServiceNotification : MessageBoxStyle = 0x00200000 ;
}