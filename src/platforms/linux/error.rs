use crate::error::{Error, ErrorKind};

impl From<nix::errno::Errno> for Error {
    fn from(e: nix::errno::Errno) -> Self {
        Self {
            code: e as i32,
            message: e.to_string(),
            kind: e.into(),
        }
    }
}

impl From<nix::errno::Errno> for ErrorKind {
    fn from(e: nix::errno::Errno) -> Self {
        use nix::errno::Errno;
        match e {
            Errno::ENOSYS | Errno::EOPNOTSUPP => Self::UnsupportedOperation,
            Errno::EPERM | Errno::EACCES => Self::AccessDenied,
            Errno::ENOENT => Self::NotFound,
            Errno::ENAMETOOLONG => Self::InvalidPath,
            _ => Self::Unknown,
        }
    }
}
