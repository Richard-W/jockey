# Jockey

**Jockey aims to provide an easy way to parse command line options in the Rust language.**

---

## Using jockey

Add the necessary dependencies to your Cargo.toml (as soon as 0.1.0 is released - which is not the case yet):

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
	pub mandatory: String,
	pub optional: Option<String>,
	pub flag: bool,
};

impl MyArguments {
	fn new() -> Self {
		MyArguments {
			mandatory: "",
			optional: None,
			flag: false,
		}
	}
}
```

And start using it:

```rust
fn main() {
	let args = match MyArguments::parse_args(std::env::args().collect()) {
		Ok(args) => args,
		Err(err) => panic!("Error parsing command line: {}", err),
	};

	println!("--mandatory \"{}\"", args.mandatory);
	match args.optional {
		Some(x) => println!("--optional Some(\"{}\")", x),
		_ => println!("--optional None"),
	};
	println!("--flag {}", args.flag);
}
```
