use result::{Result, Error};
use std::iter::Peekable;
use std::slice::Iter;

pub trait Parsable : Sized {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, long_option: String) -> Option<Result<Self>>;
}

impl Parsable for String {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, long_option: String) -> Option<Result<Self>> {
        let option_match = match iter.peek() {
            Some(x) => x.to_string() == long_option,
            None => false,
        };

        if option_match {
            iter.next();
            match iter.next() {
                Some(value) => Some(Ok(value.to_string())),
                None => Some(Err(Error::UnexpectedEnd)),
            }
        }
        else {
            None
        }
    }
}

impl Parsable for bool {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, long_option: String) -> Option<Result<Self>> {
        let result = match iter.peek() {
            Some(x) => x.to_string() == long_option,
            None => false,
        };

        if result {
            iter.next();
            Some(Ok(true))
        }
        else {
            None
        }
    }
}

impl<T : Parsable> Parsable for Option<T> {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, long_option: String) -> Option<Result<Self>> {
        match T::parse_arg(iter, long_option) {
            Some(Ok(val)) => Some(Ok(Some(val))),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}


