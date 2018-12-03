use result::{Result, Error};
use std::iter::Peekable;
use std::slice::Iter;

/// Represents a set of command line arguments that can be parsed or emitted
///
/// This trait is typically implemented by `#[derive(JockeyArguments)]`. The types
/// currently supported in this derivation are
///
/// * String - Which is a mandatory option
/// * Option<String> - Which is an optional option
/// * bool - Which is a flag
///
/// The derivation macro also requires a static `new()` function that requires
/// no further arguments and initializes `Option<String>` fields to `None` and
/// `bool` fields to `false`.
pub trait Arguments : Sized {

    /// Emits a set of command line arguments.
    fn to_args(&self) -> Vec<String>;

    /// Parses command line arguments.
    ///
    /// This function expects a vector as supplied by `env::args().collect()`. It
    /// ignores the first element which typically contains the path of the executable.
    fn parse_args(args: Vec<String>) -> Result<Self>;
}

pub trait Parsable : Sized {
    fn parse_arg(iter: &mut Peekable<Iter<String>>, long_option: String) -> Option<Result<Self>>;
}

pub trait Emittable {
    fn emit_args(&self, long_option: String) -> Vec<String>;
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

impl Emittable for String {
    fn emit_args(&self, long_option: String) -> Vec<String> {
        vec![long_option, self.to_string()]
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

impl Emittable for bool {
    fn emit_args(&self, long_option: String) -> Vec<String> {
        if self.clone() {
            vec![long_option]
        }
        else {
            vec![]
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

impl<T: Emittable> Emittable for Option<T> {
    fn emit_args(&self, long_option: String) -> Vec<String> {
        match self {
            Some(ref val) => val.emit_args(long_option),
            None => vec![],
        }
    }
}
