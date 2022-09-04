use super::device::DeviceHandle;
use std::path::PathBuf;

pub const CDROM_PATHS: &[&str] = &["CdRom0"];

pub struct CdRomDrives {
    next_i: u8,
}

impl CdRomDrives {
    pub fn new() -> Self {
        Self { next_i: 0 }
    }
}

impl Iterator for CdRomDrives {
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
