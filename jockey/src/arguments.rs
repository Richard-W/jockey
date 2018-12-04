use result::Result;

/// Represents a set of command line arguments that can be parsed.
///
/// The easiest way to get this up and running is to add `#[derive(Default, Arguments)]`
/// to the struct containing your argument data. For the derivation to work the struct may only
/// contain types that implement `Parsable`.
///
/// # Example
///
/// ```
/// # extern crate jockey;
/// # #[macro_use] extern crate jockey_derive;
/// # fn main() {
/// #[derive(Arguments, Default)]
/// struct MyArguments {
///
///     // Is set if "--my-string <value>" or "--my-string=<value>" is given.
///     pub my_string: Option<String>,
///
///     // This defines a short option so both "--with-short-opt" and "-s" will work.
///     #[jockey(short_option="s")]
///     pub with_short_opt: Option<String>,
///
///     // A flag which is set to true if "--flag" (without a value) is given.
///     pub flag: bool,
///
///     // A catch-all where all otherwise unrecognized command line options will be stored.
///     #[jockey(unknown_args)]
///     pub argn: Vec<String>,
/// };
///
/// // Emulate the behaviour of env::args().
/// let args_vec = vec!["/path/to/my/executable",
///     "--my-string", "value1",
///     "-s", "value2",
///     "--flag",
///     "./file1", "./file2"];
/// let args_iter = args_vec.iter().map(|x| x.to_string());
///
/// use jockey::Arguments;
///
/// let args = MyArguments::parse_args(args_iter).unwrap();
///
/// assert_eq!(args.my_string, Some("value1".into()));
/// assert_eq!(args.with_short_opt, Some("value2".into()));
/// assert_eq!(args.flag, true);
/// assert_eq!(args.argn, vec!["./file1".to_string(), "./file2".to_string()]);
/// # }
/// ```
pub trait Arguments : Sized + Default {

    /// Parses command line arguments.
    ///
    /// This function expects an iterator as supplied by `env::args()`. The first element which
    /// typically contains the path of the executable is ignored.
    fn parse_args<I> (args: I) -> Result<Self> where I : Iterator<Item = String>;
}
