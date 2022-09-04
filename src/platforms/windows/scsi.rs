use memoffset::offset_of;
use std::{
    ffi::c_void,
    mem::size_of,
    ops::{Index, IndexMut},
};
use windows::Win32::Storage::IscsiDisc::{SCSI_IOCTL_DATA_UNSPECIFIED, SCSI_PASS_THROUGH_DIRECT};

// Shouldn't have more than 255 bytes
pub type SenseBuffer = [u8; 32];

// Alignment must match that of the device https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddscsi/ni-ntddscsi-ioctl_scsi_pass_through_direct#remarks
// Since we don't know it we use the max alignment: double DWORD https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddstor/ns-ntddstor-_storage_adapter_descriptor
#[repr(C, align(64))]
pub struct ScsiDataBuffer<const N: usize>([u8; N]);

#[repr(C)]
pub struct ScsiPassThroughDirectSenseBuffer<const DATA_LEN: usize> {
    pub sptd: SCSI_PASS_THROUGH_DIRECT,
    pub sense_buffer: SenseBuffer,
    pub data: Box<ScsiDataBuffer<DATA_LEN>>,
}

impl<const DATA_LEN: usize> ScsiPassThroughDirectSenseBuffer<DATA_LEN> {
    pub fn new() -> Self {
        let mut data = Box::new(ScsiDataBuffer([0; DATA_LEN]));
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
                DataBuffer: data.as_mut() as *mut ScsiDataBuffer<DATA_LEN> as *mut c_void,
                DataTransferLength: DATA_LEN as u32,
                SenseInfoOffset: offset_of!(Self, sense_buffer) as u32,
                SenseInfoLength: size_of::<SenseBuffer>() as u8,
                TimeOutValue: 10,
            },
            sense_buffer: Default::default(),
            data,
        }
    }
}

impl<const N: usize> AsRef<[u8]> for ScsiDataBuffer<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> Index<usize> for ScsiDataBuffer<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for ScsiDataBuffer<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
