// will keep this while i write subpar rust, though I might remove the pedantic if this becomes too annoying. 
pub mod parser;
pub mod lexer;
pub mod semantic; 
pub mod codegen;

use std::process;
use std::fs;

use crate::lexer::lexer_core::produce_tokens;
use crate::parser::parser_core::Parser;
use crate::parser::cfg::build_cfg;
use crate::semantic::analyser_core::analyse;
use crate::codegen::generator;
use crate::codegen::generator::generate_c;


#[cfg(test)]
mod test;
mod samples;

fn main() {
    println!("CPS2000 2026 Assignment: Compiler theory and Practice");
    println!();
    println!("This program compiles resir source code into C");
    println!();
    println!("The program takes input as follows, with the header file being optional:");
    println!(" cargo run -- <input.resir> <output.c> <header.h>");
    println!();
    println!();

    let input_args: Vec<String> = std::env::args().collect();

    if input_args.len() < 3 {
        eprintln!("Error: incorrect input format. please see above");
        std::process::exit(1);
    }

    let input_path = &input_args[1];
    let output_path = &input_args[2];

    let header_name: &str;

    if input_args.len() >= 4{
        header_name = input_args[3].as_str();
    } else {
        header_name = "custom_header.h";
    }

     // READ SOURCE ==========================================
    let source = match fs::read_to_string(input_path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("Error: could not read input file '{}'.", input_path);
            eprintln!("{err}");
            process::exit(1);
        }
    };

    // LEXER 
    let tokens = match produce_tokens(&source) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexing failed:");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    // PARSER
    let mut parser = Parser::new(tokens);

    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Parsing failed:");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    // SEMANTIC
    let cfg = build_cfg(&program.function);

    match analyse(&program, &cfg) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Semantic analysis failed:");
            eprintln!("{err}");
            process::exit(1);
        }
    }

    // CODEGEN =============================================
    let c_code = generate_c(&program, header_name);

    match fs::write(output_path, c_code) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: could not write output file '{}'.", output_path);
            eprintln!("{err}");
            process::exit(1);
        }
    }

    println!("GENERATION SUCCESSFUL");
    println!("Generated C file: {}", output_path);
    println!();
    println!("Compile it with:");
    println!("  gcc -std=c11 -Wall -Wextra -pedantic {}", output_path);
}