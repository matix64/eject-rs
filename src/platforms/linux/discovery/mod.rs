use crate::error::Result;
use std::{
    collections::HashSet,
    fs::{canonicalize, read_dir, ReadDir},
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

pub const CDROM_PATHS: &[&str] = &["/dev/cdrom", "/dev/sr0"];

pub struct CdDrives {
    seen_before: HashSet<PathBuf>,
    devs_dir: ReadDir,
}

impl CdDrives {
    pub fn new() -> Result<Self> {
        Ok(Self {
            seen_before: HashSet::new(),
            devs_dir: read_dir("/dev/disk/by-id").or_else(|_| read_dir("/dev"))?,
        })
    }
}

impl Iterator for CdDrives {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in &mut self.devs_dir {
            if let Ok(path) = entry.and_then(|entry| canonicalize(entry.path())) {
                if is_cd_device_path(&path) && self.seen_before.insert(path.clone()) {
                    return Some(path);
                }
            }
        }
        None
    }
}

/// Checks if the path is /dev/sr followed by any number of digits.
fn is_cd_device_path(path: impl AsRef<Path>) -> bool {
    let path_bytes = path.as_ref().as_os_str().as_bytes();
    path_bytes
        .strip_prefix(b"/dev/sr")
        .map(|after_prefix| after_prefix.iter().all(|char| char.is_ascii_digit()))
        .unwrap_or(false)
}
