//! Jockey provides custom command-line parsers that practically write themselves.
//!
//! This crate provides the Arguments trait which can be derived using `#[derive(Arguments)]`. That
//! trait provides the `parse_args` method which parses command-line arguments by `env::args()`.
//!
//! The following code is an example of a simple command-line parser.
//!
//! ```rust
//! extern crate jockey;
//! #[macro_use]
//! extern crate jockey_derive;
//!
//! use jockey::Arguments;
//! use std::env;
//!
//! #[derive(Arguments, Default)]
//! struct MyArguments {
//!     pub my_arg: Option<String>,
//!     pub my_flag: bool,
//! }
//!
//! fn main() {
//!     let args = MyArguments::parse_args(env::args())
//!         .expect("Error parsing command-line options");
//!
//!     println!("--my-arg = {:?}", args.my_arg);
//!     println!("--my-flag = {}", args.my_flag);
//! }
//! ```
//!
//! A more sophisticated example using field attributes can be found in the
//! [Arguments](trait.Arguments.html) documentation.
mod arguments;
pub use arguments::Arguments;

mod result;
pub use result::Error;
pub use result::Result;

mod parsable;
pub use parsable::ParsableWithOption;
pub use parsable::ParseResult;
