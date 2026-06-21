#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
pub mod lexer;
pub mod parser;
use crate::lexer::lexer_core::produce_token; 
use crate::parser::parser_core::Parser;


fn main() {
	let input = r#"
	function Math::abs_diff(%a: i32, %b: i32) -> i32 {
	    locals {
	        %zero : i32;
	        %d : i32;
	        %is_neg : bool;
	        %r : i32;
	    }
	    entry bb0;

	    bb0:
	        %zero = const 0;
	        %d = bin sub %a, %b;
	        %is_neg = bin lt %d, %zero;
	        cjump %is_neg, bb1, bb2;

	    bb1:
	        %r = un neg %d;
	        return %r;

	    bb2:
	        return %d;
	}
	"#;

	println!("\n\n\n======INPUT======\n{input}\n");
	println!("======LEXER OUTPUT=====");
	let tokens = produce_token(input);

	for token in &tokens {
		println!("{}", token);
	}

	println!("\n=======PARSER OUTPUT=======");
	let mut parser = Parser::new(tokens);

    match parser.parse_program() {
        Ok(program) => println!("{:#?}", program),
        Err(message) => eprintln!("{}", message),
    }
}