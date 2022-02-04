use std::fmt;

pub(crate) use internal::InternalError;

mod internal;

//pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Wrong pin (403)
    WrongPin,

    /// An Response data type was returned
    InvalidStatus,

    /// Got an FS_FAIL
    /// usually means that the action requested was not available
    /// for the current mode
    Fail,

    /// An invalid data type was returned
    InvalidData,

    /// Something went wrong
    InternalError,

    /// Server to to long to repond
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongPin => write!(f, "Looks like you got the pin wrong"),
            Error::InvalidStatus => write!(f, "An invalid status type was returned"),
            Error::Fail => write!(f, "That action is not possible (I think)"),
            Error::InvalidData => write!(f, "An invalid data type was returned"),
            Error::InternalError => write!(f, "Oops something went wrong"),
            Error::Timeout => write!(f, "Server took to long to respond"),
        }
    }
}

impl std::error::Error for Error {}

impl From<InternalError> for Error {
    fn from(_: InternalError) -> Self {
        Self::InternalError
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_status() {
            use reqwest::StatusCode;

            match err.status() {
                Some(StatusCode::FORBIDDEN) => Self::WrongPin,
                _ => Self::InternalError,
            }
        } else if err.is_timeout() {
            Self::Timeout
        } else {
            Self::InternalError
        }
    }
}
