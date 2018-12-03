extern crate jockey;
#[macro_use]
extern crate jockey_derive;

use jockey::Arguments;

#[derive(JockeyArguments)]
struct SimpleArguments {
    pub mandatory: String,

    pub optional: Option<String>,

    pub flag: bool,
}

impl SimpleArguments {
    pub fn new() -> Self {
        SimpleArguments {
            mandatory: "".into(),
            optional: None,
            flag: false,
        }
    }
}

#[test]
pub fn parse_simple_arguments() {
    let args1 = SimpleArguments::parse_args(vec!["--mandatory".into(), "foo".into(), "--flag".into()]).unwrap();
    assert_eq!(args1.mandatory, "foo");
    assert_eq!(args1.optional, None);
    assert_eq!(args1.flag, true);

    let args2 = SimpleArguments::parse_args(vec!["--mandatory".into(), "foo".into(), "--flag".into(), "--optional".into(), "bar".into()]).unwrap();
    assert_eq!(args2.mandatory, "foo");
    assert_eq!(args2.optional, Some("bar".into()));
    assert_eq!(args2.flag, true);
}

#[test]
pub fn parse_simple_arguments_errors() {
    match SimpleArguments::parse_args(vec!["--foo".into(), "--mandatory".into(), "foo".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnknownOption),
    }

    match SimpleArguments::parse_args(vec!["--mandatory".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }

    match SimpleArguments::parse_args(vec!["--mandatory".into(), "foo".into(), "--optional".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }

    match SimpleArguments::parse_args(vec!["--optional".into(), "foo".into()]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::MissingOption),
    }
}

#[test]
pub fn output_simple_arguments() {
    let mut args1 = SimpleArguments::new();
    args1.mandatory = "foo".into();
    args1.flag = true;

    let expected1: Vec<String> = vec!["--mandatory".into(), "foo".into(), "--flag".into()];
    assert_eq!(args1.to_args(), expected1);

    let mut args2 = SimpleArguments::new();
    args2.mandatory = "foo".into();
    args2.optional = Some("bar".into());
    args2.flag = true;

    let expected2: Vec<String> = vec!["--mandatory".into(), "foo".into(), "--optional".into(), "bar".into(), "--flag".into()];
    assert_eq!(args2.to_args(), expected2);
}
