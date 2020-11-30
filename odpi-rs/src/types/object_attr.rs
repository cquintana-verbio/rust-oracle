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
use crate::types::ObjectAttrInfo;
use crate::Result;
use odpi_sys::*;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct ObjectAttr {
    ctxt: Context,
    raw: *mut dpiObjectAttr,
}

unsafe impl Send for ObjectAttr {}
unsafe impl Sync for ObjectAttr {}

impl ObjectAttr {
    pub fn from_raw(ctxt: Context, raw: *mut dpiObjectAttr) -> ObjectAttr {
        ObjectAttr {
            ctxt: ctxt,
            raw: raw,
        }
    }

    pub fn info(&self) -> Result<ObjectAttrInfo> {
        let mut info = MaybeUninit::uninit();
        chkerr!(
            self.ctxt,
            dpiObjectAttr_getInfo(self.raw, info.as_mut_ptr())
        );
        let info = unsafe { info.assume_init() };
        ObjectAttrInfo::new(self.ctxt, &info)
    }

    pub fn raw(&self) -> *mut dpiObjectAttr {
        self.raw
    }
}

impl Clone for ObjectAttr {
    fn clone(&self) -> ObjectAttr {
        unsafe {
            dpiObjectAttr_addRef(self.raw);
        }
        ObjectAttr::from_raw(self.ctxt, self.raw)
    }
}

impl Drop for ObjectAttr {
    fn drop(&mut self) {
        unsafe {
            dpiObjectAttr_release(self.raw);
        }
    }
}
