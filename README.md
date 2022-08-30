# Eject

A Rust library for controlling your CD drive's tray.

Currently supports Windows and Linux.

# Example

```rust
use eject::{device::Device, discovery::first_cdrom};

// Find a drive by path
let cdrom = Device::open("/dev/cdrom").unwrap();
// Or let the crate find one for you
let cdrom = first_cdrom().unwrap();
// Open the drive's tray
cdrom.eject().unwrap();
```
