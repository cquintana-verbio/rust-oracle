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

use crate::error::Error;
use crate::Result;
use lazy_static::lazy_static;
use odpi_sys::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::sync::Mutex;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
struct OptCString {
    s: Option<CString>,
}

impl OptCString {
    const fn new() -> OptCString {
        OptCString { s: None }
    }

    fn from_string(s: String) -> OptCString {
        OptCString {
            s: Some(unsafe { CString::from_vec_unchecked(s.into_bytes()) }),
        }
    }

    fn from_path(path: &Path) -> OptCString {
        OptCString {
            s: Some(unsafe { CString::from_vec_unchecked(path.as_os_str().as_bytes().to_vec()) }),
        }
    }

    fn as_ptr(&self) -> *const c_char {
        self.s.as_ref().map_or(ptr::null(), |s| s.as_ptr())
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct ContextCreateParams {
    default_driver_name: OptCString,
    load_error_url: OptCString,
    oracle_client_lib_dir: OptCString,
    oracle_client_config_dir: OptCString,
}

impl ContextCreateParams {
    pub const fn new() -> ContextCreateParams {
        ContextCreateParams {
            default_driver_name: OptCString::new(),
            load_error_url: OptCString::new(),
            oracle_client_lib_dir: OptCString::new(),
            oracle_client_config_dir: OptCString::new(),
        }
    }

    pub fn default_driver_name<N>(&mut self, name: N) -> &mut ContextCreateParams
    where
        N: Into<String>,
    {
        self.default_driver_name = OptCString::from_string(name.into());
        self
    }

    pub fn load_error_url<U>(&mut self, url: U) -> &mut ContextCreateParams
    where
        U: Into<String>,
    {
        self.load_error_url = OptCString::from_string(url.into());
        self
    }

    pub fn oracle_client_lib_dir<D>(&mut self, dir: D) -> &mut ContextCreateParams
    where
        D: AsRef<Path>,
    {
        self.oracle_client_lib_dir = OptCString::from_path(dir.as_ref());
        self
    }

    pub fn oracle_client_config_dir<D>(&mut self, dir: D) -> &mut ContextCreateParams
    where
        D: AsRef<Path>,
    {
        self.oracle_client_config_dir = OptCString::from_path(dir.as_ref());
        self
    }

    unsafe fn to_dpi(&self) -> dpiContextCreateParams {
        dpiContextCreateParams {
            defaultDriverName: self.default_driver_name.as_ptr(),
            defaultEncoding: ptr::null(),
            loadErrorUrl: self.load_error_url.as_ptr(),
            oracleClientLibDir: self.oracle_client_lib_dir.as_ptr(),
            oracleClientConfigDir: self.oracle_client_config_dir.as_ptr(),
        }
    }
}

/// https://oracle.github.io/odpi/doc/functions/dpiContext.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Context {
    pub(crate) context: *mut dpiContext,
}

unsafe impl Sync for Context {}
unsafe impl Send for Context {}

lazy_static! {
    static ref CONTEXT_MAP: Mutex<HashMap<ContextCreateParams, Context>> =
        Mutex::new(HashMap::new());
}

impl Context {
    pub fn new() -> Result<Context> {
        Context::from_cache(&ContextCreateParams::new())
    }

    pub fn with_params(params: &ContextCreateParams) -> Result<Context> {
        Context::from_cache(params)
    }

    pub fn last_error(&self) -> Error {
        unsafe {
            let mut err = MaybeUninit::uninit();
            dpiContext_getError(self.context, err.as_mut_ptr());
            let err = err.assume_init();
            Error::with_raw_err(&err)
        }
    }

    pub fn common_create_params(&self) -> dpiCommonCreateParams {
        let mut params = MaybeUninit::uninit();
        unsafe {
            dpiContext_initCommonCreateParams(self.context, params.as_mut_ptr());
            let mut params = params.assume_init();
            params.createMode |= DPI_MODE_CREATE_THREADED;
            params
        }
    }

    pub fn conn_create_params(&self) -> dpiConnCreateParams {
        let mut params = MaybeUninit::uninit();
        unsafe {
            dpiContext_initConnCreateParams(self.context, params.as_mut_ptr());
            params.assume_init()
        }
    }

    fn from_cache(params: &ContextCreateParams) -> Result<Context> {
        let mut map = CONTEXT_MAP.lock().unwrap();
        if let Some(ctxt) = map.get(params) {
            Ok(ctxt.clone())
        } else {
            let ctxt = Context::new_inner(params)?;
            map.insert(params.clone(), ctxt.clone());
            Ok(ctxt)
        }
    }

    pub fn as_mut_ptr(&self) -> *mut dpiContext {
        self.context
    }

    fn new_inner(params: &ContextCreateParams) -> Result<Context> {
        unsafe {
            let mut ctxt = ptr::null_mut();
            let mut err = MaybeUninit::uninit();
            let mut prms = params.to_dpi();
            if prms.defaultDriverName.is_null() {
                let driver_name: &'static str =
                    concat!("odpi-rs : ", env!("CARGO_PKG_VERSION"), "\0");
                prms.defaultDriverName = driver_name.as_ptr() as *const c_char;
            }
            if dpiContext_createWithParams(
                DPI_MAJOR_VERSION,
                DPI_MINOR_VERSION,
                &mut prms,
                &mut ctxt,
                err.as_mut_ptr(),
            ) == DPI_SUCCESS as i32
            {
                Ok(Context { context: ctxt })
            } else {
                let err = err.assume_init();
                Err(Error::with_raw_err(&err))
            }
        }
    }
}
