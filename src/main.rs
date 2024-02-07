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
    // Check for minimum arguments needed after including -f and -o options
    if args.len() < 6 {
        println!("Usage: {} -f <format> -o <output file name> <input file name>", args[0]);
        return;
    }
    
    // Parsing command line arguments for -f and -o options
    let format = match args.iter().position(|x| x == "-f") {
        Some(index) => args.get(index + 1).expect("Format not specified after -f").to_string(),
        None => {
            println!("Format not specified. Use -f option.");
            return;
        },
    };

    let output_file_name = match args.iter().position(|x| x == "-o") {
        Some(index) => args.get(index + 1).expect("Output file name not specified after -o").to_string(),
        None => {
            println!("Output file name not specified. Use -o option.");
            return;
        },
    };

    let input_file_name = args.last().expect("No input file name provided");

    let contents = fs::read_to_string(input_file_name).expect("Something went wrong reading the file");
    let mut tokenizer = tokenizer::Tokenizer::new(contents);
    let tokens = tokenizer.tokenize();
    let mut parser = parser::Parser::new(tokens);
    let node = parser.parse_prog();
    let generator = generator::Generator::new(node);
    let result = generator.generate();

    // Use the specified output file name for the assembly file
    let asm_file_name = format!("{}.asm", output_file_name);
    let mut file = fs::File::create(&asm_file_name).expect("Unable to create file");
    file.write_all(result.as_bytes()).expect("Unable to write data");

    let output = Command::new("nasm")
        .arg("-f")
        .arg(&format)
        .arg(&asm_file_name)
        .arg("-o")
        .arg(format!("{}.o", output_file_name))
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    // Use the specified output file name for the linker output
    let output = Command::new("ld")
        .arg("-o")
        .arg(&output_file_name)
        .arg(format!("{}.o", output_file_name))
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
