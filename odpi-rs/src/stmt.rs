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
use crate::util::OdpiStr;
use crate::Result;
use bitflags::bitflags;
use odpi_sys::*;
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
