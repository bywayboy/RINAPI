/********************************************************************************
<summary>
源碼路徑: ustd

關於 Windows API 中所有 C 語言類型的轉換:

所有 C 語言標準類型會重定義一次（因爲是由 Microsoft C Compiler 負責解釋）
eg: int => CCINT , float => CCFLOAT , double => CCDOUBLE
所有 Microsoft 通過 #define 或者 typedef 定義的 Windows API 類型會移植拷貝過來
eg: type PVOID = *mut c_void; type HANDLE = PVOID; type WCHAR = wchar_t;

這裏接口函數定義除了必要的重定義外基本保證與 Windows API 文檔一致。
標示參數出入方向的宏定義改爲註釋代碼追尾到形參字段後。

ustd::os::windows::common::types::win32
爲兼容 Windows C API 數據類型，標記爲：#[allow(non_camel_case_types)]
</summary>
********************************************************************************/
//#![crate_name="ustd"]
//#![crate_id = "ustd#0.01"]
//#![comment = "The Rust Windows API Binding library"]
//#![license = "BSD"]
//#![crate_type = "rlib"]
//#![crate_type = "dylib"]
#[allow(dead_code)]
#[allow(non_snake_case_functions)]
#[allow(unused_variable)]
pub mod os;