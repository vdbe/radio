use std::fmt;

use super::Error;
use crate::macros::quick_impl;

/// The default internal error type for this crate
#[derive(Debug)]
pub(crate) enum InternalError {
    Value(String),

    SessionId(String),

    Response(String),

    Item(String),

    Notify(String),

    Field(String),

    /// Just a generic error without dedicated variant,
    /// with a string to store a description
    Generic(String),

    /// An empty error without further info for when you are lazy
    Empty,
}

impl InternalError {
    pub fn new<T: ToString>(msg: T) -> Self {
        Self::Generic(msg.to_string())
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalError::Value(s) => write!(f, "Error value: {s}"),
            InternalError::SessionId(s) => write!(f, "Error session id: {s}"),
            InternalError::Notify(s) => write!(f, "Error notify: {s}"),
            InternalError::Response(s) => write!(f, "Error response: {s}"),
            InternalError::Item(s) => write!(f, "Error item: {s}"),
            InternalError::Field(s) => write!(f, "Error field: {s}"),
            InternalError::Generic(desc) => write!(f, "Error: {desc}"),
            InternalError::Empty => write!(f, "Error"),
        }
    }
}

impl std::error::Error for InternalError {}

impl From<Error> for InternalError {
    fn from(_: Error) -> Self {
        Self::Empty
    }
}

quick_impl!(From<reqwest::Error> for InternalError);
quick_impl!(From<quick_xml::Error> for InternalError);
quick_impl!(From<std::num::ParseIntError> for InternalError);
quick_impl!(From<std::string::FromUtf8Error> for InternalError);
