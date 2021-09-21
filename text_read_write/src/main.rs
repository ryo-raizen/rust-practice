extern crate path_absolutize;
use std::path::Path;
use std::fs;
use std::io;
use path_absolutize::*;

fn main() {
	let filename = Path::new("./data/sample.txt").absolutize().unwrap();
	let contents = fs::read_to_string(filename.clone())
		.expect("Something went wrong reading the file");

	println!("### File contents ###");
	println!("{}", contents);
	println!("### Please input text.");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	fs::write(filename.clone(), input).expect("Unable to write file");
}
