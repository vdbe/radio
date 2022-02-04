use fsapi::Error as FsApiError;
use std::fmt;
use tokio::task::JoinError;

/// The default error type for this crate
#[derive(Debug)]
pub enum Error {
    /// Wrong pin
    Auth,

    /// Oops we looks like I made a mistake
    Oops,

    /// Server took to long to responde
    Timeout,

    /// An invalid value was provided to a setter
    InvalidValue,

    /// An empty error without further info for when you are lazy
    Empty,

    /// Could not get a lock
    Lock,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Auth => write!(f, "Auth"),
            Error::Timeout => write!(f, "Timeout"),
            Error::Oops => write!(f, "Oops"),
            Error::InvalidValue => write!(f, "Invalid value"),
            Error::Lock => write!(f, "Could not get a lock"),
            Error::Empty => write!(f, ""),
        }
    }
}

impl std::error::Error for Error {}

impl From<FsApiError> for Error {
    fn from(err: FsApiError) -> Self {
        match err {
            FsApiError::WrongPin => Error::Auth,
            FsApiError::Timeout => Error::Timeout,
            FsApiError::Fail => Error::InvalidValue,
            _ => Error::Oops,
        }
    }
}

impl From<JoinError> for Error {
    fn from(_: JoinError) -> Self {
        Self::Oops
    }
}
