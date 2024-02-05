#![allow(nonstandard_style)]
use std::fs;
use std::env;
use std::io::Write;
use std::process::Command;
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

    let mut file = fs::File::create("out.asm").expect("Unable to create file");
    file.write_all(result.as_bytes()).expect("Unable to write data");
    
    let output = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("out.asm")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    let output = Command::new("ld")
        .arg("-o")
        .arg("out")
        .arg("out.o")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));



}