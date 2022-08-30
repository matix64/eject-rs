// Taken from Linux headers scsi/sg.h and scsi/scsi.h

use crate::error::{Error, ErrorKind, Result};
use std::{ffi::c_void, ptr::null_mut};

#[repr(C)]
pub struct SgIoHdr {
    ///\[i] 'S' for SCSI generic (required)
    pub interface_id: i32,
    ///\[i] data transfer direction
    pub dxfer_direction: DxferDirection,
    ///\[i] SCSI command length ( <= 16 bytes)
    pub cmd_len: u8,
    ///\[i] max length to write to sbp
    pub mx_sb_len: u8,
    ///\[i] 0 implies no scatter gather
    pub iovec_count: u16,
    ///\[i] byte count of data transfer
    pub dxfer_len: u32,
    ///\[i], \[*io] points to data transfer memory or scatter gather list
    pub dxferp: *mut c_void,
    ///\[i], \[*i] points to command to perform
    pub cmdp: *mut u8,
    ///\[i], \[*o] points to sense_buffer memory
    pub sbp: *mut c_void,
    ///\[i] MAX_UINT->no timeout (unit: millisec)
    pub timeout: u32,
    ///\[i] 0 -> default, see SG_FLAG...
    pub flags: u32,
    ///\[i->o] unused internally (normally)
    pub pack_id: i32,
    ///\[i->o] unused internally
    pub usr_ptr: *mut c_void,
    ///\[o] scsi status
    pub status: u8,
    ///\[o] shifted, masked scsi status
    pub masked_status: u8,
    ///\[o] messaging level data (optional)
    pub msg_status: u8,
    ///\[o] byte count actually written to sbp
    pub sb_len_wr: u8,
    ///\[o] errors from host adapter
    pub host_status: u16,
    ///\[o] errors from software driver
    pub driver_status: u16,
    ///\[o] dxfer_len - actual_transferred
    pub resid: i32,
    ///\[o] time taken by cmd (unit: millisec)
    pub duration: u32,
    ///\[o] auxiliary information
    pub info: u32,
}

impl Default for SgIoHdr {
    fn default() -> Self {
        SgIoHdr {
            interface_id: SG_INTERFACE_ID_ORIG,
            cmdp: null_mut(),
            cmd_len: 0,
            dxfer_direction: DxferDirection::None,
            dxferp: null_mut(),
            dxfer_len: 0,
            sbp: null_mut(),
            mx_sb_len: 0,
            timeout: 0,
            iovec_count: 0,
            flags: 0,
            pack_id: 0,
            usr_ptr: null_mut(),
            status: 0,
            masked_status: 0,
            msg_status: 0,
            sb_len_wr: 0,
            host_status: 0,
            driver_status: 0,
            resid: 0,
            duration: 0,
            info: 0,
        }
    }
}

impl SgIoHdr {
    pub fn check_errors(&self) -> Result<()> {
        if self.host_status != 0 {
            Err(Error {
                code: 0,
                message: format!("SG_IO failed with host_status = {}", self.host_status),
                kind: ErrorKind::Unknown,
            })
        } else if self.driver_status != 0 {
            Err(Error {
                code: 0,
                message: format!("SG_IO failed with driver_status = {}", self.driver_status),
                kind: ErrorKind::Unknown,
            })
        } else {
            Ok(())
        }
    }
}

pub const SG_INTERFACE_ID_ORIG: i32 = 'S' as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DxferDirection {
    /// e.g. a SCSI Test Unit Ready command
    None = -1,
    /// e.g. a SCSI WRITE command
    ToDev = -2,
    /// e.g. a SCSI READ command
    FromDev = -3,
    /// treated like SG_DXFER_FROM_DEV with the
    /// additional property than during indirect
    /// IO the user buffer is copied into the
    /// kernel buffers before the transfer
    ToFromDev = -4,
    /// Unknown data direction
    Unknown = -5,
}
