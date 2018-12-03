use result::Result;

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
    fn emit_args(&self) -> Vec<String>;

    /// Parses command line arguments.
    ///
    /// This function expects a vector as supplied by `env::args().collect()`. It
    /// ignores the first element which typically contains the path of the executable.
    fn parse_args(args: Vec<String>) -> Result<Self>;
}
