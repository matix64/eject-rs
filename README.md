# Eject

[![crates.io version](https://img.shields.io/crates/v/eject)](https://crates.io/crates/eject)
[![crates.io downloads](https://img.shields.io/crates/d/eject)](https://crates.io/crates/eject)
[![docs.rs](https://img.shields.io/docsrs/eject?logo=docs.rs)](https://docs.rs/eject)

A Rust library to control the tray of your CD drive.

Currently supporting Windows and Linux.

# Features

- [x] Find installed drives.
- [x] Open, close and lock the tray.
- [x] Query current tray position and whether there's a disc inside.

# Examples

## Basic usage

```rust
use eject::{device::Device, discovery::cd_drives};

// Open the drive at this path
let cdrom = Device::open("/dev/cdrom")?;
// Or get the first one available
let cdrom_path = cd_drives().next().unwrap();
let cdrom = Device::open(&cdrom_path)?;
// Open the tray
cdrom.eject()?;
```

## Find all CD drives

```rust
use eject::{device::Device, discovery::cd_drives};

// Get the paths of all CD drives
for path in cd_drives() {
    // Print the path
    println!("{:?}", path);
    // Access the drive
    let drive = Device::open(path)?;
    // Close its tray
    drive.retract()?;
}
```

## Get drive status

```rust
use eject::{device::{Device, DriveStatus}, discovery::cd_drives};

// Open a drive
let drive = Device::open("/dev/cdrom")?;
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
```
