use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Battlefield;

fn main() {
    println!("Hello, world!");
}
