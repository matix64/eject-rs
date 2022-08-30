// Real world tests: run them using
// `cargo test real_dev -- --ignored --nocapture --test-threads 1`
// You'll need a cd drive. The tests will try to find it but
// if that doesn't work set env variable TEST_CD to the drive's path

use crate::{device::Device, discovery::first_cdrom, error::ErrorKind};
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
    if let Ok(path) = env::var("TEST_CD") {
        Device::open(&path).expect("opening device")
    } else {
        match first_cdrom() {
            Err(e) if e.kind == ErrorKind::NotFound => {
                panic!(
                    "first_cdrom() didn't work: \
                    set env. variable TEST_CD to your cd drive's path"
                )
            }
            x => x.expect("opening device"),
        }
    }
}
