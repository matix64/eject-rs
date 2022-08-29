use super::ioctl::{cdrom_lockdoor, cdromclosetray, cdromeject};
use crate::error::{Error, ErrorKind, Result};
use nix::{
    errno::Errno,
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::close,
};
use std::{os::unix::prelude::RawFd, path::Path};

pub struct DeviceHandle(RawFd);

impl DeviceHandle {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        match open(
            path.as_ref(),
            OFlag::O_RDONLY | OFlag::O_NONBLOCK,
            Mode::empty(),
        ) {
            Ok(handle) => Ok(Self(handle)),
            Err(e @ Errno::EINVAL) => Err(Error::from(e).with_kind(ErrorKind::InvalidPath)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn eject(&self) -> Result<()> {
        unsafe {
            cdromeject(self.0)?;
        }
        Ok(())
    }

    pub fn retract(&self) -> Result<()> {
        unsafe {
            cdromclosetray(self.0)?;
        }
        Ok(())
    }

    pub fn set_ejection_lock(&self, lock: bool) -> Result<()> {
        unsafe {
            cdrom_lockdoor(self.0, lock.into())?;
        }
        Ok(())
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        let _ = close(self.0);
    }
}
