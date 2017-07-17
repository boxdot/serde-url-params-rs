use std::error;
use std::fmt;
use std::result;
use std::io;
use std::string;

use serde::ser;

#[derive(Debug)]
pub enum Error {
    Extern(Box<error::Error>),
    Unsupported(String),
    Custom(String),
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn unsupported<T: fmt::Display>(msg: T) -> Self {
        Error::Unsupported(format!("{}", msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Extern(ref err) => fmt::Display::fmt(err, f),
            Error::Unsupported(ref msg) |
            Error::Custom(ref msg) => fmt::Display::fmt(msg, f),
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
