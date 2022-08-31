use crate::error::ErrorKind as CrateErrorKind;
use std::io::ErrorKind as StdErrorKind;

#[test]
fn unknown_error_to_std_io_uncategorized() {
    assert_eq!(
        format!("{:?}", StdErrorKind::from(CrateErrorKind::Unknown)),
        "Uncategorized"
    );
}
