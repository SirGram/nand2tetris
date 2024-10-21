use std::env;
use std::fs::File;
use std::io::{self, Write};

mod code;
mod parser;
mod symbol_table;

fn decimal_to_binary(decimal: i32) -> String {
    format!("{:016b}", decimal)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: assembler <input file>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = "output.hack";
    println!("Input file: {}", input_file);

    let input = std::fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let mut symbol_table = symbol_table::SymbolTable::new();
    symbol_table.add_predefined_symbols();

    let mut parser = parser::Parser::new(&input);
    let output_symbols = process_instructions(&mut parser, &mut symbol_table)?;

    write_output(output_file, &output_symbols)?;

    Ok(())
}

fn process_instructions(
    parser: &mut parser::Parser,
    symbol_table: &mut symbol_table::SymbolTable,
) -> io::Result<Vec<String>> {
    let mut rom_address = 0;
    let mut starting_address = 16;
    let mut output_symbols = Vec::new();

    // First pass: Process L-instructions
    while parser.has_more_lines() {
        parser.advance();
        if let Some(parser::InstructionType::L_INSTRUCTION) = parser.instruction_type() {
            if let Some(symbol) = parser.symbol() {
                if !symbol_table.contains(&symbol) {
                    symbol_table.add_entry(&symbol, rom_address);
                }
            }
        } else if parser.instruction_type().is_some() {
            rom_address += 1;
        }
    }

    // Second pass: Process A and C instructions
    parser.reset();
    while parser.has_more_lines() {
        parser.advance();
        match parser.instruction_type() {
            Some(parser::InstructionType::A_INSTRUCTION) => {
                process_a_instruction(parser, symbol_table, &mut starting_address, &mut output_symbols);
            }
            Some(parser::InstructionType::C_INSTRUCTION) => {
                process_c_instruction(parser, &mut output_symbols);
            }
            _ => {}
        }
    }

    Ok(output_symbols)
}

fn process_a_instruction(
    parser: &parser::Parser,
    symbol_table: &mut symbol_table::SymbolTable,
    starting_address: &mut i32,
    output_symbols: &mut Vec<String>,
) {
    if let Some(symbol) = parser.symbol() {
        let address = if let Ok(num) = symbol.parse::<i32>() {
            num
        } else {
            if !symbol_table.contains(&symbol) {
                symbol_table.add_entry(&symbol, *starting_address as usize);
                *starting_address += 1;
            }
            symbol_table.getAddress(&symbol).unwrap()  as i32
        };
        output_symbols.push(decimal_to_binary(address));
    }
}

fn process_c_instruction(parser: &parser::Parser, output_symbols: &mut Vec<String>) {
    let dest = parser.dest().unwrap_or_else(|| "null".to_string());
    let comp = parser.comp().unwrap_or_else(|| "0".to_string());
    let jump = parser.jump().unwrap_or_else(|| "null".to_string());

    let a_bit = if comp.contains('M') { "1" } else { "0" };
    let comp_bits = code::comp(&comp).unwrap_or("0101010");
    let dest_bits = code::dest(&dest).unwrap_or("000");
    let jump_bits = code::jump(&jump).unwrap_or("000");

    let c_instruction_binary = format!(
        "111{}{}{}{}",
        a_bit,
        comp_bits,
        dest_bits,
        jump_bits
    );
    output_symbols.push(c_instruction_binary);
}

fn write_output(output_file: &str, output_symbols: &[String]) -> io::Result<()> {
    let mut file = File::create(output_file)?;
    for symbol in output_symbols {
        writeln!(file, "{}", symbol)?;
    }
    Ok(())
}