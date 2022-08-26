use std::{ffi::OsStr, os::windows::prelude::OsStrExt};
use thiserror::Error;
use windows::core::PCWSTR;

pub fn pcwstr(str: impl AsRef<OsStr>) -> Result<PCWSTR, StrContainsNullErr> {
    let mut vec = Vec::with_capacity(str.as_ref().len() + 1);
    for c in str.as_ref().encode_wide() {
        if c == 0 {
            return Err(StrContainsNullErr);
        }
        vec.push(c);
    }
    vec.push(0);
    Ok(PCWSTR::from_raw(vec.as_ptr()))
}

#[derive(Debug, Error, Clone, Copy)]
#[error("{:?}", self)]
pub struct StrContainsNullErr;
