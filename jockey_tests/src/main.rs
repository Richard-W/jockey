extern crate jockey;
#[macro_use]
extern crate jockey_derive;

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
