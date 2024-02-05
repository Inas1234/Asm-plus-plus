#![allow(nonstandard_style)]
use std::fs;
use std::env;
mod tokenizer;
mod parser;
mod generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut tokenizer = tokenizer::Tokenizer::new(contents);
    let tokens = tokenizer.tokenize();
    let mut parser = parser::Parser::new(tokens);
    let node = parser.parse_prog();
    let generator = generator::Generator::new(node);
    let result = generator.generate();    
    println!("{}", result);
}