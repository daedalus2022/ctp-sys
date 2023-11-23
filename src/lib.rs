// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些警告
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(unused_variables, unused_mut)]
#![allow(clippy::explicit_auto_deref)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod ffi_utils;

use bincode::{Decode, Encode};
pub use ffi_utils::*;
use std::ffi::CStr;

// 高层的 API，获取版本，一般应该出现在另一个 crate
pub fn get_api_version() -> Option<String> {
    unsafe {
        if let Ok(version) = CStr::from_ptr(CThostFtdcMdApi::GetApiVersion()).to_str() {
            return Some(version.to_string());
        }
    }
    None
}

pub mod md_api {
    use crate::*;
    include!("md_impl.rs");
    pub fn create_api(
        flow_path: &str,
        b_is_using_udp: bool,
        b_is_multicast: bool,
    ) -> Box<CThostFtdcMdApi> {
        let md_flow_path = std::ffi::CString::new(flow_path).unwrap();
        unsafe {
            Box::from_raw(CThostFtdcMdApi_CreateFtdcMdApi(
                md_flow_path.as_ptr(),
                b_is_using_udp,
                b_is_multicast,
            ))
        }
    }
}

pub mod trader_api {
    use crate::*;
    include!("trade_impl.rs");
    pub fn create_api(flow_path: &str, b_encrypt: bool) -> Box<CThostFtdcTraderApi> {
        let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
        unsafe {
            Box::from_raw(CThostFtdcTraderApi_CreateFtdcTraderApi(
                trade_flow_path.as_ptr(),
            ))
        }
    }
    pub fn unsafe_clone_api(
        source: Box<CThostFtdcTraderApi>,
    ) -> (Box<CThostFtdcTraderApi>, Box<CThostFtdcTraderApi>) {
        let p = Box::into_raw(source);
        unsafe {
            let p2 = p.clone();
            (Box::from_raw(p), Box::from_raw(p2))
        }
    }
}

/// 典型的账户配置
pub struct CtpAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub name_server: String,
    pub trade_front: String,
    pub md_front: String,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_api_version_works() {
        assert_eq!(get_api_version().is_some(), true);
        assert_eq!("v6.6.5_20210924 14:18:43.576", get_api_version().unwrap());
    }
}
