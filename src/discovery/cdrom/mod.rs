mod list;

pub use self::list::{cdrom_drives, CdRomDrives};
use crate::{
    device::Device,
    error::{Error, ErrorKind, Result},
    platform::discovery::CDROM_PATHS,
};

/// Tries to find and open a CD-ROM device.
///
/// If no device could be found, this will return an [Error] with `error.kind() = NotFound`.
/// Other errors indicate that a device was found but an error happened
/// while trying to open it.
pub fn first_cdrom() -> Result<Device> {
    for path in CDROM_PATHS {
        match Device::open(path) {
            Err(e) if e.kind == ErrorKind::NotFound => continue,
            x => return x,
        }
    }
    Err(Error {
        code: 0,
        message: "No CD drive found".to_owned(),
        kind: ErrorKind::NotFound,
    })
}
