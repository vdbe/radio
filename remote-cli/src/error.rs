use fsapi::Error as FsApiError;
#[derive(Debug)]
pub enum Error {
    NoConfig(String),
    InvalidConfig(String),
    InvalidPin,
    InvalidCommand,
    Internal,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConfig(msg) => write!(f, "No config file was found: {msg}"),
            Self::InvalidConfig(msg) => write!(f, "Invalid config file: {msg}"),
            Self::InvalidPin => write!(f, "Provided pin is invalid"),
            Self::Internal => write!(f, "Internal error"),
            Self::InvalidCommand => write!(f, "Invalid command provided"),
        }
    }
}

impl std::error::Error for Error {}

impl From<FsApiError> for Error {
    fn from(err: FsApiError) -> Self {
        match err {
            FsApiError::WrongPin => Self::InvalidPin,
            _ => Self::Internal,
        }
    }
}
