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
mod tests {
    use super::device::Device;
    use std::{env, thread::sleep, time::Duration};

    // Run this using `cargo test real -- --nocapture --ignored`
    // You'll need a cd drive. The test will try to find it but
    // if that doesn't work set env variable TEST_CD to the drive's path
    #[test]
    #[ignore]
    fn test_real() {
        #[allow(unreachable_code)]
        let drive_path = env::var("TEST_CD").unwrap_or_else(|_| {
            #[cfg(windows)]
            return "CdRom0".to_owned();
            #[cfg(target_os = "linux")]
            return "/dev/cdrom".to_owned();
            panic!("Set env. variable TEST_CD to your cd drive's path")
        });
        let dev = Device::open(&drive_path).expect("opening device");
        println!("Ejecting...");
        dev.eject().unwrap();
        println!("Retracting...");
        dev.retract().unwrap();
        println!("Locking door...");
        let guard = dev.lock_ejection().unwrap();
        println!("Locked! Releasing in 5 seconds...");
        sleep(Duration::from_secs(5));
        drop(guard);
        println!("Lock released");
    }
}
