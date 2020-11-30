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
use crate::types::ObjectAttr;
use crate::types::ObjectTypeInfo;
use crate::Result;
use odpi_sys::*;
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
pub struct ObjectType {
    ctxt: Context,
    raw: *mut dpiObjectType,
    num_attrs: u16,
}

unsafe impl Send for ObjectType {}
unsafe impl Sync for ObjectType {}

impl ObjectType {
    pub fn from_raw(ctxt: Context, raw: *mut dpiObjectType) -> Result<ObjectType> {
        let mut info = MaybeUninit::uninit();
        chkerr!(ctxt, dpiObjectType_getInfo(raw, info.as_mut_ptr()));
        let info = unsafe { info.assume_init() };
        Ok(ObjectType {
            ctxt: ctxt,
            raw: raw,
            num_attrs: info.numAttributes,
        })
    }

    pub fn with_add_ref(ctxt: Context, raw: *mut dpiObjectType) -> Result<ObjectType> {
        let objtype = ObjectType::from_raw(ctxt, raw)?;
        unsafe {
            dpiObjectType_addRef(raw);
        }
        Ok(objtype)
    }

    pub fn info(&self) -> Result<ObjectTypeInfo> {
        let mut info = MaybeUninit::uninit();
        chkerr!(
            self.ctxt,
            dpiObjectType_getInfo(self.raw, info.as_mut_ptr())
        );
        let info = unsafe { info.assume_init() };
        ObjectTypeInfo::new(self.ctxt, &info)
    }

    pub fn attributes(&self) -> Result<Vec<ObjectAttr>> {
        if self.num_attrs == 0 {
            return Ok(Vec::new());
        }
        let mut attrs = Vec::with_capacity(self.num_attrs as usize);
        attrs.resize(self.num_attrs as usize, ptr::null_mut());
        chkerr!(
            self.ctxt,
            dpiObjectType_getAttributes(self.raw, self.num_attrs, attrs.as_mut_ptr())
        );
        Ok(attrs
            .into_iter()
            .map(|attr| ObjectAttr::from_raw(self.ctxt, attr))
            .collect())
    }

    pub fn raw(&self) -> *mut dpiObjectType {
        self.raw
    }
}

impl Clone for ObjectType {
    fn clone(&self) -> ObjectType {
        unsafe {
            dpiObjectType_addRef(self.raw);
        }
        ObjectType {
            ctxt: self.ctxt,
            raw: self.raw,
            num_attrs: self.num_attrs,
        }
    }
}

impl Drop for ObjectType {
    fn drop(&mut self) {
        unsafe {
            dpiObjectType_release(self.raw);
        }
    }
}
