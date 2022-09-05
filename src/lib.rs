//! A crate to control the tray of your CD drive.
//!
//! [`Device`][device::Device] contains methods to open drives and send commands to them.
//!
//! [`cd_drives`][discovery::cd_drives] allows you to find all CD drives on a system.
//!
//! # Example
//!
//! ```no_run
//! use eject::{device::Device, discovery::cd_drives};
//!
//! // Open the drive at this path
//! let cdrom = Device::open("/dev/cdrom")?;
//! // Or get the first one available
//! let cdrom_path = cd_drives().next().unwrap();
//! let cdrom = Device::open(&cdrom_path)?;
//! // Open the tray
//! cdrom.eject()?;
//! # eject::error::Result::Ok(())
//! ```

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

pub mod device;
pub mod discovery;
pub mod error;
#[cfg_attr(windows, path = "platforms/windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "platforms/linux/mod.rs")]
#[cfg_attr(target_os = "macos", path = "platforms/macos/mod.rs")]
mod platform;
#[cfg(test)]
mod tests;
