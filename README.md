# Jockey [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.org/Richard-W/jockey.svg?branch=master
[travis]: https://travis-ci.org/Richard-W/jockey
[Latest Version]: https://img.shields.io/crates/v/jockey.svg
[crates.io]: https://crates.io/crates/jockey

**Custom command-line parsers that practically write themselves.**

---

## Using jockey

Add the necessary dependencies to your Cargo.toml:

```toml
[dependencies]
jockey = "<version>"
jockey_derive = "<version>"
```

Add the crate declarations to your code:

```rust
extern crate jockey;
#[macro_use]
extern crate jockey_derive;
```

Define your arguments struct:

```rust
#[derive(Default, Arguments, Debug)]
struct MyArguments {
	pub string: Option<String>,
	pub flag: bool,
}
```

And start using it:

```rust
fn main() {
	let args = <MyArguments as jockey::Arguments>::parse_args(std::env::args())
            .expect("Error parsing command-line");

        println!("{:#?}", args);
}
```

Your application can now be called like `./dummy --string foo --flag` or `./dummy --string`. `_` characters in field names are automatically replaced by `-` characters so a field named `two_words` would yield a command-line option named `--two-words`.
