use super::{scsi::ScsiPassThroughDirectSenseBuffer, util::pcwstr};
use crate::{
    device::DriveStatus,
    error::{Error, ErrorKind, Result},
};
use std::{
    ffi::OsString,
    mem::size_of_val,
    os::raw::c_void,
    path::Path,
    ptr::{null, null_mut},
};
use windows::{
    Win32::System::Ioctl::{IOCTL_STORAGE_EJECT_MEDIA, IOCTL_STORAGE_LOAD_MEDIA2},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Storage::{
            FileSystem::{
                CreateFileW, FILE_ACCESS_FLAGS, FILE_FLAGS_AND_ATTRIBUTES, FILE_GENERIC_READ,
                FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
            },
            IscsiDisc::{IOCTL_SCSI_PASS_THROUGH_DIRECT, SCSI_IOCTL_DATA_IN},
        },
        System::{Ioctl::IOCTL_STORAGE_MEDIA_REMOVAL, IO::DeviceIoControl},
    },
};

pub struct DeviceHandle(pub HANDLE);

impl DeviceHandle {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut result =
            Self::open_with_access_flags(&path, FILE_GENERIC_READ | FILE_GENERIC_WRITE);
        if let Some(ErrorKind::AccessDenied) = result.as_ref().err().map(|e| e.kind) {
            result = Self::open_with_access_flags(&path, FILE_GENERIC_READ);
        }
        result
    }

    pub fn exists(path: impl AsRef<Path>) -> bool {
        Self::open_with_access_flags(path, FILE_ACCESS_FLAGS(0)).is_ok()
    }

    fn open_with_access_flags(path: impl AsRef<Path>, flags: FILE_ACCESS_FLAGS) -> Result<Self> {
        let mut full_path = OsString::from("\\\\?\\");
        full_path.push(path.as_ref().as_os_str());
        let full_path = pcwstr(full_path).unwrap();
        let handle = unsafe {
            CreateFileW(
                full_path,
                flags,
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
        let lock = lock as u8;
        unsafe {
            self.ioctl(
                IOCTL_STORAGE_MEDIA_REMOVAL,
                Some(((&lock) as *const _ as *const c_void, 1)),
                None,
            )?;
        }
        Ok(())
    }

    pub fn status(&self) -> Result<DriveStatus> {
        const DATA_LEN: usize = 8;
        let mut request = ScsiPassThroughDirectSenseBuffer::<DATA_LEN>::new();
        request.sptd.DataIn = SCSI_IOCTL_DATA_IN as u8;
        request.sptd.CdbLength = 10;
        request.sptd.Cdb[0] = 0x4a; // Command: GET EVENT/STATUS NOTIFICATION
        request.sptd.Cdb[1] = 1; // Polled
        request.sptd.Cdb[4] = 0x10; // Event class: media
        request.sptd.Cdb[7] = (DATA_LEN >> 8) as u8;
        request.sptd.Cdb[8] = DATA_LEN as u8;
        unsafe {
            self.ioctl(
                IOCTL_SCSI_PASS_THROUGH_DIRECT,
                Some((&request as *const _ as *const c_void, size_of_val(&request))),
                Some((&mut request as *mut _ as *mut c_void, size_of_val(&request))),
            )?;
        }
        let media_status = request.data[5];
        match media_status {
            0 => Ok(DriveStatus::Empty),
            2 => Ok(DriveStatus::Loaded),
            _ => Ok(DriveStatus::TrayOpen),
        }
    }

    unsafe fn ioctl(
        &self,
        control_code: u32,
        in_buffer: Option<(*const c_void, usize)>,
        out_buffer: Option<(*mut c_void, usize)>,
    ) -> Result<usize> {
        let mut bytes_returned = 0u32;
        let (in_buffer, in_buffer_size) = in_buffer.unwrap_or((null(), 0));
        let (out_buffer, out_buffer_size) = out_buffer.unwrap_or((null_mut(), 0));
        let ok = unsafe {
            DeviceIoControl(
                self.0,
                control_code,
                in_buffer,
                in_buffer_size as u32,
                out_buffer,
                out_buffer_size as u32,
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
