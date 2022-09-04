use super::Device;
use crate::platform::device::DeviceHandle;
use std::os::unix::prelude::*;

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.handle.0
    }
}

impl AsFd for Device {
    fn as_fd<'a>(&'a self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw(self.handle.0) }
    }
}

impl From<OwnedFd> for Device {
    fn from(fd: OwnedFd) -> Self {
        Self {
            handle: DeviceHandle(fd.as_raw_fd()),
        }
    }
}

impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self {
            handle: DeviceHandle(fd),
        }
    }
}

impl IntoRawFd for Device {
    fn into_raw_fd(self) -> RawFd {
        self.handle.0
    }
}
