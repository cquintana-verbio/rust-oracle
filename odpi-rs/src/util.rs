// odpi-rs - straight bindings to ODPI-C
//
// URL: https://github.com/kubo/rust-oracle
//
//-----------------------------------------------------------------------------
// Copyright (c) 2020 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

pub fn to_rust_str(ptr: *const c_char, len: u32) -> String {
    if ptr.is_null() {
        "".to_string()
    } else {
        let s = unsafe { slice::from_raw_parts(ptr as *mut u8, len as usize) };
        String::from_utf8_lossy(s).into_owned()
    }
}

pub fn ptr_to_rust_str(ptr: *const c_char) -> String {
    if ptr.is_null() {
        "".to_string()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

pub trait AssertSend: Send {}
pub trait AssertSync: Sync {}

pub(crate) struct OdpiStr {
    pub ptr: *const c_char,
    pub len: u32,
}

impl OdpiStr {
    pub fn new() -> OdpiStr {
        OdpiStr {
            ptr: ptr::null(),
            len: 0,
        }
    }

    pub fn from_str(s: &str) -> OdpiStr {
        if s.len() == 0 {
            OdpiStr::new()
        } else {
            OdpiStr {
                ptr: s.as_ptr() as *const c_char,
                len: s.len() as u32,
            }
        }
    }
}
