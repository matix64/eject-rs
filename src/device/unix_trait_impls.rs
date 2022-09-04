use super::Device;
use crate::platform::device::DeviceHandle;
use std::{mem::forget, os::unix::prelude::*};

impl AsRawFd for Device {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.handle.0
    }
}

impl AsFd for Device {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl FromRawFd for Device {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self {
            handle: DeviceHandle(fd),
        }
    }
}

impl From<OwnedFd> for Device {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(fd.into_raw_fd()) }
    }
}

impl IntoRawFd for Device {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let fd = self.as_raw_fd();
        forget(self);
        fd
    }
}
