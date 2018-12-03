use std::error;
use std::fmt;

use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnknownOption,
    UnexpectedEnd,
    MissingOption,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::UnknownOption => "Unknown option",
            Error::UnexpectedEnd => "Unexpected end of arguments vector",
            Error::MissingOption => "Missing mandatory argument",
        }
    }
    
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jockey error: {}", self.description())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
