use super::{cdrom, scsi};
use crate::{
    device::DriveStatus,
    error::{ErrorKind, Result},
};
use nix::{
    fcntl::{open, OFlag},
    libc::EINVAL,
    sys::stat::Mode,
    unistd::close,
};
use std::{os::unix::prelude::RawFd, path::Path};

pub struct DeviceHandle(RawFd);

impl DeviceHandle {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut result: Result<RawFd> = open(
            path.as_ref(),
            OFlag::O_RDWR | OFlag::O_NONBLOCK,
            Mode::empty(),
        )
        .map_err(Into::into);
        if result.as_ref().err().map(|e| e.kind) == Some(ErrorKind::AccessDenied) {
            // Try again without writing permission
            result = open(
                path.as_ref(),
                OFlag::O_RDONLY | OFlag::O_NONBLOCK,
                Mode::empty(),
            )
            .map_err(Into::into);
        }
        if let Err(err) = &mut result {
            if err.code == EINVAL {
                err.kind = ErrorKind::InvalidPath;
            }
        }
        result.map(Self)
    }

    pub fn eject(&self) -> Result<()> {
        if cdrom::set_ejection_lock(self.0, false)
            .and_then(|_| cdrom::eject(self.0))
            .is_ok()
        {
            return Ok(());
        }
        scsi::set_ejection_lock(self.0, false)?;
        scsi::eject(self.0)
    }

    pub fn retract(&self) -> Result<()> {
        if cdrom::retract(self.0).is_ok() {
            return Ok(());
        }
        scsi::retract(self.0)
    }

    pub fn set_ejection_lock(&self, locked: bool) -> Result<()> {
        if cdrom::set_ejection_lock(self.0, locked).is_ok() {
            return Ok(());
        }
        scsi::set_ejection_lock(self.0, locked)
    }

    pub fn status(&self) -> Result<DriveStatus> {
        cdrom::status(self.0, 0)
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        let _ = close(self.0);
    }
}
