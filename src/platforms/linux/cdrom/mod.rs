mod ioctl;

use self::ioctl::{cdrom_lockdoor, cdromclosetray, cdromeject};
use crate::error::Result;
use std::os::unix::prelude::RawFd;

pub fn eject(fd: RawFd) -> Result<()> {
    unsafe {
        cdromeject(fd)?;
    }
    Ok(())
}

pub fn retract(fd: RawFd) -> Result<()> {
    unsafe {
        cdromclosetray(fd)?;
    }
    Ok(())
}

pub fn set_ejection_lock(fd: RawFd, locked: bool) -> Result<()> {
    unsafe {
        cdrom_lockdoor(fd, locked.into())?;
    }
    Ok(())
}
