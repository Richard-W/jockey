pub trait Arguments {

    fn to_args(self) -> Vec<String>;

    fn parse_args(args: Vec<String>) -> Self;
}
