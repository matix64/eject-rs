use super::Device;
use crate::platform::device::DeviceHandle;
use std::os::windows::prelude::*;
use windows::Win32::Foundation::HANDLE;

impl AsRawHandle for Device {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.handle.0 .0 as *mut _
    }
}

impl AsHandle for Device {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

impl From<OwnedHandle> for Device {
    #[inline]
    fn from(handle: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(handle.into_raw_handle()) }
    }
}

impl FromRawHandle for Device {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self {
            handle: DeviceHandle(HANDLE(handle as isize)),
        }
    }
}

impl IntoRawHandle for Device {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.handle.0 .0 as *mut _
    }
}
