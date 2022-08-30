mod commands;
mod ioctl;
mod sg_io_hdr;

use self::{
    commands::{ALLOW_MEDIUM_REMOVAL, START_STOP},
    ioctl::sg_io,
    sg_io_hdr::{DxferDirection, SgIoHdr},
};
use crate::error::Result;
use std::os::unix::prelude::RawFd;

pub fn eject(fd: RawFd) -> Result<()> {
    let eject_command: &mut [u8] = &mut [START_STOP, 0, 0, 0, 2, 0];
    let mut sense_buffer = [0u8; 32];
    let mut request = SgIoHdr {
        cmdp: eject_command.as_mut_ptr(),
        cmd_len: eject_command.len() as u8,
        dxfer_direction: DxferDirection::None,
        dxferp: [0u8; 2].as_mut_ptr() as *mut _,
        dxfer_len: 0,
        sbp: sense_buffer.as_mut_ptr() as *mut _,
        mx_sb_len: sense_buffer.len() as u8,
        timeout: 10 * 1000,
        ..Default::default()
    };
    unsafe {
        sg_io(fd, (&mut request) as *mut _)?;
    }
    request.check_errors()
}

pub fn retract(fd: RawFd) -> Result<()> {
    let eject_command: &mut [u8] = &mut [START_STOP, 0, 0, 0, 3, 0];
    let mut sense_buffer = [0u8; 32];
    let mut request = SgIoHdr {
        cmdp: eject_command.as_mut_ptr(),
        cmd_len: eject_command.len() as u8,
        dxfer_direction: DxferDirection::None,
        dxferp: [0u8; 2].as_mut_ptr() as *mut _,
        dxfer_len: 0,
        sbp: sense_buffer.as_mut_ptr() as *mut _,
        mx_sb_len: sense_buffer.len() as u8,
        timeout: 10 * 1000,
        ..Default::default()
    };
    unsafe {
        sg_io(fd, (&mut request) as *mut _)?;
    }
    request.check_errors()
}

pub fn set_ejection_lock(fd: RawFd, locked: bool) -> Result<()> {
    let allow_removal_command: &mut [u8] = &mut [ALLOW_MEDIUM_REMOVAL, 0, 0, 0, locked as u8, 0];
    let mut sense_buffer = [0u8; 32];
    let mut request = SgIoHdr {
        cmdp: allow_removal_command.as_mut_ptr(),
        cmd_len: allow_removal_command.len() as u8,
        dxfer_direction: DxferDirection::None,
        dxferp: [0u8; 2].as_mut_ptr() as *mut _,
        dxfer_len: 0,
        sbp: sense_buffer.as_mut_ptr() as *mut _,
        mx_sb_len: sense_buffer.len() as u8,
        timeout: 10 * 1000,
        ..Default::default()
    };
    unsafe {
        sg_io(fd, (&mut request) as *mut _)?;
    }
    request.check_errors()
}
