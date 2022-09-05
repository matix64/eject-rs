# Eject

[![crates.io version](https://img.shields.io/crates/v/eject)](https://crates.io/crates/eject)
[![crates.io downloads](https://img.shields.io/crates/d/eject)](https://crates.io/crates/eject)
[![docs.rs](https://img.shields.io/docsrs/eject?logo=docs.rs)](https://docs.rs/eject)

A Rust library to control the tray of your CD drive.

Currently supporting Windows and Linux.

# Example

```rust
use eject::{device::{Device, DriveStatus}, discovery::cd_drives};

// Find a drive by its path
let cdrom = Device::open("/dev/cdrom")?;
// Open the tray
cdrom.eject()?;

// Get the paths of all CD drives
for path in cd_drives() {
    // Print the path
    println!("Drive {:?}:", path);
    // Access the drive
    let drive = Device::open(path)?;
    // Print its status
    match drive.status()? {
        DriveStatus::Empty =>
            println!("The tray is closed and no disc is inside"),
        DriveStatus::TrayOpen =>
            println!("The tray is open"),
        DriveStatus::NotReady =>
            println!("This drive is not ready yet"),
        DriveStatus::Loaded =>
            println!("There's a disc inside"),
    }
}
```
