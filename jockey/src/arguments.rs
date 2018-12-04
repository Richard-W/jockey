use result::Result;

/// Represents a set of command line arguments that can be parsed or emitted.
///
/// The easiest way to get this up and running is to add `#[derive(Default, Arguments)]`
/// to the struct containing your argument data. For the derivation to work the struct may only
/// contain types that implement both the `Parsable` and `Emittable` traits.
pub trait Arguments : Sized + Default {

    /// Emits a set of command line arguments.
    fn emit_args(&self) -> Vec<String>;

    /// Parses command line arguments.
    ///
    /// This function expects a vector as supplied by `env::args().collect()`. It
    /// ignores the first element which typically contains the path of the executable.
    fn parse_args<I> (args: I) -> Result<Self> where I : Iterator<Item = String>;
}
