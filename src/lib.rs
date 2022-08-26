//! A crate for controlling the tray of your CD drive.
//!
//! # Example
//!
//! ```no_run
//! use eject::device::Device;
//!
//! let cdrom = Device::open("/dev/cdrom").unwrap();
//! cdrom.eject().unwrap();
//! ```

/// Interact with a specific device.
pub mod device;
/// Errors returned by this crate.
pub mod error;
#[cfg_attr(windows, path = "platforms/windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "platforms/linux/mod.rs")]
#[cfg_attr(target_os = "macos", path = "platforms/macos/mod.rs")]
mod platform;
#[cfg(test)]
mod tests;
