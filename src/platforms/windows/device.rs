use super::util::pcwstr;
use crate::error::{Error, Result};
use std::{
    ffi::OsString,
    os::raw::c_void,
    path::Path,
    ptr::{null, null_mut},
};
use windows::{
    Win32::System::Ioctl::{IOCTL_STORAGE_EJECT_MEDIA, IOCTL_STORAGE_LOAD_MEDIA2},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Storage::FileSystem::{
            CreateFileW, FILE_FLAGS_AND_ATTRIBUTES, FILE_GENERIC_READ, FILE_SHARE_READ,
            FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::{Ioctl::IOCTL_STORAGE_MEDIA_REMOVAL, IO::DeviceIoControl},
    },
};

pub struct DeviceHandle(HANDLE);

impl DeviceHandle {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut full_path = OsString::from("\\\\?\\");
        full_path.push(path.as_ref().as_os_str());
        let handle = unsafe {
            CreateFileW(
                pcwstr(&full_path).unwrap(),
                FILE_GENERIC_READ,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null(),
                OPEN_EXISTING,
                FILE_FLAGS_AND_ATTRIBUTES(0),
                HANDLE(0),
            )
        }?;
        Ok(Self(handle))
    }

    pub fn eject(&self) -> Result<()> {
        unsafe {
            self.ioctl(IOCTL_STORAGE_EJECT_MEDIA, None, None)?;
        }
        Ok(())
    }

    pub fn retract(&self) -> Result<()> {
        unsafe {
            self.ioctl(IOCTL_STORAGE_LOAD_MEDIA2, None, None)?;
        }
        Ok(())
    }

    pub fn set_ejection_lock(&self, lock: bool) -> Result<()> {
        unsafe {
            self.ioctl(IOCTL_STORAGE_MEDIA_REMOVAL, Some(&[lock as u8]), None)?;
        }
        Ok(())
    }

    unsafe fn ioctl(
        &self,
        control_code: u32,
        in_buffer: Option<&[u8]>,
        out_buffer: Option<&mut [u8]>,
    ) -> Result<usize> {
        let mut bytes_returned = 0u32;
        let (in_buffer, in_buffer_size) = if let Some(buf) = in_buffer {
            (buf.as_ptr() as *const c_void, buf.len() as u32)
        } else {
            (null(), 0)
        };
        let (out_buffer, out_buffer_size) = if let Some(buf) = out_buffer {
            (buf.as_ptr() as *mut c_void, buf.len() as u32)
        } else {
            (null_mut(), 0)
        };
        let ok = unsafe {
            DeviceIoControl(
                self.0,
                control_code,
                in_buffer,
                in_buffer_size,
                out_buffer,
                out_buffer_size,
                (&mut bytes_returned) as *mut _,
                null_mut(),
            )
        };
        if !ok.as_bool() {
            if let Some(err) = Error::get_last_error() {
                return Err(err);
            }
        }
        Ok(bytes_returned as usize)
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
