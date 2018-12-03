use std::error;
use std::fmt;

use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnknownOption(String),
    UnexpectedEnd,
    MissingOption(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::UnknownOption(_) => "Unknown option",
            Error::UnexpectedEnd => "Unexpected end of arguments vector",
            Error::MissingOption(_) => "Missing mandatory argument",
        }
    }
    
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownOption(which) => write!(f, "{}: {}", self.description(), which),
            Error::MissingOption(which) => write!(f, "{}: {}", self.description(), which),
            _ => write!(f, "{}", self.description()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
