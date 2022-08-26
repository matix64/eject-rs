use super::util::pcwstr;
use crate::error::{Error, Result};
use std::{ffi::OsString, os::windows::prelude::OsStringExt, path::Path};
use windows::Win32::{
    Foundation::ERROR_INVALID_NAME,
    Storage::FileSystem::{GetDriveTypeW, GetLogicalDriveStringsW},
};

pub fn list_drives() -> Vec<OsString> {
    let size = unsafe { GetLogicalDriveStringsW(&mut []) };
    let mut buffer = vec![0; size as usize];
    unsafe { GetLogicalDriveStringsW(&mut buffer) };
    let mut list: Vec<_> = buffer
        .split(|x| *x == 0)
        .map(OsStringExt::from_wide)
        .collect();
    list.truncate(list.len() - 2);
    list
}

pub fn get_drive_type(root_path: impl AsRef<Path>) -> Result<WindowsDriveType> {
    let path = pcwstr(root_path.as_ref()).map_err(|_| Error::from_os_err(ERROR_INVALID_NAME))?;
    let dtype = unsafe { GetDriveTypeW(path) };
    Ok(match dtype {
        _ => WindowsDriveType::Unknown,
    })
}

pub enum WindowsDriveType {
    Unknown,
    Removable,
    Fixed,
    Remote,
    CdRom,
    RamDisk,
}
