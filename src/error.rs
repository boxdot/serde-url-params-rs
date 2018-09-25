//! When serializing to URL parameters fails.

use std::error;
use std::fmt;
use std::io;
use std::result;
use std::string;

use serde::ser;

#[derive(Debug)]
/// Represents all possible errors that can occur when serializing into URL
/// parameters.
pub enum Error {
    /// External error caused by e.g. utf8 string conversion or io.
    Extern(Box<error::Error>),
    /// Error when tried to serialize an unsupported type.
    Unsupported(String),
    /// Custom error caused by any error while serializing a type.
    Custom(String),
}

/// Alias for `Result` with error type `serde_url_params::Error`.
pub type Result<T> = result::Result<T, Error>;

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

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Extern(ref err) => err.description(),
            Error::Unsupported(_) => "Unsupported error",
            Error::Custom(_) => "Param error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Extern(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(format!("{}", msg))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Extern(Box::new(err))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::Extern(Box::new(err))
    }
}
