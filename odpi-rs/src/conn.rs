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

use crate::context::Context;
use crate::FromAttrValue;
use crate::Result;
use odpi_sys::*;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct Conn {
    ctxt: Context,
    raw: *mut dpiConn,
}

unsafe impl Send for Conn {}
unsafe impl Sync for Conn {}

impl Conn {
    pub fn from_raw(ctxt: Context, raw: *mut dpiConn) -> Conn {
        Conn {
            ctxt: ctxt,
            raw: raw,
        }
    }

    pub fn ctxt(&self) -> Context {
        self.ctxt
    }

    pub unsafe fn oci_attr<T>(&self, handle_type: u32, attr: u32) -> Result<T>
    where
        T: FromAttrValue,
    {
        let mut value = MaybeUninit::uninit();
        let mut len = 0;
        if dpiConn_getOciAttr(self.raw, handle_type, attr, value.as_mut_ptr(), &mut len) != 0 {
            return Err(self.ctxt.last_error());
        }
        let value = value.assume_init();
        <T>::from_attr_value(&value, len)
    }

    pub fn raw(&self) -> *mut dpiConn {
        self.raw
    }
}

impl Clone for Conn {
    fn clone(&self) -> Conn {
        unsafe {
            dpiConn_addRef(self.raw);
        }
        Conn::from_raw(self.ctxt, self.raw)
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        unsafe {
            dpiConn_release(self.raw);
        }
    }
}
