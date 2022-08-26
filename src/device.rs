use crate::{error::Result, platform::device::DeviceHandle};

/// A reference to a drive that can be used to send commands.
///
/// # Examples
///
/// ```no_run
/// use eject::device::Device;
///
/// let cdrom = Device::open("/dev/cdrom").unwrap();
/// cdrom.eject().unwrap();
/// ```
pub struct Device {
    handle: DeviceHandle,
}

impl Device {
    /// Opens a handle to a drive.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the device. For example `/dev/cdrom` on Linux or `\\.\D:` on Windows.
    /// Do not use paths to the drive's file system such as `/cdrom` or `D:\`
    pub fn open(name: &str) -> Result<Self> {
        Ok(Self {
            handle: DeviceHandle::open(name)?,
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

    /// Prevents the medium from being ejected, for example by pressing the button on a CD drive.
    /// In case of success returns an [EjectionLock] that will release the lock when dropped.
    ///
    /// Note that some devices silently ignore this command.
    pub fn lock_ejection(&self) -> Result<EjectionLock> {
        self.handle.set_ejection_lock(true)?;
        Ok(EjectionLock { device: self })
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
