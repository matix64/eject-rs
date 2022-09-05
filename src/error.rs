//! Errors returned by this crate.
//!
//! Errors will try to be classified into the categories defined
//! in the [`ErrorKind`](ErrorKind) enum.
//! ```
//! use eject::{device::Device, error::ErrorKind};
//!
//! let error = Device::open("doesntexist").err().unwrap();
//! assert_eq!(error.kind(), ErrorKind::NotFound);
//! ```
//!
//! You can convert an [`Error`] to an [`std::io::Error`]
//! ```
//! use eject::device::Device;
//! use std::{fs::File, io};
//!
//! let std_err = File::open("doesntexist").err().unwrap();
//! let eject_err: io::Error = Device::open("doesntexist").err().unwrap().into();
//!
//! assert_eq!(std_err.to_string(), eject_err.to_string());
//! assert_eq!(std_err.kind(), eject_err.kind());
//! assert_eq!(std_err.raw_os_error(), eject_err.raw_os_error());
//! ```
//!
//! If the error comes from the OS, you can get its OS specific code.
//! ```
//! use eject::device::Device;
//!
//! let error = Device::open("doesntexist").err().unwrap();
//! println!("Error code: {}", error.os_code().unwrap());
//! ```

/// Result returned by functions in this crate.
///
/// See the [`error`][crate::error] module docs for details and examples.
pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
#[error("{}", message)]
/// Error type for functions in this crate.
///
/// See the [`error`][crate::error] module docs for more details and examples.
pub struct Error {
    /// OS error code, or 0 if the error doesn't come from the OS.
    pub(crate) code: i32,
    /// User friendly error messages, which come from the OS in OS errors.
    pub(crate) message: String,
    /// OS agnostic error category.
    pub(crate) kind: ErrorKind,
}

impl Error {
    /// Returns the OS specific error code or `None` if the
    /// error doesn't come directly from the OS.
    pub fn os_code(&self) -> Option<i32> {
        if self.code == 0 {
            None
        } else {
            Some(self.code)
        }
    }

    /// Returns an OS agnostic category for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        if e.code == 0 {
            Self::new(e.kind.into(), e.message)
        } else {
            Self::from_raw_os_error(e.code)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[non_exhaustive]
/// General categories for OS and library errors.
///
/// Bear in mind that errors that are currently [`Unknown`][Self::Unknown] may be moved to
/// a different category in the future. This would not be considered a breaking
/// change. Instead of matching against [`Unknown`][Self::Unknown], use a wildcard pattern: `_ => `
///
/// See the [`error`][crate::error] module docs for details and examples.
pub enum ErrorKind {
    /// The operation failed due to a permission issue.
    AccessDenied,
    /// The file or path doesn't exist.
    NotFound,
    /// The path contains invalid characters or is improperly formatted.
    InvalidPath,
    /// The device doesn't support performing this operation.
    /// This can often happen when a device is not of the type you expect,
    /// or you've opened something that is not a device, like a regular file.
    UnsupportedOperation,
    /// The category of this error could not be determined.
    Unknown,
}

impl From<ErrorKind> for std::io::ErrorKind {
    fn from(e: ErrorKind) -> Self {
        match e {
            ErrorKind::AccessDenied => Self::PermissionDenied,
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::InvalidPath => Self::InvalidInput,
            ErrorKind::UnsupportedOperation => Self::Unsupported,
            ErrorKind::Unknown => std::io::Error::from_raw_os_error(498498498).kind(),
        }
    }
}
