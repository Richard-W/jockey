# Jockey [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.org/Richard-W/jockey.svg?branch=master
[travis]: https://travis-ci.org/Richard-W/jockey
[Latest Version]: https://img.shields.io/crates/v/jockey.svg
[crates.io]: https://crates.io/crates/jockey

**Jockey aims to provide an easy way to parse command line options in the Rust language.**

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
#[derive(JockeyArguments)]
struct MyArguments {
	pub defaulted: String,
	pub optional: Option<String>,
	pub flag: bool,
}

impl Default for MyArguments {
	fn default() -> Self {
		MyArguments {
			defaulted: "default_value".into(),
			optional: None,
			flag: false,
		}
	}
}
```

And start using it:

```rust
fn main() {
	let args = match <MyArguments as jockey::Arguments>::parse_args(std::env::args().collect()) {
		Ok(args) => args,
		Err(err) => panic!("Error parsing command line: {}", err),
	};

	println!("--defaulted \"{}\"", args.defaulted);
	match args.optional {
		Some(x) => println!("--optional Some(\"{}\")", x),
		_ => println!("--optional None"),
	};
	println!("--flag {}", args.flag);
}
```

Instead of defining the implementation for `Default` yourself you can also use `#[derive(Default, JockeyArguments)]` and be done with it.
