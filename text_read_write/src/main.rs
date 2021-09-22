extern crate path_absolutize;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use path_absolutize::*;

fn main() {
	let filename = Path::new("./data/sample.txt").absolutize().unwrap();

	let mut file = OpenOptions::new()
		.create(true)
		.write(true)
		.read(true)
		.append(true)
		.open(filename)
		.expect("Something went wrong opening the file");

	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Something went wrong reading the file");

	println!("### File contents ###");
	println!("{}", contents);

	println!("### Please input text.");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	file.write_all(input.as_bytes())
		.expect("Something went wrong writing the file");
}
