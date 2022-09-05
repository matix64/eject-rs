use crate::platform::discovery::CdDrives as CdDrivesImpl;
use std::path::PathBuf;

/// Returns an iterator over the paths of all available CD drives.
///
/// # Example
/// ```no_run
/// use eject::{discovery::cd_drives, device::Device};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// for path in cd_drives() {
///     println!("Found a drive at {:?}", path);
///     println!("Ejecting disc...");
///     Device::open(&path)?.eject()?;
/// }
/// # Ok(())}
/// ```
pub fn cd_drives() -> CdDrives {
    CdDrives {
        inner: CdDrivesImpl::new(),
    }
}

/// An iterator over the paths of all available CD drives.
/// Created with [`cd_drives`](super::cd_drives).
pub struct CdDrives {
    inner: CdDrivesImpl,
}

impl Iterator for CdDrives {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
