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

use std::result;

macro_rules! chkerr {
    ($ctxt:expr, $code:expr) => {{
        if unsafe { $code } == DPI_SUCCESS as i32 {
            ()
        } else {
            return Err($ctxt.last_error());
        }
    }};
    ($ctxt:expr, $code:expr, $cleanup:stmt) => {{
        if unsafe { $code } == DPI_SUCCESS as i32 {
            ()
        } else {
            let err = $ctxt.last_error();
            $cleanup
            return Err(err);
        }
    }};
}

pub mod conn;
pub mod context;
pub mod error;
pub mod stmt;
mod util;

pub type Result<T> = result::Result<T, error::Error>;
