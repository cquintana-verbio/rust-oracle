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
use crate::types::DataTypeInfo;
use crate::util::to_rust_str;
use crate::util::OdpiStr;
use crate::FromAttrValue;
use crate::Result;
use bitflags::bitflags;
use odpi_sys::*;
use std::mem::MaybeUninit;
use std::ptr;

bitflags! {
    /// statement execution modes
    pub struct ExecMode: dpiExecMode {
        const DEFAULT = DPI_MODE_EXEC_DEFAULT;
        const DESCRIBE_ONLY = DPI_MODE_EXEC_DESCRIBE_ONLY;
        const COMMIT_ON_SUCCESS = DPI_MODE_EXEC_COMMIT_ON_SUCCESS;
        const BATCH_ERRORS = DPI_MODE_EXEC_BATCH_ERRORS;
        const PARSE_ONLY = DPI_MODE_EXEC_PARSE_ONLY;
        const ARRAY_DML_ROWCOUNTS = DPI_MODE_EXEC_ARRAY_DML_ROWCOUNTS;
    }
}

pub struct QueryInfo {
    pub name: String,
    pub type_info: DataTypeInfo,
    pub null_ok: bool,
}

impl QueryInfo {
    pub fn new(ctxt: Context, info: &dpiQueryInfo) -> Result<QueryInfo> {
        Ok(QueryInfo {
            name: to_rust_str(info.name, info.nameLength),
            type_info: DataTypeInfo::new(ctxt, &info.typeInfo)?,
            null_ok: info.nullOk != 0,
        })
    }
}

#[derive(Debug)]
pub struct Stmt {
    ctxt: Context,
    raw: *mut dpiStmt,
}

unsafe impl Send for Stmt {}
unsafe impl Sync for Stmt {}

impl Stmt {
    pub fn from_raw(ctxt: Context, raw: *mut dpiStmt) -> Stmt {
        Stmt {
            ctxt: ctxt,
            raw: raw,
        }
    }

    pub fn execute(&self, mode: ExecMode) -> Result<u32> {
        let mut num_query_columns = 0;
        chkerr!(
            self.ctxt,
            dpiStmt_execute(self.raw, mode.bits(), &mut num_query_columns)
        );
        Ok(num_query_columns)
    }

    pub fn fetch(&self) -> Result<Option<u32>> {
        let mut found = 0;
        let mut buffer_row_index = 0;
        chkerr!(
            self.ctxt,
            dpiStmt_fetch(self.raw, &mut found, &mut buffer_row_index)
        );
        if found != 0 {
            Ok(Some(buffer_row_index))
        } else {
            Ok(None)
        }
    }

    pub fn row_count(&self) -> Result<u64> {
        let mut rows = 0;
        chkerr!(self.ctxt, dpiStmt_getRowCount(self.raw, &mut rows));
        Ok(rows)
    }

    pub fn set_fetch_array_size(&self, size: u32) -> Result<()> {
        chkerr!(self.ctxt, dpiStmt_setFetchArraySize(self.raw, size));
        Ok(())
    }

    pub fn close(&mut self, tag: &str) -> Result<()> {
        let tag = OdpiStr::from_str(tag);

        chkerr!(self.ctxt, dpiStmt_close(self.raw, tag.ptr, tag.len));
        self.raw = ptr::null_mut();
        Ok(())
    }

    pub fn query_info(&self, pos: u32) -> Result<QueryInfo> {
        let mut info = MaybeUninit::uninit();
        chkerr!(
            self.ctxt,
            dpiStmt_getQueryInfo(self.raw, pos, info.as_mut_ptr())
        );
        let info = unsafe { info.assume_init() };
        QueryInfo::new(self.ctxt, &info)
    }

    pub unsafe fn oci_attr<T>(&self, attr: u32) -> Result<T>
    where
        T: FromAttrValue,
    {
        let mut value = MaybeUninit::uninit();
        let mut len = 0;
        if dpiStmt_getOciAttr(self.raw, attr, value.as_mut_ptr(), &mut len) != 0 {
            return Err(self.ctxt.last_error());
        }
        let value = value.assume_init();
        <T>::from_attr_value(&value, len)
    }

    pub fn raw(&self) -> *mut dpiStmt {
        self.raw
    }
}

impl Clone for Stmt {
    fn clone(&self) -> Stmt {
        unsafe {
            dpiStmt_addRef(self.raw);
        }
        Stmt::from_raw(self.ctxt, self.raw)
    }
}

impl Drop for Stmt {
    fn drop(&mut self) {
        unsafe {
            dpiStmt_release(self.raw);
        }
    }
}
