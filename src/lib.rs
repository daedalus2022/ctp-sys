// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些警告
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use std::ffi::CStr;

#[allow(clippy::all)]
mod bindings;

pub use bindings::*;

// 高层的 API，获取版本，一般应该出现在另一个 crate
pub fn get_api_version() -> Option<String>{
    unsafe {
        if let Ok(version) = CStr::from_ptr(CThostFtdcMdApi::GetApiVersion()).to_str(){
            return Some(version.to_string())
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_api_version_works() {
        assert_eq!(get_api_version().is_some(), true);
        assert_eq!("v6.6.5_20210924 14:18:43.576",get_api_version().unwrap());
    }
}
