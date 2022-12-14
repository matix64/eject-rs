//! Interact with a specific device.

mod status;
#[cfg(unix)]
mod unix_trait_impls;
#[cfg(windows)]
mod windows_trait_impls;

pub use self::status::DriveStatus;
use crate::{error::Result, platform::device::DeviceHandle};
use std::{path::Path, time::Instant};

/// A reference to a device that can be used to send commands.
///
/// # Example
///
/// ```no_run
/// use eject::device::Device;
///
/// let cdrom = Device::open("/dev/cdrom")?;
/// cdrom.eject()?;
/// # eject::error::Result::Ok(())
/// ```
pub struct Device {
    handle: DeviceHandle,
}

impl Device {
    /// Opens a handle to a device.
    ///
    /// # Arguments
    ///
    /// - `path` - The path of the device.
    ///
    ///   On **Linux** this is the path of the device's file, which almost always
    ///   will be inside `/dev`. For example: `/dev/cdrom`. Do not use paths to a drive's mount point.
    ///
    ///   On **Windows** this is the path you would use with `CreateFile` but
    ///   without the `\\?\` or `\\.\` prefix. Examples of correct paths
    ///   include `D:` (but not `D:\`), `CdRom0` and `Volume{26a21bda-a627-11d7-9931-806e6f6e6963}`.
    ///   See [docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#physical-disks-and-volumes).
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            handle: DeviceHandle::open(path)?,
        })
    }

    /// Ejects the medium contained in this drive.
    pub fn eject(&self) -> Result<()> {
        self.handle.eject()
    }

    /// Closes the tray of this drive.
    pub fn retract(&self) -> Result<()> {
        self.handle.retract()
    }

    /// Opens the tray if it's closed, closes it if it's open.
    ///
    /// Returns true after opening and false after closing.
    pub fn toggle_eject(&self) -> Result<bool> {
        if let Ok(status) = self.status() {
            if status.tray_open() {
                self.retract()?;
                Ok(false)
            } else {
                self.eject()?;
                Ok(true)
            }
        } else {
            let time = Instant::now();
            self.eject()?;
            if time.elapsed().as_millis() < 100 {
                // If it was too fast it was already open
                self.retract()?;
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    /// Prevents the medium from being ejected, even if the eject button is pressed.
    ///
    /// In case of success returns an [`EjectionLock`] that will release the lock when dropped.
    ///
    /// # Platform specific behavior
    ///
    /// **Linux:** Many distros are configured in a way that makes this call useless.
    /// See <https://unix.stackexchange.com/a/104935>
    pub fn lock_ejection(&self) -> Result<EjectionLock> {
        self.handle.set_ejection_lock(true)?;
        Ok(EjectionLock { device: self })
    }

    /// Gets the position of the tray (if it exists) and whether
    /// there's data loaded in this drive.
    pub fn status(&self) -> Result<DriveStatus> {
        self.handle.status()
    }
}

/// A struct created when a drive's ejection is disabled
/// that will enable it again when dropped.
pub struct EjectionLock<'a> {
    device: &'a Device,
}

impl Drop for EjectionLock<'_> {
    fn drop(&mut self) {
        let _ = self.device.handle.set_ejection_lock(false);
    }
}
