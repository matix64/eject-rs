//! A crate to control the tray of your CD drive.
//!
//! # Example
//!
//!```no_run
//! use eject::{device::{Device, DriveStatus}, discovery::cd_drives};
//!
//! # fn main() -> eject::error::Result<()> {
//! // Find a drive by its path
//! let cdrom = Device::open("/dev/cdrom")?;
//! // Open the tray
//! cdrom.eject()?;
//!
//! // Get the paths of all CD drives
//! for path in cd_drives() {
//!     // Print the path
//!     println!("Drive {:?}:", path);
//!     // Access the drive
//!     let drive = Device::open(path)?;
//!     // Print its status
//!     match drive.status()? {
//!         DriveStatus::Empty =>
//!             println!("The tray is closed and no disc is inside"),
//!         DriveStatus::TrayOpen =>
//!             println!("The tray is open"),
//!         DriveStatus::NotReady =>
//!             println!("This drive is not ready yet"),
//!         DriveStatus::Loaded =>
//!             println!("There's a disc inside"),
//!     }
//! }
//! # Ok(())}
//!```

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
