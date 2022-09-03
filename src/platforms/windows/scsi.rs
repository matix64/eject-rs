use std::mem::size_of;
use windows::Win32::Storage::IscsiDisc::{SCSI_IOCTL_DATA_UNSPECIFIED, SCSI_PASS_THROUGH_DIRECT};

// Shouldn't have more than 255 bytes
pub type SenseBuffer = [u8; 32];

#[repr(C, packed)]
pub struct ScsiPassThroughDirectSenseBuffer {
    pub sptd: SCSI_PASS_THROUGH_DIRECT,
    pub sense_buffer: SenseBuffer,
    pub data: Box<[u8]>,
}

impl ScsiPassThroughDirectSenseBuffer {
    pub fn new(mut data: Box<[u8]>) -> Self {
        Self {
            sptd: SCSI_PASS_THROUGH_DIRECT {
                Length: size_of::<SCSI_PASS_THROUGH_DIRECT>() as u16,
                ScsiStatus: 0,
                PathId: 0,
                TargetId: 0,
                Lun: 0,
                Cdb: [0; 16],
                CdbLength: 0,
                DataIn: SCSI_IOCTL_DATA_UNSPECIFIED as u8,
                DataBuffer: data.as_mut_ptr() as *mut _,
                DataTransferLength: data.len() as u32,
                SenseInfoOffset: size_of::<SCSI_PASS_THROUGH_DIRECT>() as u32,
                SenseInfoLength: size_of::<SenseBuffer>() as u8,
                TimeOutValue: 10,
            },
            sense_buffer: Default::default(),
            data,
        }
    }
}
