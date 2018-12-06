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
    fn parse_arg<I>(iter: &mut Peekable<I>, option: Option<String>) -> ParseResult<Self>
        where I: Iterator<Item = (usize, String)>;

    /// Assigns the right hand side to the left hand side and returns the result.
    ///
    /// This needs to be overriden for types with multiplicity (see implementation of Parsable for
    /// Vec<String>). For types without multiplicity the default implementation will do just fine.
    fn assign(_lhs: Self, rhs: Self) -> Self {
        return rhs;
    }
}

impl Parsable for String {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: Option<String>) -> ParseResult<Self>
        where I: Iterator<Item = (usize, String)>
    {
        match option {
            Some(option) => {
                match iter.peek().cloned() {
                    Some((_, val)) => {
                        // Split arguments of the form "--foo=bar" to "--foo" and "bar"
                        let split: Vec<&str> = val.splitn(2, "=").collect();
                        if split[0] == option {
                            // Advance the iterator
                            iter.next();

                            let value: Option<String> = if split.len() == 2 {
                                Some(split[1].to_string())
                            }
                            else {
                                iter.next().map(|x| x.1)
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
            },
            None => {
                match iter.next() {
                    Some((_, val)) => ParseResult::success(val, None),
                    None => ParseResult::none(),
                }
            }
        }
    }
}

#[test]
pub fn test_parsable_for_string() {
    let args_vec = vec!["--foo", "bar"];
    let mut args = args_vec.iter()
        .map(|x| x.to_string())
        .enumerate()
        .peekable();

    let result = <String as Parsable>::parse_arg(&mut args, Some("--foo".to_string()));
    assert_eq!(result.parsed, Some(Ok("bar".into())));
    assert_eq!(result.blacklist, Some("--foo".into()));
}

impl Parsable for bool {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: Option<String>) -> ParseResult<Self>
        where I: Iterator<Item = (usize, String)>
    {
        match option {
            Some(option) => {
                match iter.peek().cloned() {
                    Some((_, key)) => {
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
            },
            None => ParseResult::none(),
        }
    }
}

#[test]
pub fn test_parsable_for_bool() {
    let args_vec = vec!["--foo"];
    let mut args = args_vec.iter()
        .map(|x| x.to_string())
        .enumerate()
        .peekable();

    let result = <bool as Parsable>::parse_arg(&mut args, Some("--foo".to_string()));
    assert_eq!(result.parsed, Some(Ok(true)));
    assert_eq!(result.blacklist, Some("--foo".into()));
}

impl<T : Parsable> Parsable for Option<T> {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: Option<String>) -> ParseResult<Self>
        where I: Iterator<Item = (usize, String)>
    {
        let result = T::parse_arg(iter, option);
        match result.parsed {
            Some(Ok(val)) => ParseResult::success(Some(val), result.blacklist),
            Some(Err(err)) => ParseResult::err(err),
            None => ParseResult::none(),
        }
    }
}

#[test]
pub fn test_parsable_for_option() {
    let args_vec = vec!["--foo", "bar"];
    let mut args = args_vec.iter()
        .map(|x| x.to_string())
        .enumerate()
        .peekable();

    let result = <Option<String> as Parsable>::parse_arg(&mut args, Some("--foo".to_string()));
    assert_eq!(result.parsed, Some(Ok(Some("bar".into()))));
    assert_eq!(result.blacklist, Some("--foo".into()));
}

impl<T : Parsable> Parsable for Vec<T> {
    fn parse_arg<I>(iter: &mut Peekable<I>, option: Option<String>) -> ParseResult<Self>
        where I: Iterator<Item = (usize, String)>
    {
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

#[test]
pub fn test_parsable_for_vec() {
    let args_vec = vec!["--foo", "bar", "--foo", "baz"];
    let mut args = args_vec.iter()
        .map(|x| x.to_string())
        .enumerate()
        .peekable();

    let tmp1 = <Vec<String> as Parsable>::parse_arg(&mut args, Some("--foo".to_string()));
    let tmp2 = <Vec<String> as Parsable>::parse_arg(&mut args, Some("--foo".to_string()));

    let result = <Vec<String> as Parsable>::assign(tmp1.parsed.unwrap().unwrap(), tmp2.parsed.unwrap().unwrap());

    assert_eq!(result, vec!["bar".to_string(), "baz".to_string()]);
    assert_eq!(tmp1.blacklist, None);
    assert_eq!(tmp2.blacklist, None);
}
