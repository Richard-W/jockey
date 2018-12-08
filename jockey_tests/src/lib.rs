extern crate jockey;
#[macro_use]
extern crate jockey_derive;

#[cfg(test)]
use jockey::{Arguments, Result};

#[derive(Arguments, Default, Debug, PartialEq)]
struct TestArguments {
    #[jockey(short_option="s")]
    pub string: String,

    #[jockey(short_option="o")]
    pub option: Option<String>,

    #[jockey(short_option="f")]
    pub flag: bool,

    #[jockey(short_option="t")]
    pub two_words: Option<String>,

    #[jockey(long_option="re-named")]
    pub renamed: Option<String>,
}

#[cfg(test)]
fn parse(args: &Vec<&str>) -> Result<TestArguments> {
    let iter = args.into_iter().map(|x| x.to_string());
    <TestArguments as Arguments>::parse_args(iter)
}

#[test]
pub fn parse_single_long_option() {
    {
        let actual = parse(&vec!["dummy", "--string", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.string = "foo".into();
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "--option", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.option = Some("foo".into());
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "--flag"]).unwrap();
        let mut expected = TestArguments::default();
        expected.flag = true;
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "--two-words", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.two_words = Some("foo".into());
        assert_eq!(actual, expected);
    }
}

#[test]
pub fn parse_multiple_long_options() {
    {
        let actual = parse(&vec!["dummy", "--option", "bar", "--string", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.string = "foo".into();
        expected.option = Some("bar".into());
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "--option", "bar", "--flag", "--string", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.string = "foo".into();
        expected.option = Some("bar".into());
        expected.flag = true;
        assert_eq!(actual, expected);
    }
}

#[test]
pub fn parse_single_short_option() {
    {
        let actual = parse(&vec!["dummy", "-s", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.string = "foo".into();
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "-o", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.option = Some("foo".into());
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "-f"]).unwrap();
        let mut expected = TestArguments::default();
        expected.flag = true;
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "-t", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.two_words = Some("foo".into());
        assert_eq!(actual, expected);
    }
}

#[test]
pub fn parse_multiple_short_options() {
    {
        let actual = parse(&vec!["dummy", "-o", "bar", "-s", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.string = "foo".into();
        expected.option = Some("bar".into());
        assert_eq!(actual, expected);
    }{
        let actual = parse(&vec!["dummy", "-o", "bar", "-f", "-s", "foo"]).unwrap();
        let mut expected = TestArguments::default();
        expected.string = "foo".into();
        expected.option = Some("bar".into());
        expected.flag = true;
        assert_eq!(actual, expected);
    }
}


#[test]
pub fn parse_invalid_options() {
    match parse(&vec!["exec", "--foo", "--string", "foo"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnknownOption("--foo".into())),
    }

    match parse(&vec!["exec", "--string"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }

    match parse(&vec!["exec", "--string", "foo", "--option"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::UnexpectedEnd),
    }

    match parse(&vec!["exec", "--string", "foo", "--string", "bar"]) {
        Ok(_) => panic!(),
        Err(error) => assert_eq!(error, jockey::Error::DuplicateOption("--string".into())),
    }
}

#[test]
pub fn parse_renamed_long_option() {
    let actual = parse(&vec!["dummy", "--re-named", "foo"]).unwrap();
    let mut expected = TestArguments::default();
    expected.renamed = Some("foo".into());
    assert_eq!(actual, expected);
}

#[derive(Arguments, Default, Debug, PartialEq)]
struct TestArguments2 {
    pub string: String,

    pub multi: Vec<String>,

    #[jockey(unknown_args)]
    pub argn: Vec<String>,
}

#[cfg(test)]
fn parse2(args: &Vec<&str>) -> Result<TestArguments2> {
    let iter = args.into_iter().map(|x| x.to_string());
    <TestArguments2 as Arguments>::parse_args(iter)
}

#[test]
pub fn parse_unknown_args() {
    {
        let actual = parse2(&vec!["dummy", "a1", "--string", "foo", "a2"]).unwrap();
        let mut expected = TestArguments2::default();
        expected.string = "foo".into();
        expected.argn = vec!["a1".into(), "a2".into()];
        assert_eq!(actual, expected);
    }
}

#[test]
pub fn parse_multi_args() {
    let actual = parse2(&vec!["dummy", "--multi", "a1", "--multi", "a2"]).unwrap();
    let mut expected = TestArguments2::default();
    expected.multi = vec!["a1".into(), "a2".into()];
    assert_eq!(actual, expected);
}

#[derive(Arguments, Default, Debug, PartialEq)]
struct TestArguments3 {
    #[jockey(position=1)]
    pub subcommand: Option<String>,

    pub flag: bool,
}

#[cfg(test)]
fn parse3(args: &Vec<&str>) -> Result<TestArguments3> {
    let iter = args.into_iter().map(|x| x.to_string());
    <TestArguments3 as Arguments>::parse_args(iter)
}

#[test]
pub fn parse_positional_args() {
    let actual = parse3(&vec!["dummy", "subcommand", "--flag"]).unwrap();
    let mut expected = TestArguments3::default();
    expected.subcommand = Some("subcommand".to_string());
    expected.flag = true;
    assert_eq!(actual, expected);
}
