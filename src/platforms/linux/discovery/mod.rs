use std::path::PathBuf;

pub const CDROM_PATHS: &[&str] = &["/dev/cdrom", "/dev/sr0"];

#[derive(Debug, Clone)]
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
        let path = PathBuf::from(format!("/dev/sr{}", self.next_i));
        if path.exists() {
            self.next_i += 1;
            Some(path)
        } else {
            None
        }
    }
}
