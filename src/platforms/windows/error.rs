use crate::error::{Error, ErrorKind};
use windows::Win32::Foundation::{
    GetLastError, ERROR_ACCESS_DENIED, ERROR_FILE_NOT_FOUND, ERROR_INVALID_NAME,
    ERROR_PATH_NOT_FOUND, WIN32_ERROR,
};

impl Error {
    pub(crate) fn get_last_error() -> Option<Self> {
        let err = unsafe { GetLastError() };
        if err.is_ok() {
            None
        } else {
            Some(Self::from_os_err(err))
        }
    }

    pub(crate) fn from_os_err(err: WIN32_ERROR) -> Self {
        Self {
            code: err.0 as i32,
            message: err.to_hresult().message().to_string_lossy(),
            kind: ErrorKind::from_os_err(err),
        }
    }
}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        let err_code = e.code().0 & 0xFF;
        Self {
            code: err_code,
            message: e.message().to_string_lossy(),
            kind: ErrorKind::from_os_err(WIN32_ERROR(err_code as u32)),
        }
    }
}

impl ErrorKind {
    fn from_os_err(err: WIN32_ERROR) -> Self {
        match err {
            ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND => Self::NotFound,
            ERROR_ACCESS_DENIED => Self::AccessDenied,
            ERROR_INVALID_NAME => Self::InvalidPath,
            _ => Self::Unknown,
        }
    }
}
