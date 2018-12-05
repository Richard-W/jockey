use result::{Result, Error};
use std::iter::Peekable;

/// Result object for Parsable::parse_arg.
#[derive(Debug, Clone)]
pub struct ParseResult<T> {

    /// Parse result.
    pub parsed: Option<Result<T>>,

    /// An option string that should not be accepted anymore.
    pub blacklist: Option<String>,
}

impl<T> ParseResult<T> {
    /// Create a new ParseResult object.
    pub fn new(parsed: Option<Result<T>>, blacklist: Option<String>) -> Self {
        ParseResult {
            parsed: parsed,
            blacklist: blacklist,
        }
    }

    /// Create a successful ParseResult.
    pub fn success(parsed: T, blacklist: Option<String>) -> Self {
        ParseResult::new(Some(Ok(parsed)), blacklist)
    }

    /// Create an empty ParseResult.
    pub fn none() -> Self {
        ParseResult::new(None, None)
    }

    /// Create a failed ParseResult.
    pub fn err(err: Error) -> Self {
        ParseResult::new(Some(Err(err)), None)
    }
}

/// Implemented by types parsable in Arguments::parse_args().
pub trait Parsable : Sized {

    /// Parse the next argument on the iterator if possible.
    fn parse_arg<I>(iter: &mut Peekable<I>, option: &String) -> ParseResult<Self> where I: Iterator<Item = String>;

    /// Assigns the right hand side to the left hand side and returns the result.
    ///
    /// This needs to be overriden for types with multiplicity (see implementation of Parsable for
    /// Vec<String>). For types without multiplicity the default implementation will do just fine.
    fn assign(_lhs: Self, rhs: Self) -> Self {
        return rhs;
    }
}

impl Parsable for String {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: &String) -> ParseResult<Self> where I: Iterator<Item = String> {
        match iter.peek().cloned() {
            Some(val) => {
                // Split arguments of the form "--foo=bar" to "--foo" and "bar"
                let split: Vec<&str> = val.splitn(2, "=").collect();
                if split[0] == option {
                    // Advance the iterator
                    iter.next();

                    let value: Option<String> = if split.len() == 2 {
                        Some(split[1].to_string())
                    }
                    else {
                        iter.next()
                    };

                    match value {
                        Some(value) => ParseResult::success(value, Some(option.clone())),
                        None => ParseResult::err(Error::UnexpectedEnd),
                    }
                }
                else {
                    // Option didn't match
                    ParseResult::none()
                }
            }
            None => ParseResult::none(),
        }
    }
}

impl Parsable for bool {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: &String) -> ParseResult<Self> where I: Iterator<Item = String> {
        match iter.peek().cloned() {
            Some(key) => {
                if key == option.as_ref() {
                    iter.next();
                    ParseResult::success(true, Some(option.clone()))
                }
                else {
                    ParseResult::none()
                }
            },
            None => ParseResult::none(),
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: &String) -> ParseResult<Self> where I: Iterator<Item = String> {
        let result = T::parse_arg(iter, option);
        match result.parsed {
            Some(Ok(val)) => ParseResult::success(Some(val), Some(option.clone())),
            Some(Err(err)) => ParseResult::err(err),
            None => ParseResult::none(),
        }
    }
}

impl<T : Parsable> Parsable for Vec<T> {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: &String) -> ParseResult<Self> where I: Iterator<Item = String> {
        let result = T::parse_arg(iter, option);
        match result.parsed {
            Some(Ok(val)) => ParseResult::success(vec![val], None),
            Some(Err(err)) => ParseResult::err(err),
            None => ParseResult::none(),
        }
    }

    fn assign(mut lhs: Self, mut rhs: Self) -> Self {
        lhs.append(&mut rhs);
        return lhs;
    }
}
