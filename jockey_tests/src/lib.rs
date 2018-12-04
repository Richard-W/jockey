extern crate jockey;
#[macro_use]
extern crate jockey_derive;

#[cfg(test)]
use jockey::{Arguments, Result};

#[derive(Default, Arguments)]
struct TestArguments {
    #[jockey(short_option="d")]
    pub defaulted: String,

    #[jockey(short_option="o")]
    pub optional: Option<String>,

    #[jockey(short_option="f")]
    pub flag: bool,

    #[jockey(long_option="multiple-words", short_option="m")]
    pub multiple_words: Option<String>,
}

#[cfg(test)]
fn parse(args: &Vec<&str>) -> Result<TestArguments> {
    let iter = args.into_iter().map(|x| x.to_string());
    <TestArguments as Arguments>::parse_args(iter)
}

#[test]
pub fn parse_simple_arguments() {
    let args1 = parse(&vec!["exec", "--defaulted", "foo", "--flag"]).unwrap();
    assert_eq!(args1.defaulted, "foo");
    assert_eq!(args1.optional, None);
    assert_eq!(args1.flag, true);

    let args2 = parse(&vec!["exec", "--defaulted", "foo", "--flag", "--optional", "bar"]).unwrap();
    assert_eq!(args2.defaulted, "foo");
    assert_eq!(args2.optional, Some("bar".into()));
    assert_eq!(args2.flag, true);

    let args3 = parse(&vec!["exec", "--defaulted=foo", "--flag", "--optional=bar"]).unwrap();
    assert_eq!(args3.defaulted, "foo");
    assert_eq!(args3.optional, Some("bar".into()));
    assert_eq!(args3.flag, true);

    let args4 = parse(&vec!["exec", "--defaulted=foo", "--flag", "--optional=bar", "--multiple-words", "baz"]).unwrap();
    assert_eq!(args4.defaulted, "foo");
    assert_eq!(args4.optional, Some("bar".into()));
    assert_eq!(args4.flag, true);
    assert_eq!(args4.multiple_words, Some("baz".into()));
}

#[test]
pub fn parse_short_options() {
    let args1 = parse(&vec!["exec", "-d", "foo", "-f"]).unwrap();
    assert_eq!(args1.defaulted, "foo");
    assert_eq!(args1.optional, None);
    assert_eq!(args1.flag, true);

    let args2 = parse(&vec!["exec", "-d", "foo", "-f", "-m", "baz"]).unwrap();
    assert_eq!(args2.defaulted, "foo");
    assert_eq!(args2.optional, None);
    assert_eq!(args2.flag, true);
    assert_eq!(args2.multiple_words, Some("baz".into()));
}

#[test]
pub fn parse_simple_arguments_errors() {
    match parse(&vec!["exec", "--foo", "--defaulted", "foo"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnknownOption("--foo".into())),
    }

    match parse(&vec!["exec", "--defaulted"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }

    match parse(&vec!["exec", "--defaulted", "foo", "--optional"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }
}

#[test]
pub fn output_simple_arguments() {
    let mut args1 = TestArguments::default();
    args1.defaulted = "foo".into();
    args1.flag = true;

    let expected1: Vec<String> = vec!["--defaulted".into(), "foo".into(), "--flag".into()];
    assert_eq!(args1.emit_args(), expected1);

    let mut args2 = TestArguments::default();
    args2.defaulted = "foo".into();
    args2.optional = Some("bar".into());
    args2.flag = true;

    let expected2: Vec<String> = vec!["--defaulted".into(), "foo".into(), "--optional".into(), "bar".into(), "--flag".into()];
    assert_eq!(args2.emit_args(), expected2);
}
