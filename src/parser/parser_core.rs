use super::ir::*;
use crate::lexer::tokens::{Token, TokenKind}; 

pub struct Parser {
	tokens: Vec<Token>, // batch of tokens from lexer 
	current: usize, // index of next token to consume 
}

impl Parser {

	// =================HELPER FUNCTIONS========================
	/* 
	Most of these helpers are inspired from the "jlox" 
	implementation from Crafting Interpreters,
	part of which demonstrates the construction of a recursive descent
	parser of a simple language in java.
	*/ 
	pub fn new(tokens: Vec<Token>) -> Parser {
		Parser {
			tokens,
			current: 0,
		}
	}

	fn yell_error(&self, message: &str) -> String {
		let t = self.peek();
		format!("Syntax Error: Received:{} but expected: {}. Line: {}, Col: {}",
			t.token_attribute.lexeme,
			message,
			t.token_attribute.line,
			t.token_attribute.col,
			)
	}

	// ========================Parse x ===============================
	// parse x => returns Result<x, String>.
	// ? operator: if err return, 

	fn parse_program(&mut self) -> Result<Program, String> {

	let mut externtype = Vec::new();
	// {} so we can have more than one, loop 
	while self.check(TokenKind::Extern) {
		externtype.push(self.parse_extern_type()?);
	}

	let function = self.parse_function()?;
	self.consume(TokenKind::EndOfFile, "expected end of input")?; 

	// okay if everything succeeds from the ?. If a ? fails then error
	Ok(Program {
		externtype: externtype,
		function,
	})
	}

	fn parse_externtype

	fn parse_field

	fn parse_function

	fn parse_params

	fn parse_param

	fn parse_rettype

	fn parse_block

	fn parse_stmt

	fn parse_rhs

	fn pars_args

	fn pars_term

	fn parse_rettype

	fn parse_primtype

	fn parse_path

	fn parse_local

	fn parse_label

	fn parse_literal

	fn parse_unop

	fn parse_binop
