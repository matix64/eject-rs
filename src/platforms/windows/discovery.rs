use super::device::DeviceHandle;
use std::path::PathBuf;

pub const CDROM_PATHS: &[&str] = &["CdRom0"];

pub struct CdDrives {
    next_i: u8,
}

impl CdDrives {
    pub fn new() -> Self {
        Self { next_i: 0 }
    }
}

impl Iterator for CdDrives {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let path = PathBuf::from(format!("CdRom{}", self.next_i));
        if DeviceHandle::exists(&path) {
            self.next_i += 1;
            Some(path)
        } else {
            None
        }
    }
}
