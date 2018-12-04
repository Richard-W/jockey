use result::{Result, Error};
use std::iter::Peekable;
use std::slice::Iter;

/// Implemented by types parsable in Arguments::parse_args().
pub trait Parsable : Sized {

    /// Parse the next argument on the iterator if possible.
    fn parse_arg(iter: &mut Peekable<Iter<String>>, option: &String) -> Option<Result<Self>>;
}

impl Parsable for String {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, option: &String) -> Option<Result<Self>> {
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
                        iter.next().cloned()
                    };

                    match value {
                        Some(value) => Some(Ok(value.to_string())),
                        None => Some(Err(Error::UnexpectedEnd)),
                    }
                }
                else {
                    // Option didn't match
                    None
                }
            }
            None => None,
        }
    }
}

impl Parsable for bool {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, option: &String) -> Option<Result<Self>> {
        match iter.peek().cloned() {
            Some(key) => {
                if key == option {
                    iter.next();
                    Some(Ok(true))
                }
                else {
                    None
                }
            },
            None => None,
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, option: &String) -> Option<Result<Self>> {
        match T::parse_arg(iter, option) {
            Some(Ok(val)) => Some(Ok(Some(val))),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}


