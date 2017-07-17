use std::error;
use std::fmt;
use std::result;
use std::io;
use std::string;

use serde::ser;

#[derive(Debug)]
pub enum Error {
    Inner(Box<error::Error>),
    Custom(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Inner(ref err) => fmt::Display::fmt(err, f),
            Error::Custom(ref msg) => fmt::Display::fmt(msg, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Inner(ref err) => err.description(),
            _ => "Param error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Inner(ref err) => err.cause(),
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
        Error::Inner(Box::new(err))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::Inner(Box::new(err))
    }
}
