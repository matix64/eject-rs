pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
#[error("{}", message)]
pub struct Error {
    /// OS error code, or 0 if the error doesn't come from the OS.
    pub(crate) code: i32,
    pub message: String,
    pub kind: ErrorKind,
}

impl Error {
    /// Return the OS-specific error code or `None` if the
    /// error doesn't come directly from the OS.
    pub fn os_code(&self) -> Option<i32> {
        if self.code == 0 {
            None
        } else {
            Some(self.code)
        }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum ErrorKind {
    AccessDenied,
    NotFound,
    InvalidPath,
    UnsupportedOperation,
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
