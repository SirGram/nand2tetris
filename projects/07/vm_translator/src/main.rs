use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

mod code_writer;
mod parser;

enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
}

fn main() -> io::Result<()> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: vm <input_file>");
        std::process::exit(1);
    }

    // Read input file and create output file
    let input_file = &args[1];
    let output_file = Path::new("output")
        .with_extension("asm")
        .to_str()
        .unwrap()
        .to_string();

    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);

    // Read the input VM file content
    let input = std::fs::read_to_string(input_file).expect("Something went wrong reading the file");

    // Initialize parser and code writer
    let mut parser = parser::Parser::new(&input);
    let mut code_writer = code_writer::CodeWriter::new();

    // Process each line of the input file
    process(&mut parser, &mut code_writer);

    // Finalize and write the output to the ASM file
    let output = code_writer.close();
    File::create(&output_file)?.write_all(output.as_bytes())?;

    println!("Assembly code has been written to {}", output_file);
    Ok(())
}

fn process(parser: &mut parser::Parser, code_writer: &mut code_writer::CodeWriter) {
    while parser.has_more_lines() {
        parser.advance();
        if let Some(command_type) = parser.command_type() {
            match command_type {
                CommandType::CPush | CommandType::CPop => {
                    // Parse segment and index for push/pop commands
                    if let (Some(segment), Some(index)) = (parser.arg1(), parser.arg2()) {
                        let index = index.parse::<i16>().expect("Expected a number for index");
                        code_writer.write_push_pop(command_type, &segment, index);
                    }
                }
                CommandType::CArithmetic => {
                    // Parse arithmetic command
                    if let Some(command) = parser.arg1() {
                        code_writer.write_arithmetic(&command);
                    }
                }
                _ => {
                    eprintln!("Unsupported command type encountered.");
                }
            }
        }
    }
}
