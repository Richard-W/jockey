# Jockey [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/jockey.svg
[crates.io]: https://crates.io/crates/jockey

**Jockey aims to provide an easy way to parse command line options in the Rust language.**

---

## Using jockey

Add the necessary dependencies to your Cargo.toml:

```toml
[dependencies]
jockey = "0.1.0"
jockey_derive = "0.1.0"
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

impl MyArguments {
	fn new() -> Self {
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
use jockey::Arguments;

fn main() {
	let args = match MyArguments::parse_args(std::env::args().collect()) {
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
