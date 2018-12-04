extern crate jockey;
#[macro_use]
extern crate jockey_derive;

#[cfg(test)]
use jockey::Arguments;

#[derive(JockeyArguments)]
struct SimpleArguments {
    #[jockey(short_option="d")]
    pub defaulted: String,

    #[jockey(short_option="o")]
    pub optional: Option<String>,

    #[jockey(short_option="f")]
    pub flag: bool,

    #[jockey(long_option="multiple-words")]
    pub multiple_words: Option<String>,
}

impl Default for SimpleArguments {
    fn default() -> Self {
        SimpleArguments {
            defaulted: "default_value".into(),
            optional: None,
            flag: false,
            multiple_words: None,
        }
    }
}

#[test]
pub fn parse_simple_arguments() {
    let args1 = <SimpleArguments as Arguments>::parse_args(vec!["exec".into(), "--defaulted".into(), "foo".into(), "--flag".into()]).unwrap();
    assert_eq!(args1.defaulted, "foo");
    assert_eq!(args1.optional, None);
    assert_eq!(args1.flag, true);

    let args2 = <SimpleArguments as Arguments>::parse_args(vec!["exec".into(), "--defaulted".into(), "foo".into(), "--flag".into(), "--optional".into(), "bar".into()]).unwrap();
    assert_eq!(args2.defaulted, "foo");
    assert_eq!(args2.optional, Some("bar".into()));
    assert_eq!(args2.flag, true);

    let args3 = <SimpleArguments as Arguments>::parse_args(vec!["exec".into(), "--defaulted=foo".into(), "--flag".into(), "--optional=bar".into()]).unwrap();
    assert_eq!(args3.defaulted, "foo");
    assert_eq!(args3.optional, Some("bar".into()));
    assert_eq!(args3.flag, true);

    let args4 = <SimpleArguments as Arguments>::parse_args(vec!["exec".into(), "--defaulted=foo".into(), "--flag".into(), "--optional=bar".into(), "--multiple-words".into(), "baz".into()]).unwrap();
    assert_eq!(args4.defaulted, "foo");
    assert_eq!(args4.optional, Some("bar".into()));
    assert_eq!(args4.flag, true);
    assert_eq!(args4.multiple_words, Some("baz".into()));
}

#[test]
pub fn parse_short_options() {
    let args1 = <SimpleArguments as Arguments>::parse_args(vec!["exec".into(), "-d".into(), "foo".into(), "-f".into()]).unwrap();
    assert_eq!(args1.defaulted, "foo");
    assert_eq!(args1.optional, None);
    assert_eq!(args1.flag, true);
}

#[test]
pub fn parse_simple_arguments_errors() {
    match SimpleArguments::parse_args(vec!["exec".into(), "--foo".into(), "--defaulted".into(), "foo".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnknownOption("--foo".into())),
    }

    match SimpleArguments::parse_args(vec!["exec".into(), "--defaulted".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }

    match SimpleArguments::parse_args(vec!["exec".into(), "--defaulted".into(), "foo".into(), "--optional".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }
}

#[test]
pub fn output_simple_arguments() {
    let mut args1 = SimpleArguments::default();
    args1.defaulted = "foo".into();
    args1.flag = true;

    let expected1: Vec<String> = vec!["--defaulted".into(), "foo".into(), "--flag".into()];
    assert_eq!(args1.emit_args(), expected1);

    let mut args2 = SimpleArguments::default();
    args2.defaulted = "foo".into();
    args2.optional = Some("bar".into());
    args2.flag = true;

    let expected2: Vec<String> = vec!["--defaulted".into(), "foo".into(), "--optional".into(), "bar".into(), "--flag".into()];
    assert_eq!(args2.emit_args(), expected2);
}
