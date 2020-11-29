// Rust-oracle - Rust binding for Oracle database
//
// URL: https://github.com/kubo/rust-oracle
//
//-----------------------------------------------------------------------------
// Copyright (c) 2017-2018 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------

use crate::Context;
use odpi_rs::error::Error;
use odpi_sys::dpiContext_getError;
use std::mem::MaybeUninit;

pub(crate) fn error_from_context(ctxt: &Context) -> Error {
    unsafe {
        let mut err = MaybeUninit::uninit();
        dpiContext_getError(ctxt.context, err.as_mut_ptr());
        let err = err.assume_init();
        Error::with_raw_err(&err)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! chkerr {
    ($ctxt:expr, $code:expr) => {{
        if unsafe { $code } == DPI_SUCCESS as i32 {
            ()
        } else {
            return Err($crate::error::error_from_context($ctxt));
        }
    }};
    ($ctxt:expr, $code:expr, $cleanup:stmt) => {{
        if unsafe { $code } == DPI_SUCCESS as i32 {
            ()
        } else {
            let err = $crate::error::error_from_context($ctxt);
            $cleanup
            return Err(err);
        }
    }};
}
