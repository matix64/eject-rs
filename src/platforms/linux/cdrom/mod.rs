mod ioctl;

use self::ioctl::{cdrom_drive_status, cdrom_lockdoor, cdromclosetray, cdromeject};
use crate::{
    device::DriveStatus,
    error::{Error, ErrorKind, Result},
};
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

pub fn status(fd: RawFd, slot: i32) -> Result<DriveStatus> {
    let status = unsafe { cdrom_drive_status(fd, slot)? };
    // From linux/cdrom.h
    match status {
        0 => Err(Error {
            code: 0,
            message: "cannot get drive status".to_owned(),
            kind: ErrorKind::UnsupportedOperation,
        }),
        1 => Ok(DriveStatus::Empty),
        2 => Ok(DriveStatus::TrayOpen),
        3 => Ok(DriveStatus::NotReady),
        4 => Ok(DriveStatus::Loaded),
        // This should never happen
        _ => Err(Error {
            code: 0,
            message: format!("CDROM_DRIVE_STATUS returned unknown status: {status}"),
            kind: ErrorKind::Unknown,
        }),
    }
}
