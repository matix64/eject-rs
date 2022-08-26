// Real world tests: run them using
// `cargo test real_dev -- --ignored --nocapture --test-threads 1`
// You'll need a cd drive. The tests will try to find it but
// if that doesn't work set env variable TEST_CD to the drive's path

use crate::device::Device;
use std::{
    env,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

#[test]
#[ignore]
fn a_eject() {
    get_device().eject().unwrap();
}

#[test]
#[ignore]
fn b_retract() {
    get_device().retract().unwrap();
}

#[test]
#[ignore]
fn c_lock_ejection() {
    let dev = get_device();
    let guard = dev.lock_ejection().unwrap();
    print!("locked! releasing in 5 seconds ... ");
    let _ = stdout().flush();
    sleep(Duration::from_secs(5));
    drop(guard);
}

fn get_device() -> Device {
    #[cfg(windows)]
    sleep(Duration::from_secs_f32(0.5));
    #[allow(unreachable_code)]
    let drive_path = env::var("TEST_CD").unwrap_or_else(|_| {
        #[cfg(windows)]
        return "CdRom0".to_owned();
        #[cfg(target_os = "linux")]
        return "/dev/cdrom".to_owned();
        panic!("Set env. variable TEST_CD to your cd drive's path")
    });
    Device::open(&drive_path).expect("opening device")
}
