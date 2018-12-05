extern crate jockey;
#[macro_use]
extern crate jockey_derive;

#[derive(Default, Arguments, Debug)]
struct MyArguments {
	pub string: Option<String>,
	pub flag: bool,
}

fn main() {
	let args = <MyArguments as jockey::Arguments>::parse_args(std::env::args())
            .expect("Error parsing command-line");

        println!("{:#?}", args);
}
