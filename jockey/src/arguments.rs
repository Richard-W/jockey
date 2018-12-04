use result::Result;

/// Represents a set of command line arguments that can be parsed.
///
/// The easiest way to get this up and running is to add `#[derive(Default, Arguments)]`
/// to the struct containing your argument data. For the derivation to work the struct may only
/// contain types that implement `Parsable`.
pub trait Arguments : Sized + Default {

    /// Parses command line arguments.
    ///
    /// This function expects an iterator as supplied by `env::args()`. The first element which
    /// typically contains the path of the executable is ignored.
    fn parse_args<I> (args: I) -> Result<Self> where I : Iterator<Item = String>;
}
