pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
#[error("{}", message)]
pub struct Error {
    pub(crate) code: i32,
    pub message: String,
    pub kind: ErrorKind,
}

impl Error {
    pub(crate) fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        Self::from_raw_os_error(e.code)
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
