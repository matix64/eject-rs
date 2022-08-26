# Eject

A Rust crate for controlling your CD drive's tray.

# Example

```rust
use eject::device::Device;

let cdrom = Device::open("/dev/cdrom").unwrap();
cdrom.eject().unwrap();
```
