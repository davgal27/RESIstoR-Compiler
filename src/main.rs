#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
pub mod lexer;
use crate::lexer::lexer_core::produce_token; // why not compuling without this is mystery 

fn main() {
	// TESTING ============================
	let input = r#"function Math::abs_diff(%a: i32, %b: i32) -> i32 {
	    locals {yay it works}
	"#;
	println!("\n\n\n======INPUT======\n{input}\n");
	println!("======OUTPUT=====");

	let tokens = produce_token(input);

	for token in tokens {
		println!("{}", token);
	}
}