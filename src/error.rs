//! When serializing to URL parameters fails.

use serde::ser;
use std::fmt;

#[derive(Debug)]
/// Represents all possible errors that can occur when serializing into URL
/// parameters.
pub enum Error {
    /// External error caused by e.g. utf8 string conversion or io.
    Extern(Box<dyn std::error::Error + Send + Sync>),
    /// Error when tried to serialize an unsupported type.
    Unsupported(String),
    /// Custom error caused by any error while serializing a type.
    Custom(String),
}

/// Alias for `Result` with error type `serde_url_params::Error`.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Creates a new error when a type is not supported for serializing into
    /// URL parameters.
    pub fn unsupported<T: fmt::Display>(msg: T) -> Self {
        Error::Unsupported(format!("{}", msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Extern(ref err) => fmt::Display::fmt(err, f),
            Error::Unsupported(ref msg) | Error::Custom(ref msg) => fmt::Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Extern(Box::new(err))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Extern(Box::new(err))
    }
}
