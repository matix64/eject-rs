use std::path::PathBuf;

pub const CDROM_PATHS: &[&str] = &["/dev/cdrom", "/dev/sr0"];

#[derive(Debug, Clone)]
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
        let path = PathBuf::from(format!("/dev/sr{}", self.next_i));
        if path.exists() {
            self.next_i += 1;
            Some(path)
        } else {
            None
        }
    }
}
