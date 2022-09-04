use crate::platform::discovery::CdRomDrives as CdRomDrivesImpl;
use std::path::PathBuf;

/// Returns an iterator over the paths of all available CD-ROM drives.
///
/// # Example
/// ```no_run
/// use eject::{discovery::cdrom_drives, device::Device};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// for path in cdrom_drives() {
///     println!("Found a drive at {:?}", path);
///     println!("Ejecting disc...");
///     Device::open(&path)?.eject()?;
/// }
/// # Ok(())}
/// ```
pub fn cdrom_drives() -> CdRomDrives {
    CdRomDrives {
        inner: CdRomDrivesImpl::new(),
    }
}

/// An iterator over the paths of available CD-ROM drives.
/// Created with [cdrom_drives](super::cdrom_drives).
pub struct CdRomDrives {
    inner: CdRomDrivesImpl,
}

impl Iterator for CdRomDrives {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
