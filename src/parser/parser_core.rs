use super::ir::*;
use crate::lexer::tokens::{Token, TokenKind}; 

pub struct Parser {
	tokens: Vec<Token>, // batch of tokens from lexer 
	current: usize, // index of next token to consume 
}

impl Parser {

	pub fn new(tokens: Vec<Token>) -> Parser {
		Parser {
			tokens,
			current: 0,
		}
	}

	// =================HELPER FUNCTIONS========================
	/* 
	Most of these helpers are inspired from the "jlox" 
	implementation from Crafting Interpreters,
	part of which demonstrates the construction of a recursive descent
	parser of a simple language in java.
	*/ 

	fn yell_error(&self, message: &str) -> String {
		format!("Syntax Error: {} at Line: {}, Col: {}",
			message,
			t.token_attribute.line,
			t.token_attribute.col,
			)
	}

    // look but dont consume 
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
 
    // what did we just consume?
    fn get_previous_token(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
 
    fn is_at_end(&self) -> bool {
        self.peek().token_kind == TokenKind::EndOfFile
    }
 
    // is the current token this kind?
    fn check_tokenkind(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
        	return false; 
        }
        self.peek().token_kind == kind
    }
 
    // consume the current token and return it
    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous().clone()
    }
 	// look AND consume 
    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }
        false
    }

    // if the current token is what we expect: consume and return
    // else error
    fn consume_token(&mut self, kind: TokenKind, message: &str) -> Result<Token, String> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek().clone(), message))
    }

	// ========================Parse x ===============================
	// parse x => returns Result<x, String>.
	// ? operator: if err return, 

	fn parse_program(&mut self) -> Result<Program, String> {

		let mut externtypes = Vec::new();
		while self.check(TokenKind::Extern) {
			externtype.push(self.parse_extern_type()?); //? operator helps the file not be a bajillion lines long
		}

		let function = self.parse_function()?;

		if self.is_at_end() == false{
			return Err(self.yell_error("nothing should be written after function"))
		} 
		// okay if everything succeeds from the ?. If a ? fails then error
		Ok(Program {
			externtypes,
			function,
		})
	}

	fn parse_externtype(&mut self) -> Result<ExternType, String> {

		self.consume_token(TokenKind::Extern, "expected 'extern'")?;
		self.consume_token(TokenKind::Type, "expected 'type' after 'extern'")?;

		let path = self.parse_path()?;

		self.consume_token(TokenKind::LCurly, "expected {")?;

		let mut fields = Vec::new();
		while self.check(TokenKind::Field) {
			fields.push(self.parse_field()?);
			self.consume_token(TokenKind::SemiColon, "expected ;")?;
		}

		self.consume_token(TokenKind::SemiColon, "expected ;")?;
		self.consume_token(TokenKind::RCurly, "expected }")?;

		Ok(ExternType {
			path,
			fields, 
		})
	}

	fn parse_field(&mut self) -> Result<Field, String> {
		let ident = self.parse_ident()?;

		self.consume_token(TokenKind::Colon, "expected :")?;

		let typealt = self.parse_type()?;

		Ok(Field {
			ident,
			typealt
		})
	}

	fn parse_function(&mut self) -> Result<Function, String> {
		self.consume_token(TokenKind::Function, "expected 'function'")?;
		
		let path = self.parse_path()?;

		self.consume_token(TokenKind::LBrack, "expected (")?;

		//optional
		// check if there is a ) right after (, then param list empty
		let rbrack_present = self.check(TokenKind::RBrack);
		let mut params = None;
		if rbrack_present == false {
			params = Some(self.parse_params()?); 
		}

		self.consume_token(TokenKind::RBrack, "expected )")?;

		self.consume_token(TokenKind::Arrow, "expected Arrow")?;

		let rettype = self.parse_rettype()?;

		self.consume_token(TokenKind::LCurly, "expected {")?;

		self.consume_token(TokenKind::Locals, "expected Locals")?;

		self.consume_token(TokenKind::LCurly, "expected {")?;

		let mut locals = Vec::new(); 
		while self.check(TokenKind::Local) {
			let local = self.parse_local()?;
			self.consume_token(TokenKind::Colon, "expected : ")?;
			let typealt = self.parse_type()?;
			self.consume_token(TokenKind::SemiColon, "expected ; after specifying type!")?;
			locals.push((local, typealt)); // psh pair of local and type 
		}

		self.consume_token(TokenKind::RCurly, "expected } ")?;

		self.consume_token(TokenKind::Entry, "expected 'entry' ")?;

		let entry = self.parse_label()?;

		self.consume_token(TokenKind::SemiColon, "expected ; after entry label")?;

		let mut blocks = Vec::new();
		while self.check(TokenKind::Label) {
			blocks.push(self.parse_block()?);
		}

		self.consume_token(TokenKind::RCurly, "expected }")?; 

		Ok(Function{
			path,
			params,
			rettype,
			locals,
			entry,
			blocks
		})
	}

	fn parse_params (&mut self) -> Result<Params, String> {
		let mut params = Vec::new(); 
		params.push(self.parse_param()?);
		while self.check(TokenKind::Comma) {
			params.push(self.parse_param()?);
		}
		Ok(Params{
			params,
		})

	}

    fn parse_param(&mut self) -> Result<Param, String> {
    	let local = self.parse_local()?;

    	self.consume_token(TokenKind::Colon, "expected : between identifier and type")?;

    	let typealt = self.parse_type()?;

    	Ok(Param{
    		local,
    		typealt,
    	})
    }


	fn parse_rettype(&mut self) -> Result<RetType, String> {
	    let is_void = self.match_token(TokenKind::Void);
	    if is_void == true {
	        return Ok(RetType::Void);
	    }
	    let typealt = self.parse_type()?;
	    Ok(RetType::typealt(typealt))
	}

	fn parse_block

	fn parse_stmt

	fn parse_rhs

	fn pars_args

	fn pars_term

	fn parse_type

	fn parse_primtype

	fn parse_path

	fn parse_local

	fn parse_label

	fn parse_literal

	fn parse_unop

	fn parse_binop

	fn parse_ident

	fn parse_digit
