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

use crate::util::to_rust_str;
use odpi_sys::dpiDataBuffer;
use std::result;
use std::slice;

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

pub trait FromAttrValue {
    unsafe fn from_attr_value(data: &dpiDataBuffer, len: u32) -> Result<Self>
    where
        Self: Sized;
}

impl FromAttrValue for bool {
    unsafe fn from_attr_value(value: &dpiDataBuffer, _len: u32) -> Result<Self> {
        Ok(value.asBoolean != 0)
    }
}

impl FromAttrValue for u8 {
    unsafe fn from_attr_value(value: &dpiDataBuffer, _len: u32) -> Result<Self> {
        Ok(value.asUint8)
    }
}

impl FromAttrValue for u16 {
    unsafe fn from_attr_value(value: &dpiDataBuffer, _len: u32) -> Result<Self> {
        Ok(value.asUint16)
    }
}

impl FromAttrValue for u32 {
    unsafe fn from_attr_value(value: &dpiDataBuffer, _len: u32) -> Result<Self> {
        Ok(value.asUint32)
    }
}

impl FromAttrValue for i64 {
    unsafe fn from_attr_value(value: &dpiDataBuffer, _len: u32) -> Result<Self> {
        Ok(value.asInt64)
    }
}

impl FromAttrValue for Vec<u8> {
    unsafe fn from_attr_value(value: &dpiDataBuffer, len: u32) -> Result<Self> {
        Ok(slice::from_raw_parts(value.asRaw as *const u8, len as usize).to_vec())
    }
}

impl FromAttrValue for String {
    unsafe fn from_attr_value(value: &dpiDataBuffer, len: u32) -> Result<Self> {
        Ok(to_rust_str(value.asString, len))
    }
}
