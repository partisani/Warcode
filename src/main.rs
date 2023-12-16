use pest::Parser;
use pest_derive::Parser;
use std::{env, fs};

mod parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct WarParser;

fn main() {
	let args: Vec<String> = env::args().collect();
	let file_name = &args[1];

	let file = fs::read_to_string(file_name).expect("File not found");

	let ast = WarParser::parse(Rule::main, file.as_str()).unwrap();

	println!("{}", parser::parse(ast));
}
