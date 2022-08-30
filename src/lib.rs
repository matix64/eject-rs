//! A crate for controlling the tray of your CD drive.
//!
//! # Example
//!
//! ```no_run
//! use eject::{device::Device, discovery::first_cdrom};
//!
//! // Find a drive by path
//! let cdrom = Device::open("/dev/cdrom").unwrap();
//! // Or let the crate find one for you
//! let cdrom = first_cdrom().unwrap();
//! // Open the drive's tray
//! cdrom.eject().unwrap();
//! ```

/// Interact with a specific device.
pub mod device;
/// Find available devices.
pub mod discovery;
/// Errors returned by this crate.
pub mod error;
#[cfg_attr(windows, path = "platforms/windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "platforms/linux/mod.rs")]
#[cfg_attr(target_os = "macos", path = "platforms/macos/mod.rs")]
mod platform;
#[cfg(test)]
mod tests;
