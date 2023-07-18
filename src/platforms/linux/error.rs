use crate::error::{Error, ErrorKind};
use std::io;

impl From<nix::errno::Errno> for Error {
    fn from(e: nix::errno::Errno) -> Self {
        Self {
            code: e as i32,
            message: e.to_string(),
            kind: e.into(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self {
            code: e.raw_os_error().unwrap_or(0),
            message: e.to_string(),
            kind: e.kind().into(),
        }
    }
}

impl From<nix::errno::Errno> for ErrorKind {
    fn from(e: nix::errno::Errno) -> Self {
        use nix::errno::Errno;
        match e {
            Errno::ENOSYS | Errno::EOPNOTSUPP | Errno::ENOTTY => Self::UnsupportedOperation,
            Errno::EPERM | Errno::EACCES => Self::AccessDenied,
            Errno::ENOENT => Self::NotFound,
            Errno::ENAMETOOLONG => Self::InvalidPath,
            _ => Self::Unknown,
        }
    }
}

impl From<io::ErrorKind> for ErrorKind {
    fn from(e: io::ErrorKind) -> Self {
        use io::ErrorKind;
        match e {
            ErrorKind::NotFound => Self::NotFound,
            ErrorKind::PermissionDenied => Self::AccessDenied,
            ErrorKind::Unsupported => Self::UnsupportedOperation,
            _ => Self::Unknown,
        }
    }
}
