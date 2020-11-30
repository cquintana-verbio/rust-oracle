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

macro_rules! dpi_enum {
    () => {};
    (
        $(#[$outer_meta:meta])*
        pub enum $enum_name:ident : $t:ty {
            $(
              $(#[$inner_meta:meta])*
              $name:ident = $value:ident $(as $val_type:ty)?,
            )*
        }
        $($tail:tt)*
    ) => {
        $(#[$outer_meta])*
        pub enum $enum_name {
            $(
              $(#[$inner_meta])*
              $name = $value $(as $val_type)?,
            )*
        }
        impl ::std::convert::TryFrom<$t> for $enum_name {
            type Error = $crate::error::Error;
            fn try_from(value: $t) -> $crate::Result<$enum_name> {
                match value as u32 {
                    $($value => Ok($enum_name::$name),)*
                    _ => Err($crate::error::Error::OutOfRange(format!(concat!("Invalid ", stringify!($enum_name), " number: {}"), value)))
                }
            }
        }
        impl From<$enum_name> for $t {
            fn from(value: $enum_name) -> $t {
                value as $t
            }
        }
        dpi_enum!($($tail)*);
    }
}

pub mod conn;
pub mod context;
pub mod error;
pub mod stmt;
pub mod types;
mod util;

pub type Result<T> = result::Result<T, error::Error>;
